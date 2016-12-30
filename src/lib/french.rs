// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::borrow::Cow;
use std::default::Default;

use common::{NB_CHAR, NB_CHAR_NARROW, NB_CHAR_EM};
use common::is_whitespace;
use clean;

/// Output format, to determine how to escape characters
enum Output {
    Default,
    Latex,
}


/// French typographic formatter.
///
/// The purpose of this struct is to try to make a text more typographically correct,
/// according to french typographic rules. This means:
///
/// * making spaces before `?`, `!`, `;` narrow non-breaking space;
/// * making spaces before `:` non-breaking space;
/// * making space after `—` for dialog a demi em space;
/// * making spaces after `«` and before `»` non-breking space or narrow non-breking space,
///   according to the circumstances (dialog or a few quoted words).
/// * making spaces in numbers, e.g. `80 000` or `50 €` narrow and non-breaking.
///
/// Additionally, this feature use functions that are "generic" (not specific to french language)
/// in order to:
///
/// * replace straight quotes (`'` and `"`) with curly, typographic ones;
/// * replace ellipsis (`...`) with the unicode character (`…`).
///
/// As some of these features require a bit of guessing sometimes, there are some paremeters that
/// can be set if you want better results.
///
/// # Example
///
/// ```
/// use crowbook_text_processing::FrenchFormatter;
/// let input = "Un texte à 'formater', n'est-ce pas ?";
/// let output = FrenchFormatter::new()
///              .typographic_ellipsis(false) // don't replace ellipsis
///              .format_tex(input); // format to tex (so non-breaking
///                                  // spaces are visible in assert_eq!)
/// assert_eq!(&output, "Un texte à ‘formater’, n’est-ce pas~?");
/// ```
#[derive(Debug)]
pub struct FrenchFormatter {
    /// After that number of characters, assume it's not a currency
    threshold_currency: usize,
    /// After that number of characters assume it's not an unit
    threshold_unit: usize,
    /// After that number of characters, assume it is a dialog
    threshold_quote: usize,
    /// After that number of characters, assume it isn't an abbreviation
    threshold_real_word: usize,
    /// Enable typographic apostrophe
    typographic_quotes: bool,
    /// Enaple typographic ellipsis
    typographic_ellipsis: bool,
    /// Enable dashes replacement
    ligature_dashes: bool,
    /// Enable guillemets replacement
    ligature_guillemets: bool,
}

impl Default for FrenchFormatter {
    fn default() -> Self {
        FrenchFormatter {
            threshold_currency: 3,
            threshold_unit: 2,
            threshold_quote: 20,
            threshold_real_word: 3,
            typographic_quotes: true,
            typographic_ellipsis: true,
            ligature_dashes: false,
            ligature_guillemets: false,
        }
    }
}

impl FrenchFormatter {
    /// Create a new FrenchFormatter with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the threshold currency.
    ///
    /// After that number of characters, assume it's not a currency
    ///
    /// Default is `3`.
    pub fn threshold_currency(&mut self, t: usize) -> &mut Self {
        self.threshold_currency = t;
        self
    }

    /// Sets the threshold for unit.
    ///
    /// After that number of characters, assume it's not an unit.
    ///
    /// Default is `2`.
    pub fn threshold_unit(&mut self, t: usize) -> &mut Self {
        self.threshold_unit = t;
        self
    }

    /// Sets the threshold for quote.
    ///
    /// After that number of characters, assume it's not a quote of a single
    /// word or a few words, but a dialog.
    ///
    /// Default is `20`.
    pub fn threshold_quote(&mut self, t: usize) -> &mut Self {
        self.threshold_quote = t;
        self
    }

    /// Sets the threshold for real word.
    ///
    /// After that number of characters, assume it's not an abbreviation
    /// but a real word (used to determine if `.` marks the end of a sentence
    /// or just a title such as `M. Dupuis`.
    ///
    /// Default is `3`
    pub fn threshold_real_word(&mut self, t: usize) -> &mut Self {
        self.threshold_real_word = t;
        self
    }

    /// Enables the typographic quotes replacement.
    ///
    /// If true, "L'" will be replaced by "L’"
    ///
    /// Default is true
    pub fn typographic_quotes(&mut self, b: bool) -> &mut Self {
        self.typographic_quotes = b;
        self
    }

    /// Enables typographic ellipsis replacement.
    ///
    /// If true, "..." will be replaced by "…"
    ///
    /// Default is true
    pub fn typographic_ellipsis(&mut self, b: bool) -> &mut Self {
        self.typographic_ellipsis = b;
        self
    }

    /// If set to true, replaces `--`to `–` and `---` to `—`.
    ///
    /// Default is false.
    pub fn ligature_dashes(&mut self, b: bool) -> &mut Self {
        self.ligature_dashes = b;
        self
    }

    /// If set to true, replaces `<<` to `«` and `>>` to `»`.
    ///
    /// Default is false.
    pub fn ligature_guillemets(&mut self, b: bool) -> &mut Self {
        self.ligature_guillemets = b;
        self
    }

    /// (Try to) Format a string according to french typographic rules.
    ///
    /// This method should be called for each paragraph, as it makes some suppositions that
    /// the beginning of the string also means the beginning of a line.
    ///
    /// This method calls `remove_whitespaces` internally, as it relies on it.
    ///
    /// # Example
    ///
    /// ```
    /// use crowbook_text_processing::FrenchFormatter;
    /// let f = FrenchFormatter::new();
    /// let s = f.format("« Est-ce bien formaté ? » se demandait-elle — les espaces \
    ///                   insécables étaient tellement compliquées à gérer,
    ///                   dans cette langue !");
    /// println!("{}", s);
    /// ```
    pub fn format<'a, S: Into<Cow<'a, str>>>(&self, input: S) -> Cow<'a, str> {
        self.format_output(input, Output::Default)
    }

    /// (Try to) Format a string according to french typographic rules, and use '~' so it works
    /// correctly with LaTeX output.
    ///
    /// # Example
    ///
    /// ```
    /// use crowbook_text_processing::FrenchFormatter;
    /// let f = FrenchFormatter::new();
    /// let s = f.format_tex("« Est-ce bien formaté ? »");
    /// assert_eq!(&s, "«~Est-ce bien formaté~?~»");
    /// ```
    pub fn format_tex<'a, S: Into<Cow<'a, str>>>(&self, input: S) -> Cow<'a, str> {
        self.format_output(input, Output::Latex)
    }


    /// (Try to) Format a string according to french typographic rules, and escaping non-breaking
    /// spaces according to output format
    fn format_output<'a, S: Into<Cow<'a, str>>>(&self, input: S, output: Output) -> Cow<'a, str> {
        let mut input = clean::whitespaces(input); // first pass to remove whitespaces

        if self.ligature_dashes {
            input = clean::dashes(input);
        }

        if self.ligature_guillemets {
            input = clean::guillemets(input);
        }

        if self.typographic_quotes {
            input = clean::quotes(input);
        }

        if self.typographic_ellipsis {
            input = clean::ellipsis(input);
        }

        // Find first characters that are trouble
        let first = input.chars().position(is_trouble);
        let first_number = input.chars().position(|c| c.is_digit(10));

        // No need to do anything, return early
        if first.is_none() && first_number.is_none() {
            return input;
        }

        let (nb_char, nb_char_em, nb_char_narrow) = match output {
            Output::Default => (NB_CHAR, NB_CHAR_EM, NB_CHAR_NARROW),
            Output::Latex => ('~', '~', '~'),
        };

        let mut chars = input.chars().collect::<Vec<_>>();
        let mut is_number_series = false;

        // Handle numbers
        if let Some(first) = first_number {
            // Go back one step
            let first = if first > 1 { first - 1 } else { 0 };
            for i in first..(chars.len() - 1) {
                // Handle numbers (that's easy)
                let current = chars[i];
                let next = chars[i + 1];

                match current {
                    '0'...'9' => {
                        if i == 0 || !chars[i - 1].is_alphabetic() {
                            is_number_series = true;
                        }
                    }
                    c if c.is_whitespace() => {
                        if is_number_series &&
                           (next.is_digit(10) || self.char_is_symbol(&chars, i + 1)) {
                            // Next char is a number or symbol such as $, and previous was number
                            chars[i] = nb_char_narrow;
                        }
                    }
                    _ => {
                        is_number_series = false;
                    }
                }
            }
        }

        // Handle the rest
        if let Some(first) = first {
            // Go back one step
            let first = if first > 1 { first - 1 } else { 0 };
            for i in first..(chars.len() - 1) {
                let current = chars[i];
                let next = chars[i + 1];
                if is_whitespace(current) {
                    match next {
                        // handle narrow nb space before char
                        '?' | '!' | ';' => chars[i] = nb_char_narrow,
                        ':' => chars[i] = nb_char,
                        '»' => {
                            if current == ' ' {
                                // Assumne that if it isn't a normal space it
                                // was used here for good reason, don't replace it
                                chars[i] = nb_char;
                            }
                        }
                        _ => (),
                    }
                } else {
                    match current {
                        // handle nb space after char
                        '—' | '«' | '-' | '–' => {
                            if is_whitespace(next) {
                                let replacing_char = match current {
                                    '—' | '-' | '–' => {
                                        if i <= 1 {
                                            nb_char_em
                                        } else if chars[i - 1] == nb_char {
                                            // non breaking space before, so probably
                                            // should have a breakable one after
                                            ' '
                                        } else {
                                            if let Some(closing) =
                                                   self.find_closing_dash(&chars, i + 1) {
                                                chars[closing] = nb_char;
                                            }
                                            nb_char
                                        }
                                    }
                                    '«' => {
                                        let j = find_next(&chars, '»', i);
                                        if let Some(j) = j {
                                            if chars[j - 1].is_whitespace() {
                                                if i <= 1 ||
                                                    j - i > self.threshold_quote {
                                                        // Either '«' was at the beginning
                                                        // => assume it is a dialogue
                                                        // or it's a quote
                                                        // => 'large' space too
                                                        chars[j - 1] = nb_char;
                                                        nb_char
                                                    } else {
                                                        // Not long enough to be a quote,
                                                        // use narrow nb char
                                                        chars[j - 1] = nb_char_narrow;
                                                        nb_char_narrow
                                                    }
                                            } else {
                                                // wtf formatting?
                                                nb_char
                                            }
                                        } else {
                                            // No ending quote found, assume is a dialogue
                                            nb_char
                                        }
                                    }, // TODO: better heuristic: use narrow nb_char if not at front?
                                    _ => unreachable!(),
                                };
                                chars[i + 1] = replacing_char;
                        }
                        }
                        _ => (),
                    }
                }
            }
        }
        Cow::Owned(chars.into_iter().collect())
    }

    /// Return true if the character is a symbol that is used after number
    /// and should have a nb_char before
    fn char_is_symbol(&self, v: &[char], i: usize) -> bool {
        let is_next_letter = if i < v.len() - 1 {
            v[i + 1].is_alphabetic()
        } else {
            false
        };
        if is_next_letter {
            match v[i] {
                '°' => true,
                c if c.is_uppercase() => {
                    let word = get_next_word(v, i);
                    if word.len() > self.threshold_currency {
                        // not a currency
                        false
                    } else {
                        // if all uppercase and less than THRESHOLD,
                        // assume it's a currency or a unit
                        word.iter().all(|c| c.is_uppercase())
                    }
                }
                c if c.is_alphabetic() => {
                    let word = get_next_word(v, i);
                    // if two letters, assume it is a unit
                    word.len() <= self.threshold_unit
                }
                _ => false,
            }
        } else {
            match v[i] {
                c if (!c.is_alphabetic() && !c.is_whitespace()) => true, // special symbol
                c if c.is_uppercase() => true, //single uppercase letter
                _ => false,
            }
        }
    }

    // Return Some(pos) if a closing dash was found before what looks
    // like the end of a sentence, None else
    fn find_closing_dash(&self, v: &[char], n: usize) -> Option<usize> {
        let mut word = String::new();
        for j in n..v.len() {
            match v[j] {
                '!' | '?' => {
                    if is_next_char_uppercase(v, j + 1) {
                        return None;
                    }
                }
                '-' | '–' | '—' => {
                    if v[j - 1].is_whitespace() {
                        return Some(j - 1);
                    }
                }
                '.' => {
                    if !is_next_char_uppercase(v, j + 1) {
                        continue;
                    } else if let Some(c) = word.chars().next() {
                        if !c.is_uppercase() || word.len() > self.threshold_real_word {
                            return None;
                        }
                    }
                }
                c if c.is_whitespace() => word = String::new(),
                c => word.push(c),
            }
        }
        None
    }
}

fn is_trouble(c: char) -> bool {
    match c {
        '?' | '!' | ';' | ':' | '»' | '«' | '—' | '–' => true,
        _ => false,
    }
}



// Find first char `c` in slice `v` after index `n`
fn find_next(v: &[char], c: char, n: usize) -> Option<usize> {
    for i in n..v.len() {
        if v[i] == c {
            return Some(i);
        }
    }
    None
}

// Return true if next non whitespace char in `v` after index `n` is uppercase
fn is_next_char_uppercase(v: &[char], n: usize) -> bool {
    for i in n..v.len() {
        if v[i].is_whitespace() {
            continue;
        }
        if v[i].is_uppercase() {
            return true;
        }
        if v[i].is_lowercase() {
            return false;
        }
    }
    false
}


/// Returns the next word in `v` starting from index `n`
fn get_next_word(v: &[char], n: usize) -> &[char] {
    let mut beginning = n;
    let mut end = v.len();

    for i in n..v.len() {
        if v[i].is_alphabetic() {
            beginning = i;
            break;
        }
    }

    for i in beginning..v.len() {
        if v[i].is_whitespace() {
            end = i - 1;
            break;
        }
    }

    &v[beginning..end]
}


#[cfg(test)]
#[test]
fn french() {
    let s = "  «  Comment allez-vous ? » demanda-t-elle à son   \
             interlocutrice  qui lui répondit  \
             : « Mais très bien ma chère  !  »";
    let res = FrenchFormatter::new().format(s);
    assert_eq!(&res,
               " « Comment allez-vous ? » demanda-t-elle à son \
                interlocutrice qui lui répondit : \
                « Mais très bien ma chère ! »");
}

#[test]
fn french_quotes_1() {
    let s = "« Un test »";
    let res = FrenchFormatter::new().format_tex(s);
    assert_eq!(&res, "«~Un test~»");
}

#[test]
fn french_quotes_2() {
    let s = "« Un test";
    let res = FrenchFormatter::new().format_tex(s);
    assert_eq!(&res, "«~Un test");
}

#[test]
fn french_quotes_3() {
    let s = "Un test »";
    let res = FrenchFormatter::new().format_tex(s);
    assert_eq!(&res, "Un test~»");
}

#[test]
fn french_quotes_4() {
    let s = "test « court »";
    let res = FrenchFormatter::new().format(s);
    assert_eq!(&res, "test « court »");
}

#[test]
fn french_quotes_5() {
    let s = "test « beaucoup, beaucoup plus long »";
    let res = FrenchFormatter::new().format(s);
    assert_eq!(&res, "test « beaucoup, beaucoup plus long »");
}

#[test]
fn french_dashes_1() {
    let s = "Il faudrait gérer ces tirets – sans ça certains textes rendent mal – un jour ou \
             l'autre";
    let res = FrenchFormatter::new().format_tex(s);
    assert_eq!(&res,
               "Il faudrait gérer ces tirets –~sans ça certains textes \
                rendent mal~– un jour ou l’autre");
}

#[test]
fn french_dashes_2() {
    let s = "Il faudrait gérer ces tirets – sans ça certains textes rendent mal. Mais ce n'est \
             pas si simple – si ?";
    let res = FrenchFormatter::new().format_tex(s);
    assert_eq!(&res,
               "Il faudrait gérer ces tirets –~sans ça certains textes rendent mal. Mais ce \
                n’est pas si simple –~si~?");
}

#[test]
fn french_numbers() {
    let french = FrenchFormatter::new();

    let s = Cow::Borrowed("10 000");
    let res = french.format_tex(s);
    assert_eq!(&res, "10~000");

    let s = Cow::Borrowed("10 000 €");
    let res = french.format_tex(s);
    assert_eq!(&res, "10~000~€");

    let s = Cow::Borrowed("10 000 euros");
    let res = french.format_tex(s);
    assert_eq!(&res, "10~000 euros");

    let s = Cow::Borrowed("10 000 EUR");
    let res = french.format_tex(s);
    assert_eq!(&res, "10~000~EUR");

    let s = Cow::Borrowed("50 km");
    let res = french.format_tex(s);
    assert_eq!(&res, "50~km");

    let s = Cow::Borrowed("50 %");
    let res = french.format_tex(s);
    assert_eq!(&res, "50~%");

    let s = Cow::Borrowed("20 °C");
    let res = french.format_tex(s);
    assert_eq!(&res, "20~°C");

    let s = Cow::Borrowed("20 F");
    let res = french.format_tex(s);
    assert_eq!(&res, "20~F");

    let s = Cow::Borrowed("20 BALLES");
    let res = french.format_tex(s);
    assert_eq!(&res, "20 BALLES");
}
