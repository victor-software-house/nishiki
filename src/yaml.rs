//! YAML, with the scopes of the `yaml` grammar that Shiki bundles.
//!
//! The lexer is built from the YAML 1.2 specification and from Shiki's
//! output alone: the grammar file is not read, because it carries no licence.
//! It is a hand-written state machine over bytes whose contexts (block
//! scalars, quoted strings and flow collections) are frames of the
//! highlighter's interned state.

mod lexer;
mod scalar;
mod scope;
mod state;

pub(crate) use scope::NAMES;
use scope::S;
use state::Kind;

use crate::Token;
use crate::arena::{Arena, ROOT};
use crate::limits::{Limits, LineReport};

/// The state at the start of a YAML document whose scopes sit on the stack
/// `base`: one context with `source.yaml` on top.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    let stack = arena.child(base, S::Source.scope()).unwrap_or(base);
    arena.push(ROOT, Kind::Top.code(), 0, stack).unwrap_or(ROOT)
}

/// The state at the start of the YAML lines of a fenced code block, whose
/// text sits on the stack `base` with no `source.yaml` of its own, as in
/// Shiki's embedding.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    arena.push(ROOT, Kind::Top.code(), 0, base).unwrap_or(ROOT)
}

/// Lexes one line from `state`, appending its tokens, and returns the state
/// after it.
///
/// An empty line appends nothing and leaves the state as it is. `line_start`
/// says whether `^` matches at the start of `line`, which it does not for the
/// rest of a line that a Markdown container began.
pub(crate) fn line(
    arena: &mut Arena,
    limits: &Limits,
    line: &[u8],
    line_start: bool,
    state: u32,
    out: &mut Vec<Token>,
    report: &mut LineReport,
) -> u32 {
    if line.is_empty() {
        return state;
    }
    let mut lexer = lexer::Lexer::new(arena, limits, line, line_start, out, state, report);
    lexer.run();
    lexer.into_state()
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
) -> u32 {
    let line_start = line.start == 0;
    cx.lex_from_start(line, |cx, rest| {
        self::line(
            cx.arena,
            cx.limits,
            rest.as_bytes(),
            line_start,
            state,
            cx.out,
            cx.report,
        )
    })
}
