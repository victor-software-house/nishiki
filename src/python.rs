//! Python, with the scopes of the `python` grammar that Shiki bundles,
//! `MagicPython`.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers over
//! bytes, in the `engine` module.
//! The tables in `grammar` are generated from the grammar.
//! Its `matchers` decide where each pattern matches.
//!
//! The rules left open at the end of a line are frames of the highlighter's
//! interned state.
//! Triple-quoted strings, brackets and a statement continued by a backslash are
//! such rules.

mod engine;
mod grammar;
mod matchers;
mod scope;
#[rustfmt::skip]
mod words;

use engine::Python;
pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Halt, Line, Stopped};
use crate::limits::Finished;
use crate::rules;

/// The state at the start of a Python document whose scopes sit on the stack
/// `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    rules::start::<Python>(arena, base)
}

/// The state at the start of the Python lines of a fenced code block.
#[cfg(feature = "_start-embedded")]
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    rules::start_embedded::<Python>(arena, base)
}

/// Lexes one line under the crate's lexer contract.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    rules::lex::<Python>(cx, line, state, |scratch| &mut scratch.python)
}

/// Lexes `line` like [`lex`], for Python that another language embeds, up to
/// where `end` or `after` halts it (see [`rules::lex_until`]).
/// Julia embeds Python in a `py"""` string macro, up to its closing quotes.
#[inline]
pub(crate) fn lex_until(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    end: &dyn Halt,
    after: Option<&dyn Halt>,
) -> (Finished, Option<Stopped>) {
    rules::lex_until::<Python>(cx, line, state, |scratch| &mut scratch.python, end, after)
}

#[cfg(test)]
mod tests;
