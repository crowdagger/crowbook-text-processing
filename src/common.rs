// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub const NB_CHAR: char = ' '; // non breaking space
pub const NB_CHAR_NARROW: char = '\u{202F}'; // narrow non breaking space
pub const NB_CHAR_EM: char = '\u{2002}'; // demi em space


/// Custom function because we don't really want to touch \t or \n
///
/// This function detects spaces and non breking spaces
pub fn is_whitespace(c: char) -> bool {
    c == ' ' || c == ' ' || c == ' '
}
