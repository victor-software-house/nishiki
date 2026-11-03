//! The line tokenizer: the engine's loop over a compiled grammar.
//!
//! A line starts by checking the `while` pattern of each open rule; the first
//! that fails closes itself and every rule inside it. Then the patterns of the
//! innermost rule, and its `end` pattern, are searched from the cursor, and
//! the match that starts first wins (the earliest listed on a tie). A match
//! opens a rule, closes one, or only scopes text, and its capture groups add
//! scopes and may hand their text to other rules.
//!
//! The open rules of a line are the frames of the arena's state, so a state
//! is one integer and equal states are equal ids. A frame packs its rule, the
//! interned text of an `end` or `while` pattern that holds back-references,
//! and whether the rule opened at the end of its line into the frame's
//! payload; its scope stack is the stack of text inside the rule.

use std::collections::HashMap;

use super::grammar::{Cap, DYNAMIC_NAME, Grammar, NONE, Rule, RuleKind};
use super::regex::{self, Context, Regex};
use crate::arena::{Arena, ROOT};
use crate::limits::{Limit, Limits, LineReport};
use crate::scope::Scope;
use crate::{Stack, Token};

/// The frame kind of the engine's states, tagged as the engine's.
const KIND: u16 = crate::tag::TEXTMATE;
/// The payload bit that says the rule opened at the end of its line.
const EOL: u32 = 1 << 16;
/// Where the interned end pattern's number starts in the payload.
const END_SHIFT: u32 = 17;
/// The largest interned end pattern number a payload holds.
const MAX_END: u32 = (1 << 15) - 1;
/// The end-pattern number of the root frame at the start of a document,
/// where `\A` matches.
const FIRST: u32 = 1;
/// A state the arena has not been given yet.
const NEW: u32 = u32::MAX;
/// The capture groups a match keeps: more are never named by a grammar.
const GROUPS: usize = 32;

/// The span of each group of a match.
type Caps = [Option<(usize, usize)>; GROUPS];

/// Whether `\A` can still match: the cursor has not left the start of the
/// document.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DocStart {
    /// The cursor is at the start of the document.
    Open,
    /// The cursor has moved past it, or the line is not the first.
    Passed,
}

/// How a run of the loop is entered.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Entry {
    /// At the start of a line, where the open `while` rules are checked.
    Line,
    /// Inside a capture group whose text rules lex again.
    Capture,
}

/// One open rule.
#[derive(Clone, Copy, Debug)]
struct Frame {
    rule: usize,
    /// The arena state of this frame and those below, or [`NEW`].
    state: u32,
    /// Where the rule was entered on this line, or `usize::MAX`.
    enter: usize,
    /// Where `\G` matches while the rule is the innermost: the end of its
    /// last begin, while or end match; `usize::MAX` for none.
    anchor: usize,
    /// Whether the rule's begin match reached the end of its line.
    eol: bool,
    /// The interned `end` or `while` pattern with back-references filled in,
    /// or 0 for the grammar's own.
    end: u32,
    /// The scope stack of the rule's own text.
    name: u32,
    /// The scope stack of text inside the rule.
    content: u32,
}

/// What a search runs.
#[derive(Clone, Copy)]
enum Pat {
    /// A pattern of the grammar.
    Table(usize),
    /// A pattern built from a match, in [`Scratch::dynamic`].
    Built(usize),
}

/// The buffers and caches the tokenizer keeps between lines.
#[derive(Debug, Default)]
pub(crate) struct Scratch {
    text: Vec<u8>,
    frames: Vec<Frame>,
    pool: Vec<Vec<Frame>>,
    locals: Vec<Vec<(u32, usize)>>,
    cur: regex::Scratch,
    best: regex::Scratch,
    /// The scope of each name of the grammar the caches belong to.
    scopes: Vec<Option<Scope>>,
    cached_for: Option<&'static Grammar>,
    dynamic: Vec<Option<Regex>>,
    dynamic_ids: HashMap<u32, usize>,
}

/// The state at the start of a document in `grammar`, whose text sits on the
/// stack `base` with the grammar's root scope on top.
pub(crate) fn start(arena: &mut Arena, grammar: &Grammar, base: u32) -> u32 {
    let stack = arena
        .named_scope(grammar.scope)
        .and_then(|scope| arena.child(base, scope))
        .unwrap_or(base);
    arena
        .push(ROOT, KIND, pack(grammar.root, FIRST), stack)
        .unwrap_or(ROOT)
}

fn pack(rule: usize, end: u32) -> u32 {
    u32::try_from(rule).unwrap_or(0) | (end << END_SHIFT)
}

/// Lexes one line of `grammar` from `state`, appending its tokens, and
/// returns the state after it. An empty line leaves the state as it is.
#[expect(
    clippy::too_many_arguments,
    reason = "the lexers of the crate all take the same shared parts"
)]
pub(crate) fn line(
    grammar: &'static Grammar,
    scratch: &mut Scratch,
    arena: &mut Arena,
    limits: &Limits,
    line: &[u8],
    state: u32,
    out: &mut Vec<Token>,
    report: &mut LineReport,
) -> u32 {
    if line.is_empty() {
        return state;
    }
    if !scratch.cached_for.is_some_and(|g| std::ptr::eq(g, grammar)) {
        scratch.cached_for = Some(grammar);
        scratch.scopes.clear();
        scratch.scopes.resize(grammar.names.len(), None);
    }
    scratch.text.clear();
    scratch.text.extend_from_slice(line);
    scratch.text.push(b'\n');
    let mut frames = std::mem::take(&mut scratch.frames);
    let first = rebuild(grammar, arena, state, &mut frames);
    let steps = limits.work.map_or(u64::MAX, |work| work.steps(line.len()));
    let out_start = out.len();
    let mut tokenizer = Tokenizer {
        grammar,
        arena,
        limits,
        report,
        out,
        s: scratch,
        len: line.len(),
        last: 0,
        steps,
        out_start,
        stopped: false,
    };
    tokenizer.tokenize_string(
        line.len() + 1,
        if first {
            DocStart::Open
        } else {
            DocStart::Passed
        },
        0,
        &mut frames,
        Entry::Line,
    );
    let top = frames.last().map_or(ROOT, |f| f.content);
    tokenizer.produce(top, line.len());
    let next = tokenizer.intern(&mut frames);
    tokenizer.s.frames = frames;
    next
}

/// Loads the frames of `state`, outermost first; true when it is the start of
/// the document.
fn rebuild(grammar: &Grammar, arena: &Arena, state: u32, frames: &mut Vec<Frame>) -> bool {
    frames.clear();
    let mut at = state;
    while at != ROOT {
        let f = arena.frame(at);
        let rule = usize::try_from(f.payload & 0xffff).unwrap_or(0);
        let rule = if rule < grammar.rules.len() {
            rule
        } else {
            grammar.root
        };
        let mut name = f.stack;
        for _ in 0..grammar.rules[rule].content_name.len() {
            name = arena.parent(name);
        }
        frames.push(Frame {
            rule,
            state: at,
            enter: usize::MAX,
            anchor: usize::MAX,
            eol: f.payload & EOL != 0,
            end: f.payload >> END_SHIFT,
            name,
            content: f.stack,
        });
        at = f.parent;
    }
    frames.reverse();
    let first = frames.first().is_some_and(|f| f.end == FIRST);
    if first && let Some(root) = frames.first_mut() {
        root.end = 0;
        root.state = NEW;
    }
    if frames.is_empty() {
        frames.push(Frame {
            rule: grammar.root,
            state: NEW,
            enter: usize::MAX,
            anchor: usize::MAX,
            eol: false,
            end: 0,
            name: ROOT,
            content: ROOT,
        });
    }
    first
}

/// Replaces `\1`..`\9` in `source` by the escaped text of the groups.
fn fill_backrefs(source: &str, text: &[u8], caps: &Caps) -> String {
    let mut out = String::with_capacity(source.len());
    let mut chars = source.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        let Some(next) = chars.next() else {
            out.push(c);
            break;
        };
        if let Some(d) = next.to_digit(10).filter(|&d| d > 0) {
            let group = usize::try_from(d)
                .ok()
                .and_then(|d| caps.get(d).copied().flatten())
                .and_then(|(s, e)| text.get(s..e))
                .map_or_else(String::new, |t| String::from_utf8_lossy(t).into_owned());
            for g in group.chars() {
                if "-\\{}*+?|^$.,[]()# \t\n".contains(g) {
                    out.push('\\');
                }
                out.push(g);
            }
        } else {
            out.push(c);
            out.push(next);
        }
    }
    out
}

/// A scope name with `$1` replaced by the text of group 1.
fn fill_name(name: &str, text: &[u8], caps: &Caps) -> String {
    let mut out = String::with_capacity(name.len());
    let mut chars = name.chars().peekable();
    while let Some(c) = chars.next() {
        let digit = (c == '$')
            .then(|| chars.peek().and_then(|d| d.to_digit(10)))
            .flatten();
        if let Some(d) = digit {
            chars.next();
            if let Some((s, e)) = usize::try_from(d)
                .ok()
                .and_then(|d| caps.get(d).copied().flatten())
                && let Some(t) = text.get(s..e)
            {
                out.push_str(&String::from_utf8_lossy(t));
            }
        } else {
            out.push(c);
        }
    }
    out
}

struct Tokenizer<'a> {
    grammar: &'static Grammar,
    arena: &'a mut Arena,
    limits: &'a Limits,
    report: &'a mut LineReport,
    out: &'a mut Vec<Token>,
    s: &'a mut Scratch,
    /// The line's length, without the newline the text ends with.
    len: usize,
    /// Where the tokens produced so far end.
    last: usize,
    /// The steps the line may still spend.
    steps: u64,
    /// Where this line's tokens start in `out`.
    out_start: usize,
    /// Whether the work limit ended the line.
    stopped: bool,
}

impl Tokenizer<'_> {
    /// Ends the current token at `end`, with the scopes of `stack`.
    fn produce(&mut self, stack: u32, end: usize) {
        let end = end.min(self.len);
        if end <= self.last {
            return;
        }
        self.last = end;
        if self.out.len() > self.out_start
            && let Some(prev) = self.out.last_mut()
            && prev.stack == Stack(stack)
        {
            prev.end = end;
            return;
        }
        self.out.push(Token {
            end,
            stack: Stack(stack),
        });
    }

    /// The scope of the grammar's name `index`.
    fn scope(&mut self, index: usize) -> Option<Scope> {
        if let Some(Some(scope)) = self.s.scopes.get(index) {
            return Some(*scope);
        }
        let scope = self.arena.named_scope(self.grammar.names.get(index)?);
        if scope.is_none() {
            self.report.record(Limit::Nodes);
        }
        if let Some(slot) = self.s.scopes.get_mut(index) {
            *slot = scope;
        }
        scope
    }

    fn push_scope(&mut self, base: u32, scope: Option<Scope>) -> u32 {
        let Some(scope) = scope else { return base };
        self.arena.child(base, scope).unwrap_or_else(|| {
            self.report.record(Limit::Nodes);
            base
        })
    }

    /// `base` with the scopes `names` name on top, built from `caps` where a
    /// name asks for it.
    fn push_names(&mut self, base: u32, names: &[u16], caps: &Caps) -> u32 {
        let mut stack = base;
        for &reference in names {
            let index = usize::from(reference & !DYNAMIC_NAME);
            if reference & DYNAMIC_NAME == 0 {
                let scope = self.scope(index);
                stack = self.push_scope(stack, scope);
                continue;
            }
            let Some(name) = self.grammar.names.get(index) else {
                continue;
            };
            let name = fill_name(name, &self.s.text, caps);
            for piece in name.split_whitespace() {
                let scope = self.arena.named_scope(piece);
                stack = self.push_scope(stack, scope);
            }
        }
        stack
    }

    /// Keeps the search that just matched if it starts before the best so
    /// far (the earlier rule wins a tie).
    fn consider(
        &mut self,
        pos: usize,
        rule: Option<usize>,
        best: &mut Option<(usize, Option<usize>)>,
    ) {
        let start = self.s.cur.group(0).map_or(pos, |g| g.0);
        if best.is_none_or(|(s, _)| start < s) {
            std::mem::swap(&mut self.s.cur, &mut self.s.best);
            *best = Some((start, rule));
        }
    }

    /// The number of an `end` or `while` pattern built from a match, or 0
    /// when the grammar's own serves.
    fn built_end(&mut self, pattern: usize, caps: &Caps) -> u32 {
        let Some(slot) = self.grammar.patterns.get(pattern) else {
            return 0;
        };
        if !slot.backrefs {
            return 0;
        }
        let source = fill_backrefs(slot.source, &self.s.text, caps);
        match self.arena.intern(&source) {
            Some(id) if id > FIRST && id <= MAX_END => id,
            _ => {
                self.report.record(Limit::Nodes);
                0
            }
        }
    }

    /// The search of `pat` from `from` in the text up to `end`.
    fn search(
        &mut self,
        pat: Pat,
        from: usize,
        end: usize,
        anchor: usize,
        allow_a: DocStart,
    ) -> bool {
        let regex = match pat {
            Pat::Table(index) => self.grammar.patterns.get(index).and_then(|p| p.regex()),
            Pat::Built(index) => self.s.dynamic.get(index).and_then(Option::as_ref),
        };
        let Some(regex) = regex else { return false };
        let Some(text) = self.s.text.get(..end) else {
            return false;
        };
        let context = Context {
            search: (anchor != usize::MAX).then_some(anchor),
            text_start: allow_a == DocStart::Open,
        };
        let found = regex.find(&mut self.s.cur, text, from, context, &mut self.steps);
        if self.steps == 0 && !found {
            self.stopped = true;
            self.report.record(Limit::Work);
        }
        found
    }

    /// The pattern of an `end` or `while`: the table's, or the built one.
    fn end_pat(&mut self, pattern: usize, end: u32) -> Pat {
        if end == 0 {
            return Pat::Table(pattern);
        }
        if let Some(&index) = self.s.dynamic_ids.get(&end) {
            return Pat::Built(index);
        }
        let regex = Regex::new(self.arena.string(end)).ok();
        self.s.dynamic.push(regex);
        let index = self.s.dynamic.len() - 1;
        self.s.dynamic_ids.insert(end, index);
        Pat::Built(index)
    }

    fn tokenize_string(
        &mut self,
        end: usize,
        first_line: DocStart,
        start: usize,
        frames: &mut Vec<Frame>,
        entry: Entry,
    ) {
        let mut pos = start;
        let mut anchor = usize::MAX;
        let mut allow_a = first_line;
        if entry == Entry::Line {
            if frames.last().is_some_and(|f| f.eol) {
                anchor = 0;
            }
            let mut i = 0;
            while i < frames.len() {
                let frame = frames[i];
                let rule = &self.grammar.rules[frame.rule];
                if rule.kind != RuleKind::BeginWhile {
                    i += 1;
                    continue;
                }
                let pat = self.end_pat(rule.until, frame.end);
                if self.search(pat, pos, end, anchor, allow_a) {
                    let caps = read_caps(&self.s.cur);
                    let (s, e) = caps[0].unwrap_or((pos, pos));
                    self.produce(frame.content, s);
                    self.handle_captures(rule.end_captures, &caps, frames, i, end);
                    self.produce(frame.content, e);
                    anchor = e;
                    if e > pos {
                        pos = e;
                        allow_a = DocStart::Passed;
                    }
                    i += 1;
                } else {
                    frames.truncate(i);
                    break;
                }
                if self.stopped {
                    return;
                }
            }
            if self.stopped {
                return;
            }
        }
        while !self.stopped {
            if self.scan_next(end, &mut pos, &mut anchor, &mut allow_a, frames) {
                break;
            }
        }
    }

    /// One step of the loop; true when the line is done. The match that
    /// starts first wins, kept as where it starts and the rule it opens, or
    /// `None` for the `end` pattern, which closes the open rule.
    fn scan_next(
        &mut self,
        end: usize,
        pos: &mut usize,
        anchor: &mut usize,
        allow_a: &mut DocStart,
        frames: &mut Vec<Frame>,
    ) -> bool {
        let Some(&top) = frames.last() else {
            return true;
        };
        let rule: &'static Rule = &self.grammar.rules[top.rule];
        let mut best: Option<(usize, Option<usize>)> = None;
        let end_pat = (rule.kind == RuleKind::BeginEnd).then(|| self.end_pat(rule.end, top.end));
        if !rule.apply_end_last
            && let Some(pat) = end_pat
            && self.search(pat, *pos, end, *anchor, *allow_a)
        {
            self.consider(*pos, None, &mut best);
        }
        for &id in rule.patterns {
            if self.stopped || best.is_some_and(|(s, _)| s == *pos) {
                break;
            }
            let child = &self.grammar.rules[id];
            if self.search(Pat::Table(child.begin), *pos, end, *anchor, *allow_a) {
                self.consider(*pos, Some(id), &mut best);
            }
        }
        if rule.apply_end_last
            && !self.stopped
            && let Some(pat) = end_pat
            && self.search(pat, *pos, end, *anchor, *allow_a)
        {
            self.consider(*pos, None, &mut best);
        }
        if self.stopped {
            return true;
        }
        let Some((_, matched)) = best else {
            self.produce(top.content, end);
            return true;
        };
        let caps = read_caps(&self.s.best);
        let (s, e) = caps[0].unwrap_or((*pos, *pos));
        let advanced = e > *pos;
        let depth = frames.len();
        if let Some(id) = matched {
            if self.enter(id, frames, *pos, anchor, &caps, (s, e), end) {
                return true;
            }
        } else {
            self.produce(frames[depth - 1].content, s);
            frames[depth - 1].content = frames[depth - 1].name;
            let name = frames[depth - 1].name;
            self.handle_captures(rule.end_captures, &caps, frames, depth - 1, end);
            self.produce(name, e);
            let Some(popped) = frames.pop() else {
                return true;
            };
            *anchor = popped.anchor;
            if !advanced && popped.enter == *pos {
                frames.push(Frame {
                    state: NEW,
                    ..popped
                });
                let stack = frames.last().map_or(ROOT, |f| f.content);
                self.produce(stack, end);
                return true;
            }
        }
        if e > *pos {
            *pos = e;
            *allow_a = DocStart::Passed;
        }
        false
    }

    /// Opens `id`, which matched `(s, e)`; true when the line is done.
    #[expect(
        clippy::too_many_arguments,
        reason = "one step of the loop, split out to keep it readable"
    )]
    fn enter(
        &mut self,
        id: usize,
        frames: &mut Vec<Frame>,
        pos: usize,
        anchor: &mut usize,
        caps: &Caps,
        (s, e): (usize, usize),
        end: usize,
    ) -> bool {
        let child: &'static Rule = &self.grammar.rules[id];
        let outer = frames.last().map_or(ROOT, |f| f.content);
        self.produce(outer, s);
        let name = self.push_names(outer, child.name, caps);
        let opens = matches!(child.kind, RuleKind::BeginEnd | RuleKind::BeginWhile);
        if opens && self.limits.depth.is_some_and(|max| frames.len() >= max) {
            self.report.record(Limit::Depth);
            self.produce(name, e);
            if e == pos {
                self.produce(outer, end);
                return true;
            }
            return false;
        }
        let before = frames.len();
        let pattern = if child.kind == RuleKind::BeginEnd {
            child.end
        } else {
            child.until
        };
        let built = if opens {
            self.built_end(pattern, caps)
        } else {
            0
        };
        frames.push(Frame {
            rule: id,
            state: NEW,
            enter: pos,
            anchor: *anchor,
            eol: e == end,
            end: built,
            name,
            content: name,
        });
        let advanced = e > pos;
        match child.kind {
            RuleKind::BeginEnd | RuleKind::BeginWhile => {
                self.handle_captures(child.captures, caps, frames, before, end);
                self.produce(name, e);
                *anchor = e;
                if !child.content_name.is_empty() {
                    let content = self.push_names(name, child.content_name, caps);
                    frames[before].content = content;
                }
                if !advanced && same_rule_as_before(frames, before) {
                    frames.pop();
                    let stack = frames.last().map_or(ROOT, |f| f.content);
                    self.produce(stack, end);
                    return true;
                }
            }
            RuleKind::Match => {
                self.handle_captures(child.captures, caps, frames, before, end);
                self.produce(name, e);
                frames.pop();
                if !advanced {
                    if frames.len() > 1 {
                        frames.pop();
                    }
                    let stack = frames.last().map_or(ROOT, |f| f.content);
                    self.produce(stack, end);
                    return true;
                }
            }
            RuleKind::Container => {}
        }
        false
    }

    /// Gives each capture group its scopes, and hands its text to the rules
    /// it names. `index` is the frame whose scopes the groups sit inside.
    ///
    /// Handing text to rules opens a frame for them, and those rules' captures
    /// can do the same, so the depth limit bounds how deep it goes: past it,
    /// the group's text gets the group's scopes only.
    fn handle_captures(
        &mut self,
        captures: &'static [Cap],
        caps: &Caps,
        frames: &mut [Frame],
        index: usize,
        end: usize,
    ) {
        if captures.is_empty() {
            return;
        }
        let base = frames[index].content;
        let max_end = caps[0].map_or(0, |c| c.1);
        let mut local = self.s.locals.pop().unwrap_or_default();
        local.clear();
        for capture in captures {
            let Some(Some((s, e))) = caps.get(capture.index).copied() else {
                continue;
            };
            if s == e {
                continue;
            }
            if s > max_end {
                break;
            }
            while let Some(&(stack, until)) = local.last() {
                if until <= s {
                    self.produce(stack, until);
                    local.pop();
                } else {
                    break;
                }
            }
            let inside = local.last().map_or(base, |&(stack, _)| stack);
            self.produce(inside, s);
            if capture.retokenize != NONE {
                let named = self.push_names(base, capture.name, caps);
                if self.limits.depth_reached(frames.len()) {
                    self.report.record(Limit::Depth);
                    self.produce(named, e);
                    continue;
                }
                let content = self.push_names(named, capture.content_name, caps);
                let mut sub = self.s.pool.pop().unwrap_or_default();
                sub.clear();
                sub.extend_from_slice(frames);
                sub.push(Frame {
                    rule: capture.retokenize,
                    state: NEW,
                    enter: s,
                    anchor: usize::MAX,
                    eol: false,
                    end: 0,
                    name: named,
                    content,
                });
                let first = if self.last == 0 && s == 0 {
                    DocStart::Open
                } else {
                    DocStart::Passed
                };
                self.tokenize_string(e.min(end), first, s, &mut sub, Entry::Capture);
                self.s.pool.push(sub);
                continue;
            }
            if !capture.name.is_empty() {
                let stack = self.push_names(inside, capture.name, caps);
                local.push((stack, e));
            }
        }
        while let Some((stack, until)) = local.pop() {
            self.produce(stack, until);
        }
        self.s.locals.push(local);
    }

    /// Gives the frames their arena states and returns the innermost.
    fn intern(&mut self, frames: &mut [Frame]) -> u32 {
        let mut parent = ROOT;
        let mut changed = false;
        for frame in frames.iter_mut() {
            changed |= frame.state == NEW;
            if changed {
                let payload = pack(frame.rule, frame.end) | if frame.eol { EOL } else { 0 };
                frame.state = self
                    .arena
                    .push(parent, KIND, payload, frame.content)
                    .unwrap_or_else(|| {
                        self.report.record(Limit::Nodes);
                        ROOT
                    });
            }
            parent = frame.state;
        }
        parent
    }
}

/// The span of each group of a search.
fn read_caps(scratch: &regex::Scratch) -> Caps {
    let mut caps = [None; GROUPS];
    for (index, slot) in caps.iter_mut().enumerate() {
        *slot = scratch.group(index);
    }
    caps
}

/// vscode-textmate's check that a rule pushed without advancing is not the
/// same rule that is already open at the same position.
fn same_rule_as_before(frames: &[Frame], pushed: usize) -> bool {
    let new = frames[pushed];
    let mut at = pushed;
    while at > 0 {
        at -= 1;
        let frame = frames[at];
        if frame.enter != new.enter {
            return false;
        }
        if frame.rule == new.rule {
            return true;
        }
    }
    false
}
