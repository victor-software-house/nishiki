//! SQL, with the scopes of the `sql` grammar that Shiki bundles.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers
//! over bytes (see the `engine` module). The rules left open at the end of a
//! line (a string, a block comment, brackets) are frames of the highlighter's
//! interned state. The keyword and function tables in `words` are generated
//! from the grammar by `oracle/sql-words.ts`.
//!
//! It reproduces Shiki's output, quirks included: a `#` comment is no comment,
//! `tablespace` after `drop` is `table` and then a word, a name that follows a
//! `.` is a table name even when it is a number, and a string that closes on
//! its line is matched without regard to a backslash before its quote.
//!
//! An empty line leaves the state as it is, because Shiki does not tokenize
//! empty lines.

mod engine;
mod keywords;
mod matchers;
mod rules;
mod scope;
mod state;
mod words;

pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Line};

/// The state at the start of a SQL document whose scopes sit on the stack
/// `base`: one rule with `source.sql` on top of it.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    crate::rules::start::<engine::Sql>(arena, base)
}

/// The state at the start of the SQL lines of a fenced code block, whose text
/// sits on the stack `base` with no `source.sql` of its own, as in Shiki's
/// embedding.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    crate::rules::start_embedded::<engine::Sql>(arena, base)
}

/// Lexes one line under the crate's lexer contract. For a Markdown fence's
/// line, `\G` stays where the fence's `while` rule matched the container
/// prefix, as in Shiki; a SQL document has no such rule.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> u32 {
    crate::rules::lex::<engine::Sql>(cx, line, state, |scratch| &mut scratch.sql)
}

#[cfg(test)]
mod tests;
