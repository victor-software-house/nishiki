//! The per-line engine: it runs the grammar's rules the way a `TextMate` engine
//! does, with hand-written matchers instead of regular expressions.
//!
//! At each position the engine tries the end pattern of the innermost rule and
//! then the rule's patterns in order, and takes the first that matches at the
//! earliest position. A begin match opens a rule and an end match closes it.
//! The corner cases of `TextMate` engines are kept because Shiki's output
//! depends on them: `\G` matches only where the enclosing begin ended, the
//! search goes on past the line feed to the end of the text, and a rule that
//! opens or closes without advancing stops the line.

use std::cell::Cell;

use super::rules::{Pat, patterns};
use super::scope::S;
use super::state::Rule;
use crate::arena::{Arena, ROOT};
use crate::language::{Context, Line};
use crate::limits::{Limit, Limits, LineReport};
use crate::utf8::char_len;
use crate::{Stack, Token};

/// A capture group of a match that adds a scope.
#[derive(Clone, Copy, Debug)]
pub(super) struct Cap {
    pub(super) from: usize,
    pub(super) to: usize,
    pub(super) scope: S,
}

/// A match: the bytes `start..end` and the capture groups in order.
#[derive(Clone, Copy, Debug)]
pub(super) struct Hit {
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) caps: [Option<Cap>; 4],
}

impl Hit {
    pub(super) fn new(start: usize, end: usize) -> Hit {
        Hit {
            start,
            end,
            caps: [None; 4],
        }
    }

    /// The match with a group `from..to` that adds `scope`, in slot `slot`.
    pub(super) fn cap(mut self, slot: usize, from: usize, to: usize, scope: S) -> Hit {
        self.caps[slot] = Some(Cap { from, to, scope });
        self
    }
}

/// An open rule.
#[derive(Clone, Copy, Debug)]
struct Frame {
    rule: Rule,
    /// Where the rule opened in this line; `None` for a rule that an earlier
    /// line opened.
    enter: Option<usize>,
    /// The `\G` position when the rule opened.
    anchor: Option<usize>,
    /// Interned scope stack of the rule's begin and end.
    name: u32,
    /// Interned scope stack of the rule's content.
    content: u32,
    /// The interned state with this rule on top. A rule that a limit kept
    /// out of the arena has its parent's.
    state: u32,
}

/// A byte that closes a match, which the engine searches for once per line
/// however many openers there are.
#[derive(Clone, Copy, Debug)]
pub(super) enum Closer {
    /// The `'` that closes a single-quoted string.
    Quote,
    /// The `}` that closes an interpolation.
    Brace,
}

/// What the search found.
enum Found {
    End(Hit),
    Pat(Pat, Hit),
}

/// Lexes one line.
pub(super) struct Lexer<'a> {
    pub(super) s: &'a [u8],
    /// The text ends here.
    pub(super) lim: usize,
    /// Whether a line feed follows the text, which the regular expressions of
    /// the grammar see at the end of a line.
    pub(super) nl: bool,
    out: &'a mut Vec<Token>,
    arena: &'a mut Arena,
    limits: &'a Limits,
    report: &'a mut LineReport,
    /// Steps spent on this line, counted in scans too.
    steps: Cell<u64>,
    /// Steps this line may spend; `u64::MAX` for no limit.
    budget: u64,
    /// End of the last token.
    last: usize,
    /// Where the current search starts.
    pub(super) pos: usize,
    anchor: Option<usize>,
    /// Whether `\G` matches at `pos`.
    pub(super) g: bool,
    /// The last search for each [`Closer`]: where it started and where it
    /// found the byte, so a line of openers that never close does not scan
    /// to its end for each.
    quote_find: Cell<(usize, Option<usize>)>,
    brace_find: Cell<(usize, Option<usize>)>,
    /// The last run of word characters measured: where it starts and ends.
    /// The rules of a word ask about the same one in turn.
    pub(super) run: Cell<(usize, usize)>,
    /// The open rules, the root first.
    frames: Vec<Frame>,
}

impl<'a> Lexer<'a> {
    pub(super) fn new(cx: &'a mut Context<'_>, line: Line<'a>, state: u32) -> Self {
        let (s, start) = (line.text.as_bytes(), line.start);
        let budget = cx.limits.work.map_or(u64::MAX, |work| work.steps(s.len()));
        let mut lexer = Lexer {
            s,
            lim: s.len(),
            nl: true,
            out: &mut *cx.out,
            arena: &mut *cx.arena,
            limits: cx.limits,
            report: &mut *cx.report,
            steps: Cell::new(0),
            budget,
            last: start,
            pos: start,
            anchor: line.embedded.then_some(start),
            g: false,
            quote_find: Cell::new((usize::MAX, None)),
            brace_find: Cell::new((usize::MAX, None)),
            run: Cell::new((usize::MAX, 0)),
            frames: Vec::new(),
        };
        // The open rules of `state`, from its root down.
        let mut chain = Vec::new();
        let mut id = state;
        loop {
            let frame = lexer.arena.frame(id);
            chain.push((id, frame));
            if Rule::from_code(frame.kind) == Rule::Root || frame.parent == ROOT {
                break;
            }
            id = frame.parent;
        }
        for (index, &(id, frame)) in chain.iter().rev().enumerate() {
            if index == 0 {
                lexer.frames.push(Frame {
                    rule: Rule::Root,
                    enter: None,
                    anchor: None,
                    name: frame.stack,
                    content: frame.stack,
                    state: id,
                });
                continue;
            }
            let rule = Rule::from_code(frame.kind);
            let parent = lexer.top().content;
            let name = lexer.path(parent, rule.name());
            lexer.frames.push(Frame {
                rule,
                enter: None,
                anchor: None,
                name,
                content: frame.stack,
                state: id,
            });
        }
        lexer
    }

    /// Lexes the line and returns the state after it.
    pub(super) fn run(mut self) -> u32 {
        self.tokenize();
        self.top().state
    }

    // Bytes.

    /// The byte at `i`, with the line feed at the end of a line, or 0 where no
    /// character stands.
    pub(super) fn at(&self, i: usize) -> u8 {
        if i < self.lim {
            self.s[i]
        } else if i == self.lim && self.nl {
            b'\n'
        } else {
            0
        }
    }

    /// The byte before `i`, or `None` at the start.
    pub(super) fn prev(&self, i: usize) -> Option<u8> {
        if i == 0 { None } else { Some(self.at(i - 1)) }
    }

    /// The first `closer` at or after `from` in the text, answered from the
    /// last search for it when that covers `from`.
    pub(super) fn find_closer(&self, closer: Closer, from: usize) -> Option<usize> {
        let (cell, byte) = match closer {
            Closer::Quote => (&self.quote_find, b'\''),
            Closer::Brace => (&self.brace_find, b'}'),
        };
        let (start, found) = cell.get();
        if start <= from && found.is_none_or(|at| at >= from) {
            return found;
        }
        let rest = self.s.get(from..self.lim)?;
        let found = memchr::memchr(byte, rest).map(|at| at + from);
        self.spend(found.map_or(rest.len(), |at| at - from));
        cell.set((from, found));
        found
    }

    /// Counts `steps` of work against the line's budget.
    pub(super) fn spend(&self, steps: usize) {
        let steps = u64::try_from(steps).unwrap_or(u64::MAX);
        self.steps.set(self.steps.get().saturating_add(steps));
    }

    fn over_budget(&self) -> bool {
        self.steps.get() > self.budget
    }

    // Scopes and tokens.

    fn top(&self) -> Frame {
        self.frames[self.frames.len() - 1]
    }

    /// The stack `base` with `scopes` on top; a node limit keeps the part
    /// that does not fit off.
    fn path(&mut self, base: u32, scopes: &[S]) -> u32 {
        let mut stack = base;
        for scope in scopes {
            let Some(child) = self.arena.child(stack, scope.scope()) else {
                self.report.record(Limit::Nodes);
                break;
            };
            stack = child;
        }
        stack
    }

    /// Ends a token at `to` with `stack`.
    fn tok(&mut self, stack: u32, to: usize) {
        let to = to.min(self.lim);
        if to > self.last {
            self.out.push(Token {
                end: to,
                stack: Stack(stack),
            });
            self.last = to;
        }
    }

    // The loop.

    /// Takes matches until nothing matches at or after the position, as
    /// `TextMate` does, or a guard against a loop that does not advance stops
    /// the line.
    ///
    /// A step that does not advance opens a rule that is not yet open at the
    /// byte or closes one, because a rule that opens again where it opened, or
    /// closes where it opened, ends the line. A run of such steps is then at
    /// most one open and one close per rule of the grammar, plus one close per
    /// rule that was open when the run began; the debug assertion holds the
    /// loop to that bound.
    fn tokenize(&mut self) {
        let mut from = self.pos;
        let mut open = self.frames.len();
        let mut stalled = 0;
        loop {
            self.spend(1);
            if self.over_budget() {
                self.report.record(Limit::Work);
                self.finish();
                return;
            }
            self.g = self.anchor == Some(self.pos);
            let Some(found) = self.search() else {
                if self.over_budget() {
                    self.report.record(Limit::Work);
                }
                self.finish();
                return;
            };
            let stopped = match found {
                Found::End(hit) => self.close(&hit),
                Found::Pat(pat, hit) => self.apply(pat, &hit),
            };
            if stopped {
                return;
            }
            if self.pos > from {
                from = self.pos;
                open = self.frames.len();
                stalled = 0;
            } else {
                stalled += 1;
                debug_assert!(
                    stalled <= open + 2 * Rule::ALL.len(),
                    "the SQL lexer made no progress at byte {from}"
                );
            }
        }
    }

    /// Gives the rest of the text to the innermost rule's content.
    fn finish(&mut self) {
        let stack = self.top().content;
        self.tok(stack, self.lim);
    }

    /// The first match from the position: the end pattern of the innermost
    /// rule first at each position, then its patterns in order. A bracket's
    /// patterns come before its end pattern.
    fn search(&self) -> Option<Found> {
        let rule = self.top().rule;
        let end = self.lim + usize::from(self.nl);
        let mut p = self.pos;
        while p <= end {
            self.spend(1);
            if self.over_budget() {
                return None;
            }
            p = self.skip(rule, p);
            if p > end {
                break;
            }
            if p >= self.lim || self.s[p] & 0xc0 != 0x80 {
                if rule == Rule::Bracket {
                    if let Some(found) = self.try_patterns(rule, p) {
                        return Some(found);
                    }
                    if let Some(hit) = self.end_at(rule, p) {
                        return Some(Found::End(hit));
                    }
                } else {
                    if let Some(hit) = self.end_at(rule, p) {
                        return Some(Found::End(hit));
                    }
                    if let Some(found) = self.try_patterns(rule, p) {
                        return Some(found);
                    }
                }
            }
            p += 1;
        }
        None
    }

    /// The first pattern of `rule` that matches at `p`.
    fn try_patterns(&self, rule: Rule, p: usize) -> Option<Found> {
        patterns(rule)
            .iter()
            .find_map(|&pat| self.pat_at(pat, p).map(|hit| Found::Pat(pat, hit)))
    }

    /// Applies an end match; true when the line stops.
    fn close(&mut self, hit: &Hit) -> bool {
        let top = self.top();
        self.tok(top.content, hit.start);
        self.captures(top.name, hit);
        self.tok(top.name, hit.end);
        let advanced = hit.end > self.pos;
        if !advanced && top.enter == Some(self.pos) {
            // The rule opened and closed without advancing: the grammar is
            // taken to mean that it stays open, for the rest of the line.
            self.tok(top.name, self.lim);
            return true;
        }
        self.frames.pop();
        self.anchor = top.anchor;
        self.pos = self.pos.max(hit.end);
        false
    }

    /// Applies a pattern's match; true when the line stops.
    fn apply(&mut self, pat: Pat, hit: &Hit) -> bool {
        let top = self.top();
        self.tok(top.content, hit.start);
        let Some(rule) = pat.opens() else {
            let name = self.path(top.content, pat.name());
            self.captures(name, hit);
            self.tok(name, hit.end);
            self.pos = self.pos.max(hit.end);
            return false;
        };
        let depth = self.frames.len();
        if self.limits.depth.is_some_and(|max| depth >= max) {
            self.report.record(Limit::Depth);
            // The text stays plain; a rule that matched nothing would match
            // again where it did, so the search goes on after one character.
            let next = if hit.end > hit.start {
                hit.end
            } else {
                hit.start + self.s.get(hit.start).map_or(1, |&lead| char_len(lead))
            };
            let next = next.min(self.lim);
            self.tok(top.content, next);
            self.pos = self.pos.max(next);
            return false;
        }
        if hit.end <= self.pos && self.same_rule_here(rule) {
            // The rule opened again where it had opened without advancing.
            self.finish();
            return true;
        }
        let name = self.path(top.content, rule.name());
        let content = name;
        let state = self
            .arena
            .push(top.state, rule.code(), 0, content)
            .unwrap_or_else(|| {
                self.report.record(Limit::Nodes);
                top.state
            });
        self.frames.push(Frame {
            rule,
            enter: Some(self.pos),
            anchor: self.anchor,
            name,
            content,
            state,
        });
        self.captures(name, hit);
        self.tok(name, hit.end);
        self.anchor = Some(hit.end);
        self.pos = self.pos.max(hit.end);
        false
    }

    /// Whether an open rule entered at the position is `rule`, looking down
    /// the rules that entered there.
    fn same_rule_here(&self, rule: Rule) -> bool {
        self.frames
            .iter()
            .rev()
            .take_while(|frame| frame.enter == Some(self.pos))
            .any(|frame| frame.rule == rule)
    }

    /// Emits a match's capture groups over `base`, the stack of the text of
    /// the match outside them.
    fn captures(&mut self, base: u32, hit: &Hit) {
        for cap in hit.caps.iter().flatten() {
            if cap.from >= cap.to {
                continue;
            }
            self.tok(base, cap.from);
            let stack = self.path(base, &[cap.scope]);
            self.tok(stack, cap.to);
        }
    }
}
