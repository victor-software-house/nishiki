//! Dockerfile, with the scopes of the `docker` grammar that Shiki bundles.
//!
//! The grammar has three rules that start with `^` (a `FROM ... AS` line, an
//! instruction keyword, a comment) and two string rules that run across
//! lines. The lexer tries the first three at the start of a line, in the
//! grammar's order, and then scans for quotes and backslashes; an open string
//! is the one frame the lexer pushes.
//!
//! It reproduces Shiki's output, quirks included: a comment is read only at
//! the start of a line and never inside a string, a backslash escapes any
//! character, a string does not end at a line end, and a line that continues
//! another still reads its first word as an instruction.
//!
//! An empty line leaves the state as it is, because Shiki does not tokenize
//! empty lines.

mod rules;
mod scope;

use memchr::{memchr2, memchr3};
pub(crate) use scope::NAMES;
use scope::S;

use crate::arena::{Arena, Frame, ROOT};
use crate::language::{Context, Line};
use crate::limits::{Limit, Limits, LineReport};
use crate::utf8::char_len;
use crate::{Stack, Token};

/// The high byte that marks a frame as one of the Dockerfile lexer's.
const TAG: u16 = crate::tag::DOCKERFILE;

/// What a frame is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    /// Outside a string.
    Top,
    /// Inside `"`.
    Double,
    /// Inside `'`.
    Single,
}

impl Kind {
    fn code(self) -> u16 {
        TAG | match self {
            Kind::Top => 0,
            Kind::Double => 1,
            Kind::Single => 2,
        }
    }

    fn from_code(code: u16) -> Kind {
        match code & !TAG {
            1 => Kind::Double,
            2 => Kind::Single,
            _ => Kind::Top,
        }
    }

    /// The quote that ends the string.
    fn string(self) -> Option<u8> {
        match self {
            Kind::Top => None,
            Kind::Double => Some(b'"'),
            Kind::Single => Some(b'\''),
        }
    }
}

/// The state at the start of a Dockerfile whose scopes sit on the stack
/// `base`: one context with `source.dockerfile` on top.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    let stack = arena.child(base, S::Source.scope()).unwrap_or(base);
    arena.push(ROOT, Kind::Top.code(), 0, stack).unwrap_or(ROOT)
}

/// The state at the start of the Dockerfile lines of a Markdown fence, whose
/// text sits on the stack `base` with no `source.dockerfile` of its own, as in
/// Shiki's embedding.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    arena.push(ROOT, Kind::Top.code(), 0, base).unwrap_or(ROOT)
}

/// Lexes `line` from `state` under the crate's one lexer contract
/// ([`Language::lex`](crate::Language)), appending tokens whose ends count
/// from the start of the whole line, and returns the state after it.
///
/// Lexing starts at `line.start`, where a Markdown fence's text begins after
/// the indentation of the containers around it; the bytes before it stay in
/// the line, because `^` matches only at 0.
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> u32 {
    let (bytes, start) = (line.text.as_bytes(), line.start);
    if bytes.len() <= start {
        return state;
    }
    let budget = cx
        .limits
        .work
        .map_or(u64::MAX, |work| work.steps(bytes.len() - start));
    let frame = cx.arena.frame(state);
    let mut lexer = Lexer {
        arena: cx.arena,
        limits: cx.limits,
        out: cx.out,
        report: cx.report,
        bytes,
        state,
        frame,
        pos: start,
    };
    lexer.run(start, budget);
    lexer.state
}

struct Lexer<'a> {
    arena: &'a mut Arena,
    limits: &'a Limits,
    out: &'a mut Vec<Token>,
    report: &'a mut LineReport,
    bytes: &'a [u8],
    state: u32,
    frame: Frame,
    /// The end of the last token.
    pos: usize,
}

impl Lexer<'_> {
    fn run(&mut self, start: usize, budget: u64) {
        let len = self.bytes.len();
        let mut text = rules::Line::new(self.bytes);
        let mut pos = start;
        if start == 0
            && Kind::from_code(self.frame.kind) == Kind::Top
            && let Some(hit) = text.at_start()
        {
            self.hit(&hit);
            pos = hit.end;
        }
        let mut steps = text.spent;
        while pos < len {
            steps = steps.saturating_add(1);
            if steps > budget {
                self.report.record(Limit::Work);
                break;
            }
            let from = pos;
            pos = match Kind::from_code(self.frame.kind).string() {
                None => self.top(pos),
                Some(quote) => self.string(pos, quote),
            };
            debug_assert!(
                pos > from,
                "the Dockerfile lexer made no progress at byte {from}"
            );
        }
        self.tok(len, &[]);
    }

    /// Scans the text outside a string from `pos` for its next quote or
    /// escape, and returns where scanning goes on.
    fn top(&mut self, pos: usize) -> usize {
        let Some(found) = memchr3(b'"', b'\'', b'\\', &self.bytes[pos..]) else {
            return self.bytes.len();
        };
        let at = pos + found;
        match self.bytes[at] {
            b'\\' => self.escape(at),
            quote => self.open(at, quote),
        }
    }

    /// Scans a string's text from `pos`.
    fn string(&mut self, pos: usize, quote: u8) -> usize {
        let Some(found) = memchr2(quote, b'\\', &self.bytes[pos..]) else {
            return self.bytes.len();
        };
        let at = pos + found;
        if self.bytes[at] == b'\\' {
            return self.escape(at);
        }
        self.tok(at + 1, &[]);
        self.state = self.frame.parent;
        self.frame = self.arena.frame(self.state);
        at + 1
    }

    /// A backslash at `at` and the character after it, which is not the line
    /// feed, make one escape; at the end of the line it is plain text.
    fn escape(&mut self, at: usize) -> usize {
        let Some(&next) = self.bytes.get(at + 1) else {
            return at + 1;
        };
        let end = at + 1 + char_len(next);
        self.tok(at, &[]);
        self.tok(end, &[S::Escape]);
        end
    }

    /// Opens the string whose quote is at `at`, unless a limit keeps it
    /// closed, and returns where scanning goes on.
    fn open(&mut self, at: usize, quote: u8) -> usize {
        let depth = usize::try_from(self.frame.depth).unwrap_or(usize::MAX);
        if self.limits.depth.is_some_and(|max| depth >= max) {
            self.report.record(Limit::Depth);
            return at + 1;
        }
        let (kind, scope) = if quote == b'"' {
            (Kind::Double, S::Double)
        } else {
            (Kind::Single, S::Single)
        };
        let stack = self.arena.child(self.frame.stack, scope.scope());
        let Some(state) =
            stack.and_then(|stack| self.arena.push(self.state, kind.code(), 0, stack))
        else {
            self.report.record(Limit::Nodes);
            return at + 1;
        };
        self.tok(at, &[]);
        self.state = state;
        self.frame = self.arena.frame(state);
        self.tok(at + 1, &[]);
        at + 1
    }

    /// Emits the pieces of a rule's match.
    fn hit(&mut self, hit: &rules::Hit) {
        for seg in hit.segs.iter().flatten() {
            self.tok(seg.to, seg.leaf);
        }
        self.tok(hit.end, &[]);
    }

    /// Emits the bytes up to `to` under the frame's stack with `leaf` on top.
    fn tok(&mut self, to: usize, leaf: &[S]) {
        let to = to.min(self.bytes.len());
        if to <= self.pos {
            return;
        }
        let mut stack = self.frame.stack;
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
}

#[cfg(test)]
mod tests;
