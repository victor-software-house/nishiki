//! SCSS, with the scopes of the `scss` grammar that Shiki bundles, which
//! comes from VS Code.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers
//! over bytes (see the `engine` module): the tables in `grammar` are
//! generated from the grammar, and the `matchers` decide where each pattern
//! matches. The grammar includes rules of CSS's (`source.css#…`), which run
//! on the CSS lexer's matchers. The rules left open at the end of a line
//! (property lists, at-rules, comments, strings, maps) are frames of the
//! highlighter's interned state.

mod engine;
mod grammar;
mod matchers;
mod scope;

pub(crate) use engine::Scratch;
use engine::Scss;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Line};
use crate::rules;

/// The state at the start of an SCSS document whose scopes sit on the stack
/// `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    rules::start::<Scss>(arena, base)
}

/// The state at the start of the SCSS lines of a fenced code block.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    rules::start_embedded::<Scss>(arena, base)
}

/// Lexes one line under the crate's lexer contract.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> u32 {
    rules::lex::<Scss>(cx, line, state, |scratch| &mut scratch.scss)
}

#[cfg(test)]
mod tests;
