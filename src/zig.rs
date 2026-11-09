//! Zig, with the scopes of the `zig` grammar that Shiki bundles.
//! The grammar comes from the Zig project's vscode-zig.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers over
//! bytes, in the `engine` module.
//! The tables in `grammar` are generated from the grammar.
//! The `matchers` decide where each pattern matches.
//!
//! The rules left open at the end of a line are frames of the highlighter's
//! interned state.
//! Only quoted strings and quoted names can be open there.
//! Shiki's Markdown grammar has no Zig fence.
//! No language therefore embeds Zig yet.
mod engine;
mod grammar;
mod matchers;
mod scope;

pub(crate) use engine::Scratch;
use engine::Zig;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Line};
use crate::limits::Finished;
use crate::rules;

/// Zig's buffers in the highlighter's scratch.
/// The first Zig line allocates them.
fn buffers(scratch: &mut crate::scratch::Scratch) -> &mut Scratch {
    scratch.zig.get_or_insert_with(Box::default)
}

/// The state at the start of a Zig document whose scopes sit on the stack
/// `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    rules::start::<Zig>(arena, base)
}

/// The state at the start of the Zig lines of a fenced code block.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    rules::start_embedded::<Zig>(arena, base)
}

/// Lexes one line under the crate's lexer contract.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    rules::lex::<Zig>(cx, line, state, buffers)
}

#[cfg(test)]
mod tests;
