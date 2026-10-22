# Changelog

## nishiki 0.0.1

- Highlight Rust: `nishiki::rust::Highlighter` lexes one line at a time and gives each token the scope stack that Shiki's Rust grammar gives it, so a TextMate theme colours it as Shiki does ([#3](https://github.com/victor-software-house/nishiki-archive/pull/3)).
- The crate ships `THIRD-PARTY-NOTICES.md`, with the MIT notice of rust-syntax, whose scope names the Rust lexer emits ([#5](https://github.com/victor-software-house/nishiki-archive/pull/5)).

