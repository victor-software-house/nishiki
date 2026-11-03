//! Ruby, with the scopes of the `ruby` grammar that Shiki bundles, which
//! comes from Shopify's ruby-lsp through VS Code.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers
//! over bytes (see the `engine` module): the tables in `grammar` are
//! generated from the grammar, and the `matchers` decide where each pattern
//! matches. The rules left open at the end of a line (strings, heredocs,
//! `=begin` comments, blocks) are frames of the highlighter's interned state.

mod engine;
mod grammar;
mod matchers;
mod nested;
mod scope;
mod state;

pub(crate) use engine::Scratch;
use grammar::Rule;
pub(crate) use scope::NAMES;
use scope::S;

use crate::Token;
use crate::arena::{Arena, ROOT};
use crate::limits::{Limits, LineReport};
use crate::rules::Text;

/// The state at the start of a Ruby document whose scopes sit on the stack
/// `base`: one rule with `source.ruby` on top of it.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    let stack = arena.child(base, S::Source.scope()).unwrap_or(base);
    arena
        .push(ROOT, Rule::Root.code(), 0, stack)
        .unwrap_or(ROOT)
}

/// The state at the start of the Ruby lines of a fenced code block, whose
/// text sits on the stack `base` with no `source.ruby` of its own, as in
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
    let lexer = engine::Lexer::new(arena, limits, line, out, state, report, &mut scratch.ruby);
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
