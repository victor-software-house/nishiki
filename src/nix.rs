//! Nix, with the scopes of the `nix` grammar that Shiki bundles.
//! The grammar comes from `nix-community/vscode-nix-ide`.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers over
//! bytes, in the `engine` module.
//! The tables in `grammar` are generated from the grammar.
//! Its `matchers` decide where each pattern matches.
//!
//! The rules left open at the end of a line are frames of the highlighter's
//! interned state.
//! Attribute sets, function heads, lists, strings and comments are such rules.
//! Shiki's Markdown grammar has no Nix fence.
//! No language therefore embeds Nix yet.

pub(crate) mod engine;
mod grammar;
mod matchers;
mod scope;

use engine::Nix;
pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Line};
use crate::limits::Finished;
use crate::rules;

/// Nix's buffers in the highlighter's scratch.
/// The first Nix line allocates them.
fn buffers(scratch: &mut crate::scratch::Scratch) -> &mut Scratch {
    scratch.nix.get_or_insert_with(Box::default)
}

/// The state at the start of a Nix document whose scopes sit on the stack
/// `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    rules::start::<Nix>(arena, base)
}

/// The state at the start of the Nix lines of a fenced code block.
#[cfg(feature = "_start-embedded")]
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    rules::start_embedded::<Nix>(arena, base)
}

/// Lexes one line under the crate's lexer contract.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    rules::lex::<Nix>(cx, line, state, buffers)
}

#[cfg(test)]
mod tests;
