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

use std::sync::OnceLock;

use regex_automata::meta::Cache;

use super::grammar::{Cap, Cells, DYNAMIC_NAME, Grammar, NONE, Rule, RuleKind};
use super::regex::{self, Context, Regex};
use super::scanner::Scanner;
use super::selector::{self, Expr};
use crate::arena::{Arena, IdMap, ROOT};
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

/// The span of each group of a match.
type Caps = [Option<(usize, usize)>];

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
    /// At the start of a line, where the open `while` rules are checked;
    /// `anchored` when `\G` matches at the line's start, as a fence's
    /// `while` rule leaves it.
    Line { anchored: bool },
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
    tier: u8,
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
                && let Some(scanner) = grammar
                    .cells
                    .get()
                    .and_then(|cells| cells.scanners.get(index))
                    .and_then(OnceLock::get)
            {
                scanner.give_cache(cache);
            }
        }
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        self.release_caches();
    }
}

/// One selector of an injection, ready to test.
#[derive(Debug)]
struct Injected {
    priority: i8,
    expr: Expr,
    /// The list of rules it tries.
    list: usize,
}

/// The buffers and caches the tokenizer keeps between lines.
#[derive(Debug, Default)]
pub(crate) struct Scratch {
    text: Vec<u8>,
    frames: Vec<Frame>,
    pool: Vec<Vec<Frame>>,
    locals: Vec<Vec<(u32, usize)>>,
    /// Spans of the groups of matches in progress, kept for reuse.
    caps: Vec<Vec<Option<(usize, usize)>>>,
    /// The machine's buffers, which every search reuses.
    cur: regex::Scratch,
    /// The groups of the best match so far, copied out of `cur`. A copy,
    /// rather than swapping two machine buffers, keeps each buffer in one role,
    /// so both reach the size the document needs on its first pass and a
    /// second pass allocates nothing.
    best: Vec<Option<(usize, usize)>>,
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
    /// The last scope name filled with a match's groups, kept so that filling
    /// the next reuses its buffer.
    name: String,
    /// Each filled pattern, compiled once: its text is interned in the arena,
    /// so the arena's node limit and [`MAX_END`] bound how many there are.
    dynamic: Vec<Option<Regex>>,
    /// The place in `dynamic` of each interned pattern number.
    dynamic_ids: IdMap<u32, usize>,
    /// The grammar's injections, by priority.
    injected: Vec<Injected>,
    /// Per scope stack, the injections whose selector it satisfies.
    injected_at: IdMap<u32, Vec<usize>>,
    /// The injections that apply at the cursor, reused.
    hits: Vec<usize>,
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

/// The state at the start of the body of a fence in `grammar`, whose text sits
/// on the stack `base` with no root scope of the grammar's own, as in Shiki's
/// embedding, and where `\A` never matches.
pub(crate) fn start_embedded(arena: &mut Arena, grammar: &Grammar, base: u32) -> u32 {
    arena
        .push(ROOT, KIND, pack(grammar.root, 0), base)
        .unwrap_or(ROOT)
}

/// A line to lex.
#[derive(Clone, Copy)]
pub(crate) struct Text<'a> {
    pub(crate) grammar: &'static Grammar,
    /// The whole line, without its ending.
    pub(crate) bytes: &'a [u8],
    /// Where the grammar's own text starts: past the containers of a Markdown
    /// fence, so that `^` and lookbehind see the whole line, as in Shiki.
    pub(crate) start: usize,
    /// Whether `\G` matches at `start`, as the `while` rule of a fence leaves
    /// it.
    pub(crate) anchored: bool,
}

/// Lexes one line of a grammar from `state`, appending its tokens, which
/// cover the line from `text.start` with ends counted from the line's start,
/// and returns the state after it. An empty line leaves the state as it is.
pub(crate) fn line(
    scratch: &mut Scratch,
    arena: &mut Arena,
    limits: &Limits,
    text: Text<'_>,
    state: u32,
    out: &mut Vec<Token>,
    report: &mut LineReport,
) -> u32 {
    let Text {
        grammar,
        bytes: line,
        start,
        anchored,
    } = text;
    scratch.spent = 0;
    if line.len() <= start {
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
        scratch.injected = grammar
            .injections
            .iter()
            .flat_map(|injection| {
                let list = grammar
                    .rules
                    .get(injection.rule)
                    .map_or(0, |rule| rule.list);
                selector::parse(injection.selector)
                    .into_iter()
                    .map(move |matcher| Injected {
                        priority: matcher.priority,
                        expr: matcher.expr,
                        list,
                    })
            })
            .collect();
        scratch.injected.sort_by_key(|entry| entry.priority);
        scratch.injected_at.clear();
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
        last: start,
        steps,
        out_start,
        stopped: false,
        cells: grammar.cells(),
    };
    tokenizer.tokenize_string(
        line.len() + 1,
        if first {
            DocStart::Open
        } else {
            DocStart::Passed
        },
        start,
        &mut frames,
        Entry::Line { anchored },
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

/// Writes what a `$` in a scope name stands for into `out`: `$1` the text of
/// group 1, `${1:/downcase}` and `${1:/upcase}` that text in one case.
/// Returns how many characters of `rest`, which follows the `$`, it took, or
/// `None`, writing nothing, when the `$` starts no such form.
///
/// Leading dots of a group's text are dropped, since they would make an invalid
/// scope name. Case is changed a character at a time, which differs from
/// Shiki's only for a final capital sigma.
fn dollar(rest: &str, text: &[u8], caps: &Caps, out: &mut String) -> Option<usize> {
    let group = |digits: &str| {
        let (s, e) = caps.get(digits.parse::<usize>().ok()?).copied().flatten()?;
        text.get(s..e).map(String::from_utf8_lossy)
    };
    if let Some(body) = rest.strip_prefix('{') {
        let close = body.find('}')?;
        let (digits, case) = body[..close].split_once(":/")?;
        let found = group(digits);
        let found = found.as_deref().unwrap_or_default().trim_start_matches('.');
        match case {
            "downcase" => out.extend(found.chars().flat_map(char::to_lowercase)),
            "upcase" => out.extend(found.chars().flat_map(char::to_uppercase)),
            _ => return None,
        }
        return Some(close + 2);
    }
    let digits = rest.bytes().take_while(u8::is_ascii_digit).count();
    if digits == 0 {
        return None;
    }
    let found = group(&rest[..digits]);
    out.push_str(found.as_deref().unwrap_or_default().trim_start_matches('.'));
    Some(digits)
}

/// Writes the scope name `name` into `out` with its `$` forms replaced by the
/// text of the groups, reusing `out`'s buffer.
fn fill_name(name: &str, text: &[u8], caps: &Caps, out: &mut String) {
    out.clear();
    let mut rest = name;
    while let Some(at) = rest.find('$') {
        out.push_str(&rest[..at]);
        rest = &rest[at + 1..];
        match dollar(rest, text, caps, out) {
            Some(taken) => rest = &rest[taken..],
            None => out.push('$'),
        }
    }
    out.push_str(rest);
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
    /// The grammar's [`Grammar::cells`], read once per line.
    cells: &'static Cells,
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
            let mut filled = std::mem::take(&mut self.s.name);
            fill_name(name, &self.s.text, caps, &mut filled);
            for piece in filled.split_whitespace() {
                let scope = self.arena.named_scope(piece);
                stack = self.push_scope(stack, scope);
            }
            self.s.name = filled;
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
                    && self.cells.regex(index).is_some_and(Regex::context_free) =>
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
        (pat, tier, order): (Pat, u8, usize),
        best: &mut Option<Best>,
    ) {
        let Some(start) = start else { return };
        if best.is_none_or(|b| (start, tier, order) < (b.start, b.tier, b.order)) {
            if fresh {
                read_caps(&self.s.cur, &mut self.s.best);
            }
            *best = Some(Best {
                start,
                rule,
                tier,
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
            Pat::Table(index) => self.cells.regex(index),
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
        let regex = Regex::for_grammar(self.arena.string(end)).ok();
        self.s.dynamic.push(regex);
        let index = self.s.dynamic.len() - 1;
        self.s.dynamic_ids.insert(end, index);
        Pat::Built(index)
    }

    /// Offers the rules of the injections that apply to the scopes of `stack`
    /// to `best`. One that starts earlier than the rules of the open rule
    /// wins, and so does one with `L:` that starts where they do; the others
    /// lose a tie.
    fn scan_injections(
        &mut self,
        stack: u32,
        cursor: (usize, usize, usize, DocStart),
        best: &mut Option<Best>,
    ) {
        let mut hits = std::mem::take(&mut self.s.hits);
        hits.clear();
        if let Some(known) = self.s.injected_at.get(&stack) {
            hits.extend_from_slice(known);
        } else {
            let mut names: Vec<&str> = self.arena.scopes(stack).collect();
            names.reverse();
            hits.extend(
                self.s
                    .injected
                    .iter()
                    .enumerate()
                    .filter(|(_, entry)| entry.expr.matches(&names))
                    .map(|(index, _)| index),
            );
            self.s.injected_at.insert(stack, hits.clone());
        }
        for &index in &hits {
            if self.stopped {
                break;
            }
            let entry = &self.s.injected[index];
            let (list, tier) = (entry.list, if entry.priority < 0 { 0 } else { 2 });
            self.scan_list(list, cursor, (tier, (index + 1) << 24), best);
        }
        self.s.hits = hits;
    }

    /// Offers every rule of a list to `best`, through the list's scanner
    /// unless the scanners are off.
    fn scan_list(
        &mut self,
        list_id: usize,
        (pos, end, anchor, allow_a): (usize, usize, usize, DocStart),
        (tier, base): (u8, usize),
        best: &mut Option<Best>,
    ) {
        let decided = |best: &Option<Best>, order: usize| {
            best.is_some_and(|b| b.start == pos && (b.tier, b.order) < (tier, base + order))
        };
        if self.s.no_scanner {
            let Some(list) = self.grammar.lists.get(list_id) else {
                return;
            };
            for (index, &id) in list.rules.iter().enumerate() {
                if self.stopped || decided(best, index + 1) {
                    return;
                }
                let pat = Pat::Table(self.grammar.rules[id].begin);
                let probed = self.probe(pat, pos, end, anchor, allow_a);
                self.consider(probed, Some(id), (pat, tier, base + index + 1), best);
            }
            return;
        }
        let Some(scanner) = self.cells.scanner(list_id) else {
            return;
        };
        for &index in &scanner.residual {
            if self.stopped || decided(best, index + 1) {
                break;
            }
            let id = scanner.rules[index];
            let pat = Pat::Table(self.grammar.rules[id].begin);
            let probed = self.probe(pat, pos, end, anchor, allow_a);
            self.consider(probed, Some(id), (pat, tier, base + index + 1), best);
        }
        if self.stopped || decided(best, 1) {
            return;
        }
        let (found, fresh) = self.winner(list_id, scanner, pos, end);
        if let Some((start, index)) = found {
            let id = scanner.rules[index];
            let pat = Pat::Table(self.grammar.rules[id].begin);
            self.consider(
                (Some(start), fresh),
                Some(id),
                (pat, tier, base + index + 1),
                best,
            );
        }
    }

    /// The first rule of a list the scanner covers that matches at or after
    /// `from`: where it starts and its place in the list, and whether the
    /// search ran now, leaving its groups in `cur`. Kept for the rest of the
    /// line like a pattern's match.
    fn winner(
        &mut self,
        list_id: usize,
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
        let found = self.scan(list_id, scanner, from, end);
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
    ///
    /// The list's automaton cache stays in its slot, borrowed for each search:
    /// moving it out and back would copy the whole cache on every call.
    fn scan(
        &mut self,
        list_id: usize,
        scanner: &'static Scanner,
        from: usize,
        end: usize,
    ) -> Option<(usize, usize)> {
        let slot = self.s.meta_caches.get_mut(list_id)?;
        if slot.is_none() {
            *slot = Some(scanner.take_cache()?);
        }
        let mut at = from;
        let mut found = None;
        while at < end && !self.stopped {
            let (Some(text), Some(Some(cache))) =
                (self.s.text.get(..end), self.s.meta_caches.get_mut(list_id))
            else {
                break;
            };
            let candidate = scanner.candidate(cache, text, at);
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
                let pat = Pat::Table(self.grammar.rules[scanner.rules[index]].begin);
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
            Pat::Table(index) => self.cells.regex(index),
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
        if let Entry::Line { anchored } = entry {
            if anchored {
                anchor = start;
            } else if frames.last().is_some_and(|f| f.eol) {
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
                    let mut caps = self.s.caps.pop().unwrap_or_default();
                    read_caps(&self.s.cur, &mut caps);
                    let (s, e) = caps[0].unwrap_or((pos, pos));
                    self.produce(frame.content, s);
                    self.handle_captures(rule.end_captures, &caps, frames, i, end);
                    self.s.caps.push(caps);
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
            self.consider(probed, None, (pat, 1, 0), &mut best);
        }
        self.scan_list(rule.list, (*pos, end, *anchor, *allow_a), (1, 0), &mut best);
        if rule.apply_end_last
            && !self.stopped
            && let Some(pat) = end_pat
        {
            let probed = self.probe(pat, *pos, end, *anchor, *allow_a);
            self.consider(probed, None, (pat, 1, usize::MAX), &mut best);
        }
        if !self.s.injected.is_empty() && !self.stopped {
            self.scan_injections(top.content, (*pos, end, *anchor, *allow_a), &mut best);
        }
        if self.stopped {
            return true;
        }
        let Some(best) = best else {
            self.produce(top.content, end);
            return true;
        };
        let mut caps = self.s.caps.pop().unwrap_or_default();
        if best.groups_kept {
            caps.clear();
            caps.extend_from_slice(&self.s.best);
        } else {
            if !self.search(best.pat, best.start, end, *anchor, *allow_a) {
                self.s.caps.push(caps);
                self.produce(top.content, end);
                return true;
            }
            read_caps(&self.s.cur, &mut caps);
        }
        let finished = self.apply(best.rule, &caps, rule, end, (pos, anchor, allow_a), frames);
        self.s.caps.push(caps);
        finished
    }

    /// Applies the match `caps` describes, of the rule `matched` (`None` for
    /// the `end` of `rule`); true when the line is done.
    fn apply(
        &mut self,
        matched: Option<usize>,
        caps: &Caps,
        rule: &'static Rule,
        end: usize,
        (pos, anchor, allow_a): (&mut usize, &mut usize, &mut DocStart),
        frames: &mut Vec<Frame>,
    ) -> bool {
        let (s, e) = caps[0].unwrap_or((*pos, *pos));
        let advanced = e > *pos;
        let depth = frames.len();
        if let Some(id) = matched {
            if self.enter(id, frames, *pos, anchor, caps, (s, e), end) {
                return true;
            }
        } else {
            self.produce(frames[depth - 1].content, s);
            frames[depth - 1].content = frames[depth - 1].name;
            let name = frames[depth - 1].name;
            self.handle_captures(rule.end_captures, caps, frames, depth - 1, end);
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

/// The span of each group of a search, in `into`.
fn read_caps(scratch: &regex::Scratch, into: &mut Vec<Option<(usize, usize)>>) {
    into.clear();
    into.extend((0..scratch.group_count()).map(|index| scratch.group(index)));
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
