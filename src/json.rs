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
use crate::language::{Context, Halt, Line, Stopped};
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
        let mut lexer = lexer::Lexer::new(cx, dialect, rest.as_bytes(), state, lexer::NoHalts);
        lexer.run();
        lexer.finish_line()
    })
}

/// Lexes `line` like [`lex`], for JSON that another language embeds from
/// [`start_embedded`], and returns where the line went back to it.
///
/// The halts are tested only at the top of the document, where no array,
/// object, string or comment is open.
/// Only there does Shiki try the embedding rule's patterns beside JSON's.
/// - `end` is tested where a step begins and at each blank.
/// - `after` is tested where no JSON token starts.
///
/// A PHP heredoc tagged `JSON` halts at its closing label and at an
/// interpolation.
pub(crate) fn lex_until(
    cx: &mut Context<'_>,
    dialect: Dialect,
    line: Line<'_>,
    state: u32,
    end: &dyn Halt,
    after: Option<&dyn Halt>,
) -> (Finished, Option<Stopped>) {
    let mut halted = None;
    let next = cx.lex_from_start(line, |cx, rest| {
        if rest.is_empty() {
            return Finished::unspent(state);
        }
        let halts = lexer::Halts {
            line: line.text.as_bytes(),
            start: line.start,
            end,
            after,
        };
        let mode = lexer::Embedded::new(halts);
        let mut lexer = lexer::Lexer::new(cx, dialect, rest.as_bytes(), state, mode);
        lexer.run();
        halted = lexer.halted().map(|p| line.start + p);
        lexer.finish_line()
    });
    (next, halted.map(Stopped::halt))
}

#[cfg(test)]
mod tests;
