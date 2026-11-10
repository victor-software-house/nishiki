//! Shell, with the scopes of the `shellscript` grammar that Shiki bundles.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers over
//! bytes, in the `engine` module.
//! The rules left open at the end of a line are frames of the highlighter's
//! interned state.
//! Strings, here-documents, `if` blocks and statements continued by a
//! backslash are such rules.
//! A here-document's delimiter is the frame's carried value, an interned
//! string.
//!
//! It reproduces Shiki's output, quirks included:
//! - after an unterminated string or here-document every later line stays
//!   inside it;
//! - `||` outside a statement is two `|` operators;
//! - an argument such as `ab>c` is split as `a`, `b>c`.
//!
//! An empty line leaves the state as it is, because Shiki does not tokenize
//! empty lines.
//!
//! Limits:
//! - the highlighter's depth limit bounds nesting;
//! - its node limit bounds the number of states and strings;
//! - its work limit bounds the work of a line.
//!
//! A non-ASCII character is a word character or white space as Shiki's regex
//! engine reads `\w` and `\s` (`crate::classes`).

mod engine;
mod matchers;
mod rules;
mod scope;
mod state;

pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;
use scope::S;
use state::Rule;

use crate::arena::{Arena, ROOT};
use crate::language::Stopped;
use crate::limits::Finished;

/// The state at the start of a shell document whose scopes sit on the stack
/// `base`: one rule with `source.shell` on top of it.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    let stack = arena.child(base, S::Source.scope()).unwrap_or(base);
    arena
        .push(ROOT, Rule::Root.code(), 0, stack)
        .unwrap_or(ROOT)
}

/// The state at the start of a shell fence of a Markdown document whose scopes
/// sit on the stack `base`.
/// Shiki includes the grammar's patterns there without its `source.shell`
/// root.
/// The rule therefore sits on `base` itself.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    arena.push(ROOT, Rule::Root.code(), 0, base).unwrap_or(ROOT)
}

/// A line and where lexing it starts: byte `start`, where a Markdown fence's
/// text begins after the indentation of the containers around it.
/// The bytes before it stay in the line, because the grammar looks at them.
/// `^` matches only at 0, so a command that follows indentation is not at the
/// start of the line.
///
/// `anchored` is set for a Markdown fence line.
/// There the fence's `while` rule has matched the container prefix right before
/// `start`.
/// `\G` stays there for the line, as it does in Shiki.
/// A shell document has no such rule, so `\G` matches nowhere at the start of
/// its lines.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Text<'a> {
    pub(crate) bytes: &'a [u8],
    pub(crate) start: usize,
    pub(crate) anchored: bool,
}

/// Lexes one line from `state`, appending its tokens, and returns the state
/// after it.
///
/// An empty line leaves the state as it is.
/// Shiki does not tokenize empty lines.
/// The grammar would otherwise close a statement it left open at the end of
/// the line before.
pub(crate) fn line(cx: &mut crate::language::Context<'_>, line: Text<'_>, state: u32) -> Finished {
    if line.bytes.is_empty() {
        return Finished::unspent(state);
    }
    let (parts, scratch) = cx.parts();
    let scratch = &mut scratch.shell;
    engine::Lexer::new(parts, line, state, scratch).run(scratch)
}

#[cfg(test)]
mod tests;

/// Lexes `line` like [`lex`], for shell that another language embeds.
/// The line halts at the first place where `end` holds before shell's
/// patterns, or `after` holds once they fail, while no shell rule is open.
/// The second value is where it halted.
pub(crate) fn lex_until(
    cx: &mut crate::language::Context<'_>,
    line: crate::language::Line<'_>,
    state: u32,
    end: &dyn crate::language::Halt,
    after: Option<&dyn crate::language::Halt>,
) -> (Finished, Option<Stopped>) {
    if line.text.len() <= line.start {
        return (Finished::unspent(state), None);
    }
    let text = Text {
        bytes: line.text.as_bytes(),
        start: line.start,
        anchored: line.embedded,
    };
    let halts = (end, after);
    let (parts, scratch) = cx.parts();
    let scratch = &mut scratch.shell;
    let (finished, halt) = engine::Lexer::new(parts, text, state, scratch)
        .halting(&halts)
        .run_halting(scratch);
    (finished, halt.map(Stopped::halt))
}

/// Lexes `line` from `state` and returns the state after it.
///
/// It follows the crate's one lexer contract
/// ([`Language::lex`](crate::Language)).
/// Token ends count from the start of the whole line.
#[inline]
pub(crate) fn lex(
    cx: &mut crate::language::Context<'_>,
    line: crate::language::Line<'_>,
    state: u32,
) -> Finished {
    let text = Text {
        bytes: line.text.as_bytes(),
        start: line.start,
        anchored: line.embedded,
    };
    self::line(cx, text, state)
}
