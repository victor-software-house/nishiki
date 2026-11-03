//! Java, with the scopes of the `java` grammar that Shiki bundles, which
//! comes from the Red Hat Java extension for VS Code.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers
//! over bytes (see the `engine` module): the tables in `grammar` are
//! generated from the grammar, and the `matchers` decide where each pattern
//! matches. The rules left open at the end of a line (blocks, parentheses,
//! comments, text blocks) are frames of the highlighter's interned state.

mod engine;
mod grammar;
mod matchers;
mod nested;
mod scope;

use engine::Java;
pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Line};
use crate::rules;

/// The state at the start of a Java document whose scopes sit on the stack
/// `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    rules::start::<Java>(arena, base)
}

/// The state at the start of the Java lines of a fenced code block.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    rules::start_embedded::<Java>(arena, base)
}

/// Lexes one line under the crate's lexer contract.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> u32 {
    rules::lex::<Java>(cx, line, state, |scratch| &mut scratch.java)
}

#[cfg(test)]
mod tests;
