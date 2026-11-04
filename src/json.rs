//! JSON, with the scopes of the `json`, `jsonc` and `jsonl` grammars that Shiki
//! bundles.
//!
//! The three grammars are one grammar with three spellings of its scope names.
//! One lexer therefore serves them, told apart by [`Dialect`].
//! It is a hand-written state machine over bytes whose contexts (arrays,
//! objects, the value after a colon, strings and comments) are frames of the
//! highlighter's interned state.
//!
//! It reproduces Shiki's output, quirks included:
//! - every dialect takes `//` and `/* */` comments;
//! - a stray character in an array or object is an `invalid.illegal` scope of
//!   its own;
//! - `01` is the number `0` and then the number `1`.
//!
//! A non-ASCII digit is not a digit.
//! A non-ASCII word character is one when it is alphanumeric.
//! Combining marks therefore are not.

mod lexer;
mod scope;
mod state;

pub(crate) use scope::NAMES;
use scope::S;
use state::Kind;

use crate::arena::{Arena, ROOT};
use crate::language::{Context, Line};
use crate::limits::Finished;

/// Which of the grammars' scope-name spellings to use.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Dialect {
    /// `json`: names end in `.json`.
    Json,
    /// `jsonc`: names end in `.json.comments`.
    Jsonc,
    /// `jsonl`: names end in `.json.lines`.
    Jsonl,
}

impl Dialect {
    /// The dialect's position among the three.
    /// It numbers the dialect's table.
    pub(crate) const fn index(self) -> u16 {
        match self {
            Dialect::Json => 0,
            Dialect::Jsonc => 1,
            Dialect::Jsonl => 2,
        }
    }
}

/// The state at the start of a JSON document whose scopes sit on the stack
/// `base`: one context with `source.json` (in `dialect`'s spelling) on top.
pub(crate) fn start(arena: &mut Arena, base: u32, dialect: Dialect) -> u32 {
    let stack = arena.child(base, S::Source.scope(dialect)).unwrap_or(base);
    arena.push(ROOT, Kind::Top.code(), 0, stack).unwrap_or(ROOT)
}

/// The state at the start of the JSON lines of a fenced code block, whose
/// text sits on the stack `base` with no `source.json` of its own, as in
/// Shiki's embedding.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    arena.push(ROOT, Kind::Top.code(), 0, base).unwrap_or(ROOT)
}

/// Lexes `line` in `dialect` from `state` under the crate's one lexer contract
/// ([`Language::lex`](crate::Language)), with token ends counted from the
/// start of the whole line, and returns the state after it.
///
/// An empty line appends nothing and leaves the state as it is.
pub(crate) fn lex(cx: &mut Context<'_>, dialect: Dialect, line: Line<'_>, state: u32) -> Finished {
    cx.lex_from_start(line, |cx, rest| {
        if rest.is_empty() {
            return Finished::unspent(state);
        }
        let mut lexer = lexer::Lexer::new(cx, dialect, rest.as_bytes(), state);
        lexer.run();
        lexer.finish_line()
    })
}

#[cfg(test)]
mod tests;
