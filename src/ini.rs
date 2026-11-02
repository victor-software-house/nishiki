//! INI, with the scopes of the `ini` grammar that Shiki bundles.
//!
//! Every rule of the grammar is a regular expression that can match at any
//! position of a line, and the line's next token is the match that starts
//! first. The lexer keeps each rule's next match (see `rules`) until the
//! cursor passes it, so a line is scanned once. A string runs across lines;
//! an open string is the one frame the lexer pushes. A comment runs to the
//! end of its line.
//!
//! It reproduces Shiki's output, quirks included: a `#` or `;` starts a
//! comment anywhere outside a string, so `a = b # c` has a comment; a
//! `key = value` is found anywhere in a line, so `a = b = c` has two keys; a
//! double-quoted string has no escapes; and a section header is read only at
//! the start of a line.
//!
//! An empty line leaves the state as it is, because Shiki does not tokenize
//! empty lines.

mod rules;
mod scope;

use memchr::{memchr, memchr2};
use rules::Hit;
pub(crate) use scope::NAMES;
use scope::S;

use crate::arena::{Arena, Frame, ROOT};
use crate::limits::{Limit, Limits, LineReport};
use crate::utf8::char_len;
use crate::{Stack, Token};

/// The high byte that marks a frame as one of the INI lexer's.
const TAG: u16 = crate::tag::INI;

/// What a frame is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    /// Outside a string.
    Top,
    /// Inside `'`.
    Single,
    /// Inside `"`.
    Double,
}

impl Kind {
    fn code(self) -> u16 {
        TAG | match self {
            Kind::Top => 0,
            Kind::Single => 1,
            Kind::Double => 2,
        }
    }

    fn from_code(code: u16) -> Kind {
        match code & !TAG {
            1 => Kind::Single,
            2 => Kind::Double,
            _ => Kind::Top,
        }
    }
}

/// The state at the start of an INI document whose scopes sit on the stack
/// `base`: one context with `source.ini` on top.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    let stack = arena.child(base, S::Source.scope()).unwrap_or(base);
    arena.push(ROOT, Kind::Top.code(), 0, stack).unwrap_or(ROOT)
}

/// The state at the start of the INI lines of a Markdown fence, whose text
/// sits on the stack `base` with no `source.ini` of its own, as in Shiki's
/// embedding.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    arena.push(ROOT, Kind::Top.code(), 0, base).unwrap_or(ROOT)
}

/// A line and where lexing it starts: byte `start`, where a Markdown fence's
/// text begins after the indentation of the containers around it. The bytes
/// before it stay in the line, because `^` matches only at 0.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Text<'a> {
    pub(crate) bytes: &'a [u8],
    pub(crate) start: usize,
}

/// Lexes one line from `state`, appending its tokens with ends measured from
/// the line's first byte, and returns the state after it.
pub(crate) fn line(
    arena: &mut Arena,
    limits: &Limits,
    line: Text<'_>,
    state: u32,
    out: &mut Vec<Token>,
    report: &mut LineReport,
) -> u32 {
    let Text { bytes, start } = line;
    if bytes.len() <= start {
        return state;
    }
    let budget = limits
        .work
        .map_or(u64::MAX, |work| work.steps(bytes.len() - start));
    let frame = arena.frame(state);
    let mut lexer = Lexer {
        arena,
        limits,
        out,
        report,
        bytes,
        state,
        frame,
        pos: start,
    };
    lexer.run(start, budget);
    lexer.state
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
    fn run(&mut self, start: usize, budget: u64) {
        let len = self.bytes.len();
        let mut text = rules::Line::new(self.bytes);
        let mut slots = [Slot::Unknown; 6];
        let mut pos = start;
        let mut steps = 0_u64;
        while pos < len {
            steps = steps.saturating_add(1);
            if steps.saturating_add(text.spent) > budget {
                self.report.record(Limit::Work);
                break;
            }
            let from = pos;
            pos = match Kind::from_code(self.frame.kind) {
                Kind::Top => self.top(&mut text, &mut slots, pos),
                Kind::Single => self.string(pos, b'\'', true),
                Kind::Double => self.string(pos, b'"', false),
            };
            debug_assert!(pos > from, "the INI lexer made no progress at byte {from}");
        }
        self.tok(len, &[]);
    }

    /// Finds the next token outside a string at or after `pos` and returns
    /// where scanning goes on.
    fn top(&mut self, text: &mut rules::Line<'_>, slots: &mut [Slot; 6], pos: usize) -> usize {
        let [hash, semicolon, key, section, single, double] = slots;
        let found = [
            hash.at(pos, || text.comment(pos, b'#')),
            semicolon.at(pos, || text.comment(pos, b';')),
            key.at(pos, || text.key(pos)),
            section.at(pos, || if pos == 0 { text.section() } else { None }),
            single.at(pos, || text.quote(pos, b'\'')),
            double.at(pos, || text.quote(pos, b'"')),
        ];
        let Some((index, hit)) = found
            .into_iter()
            .enumerate()
            .filter_map(|(index, hit)| Some((index, hit?)))
            .min_by_key(|&(index, hit)| (hit.start(), index))
        else {
            return self.bytes.len();
        };
        match hit {
            Hit::Comment { start, mark } => {
                let scope = if index == 0 {
                    S::CommentHash
                } else {
                    S::CommentSemicolon
                };
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
            Hit::Quote { at } => self.open(at, index == 4),
        }
    }

    /// Scans a string's text from `pos` for its closing quote, and for an
    /// escape when the string has them.
    fn string(&mut self, pos: usize, quote: u8, escapes: bool) -> usize {
        let rest = &self.bytes[pos..];
        let found = if escapes {
            memchr2(quote, b'\\', rest)
        } else {
            memchr(quote, rest)
        };
        let Some(found) = found else {
            return self.bytes.len();
        };
        let at = pos + found;
        if self.bytes[at] == quote {
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
    fn open(&mut self, at: usize, single: bool) -> usize {
        let depth = usize::try_from(self.frame.depth).unwrap_or(usize::MAX);
        if self.limits.depth.is_some_and(|max| depth >= max) {
            self.report.record(Limit::Depth);
            return at + 1;
        }
        let (kind, scope) = if single {
            (Kind::Single, S::Single)
        } else {
            (Kind::Double, S::Double)
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
        self.tok(at + 1, &[S::PunctBegin]);
        at + 1
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
    };
    self::line(cx.arena, cx.limits, text, state, cx.out, cx.report)
}
