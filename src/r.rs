//! R, with the scopes of the `r` grammar that Shiki bundles.
//! The grammar comes from Positron's R extension.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers over
//! bytes, in the `engine` module.
//! The tables in `grammar` are generated from the grammar, and its
//! `matchers` decide where each pattern matches.
//! Rules left open at the end of a line are frames of the highlighter's
//! interned state.
//! Strings, raw strings, brackets, roxygen comments and their `@examples`
//! blocks are such rules.

pub(crate) mod engine;
mod grammar;
mod matchers;
mod nested;
mod scope;

use engine::R;
pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Halt, Line, Stopped};
use crate::limits::Finished;
use crate::rules;

/// R's buffers in the highlighter's scratch.
/// The first R line allocates them.
fn buffers(scratch: &mut crate::scratch::Scratch) -> &mut Scratch {
    scratch.r.get_or_insert_with(Box::default)
}

/// The state at the start of an R document whose scopes sit on the stack
/// `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    rules::start::<R>(arena, base)
}

/// The state at the start of the R lines of a fenced code block.
#[cfg(feature = "_start-embedded")]
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    rules::start_embedded::<R>(arena, base)
}

/// Lexes one line under the crate's lexer contract.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    rules::lex::<R>(cx, line, state, buffers)
}

/// Lexes `line` like [`lex`], for R that another language embeds, up to
/// where `end` or `after` halts it (see [`rules::lex_until`]).
/// Julia's `R"""` string macro embeds R.
pub(crate) fn lex_until(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    end: &dyn Halt,
    after: Option<&dyn Halt>,
) -> (Finished, Option<Stopped>) {
    rules::lex_until::<R>(cx, line, state, buffers, end, after)
}

#[cfg(test)]
mod tests;
