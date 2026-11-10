//! nishiki's own `TextMate` engine, which runs the grammars that Shiki bundles.
//!
//! It runs them for the languages that have no hand-written lexer, and checks
//! the ones that have.
//!
//! - A grammar is a set of tables that the compiler in [`compile`] links from a
//!   grammar's JSON.
//! - The tokenizer walks them a line at a time over the arena's states.
//! - The regular-expression machine in [`regex`] runs the patterns.

#[cfg(feature = "json")]
pub(crate) mod compile;
pub(crate) mod grammar;
mod regex;
mod scanner;
mod selector;
mod tokenizer;

pub(crate) use grammar::Grammar;
pub(crate) use tokenizer::Scratch;
#[cfg(feature = "json")]
pub(crate) use tokenizer::start;
use tokenizer::{Embedded, Text};

use crate::language::{Context, Halt, Line, Stopped};
use crate::limits::{Finished, Steps};

/// Lexes `line` of a language the engine runs with `grammar`, under the
/// crate's one lexer contract (`Language::lex`).
///
/// Never inlined, as `lex_handing` is not. With the engine's scratch boxed,
/// this function was inlined into `Highlighter::line` and `line_until` was
/// not inlined into it. Batch then rose 0.2% at one codegen unit.
#[cfg_attr(
    not(any(test, feature = "json")),
    expect(
        dead_code,
        reason = "without json, no grammar reaches the engine; its tests run it"
    )
)]
#[inline(never)]
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

/// Lexes `line` like [`lex`], for a grammar that another language embeds.
/// It returns what the engine spent beside the state, so the embedding lexer
/// counts it against the line's one budget.
/// The engine starts from the work its limits lend, and spends it in steps
/// of the work limit.
#[expect(dead_code, reason = "no language runs on the TextMate engine now")]
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
/// takes the line back.
///
/// - It halts at the first place where `end` holds with only the root rule
///   open. `end` is tried before the root's patterns, so a match that starts
///   earlier wins and one that starts there loses.
/// - It also halts where `after` holds and none of the root's patterns match
///   there.
///
/// The second value is where.
/// The state at a halt is the root's, and without one the state resumes on
/// the next line.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "no language embeds an engine grammar now; its tests do"
    )
)]
pub(crate) fn lex_until(
    cx: &mut Context<'_>,
    grammar: &'static Grammar,
    line: Line<'_>,
    state: u32,
    end: &dyn Halt,
    after: Option<&dyn Halt>,
) -> (Finished, Option<Stopped>) {
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
    (counted(cx, line, next), halted.map(Stopped::halt))
}

/// Lexes `line` like [`lex`], for a grammar that hands regions to languages
/// lexed by hand (`Grammar::hand_offs`).
///
/// Never inlined. Inlined into `Language::lex`, the handing loop widened
/// `Highlighter::line` for every language. Its own cost rose about 6% on the
/// diff, Rust and JSON fixtures at one codegen unit.
#[cfg_attr(
    not(any(test, feature = "json")),
    expect(
        dead_code,
        reason = "without json, no grammar reaches the engine; its tests run it"
    )
)]
#[inline(never)]
pub(crate) fn lex_handing(
    cx: &mut Context<'_>,
    grammar: &'static Grammar,
    line: Line<'_>,
    state: u32,
) -> u32 {
    tokenizer::line_handing(
        cx.scratch,
        cx.arena,
        cx.limits,
        grammar,
        line,
        state,
        cx.out,
        cx.report,
        cx.first_token,
        None,
    )
    .0
}

/// Lexes `line` like [`lex_embedded`], for a grammar that hands regions to
/// languages lexed by hand.
#[cfg_attr(
    any(not(test), not(feature = "_hand-lexers")),
    expect(
        dead_code,
        reason = "no language embeds an engine grammar now; its hand-off tests do, with a hand-written lexer on"
    )
)]
pub(crate) fn lex_handing_embedded(
    cx: &mut Context<'_>,
    grammar: &'static Grammar,
    line: Line<'_>,
    state: u32,
) -> Finished {
    let next = lex_handing(cx, grammar, line, state);
    counted(cx, line, next)
}

/// Lexes `line` like [`lex_until`], for a grammar that hands regions to
/// languages lexed by hand.
#[expect(
    dead_code,
    reason = "no language runs on the TextMate engine now, and no test hands off under a halt"
)]
pub(crate) fn lex_handing_until(
    cx: &mut Context<'_>,
    grammar: &'static Grammar,
    line: Line<'_>,
    state: u32,
    end: &dyn Halt,
    after: Option<&dyn Halt>,
) -> (Finished, Option<Stopped>) {
    let (next, halted) = tokenizer::line_handing(
        cx.scratch,
        cx.arena,
        cx.limits,
        grammar,
        line,
        state,
        cx.out,
        cx.report,
        cx.first_token,
        Some(Embedded { end, after }),
    );
    (counted(cx, line, next), halted.map(Stopped::halt))
}

/// Lexes `line` of a linked grammar that a highlighter runs.
///
/// A grammar that hands regions to languages lexed by hand runs the handing
/// loop, as its language would.
#[cfg(any(test, feature = "json"))]
pub(crate) fn lex_linked(
    cx: &mut Context<'_>,
    grammar: &'static Grammar,
    line: Line<'_>,
    state: u32,
) -> u32 {
    if grammar.hand_offs.is_empty() {
        lex(cx, grammar, line, state)
    } else {
        lex_handing(cx, grammar, line, state)
    }
}

/// `state` with the work the engine spent on `line`.
fn counted(cx: &Context<'_>, line: Line<'_>, state: u32) -> Finished {
    let steps = Steps::new(cx.limits, line.text.len());
    steps.add(cx.scratch.textmate.work_spent());
    steps.finish(state)
}

#[cfg(test)]
mod tests;
