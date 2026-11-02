//! Java, with the scopes of the `java` grammar that Shiki bundles, which
//! comes from the Red Hat Java extension for VS Code.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers
//! over bytes (see the `engine` module): the tables in `grammar` are
//! generated from the grammar, and the `matchers` decide where each pattern
//! matches. The rules left open at the end of a line (blocks, parentheses,
//! comments, text blocks) are frames of the highlighter's interned state.

mod engine;
mod grammar;
mod matchers;
mod nested;
mod scope;
mod state;
mod tables;

pub(crate) use engine::Scratch;
use grammar::Rule;
pub(crate) use scope::NAMES;
use scope::S;

use crate::Token;
use crate::arena::{Arena, ROOT};
use crate::limits::{Limits, LineReport};

/// The state at the start of a Java document whose scopes sit on the stack
/// `base`: one rule with `source.java` on top of it.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    let stack = arena.child(base, S::Source.scope()).unwrap_or(base);
    arena
        .push(ROOT, Rule::Root.code(), 0, stack)
        .unwrap_or(ROOT)
}

/// The state at the start of the Java lines of a fenced code block, whose
/// text sits on the stack `base` with no `source.java` of its own, as in
/// Shiki's embedding.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    arena.push(ROOT, Rule::Root.code(), 0, base).unwrap_or(ROOT)
}

/// Lexes one line from `state`, appending its tokens, and returns the state
/// after it.
///
/// An empty line leaves the state as it is, because Shiki does not tokenize
/// empty lines.
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
    let lexer = engine::Lexer::new(arena, limits, line, out, state, report, &mut scratch.java);
    lexer.run(&mut scratch.java)
}

/// A line and where lexing it starts: byte `start`, where a Markdown fence's
/// text begins after the indentation of the containers around it. The bytes
/// before it stay in the line, because the grammar looks at them: `^` matches
/// only at 0, so a decorator that follows indentation is not at the start of
/// the line.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Text<'a> {
    pub(crate) bytes: &'a [u8],
    pub(crate) start: usize,
}

#[cfg(test)]
mod tests;
