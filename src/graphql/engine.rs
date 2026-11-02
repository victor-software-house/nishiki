//! The per-line engine: it runs the grammar's rules the way a `TextMate`
//! engine does, with hand-written matchers instead of regular expressions.
//!
//! At each position the engine tries the end pattern of the innermost rule and
//! then the rule's patterns in order, and takes the first that matches at the
//! earliest position; a rule marked `applyEndPatternLast` tries its end
//! pattern after its patterns. A begin match opens a rule and an end match
//! closes it. The corner cases of `TextMate` engines are kept because Shiki's
//! output depends on them: the search goes on past the line feed to the end of
//! the text, a rule that opens and closes without advancing stays open for the
//! rest of the line, and a match rule that does not advance closes the rule it
//! sits in.

use std::cell::Cell;

use super::rules::{Pat, patterns};
use super::scope::S;
use super::state::Rule;
use crate::arena::{Arena, ROOT};
use crate::language::{Context, Line};
use crate::limits::{Limit, Limits, LineReport};
use crate::scratch::Scratch;
use crate::utf8::char_len;
use crate::{Stack, Token, typescript};

/// A capture group that carries a scope; `inner` is a second group over the
/// same bytes, nested in it.
#[derive(Clone, Copy, Debug)]
pub(super) struct Cap {
    pub(super) from: usize,
    pub(super) to: usize,
    pub(super) scope: S,
    pub(super) inner: Option<S>,
}

/// A match: the bytes `start..end` and the capture groups in order.
#[derive(Clone, Copy, Debug)]
pub(super) struct Hit {
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) caps: [Option<Cap>; 5],
}

impl Hit {
    pub(super) fn new(start: usize, end: usize) -> Hit {
        Hit {
            start,
            end,
            caps: [None; 5],
        }
    }

    /// The match with a group `from..to` that adds `scope`, in slot `slot`.
    pub(super) fn cap(mut self, slot: usize, from: usize, to: usize, scope: S) -> Hit {
        self.caps[slot] = Some(Cap {
            from,
            to,
            scope,
            inner: None,
        });
        self
    }

    /// The match with a group `from..to` that adds `scope` and, over the same
    /// bytes, a nested group that adds `inner`.
    pub(super) fn nested(mut self, slot: usize, from: usize, to: usize, scope: S, inner: S) -> Hit {
        self.caps[slot] = Some(Cap {
            from,
            to,
            scope,
            inner: Some(inner),
        });
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
    /// Interned scope stack of the rule's begin and end.
    name: u32,
    /// Interned scope stack of the rule's content.
    content: u32,
    /// The interned state with this rule on top. A rule that a limit kept
    /// out of the arena has its parent's.
    state: u32,
}

/// What the search found.
enum Found {
    End(Hit),
    Pat(Pat, Hit),
}

/// Lexes one line.
pub(super) struct Lexer<'a> {
    pub(super) s: &'a [u8],
    /// The same text, for the JavaScript lexer.
    text: &'a str,
    scratch: &'a mut Scratch,
    /// The text ends here; the line feed after it is one more character.
    pub(super) lim: usize,
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
    pos: usize,
    /// The open rules, the root first.
    frames: Vec<Frame>,
    /// The last run of white space found: where it was found from, and where
    /// it ends, so a scan from any byte inside it is one lookup.
    pub(super) space_run: Cell<(usize, usize)>,
    /// The last run of name characters found, in the same way.
    pub(super) word_run: Cell<(usize, usize)>,
}

impl<'a> Lexer<'a> {
    pub(super) fn new(
        scratch: &'a mut Scratch,
        arena: &'a mut Arena,
        limits: &'a Limits,
        text: super::Text<'a>,
        out: &'a mut Vec<Token>,
        state: u32,
        report: &'a mut LineReport,
    ) -> Self {
        let (s, start) = (text.bytes, text.start);
        let budget = limits.work.map_or(u64::MAX, |work| work.steps(s.len()));
        let mut lexer = Lexer {
            s,
            text: text.whole,
            scratch,
            lim: s.len(),
            out,
            arena,
            limits,
            report,
            steps: Cell::new(0),
            budget,
            last: start,
            pos: start,
            frames: Vec::new(),
            space_run: Cell::new((0, 0)),
            word_run: Cell::new((0, 0)),
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

    /// The byte at `i`, with the line feed after the text, or 0 where no
    /// character stands.
    pub(super) fn at(&self, i: usize) -> u8 {
        match i.cmp(&self.lim) {
            std::cmp::Ordering::Less => self.s[i],
            std::cmp::Ordering::Equal => b'\n',
            std::cmp::Ordering::Greater => 0,
        }
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
            let found = if self.top().rule == Rule::Interpolation {
                self.interpolation()
            } else {
                self.search()
            };
            let Some(found) = found else {
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
                    "the GraphQL lexer made no progress at byte {from}"
                );
            }
        }
    }

    /// Gives the rest of the text to the innermost rule's content.
    fn finish(&mut self) {
        let stack = self.top().content;
        self.tok(stack, self.lim);
    }

    /// The first match from the position: at each position the innermost
    /// rule's end pattern and then its patterns in order, or its patterns and
    /// then its end pattern for a rule that applies the end pattern last.
    fn search(&self) -> Option<Found> {
        let rule = self.top().rule;
        let end = self.lim + 1;
        let end_last = Self::end_last(rule);
        let mut p = self.pos;
        while p <= end {
            self.spend(1);
            if self.over_budget() {
                return None;
            }
            if p >= self.lim || self.s[p] & 0xc0 != 0x80 {
                if !end_last && let Some(hit) = self.end_at(rule, p) {
                    return Some(Found::End(hit));
                }
                for &pat in patterns(rule) {
                    if let Some(hit) = self.pat_at(pat, p) {
                        return Some(Found::Pat(pat, hit));
                    }
                }
                if end_last && let Some(hit) = self.end_at(rule, p) {
                    return Some(Found::End(hit));
                }
            }
            p += 1;
        }
        None
    }

    /// The rest of an interpolation on this line: the JavaScript up to the `}`
    /// that no context of the code is waiting for, and then the end. The
    /// JavaScript lexer reads the code, and its state is the payload of the
    /// rule's frame, so an interpolation that goes on across lines resumes
    /// where it stopped.
    fn interpolation(&mut self) -> Option<Found> {
        let top = self.top();
        let mut js = self.arena.frame(top.state).payload;
        if js == 0 {
            js = typescript::start_interpolation(self.arena, top.content);
        }
        self.spend(self.lim.saturating_sub(self.pos) + 1);
        let mut cx = Context {
            scratch: &mut *self.scratch,
            arena: &mut *self.arena,
            limits: self.limits,
            out: &mut *self.out,
            report: &mut *self.report,
        };
        let line = Line {
            text: self.text,
            start: self.pos,
            embedded: false,
        };
        let (next, close) = typescript::lex_interpolation(&mut cx, line, js);
        let to = close.unwrap_or(self.lim);
        if to > self.pos {
            self.last = to;
            self.pos = to;
        }
        let Some(at) = close else {
            let parent = self.frames[self.frames.len() - 2].state;
            let state = self
                .arena
                .push(parent, Rule::Interpolation.code(), next, top.content)
                .unwrap_or_else(|| {
                    self.report.record(Limit::Nodes);
                    top.state
                });
            let last = self.frames.len() - 1;
            self.frames[last].state = state;
            return None;
        };
        Some(Found::End(Hit::new(at, at + 1).cap(
            0,
            at,
            at + 1,
            S::SubstitutionEnd,
        )))
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
            self.tok(top.content, self.lim);
            return true;
        }
        self.frames.pop();
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
            if hit.end <= self.pos {
                // A match rule that does not advance: the rule it sits in is
                // closed and the rest of the line belongs to the rule below.
                if self.frames.len() > 1 {
                    self.frames.pop();
                }
                self.finish();
                return true;
            }
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
        let content = self.path(name, rule.content());
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
            name,
            content,
            state,
        });
        self.captures(name, hit);
        self.tok(name, hit.end);
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
            let mut stack = self.path(base, &[cap.scope]);
            if let Some(inner) = cap.inner {
                stack = self.path(stack, &[inner]);
            }
            self.tok(stack, cap.to);
        }
    }
}
