//! XML, with the scopes of the `xml` grammar that Shiki bundles, which VS Code
//! ships and which derives from the `TextMate` XML bundle.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers
//! over bytes (see the `engine` module). The rules left open at the end of a
//! line (a tag with attributes, a comment, a CDATA section, a string, a
//! doctype) are frames of the highlighter's interned state.
//!
//! It reproduces Shiki's output, quirks included: a `--` inside a comment
//! opens a rule that never ends, so the rest of the document is a comment, and
//! `<%--` opens one that never ends and has no scope at all. The grammar
//! embeds Java in `<% ... %>`.
//!
//! An empty line leaves the state as it is, because Shiki does not tokenize
//! empty lines.

mod engine;
mod matchers;
mod rules;
mod scope;
mod state;

pub(crate) use engine::Buffers as Scratch;
pub(crate) use scope::NAMES;
use scope::S;
use state::Rule;

use crate::arena::{Arena, ROOT};
use crate::language::{Context, Halt, Line};
use crate::limits::Finished;

/// The state at the start of an XML document whose scopes sit on the stack
/// `base`: one rule with `text.xml` on top of it.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    let stack = arena.child(base, S::Text.scope()).unwrap_or(base);
    arena
        .push(ROOT, Rule::Root.code(), 0, stack)
        .unwrap_or(ROOT)
}

/// The state at the start of an XML body whose text sits on the stack `base`
/// with no `text.xml` of its own, as a Markdown fence starts it.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    arena.push(ROOT, Rule::Root.code(), 0, base).unwrap_or(ROOT)
}

/// Lexes `line` from its start, as [`Language::lex`](crate::Language) asks,
/// with token ends counted from the start of the whole line, and returns the
/// state after it.
///
/// An empty line leaves the state as it is: Shiki does not tokenize empty
/// lines, and the grammar would otherwise close a rule it left open at the end
/// of the line before.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    if line.text.is_empty() {
        return Finished::unspent(state);
    }
    engine::Lexer::new(cx, line, state).run()
}

/// Lexes `line` like [`lex`], for XML that another language embeds from
/// [`start_embedded`]: the line halts at the first place where `end` holds
/// before the root's patterns, or `after` holds where none of them matches,
/// while no rule of XML is open, and the second value is where.
#[inline]
pub(crate) fn lex_until(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    end: &dyn Halt,
    after: Option<&dyn Halt>,
) -> (Finished, Option<usize>) {
    if line.text.len() <= line.start {
        return (Finished::unspent(state), None);
    }
    let halts = (end, after);
    let mut lexer = engine::Lexer::new(cx, line, state);
    lexer.halts = Some(&halts);
    let (finished, halt) = lexer.run_until();
    (finished, halt)
}

#[cfg(test)]
mod tests;
