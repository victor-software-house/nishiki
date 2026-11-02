//! C, with the scopes of the `c` grammar that Shiki bundles.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers
//! over bytes (see the `engine` module). The rules left open at the end of a
//! line (comments, strings, blocks, macros with a backslash at the end) are
//! frames of the highlighter's interned state.
//!
//! It reproduces Shiki's output, quirks included: a `#define` is found
//! anywhere in a line, an unterminated string runs to the end of the
//! document, `x or(` is an initialization, and `a.b.c (` is a member access.

mod access;
mod comments;
mod ends;
mod engine;
mod lists;
mod numbers;
mod pats;
mod preproc;
mod scope;
mod simple;
mod state;
mod strings;
mod switch;
mod text;
mod words;
mod wordset;

pub(crate) use scope::NAMES;
use scope::S;
use state::R;

use crate::Token;
use crate::arena::{Arena, ROOT};
use crate::limits::{Limits, LineReport};

/// The state at the start of a C document whose scopes sit on the stack
/// `base`: one context with `source.c` on top of it.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    let stack = arena.child(base, S::Source.scope()).unwrap_or(base);
    arena
        .push(ROOT, R::Top.code(), stack, stack)
        .unwrap_or(ROOT)
}

/// The state at the start of the C lines of a fenced code block, whose text
/// sits on the stack `base` with no `source.c` of its own, as in Shiki's
/// embedding.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    arena.push(ROOT, R::Top.code(), base, base).unwrap_or(ROOT)
}

/// A line and where lexing it starts: byte `start`, where a Markdown fence's
/// text begins after the indentation of the containers around it. The bytes
/// before it stay in the line, because the grammar looks at them: `^` matches
/// only at 0, so a directive that follows indentation is not at the start of
/// the line.
///
/// `anchored` is set for a Markdown fence line: the fence's `while` rule has
/// matched the container prefix just before `start`, which is where `\G`
/// stays for the line, as it does in Shiki. A C document has no such rule, so
/// `\G` matches at the start of a line only after a begin that ran to the end
/// of the one before.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Text<'a> {
    pub(crate) bytes: &'a [u8],
    pub(crate) start: usize,
    pub(crate) anchored: bool,
}

/// Lexes one line from `state`, appending its tokens, and returns the state
/// after it.
///
/// An empty line appends nothing and leaves the state as it is: Shiki does
/// not tokenize empty lines.
pub(crate) fn line(
    arena: &mut Arena,
    limits: &Limits,
    line: Text<'_>,
    state: u32,
    out: &mut Vec<Token>,
    report: &mut LineReport,
) -> u32 {
    if line.bytes.is_empty() {
        return state;
    }
    let mut lexer = engine::Lexer::new(arena, limits, line, out, state, report);
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
    let text = Text {
        bytes: line.text.as_bytes(),
        start: line.start,
        anchored: line.embedded,
    };
    self::line(cx.arena, cx.limits, text, state, cx.out, cx.report)
}
