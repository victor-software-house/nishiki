//! Python, with the scopes of the `python` grammar that Shiki bundles
//! (`MagicPython`).
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers
//! over bytes (see the `engine` module): the tables in `grammar` are
//! generated from the grammar, and the `matchers` decide where each pattern
//! matches. The rules left open at the end of a line (triple-quoted strings,
//! brackets, a statement continued by a backslash) are frames of the
//! highlighter's interned state.

mod engine;
mod grammar;
mod matchers;
mod scope;
mod words;

use engine::Python;
pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Line};
use crate::limits::Finished;
use crate::rules;

/// The state at the start of a Python document whose scopes sit on the stack
/// `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    rules::start::<Python>(arena, base)
}

/// The state at the start of the Python lines of a fenced code block.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    rules::start_embedded::<Python>(arena, base)
}

/// Lexes one line under the crate's lexer contract.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    rules::lex::<Python>(cx, line, state, |scratch| &mut scratch.python)
}

#[cfg(test)]
mod tests;
