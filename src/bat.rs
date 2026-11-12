//! Batch, with the scopes of the `bat` grammar that Shiki bundles.
//! The grammar comes from mmims/language-batchfile.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers over
//! bytes, in the `engine` module.
//! The tables in `grammar` are generated from the grammar, and its
//! `matchers` decide where each pattern matches.
//! Rules left open at the end of a line are frames of the highlighter's
//! interned state.
//! Shiki's Markdown grammar embeds Batch in `bat` and `batch` fences.

mod engine;
mod grammar;
mod matchers;
mod scope;

use engine::Bat;
pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Line};
use crate::limits::Finished;
use crate::rules;

/// Batch's buffers in the highlighter's scratch.
/// The first Batch line allocates them.
fn buffers(scratch: &mut crate::scratch::Scratch) -> &mut Scratch {
    scratch.bat.get_or_insert_with(Box::default)
}

/// The state at the start of a Batch document whose scopes sit on the stack
/// `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    rules::start::<Bat>(arena, base)
}

/// The state at the start of the Batch lines of a fenced code block.
#[cfg(feature = "_start-embedded")]
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    rules::start_embedded::<Bat>(arena, base)
}

/// Lexes one line under the crate's lexer contract.
///
/// The matchers learn whether a Markdown fence embeds the line.
/// Shiki applies a grammar's injections only where it is the document's
/// grammar, so a fence's `for` lines take no injected parameters.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    buffers(cx.scratch).set_kept(line.embedded);
    rules::lex::<Bat>(cx, line, state, buffers)
}

#[cfg(test)]
mod tests;
