//! Swift, with the scopes of the `swift` grammar that Shiki bundles.
//! The grammar comes from Jacob Bandes-Storch's `swift-tmlanguage`.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers over
//! bytes, in the `engine` module.
//! The tables in `grammar` are generated from the grammar.
//! Its `matchers` decide where each pattern matches.
//! The rules left open at the end of a line (blocks, parentheses, strings,
//! comments) are frames of the highlighter's interned state.

mod dispatch;
mod engine;
mod grammar;
mod matchers;
mod scope;

pub(crate) use engine::Scratch;
use engine::Swift;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Line};
use crate::limits::Finished;
use crate::rules;

/// The state at the start of a Swift document whose scopes sit on the stack
/// `base`, on the line where `\A` matches, for the shebang comment.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    rules::start::<Swift>(arena, base)
}

/// The state at the start of the Swift lines of a fenced code block.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    rules::start_embedded::<Swift>(arena, base)
}

/// Lexes one line under the crate's lexer contract.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    rules::lex::<Swift>(cx, line, state, |scratch| &mut scratch.swift)
}

#[cfg(test)]
mod tests;
