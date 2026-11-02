# Changelog

## nishiki 0.1.0

- One `Highlighter` for every language, chosen by `Language`, with configurable per-line `Limits` and a `LineReport` per line; `State` is now an interned handle, so nesting has no fixed ceiling, and `Highlighter::scopes` lists a stack innermost first ([#26](https://github.com/victor-software-house/nishiki-archive/pull/26)).
- The Rust lexer no longer stops advancing on an uppercase-led word followed by non-ASCII text, such as `R日`, which made it spin until the work limit ([#28](https://github.com/victor-software-house/nishiki-archive/pull/28)).

## nishiki 0.0.1

- Highlight Rust: `nishiki::rust::Highlighter` lexes one line at a time and gives each token the scope stack that Shiki's Rust grammar gives it, so a TextMate theme colours it as Shiki does ([#3](https://github.com/victor-software-house/nishiki-archive/pull/3)).
- The crate ships `THIRD-PARTY-NOTICES.md`, with the MIT notice of rust-syntax, whose scope names the Rust lexer emits ([#5](https://github.com/victor-software-house/nishiki-archive/pull/5)).

