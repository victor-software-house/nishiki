//! GraphQL, with the scopes of the `graphql` grammar that Shiki bundles.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers
//! over bytes (see the `engine` module). The rules left open at the end of a
//! line (a selection set, a type body, a multi-line string) are frames of the
//! highlighter's interned state.
//!
//! It reproduces Shiki's output, quirks included: the grammar's comment rule
//! comes first everywhere and reads every `"..."` and `"""..."""` as a
//! comment, across lines, except a string that follows white space where a
//! value is expected; a string has no end at the line feed; and a match rule
//! that does not advance closes the rule it sits in.
//!
//! An empty line leaves the state as it is, because Shiki does not tokenize
//! empty lines.

mod engine;
mod matchers;
mod rules;
mod scope;
mod state;

pub(crate) use scope::NAMES;
use scope::S;
use state::Rule;

use crate::arena::{Arena, ROOT};
use crate::language::{Context, Line};

/// The state at the start of a GraphQL document whose scopes sit on the stack
/// `base`: one rule with `source.graphql` on top of it.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    let stack = arena.child(base, S::Source.scope()).unwrap_or(base);
    arena
        .push(ROOT, Rule::Root.code(), 0, stack)
        .unwrap_or(ROOT)
}

/// The state at the start of a GraphQL body whose text sits on the stack
/// `base` with no `source.graphql` of its own, as an embedding language would
/// start it. No Markdown fence embeds GraphQL, because Shiki's Markdown
/// grammar has no rule for it; this completes the contract every language
/// keeps.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    arena.push(ROOT, Rule::Root.code(), 0, base).unwrap_or(ROOT)
}

/// Lexes `line` from `state` under the crate's one lexer contract
/// ([`Language::lex`](crate::Language)), with token ends counted from the
/// start of the whole line, and returns the state after it.
///
/// Lexing starts at `line.start`, where an embedding language's text begins;
/// the bytes before it stay in the line, because the grammar looks at them.
/// An empty line leaves the state as it is: Shiki does not tokenize empty
/// lines, and the grammar would otherwise close a rule it left open at the end
/// of the line before.
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> u32 {
    if line.text.is_empty() {
        return state;
    }
    engine::Lexer::new(cx, line, state).run()
}

#[cfg(test)]
mod tests;
