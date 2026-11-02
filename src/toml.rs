//! TOML, with the scopes of the `toml` grammar that Shiki bundles.
//!
//! The grammar has no licence, so this lexer reads neither it nor any file
//! derived from it. It is written from the TOML 1.0 specification and from
//! Shiki's output on probe lines and on the oracle's inputs, which fix what
//! the grammar does where the specification is silent.
//!
//! It is a hand-written state machine over bytes whose contexts (arrays,
//! inline tables and strings) are frames of the highlighter's interned state.
//! It reproduces Shiki's output, quirks included: a value that follows
//! another in an array without a comma, and any text that is not a key, a
//! value or a table header, is an `invalid.illegal` scope; `01` is the number
//! `0` and then an invalid `1`; and a table header is read only at the start
//! of a line.

mod key;
mod lexer;
mod scope;
mod state;
mod string;
mod value;

pub(crate) use scope::NAMES;
use scope::S;
use state::Kind;

use crate::Token;
use crate::arena::{Arena, ROOT};
use crate::limits::{Limits, LineReport};

/// The state at the start of a TOML document whose scopes sit on the stack
/// `base`: one context with `source.toml` on top.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    let stack = arena.child(base, S::Source.scope()).unwrap_or(base);
    arena.push(ROOT, Kind::Top.code(), 0, stack).unwrap_or(ROOT)
}

/// The state at the start of a TOML body whose text sits on the stack `base`
/// with no `source.toml` of its own, as an embedding language would start it.
/// No Markdown fence embeds TOML, because Shiki's Markdown grammar has no
/// rule for it; this completes the contract every language keeps.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    arena.push(ROOT, Kind::Top.code(), 0, base).unwrap_or(ROOT)
}

/// Lexes one line from `state`, appending its tokens, and returns the state
/// after it.
///
/// An empty line appends nothing and leaves the state as it is.
pub(crate) fn line(
    arena: &mut Arena,
    limits: &Limits,
    line: &[u8],
    state: u32,
    out: &mut Vec<Token>,
    report: &mut LineReport,
) -> u32 {
    if line.is_empty() {
        return state;
    }
    let mut lexer = lexer::Lexer::new(arena, limits, line, out, state, report);
    lexer.run();
    lexer.into_state()
}

#[cfg(test)]
mod tests;
