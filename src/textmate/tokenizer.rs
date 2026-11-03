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

use regex_automata::meta::Cache;

use super::grammar::{Cap, DYNAMIC_NAME, Grammar, List, NONE, Rule, RuleKind};
use super::regex::{self, Context, Regex};
use super::scanner::Scanner;
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
/// The start a pattern has when it matches nowhere in the rest of the line.
const NO_MATCH: u32 = u32::MAX;
/// Machine steps per step of the work limit: the limit counts decisions of a
/// lexer, and one decision of the engine is some dozens of the machine's
/// instructions. Chosen so that no oracle line spends half its default budget.
const STEP: u64 = 32;
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

/// The match that starts first so far.
#[derive(Clone, Copy)]
struct Best {
    start: usize,
    /// The rule it opens, or `None` for the `end` pattern.
    rule: Option<usize>,
    /// Its place in the order the rules are tried: the `end` pattern first,
    /// then the list, or last with `applyEndPatternLast`.
    order: usize,
    pat: Pat,
    /// Whether its groups are in the `best` scratch.
    groups_kept: bool,
}

/// What a search runs.
#[derive(Clone, Copy)]
enum Pat {
    /// A pattern of the grammar.
    Table(usize),
    /// A pattern built from a match, in [`Scratch::dynamic`].
    Built(usize),
}

impl Scratch {
    /// Hands the automata's caches back to their scanners, so that the next
    /// highlighter of the grammar does not build their states again.
    fn release_caches(&mut self) {
        let Some(grammar) = self.cached_for else {
            return;
        };
        for (index, slot) in self.meta_caches.iter_mut().enumerate() {
            if let Some(cache) = slot.take()
                && let Some(list) = grammar.lists.get(index)
            {
                list.scanner(grammar).give_cache(cache);
            }
        }
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        self.release_caches();
    }
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
    /// Per pattern of the grammar: the line it was searched on and where its
    /// next match starts.
    found: Vec<(u32, u32)>,
    /// Per list of rules: the line it was scanned on, where its next match
    /// starts and which rule of the list it opens.
    list_found: Vec<(u32, u32, u32)>,
    meta_caches: Vec<Option<Cache>>,
    /// Whether the scanners are left out, to measure what they gain.
    pub(crate) no_scanner: bool,
    /// The steps the last line spent.
    pub(crate) spent: u64,
    epoch: u32,
    /// The last `end` or `while` pattern filled with a match's groups, kept
    /// so that filling the next reuses its buffer.
    filled: String,
    /// Each filled pattern, compiled once: its text is interned in the arena,
    /// so the arena's node limit and [`MAX_END`] bound how many there are.
    dynamic: Vec<Option<Regex>>,
    /// The place in `dynamic` of each interned pattern number.
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
    scratch.spent = 0;
    if line.is_empty() {
        return state;
    }
    if !scratch.cached_for.is_some_and(|g| std::ptr::eq(g, grammar)) {
        scratch.release_caches();
        scratch.cached_for = Some(grammar);
        scratch.scopes.clear();
        scratch.scopes.resize(grammar.names.len(), None);
        scratch.found.clear();
        scratch.found.resize(grammar.patterns.len(), (0, NO_MATCH));
        scratch.list_found.clear();
        scratch
            .list_found
            .resize(grammar.lists.len(), (0, NO_MATCH, 0));
        scratch.meta_caches.clear();
        scratch
            .meta_caches
            .resize_with(grammar.lists.len(), || None);
        scratch.epoch = 0;
    }
    scratch.epoch = scratch.epoch.wrapping_add(1);
    if scratch.epoch == 0 {
        scratch.found.iter_mut().for_each(|f| f.0 = 0);
        scratch.list_found.iter_mut().for_each(|f| f.0 = 0);
        scratch.epoch = 1;
    }
    scratch.text.clear();
    scratch.text.extend_from_slice(line);
    scratch.text.push(b'\n');
    let mut frames = std::mem::take(&mut scratch.frames);
    let first = rebuild(grammar, arena, state, &mut frames);
    let steps = limits
        .work
        .map_or(u64::MAX, |work| work.steps(line.len()).saturating_mul(STEP));
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
    tokenizer.s.spent = steps.saturating_sub(tokenizer.steps);
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

/// Writes `source` into `out` with `\1`..`\9` replaced by the escaped text
/// of the groups, reusing `out`'s buffer.
fn fill_backrefs(source: &str, text: &[u8], caps: &Caps, out: &mut String) {
    out.clear();
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
                .unwrap_or_default();
            for g in String::from_utf8_lossy(group).chars() {
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

    /// Where `pat` next matches at or after `from`, and whether the search
    /// ran now, leaving its groups in `cur`.
    ///
    /// A pattern that reads nothing but the text gives the same match from
    /// every position up to its start, and none from any later position when
    /// it found none, so the answer is kept for the rest of the line.
    fn probe(
        &mut self,
        pat: Pat,
        from: usize,
        end: usize,
        anchor: usize,
        allow_a: DocStart,
    ) -> (Option<usize>, bool) {
        let slot = match pat {
            Pat::Table(index)
                if end == self.s.text.len()
                    && self
                        .grammar
                        .patterns
                        .get(index)
                        .and_then(|p| p.regex())
                        .is_some_and(Regex::context_free) =>
            {
                Some(index)
            }
            Pat::Table(_) | Pat::Built(_) => None,
        };
        if let Some(index) = slot
            && let Some(&(epoch, start)) = self.s.found.get(index)
            && epoch == self.s.epoch
        {
            if start == NO_MATCH {
                return (None, false);
            }
            if let Ok(start) = usize::try_from(start)
                && start >= from
            {
                return (Some(start), false);
            }
        }
        let found = self.search(pat, from, end, anchor, allow_a);
        let start = found.then(|| self.s.cur.group(0).map_or(from, |g| g.0));
        if let Some(index) = slot
            && !self.stopped
            && let Some(entry) = self.s.found.get_mut(index)
        {
            let stored = match start {
                None => Some(NO_MATCH),
                Some(at) => u32::try_from(at).ok(),
            };
            if let Some(stored) = stored {
                *entry = (self.s.epoch, stored);
            }
        }
        (start, true)
    }

    /// Keeps `start` as the best match if it starts before the best so far
    /// (the earlier rule wins a tie).
    fn consider(
        &mut self,
        (start, fresh): (Option<usize>, bool),
        rule: Option<usize>,
        (pat, order): (Pat, usize),
        best: &mut Option<Best>,
    ) {
        let Some(start) = start else { return };
        if best.is_none_or(|b| (start, order) < (b.start, b.order)) {
            if fresh {
                std::mem::swap(&mut self.s.cur, &mut self.s.best);
            }
            *best = Some(Best {
                start,
                rule,
                order,
                pat,
                groups_kept: fresh,
            });
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
        let mut filled = std::mem::take(&mut self.s.filled);
        fill_backrefs(slot.source, &self.s.text, caps, &mut filled);
        let interned = self.arena.intern(&filled);
        self.s.filled = filled;
        match interned {
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

    /// Offers every rule of a list to `best`, through the list's scanner
    /// unless the scanners are off.
    fn scan_list(
        &mut self,
        list_id: usize,
        (pos, end, anchor, allow_a): (usize, usize, usize, DocStart),
        best: &mut Option<Best>,
    ) {
        let list: &'static List = &self.grammar.lists[list_id];
        let decided = |best: &Option<Best>, order: usize| {
            best.is_some_and(|b| b.start == pos && b.order < order)
        };
        if self.s.no_scanner {
            for (index, &id) in list.rules.iter().enumerate() {
                if self.stopped || decided(best, index + 1) {
                    return;
                }
                let pat = Pat::Table(self.grammar.rules[id].begin);
                let probed = self.probe(pat, pos, end, anchor, allow_a);
                self.consider(probed, Some(id), (pat, index + 1), best);
            }
            return;
        }
        let scanner = list.scanner(self.grammar);
        for &index in &scanner.residual {
            if self.stopped || decided(best, index + 1) {
                break;
            }
            let id = list.rules[index];
            let pat = Pat::Table(self.grammar.rules[id].begin);
            let probed = self.probe(pat, pos, end, anchor, allow_a);
            self.consider(probed, Some(id), (pat, index + 1), best);
        }
        if self.stopped || decided(best, 1) {
            return;
        }
        let (found, fresh) = self.winner(list_id, list, scanner, pos, end);
        if let Some((start, index)) = found {
            let id = list.rules[index];
            let pat = Pat::Table(self.grammar.rules[id].begin);
            self.consider((Some(start), fresh), Some(id), (pat, index + 1), best);
        }
    }

    /// The first rule of a list the scanner covers that matches at or after
    /// `from`: where it starts and its place in the list, and whether the
    /// search ran now, leaving its groups in `cur`. Kept for the rest of the
    /// line like a pattern's match.
    fn winner(
        &mut self,
        list_id: usize,
        list: &'static List,
        scanner: &'static Scanner,
        from: usize,
        end: usize,
    ) -> (Option<(usize, usize)>, bool) {
        let cacheable = end == self.s.text.len();
        if cacheable
            && let Some(&(epoch, start, index)) = self.s.list_found.get(list_id)
            && epoch == self.s.epoch
        {
            if start == NO_MATCH {
                return (None, false);
            }
            if let (Ok(start), Ok(index)) = (usize::try_from(start), usize::try_from(index))
                && start >= from
            {
                return (Some((start, index)), false);
            }
        }
        let found = self.scan(list_id, list, scanner, from, end);
        if cacheable
            && !self.stopped
            && let Some(entry) = self.s.list_found.get_mut(list_id)
        {
            let stored = match found {
                None => Some((NO_MATCH, 0)),
                Some((start, index)) => u32::try_from(start).ok().zip(u32::try_from(index).ok()),
            };
            if let Some((start, index)) = stored {
                *entry = (self.s.epoch, start, index);
            }
        }
        (found, true)
    }

    /// Runs the automaton from `from`, and the exact machine at each place it
    /// finds, until a pattern really matches.
    ///
    /// The automaton reads every byte it passes, and the work limit counts
    /// them: a line of candidates that each read to its end would otherwise
    /// cost the square of its length.
    fn scan(
        &mut self,
        list_id: usize,
        list: &'static List,
        scanner: &'static Scanner,
        from: usize,
        end: usize,
    ) -> Option<(usize, usize)> {
        let mut cache = match self.s.meta_caches.get_mut(list_id)?.take() {
            Some(cache) => cache,
            None => scanner.take_cache()?,
        };
        let mut at = from;
        let mut found = None;
        while at < end && !self.stopped {
            let Some(text) = self.s.text.get(..end) else {
                break;
            };
            let candidate = scanner.candidate(&mut cache, text, at);
            let read = candidate
                .map_or(end, |(_, _, stop)| stop)
                .saturating_sub(at);
            self.charge(u64::try_from(read + 1).unwrap_or(u64::MAX));
            let Some((start, first, _)) = candidate else {
                break;
            };
            if self.stopped {
                break;
            }
            let Some(&byte) = self.s.text.get(start) else {
                break;
            };
            for index in scanner.starting_with(byte, first) {
                let pat = Pat::Table(self.grammar.rules[list.rules[index]].begin);
                if self.match_at(pat, start, end) {
                    found = Some((start, index));
                    break;
                }
                if self.stopped {
                    break;
                }
            }
            if found.is_some() {
                break;
            }
            at = start + 1;
            while self.s.text.get(at).is_some_and(|&b| b & 0xc0 == 0x80) {
                at += 1;
            }
        }
        if let Some(slot) = self.s.meta_caches.get_mut(list_id) {
            *slot = Some(cache);
        }
        found
    }

    /// Spends `amount` steps of the line's work, and ends the line when none
    /// are left.
    fn charge(&mut self, amount: u64) {
        self.steps = self.steps.saturating_sub(amount);
        if self.steps == 0 && !self.stopped {
            self.stopped = true;
            self.report.record(Limit::Work);
        }
    }

    /// Whether `pat`, a pattern of the grammar that reads only the text,
    /// matches exactly at `at`, leaving its groups in `cur`.
    fn match_at(&mut self, pat: Pat, at: usize, end: usize) -> bool {
        let regex = match pat {
            Pat::Table(index) => self.grammar.patterns.get(index).and_then(|p| p.regex()),
            Pat::Built(index) => self.s.dynamic.get(index).and_then(Option::as_ref),
        };
        let Some(regex) = regex else { return false };
        let Some(text) = self.s.text.get(..end) else {
            return false;
        };
        let context = Context {
            search: None,
            text_start: false,
        };
        let found = regex.match_at(&mut self.s.cur, text, at, context, &mut self.steps);
        if self.steps == 0 && !found {
            self.stopped = true;
            self.report.record(Limit::Work);
        }
        found
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
    /// starts first wins, and a match of the `end` pattern closes the open
    /// rule.
    ///
    /// A match kept from an earlier search holds no groups, so it is found
    /// again from where it starts.
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
        let mut best: Option<Best> = None;
        let end_pat = (rule.kind == RuleKind::BeginEnd).then(|| self.end_pat(rule.end, top.end));
        if !rule.apply_end_last
            && let Some(pat) = end_pat
        {
            let probed = self.probe(pat, *pos, end, *anchor, *allow_a);
            self.consider(probed, None, (pat, 0), &mut best);
        }
        self.scan_list(rule.list, (*pos, end, *anchor, *allow_a), &mut best);
        if rule.apply_end_last
            && !self.stopped
            && let Some(pat) = end_pat
        {
            let probed = self.probe(pat, *pos, end, *anchor, *allow_a);
            self.consider(probed, None, (pat, usize::MAX), &mut best);
        }
        if self.stopped {
            return true;
        }
        let Some(best) = best else {
            self.produce(top.content, end);
            return true;
        };
        let matched = best.rule;
        let caps = if best.groups_kept {
            read_caps(&self.s.best)
        } else {
            if !self.search(best.pat, best.start, end, *anchor, *allow_a) {
                self.produce(top.content, end);
                return true;
            }
            read_caps(&self.s.cur)
        };
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
