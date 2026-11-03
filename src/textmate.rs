//! nishiki's own `TextMate` engine, which runs the grammars that Shiki bundles
//! for the languages that have no hand-written lexer, and checks the ones that
//! have.
//!
//! A grammar is a set of static tables that `oracle/engine-tables.ts` writes
//! from Shiki's grammar; the tokenizer walks them a line at a time over the
//! arena's states, and the regular-expression machine in [`regex`] runs the
//! patterns.

pub(crate) mod grammar;
mod regex;
mod scanner;
mod selector;
mod tokenizer;

pub(crate) use grammar::Grammar;
use tokenizer::Text;
pub(crate) use tokenizer::{Scratch, start, start_embedded};

use crate::language::{Context, Line};

/// Lexes `line` of a language the engine runs with `grammar`, under the
/// crate's one lexer contract (`Language::lex`).
pub(crate) fn lex(
    cx: &mut Context<'_>,
    grammar: &'static Grammar,
    line: Line<'_>,
    state: u32,
) -> u32 {
    tokenizer::line(
        &mut cx.scratch.textmate,
        cx.arena,
        cx.limits,
        Text {
            grammar,
            bytes: line.text.as_bytes(),
            start: line.start,
            anchored: line.embedded,
        },
        state,
        cx.out,
        cx.report,
    )
}

#[cfg(test)]
mod tests;
