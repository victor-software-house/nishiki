//! The rule engine that the languages written from a `TextMate` grammar share:
//! Python, Java, CSS, Make, Kotlin, Go, Ruby, Swift and C run on it, and each
//! language supplies a [`Grammar`], the generated tables of its rules and
//! patterns, and the hand-written matchers that decide where each pattern
//! matches.
//!
//! The engine runs the grammar's rules the way a `TextMate` engine does, with
//! those matchers instead of regular expressions: the per-line loop, `\G`,
//! capture groups, `while` rules, the limits and the frames that carry a rule
//! from line to line are here, once.

mod chars;
mod lexer;
#[cfg(any(feature = "lang-go", feature = "lang-ruby", feature = "lang-lua"))]
mod runs;
mod tables;

use std::fmt::Debug;

#[cfg(any(
    feature = "lang-cmake",
    feature = "lang-python",
    feature = "lang-java",
    feature = "lang-css",
    feature = "lang-kotlin",
    feature = "lang-go",
    feature = "lang-ruby",
    feature = "lang-csharp",
    feature = "lang-dart",
    feature = "lang-scala",
    feature = "lang-swift",
    feature = "lang-cpp",
    feature = "lang-lua",
    feature = "lang-nix",
    feature = "lang-less",
    feature = "lang-powershell"
))]
pub(crate) use chars::Chars;
use lexer::FIRST_LINE;
#[cfg(feature = "_embedding")]
use lexer::with_payload;
pub(crate) use lexer::{Lexer, Scratch};
#[cfg(any(feature = "lang-go", feature = "lang-ruby", feature = "lang-lua"))]
pub(crate) use runs::{CachedRuns, HasRuns, Runs};
pub(crate) use tables::Tables;
#[cfg(feature = "_rule-tables")]
pub(crate) use tables::table_scan;

use crate::arena::{Arena, ROOT};
use crate::language::{Context, Halt, Line};
#[cfg(feature = "_embedding")]
use crate::language::{Halts, Parts};
use crate::limits::Finished;
#[cfg(feature = "_embedding")]
use crate::limits::{Limit, Steps};
use crate::scope::{Scope, Scopes};

/// One language's grammar: its rules and patterns, by type, and the
/// hand-written matchers that decide where each pattern matches.
pub(crate) trait Grammar: Sized + Copy + Debug + 'static {
    /// A scope name of the grammar.
    type Scope: Copy + Debug + 'static;
    /// The empty list of the grammar's scopes.
    const NO_SCOPES: Scopes<Self::Scope>;
    /// A rule of the grammar that can be open at the end of a line.
    type Rule: Copy + Debug + PartialEq + 'static;
    /// One regular expression of the grammar.
    type Pat: Copy + Debug + 'static;
    /// A kind of capture group that the grammar tokenizes with patterns of
    /// its own.
    type Nested: Copy + Debug + 'static;
    /// What the language's matchers remember about the line being lexed.
    type Extra: Default;
    /// What the matchers keep from one line to the next, such as a buffer
    /// whose capacity is worth keeping. The highlighter's scratch holds it,
    /// so it is `Send` and `Sync` as the highlighter is.
    type Kept: Default + Debug + Send + Sync;

    /// The language's name, for diagnostics.
    const NAME: &'static str;
    /// The rule that is open at the start of a document.
    const ROOT: Self::Rule;
    /// One more than the highest capture group a pattern of the grammar
    /// records. The groups are cleared before a pattern is tried after one
    /// that recorded some, so a grammar with few keeps that cheap.
    const GROUPS: usize = 12;
    /// Whether a begin match that takes the line feed leaves `\G` at the
    /// start of the next line, as vscode-textmate does.
    const CAPTURED_EOL: bool = false;
    /// Whether a pattern of the grammar reads `\A`, so that the state at the
    /// start of a document marks its first line.
    const MARKS_FIRST_LINE: bool = false;
    /// The scope that names a whole document, such as `source.python`.
    const SOURCE: Scope;
    /// Whether a matcher's span can lie inside one it recorded before for
    /// the same match, and so takes that span's scopes too, as nested
    /// capture groups do. Off, every span sits directly under the match.
    const NESTED_SPANS: bool = false;
    /// The longest payload that the name of a rule with a delimiter repeats.
    /// A longer one, which the begin pattern takes in another group than the
    /// one the name reads, as C++'s raw strings do with a delimiter of more
    /// than 16 bytes, leaves the delimiter out of the name.
    const NAMED_PAYLOAD: usize = usize::MAX;

    /// The scope in the crate's shared numbering, or the name it makes
    /// from a capture group's text.
    fn scope(scope: Self::Scope) -> ScopeName;
    /// The matchers' memory for a line, given what the last line kept.
    fn extra(kept: Self::Kept) -> Self::Extra {
        drop(kept);
        Self::Extra::default()
    }
    /// What a line's memory leaves for the next line.
    fn keep(extra: Self::Extra) -> Self::Kept {
        drop(extra);
        Self::Kept::default()
    }
    /// The rule as a frame stores it.
    fn rule_code(rule: Self::Rule) -> u16;
    /// The rule a frame's code names; [`Grammar::ROOT`] for a code that is
    /// not the language's.
    fn rule_from_code(code: u16) -> Self::Rule;
    /// The scopes of a rule's begin and end text and everything between.
    fn rule_name(rule: Self::Rule) -> Scopes<Self::Scope>;
    /// The scopes of what lies between a rule's begin and end text.
    fn rule_content(rule: Self::Rule) -> Scopes<Self::Scope>;
    /// The `while` pattern of a rule that has one: the rule stays open on a
    /// later line only when the pattern matches at its start.
    fn rule_while(_rule: Self::Rule) -> Option<Self::Pat> {
        None
    }
    /// Whether a rule of the grammar has a `while` pattern
    /// ([`Grammar::rule_while`]).
    /// The engine then keeps the indices of the open ones between lines.
    /// A line that continues the last tries only those frames.
    /// Without the flag, a line visits every frame.
    /// The flag only saves work: the tokens are the same either way.
    const WHILES: bool = false;
    /// Whether the loop tests at each step whether the innermost rule embeds
    /// another language ([`Payload::Embed`]). A rule that embeds hands its line
    /// over when it opens, without this test. The test is for a grammar whose
    /// line can come back into such a rule mid-line. Ruby needs it: an
    /// interpolation can close inside a heredoc, and an escape resumes it.
    const EMBEDS: bool = false;
    /// Whether the grammar lexes each line without its embedded languages first
    /// ([`lex_on_demand`]). Such a grammar has no per-step test.
    /// A line that starts inside a rule that embeds is noted where the rules
    /// are opened from the state, not on every line.
    #[cfg(feature = "_embedding")]
    const EMBEDS_ON_DEMAND: bool = false;
    /// What a rule carries from line to line.
    fn rule_payload(_rule: Self::Rule) -> Payload {
        Payload::None
    }
    /// The byte a match of `pat` must have right after its first, where the
    /// pattern fixes it. Such a pattern needs two bytes, so the tables leave
    /// it out where no byte stands (see [`Tables::all`]).
    fn pat_next(_pat: Self::Pat) -> Option<u8> {
        None
    }
    /// The prefix of a content scope that ends in the rule's payload text,
    /// for a rule whose content scope is named by it.
    fn rule_content_delimiter(_rule: Self::Rule) -> Option<&'static str> {
        None
    }
    /// The prefix of a name scope that ends in the rule's payload text, for
    /// a rule whose name repeats the begin capture group it carries; its
    /// [`Grammar::rule_name`] then leaves that scope out. The next line
    /// rebuilds the name from the payload, as it has no groups.
    fn rule_name_delimiter(_rule: Self::Rule) -> Option<&'static str> {
        None
    }
    /// What `pat` does when it matches.
    fn pat_act(pat: Self::Pat) -> Act<Self>;
    /// The scopes of the capture groups of `pat`, and the groups the grammar
    /// tokenizes with patterns of their own.
    fn pat_captures(pat: Self::Pat) -> Captures<Self>;
    /// Both answers about `pat` that the engine needs for each match; a
    /// grammar whose table holds them together answers with one lookup.
    fn pat_data(pat: Self::Pat) -> (Act<Self>, Captures<Self>) {
        (Self::pat_act(pat), Self::pat_captures(pat))
    }
    /// The first pattern of the innermost rule that matches at or after the
    /// cursor, with where its match starts and ends. At the first place where
    /// [`Lexer::root_halts`] holds it calls [`Lexer::halt_here`] and returns
    /// `None`. A language that keeps the candidate tables of [`Tabled`] calls
    /// [`table_scan`].
    fn scan(lexer: &mut Lexer<'_, Self>) -> Option<(usize, Self::Pat, usize)>;
    /// Whether `pat` matches at `p`, and where it ends.
    fn matcher(lexer: &mut Lexer<'_, Self>, pat: Self::Pat, p: usize) -> Option<usize>;
    /// Emits the tokens of the capture group text `from..to` that sits under
    /// the stack `base`, and leaves the cursor at `to`.
    fn tokenize_nested(
        lexer: &mut Lexer<'_, Self>,
        kind: Self::Nested,
        from: usize,
        to: usize,
        base: u32,
    );
    /// Whether no pattern of `rule` can match in `text`, so that lexing a
    /// span again with it can be skipped.
    fn skip_retok(_rule: Self::Rule, _text: &[u8]) -> bool {
        false
    }
    /// How a [`DynName`] with `prefix` spells the line's text: as written,
    /// unless the grammar's capture lowers it, as CSS's units do. Asked only
    /// where a dynamic name is built, so that [`Span`] stays its size.
    fn dyn_name_case(_prefix: &'static str) -> NameCase {
        NameCase::AsWritten
    }
}

/// The state at the start of a document of grammar `G` whose scopes sit on
/// the stack `base`: the root rule with [`Grammar::SOURCE`] on top of it.
pub(crate) fn start<G: Grammar>(arena: &mut Arena, base: u32) -> u32 {
    let stack = arena.child(base, G::SOURCE).unwrap_or(base);
    let payload = if G::MARKS_FIRST_LINE { FIRST_LINE } else { 0 };
    arena
        .push(ROOT, G::rule_code(G::ROOT), payload, stack)
        .unwrap_or(ROOT)
}

/// The state at the start of the lines of grammar `G` in a fenced code block,
/// whose text sits on the stack `base` with no source scope of its own, as in
/// Shiki's embedding.
pub(crate) fn start_embedded<G: Grammar>(arena: &mut Arena, base: u32) -> u32 {
    arena
        .push(ROOT, G::rule_code(G::ROOT), 0, base)
        .unwrap_or(ROOT)
}

/// Lexes `line` from `state` with grammar `G` under the crate's one lexer
/// contract ([`Language::lex`](crate::Language)), with the buffers `scratch`
/// picks out of the highlighter's, and returns the state after it.
///
/// An empty line leaves the state as it is, because Shiki does not tokenize
/// empty lines.
#[inline]
pub(crate) fn lex<G: Grammar>(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    scratch: fn(&mut crate::scratch::Scratch) -> &mut Scratch<G>,
) -> Finished {
    if line.text.is_empty() {
        return Finished::unspent(state);
    }
    let text = Text {
        bytes: line.text.as_bytes(),
        start: line.start,
        anchored: line.embedded,
    };
    let (parts, buffers) = cx.parts();
    Lexer::new(parts, text, state, scratch(buffers)).run()
}

/// Lexes `line` like [`lex`], for grammar `G` that another grammar embeds up
/// to its end: the line halts at the first place where `end` holds that the
/// scan reaches with only the root rule open, or where `after` holds there
/// and no pattern matches (see [`Halt`]), and the second value is where.
/// Without one, the state resumes on the next line.
#[inline]
pub(crate) fn lex_until<G: Grammar>(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    scratch: fn(&mut crate::scratch::Scratch) -> &mut Scratch<G>,
    end: &dyn Halt,
    after: Option<&dyn Halt>,
) -> (Finished, Option<usize>) {
    if line.text.len() <= line.start {
        return (Finished::unspent(state), None);
    }
    let text = Text {
        bytes: line.text.as_bytes(),
        start: line.start,
        anchored: line.embedded,
    };
    let halts = (end, after);
    let (parts, buffers) = cx.parts();
    let (finished, halt) = Lexer::new(parts, text, state, scratch(buffers))
        .halting(&halts)
        .run_halting();
    (finished, halt)
}

/// `embedded!(Variant, "lang-…")` is `Some(Language::Variant)` when the
/// language's feature is on. It is `None` otherwise.
/// The generated table of a rule that embeds a language uses it.
#[cfg(any(feature = "lang-ruby", feature = "lang-lua"))]
macro_rules! embedded {
    ($variant:ident, $feature:literal) => {{
        #[cfg(feature = $feature)]
        let language = Some(crate::Language::$variant);
        #[cfg(not(feature = $feature))]
        let language = None;
        language
    }};
}
#[cfg(any(feature = "lang-ruby", feature = "lang-lua"))]
pub(crate) use embedded;

/// A halt without context.
/// The patterns an embedding rule lists after the embedded grammar are one.
#[cfg(feature = "_embedding")]
pub(crate) type FnHalt = fn(&[u8], usize) -> bool;

/// A grammar whose rules embed other languages ([`Payload::Embed`]).
/// It says where the embedding rule takes the line back.
#[cfg(feature = "_embedding")]
pub(crate) trait Embeds: Grammar {
    /// The halt of the embedding rule's end and of the patterns it lists before
    /// the embedded grammar.
    type End: Halt;

    /// The halts for `embedding`.
    /// The first is tried before the embedded patterns at each place.
    /// The second stands for the patterns after the embedded grammar.
    /// It is tried where none of the embedded patterns match.
    /// The halts may read the payload of the embedding rule's parent frame.
    fn embed_halts(embedding: &Embedding, arena: &Arena) -> (Self::End, Option<FnHalt>);
}

/// Lexes `line` like [`lex`], for grammar `G` whose rules embed other languages
/// ([`Payload::Embed`]). The engine stops where the innermost rule embeds a
/// language. That language lexes the text from there.
/// It stops where the halts from [`Embeds::embed_halts`] hold.
/// The engine goes on from there.
/// The rule's payload keeps the embedded language's state.
/// A language that cannot halt lexes the rest of the line.
#[cfg(feature = "_embedding")]
pub(crate) fn lex_embedding<G: Embeds>(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    scratch: fn(&mut crate::scratch::Scratch) -> &mut Scratch<G>,
) -> Finished {
    if line.text.is_empty() {
        return Finished::unspent(state);
    }
    let text = Text {
        bytes: line.text.as_bytes(),
        start: line.start,
        anchored: line.embedded,
    };
    let (parts, buffers) = cx.parts();
    let (first, embedding) = Lexer::new(parts, text, state, scratch(buffers)).run_embedding();
    match embedding {
        Some(embedding) => hand_over::<G, false>(cx, line, scratch, None, first, embedding).0,
        None => first,
    }
}

/// Lexes `line` like [`lex_embedding`], for grammar `G` that another grammar
/// embeds in turn.
/// The line also halts where `outer` holds while only `G`'s root rule is
/// open, as in [`lex_until`], and the second value is where.
/// A line with no text from its start leaves the state as it is.
#[cfg(feature = "lang-graphql")]
pub(crate) fn lex_embedding_until<G: Embeds>(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    scratch: fn(&mut crate::scratch::Scratch) -> &mut Scratch<G>,
    outer: Halts<'_>,
) -> (Finished, Option<usize>) {
    if line.text.len() <= line.start {
        return (Finished::unspent(state), None);
    }
    let text = Text {
        bytes: line.text.as_bytes(),
        start: line.start,
        anchored: line.embedded,
    };
    let (parts, buffers) = cx.parts();
    let (first, stop) = Lexer::new(parts, text, state, scratch(buffers))
        .halting(&outer)
        .run_embedding_halting();
    match stop {
        Stop::Line => (first, None),
        Stop::Halted(at) => (first, Some(at)),
        Stop::Embedding(embedding) => {
            hand_over::<G, true>(cx, line, scratch, Some(&outer), first, embedding)
        }
    }
}

/// Lexes `line` like [`lex_embedding`], for grammar `G` without the per-step
/// test ([`Grammar::EMBEDS`]).
/// Most lines need none of the embedded languages.
/// The line is lexed without them first.
/// A line that needs them is lexed again with them.
#[cfg(feature = "_embedding")]
#[inline]
pub(crate) fn lex_on_demand<G: Embeds>(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    scratch: fn(&mut crate::scratch::Scratch) -> &mut Scratch<G>,
) -> Finished {
    if line.text.is_empty() {
        return Finished::unspent(state);
    }
    let text = Text {
        bytes: line.text.as_bytes(),
        start: line.start,
        anchored: line.embedded,
    };
    let first = cx.out.len();
    let (parts, buffers) = cx.parts();
    let (finished, embeds) = Lexer::new(parts, text, state, scratch(buffers)).run_on_demand();
    if embeds {
        return lex_again(cx, line, state, scratch, first, finished);
    }
    finished
}

/// Lexes `line` again with [`lex_embedding`].
/// The first pass of [`lex_on_demand`] found that it needs its embedded
/// languages.
/// A line that ends inside a rule that embeds drops the frames kept for the
/// next line. That line then opens its rules from the state, which notes the
/// rule.
/// Its tokens start at `first` in the output.
/// The second pass gets the work the first pass left.
/// The line spends one budget.
#[cfg(feature = "_embedding")]
#[cold]
fn lex_again<G: Embeds>(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    scratch: fn(&mut crate::scratch::Scratch) -> &mut Scratch<G>,
    first: usize,
    finished: Finished,
) -> Finished {
    cx.out.truncate(first);
    let steps = Steps::new(cx.limits, line.text.len());
    steps.take_back(finished);
    let lent = steps.lend(cx.limits);
    let mut again = Context {
        scratch: &mut *cx.scratch,
        arena: &mut *cx.arena,
        limits: &lent,
        out: &mut *cx.out,
        report: &mut *cx.report,
    };
    let state = lex_embedding::<G>(&mut again, line, state, scratch);
    let state = steps.take_back(state);
    let rule = G::rule_from_code(cx.arena.frame(state).kind);
    if matches!(G::rule_payload(rule), Payload::Embed(_)) {
        scratch(cx.scratch).forget_resume();
    }
    steps.finish(state)
}

/// The rest of [`lex_embedding`]'s line from where the grammar's `first`
/// lexer stopped for `embedding`: the embedded language lexes up to its
/// halt, and the grammar goes on from there, as often as the line asks.
/// One count for the line starts from what `first` spent; each part is lent
/// what it has left and adds what it spends, so the whole line spends one
/// work budget. Out of line, since most lines embed nothing.
#[cfg(feature = "_embedding")]
#[cold]
fn hand_over<G: Embeds, const HALTING: bool>(
    cx: &mut Context<'_>,
    line: Line<'_>,
    scratch: fn(&mut crate::scratch::Scratch) -> &mut Scratch<G>,
    outer: Option<&Halts<'_>>,
    first: Finished,
    mut embedding: Embedding,
) -> (Finished, Option<usize>) {
    let steps = Steps::new(cx.limits, line.text.len());
    let mut next = steps.take_back(first);
    let (state, halted) = loop {
        let language = embedding.language;
        let inner = match embedding.state {
            0 => language.start_embedded(cx.arena, embedding.base),
            known => known,
        };
        let part = Line {
            text: line.text,
            start: embedding.at,
            embedded: false,
        };
        let (end, after) = G::embed_halts(&embedding, cx.arena);
        let after = after.as_ref().map(|after| {
            let after: &dyn Halt = after;
            after
        });
        let lent = steps.lend(cx.limits);
        let mut part_cx = Context {
            scratch: &mut *cx.scratch,
            arena: &mut *cx.arena,
            limits: &lent,
            out: &mut *cx.out,
            report: &mut *cx.report,
        };
        let (inner, stop) = match language.lex_until(&mut part_cx, part, inner, &end, after) {
            Some(halted) => halted,
            None => (language.lex_embedded(&mut part_cx, part, inner), None),
        };
        let inner = steps.take_back(inner);
        let state = with_payload(cx.arena, next, inner).unwrap_or_else(|| {
            cx.report.record(Limit::Nodes);
            next
        });
        let Some(at) = stop else {
            break (state, None);
        };
        let text = Text {
            bytes: line.text.as_bytes(),
            start: at,
            anchored: false,
        };
        let lent = steps.lend(cx.limits);
        let parts = Parts {
            arena: &mut *cx.arena,
            limits: &lent,
            out: &mut *cx.out,
            report: &mut *cx.report,
        };
        let lexer = Lexer::new(parts, text, state, scratch(&mut *cx.scratch));
        let (finished, found) = if HALTING {
            resume_halting(lexer, outer)
        } else {
            let (finished, found) = lexer.resume_embedding();
            (finished, found.map_or(Stop::Line, Stop::Embedding))
        };
        let resumed = steps.take_back(finished);
        match found {
            Stop::Line => break (resumed, None),
            Stop::Halted(at) => break (resumed, Some(at)),
            Stop::Embedding(found) => (next, embedding) = (resumed, found),
        }
    };
    (steps.finish(state), halted)
}

/// Goes on with a halting line where an embedded language halted, halting
/// at `outer` when the grammar is itself embedded.
#[cfg(feature = "lang-graphql")]
fn resume_halting<'a, G: Grammar>(
    lexer: Lexer<'a, G>,
    outer: Option<&'a Halts<'a>>,
) -> (Finished, Stop) {
    match outer {
        Some(halts) => lexer.halting(halts).resume_embedding_halting(),
        None => lexer.resume_embedding_halting(),
    }
}

/// A grammar that nothing embeds never resumes a halting line.
#[cfg(all(feature = "_embedding", not(feature = "lang-graphql")))]
fn resume_halting<G: Grammar>(lexer: Lexer<'_, G>, _: Option<&Halts<'_>>) -> (Finished, Stop) {
    let (finished, found) = lexer.resume_embedding();
    (finished, found.map_or(Stop::Line, Stop::Embedding))
}

/// The scopes of a pattern's capture groups by group number, and the groups
/// that the grammar tokenizes with patterns of their own.
pub(crate) type Captures<G> = (
    &'static [(u8, Scopes<<G as Grammar>::Scope>)],
    &'static [(u8, <G as Grammar>::Nested)],
);

/// A grammar whose rules are generated tables: the patterns of each rule and
/// the bytes each can start at.
pub(crate) trait Tabled: Grammar {
    /// The bytes `pat` can start at.
    fn pat_start(pat: Self::Pat) -> Start;
    /// The tables of `rule`, built on first use.
    fn tables(rule: Self::Rule) -> &'static Tables<Self>;
}

/// What a rule carries from line to line.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Payload {
    /// Nothing.
    None,
    /// The text of a begin capture group, which the end pattern repeats.
    Backref(u8),
    /// The state of `Language`, whose grammar the rule's patterns include: a
    /// rule that embeds another language keeps that language's state, and
    /// the lexer hands the text to it (see [`Lexer::run_embedding`]).
    Embed(crate::Language),
}

/// How a line whose grammar embeds other languages stopped.
#[cfg(feature = "_embedding")]
#[derive(Clone, Copy, Debug)]
pub(crate) enum Stop {
    /// At the end of the line.
    Line,
    /// Where the line halted for the grammar that embeds it.
    Halted(usize),
    /// Where the innermost rule hands the text to the language it embeds.
    Embedding(Embedding),
}

/// Where a line stopped for an embedded language.
/// It holds the cursor, the language and its state (0 before it lexed
/// anything). It holds the scope stack the language's text sits on.
/// It holds the payload of the embedding rule's parent frame, such as a begin
/// capture of the outer rule.
#[cfg(feature = "_embedding")]
#[derive(Clone, Copy, Debug)]
pub(crate) struct Embedding {
    pub(crate) at: usize,
    pub(crate) language: crate::Language,
    pub(crate) state: u32,
    pub(crate) base: u32,
    pub(crate) parent: u32,
}

/// A scope whose name ends in text of the line, such as the digits of `2>`:
/// the scope is `prefix`, the text `start..end` spelled as
/// [`Grammar::dyn_name_case`] says, and `suffix`.
#[derive(Clone, Copy, Debug)]
pub(crate) struct DynName {
    pub(crate) prefix: &'static str,
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) suffix: &'static str,
}

/// How a scope name spells the text it is built from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum NameCase {
    /// As the line writes it, as `$1` does.
    AsWritten,
    /// Lowered, as `${1:/downcase}` does with JavaScript's `toLowerCase`:
    /// the Kelvin sign is `k`, but `ſ` and the ligatures stay.
    Lowered,
}

/// What a scope of a grammar names.
#[derive(Clone, Copy, Debug)]
pub(crate) enum ScopeName {
    /// A scope of the crate's shared numbering.
    Fixed(Scope),
    /// A scope whose name repeats the text of a capture group of the match,
    /// as `storage.type.$1.cpp` does: `prefix`, the text of group `group`
    /// and `suffix`.
    Capture {
        prefix: &'static str,
        group: u8,
        suffix: &'static str,
    },
}

/// A run of a match that carries scopes of its own, or is lexed again.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Span<G: Grammar> {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) scopes: Scopes<G::Scope>,
    /// One more scope, after `scopes`, chosen when the matcher ran.
    pub(crate) single: Option<G::Scope>,
    pub(crate) dynamic: Option<DynName>,
    pub(crate) retok: Option<G::Rule>,
}

/// What a matching pattern does.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Act<G: Grammar> {
    /// Names the text and opens nothing.
    Match(Scopes<G::Scope>),
    /// Opens a rule.
    Begin(G::Rule),
    /// Closes the innermost rule.
    End,
}

/// The data of one pattern.
#[derive(Clone, Copy, Debug)]
pub(crate) struct PatData<G: Grammar> {
    pub(crate) act: Act<G>,
    /// The scopes of the match's capture groups, by group number; group 0 is
    /// the whole match.
    pub(crate) captures: &'static [(u8, Scopes<G::Scope>)],
    /// The capture groups whose text the grammar tokenizes with patterns of
    /// their own, by group number.
    pub(crate) nested: &'static [(u8, G::Nested)],
}

/// The data of one rule.
#[derive(Clone, Copy, Debug)]
pub(crate) struct RuleData<G: Grammar> {
    /// The scopes of its begin and end text and everything between.
    pub(crate) name: Scopes<G::Scope>,
    /// The scopes of what lies between its begin and end text.
    pub(crate) content: Scopes<G::Scope>,
    /// The pattern that closes it.
    pub(crate) end: Option<G::Pat>,
    /// The begin capture group that the end pattern repeats, when it does.
    pub(crate) backref: Option<u8>,
    /// The patterns it tries, in order, after its end pattern.
    pub(crate) patterns: &'static [G::Pat],
}

/// The bytes a pattern can start at.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Start {
    /// Bit `b` of word `b / 64` says that a match can start before byte `b`.
    pub(crate) bytes: [u64; 4],
    /// The pattern matches only at the start of the line.
    pub(crate) bol: bool,
    /// A `\b` or a lookbehind for a non-word character comes before its first
    /// character, so it cannot match inside a word.
    pub(crate) wb: bool,
}

/// A line and where lexing it starts: byte `start`, where a Markdown fence's
/// text begins after the indentation of the containers around it. The bytes
/// before it stay in the line, because the grammar looks at them: `^` matches
/// only at 0, so a decorator that follows indentation is not at the start of
/// the line.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Text<'a> {
    pub(crate) bytes: &'a [u8],
    pub(crate) start: usize,
    /// Whether `\G` matches at `start`: set for a Markdown fence line, whose
    /// `while` rule has matched the container prefix just before `start`, as
    /// it does in Shiki.
    pub(crate) anchored: bool,
}
