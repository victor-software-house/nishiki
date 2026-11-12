//! PostCSS, with the scopes of the `postcss` grammar that Shiki bundles.
//! The grammar comes from vetur, which took it from hudochenkov's
//! Syntax-highlighting-for-PostCSS.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers over
//! bytes, in the `engine` module.
//! The tables in `grammar` are generated from the grammar.
//! Its `matchers` decide where each pattern matches.
//!
//! The rules left open at the end of a line are frames of the highlighter's
//! interned state.
//! Comments, strings, interpolations and property lists are such rules.
//! Shiki's Markdown grammar has no PostCSS fence.
//! Svelte embeds PostCSS for `<style lang="postcss">`, through [`lex_until`].
mod engine;
mod grammar;
mod matchers;
mod scope;

use engine::PostCss;
pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Halt, Line, Stopped};
use crate::limits::Finished;
use crate::rules;

/// PostCSS's buffers in the highlighter's scratch.
/// The first PostCSS line allocates them.
fn buffers(scratch: &mut crate::scratch::Scratch) -> &mut Scratch {
    scratch.postcss.get_or_insert_with(Box::default)
}

/// The state at the start of a PostCSS document whose scopes sit on the stack
/// `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    rules::start::<PostCss>(arena, base)
}

/// The state at the start of the PostCSS lines of a fenced code block.
#[cfg(feature = "_start-embedded")]
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    rules::start_embedded::<PostCss>(arena, base)
}

/// Lexes one line under the crate's lexer contract.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    rules::lex::<PostCss>(cx, line, state, buffers)
}

/// Lexes `line` like [`lex`], for PostCSS that another language embeds, up to
/// where `end` or `after` halts it (see [`rules::lex_until`]).
#[inline]
pub(crate) fn lex_until(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    end: &dyn Halt,
    after: Option<&dyn Halt>,
) -> (Finished, Option<Stopped>) {
    rules::lex_until::<PostCss>(cx, line, state, buffers, end, after)
}

#[cfg(test)]
mod tests;
