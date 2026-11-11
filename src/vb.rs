//! Visual Basic .NET, with the scopes of the `vb` grammar that Shiki bundles.
//! The grammar comes from textmate/asp.vb.net.tmbundle, through VS Code.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers over
//! bytes, in the `engine` module.
//! The tables in `grammar` are generated from the grammar.
//! Its `matchers` decide where each pattern matches.
//! The rules left open at the end of a line are frames of the highlighter's
//! interned state.
//! Strings and brackets are such rules.
//! Markdown's `vb` fence embeds it.

mod engine;
mod grammar;
mod matchers;
mod scope;

pub(crate) use engine::Scratch;
use engine::Vb;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Line};
use crate::limits::Finished;
use crate::rules;

/// Visual Basic's buffers in the highlighter's scratch.
/// The first Visual Basic line allocates them.
fn buffers(scratch: &mut crate::scratch::Scratch) -> &mut Scratch {
    scratch.vb.get_or_insert_with(Box::default)
}

/// The state at the start of a Visual Basic document whose scopes sit on the
/// stack `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    rules::start::<Vb>(arena, base)
}

/// The state at the start of the Visual Basic lines of a fenced code block.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    rules::start_embedded::<Vb>(arena, base)
}

/// Lexes one line under the crate's lexer contract.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    rules::lex::<Vb>(cx, line, state, buffers)
}

#[cfg(test)]
mod tests;
