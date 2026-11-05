//! CoffeeScript, with the scopes of the `coffee` grammar that Shiki bundles.
//! The grammar comes from atom/language-coffee-script, through VS Code.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers over
//! bytes, in the `engine` module.
//! The tables in `grammar` are generated from the grammar.
//! The `matchers` decide where each pattern matches.
//! The rules left open at the end of a line are frames of the highlighter's
//! interned state.
//! Heredocs, block comments, brackets and function heads are such rules.
//!
//! The JavaScript between backticks is lexed by the JavaScript lexer, from a
//! fresh start, on the line cut at the last backtick
//! ([`rules::Embeds::lex_nested`]).
//! A Markdown fence named `coffee` embeds CoffeeScript.
//! Vue's `<script lang="coffee">` and HAML's `:coffee` filter embed it through
//! [`lex_until`].

mod engine;
mod grammar;
mod matchers;
mod scope;
mod words;

use engine::Coffee;
pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Halt, Line};
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
/// The line halts where `end` holds before CoffeeScript's patterns are tried.
/// It also halts where `after` holds and the patterns fail.
/// Both apply only while no CoffeeScript rule is open.
/// The second value is where the line halted.
pub(crate) fn lex_until(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    end: &dyn Halt,
    after: Option<&dyn Halt>,
) -> (Finished, Option<usize>) {
    rules::lex_embedding_until::<Coffee>(cx, line, state, buffers, (end, after))
}

/// No rule of the grammar embeds a language.
/// Its one embedded language is the JavaScript of a script string's middle
/// group, which [`rules::Embeds::lex_nested`] lexes.
impl rules::Embeds for Coffee {
    type End = rules::FnHalt;

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

/// The end of an embedding rule, which the grammar has none of.
fn no_end(_: &[u8], _: usize) -> bool {
    false
}

#[cfg(test)]
mod tests;
