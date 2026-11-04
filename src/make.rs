//! Makefile, with the scopes of the `make` grammar that Shiki bundles.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers
//! over bytes (see the `engine` module). The rules left open at the end of a
//! line (a definition, a conditional, a recipe or a target continued by a
//! backslash, an unclosed variable reference) are frames of the highlighter's
//! interned state.
//!
//! It reproduces Shiki's output, quirks included: `$` at the end of a recipe
//! line is not a variable, a `:` anywhere in a line that is not an assignment
//! makes it a target, and the `endef` of a definition has the scope of an
//! `override`.
//!
//! An empty line leaves the state as it is, because Shiki does not tokenize
//! empty lines.

mod engine;
mod matchers;
mod rules;
mod scope;
mod state;

use engine::Make;
pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Line};
use crate::limits::Finished;

/// The state at the start of a Makefile whose scopes sit on the stack
/// `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    crate::rules::start::<Make>(arena, base)
}

/// The state at the start of the Makefile lines of a fenced code block.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    crate::rules::start_embedded::<Make>(arena, base)
}

/// Lexes one line under the crate's lexer contract.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    crate::rules::lex::<Make>(cx, line, state, |scratch| &mut scratch.make)
}

#[cfg(test)]
mod tests;
