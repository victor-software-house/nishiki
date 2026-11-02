//! The per-line engine: it runs a grammar's rules the way a `TextMate` engine
//! does, with hand-written matchers instead of regular expressions.
//!
//! At each position the engine tries the end pattern of the innermost rule and
//! then the rule's patterns in order, and takes the first that matches at the
//! earliest position. A begin match opens a rule and an end match closes it;
//! a match's capture groups name parts of it. The corner cases of `TextMate`
//! engines are kept because Shiki's output depends on them: `\G` matches only
//! where the enclosing begin ended, and a rule that opens or closes without
//! advancing stops the line.

use std::cell::Cell;

use super::{Act, Grammar, MAX_GROUPS, Place, Text};
use crate::arena::{Arena, ROOT};
use crate::limits::{Limit, Limits, LineReport};
use crate::{Stack, Token};

/// The scopes of a pattern's capture groups, and the groups that the grammar
/// tokenizes with patterns of their own.
type Captures<G> = (
    &'static [(u8, &'static [<G as Grammar>::Scope])],
    &'static [(u8, <G as Grammar>::Nested)],
);

/// An open rule.
#[derive(Clone, Copy, Debug)]
struct Frame<G: Grammar> {
    rule: G::Rule,
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
    /// What the rule carries: the interned text its end pattern repeats.
    payload: u32,
}

/// What a highlighter keeps from line to line, so that lexing a line
/// allocates nothing once the buffers have grown.
#[derive(Debug)]
pub(crate) struct Scratch<G: Grammar> {
    frames: Vec<Frame<G>>,
    chain: Vec<(u32, crate::arena::Frame)>,
}

impl<G: Grammar> Default for Scratch<G> {
    fn default() -> Self {
        Scratch {
            frames: Vec::new(),
            chain: Vec::new(),
        }
    }
}

/// The payload of a frame whose begin match took the line feed, so that `\G`
/// holds at the start of the next line, as in vscode-textmate.
const CAPTURED_EOL: u32 = 1;

/// The payload of a root frame that is the start of a document: its first
/// line is the one where `\A` matches, and the root of every state after it
/// has no payload.
pub(crate) const FIRST_LINE: u32 = 1;

/// Lexes one line.
#[expect(
    clippy::struct_excessive_bools,
    reason = "each flag is one fact the grammar's anchors ask about"
)]
pub(crate) struct Lexer<'a, G: Grammar> {
    pub(crate) s: &'a [u8],
    out: &'a mut Vec<Token>,
    arena: &'a mut Arena,
    limits: &'a Limits,
    report: &'a mut LineReport,
    /// Steps spent on this line, counted in `&self` scans too.
    steps: Cell<u64>,
    /// Steps this line may spend; `u64::MAX` for no limit.
    budget: u64,
    /// End of the last token.
    last: usize,
    /// Where the current search starts.
    pub(crate) pos: usize,
    anchor: Option<usize>,
    /// Whether `\G` matches at `pos`.
    pub(crate) g: bool,
    /// Whether this is the first line of the document, where `\A` matches.
    pub(crate) first_line: bool,
    /// The open rules, the root first.
    frames: Vec<Frame<G>>,
    /// The capture groups of the last match, by group number.
    caps: [Option<(usize, usize)>; MAX_GROUPS],
    caps_dirty: bool,
    /// What the language's matchers remember about this line.
    pub(crate) x: G::Extra,
    /// A rule closed without advancing, which keeps its begin scopes for the
    /// rest of the line.
    kept: bool,
}

impl<'a, G: Grammar> Lexer<'a, G> {
    pub(crate) fn new(
        arena: &'a mut Arena,
        limits: &'a Limits,
        text: Text<'a>,
        out: &'a mut Vec<Token>,
        state: u32,
        report: &'a mut LineReport,
        scratch: &mut Scratch<G>,
    ) -> Self {
        let (s, start) = (text.bytes, text.start);
        let budget = limits.work.map_or(u64::MAX, |work| work.steps(s.len()));
        let mut lexer = Lexer {
            s,
            out,
            arena,
            limits,
            report,
            steps: Cell::new(0),
            budget,
            last: start,
            pos: start,
            anchor: text.anchored.then_some(start),
            g: false,
            first_line: false,
            frames: std::mem::take(&mut scratch.frames),
            caps: [None; MAX_GROUPS],
            caps_dirty: false,
            x: G::Extra::default(),
            kept: false,
        };
        // The open rules of `state`, from its root down.
        lexer.frames.clear();
        let mut chain = std::mem::take(&mut scratch.chain);
        chain.clear();
        let mut id = state;
        loop {
            let frame = lexer.arena.frame(id);
            chain.push((id, frame));
            if G::rule_from_code(frame.kind) == G::ROOT || frame.parent == ROOT {
                break;
            }
            id = frame.parent;
        }
        for (index, &(id, frame)) in chain.iter().rev().enumerate() {
            if index == 0 {
                // After the first line the root is another state, without
                // the payload, so `\A` matches on one line only.
                let mut state = id;
                if frame.payload == FIRST_LINE {
                    lexer.first_line = true;
                    state = lexer
                        .arena
                        .push(frame.parent, frame.kind, 0, frame.stack)
                        .unwrap_or(id);
                }
                lexer.frames.push(Frame {
                    rule: G::ROOT,
                    enter: None,
                    anchor: None,
                    name: frame.stack,
                    content: frame.stack,
                    state,
                    payload: 0,
                });
                continue;
            }
            let rule = G::rule_from_code(frame.kind);
            let parent = lexer.top().content;
            let name = lexer.path(parent, G::rule_data(rule).name);
            lexer.frames.push(Frame {
                rule,
                enter: None,
                anchor: None,
                name,
                content: frame.stack,
                state: id,
                payload: frame.payload,
            });
        }
        scratch.chain = chain;
        if lexer.frames.len() > 1
            && G::rule_data(lexer.top().rule).backref.is_none()
            && lexer.top().payload == CAPTURED_EOL
        {
            lexer.anchor = Some(start);
        }
        lexer
    }

    /// Lexes the line, returns the state after it, and gives the buffers back
    /// to `scratch`.
    pub(crate) fn run(mut self, scratch: &mut Scratch<G>) -> u32 {
        self.tokenize();
        let state = self.top().state;
        scratch.frames = self.frames;
        state
    }

    // Bytes.

    /// The byte at `i`, with the line feed at the end of a line, or 0 where no
    /// character stands.
    pub(crate) fn at(&self, i: usize) -> u8 {
        match i.cmp(&self.s.len()) {
            std::cmp::Ordering::Less => self.s[i],
            std::cmp::Ordering::Equal => b'\n',
            std::cmp::Ordering::Greater => 0,
        }
    }

    /// The end of the text, where the line feed stands.
    pub(crate) fn lim(&self) -> usize {
        self.s.len()
    }

    /// The regex `$` at `i`: before the line feed or at the end of the text.
    pub(crate) fn at_eol(&self, i: usize) -> bool {
        i == self.s.len() || i == self.s.len() + 1
    }

    /// Whether the text at `i` starts with `text`.
    pub(crate) fn starts(&self, i: usize, text: &[u8]) -> bool {
        text.first().is_some_and(|b| self.s.get(i) == Some(b))
            && self.s.get(i..).is_some_and(|rest| rest.starts_with(text))
    }

    /// The end of the run of bytes from `i` that `class` accepts.
    pub(crate) fn skip(&self, mut i: usize, class: impl Fn(u8) -> bool) -> usize {
        let from = i;
        while i < self.s.len() && class(self.s[i]) {
            i += 1;
        }
        self.spend(i - from);
        i
    }

    /// Counts `steps` of work against the line's budget.
    pub(crate) fn spend(&self, steps: usize) {
        let steps = u64::try_from(steps).unwrap_or(u64::MAX);
        self.steps.set(self.steps.get().saturating_add(steps));
    }

    /// The end of the character at `i`, which is a whole UTF-8 sequence.
    pub(crate) fn char_end(&self, i: usize) -> usize {
        let len = match self.s.get(i) {
            Some(0xf0..) => 4,
            Some(0xe0..) => 3,
            Some(0xc0..) => 2,
            _ => 1,
        };
        (i + len).min(self.s.len())
    }

    /// The character at `i` and its end; the line feed at the end of a line.
    pub(crate) fn char_at(&self, i: usize) -> Option<(char, usize)> {
        let b = *self.s.get(i)?;
        if b < 0x80 {
            return Some((char::from(b), i + 1));
        }
        let end = self.char_end(i);
        let text = std::str::from_utf8(&self.s[i..end]).ok()?;
        Some((text.chars().next()?, end))
    }

    /// The character that ends at `i`.
    pub(crate) fn char_before(&self, i: usize) -> Option<char> {
        let mut start = i.checked_sub(1)?;
        while start > 0 && self.s.get(start).is_some_and(|b| b & 0xc0 == 0x80) {
            start -= 1;
        }
        self.char_at(start).map(|(c, _)| c)
    }

    // Captures.

    /// Records capture group `group` of the match being tried.
    pub(crate) fn cap(&mut self, group: usize, start: usize, end: usize) {
        self.caps[group] = Some((start, end));
        self.caps_dirty = true;
    }

    /// The bytes `start..end` of the line, empty where they lie outside it.
    pub(crate) fn text(&self, start: usize, end: usize) -> &'a [u8] {
        let s = self.s;
        s.get(start..end).unwrap_or(&[])
    }

    /// The text of capture group `group` of the last match.
    pub(crate) fn cap_text(&self, group: usize) -> &'a [u8] {
        let s = self.s;
        self.caps[group].map_or(&[][..], |(start, end)| &s[start..end.min(s.len())])
    }

    // Frames.

    fn top(&self) -> &Frame<G> {
        &self.frames[self.frames.len() - 1]
    }

    /// The width of the innermost rule's payload text when the line has it at
    /// `p`.
    pub(crate) fn payload_at(&self, p: usize) -> Option<usize> {
        let text = self.arena.string(self.top().payload).as_bytes();
        (!text.is_empty() && self.starts(p, text)).then_some(text.len())
    }

    /// Whether the line has the innermost rule's payload text just before `p`.
    pub(crate) fn payload_before(&self, p: usize) -> bool {
        let text = self.arena.string(self.top().payload).as_bytes();
        !text.is_empty() && p >= text.len() && self.text(p - text.len(), p) == text
    }

    /// The rules open above the root.
    fn depth(&self) -> usize {
        self.frames.len() - 1
    }

    /// Whether the depth limit keeps another rule from opening.
    fn at_depth_limit(&self) -> bool {
        self.limits.depth.is_some_and(|max| self.depth() >= max)
    }

    /// The stack `base` with `scopes` on top. A scope the node limit keeps
    /// from being made is left out.
    pub(crate) fn path(&mut self, base: u32, scopes: &[G::Scope]) -> u32 {
        let mut stack = base;
        for &scope in scopes {
            let Some(child) = self.arena.child(stack, G::scope(scope)) else {
                self.report.record(Limit::Nodes);
                break;
            };
            stack = child;
        }
        stack
    }

    fn push_frame(
        &mut self,
        rule: G::Rule,
        enter: Option<usize>,
        anchor: Option<usize>,
        payload: u32,
    ) {
        let data = G::rule_data(rule);
        let parent_state = self.top().state;
        let parent = self.top().content;
        let name = self.path(parent, data.name);
        let content = self.path(name, data.content);
        let state = self
            .arena
            .push(parent_state, G::rule_code(rule), payload, content)
            .unwrap_or_else(|| {
                self.report.record(Limit::Nodes);
                parent_state
            });
        self.frames.push(Frame {
            rule,
            enter,
            anchor,
            name,
            content,
            state,
            payload,
        });
    }

    fn pop_frame(&mut self) -> Frame<G> {
        let frame = self.frames[self.frames.len() - 1];
        self.frames.pop();
        frame
    }

    // Tokens.

    pub(crate) fn emit(&mut self, to: usize, node: u32) {
        let to = to.min(self.s.len());
        if to <= self.last {
            return;
        }
        self.out.push(Token {
            end: to,
            stack: Stack(node),
        });
        self.last = to;
    }

    /// Emits the match `start..end` under `base`, with the scopes of its
    /// capture groups on top of it where they lie.
    fn emit_captures(&mut self, (table, nested): Captures<G>, base: u32, start: usize, end: usize) {
        self.caps[0] = Some((start, end));
        let mut stack = [(0_usize, 0_u32); MAX_GROUPS];
        let mut depth = 0;
        let (mut named, mut tokenized) = (0, 0);
        loop {
            let (group, scopes, kind) = match (table.get(named), nested.get(tokenized)) {
                (None, None) => break,
                (Some(&(group, scopes)), Some(&(other, _))) if group <= other => {
                    named += 1;
                    (group, scopes, None)
                }
                (Some(&(group, scopes)), None) => {
                    named += 1;
                    (group, scopes, None)
                }
                (_, Some(&(group, kind))) => {
                    tokenized += 1;
                    (group, &[][..], Some(kind))
                }
            };
            let Some((from, to)) = self.caps[usize::from(group)] else {
                continue;
            };
            if to <= from || from > self.s.len() {
                continue;
            }
            while depth > 0 && stack[depth - 1].0 <= from {
                self.emit(stack[depth - 1].0, stack[depth - 1].1);
                depth -= 1;
            }
            let outer = if depth > 0 { stack[depth - 1].1 } else { base };
            self.emit(from, outer);
            if let Some(kind) = kind {
                G::tokenize_nested(self, kind, from, to, outer);
                continue;
            }
            if scopes.is_empty() {
                continue;
            }
            let node = self.path(outer, scopes);
            stack[depth] = (to, node);
            depth += 1;
        }
        while depth > 0 {
            self.emit(stack[depth - 1].0, stack[depth - 1].1);
            depth -= 1;
        }
        self.emit(end, base);
    }

    // The loop.

    /// Runs from `pos` until the text ends or the line stops.
    fn tokenize(&mut self) {
        let line_length = self.s.len() + 1;
        let mut stalled = 0_u32;
        loop {
            self.spend(1);
            if self.steps.get() > self.budget {
                // Out of work: the rest of the line is the rule's.
                self.report.record(Limit::Work);
                let node = self.top().content;
                self.emit(line_length, node);
                return;
            }
            self.g = self.anchor == Some(self.pos);
            let Some((start, pat, end)) = self.scan() else {
                if self.steps.get() > self.budget {
                    self.report.record(Limit::Work);
                }
                let node = self.top().content;
                self.emit(line_length, node);
                return;
            };
            let advanced = end > self.pos;
            let data = G::pat_data(pat);
            let captures = (data.captures, data.nested);
            let stop = match data.act {
                Act::End => self.end_rule(captures, start, end, advanced, line_length),
                Act::Begin(rule) => {
                    self.begin_rule(rule, captures, start, end, advanced, line_length)
                }
                Act::Match(name) => {
                    self.match_rule(name, captures, start, end, advanced, line_length)
                }
            };
            if stop {
                return;
            }
            if end > self.pos {
                self.pos = end;
                stalled = 0;
            } else {
                stalled += 1;
                // Each step that does not advance opens or closes a rule at
                // this byte, and a rule that repeats ends the line, so a run
                // is at most one open and one close per rule of the grammar.
                // 4096 is far above that: reaching it means a rule matches
                // empty without being guarded.
                debug_assert!(
                    stalled < 4096,
                    "the {} lexer made no progress at byte {}",
                    G::NAME,
                    self.pos
                );
            }
        }
    }

    /// The first pattern of the innermost rule that matches at or after the
    /// cursor, with where its match starts and ends.
    fn scan(&mut self) -> Option<(usize, G::Pat, usize)> {
        let tables = G::tables(self.top().rule);
        let full = self.at_depth_limit();
        let lim = self.s.len();
        let mut p = self.pos;
        let mut visited = 0;
        let found = loop {
            // The text a pattern sees ends after the line feed, where `$` and
            // lookaheads can still match.
            if p > lim + 1 {
                break None;
            }
            visited += 1;
            if visited % 64 == 0 && self.steps.get() > self.budget {
                break None;
            }
            let list = if p > lim {
                tables.all()
            } else {
                let byte = if p < lim { self.s[p] } else { b'\n' };
                let place = if p == 0 {
                    Place::LineStart
                } else if is_word_byte(byte) && is_word_byte(self.s[p - 1]) {
                    Place::InWord
                } else {
                    Place::Away
                };
                tables.candidates(place, byte)
            };
            let mut hit = None;
            for &pat in list {
                if full && matches!(G::pat_data(pat).act, Act::Begin(_)) {
                    // A rule the depth limit keeps closed: its text is plain.
                    if self.try_pat(pat, p).is_some() {
                        self.report.record(Limit::Depth);
                    }
                    continue;
                }
                if let Some(end) = self.try_pat(pat, p) {
                    hit = Some((p, pat, end));
                    break;
                }
            }
            if hit.is_some() {
                break hit;
            }
            // A pattern that fails at a blank fails at the blanks after it: the
            // blanks are consumed first and what follows decides, and no
            // lookbehind or boundary holds between two of them.
            p = if p < lim && matches!(self.s[p], b' ' | b'\t') {
                self.skip(p, |b| matches!(b, b' ' | b'\t'))
            } else if p < lim {
                self.char_end(p)
            } else {
                p + 1
            };
        };
        self.spend(visited);
        found
    }

    fn try_pat(&mut self, pat: G::Pat, p: usize) -> Option<usize> {
        if self.caps_dirty {
            self.caps = [None; MAX_GROUPS];
            self.caps_dirty = false;
        }
        G::matcher(self, pat, p)
    }

    /// Closes the innermost rule. True when the line stops.
    fn end_rule(
        &mut self,
        table: Captures<G>,
        start: usize,
        end: usize,
        advanced: bool,
        line_length: usize,
    ) -> bool {
        let top = *self.top();
        self.emit(start, top.content);
        self.emit_captures(table, top.name, start, end);
        let popped = self.pop_frame();
        self.anchor = popped.anchor;
        if !advanced && popped.enter == Some(self.pos) {
            // The rule opened and closed without advancing: the grammar would
            // loop, so the rest of the line is the rule's.
            self.kept = true;
            self.frames.push(Frame {
                content: popped.name,
                ..popped
            });
            self.emit(line_length, popped.name);
            return true;
        }
        false
    }

    /// Opens `rule`. True when the line stops.
    fn begin_rule(
        &mut self,
        rule: G::Rule,
        table: Captures<G>,
        start: usize,
        end: usize,
        advanced: bool,
        line_length: usize,
    ) -> bool {
        let top_content = self.top().content;
        self.emit(start, top_content);
        let before = self.depth();
        let payload = match G::rule_data(rule).backref {
            Some(group) => {
                self.caps[0] = Some((start, end));
                let text = std::str::from_utf8(self.cap_text(usize::from(group))).unwrap_or("");
                self.arena.intern(text).unwrap_or_else(|| {
                    self.report.record(Limit::Nodes);
                    0
                })
            }
            // A begin that took the line feed leaves `\G` at the start of the
            // next line, which the payload of the saved frame remembers.
            None if end > self.s.len() => CAPTURED_EOL,
            None => 0,
        };
        // The content scopes come after the begin tokens.
        let anchor = self.anchor;
        self.push_frame(rule, Some(self.pos), anchor, payload);
        let name = self.top().name;
        let content = self.top().content;
        let top = self.frames.len() - 1;
        self.frames[top].content = name;
        self.emit_captures(table, name, start, end);
        self.anchor = Some(end);
        self.frames[top].content = content;
        if !advanced && self.loops_below(before, rule) {
            self.pop_frame();
            let node = self.top().content;
            self.emit(line_length, node);
            return true;
        }
        false
    }

    /// Whether `rule`, just opened above frame `before`, repeats a rule that
    /// opened at the same position without advancing.
    fn loops_below(&self, before: usize, rule: G::Rule) -> bool {
        let pos = self.pos;
        let mut i = before;
        loop {
            let frame = &self.frames[i];
            if frame.enter != Some(pos) {
                return false;
            }
            if frame.rule == rule {
                return true;
            }
            if i == 0 {
                return false;
            }
            i -= 1;
        }
    }

    /// Names the text of a match rule. True when the line stops.
    fn match_rule(
        &mut self,
        name: &'static [G::Scope],
        table: Captures<G>,
        start: usize,
        end: usize,
        advanced: bool,
        line_length: usize,
    ) -> bool {
        let top_content = self.top().content;
        self.emit(start, top_content);
        let base = self.path(top_content, name);
        self.emit_captures(table, base, start, end);
        if !advanced {
            // A match that is empty and not at an advance: the grammar would
            // loop, so the enclosing rule is dropped and the line stops.
            if self.depth() > 0 {
                self.pop_frame();
            }
            let node = self.top().content;
            self.emit(line_length, node);
            return true;
        }
        false
    }
}

/// Whether `byte` is an ASCII word character.
fn is_word_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}
