//! The per-line engine: it runs a grammar's rules the way a `TextMate` engine
//! does, with hand-written matchers instead of regular expressions.
//!
//! At each position the engine tries the end pattern of the innermost rule and
//! then the rule's patterns in order.
//! It takes the first that matches at the earliest position.
//! A begin match opens a rule and an end match closes it.
//!
//! A match's capture groups name parts of it, through the pattern's table or
//! through spans that the matcher records.
//! A span may be lexed again with the patterns of another rule.
//!
//! The corner cases of `TextMate` engines are kept because Shiki's output
//! depends on them:
//!
//! - `\G` matches only where the enclosing begin ended.
//! - A rule that opens or closes without advancing stops the line.

#[cfg(feature = "_embedding")]
use super::Embedding;
#[cfg(feature = "_halting-embedding")]
use super::Stop;
use super::{Act, Captures, DynName, Grammar, NameCase, Payload, ScopeName, Span, Text};
use crate::arena::{Arena, ROOT};
use crate::language::{Halt, Halts, Parts, StopReason, Stopped};
use crate::limits::{Finished, Limit, Limits, LineReport, Steps};
use crate::scope::{PATH_SLOTS, Scopes, path_slot};
use crate::{Language, Stack, Token};

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
    /// The index of the next frame above with a `while` pattern.
    /// It is 0 when there is none, and the root's names the first.
    /// Only a grammar with [`Grammar::WHILES`] links them, at the end of a
    /// line. An index fits in a `u32`, because frames outnumber arena states
    /// only by the few kept or shared ones.
    next_while: u32,
}

/// A cached `path`: the stack `node` that a list makes on `base`.
///
/// The slot comes from the list's key ([`Keyed`](crate::scope::Keyed)), not its
/// address.
/// A change to another language's tables then cannot move it.
/// The entry holds the address of the list's key.
/// Every list is a static with its own key, so that address names one list.
/// A thin reference keeps the entry at 16 bytes.
#[derive(Clone, Copy, Debug)]
struct PathEntry {
    base: u32,
    node: u32,
    key: &'static u16,
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

/// The steps at one byte that open a rule, past which the tokenizer asserts
/// it is looping.
///
/// A rule that repeats ends the line.
/// So such a run is at most one open and one close per rule of the grammar.
/// 4096 is far above that, and reaching it means a rule matches empty without
/// being guarded.
///
/// Closes are bounded by the rules open instead, up to the depth limit.
/// A line's end closes every rule whose end is `$`, for one.
/// So the bound adds the depth limit.
const STALLED_OPENS: usize = 4096;

const _: () = assert!(size_of::<Option<PathEntry>>() == 16);
const PUSH_ENTRIES: usize = 256;

/// Two things that happen while a line is lexed.
#[derive(Clone, Copy, Debug, Default)]
struct Flags {
    /// A node limit kept something from being made since the flag was last
    /// cleared.
    failed: bool,
    /// A rule closed without advancing, which keeps its begin scopes for the
    /// rest of the line.
    /// The next line takes the rule over with its content scopes put back.
    kept: bool,
    /// A node limit denied a rule its own state, so it shares its parent's.
    /// Only then do two open rules share a state at the end of a line.
    /// The frames that [`Lexer::retokenise`] adds are gone by then.
    shared: bool,
}

/// What a highlighter keeps from line to line.
///
/// Lexing a line then allocates nothing once it has grown, and the hot
/// lookups of the arena are not repeated.
/// It belongs to one arena: every id it holds names a node that arena made
/// and never removes.
#[derive(Debug)]
pub(crate) struct Scratch<G: Grammar> {
    frames: Vec<Frame<G>>,
    spans: Vec<Span<G>>,
    /// The spans being emitted that later spans of their match lie inside:
    /// where each ends and its stack, the innermost last.
    open_spans: Vec<(usize, u32)>,
    /// The capture groups of the last match, kept clear between lines.
    captures: Vec<Option<(usize, usize)>>,
    /// The capture groups of the matches whose nested groups are being
    /// lexed, one after another, the outermost first.
    saved_captures: Vec<Option<(usize, usize)>>,
    chain: Vec<(u32, crate::arena::Frame)>,
    /// The text of a scope name made from source text.
    name: String,
    /// The state the last line ended in and the open rules of it.
    /// The next line starts from them when it continues that state.
    ///
    /// The two frame buffers trade places at such a line.
    /// So a line that starts afresh takes the larger one, and lexing a
    /// document again allocates nothing.
    resume: Option<u32>,
    resume_frames: Vec<Frame<G>>,
    paths: Vec<Option<PathEntry>>,
    pushes: Vec<Option<PushEntry>>,
    /// What the matchers kept from the last line ([`Grammar::Kept`]).
    kept: G::Kept,
}

/// A line's capture groups that another language lexes again, and the
/// buffer their tokens go through.
/// The lexer notes each group as it emits the group's placeholder token.
/// The engine lexes them where the line hands control back.
/// Only a grammar that defers groups keeps one ([`Grammar::deferred`]).
#[cfg(feature = "_embedding")]
#[derive(Debug, Default)]
pub(crate) struct Deferrals {
    /// The groups, in the order their placeholders went out.
    pub(crate) groups: Vec<Deferred>,
    /// The tokens a group's language emits, before they take its
    /// placeholder's place.
    pub(crate) tokens: Vec<Token>,
}

/// A capture group that `language` lexes again.
/// `token` is the index of its placeholder token in the output.
/// Its text is `from..to`, on the stack `base`.
#[cfg(feature = "_embedding")]
#[derive(Debug)]
pub(crate) struct Deferred {
    pub(crate) language: Language,
    pub(crate) token: usize,
    pub(crate) from: usize,
    pub(crate) to: usize,
    pub(crate) base: u32,
}

impl<G: Grammar> Scratch<G> {
    /// The deferred capture groups the last run left, for a grammar that
    /// keeps them ([`Grammar::deferred_kept`]).
    #[cfg(feature = "_embedding")]
    pub(crate) fn deferred(&mut self) -> Option<&mut Option<Box<Deferrals>>> {
        G::deferred_kept(&mut self.kept)
    }

    /// The buffer for an embedding rule's parent payload text, for a
    /// grammar that keeps one ([`crate::rules::Embeds::parent_text`]).
    #[cfg(feature = "_embedding")]
    pub(crate) fn parent_text(&mut self) -> Option<&mut Vec<u8>>
    where
        G: crate::rules::Embeds,
    {
        G::parent_text(&mut self.kept)
    }

    /// Moves the frames kept for `from` to `to`, a state that differs from it
    /// only in the top frame's payload, which becomes `payload`.
    /// Where the node limit kept `to` out of the arena, `to` is `from` with
    /// its old payload, and the frames stay as they are.
    #[cfg(feature = "_embedding")]
    pub(crate) fn move_resume(&mut self, from: u32, to: u32, payload: u32) {
        if from != to
            && self.resume == Some(from)
            && let Some(top) = self.resume_frames.last_mut()
            && top.state == from
        {
            top.state = to;
            top.payload = payload;
            self.resume = Some(to);
        }
    }

    /// Drops the frames kept for the next line.
    /// That line then opens its rules from its state.
    #[cfg(feature = "_embedding")]
    pub(crate) fn forget_resume(&mut self) {
        self.resume = None;
    }

    /// Sets what the next line's matchers start from ([`Grammar::Kept`]).
    /// Batch tells its matchers there whether a Markdown fence embeds it.
    #[cfg(feature = "lang-bat")]
    pub(crate) fn set_kept(&mut self, kept: G::Kept) {
        self.kept = kept;
    }
}

impl<G: Grammar> Default for Scratch<G> {
    fn default() -> Self {
        Scratch {
            frames: Vec::new(),
            spans: Vec::new(),
            open_spans: Vec::new(),
            captures: Vec::new(),
            saved_captures: Vec::new(),
            chain: Vec::new(),
            name: String::new(),
            resume: None,
            resume_frames: Vec::new(),
            paths: Vec::new(),
            pushes: Vec::new(),
            kept: G::Kept::default(),
        }
    }
}

/// The payload of a frame whose begin match took the line feed.
///
/// `\G` then holds at the start of the next line, as in vscode-textmate.
/// Only a grammar with [`Grammar::CAPTURED_EOL`] keeps it.
const CAPTURED_EOL: u32 = 1;

/// Where the text of a scope name made at run time comes from.
#[derive(Clone, Copy, Debug)]
enum NameSource {
    /// The bytes `start..end` of the line, spelled as `case` says.
    Line {
        start: usize,
        end: usize,
        case: NameCase,
    },
    /// A string the arena interned, such as a rule's payload.
    Interned(u32),
    /// The text of a capture group of the match.
    Group(u8),
}

/// A probe's answer: where the match ends, and its capture groups.
#[cfg(test)]
pub(crate) type Probe = (usize, Vec<Option<(usize, usize)>>);

/// The payload of a root frame that is the start of a document.
///
/// Its first line is the one where `\A` matches.
/// The root of every state after it has no payload.
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
    /// Whether a line feed follows the text.
    /// The regular expressions of the grammar see one at the end of a line but
    /// not at the end of a capture.
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
    /// Whether `\G` matches at the line's start while the root rule alone is
    /// open ([`crate::language::Line::anchored_at_root`]).
    anchored_at_root: bool,
    /// Whether `\G` matches at `pos`.
    pub(crate) g: bool,
    /// Whether this is the first line of the document, where `\A` matches.
    pub(crate) first_line: bool,
    /// The open rules, the root first.
    frames: Vec<Frame<G>>,
    /// The capture groups of the last match, by group number.
    captures: Vec<Option<(usize, usize)>>,
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
    /// The lowest index the line pushed a frame at.
    /// The frames below it are ones the line started with.
    /// The line that left them already cleared their `enter` and `anchor`.
    low: usize,
    /// Whether the `while` links of the frames below `low` hold.
    /// They hold when the line took over the last line's frames.
    /// They also hold once a full `while` check has linked the frames.
    linked: bool,
    /// The state the line starts in.
    state: u32,
    /// What the language's matchers remember about this line.
    pub(crate) x: G::Extra,
    /// The halts of a line that another language embeds (see [`Halts`]).
    /// The field is one reference to the caller's pair.
    /// It is as wide as the function pointer it replaced.
    /// A line that pops the rule that embeds the grammar drops them, since
    /// that rule's end no longer applies ([`StopReason::PopHost`]).
    halt: Option<&'a Halts<'a>>,
    /// Where the line halted.
    halted: Option<usize>,
    /// Whether the line runs through [`Lexer::run_embedding`].
    ///
    /// That function hands an embedding rule's text to the language it embeds.
    /// A line run any other way lexes such a rule's text as the rule's own.
    embedding: bool,
    /// The language the innermost rule embeds, which the loop hands the
    /// text to at the cursor; only an embedding line sets it.
    embeds: Option<Language>,
    /// Whether the cursor is where an embedded language halted, so the
    /// rule's own patterns are tried there before the text goes back to it.
    host: bool,
    /// Where the line stopped for the embedded language, the language, and
    /// whether `\G` holds there.
    yielded: Option<(usize, Language, bool)>,
}

/// Where an embedding line enters the rule engine.
#[cfg(feature = "_embedding")]
#[derive(Clone, Copy, Debug)]
enum Entry {
    /// At the start of the line, where the `while` rules are tried.
    Start,
    /// Where an embedded language halted, where the rule's own patterns are
    /// tried at the cursor before the text goes back to that language.
    Resume,
}

impl<'a, G: Grammar> Lexer<'a, G> {
    #[expect(
        clippy::inline_always,
        reason = "measured at one codegen unit: with `lex_until` a second caller, \
                  it stopped being inlined, \
                  which cost CSS, C and SQL a call and a copy of the lexer once a line"
    )]
    #[inline(always)]
    pub(crate) fn new(
        Parts {
            arena,
            limits,
            out,
            report,
        }: Parts<'a>,
        text: Text<'a>,
        state: u32,
        scratch: &'a mut Scratch<G>,
    ) -> Self {
        let (s, start) = (text.bytes, text.start);
        let steps = Steps::new(limits, s.len());
        let mut frames = std::mem::take(&mut scratch.frames);
        let resumed = scratch.resume == Some(state);
        if resumed {
            std::mem::swap(&mut frames, &mut scratch.resume_frames);
        } else {
            if frames.capacity() < scratch.resume_frames.capacity() {
                std::mem::swap(&mut frames, &mut scratch.resume_frames);
            }
            frames.clear();
        }
        let mut spans = std::mem::take(&mut scratch.spans);
        spans.clear();
        let mut captures = std::mem::take(&mut scratch.captures);
        if captures.is_empty() {
            captures.resize(G::GROUPS, None);
        }
        if scratch.paths.is_empty() {
            scratch.paths.resize(PATH_SLOTS, None);
        }
        if scratch.pushes.is_empty() {
            scratch.pushes.resize(PUSH_ENTRIES, None);
        }
        let x = G::extra(std::mem::take(&mut scratch.kept));
        Lexer {
            s,
            nl: true,
            out,
            arena,
            limits,
            report,
            steps,
            last: start,
            pos: start,
            anchor: text.anchored.then_some(start),
            anchored_at_root: text.anchored_at_root,
            g: false,
            first_line: false,
            frames,
            captures,
            dirty: false,
            spans,
            span_base: 0,
            scratch,
            flags: Flags::default(),
            low: usize::MAX,
            linked: resumed,
            state,
            x,
            halt: None,
            halted: None,
            embedding: false,
            embeds: None,
            host: false,
            yielded: None,
        }
    }

    /// Notes whether the innermost rule embeds another language, for a
    /// grammar that has such rules.
    fn note_embedding(&mut self) {
        if G::EMBEDS && self.embedding {
            self.embeds = match G::rule_payload(self.top().rule) {
                Payload::Embed(language) => Some(language),
                Payload::None | Payload::Backref(_) => None,
            };
        }
    }

    /// Ends the line where the first of `halts` holds before the patterns.
    /// It also ends the line where the second holds and no pattern matches.
    /// Both apply only while the root rule alone is open.
    /// The patterns still see the whole line.
    /// A lookahead reads past the halt, as the grammar's does.
    pub(crate) fn halting(mut self, halts: &'a Halts<'a>) -> Self {
        self.halt = Some(halts);
        self
    }

    /// Opens the rules of the state the line starts in, unless the frames
    /// the last line ended with are already in place.
    #[expect(
        clippy::inline_always,
        reason = "measured at one codegen unit: once its callers were forced inline, \
                  it stopped being inlined, at a call once a line"
    )]
    #[inline(always)]
    fn open(&mut self) {
        if self.frames.is_empty() {
            let mut chain = std::mem::take(&mut self.scratch.chain);
            self.open_frames(self.state, &mut chain);
            self.scratch.chain = chain;
            if G::WATCHES_FRAMES {
                G::frames_opened(self);
            }
            #[cfg(feature = "_embedding")]
            if G::EMBEDS_ON_DEMAND
                && !self.embedding
                && let Payload::Embed(language) = G::rule_payload(self.top().rule)
            {
                self.yielded = Some((self.pos, language, self.anchor == Some(self.pos)));
            }
        }
        if G::CAPTURED_EOL
            && self.frames.len() > 1
            && G::rule_payload(self.top().rule) == Payload::None
            && self.top().payload == CAPTURED_EOL
        {
            self.anchor = Some(self.pos);
        }
        if self.anchored_at_root && self.frames.len() == 1 {
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
            if G::rule_from_code(frame.rule) == G::ROOT || frame.parent == ROOT {
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
                        .push(frame.parent, frame.rule, 0, frame.stack)
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
                    next_while: 0,
                });
                continue;
            }
            let rule = G::rule_from_code(frame.rule);
            let parent = if G::BEGIN_STACKS {
                G::begin_stack(self, rule).unwrap_or(self.top().content)
            } else {
                self.top().content
            };
            let name = self.rule_name_path(parent, rule, frame.payload);
            self.frames.push(Frame {
                rule,
                enter: None,
                anchor: None,
                name,
                content: frame.stack,
                state: id,
                payload: frame.payload,
                next_while: 0,
            });
        }
    }

    /// Lexes the line, returns the state after it, and gives the buffers back
    /// to the scratch.
    /// When each open rule has a state of its own, the open rules are the
    /// state's chain, and the next line takes them over.
    /// A node limit can deny a rule its own state.
    #[inline]
    pub(crate) fn run(self) -> Finished {
        self.run_halting().0
    }

    /// Lexes the line like [`Lexer::run`], and returns where it halted too:
    /// the tokens end there, and the embedding lexer goes on from there.
    #[expect(
        clippy::inline_always,
        reason = "measured at one codegen unit: with `lex_until` a second caller, \
                  it stopped being inlined, which cost CSS, C and SQL a call once a line"
    )]
    #[inline(always)]
    pub(crate) fn run_halting(mut self) -> (Finished, Option<Stopped>) {
        self.open();
        self.check_while();
        self.tokenize();
        let stopped = self.stopped();
        (self.finish(), stopped)
    }

    /// Lexes the line like [`Lexer::run`], and returns whether it needs the
    /// languages its rules embed ([`Grammar::EMBEDS_ON_DEMAND`]).
    /// The rule's text is lexed as the rule's own.
    /// It needs them where it starts inside a rule that embeds one, or opens
    /// one. The caller then lexes the line again with [`Lexer::run_embedding`].
    #[cfg(feature = "_embedding")]
    pub(crate) fn run_on_demand(mut self) -> (Finished, bool) {
        self.open();
        self.check_while();
        self.tokenize();
        let embeds = self.yielded.is_some();
        (self.finish(), embeds)
    }

    /// Lexes the line like [`Lexer::run`], but stops where the innermost
    /// rule embeds another language and returns where.
    ///
    /// Its count is given back by then.
    /// So the caller lends what the line has left to the embedded language.
    /// Where that language halts, the caller lends the rest to
    /// [`Lexer::resume_embedding`].
    /// Every part of the line spends one budget.
    #[cfg(feature = "_embedding")]
    pub(crate) fn run_embedding(mut self) -> (Finished, Option<Embedding>) {
        let embedding = self.embed(Entry::Start);
        (self.finish(), embedding)
    }

    /// Goes on with the line where an embedded language halted, like
    /// [`Lexer::run_embedding`].
    ///
    /// The rule's own patterns are tried at the cursor first.
    /// The `while` rules, which only the start of a line tries, are not.
    #[cfg(feature = "_embedding")]
    pub(crate) fn resume_embedding(mut self) -> (Finished, Option<Embedding>) {
        let embedding = self.embed(Entry::Resume);
        (self.finish(), embedding)
    }

    /// Fills `starts` with the places from the cursor where one of the
    /// innermost rule's patterns matches, in order.
    /// They are an embedding rule's own patterns
    /// ([`crate::rules::Embeds::embeds_after_own`]).
    /// Each place is the one the loop's search finds from the place before.
    /// So the patterns are tried as the loop would try them there.
    /// Nothing is emitted.
    #[cfg(feature = "_embedding")]
    pub(crate) fn pattern_starts(mut self, starts: &mut Vec<usize>) -> Finished {
        self.open();
        starts.clear();
        while let Some((at, _, _)) = G::scan(&mut self) {
            starts.push(at);
            if at >= self.s.len() {
                break;
            }
            self.pos = self.char_end(at);
        }
        self.finish()
    }

    /// Lexes the line like [`Lexer::run_embedding`], for a grammar that
    /// another language embeds in turn.
    /// A line that halts ([`Lexer::halting`]) says where.
    #[cfg(feature = "_halting-embedding")]
    pub(crate) fn run_embedding_halting(mut self) -> (Finished, Stop) {
        let embedding = self.embed(Entry::Start);
        let stop = self.stop(embedding);
        (self.finish(), stop)
    }

    /// Goes on with the line like [`Lexer::resume_embedding`], for a grammar
    /// that another language embeds in turn.
    /// A line that halts says where.
    #[cfg(feature = "_halting-embedding")]
    pub(crate) fn resume_embedding_halting(mut self) -> (Finished, Stop) {
        let embedding = self.embed(Entry::Resume);
        let stop = self.stop(embedding);
        (self.finish(), stop)
    }

    /// How a halting line stopped: for `embedding`, where it halted, or at
    /// its end.
    #[cfg(feature = "_halting-embedding")]
    fn stop(&self, embedding: Option<Embedding>) -> Stop {
        match (embedding, self.stopped()) {
            (Some(embedding), _) => Stop::Embedding(embedding),
            (None, Some(stopped)) => Stop::Halted(stopped),
            (None, None) => Stop::Line,
        }
    }

    /// The loop of the embedding entry points.
    /// It returns where the innermost rule hands the text to the language it
    /// embeds.
    #[cfg(feature = "_embedding")]
    #[expect(
        clippy::inline_always,
        reason = "measured at one codegen unit: the halting entry points are a second call site, \
                  and Ruby's lines kept their count only with it inlined"
    )]
    #[inline(always)]
    fn embed(&mut self, entry: Entry) -> Option<Embedding> {
        self.embedding = true;
        self.open();
        match entry {
            Entry::Start => self.check_while(),
            Entry::Resume => self.host = true,
        }
        self.note_embedding();
        if !G::EMBEDS
            && !self.host
            && let Payload::Embed(language) = G::rule_payload(self.top().rule)
        {
            self.yield_to(language);
        } else {
            self.tokenize();
        }
        self.yielded.map(|(at, language, anchored)| {
            let top = self.top();
            Embedding {
                at,
                anchored,
                language,
                state: top.payload,
                base: top.content,
                parent: self.frames[self.frames.len() - 2].payload,
            }
        })
    }

    /// The state after the line.
    ///
    /// The buffers go back to the scratch.
    /// Where the open rules are the state's chain, they are kept for the next
    /// line (see [`Lexer::run`]).
    /// The line's count goes back to the lender (see [`Steps::finish`]).
    #[expect(
        clippy::inline_always,
        reason = "measured at one codegen unit: a call here, with the lexer moved into it, \
                  costs CSS 0.7% of its instructions"
    )]
    #[inline(always)]
    fn finish(mut self) -> Finished {
        let state = self.top().state;
        let exact = !self.flags.shared;
        #[cfg(test)]
        let exact = exact && !crate::resume_check::rebuilds();
        self.scratch.resume = exact.then_some(state);
        if exact {
            if self.flags.kept
                && let Some(top) = self.frames.last_mut()
            {
                top.content = self.arena.frame(top.state).stack;
            }
            let low = self.low.min(self.frames.len());
            for frame in &mut self.frames[low..] {
                (frame.enter, frame.anchor) = (None, None);
            }
            if G::WHILES && (!self.linked || low < self.frames.len()) {
                self.link_whiles(low);
            }
            if G::WATCHES_FRAMES {
                G::frames_settled(&mut self, low);
            }
            std::mem::swap(&mut self.frames, &mut self.scratch.resume_frames);
        }
        if self.dirty {
            self.captures[..G::GROUPS].fill(None);
        }
        self.scratch.frames = self.frames;
        self.scratch.spans = self.spans;
        self.scratch.captures = self.captures;
        self.scratch.kept = G::keep(self.x);
        self.steps.finish(state)
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

    /// The line's halts while only the root rule is open, and `None` otherwise.
    /// A scan opens and closes no rules.
    /// A scan calls it once.
    pub(crate) fn root_halts(&self) -> Option<&'a Halts<'a>> {
        self.halt.filter(|_| self.frames.len() == 1)
    }

    /// The deep halt of the line's halts below the root, with whether it
    /// loses a tie with the patterns ([`Halt::deep_loses_ties`]).
    ///
    /// There is one where the grammar takes one ([`Grammar::DEPTH_HALTS`])
    /// and no open rule shuts it out.
    pub(crate) fn deep_halt(&self) -> Option<(&'a dyn Halt, bool)> {
        let &(end, _) = self.halt?;
        end.deep()
            .filter(|_| G::deep_halts_open(self))
            .map(|deep| (deep, end.deep_loses_ties()))
    }

    /// Records that the scan stopped at the halt at `p`, where the line ends.
    pub(crate) fn halt_here(&mut self, p: usize) {
        self.halted = Some(p);
    }

    /// Where and why the line stopped for the grammar that embeds this one.
    ///
    /// The `reason` rests on one invariant: only [`Lexer::match_rule`] takes
    /// the halts.
    /// It takes them where an empty match at the root of a halting line pops
    /// the rule that embeds the grammar.
    /// A halt, the end of the line and every other path leave them set.
    /// Less's tests `an_embedded_line_halts_at_the_hosts_end` and
    /// `an_embedded_line_pops_the_host_at_a_root_close` check both reasons.
    fn stopped(&self) -> Option<Stopped> {
        let reason = match self.halt {
            Some(_) => StopReason::Halt,
            None => StopReason::PopHost,
        };
        self.halted.map(|at| Stopped { at, reason })
    }

    /// Whether `\G` matches at `p`.
    pub(crate) fn at_g(&self, p: usize) -> bool {
        self.g && p == self.pos
    }

    /// The regex `$` at `i`: before the line feed or at the end of the text.
    pub(crate) fn at_eol(&self, i: usize) -> bool {
        i == self.s.len() || (self.nl && i == self.s.len() + 1)
    }

    /// Whether the text at `i` starts with `text`, which must not be empty.
    ///
    /// The first byte decides almost always.
    /// So it is tested before the call that compares the rest, and an empty
    /// `text` would never match.
    ///
    /// A needle that can be empty goes through `Chars::lit`, where it matches
    /// as an empty literal does in a regular expression.
    /// Matchers call this one with words from lists in their hot loops, where
    /// one more test costs (Make +1.9% instructions, 2026-11-03).
    #[expect(
        clippy::inline_always,
        reason = "measured at one codegen unit: called from both instances of C's scan, \
                  it stopped being inlined there, at a call per place"
    )]
    #[inline(always)]
    pub(crate) fn starts(&self, i: usize, text: &[u8]) -> bool {
        debug_assert!(!text.is_empty(), "an empty needle goes through Chars::lit");
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
        self.steps.spend(steps);
    }

    /// Whether the line has spent its work budget.
    pub(crate) fn over_budget(&self) -> bool {
        self.steps.exhausted()
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
        crate::utf8::multibyte_at(self.s, i, b).map(|(c, width)| (c, i + width))
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
    pub(crate) fn capture(&mut self, group: usize, start: usize, end: usize) {
        self.captures[group] = Some((start, end));
        self.dirty = true;
    }

    /// The span of capture group `group` of the last match, for a test that
    /// compares the matchers with another regex engine.
    #[cfg(test)]
    pub(crate) fn capture_span(&self, group: usize) -> Option<(usize, usize)> {
        self.captures[group]
    }

    /// The bytes `start..end` of the line, empty where they lie outside it.
    pub(crate) fn text(&self, start: usize, end: usize) -> &'a [u8] {
        let s = self.s;
        s.get(start..end).unwrap_or(&[])
    }

    /// The text of capture group `group` of the last match, cut at the end of
    /// the line, and empty where the group starts past it.
    pub(crate) fn capture_text(&self, group: usize) -> &'a [u8] {
        self.captures[group].map_or(&[][..], |(start, end)| {
            self.text(start, end.min(self.s.len()))
        })
    }

    // Spans.

    /// Names `start..end` with `scopes`.
    pub(crate) fn span(&mut self, start: usize, end: usize, scopes: Scopes<G::Scope>) {
        self.spans.push(Span {
            start,
            end,
            scopes,
            single: None,
            dynamic: None,
            retok: None,
        });
        self.dirty = true;
    }

    /// Lexes `start..end` again with the patterns of `rule`, under the
    /// match's stack and `scopes`.
    pub(crate) fn span_retok(
        &mut self,
        start: usize,
        end: usize,
        scopes: Scopes<G::Scope>,
        rule: G::Rule,
    ) {
        self.spans.push(Span {
            start,
            end,
            scopes,
            single: None,
            dynamic: None,
            retok: Some(rule),
        });
        self.dirty = true;
    }

    /// Names `start..end` with one scope that ends in `dynamic`.
    pub(crate) fn span_dyn(&mut self, start: usize, end: usize, dynamic: DynName) {
        self.spans.push(Span {
            start,
            end,
            scopes: G::NO_SCOPES,
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
            scopes: G::NO_SCOPES,
            single: scope,
            dynamic: None,
            retok,
        });
        self.dirty = true;
    }

    /// Runs `matcher` at `p` with `\G` matching there or not, and gives
    /// where its match ends and its capture groups: for the tests that
    /// compare matchers with Oniguruma.
    #[cfg(test)]
    pub(crate) fn probe_matcher(
        &mut self,
        p: usize,
        g: bool,
        matcher: impl FnOnce(&mut Self, usize) -> Option<usize>,
    ) -> Option<Probe> {
        self.open();
        self.captures.fill(None);
        self.pos = p;
        self.g = g;
        let end = matcher(self, p)?;
        Some((end, self.captures.clone()))
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

    /// The innermost rule's payload text.
    pub(crate) fn payload_text(&self) -> &str {
        self.arena.string(self.top().payload)
    }

    /// The rules open on the line, innermost first, each with its index for
    /// [`Lexer::payload_text_of`].
    #[cfg(any(
        feature = "lang-svelte",
        feature = "lang-astro",
        feature = "lang-blade"
    ))]
    pub(crate) fn open_rules(&self) -> impl Iterator<Item = (usize, G::Rule)> + '_ {
        self.frames.iter().map(|frame| frame.rule).enumerate().rev()
    }

    /// The payload text of the open rule at `index`, a rule whose payload is
    /// a begin capture.
    #[cfg(any(feature = "lang-svelte", feature = "lang-astro"))]
    pub(crate) fn payload_text_of(&self, index: usize) -> &str {
        self.arena.string(self.frames[index].payload)
    }

    /// The payload text of the rule around the innermost one.
    /// An example is the begin capture of an outer rule whose end the innermost
    /// rule waits for.
    #[cfg(any(feature = "lang-lua", feature = "lang-svelte", feature = "lang-haml"))]
    pub(crate) fn parent_payload_text(&self) -> &str {
        self.frames
            .len()
            .checked_sub(2)
            .map_or("", |parent| self.arena.string(self.frames[parent].payload))
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
        self.limits.depth_reached(self.depth())
    }

    /// Leaves the text from the cursor up to `next` plain, as the innermost
    /// rule's content, with a token boundary at `start`.
    /// It moves the cursor to `next`.
    /// A language calls it for a rule that the depth limit keeps closed when
    /// it has not skipped the pattern.
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
    /// from being made is left out. The path is cached by its base and the
    /// list's key ([`PathEntry`]).
    pub(crate) fn path(&mut self, base: u32, scopes: Scopes<G::Scope>) -> u32 {
        if scopes.list.is_empty() {
            return base;
        }
        let slot = path_slot(base, scopes.key);
        if let Some(entry) = &self.scratch.paths[slot]
            && entry.base == base
            && std::ptr::eq(entry.key, &raw const scopes.key)
        {
            return entry.node;
        }
        self.path_miss(base, scopes, slot)
    }

    /// A [`path`](Self::path) the cache does not hold: makes the stack, and
    /// caches it in `slot`.
    ///
    /// It does not cache a stack with a scope named from a capture group,
    /// which depends on the match.
    ///
    /// Out of line, so a hit saves no registers for the walk.
    #[inline(never)]
    fn path_miss(&mut self, base: u32, scopes: Scopes<G::Scope>, slot: usize) -> u32 {
        let mut stack = base;
        let mut whole = true;
        let mut fixed = true;
        for &scope in &scopes.list {
            let fixed_scope = match G::scope(scope) {
                ScopeName::Fixed(fixed_scope) => fixed_scope,
                ScopeName::Capture { .. } => {
                    fixed = false;
                    stack = self.path_one(stack, scope);
                    continue;
                }
            };
            let Some(child) = self.arena.child_or_insert(stack, fixed_scope) else {
                self.report.record(Limit::Nodes);
                self.flags.failed = true;
                whole = false;
                break;
            };
            stack = child;
        }
        if whole && fixed {
            self.scratch.paths[slot] = Some(PathEntry {
                base,
                node: stack,
                key: &scopes.key,
            });
        }
        stack
    }

    /// The stack `base` with `scope` on top: a scope outside the grammar's
    /// own table, such as CSS's in SCSS, which runs CSS's nested patterns.
    pub(crate) fn path_fixed(&mut self, base: u32, scope: crate::scope::Scope) -> u32 {
        self.arena.child(base, scope).unwrap_or_else(|| {
            self.report.record(Limit::Nodes);
            self.flags.failed = true;
            base
        })
    }

    /// The stack `base` with one scope on top, which is not cached.
    /// It is the scope of a run-time choice, with no slice of its own to key a
    /// cache.
    pub(crate) fn path_one(&mut self, base: u32, scope: G::Scope) -> u32 {
        match G::scope(scope) {
            ScopeName::Fixed(fixed) => self.arena.child(base, fixed).unwrap_or_else(|| {
                self.report.record(Limit::Nodes);
                self.flags.failed = true;
                base
            }),
            ScopeName::Capture {
                prefix,
                group,
                suffix,
            } => self.dynamic_path(base, prefix, NameSource::Group(group), suffix),
        }
    }

    /// The stack `base` with the scopes `prefix`, the text of `source` and
    /// `suffix` spell out on top. A name with spaces is several scopes, and
    /// leading dots of the text are dropped, as a `TextMate` engine does for
    /// the text of a capture.
    fn dynamic_path(
        &mut self,
        base: u32,
        prefix: &'static str,
        source: NameSource,
        suffix: &str,
    ) -> u32 {
        let mut name = std::mem::take(&mut self.scratch.name);
        name.clear();
        name.push_str(prefix);
        {
            let s = self.s;
            match source {
                NameSource::Line { start, end, case } => {
                    let text = String::from_utf8_lossy(s.get(start..end).unwrap_or_default());
                    push_spelled(&mut name, &text, case);
                }
                NameSource::Interned(id) => {
                    name.push_str(self.arena.string(id).trim_start_matches('.'));
                }
                NameSource::Group(group) => {
                    let text = String::from_utf8_lossy(self.capture_text(group.into()));
                    push_spelled(&mut name, &text, G::dyn_name_case(prefix));
                }
            }
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

    /// The stack `parent` with the name scopes of `rule`, which carries
    /// `payload`, on top.
    fn rule_name_path(&mut self, parent: u32, rule: G::Rule, payload: u32) -> u32 {
        let name = self.path(parent, G::rule_name(rule));
        let Some(prefix) = G::rule_name_delimiter(rule) else {
            return name;
        };
        let source = if self.arena.string(payload).len() > G::NAMED_PAYLOAD {
            NameSource::Line {
                start: 0,
                end: 0,
                case: NameCase::AsWritten,
            }
        } else {
            NameSource::Interned(payload)
        };
        let named = self.dynamic_path(name, prefix, source, G::rule_name_suffix(rule));
        self.path(named, G::rule_name_after(rule))
    }

    fn push_frame(
        &mut self,
        rule: G::Rule,
        enter: Option<usize>,
        anchor: Option<usize>,
        payload: u32,
    ) {
        self.low = self.low.min(self.frames.len());
        let parent_state = self.top().state;
        let parent_scopes = if G::BEGIN_STACKS {
            G::begin_stack(self, rule).unwrap_or(self.top().content)
        } else {
            self.top().content
        };
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
                next_while: 0,
            });
            if G::DEPTH_HALTS {
                G::frame_pushed(self);
            }
            return;
        }
        self.flags.failed = false;
        let name = self.rule_name_path(parent_scopes, rule, payload);
        let content = match G::rule_content_delimiter(rule) {
            Some(prefix) => self.dynamic_path(name, prefix, NameSource::Interned(payload), ""),
            None => self.path(name, G::rule_content(rule)),
        };
        let state = self
            .arena
            .push_or_insert(parent_state, code, payload, content)
            .unwrap_or_else(|| {
                self.report.record(Limit::Nodes);
                self.flags.failed = true;
                self.flags.shared = true;
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
            next_while: 0,
        });
        if G::DEPTH_HALTS {
            G::frame_pushed(self);
        }
    }

    fn pop_frame(&mut self) -> Frame<G> {
        let frame = self.frames[self.frames.len() - 1];
        self.frames.pop();
        if G::DEPTH_HALTS {
            G::frames_popped(self);
        }
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

    /// Emits a pattern's table of capture groups.
    ///
    /// Some groups are tokenized with patterns of their own.
    /// Such a group runs them under the match's stack `base` plus its name, as
    /// vscode-textmate's `handleCaptures` does.
    /// It does not run them under the groups around it.
    /// A group that starts past the match, in a lookahead, ends the groups, as
    /// in vscode-textmate.
    fn emit_captures(
        &mut self,
        (table, tokenized_groups): Captures<G>,
        base: u32,
        start: usize,
        end: usize,
    ) {
        self.captures[0] = Some((start, end));
        self.dirty = true;
        let mut stack = [(0_usize, 0_u32); NEST];
        let mut depth = 0;
        let (mut named, mut tokenized) = (0, 0);
        loop {
            let (group, scopes, nested) = match (table.get(named), tokenized_groups.get(tokenized))
            {
                (None, None) => break,
                (Some(&(group, scopes)), Some(&(other, _))) if group <= other => {
                    named += 1;
                    (group, scopes, None)
                }
                (Some(&(group, scopes)), None) => {
                    named += 1;
                    (group, scopes, None)
                }
                (_, Some(&(group, nested))) => {
                    tokenized += 1;
                    (group, G::NO_SCOPES, Some(nested))
                }
            };
            let Some((from, to)) = self.captures[usize::from(group)] else {
                continue;
            };
            if to <= from {
                continue;
            }
            if from > end {
                break;
            }
            if from > self.s.len() {
                continue;
            }
            while depth > 0 && stack[depth - 1].0 <= from {
                self.emit(stack[depth - 1].0, stack[depth - 1].1);
                depth -= 1;
            }
            let outer = if depth > 0 { stack[depth - 1].1 } else { base };
            self.emit(from, outer);
            if let Some(nested) = nested {
                let named = table
                    .iter()
                    .find(|&&(number, _)| number == group)
                    .map_or(G::NO_SCOPES, |&(_, scopes)| scopes);
                let inner = self.path(base, named);
                self.nested(nested, from, to, inner);
                continue;
            }
            if scopes.list.is_empty() {
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
    /// matches patterns of its own.
    /// The groups of the match being emitted are saved meanwhile, on a stack.
    /// The hook's matches, and the nested groups those emit in turn, then do
    /// not overwrite them.
    fn nested(&mut self, nested: G::Nested, from: usize, to: usize, base: u32) {
        #[cfg(feature = "_embedding")]
        if G::DEFERS_NESTED
            && let Some(language) = G::defers_nested(nested)
        {
            self.defer(language, from, to, base);
            return;
        }
        let mark = self.scratch.saved_captures.len();
        self.scratch
            .saved_captures
            .extend_from_slice(&self.captures);
        let dirty = self.dirty;
        self.captures[..G::GROUPS].fill(None);
        self.dirty = false;
        G::tokenize_nested(self, nested, from, to, base);
        self.captures
            .copy_from_slice(&self.scratch.saved_captures[mark..]);
        self.scratch.saved_captures.truncate(mark);
        self.dirty = dirty;
    }

    /// Emits the capture group text `from..to` as one placeholder token on
    /// `base`, and notes it for `language` ([`super::Embeds::lex_nested`]).
    #[cfg(feature = "_embedding")]
    #[cold]
    fn defer(&mut self, language: Language, from: usize, to: usize, base: u32) {
        let token = self.out.len();
        self.emit(to, base);
        if self.out.len() > token
            && let Some(slot) = G::deferred(&mut self.x)
        {
            slot.get_or_insert_with(Box::default).groups.push(Deferred {
                language,
                token,
                from,
                to: to.min(self.s.len()),
                base,
            });
        }
    }

    /// Emits the spans the matcher recorded, in order. In a grammar with
    /// [`Grammar::NESTED_SPANS`], a span that lies inside one recorded before
    /// it takes that span's scopes too, as capture groups do. A span lexed
    /// again does not, and runs under the match's stack `base` and its own
    /// scopes, as vscode-textmate's `handleCaptures` does.
    fn emit_spans(&mut self, base: u32) {
        let mut open = 0;
        for i in self.span_base..self.spans.len() {
            let span = self.spans[i];
            if span.start == span.end {
                continue;
            }
            let mut outer = base;
            while open > 0 {
                let Some(&(end, node)) = self.scratch.open_spans.last() else {
                    break;
                };
                if end > span.start {
                    outer = node;
                    break;
                }
                self.emit(end, node);
                self.scratch.open_spans.pop();
                open -= 1;
            }
            self.emit(span.start, outer);
            let under = if span.retok.is_some() { base } else { outer };
            let mut named = self.path(under, span.scopes);
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
                        case: G::dyn_name_case(dynamic.prefix),
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
                None if G::NESTED_SPANS
                    && self
                        .spans
                        .get(i + 1)
                        .is_some_and(|next| next.start < span.end) =>
                {
                    self.scratch.open_spans.push((span.end, node));
                    open += 1;
                }
                None => self.emit(span.end, node),
            }
        }
        for _ in 0..open {
            if let Some((end, node)) = self.scratch.open_spans.pop() {
                self.emit(end, node);
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
        let saved_embeds = if G::EMBEDS { self.embeds.take() } else { None };
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
            next_while: 0,
        });
        if G::DEPTH_HALTS {
            G::frame_pushed(self);
        }
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
        if G::DEPTH_HALTS {
            G::frames_popped(self);
        }
        if G::EMBEDS {
            self.embeds = saved_embeds;
        }
        self.spans.truncate(self.span_base);
        self.span_base = saved_base;
        self.dirty = true;
    }

    // The loop.

    /// Runs from `pos` until the text ends or the line stops.
    ///
    /// Group 0 of each match is set before its scopes are made, for a name
    /// that repeats the whole match (`$0`).
    /// It does not mark the groups for clearing.
    /// Each match sets it before anything reads it, so the next try need not
    /// clear the groups for it.
    fn tokenize(&mut self) {
        let line_length = self.s.len() + usize::from(self.nl);
        let mut stalled = 0_usize;
        loop {
            self.spend(1);
            if self.steps.exhausted() {
                // Out of work: the rest of the line is the rule's.
                self.report.record(Limit::Work);
                let node = self.top().content;
                self.emit(line_length, node);
                return;
            }
            if G::EMBEDS
                && let Some(language) = self.embeds
            {
                if !self.host {
                    self.yield_to(language);
                    return;
                }
                self.host = false;
            }
            self.g = self.anchor == Some(self.pos);
            let Some((start, pattern, end)) = G::scan(self) else {
                if let Some(at) = self.halted {
                    let node = self.top().content;
                    self.emit(at, node);
                    return;
                }
                if self.steps.exhausted() {
                    self.report.record(Limit::Work);
                }
                let node = self.top().content;
                self.emit(line_length, node);
                return;
            };
            self.captures[0] = Some((start, end));
            let advanced = end > self.pos;
            let (act, captures) = G::pattern_definition(pattern);
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
                debug_assert!(
                    stalled < STALLED_OPENS.saturating_add(self.limits.depth.unwrap_or(usize::MAX)),
                    "the {} lexer made no progress at byte {}",
                    G::NAME,
                    self.pos
                );
            }
        }
    }

    /// Tries the `while` pattern of each open rule that has one, from the
    /// root up, at the start of the line, as vscode-textmate does. A match
    /// names its text under that rule's content scopes, not those of the rules
    /// above it, and moves the cursor and `\G` past it. The first rule whose
    /// pattern fails closes, with every rule above it.
    #[expect(
        clippy::inline_always,
        reason = "measured at one codegen unit: with run_embedding a second caller, \
                  it stopped being inlined, at a call once a line"
    )]
    #[inline(always)]
    fn check_while(&mut self) {
        if G::WHILES && self.linked {
            let mut i = usize::try_from(self.frames[0].next_while).unwrap_or(0);
            while i != 0 && i < self.frames.len() {
                let frame = self.frames[i];
                let Some(pattern) = G::rule_while(frame.rule) else {
                    return;
                };
                self.g = self.anchor == Some(self.pos);
                let Some(end) = self.try_pattern(pattern, self.pos) else {
                    self.frames.truncate(i);
                    if G::DEPTH_HALTS {
                        G::frames_popped(self);
                    }
                    return;
                };
                self.emit_match(G::pattern_captures(pattern), frame.content, self.pos, end);
                self.anchor = Some(end);
                self.pos = end;
                i = usize::try_from(frame.next_while).unwrap_or(0);
            }
            return;
        }
        let mut last = 0;
        for i in 1..self.frames.len() {
            let frame = self.frames[i];
            let Some(pattern) = G::rule_while(frame.rule) else {
                continue;
            };
            self.g = self.anchor == Some(self.pos);
            let Some(end) = self.try_pattern(pattern, self.pos) else {
                self.frames.truncate(i);
                if G::DEPTH_HALTS {
                    G::frames_popped(self);
                }
                break;
            };
            self.emit_match(G::pattern_captures(pattern), frame.content, self.pos, end);
            self.anchor = Some(end);
            self.pos = end;
            if G::WHILES {
                self.frames[last].next_while = u32::try_from(i).unwrap_or(0);
                last = i;
            }
        }
        if G::WHILES {
            self.frames[last].next_while = 0;
            self.linked = true;
        }
    }

    /// Links the frames with a `while` pattern for the next line.
    /// The links run from the root up.
    /// A link past the last frame is left for the next line to stop at, so a
    /// line that only closed rules relinks nothing.
    /// Where the links below `low`, the lowest frame the line pushed, hold,
    /// the line keeps them and relinks from there.
    #[expect(
        clippy::inline_always,
        reason = "measured at one codegen unit: out of line, \
                  its `&mut self` made ERB build each Ruby lexer in memory, one memcpy a run"
    )]
    #[inline(always)]
    fn link_whiles(&mut self, low: usize) {
        let from = if self.linked { low.max(1) } else { 1 };
        let mut last = 0;
        if self.linked {
            let mut next = usize::try_from(self.frames[0].next_while).unwrap_or(0);
            while next != 0 && next < from {
                last = next;
                next = usize::try_from(self.frames[next].next_while).unwrap_or(0);
            }
        }
        for i in from..self.frames.len() {
            if G::rule_while(self.frames[i].rule).is_some() {
                self.frames[last].next_while = u32::try_from(i).unwrap_or(0);
                last = i;
            }
        }
        self.frames[last].next_while = 0;
    }

    /// Tries `pattern` at `p`, clearing what the last try left.
    #[inline]
    pub(crate) fn try_pattern(&mut self, pattern: G::Pattern, p: usize) -> Option<usize> {
        if self.dirty {
            self.captures[..G::GROUPS].fill(None);
            self.spans.truncate(self.span_base);
            self.dirty = false;
        }
        G::matcher(self, pattern, p)
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
        self.note_embedding();
        self.anchor = popped.anchor;
        if !advanced && popped.enter == Some(self.pos) {
            // The rule opened and closed without advancing: the grammar would
            // loop, so the rest of the line is the rule's.
            self.flags.kept = true;
            self.frames.push(Frame {
                content: popped.name,
                ..popped
            });
            if G::DEPTH_HALTS {
                G::frame_pushed(self);
            }
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
        let mut embedded = None;
        let payload = match G::rule_payload(rule) {
            Payload::None if G::CAPTURED_EOL && end > self.s.len() => CAPTURED_EOL,
            Payload::None => 0,
            Payload::Embed(language) => {
                embedded = Some(language);
                0
            }
            Payload::Backref(group) => {
                let text = std::str::from_utf8(self.capture_text(usize::from(group))).unwrap_or("");
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
        self.note_embedding();
        match embedded {
            Some(language) => self.opened_embedding(language, end),
            None => false,
        }
    }

    /// Runs after a rule that embeds `language` opened with a begin match
    /// ending at `end`. A grammar with the per-step test
    /// ([`Grammar::EMBEDS`]) does nothing here.
    /// On an embedding line, it hands the line to `language` past the match.
    /// It returns true there, to stop the line.
    /// On another line, it notes the language and the line goes on.
    /// [`Lexer::run_on_demand`] reads that note.
    #[cold]
    fn opened_embedding(&mut self, language: Language, end: usize) -> bool {
        if G::EMBEDS {
            return false;
        }
        if !self.embedding {
            self.yielded = Some((self.pos, language, self.anchor == Some(self.pos)));
            return false;
        }
        self.pos = self.pos.max(end);
        self.yield_to(language);
        true
    }

    /// Ends the line at the cursor for `language`, which lexes the text from
    /// there.
    fn yield_to(&mut self, language: Language) {
        let node = self.top().content;
        self.emit(self.pos, node);
        self.yielded = Some((self.pos, language, self.anchor == Some(self.pos)));
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
    /// At the root of a halting line, the rule an empty match drops is the
    /// one that embeds the grammar.
    /// The line then drops its halts and stops at the match
    /// ([`StopReason::PopHost`]).
    ///
    /// That stop is read from the cursor, which equals `start` there:
    /// - the scan searches from the cursor, so `start` is at or past it;
    /// - a match that did not advance ends at or before the cursor;
    /// - [`Lexer::emit_match`] leaves the cursor where it was, and
    ///   [`Lexer::retokenise`] restores it after a capture's patterns.
    ///
    /// Reading `start` instead keeps it live across `emit_match`'s calls,
    /// which costs `tokenize` a stack reload on every match.
    fn match_rule(
        &mut self,
        name: Scopes<G::Scope>,
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
            } else if self.halt.take().is_some() {
                self.halted = Some(self.pos);
            }
            let node = self.top().content;
            self.emit(line_length, node);
            return true;
        }
        false
    }
}

/// Pushes `text` onto `name` without its leading dots, spelled as `case`
/// says.
/// Inlined, so a grammar that spells every name as written pushes the text
/// directly, as before names could be lowered.
#[inline]
fn push_spelled(name: &mut String, text: &str, case: NameCase) {
    let text = text.trim_start_matches('.');
    match case {
        NameCase::AsWritten => name.push_str(text),
        NameCase::Lowered => push_lowered(name, text),
    }
}

/// Pushes `text` onto `name` lowered, as `${1:/downcase}` does.
///
/// The text lowers one character at a time, with no allocation.
/// A text that holds a `Σ` goes to [`lower_with_final_sigma`].
///
/// This is out of line.
/// Inlined into `emit_match`, the loop raised CSS's count by 0.22% and
/// SCSS's by 0.17% at one codegen unit.
/// All of it was in `emit_match`'s own cost.
#[inline(never)]
fn push_lowered(name: &mut String, text: &str) {
    if text.contains('Σ') {
        lower_with_final_sigma(name, text);
    } else {
        name.extend(text.chars().flat_map(char::to_lowercase));
    }
}

/// Pushes `text` onto `name` lowered as JavaScript's `toLowerCase` does, for a
/// text that holds a `Σ`.
///
/// JavaScript lowers a `Σ` that ends a word to the final sigma `ς`.
/// It lowers every other `Σ` to `σ`.
/// `char::to_lowercase` sees one character at a time, so it gives `σ` for
/// every `Σ`.
/// `str::to_lowercase` applies the final-sigma rule.
/// std has no public test for the cased and case-ignorable letters that the
/// rule needs, so the lexer cannot apply it in place.
///
/// This allocates the lowered text.
/// It is cold, so only a text with a `Σ` pays for it.
#[cold]
#[inline(never)]
fn lower_with_final_sigma(name: &mut String, text: &str) {
    name.push_str(&text.to_lowercase());
}

/// The state `state` with its innermost rule's payload replaced by
/// `payload`.
///
/// The payload can be the state of the language the rule embeds after a
/// line.
/// The result is `None` when the node limit keeps the new state from being
/// made.
#[cfg(feature = "_embedding")]
pub(crate) fn with_payload(arena: &mut Arena, state: u32, payload: u32) -> Option<u32> {
    let frame = arena.frame(state);
    arena.push(frame.parent, frame.rule, payload, frame.stack)
}

#[cfg(test)]
mod tests {

    #[cfg(feature = "_embedding")]
    use super::Deferrals;
    use super::{Lexer, Scratch};
    #[cfg(feature = "_embedding")]
    use crate::Language;
    use crate::arena::{Arena, ROOT};
    #[cfg(feature = "_embedding")]
    use crate::language::Halt;
    use crate::language::Parts;
    use crate::limits::{Limits, LineReport};
    use crate::rules::{Act, Captures, Grammar, NameCase, Payload, ScopeName, Text};
    #[cfg(feature = "_embedding")]
    use crate::rules::{Embedding, Embeds, EndText, FnHalt};
    use crate::scope::{Scope, Scopes, scopes};

    #[derive(Clone, Copy, Debug, PartialEq)]
    struct Root;

    /// A grammar of bracket pairs whose inside is lexed again with the same
    /// rule, and words.
    ///
    /// `[[ab]]` nests one capture group's hook inside another's.
    /// That must leave the outer match's later groups intact.
    #[derive(Clone, Copy, Debug)]
    struct Brackets;

    #[derive(Clone, Copy, Debug)]
    enum Pattern {
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
        const NO_SCOPES: Scopes<Scope> = Scope::NO_SCOPES;
        type Rule = Root;
        type Pattern = Pattern;
        type Nested = Inside;
        type Extra = ();
        type Kept = ();

        const NAME: &'static str = "brackets";
        const ROOT: Root = Root;
        const SOURCE: Scope = OPEN;
        const GROUPS: usize = 4;

        fn scope(scope: Scope) -> ScopeName {
            ScopeName::Fixed(scope)
        }

        fn rule_code(_: Root) -> u16 {
            0
        }

        fn rule_from_code(_: u16) -> Root {
            Root
        }

        fn rule_name(_: Root) -> Scopes<Scope> {
            Scope::NO_SCOPES
        }

        fn rule_content(_: Root) -> Scopes<Scope> {
            Scope::NO_SCOPES
        }

        fn pattern_act(_: Pattern) -> Act<Self> {
            Act::Match(Scope::NO_SCOPES)
        }

        fn pattern_captures(pattern: Pattern) -> Captures<Self> {
            match pattern {
                Pattern::Bracket => (
                    &[(1, scopes![OPEN]), (3, scopes![CLOSE])],
                    &[(2, Inside::Again)],
                ),
                Pattern::Word => (&[(1, scopes![WORD])], &[]),
            }
        }

        fn scan(lexer: &mut Lexer<'_, Self>) -> Option<(usize, Pattern, usize)> {
            let from = lexer.pos;
            for p in from..lexer.lim() {
                for pattern in [Pattern::Bracket, Pattern::Word] {
                    if let Some(end) = lexer.try_pattern(pattern, p) {
                        return Some((p, pattern, end));
                    }
                }
            }
            None
        }

        fn matcher(lexer: &mut Lexer<'_, Self>, pattern: Pattern, p: usize) -> Option<usize> {
            match pattern {
                Pattern::Bracket => {
                    if lexer.at(p) != b'[' {
                        return None;
                    }
                    let close = p + lexer.s.get(p..)?.iter().rposition(|&b| b == b']')?;
                    lexer.capture(1, p, p + 1);
                    lexer.capture(2, p + 1, close);
                    lexer.capture(3, close, close + 1);
                    Some(close + 1)
                }
                Pattern::Word => {
                    let end = lexer.skip(p, |b| b.is_ascii_alphabetic());
                    (end > p).then(|| {
                        lexer.capture(1, p, end);
                        end
                    })
                }
            }
        }

        fn tokenize_nested(
            lexer: &mut Lexer<'_, Self>,
            nested: Inside,
            from: usize,
            to: usize,
            base: u32,
        ) {
            match nested {
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
            anchored_at_root: false,
        };
        let parts = Parts {
            arena: &mut arena,
            limits: &limits,
            out: &mut tokens,
            report: &mut report,
        };
        let _ = Lexer::new(parts, text, ROOT, &mut scratch).run();
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

    #[test]
    fn a_group_past_the_end_of_the_line_has_no_text() {
        let mut arena = Arena::new(None);
        let limits = Limits::default();
        let mut tokens = Vec::new();
        let mut report = LineReport::default();
        let mut scratch = Scratch::<Brackets>::default();
        let text = Text {
            bytes: b"ab",
            start: 0,
            anchored: false,
            anchored_at_root: false,
        };
        let parts = Parts {
            arena: &mut arena,
            limits: &limits,
            out: &mut tokens,
            report: &mut report,
        };
        let mut lexer = Lexer::new(parts, text, ROOT, &mut scratch);
        lexer.captures[1] = Some((1, 3));
        lexer.captures[2] = Some((3, 4));
        assert_eq!(lexer.capture_text(1), b"b");
        assert_eq!(lexer.capture_text(2), b"");
    }

    /// A lexer over `b"ab"` for the needle tests.
    fn ab_lexer<'a>(parts: Parts<'a>, scratch: &'a mut Scratch<Brackets>) -> Lexer<'a, Brackets> {
        let text = Text {
            bytes: b"ab",
            start: 0,
            anchored: false,
            anchored_at_root: false,
        };
        Lexer::new(parts, text, ROOT, scratch)
    }

    #[cfg(feature = "_rule-tables")]
    #[test]
    fn an_empty_literal_matches_anywhere_in_the_line() {
        use crate::rules::Chars;
        let (mut arena, limits, mut tokens) = (Arena::new(None), Limits::default(), Vec::new());
        let (mut report, mut scratch) = (LineReport::default(), Scratch::<Brackets>::default());
        let parts = Parts {
            arena: &mut arena,
            limits: &limits,
            out: &mut tokens,
            report: &mut report,
        };
        let lexer = ab_lexer(parts, &mut scratch);
        assert_eq!(lexer.lit(0, b""), Some(0));
        assert_eq!(lexer.lit(2, b""), Some(2));
        assert_eq!(lexer.lit(3, b""), None);
        assert_eq!(lexer.lit(1, b"b"), Some(2));
        assert_eq!(lexer.lit(2, b"b"), None);
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic(expected = "an empty needle goes through Chars::lit")]
    fn starts_refuses_an_empty_needle() {
        let (mut arena, limits, mut tokens) = (Arena::new(None), Limits::default(), Vec::new());
        let (mut report, mut scratch) = (LineReport::default(), Scratch::<Brackets>::default());
        let parts = Parts {
            arena: &mut arena,
            limits: &limits,
            out: &mut tokens,
            report: &mut report,
        };
        let lexer = ab_lexer(parts, &mut scratch);
        let _ = lexer.starts(0, b"");
    }

    /// A grammar whose names repeat text of the line.
    ///
    /// - A word is named `word.<the word>.test` (`$0`).
    /// - `!` and `?` are named after themselves as group 1, with a group 2 past
    ///   the match and a group 3 after it.
    /// - `<<ab` opens a quote named `quote.ab.q after.test` that the next `ab`
    ///   closes.
    /// - `>>` opens a rule that embeds Rust from an entry
    ///   ([`Grammar::embed_start`]), and takes the line feed when it ends the
    ///   line.
    /// - `%%` opens a rule that embeds JavaScript, which halts before it lexes
    ///   anything, and then names a `!` only where `\G` holds.
    /// - `##` opens a rule that embeds HTML and lists the mark of `!` and `?`
    ///   after it ([`Embeds::embeds_after_own`]). A quote takes `##` too.
    /// - `@@` starts a capture group that runs to the next blank, which
    ///   JavaScript lexes again ([`Grammar::defers_nested`]).
    ///
    /// The end halt of the rules that embed records the text of the embedding
    /// rule's parent payload ([`EndText::ParentPayload`]).
    #[derive(Clone, Copy, Debug)]
    struct Named;

    /// The one of [`Named`]'s capture groups that JavaScript lexes again.
    #[derive(Clone, Copy, Debug)]
    enum Lexed {
        Script,
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    enum Rule {
        Root,
        Quote,
        Inner,
        Held,
        Mixed,
    }

    #[derive(Clone, Copy, Debug)]
    enum Mark {
        Word,
        Look,
        Open,
        Close,
        Embed,
        Hold,
        Mix,
        Anchored,
        Call,
    }

    /// The entry [`Rule::Inner`] embeds Rust from: Rust's own start for an
    /// embedded body, under another function, so a test can tell it apart.
    #[cfg(all(feature = "_embedding", feature = "lang-rust"))]
    fn inner_start(arena: &mut Arena, base: u32) -> u32 {
        crate::Language::Rust.start_embedded(arena, base)
    }

    /// What [`Rule::Inner`] carries: Rust, or nothing in a build without Rust.
    fn inner_payload() -> Payload {
        #[cfg(feature = "lang-rust")]
        let payload = Payload::Embed(crate::Language::Rust);
        #[cfg(not(feature = "lang-rust"))]
        let payload = Payload::None;
        payload
    }

    /// What [`Rule::Held`] carries: JavaScript, or nothing in a build
    /// without it.
    fn held_payload() -> Payload {
        #[cfg(feature = "lang-javascript")]
        let payload = Payload::Embed(crate::Language::JavaScript);
        #[cfg(not(feature = "lang-javascript"))]
        let payload = Payload::None;
        payload
    }

    /// What [`Rule::Mixed`] carries: HTML, or nothing in a build without it.
    fn mixed_payload() -> Payload {
        #[cfg(feature = "lang-html")]
        let payload = Payload::Embed(crate::Language::Html);
        #[cfg(not(feature = "lang-html"))]
        let payload = Payload::None;
        payload
    }

    /// The halt of the rules of [`Named`] that embed.
    /// For [`Rule::Inner`] and [`Rule::Held`] it holds where the embedded
    /// text starts, so the language halts before it lexes anything.
    /// For [`Rule::Mixed`] it holds nowhere.
    #[cfg(feature = "_embedding")]
    struct AtOnce {
        at: usize,
    }

    #[cfg(feature = "_embedding")]
    impl Halt for AtOnce {
        fn first(&self, _: &[u8], from: usize, to: usize) -> Option<usize> {
            (from..=to).contains(&self.at).then_some(self.at)
        }
    }

    /// What [`Named`] keeps from line to line: the deferred capture groups,
    /// and the buffer for its end halt's parent payload text.
    #[cfg(feature = "_embedding")]
    #[derive(Debug, Default)]
    struct NamedKept {
        deferrals: Option<Box<Deferrals>>,
        parent_text: Vec<u8>,
    }

    #[cfg(feature = "_embedding")]
    std::thread_local! {
        /// How many times the engine asked [`Named`] to lex an embedded
        /// language its own way, on this test's thread.
        static ASKED: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
        /// The parent payload texts the engine gave [`Named`]'s end halt,
        /// on this test's thread.
        static PARENT_TEXTS: std::cell::RefCell<Vec<Vec<u8>>> =
            const { std::cell::RefCell::new(Vec::new()) };
    }

    #[cfg(feature = "_embedding")]
    impl Embeds for Named {
        type End = AtOnce;

        fn embed_halts(embedding: &Embedding, _: &Arena) -> (AtOnce, Option<FnHalt>) {
            (AtOnce { at: embedding.at }, None)
        }

        fn embed_halts_for(
            rule: Rule,
            embedding: &Embedding,
            arena: &Arena,
        ) -> (AtOnce, Option<FnHalt>) {
            match rule {
                Rule::Mixed => (AtOnce { at: usize::MAX }, None),
                Rule::Root | Rule::Quote | Rule::Inner | Rule::Held => {
                    Self::embed_halts(embedding, arena)
                }
            }
        }

        fn embeds_after_own(rule: Rule) -> bool {
            matches!(rule, Rule::Mixed)
        }

        const END_TEXT: EndText = EndText::ParentPayload;

        /// Records the parent payload text, then asks `end`.
        fn end_holds(end: &AtOnce, parent_text: &[u8], s: &[u8], p: usize) -> bool {
            PARENT_TEXTS.with_borrow_mut(|texts| texts.push(parent_text.to_vec()));
            end.holds(s, p)
        }

        fn parent_text(kept: &mut NamedKept) -> Option<&mut Vec<u8>> {
            Some(&mut kept.parent_text)
        }

        /// Counts the call and declines, so the language lexes itself.
        fn lex_embedded_until(
            _: Rule,
            _: crate::Language,
            _: &mut crate::language::Context<'_>,
            _: crate::language::Line<'_>,
            _: u32,
            _: (&dyn Halt, Option<&dyn Halt>),
        ) -> Option<(crate::limits::Finished, Option<crate::language::Stopped>)> {
            ASKED.with(|asked| asked.set(asked.get() + 1));
            None
        }

        /// The group's language lexes it from its start, as an embedded body
        /// on the group's stack.
        fn lex_nested(
            language: Language,
            cx: &mut crate::language::Context<'_>,
            line: crate::language::Line<'_>,
            base: u32,
        ) -> Option<crate::limits::Finished> {
            let state = language.start_embedded(cx.arena, base);
            Some(language.lex_embedded(cx, line, state))
        }
    }

    #[derive(Clone, Copy, Debug)]
    enum Name {
        Fixed(Scope),
        Group(&'static str, u8, &'static str),
    }

    impl Name {
        /// The code that keys a list of names: a fixed scope's, or none for a
        /// name a capture group spells, which is never cached.
        const fn code(self) -> u16 {
            match self {
                Name::Fixed(scope) => scope.code(),
                Name::Group(..) => u16::MAX,
            }
        }

        /// The empty list of names.
        const NO_SCOPES: Scopes<Name> = &crate::scope::Keyed::new([], []);
    }

    const AFTER: Scope = Scope::dynamic(1);

    impl Grammar for Named {
        const NO_SCOPES: Scopes<Name> = Name::NO_SCOPES;
        type Scope = Name;
        type Rule = Rule;
        type Pattern = Mark;
        type Nested = Lexed;
        #[cfg(feature = "_embedding")]
        type Extra = NamedKept;
        #[cfg(not(feature = "_embedding"))]
        type Extra = ();
        #[cfg(feature = "_embedding")]
        type Kept = NamedKept;
        #[cfg(not(feature = "_embedding"))]
        type Kept = ();

        const NAME: &'static str = "named";
        const ROOT: Rule = Rule::Root;
        const SOURCE: Scope = AFTER;
        const GROUPS: usize = 4;

        fn scope(scope: Name) -> ScopeName {
            match scope {
                Name::Fixed(scope) => ScopeName::Fixed(scope),
                Name::Group(prefix, group, suffix) => ScopeName::Capture {
                    prefix,
                    group,
                    suffix,
                },
            }
        }

        /// A word's name lowers its text, as `${0:/downcase}` would.
        fn dyn_name_case(prefix: &'static str) -> NameCase {
            if prefix == "word." {
                NameCase::Lowered
            } else {
                NameCase::AsWritten
            }
        }

        fn rule_code(rule: Rule) -> u16 {
            match rule {
                Rule::Root => 0,
                Rule::Quote => 1,
                Rule::Inner => 2,
                Rule::Held => 3,
                Rule::Mixed => 4,
            }
        }

        fn rule_from_code(code: u16) -> Rule {
            match code {
                1 => Rule::Quote,
                2 => Rule::Inner,
                3 => Rule::Held,
                4 => Rule::Mixed,
                _ => Rule::Root,
            }
        }

        fn rule_name(_: Rule) -> Scopes<Name> {
            Name::NO_SCOPES
        }

        fn rule_content(_: Rule) -> Scopes<Name> {
            Name::NO_SCOPES
        }

        fn rule_payload(rule: Rule) -> Payload {
            match rule {
                Rule::Root => Payload::None,
                Rule::Quote => Payload::Backref(1),
                Rule::Inner => inner_payload(),
                Rule::Held => held_payload(),
                Rule::Mixed => mixed_payload(),
            }
        }

        fn rule_name_delimiter(rule: Rule) -> Option<&'static str> {
            match rule {
                Rule::Root | Rule::Inner | Rule::Held | Rule::Mixed => None,
                Rule::Quote => Some("quote."),
            }
        }

        fn rule_name_suffix(rule: Rule) -> &'static str {
            match rule {
                Rule::Root | Rule::Inner | Rule::Held | Rule::Mixed => "",
                Rule::Quote => ".q",
            }
        }

        fn rule_name_after(rule: Rule) -> Scopes<Name> {
            match rule {
                Rule::Root | Rule::Inner | Rule::Held | Rule::Mixed => Name::NO_SCOPES,
                Rule::Quote => scopes![Name::Fixed(AFTER)],
            }
        }

        #[cfg(all(feature = "_embedding", feature = "lang-rust"))]
        fn embed_start(rule: Rule) -> Option<fn(&mut Arena, u32) -> u32> {
            match rule {
                Rule::Inner => Some(inner_start),
                Rule::Root | Rule::Quote | Rule::Held | Rule::Mixed => None,
            }
        }

        fn pattern_act(pattern: Mark) -> Act<Self> {
            match pattern {
                Mark::Word => Act::Match(scopes![Name::Group("word.", 0, ".test")]),
                Mark::Look | Mark::Call => Act::Match(Name::NO_SCOPES),
                Mark::Open => Act::Begin(Rule::Quote),
                Mark::Embed => Act::Begin(Rule::Inner),
                Mark::Hold => Act::Begin(Rule::Held),
                Mark::Mix => Act::Begin(Rule::Mixed),
                Mark::Anchored => Act::Match(scopes![Name::Fixed(AFTER)]),
                Mark::Close => Act::End,
            }
        }

        fn pattern_captures(pattern: Mark) -> Captures<Self> {
            match pattern {
                Mark::Look => (
                    &[
                        (1, scopes![Name::Group("mark.", 1, "")]),
                        (2, scopes![Name::Fixed(AFTER)]),
                        (3, scopes![Name::Fixed(AFTER)]),
                    ],
                    &[],
                ),
                Mark::Call => (&[], &[(1, Lexed::Script)]),
                Mark::Word
                | Mark::Open
                | Mark::Close
                | Mark::Embed
                | Mark::Hold
                | Mark::Mix
                | Mark::Anchored => (&[], &[]),
            }
        }

        fn scan(lexer: &mut Lexer<'_, Self>) -> Option<(usize, Mark, usize)> {
            let patterns: &[Mark] = match lexer.top_rule() {
                Rule::Root => &[
                    Mark::Open,
                    Mark::Embed,
                    Mark::Hold,
                    Mark::Mix,
                    Mark::Call,
                    Mark::Look,
                    Mark::Word,
                ],
                Rule::Quote => &[Mark::Close, Mark::Mix, Mark::Word],
                Rule::Inner => &[],
                Rule::Held => &[Mark::Anchored],
                Rule::Mixed => &[Mark::Look],
            };
            for p in lexer.pos..lexer.lim() {
                for &pattern in patterns {
                    if let Some(end) = lexer.try_pattern(pattern, p) {
                        return Some((p, pattern, end));
                    }
                }
            }
            None
        }

        fn matcher(lexer: &mut Lexer<'_, Self>, pattern: Mark, p: usize) -> Option<usize> {
            match pattern {
                Mark::Word => {
                    let end = lexer.skip(p, |b| b.is_ascii_alphabetic() || !b.is_ascii());
                    (end > p).then_some(end)
                }
                Mark::Look => {
                    if !matches!(lexer.at(p), b'!' | b'?') {
                        return None;
                    }
                    lexer.capture(1, p, p + 1);
                    lexer.capture(2, p + 2, p + 3);
                    lexer.capture(3, p, p + 1);
                    Some(p + 1)
                }
                Mark::Open => {
                    if !lexer.starts(p, b"<<") {
                        return None;
                    }
                    let end = lexer.skip(p + 2, |b| b.is_ascii_alphabetic());
                    lexer.capture(1, p + 2, end);
                    Some(end)
                }
                Mark::Close => lexer.payload_at(p).map(|width| p + width),
                Mark::Embed => lexer
                    .starts(p, b">>")
                    .then(|| p + 2 + usize::from(p + 2 == lexer.lim())),
                Mark::Hold => lexer.starts(p, b"%%").then_some(p + 2),
                Mark::Mix => lexer.starts(p, b"##").then_some(p + 2),
                Mark::Anchored => (lexer.at_g(p) && lexer.at(p) == b'!').then_some(p + 1),
                Mark::Call => {
                    if !lexer.starts(p, b"@@") {
                        return None;
                    }
                    let end = lexer.skip(p + 2, |b| b != b' ');
                    lexer.capture(1, p + 2, end);
                    Some(end)
                }
            }
        }

        /// A group of [`Lexed::Script`] is deferred, so this runs only in a
        /// build without the embedding engine.
        fn tokenize_nested(
            lexer: &mut Lexer<'_, Self>,
            nested: Lexed,
            _: usize,
            to: usize,
            base: u32,
        ) {
            match nested {
                Lexed::Script => lexer.emit(to, base),
            }
        }

        #[cfg(feature = "_embedding")]
        const DEFERS_NESTED: bool = true;

        #[cfg(feature = "_embedding")]
        fn extra(kept: Self::Kept) -> Self::Extra {
            kept
        }

        #[cfg(feature = "_embedding")]
        fn keep(extra: Self::Extra) -> Self::Kept {
            extra
        }

        #[cfg(feature = "_embedding")]
        fn deferred(extra: &mut Self::Extra) -> Option<&mut Option<Box<Deferrals>>> {
            Some(&mut extra.deferrals)
        }

        #[cfg(feature = "_embedding")]
        fn deferred_kept(kept: &mut Self::Kept) -> Option<&mut Option<Box<Deferrals>>> {
            Some(&mut kept.deferrals)
        }

        #[cfg(feature = "_embedding")]
        fn defers_nested(nested: Lexed) -> Option<Language> {
            match nested {
                #[cfg(feature = "lang-javascript")]
                Lexed::Script => Some(Language::JavaScript),
                #[cfg(not(feature = "lang-javascript"))]
                Lexed::Script => None,
            }
        }
    }

    /// Lexes `lines` with [`Named`] from its start state, as a language's
    /// `start` makes it.
    ///
    /// It gives each token's end and scopes, joined by spaces, line by line.
    /// Each line starts from a new scratch, so it opens the rules of its state
    /// afresh.
    fn named_tokens(arena: &mut Arena, lines: &[&[u8]]) -> Vec<Vec<(usize, String)>> {
        let limits = Limits::default();
        let mut state = arena
            .push(ROOT, Named::rule_code(Rule::Root), 0, ROOT)
            .unwrap_or(ROOT);
        let mut rows = Vec::new();
        for &bytes in lines {
            let mut scratch = Scratch::<Named>::default();
            let mut tokens = Vec::new();
            let mut report = LineReport::default();
            let text = Text {
                bytes,
                start: 0,
                anchored: false,
                anchored_at_root: false,
            };
            let parts = Parts {
                arena: &mut *arena,
                limits: &limits,
                out: &mut tokens,
                report: &mut report,
            };
            state = Lexer::new(parts, text, state, &mut scratch).run().state();
            rows.push(
                tokens
                    .iter()
                    .map(|token| {
                        let mut names: Vec<&str> = arena.scopes(token.stack().0).collect();
                        names.reverse();
                        (token.end(), names.join(" "))
                    })
                    .collect(),
            );
        }
        rows
    }

    #[test]
    fn a_name_repeats_the_text_of_its_group_and_the_whole_match() {
        let mut arena = Arena::new(None);
        assert_eq!(arena.named_scope("after.test"), Some(AFTER));
        let rows = named_tokens(&mut arena, &[b"ab ! cd ?"]);
        let named: Vec<&str> = rows[0].iter().map(|(_, names)| names.as_str()).collect();
        assert_eq!(
            named,
            [
                "word.ab.test",
                "",
                "mark.!",
                "",
                "word.cd.test",
                "",
                "mark.?"
            ]
        );
    }

    #[test]
    fn a_lowered_name_folds_its_group_and_no_other_name() {
        let mut arena = Arena::new(None);
        assert_eq!(arena.named_scope("after.test"), Some(AFTER));
        let rows = named_tokens(&mut arena, &[b"<<AB Xy", b"zz AB"]);
        assert_eq!(rows[0][0], (4, "quote.AB.q after.test".to_owned()));
        assert_eq!(
            rows[0][2],
            (7, "quote.AB.q after.test word.xy.test".to_owned())
        );
        assert_eq!(
            rows[1][0],
            (2, "quote.AB.q after.test word.zz.test".to_owned())
        );
    }

    /// JavaScript's `toLowerCase` lowers a `Σ` that ends a word to `ς`, and
    /// any other `Σ` to `σ`.
    /// No grammar the crate ships lowers a text that can hold a `Σ`.
    /// `Named` lowers its `word.` names, and its words take any letter above
    /// ASCII.
    #[test]
    fn a_lowered_name_ends_its_word_with_a_final_sigma() {
        let mut arena = Arena::new(None);
        let rows = named_tokens(&mut arena, &["ΣΑΣ".as_bytes()]);
        assert_eq!(rows[0], [(6, "word.σας.test".to_owned())]);
    }

    #[test]
    fn a_group_past_the_match_ends_its_groups() {
        let mut arena = Arena::new(None);
        assert_eq!(arena.named_scope("after.test"), Some(AFTER));
        let rows = named_tokens(&mut arena, &[b"!"]);
        assert_eq!(rows[0], [(1, "mark.!".to_owned())]);
    }

    #[test]
    fn a_rule_named_from_its_payload_keeps_the_name_on_the_next_line() {
        let mut arena = Arena::new(None);
        assert_eq!(arena.named_scope("after.test"), Some(AFTER));
        let rows = named_tokens(&mut arena, &[b"<<ab xy", b"zz ab"]);
        assert_eq!(rows[0][0], (4, "quote.ab.q after.test".to_owned()));
        assert_eq!(
            rows[0][2],
            (7, "quote.ab.q after.test word.xy.test".to_owned())
        );
        assert_eq!(
            rows[1][0],
            (2, "quote.ab.q after.test word.zz.test".to_owned())
        );
        assert_eq!(rows[1][2], (5, "quote.ab.q after.test".to_owned()));
    }

    #[cfg(all(feature = "_embedding", feature = "lang-rust"))]
    #[test]
    fn a_rule_that_embeds_from_an_entry_stops_on_its_frame() {
        let mut arena = Arena::new(None);
        let limits = Limits::default();
        let state = arena
            .push(ROOT, Named::rule_code(Rule::Root), 0, ROOT)
            .unwrap_or(ROOT);
        let mut scratch = Scratch::<Named>::default();
        let mut tokens = Vec::new();
        let mut report = LineReport::default();
        let text = Text {
            bytes: b">>fn",
            start: 0,
            anchored: false,
            anchored_at_root: false,
        };
        let parts = Parts {
            arena: &mut arena,
            limits: &limits,
            out: &mut tokens,
            report: &mut report,
        };
        let (finished, embedding) = Lexer::new(parts, text, state, &mut scratch).run_embedding();
        let embedding = embedding.expect("the line stops where Rust starts");
        assert_eq!(embedding.at, 2);
        assert_eq!(embedding.language, crate::Language::Rust);
        let rule = Named::rule_from_code(arena.frame(finished.state()).rule);
        assert_eq!(Named::rule_code(rule), Named::rule_code(Rule::Inner));
        let start = Named::embed_start(rule).expect("the entry's start function");
        let expected: fn(&mut Arena, u32) -> u32 = inner_start;
        assert!(std::ptr::fn_addr_eq(start, expected));
    }

    /// [`Named`]'s buffers for [`crate::rules::lex_embedding`], in the crate
    /// scratch's slot for a test grammar.
    /// The groups a run defers stay there until the engine lexes them.
    #[cfg(all(
        feature = "_embedding",
        any(
            feature = "lang-javascript",
            feature = "lang-html",
            feature = "lang-rust"
        )
    ))]
    fn named_buffers(scratch: &mut crate::scratch::Scratch) -> &mut Scratch<Named> {
        scratch
            .test_grammar
            .get_or_insert_with(|| Box::new(Scratch::<Named>::default()))
            .downcast_mut()
            .expect("the slot holds Named's buffers")
    }

    /// `@@1 x @@2` holds two groups that JavaScript lexes again, deferred to
    /// where the line hands control back.
    /// Each group's tokens take its placeholder's place, the second group's
    /// first, so the tokens still end in order and cover the line.
    /// Inside each group the digit is JavaScript's number.
    #[cfg(all(feature = "_embedding", feature = "lang-javascript"))]
    #[test]
    fn deferred_groups_take_the_tokens_of_their_language() {
        let mut arena = Arena::new(None);
        let state = arena
            .push(ROOT, Named::rule_code(Rule::Root), 0, ROOT)
            .unwrap_or(ROOT);
        let limits = Limits::default();
        let mut scratch = crate::scratch::Scratch::default();
        scratch.start_line();
        let mut out = Vec::new();
        let mut report = LineReport::default();
        let mut cx = crate::language::Context {
            scratch: &mut scratch,
            arena: &mut arena,
            limits: &limits,
            out: &mut out,
            report: &mut report,
            first_token: 0,
        };
        let line = crate::language::Line {
            text: "@@1 x @@2",
            start: 0,
            embedded: false,
            anchored_at_root: false,
        };
        crate::rules::lex_embedding::<Named>(&mut cx, line, state, named_buffers).state();
        let ends: Vec<usize> = out.iter().map(|token| token.end()).collect();
        assert!(ends.windows(2).all(|pair| pair[0] < pair[1]), "{ends:?}");
        assert_eq!(ends.last(), Some(&9), "{ends:?}");
        assert!(report.is_complete());
        for end in [3, 9] {
            let digit = out
                .iter()
                .find(|token| token.end() == end)
                .expect("a token ends after each digit");
            let names: Vec<&str> = arena.scopes(digit.stack().0).collect();
            assert!(names.contains(&"constant.numeric.decimal.js"), "{names:?}");
        }
    }

    /// Lexes the line `bytes` of [`Named`] from `state`, which must stop for
    /// JavaScript, and returns the state and where it stopped.
    #[cfg(all(feature = "_embedding", feature = "lang-javascript"))]
    fn javascript_yield(arena: &mut Arena, bytes: &'static [u8], state: u32) -> (u32, Embedding) {
        let limits = Limits::default();
        let mut scratch = Scratch::<Named>::default();
        let mut tokens = Vec::new();
        let mut report = LineReport::default();
        let text = Text {
            bytes,
            start: 0,
            anchored: false,
            anchored_at_root: false,
        };
        let parts = Parts {
            arena,
            limits: &limits,
            out: &mut tokens,
            report: &mut report,
        };
        let (finished, embedding) = Lexer::new(parts, text, state, &mut scratch).run_embedding();
        let embedding = embedding.expect("the line stops where JavaScript goes on");
        assert_eq!(embedding.language, crate::Language::JavaScript);
        (finished.state(), embedding)
    }

    /// `%%` opens a rule that embeds JavaScript at the end of its line.
    /// That line stops with `\G` holding where JavaScript starts.
    /// The next line stops at its start, where no match has ended, so `\G`
    /// does not hold there.
    #[cfg(all(feature = "_embedding", feature = "lang-javascript"))]
    #[test]
    fn a_later_line_stops_for_its_language_without_the_anchor() {
        let mut arena = Arena::new(None);
        let root = arena
            .push(ROOT, Named::rule_code(Rule::Root), 0, ROOT)
            .unwrap_or(ROOT);
        let (state, first) = javascript_yield(&mut arena, b"%%", root);
        assert_eq!((first.at, first.anchored), (2, true));
        let (_, later) = javascript_yield(&mut arena, b"!x", state);
        assert_eq!((later.at, later.anchored), (0, false));
    }

    /// JavaScript halts before it lexes anything after `%%`.
    /// So no match has ended since the one that opened the rule, and `\G`
    /// still holds where the grammar goes on: the `!` is named.
    /// The engine asks [`Named`] first ([`Embeds::lex_embedded_until`]).
    /// It declines, and JavaScript's own `lex_until` halts.
    #[cfg(all(feature = "_embedding", feature = "lang-javascript"))]
    #[test]
    fn a_language_that_halts_at_once_leaves_the_anchor_where_it_was() {
        let mut arena = Arena::new(None);
        assert_eq!(arena.named_scope("after.test"), Some(AFTER));
        let state = arena
            .push(ROOT, Named::rule_code(Rule::Root), 0, ROOT)
            .unwrap_or(ROOT);
        let limits = Limits::default();
        let mut scratch = crate::scratch::Scratch::default();
        scratch.start_line();
        let mut out = Vec::new();
        let mut report = LineReport::default();
        let mut cx = crate::language::Context {
            scratch: &mut scratch,
            arena: &mut arena,
            limits: &limits,
            out: &mut out,
            report: &mut report,
            first_token: 0,
        };
        let line = crate::language::Line {
            text: "%%!x",
            start: 0,
            embedded: false,
            anchored_at_root: false,
        };
        ASKED.with(|asked| asked.set(0));
        crate::rules::lex_embedding::<Named>(&mut cx, line, state, named_buffers).state();
        assert_ne!(
            ASKED.with(std::cell::Cell::get),
            0,
            "the engine asked the grammar"
        );
        let bang = out
            .iter()
            .find(|token| token.end() == 3)
            .expect("a token ends after the `!`");
        let names: Vec<&str> = arena.scopes(bang.stack().0).collect();
        assert!(names.contains(&"after.test"), "{names:?}");
    }

    /// `%%` ends the line, so the hand-over leaves no text for JavaScript.
    /// [`Rule::Held`]'s end halt holds at the line's end.
    /// So the engine stops there without asking a language.
    /// It tries the grammar's own patterns at that place, as vscode-textmate
    /// tries a rule's end there.
    /// No pattern of the rule matches, and the line ends inside it.
    #[cfg(all(feature = "_embedding", feature = "lang-javascript"))]
    #[test]
    fn a_hand_over_that_leaves_no_text_stops_where_its_end_holds() {
        let mut arena = Arena::new(None);
        let state = arena
            .push(ROOT, Named::rule_code(Rule::Root), 0, ROOT)
            .unwrap_or(ROOT);
        let limits = Limits::default();
        let mut scratch = crate::scratch::Scratch::default();
        scratch.start_line();
        let mut out = Vec::new();
        let mut report = LineReport::default();
        let mut cx = crate::language::Context {
            scratch: &mut scratch,
            arena: &mut arena,
            limits: &limits,
            out: &mut out,
            report: &mut report,
            first_token: 0,
        };
        let line = crate::language::Line {
            text: "a %%",
            start: 0,
            embedded: false,
            anchored_at_root: false,
        };
        ASKED.with(|asked| asked.set(0));
        let next =
            crate::rules::lex_embedding::<Named>(&mut cx, line, state, named_buffers).state();
        assert_eq!(ASKED.with(std::cell::Cell::get), 0, "no language was asked");
        assert_eq!(
            Named::rule_from_code(arena.frame(next).rule),
            Rule::Held,
            "the line ends inside the rule"
        );
        assert_eq!(out.last().map(|token| token.end()), Some(4));
    }

    /// An end that reads its parent's text is asked at character starts only.
    /// In `aé`, the end holds at byte 2, inside `é`, and is never found there.
    /// It is found at byte 1, where `é` starts.
    #[cfg(feature = "_embedding")]
    #[test]
    fn an_end_that_reads_parent_text_holds_at_character_starts_only() {
        let s = "aé".as_bytes();
        let inside = crate::rules::ParentTextEnd::<Named> {
            end: &AtOnce { at: 2 },
            parent_text: b"",
        };
        assert_eq!(inside.first(s, 0, s.len()), None);
        let start = crate::rules::ParentTextEnd::<Named> {
            end: &AtOnce { at: 1 },
            parent_text: b"",
        };
        assert_eq!(start.first(s, 0, s.len()), Some(1));
    }

    /// `>>` ends the first line and takes its line feed, so no text is left
    /// for Rust there.
    /// The line ends inside the rule, and Rust lexes the second line.
    #[cfg(all(feature = "_embedding", feature = "lang-rust"))]
    #[test]
    fn a_rule_that_takes_the_line_feed_starts_its_language_on_the_next_line() {
        let mut arena = Arena::new(None);
        let mut state = arena
            .push(ROOT, Named::rule_code(Rule::Root), 0, ROOT)
            .unwrap_or(ROOT);
        let limits = Limits::default();
        let mut scratch = crate::scratch::Scratch::default();
        let mut out = Vec::new();
        let mut report = LineReport::default();
        for text in [">>", "fn"] {
            scratch.start_line();
            out.clear();
            let mut cx = crate::language::Context {
                scratch: &mut scratch,
                arena: &mut arena,
                limits: &limits,
                out: &mut out,
                report: &mut report,
                first_token: 0,
            };
            let line = crate::language::Line {
                text,
                start: 0,
                embedded: false,
                anchored_at_root: false,
            };
            state =
                crate::rules::lex_embedding::<Named>(&mut cx, line, state, named_buffers).state();
        }
        let rule = Named::rule_from_code(arena.frame(state).rule);
        assert_eq!(Named::rule_code(rule), Named::rule_code(Rule::Inner));
        let names: Vec<&str> = arena.scopes(out[0].stack().0).collect();
        assert!(names.iter().any(|name| name.contains(".rust")), "{names:?}");
    }

    /// `<<ab` opens a quote whose payload is `ab`, and `##` inside it opens a
    /// rule that embeds HTML.
    /// The engine hands HTML's end halt that text: the payload of the
    /// embedding rule's parent ([`EndText::ParentPayload`]).
    #[cfg(all(feature = "_embedding", feature = "lang-html"))]
    #[test]
    fn an_end_halt_reads_the_parent_payload_text() {
        let mut arena = Arena::new(None);
        let state = arena
            .push(ROOT, Named::rule_code(Rule::Root), 0, ROOT)
            .unwrap_or(ROOT);
        let limits = Limits::default();
        let mut scratch = crate::scratch::Scratch::default();
        scratch.start_line();
        let mut out = Vec::new();
        let mut report = LineReport::default();
        let mut cx = crate::language::Context {
            scratch: &mut scratch,
            arena: &mut arena,
            limits: &limits,
            out: &mut out,
            report: &mut report,
            first_token: 0,
        };
        let line = crate::language::Line {
            text: "<<ab ##x y",
            start: 0,
            embedded: false,
            anchored_at_root: false,
        };
        PARENT_TEXTS.with_borrow_mut(Vec::clear);
        crate::rules::lex_embedding::<Named>(&mut cx, line, state, named_buffers).state();
        let texts = PARENT_TEXTS.with_borrow(Clone::clone);
        assert!(!texts.is_empty(), "the engine asked the end halt");
        assert!(texts.iter().all(|text| text == b"ab"), "{texts:?}");
        assert_eq!(
            named_buffers(&mut scratch)
                .parent_text()
                .map(|buffer| buffer.as_slice()),
            Some(&b"ab"[..]),
            "the grammar keeps the buffer"
        );
    }

    /// After `##`, HTML lexes `a ` as text and matches nothing at the `!`.
    /// The rule lists its mark of `!` after HTML, so HTML halts there, and
    /// the grammar names the `!`.
    #[cfg(all(feature = "_embedding", feature = "lang-html"))]
    #[test]
    fn a_rule_that_lists_its_patterns_after_the_language_takes_them_back() {
        let mut arena = Arena::new(None);
        let state = arena
            .push(ROOT, Named::rule_code(Rule::Root), 0, ROOT)
            .unwrap_or(ROOT);
        let limits = Limits::default();
        let mut scratch = crate::scratch::Scratch::default();
        scratch.start_line();
        let mut out = Vec::new();
        let mut report = LineReport::default();
        let mut cx = crate::language::Context {
            scratch: &mut scratch,
            arena: &mut arena,
            limits: &limits,
            out: &mut out,
            report: &mut report,
            first_token: 0,
        };
        let line = crate::language::Line {
            text: "##a ! b",
            start: 0,
            embedded: false,
            anchored_at_root: false,
        };
        crate::rules::lex_embedding::<Named>(&mut cx, line, state, named_buffers).state();
        let bang = out
            .iter()
            .find(|token| token.end() == 5)
            .expect("a token ends after the `!`");
        let names: Vec<&str> = arena.scopes(bang.stack().0).collect();
        assert!(names.contains(&"mark.!"), "{names:?}");
        assert_eq!(scratch.rule_starts, [4]);
    }

    /// The depth halts of [`Grammar::DEPTH_HALTS`].
    #[cfg(feature = "_embedding")]
    #[cfg(feature = "_rule-tables")]
    mod depth_halts {
        use super::{
            Act, Arena, Captures, Grammar, Halt, Inside, Lexer, Limits, LineReport, Parts, ROOT,
            Scope, ScopeName, Scopes, Scratch, Text, WORD, scopes,
        };

        /// A grammar of words, `"` strings with `@` mentions, `#` line comments
        /// and `[` `]` groups that shut the deep halt out, on the table scan,
        /// for [`Grammar::DEPTH_HALTS`].
        /// `DEEP` sets the const, so one grammar is its own control.
        ///
        /// - A group also opens and ends at a `!`, without advancing.
        /// - A `<` `>` match lexes its inside again as a group.
        #[derive(Clone, Copy, Debug)]
        struct Strings<const DEEP: bool>;

        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        enum DeepRule {
            Root,
            Str,
            Shut,
            Comment,
        }

        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        enum DeepPattern {
            Quote,
            QuoteEnd,
            Open,
            Close,
            Angle,
            Hash,
            LineFeed,
            Mention,
            Word,
        }

        impl<const DEEP: bool> Grammar for Strings<DEEP> {
            type Scope = Scope;
            const NO_SCOPES: Scopes<Scope> = Scope::NO_SCOPES;
            type Rule = DeepRule;
            type Pattern = DeepPattern;
            type Nested = Inside;
            type Extra = Option<usize>;
            type Kept = Option<usize>;

            const NAME: &'static str = "strings";
            const ROOT: DeepRule = DeepRule::Root;
            const SOURCE: Scope = WORD;
            const DEPTH_HALTS: bool = DEEP;
            const WATCHES_FRAMES: bool = DEEP;

            fn scope(scope: Scope) -> ScopeName {
                ScopeName::Fixed(scope)
            }

            fn extra(kept: Option<usize>) -> Option<usize> {
                kept
            }

            fn keep(extra: Option<usize>) -> Option<usize> {
                extra
            }

            fn rule_code(rule: DeepRule) -> u16 {
                match rule {
                    DeepRule::Root => 0,
                    DeepRule::Str => 1,
                    DeepRule::Shut => 2,
                    DeepRule::Comment => 3,
                }
            }

            fn rule_from_code(code: u16) -> DeepRule {
                match code {
                    1 => DeepRule::Str,
                    2 => DeepRule::Shut,
                    3 => DeepRule::Comment,
                    _ => DeepRule::Root,
                }
            }

            fn rule_name(_: DeepRule) -> Scopes<Scope> {
                Scope::NO_SCOPES
            }

            fn rule_content(_: DeepRule) -> Scopes<Scope> {
                Scope::NO_SCOPES
            }

            fn pattern_act(pattern: DeepPattern) -> Act<Self> {
                match pattern {
                    DeepPattern::Quote => Act::Begin(DeepRule::Str),
                    DeepPattern::Open => Act::Begin(DeepRule::Shut),
                    DeepPattern::Hash => Act::Begin(DeepRule::Comment),
                    DeepPattern::QuoteEnd | DeepPattern::Close | DeepPattern::LineFeed => Act::End,
                    DeepPattern::Angle | DeepPattern::Mention | DeepPattern::Word => {
                        Act::Match(scopes![WORD])
                    }
                }
            }

            fn pattern_captures(pattern: DeepPattern) -> Captures<Self> {
                match pattern {
                    DeepPattern::Angle => (&[], &[(1, Inside::Again)]),
                    DeepPattern::Quote
                    | DeepPattern::QuoteEnd
                    | DeepPattern::Open
                    | DeepPattern::Close
                    | DeepPattern::Hash
                    | DeepPattern::LineFeed
                    | DeepPattern::Mention
                    | DeepPattern::Word => (&[], &[]),
                }
            }

            fn scan(lexer: &mut Lexer<'_, Self>) -> Option<(usize, DeepPattern, usize)> {
                crate::rules::table_scan(lexer)
            }

            fn matcher(
                lexer: &mut Lexer<'_, Self>,
                pattern: DeepPattern,
                p: usize,
            ) -> Option<usize> {
                match pattern {
                    DeepPattern::Quote | DeepPattern::QuoteEnd => {
                        (lexer.at(p) == b'"').then_some(p + 1)
                    }
                    DeepPattern::Open => match lexer.at(p) {
                        b'[' => Some(p + 1),
                        b'!' => Some(p),
                        _ => None,
                    },
                    DeepPattern::Close => match lexer.at(p) {
                        b']' => Some(p + 1),
                        b'!' => Some(p),
                        _ => None,
                    },
                    DeepPattern::Angle => {
                        let close = lexer.skip(p + 1, |b| b != b'>');
                        if lexer.at(p) != b'<' || lexer.at(close) != b'>' {
                            return None;
                        }
                        lexer.capture(1, p + 1, close);
                        Some(close + 1)
                    }
                    DeepPattern::Hash => (lexer.at(p) == b'#').then_some(p + 1),
                    DeepPattern::LineFeed => (p == lexer.lim() && lexer.avail(p)).then_some(p + 1),
                    DeepPattern::Mention => {
                        let end = lexer.skip(p + 1, |b| b.is_ascii_alphabetic());
                        (lexer.at(p) == b'@' && end > p + 1).then_some(end)
                    }
                    DeepPattern::Word => {
                        let end = lexer.skip(p, |b| b.is_ascii_alphabetic());
                        (end > p).then_some(end)
                    }
                }
            }

            fn tokenize_nested(
                lexer: &mut Lexer<'_, Self>,
                nested: Inside,
                from: usize,
                to: usize,
                base: u32,
            ) {
                match nested {
                    Inside::Again => lexer.retokenise(DeepRule::Shut, from, to, base),
                }
            }

            fn frames_opened(lexer: &mut Lexer<'_, Self>) {
                lexer.x = lexer
                    .frames
                    .iter()
                    .position(|frame| frame.rule == DeepRule::Shut);
            }

            fn frame_pushed(lexer: &mut Lexer<'_, Self>) {
                if lexer.x.is_none() && lexer.top_rule() == DeepRule::Shut {
                    lexer.x = Some(lexer.frames.len() - 1);
                }
            }

            fn frames_popped(lexer: &mut Lexer<'_, Self>) {
                if lexer.x.is_some_and(|at| at >= lexer.frames.len()) {
                    lexer.x = None;
                }
            }

            fn deep_halts_open(lexer: &Lexer<'_, Self>) -> bool {
                lexer.x.is_none()
            }
        }

        impl<const DEEP: bool> crate::rules::Tabled for Strings<DEEP> {
            const WORDS_AT_START: bool = false;

            fn pattern_start(_: DeepPattern) -> crate::rules::Start {
                crate::rules::Start {
                    bytes: [u64::MAX; 4],
                    places: crate::rules::PlaceSet::of(&[
                        crate::rules::Place::Away,
                        crate::rules::Place::LineStart,
                        crate::rules::Place::InWord,
                    ]),
                    matches_at_end: true,
                }
            }

            fn tables(rule: DeepRule) -> &'static crate::rules::Tables<Self> {
                let (end, patterns): (Option<DeepPattern>, &'static [DeepPattern]) = match rule {
                    DeepRule::Root => (
                        None,
                        &[
                            DeepPattern::Quote,
                            DeepPattern::Open,
                            DeepPattern::Angle,
                            DeepPattern::Hash,
                            DeepPattern::Word,
                        ],
                    ),
                    DeepRule::Str => (
                        Some(DeepPattern::QuoteEnd),
                        &[DeepPattern::Mention, DeepPattern::Word],
                    ),
                    DeepRule::Shut => (
                        Some(DeepPattern::Close),
                        &[DeepPattern::Quote, DeepPattern::Word],
                    ),
                    DeepRule::Comment => (Some(DeepPattern::LineFeed), &[DeepPattern::Word]),
                };
                Box::leak(Box::new(crate::rules::Tables::build(
                    crate::rules::RuleDefinition {
                        name: Scope::NO_SCOPES,
                        content: Scope::NO_SCOPES,
                        end,
                        backref: None,
                        patterns,
                    },
                )))
            }
        }

        /// The end of the rule that embeds [`Strings`]: a `}` or an `@` at its
        /// root, and an `@` at any depth.
        struct BladeLike;

        /// The deep half of [`BladeLike`]: an `@`.
        struct AtSign;

        impl Halt for AtSign {
            fn first(&self, s: &[u8], from: usize, to: usize) -> Option<usize> {
                (from..=to).find(|&p| s.get(p) == Some(&b'@'))
            }
        }

        impl Halt for BladeLike {
            fn first(&self, s: &[u8], from: usize, to: usize) -> Option<usize> {
                (from..=to).find(|&p| matches!(s.get(p), Some(b'}' | b'@')))
            }

            fn deep(&self) -> Option<&dyn Halt> {
                Some(&AtSign)
            }
        }

        /// [`BladeLike`] whose deep half loses a tie with the patterns, as an
        /// injection without `L:` does.
        struct Losing;

        impl Halt for Losing {
            fn first(&self, s: &[u8], from: usize, to: usize) -> Option<usize> {
                BladeLike.first(s, from, to)
            }

            fn deep(&self) -> Option<&dyn Halt> {
                Some(&AtSign)
            }

            fn deep_loses_ties(&self) -> bool {
                true
            }
        }

        /// Where a line of [`Strings`] halts under [`BladeLike`], and how many
        /// rules are open there.
        fn deep_stop<const DEEP: bool>(line: &[u8]) -> (Option<usize>, u32) {
            let (stop, state, arena) =
                lex_from::<DEEP>(line, 0, ROOT, &BladeLike, Arena::new(None));
            (stop, arena.frame(state).depth)
        }

        /// Lexes `line` of [`Strings`] from `start` and `state` under `halt`,
        /// and gives where it halted, the state after it and the arena.
        fn lex_from<const DEEP: bool>(
            line: &[u8],
            start: usize,
            state: u32,
            halt: &dyn Halt,
            mut arena: Arena,
        ) -> (Option<usize>, u32, Arena) {
            let limits = Limits::default();
            let mut tokens = Vec::new();
            let mut report = LineReport::default();
            let mut scratch = Scratch::<Strings<DEEP>>::default();
            let parts = Parts {
                arena: &mut arena,
                limits: &limits,
                out: &mut tokens,
                report: &mut report,
            };
            let text = Text {
                bytes: line,
                start,
                anchored: false,
                anchored_at_root: false,
            };
            let halts: crate::language::Halts<'_> = (halt, None);
            let (finished, stopped) = Lexer::new(parts, text, state, &mut scratch)
                .halting(&halts)
                .run_halting();
            (stopped.map(|stop| stop.at), finished.state(), arena)
        }

        /// Lexes `lines` of [`Strings`] in turn under `halt`, on one scratch,
        /// so a line resumes the frames the last one kept, and gives where each
        /// halted.
        fn lex_lines<const DEEP: bool>(lines: &[&[u8]], halt: &dyn Halt) -> Vec<Option<usize>> {
            let limits = Limits::default();
            let mut arena = Arena::new(None);
            let mut scratch = Scratch::<Strings<DEEP>>::default();
            let mut state = crate::rules::start_embedded::<Strings<DEEP>>(&mut arena, ROOT);
            let mut stops = Vec::new();
            for line in lines {
                let mut tokens = Vec::new();
                let mut report = LineReport::default();
                let parts = Parts {
                    arena: &mut arena,
                    limits: &limits,
                    out: &mut tokens,
                    report: &mut report,
                };
                let text = Text {
                    bytes: line,
                    start: 0,
                    anchored: false,
                    anchored_at_root: false,
                };
                let halts: crate::language::Halts<'_> = (halt, None);
                let (finished, stopped) = Lexer::new(parts, text, state, &mut scratch)
                    .halting(&halts)
                    .run_halting();
                stops.push(stopped.map(|stop| stop.at));
                state = finished.state();
            }
            stops
        }

        #[test]
        fn a_deep_halt_stops_the_line_inside_a_string() {
            assert_eq!(deep_stop::<true>(b"a \"b @c\" d"), (Some(5), 1));
            assert_eq!(deep_stop::<false>(b"a \"b @c\" d"), (None, 0));
        }

        #[test]
        fn a_rule_that_shuts_the_deep_halt_out_keeps_it_out_of_its_strings() {
            assert_eq!(deep_stop::<true>(b"[x \"@y\"] \"@z\""), (Some(10), 1));
        }

        #[test]
        fn a_deep_halt_that_loses_ties_gives_way_to_a_pattern_at_its_place() {
            let mention = lex_from::<true>(b"a \"b @c\" d", 0, ROOT, &Losing, Arena::new(None));
            assert_eq!(mention.0, None);
            let alone = lex_from::<true>(b"a \"b @ c\" d", 0, ROOT, &Losing, Arena::new(None));
            assert_eq!(alone.0, Some(5));
        }

        #[test]
        fn a_line_resumed_at_its_end_closes_an_open_line_comment() {
            let line = b"a #b @";
            let mut arena = Arena::new(None);
            let root = crate::rules::start_embedded::<Strings<true>>(&mut arena, ROOT);
            let (stop, state, arena) = lex_from::<true>(line, 0, root, &BladeLike, arena);
            let open = arena.frame(root).depth + 1;
            assert_eq!(stop, Some(5));
            assert_eq!(arena.frame(state).depth, open);
            let (stop, state, arena) = lex_from::<true>(line, line.len(), state, &BladeLike, arena);
            assert_eq!(stop, None);
            assert_eq!(arena.frame(state).depth, open - 1);
        }

        #[test]
        fn a_rule_that_closes_where_it_opened_still_shuts_the_deep_halt_out() {
            assert_eq!(
                lex_lines::<true>(&[b"!x", b"\"@y\""], &BladeLike),
                [None, None]
            );
        }

        #[test]
        fn a_capture_lexed_again_as_a_rule_that_shuts_the_deep_halt_out_keeps_it_out() {
            assert_eq!(deep_stop::<true>(b"<\"@y\"> z"), (None, 0));
        }

        #[test]
        fn the_halts_at_the_root_hold_with_or_without_depth_halts() {
            assert_eq!(deep_stop::<true>(b"a @b"), (Some(2), 0));
            assert_eq!(deep_stop::<false>(b"a @b"), (Some(2), 0));
        }
    }
}
