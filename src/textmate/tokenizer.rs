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

use super::grammar::{Cap, Cells, DYNAMIC_NAME, Folding, Grammar, HandOff, NONE, Rule, RuleKind};
use super::regex::{self, Context, LineKind, Regex};
use super::scanner::Scanner;
use super::selector::{self, Expr};
use crate::arena::{Arena, IdMap, ROOT};
use crate::language::{Context as Lexing, Halt, Line, StopKind, Stopped, halt_before};
use crate::limits::{Limit, Limits, LineReport, Steps, Work};
use crate::scope::Scope;
use crate::{Stack, Token};

#[cfg(test)]
mod tests;

/// The frame kind of the engine's states, tagged as the engine's.
const KIND: u16 = crate::tag::TEXTMATE;
/// The frame kind of a hand-off region. The frame sits on top of the frame of
/// the rule that opened the region. Its payload is the state of the language
/// lexed by hand.
const HAND_OFF_KIND: u16 = KIND | 1;
/// The rule of a hand-off frame. No rule of a grammar has this index.
const HANDED: usize = usize::MAX;
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
/// How many capture groups may hand their text to rules inside one another
/// on a line.
///
/// Each such group runs its rules in a nested call.
/// A debug build spends kilobytes of stack per level, so the depth limit's
/// default of 4,096 contexts overflows a 2 MiB thread.
/// Past this bound, a group gets its own scopes only, as past the depth
/// limit, and the line reports [`Limit::Depth`].
/// Shiki has no such bound.
/// Every oracle fixture lexes the same with this bound.
const MAX_NESTED_CAPTURES: usize = 256;

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

/// Where the loop goes on after a language lexed by hand lexed part of the
/// line. It holds the cursor, the place where `\G` matches, whether `\A` may
/// match, and what the language halted for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Resume {
    at: usize,
    anchor: usize,
    allow_a: DocStart,
    part: Part,
}

/// What a hand-off's language halted for. The engine applies it at the halt.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Part {
    /// Nothing yet. The region starts or goes on here.
    #[default]
    Fresh,
    /// A rule listed before the include, or the end.
    Before,
    /// A rule listed after it.
    After,
}

/// What a step in a hand-off region scans. It replaces the open rule's own
/// list. The step tries the end of `rule` and the rules of `list`.
#[derive(Clone, Copy, Debug)]
struct HandPart {
    /// The rule that opened the region.
    rule: usize,
    /// The hand-off's `before` or `after` list.
    list: usize,
    /// The interned end pattern of the rule's frame.
    end: u32,
    /// Whether the step tries the end.
    with_end: bool,
    /// The hand-off's `before` list.
    before: usize,
    /// The hand-off's `after` list.
    after: usize,
    /// Where the language's halts may start.
    from: usize,
}

/// How a run of the loop over a line stopped.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Stop {
    /// At the line's end.
    End,
    /// Where the language that embeds the grammar takes the line back.
    Halted(usize),
}

/// Where a line halts for the language that embeds the grammar. It also says
/// whether the grammar hands regions to languages lexed by hand.
pub(crate) trait Halting: Copy {
    /// Whether the line can halt at all: false compiles the test out, so a
    /// line lexed on its own pays nothing for it.
    const HALTS: bool;

    /// Whether the grammar hands regions off. When it is false, every
    /// hand-off branch compiles out. A grammar without hand-offs then pays
    /// nothing for them.
    const HANDS_OFF: bool;

    /// The place in `text`, the line's `len` bytes, from `from` where the line
    /// halts before the root rule's next match, which starts at `to`, and how
    /// many bytes the test passed.
    fn halt(self, text: &[u8], from: usize, to: usize, len: usize) -> (Option<usize>, usize);
}

/// A line lexed on its own, which never halts.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Whole;

impl Halting for Whole {
    const HALTS: bool = false;
    const HANDS_OFF: bool = false;

    #[inline]
    fn halt(self, _: &[u8], _: usize, _: usize, _: usize) -> (Option<usize>, usize) {
        (None, 0)
    }
}

/// A line of a grammar that another language embeds, which halts where that
/// language takes it back: at the first place where `end` holds with only the
/// root rule open, tried before the root's patterns, or where `after` holds
/// and none of them match (see [`Halt`]).
#[derive(Clone, Copy, Debug)]
pub(crate) struct Embedded<'a> {
    pub(crate) end: &'a dyn Halt,
    pub(crate) after: Option<&'a dyn Halt>,
}

impl Halting for Embedded<'_> {
    const HALTS: bool = true;
    const HANDS_OFF: bool = false;

    #[inline]
    fn halt(self, text: &[u8], from: usize, to: usize, len: usize) -> (Option<usize>, usize) {
        halt_before(text, from, to, len, self.end, self.after)
    }
}

/// A line of a grammar that hands regions to languages lexed by hand. When
/// another language embeds the grammar, the line halts as [`Embedded`] does.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Handing<'a> {
    pub(crate) halts: Option<Embedded<'a>>,
}

impl Halting for Handing<'_> {
    const HALTS: bool = true;
    const HANDS_OFF: bool = true;

    #[inline]
    fn halt(self, text: &[u8], from: usize, to: usize, len: usize) -> (Option<usize>, usize) {
        self.halts
            .map_or((None, 0), |halts| halts.halt(text, from, to, len))
    }
}

/// The sorted places on a line where a hand-off's halts start. The engine
/// finds them before the language runs. Design section 14.4 of
/// `openspec/changes/engine-handoff` describes the search.
struct Starts<'a>(&'a [usize]);

impl Halt for Starts<'_> {
    fn first(&self, _: &[u8], from: usize, to: usize) -> Option<usize> {
        let at = self.0.partition_point(|&start| start < from);
        self.0.get(at).copied().filter(|&start| start <= to)
    }
}

/// What one step of the loop leaves.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Flow {
    /// The line goes on.
    Next,
    /// The line is done.
    Done,
    /// The line halted at this place for the language that embeds it.
    Halted(usize),
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
    /// Takes the frames of `state` for a line of `grammar`, with whether the
    /// state is the start of the document. The frames the last line left are
    /// reused when they are that state's. Otherwise they are rebuilt from the
    /// arena.
    #[expect(
        clippy::inline_always,
        reason = "at a call once a line: split out of `line_until`, whose code it must keep"
    )]
    #[inline(always)]
    fn take_frames<H: Halting>(
        &mut self,
        grammar: &'static Grammar,
        arena: &Arena,
        state: u32,
    ) -> (Vec<Frame>, bool) {
        let mut frames = std::mem::take(&mut self.frames);
        if self
            .kept
            .is_some_and(|kept| kept.state == state && std::ptr::eq(kept.grammar, grammar))
        {
            return (frames, false);
        }
        let first = rebuild::<H>(grammar, arena, state, &mut frames, &mut self.whiles);
        (frames, first)
    }

    /// Readies the scratch for a line of `grammar`. It loads the grammar's
    /// caches, starts a fresh epoch for the searches, and copies the line's
    /// text with its newline. Returns the grammar's cells for the line's kind.
    #[expect(
        clippy::inline_always,
        reason = "at a call once a line: split out of `line_until`, whose code it must keep"
    )]
    #[inline(always)]
    fn open_line(&mut self, grammar: &'static Grammar, line: &[u8]) -> &'static Cells {
        if !self.current.serves(grammar) {
            self.switch(grammar);
        }
        let cells = self.current.line_cells(grammar, line);
        self.current.next_epoch();
        self.text.clear();
        self.text.extend_from_slice(line);
        self.text.push(b'\n');
        cells
    }

    /// The steps of the work limit the last line spent: its machine steps
    /// over [`STEP`], rounded up, so a line that spent any spent a step.
    pub(crate) fn work_spent(&self) -> u64 {
        self.spent.div_ceil(STEP)
    }

    /// Hands the automata's caches of the current grammar and of every parked
    /// one back to their scanners, so that the next highlighter of a grammar
    /// does not build their states again.
    fn release_caches(&mut self) {
        self.current.give_back();
        for set in &mut self.parked {
            set.give_back();
        }
    }

    /// Makes `grammar`'s caches the current ones: those it left when another
    /// grammar's line came, or new ones on its first line. The current
    /// grammar's caches are parked in their place, so a document that
    /// switches between grammars, as a Markdown document with fences in two
    /// of the engine's languages does, builds each grammar's caches once, and
    /// lexing it again allocates nothing.
    fn switch(&mut self, grammar: &'static Grammar) {
        if self.current.grammar.is_none() {
            self.current.fill(grammar);
            return;
        }
        if let Some(set) = self.parked.iter_mut().find(|set| set.serves(grammar)) {
            std::mem::swap(&mut self.current, set);
        } else {
            let mut fresh = GrammarCaches::default();
            fresh.fill(grammar);
            self.parked
                .push(std::mem::replace(&mut self.current, fresh));
        }
    }
}

/// The caches the tokenizer keeps for one grammar between lines.
#[derive(Debug, Default)]
struct GrammarCaches {
    /// The grammar they belong to, none before the first line.
    grammar: Option<&'static Grammar>,
    /// The scope of each name of the grammar.
    scopes: Vec<Option<Scope>>,
    /// Per pattern of the grammar: the line it was searched on and where its
    /// next match starts.
    found: Vec<(u32, u32)>,
    /// Per list of rules: the line it was scanned on, where its next match
    /// starts and which rule of the list it opens.
    list_found: Vec<(u32, u32, u32)>,
    /// The automata caches of the lists' scanners for lines of
    /// `caches_kind`, boxed so that a slot costs a pointer.
    meta_caches: Vec<Option<Box<Cache>>>,
    /// The kind of line whose scanners `meta_caches` belong to.
    caches_kind: LineKind,
    /// The automata caches of the other kind of line, kept for its next
    /// line.
    idle_caches: Vec<Option<Box<Cache>>>,
    /// Whether the grammar's line kinds run different programs.
    folding: Folding,
    /// The grammar's cells for lean lines, once a line has needed them.
    lean: Option<&'static Cells>,
    /// The grammar's cells for full lines, once a full line has needed them.
    full: Option<&'static Cells>,
    epoch: u32,
    /// The grammar's injections, by priority.
    injected: Vec<Injected>,
    /// Per scope stack, the injections whose selector it satisfies.
    injected_at: IdMap<u32, Vec<usize>>,
}

impl GrammarCaches {
    /// Starts a new epoch. No match an earlier search found is reused after
    /// it. It runs once per line, and again after a hand-off's searches past
    /// the cursor.
    #[expect(
        clippy::inline_always,
        reason = "at a call once a line: split out of `line_until`, whose code it must keep"
    )]
    #[inline(always)]
    fn next_epoch(&mut self) {
        self.epoch = self.epoch.wrapping_add(1);
        if self.epoch == 0 {
            self.found.iter_mut().for_each(|f| f.0 = 0);
            self.list_found.iter_mut().for_each(|f| f.0 = 0);
            self.epoch = 1;
        }
    }

    /// Whether these are `grammar`'s caches.
    fn serves(&self, grammar: &'static Grammar) -> bool {
        self.grammar.is_some_and(|g| std::ptr::eq(g, grammar))
    }

    /// Builds the caches of `grammar`'s first line.
    fn fill(&mut self, grammar: &'static Grammar) {
        self.grammar = Some(grammar);
        self.scopes.clear();
        self.scopes.resize(grammar.names.len(), None);
        self.found.clear();
        self.found.resize(grammar.patterns.len(), (0, NO_MATCH));
        self.list_found.clear();
        self.list_found
            .resize(grammar.lists.len(), (0, NO_MATCH, 0));
        self.meta_caches.clear();
        self.meta_caches.resize_with(grammar.lists.len(), || None);
        self.caches_kind = LineKind::Lean;
        self.idle_caches.clear();
        self.folding = grammar.folding();
        self.lean = None;
        self.full = None;
        self.epoch = 0;
        self.injected = grammar
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
        self.injected.sort_by_key(|entry| entry.priority);
        self.injected_at.clear();
    }

    /// Hands the automaton caches back to their scanners.
    fn give_back(&mut self) {
        let (active, idle) = match self.caches_kind {
            LineKind::Lean => (self.lean, self.full),
            LineKind::Full => (self.full, self.lean),
        };
        give_back(&mut self.meta_caches, active);
        give_back(&mut self.idle_caches, idle);
    }

    /// The kind of `line`, and the grammar's cells for it, making the
    /// automata caches those of that kind: the other kind's are kept, so
    /// that lines of the two kinds in turn keep the states their automata
    /// built.
    #[expect(
        clippy::inline_always,
        reason = "measured at one codegen unit: with `line_until` a second caller, it stopped being inlined, at a call once a line"
    )]
    #[inline(always)]
    fn line_cells(&mut self, grammar: &'static Grammar, line: &[u8]) -> &'static Cells {
        let kind = match self.folding {
            Folding::Same => LineKind::Lean,
            Folding::Differs => LineKind::of(line),
        };
        if self.caches_kind != kind {
            std::mem::swap(&mut self.meta_caches, &mut self.idle_caches);
            self.caches_kind = kind;
            self.meta_caches.resize_with(grammar.lists.len(), || None);
        }
        let slot = match kind {
            LineKind::Lean => &mut self.lean,
            LineKind::Full => &mut self.full,
        };
        if let Some(cells) = *slot {
            return cells;
        }
        let cells = grammar.cells(kind);
        *slot = Some(cells);
        cells
    }
}

/// Hands each of `caches` back to its scanner among `cells`.
fn give_back(caches: &mut [Option<Box<Cache>>], cells: Option<&Cells>) {
    let Some(cells) = cells else {
        return;
    };
    for (index, slot) in caches.iter_mut().enumerate() {
        if let Some(cache) = slot.take()
            && let Some(scanner) = cells.scanners.get(index).and_then(OnceLock::get)
        {
            scanner.give_cache(cache);
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

/// The frames a line left in [`Scratch::frames`], named by their grammar and
/// the state they make.
#[derive(Clone, Copy, Debug)]
struct Kept {
    grammar: &'static Grammar,
    state: u32,
}

/// The buffers and caches the tokenizer keeps between lines.
#[derive(Debug, Default)]
pub(crate) struct Scratch {
    text: Vec<u8>,
    frames: Vec<Frame>,
    /// The grammar and the state of the frames in `frames`, when the last
    /// line left them there. A line from that state reuses them.
    kept: Option<Kept>,
    /// The places in `frames` of the open `while` rules, from the bottom.
    whiles: Vec<usize>,
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
    /// The caches of the grammar whose line is being lexed.
    current: GrammarCaches,
    /// Whether the scanners are left out, to measure what they gain.
    pub(crate) no_scanner: bool,
    /// The machine steps the last line spent.
    pub(crate) spent: u64,
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
    /// The injections that apply at the cursor, reused.
    hits: Vec<usize>,
    /// The caches of every other grammar this scratch has lexed.
    parked: Vec<GrammarCaches>,
    /// Where a hand-off's `before` rules and end start on the line, from the
    /// region's start. The language halts at these places (see [`Starts`]).
    starts_before: Vec<usize>,
    /// Where its `after` rules start.
    starts_after: Vec<usize>,
    /// What the language halted for, for the first step after a hand-off.
    part: Part,
    /// The scratch for an engine line inside a hand-off. An example is
    /// PowerShell in a Markdown fence inside a HAML filter. It swaps in for
    /// this scratch while the hand-lexed language runs. The outer line's
    /// buffers and caches stay as they were. Each engine level keeps the next
    /// level's scratch. The depth of the nesting bounds their number.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "no language runs on the TextMate engine now; its hand-off tests do"
        )
    )]
    spare: Option<Box<Scratch>>,
}

/// The state at the start of a document in `grammar`, whose text sits on the
/// stack `base` with the grammar's root scope on top.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "no language runs on the TextMate engine now; its tests do"
    )
)]
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
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "no language runs on the TextMate engine now; its tests do"
    )
)]
pub(crate) fn start_embedded(arena: &mut Arena, grammar: &Grammar, base: u32) -> u32 {
    arena
        .push(ROOT, KIND, pack(grammar.root, 0), base)
        .unwrap_or(ROOT)
}

/// A line to lex.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "no language runs on the TextMate engine now; its tests do"
    )
)]
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
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "no language runs on the TextMate engine now; its tests do"
    )
)]
pub(crate) fn line(
    scratch: &mut Scratch,
    arena: &mut Arena,
    limits: &Limits,
    text: Text<'_>,
    state: u32,
    out: &mut Vec<Token>,
    report: &mut LineReport,
) -> u32 {
    line_until(scratch, arena, limits, text, state, out, report, Whole).0
}

/// Lexes one line like [`line`], halting where `halt` says the language that
/// embeds the grammar takes the line back: the tokens then end at the halt,
/// and the second value is where. The state at a halt is the root's, since a
/// halt holds only with the root rule open.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "no language runs on the TextMate engine now; its tests do"
    )
)]
#[expect(
    clippy::too_many_arguments,
    reason = "line's arguments and the halt; a struct for them would be built on every line"
)]
pub(crate) fn line_until<H: Halting>(
    scratch: &mut Scratch,
    arena: &mut Arena,
    limits: &Limits,
    text: Text<'_>,
    state: u32,
    out: &mut Vec<Token>,
    report: &mut LineReport,
    halt: H,
) -> (u32, Option<usize>) {
    let Text {
        grammar,
        bytes: line,
        start,
        anchored,
    } = text;
    scratch.spent = 0;
    if line.len() <= start {
        return (state, None);
    }
    let cells = scratch.open_line(grammar, line);
    let (mut frames, first) = scratch.take_frames::<H>(grammar, arena, state);
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
        cells,
        hand: None,
        nested: 0,
    };
    let halted = match tokenizer.tokenize_string(
        line.len() + 1,
        if first {
            DocStart::Open
        } else {
            DocStart::Passed
        },
        start,
        &mut frames,
        Entry::Line { anchored },
        halt,
    ) {
        Stop::Halted(at) => Some(at),
        Stop::End => None,
    };
    if halted.is_none() {
        let top = frames.last().map_or(ROOT, |f| f.content);
        tokenizer.produce(top, line.len());
    }
    let next = tokenizer.intern::<H>(&mut frames);
    tokenizer.s.spent = steps.saturating_sub(tokenizer.steps);
    tokenizer.s.frames = frames;
    (next, halted)
}

/// Lexes one line like [`line_until`], for a grammar that hands regions to
/// languages lexed by hand. Where such a language takes the line over, the
/// step calls it in place. The step goes on where the language halted.
///
/// The language needs the whole scratch. The line swaps the engine's scratch
/// for its spare, two pointers, and lexes with the engine's scratch apart from
/// the whole one. The spare then sits in the whole scratch for any engine line
/// inside a hand-off. The line swaps them back at its end.
///
/// `first_token` is the caller's [`Lexing::first_token`].
/// A hand-off passes it on to its language.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "no language runs on the TextMate engine now; its hand-off tests do"
    )
)]
#[expect(
    clippy::too_many_arguments,
    reason = "line_until's arguments with the whole scratch the languages it hands off to need"
)]
pub(crate) fn line_handing(
    whole: &mut crate::scratch::Scratch,
    arena: &mut Arena,
    limits: &Limits,
    grammar: &'static Grammar,
    line: Line<'_>,
    state: u32,
    out: &mut Vec<Token>,
    report: &mut LineReport,
    first_token: usize,
    halts: Option<Embedded<'_>>,
) -> (u32, Option<usize>) {
    let bytes = line.text.as_bytes();
    let len = bytes.len();
    whole.textmate.spent = 0;
    if len <= line.start {
        return (state, None);
    }
    let mut own = whole.textmate.spare.take().unwrap_or_default();
    std::mem::swap(&mut whole.textmate, &mut own);
    let cells = own.open_line(grammar, bytes);
    let (mut frames, first) = own.take_frames::<Handing<'static>>(grammar, arena, state);
    let steps = limits
        .work
        .map_or(u64::MAX, |work| work.steps(len).saturating_mul(STEP));
    let out_start = out.len();
    let mut tokenizer = Tokenizer {
        grammar,
        arena,
        limits,
        report,
        out,
        s: &mut own,
        len,
        last: line.start,
        steps,
        out_start,
        stopped: false,
        cells,
        hand: Some(Hand {
            whole: &mut *whole,
            line,
            first_token,
        }),
        nested: 0,
    };
    let halted = match tokenizer.tokenize_string(
        len + 1,
        if first {
            DocStart::Open
        } else {
            DocStart::Passed
        },
        line.start,
        &mut frames,
        Entry::Line {
            anchored: line.embedded,
        },
        Handing { halts },
    ) {
        Stop::Halted(at) => Some(at),
        Stop::End => None,
    };
    if halted.is_none() {
        let top = frames.last().map_or(ROOT, |f| f.content);
        tokenizer.produce(top, len);
    }
    let next = tokenizer.intern::<Handing<'static>>(&mut frames);
    tokenizer.s.spent = steps.saturating_sub(tokenizer.steps);
    tokenizer.s.frames = frames;
    std::mem::swap(&mut whole.textmate, &mut own);
    whole.textmate.spare = Some(own);
    (next, halted)
}

/// What a line of a grammar that hands off lends a language lexed by hand:
/// the whole scratch, whose engine part is the spare, the line, and where
/// the line's tokens start in `out`.
///
/// `first_token` is the caller's [`Lexing::first_token`].
/// This line's own tokens may start later, after the tokens of a host that
/// embeds the grammar.
/// A language that cuts tokens back reads from `first_token`, so it sees
/// the host's tokens of the same line.
struct Hand<'a> {
    whole: &'a mut crate::scratch::Scratch,
    line: Line<'a>,
    first_token: usize,
}

/// Loads the frames of `state`, outermost first; true when it is the start of
/// the document. Only a grammar that hands off has hand-off frames to load.
/// Writes the places of the open `while` rules into `whiles`.
///
/// Out of line: inlined into `textmate::lex`, its frame walk cost Nix about
/// 2% more than the call, measured at one codegen unit.
#[inline(never)]
fn rebuild<H: Halting>(
    grammar: &Grammar,
    arena: &Arena,
    state: u32,
    frames: &mut Vec<Frame>,
    whiles: &mut Vec<usize>,
) -> bool {
    frames.clear();
    let mut at = state;
    while at != ROOT {
        let f = arena.frame(at);
        if H::HANDS_OFF && f.kind == HAND_OFF_KIND {
            frames.push(Frame {
                rule: HANDED,
                state: at,
                enter: usize::MAX,
                anchor: usize::MAX,
                eol: false,
                end: f.payload,
                name: f.stack,
                content: f.stack,
            });
            at = f.parent;
            continue;
        }
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
    whiles.clear();
    whiles.extend(
        frames
            .iter()
            .enumerate()
            .filter(|&(_, frame)| opens_while(grammar, frame))
            .map(|(at, _)| at),
    );
    first
}

/// Whether `frame` is an open `while` rule.
fn opens_while(grammar: &Grammar, frame: &Frame) -> bool {
    grammar
        .rules
        .get(frame.rule)
        .is_some_and(|rule| rule.kind == RuleKind::BeginWhile)
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
/// A form whose group the pattern does not have is no such form.
/// It stays as written, as in vscode-textmate.
/// Objective-C++'s `keyword.control.namespace.$2.objcpp` names a group 2 that
/// its end pattern lacks.
/// A group that took no part in the match writes nothing.
///
/// Leading dots of a group's text are dropped, since they would make an invalid
/// scope name. Case is changed a character at a time, which differs from
/// Shiki's only for a final capital sigma.
fn dollar(rest: &str, text: &[u8], caps: &Caps, out: &mut String) -> Option<usize> {
    let group = |digits: &str| {
        let slot = caps.get(digits.parse::<usize>().ok()?)?;
        Some(slot.and_then(|(s, e)| text.get(s..e).map(String::from_utf8_lossy)))
    };
    if let Some(body) = rest.strip_prefix('{') {
        let close = body.find('}')?;
        let (digits, case) = body[..close].split_once(":/")?;
        let found = group(digits)?;
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
    let found = group(&rest[..digits])?;
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
    /// The grammar's [`Grammar::cells`] for the line's kind, read once per
    /// line.
    cells: &'static Cells,
    /// What a hand-off lends its language, on a line of a grammar that hands
    /// off.
    hand: Option<Hand<'a>>,
    /// How many capture groups are running their rules inside one another,
    /// bounded by [`MAX_NESTED_CAPTURES`].
    nested: usize,
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
        if let Some(Some(scope)) = self.s.current.scopes.get(index) {
            return Some(*scope);
        }
        let scope = self.arena.named_scope(self.grammar.names.get(index)?);
        if scope.is_none() {
            self.report.record(Limit::Nodes);
        }
        if let Some(slot) = self.s.current.scopes.get_mut(index) {
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
    ///
    /// A filled name splits into scopes at spaces only, as vscode-textmate's
    /// `split(/ /g)` does.
    /// A tab that a capture brings in stays inside its scope.
    /// The shell grammar's quoted `<<-` heredoc end names its tab-indented
    /// delimiter.
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
            for piece in filled.split(' ').filter(|piece| !piece.is_empty()) {
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
            && let Some(&(epoch, start)) = self.s.current.found.get(index)
            && epoch == self.s.current.epoch
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
            && let Some(entry) = self.s.current.found.get_mut(index)
        {
            let stored = match start {
                None => Some(NO_MATCH),
                Some(at) => u32::try_from(at).ok(),
            };
            if let Some(stored) = stored {
                *entry = (self.s.current.epoch, stored);
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
    #[expect(
        clippy::inline_always,
        reason = "measured at one codegen unit: with `line_until` a second caller, it stopped being inlined, at a call once a step"
    )]
    #[inline(always)]
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
    ///
    /// Never inlined: measured at one codegen unit, inlining its map lookup
    /// into `tokenize_string` cost that loop registers on every step.
    #[inline(never)]
    fn end_pat(&mut self, pattern: usize, end: u32) -> Pat {
        if end == 0 {
            return Pat::Table(pattern);
        }
        if let Some(&index) = self.s.dynamic_ids.get(&end) {
            return Pat::Built(index);
        }
        Pat::Built(self.build_end(end))
    }

    /// Compiles the `end` or `while` pattern built from a match, keeps it,
    /// and gives its index. Out of line, because a line compiles one rarely
    /// and looks one up often, and the compile's registers would otherwise
    /// widen every lookup's call.
    #[cold]
    #[inline(never)]
    fn build_end(&mut self, end: u32) -> usize {
        let regex = Regex::for_grammar(self.arena.string(end), LineKind::Full).ok();
        self.s.dynamic.push(regex);
        let index = self.s.dynamic.len() - 1;
        self.s.dynamic_ids.insert(end, index);
        index
    }

    /// Offers the rules of the injections that apply to the scopes of `stack`
    /// to `best`. One that starts earlier than the rules of the open rule
    /// wins, and so does one with `L:` that starts where they do; the others
    /// lose a tie.
    #[expect(
        clippy::inline_always,
        reason = "measured at one codegen unit: with `line_until` a second caller, it stopped being inlined, at a call once a step"
    )]
    #[inline(always)]
    fn scan_injections(
        &mut self,
        stack: u32,
        cursor: (usize, usize, usize, DocStart),
        best: &mut Option<Best>,
    ) {
        let mut hits = std::mem::take(&mut self.s.hits);
        hits.clear();
        if let Some(known) = self.s.current.injected_at.get(&stack) {
            hits.extend_from_slice(known);
        } else {
            let mut names: Vec<&str> = self.arena.scopes(stack).collect();
            names.reverse();
            hits.extend(
                self.s
                    .current
                    .injected
                    .iter()
                    .enumerate()
                    .filter(|(_, entry)| entry.expr.matches(&names))
                    .map(|(index, _)| index),
            );
            self.s.current.injected_at.insert(stack, hits.clone());
        }
        for &index in &hits {
            if self.stopped {
                break;
            }
            let entry = &self.s.current.injected[index];
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
            && let Some(&(epoch, start, index)) = self.s.current.list_found.get(list_id)
            && epoch == self.s.current.epoch
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
            && let Some(entry) = self.s.current.list_found.get_mut(list_id)
        {
            let stored = match found {
                None => Some((NO_MATCH, 0)),
                Some((start, index)) => u32::try_from(start).ok().zip(u32::try_from(index).ok()),
            };
            if let Some((start, index)) = stored {
                *entry = (self.s.current.epoch, start, index);
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
        let slot = self.s.current.meta_caches.get_mut(list_id)?;
        if slot.is_none() {
            *slot = Some(scanner.take_cache()?);
        }
        let mut at = from;
        let mut found = None;
        while at < end && !self.stopped {
            let (Some(text), Some(Some(cache))) = (
                self.s.text.get(..end),
                self.s.current.meta_caches.get_mut(list_id),
            ) else {
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

    /// Returns how the run stopped. It stops at `end` or where `halt` says
    /// the embedding language takes the line back.
    fn tokenize_string<H: Halting>(
        &mut self,
        end: usize,
        first_line: DocStart,
        start: usize,
        frames: &mut Vec<Frame>,
        entry: Entry,
        halt: H,
    ) -> Stop {
        let mut pos = start;
        let mut anchor = usize::MAX;
        let mut allow_a = first_line;
        if let Entry::Line { anchored } = entry {
            if anchored {
                anchor = start;
            } else if frames.last().is_some_and(|f| f.eol) {
                anchor = 0;
            }
            let mut next = 0;
            while let Some(&i) = self.s.whiles.get(next) {
                let Some(&frame) = frames.get(i) else {
                    break;
                };
                let rule = &self.grammar.rules[frame.rule];
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
                    next += 1;
                } else {
                    frames.truncate(i);
                    break;
                }
                if self.stopped {
                    return Stop::End;
                }
            }
            if self.stopped {
                return Stop::End;
            }
        }
        while !self.stopped {
            match self.scan_next(end, &mut pos, &mut anchor, &mut allow_a, frames, halt) {
                Flow::Next => {}
                Flow::Done => break,
                Flow::Halted(at) => return Stop::Halted(at),
            }
        }
        Stop::End
    }

    /// One step of the loop. The match that starts first wins, and a match of
    /// the `end` pattern closes the open rule. With only the root rule open,
    /// the line halts first where `halt` says the embedding language takes
    /// it back, before the root's next match.
    ///
    /// A match kept from an earlier search holds no groups, so it is found
    /// again from where it starts.
    #[expect(
        clippy::inline_always,
        reason = "measured at one codegen unit: with `line_until` a second caller, it stopped being inlined, at a call once a step"
    )]
    #[inline(always)]
    fn scan_next<H: Halting>(
        &mut self,
        end: usize,
        pos: &mut usize,
        anchor: &mut usize,
        allow_a: &mut DocStart,
        frames: &mut Vec<Frame>,
        halt: H,
    ) -> Flow {
        let Some(&top) = frames.last() else {
            return Flow::Done;
        };
        let handing = if H::HANDS_OFF {
            self.region(*pos, frames)
        } else {
            None
        };
        let rule: &'static Rule = &self.grammar.rules[handing.map_or(top.rule, |part| part.rule)];
        let mut best: Option<Best> = None;
        let end_pat = (rule.kind == RuleKind::BeginEnd && handing.is_none_or(|part| part.with_end))
            .then(|| self.end_pat(rule.end, handing.map_or(top.end, |part| part.end)));
        if !rule.apply_end_last
            && let Some(pat) = end_pat
        {
            let probed = self.probe(pat, *pos, end, *anchor, *allow_a);
            self.consider(probed, None, (pat, 1, 0), &mut best);
        }
        let list = handing.map_or(rule.list, |part| part.list);
        self.scan_list(list, (*pos, end, *anchor, *allow_a), (1, 0), &mut best);
        if rule.apply_end_last
            && !self.stopped
            && let Some(pat) = end_pat
        {
            let probed = self.probe(pat, *pos, end, *anchor, *allow_a);
            self.consider(probed, None, (pat, 1, usize::MAX), &mut best);
        }
        if !self.s.current.injected.is_empty() && !self.stopped {
            self.scan_injections(top.content, (*pos, end, *anchor, *allow_a), &mut best);
        }
        if self.stopped {
            return Flow::Done;
        }
        if H::HALTS && frames.len() == 1 {
            let to = best.map_or(self.len, |b| b.start.min(self.len));
            let text = self.s.text.get(..self.len).unwrap_or_default();
            let (halted, passed) = halt.halt(text, *pos, to, self.len);
            self.charge(u64::try_from(passed).unwrap_or(u64::MAX));
            if let Some(at) = halted {
                self.produce(top.content, at);
                return Flow::Halted(at);
            }
        }
        if H::HANDS_OFF
            && let Some(part) = handing
            && best.is_none_or(|b| b.start != *pos)
        {
            return self.hand_off_at(part, end, (pos, anchor, allow_a), top.content, frames);
        }
        let Some(best) = best else {
            self.produce(top.content, end);
            return Flow::Done;
        };
        if H::HANDS_OFF && handing.is_some() && best.rule.is_none() {
            frames.pop();
        }
        let mut caps = self.s.caps.pop().unwrap_or_default();
        if best.groups_kept {
            caps.clear();
            caps.extend_from_slice(&self.s.best);
        } else {
            if !self.search(best.pat, best.start, end, *anchor, *allow_a) {
                self.s.caps.push(caps);
                self.produce(top.content, end);
                return Flow::Done;
            }
            read_caps(&self.s.cur, &mut caps);
        }
        let finished = self.apply(best.rule, &caps, rule, end, (pos, anchor, allow_a), frames);
        self.s.caps.push(caps);
        if finished { Flow::Done } else { Flow::Next }
    }

    /// The part of a hand-off a step scans, or `None` when the open rule hands
    /// nothing off. The first step of a region pushes the hand-off frame. It
    /// holds the state of the language lexed by hand. Design section 14.4 of
    /// `openspec/changes/engine-handoff` has the steps.
    #[cold]
    #[inline(never)]
    fn region(&mut self, pos: usize, frames: &mut Vec<Frame>) -> Option<HandPart> {
        let top = *frames.last()?;
        let (embedding, hand_off) = if top.rule == HANDED {
            let embedding = *frames.get(frames.len().checked_sub(2)?)?;
            (embedding, self.hand_off_of(embedding.rule)?)
        } else {
            let hand_off = self.hand_off_of(top.rule)?;
            let state = hand_off.language.start_embedded(self.arena, top.content);
            frames.push(Frame {
                rule: HANDED,
                state: NEW,
                enter: pos,
                anchor: usize::MAX,
                eol: false,
                end: state,
                name: top.content,
                content: top.content,
            });
            (top, hand_off)
        };
        let apply_end_last = self.grammar.rules[embedding.rule].apply_end_last;
        let part = std::mem::take(&mut self.s.part);
        let after = part == Part::After;
        Some(HandPart {
            rule: embedding.rule,
            list: if after {
                hand_off.after
            } else {
                hand_off.before
            },
            end: embedding.end,
            with_end: apply_end_last == after,
            before: hand_off.before,
            after: hand_off.after,
            from: match part {
                Part::Fresh => pos,
                Part::Before | Part::After => pos + self.char_len_at(pos),
            },
        })
    }

    /// Hands the line to a hand-off's language at the cursor. It finds where
    /// the hand-off's rules and end start on the rest of the run. The
    /// language halts at those places, and the step goes on there.
    #[cold]
    #[inline(never)]
    fn hand_off_at(
        &mut self,
        part: HandPart,
        end: usize,
        (pos, anchor, allow_a): (&mut usize, &mut usize, &mut DocStart),
        content: u32,
        frames: &mut [Frame],
    ) -> Flow {
        let rule: &'static Rule = &self.grammar.rules[part.rule];
        let end_pat = (rule.kind == RuleKind::BeginEnd).then(|| self.end_pat(rule.end, part.end));
        let before_end = end_pat.filter(|_| !rule.apply_end_last);
        let after_end = end_pat.filter(|_| rule.apply_end_last);
        let mut before = std::mem::take(&mut self.s.starts_before);
        let mut after = std::mem::take(&mut self.s.starts_after);
        let at = (part.from, end, *anchor, *allow_a);
        self.collect_starts(&mut before, part.before, before_end, at);
        self.collect_starts(&mut after, part.after, after_end, at);
        self.s.starts_before = before;
        self.s.starts_after = after;
        self.s.current.next_epoch();
        if self.stopped {
            return Flow::Done;
        }
        self.produce(content, *pos);
        let from = Resume {
            at: *pos,
            anchor: *anchor,
            allow_a: *allow_a,
            part: Part::Fresh,
        };
        let Some(next) = self.run_language(frames, from, end) else {
            return Flow::Done;
        };
        *pos = next.at;
        *anchor = next.anchor;
        *allow_a = next.allow_a;
        self.s.part = next.part;
        Flow::Next
    }

    /// Lexes the line from `from` with the language of the hand-off on top
    /// of `frames`. The language halts at the starts the step found. Its
    /// state goes into the hand-off frame. Returns where the engine goes on,
    /// or `None` when the language ran to `end`.
    ///
    /// The language sees the line cut at `end`, the end of the run. A capture
    /// ends its run at the capture's end. Shiki also lexes a capture's text
    /// as a line cut there.
    ///
    /// The cursor can sit past the cut, on the line's newline. A heredoc's
    /// opener `<<<'JSON'(\s*)$` consumes that newline. The language then
    /// starts at the cut and lexes nothing of this line.
    ///
    /// The language may run the engine again, as Markdown does for a fence.
    /// That engine line uses the spare scratch, which sits in the whole
    /// scratch while the line runs.
    #[cold]
    #[inline(never)]
    fn run_language(&mut self, frames: &mut [Frame], from: Resume, end: usize) -> Option<Resume> {
        let depth = frames.len();
        let top = depth.checked_sub(1)?;
        let embedding = frames.get(depth.checked_sub(2)?)?.rule;
        let language = self.hand_off_of(embedding)?.language;
        let hand = self.hand.as_mut()?;
        let first_token = hand.first_token;
        let cut = end.min(hand.line.text.len());
        let text = hand.line.text.get(..cut)?;
        let before = std::mem::take(&mut self.s.starts_before);
        let after = std::mem::take(&mut self.s.starts_after);
        let lent = Limits {
            work: self.limits.work.map(|_| Work {
                per_byte: 0,
                base: u32::try_from(self.steps / STEP).unwrap_or(u32::MAX),
            }),
            ..*self.limits
        };
        let part = Line {
            text,
            start: from.at.min(cut),
            embedded: false,
        };
        let state = frames[top].end;
        let (finished, halted) = {
            let mut cx = Lexing {
                scratch: &mut *hand.whole,
                arena: &mut *self.arena,
                limits: &lent,
                out: &mut *self.out,
                report: &mut *self.report,
                first_token,
            };
            let halting = if before.is_empty() && after.is_empty() {
                None
            } else {
                language.lex_until(
                    &mut cx,
                    part,
                    state,
                    &Starts(&before),
                    Some(&Starts(&after)),
                )
            };
            halting.unwrap_or_else(|| (language.lex_embedded(&mut cx, part, state), None))
        };
        let halted = match halted {
            Some(Stopped {
                at,
                kind: StopKind::Halt,
            }) => Some(at),
            Some(Stopped {
                kind: StopKind::PopHost,
                ..
            })
            | None => None,
        };
        let counter = Steps::new(&lent, text.len());
        frames[top].end = counter.take_back(finished);
        frames[top].state = NEW;
        self.steps = self
            .steps
            .saturating_sub(counter.into_spent().saturating_mul(STEP));
        if self.steps == 0 && !self.stopped {
            self.stopped = true;
            self.report.record(Limit::Work);
        }
        self.last = halted.unwrap_or(cut);
        let next = halted.map(|at| Resume {
            at,
            anchor: from.anchor,
            allow_a: if at > from.at {
                DocStart::Passed
            } else {
                from.allow_a
            },
            part: if before.binary_search(&at).is_ok() {
                Part::Before
            } else {
                Part::After
            },
        });
        self.s.starts_before = before;
        self.s.starts_after = after;
        next
    }

    /// The hand-off of the list of `rule`, if it has one.
    fn hand_off_of(&self, rule: usize) -> Option<&'static HandOff> {
        self.grammar.hand_off(self.grammar.rules.get(rule)?.list)
    }

    /// The first match from the cursor among the rules of `list` and the
    /// given end. It tries them in the order a step does.
    fn first_start(
        &mut self,
        list: usize,
        end_pat: Option<Pat>,
        (pos, end, anchor, allow_a): (usize, usize, usize, DocStart),
    ) -> Option<Best> {
        let mut best: Option<Best> = None;
        if let Some(pat) = end_pat {
            let probed = self.probe(pat, pos, end, anchor, allow_a);
            self.consider(probed, None, (pat, 1, 0), &mut best);
        }
        self.scan_list(list, (pos, end, anchor, allow_a), (1, 0), &mut best);
        best
    }

    /// Writes into `into` where the rules of `list` and the given end start
    /// on the rest of the line from `from`. It writes the first start, then
    /// the first start past each one. The generator refuses `\G` in these
    /// rules. A search from each place then finds what a search from the
    /// region's start would.
    fn collect_starts(
        &mut self,
        into: &mut Vec<usize>,
        list: usize,
        end_pat: Option<Pat>,
        (from, end, anchor, allow_a): (usize, usize, usize, DocStart),
    ) {
        into.clear();
        let mut at = from;
        while at <= self.len && !self.stopped {
            let Some(best) = self.first_start(list, end_pat, (at, end, anchor, allow_a)) else {
                break;
            };
            into.push(best.start);
            at = best.start + self.char_len_at(best.start);
        }
    }

    /// The length of the character at `at` in the line, or 1 past its end.
    fn char_len_at(&self, at: usize) -> usize {
        self.s
            .text
            .get(at)
            .map_or(1, |&byte| crate::utf8::char_len(byte))
    }

    /// Applies the match `caps` describes, of the rule `matched` (`None` for
    /// the `end` of `rule`); true when the line is done.
    #[expect(
        clippy::inline_always,
        reason = "measured at one codegen unit: with `line_until` a second caller, it stopped being inlined, at a call once a step"
    )]
    #[inline(always)]
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
    #[expect(
        clippy::inline_always,
        reason = "measured at one codegen unit: with `line_until` a second caller, it stopped being inlined, at a call once a step"
    )]
    #[inline(always)]
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
    /// can do the same.
    /// The depth limit and [`MAX_NESTED_CAPTURES`] bound how deep it goes.
    /// Past either, the group's text gets the group's scopes only.
    ///
    /// The frame goes on top of `frames`, and `frames` get back their length
    /// after the group. A capture's rule has no end, so nothing below it is
    /// closed.
    ///
    /// A capture runs the handing loop only when its rule can reach a
    /// hand-off, as [`Grammar::handing_captures`] lists. Every other capture
    /// runs the plain loop, which costs less per step.
    fn handle_captures(
        &mut self,
        captures: &'static [Cap],
        caps: &Caps,
        frames: &mut Vec<Frame>,
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
                if self.limits.depth_reached(frames.len()) || self.nested >= MAX_NESTED_CAPTURES {
                    self.report.record(Limit::Depth);
                    self.produce(named, e);
                    continue;
                }
                let content = self.push_names(named, capture.content_name, caps);
                let depth = frames.len();
                self.nested += 1;
                frames.push(Frame {
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
                if self.hand.is_some()
                    && self
                        .grammar
                        .handing_captures
                        .binary_search(&capture.retokenize)
                        .is_ok()
                {
                    self.tokenize_string(
                        e.min(end),
                        first,
                        s,
                        frames,
                        Entry::Capture,
                        Handing { halts: None },
                    );
                    self.s.part = Part::Fresh;
                } else {
                    self.tokenize_string(e.min(end), first, s, frames, Entry::Capture, Whole);
                }
                self.nested -= 1;
                frames.truncate(depth);
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

    /// Gives the frames their arena states and returns the innermost. Only a
    /// grammar that hands off has hand-off frames to store.
    ///
    /// The frames a line opened sit on top, above every frame it kept, so
    /// the walk starts at the lowest of them. It leaves each frame as
    /// [`rebuild`] would load it, and keeps the frames for the next line.
    fn intern<H: Halting>(&mut self, frames: &mut [Frame]) -> u32 {
        let low = frames
            .iter()
            .rposition(|frame| frame.state != NEW)
            .map_or(0, |at| at + 1);
        let whiles = &mut self.s.whiles;
        whiles.truncate(whiles.partition_point(|&at| at < low));
        let mut parent = low
            .checked_sub(1)
            .and_then(|at| frames.get(at))
            .map_or(ROOT, |frame| frame.state);
        let mut kept = true;
        for (at, frame) in frames.iter_mut().enumerate().skip(low) {
            let (kind, payload) = if H::HANDS_OFF && frame.rule == HANDED {
                (HAND_OFF_KIND, frame.end)
            } else {
                (
                    KIND,
                    pack(frame.rule, frame.end) | if frame.eol { EOL } else { 0 },
                )
            };
            frame.state = self
                .arena
                .push(parent, kind, payload, frame.content)
                .unwrap_or_else(|| {
                    self.report.record(Limit::Nodes);
                    kept = false;
                    ROOT
                });
            frame.enter = usize::MAX;
            frame.anchor = usize::MAX;
            if let Some(rule) = self.grammar.rules.get(frame.rule) {
                frame.name = frame.content;
                for _ in 0..rule.content_name.len() {
                    frame.name = self.arena.parent(frame.name);
                }
                if rule.kind == RuleKind::BeginWhile {
                    self.s.whiles.push(at);
                }
            }
            parent = frame.state;
        }
        self.s.kept = kept.then_some(Kept {
            grammar: self.grammar,
            state: parent,
        });
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
