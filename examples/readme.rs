extern crate crowbook_text_processing;

use crowbook_text_processing::escape::*;
use crowbook_text_processing::clean;
use crowbook_text_processing::FrenchFormatter;

fn main() {

    let s = " Some  string with  too much   whitespaces & around 1% characters that might cause trouble to HTML or LaTeX.";
    let new_s = clean::whitespaces(s);
    println!("for HTML: {}", escape_html(new_s.clone()));
    println!("for LaTeX: {}", escape_tex(new_s));
    
    let s = " Une chaîne en français ! On voudrait un résultat « typographiquement correct ».";
    let new_s = FrenchFormatter::new().format(s);
    println!("for HTML: {}", escape_nb_spaces(escape_html(new_s.clone())));
    println!("for LaTeX: {}", escape_nb_spaces_tex(escape_tex(new_s)));
}
