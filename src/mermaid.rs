//! Mermaid, with the scopes of the `mermaid` grammar that Shiki bundles.
//! The grammar comes from Brian Pruitt-Goddard's
//! vscode-mermaid-syntax-highlight.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers over
//! bytes, in the `engine` module.
//! The tables in `grammar` are generated from the grammar.
//! The `matchers` decide where each pattern matches.
//! The rules left open at the end of a line are frames of the highlighter's
//! interned state.
//! Each diagram is such a rule, and so are a class's body and a flowchart
//! node's label.
//! Shiki's Markdown grammar has no Mermaid fence.
//! No language therefore embeds Mermaid.

mod engine;
mod grammar;
mod matchers;
mod scope;

use engine::Mermaid;
pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Line};
use crate::limits::Finished;
use crate::rules;

/// Mermaid's buffers in the highlighter's scratch.
/// The first Mermaid line allocates them.
fn buffers(scratch: &mut crate::scratch::Scratch) -> &mut Scratch {
    scratch.mermaid.get_or_insert_with(Box::default)
}

/// The state at the start of a Mermaid document whose scopes sit on the stack
/// `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    rules::start::<Mermaid>(arena, base)
}

/// The state at the start of the Mermaid lines of a fenced code block.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    rules::start_embedded::<Mermaid>(arena, base)
}

/// Lexes one line under the crate's lexer contract.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    rules::lex::<Mermaid>(cx, line, state, buffers)
}

#[cfg(test)]
mod tests;
