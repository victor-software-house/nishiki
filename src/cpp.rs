//! C++, with the scopes of the `cpp` grammar that Shiki bundles, which
//! comes from the C and C++ extension of VS Code.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers
//! over bytes (see the `engine` module): the tables in `grammar` are
//! generated from the grammar, and the `matchers` decide where each regular
//! expression matches. The rules left open at the end of a line are frames of
//! the highlighter's interned state.

mod engine;
mod few;
mod grammar;
mod limits;
mod matchers;
mod scope;
mod types;

use engine::Cpp;
pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Line};
use crate::rules;

/// The state at the start of a C++ document whose scopes sit on the stack
/// `base`: one rule with `source.cpp` on top of it.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    rules::start::<Cpp>(arena, base)
}

/// The state at the start of the C++ lines of a fenced code block, whose
/// text sits on the stack `base`. Shiki's embedding keeps `source.cpp` on the
/// stack, as for a document of its own, unlike C's.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    rules::start::<Cpp>(arena, base)
}

/// Lexes one line under the crate's lexer contract.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> u32 {
    rules::lex::<Cpp>(cx, line, state, |scratch| &mut scratch.cpp)
}

#[cfg(test)]
mod tests;
