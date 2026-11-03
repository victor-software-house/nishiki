//! Makefile, with the scopes of the `make` grammar that Shiki bundles.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers
//! over bytes (see the `engine` module). The rules left open at the end of a
//! line (a definition, a conditional, a recipe or a target continued by a
//! backslash, an unclosed variable reference) are frames of the highlighter's
//! interned state.
//!
//! It reproduces Shiki's output, quirks included: `$` at the end of a recipe
//! line is not a variable, a `:` anywhere in a line that is not an assignment
//! makes it a target, and the `endef` of a definition has the scope of an
//! `override`.
//!
//! An empty line leaves the state as it is, because Shiki does not tokenize
//! empty lines.

mod engine;
mod matchers;
mod rules;
mod scope;
mod state;

pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;
use scope::S;
use state::Rule;

use crate::Token;
use crate::arena::{Arena, ROOT};
use crate::limits::{Limits, LineReport};
use crate::rules::Text;

/// The state at the start of a Makefile whose scopes sit on the stack `base`:
/// one rule with `source.makefile` on top of it.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    let stack = arena.child(base, S::Source.scope()).unwrap_or(base);
    arena
        .push(ROOT, Rule::Root.code(), 0, stack)
        .unwrap_or(ROOT)
}

/// The state at the start of the Makefile lines of a fenced code block, whose
/// text sits on the stack `base` with no `source.makefile` of its own, as in
/// Shiki's embedding.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    arena.push(ROOT, Rule::Root.code(), 0, base).unwrap_or(ROOT)
}

/// Lexes one line from `state`, appending its tokens, and returns the state
/// after it.
///
/// An empty line leaves the state as it is: Shiki does not tokenize empty
/// lines, and the grammar would otherwise close a rule it left open at the end
/// of the line before.
pub(crate) fn line(
    scratch: &mut crate::scratch::Scratch,
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
    let lexer = engine::Lexer::new(arena, limits, line, out, state, report, &mut scratch.make);
    lexer.run()
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
    self::line(
        cx.scratch, cx.arena, cx.limits, text, state, cx.out, cx.report,
    )
}
