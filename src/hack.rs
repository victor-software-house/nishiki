//! Hack, with the scopes of the `hack` grammar that Shiki bundles.
//! The grammar comes from slackhq/vscode-hack, converted from
//! hhvm/atom-ide-hack, and descends from PHP's.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers over
//! bytes, in the `engine` module.
//! The tables in `grammar` and `words` are generated from the grammar.
//! The `matchers` decide where each pattern matches.
//! Where an expression is PHP's word for word, the matcher is the one PHP
//! uses, in [`crate::php_family`].
//! The rules left open at the end of a line are frames of the highlighter's
//! interned state.
//!
//! The grammar's root lists HTML first and Hack's patterns after it.
//! A document starts in the `Document` rule, above the root, which holds that
//! list.
//! HTML lexes the line, and halts where one of Hack's patterns starts and none
//! of HTML's own match.
//! Strings that start with a SQL keyword hand their text to the SQL lexer.

mod engine;
mod grammar;
mod matchers;
mod nested;
mod scope;
mod words;

use engine::Hack;
pub(crate) use engine::Scratch;
use grammar::Rule;
pub(crate) use scope::NAMES;

use crate::arena::{Arena, ROOT};
use crate::language::{Context, Line};
use crate::limits::Finished;
use crate::rules;

/// Hack's buffers in the highlighter's scratch.
/// The first Hack line allocates them.
fn buffers(scratch: &mut crate::scratch::Scratch) -> &mut Scratch {
    scratch.hack.get_or_insert_with(Box::default)
}

/// The state at the start of a Hack document whose scopes sit on the stack
/// `base`: the root rule, with the `Document` rule above it.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    let root = rules::start::<Hack>(arena, base);
    document(arena, root)
}

/// The state at the start of Hack that another language embeds.
/// No Markdown fence embeds Hack, because Shiki's Markdown grammar has no rule
/// for it.
/// This completes the contract every language keeps.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    let root = rules::start_embedded::<Hack>(arena, base);
    document(arena, root)
}

/// The `Document` rule opened above the state `root`, with the root's scopes.
fn document(arena: &mut Arena, root: u32) -> u32 {
    let stack = arena.frame(root).stack;
    arena
        .push(root, Rule::Document.code(), 0, stack)
        .unwrap_or(ROOT)
}

/// Lexes one line under the crate's lexer contract.
/// A rule that embeds a language hands its text to that language up to where
/// Hack takes it back. See [`rules::lex_embedding`].
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    rules::lex_embedding::<Hack>(cx, line, state, buffers)
}

#[cfg(test)]
mod tests;
