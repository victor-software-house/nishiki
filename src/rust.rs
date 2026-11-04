//! Rust, with the scopes of the Rust grammar that Shiki bundles.
//!
//! The lexer is a hand-written state machine over bytes.
//! Its contexts (strings, comments, calls, generics and so on) are frames of
//! the highlighter's interned state.
//!
//! It reproduces Shiki's output, quirks included:
//! - `||` is two `|` operators;
//! - `a!=b` reads `a!` as a macro;
//! - a camel-case name such as `fooBar` gets no scope.
//!
//! Every non-ASCII byte counts as a word character.

mod code;
mod lexer;
mod numbers;
mod operators;
mod scope;
mod state;
mod text;

pub(crate) use scope::NAMES;
use scope::S;
use state::Kind;

use crate::Token;
use crate::arena::{Arena, ROOT};
use crate::limits::{Finished, Limits, LineReport};

/// The state at the start of a Rust document whose scopes sit on the stack
/// `base`: one context with `source.rust` on top of it.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    let stack = arena.child(base, S::Source.scope()).unwrap_or(base);
    arena.push(ROOT, Kind::Top.code(), 0, stack).unwrap_or(ROOT)
}

/// The state at the start of the Rust lines of a fenced code block, whose
/// text sits on the stack `base` with no `source.rust` of its own, as in
/// Shiki's embedding.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    arena.push(ROOT, Kind::Top.code(), 0, base).unwrap_or(ROOT)
}

/// Lexes one line from `state`, appending its tokens, and returns the state
/// after it, with the steps it spent.
pub(crate) fn line(
    arena: &mut Arena,
    limits: &Limits,
    line: &[u8],
    state: u32,
    out: &mut Vec<Token>,
    report: &mut LineReport,
) -> Finished {
    let mut lexer = lexer::Lexer::new(arena, limits, line, out, state, report);
    lexer.run();
    lexer.finish_line()
}

#[cfg(test)]
mod tests;

/// Lexes `line` from `state` under the crate's one lexer contract
/// ([`Language::lex`](crate::Language)), with token ends counted from the
/// start of the whole line, and returns the state after it.
#[inline]
pub(crate) fn lex(
    cx: &mut crate::language::Context<'_>,
    line: crate::language::Line<'_>,
    state: u32,
) -> Finished {
    cx.lex_from_start(line, |cx, rest| {
        self::line(
            cx.arena,
            cx.limits,
            rest.as_bytes(),
            state,
            cx.out,
            cx.report,
        )
    })
}
