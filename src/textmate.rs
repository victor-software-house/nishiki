//! nishiki's own `TextMate` engine, which runs the grammars that Shiki bundles
//! for the languages that have no hand-written lexer, and checks the ones that
//! have.
//!
//! This module holds the regular-expression engine; the grammar tables and
//! the tokenizer come with the next changes.

#![expect(
    dead_code,
    reason = "the engine is built in stages and nothing calls it yet"
)]

mod regex;
