// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! This module provides function to "clean" a text typographically.
//!
//! # Example
//!
//! ```
//! use crowbook_text_processing::clean;
//! let input = "Some  'text'  whose formatting  could be enhanced...";
//! let output = clean::quotes(clean::ellipsis(clean::whitespaces(input)));
//! assert_eq!(&output, "Some ‘text’ whose formatting could be enhanced…");
//! ```

use common::is_whitespace;

use regex::Regex;
use std::borrow::Cow;

/// Removes unnecessary whitespaces from a String.
///
/// # Example
///
/// ```
/// use crowbook_text_processing::clean;
/// let s = clean::whitespaces("  A  string   with   more   whitespaces  than  needed   ");
/// assert_eq!(&s, " A string with more whitespaces than needed ");
/// ```
pub fn whitespaces<'a, S: Into<Cow<'a, str>>>(input: S) -> Cow<'a, str> {
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

/// Class of a character
#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy)]
enum CharClass {
    Whitespace = 0,
    Punctuation,
    Alphanumeric,
}

/// Get class of a character
fn char_class(c: char) -> CharClass {
    if c.is_alphanumeric() {
        CharClass::Alphanumeric
    } else if c.is_whitespace() {
        CharClass::Whitespace
    } else {
        CharClass::Punctuation
    }
}

/// Replace ellipsis (...) with the appropriate unicode character
///
/// # Example
///
/// ```
/// use crowbook_text_processing::clean;
/// let s = clean::ellipsis("foo...");
/// assert_eq!(&s, "foo…");
/// let s = clean::ellipsis("foo. . . ");
/// assert_eq!(&s, "foo.\u{a0}.\u{a0}. "); // non breaking spaces
/// ```
pub fn ellipsis<'a, S: Into<Cow<'a, str>>>(input: S) -> Cow<'a, str> {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"\.\.\.|\. \. \. ").unwrap();
        static ref UNICODE_ELLIPSIS: &'static [u8] = "…".as_bytes();
        static ref NB_ELLIPSIS: &'static [u8] = ". . . ".as_bytes();
        static ref FULL_NB_ELLIPSIS: &'static [u8] = ". . . ".as_bytes();
    }
    let input = input.into();
    let first = REGEX.find(&input);
    if let Some((first, _)) = first {
        let mut output: Vec<u8> = Vec::with_capacity(input.len());
        output.extend_from_slice(input[0..first].as_bytes());
        let rest = input[first..].bytes().collect::<Vec<_>>();
        let len = rest.len();
        let mut i = 0;
        while i < len {
            if i + 3 <= len && &rest[i..(i + 3)] == &[b'.', b'.', b'.'] {
                output.extend_from_slice(*UNICODE_ELLIPSIS);
                i += 3;
            } else if i + 6 <= len && &rest[i..(i + 6)] == &[b'.', b' ', b'.', b' ', b'.', b' '] {
                if i + 6 == len || rest[i + 6] != b'.' {
                    output.extend_from_slice(*NB_ELLIPSIS);
                } else {
                    output.extend_from_slice(*FULL_NB_ELLIPSIS);
                }
                i += 6;
            } else {
                output.push(rest[i]);
                i += 1;
            }
        }
        Cow::Owned(String::from_utf8(output).unwrap())
    } else {
        input
    }
}


/// Replace straight quotes with more typographic variants
///
/// While it should work pretty well for double quotes (`"`), the rules for single
/// quote (`'`) are more ambiguous, as it can be a quote or an apostrophe and it's not
/// that easy (and, in some circumstances, impossible without understanding the meaning
/// of the text) to get right.
///
/// # Example
///
/// ```
/// use crowbook_text_processing::clean;
/// let s = clean::quotes("\"foo\"");
/// assert_eq!(&s, "“foo”");
/// let s = clean::quotes("'foo'");
/// assert_eq!(&s, "‘foo’");
/// ```
pub fn quotes<'a, S: Into<Cow<'a, str>>>(input: S) -> Cow<'a, str> {
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
        let mut chars = input[first..].chars().collect::<Vec<_>>();
        let mut closing_quote = None;
        let mut opened_doubles = 0;
        for i in 0..chars.len() {
            let c = chars[i];
            let has_opened_quote = if let Some(n) = closing_quote {
                i <= n
            } else {
                false
            };
            match c {
                '"' => {
                    let prev = if i > 0 {
                        char_class(chars[i - 1])
                    } else {
                        CharClass::Whitespace
                    };
                    let next = if i < chars.len() - 1 {
                        char_class(chars[i + 1])
                    } else {
                        CharClass::Whitespace
                    };

                    if prev < next {
                        opened_doubles += 1;
                        new_s.push('“');
                    } else if opened_doubles > 0 {
                        opened_doubles -= 1;
                        new_s.push('”');
                    } else {
                        new_s.push('"');
                    }
                }
                '\'' => {
                    let prev = if i > 0 {
                        char_class(chars[i - 1])
                    } else {
                        CharClass::Whitespace
                    };
                    let next = if i < chars.len() - 1 {
                        char_class(chars[i + 1])
                    } else {
                        CharClass::Whitespace
                    };

                    let replacement = match (prev, next) {
                        // Elision or possessive
                        (CharClass::Alphanumeric, CharClass::Alphanumeric)
//                            | (CharClass::Punctuation, CharClass::Alphanumeric)
                            => '’',

                        // Beginning of word, it's opening (not always though)
                        (x, y) if x < y
                            => {
                                let mut is_next_closing = false;
                                for j in (i + 1)..chars.len() {
                                    if chars[j] == '\'' {
                                        if chars[j-1].is_whitespace() {
                                            continue;
                                        } else if j >= chars.len() - 1
                                            || char_class(chars[j+1]) != CharClass::Alphanumeric {
                                                is_next_closing = true;
                                                closing_quote = Some(j);
                                                chars[j] = '’';
                                                break;
                                            }
                                    }
                                }
                                if is_next_closing && !has_opened_quote {
                                    '‘'
                                } else {
                                    '’'
                                }
                            }

                        // Apostrophe at end of word, it's closing
                        (x, y) if x > y
                            => {
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
                }
                _ => new_s.push(c),
            }
        }
        Cow::Owned(new_s)
    } else {
        input
    }
}


/// Replace double dashes (`--`) and triple dashes (`---`) to en dash and em dash, respectively.
///
/// This function can be useful when writing literaty texts, but should be used with caution
/// as double and triple dashes can have special meanings.
///
/// # Example
///
/// ```
/// use crowbook_text_processing::clean;
/// let s = clean::dashes("--- Hi, he said -- unexpectedly");
/// assert_eq!(&s, "— Hi, he said – unexpectedly");
/// ```
pub fn dashes<'a, S: Into<Cow<'a, str>>>(input: S) -> Cow<'a, str> {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"\x2D\x2D").unwrap();
        static ref EN_SPACE: &'static [u8] = "–".as_bytes();
        static ref EM_SPACE: &'static [u8] = "—".as_bytes();
    }
    let input = input.into();
    let first = REGEX.find(&input);
    if let Some((first, _)) = first {
        let mut output: Vec<u8> = Vec::with_capacity(input.len());
        output.extend_from_slice(input[0..first].as_bytes());
        let rest = input[first..].bytes().collect::<Vec<_>>();
        let len = rest.len();
        let mut i = 0;
        while i < len {
            if i + 2 <= len && &rest[i..(i + 2)] == &[b'-', b'-'] {
                if i + 2 < len && rest[i + 2] == b'-' {
                    output.extend_from_slice(*EM_SPACE);
                    i += 3;
                } else {
                    output.extend_from_slice(*EN_SPACE);
                    i += 2;
                }
            } else {
                output.push(rest[i]);
                i += 1;
            }
        }
        Cow::Owned(String::from_utf8(output).unwrap())
    } else {
        input
    }
}

/// Replaces `<<` with `«` and `>>` with `»`.
///
/// This can be useful if you need those characters (e.g. for french texts) but
/// don't have an easy access to them on your computer but, as the `dashes` function,
/// it should be used with caution, as `<<` and `>>` can also be used for other things
/// (typically to mean "very inferior to" or "very superior to").
///
/// # Example
///
/// use crowbook_text_processing::clean;
/// let s = clean::guillemets("<< Foo >>");
/// assert_eq!(&s, "« Foo »");
/// ```
pub fn guillemets<'a, S: Into<Cow<'a, str>>>(input: S) -> Cow<'a, str> {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"<<|>>").unwrap();
        static ref OPENING_GUILLEMET: &'static [u8] = "«".as_bytes();
        static ref CLOSING_GUILLEMET: &'static [u8] = "»".as_bytes();
    }
    let input = input.into();
    let first = REGEX.find(&input);
    if let Some((first, _)) = first {
        let mut output: Vec<u8> = Vec::with_capacity(input.len());
        output.extend_from_slice(input[0..first].as_bytes());
        let rest = input[first..].bytes().collect::<Vec<_>>();
        let len = rest.len();
        let mut i = 0;
        while i < len {
            if i + 2 <= len && &rest[i..(i + 2)] == &[b'<', b'<'] {
                output.extend_from_slice(*OPENING_GUILLEMET);
                i += 2;
            } else if i+2 <= len && &rest[i..(i + 2)] == &[b'>', b'>'] {
                output.extend_from_slice(*CLOSING_GUILLEMET);
                i += 2;
            } else {
                output.push(rest[i]);
                i += 1;
            }
        }
        Cow::Owned(String::from_utf8(output).unwrap())
    } else {
        input
    }
}



#[test]
fn whitespaces_1() {
    let s = "   Remove    supplementary   spaces    but    don't    trim     either   ";
    let res = whitespaces(s);
    assert_eq!(&res, " Remove supplementary spaces but don't trim either ");
}

#[test]
fn quotes_1() {
    let s = "Some string without ' typographic ' quotes";
    let res = quotes(s);
    assert_eq!(&res, s);
}

#[test]
fn quotes_2() {
    let s = quotes("\"foo\"");
    assert_eq!(&s, "“foo”");
    let s = quotes("'foo'");
    assert_eq!(&s, "‘foo’");
}

#[test]
fn quotes_3() {
    let s = quotes("\'mam, how are you?");
    assert_eq!(&s, "’mam, how are you?");
}

#[test]
fn quotes_4() {
    let s = quotes("some char: 'c', '4', '&'");
    assert_eq!(&s, "some char: ‘c’, ‘4’, ‘&’");
}

#[test]
fn quotes_5() {
    let s = quotes("It's a good day to say 'hi'");
    assert_eq!(&s, "It’s a good day to say ‘hi’");
}

#[test]
fn quotes_6() {
    let s = quotes("The '60s were nice, weren't they?");
    assert_eq!(&s, "The ’60s were nice, weren’t they?");
}

#[test]
fn quotes_7() {
    let s = quotes("Plurals' possessive");
    assert_eq!(&s, "Plurals’ possessive");
}

#[test]
fn quotes_8() {
    let s = quotes("\"I like 'That '70s show'\", she said");
    assert_eq!(&s, "“I like ‘That ’70s show’”, she said");
}


#[test]
fn quotes_9() {
    let s = quotes("some char: '!', '?', ','");
    assert_eq!(&s, "some char: ‘!’, ‘?’, ‘,’");
}

#[test]
fn quotes_10() {
    let s = quotes("\"'Let's try \"nested\" quotes,' he said.\"");
    assert_eq!(&s, "“‘Let’s try “nested” quotes,’ he said.”");
}

#[test]
fn quotes_11() {
    let s = quotes("Enhanced \"quotes\"'s heuristics");
    assert_eq!(&s, "Enhanced “quotes”’s heuristics");
}

#[test]
fn quotes_12() {
    let s = quotes("A double quote--\"within\" dashes--would be nice.");
    assert_eq!(&s, "A double quotes--“within” dashes-- would be nice.");
}


#[test]
fn ellipsis_0() {
    let s = ellipsis("Foo...");
    assert_eq!(&s, "Foo…");
}

#[test]
fn ellipsis_1() {
    let s = ellipsis("Foo... Bar");
    assert_eq!(&s, "Foo… Bar");
}

#[test]
fn ellipsis_2() {
    let s = ellipsis("foo....");
    assert_eq!(&s, "foo….");
}

#[test]
fn ellipsis_3() {
    let s = ellipsis("foo. . . ");
    assert_eq!(&s, "foo. . . ");
}

#[test]
fn ellipsis_4() {
    let s = ellipsis("foo. . . .");
    assert_eq!(&s, "foo. . . .");
}

#[test]
fn ellipsis_5() {
    let s = ellipsis("foo..");
    assert_eq!(&s, "foo..");
}

#[test]
fn dashes_0() {
    let s = dashes("foo - bar");
    assert_eq!(&s, "foo - bar");
}

#[test]
fn dashes_1() {
    let s = dashes("foo -- bar");
    assert_eq!(&s, "foo – bar");
}

#[test]
fn dashes_2() {
    let s = dashes("foo --- bar");
    assert_eq!(&s, "foo — bar");
}

#[test]
fn dashes_3() {
    let s = dashes("foo --- bar--");
    assert_eq!(&s, "foo — bar–");
}
    
#[test]
fn guillemets_1() {
    let s = guillemets("<< Foo >>");
    assert_eq!(&s, "« Foo »");
}

#[test]
fn guillemets_2() {
    let s = guillemets("<< Foo");
    assert_eq!(&s, "« Foo");
}

#[test]
fn guillemets_3() {
    let s = guillemets("Foo >>");
    assert_eq!(&s, "Foo »");
}

#[test]
fn guillemets_4() {
    let s = guillemets("<< Foo < Bar >>");
    assert_eq!(&s, "« Foo < Bar »");
}
