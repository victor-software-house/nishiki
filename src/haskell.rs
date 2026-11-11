//! Haskell, with the scopes of the `haskell` grammar that Shiki bundles.
//! The grammar comes from octref/language-haskell.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers over
//! bytes, in the `engine` module.
//! The tables in `grammar` are generated from the grammar.
//! Its `matchers` decide where each pattern matches.
//!
//! The rules left open at the end of a line are frames of the highlighter's
//! interned state.
//! A declaration that runs while the lines after it are indented further is
//! such a rule.
//! Shiki's Markdown grammar has no Haskell fence, so no language embeds
//! Haskell.

mod engine;
mod grammar;
mod matchers;
mod nested;
mod scope;

use engine::Haskell;
pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Line};
use crate::limits::Finished;
use crate::rules;

/// Haskell's buffers in the highlighter's scratch.
/// The first Haskell line allocates them.
fn buffers(scratch: &mut crate::scratch::Scratch) -> &mut Scratch {
    scratch.haskell.get_or_insert_with(Box::default)
}

/// The state at the start of a Haskell document whose scopes sit on the
/// stack `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    rules::start::<Haskell>(arena, base)
}

/// The state at the start of the Haskell lines of a fenced code block.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    rules::start_embedded::<Haskell>(arena, base)
}

/// Lexes one line under the crate's lexer contract.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    rules::lex::<Haskell>(cx, line, state, buffers)
}

#[cfg(test)]
mod tests;
