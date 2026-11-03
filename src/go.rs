//! Go, with the scopes of the `go` grammar that Shiki bundles, which
//! comes from VS Code.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers
//! over bytes (see the `engine` module): the tables in `grammar` are
//! generated from the grammar, and the `matchers` decide where each pattern
//! matches. The rules left open at the end of a line (blocks, comments, raw
//! strings, declarations) are frames of the highlighter's interned state.

mod engine;
mod grammar;
mod matchers;
mod nested;
mod scope;

use engine::Go;
pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Line};
use crate::rules;

/// The state at the start of a Go document whose scopes sit on the stack
/// `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    rules::start::<Go>(arena, base)
}

/// The state at the start of the Go lines of a fenced code block.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    rules::start_embedded::<Go>(arena, base)
}

/// Lexes one line under the crate's lexer contract.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> u32 {
    rules::lex::<Go>(cx, line, state, |scratch| &mut scratch.go)
}

#[cfg(test)]
mod tests;
