//! GraphQL, with the scopes of the `graphql` grammar that Shiki bundles.
//!
//! The lexer runs the grammar's begin and end rules on the shared rule
//! engine ([`crate::rules`]).
//! Hand-written matchers over bytes decide where each pattern matches.
//!
//! A rule left open at the end of a line is a frame of the highlighter's
//! interned state.
//! Such a rule is a selection set, a type body or a multi-line string.
//! The JavaScript of an interpolation that goes on across lines is the
//! payload of its frame.
//!
//! It reproduces Shiki's output, quirks included:
//! - The grammar's comment rule comes first everywhere. It reads every `"..."`
//!   and `"""..."""` as a comment, across lines. A string that follows white
//!   space where a value is expected is the exception.
//! - A string has no end at the line feed.
//! - A match rule that does not advance closes the rule it sits in.
//!
//! An empty line leaves the state as it is.
//! Shiki does not tokenize empty lines.

mod engine;
mod matchers;
mod rules;
mod scope;
mod state;

use engine::GraphQl;
pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Halt, Line, Stopped};
use crate::limits::Finished;

/// GraphQL's buffers in the highlighter's scratch, allocated by the first
/// GraphQL line.
fn buffers(scratch: &mut crate::scratch::Scratch) -> &mut Scratch {
    scratch.graphql.get_or_insert_with(Box::default)
}

/// The state at the start of a GraphQL document whose scopes sit on the stack
/// `base`: one rule with `source.graphql` on top of it.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    crate::rules::start::<GraphQl>(arena, base)
}

/// The state at the start of a GraphQL body whose text sits on the stack
/// `base`, with no `source.graphql` of its own.
/// An embedding language starts it this way.
/// No Markdown fence embeds GraphQL, because Shiki's Markdown grammar has no
/// rule for it.
/// The function completes the contract every language keeps.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    crate::rules::start_embedded::<GraphQl>(arena, base)
}

/// Lexes `line` from `state` under the crate's one lexer contract
/// ([`Language::lex`](crate::Language)), and returns the state after it.
/// Token ends count from the start of the whole line.
///
/// Lexing starts at `line.start`, where an embedding language's text begins.
/// The bytes before it stay in the line, because the grammar looks at them.
/// An empty line leaves the state as it is.
/// Shiki does not tokenize empty lines.
/// Otherwise the grammar would close a rule the line before left open.
///
/// An interpolation's text goes to the JavaScript lexer, up to its `}`
/// ([`crate::rules::lex_embedding`]).
#[expect(
    clippy::inline_always,
    reason = "measured at one codegen unit: inlined by hint, \
              it stayed a call from Language::lex once that dispatch inlined"
)]
#[inline(always)]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    crate::rules::lex_embedding::<GraphQl>(cx, line, state, buffers)
}

/// Lexes `line` like [`lex`], for GraphQL that another language embeds.
/// The line halts where `end` holds before GraphQL's patterns are tried.
/// It also halts where `after` holds and the patterns fail.
/// Both apply only while no GraphQL rule is open.
/// The second value is where the line halted.
pub(crate) fn lex_until(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    end: &dyn Halt,
    after: Option<&dyn Halt>,
) -> (Finished, Option<Stopped>) {
    crate::rules::lex_embedding_until::<GraphQl>(cx, line, state, buffers, (end, after))
}

#[cfg(test)]
mod tests;
