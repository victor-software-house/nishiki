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

pub(crate) use scope::NAMES;
use scope::S;
use state::Rule;

use crate::arena::{Arena, ROOT};
use crate::language::{Context, Line};

/// The state at the start of a SQL document whose scopes sit on the stack
/// `base`: one rule with `source.sql` on top of it.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    let stack = arena.child(base, S::Source.scope()).unwrap_or(base);
    arena
        .push(ROOT, Rule::Root.code(), 0, stack)
        .unwrap_or(ROOT)
}

/// The state at the start of the SQL lines of a fenced code block, whose text
/// sits on the stack `base` with no `source.sql` of its own, as in Shiki's
/// embedding.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    arena.push(ROOT, Rule::Root.code(), 0, base).unwrap_or(ROOT)
}

/// Lexes `line` from `state` under the crate's one lexer contract
/// ([`Language::lex`](crate::Language)), with token ends counted from the
/// start of the whole line, and returns the state after it.
///
/// Lexing starts at `line.start`, where a Markdown fence's text begins after
/// the indentation of the containers around it. The bytes before it stay in
/// the line, because the grammar looks at them: `^` matches only at 0. For an
/// embedded line the fence's `while` rule has matched the container prefix
/// just before `start`, which is where `\G` stays for the line, as it does in
/// Shiki; a SQL document has no such rule, so `\G` matches nowhere at the
/// start of its lines.
///
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
