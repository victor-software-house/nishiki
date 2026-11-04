//! C, with the scopes of the `c` grammar that Shiki bundles.
//!
//! The lexer runs the grammar's begin/end rules on the shared rule engine
//! ([`crate::rules`]), with C's hand-written pattern lists and matchers (see
//! the `engine` module). The rules left open at the end of a
//! line (comments, strings, blocks, macros with a backslash at the end) are
//! frames of the highlighter's interned state.
//!
//! It reproduces Shiki's output, quirks included: a `#define` is found
//! anywhere in a line, an unterminated string runs to the end of the
//! document, `x or(` is an initialization, and `a.b.c (` is a member access.

mod access;
mod comments;
mod ends;
mod engine;
mod lists;
mod numbers;
mod pats;
mod preproc;
mod scope;
mod simple;
mod state;
mod strings;
mod switch;
mod text;
mod words;
mod wordset;

pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Halt, Line};
use crate::limits::Finished;

/// The state at the start of a C document whose scopes sit on the stack
/// `base`: one rule with `source.c` on top of it.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    crate::rules::start::<engine::C>(arena, base)
}

/// The state at the start of the C lines of a fenced code block, whose text
/// sits on the stack `base` with no `source.c` of its own, as in Shiki's
/// embedding.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    crate::rules::start_embedded::<engine::C>(arena, base)
}

/// Lexes one line under the crate's lexer contract.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    crate::rules::lex::<engine::C>(cx, line, state, |scratch| &mut scratch.c)
}

/// Lexes `line` like [`lex`], for C that another language embeds, up to
/// where `end` or `after` halts it (see [`crate::rules::lex_until`]).
#[inline]
pub(crate) fn lex_until(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    end: Halt,
    after: Option<Halt>,
) -> (Finished, Option<usize>) {
    crate::rules::lex_until::<engine::C>(cx, line, state, |scratch| &mut scratch.c, end, after)
}

#[cfg(test)]
mod tests;
