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

use super::lists::{EOT, Item, Tables, blank_run_live, kind};
use super::scope::S;
use super::state::{EOL, R};
use crate::arena::{Arena, Frame};
use crate::limits::{Limit, Limits, LineReport};
use crate::utf8::char_len;
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

/// The buffers a C line works in, which the highlighter keeps from line to
/// line, so that a line allocates nothing once they have grown.
#[derive(Debug, Default)]
pub(crate) struct Scratch {
    open: Vec<Open>,
    sp: Vec<Span>,
    local: Vec<(u32, usize)>,
    name: String,
    ends: Vec<usize>,
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
    /// The last run of each class found.
    pub(super) runs: Runs,
    /// The last scoped name found. A later search that steps onto that walk
    /// ends where it did.
    pub(super) scoped: Cell<Run>,
    /// The last word looked up.
    pub(super) lists: Cell<WordLists>,
    /// Whether the line starts with a directive.
    pub(super) head: Cell<Head>,
}

/// A stretch of the text a search found: where it starts and ends.
#[derive(Clone, Copy, Debug)]
pub(super) struct Run {
    pub(super) start: usize,
    pub(super) end: usize,
}

impl Run {
    /// No run: it contains no position.
    pub(super) const NONE: Run = Run {
        start: usize::MAX,
        end: 0,
    };

    /// Whether a search from `i` lies in the run, and so ends where it does.
    pub(super) fn holds(self, i: usize) -> bool {
        self.start <= i && i <= self.end
    }
}

/// The last run of each character class found. A search from inside a run
/// ends where the run does, so a run is scanned once.
#[derive(Debug)]
pub(super) struct Runs {
    pub(super) space: Cell<Run>,
    pub(super) word: Cell<Run>,
    pub(super) ascii_name: Cell<Run>,
    pub(super) colons: Cell<Run>,
}

impl Runs {
    fn new() -> Self {
        Runs {
            space: Cell::new(Run::NONE),
            word: Cell::new(Run::NONE),
            ascii_name: Cell::new(Run::NONE),
            colons: Cell::new(Run::NONE),
        }
    }

    fn reset(&self) {
        for run in [&self.space, &self.word, &self.ascii_name, &self.colons] {
            run.set(Run::NONE);
        }
    }
}

/// A word looked up in the word lists: where it starts and ends, and the
/// lists it is in, an empty set where no word starts.
#[derive(Clone, Copy, Debug)]
pub(super) struct WordLists {
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) lists: u32,
}

impl WordLists {
    const NONE: WordLists = WordLists {
        start: usize::MAX,
        end: 0,
        lists: 0,
    };
}

/// Whether the line starts with a directive, found once per line.
#[derive(Clone, Copy, Debug)]
pub(super) enum Head {
    /// Not looked up yet.
    Unknown,
    /// The line is no directive.
    Plain,
    /// The line is a directive: where its `#` is and where its word starts.
    Directive { hash: usize, word: usize },
}

impl<'a> Lexer<'a> {
    pub(super) fn new(
        arena: &'a mut Arena,
        limits: &'a Limits,
        text: super::Text<'a>,
        out: &'a mut Vec<Token>,
        state: u32,
        report: &'a mut LineReport,
        scratch: &mut Scratch,
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
            open: cleared(&mut scratch.open),
            anchor,
            sp: cleared(&mut scratch.sp),
            span_base: 0,
            hit: Hit {
                start: 0,
                end: 0,
                act: Act::End,
            },
            local: cleared(&mut scratch.local),
            name: std::mem::take(&mut scratch.name),
            tables: Tables::get(),
            stopped: false,
            ends: cleared(&mut scratch.ends),
            runs: Runs::new(),
            scoped: Cell::new(Run::NONE),
            lists: Cell::new(WordLists::NONE),
            head: Cell::new(Head::Unknown),
        }
    }

    /// The state after the line, with the buffers given back to `scratch`.
    pub(super) fn into_state(self, scratch: &mut Scratch) -> u32 {
        scratch.open = self.open;
        scratch.sp = self.sp;
        scratch.local = self.local;
        scratch.name = self.name;
        scratch.ends = self.ends;
        self.state
    }

    /// Forgets the runs found, when the text they were found in changes.
    pub(super) fn reset_runs(&self) {
        self.runs.reset();
        self.scoped.set(Run::NONE);
        self.lists.set(WordLists::NONE);
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
            if !self.find(rule, pos) {
                break;
            }
            let hit = self.hit;
            pos = self.apply(hit, pos);
        }
        let stack = self.content_stack();
        self.produce(stack, top);
    }

    /// Finds the first match from `from` on, the earliest position where the
    /// rule's end or one of its patterns matches, the end first, and leaves
    /// it in `hit`. False when nothing matches or the work runs out.
    ///
    /// Each position tries only the patterns its kind of position allows
    /// (`lists::kind`): inside an ASCII word the search tried at its start,
    /// the patterns that read only the word's chain are left out.
    fn find(&mut self, rule: R, from: usize) -> bool {
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
                return false;
            }
            let index = if p >= top {
                EOT
            } else {
                usize::from(self.b(p))
            };
            self.sp.truncate(self.span_base);
            if self.try_end(rule, p) {
                return true;
            }
            let lead = self.b(p);
            let letter = lead.is_ascii_alphabetic() || lead == b'_';
            let mut blank = false;
            let kind = if p < word_end {
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
                    blank_dead = !blank_run_live(self.b(blank_end));
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
                    return true;
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
                return false;
            }
            p += self.width_at(p);
        }
    }

    /// Takes the match `hit`, found from `pos`, and returns the next
    /// position.
    ///
    /// A match that takes nothing would come back at the same place, so it
    /// stops the line: an end that closes the rule entered at `pos`, a begin
    /// of a rule already open there, a begin a limit kept closed, and a plain
    /// match.
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
            Some(&lead) if i < self.lim => char_len(lead),
            _ => 1,
        }
    }
}

/// Takes `buffer` out of the scratch, emptied but with its capacity.
fn cleared<T>(buffer: &mut Vec<T>) -> Vec<T> {
    let mut taken = std::mem::take(buffer);
    taken.clear();
    taken
}
