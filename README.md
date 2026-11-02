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

1. Rust, Markdown, TypeScript, shell (`Language::Shell`, also `bash`, `sh` and `zsh`), JSON (`Language::Json`, `Language::Jsonc` and `Language::Jsonl`), Python (`Language::Python`, also `py`), YAML (`Language::Yaml`, also `yml`), TOML (`Language::Toml`) and Makefile (`Language::Make`, also `makefile`) are the languages.
   A Markdown fence of `rust`, `typescript` (or `ts`), `markdown`, `shell`, `sh`, `bash` or `zsh`, `json` (or `json5`), `jsonc`, `jsonl`, `python` (or `py`), `yaml` or `yml`, or `makefile` (also `Makefile`, `GNUmakefile` and `OCamlMakefile`) is lexed as that language, and so is YAML front matter; any other fence is plain text, as in Shiki when only these languages are loaded.
   That includes `toml`: Shiki's Markdown grammar has no rule that embeds it.
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
8. JSON reads every dialect's comments, and a stray character in an array or object gets the grammar's `invalid.illegal` scope.
   `\d`, `\w` and `\s` for non-ASCII characters come from a table measured on Shiki 4.6.0 (`oracle/json-classes.ts`), so a later Unicode version in Shiki's engine would need the table regenerated.
9. The Python scopes keep the grammar's quirks: `def` opens a function after the end of any word, as in `x def f(): pass`, and an empty line leaves the state as it is.
   `\d`, `\w`, `\s`, `[[:alpha:]]` and `\p{upper}` for non-ASCII characters come from tables measured on Shiki 4.6.0 (`oracle/python-classes.ts`), so a later Unicode version in Shiki's engine would need them regenerated.
10. The YAML lexer follows the YAML 1.1 type patterns that Shiki's grammar uses: `0o17` is plain text, an exponent needs its sign, and `y` and `n` are booleans, as keys too.
   Each line is read on its own, so a plain scalar on several lines is scoped line by line, and a comment whose text holds a tab is not a comment.
   Front matter starts only on the first line of a Markdown document, so the state of a document's first line differs from the states after it.
   A line of a block scalar is text up to the first line indented less than the scalar's first line.
11. TOML keeps the grammar's quirks.
   A value that follows another in an array without a comma, text that is neither a key, a value nor a table header, and a stray character after a value are `invalid.illegal`; `01` is the number `0` and then an invalid `1`; a table header is read only at the start of a line; and a key that begins with a `"` ends at the first later `"` that an `=` follows.
   The grammar has no licence, so the lexer is written from the TOML specification and from Shiki's output alone.
   `\s` for non-ASCII characters is JSON's table, which a pass over every character confirmed for this grammar too.
12. In a Makefile, `$` at the end of a recipe line is not a variable, a `:` in any line that is not an assignment makes it a target, and the `endef` of a definition has the scope of `override`, as in Shiki.
    A Markdown fence's Makefile lines see the line feed of a line that a quote or list took whole, so they close the references that line left open.

The API is unstable while the version is `0.x`.

## Licence

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](https://docs.rs/crate/nishiki/latest/source/LICENSE-APACHE))
- MIT license ([LICENSE-MIT](https://docs.rs/crate/nishiki/latest/source/LICENSE-MIT))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
