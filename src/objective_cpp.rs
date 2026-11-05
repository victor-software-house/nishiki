//! Objective-C++, with the scopes of the `objective-cpp` grammar that Shiki
//! bundles.
//! The grammar comes from VS Code, which took it from jeff-hykin's
//! better-objcpp-syntax.
//!
//! The lexer runs the grammar's begin/end rules on the shared rule engine, in
//! the `engine` module.
//! The tables in `grammar` are generated from the grammar.
//! The grammar repeats every expression of Objective-C's, and Objective-C's
//! matchers answer those.
//! The `matchers` here answer the rest, which come from the grammar's C++
//! rules.
//! The rules left open at the end of a line are frames of the highlighter's
//! interned state.
//! Shiki's Markdown grammar has no Objective-C++ fence.

mod engine;
mod grammar;
mod matchers;
mod scope;

use engine::ObjectiveCpp;
pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Halt, Line};
use crate::limits::Finished;
use crate::rules;

/// Objective-C++'s buffers in the highlighter's scratch.
/// The first Objective-C++ line allocates them.
fn buffers(scratch: &mut crate::scratch::Scratch) -> &mut Scratch {
    scratch.objective_cpp.get_or_insert_with(Box::default)
}

/// The state at the start of an Objective-C++ document whose scopes sit on
/// the stack `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    rules::start::<ObjectiveCpp>(arena, base)
}

/// The state at the start of the Objective-C++ lines of a fenced code block.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    rules::start_embedded::<ObjectiveCpp>(arena, base)
}

/// Lexes one line under the crate's lexer contract.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    rules::lex::<ObjectiveCpp>(cx, line, state, buffers)
}

/// Lexes `line` like [`lex`], for Objective-C++ that another language
/// embeds, up to where `end` or `after` halts it (see [`rules::lex_until`]).
#[inline]
pub(crate) fn lex_until(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    end: &dyn Halt,
    after: Option<&dyn Halt>,
) -> (Finished, Option<usize>) {
    rules::lex_until::<ObjectiveCpp>(cx, line, state, buffers, end, after)
}

#[cfg(test)]
mod tests;
