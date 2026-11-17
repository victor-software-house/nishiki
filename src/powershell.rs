//! PowerShell, with the scopes of the `powershell` grammar that Shiki bundles.
//! The grammar comes from PowerShell's `EditorSyntax` repository.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers over
//! bytes, in the `engine` module.
//! The tables in `grammar` are generated from the grammar.
//! Its `matchers` decide where each pattern matches.
//! The rules left open at the end of a line are frames of the highlighter's
//! interned state.
//! Comments, strings, groups and script blocks are such rules.

pub(crate) mod engine;
mod grammar;
mod matchers;
mod scope;

use engine::PowerShell;
pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Line};
use crate::limits::Finished;
use crate::rules;

/// PowerShell's buffers in the highlighter's scratch.
/// The first PowerShell line allocates them.
fn buffers(scratch: &mut crate::scratch::Scratch) -> &mut Scratch {
    scratch.powershell.get_or_insert_with(Box::default)
}

/// The state at the start of a PowerShell document whose scopes sit on the
/// stack `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    rules::start::<PowerShell>(arena, base)
}

/// The state at the start of the PowerShell lines of a fenced code block.
#[cfg(feature = "_start-embedded")]
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    rules::start_embedded::<PowerShell>(arena, base)
}

/// Lexes one line under the crate's lexer contract.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    rules::lex::<PowerShell>(cx, line, state, buffers)
}

#[cfg(test)]
mod tests;
