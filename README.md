# nishiki

A syntax highlighting engine in Rust, written from scratch.

nishiki aims to be fast, small, and to colour code as Shiki does, with every supported language built in and nothing downloaded at run time.
The name comes from nishiki-e (錦絵, "brocade picture"), the multi-colour Japanese woodblock prints of the 1760s.

## Usage

```sh
cargo add nishiki
```

`nishiki::rust::Highlighter` lexes one line at a time.
Each token ends at a byte offset and carries the TextMate scope stack that Shiki gives it.
The `State` after one line is the input of the next.

```rust
use nishiki::rust::{Highlighter, State};

let mut highlighter = Highlighter::new();
let mut state = State::default();
let mut tokens = Vec::new();
let line = "let x = 42;";
highlighter.line(line, &mut state, &mut tokens);

let mut start = 0;
for token in &tokens {
    let scopes = highlighter.scopes(token.stack());
    println!("{:?} {}", &line[start..token.end()], scopes.join(" "));
    start = token.end();
}
```

## Scope and limits

1. Rust is the only language.
2. nishiki returns scopes, not colours: theme colouring is not built yet.
3. The scopes match Shiki's, quirks included: `||` is two `|` operators, `a!=b` reads `a!` as a macro, and a camel-case name such as `fooBar` gets no scope.
4. A context nested deeper than 32 is not tracked, a raw string with more than 255 `#` is not a raw string, and every non-ASCII byte counts as a word character.

The API is unstable while the version is `0.0.x`.

## Licence

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](https://docs.rs/crate/nishiki/latest/source/LICENSE-APACHE))
- MIT license ([LICENSE-MIT](https://docs.rs/crate/nishiki/latest/source/LICENSE-MIT))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
