//! MATLAB, with the scopes of the `matlab` grammar that Shiki bundles, from
//! mathworks/MATLAB-Language-grammar.
//!
//! The lexer runs the grammar's begin/end rules on the shared rule engine, in
//! the `engine` module.
//! The tables in `grammar` are generated from the grammar.
//! Its `matchers` decide where each regular expression matches.
//! The rules left open at the end of a line are frames of the highlighter's
//! interned state.
//! Shiki's Markdown grammar has no MATLAB fence.

mod engine;
mod grammar;
mod matchers;
mod scope;

use engine::Matlab;
pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Halt, Line, Stopped};
use crate::limits::Finished;
use crate::rules;

/// MATLAB's buffers in the highlighter's scratch.
/// The first MATLAB line allocates them.
fn buffers(scratch: &mut crate::scratch::Scratch) -> &mut Scratch {
    scratch.matlab.get_or_insert_with(Box::default)
}

/// The state at the start of a MATLAB document whose scopes sit on the stack
/// `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    rules::start::<Matlab>(arena, base)
}

/// The state at the start of the MATLAB lines of a fenced code block.
#[cfg(feature = "_start-embedded")]
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    rules::start_embedded::<Matlab>(arena, base)
}

/// Lexes one line under the crate's lexer contract.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    rules::lex::<Matlab>(cx, line, state, buffers)
}

/// Lexes `line` like [`lex`], for MATLAB that another language embeds, up to
/// where `end` or `after` halts it (see [`rules::lex_until`]).
#[inline]
pub(crate) fn lex_until(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    end: &dyn Halt,
    after: Option<&dyn Halt>,
) -> (Finished, Option<Stopped>) {
    rules::lex_until::<Matlab>(cx, line, state, buffers, end, after)
}

#[cfg(test)]
mod tests;
