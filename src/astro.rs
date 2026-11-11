//! Astro, with the scopes of the `astro` grammar that Shiki bundles.
//! The grammar comes from withastro/language-tools.
//!
//! The lexer runs the grammar's begin/end rules on the shared rule engine
//! ([`crate::rules`]).
//! The tables in `grammar` are generated from the grammar.
//! Its `matchers` decide where each pattern matches.
//! A rule left open at the end of a line is a frame of the highlighter's
//! interned state.
//!
//! A rule that includes another grammar embeds that language:
//!
//! - the frontmatter goes to the TypeScript lexer;
//! - expressions and backtick attribute values go to the TSX lexer, some
//!   starting at an entry of its grammar, such as an object literal;
//! - event handler values go to the JavaScript lexer;
//! - `<script>` bodies go to the JavaScript, TypeScript or JSON lexer;
//! - `<style>` bodies go to the CSS, PostCSS, SCSS, Less or Sass lexer.
//!
//! The embedded language's state is the carried value of its rule's frame.
//! Each embedding rule sets where it takes the line back
//! (`engine::End`).
mod engine;
mod grammar;
mod matchers;
mod scope;

use engine::Astro;
pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Line};
use crate::limits::Finished;

/// Astro's buffers in the highlighter's scratch.
/// The first Astro line allocates them.
fn buffers(scratch: &mut crate::scratch::Scratch) -> &mut Scratch {
    scratch.astro.get_or_insert_with(Box::default)
}

/// The state at the start of an Astro document whose scopes sit on the stack
/// `base`: one rule with `source.astro` on top of it.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    crate::rules::start::<Astro>(arena, base)
}

/// The state at the start of Astro whose text sits on the stack `base`, with
/// no `source.astro` of its own.
/// No Markdown fence embeds Astro, because Shiki's Markdown grammar has no
/// rule for it.
/// The function completes the contract every language keeps.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    crate::rules::start_embedded::<Astro>(arena, base)
}

/// Lexes `line` from `state` under the crate's one lexer contract
/// ([`Language::lex`](crate::Language)), and returns the state after it.
/// Token ends count from the start of the whole line.
/// An embedded language's text goes to that language's lexer, up to where
/// its rule takes the line back ([`crate::rules::lex_embedding`]).
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    crate::rules::lex_embedding::<Astro>(cx, line, state, buffers)
}

#[cfg(test)]
mod tests;
