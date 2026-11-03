//! CSS, with the scopes of the `css` grammar that Shiki bundles, which
//! comes from VS Code.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers
//! over bytes (see the `engine` module): the tables in `grammar` are
//! generated from the grammar, and the `matchers` decide where each pattern
//! matches. The rules left open at the end of a line (rule lists, at-rules,
//! comments, strings, selectors) are frames of the highlighter's interned
//! state.

mod ahead;
mod engine;
mod grammar;
mod matchers;
mod nested;
mod scope;
mod words;

use engine::Css;
pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Line};
use crate::rules;

/// The state at the start of a CSS document whose scopes sit on the stack
/// `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    rules::start::<Css>(arena, base)
}

/// The state at the start of the CSS lines of a fenced code block.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    rules::start_embedded::<Css>(arena, base)
}

/// Lexes one line under the crate's lexer contract.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> u32 {
    rules::lex::<Css>(cx, line, state, |scratch| &mut scratch.css)
}

#[cfg(test)]
mod tests;
