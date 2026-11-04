//! C#, with the scopes of the `csharp` grammar that Shiki bundles, which
//! comes from VS Code and is converted from dotnet/csharp-tmLanguage.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers
//! over bytes (see the `engine` module): the tables in `grammar` are
//! generated from the grammar, and the `matchers` decide where each pattern
//! matches. The rules left open at the end of a line (blocks, parentheses,
//! comments, strings, interpolations) are frames of the highlighter's interned
//! state.

mod engine;
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
