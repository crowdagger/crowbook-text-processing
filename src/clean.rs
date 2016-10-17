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
        for i in 0..chars.len() {
            let c = chars[i];
            match c {
                '"' => {
                    if i > 0 && chars[i - 1].is_alphabetic() {
                        new_s.push('”');
                    } else if i < chars.len() - 1 && chars[i + 1].is_alphabetic() {
                        new_s.push('“');
                    } else {
                        new_s.push('"');
                    }
                },
                '\'' => {
                    if i > 0 && chars[i - 1].is_alphabetic() {
                        new_s.push('’');
                    } else if i < chars.len() - 1 && chars[i + 1].is_alphabetic() {
                        new_s.push('‘');
                    } else {
                        new_s.push('\'');
                    }
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
