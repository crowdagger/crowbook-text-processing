// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.  

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

/// Custom function because we don't really want to touch \t or \n
///
/// This function detects spaces and non breking spaces
fn is_whitespace(c: char) -> bool {
    c == ' ' || c == ' ' || c == ' '
}


#[test]
fn remove_whitespaces_1() {
    let s = "   Remove    supplementary   spaces    but    don't    trim     either   ";
    let res = remove_whitespaces(s);
    assert_eq!(&res, " Remove supplementary spaces but don't trim either ");
}
