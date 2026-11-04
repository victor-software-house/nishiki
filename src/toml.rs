//! TOML, with the scopes of the `toml` grammar that Shiki bundles.
//!
//! It is written from the TOML 1.0 specification and from Shiki's output on
//! probe lines and on the oracle's inputs.
//! Shiki's output fixes what the grammar does where the specification is
//! silent.
//!
//! It is a hand-written state machine over bytes whose contexts (arrays, inline
//! tables and strings) are frames of the highlighter's interned state.
//!
//! It reproduces Shiki's output, quirks included:
//! - a value that follows another in an array without a comma is an
//!   `invalid.illegal` scope;
//! - so is any text that is not a key, a value or a table header;
//! - `01` is the number `0` and then an invalid `1`;
//! - a table header is read only at the start of a line.

mod key;
mod lexer;
mod scope;
mod state;
mod string;
mod value;

pub(crate) use scope::NAMES;
use scope::S;
use state::Kind;

use crate::arena::{Arena, ROOT};
use crate::language::{Context, Line};
use crate::limits::Finished;

/// The state at the start of a TOML document whose scopes sit on the stack
/// `base`: one context with `source.toml` on top.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    let stack = arena.child(base, S::Source.scope()).unwrap_or(base);
    arena.push(ROOT, Kind::Top.code(), 0, stack).unwrap_or(ROOT)
}

/// The state at the start of a TOML body whose text sits on the stack `base`
/// with no `source.toml` of its own, as an embedding language would start it.
/// No Markdown fence embeds TOML, because Shiki's Markdown grammar has no
/// rule for it.
/// This completes the contract every language keeps.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    arena.push(ROOT, Kind::Top.code(), 0, base).unwrap_or(ROOT)
}

/// Lexes `line` from `state` under the crate's one lexer contract
/// ([`Language::lex`](crate::Language)), with token ends counted from the
/// start of the whole line, and returns the state after it.
///
/// An empty line appends nothing and leaves the state as it is.
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    cx.lex_from_start(line, |cx, rest| {
        if rest.is_empty() {
            return Finished::unspent(state);
        }
        let mut lexer = lexer::Lexer::new(cx, rest.as_bytes(), state);
        lexer.run();
        lexer.finish_line()
    })
}

#[cfg(test)]
mod tests;
