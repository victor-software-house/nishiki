//! HAML, with the scopes of the `haml` grammar that Shiki bundles.
//! The grammar comes from karuna/haml-vscode.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers over
//! bytes, in the `engine` module.
//! The tables in `grammar` are generated from the grammar, and its
//! `matchers` decide where each pattern matches.
//! Rules left open at the end of a line are frames of the highlighter's
//! interned state.
//! Filters, comment blocks, attribute hashes and Ruby lines are such rules.
//!
//! The grammar includes the whole Ruby, JavaScript, CSS, Sass, CoffeeScript
//! and Markdown grammars.
//! Each such include hands its region to nishiki's lexer for that language
//! ([`rules::lex_embedding`]).
//! Ruby's `<<~HAML` heredoc embeds HAML through [`lex_until`].
//! Markdown cannot embed it, because Shiki's Markdown grammar has no HAML
//! fence.

mod engine;
mod grammar;
mod matchers;
mod scope;

use engine::Haml;
pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Halt, Line, Stopped};
use crate::limits::Finished;
use crate::rules;

/// HAML's buffers in the highlighter's scratch.
/// The first HAML line allocates them.
fn buffers(scratch: &mut crate::scratch::Scratch) -> &mut Scratch {
    scratch.haml.get_or_insert_with(Box::default)
}

/// The state at the start of a HAML document whose scopes sit on the stack
/// `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    rules::start::<Haml>(arena, base)
}

/// The state at the start of the HAML lines of an embedded body.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    rules::start_embedded::<Haml>(arena, base)
}

/// Lexes one line under the crate's lexer contract.
/// A filter, a Ruby line, an attribute hash and an interpolation hand their
/// text to the languages they embed.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    rules::lex_embedding::<Haml>(cx, line, state, buffers)
}

/// Lexes `line` like [`lex`], for HAML that another language embeds.
/// The line halts where `end` holds before HAML's patterns are tried,
/// and also where `after` holds and the patterns fail.
/// Both apply only while no HAML rule is open.
/// The second value holds where and why the line stopped (see
/// [`rules::lex_until`]).
pub(crate) fn lex_until(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    end: &dyn Halt,
    after: Option<&dyn Halt>,
) -> (Finished, Option<Stopped>) {
    rules::lex_embedding_until::<Haml>(cx, line, state, buffers, (end, after))
}

#[cfg(test)]
mod tests;
