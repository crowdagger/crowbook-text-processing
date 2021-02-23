// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Some functions to escape character for display in HTML or LaTeX.
//!
//! The two most useful ones are `tex` and `html`.
//!
//! # Example
//!
//! ```
//! use crowbook_text_processing::escape;
//! let input = "<foo> & <bar>";
//! let output = escape::html(input);
//! assert_eq!(&output, "&lt;foo&gt; &amp; &lt;bar&gt;");
//!
//! let input = "#2: 20%";
//! let output = escape::tex(input);
//! assert_eq!(&output, r"\#2: 20\%");
//! ```


use std::borrow::Cow;

use regex::Regex;
use regex::Captures;

use crate::common::{NB_CHAR, NB_CHAR_NARROW, NB_CHAR_EM};


/// Escape narrow non-breaking spaces for HTML.
///
/// This is unfortunately sometimes necessary as some fonts/renderers don't support the
/// narrow non breaking space character.
///
/// This function works by declaring a span with class "nnbsp" containing
/// the previous and next word, and replacing narrow non breaking space with the non-breaking
/// space character.
///
/// Thus, in order to display correctly, you will need to add some style to this span, e.g.:
///
/// ```css
/// .nnbsp {
///    word-spacing: -0.13em;
///  }
/// ```
pub fn nb_spaces_html<'a, S: Into<Cow<'a, str>>>(input: S) -> Cow<'a, str> {
    let input = input.into();
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"\S*\x{202F}[\S\x{202F}]*").unwrap();
        static ref REGEX_LOCAL: Regex = Regex::new(r"\x{202F}").unwrap();
    }
    if REGEX.is_match(&input) {
        let res = REGEX.replace_all(&input, |caps: &Captures| {
            format!("<span class = \"nnbsp\">{}</span>",
                    REGEX_LOCAL.replace_all(&caps[0], "&#160;"))
        });
        Cow::Owned(res.into_owned())
    } else {
        input
    }
}

/// Old name of nb_spaces html
#[deprecated(
    since="1.0.0",
    note="Renamed nb_spaces_html"
)]
pub fn nnbsp<'a, S: Into<Cow<'a, str>>>(input: S) -> Cow<'a, str> {
    nb_spaces_html(input)
}




/// Escape non breaking spaces for LaTeX, replacing them with the appropriate TeX code.
/// This ensures it works correctly with some LaTeX versions (and it makes
/// the non-breaking spaces shenanigans more visible with most editors)
///
/// # Achtung
///
/// Since this function adds some LaTeX codes that use backslashes, it will cause issues
/// if you then try to escape those characters. So if you must escape the text for LaTeX,
/// this function should always be called **after** `escape::tex`.
///
/// # Example
///
/// ```
/// use crowbook_text_processing::escape;
/// let s = escape::nb_spaces_tex("Des espaces insécables ? Ça alors !");
/// assert_eq!(&s, "Des espaces insécables\\,? Ça alors\\,!");
/// ```
pub fn nb_spaces_tex<'a, S: Into<Cow<'a, str>>>(input: S) -> Cow<'a, str> {
    let input = input.into();
    if let Some(first) = input.chars().position(|c| match c {
        NB_CHAR | NB_CHAR_NARROW | NB_CHAR_EM => true,
        _ => false,
    }) {
        let mut chars = input.chars().collect::<Vec<_>>();
        let rest = chars.split_off(first);
        let mut output = chars.into_iter().collect::<String>();
        for c in rest {
            match c {
                NB_CHAR_NARROW => output.push_str("\\,"),
                NB_CHAR_EM => output.push_str("\\enspace "),
                NB_CHAR => output.push('~'),
                _ => output.push(c),
            }
        }
        Cow::Owned(output)
    } else {
        input.into()
    }
}


/// Escape characters for HTML output, replacing  `<`, `>`, and `&` with appropriate
/// HTML entities.
///
/// **Warning**: this function was written for escaping text in a markdown
/// text processor that is designed to run on a local machine, where the content
/// can actually be trusted. It should *not* be used for untrusted content.
///
/// # Example
///
/// ```
/// use crowbook_text_processing::escape;
/// let s = escape::html("<foo> & <bar>");
/// assert_eq!(&s, "&lt;foo&gt; &amp; &lt;bar&gt;");
/// ```
pub fn html<'a, S: Into<Cow<'a, str>>>(input: S) -> Cow<'a, str> {
    lazy_static! {
        static ref REGEX: Regex = Regex::new("[<>&]").unwrap();
    }
    let input = input.into();
    let first = REGEX.find(&input)
        .map(|mat| mat.start());
    if let Some(first) = first {
        let len = input.len();
        let mut output = Vec::with_capacity(len + len / 2);
        output.extend_from_slice(input[0..first].as_bytes());
        let rest = input[first..].bytes();
        for c in rest {
            match c {
                b'<' => output.extend_from_slice(b"&lt;"),
                b'>' => output.extend_from_slice(b"&gt;"),
                b'&' => output.extend_from_slice(b"&amp;"),
                _ => output.push(c),
            }
        }
        Cow::Owned(String::from_utf8(output).unwrap())
    } else {
        input
    }
}

/// Very naively escape quotes
///
/// Simply replace `"` by `'`
pub fn quotes<'a, S: Into<Cow<'a, str>>>(input: S) -> Cow<'a, str> {
    let input = input.into();
    if input.contains('"') {
        let mut output = String::with_capacity(input.len());
        for c in input.chars() {
            match c {
                '"' => output.push('\''),
                _ => output.push(c),
            }
        }
        Cow::Owned(output)
    } else {
        input
    }
}


/// Escape characters for LaTeX
///
/// # Example
///
/// ```
/// use crowbook_text_processing::escape;
/// let s = escape::tex("command --foo # calls command with option foo");
/// assert_eq!(&s, r"command -{}-foo \# calls command with option foo");
/// ```
pub fn tex<'a, S: Into<Cow<'a, str>>>(input: S) -> Cow<'a, str> {
    let input = input.into();
    const REGEX_LITERAL: &'static str = r"[!<>&%$#_\x7E\x2D\{\}\[\]\^\\]";
    lazy_static! {
       static ref REGEX: Regex = Regex::new(REGEX_LITERAL).unwrap();
    }

    let first = REGEX.find(&input)
        .map(|mat| mat.start());
    if let Some(first) = first {
        let len = input.len();
        let mut output = Vec::with_capacity(len + len / 2);
        output.extend_from_slice(input[0..first].as_bytes());
        let mut bytes: Vec<_> = input[first..].bytes().collect();
        bytes.push(b' '); // add a dummy char for call to .windows()
        // for &[c, next] in chars.windows(2) { // still experimental, uncomment when stable
        for win in bytes.windows(2) {
            let c = win[0];
            let next = win[1];
            match c {
                b'-' => {
                    if next == b'-' {
                        // if next char is also a -, to avoid tex ligatures
                        output.extend_from_slice(br"-{}");
                    } else {
                        output.push(c);
                    }
                }
                b'&' => output.extend_from_slice(br"\&"),
                b'%' => output.extend_from_slice(br"\%"),
                b'$' => output.extend_from_slice(br"\$"),
                b'#' => output.extend_from_slice(br"\#"),
                b'_' => output.extend_from_slice(br"\_"),
                b'{' => output.extend_from_slice(br"\{"),
                b'}' => output.extend_from_slice(br"\}"),
                b'[' => output.extend_from_slice(br"{[}"),
                b']' => output.extend_from_slice(br"{]}"),
                b'~' => output.extend_from_slice(br"\textasciitilde{}"),
                b'^' => output.extend_from_slice(br"\textasciicircum{}"),
                b'<' => output.extend_from_slice(br"\textless{}"),
                b'>' => output.extend_from_slice(br"\textgreater{}"),
                b'!' => output.extend_from_slice(br"!{}"),
                b'\\' => output.extend_from_slice(br"\textbackslash{}"),
                _ => output.push(c),
            }
        }
        Cow::Owned(String::from_utf8(output).unwrap())
    } else {
        input
    }
}


#[test]
fn html_0() {
    let s = "Some string without any character to escape";
    let result = html(s);
    assert_eq!(s, &result);
}

#[test]
fn tex_0() {
    let s = "Some string without any character to escape";
    let result = tex(s);
    assert_eq!(s, &result);
}

#[test]
fn nb_spaces_0() {
    let s = "Some string without any character to escape";
    let result = nb_spaces_html(s);
    assert_eq!(s, &result);
}

#[test]
fn tex_nb_spaces_0() {
    let s = "Some string without any character to escape";
    let result = nb_spaces_tex(s);
    assert_eq!(s, &result);
}

#[test]
fn quotes_0() {
    let s = "Some string without any character to escape";
    let result = quotes(s);
    assert_eq!(s, &result);
}

#[test]
fn html_1() {
    let s = "<p>Some characters need escaping & something</p>";
    let expected = "&lt;p&gt;Some characters need escaping &amp; something&lt;/p&gt;";
    let actual = html(s);
    assert_eq!(expected, &actual);
}

#[test]
fn html_2() {
    let actual = html("<foo> & <bar>");
    let expected = "&lt;foo&gt; &amp; &lt;bar&gt;";
    assert_eq!(&actual, expected);
}

#[test]
fn tex_braces() {
    let actual = tex(r"\foo{bar}");
    let expected = r"\textbackslash{}foo\{bar\}";
    assert_eq!(&actual, expected);
}

#[test]
fn tex_square_braces() {
    let actual = tex(r"foo[bar]");
    let expected = r"foo{[}bar{]}";
    assert_eq!(&actual, expected);
}

#[test]
fn tex_dashes() {
    let actual = tex("--foo, ---bar");
    let expected = r"-{}-foo, -{}-{}-bar";
    assert_eq!(&actual, expected);
}

#[test]
fn tex_numbers() {
    let actual = tex(r"30000$ is 10% of number #1 income");
    let expected = r"30000\$ is 10\% of number \#1 income";
    assert_eq!(&actual, expected);
}

#[test]
fn quotes_escape() {
    let actual = quotes(r#"Some text with "quotes""#);
    let expected = r#"Some text with 'quotes'"#;
    assert_eq!(&actual, expected);
}


#[test]
fn nnbsp_1() {
    let actual = nb_spaces_html("Test ?"); // nnbsp before ?
    let expected = "<span class = \"nnbsp\">Test&#160;?</span>";
    assert_eq!(&actual, expected);
}

#[test]
fn nnbsp_2() {
    let actual = nb_spaces_html("Ceci est un « Test » !"); // nnbsp before ! and before/after quotes
    let expected = "Ceci est un <span class = \"nnbsp\">«&#160;Test&#160;»&#160;!</span>";
    assert_eq!(&actual, expected);
}
