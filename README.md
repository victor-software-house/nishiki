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

## Themes

A `Theme` is a TextMate theme in Shiki's format, and `Theme::style` gives a scope stack the colour and font style Shiki would.
With the `json` feature, `Theme::from_json` reads a theme file as `@shikijs/themes` ships it; without it, `Theme::new` takes the rules as values and nishiki depends on `memchr` alone.

```rust
use nishiki::{Highlighter, Language, RuleScope, State, Theme, ThemeRule, ThemeSource};

let theme = Theme::new(ThemeSource {
    rules: vec![ThemeRule {
        scope: RuleScope::Selectors("storage.type".into()),
        foreground: Some("#f97583".into()),
        ..ThemeRule::default()
    }],
    ..ThemeSource::default()
});
let mut highlighter = Highlighter::new(Language::Rust);
let mut tokens = Vec::new();
let _ = highlighter.line("let x = 42;", &mut State::default(), &mut tokens);
let style = theme.style(highlighter.scopes(tokens[0].stack()));
assert_eq!(style.foreground.to_string(), "#F97583");
```

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

1. Rust, Markdown, TypeScript and shell (`Language::Shell`, also `bash`, `sh` and `zsh`) are the languages.
   A Markdown fence of `rust`, `typescript` (or `ts`), `markdown`, or `shell`, `sh`, `bash` or `zsh` is lexed as that language; any other fence is plain text, as in Shiki when only these languages are loaded.
2. A theme colour that is not hex, such as a CSS variable, sets no colour; Shiki passes it through.
3. The scopes match Shiki's, quirks included: `||` is two `|` operators, `a!=b` reads `a!` as a macro, and a camel-case name such as `fooBar` gets no scope.
   In Markdown an empty line leaves the state unchanged while a line of spaces does not, and HTML blocks other than comments are plain text.
4. In Rust, every non-ASCII byte counts as a word character.
5. In Markdown, a label or link destination nested deeper than 32 brackets ends there.
   The grammar's backtracking is followed where it matters in practice, not everywhere: a code span gives back backticks, but an escaped `>` does not end an angle-bracket destination.
   A fence of more than 255 characters is never closed.
6. The TypeScript scopes keep the grammar's own quirks: `Number.MAX_VALUE` gets no scope, and an empty line leaves the state as it is.
7. In shell, an unterminated string or here-document keeps every later line, `||` outside a statement is two `|` operators, and an argument such as `ab>c` is split as `a` and `b>c`.
   An empty line leaves the state as it is, because Shiki does not tokenize empty lines.
   A non-ASCII character is a word character when it is alphanumeric, so combining marks are not, and it is white space when Unicode says so.

The API is unstable while the version is `0.x`.

## Licence

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](https://docs.rs/crate/nishiki/latest/source/LICENSE-APACHE))
- MIT license ([LICENSE-MIT](https://docs.rs/crate/nishiki/latest/source/LICENSE-MIT))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
