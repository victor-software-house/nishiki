//! TypeScript, with the scopes of the TypeScript grammar that Shiki bundles.
//!
//! The lexer is a hand-written state machine over bytes. Its contexts (blocks,
//! calls, types, templates, comments and so on) are frames of the
//! highlighter's interned state.
//!
//! Each context is one begin/end rule of the grammar, and each rule is tried
//! in the grammar's order at every position, as `TextMate` does. The grammar
//! decides many rules by looking ahead on the line (an arrow function, a call
//! with type arguments, a name assigned a function), and the lexer reproduces
//! those lookaheads, quirks included:
//!
//! - `Number.MAX_VALUE` and the other `Number` constants get no scope, because
//!   the grammar numbers the capture that would scope them inside a lookahead.
//! - A bracket group in a lookahead nests three levels and no more, as the
//!   grammar's regular expressions do.
//! - An empty line is skipped and leaves the state as it is, as Shiki does, so
//!   the end rules that test for a blank line see only lines of blanks.
//! - A context that opens and closes without the line advancing keeps the rest
//!   of the line, as `TextMate` does for a grammar that loops.
//!
//!
//! Work is counted against [`Limits::work`](crate::Limits): every scan ahead
//! spends a step per byte it reads, so a line of any length is lexed in time
//! proportional to its length times the work limit's per-byte allowance.

#![expect(
    clippy::many_single_char_names,
    reason = "a scanner names positions by role: `p` where a rule starts, `e` where it ends, `j` and `k` for the cursor"
)]

mod calls;
mod decl;
mod decl2;
mod ends;
mod expr;
mod jsdoc;
mod kind;
mod lexer;
mod look;
mod members;
mod misc;
mod num;
mod rules;
mod scope;
mod text;
mod typ;

use kind::Kind;
pub(crate) use scope::NAMES;
use scope::S;

use crate::Token;
use crate::arena::{Arena, ROOT};
use crate::limits::{Limits, LineReport};

/// The state at the start of a TypeScript document whose scopes sit on the
/// stack `base`: one context with `source.ts` on top of it.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    let stack = arena.child(base, S::Source.scope()).unwrap_or(base);
    arena.push(ROOT, Kind::Top.code(), 0, stack).unwrap_or(ROOT)
}

/// The state at the start of the TypeScript lines of a fenced code block,
/// whose text sits on the stack `base` with no `source.ts` of its own, as in
/// Shiki's embedding. The marker kind keeps `\A`, which only the first line of
/// the document matches, from matching in a fence.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    arena
        .push(ROOT, Kind::Started.code(), 0, base)
        .unwrap_or(ROOT)
}

/// Lexes one line from `state`, appending its tokens, and returns the state
/// after it.
///
/// An empty line appends nothing and leaves the state as it is, which is how
/// Shiki treats it: the end rules that test for a blank line (`^\s*$`) never
/// run on an empty one.
pub(crate) fn line(
    arena: &mut Arena,
    limits: &Limits,
    line: &str,
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
