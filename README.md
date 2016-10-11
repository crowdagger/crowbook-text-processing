# crowbook-text-processing

Some text processing functions initially written
for [Crowbook](https://github.com/lise-henry/crowbook) and moved in a
separate library (and a more permissive license) so they can be used in other projects.

## Usage ##

Add this to your `Cargo.toml` dependencies section:

```toml
crowbook-text-processing = "0.1"
```

## Example ##

```rust
extern crate crowbook_text_processing;

use crowbook_text_processing::escape::*;
use crowbook_text_processing::clean::*;
use crowbook_text_processing::french::FrenchFormatter;

fn main() {

    let s = " Some  string with  too much   whitespaces & around 1% characters that might cause trouble to HTML or LaTeX.";
    let new_s = remove_whitespaces(s);
    println!("for HTML: {}", escape_html(new_s.clone()));
    println!("for LaTeX: {}", escape_tex(new_s));
    
    let s = " Une chaîne en français ! On voudrait un résultat « typographiquement correct ».";
    let new_s = FrenchFormatter::new().format(s);
    println!("for HTML: {}", escape_nb_spaces(escape_html(new_s.clone())));
    println!("for LaTeX: {}", escape_nb_spaces_tex(escape_tex(new_s)));
}
```

## Documentation ##

See the
[documentation on docs.rs](https://docs.rs/crowbook-text-processing).

## ChangeLog ##

See [the ChangeLog file](ChangeLog.md).


## Author ##

[Élisabeth Henry](http://lise-henry.github.io/) <liz.henry@ouvaton.org>. 

## License ##

This is free software, published under the [Mozilla Public License,
version 2.0](https://www.mozilla.org/en-US/MPL/2.0/).

## Semantic versioning ##

While not yet at version `1.0`, this crates tries to follows semantic versioning in the following way:

* an increase of `x` in `0.x.y` means there is some breaking change. 
* an increase of `y` in `0.x.y` means there should not be any breaking changes.
