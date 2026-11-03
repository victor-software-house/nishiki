//! The per-line engine: it runs a grammar's rules the way a `TextMate` engine
//! does, with hand-written matchers instead of regular expressions.
//!
//! At each position the engine tries the end pattern of the innermost rule and
//! then the rule's patterns in order, and takes the first that matches at the
//! earliest position. A begin match opens a rule and an end match closes it;
//! a match's capture groups name parts of it, through the pattern's table or
//! through spans that the matcher records, and a span may be lexed again with
//! the patterns of another rule. The corner cases of `TextMate` engines are
//! kept because Shiki's output depends on them: `\G` matches only where the
//! enclosing begin ended, and a rule that opens or closes without advancing
//! stops the line.

use std::borrow::Cow;
use std::cell::Cell;

use super::{Act, Captures, DynName, Grammar, Payload, Span, Text};
use crate::arena::{Arena, ROOT};
use crate::limits::{Limit, Limits, LineReport};
use crate::{Stack, Token};

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

/// A cached `path`: the stack `base` with the scopes at `ptr` on top.
#[derive(Clone, Copy, Debug)]
struct PathEntry {
    base: u32,
    ptr: usize,
    len: usize,
    node: u32,
}

/// A cached `push_frame`: what opening `code` with `payload` on `parent`,
/// whose text has the stack `scopes`, made. A rule opened inside a lexed-again
/// capture has its parent's state but not its parent's stack, so both key it.
#[derive(Clone, Copy, Debug)]
struct PushEntry {
    parent: u32,
    scopes: u32,
    code: u16,
    payload: u32,
    name: u32,
    content: u32,
    state: u32,
}

/// The deepest the capture groups of one match nest.
const NEST: usize = 12;

const PATH_ENTRIES: usize = 512;
const PUSH_ENTRIES: usize = 256;

/// Two things that happen while a line is lexed.
#[derive(Clone, Copy, Debug, Default)]
struct Flags {
    /// A node limit kept something from being made since the flag was last
    /// cleared.
    failed: bool,
    /// A rule closed without advancing, which keeps its begin scopes for the
    /// rest of the line.
    kept: bool,
}

/// What a highlighter keeps from line to line, so that lexing a line
/// allocates nothing once it has grown, and the hot lookups of the arena are
/// not repeated. It belongs to one arena: every id it holds names a node
/// that arena made and never removes.
#[derive(Debug)]
pub(crate) struct Scratch<G: Grammar> {
    frames: Vec<Frame<G>>,
    spans: Vec<Span<G>>,
    /// The capture groups of the last match, kept clear between lines.
    caps: Vec<Option<(usize, usize)>>,
    /// The capture groups of the matches whose nested groups are being
    /// lexed, one after another, the outermost first.
    saved_caps: Vec<Option<(usize, usize)>>,
    chain: Vec<(u32, crate::arena::Frame)>,
    /// The text of a scope name made from source text.
    name: String,
    /// The state the last line ended in and the open rules of it, which the
    /// next line starts from when it continues that state.
    resume: Option<u32>,
    resume_frames: Vec<Frame<G>>,
    paths: Vec<Option<PathEntry>>,
    pushes: Vec<Option<PushEntry>>,
}

impl<G: Grammar> Default for Scratch<G> {
    fn default() -> Self {
        Scratch {
            frames: Vec::new(),
            spans: Vec::new(),
            caps: Vec::new(),
            saved_caps: Vec::new(),
            chain: Vec::new(),
            name: String::new(),
            resume: None,
            resume_frames: Vec::new(),
            paths: Vec::new(),
            pushes: Vec::new(),
        }
    }
}

/// The payload of a frame whose begin match took the line feed, so that `\G`
/// holds at the start of the next line, as in vscode-textmate. Only a grammar
/// with [`Grammar::CAPTURED_EOL`] keeps it.
const CAPTURED_EOL: u32 = 1;

/// Where the text of a scope name made at run time comes from.
#[derive(Clone, Copy, Debug)]
enum NameSource {
    /// The bytes `start..end` of the line.
    Line { start: usize, end: usize },
    /// A string the arena interned, such as a rule's payload.
    Interned(u32),
}

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
    /// The text: the line, or the part of it up to where a capture that is
    /// lexed again ends.
    pub(crate) s: &'a [u8],
    /// Whether a line feed follows the text, which the regular expressions of
    /// the grammar see at the end of a line but not at the end of a capture.
    pub(crate) nl: bool,
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
    caps: Vec<Option<(usize, usize)>>,
    /// A matcher recorded a capture group or a span since the last try
    /// cleared them, so the next try clears them first.
    dirty: bool,
    /// The spans a matcher recorded; those of the current match start at
    /// `span_base`, and a span lexed again matches above them.
    spans: Vec<Span<G>>,
    span_base: usize,
    /// The highlighter's buffers, of which the lexer takes out only the
    /// ones it reaches at every position, so that starting a line costs
    /// little.
    scratch: &'a mut Scratch<G>,
    flags: Flags,
    /// The state the line starts in.
    state: u32,
    /// What the language's matchers remember about this line.
    pub(crate) x: G::Extra,
}

impl<'a, G: Grammar> Lexer<'a, G> {
    #[inline]
    pub(crate) fn new(
        arena: &'a mut Arena,
        limits: &'a Limits,
        text: Text<'a>,
        out: &'a mut Vec<Token>,
        state: u32,
        report: &'a mut LineReport,
        scratch: &'a mut Scratch<G>,
    ) -> Self {
        let (s, start) = (text.bytes, text.start);
        let budget = limits.work.map_or(u64::MAX, |work| work.steps(s.len()));
        let mut frames = std::mem::take(&mut scratch.frames);
        if scratch.resume == Some(state) {
            std::mem::swap(&mut frames, &mut scratch.resume_frames);
        } else {
            frames.clear();
        }
        let mut spans = std::mem::take(&mut scratch.spans);
        spans.clear();
        let mut caps = std::mem::take(&mut scratch.caps);
        if caps.is_empty() {
            caps.resize(G::GROUPS, None);
        }
        if scratch.paths.is_empty() {
            scratch.paths.resize(PATH_ENTRIES, None);
        }
        if scratch.pushes.is_empty() {
            scratch.pushes.resize(PUSH_ENTRIES, None);
        }
        Lexer {
            s,
            nl: true,
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
            frames,
            caps,
            dirty: false,
            spans,
            span_base: 0,
            scratch,
            flags: Flags::default(),
            state,
            x: G::Extra::default(),
        }
    }

    /// Opens the rules of the state the line starts in, unless the frames
    /// the last line ended with are already in place.
    fn open(&mut self) {
        if self.frames.is_empty() {
            let mut chain = std::mem::take(&mut self.scratch.chain);
            self.open_frames(self.state, &mut chain);
            self.scratch.chain = chain;
        }
        if G::CAPTURED_EOL
            && self.frames.len() > 1
            && G::rule_payload(self.top().rule) == Payload::None
            && self.top().payload == CAPTURED_EOL
        {
            self.anchor = Some(self.pos);
        }
    }

    /// Opens the rules of `state`, from its root down.
    fn open_frames(&mut self, state: u32, chain: &mut Vec<(u32, crate::arena::Frame)>) {
        // The open rules of `state`, from its root down.
        chain.clear();
        let mut id = state;
        loop {
            let frame = self.arena.frame(id);
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
                    self.first_line = true;
                    state = self
                        .arena
                        .push(frame.parent, frame.kind, 0, frame.stack)
                        .unwrap_or(id);
                }
                self.frames.push(Frame {
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
            let parent = self.top().content;
            let name = self.path(parent, G::rule_name(rule));
            self.frames.push(Frame {
                rule,
                enter: None,
                anchor: None,
                name,
                content: frame.stack,
                state: id,
                payload: frame.payload,
            });
        }
    }

    /// Lexes the line, returns the state after it, and gives the buffers back
    /// to the scratch. When each open rule is a state of its own, which a
    /// node limit can deny, and none kept its begin scopes, the open rules
    /// are the state's chain, and the next line takes them over as the
    /// frames it starts with.
    #[inline]
    pub(crate) fn run(mut self) -> u32 {
        self.open();
        self.tokenize();
        let state = self.top().state;
        let exact = !self.flags.kept
            && self
                .frames
                .windows(2)
                .all(|pair| pair[0].state != pair[1].state);
        self.scratch.resume = exact.then_some(state);
        if exact {
            for frame in &mut self.frames {
                (frame.enter, frame.anchor) = (None, None);
            }
            std::mem::swap(&mut self.frames, &mut self.scratch.resume_frames);
        }
        if self.dirty {
            self.caps[..G::GROUPS].fill(None);
        }
        self.scratch.frames = self.frames;
        self.scratch.spans = self.spans;
        self.scratch.caps = self.caps;
        state
    }

    // Bytes.

    /// Whether a character stands at `i`: the text, or its line feed.
    pub(crate) fn avail(&self, i: usize) -> bool {
        i < self.s.len() || (i == self.s.len() && self.nl)
    }

    /// The byte at `i`, with the line feed at the end of a line, or 0 where no
    /// character stands.
    pub(crate) fn at(&self, i: usize) -> u8 {
        match self.s.get(i) {
            Some(&b) => b,
            None if i == self.s.len() && self.nl => b'\n',
            None => 0,
        }
    }

    /// The byte before `i`, or `None` at the start.
    pub(crate) fn prev(&self, i: usize) -> Option<u8> {
        if i == 0 { None } else { Some(self.at(i - 1)) }
    }

    /// The end of the text, where the line feed stands.
    pub(crate) fn lim(&self) -> usize {
        self.s.len()
    }

    /// Whether `\G` matches at `p`.
    pub(crate) fn at_g(&self, p: usize) -> bool {
        self.g && p == self.pos
    }

    /// The regex `$` at `i`: before the line feed or at the end of the text.
    pub(crate) fn at_eol(&self, i: usize) -> bool {
        i == self.s.len() || (self.nl && i == self.s.len() + 1)
    }

    /// Whether the text at `i` starts with `text`. The first byte decides
    /// almost always, so it is tested before the call that compares the rest.
    pub(crate) fn starts(&self, i: usize, text: &[u8]) -> bool {
        text.first().is_some_and(|b| self.s.get(i) == Some(b))
            && self.s.get(i..).is_some_and(|rest| rest.starts_with(text))
    }

    /// The end of the run of bytes from `i` that `class` accepts.
    pub(crate) fn skip(&self, mut i: usize, class: impl Fn(u8) -> bool) -> usize {
        let from = i;
        while let Some(&b) = self.s.get(i)
            && class(b)
        {
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

    /// Whether the line has spent its work budget.
    pub(crate) fn over_budget(&self) -> bool {
        self.steps.get() > self.budget
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
        self.dirty = true;
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

    // Spans.

    /// Names `start..end` with `scopes` and then one scope that ends in
    /// `dynamic`.
    pub(crate) fn span_dyn(
        &mut self,
        start: usize,
        end: usize,
        scopes: &'static [G::Scope],
        dynamic: DynName,
    ) {
        self.spans.push(Span {
            start,
            end,
            scopes,
            single: None,
            dynamic: Some(dynamic),
            retok: None,
        });
        self.dirty = true;
    }

    /// Names `start..end` with `scope`, a scope the matcher chose, and lexes
    /// it again with `retok` when it names a rule.
    pub(crate) fn span_one(
        &mut self,
        start: usize,
        end: usize,
        scope: Option<G::Scope>,
        retok: Option<G::Rule>,
    ) {
        self.spans.push(Span {
            start,
            end,
            scopes: &[],
            single: scope,
            dynamic: None,
            retok,
        });
        self.dirty = true;
    }

    /// Drops the spans of a match that failed after naming some.
    pub(crate) fn clear_spans(&mut self) {
        self.spans.truncate(self.span_base);
    }

    // Frames.

    fn top(&self) -> &Frame<G> {
        &self.frames[self.frames.len() - 1]
    }

    /// The innermost rule.
    pub(crate) fn top_rule(&self) -> G::Rule {
        self.top().rule
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
    pub(crate) fn at_depth_limit(&self) -> bool {
        self.limits.depth.is_some_and(|max| self.depth() >= max)
    }

    /// The open rules, the root included.
    pub(crate) fn frame_count(&self) -> usize {
        self.frames.len()
    }

    /// The most rules the depth limit lets be open, when it is set.
    pub(crate) fn depth_limit(&self) -> Option<usize> {
        self.limits.depth
    }

    /// Leaves the text from the cursor up to `next` plain, as the innermost
    /// rule's content, with a token boundary at `start`, and moves the cursor
    /// there: what a language does for a rule that the depth limit keeps
    /// closed when it has not skipped the pattern.
    pub(crate) fn skip_plain(&mut self, start: usize, next: usize) {
        let next = next.min(self.s.len());
        let node = self.top().content;
        self.emit(start, node);
        self.emit(next, node);
        self.pos = self.pos.max(next);
        self.g = self.anchor == Some(self.pos);
    }

    /// Notes that a limit kept a rule from opening.
    pub(crate) fn record_depth(&mut self) {
        self.report.record(Limit::Depth);
    }

    /// The stack `base` with `scopes` on top. A scope the node limit keeps
    /// from being made is left out.
    pub(crate) fn path(&mut self, base: u32, scopes: &[G::Scope]) -> u32 {
        if scopes.is_empty() {
            return base;
        }
        let ptr = scopes.as_ptr().addr();
        let slot =
            (usize::try_from(base).unwrap_or(0).wrapping_mul(0x9e37) ^ (ptr >> 3)) % PATH_ENTRIES;
        if let Some(entry) = self.scratch.paths[slot]
            && entry.base == base
            && entry.ptr == ptr
            && entry.len == scopes.len()
        {
            return entry.node;
        }
        let mut stack = base;
        let mut whole = true;
        for &scope in scopes {
            let Some(child) = self.arena.child(stack, G::scope(scope)) else {
                self.report.record(Limit::Nodes);
                self.flags.failed = true;
                whole = false;
                break;
            };
            stack = child;
        }
        if whole {
            self.scratch.paths[slot] = Some(PathEntry {
                base,
                ptr,
                len: scopes.len(),
                node: stack,
            });
        }
        stack
    }

    /// The stack `base` with one scope on top, which is not cached: it is the
    /// scope of a run-time choice, with no slice of its own to key a cache.
    fn path_one(&mut self, base: u32, scope: G::Scope) -> u32 {
        self.arena.child(base, G::scope(scope)).unwrap_or_else(|| {
            self.report.record(Limit::Nodes);
            self.flags.failed = true;
            base
        })
    }

    /// The stack `base` with the scopes `prefix`, the text of `source` and
    /// `suffix` spell out on top. A name with spaces is several scopes, and
    /// leading dots of the text are dropped, as a `TextMate` engine does for
    /// the text of a capture.
    fn dynamic_path(&mut self, base: u32, prefix: &str, source: NameSource, suffix: &str) -> u32 {
        let mut name = std::mem::take(&mut self.scratch.name);
        name.clear();
        name.push_str(prefix);
        {
            let s = self.s;
            let text = match source {
                NameSource::Line { start, end } => {
                    String::from_utf8_lossy(s.get(start..end).unwrap_or_default())
                }
                NameSource::Interned(id) => Cow::Borrowed(self.arena.string(id)),
            };
            name.push_str(text.trim_start_matches('.'));
        }
        name.push_str(suffix);
        let mut stack = base;
        for part in name.split(' ') {
            let child = self
                .arena
                .named_scope(part)
                .and_then(|scope| self.arena.child(stack, scope));
            let Some(child) = child else {
                self.report.record(Limit::Nodes);
                self.flags.failed = true;
                break;
            };
            stack = child;
        }
        self.scratch.name = name;
        stack
    }

    fn push_frame(
        &mut self,
        rule: G::Rule,
        enter: Option<usize>,
        anchor: Option<usize>,
        payload: u32,
    ) {
        let parent_state = self.top().state;
        let parent_scopes = self.top().content;
        let code = G::rule_code(rule);
        let slot = (usize::try_from(parent_state)
            .unwrap_or(0)
            .wrapping_mul(0x9e37)
            ^ usize::try_from(parent_scopes)
                .unwrap_or(0)
                .wrapping_mul(0x85eb)
            ^ usize::from(code).wrapping_mul(31)
            ^ usize::try_from(payload).unwrap_or(0))
            % PUSH_ENTRIES;
        if let Some(entry) = self.scratch.pushes[slot]
            && entry.parent == parent_state
            && entry.scopes == parent_scopes
            && entry.code == code
            && entry.payload == payload
        {
            self.frames.push(Frame {
                rule,
                enter,
                anchor,
                name: entry.name,
                content: entry.content,
                state: entry.state,
                payload,
            });
            return;
        }
        self.flags.failed = false;
        let parent = self.top().content;
        let name = self.path(parent, G::rule_name(rule));
        let content = match G::rule_content_delimiter(rule) {
            Some(prefix) => self.dynamic_path(name, prefix, NameSource::Interned(payload), ""),
            None => self.path(name, G::rule_content(rule)),
        };
        let state = self
            .arena
            .push(parent_state, code, payload, content)
            .unwrap_or_else(|| {
                self.report.record(Limit::Nodes);
                self.flags.failed = true;
                parent_state
            });
        if !self.flags.failed {
            self.scratch.pushes[slot] = Some(PushEntry {
                parent: parent_state,
                scopes: parent_scopes,
                code,
                payload,
                name,
                content,
                state,
            });
        }
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
    /// capture groups and spans on top of it where they lie.
    fn emit_match(&mut self, captures: Captures<G>, base: u32, start: usize, end: usize) {
        if !captures.0.is_empty() || !captures.1.is_empty() {
            self.emit_captures(captures, base, start, end);
        }
        self.emit_spans(base);
        self.emit(end, base);
    }

    /// Emits a pattern's table of capture groups. A group that the grammar
    /// tokenizes with patterns of its own runs them under the match's stack
    /// `base` plus that group's name, not under the groups around it, as
    /// vscode-textmate's `handleCaptures` does.
    fn emit_captures(&mut self, (table, nested): Captures<G>, base: u32, start: usize, end: usize) {
        self.caps[0] = Some((start, end));
        self.dirty = true;
        let mut stack = [(0_usize, 0_u32); NEST];
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
                let named = table
                    .iter()
                    .find(|&&(number, _)| number == group)
                    .map_or(&[][..], |&(_, scopes)| scopes);
                let inner = self.path(base, named);
                self.nested(kind, from, to, inner);
                continue;
            }
            if scopes.is_empty() {
                continue;
            }
            if depth == NEST {
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
    }

    /// Lexes the capture group text `from..to` with the grammar's hook, which
    /// matches patterns of its own: the groups of the match being emitted
    /// are saved meanwhile, on a stack, so the hook's matches, and the
    /// nested groups those emit in turn, do not overwrite them.
    fn nested(&mut self, kind: G::Nested, from: usize, to: usize, base: u32) {
        let mark = self.scratch.saved_caps.len();
        self.scratch.saved_caps.extend_from_slice(&self.caps);
        let dirty = self.dirty;
        self.caps[..G::GROUPS].fill(None);
        self.dirty = false;
        G::tokenize_nested(self, kind, from, to, base);
        self.caps.copy_from_slice(&self.scratch.saved_caps[mark..]);
        self.scratch.saved_caps.truncate(mark);
        self.dirty = dirty;
    }

    /// Emits the spans the matcher recorded, in order.
    fn emit_spans(&mut self, base: u32) {
        for i in self.span_base..self.spans.len() {
            let span = self.spans[i];
            if span.start == span.end {
                continue;
            }
            self.emit(span.start, base);
            let mut named = self.path(base, span.scopes);
            if let Some(scope) = span.single {
                named = self.path_one(named, scope);
            }
            let node = match span.dynamic {
                Some(dynamic) => self.dynamic_path(
                    named,
                    dynamic.prefix,
                    NameSource::Line {
                        start: dynamic.start,
                        end: dynamic.end,
                    },
                    dynamic.suffix,
                ),
                None => named,
            };
            match span.retok {
                Some(rule) if G::skip_retok(rule, &self.s[span.start..span.end]) => {
                    self.emit(span.end, node);
                }
                Some(rule) => self.retokenise(rule, span.start, span.end, node),
                None => self.emit(span.end, node),
            }
        }
    }

    /// Lexes `start..end` again with the patterns of `rule`, in a stack whose
    /// innermost scopes are `node`. Whatever it opens is dropped at `end`.
    /// Afterwards the groups are the inner matches', and the spans of the
    /// match being emitted are still recorded, so the next try clears both.
    pub(crate) fn retokenise(&mut self, rule: G::Rule, start: usize, end: usize, node: u32) {
        if self.at_depth_limit() {
            self.report.record(Limit::Depth);
            self.emit(end, node);
            return;
        }
        let saved = (self.s, self.nl, self.pos, self.anchor, self.frames.len());
        let saved_base = self.span_base;
        self.span_base = self.spans.len();
        let state = self.top().state;
        self.frames.push(Frame {
            rule,
            enter: Some(start),
            anchor: None,
            name: node,
            content: node,
            state,
            payload: 0,
        });
        if let Some(text) = self.s.get(..end) {
            self.s = text;
            self.nl = false;
        }
        self.pos = start;
        self.anchor = None;
        self.tokenize();
        let depth;
        (self.s, self.nl, self.pos, self.anchor, depth) = saved;
        self.frames.truncate(depth);
        self.spans.truncate(self.span_base);
        self.span_base = saved_base;
        self.dirty = true;
    }

    // The loop.

    /// Runs from `pos` until the text ends or the line stops.
    fn tokenize(&mut self) {
        let line_length = self.s.len() + usize::from(self.nl);
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
            let Some((start, pat, end)) = G::scan(self) else {
                if self.steps.get() > self.budget {
                    self.report.record(Limit::Work);
                }
                let node = self.top().content;
                self.emit(line_length, node);
                return;
            };
            let advanced = end > self.pos;
            let (act, captures) = G::pat_data(pat);
            let stop = match act {
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

    /// Tries `pat` at `p`, clearing what the last try left.
    #[inline]
    pub(crate) fn try_pat(&mut self, pat: G::Pat, p: usize) -> Option<usize> {
        if self.dirty {
            self.caps[..G::GROUPS].fill(None);
            self.spans.truncate(self.span_base);
            self.dirty = false;
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
        self.emit_match(table, top.name, start, end);
        let popped = self.pop_frame();
        self.anchor = popped.anchor;
        if !advanced && popped.enter == Some(self.pos) {
            // The rule opened and closed without advancing: the grammar would
            // loop, so the rest of the line is the rule's.
            self.flags.kept = true;
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
        let payload = match G::rule_payload(rule) {
            Payload::None if G::CAPTURED_EOL && end > self.s.len() => CAPTURED_EOL,
            Payload::None => 0,
            Payload::Backref(group) => {
                self.caps[0] = Some((start, end));
                self.dirty = true;
                let text = std::str::from_utf8(self.cap_text(usize::from(group))).unwrap_or("");
                self.arena.intern(text).unwrap_or_else(|| {
                    self.report.record(Limit::Nodes);
                    0
                })
            }
        };
        // The content scopes come after the begin tokens.
        let anchor = self.anchor;
        self.push_frame(rule, Some(self.pos), anchor, payload);
        let name = self.top().name;
        let content = self.top().content;
        let top = self.frames.len() - 1;
        self.frames[top].content = name;
        self.emit_match(table, name, start, end);
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
        self.emit_match(table, base, start, end);
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

#[cfg(test)]
mod tests {
    use super::{Lexer, Scratch};
    use crate::arena::{Arena, ROOT};
    use crate::limits::{Limits, LineReport};
    use crate::rules::{Act, Captures, Grammar, Text};
    use crate::scope::Scope;

    #[derive(Clone, Copy, Debug, PartialEq)]
    struct Root;

    /// A grammar of bracket pairs whose inside is lexed again with the same
    /// rule, and words: `[[ab]]` nests one capture group's hook inside
    /// another's, which must leave the outer match's later groups intact.
    #[derive(Clone, Copy, Debug)]
    struct Brackets;

    #[derive(Clone, Copy, Debug)]
    enum Pat {
        Bracket,
        Word,
    }

    #[derive(Clone, Copy, Debug)]
    enum Inside {
        Again,
    }

    /// The scopes the test interns first, in this order.
    const OPEN: Scope = Scope::dynamic(1);
    const WORD: Scope = Scope::dynamic(2);
    const CLOSE: Scope = Scope::dynamic(3);

    impl Grammar for Brackets {
        type Scope = Scope;
        type Rule = Root;
        type Pat = Pat;
        type Nested = Inside;
        type Extra = ();

        const NAME: &'static str = "brackets";
        const ROOT: Root = Root;
        const GROUPS: usize = 4;

        fn scope(scope: Scope) -> Scope {
            scope
        }

        fn rule_code(_: Root) -> u16 {
            0
        }

        fn rule_from_code(_: u16) -> Root {
            Root
        }

        fn rule_name(_: Root) -> &'static [Scope] {
            &[]
        }

        fn rule_content(_: Root) -> &'static [Scope] {
            &[]
        }

        fn pat_act(_: Pat) -> Act<Self> {
            Act::Match(&[])
        }

        fn pat_captures(pat: Pat) -> Captures<Self> {
            match pat {
                Pat::Bracket => (&[(1, &[OPEN]), (3, &[CLOSE])], &[(2, Inside::Again)]),
                Pat::Word => (&[(1, &[WORD])], &[]),
            }
        }

        fn scan(lexer: &mut Lexer<'_, Self>) -> Option<(usize, Pat, usize)> {
            let from = lexer.pos;
            for p in from..lexer.lim() {
                for pat in [Pat::Bracket, Pat::Word] {
                    if let Some(end) = lexer.try_pat(pat, p) {
                        return Some((p, pat, end));
                    }
                }
            }
            None
        }

        fn matcher(lexer: &mut Lexer<'_, Self>, pat: Pat, p: usize) -> Option<usize> {
            match pat {
                Pat::Bracket => {
                    if lexer.at(p) != b'[' {
                        return None;
                    }
                    let close = p + lexer.s.get(p..)?.iter().rposition(|&b| b == b']')?;
                    lexer.cap(1, p, p + 1);
                    lexer.cap(2, p + 1, close);
                    lexer.cap(3, close, close + 1);
                    Some(close + 1)
                }
                Pat::Word => {
                    let end = lexer.skip(p, |b| b.is_ascii_alphabetic());
                    (end > p).then(|| {
                        lexer.cap(1, p, end);
                        end
                    })
                }
            }
        }

        fn tokenize_nested(
            lexer: &mut Lexer<'_, Self>,
            kind: Inside,
            from: usize,
            to: usize,
            base: u32,
        ) {
            match kind {
                Inside::Again => lexer.retokenise(Root, from, to, base),
            }
        }
    }

    #[test]
    fn a_nested_hook_keeps_the_groups_of_the_matches_around_it() {
        let mut arena = Arena::new(None);
        let names = ["open.test", "word.test", "close.test"].map(|name| arena.named_scope(name));
        assert_eq!(names, [Some(OPEN), Some(WORD), Some(CLOSE)]);
        let limits = Limits::default();
        let mut tokens = Vec::new();
        let mut report = LineReport::default();
        let mut scratch = Scratch::<Brackets>::default();
        let text = Text {
            bytes: b"[[ab]]",
            start: 0,
            anchored: false,
        };
        Lexer::new(
            &mut arena,
            &limits,
            text,
            &mut tokens,
            ROOT,
            &mut report,
            &mut scratch,
        )
        .run();
        let innermost: Vec<(usize, &str)> = tokens
            .iter()
            .map(|token| {
                (
                    token.end(),
                    arena.scopes(token.stack().0).next().unwrap_or(""),
                )
            })
            .collect();
        assert_eq!(
            innermost,
            [
                (1, "open.test"),
                (2, "open.test"),
                (4, "word.test"),
                (5, "close.test"),
                (6, "close.test")
            ]
        );
    }
}
