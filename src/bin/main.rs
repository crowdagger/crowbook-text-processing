// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

extern crate crowbook_text_processing;

use crowbook_text_processing::{FrenchFormatter, clean, escape};

use std::env;
use std::io;
use std::io::BufRead;
use std::process::exit;

const TOOLS: &'static[(&'static str, &'static str)] = &[
    ("escape_html", "escape text for HTML display"),
    ("escape_tex", "escape text for LaTeX display"),
    ("escape_nb_spaces_html", "replace narrow non-breaking spaces with HTML elements"),
    ("escape_nb_spaces_tex", "escape non-breaking spaces using TeX symbol"),
    ("clean_ellipsis", "use unicode character ‘…’ for ellipsis"),
    ("clean_quotes", "try to replace straight quotes with curly ones"),
    ("ligature_dashes", "replace ‘--’ by ‘–’ and ‘---’ by ‘—’"),
    ("ligature_guillemets", "replace ‘<<’ by ‘«’ and ‘>>’ by ‘»’"),
    ("format_french", "try to apply french typographic rules"),
];

fn print_transformations() {
    for &(name, desc) in TOOLS {
        println!("    {name}: {desc}",
                 name = name,
                 desc = desc);
    }
}

fn main() {
    let args:Vec<_> = env::args()
        .collect();
    if args.len() == 1 {
        println!("\
{bin} {version}

USAGE: {bin} <TRANSFORMATIONS>

Read standard input, sequentially apply each TRANSFORMATION on the text, and print the
result on standard output.

Valid transformations are the following:",
                 bin = args[0],
                 version = env!("CARGO_PKG_VERSION"));
        print_transformations();
        println!("");
        println!("EXAMPLE: {bin} clean_quotes clean_ellipsis escape_html",
                 bin = args[0]);
    } else {
        let french = FrenchFormatter::new();

        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let line = line.expect("Error reading from standard input");
            let mut output = clean::whitespaces(line);
            for argument in &args[1..] {
                output = match argument.as_ref() {
                    "escape_html" => escape::html(output),
                    "escape_tex" => escape::tex(output),
                    "escape_nbsp" => escape::nb_spaces_html(output),
                    "escape_nb_spaces_tex" => escape::nb_spaces_tex(output),
                    "clean_quotes" => clean::quotes(output),
                    "clean_ellipsis" => clean::ellipsis(output),
                    "format_french" => french.format(output),
                    "ligature_dashes" => clean::dashes(output),
                    "ligature_guillemets" => clean::guillemets(output),
                    t => {
                        println!("Error: transformation “{}” not recognized.", t);
                        println!("Valid transformations are:");
                        print_transformations();
                        exit(0);
                    },
                }
            }
            println!("{}", output);
        }
    }
}
