//! Diff, with the scopes of the `diff` grammar that Shiki bundles.
//!
//! The grammar has no state: every rule is one regular expression over a
//! line, and all but one start with `^`. So a line is lexed on its own, by
//! matchers written by hand for each rule (see `rules`), and the state after
//! any line is the state before it.
//!
//! It reproduces Shiki's output, quirks included: `--- a` is a from-file
//! header wherever it is, a lone `---` is a separator, and a ` - x ====` at
//! the end of any line is a to-file header. `\d` and `\s` for non-ASCII
//! characters come from the table the JSON lexer shares.
//!
//! An empty line leaves the state as it is, because Shiki does not tokenize
//! empty lines.

mod rules;
mod scope;

pub(crate) use scope::NAMES;
use scope::S;

use crate::arena::{Arena, ROOT};
use crate::language::{Context, Line};
use crate::limits::{Limit, LineReport};
use crate::{Stack, Token};

/// The high byte that marks a frame as one of the diff lexer's.
const TAG: u16 = crate::tag::DIFF;

/// The state at the start of a diff document whose scopes sit on the stack
/// `base`: one context with `source.diff` on top.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    let stack = arena.child(base, S::Source.scope()).unwrap_or(base);
    arena.push(ROOT, TAG, 0, stack).unwrap_or(ROOT)
}

/// The state at the start of a diff fence of a Markdown document whose
/// scopes sit on the stack `base`, which has no `source.diff` of its own, as
/// in Shiki's embedding.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    arena.push(ROOT, TAG, 0, base).unwrap_or(ROOT)
}

/// Lexes `line` from `state` under the crate's one lexer contract
/// ([`Language::lex`](crate::Language)), appending tokens whose ends count
/// from the start of the whole line, and returns the state after it, which
/// is `state`.
///
/// Lexing starts at `line.start`, where a Markdown fence's text begins after
/// the indentation of the containers around it; the bytes before it stay in
/// the line, because `^` matches only at 0.
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> u32 {
    let (bytes, start) = (line.text.as_bytes(), line.start);
    if bytes.len() <= start {
        return state;
    }
    let base = cx.arena.frame(state).stack;
    let budget = cx
        .limits
        .work
        .map_or(u64::MAX, |work| work.steps(bytes.len() - start));
    let mut text = rules::Line::new(bytes);
    let mut pos = start;
    let mut emit = Emit {
        arena: cx.arena,
        out: cx.out,
        report: cx.report,
        base,
        pos,
    };
    let mut hit = if start == 0 { text.at_start() } else { None };
    while pos < bytes.len() {
        if text.spent > budget {
            emit.report.record(Limit::Work);
            break;
        }
        let Some(found) = hit.take().or_else(|| text.header_to_inline(pos)) else {
            break;
        };
        emit.hit(&found);
        debug_assert!(
            found.end > pos,
            "the diff lexer made no progress at byte {pos}"
        );
        pos = found.end;
    }
    emit.tok(bytes.len(), &[]);
    state
}

/// Appends tokens from a position to a line's end.
struct Emit<'a> {
    arena: &'a mut Arena,
    out: &'a mut Vec<Token>,
    report: &'a mut LineReport,
    /// The scope stack of text no rule matched.
    base: u32,
    /// The end of the last token.
    pos: usize,
}

impl Emit<'_> {
    /// Emits the bytes up to `to` with the base stack and `leaf` on top.
    fn tok(&mut self, to: usize, leaf: &[S]) {
        if to <= self.pos {
            return;
        }
        let mut stack = self.base;
        for scope in leaf {
            let Some(child) = self.arena.child(stack, scope.scope()) else {
                self.report.record(Limit::Nodes);
                break;
            };
            stack = child;
        }
        self.out.push(Token {
            end: to,
            stack: Stack(stack),
        });
        self.pos = to;
    }

    /// Emits a match: the text before it, then its bytes under its scope
    /// with each capture group's under that scope and the group's own.
    fn hit(&mut self, hit: &rules::Hit) {
        self.tok(hit.start, &[]);
        for cap in hit.caps.iter().flatten() {
            self.tok(cap.from, &[hit.scope]);
            self.tok(cap.to, &[hit.scope, cap.scope]);
        }
        self.tok(hit.end, &[hit.scope]);
    }
}

#[cfg(test)]
mod tests;
