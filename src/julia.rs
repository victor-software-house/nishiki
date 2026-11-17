//! Julia, with the scopes of the `julia` grammar that Shiki bundles.
//! The grammar comes from `JuliaEditorSupport`'s atom-language-julia.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers over
//! bytes, in the `engine` module.
//! The tables in `grammar` are generated from the grammar.
//! Its `matchers` decide where each pattern matches.
//! The rules left open at the end of a line are frames of the highlighter's
//! interned state.
//! Strings, comments, brackets and `for` heads are such rules.
//!
//! The body of a `py"""`, `js"""`, `sql"""` or `R"""` string macro is lexed as
//! Python, JavaScript, SQL or R (see [`rules::lex_embedding`]).

mod chars;
mod classes;
pub(crate) mod engine;
mod grammar;
mod matchers;
mod scope;

use engine::Julia;
pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Halt, Line};
use crate::limits::Finished;
use crate::rules;

/// Julia's buffers in the highlighter's scratch.
/// The first Julia line allocates them.
fn buffers(scratch: &mut crate::scratch::Scratch) -> &mut Scratch {
    scratch.julia.get_or_insert_with(Box::default)
}

/// The state at the start of a Julia document whose scopes sit on the stack
/// `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    rules::start::<Julia>(arena, base)
}

/// The state at the start of the Julia lines of a fenced code block.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    rules::start_embedded::<Julia>(arena, base)
}

/// Lexes one line under the crate's lexer contract.
/// A string macro named for a language hands its body to that language.
/// The body goes back to Julia at the macro's closing quotes ([`Close`]) and
/// at an interpolation ([`interpolation`]).
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    rules::lex_embedding::<Julia>(cx, line, state, buffers)
}

/// A string macro named for a language takes the line back at its end, which
/// the grammar tries before the embedded grammar.
/// It also takes it back at an interpolation, which the grammar lists after
/// the embedded grammar.
impl rules::Embeds for Julia {
    type End = Close;

    fn embed_halts(embedding: &rules::Embedding, _: &Arena) -> (Self::End, Option<rules::FnHalt>) {
        let close = if matches!(embedding.language, crate::Language::Python) {
            Close::Python
        } else {
            Close::Quotes
        };
        (close, Some(interpolation))
    }
}

/// The end of a string macro that embeds a language.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Close {
    /// `"""`, the end of `js"""`, `sql"""` and `R"""`.
    Quotes,
    /// `[\s\w]*"""`, the end of `py"""`.
    /// It starts at the run of spaces and word characters before the quotes.
    Python,
}

impl Halt for Close {
    fn first(&self, s: &[u8], from: usize, to: usize) -> Option<usize> {
        let quotes = from + memchr::memmem::find(s.get(from..)?, b"\"\"\"")?;
        let start = match self {
            Close::Quotes => quotes,
            Close::Python => chars::space_or_word_start(s, from, quotes),
        };
        (start <= to).then_some(start)
    }
}

/// Where an interpolation begins: `$(`, or `$` and an identifier's first
/// character.
/// A string macro that embeds a language lists it after the embedded grammar.
fn interpolation(s: &[u8], p: usize) -> bool {
    s.get(p) == Some(&b'$')
        && (s.get(p + 1) == Some(&b'(') || chars::interpolated_start(s, p + 1).is_some())
}

#[cfg(test)]
mod tests;
