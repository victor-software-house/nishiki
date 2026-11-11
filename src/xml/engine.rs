//! The per-line engine.
//! It runs the grammar's rules the way a `TextMate` engine does, with
//! hand-written matchers instead of regular expressions.
//!
//! At each position the engine tries the end pattern of the innermost rule,
//! then the rule's patterns in order.
//! It takes the first that matches at the earliest position.
//! A begin match opens a rule, and an end match closes it.
//! Every pattern of this grammar consumes at least one byte.
//! A match therefore always advances the position.

use std::cell::Cell;

use super::rules::{Pattern, next_start, patterns};
use super::scope::S;
use super::state::Rule;
use crate::arena::{Arena, ROOT};
use crate::language::{Context, Halts, Line, StopReason, Stopped};
use crate::limits::{Finished, Limit, Limits, LineReport, Steps};
use crate::scratch::Scratch;
use crate::{Stack, Token, java};

/// The most capture groups a match carries.
/// The end of an empty tag has a three-part name between
/// three punctuation marks.
const CAPTURES: usize = 7;

/// The end of a `<% ... %>`.
/// It halts the Java inside.
fn jsp_end(s: &[u8], p: usize) -> bool {
    s.get(p..).is_some_and(|rest| rest.starts_with(b"%>"))
}

/// A capture group that carries a scope.
/// `inner` is a second group over the same bytes, nested in it.
#[derive(Clone, Copy, Debug)]
pub(super) struct Capture {
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
    captures: [Option<Capture>; CAPTURES],
}

impl Hit {
    pub(super) fn new(start: usize, end: usize) -> Hit {
        Hit {
            start,
            end,
            captures: [None; CAPTURES],
        }
    }

    /// The match with a group `from..to` that adds `scope`.
    pub(super) fn capture(self, from: usize, to: usize, scope: S) -> Hit {
        self.group(Capture {
            from,
            to,
            scope,
            inner: None,
        })
    }

    /// The match with a group `from..to` that adds `scope` and, over the same
    /// bytes, a nested group that adds `inner`.
    pub(super) fn nested(self, from: usize, to: usize, scope: S, inner: S) -> Hit {
        self.group(Capture {
            from,
            to,
            scope,
            inner: Some(inner),
        })
    }

    fn group(mut self, capture: Capture) -> Hit {
        if let Some(slot) = self.captures.iter_mut().find(|slot| slot.is_none()) {
            *slot = Some(capture);
        }
        self
    }
}

/// An open rule.
#[derive(Clone, Copy, Debug)]
struct Frame {
    rule: Rule,
    /// Interned scope stack of the rule's begin, content and end.
    stack: u32,
    /// The interned state with this rule on top.
    /// A rule that a limit kept out of the arena has its parent's.
    state: u32,
}

/// What the search found.
enum Found {
    End(Hit),
    Pattern(Pattern, Hit),
}

/// What the highlighter keeps from one line to the next.
/// Lexing a line then allocates nothing once these have grown.
#[derive(Debug, Default)]
pub(crate) struct Buffers {
    frames: Vec<Frame>,
    chain: Vec<(u32, crate::arena::Frame)>,
}

/// Lexes one line.
pub(super) struct Lexer<'a> {
    pub(super) s: &'a [u8],
    /// The same text, for the Java lexer.
    text: &'a str,
    scratch: &'a mut Scratch,
    /// The text ends here.
    /// The line feed after it is one more character.
    pub(super) lim: usize,
    out: &'a mut Vec<Token>,
    arena: &'a mut Arena,
    limits: &'a Limits,
    report: &'a mut LineReport,
    /// Steps spent on this line, counted in scans too, and lent to the
    /// languages this one embeds.
    steps: Steps,
    /// End of the last token.
    last: usize,
    /// Where the current search starts.
    pos: usize,
    /// The open rules, the root first.
    frames: Vec<Frame>,
    chain: Vec<(u32, crate::arena::Frame)>,
    /// The last run of white space found: where it was found from, and where
    /// it ends.
    /// A scan from any byte inside it is then one lookup.
    pub(super) space_run: Cell<(usize, usize)>,
    /// The last run of name characters found, in the same way.
    pub(super) name_run: Cell<(usize, usize)>,
    /// The last stretch without a `>`: where it was found from, and where the
    /// `>` or the end of the text is.
    pub(super) angle_run: Cell<(usize, usize)>,
    /// Where the language that embeds XML takes the line back, while only the
    /// root is open.
    /// `end` is tried before the root's patterns at a place.
    /// `after` is tried where none of them matches.
    /// See [`Halt`].
    pub(super) halts: Option<&'a Halts<'a>>,
    /// Where the line went back to the embedding language.
    halted: Option<usize>,
}

impl<'a> Lexer<'a> {
    /// A lexer for `line` from `state`, with the open rules of `state` as its
    /// frames, from the root down.
    ///
    /// Out of line, XML rose 5% (1.0509).
    #[expect(
        clippy::inline_always,
        reason = "`lex_until`, a second caller, kept it out of line at one codegen unit"
    )]
    #[inline(always)]
    pub(super) fn new(cx: &'a mut Context<'_>, line: Line<'a>, state: u32) -> Self {
        let (s, start) = (line.text.as_bytes(), line.start);
        let steps = Steps::new(cx.limits, s.len());
        let buffers = std::mem::take(&mut cx.scratch.xml);
        let mut lexer = Lexer {
            s,
            text: line.text,
            scratch: &mut *cx.scratch,
            lim: s.len(),
            out: &mut *cx.out,
            arena: &mut *cx.arena,
            limits: cx.limits,
            report: &mut *cx.report,
            steps,
            last: start,
            pos: start,
            frames: buffers.frames,
            chain: buffers.chain,
            space_run: Cell::new((0, 0)),
            name_run: Cell::new((0, 0)),
            angle_run: Cell::new((0, 0)),
            halts: None,
            halted: None,
        };
        lexer.frames.clear();
        lexer.chain.clear();
        let mut id = state;
        loop {
            let frame = lexer.arena.frame(id);
            lexer.chain.push((id, frame));
            if Rule::from_code(frame.rule) == Rule::Root || frame.parent == ROOT {
                break;
            }
            id = frame.parent;
        }
        for index in (0..lexer.chain.len()).rev() {
            let (id, frame) = lexer.chain[index];
            lexer.frames.push(Frame {
                rule: if index + 1 == lexer.chain.len() {
                    Rule::Root
                } else {
                    Rule::from_code(frame.rule)
                },
                stack: frame.stack,
                state: id,
            });
        }
        lexer
    }

    /// Lexes the line and returns the state after it.
    ///
    /// It is `#[inline]`, because `xml::lex`, in another module, calls it once
    /// a line.
    /// [`run_until`](Self::run_until) moves the lexer's buffers out.
    /// The lexer then needs no drop glue where it is inlined.
    #[inline]
    pub(super) fn run(self) -> Finished {
        self.run_until().0
    }

    /// Lexes the line and returns the state after it, with where it went back
    /// to the embedding language, if it did.
    ///
    /// Out of line, XML rose 5% (1.0509).
    #[expect(
        clippy::inline_always,
        reason = "`lex_until`, a second caller, kept it out of line at one codegen unit"
    )]
    #[inline(always)]
    pub(super) fn run_until(mut self) -> (Finished, Option<usize>) {
        self.tokenize();
        let state = self.top().state;
        let Lexer {
            scratch,
            frames,
            chain,
            steps,
            halted,
            ..
        } = self;
        scratch.xml = Buffers { frames, chain };
        (steps.finish(state), halted)
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
        self.steps.spend(steps);
    }

    fn over_budget(&self) -> bool {
        self.steps.exhausted()
    }

    // Scopes and tokens.

    fn top(&self) -> Frame {
        self.frames[self.frames.len() - 1]
    }

    /// The stack `base` with `scope` on top.
    /// A node limit keeps off a scope that does not fit.
    fn path(&mut self, base: u32, scope: Option<S>) -> u32 {
        let Some(scope) = scope else {
            return base;
        };
        self.arena.child(base, scope.scope()).unwrap_or_else(|| {
            self.report.record(Limit::Nodes);
            base
        })
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

    /// Takes matches until nothing matches at or after the position, or the
    /// budget runs out.
    fn tokenize(&mut self) {
        loop {
            self.spend(1);
            if self.over_budget() {
                self.report.record(Limit::Work);
                self.finish();
                return;
            }
            let before = self.pos;
            let found = if self.top().rule == Rule::Jsp {
                self.jsp()
            } else {
                self.search()
            };
            if self.halts.is_some()
                && let Some(at) = self.halt_before(found.as_ref())
            {
                let stack = self.top().stack;
                self.tok(stack, at);
                self.pos = at;
                self.halted = Some(at);
                return;
            }
            let Some(found) = found else {
                if self.over_budget() {
                    self.report.record(Limit::Work);
                }
                self.finish();
                return;
            };
            match found {
                Found::End(hit) => self.close(&hit),
                Found::Pattern(pattern, hit) => self.apply(pattern, &hit),
            }
            debug_assert!(
                self.pos > before,
                "every pattern consumes a byte, so a match advances"
            );
        }
    }

    /// Where `halts` takes the line back before `found`, the next match, while
    /// only the root is open (see [`crate::language::halt_before`]).
    ///
    /// `#[cold]`: only XML that another language embeds has halts, and kept
    /// out of `tokenize`, the test costs a document one branch per match.
    #[cold]
    fn halt_before(&self, found: Option<&Found>) -> Option<usize> {
        let &(end, after) = self.halts?;
        if self.frames.len() != 1 || self.over_budget() {
            return None;
        }
        let to = found.map_or(self.lim, |found| match found {
            Found::End(hit) | Found::Pattern(_, hit) => hit.start,
        });
        let (halted, scanned) =
            crate::language::halt_before(self.s, self.pos, to, self.lim, end, after);
        self.spend(scanned);
        halted
    }

    /// Gives the rest of the text to the innermost rule's content.
    fn finish(&mut self) {
        let stack = self.top().stack;
        self.tok(stack, self.lim);
    }

    /// The first match from the position: at each position the innermost
    /// rule's end pattern and then its patterns in order.
    fn search(&self) -> Option<Found> {
        let rule = self.top().rule;
        let mut p = self.pos;
        while let Some(at) = next_start(rule, self.s, p) {
            self.spend((at - p).div_ceil(8) + 1);
            if self.over_budget() {
                return None;
            }
            p = at;
            if let Some(hit) = self.end_at(rule, p) {
                return Some(Found::End(hit));
            }
            for &pattern in patterns(rule) {
                if let Some(hit) = self.pattern_at(pattern, p) {
                    return Some(Found::Pattern(pattern, hit));
                }
            }
            p = self.after_miss(rule, p);
        }
        None
    }

    /// Where the search resumes after nothing matched at `p`.
    /// In a rule whose patterns start at white space, a run of it is tried
    /// once.
    /// Every position in the run ends in the same place.
    fn after_miss(&self, rule: Rule, p: usize) -> usize {
        match rule {
            Rule::Tag | Rule::NoContent | Rule::Doctype if self.space_at(p) > 0 => {
                self.ws(p).max(p + 1)
            }
            Rule::Root
            | Rule::Instruction
            | Rule::Subset
            | Rule::EntityDeclaration
            | Rule::Comment
            | Rule::CommentBad
            | Rule::CommentJsp
            | Rule::NoContent
            | Rule::Tag
            | Rule::JspDirective
            | Rule::Jsp
            | Rule::Cdata
            | Rule::Doctype
            | Rule::Double
            | Rule::Single => p + 1,
        }
    }

    /// The rest of a `<% ... %>` on this line: the Java up to the first `%>`
    /// that Java is not inside a context of, and then the end.
    /// The Java lexer reads the code.
    /// Its state is the carried value of the rule's frame.
    /// Code that goes on across lines therefore resumes where it stopped.
    ///
    /// The grammar's end pattern is tried only when no Java rule is open.
    /// A `%>` inside a string, a comment or a block therefore belongs to Java.
    /// The Java lexer halts at the first `%>` it reaches with no rule open.
    /// It reads the whole line, as the grammar's lookaheads do.
    fn jsp(&mut self) -> Option<Found> {
        let top = self.top();
        let mut state = self.arena.frame(top.state).carried;
        if state == 0 {
            state = java::start_embedded(self.arena, top.stack);
        }
        let (state, close) = self.java(self.pos, state);
        let to = close.unwrap_or(self.lim);
        self.spend(to - self.pos + 1);
        if to > self.pos {
            self.last = to;
            self.pos = to;
        }
        let Some(at) = close else {
            let parent = self.frames[self.frames.len() - 2].state;
            let next = self
                .arena
                .push(parent, Rule::Jsp.code(), state, top.stack)
                .unwrap_or_else(|| {
                    self.report.record(Limit::Nodes);
                    top.state
                });
            let last = self.frames.len() - 1;
            self.frames[last].state = next;
            return None;
        };
        Some(Found::End(Hit::new(at, at + 2).capture(
            at,
            at + 2,
            S::EmbeddedEnd,
        )))
    }

    /// Lexes the Java of the line from `from` and `state`, up to the `%>`
    /// that ends it, and returns the state after it and where that `%>` is.
    ///
    /// Java's tokens start at the end of `out`.
    /// A pop of the host cuts only tokens Java gave.
    /// So the lexer uses that end as [`Context::first_token`] and keeps no
    /// copy of it per line.
    fn java(&mut self, from: usize, state: u32) -> (u32, Option<usize>) {
        let lent = self.steps.lend(self.limits);
        let first_token = self.out.len();
        let mut cx = Context {
            scratch: &mut *self.scratch,
            arena: &mut *self.arena,
            limits: &lent,
            out: &mut *self.out,
            report: &mut *self.report,
            first_token,
        };
        let line = Line {
            text: self.text,
            start: from,
            embedded: false,
            anchored_at_root: false,
        };
        let (finished, at) = Self::lex_jsp(&mut cx, line, state);
        (self.steps.take_back(finished), at)
    }

    /// Lexes Java up to the `%>` that ends JSP code.
    /// It stays out of line.
    /// `tokenize` then never builds the `&dyn Halt` it passes.
    /// Inlined, that value cost every XML line a store.
    #[cold]
    #[inline(never)]
    fn lex_jsp(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> (Finished, Option<usize>) {
        let (finished, stopped) = java::lex_until(cx, line, state, &jsp_end);
        let at = match stopped {
            Some(Stopped {
                at,
                reason: StopReason::Halt,
            }) => Some(at),
            Some(Stopped {
                reason: StopReason::PopHost,
                ..
            })
            | None => None,
        };
        (finished, at)
    }

    /// Applies an end match.
    fn close(&mut self, hit: &Hit) {
        let stack = self.top().stack;
        self.tok(stack, hit.start);
        self.captures(stack, hit);
        self.tok(stack, hit.end);
        self.frames.pop();
        self.pos = self.pos.max(hit.end);
    }

    /// Applies a pattern's match.
    /// A rule the depth limit keeps from opening leaves its text plain.
    /// The search then goes on after the match.
    /// The depth counts the rules open above the root, as every lexer does.
    fn apply(&mut self, pattern: Pattern, hit: &Hit) {
        let top = self.top();
        self.tok(top.stack, hit.start);
        let Some(rule) = pattern.opens() else {
            let name = self.path(top.stack, pattern.name());
            self.captures(name, hit);
            self.tok(name, hit.end);
            self.pos = self.pos.max(hit.end);
            return;
        };
        if self.limits.depth_reached(self.frames.len() - 1) {
            self.report.record(Limit::Depth);
            let next = hit.end.min(self.lim);
            self.tok(top.stack, next);
            self.pos = self.pos.max(next);
            return;
        }
        let stack = self.path(top.stack, rule.name());
        let state = self
            .arena
            .push(top.state, rule.code(), 0, stack)
            .unwrap_or_else(|| {
                self.report.record(Limit::Nodes);
                top.state
            });
        self.frames.push(Frame { rule, stack, state });
        self.captures(stack, hit);
        self.tok(stack, hit.end);
        self.pos = self.pos.max(hit.end);
    }

    /// Emits a match's capture groups over `base`, the stack of the text of
    /// the match outside them.
    fn captures(&mut self, base: u32, hit: &Hit) {
        for capture in hit.captures.iter().flatten() {
            self.tok(base, capture.from);
            let mut stack = self.path(base, Some(capture.scope));
            if let Some(inner) = capture.inner {
                stack = self.path(stack, Some(inner));
            }
            self.tok(stack, capture.to);
        }
    }
}
