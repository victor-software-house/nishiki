//! The per-line engine: it runs the grammar's rules the way a `TextMate` engine
//! does, with hand-written matchers instead of regular expressions.
//!
//! At each position the engine tries the end pattern of the innermost rule and
//! then the rule's patterns in order, and takes the first that matches at the
//! earliest position. A begin match opens a rule, an end match closes it, and
//! a capture may be lexed again with its own pattern list. The corner cases of
//! `TextMate` engines are kept because Shiki's output depends on them: `\G`
//! matches only where the enclosing begin ended, and a rule that opens or
//! closes without advancing stops the line.

use std::cell::Cell;

use super::rules::{Act, Pat, rule_info};
use super::scope::S;
use super::state::Rule;
use crate::arena::{Arena, ROOT};
use crate::language::{Halt, Parts};
use crate::limits::{Finished, Limit, Limits, LineReport, Steps};
use crate::utf8::char_len;
use crate::{Stack, Token};

/// A scope whose name ends in text of the line, such as the digits of `2>`.
#[derive(Clone, Copy, Debug)]
pub(crate) struct DynName {
    pub(crate) prefix: &'static str,
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) suffix: &'static str,
}

/// A run of a match that carries scopes of its own, or is lexed again.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Span {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) scopes: &'static [S],
    pub(crate) dynamic: Option<DynName>,
    pub(crate) retok: Option<Rule>,
}

/// An open rule.
#[derive(Clone, Copy, Debug)]
struct Frame {
    rule: Rule,
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
    /// What the rule carries: a here-document's delimiter, as a string
    /// number.
    payload: u32,
}

/// The answer to a search that scanned ahead for a position, kept so that the
/// next search of the same text, from a later position, does not scan again.
#[derive(Clone, Copy, Debug)]
struct Find {
    /// Where the search started.
    from: usize,
    /// The text the search ran over: its end and whether a line feed follows.
    lim: usize,
    nl: bool,
    answer: Option<usize>,
    valid: bool,
}

/// The searches with a lazy `.+?` that scan ahead, each answered from the
/// last search of its kind; see [`Lexer::find_from`].
#[derive(Clone, Copy, Debug)]
pub(crate) enum Lazy {
    /// The word of a `case`, up to `in`.
    CaseWord,
    /// The variable of a `for`, up to white space and `in`.
    ForVariable,
    /// The index of an array element, up to `]=`.
    ArrayIndex,
}

/// The last search of each [`Lazy`] kind.
#[derive(Debug)]
struct Finds {
    case_word: Cell<Find>,
    for_variable: Cell<Find>,
    array_index: Cell<Find>,
}

impl Finds {
    fn new() -> Self {
        Finds {
            case_word: Cell::new(Find::NONE),
            for_variable: Cell::new(Find::NONE),
            array_index: Cell::new(Find::NONE),
        }
    }

    fn of(&self, lazy: Lazy) -> &Cell<Find> {
        match lazy {
            Lazy::CaseWord => &self.case_word,
            Lazy::ForVariable => &self.for_variable,
            Lazy::ArrayIndex => &self.array_index,
        }
    }
}

impl Find {
    const NONE: Find = Find {
        from: 0,
        lim: 0,
        nl: false,
        answer: None,
        valid: false,
    };
}

/// A run of function-name characters: where it starts and ends, and the end
/// of the text it was found in.
#[derive(Clone, Copy, Debug)]
pub(crate) struct NameRun {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) lim: usize,
}

/// An answer of [`Lexer::name_run`]: where it was asked, the end of the
/// text, and the answer.
#[derive(Clone, Copy, Debug)]
pub(crate) struct NameAnswer {
    pub(crate) at: usize,
    pub(crate) lim: usize,
    pub(crate) end: Option<usize>,
}

/// Two things that happen while a line is lexed.
#[derive(Clone, Copy, Debug, Default)]
struct Flags {
    /// A node limit kept something from being made since the flag was last
    /// cleared.
    failed: bool,
    /// A rule closed without advancing, which keeps its begin scopes for the
    /// rest of the line.
    kept: bool,
    /// A node limit denied a rule its own state, so it shares its parent's.
    /// Only then do two open rules share a state at the end of a line.
    /// The frames that [`Lexer::retokenise`] adds are gone by then.
    shared: bool,
}

/// Whether no pattern that lexes an argument's word again can match in `word`:
/// those match `*`, variables (`$`), numbers (digits, `-`) and `true` or
/// `false`.
fn is_plain_word(word: &[u8]) -> bool {
    !word
        .iter()
        .any(|b| b.is_ascii_digit() || matches!(b, b'*' | b'$' | b'-'))
        && !word.windows(4).any(|w| w == b"true")
        && !word.windows(5).any(|w| w == b"false")
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

const PATH_ENTRIES: usize = 512;
const PUSH_ENTRIES: usize = 256;

/// Odd multipliers that spread the parts of a cache key over the slots
/// before they are combined: the high bits of the golden ratio
/// (`0x9e3779b9`) and of a `MurmurHash3` finaliser constant (`0x85ebca6b`),
/// and a small prime. Any odd constants keep the caches correct; these
/// only make collisions rarer.
const MIX_STATE: usize = 0x9e37;
const MIX_SCOPES: usize = 0x85eb;
const MIX_CODE: usize = 31;

/// What a highlighter keeps from line to line, so that lexing a line
/// allocates nothing once it has grown, and the hot lookups of the arena are
/// not repeated. It belongs to one arena: every id it holds names a node
/// that arena made and never removes.
#[derive(Debug)]
pub(crate) struct Scratch {
    frames: Vec<Frame>,
    spans: Vec<Span>,
    chain: Vec<(u32, crate::arena::Frame)>,
    ends: Vec<usize>,
    name: String,
    /// The state the last line ended in, when `frames` still holds its open
    /// rules, which the next line starts from when it continues that state.
    resume: Option<u32>,
    paths: Vec<Option<PathEntry>>,
    pushes: Vec<Option<PushEntry>>,
}

impl Default for Scratch {
    fn default() -> Self {
        Scratch {
            frames: Vec::new(),
            spans: Vec::new(),
            chain: Vec::new(),
            ends: Vec::new(),
            name: String::new(),
            resume: None,
            paths: vec![None; PATH_ENTRIES],
            pushes: vec![None; PUSH_ENTRIES],
        }
    }
}

/// Lexes one line.
pub(crate) struct Lexer<'a> {
    pub(crate) s: &'a [u8],
    /// The text ends here; a retokenised capture ends before the line does.
    pub(crate) lim: usize,
    /// Whether a line feed follows the text, which the regular expressions of
    /// the grammar see at the end of a line but not at the end of a capture.
    pub(crate) nl: bool,
    out: &'a mut Vec<Token>,
    arena: &'a mut Arena,
    limits: &'a Limits,
    report: &'a mut LineReport,
    /// Steps spent on this line, counted in `&self` scans too.
    steps: Steps,
    /// End of the last token.
    last: usize,
    /// Where the current search starts.
    pub(crate) pos: usize,
    anchor: Option<usize>,
    /// Whether `\G` matches at `pos`.
    pub(crate) g: bool,
    /// The open rules, the root first.
    frames: Vec<Frame>,
    spans: Vec<Span>,
    /// The spans of the current match start here; a retokenised capture
    /// matches above them.
    span_base: usize,
    /// The rule a begin pattern chose, for [`Act::BeginChosen`].
    pub(crate) chosen: Rule,
    /// The string number of the delimiter a here-document begin matched.
    payload: u32,
    chain: Vec<(u32, crate::arena::Frame)>,
    /// Where each option of an `alias` statement, or each unit of a `select`
    /// word, ends, for backtracking.
    pub(crate) ends: Vec<usize>,
    /// The text of a scope name made from source text.
    name: String,
    paths: Vec<Option<PathEntry>>,
    pushes: Vec<Option<PushEntry>>,
    flags: Flags,
    /// The lowest index the line pushed a frame at.
    /// The frames below it are ones the line started with.
    /// The line that left them already cleared their `enter` and `anchor`.
    low: usize,
    /// The searches that scan ahead, one per kind of search.
    finds: Finds,
    /// The last run of function-name characters found: where it starts and
    /// ends, and the end of the text. A search from inside it ends where it
    /// does.
    pub(crate) function_name_run: Cell<NameRun>,
    /// The last [`Lexer::name_run`] answer: where it was asked, the end of
    /// the text, and the answer. Several rules ask about the same word.
    pub(crate) names: Cell<NameAnswer>,
    /// Where the language that embeds shell takes the text back, while only
    /// the root is open: before shell's patterns at a place, and after they
    /// fail there (see [`Halt`]).
    halts: Option<(Halt, Option<Halt>)>,
    /// Where the line halted.
    halted: Option<usize>,
}

impl<'a> Lexer<'a> {
    /// A lexer for one line from `state`. Its open rules are the ones the
    /// last line left in `scratch` when that line ended in `state`, and
    /// otherwise the state's chain of frames, rebuilt from its root down.
    #[expect(
        clippy::inline_always,
        reason = "measured at one codegen unit: once `lex_until` became a second caller, rustc stopped inlining it, and shell rose 0.8% (1.0081)"
    )]
    #[inline(always)]
    pub(crate) fn new(
        Parts {
            arena,
            limits,
            out,
            report,
        }: Parts<'a>,
        text: super::Text<'a>,
        state: u32,
        scratch: &mut Scratch,
    ) -> Self {
        let (s, start) = (text.bytes, text.start);
        let anchor = text.anchored.then_some(start);
        let steps = Steps::new(limits, s.len());
        let mut frames = std::mem::take(&mut scratch.frames);
        let resume = scratch.resume == Some(state);
        if !resume {
            frames.clear();
        }
        let mut spans = std::mem::take(&mut scratch.spans);
        spans.clear();
        let mut chain = std::mem::take(&mut scratch.chain);
        let mut lexer = Lexer {
            s,
            lim: s.len(),
            nl: true,
            out,
            arena,
            limits,
            report,
            steps,
            last: start,
            pos: start,
            anchor,
            g: false,
            frames,
            spans,
            span_base: 0,
            chosen: Rule::Root,
            payload: 0,
            chain: Vec::new(),
            ends: std::mem::take(&mut scratch.ends),
            name: std::mem::take(&mut scratch.name),
            paths: std::mem::take(&mut scratch.paths),
            pushes: std::mem::take(&mut scratch.pushes),
            flags: Flags::default(),
            low: usize::MAX,
            finds: Finds::new(),
            function_name_run: Cell::new(NameRun {
                start: 0,
                end: 0,
                lim: 0,
            }),
            names: Cell::new(NameAnswer {
                at: usize::MAX,
                lim: 0,
                end: None,
            }),
            halts: None,
            halted: None,
        };
        if !resume {
            chain.clear();
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
                        anchor: None,
                        name: frame.stack,
                        content: frame.stack,
                        state: id,
                        payload: 0,
                    });
                    continue;
                }
                let rule = Rule::from_code(frame.kind);
                let parent = lexer.top().content;
                let name = lexer.path(parent, rule_info(rule).name);
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
        }
        lexer.chain = chain;
        lexer
    }

    /// Ends the line at the first place where `end` holds before shell's
    /// patterns, or `after` holds once they fail, while only the root is open.
    pub(crate) fn halting(mut self, end: Halt, after: Option<Halt>) -> Self {
        self.halts = Some((end, after));
        self
    }

    /// The halts a scan tests at each place: the line's, while only the root
    /// is open, and `None` otherwise. A scan does not open or close rules,
    /// so it asks once.
    fn root_halts(&self) -> Option<(Halt, Option<Halt>)> {
        self.halts.filter(|_| self.frames.len() == 1)
    }

    /// Lexes the line, returns the state after it, and gives the buffers back
    /// to `scratch`.
    pub(crate) fn run(self, scratch: &mut Scratch) -> Finished {
        self.run_halting(scratch).0
    }

    /// Lexes the line like [`Lexer::run`], and returns where it halted too.
    ///
    /// The next line may resume the open rules only when they are the
    /// state's chain: each a state of its own, which a node limit can deny,
    /// and none that kept its begin scopes.
    #[expect(
        clippy::inline_always,
        reason = "measured at one codegen unit: once `lex_until` became a second caller, rustc stopped inlining it, and shell rose 0.8% (1.0081)"
    )]
    #[inline(always)]
    pub(crate) fn run_halting(mut self, scratch: &mut Scratch) -> (Finished, Option<usize>) {
        self.tokenize();
        let state = self.top().state;
        let exact = !self.flags.kept && !self.flags.shared;
        scratch.resume = exact.then_some(state);
        if exact {
            let low = self.low.min(self.frames.len());
            for frame in &mut self.frames[low..] {
                (frame.enter, frame.anchor) = (None, None);
            }
        }
        scratch.frames = self.frames;
        scratch.spans = self.spans;
        scratch.chain = self.chain;
        scratch.ends = self.ends;
        scratch.name = self.name;
        scratch.paths = self.paths;
        scratch.pushes = self.pushes;
        (self.steps.finish(state), self.halted)
    }

    // Bytes.

    /// Whether a character stands at `i`: the text, or its line feed.
    pub(crate) fn avail(&self, i: usize) -> bool {
        i < self.lim || (i == self.lim && self.nl)
    }

    /// The byte at `i`, with the line feed at the end of a line, or 0 where no
    /// character stands.
    pub(crate) fn at(&self, i: usize) -> u8 {
        if i < self.lim {
            self.s[i]
        } else if i == self.lim && self.nl {
            b'\n'
        } else {
            0
        }
    }

    /// The byte before `i`, or `None` at the start.
    pub(crate) fn prev(&self, i: usize) -> Option<u8> {
        if i == 0 { None } else { Some(self.at(i - 1)) }
    }

    /// Whether `\G` matches at `p`.
    pub(crate) fn at_g(&self, p: usize) -> bool {
        self.g && p == self.pos
    }

    /// The regex `$` at `i`: before the line feed or at the end of the text.
    pub(crate) fn at_eol(&self, i: usize) -> bool {
        i == self.lim || (self.nl && i == self.lim + 1)
    }

    /// Whether the text at `i` starts with `text`. The first byte decides
    /// almost always, so it is compared before the rest.
    pub(crate) fn starts(&self, i: usize, text: &[u8]) -> bool {
        text.first().is_some_and(|b| self.s.get(i) == Some(b))
            && self
                .s
                .get(i..self.lim)
                .is_some_and(|rest| rest.starts_with(text))
    }

    pub(crate) fn skip(&self, mut i: usize, class: impl Fn(u8) -> bool) -> usize {
        let from = i;
        while i < self.lim && class(self.s[i]) {
            i += 1;
        }
        self.spend(i - from);
        i
    }

    /// The end of the blanks from `i`. Most positions are not blank, and a
    /// matcher asks at every candidate position, so that case returns first.
    #[inline]
    pub(crate) fn blanks(&self, i: usize) -> usize {
        if i < self.lim && matches!(self.s[i], b' ' | b'\t') {
            self.skip(i, crate::shell::matchers::is_blank)
        } else {
            i
        }
    }

    /// Counts `steps` of work against the line's budget.
    pub(crate) fn spend(&self, steps: usize) {
        self.steps.spend(steps);
    }

    /// The first position from `from` to the end of the text where `test`
    /// holds, stepping a character at a time.
    ///
    /// A pattern with a lazy `.+?` scans ahead like this, and a line made of
    /// such tokens would scan to its end for each. `lazy` names the test: the
    /// answer of the last search of that kind answers a later one that starts
    /// no earlier and no later than the answer.
    pub(crate) fn find_from(
        &self,
        lazy: Lazy,
        from: usize,
        test: impl Fn(&Self, usize) -> bool,
    ) -> Option<usize> {
        let known = self.finds.of(lazy).get();
        if known.valid
            && known.lim == self.lim
            && known.nl == self.nl
            && known.from <= from
            && known.answer.is_none_or(|answer| answer >= from)
        {
            return known.answer;
        }
        let mut e = from;
        let answer = loop {
            self.spend(1);
            if test(self, e) {
                break Some(e);
            }
            if e >= self.lim {
                break None;
            }
            e = self.char_end(e);
        };
        self.finds.of(lazy).set(Find {
            from,
            lim: self.lim,
            nl: self.nl,
            answer,
            valid: true,
        });
        answer
    }

    /// The end of the character at `i`, which is a whole UTF-8 sequence.
    pub(crate) fn char_end(&self, i: usize) -> usize {
        (i + char_len(self.s[i])).min(self.lim)
    }

    // Spans.

    /// Names `start..end` with `scopes`.
    pub(crate) fn span(&mut self, start: usize, end: usize, scopes: &'static [S]) {
        self.push_span(Span {
            start,
            end,
            scopes,
            dynamic: None,
            retok: None,
        });
    }

    /// Names `start..end` with `scopes` and then one scope that ends in
    /// `dynamic`.
    pub(crate) fn span_dyn(
        &mut self,
        start: usize,
        end: usize,
        scopes: &'static [S],
        dynamic: DynName,
    ) {
        self.push_span(Span {
            start,
            end,
            scopes,
            dynamic: Some(dynamic),
            retok: None,
        });
    }

    /// Names `start..end` with `scopes`, and lexes it again with `rule`.
    pub(crate) fn span_retok(
        &mut self,
        start: usize,
        end: usize,
        scopes: &'static [S],
        rule: Rule,
    ) {
        self.push_span(Span {
            start,
            end,
            scopes,
            dynamic: None,
            retok: Some(rule),
        });
    }

    fn push_span(&mut self, span: Span) {
        self.spans.push(span);
    }

    /// Drops the spans of a match that failed after naming some.
    pub(crate) fn clear_spans(&mut self) {
        self.spans.truncate(self.span_base);
    }

    // Frames.

    fn top(&self) -> &Frame {
        &self.frames[self.frames.len() - 1]
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
    ///
    /// The cache keys a list by its address, which names one list only
    /// because every list is a static of the grammar's tables.
    fn path(&mut self, base: u32, scopes: &'static [S]) -> u32 {
        if scopes.is_empty() {
            return base;
        }
        let ptr = scopes.as_ptr().addr();
        let slot = (usize::try_from(base).unwrap_or(0).wrapping_mul(MIX_STATE) ^ (ptr >> 3))
            % PATH_ENTRIES;
        if let Some(entry) = self.paths[slot]
            && entry.base == base
            && entry.ptr == ptr
            && entry.len == scopes.len()
        {
            return entry.node;
        }
        let mut stack = base;
        let mut whole = true;
        for scope in scopes {
            let Some(child) = self.arena.child(stack, scope.scope()) else {
                self.report.record(Limit::Nodes);
                self.flags.failed = true;
                whole = false;
                break;
            };
            stack = child;
        }
        if whole {
            self.paths[slot] = Some(PathEntry {
                base,
                ptr,
                len: scopes.len(),
                node: stack,
            });
        }
        stack
    }

    /// The stack `base` with the scopes `prefix`, `text` and `suffix` spell
    /// out on top. A name with spaces is several scopes, and leading dots of
    /// the text are dropped, as a `TextMate` engine does for the text of a
    /// capture.
    fn dynamic_path(&mut self, base: u32, prefix: &str, text: &str, suffix: &str) -> u32 {
        self.name.clear();
        self.name.push_str(prefix);
        self.name.push_str(text.trim_start_matches('.'));
        self.name.push_str(suffix);
        self.spelled_path(base)
    }

    /// The stack `base` with the scopes the name in `self.name` spells out
    /// on top, a scope per word.
    fn spelled_path(&mut self, base: u32) -> u32 {
        let name = std::mem::take(&mut self.name);
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
        self.name = name;
        stack
    }

    fn push_frame(
        &mut self,
        rule: Rule,
        enter: Option<usize>,
        anchor: Option<usize>,
        payload: u32,
    ) {
        self.low = self.low.min(self.frames.len());
        let parent_state = self.top().state;
        let parent_scopes = self.top().content;
        let code = rule.code();
        let slot = (usize::try_from(parent_state)
            .unwrap_or(0)
            .wrapping_mul(MIX_STATE)
            ^ usize::try_from(parent_scopes)
                .unwrap_or(0)
                .wrapping_mul(MIX_SCOPES)
            ^ usize::from(code).wrapping_mul(MIX_CODE)
            ^ usize::try_from(payload).unwrap_or(0))
            % PUSH_ENTRIES;
        if let Some(entry) = self.pushes[slot]
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
        let info = rule_info(rule);
        let parent = self.top().content;
        let name = self.path(parent, info.name);
        let content = match info.content_delimiter {
            Some(prefix) => {
                self.name.clear();
                self.name.push_str(prefix);
                self.name
                    .push_str(self.arena.string(payload).trim_start_matches('.'));
                self.spelled_path(name)
            }
            None => self.path(name, info.content),
        };
        let state = self
            .arena
            .push(parent_state, code, payload, content)
            .unwrap_or_else(|| {
                self.report.record(Limit::Nodes);
                self.flags.failed = true;
                self.flags.shared = true;
                parent_state
            });
        if !self.flags.failed {
            self.pushes[slot] = Some(PushEntry {
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

    fn pop_frame(&mut self) -> Frame {
        let frame = self.frames[self.frames.len() - 1];
        self.frames.pop();
        frame
    }

    // Tokens.

    fn emit(&mut self, to: usize, node: u32) {
        let to = to.min(self.lim);
        if to <= self.last {
            return;
        }
        self.out.push(Token {
            end: to,
            stack: Stack(node),
        });
        self.last = to;
    }

    /// Emits the spans of the current match on `base`. An argument whose
    /// text is a plain word is emitted whole: no pattern of the argument's
    /// list can match in it.
    fn emit_spans(&mut self, base: u32) {
        for i in self.span_base..self.spans.len() {
            let span = self.spans[i];
            if span.start == span.end {
                continue;
            }
            self.emit(span.start, base);
            let named = self.path(base, span.scopes);
            let node = match span.dynamic {
                Some(dynamic) => {
                    let s = self.s;
                    let text = String::from_utf8_lossy(&s[dynamic.start..dynamic.end]);
                    self.dynamic_path(named, dynamic.prefix, &text, dynamic.suffix)
                }
                None => named,
            };
            match span.retok {
                Some(Rule::RetokArgument) if is_plain_word(&self.s[span.start..span.end]) => {
                    self.emit(span.end, node);
                }
                Some(rule) => self.retokenise(rule, span.start, span.end, node),
                None => self.emit(span.end, node),
            }
        }
    }

    /// Lexes `start..end` again with the patterns of `rule`, in a stack whose
    /// innermost scopes are `node`. Whatever it opens is dropped at `end`.
    fn retokenise(&mut self, rule: Rule, start: usize, end: usize, node: u32) {
        if self.at_depth_limit() {
            self.report.record(Limit::Depth);
            self.emit(end, node);
            return;
        }
        let saved = (self.lim, self.nl, self.pos, self.anchor, self.frames.len());
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
        self.lim = end;
        self.nl = false;
        self.pos = start;
        self.anchor = None;
        self.tokenize();
        let depth;
        (self.lim, self.nl, self.pos, self.anchor, depth) = saved;
        self.frames.truncate(depth);
        self.spans.truncate(self.span_base);
        self.span_base = saved_base;
    }

    // The loop.

    /// Runs from `pos` until the text ends or the line stops. When the work
    /// runs out, the rest of the line is the innermost rule's.
    fn tokenize(&mut self) {
        let line_length = self.lim + usize::from(self.nl);
        let mut stalled = 0_u32;
        loop {
            self.spend(1);
            if self.steps.exhausted() {
                self.report.record(Limit::Work);
                let node = self.top().content;
                self.emit(line_length, node);
                return;
            }
            self.g = self.anchor == Some(self.pos);
            let Some((start, pat, end)) = self.scan() else {
                let node = self.top().content;
                self.emit(self.halted.unwrap_or(line_length), node);
                return;
            };
            let advanced = end > self.pos;
            match pat.act() {
                Act::End => {
                    if self.end_rule(start, end, advanced, line_length) {
                        return;
                    }
                }
                Act::Begin(rule) => {
                    if self.begin_rule(rule, start, end, advanced, line_length) {
                        return;
                    }
                }
                Act::BeginChosen => {
                    let rule = self.chosen;
                    if self.begin_rule(rule, start, end, advanced, line_length) {
                        return;
                    }
                }
                Act::Match(name) => {
                    if self.match_rule(name, start, end, advanced, line_length) {
                        return;
                    }
                }
            }
            if end > self.pos {
                self.pos = end;
                stalled = 0;
            } else {
                stalled += 1;
                // Each step that does not advance opens or closes a rule at
                // this byte, and a rule that repeats ends the line, so a run
                // is at most one open and one close per rule of the grammar,
                // about 120 steps. 4096 is far above that: reaching it means a
                // rule matches empty without being guarded.
                debug_assert!(
                    stalled < 4096,
                    "the shell lexer made no progress at byte {}",
                    self.pos
                );
            }
        }
    }

    /// Closes the innermost rule. True when the line stops: a rule that
    /// opened and closed without advancing would loop, so it stays open and
    /// the rest of the line is its.
    fn end_rule(&mut self, start: usize, end: usize, advanced: bool, line_length: usize) -> bool {
        let top = *self.top();
        self.emit(start, top.content);
        self.emit_spans(top.name);
        self.emit(end, top.name);
        let popped = self.pop_frame();
        self.anchor = popped.anchor;
        if !advanced && popped.enter == Some(self.pos) {
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

    /// Opens `rule`, with its content scopes after its begin tokens. True
    /// when the line stops.
    fn begin_rule(
        &mut self,
        rule: Rule,
        start: usize,
        end: usize,
        advanced: bool,
        line_length: usize,
    ) -> bool {
        let top_content = self.top().content;
        self.emit(start, top_content);
        let before = self.depth();
        let payload = if rule.is_heredoc() { self.payload } else { 0 };
        let anchor = self.anchor;
        self.push_frame(rule, Some(self.pos), anchor, payload);
        let name = self.top().name;
        let content = self.top().content;
        let top = self.frames.len() - 1;
        self.frames[top].content = name;
        self.emit_spans(name);
        self.emit(end, name);
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

    /// [`Lexer::loops`] for the rule just opened above frame `before`.
    fn loops_below(&self, before: usize, rule: Rule) -> bool {
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

    /// Names the text of a match rule. True when the line stops: an empty
    /// match that is not at an advance would loop, so the enclosing rule is
    /// dropped and the line stops.
    fn match_rule(
        &mut self,
        name: &'static [S],
        start: usize,
        end: usize,
        advanced: bool,
        line_length: usize,
    ) -> bool {
        let top_content = self.top().content;
        self.emit(start, top_content);
        let base = self.path(top_content, name);
        self.emit_spans(base);
        self.emit(end, base);
        if !advanced {
            if self.depth() > 0 {
                self.pop_frame();
            }
            let node = self.top().content;
            self.emit(line_length, node);
            return true;
        }
        false
    }

    /// The first match at the earliest position from `pos`: its start, pattern
    /// and end, with the match's spans in `self.spans`.
    fn scan(&mut self) -> Option<(usize, Pat, usize)> {
        let mut visited = 0;
        let found = match self.root_halts() {
            Some((halt, after)) => self.scan_positions::<true>(&mut visited, halt, after),
            None => self.scan_positions::<false>(&mut visited, |_, _| false, None),
        };
        self.spend(visited);
        found
    }

    /// The positions of [`Lexer::scan`] one by one, counting how many it
    /// visits in `visited` so that the work is spent once, not per position.
    ///
    /// Each position tries only the patterns that can start there, or all
    /// of them where the text ends without a line feed. A begin the depth
    /// limit keeps closed leaves its text plain. A pattern that fails at a
    /// blank fails at the blanks after it, because the blanks are consumed
    /// first and what follows decides; the position after the first blank is
    /// the exception, because `normal_statement` may look back over a
    /// keyword and one space. A match starts at the start of a character.
    ///
    /// When `HALTING`, a language that embeds shell takes the text back where
    /// `halt` holds before the patterns at a place or `after` holds once they
    /// fail there; only the root is open then. Without it the tests compile
    /// out, so shell that nothing embeds pays nothing for them.
    fn scan_positions<const HALTING: bool>(
        &mut self,
        visited: &mut usize,
        halt: Halt,
        after: Option<Halt>,
    ) -> Option<(usize, Pat, usize)> {
        let info = rule_info(self.top().rule);
        let full = self.at_depth_limit();
        let mut p = self.pos;
        while p <= self.lim {
            *visited += 1;
            if HALTING && halt(self.s, p) {
                self.halted = Some(p);
                return None;
            }
            let list: &[Pat] = if p < self.lim || self.nl {
                let byte = if p < self.lim { self.s[p] } else { b'\n' };
                let prev = if p > 0 { self.s[p - 1] } else { 0 };
                info.candidates(p == self.pos, p == 0, prev, byte)
            } else {
                &info.order
            };
            for &pat in list {
                if full && matches!(pat.act(), Act::Begin(_) | Act::BeginChosen) {
                    if self.try_pat(pat, p).is_some() {
                        self.report.record(Limit::Depth);
                    }
                    continue;
                }
                if let Some(end) = self.try_pat(pat, p) {
                    return Some((p, pat, end));
                }
            }
            if HALTING && after.is_some_and(|after| after(self.s, p)) {
                self.halted = Some(p);
                return None;
            }
            if info.skip_blanks
                && p < self.lim
                && matches!(self.s[p], b' ' | b'\t')
                && p > 0
                && matches!(self.s[p - 1], b' ' | b'\t')
            {
                p = self.skip(p, |b| matches!(b, b' ' | b'\t'));
            } else if p < self.lim && self.s[p] >= 0x80 {
                p = self.char_end(p);
            } else {
                p += 1;
            }
        }
        None
    }

    fn try_pat(&mut self, pat: Pat, p: usize) -> Option<usize> {
        self.spans.truncate(self.span_base);
        self.matcher(pat, p)
    }

    /// The text that ends the innermost rule when it is a literal.
    pub(crate) fn top_end_lit(&self) -> (&'static [u8], &'static [S]) {
        let info = rule_info(self.top().rule);
        (info.end_lit, info.end_scopes)
    }

    /// The innermost rule.
    pub(crate) fn top_rule(&self) -> Rule {
        self.top().rule
    }

    /// What the innermost rule carries: a here-document's delimiter, as a
    /// string number.
    pub(crate) fn top_payload(&self) -> u32 {
        self.top().payload
    }

    /// Where the interned string `id` ends when the text at `i` starts with
    /// it.
    pub(crate) fn string_end(&self, i: usize, id: u32) -> Option<usize> {
        let text = self.arena.string(id).as_bytes();
        let end = i.checked_add(text.len())?;
        self.spend(text.len());
        (end <= self.lim && &self.s[i..end] == text).then_some(end)
    }

    /// Interns the text `start..end` as the delimiter of the here-document
    /// that begins; `None` when the node limit keeps it from being interned.
    pub(crate) fn set_delimiter(&mut self, start: usize, end: usize) -> Option<()> {
        let s = self.s;
        let text = std::str::from_utf8(&s[start..end]).ok()?;
        if let Some(id) = self.arena.intern(text) {
            self.payload = id;
            Some(())
        } else {
            self.report.record(Limit::Nodes);
            None
        }
    }
}
