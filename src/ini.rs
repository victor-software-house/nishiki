//! INI, with the scopes of the `ini` grammar that Shiki bundles.
//!
//! Every rule of the grammar is a regular expression that can match at any
//! position of a line.
//! The line's next token is the match that starts first.
//! The lexer keeps each rule's next match, in `rules`, until the cursor passes
//! it.
//! A line is therefore scanned once.
//!
//! A string runs across lines.
//! An open string is the one frame the lexer pushes.
//! A comment runs to the end of its line.
//!
//! It reproduces Shiki's output, quirks included:
//! - a `#` or `;` outside a string starts a comment, as in `a = b # c`;
//! - a `key = value` is found anywhere in a line, so `a = b = c` has two keys;
//! - a double-quoted string has no escapes;
//! - a section header is read only at the start of a line.
//!
//! An empty line leaves the state as it is, because Shiki does not tokenize
//! empty lines.

mod rules;
mod scope;

use memchr::{memchr, memchr2};
use rules::{CommentMark, Hit, Quote};
pub(crate) use scope::NAMES;
use scope::S;

use crate::arena::{Arena, Frame, ROOT};
use crate::language::{Context, Line};
use crate::limits::{Finished, Limit, Limits, LineReport, Steps};
use crate::utf8::char_len;
use crate::{Stack, Token};

/// The high byte that marks a frame as one of the INI lexer's.
const TAG: u16 = crate::tag::INI;

crate::tag::frame_kinds! {
    /// What a frame is.
    enum Kind: TAG {
        /// Outside a string.
        Top,
        /// Inside `'`.
        Single,
        /// Inside `"`.
        Double,
    }
}

/// The state at the start of an INI document whose scopes sit on the stack
/// `base`: one context with `source.ini` on top.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    let stack = arena.child(base, S::Source.scope()).unwrap_or(base);
    arena.push(ROOT, Kind::Top.code(), 0, stack).unwrap_or(ROOT)
}

/// The state at the start of the INI lines of a Markdown fence.
/// The fence's text sits on the stack `base` with no `source.ini` of its own,
/// as in Shiki's embedding.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    arena.push(ROOT, Kind::Top.code(), 0, base).unwrap_or(ROOT)
}

/// Lexes `line` from `state` under the crate's one lexer contract
/// ([`Language::lex`](crate::Language)).
/// It appends tokens whose ends count from the start of the whole line.
/// It returns the state after the line.
///
/// Lexing starts at `line.start`, where a Markdown fence's text begins after
/// the indentation of the containers around it.
/// The bytes before it stay in the line, because `^` matches only at 0.
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    let (bytes, start) = (line.text.as_bytes(), line.start);
    if bytes.len() <= start {
        return Finished::unspent(state);
    }
    let steps = Steps::new(cx.limits, bytes.len() - start);
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
    lexer.run(start, steps)
}

/// Each top-level rule's next match, in the grammar's order.
#[derive(Clone, Copy)]
struct Slots {
    hash: Slot,
    semicolon: Slot,
    key: Slot,
    section: Slot,
    single: Slot,
    double: Slot,
}

impl Slots {
    const UNKNOWN: Slots = Slots {
        hash: Slot::Unknown,
        semicolon: Slot::Unknown,
        key: Slot::Unknown,
        section: Slot::Unknown,
        single: Slot::Unknown,
        double: Slot::Unknown,
    };
}

/// A rule's next match, once it has been looked for.
#[derive(Clone, Copy)]
enum Slot {
    Unknown,
    Missing,
    Found(Hit),
}

impl Slot {
    /// The rule's first match at or after `pos`, searching again only when
    /// the last one has been passed.
    fn at(&mut self, pos: usize, search: impl FnOnce() -> Option<Hit>) -> Option<Hit> {
        match *self {
            Slot::Found(hit) if hit.start() >= pos => Some(hit),
            Slot::Missing => None,
            Slot::Unknown | Slot::Found(_) => {
                let found = search();
                *self = found.map_or(Slot::Missing, Slot::Found);
                found
            }
        }
    }
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
    fn run(&mut self, start: usize, steps: Steps) -> Finished {
        let len = self.bytes.len();
        let mut text = rules::Line::new(self.bytes, &steps);
        let mut slots = Slots::UNKNOWN;
        let mut pos = start;
        while pos < len {
            steps.spend(1);
            if steps.exhausted() {
                self.report.record(Limit::Work);
                break;
            }
            let from = pos;
            pos = match Kind::from_code(self.frame.kind) {
                Kind::Top => self.top(&mut text, &mut slots, pos),
                Kind::Single => self.string(pos, Quote::Single),
                Kind::Double => self.string(pos, Quote::Double),
            };
            debug_assert!(pos > from, "the INI lexer made no progress at byte {from}");
        }
        self.tok(len, &[]);
        steps.finish(self.state)
    }

    /// Finds the next token outside a string at or after `pos` and returns
    /// where scanning goes on.
    ///
    /// The match that starts first wins.
    /// Of matches that start at one byte, the rule the grammar lists first
    /// wins, and it is the first in `found`.
    /// `min_by_key` keeps the first of equal keys.
    fn top(&mut self, text: &mut rules::Line<'_>, slots: &mut Slots, pos: usize) -> usize {
        let found = [
            slots.hash.at(pos, || text.comment(pos, CommentMark::Hash)),
            slots
                .semicolon
                .at(pos, || text.comment(pos, CommentMark::Semicolon)),
            slots.key.at(pos, || text.key(pos)),
            slots
                .section
                .at(pos, || if pos == 0 { text.section() } else { None }),
            slots.single.at(pos, || text.quote(pos, Quote::Single)),
            slots.double.at(pos, || text.quote(pos, Quote::Double)),
        ];
        let Some(hit) = found.into_iter().flatten().min_by_key(|hit| hit.start()) else {
            return self.bytes.len();
        };
        match hit {
            Hit::Comment { start, mark, kind } => {
                let scope = kind.scope();
                self.tok(start, &[]);
                self.tok(mark, &[S::LeadingSpace]);
                self.tok(mark + 1, &[scope, S::PunctComment]);
                self.tok(self.bytes.len(), &[scope]);
                self.bytes.len()
            }
            Hit::Key { start, end, equals } => {
                self.tok(start, &[]);
                self.tok(end, &[S::Key]);
                self.tok(equals, &[]);
                self.tok(equals + 1, &[S::KeyValue]);
                equals + 1
            }
            Hit::Section { close } => {
                self.tok(1, &[S::Section, S::PunctEntity]);
                self.tok(close, &[S::Section]);
                self.tok(close + 1, &[S::Section, S::PunctEntity]);
                close + 1
            }
            Hit::Quote { at, quote } => self.open(at, quote),
        }
    }

    /// Scans a string's text from `pos` for its closing quote, and for an
    /// escape when the string has them.
    fn string(&mut self, pos: usize, quote: Quote) -> usize {
        let rest = &self.bytes[pos..];
        let found = if quote.escapes() {
            memchr2(quote.byte(), b'\\', rest)
        } else {
            memchr(quote.byte(), rest)
        };
        let Some(found) = found else {
            return self.bytes.len();
        };
        let at = pos + found;
        if self.bytes[at] == quote.byte() {
            self.tok(at, &[]);
            self.tok(at + 1, &[S::PunctEnd]);
            self.state = self.frame.parent;
            self.frame = self.arena.frame(self.state);
            return at + 1;
        }
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
    fn open(&mut self, at: usize, quote: Quote) -> usize {
        let open = usize::try_from(self.frame.depth.saturating_sub(1)).unwrap_or(usize::MAX);
        if self.limits.depth_reached(open) {
            self.report.record(Limit::Depth);
            return at + 1;
        }
        let (kind, scope) = quote.kind();
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
        self.tok(at + 1, &[S::PunctBegin]);
        at + 1
    }

    /// Emits the bytes up to `to` under the frame's stack with `leaf` on top.
    fn tok(&mut self, to: usize, leaf: &[S]) {
        let to = to.min(self.bytes.len());
        if to <= self.pos {
            return;
        }
        let stack = self.arena.path(
            self.frame.stack,
            leaf.iter().map(|scope| scope.scope()),
            self.report,
        );
        self.out.push(Token {
            end: to,
            stack: Stack(stack),
        });
        self.pos = to;
    }
}

#[cfg(test)]
mod tests;
