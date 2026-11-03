//! nishiki's own `TextMate` engine, which runs the grammars that Shiki bundles
//! for the languages that have no hand-written lexer, and checks the ones that
//! have.
//!
//! A grammar is a set of static tables that `oracle/engine-tables.ts` writes
//! from Shiki's grammar; the tokenizer walks them a line at a time over the
//! arena's states, and the regular-expression machine in [`regex`] runs the
//! patterns.

#![cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "no language runs on the engine yet; only its tests call it"
    )
)]

mod grammar;
mod regex;
mod scanner;
mod selector;
mod tokenizer;

#[cfg(test)]
mod tests;
