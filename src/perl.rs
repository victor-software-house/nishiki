//! Perl, with the scopes of the `perl` grammar that Shiki bundles.
//! The grammar comes from textmate/perl.tmbundle through VS Code.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers over
//! bytes, in the `engine` module.
//! The tables in `grammar` and `words` are generated from the grammar.
//! Its `matchers` decide where each pattern matches.
//! The rules left open at the end of a line are frames of the highlighter's
//! interned state.
//!
//! A heredoc tagged HTML, XML, CSS, JAVASCRIPT or SQL hands each of its lines
//! to that language's lexer, which halts at the line feed.
//! So does a POD `=begin html` block, up to its `=end html` or `=cut`.

pub(crate) mod engine;
mod grammar;
mod matchers;
mod nested;
mod scope;
mod words;

use engine::Perl;
pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Halt, Line, Stopped};
use crate::limits::Finished;
use crate::rules;

/// Perl's buffers in the highlighter's scratch.
/// The first Perl line allocates them.
fn buffers(scratch: &mut crate::scratch::Scratch) -> &mut Scratch {
    scratch.perl.get_or_insert_with(Box::default)
}

/// The state at the start of a Perl document whose scopes sit on the stack
/// `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    rules::start::<Perl>(arena, base)
}

/// The state at the start of Perl that another language embeds, such as the
/// lines of a Markdown fence.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    rules::start_embedded::<Perl>(arena, base)
}

/// Lexes one line under the crate's lexer contract.
/// A rule that embeds a language hands its text to that language up to where
/// Perl takes it back. See [`rules::lex_embedding`].
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    rules::lex_embedding::<Perl>(cx, line, state, buffers)
}

/// Lexes `line` like [`lex`], for Perl that another language embeds up to a
/// halt (see [`rules::lex_until`]).
#[inline]
pub(crate) fn lex_until(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    end: &dyn Halt,
    after: Option<&dyn Halt>,
) -> (Finished, Option<Stopped>) {
    rules::lex_embedding_until::<Perl>(cx, line, state, buffers, (end, after))
}

#[cfg(test)]
mod tests;
