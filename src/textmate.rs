//! nishiki's own `TextMate` engine, which runs the grammars that Shiki bundles
//! for the languages that have no hand-written lexer, and checks the ones that
//! have.
//!
//! A grammar is a set of static tables that `oracle/engine-tables.ts` writes
//! from Shiki's grammar; the tokenizer walks them a line at a time over the
//! arena's states, and the regular-expression machine in [`regex`] runs the
//! patterns.

#![expect(
    dead_code,
    reason = "the engine is built in stages and nothing calls it yet"
)]

mod grammar;
mod regex;
mod tokenizer;

#[cfg(test)]
mod tests;
