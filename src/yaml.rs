//! YAML, with the scopes of the `yaml` grammar that Shiki bundles.
//!
//! The lexer is built from the YAML 1.2 specification and from Shiki's
//! output.
//! It is a hand-written state machine over bytes whose contexts (block
//! scalars, quoted strings and flow collections) are frames of the
//! highlighter's interned state.

mod lexer;
mod scalar;
mod scope;
mod state;

use lexer::TextStart;
pub(crate) use scope::NAMES;
use scope::S;
use state::Kind;

use crate::arena::{Arena, ROOT};
use crate::language::{Context, Halt, Line};
use crate::limits::Finished;

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

/// Lexes `line` from `state` under the crate's one lexer contract
/// ([`Language::lex`](crate::Language)), with token ends counted from the
/// start of the whole line, and returns the state after it.
///
/// An empty line appends nothing and leaves the state as it is. The lexer
/// reads the text from `line.start`, where `^` does not match when a Markdown
/// container began the line.
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    let start = if line.start == 0 {
        TextStart::Line
    } else {
        TextStart::MidLine
    };
    cx.lex_from_start(line, |cx, rest| {
        if rest.is_empty() {
            return Finished::unspent(state);
        }
        let mut lexer = lexer::Lexer::new(cx, rest.as_bytes(), start, state);
        lexer.run();
        lexer.finish_line()
    })
}

/// Lexes `line` like [`lex`], for YAML that another language embeds from
/// [`start_embedded`], and returns where the line went back to it.
///
/// The halts are tested only where the lexer begins a step while its top
/// context alone is open: `end` there, and `after` where that step is no
/// YAML token. HTML and XML test them at every place up to their next match,
/// but a YAML step takes its blanks with it, as the grammar does: the
/// comment rule begins at the blanks before `#`, so an indented `#{` in a
/// Ruby here document is a YAML comment, and only one where a step begins,
/// such as at the start of an unindented line, goes back to Ruby.
#[inline]
pub(crate) fn lex_until(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    end: &dyn Halt,
    after: Option<&dyn Halt>,
) -> (Finished, Option<usize>) {
    let start = if line.start == 0 {
        TextStart::Line
    } else {
        TextStart::MidLine
    };
    let mut halted = None;
    let next = cx.lex_from_start(line, |cx, rest| {
        if rest.is_empty() {
            return Finished::unspent(state);
        }
        let mut lexer = lexer::Lexer::new(cx, rest.as_bytes(), start, state);
        lexer.halts = Some(lexer::Halts {
            line: line.text.as_bytes(),
            start: line.start,
            end,
            after,
        });
        lexer.run();
        halted = lexer.halted().map(|p| line.start + p);
        lexer.finish_line()
    });
    (next, halted)
}

#[cfg(test)]
mod tests;
