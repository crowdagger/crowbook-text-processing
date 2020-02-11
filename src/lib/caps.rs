// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Tihs module provides function to automatically transform uppercase words
//! to small caps. Since it is dependent on the output format, functions differ for
//! LaTeX or HTML output.


use regex::{Regex, Captures};
use std::borrow::Cow;

/// Transform uppercase words to small caps for LaTeX output.
///
/// Note that it will put all the text in small capitals in lowercase: sometimes,
/// it would be best to do otherwise (e.g. put the first letter in uppercase or whatever).
///
/// It only applies to words (or abbreviations: you can use dots to separate each letter) that
/// have strictly more than one letter that are in uppercase in the input.
/// 
///
/// # Example
///
/// ```
/// use crowbook_text_processing::caps;
///
/// let s = caps::latex("Some ACRONYM or SCREAMING or whatever.");
/// assert_eq!(&s, "Some \\textsc{acronym} or \\textsc{screaming} or whatever.");
/// ```
pub fn latex<'a, S: Into<Cow<'a, str>>>(input: S) -> Cow<'a, str> {
    let mut res = input.into();

    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"\b\p{Lu}{2,}\b").unwrap();
        static ref REGEX_DOTS: Regex = Regex::new(r"\b((\p{Lu}\.){1,}\p{Lu})\b").unwrap();
    }

    for cap in REGEX.captures_iter(&res) {
        println!("capture: {:?}", cap);
    }

    if REGEX.is_match(&res) {
        let tmp = REGEX.replace_all(&res, |caps: &Captures| {
            format!("\\textsc{{{}}}",
                    caps[0].to_lowercase())
        });
        res = Cow::Owned(tmp.into_owned())
    }
    if REGEX_DOTS.is_match(&res) {
        let tmp = REGEX_DOTS.replace_all(&res, |caps: &Captures| {
            format!("\\textsc{{{}}}",
                    caps[0].to_lowercase())
        });
        res = Cow::Owned(tmp.into_owned())
    }
    res
}


#[test]
fn latex_1() {
    use crate::caps;

   
    let s = caps::latex("Some ACRONYM or SCREAMING or whatever.");
    assert_eq!(&s, "Some \\textsc{acronym} or \\textsc{screaming} or whatever.");

    let s = caps::latex("Nothing to change.");
    assert_eq!(&s, "Nothing to change.");

    let s = caps::latex("A single letter is not capitalized. TWO or more are.");
    assert_eq!(&s, "A single letter is not capitalized. \\textsc{two} or more are.");

    let s = caps::latex("BEGIN with caps");
    assert_eq!(&s, "\\textsc{begin} with caps");

    let s = caps::latex("BEGINning with caps");
    assert_eq!(&s, "BEGINning with caps");

    let s = caps::latex("Ending with CAPS");
    assert_eq!(&s, "Ending with \\textsc{caps}");

    let s = caps::latex("Some A.W.D (Acronym With Dots)");
    assert_eq!(&s, "Some \\textsc{a.w.d} (Acronym With Dots)");

    let s = caps::latex("Sentence ennding with A.W.D.");
    assert_eq!(&s, "Sentence ennding with \\textsc{a.w.d}.");
}
