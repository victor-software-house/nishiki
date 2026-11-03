//! Ruby, with the scopes of the `ruby` grammar that Shiki bundles, which
//! comes from Shopify's ruby-lsp through VS Code.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers
//! over bytes (see the `engine` module): the tables in `grammar` are
//! generated from the grammar, and the `matchers` decide where each pattern
//! matches. The rules left open at the end of a line (strings, heredocs,
//! `=begin` comments, blocks) are frames of the highlighter's interned state.

mod engine;
mod grammar;
mod matchers;
mod nested;
mod scope;

use engine::Ruby;
pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Line};
use crate::rules;

/// The state at the start of a Ruby document whose scopes sit on the stack
/// `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    rules::start::<Ruby>(arena, base)
}

/// The state at the start of the Ruby lines of a fenced code block.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    rules::start_embedded::<Ruby>(arena, base)
}

/// Lexes one line under the crate's lexer contract.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> u32 {
    rules::lex::<Ruby>(cx, line, state, |scratch| &mut scratch.ruby)
}

/// Lexes `line` like [`lex`], for Ruby that ERB embeds up to the end of its
/// code (see [`rules::lex_until`]).
#[cfg(feature = "lang-erb")]
#[inline]
pub(crate) fn lex_until(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    end: crate::language::Halt,
) -> (u32, Option<usize>) {
    rules::lex_until::<Ruby>(cx, line, state, |scratch| &mut scratch.ruby, end)
}

#[cfg(test)]
mod tests;
