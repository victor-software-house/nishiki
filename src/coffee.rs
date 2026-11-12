//! CoffeeScript, with the scopes of the `coffee` grammar that Shiki bundles.
//! The grammar comes from atom/language-coffee-script, through VS Code.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers over
//! bytes, in the `engine` module.
//! The tables in `grammar` are generated from the grammar, and its
//! `matchers` decide where each pattern matches.
//! Rules left open at the end of a line are frames of the highlighter's
//! interned state.
//! Heredocs, block comments, brackets and function heads are such rules.
//!
//! The JavaScript between backticks is lexed by the JavaScript lexer, from a
//! fresh start, on the line cut at the last backtick
//! ([`rules::Embeds::lex_nested`]).
//! A Markdown fence named `coffee` embeds CoffeeScript.
//! Vue's `<script lang="coffee">` and HAML's `:coffee` filter embed it through
//! [`lex_until`].

pub(crate) mod engine;
mod grammar;
mod matchers;
mod scope;
#[cfg(any(feature = "lang-svelte", feature = "_textmate"))]
mod svelte;
#[rustfmt::skip]
mod words;

use engine::Coffee;
#[cfg(any(feature = "lang-svelte", feature = "_textmate"))]
pub(crate) use engine::SVELTE;
pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Halt, Line, Stopped};
use crate::limits::Finished;
use crate::rules;

/// CoffeeScript's buffers in the highlighter's scratch.
/// The first CoffeeScript line allocates them.
fn buffers(scratch: &mut crate::scratch::Scratch) -> &mut Scratch {
    scratch.coffee.get_or_insert_with(Box::default)
}

/// The state at the start of a CoffeeScript document whose scopes sit on the
/// stack `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    rules::start::<Coffee>(arena, base)
}

/// The state at the start of the CoffeeScript lines of a fenced code block.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    rules::start_embedded::<Coffee>(arena, base)
}

/// Lexes one line under the crate's lexer contract.
/// A script string hands the text between its backticks to JavaScript.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    rules::lex_embedding::<Coffee>(cx, line, state, buffers)
}

/// Lexes `line` like [`lex`], for CoffeeScript that another language embeds.
/// The line halts where `end` holds before CoffeeScript's patterns are tried,
/// and also where `after` holds and the patterns fail.
/// Both apply only while no CoffeeScript rule is open.
/// The second value is where the line halted.
pub(crate) fn lex_until(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    end: &dyn Halt,
    after: Option<&dyn Halt>,
) -> (Finished, Option<Stopped>) {
    rules::lex_embedding_until::<Coffee>(cx, line, state, buffers, (end, after))
}

/// Lexes `line` like [`lex_until`], for the CoffeeScript that Svelte embeds.
/// Svelte's grammar injects its `$` prefixes there, in every rule
/// ([`SVELTE`]).
#[cfg(any(feature = "lang-svelte", feature = "_textmate"))]
pub(crate) fn lex_until_svelte(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    end: &dyn Halt,
    after: Option<&dyn Halt>,
) -> (Finished, Option<Stopped>) {
    rules::lex_embedding_until::<Coffee<SVELTE>>(cx, line, state, svelte_buffers, (end, after))
}

/// The buffers of the CoffeeScript that Svelte embeds.
#[cfg(any(feature = "lang-svelte", feature = "_textmate"))]
fn svelte_buffers(scratch: &mut crate::scratch::Scratch) -> &mut Scratch<SVELTE> {
    scratch.coffee_svelte.get_or_insert_with(Box::default)
}

/// No rule of the grammar embeds a language.
/// Its one embedded language is the JavaScript of a script string's middle
/// group, which [`rules::Embeds::lex_nested`] lexes.
impl rules::Embeds for Coffee {
    type End = rules::FnHalt;
    const HALTS: bool = true;

    fn embed_halts(_: &rules::Embedding, _: &Arena) -> (Self::End, Option<rules::FnHalt>) {
        (no_end, None)
    }

    /// JavaScript lexes the middle group of `` (`)(.*)(`) `` from its start, as
    /// an embedded body on the group's stack.
    /// The line ends at the group's end, the last backtick, as vscode-textmate
    /// lexes a group whose patterns include another grammar.
    fn lex_nested(
        language: crate::Language,
        cx: &mut Context<'_>,
        line: Line<'_>,
        base: u32,
    ) -> Option<Finished> {
        let state = language.start_embedded(cx.arena, base);
        Some(language.lex_embedded(cx, line, state))
    }
}

/// The CoffeeScript that Svelte embeds embeds the same language.
#[cfg(any(feature = "lang-svelte", feature = "_textmate"))]
impl rules::Embeds for Coffee<SVELTE> {
    type End = rules::FnHalt;
    const HALTS: bool = true;

    fn embed_halts(_: &rules::Embedding, _: &Arena) -> (Self::End, Option<rules::FnHalt>) {
        (no_end, None)
    }

    /// The script string's JavaScript takes Svelte's `$` prefixes too, since
    /// its scope, `source.js.embedded.coffee`, is a `source.js` scope.
    fn lex_nested(
        language: crate::Language,
        cx: &mut Context<'_>,
        line: Line<'_>,
        base: u32,
    ) -> Option<Finished> {
        let state = language.start_embedded(cx.arena, base);
        let dialect = crate::typescript::Dialect::JavaScript;
        let (finished, _) =
            crate::typescript::lex_until_svelte(cx, dialect, line, state, &no_end, None);
        Some(finished)
    }
}

/// The end of an embedding rule, which the grammar has none of.
fn no_end(_: &[u8], _: usize) -> bool {
    false
}

#[cfg(test)]
mod tests;
