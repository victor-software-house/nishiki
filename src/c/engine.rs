//! The per-line engine: it runs the grammar's rules the way a `TextMate`
//! engine does, with hand-written matchers instead of regular expressions.
//!
//! At each position the engine tries the end pattern of the innermost rule
//! and then the rule's patterns in order, and takes the first that matches at
//! the earliest position. A begin match opens a rule, an end match closes it,
//! and a capture may be lexed again with its own pattern list. The corner
//! cases of `TextMate` engines are kept because Shiki's output depends on
//! them: `\G` matches only where the enclosing begin ended, and a rule that
//! opens or closes without advancing stops the line.
//!
//! The text a pattern sees is the line and a line feed, as Shiki tokenizes
//! it; a capture that is lexed again sees only the text up to its end.

use std::cell::Cell;

use super::lists::{EOT, Item, Tables, kind};
use super::scope::S;
use super::state::{EOL, R};
use crate::arena::{Arena, Frame};
use crate::limits::{Limit, Limits, LineReport};
use crate::{Stack, Token};

/// A scope whose name is made from the line's text.
#[derive(Clone, Copy, Debug)]
pub(super) struct Dyn {
    pub(super) prefix: &'static str,
    pub(super) start: usize,
    pub(super) end: usize,
}

/// The pattern list a capture is lexed again with.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Retok {
    No,
    Doxy,
    Inline,
    InlineClose,
    Access,
    DefineParams,
    ParamDir,
    NumSep,
    Number,
}

impl Retok {
    fn list(self) -> &'static [Item] {
        use super::lists as l;
        match self {
            Retok::No | Retok::Number => &[],
            Retok::Doxy => l::DOXY,
            Retok::Inline => l::INLINE,
            Retok::InlineClose => l::INLINE_CLOSE,
            Retok::Access => l::ACCESS,
            Retok::DefineParams => l::DEFINE_PARAMS,
            Retok::ParamDir => l::PARAM_DIR,
            Retok::NumSep => l::NUM_SEP,
        }
    }
}

/// A capture of a match: the scopes it adds, or the list it is lexed again
/// with.
#[derive(Clone, Copy, Debug)]
pub(super) struct Span {
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) scopes: &'static [S],
    pub(super) dynamic: Option<Dyn>,
    pub(super) retok: Retok,
}

/// What a match does.
#[derive(Clone, Copy, Debug)]
pub(super) enum Act {
    /// A `match` rule, adding `scopes` (and `dynamic`) to the whole match.
    Plain(&'static [S], Option<Dyn>),
    /// A begin: open the rule.
    Begin(R),
    /// The end of the innermost rule.
    End,
}

/// The match a pattern found.
#[derive(Clone, Copy, Debug)]
pub(super) struct Hit {
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) act: Act,
}

/// A rule opened in this line.
#[derive(Clone, Copy, Debug)]
struct Open {
    rule: R,
    /// Where it was entered.
    enter: usize,
    /// The `\G` position before it was entered.
    anchor: Option<usize>,
}

/// Lexes one line.
pub(super) struct Lexer<'a> {
    pub(super) s: &'a [u8],
    /// The text ends here; a capture lexed again ends before the line does.
    pub(super) lim: usize,
    /// Whether a line feed follows the text.
    pub(super) nl: bool,
    out: &'a mut Vec<Token>,
    arena: &'a mut Arena,
    limits: &'a Limits,
    report: &'a mut LineReport,
    steps: Cell<u64>,
    budget: u64,
    /// End of the last token, which may lie past the line's end.
    last: usize,
    /// Where lexing starts.
    start: usize,
    state: u32,
    frame: Frame,
    open: Vec<Open>,
    /// Where `\G` matches.
    pub(super) anchor: Option<usize>,
    /// The captures of the current match, from `span_base` on.
    pub(super) sp: Vec<Span>,
    span_base: usize,
    pub(super) hit: Hit,
    local: Vec<(u32, usize)>,
    name: String,
    tables: &'static Tables,
    /// Set when the line stops early.
    stopped: bool,
    /// Scratch for the chains of member access.
    pub(super) ends: Vec<usize>,
    /// The last run of white space, of word characters, of ASCII name
    /// characters and of colons found: where it starts and ends. A search from
    /// inside a run ends where the run does, so a run is scanned once.
    pub(super) runs: [std::cell::Cell<(usize, usize)>; 4],
    /// The last scoped name found: where its search started and where it
    /// ended. A later search that steps onto that walk ends where it did.
    pub(super) scoped: std::cell::Cell<(usize, usize)>,
    /// The last word looked up: where it starts, where it ends, and the
    /// lists it is in; an empty set where no word starts.
    pub(super) mask: std::cell::Cell<(usize, usize, u32)>,
    /// Where the line's first directive keyword starts: 0 before it is
    /// looked up, 1 when the line is not a directive, 2 when it is, with the
    /// positions of its `#` and its word.
    pub(super) head: std::cell::Cell<(u8, usize, usize)>,
}

impl<'a> Lexer<'a> {
    pub(super) fn new(
        arena: &'a mut Arena,
        limits: &'a Limits,
        text: super::Text<'a>,
        out: &'a mut Vec<Token>,
        state: u32,
        report: &'a mut LineReport,
    ) -> Self {
        let s = text.bytes;
        let frame = arena.frame(state);
        let anchor = if text.anchored {
            Some(text.start)
        } else {
            (frame.kind & EOL != 0).then_some(0)
        };
        let budget = limits
            .work
            .map_or(u64::MAX, |work| work.steps(s.len() - text.start));
        Lexer {
            s,
            lim: s.len(),
            nl: true,
            out,
            arena,
            limits,
            report,
            steps: Cell::new(0),
            budget,
            last: text.start,
            start: text.start,
            state,
            frame,
            open: Vec::new(),
            anchor,
            sp: Vec::new(),
            span_base: 0,
            hit: Hit {
                start: 0,
                end: 0,
                act: Act::End,
            },
            local: Vec::new(),
            name: String::new(),
            tables: Tables::get(),
            stopped: false,
            ends: Vec::new(),
            runs: [const { std::cell::Cell::new((usize::MAX, 0)) }; 4],
            scoped: std::cell::Cell::new((usize::MAX, 0)),
            mask: std::cell::Cell::new((usize::MAX, 0, 0)),
            head: std::cell::Cell::new((0, 0, 0)),
        }
    }

    pub(super) fn into_state(self) -> u32 {
        self.state
    }

    /// Forgets the runs found, when the text they were found in changes.
    pub(super) fn reset_runs(&self) {
        for run in &self.runs {
            run.set((usize::MAX, 0));
        }
        self.scoped.set((usize::MAX, 0));
        self.mask.set((usize::MAX, 0, 0));
    }

    /// Where the captures of the current match start.
    pub(super) fn span_base(&self) -> usize {
        self.span_base
    }

    /// Whether the innermost frame is one whose begin ran to the end of its
    /// line.
    fn frame_eol(&self) -> bool {
        self.frame.kind & EOL != 0
    }

    fn rule(&self) -> R {
        R::from_code(self.frame.kind)
    }

    /// The scope stack of the innermost rule's begin and end.
    fn name_stack(&self) -> u32 {
        self.frame.payload
    }

    /// The scope stack of the text inside the innermost rule.
    fn content_stack(&self) -> u32 {
        self.frame.stack
    }

    // Text.

    /// The number of characters the text has: the line, and its line feed.
    pub(super) fn len_all(&self) -> usize {
        self.lim + usize::from(self.nl)
    }

    /// The byte at `i`; the line feed after the text, and 0 past it.
    #[inline]
    pub(super) fn b(&self, i: usize) -> u8 {
        if i < self.lim {
            self.s.get(i).copied().unwrap_or(0)
        } else if i == self.lim && self.nl {
            b'\n'
        } else {
            0
        }
    }

    /// The byte before `i`, if any.
    pub(super) fn prev(&self, i: usize) -> Option<u8> {
        let j = i.checked_sub(1)?;
        (j < self.len_all()).then(|| self.b(j))
    }

    /// Whether the text at `i` starts with `lit`.
    pub(super) fn starts(&self, i: usize, lit: &[u8]) -> bool {
        lit.iter().enumerate().all(|(k, &c)| self.b(i + k) == c) && i + lit.len() <= self.len_all()
    }

    /// Counts `bytes` read ahead against the line's work.
    pub(super) fn spend(&self, bytes: usize) {
        let bytes = u64::try_from(bytes).unwrap_or(u64::MAX);
        self.steps.set(self.steps.get().saturating_add(bytes));
    }

    // Emission.

    /// `stack` with `scopes` on top, falling back to `stack` when the node
    /// limit keeps a new stack from being made.
    pub(super) fn push_scopes(&mut self, mut stack: u32, scopes: &[S]) -> u32 {
        for scope in scopes {
            if let Some(child) = self.arena.child(stack, scope.scope()) {
                stack = child;
            } else {
                self.report.record(Limit::Nodes);
                break;
            }
        }
        stack
    }

    fn push_dyn(&mut self, stack: u32, dynamic: Dyn) -> u32 {
        self.name.clear();
        self.name.push_str(dynamic.prefix);
        let text = self.s.get(dynamic.start..dynamic.end).unwrap_or(&[]);
        self.name.push_str(&String::from_utf8_lossy(text));
        self.name.push_str(".c");
        let named = self.arena.named_scope(&self.name);
        let child = named.and_then(|scope| self.arena.child(stack, scope));
        if let Some(child) = child {
            child
        } else {
            self.report.record(Limit::Nodes);
            stack
        }
    }

    fn push_all(&mut self, stack: u32, scopes: &[S], dynamic: Option<Dyn>) -> u32 {
        let stack = self.push_scopes(stack, scopes);
        match dynamic {
            Some(dynamic) => self.push_dyn(stack, dynamic),
            None => stack,
        }
    }

    /// Emits the text up to `to` under `stack`.
    pub(super) fn produce(&mut self, stack: u32, to: usize) {
        if to <= self.last {
            return;
        }
        let end = to.min(self.s.len());
        self.last = to;
        if end == 0 || self.out.last().is_some_and(|token| token.end >= end) {
            return;
        }
        self.out.push(Token {
            end,
            stack: Stack(stack),
        });
    }

    /// Takes the captures of the current match from `base` on.
    pub(super) fn emit_spans(&mut self, base: u32, match_end: usize, from: usize) {
        let to = self.sp.len();
        let local_base = self.local.len();
        for k in from..to {
            let span = self.sp[k];
            if span.end <= span.start {
                continue;
            }
            if span.start > match_end {
                break;
            }
            while let Some(&(stack, end)) = self.local.get(local_base..).and_then(<[_]>::last) {
                if end > span.start {
                    break;
                }
                self.produce(stack, end);
                self.local.pop();
            }
            let current = self
                .local
                .get(local_base..)
                .and_then(<[_]>::last)
                .map_or(base, |&(stack, _)| stack);
            self.produce(current, span.start);
            if span.retok != Retok::No {
                let stack = self.push_all(base, span.scopes, span.dynamic);
                self.retok(span, stack);
                continue;
            }
            if !span.scopes.is_empty() || span.dynamic.is_some() {
                let stack = self.push_all(current, span.scopes, span.dynamic);
                self.local.push((stack, span.end));
            }
        }
        while self.local.len() > local_base {
            if let Some((stack, end)) = self.local.pop() {
                self.produce(stack, end);
            }
        }
    }

    /// Lexes the capture `span` again.
    fn retok(&mut self, span: Span, stack: u32) {
        if span.retok == Retok::Number {
            self.number(span.start, span.end, stack);
            return;
        }
        let saved = (self.lim, self.nl, self.anchor, self.span_base);
        self.lim = span.end;
        self.nl = false;
        self.anchor = None;
        self.span_base = self.sp.len();
        self.reset_runs();
        let list = span.retok.list();
        let mut p = span.start;
        while p < span.end {
            self.spend(1);
            if self.steps.get() > self.budget {
                self.report.record(Limit::Work);
                break;
            }
            if !self.try_list(list, p) {
                p += self.width_at(p);
                continue;
            }
            let hit = self.hit;
            self.emit_hit(hit, stack);
            if hit.end <= p {
                break;
            }
            p = hit.end;
        }
        self.produce(stack, span.end);
        self.sp.truncate(self.span_base);
        (self.lim, self.nl, self.anchor, self.span_base) = saved;
        self.reset_runs();
    }

    /// Tries the patterns of `list` at `p`, in order.
    fn try_list(&mut self, list: &[Item], p: usize) -> bool {
        for item in list {
            match *item {
                Item::P(pat) => {
                    if self.try_pat(pat, p) {
                        return true;
                    }
                }
                Item::G(group) => {
                    if self.try_list(group, p) {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Emits a hit of a plain match found in a capture lexed again.
    fn emit_hit(&mut self, hit: Hit, base: u32) {
        self.produce(base, hit.start);
        let Act::Plain(scopes, dynamic) = hit.act else {
            return;
        };
        let stack = self.push_all(base, scopes, dynamic);
        let from = self.span_base;
        self.emit_spans(stack, hit.end, from);
        self.produce(stack, hit.end);
        self.sp.truncate(from);
    }

    // Frames.

    fn push_rule(&mut self, rule: R, name: u32, content: u32, eol: bool) -> bool {
        let depth = usize::try_from(self.frame.depth).unwrap_or(usize::MAX);
        if self.limits.depth.is_some_and(|max| depth >= max) {
            self.report.record(Limit::Depth);
            return false;
        }
        let code = rule.code() | if eol { EOL } else { 0 };
        let Some(state) = self.arena.push(self.state, code, name, content) else {
            self.report.record(Limit::Nodes);
            return false;
        };
        self.state = state;
        self.frame = self.arena.frame(state);
        true
    }

    fn pop_rule(&mut self) {
        self.state = self.frame.parent;
        self.frame = self.arena.frame(self.state);
    }

    // The loop.

    pub(super) fn run(&mut self) {
        let top = self.len_all();
        let mut pos = self.start;
        while pos <= top && !self.stopped {
            self.spend(1);
            if self.steps.get() > self.budget {
                self.report.record(Limit::Work);
                break;
            }
            let rule = self.rule();
            let Some(start) = self.find(rule, pos) else {
                break;
            };
            let _ = start;
            let hit = self.hit;
            pos = self.apply(hit, pos);
        }
        let stack = self.content_stack();
        self.produce(stack, top);
    }

    /// The first match from `from` on: the earliest position where the
    /// rule's end or one of its patterns matches, the end first.
    fn find(&mut self, rule: R, from: usize) -> Option<usize> {
        let top = self.len_all();
        let mut p = from;
        let mut blank_end = 0;
        let mut blank_dead = false;
        let mut word_end = 0;
        let mut word_scan = 0;
        let mut word_nocall = false;
        loop {
            self.spend(1);
            if self.steps.get() > self.budget {
                self.report.record(Limit::Work);
                return None;
            }
            let index = if p >= top {
                EOT
            } else {
                usize::from(self.b(p))
            };
            self.sp.truncate(self.span_base);
            if self.try_end(rule, p) {
                return Some(p);
            }
            let lead = self.b(p);
            let letter = lead.is_ascii_alphabetic() || lead == b'_';
            let mut blank = false;
            let kind = if p < word_end {
                // Inside a word that was tried at its start.
                let mut flags = kind::AFTER_WORD | kind::IN_WORD;
                if word_nocall {
                    flags |= kind::NO_CALL;
                }
                if !letter {
                    flags |= kind::NO_OBJECT;
                }
                flags
            } else {
                blank = p < top && self.space_at(p) > 0;
                if blank && p >= blank_end {
                    blank_end = self.skip_space(p);
                    blank_dead = !b"/#scd(abnotx".contains(&self.b(blank_end));
                }
                let no_object = !letter && !matches!(self.prev(p), Some(b']' | b')'));
                let mut flags = 0;
                if p == 0 {
                    flags |= kind::START;
                }
                if self.word_before(p) {
                    flags |= kind::AFTER_WORD;
                }
                if blank && blank_dead {
                    flags |= kind::BLANK_DEAD;
                }
                if no_object {
                    flags |= kind::NO_OBJECT;
                }
                flags
            };
            for &pat in self.tables.candidates(rule, kind, index) {
                if self.try_pat(pat, p) {
                    return Some(p);
                }
            }
            blank_dead |= blank;
            if p >= word_scan && letter && p < top {
                word_scan = self.skip_word(p);
                if self.text(p, word_scan).is_ascii() {
                    word_end = word_scan;
                    word_nocall = !self.call_shape(p);
                }
            }
            if p >= top {
                return None;
            }
            p += self.width_at(p);
        }
    }

    /// Takes the match `hit`, found from `pos`, and returns the next
    /// position.
    fn apply(&mut self, hit: Hit, pos: usize) -> usize {
        let advanced = hit.end > pos;
        let content = self.content_stack();
        self.produce(content, hit.start);
        match hit.act {
            Act::End => {
                let name = self.name_stack();
                self.emit_match(name, hit.end);
                let rule = self.rule();
                let popped_eol = self.frame_eol();
                let popped = self.open.pop();
                let enter = popped.map(|open| open.enter);
                self.pop_rule();
                self.anchor = popped.and_then(|open| open.anchor);
                if !advanced && enter == Some(pos) {
                    if let Some(open) = popped {
                        self.push_rule(rule, name, name, popped_eol);
                        self.open.push(open);
                    }
                    self.stopped = true;
                    let top = self.len_all();
                    self.produce(name, top);
                }
            }
            Act::Begin(rule) => {
                let parent = self.frame;
                let name = self.push_all(parent.stack, rule.name(), None);
                let content = self.push_scopes(name, rule.content());
                let before_open = self.open.len();
                let anchor = self.anchor;
                let eol = self.nl && self.lim == self.s.len() && hit.end >= self.len_all();
                let pushed = self.push_rule(rule, name, content, eol);
                self.emit_match(name, hit.end);
                self.anchor = Some(hit.end);
                if pushed {
                    self.open.push(Open {
                        rule,
                        enter: pos,
                        anchor,
                    });
                    if !advanced && self.same_rule(rule, pos, before_open) {
                        self.open.pop();
                        self.pop_rule();
                        self.stopped = true;
                        let stack = self.content_stack();
                        let top = self.len_all();
                        self.produce(stack, top);
                    }
                } else if !advanced {
                    // A limit kept the rule closed and the match took
                    // nothing: the same match would come back, so the line
                    // stops.
                    self.stopped = true;
                    let stack = self.content_stack();
                    let top = self.len_all();
                    self.produce(stack, top);
                }
            }
            Act::Plain(scopes, dynamic) => {
                let base = self.push_all(content, scopes, dynamic);
                self.emit_match(base, hit.end);
                if !advanced {
                    self.stopped = true;
                    let top = self.len_all();
                    self.produce(content, top);
                }
            }
        }
        pos.max(hit.end)
    }

    /// Whether a rule entered at `pos` is open, as `hasSameRuleAs` asks:
    /// among the frames entered at the same position as the new one.
    fn same_rule(&self, rule: R, pos: usize, below: usize) -> bool {
        self.open
            .get(..below)
            .unwrap_or(&[])
            .iter()
            .rev()
            .take_while(|open| open.enter == pos)
            .any(|open| open.rule == rule)
    }

    /// Emits the captures of the current match and the rest of it under
    /// `base`.
    fn emit_match(&mut self, base: u32, end: usize) {
        let from = self.span_base;
        self.emit_spans(base, end, from);
        self.produce(base, end);
        self.sp.truncate(from);
    }

    /// The width of the character at `i`, at least 1.
    pub(super) fn width_at(&self, i: usize) -> usize {
        match self.s.get(i) {
            Some(&lead) if i < self.lim => match lead {
                0xf0.. => 4,
                0xe0.. => 3,
                0xc0.. => 2,
                _ => 1,
            },
            _ => 1,
        }
    }
}
