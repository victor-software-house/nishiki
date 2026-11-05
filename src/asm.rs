//! x86 and x86-64 assembly, with the scopes of the `asm` grammar that Shiki
//! bundles.
//! The grammar comes from 13xforever/x86_64-assembly-vscode.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers over
//! bytes, in the `engine` module.
//! The tables in `grammar` and `words` are generated from the grammar.
//! The `matchers` decide where each pattern matches.
//! The rules left open at the end of a line are frames of the highlighter's
//! interned state.
//! Block comments, `#if 0` blocks, strings and continued preprocessor lines
//! are such rules.
//! Shiki's Markdown grammar has no assembly fence.

mod engine;
mod grammar;
mod matchers;
mod scope;
mod words;

use engine::Asm;
pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Line};
use crate::limits::Finished;
use crate::rules;

/// Assembly's buffers in the highlighter's scratch.
/// The first Assembly line allocates them.
fn buffers(scratch: &mut crate::scratch::Scratch) -> &mut Scratch {
    scratch.asm.get_or_insert_with(Box::default)
}

/// The state at the start of an Assembly document whose scopes sit on the
/// stack `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    rules::start::<Asm>(arena, base)
}

/// The state at the start of the Assembly lines of a fenced code block.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    rules::start_embedded::<Asm>(arena, base)
}

/// Lexes one line under the crate's lexer contract.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    rules::lex::<Asm>(cx, line, state, buffers)
}

#[cfg(test)]
mod tests;
