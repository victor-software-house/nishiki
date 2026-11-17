//! C#, with the scopes of the `csharp` grammar that Shiki bundles, which
//! comes from VS Code and is converted from dotnet/csharp-tmLanguage.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers
//! over bytes, in the `engine` module.
//! The tables in `grammar` are generated from the grammar.
//! Its `matchers` decide where each pattern matches.
//!
//! The rules left open at the end of a line are frames of the highlighter's
//! interned state.
//! Blocks, parentheses, comments, strings and interpolations are such rules.

pub(crate) mod engine;
mod grammar;
mod limits;
mod matchers;
mod scope;

use engine::CSharp;
pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Line};
use crate::limits::Finished;
use crate::rules;

/// The state at the start of a C# document whose scopes sit on the stack
/// `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    rules::start::<CSharp>(arena, base)
}

/// The state at the start of the C# lines of a fenced code block.
#[cfg(feature = "_start-embedded")]
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    rules::start_embedded::<CSharp>(arena, base)
}

/// Lexes one line under the crate's lexer contract.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    rules::lex::<CSharp>(cx, line, state, |scratch| &mut scratch.csharp)
}

#[cfg(test)]
mod tests;
