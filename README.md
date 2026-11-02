# nishiki

A syntax highlighting engine in Rust, written from scratch.

nishiki aims to be fast, small, and to colour code as Shiki does, with every supported language built in and nothing downloaded at run time.
The name comes from nishiki-e (錦絵, "brocade picture"), the multi-colour Japanese woodblock prints of the 1760s.

## Usage

```sh
cargo add nishiki
```

A `Highlighter` lexes one language a line at a time.
Each token ends at a byte offset and carries the TextMate scope stack that Shiki gives it.
The `State` after one line is the input of the next, so an editor can re-highlight from any line, and a streamed reply can be highlighted as it arrives.

```rust
use nishiki::{Highlighter, Language, State};

let mut highlighter = Highlighter::new(Language::Rust);
let mut state = State::default();
let mut tokens = Vec::new();
let line = "let x = 42;";
let report = highlighter.line(line, &mut state, &mut tokens);
assert!(report.is_complete());

let mut start = 0;
for token in &tokens {
    let mut scopes: Vec<&str> = highlighter.scopes(token.stack()).collect();
    scopes.reverse();
    println!("{:?} {}", &line[start..token.end()], scopes.join(" "));
    start = token.end();
}
```

`scopes` lists a stack innermost first, the order theme rules match it in.

## Limits

Every line is bounded by `Limits`: its length in bytes, the lexer steps it may take, the contexts open at once, and the scope stacks and states a highlighter interns.
Each limit can be set, raised, lowered or turned off, and `Limits::for_budget` derives them from a memory budget and a per-line time budget.
Limits count bytes and steps, never time, so the same input and limits give the same tokens on every machine.
Each line's `LineReport` names the limits it reached.

```rust
use nishiki::{Highlighter, Language, Limits};

let limits = Limits { depth: None, ..Limits::default() };
let highlighter = Highlighter::new_with_limits(Language::Rust, limits);
assert_eq!(highlighter.limits().depth, None);
```

## Scope and limits

1. Rust is the only language.
2. nishiki returns scopes, not colours: theme colouring is not built yet.
3. The scopes match Shiki's, quirks included: `||` is two `|` operators, `a!=b` reads `a!` as a macro, and a camel-case name such as `fooBar` gets no scope.
4. Every non-ASCII byte counts as a word character.

The API is unstable while the version is `0.0.x`.

## Licence

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](https://docs.rs/crate/nishiki/latest/source/LICENSE-APACHE))
- MIT license ([LICENSE-MIT](https://docs.rs/crate/nishiki/latest/source/LICENSE-MIT))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
