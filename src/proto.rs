//! Protobuf, with the scopes of the `proto` grammar that Shiki bundles.
//! The grammar comes from zxh0's vscode-proto3.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers over
//! bytes, in the `engine` module.
//! The tables in `grammar` are generated from the grammar.
//! Its `matchers` decide where each pattern matches.
//!
//! The rules left open at the end of a line are frames of the highlighter's
//! interned state.
//! Messages, enums, services, block comments and option values are such rules.
//! Shiki's Markdown grammar has no Protobuf fence.
//! No language therefore embeds Protobuf yet.
mod engine;
mod grammar;
mod matchers;
mod scope;

use engine::Proto;
pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Line};
use crate::limits::Finished;
use crate::rules;

/// Protobuf's buffers in the highlighter's scratch.
/// The first Protobuf line allocates them.
fn buffers(scratch: &mut crate::scratch::Scratch) -> &mut Scratch {
    scratch.proto.get_or_insert_with(Box::default)
}

/// The state at the start of a Protobuf document whose scopes sit on the stack
/// `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    rules::start::<Proto>(arena, base)
}

/// The state at the start of the Protobuf lines of a fenced code block.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    rules::start_embedded::<Proto>(arena, base)
}

/// Lexes one line under the crate's lexer contract.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    rules::lex::<Proto>(cx, line, state, buffers)
}

#[cfg(test)]
mod tests;
