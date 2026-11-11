//! C, with the scopes of the `c` grammar that Shiki bundles.
//!
//! The lexer runs the grammar's begin/end rules on the shared rule engine
//! ([`crate::rules`]).
//! It uses C's hand-written pattern lists and matchers, in the `engine`
//! module.
//! The rules left open at the end of a line are frames of the highlighter's
//! interned state.
//! Comments, strings, blocks and macros with a backslash at the end are such
//! rules.
//!
//! It reproduces Shiki's output, quirks included:
//! - a `#define` is found anywhere in a line;
//! - an unterminated string runs to the end of the document;
//! - `x or(` is an initialization;
//! - `a.b.c (` is a member access.

mod access;
mod comments;
mod ends;
mod engine;
mod lists;
mod numbers;
mod patterns;
mod preproc;
mod scope;
mod simple;
mod state;
mod strings;
mod switch;
mod text;
#[rustfmt::skip]
mod words;
mod wordset;

pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Halt, Line, Stopped};
use crate::limits::Finished;

/// The state at the start of a C document whose scopes sit on the stack
/// `base`: one rule with `source.c` on top of it.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    crate::rules::start::<engine::C>(arena, base)
}

/// The state at the start of the C lines of a fenced code block.
/// Their text sits on the stack `base` with no `source.c` of its own, as in
/// Shiki's embedding.
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
    end: &dyn Halt,
    after: Option<&dyn Halt>,
) -> (Finished, Option<Stopped>) {
    crate::rules::lex_until::<engine::C>(cx, line, state, |scratch| &mut scratch.c, end, after)
}

#[cfg(test)]
mod tests;
