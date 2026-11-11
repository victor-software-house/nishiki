//! HCL, with the scopes of the `hcl` grammar that Shiki bundles.
//!
//! Shiki's grammar is MPL-2.0, so nothing is generated from it.
//! It is read as the reference only.
//! The lexer is hand-written from Shiki's output for probe lines and the
//! oracle's inputs.
//!
//! It is a hand-written state machine over bytes.
//! Its contexts are frames of the highlighter's interned state, one per
//! begin/end rule of the grammar.
//! At each byte it tries a context's end, then the context's rules in the
//! grammar's order.
//! That is the order `TextMate`'s earliest match takes.

mod bytes;
mod contexts;
mod expressions;
mod header;
mod lexer;
mod scope;
mod state;
mod strings;

pub(crate) use scope::NAMES;
use scope::S;
use state::Rule;

use crate::arena::{Arena, ROOT};
use crate::language::{Context, Line};
use crate::limits::Finished;

/// The state at the start of an HCL document whose scopes sit on the stack
/// `base`: one context with `source.hcl` on top.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    let stack = arena.child(base, S::Source.scope()).unwrap_or(base);
    arena.push(ROOT, Rule::Top.code(), 0, stack).unwrap_or(ROOT)
}

/// The state at the start of an HCL body whose text sits on the stack `base`.
/// The body has no `source.hcl` of its own, as an embedding language would
/// start it.
/// No Markdown fence embeds HCL, because Shiki's Markdown grammar has no
/// rule for it.
/// This completes the contract every language keeps.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    arena.push(ROOT, Rule::Top.code(), 0, base).unwrap_or(ROOT)
}

/// Lexes `line` from `state` under the crate's one lexer contract
/// ([`Language::lex`](crate::Language)), and returns the state after it.
/// Token ends count from the start of the whole line.
///
/// An empty line appends nothing.
/// It still ends an `if` that its line feed closes.
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    cx.lex_from_start(line, |cx, rest| {
        let mut lexer = lexer::Lexer::new(cx, rest.as_bytes(), state);
        lexer.run();
        lexer.finish_line()
    })
}

#[cfg(test)]
mod tests;
