// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![feature(test)]

extern crate test;
extern crate crowbook_text_processing;

use test::Bencher;
use crowbook_text_processing::{FrenchFormatter, clean, escape};

const STRINGS: &'static [&'static str] = &[
    "Some string with nothing special",
    r#""Hi!",  he  said. "I am 'pleased' to meet you..."#,
    "--- Comment  allez-vous ? demanda-t-elle. Moi, ça va !",
    "<< Comment allez-vous ? demanda-t-elle. Moi, ça va  ! >>",
    "Fast & Furious 7 > Fast & Furious 6",
    "20 % & 30 % -- Ce n'est pas assez, mon cher !",
    "Finding  some  random strings -- possibly exposing   all features -- is not 'easy'. . . ",
    r#""What about some longer string?", he asked.
"What about them?"
"Well, they might be better to bench this whole stuff, wouldn't they?", he replied.
"Yeah, I suppose...   Why not?""#,
    r#"<< Et de plus  grandes chaînes ? demanda-t-il.
--- Quoi, de plus grandes chaînes ?
--- Hé bien, elles pourraient être plus appropriées pour mesurer la performance de toute cette chose, ne crois tu pas ? répondit-il.
--- Ouais,  je suppose... Pourquoi pas ? >>"#
];

#[bench]
fn escape_tex(b: &mut Bencher) {
    b.iter(|| {
        for s in STRINGS {
            escape::tex(*s);
        }
    });
}

#[bench]
fn escape_html(b: &mut Bencher) {
    b.iter(|| {
        for s in STRINGS {
            escape::html(*s);
        }
    });
}

#[bench]
fn escape_quotes(b: &mut Bencher) {
    b.iter(|| {
        for s in STRINGS {
            escape::quotes(*s);
        }
    });
}

#[bench]
fn escape_nb_spaces_tex(b: &mut Bencher) {
    b.iter(|| {
        for s in STRINGS {
            escape::nb_spaces_tex(*s);
        }
    });
}

#[bench]
fn escape_nb_spaces(b: &mut Bencher) {
    b.iter(|| {
        for s in STRINGS {
            escape::nb_spaces(*s);
        }
    });
}

#[bench]
fn clean_ellpisis(b: &mut Bencher) {
    b.iter(|| {
        for s in STRINGS {
            clean::ellipsis(*s);
        }
    });
}

#[bench]
fn clean_quotes(b: &mut Bencher) {
    b.iter(|| {
        for s in STRINGS {
            clean::quotes(*s);
        }
    });
}

#[bench]
fn clean_dashes(b: &mut Bencher) {
    b.iter(|| {
        for s in STRINGS {
            clean::dashes(*s);
        }
    });
}


#[bench]
fn clean_whitespaces(b: &mut Bencher) {
    b.iter(|| {
        for s in STRINGS {
            clean::whitespaces(*s);
        }
    });
}

#[bench]
fn clean_guillemets(b: &mut Bencher) {
    b.iter(|| {
        for s in STRINGS {
            clean::guillemets(*s);
        }
    });
}


#[bench]
fn french_default(b: &mut Bencher) {
    b.iter(|| {
        let french = FrenchFormatter::new();
        for s in STRINGS {
            french.format(*s);
        }
    });
}

#[bench]
fn french_default_tex(b: &mut Bencher) {
    b.iter(|| {
        let french = FrenchFormatter::new();
        for s in STRINGS {
            french.format_tex(*s);
        }
    });
}

#[bench]
fn french_all(b: &mut Bencher) {
    b.iter(|| {
        let mut french = FrenchFormatter::new();
        french.ligature_dashes(true)
            .ligature_guillemets(true);
        for s in STRINGS {
            french.format(*s);
        }
    });
}

#[bench]
fn french_all_tex(b: &mut Bencher) {
    b.iter(|| {
        let mut french = FrenchFormatter::new();
        french.ligature_dashes(true)
            .ligature_guillemets(true);
        for s in STRINGS {
            french.format_tex(*s);
        }
    });
}
