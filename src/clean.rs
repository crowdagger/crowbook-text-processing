// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

use common::is_whitespace;

use regex::Regex;
use std::borrow::Cow;

/// Removes unnecessary whitespaces from a String.
///
/// # Example
///
/// ```
/// use crowbook_text_processing::clean::remove_whitespaces;
/// let s = remove_whitespaces("  A  string   with   more   whitespaces  than  needed   ");
/// assert_eq!(&s, " A string with more whitespaces than needed ");
/// ```
pub fn remove_whitespaces<'a, S: Into<Cow<'a, str>>>(input: S) -> Cow<'a, str> {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"[  \x{202F}\x{2002}]{2,}?").unwrap();
    }
    let input = input.into();
    let first = REGEX.find(&input);
    if let Some((first, _)) = first {
        let mut new_s = String::with_capacity(input.len());
        new_s.push_str(&input[0..first]);
        let mut previous_space = false;
        for c in input[first..].chars() {
            if is_whitespace(c) {
                if previous_space {
                    // previous char already a space, don't copy it
                } else {
                    new_s.push(c);
                    previous_space = true;
                }
            } else {
                previous_space = false;
                new_s.push(c);
            }
        }
        Cow::Owned(new_s)
    } else {
        input
    }
}

/// Replace quotes with more typographic variants
///
/// While it should work pretty well for double quotes (`"`), the rules for single
/// quote (`'`) are more ambiguous, as it can be a quote, or an apostrophe and it's not
/// that easy to get right.
///
/// # Example
///
/// ```
/// use crowbook_text_processing::typographic_quotes;
/// let s = typographic_quotes("\"foo\"");
/// assert_eq!(&s, "“foo”");
/// let s = typographic_quotes("'foo'");
/// assert_eq!(&s, "‘foo’");
/// ```
pub fn typographic_quotes<'a, S: Into<Cow<'a, str>>>(input: S) -> Cow<'a, str> {
    /// Custom whitespace-detection function, including `,`, `;`, `.`, `!`, `?`, and `:`
    fn is_whitespace(c: char) -> bool {
        match c {
            ',' | '.' | ';' | '!' | '?' | ':' => true,
            _ => c.is_whitespace()
        }
    }
    
    lazy_static! {
        static ref REGEX: Regex = Regex::new("[\"\']").unwrap();
    }
    let input = input.into();
    let first = REGEX.find(&input);
    if let Some((mut first, _)) = first {
        let mut new_s = String::with_capacity(input.len());
        if first > 0 {
            // Move one step backward since we might need to know if previous char was
            // a letter or not
            first -= 1;
        }
        new_s.push_str(&input[0..first]);
        let chars = input[first..].chars().collect::<Vec<_>>();
        let mut has_opened_quote = false;
        for i in 0..chars.len() {
            let c = chars[i];
            match c {
                '"' => {
                    if i > 0 && !is_whitespace(chars[i - 1]) {
                        new_s.push('”');
                    } else if i < chars.len() - 1 && !is_whitespace(chars[i + 1]) {
                        new_s.push('“');
                    } else {
                        new_s.push('"');
                    }
                },
                '\'' => {
                    let prev = if i > 0 {
                        Some(!is_whitespace(chars[i - 1]))
                    } else {
                        None
                    };
                    let next = if i < chars.len() - 1 {
                        Some(!is_whitespace(chars[i + 1]))
                    } else {
                        None
                    };
                    let replacement = match (prev, next) {
                        // Elision, it's closing
                        (Some(true), Some(true)) => '’',

                        // Beginning of word, it's opening (not always though)
                        (Some(false), Some(true))
                            | (None, Some(true))
                            => {
                                let mut is_next_closing = false;
                                for j in (i + 1)..chars.len() {
                                    if chars[j] == '\'' {
                                        println!("match at {}", j);
                                        if chars[j-1].is_whitespace() {
                                            println!("prev is whitespace, not closing quote");
                                            continue;
                                        } else {
                                            if j >= chars.len() - 1
                                                || is_whitespace(chars[j+1])
                                                || chars[j+1] == '"' {
                                                    is_next_closing = true;
                                                    break;
                                                }
                                            else {
                                                println!("j: {}, len: {}", j, chars.len());
                                            }
                                        }
                                    }
                                }
                                println!("is_next_closing: {}", is_next_closing);
                                if is_next_closing && !has_opened_quote {
                                    has_opened_quote = true;
                                    '‘'
                                } else {
                                    '’'
                                }
                            }

                        // Apostrophe at end of word, it's closing
                        (Some(true), Some(false))
                            | (Some(true), None)
                            => {
                                has_opened_quote = false;
                                '’'
                            }, 
                        
                        _ => '\'',
                    };
                    new_s.push(replacement);
                    // if i > 0 && !chars[i - 1].is_whitespace() {
                    //     new_s.push('’');
                    // } else if i < chars.len() - 1 && !chars[i + 1].is_whitespace() {
                    //     new_s.push('‘');
                    // } else {
                    //     new_s.push('\'');
                    // }
                },
                _ => new_s.push(c)
            }
        }
        Cow::Owned(new_s)
    } else {
        input
    }
}


#[test]
fn remove_whitespaces_1() {
    let s = "   Remove    supplementary   spaces    but    don't    trim     either   ";
    let res = remove_whitespaces(s);
    assert_eq!(&res, " Remove supplementary spaces but don't trim either ");
}

#[test]
fn typographic_quotes_1() {
    let s = "Some string without ' typographic ' quotes";
    let res = typographic_quotes(s);
    assert_eq!(&res, s);
}

#[test]
fn typographic_quotes_2() {
    let s = typographic_quotes("\"foo\"");
    assert_eq!(&s, "“foo”");
    let s = typographic_quotes("'foo'");
    assert_eq!(&s, "‘foo’");
}

#[test]
fn typographic_quotes_3() {
    let s = typographic_quotes("\'mam, how are you?");
    assert_eq!(&s, "’mam, how are you?");
}

#[test]
fn typographic_quotes_4() {
    let s = typographic_quotes("some char: 'c', '4', '&'");
    assert_eq!(&s, "some char: ‘c’, ‘4’, ‘&’");
}

#[test]
fn typographic_quotes_5() {
    let s = typographic_quotes("It's a good day to say 'hi'");
    assert_eq!(&s, "It’s a good day to say ‘hi’");
}

#[test]
fn typographic_quotes_6() {
    let s = typographic_quotes("The '60s were nice, weren't they?");
    assert_eq!(&s, "The ’60s were nice, weren’t they?");
}

#[test]
fn typographic_quotes_7() {
    let s = typographic_quotes("Plurals' possessive");
    assert_eq!(&s, "Plurals’ possessive");
}

#[test]
fn typographic_quotes_8() {
    let s = typographic_quotes("\"I like 'That '70s show'\", she said");
    assert_eq!(&s, "“I like ‘That ’70s show’”, she said");
}
