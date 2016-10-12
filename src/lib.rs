// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Some utilities functions for processing texts.
//!
//! These functions were originally written for [Crowbook](https://github.com/lise-henry/crowbook),
//! but have been published on a separate crate and under a less
//! restrictive license (MPL instead of LGPL) so they can be used
//! elsewhere.
//!
//! # Example
//!
//! ```
//! use crowbook_text_processing::escape::*;
//! use crowbook_text_processing::clean::*;
//! use crowbook_text_processing::french::FrenchFormatter;
//!
//! let s = " Some  string with  too much   whitespaces & around 1% characters that might cause trouble to HTML or LaTeX.";
//! let new_s = remove_whitespaces(s);
//! println!("for HTML: {}", escape_html(new_s.clone()));
//! println!("for LaTeX: {}", escape_tex(new_s));
//!
//! let s = " Une chaîne en français ! On voudrait un résultat « typographiquement correct ».";
//! let new_s = FrenchFormatter::new().format(s);
//! println!("for HTML: {}", escape_nb_spaces(escape_html(new_s.clone())));
//! println!("for LaTeX: {}", escape_nb_spaces_tex(escape_tex(new_s)));
//! ```
//! # Requirements
//!
//! * `rustc >= 1.6.0`
//!
//! # Semantic versioning
//!
//! While not yet at version `1.0`, this crates tries to follows semantic versioning in the following way:
//!
//! * an increase of `x` in `0.x.y` means breaking changes.
//! * an increase of `y` in `0.x.y` means non-breaking changes.
extern crate regex;
#[macro_use] extern crate lazy_static;

pub mod escape;
pub mod clean;
pub mod french;

mod common;
