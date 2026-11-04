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
use tokenizer::{Embedded, Text};
pub(crate) use tokenizer::{Scratch, start, start_embedded};

use crate::language::{Context, Halt, Line};
use crate::limits::{Finished, Steps};

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

/// Lexes `line` like [`lex`], for a grammar that another language embeds,
/// and returns what the engine spent beside the state, so the embedding
/// lexer counts it against the line's one budget: the engine starts from
/// the work its limits lend, and spends it in steps of the work limit.
pub(crate) fn lex_embedded(
    cx: &mut Context<'_>,
    grammar: &'static Grammar,
    line: Line<'_>,
    state: u32,
) -> Finished {
    let next = lex(cx, grammar, line, state);
    counted(cx, line, next)
}

/// Lexes `line` like [`lex_embedded`], halting where the embedding language
/// takes the line back: at the first place where `end` holds with only the
/// root rule open, tried before the root's patterns, so a match that starts
/// earlier wins and one that starts there loses, or where `after` holds and
/// none of the root's patterns match there; the second value is where. The
/// state at a halt is the root's, and without one the state resumes on the
/// next line.
pub(crate) fn lex_until(
    cx: &mut Context<'_>,
    grammar: &'static Grammar,
    line: Line<'_>,
    state: u32,
    end: &dyn Halt,
    after: Option<&dyn Halt>,
) -> (Finished, Option<usize>) {
    let (next, halted) = tokenizer::line_until(
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
        Embedded { end, after },
    );
    (counted(cx, line, next), halted)
}

/// `state` with the work the engine spent on `line`.
fn counted(cx: &Context<'_>, line: Line<'_>, state: u32) -> Finished {
    let steps = Steps::new(cx.limits, line.text.len());
    steps.add(cx.scratch.textmate.work_spent());
    steps.finish(state)
}

#[cfg(test)]
mod tests;
