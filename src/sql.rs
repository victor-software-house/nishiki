//! SQL, with the scopes of the `sql` grammar that Shiki bundles.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers over
//! bytes, in the `engine` module.
//! The rules left open at the end of a line (a string, a block comment,
//! brackets) are frames of the highlighter's interned state.
//! The keyword and function tables in `words` are generated from the grammar by
//! `codegen/src/words/sql.rs`.
//!
//! It reproduces Shiki's output, quirks included:
//! - a `#` comment is no comment;
//! - `tablespace` after `drop` is `table` and then a word;
//! - a name that follows a `.` is a table name even when it is a number;
//! - a string that closes on its line is matched without regard to a backslash
//!   before its quote.
//!
//! An empty line leaves the state as it is, because Shiki does not tokenize
//! empty lines.

mod engine;
mod keywords;
mod matchers;
mod rules;
mod scope;
mod state;
#[rustfmt::skip]
mod words;

pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Halt, Line, Stopped};
use crate::limits::Finished;

/// The state at the start of a SQL document whose scopes sit on the stack
/// `base`: one rule with `source.sql` on top of it.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    crate::rules::start::<engine::Sql>(arena, base)
}

/// The state at the start of the SQL lines of a fenced code block.
/// The block's text sits on the stack `base` with no `source.sql` of its own,
/// as in Shiki's embedding.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    crate::rules::start_embedded::<engine::Sql>(arena, base)
}

/// Lexes one line under the crate's lexer contract.
/// For a Markdown fence's line, `\G` stays where the fence's `while` rule
/// matched the container prefix, as in Shiki.
/// A SQL document has no such rule.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    crate::rules::lex::<engine::Sql>(cx, line, state, |scratch| &mut scratch.sql)
}

/// Lexes `line` like [`lex`], for SQL that another language embeds, up to
/// where `end` or `after` halts it (see [`crate::rules::lex_until`]).
#[inline]
pub(crate) fn lex_until(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    end: &dyn Halt,
    after: Option<&dyn Halt>,
) -> (Finished, Option<Stopped>) {
    crate::rules::lex_until::<engine::Sql>(cx, line, state, |scratch| &mut scratch.sql, end, after)
}

#[cfg(test)]
mod tests;
