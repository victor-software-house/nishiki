//! Kotlin, with the scopes of the `kotlin` grammar that Shiki bundles.
//! The grammar comes from the Kotlin extension for VS Code by fwcd.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers over
//! bytes, in the `engine` module.
//! The tables in `grammar` are generated from the grammar.
//! Its `matchers` decide where each pattern matches.
//! The rules left open at the end of a line (comments, strings, template
//! expressions) are frames of the highlighter's interned state.

mod engine;
mod grammar;
mod matchers;
mod nested;
mod scope;

use engine::Kotlin;
pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Line};
use crate::limits::Finished;
use crate::rules;

/// The state at the start of a Kotlin document whose scopes sit on the stack
/// `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    rules::start::<Kotlin>(arena, base)
}

/// The state at the start of the Kotlin lines of a fenced code block.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    rules::start_embedded::<Kotlin>(arena, base)
}

/// Lexes one line under the crate's lexer contract.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    rules::lex::<Kotlin>(cx, line, state, |scratch| &mut scratch.kotlin)
}

#[cfg(test)]
mod tests;
