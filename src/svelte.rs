//! Svelte, with the scopes of the `svelte` grammar that Shiki bundles.
//! The grammar comes from sveltejs/language-tools.
//!
//! The lexer runs the grammar's begin/end rules on the shared rule engine
//! ([`crate::rules`]).
//! The tables in `grammar` are generated from the grammar.
//! The `matchers` decide where each pattern matches.
//! A rule left open at the end of a line is a frame of the highlighter's
//! interned state.
//!
//! A rule that includes another grammar embeds that language.
//! Expressions go to the TypeScript lexer, some of them at an entry of
//! TypeScript's grammar, such as an object literal.
//! `<script>` bodies go to the JavaScript or TypeScript lexer, and `<style>`
//! bodies to the CSS lexer.
//!
//! The embedded language's state is the payload of its rule's frame.
//! Each embedding rule says where it takes the line back
//! (`engine::End`).
mod engine;
mod grammar;
mod matchers;
mod scope;

pub(crate) use engine::Scratch;
use engine::Svelte;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Line};
use crate::limits::Finished;

/// Svelte's buffers in the highlighter's scratch.
/// The first Svelte line allocates them.
fn buffers(scratch: &mut crate::scratch::Scratch) -> &mut Scratch {
    scratch.svelte.get_or_insert_with(Box::default)
}

/// The scope that a Svelte document's stack starts with.
#[cfg(feature = "lang-html")]
pub(crate) const DOCUMENT: crate::scope::Scope = scope::S::Source.scope();

/// The state at the start of a Svelte document whose scopes sit on the stack
/// `base`: one rule with `source.svelte` on top of it.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    crate::rules::start::<Svelte>(arena, base)
}

/// The state at the start of a Svelte body whose text sits on the stack
/// `base`, with no `source.svelte` of its own.
/// No Markdown fence embeds Svelte, because Shiki's Markdown grammar has no
/// rule for it.
/// The function completes the contract every language keeps.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    crate::rules::start_embedded::<Svelte>(arena, base)
}

/// Lexes `line` from `state` under the crate's one lexer contract
/// ([`Language::lex`](crate::Language)), and returns the state after it.
/// Token ends count from the start of the whole line.
/// An embedded language's text goes to that language's lexer, up to where
/// its rule takes the line back ([`crate::rules::lex_embedding`]).
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    crate::rules::lex_embedding::<Svelte>(cx, line, state, buffers)
}

#[cfg(test)]
mod tests;
