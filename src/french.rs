// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

use common::{is_whitespace, NB_CHAR, NB_CHAR_NARROW, NB_CHAR_EM};
use clean::remove_whitespaces;

use std::borrow::Cow;

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
/// As this requires a bit of guessing sometimes, there are some paremeters that can be set
/// if you want better results.
pub struct FrenchFormatter {
    /// After that number of characters, assume it's not a currency
    threshold_currency: usize,
    /// After that number of characters assume it's not an unit
    threshold_unit: usize,
    /// After that number of characters, assume it is a dialog
    threshold_quote: usize,
    /// After that number of characters, assume it isn't an abbreviation
    threshold_real_word: usize,
}

impl FrenchFormatter {
    /// Create a new FrenchFormatter with default settings
    pub fn new() -> FrenchFormatter {
        FrenchFormatter {
            threshold_currency: 3,
            threshold_unit: 2,
            threshold_quote: 28,
            threshold_real_word: 3,
        }
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
    /// Default is `28` (just enough for « anticonstitutionnellement »).
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
    /// use crowbook_text_processing::french::FrenchFormatter;
    /// let f = FrenchFormatter::new();
    /// let s = f.format("« Est-ce bien formaté ? » se demandait-elle — les espaces insécables étaient tellement compliquées à gérer, dans cette langue !");
    /// println!("{}", s);
    /// ```
    pub fn format<'a, S:Into<Cow<'a, str>>>(&self, input: S) -> Cow<'a, str> {
        let input = remove_whitespaces(input); // first pass to remove whitespaces

        // Find first character that is trouble
        let first = input.chars().position(is_trouble);
        let first_number = input.chars().position(|c| c.is_digit(10));

        // No need to do anything, return early
        if first.is_none() && first_number.is_none() {
            return input;
        }

        let mut found_opening_quote = false; // we didn't find an opening quote yet
        let mut chars = input.chars().collect::<Vec<_>>();
        let mut is_number_series = false;

        // Handle numbers
        if let Some(first) = first_number {
            // Go back one step
            let first = if first > 1 {
                first - 1
            } else {
                0
            };
            for i in first..(chars.len()-1) {
                // Handle numbers (that's easy)
                let current = chars[i];
                let next = chars[i+1];

                match current {
                    '0'...'9' => if i == 0 {
                        is_number_series = true;
                    } else if !chars[i-1].is_alphabetic() {
                        is_number_series = true;
                    },
                    c if c.is_whitespace() => {
                        if is_number_series && (next.is_digit(10) || self.char_is_symbol(&chars, i+1)) {
                            // Next char is a number or symbol such as $, and previous was number
                            chars[i] = NB_CHAR_NARROW;
                        }
                    },
                    _ => { is_number_series = false; }
                }
            }
        }

        // Handle the rest
        if let Some(first) = first {
            // Go back one step
            let first = if first > 1 {
                first - 1
            } else {
                0
            };
            for i in first..(chars.len()-1) {
                let current = chars[i];
                let next = chars[i+1];
                if is_whitespace(current) {
                    match next {
                        // handle narrow nb space before char
                        '?' | '!' | ';' => chars[i] = NB_CHAR_NARROW,
                        ':' => chars[i] = NB_CHAR,
                        '»' => if current == ' ' {
                            // Assumne that if it isn't a normal space it was used here for good reason, don't replace it
                            if found_opening_quote {
                                // not the end of a dialogue
                                chars[i] = NB_CHAR;
                            } else {
                                chars[i] = NB_CHAR;
                            }
                        },
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
                                            NB_CHAR_EM
                                        } else {
                                            if chars[i-1] == NB_CHAR {
                                                // non breaking space before, so probably should have a breakable one after
                                                ' '
                                            } else {
                                                if let Some(closing) = self.find_closing_dash(&chars, i+1) {
                                                    chars[closing] = NB_CHAR;
                                                }
                                                NB_CHAR
                                            }
                                        }
                                    },
                                    '«' => {
                                        found_opening_quote = true;
                                        if i <= 1 {
                                            NB_CHAR
                                        } else {
                                            let j = find_next(&chars, '»', i);
                                            if let Some(j) = j {
                                            if chars[j-1].is_whitespace() {
                                                if j >= chars.len() - 1 {
                                                    // » is at the end, assume it is a dialogue
                                                    chars[j-1] = NB_CHAR;
                                                    NB_CHAR
                                                } else {
                                                    if j - i > self.threshold_quote {
                                                        // It's a quote, so use large space?
                                                        chars[j-1] = NB_CHAR;
                                                        NB_CHAR
                                                    } else {
                                                        // Not long enough to be a quote, use narrow nb char
                                                        chars[j-1] = NB_CHAR_NARROW;
                                                        NB_CHAR_NARROW
                                                    }
                                                }
                                            } else {
                                                // wtf formatting?
                                                NB_CHAR
                                            }
                                        } else {
                                                // No ending quote found, assume is a dialogue
                                                NB_CHAR
                                            }
                                        }
                                    }, // TODO: better heuristic: use narrow nb_char if not at front???
                                    _ => unreachable!(),
                                };
                                chars[i+1] = replacing_char;
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
        Cow::Owned(chars.into_iter().collect())
    }

    /// Return true if the character is a symbol that is used after number and should have a nb_char before
    fn char_is_symbol(&self, v: &[char], i: usize) -> bool {
        let is_next_letter = if i < v.len() - 1 {
            v[i+1].is_alphabetic()
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
                        // if all uppercase and less than THRESHOLD, assume it's a currency or a unit
                        word.iter().all(|c| c.is_uppercase())
                    }
                },
                c if c.is_alphabetic() => {
                    let word = get_next_word(v, i);
                    // if two letters, assume it is a unit
                    word.len() <= self.threshold_unit
                },
                _ => false
            }
        } else {
            match v[i] {
                c if (!c.is_alphabetic() && !c.is_whitespace()) => true, // special symbol
                c if c.is_uppercase() => true, //single uppercase letter
                _ => false,
            }
        }
    }

    // Return true(some) if a closing dash was found before what looks like the end of a sentence, None else
    fn find_closing_dash(&self, v: &[char], n: usize) -> Option<usize> {
        let mut word = String::new();
        for j in n..v.len() {
            match v[j] {
                '!' | '?' => if is_next_char_uppercase(v, j+1) {
                    return None;
                },
                '-' | '–' | '—' => if v[j-1].is_whitespace() {
                    return Some(j-1);
                },
                '.' => if !is_next_char_uppercase(v, j+1) {
                    continue;
                } else {
                    if let Some(c) = word.chars().next() {
                        if !c.is_uppercase() {
                            return None;
                        } else {
                            if word.len() > self.threshold_real_word {
                                return None;
                            }
                        }
                    } 
                },
                c if c.is_whitespace() => word = String::new(),
                c => word.push(c),
            }
        }
        return None;
    }
}

fn is_trouble(c: char) -> bool {
    match c {
        '?'|'!'|';'|':'|'»'|'«'|'—'|'–' => true,
        _ => false
    }
}



// Find first char `c` in slice `v` after index `n`
fn find_next(v: &[char], c: char, n: usize) -> Option<usize> {
    for i in n..v.len() {
        if v[i] == c  {
            return Some(i);
        } 
    }
    None
}

// Return true if next non whitespace char in `v` after index `n` is uppercase
fn is_next_char_uppercase(v: &[char], n: usize)-> bool {
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
            end = i-1;
            break;
        }
    }

    &v[beginning..end]
}


#[cfg(test)]
use escape::escape_nb_spaces_tex;

#[test]
fn french() {
    let s = "  «  Comment allez-vous ? » demanda-t-elle à son   interlocutrice  qui lui répondit  : « Mais très bien ma chère  !  »";
    let res = FrenchFormatter::new().format(s);
    assert_eq!(&res, " « Comment allez-vous ? » demanda-t-elle à son interlocutrice qui lui répondit : « Mais très bien ma chère ! »");
}

#[test]
fn french_dashes_1() {
    let s = "Il faudrait gérer ces tirets – sans ça certains textes rendent mal – un jour ou l'autre";
    let res = escape_nb_spaces_tex(FrenchFormatter::new().format(s));
    assert_eq!(&res, "Il faudrait gérer ces tirets –~sans ça certains textes rendent mal~– un jour ou l'autre");
}

#[test]
fn french_dashes_2() {
    let s = "Il faudrait gérer ces tirets – sans ça certains textes rendent mal. Mais ce n'est pas si simple – si ?";
    let res = escape_nb_spaces_tex(FrenchFormatter::new().format(s));
    assert_eq!(&res, "Il faudrait gérer ces tirets –~sans ça certains textes rendent mal. Mais ce n'est pas si simple –~si~?");
}

#[test]
fn french_numbers() {
    let french = FrenchFormatter::new();
    
    let s = Cow::Borrowed("10 000");
    let res = escape_nb_spaces_tex(french.format(s));
    assert_eq!(&res, "10~000");

    let s = Cow::Borrowed("10 000 €");
    let res = escape_nb_spaces_tex(french.format(s));
    assert_eq!(&res, "10~000~€");
    
    let s = Cow::Borrowed("10 000 euros");
    let res = escape_nb_spaces_tex(french.format(s));
    assert_eq!(&res, "10~000 euros");

    let s = Cow::Borrowed("10 000 EUR");
    let res = escape_nb_spaces_tex(french.format(s));
    assert_eq!(&res, "10~000~EUR");

    let s = Cow::Borrowed("50 km");
    let res = escape_nb_spaces_tex(french.format(s));
    assert_eq!(&res, "50~km");

    let s = Cow::Borrowed("50 %");
    let res = escape_nb_spaces_tex(french.format(s));
    assert_eq!(&res, "50~%");

    let s = Cow::Borrowed("20 °C");
    let res = escape_nb_spaces_tex(french.format(s));
    assert_eq!(&res, "20~°C");

    let s = Cow::Borrowed("20 F");
    let res = escape_nb_spaces_tex(french.format(s));
    assert_eq!(&res, "20~F");

    let s = Cow::Borrowed("20 BALLES");
    let res = escape_nb_spaces_tex(french.format(s));
    assert_eq!(&res, "20 BALLES");
}

