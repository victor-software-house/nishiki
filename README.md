# nishiki

A syntax highlighting engine in Rust, written from scratch.

nishiki aims to be fast, small, and to colour code as Shiki does, with every supported language built in and nothing downloaded at run time.
The name comes from nishiki-e (錦絵, "brocade picture"), the multi-colour Japanese woodblock prints of the 1760s.

## Status

Early. nishiki highlights Rust: `nishiki::rust::Highlighter` lexes one line at a time and gives each token the TextMate scope stack that Shiki gives it.
More languages and theme colouring come next.

```rust
use nishiki::rust::{Highlighter, State};

let mut highlighter = Highlighter::new();
let mut state = State::default();
let mut tokens = Vec::new();
highlighter.line("let x = 42;", &mut state, &mut tokens);
for token in &tokens {
    println!("{} {:?}", token.end(), highlighter.scopes(token.stack()));
}
```

## Licence

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Credits

The Rust lexer is written from scratch, but it emits the scope names of the Rust TextMate grammar in [dustypomerleau/rust-syntax](https://github.com/dustypomerleau/rust-syntax), which Shiki bundles through VS Code, and tries that grammar's rules in its order.
That grammar is licensed under the MIT License:

```text
MIT License

Copyright (c) 2020 Dustin Pomerleau

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
