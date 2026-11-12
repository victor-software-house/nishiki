//! The rule engine that the languages written from a `TextMate` grammar share.
//! Python, Java, CSS, Make, Kotlin, Go, Ruby, Swift and C run on it.
//! Each language supplies a [`Grammar`]: the generated tables of its rules and
//! patterns, and the hand-written matchers that decide where each pattern
//! matches.
//!
//! The engine runs the grammar's rules the way a `TextMate` engine does, with
//! those matchers instead of regular expressions.
//! The per-line loop, `\G`, capture groups, `while` rules, the limits and the
//! frames that carry a rule from line to line are here, once.

#[cfg(feature = "_rule-tables")]
mod chars;
#[cfg(feature = "lang-proto")]
pub(crate) mod generated;
mod lexer;
#[cfg(any(feature = "lang-go", feature = "lang-ruby", feature = "lang-lua"))]
mod runs;
// `RuleSyntax`'s generic trait impl makes rustc export `Arena::child` and
// `Arena::push`, and each call in the crate then goes through the GOT.
// So the module exists only where it is used: with the cfg and in its test.
#[cfg(any(nishiki_unstable, all(test, feature = "lang-toml")))]
mod syntax;
#[cfg(feature = "_rule-tables")]
mod tables;

use std::fmt::Debug;
#[cfg(feature = "_embedding")]
use std::ops::ControlFlow;

#[cfg(any(
    feature = "lang-cmake",
    feature = "lang-coffee",
    feature = "lang-python",
    feature = "lang-proto",
    feature = "lang-java",
    feature = "lang-css",
    feature = "lang-kotlin",
    feature = "lang-go",
    feature = "lang-ruby",
    feature = "lang-csharp",
    feature = "lang-dart",
    feature = "lang-sass",
    feature = "lang-scala",
    feature = "lang-bat",
    feature = "lang-haskell",
    feature = "lang-swift",
    feature = "lang-zig",
    feature = "lang-cpp",
    feature = "lang-lua",
    feature = "lang-nix",
    feature = "lang-less",
    feature = "lang-postcss",
    feature = "lang-powershell",
    feature = "lang-julia",
    feature = "lang-matlab",
    feature = "lang-mermaid",
    feature = "lang-objective-c",
    feature = "lang-objective-cpp",
    feature = "lang-pascal",
    feature = "lang-hlsl",
    feature = "lang-shaderlab",
    feature = "lang-svelte",
    feature = "lang-php",
    feature = "lang-perl",
    feature = "lang-elixir",
    feature = "lang-gdscript",
    feature = "lang-vb",
    feature = "lang-common-lisp",
    feature = "lang-r",
    feature = "lang-jinja",
    feature = "lang-asm",
    feature = "lang-tex",
    feature = "lang-haml",
    feature = "lang-astro"
))]
pub(crate) use chars::Chars;
#[cfg(feature = "_embedding")]
#[cfg_attr(
    not(nishiki_unstable),
    expect(unreachable_pub, reason = "public only through nishiki::unstable")
)]
pub use lexer::Deferrals;
use lexer::FIRST_LINE;
#[cfg_attr(
    not(nishiki_unstable),
    expect(unreachable_pub, reason = "public only through nishiki::unstable")
)]
pub use lexer::Lexer;
pub(crate) use lexer::Scratch;
#[cfg(feature = "_embedding")]
use lexer::with_carried;
#[cfg(any(feature = "lang-go", feature = "lang-ruby", feature = "lang-lua"))]
pub(crate) use runs::{CachedRuns, HasRuns, Runs};
#[cfg(nishiki_unstable)]
pub use syntax::RuleSyntax;
#[cfg(feature = "_rule-tables")]
pub(crate) use tables::{Place, table_scan};
#[cfg(feature = "_rule-tables")]
pub(crate) use tables::{PlaceSet, Tables};

use crate::Language;
use crate::arena::{Arena, ROOT};
use crate::language::{Context, Halt, Line, Stopped};
#[cfg(feature = "_embedding")]
use crate::language::{Halts, Parts, StopReason, cut_tokens};
use crate::limits::Finished;
#[cfg(feature = "_embedding")]
use crate::limits::{Limit, Steps};
use crate::scope::{Scope, Scopes};
#[cfg(feature = "_embedding")]
use crate::{Stack, Token};

/// One language's grammar: its rules and patterns, by type, and the
/// hand-written matchers that decide where each pattern matches.
#[cfg_attr(
    not(nishiki_unstable),
    expect(unreachable_pub, reason = "public only through nishiki::unstable")
)]
pub trait Grammar: Sized + Copy + Debug + 'static {
    /// A scope name of the grammar.
    type Scope: Copy + Debug + 'static;
    /// The empty list of the grammar's scopes.
    const NO_SCOPES: Scopes<Self::Scope>;
    /// A rule of the grammar that can be open at the end of a line.
    type Rule: Copy + Debug + PartialEq + 'static;
    /// One regular expression of the grammar.
    type Pattern: Copy + Debug + 'static;
    /// What a capture group holds that the grammar tokenizes with patterns
    /// of its own.
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
    /// the same match.
    /// Such a span takes that span's scopes too, as nested capture groups do.
    /// Off, every span sits directly under the match.
    const NESTED_SPANS: bool = false;
    /// The longest begin text that the name of a rule with a delimiter
    /// repeats. A longer one leaves the delimiter out of the name.
    /// The begin pattern takes it in another group than the one the name
    /// reads.
    /// C++'s raw strings do this with a delimiter of more than 16 bytes.
    const LONGEST_NAMED_DELIMITER: usize = usize::MAX;

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
    fn rule_while(_rule: Self::Rule) -> Option<Self::Pattern> {
        None
    }
    /// Whether a rule of the grammar has a `while` pattern
    /// ([`Grammar::rule_while`]).
    /// The engine then keeps the indices of the open ones between lines.
    /// A line that continues the last tries only those frames, and without the
    /// flag it visits every frame.
    /// The flag only saves work: the tokens are the same either way.
    const WHILES: bool = false;
    /// What the scan does with the rest of a space and tab run where no
    /// pattern matched at its first byte ([`BlankRuns`]).
    const BLANK_RUNS: BlankRuns = BlankRuns::Skip;
    /// Whether `pattern` takes exactly one blank and then requires a character
    /// that is not a blank.
    /// A [`BlankRuns::ToLast`] grammar tries these patterns at the last blank
    /// of a run.
    /// Its generator writes them from `oracle/rule-grammar.ts`'s
    /// `takesOneBlank`.
    #[cfg(any(nishiki_unstable, feature = "_blank-runs-to-last"))]
    fn takes_one_blank(_pattern: Self::Pattern) -> bool {
        false
    }
    /// Whether the loop tests at each step whether the innermost rule embeds
    /// another language ([`CarriedState::Embed`]). A rule that embeds hands its
    /// line over when it opens, without this test. The test is for a grammar
    /// whose line can come back into such a rule mid-line. Ruby needs it: an
    /// interpolation can close inside a heredoc, and an escape resumes it.
    const EMBEDS: bool = false;
    /// Whether the grammar lexes each line without its embedded languages first
    /// (`lex_on_demand`). Such a grammar has no per-step test.
    /// A line that starts inside a rule that embeds is noted where the rules
    /// are opened from the state, not on every line.
    #[cfg(feature = "_embedding")]
    const EMBEDS_ON_DEMAND: bool = false;
    /// What a rule carries from line to line.
    fn rule_carried_state(_rule: Self::Rule) -> CarriedState {
        CarriedState::None
    }
    /// The prefix of a content scope that ends in the rule's begin text,
    /// for a rule whose content scope is named by it.
    fn rule_content_delimiter(_rule: Self::Rule) -> Option<&'static str> {
        None
    }
    /// The prefix of a name scope that ends in the rule's begin text.
    ///
    /// It is for a rule whose name repeats the begin capture group it carries.
    /// Its [`Grammar::rule_name`] then leaves that scope out.
    /// The next line rebuilds the name from the begin text, as it has no
    /// groups.
    fn rule_name_delimiter(_rule: Self::Rule) -> Option<&'static str> {
        None
    }
    /// The text after the begin text in that name scope, such as `.svelte`
    /// in `meta.lang.$3.svelte`; none by default.
    fn rule_name_suffix(_rule: Self::Rule) -> &'static str {
        ""
    }
    /// The name scopes that come after that name scope, such as
    /// `meta.special.start.svelte` in
    /// `meta.special.$3.svelte meta.special.start.svelte`; none by default.
    /// Only a rule with a [`Grammar::rule_name_delimiter`] reads them.
    fn rule_name_after(_rule: Self::Rule) -> Scopes<Self::Scope> {
        Self::NO_SCOPES
    }
    /// Where a rule that embeds a language ([`CarriedState::Embed`]) starts it.
    ///
    /// It is the function that gives the language's state at one of its
    /// grammar's rules other than the root.
    /// An `include` of `source.ts#object-literal` does this.
    /// None, the default, starts the language at its root.
    #[cfg(feature = "_embedding")]
    fn embed_start(_rule: Self::Rule) -> Option<fn(&mut Arena, u32) -> u32> {
        None
    }
    /// What `pattern` does when it matches.
    fn pattern_act(pattern: Self::Pattern) -> Act<Self>;
    /// The scopes of the capture groups of `pattern`, and the groups the
    /// grammar tokenizes with patterns of their own.
    fn pattern_captures(pattern: Self::Pattern) -> Captures<Self>;
    /// Both answers about `pattern` that the engine needs for each match; a
    /// grammar whose table holds them together answers with one lookup.
    fn pattern_definition(pattern: Self::Pattern) -> (Act<Self>, Captures<Self>) {
        (Self::pattern_act(pattern), Self::pattern_captures(pattern))
    }
    /// The first pattern of the innermost rule that matches at or after the
    /// cursor, with where its match starts and ends. At the first place where
    /// `Lexer::root_halts` holds it calls `Lexer::halt_here` and returns
    /// `None`. A language that keeps the candidate tables of `Tabled` calls
    /// `table_scan`.
    fn scan(lexer: &mut Lexer<'_, Self>) -> Option<(usize, Self::Pattern, usize)>;
    /// Whether `pattern` matches at `p`, and where it ends.
    fn matcher(lexer: &mut Lexer<'_, Self>, pattern: Self::Pattern, p: usize) -> Option<usize>;
    /// Emits the tokens of the capture group text `from..to` that sits under
    /// the stack `base`, and leaves the cursor at `to`.
    fn tokenize_nested(
        lexer: &mut Lexer<'_, Self>,
        nested: Self::Nested,
        from: usize,
        to: usize,
        base: u32,
    );
    /// Whether some rules open on another stack than the innermost rule's
    /// text ([`Grammar::begin_stack`]).
    const BEGIN_STACKS: bool = false;

    /// The stack `rule` opens on, when it is not the innermost rule's text.
    ///
    /// Svelte's template body, injected into the Markdown of an `@component`
    /// comment, opens on the stack Markdown reached.
    /// Read where a rule opens and where the frames are rebuilt from a state,
    /// so both name it alike.
    /// Read only where [`Grammar::BEGIN_STACKS`] holds.
    fn begin_stack(_lexer: &Lexer<'_, Self>, _rule: Self::Rule) -> Option<u32> {
        None
    }

    /// Whether the grammar keeps a fact about its open rules in its
    /// [`Grammar::Extra`], for the next line to read before it lexes.
    /// The fact is redone only where the open rules changed, as the links of
    /// the `while` rules are.
    const WATCHES_FRAMES: bool = false;

    /// Redoes the fact after the open rules came from the line's state, not
    /// from the frames the last line kept.
    /// Read only where [`Grammar::WATCHES_FRAMES`] holds.
    fn frames_opened(_lexer: &mut Lexer<'_, Self>) {}

    /// Updates the fact at the end of a line whose frames the next line
    /// keeps.
    /// The frames from `low` up opened on this line; any above those still
    /// open closed.
    /// Read only where [`Grammar::WATCHES_FRAMES`] holds.
    fn frames_settled(_lexer: &mut Lexer<'_, Self>, _low: usize) {}

    /// Whether the halts of the language that embeds the grammar can hold
    /// below its root, through `Halt::deep`.
    /// Blade's injection holds inside the PHP of an echo at every depth, even
    /// inside a string.
    ///
    /// A grammar that sets it keeps in its [`Grammar::Extra`] whether an open
    /// rule shuts the deep halt out.
    /// It also sets [`Grammar::WATCHES_FRAMES`] to redo that fact where the
    /// open rules come from the line's state.
    const DEPTH_HALTS: bool = false;

    /// Updates the fact after a rule opened.
    /// Read only where [`Grammar::DEPTH_HALTS`] holds.
    fn frame_pushed(_lexer: &mut Lexer<'_, Self>) {}

    /// Updates the fact after rules closed.
    /// Read only where [`Grammar::DEPTH_HALTS`] holds.
    fn frames_popped(_lexer: &mut Lexer<'_, Self>) {}

    /// Whether the deep halt holds with the rules open now: no open rule shuts
    /// it out.
    /// Read only where [`Grammar::DEPTH_HALTS`] holds.
    fn deep_halts_open(_lexer: &Lexer<'_, Self>) -> bool {
        false
    }

    /// Whether another language lexes some capture groups of the grammar
    /// again, as `TextMate` does for a group whose patterns include another
    /// grammar ([`Grammar::defers_nested`]).
    /// Only a grammar that embeds languages (`Embeds`) sets it.
    const DEFERS_NESTED: bool = false;
    /// The language that lexes the `nested` capture groups again, if one
    /// does.
    /// The engine then emits such a group as one placeholder token and
    /// defers it.
    /// Where the line hands control back, `Embeds::lex_nested` lexes it in
    /// place of the placeholder.
    /// It is read only where [`Grammar::DEFERS_NESTED`] is set.
    fn defers_nested(_nested: Self::Nested) -> Option<Language> {
        None
    }
    /// Where the line being lexed notes its deferred capture groups, in the
    /// matchers' memory ([`Grammar::Extra`]).
    /// A grammar with [`Grammar::DEFERS_NESTED`] keeps the slot there and in
    /// [`Grammar::Kept`], which [`Grammar::extra`] and [`Grammar::keep`]
    /// pass between them.
    /// Every other grammar keeps none, so its scratch carries nothing of it.
    #[cfg(feature = "_embedding")]
    fn deferred(_extra: &mut Self::Extra) -> Option<&mut Option<Box<Deferrals>>> {
        None
    }
    /// The same slot after the run, in what the line keeps
    /// ([`Grammar::Kept`]), where the engine lexes the groups.
    #[cfg(feature = "_embedding")]
    fn deferred_kept(_kept: &mut Self::Kept) -> Option<&mut Option<Box<Deferrals>>> {
        None
    }
    /// Whether the scan asks [`Grammar::next_place`] where to go on after a
    /// place where no pattern matched.
    /// Off, it goes on at the next character.
    const SKIPS_PLACES: bool = false;
    /// Where the scan goes on after no pattern matched at `p`, which holds a
    /// byte of the text, when [`Grammar::SKIPS_PLACES`] is on.
    /// The grammar skips only places where it is certain no pattern matches.
    fn next_place(lexer: &Lexer<'_, Self>, p: usize) -> usize {
        lexer.char_end(p)
    }
    /// Whether no pattern of `rule` can match in `text`, so that lexing a
    /// span again with it can be skipped.
    fn skip_retok(_rule: Self::Rule, _text: &[u8]) -> bool {
        false
    }
    /// How a `DynName` or a [`ScopeName::Capture`] with `prefix` spells
    /// the text it reads: as written, unless the grammar lowers it, as CSS's
    /// units do with `${2:/downcase}`. Asked only where a dynamic name is
    /// built, so that `Span` stays its size.
    #[must_use]
    fn dyn_name_case(_prefix: &'static str) -> NameCase {
        NameCase::AsWritten
    }
}

/// The state at the start of a `G` document whose scopes sit on
/// the stack `base`: the root rule with [`Grammar::SOURCE`] on top of it.
pub(crate) fn start<G: Grammar>(arena: &mut Arena, base: u32) -> u32 {
    let stack = arena.child(base, G::SOURCE).unwrap_or(base);
    let carried = if G::MARKS_FIRST_LINE { FIRST_LINE } else { 0 };
    arena
        .push(ROOT, G::rule_code(G::ROOT), carried, stack)
        .unwrap_or(ROOT)
}

/// The start state of grammar `G`'s lines in a fenced code block.
///
/// The block's text sits on the stack `base` with no source scope of its own,
/// as in Shiki's embedding.
#[cfg(any(
    feature = "_start-embedded",
    feature = "lang-html",
    feature = "lang-java"
))]
pub(crate) fn start_embedded<G: Grammar>(arena: &mut Arena, base: u32) -> u32 {
    arena
        .push(ROOT, G::rule_code(G::ROOT), 0, base)
        .unwrap_or(ROOT)
}

/// Lexes `line` from `state` with grammar `G` and returns the state after it.
///
/// It follows the crate's one lexer contract
/// ([`Language::lex`](crate::Language)). It uses the buffers `scratch` picks
/// out of the highlighter's.
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
        anchored_at_root: line.anchored_at_root,
    };
    let (parts, buffers) = cx.parts();
    Lexer::new(parts, text, state, scratch(buffers)).run()
}

/// Lexes `line` like [`lex`], with buffers the caller keeps.
///
/// A grammar that [`RuleSyntax`](syntax::RuleSyntax) runs has no field in the
/// highlighter's scratch.
/// Its buffers sit in the highlighter's slot for its syntax instead.
#[inline]
#[cfg(any(nishiki_unstable, all(test, feature = "lang-toml")))]
pub(crate) fn lex_with<G: Grammar>(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    scratch: &mut Scratch<G>,
) -> Finished {
    if line.text.is_empty() {
        return Finished::unspent(state);
    }
    let text = Text {
        bytes: line.text.as_bytes(),
        start: line.start,
        anchored: line.embedded,
        anchored_at_root: line.anchored_at_root,
    };
    let (parts, _) = cx.parts();
    Lexer::new(parts, text, state, scratch).run()
}

/// Lexes `line` like [`lex`], for grammar `G` that another grammar embeds up
/// to its end.
/// The line halts at the first place where `end` holds that the scan reaches
/// with only the root rule open.
/// It also halts where `after` holds there and no pattern matches (see
/// [`Halt`]).
///
/// The second value holds where the line halted.
/// It also holds where a pattern matched an empty text at the root without
/// advancing ([`StopReason::PopHost`]); the grammar lexes the rest of the line.
/// Without one, the state resumes on the next line.
#[inline]
pub(crate) fn lex_until<G: Grammar>(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    scratch: fn(&mut crate::scratch::Scratch) -> &mut Scratch<G>,
    end: &dyn Halt,
    after: Option<&dyn Halt>,
) -> (Finished, Option<Stopped>) {
    if line.text.len() <= line.start {
        return (Finished::unspent(state), None);
    }
    let text = Text {
        bytes: line.text.as_bytes(),
        start: line.start,
        anchored: line.embedded,
        anchored_at_root: line.anchored_at_root,
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
#[cfg(any(
    feature = "lang-ruby",
    feature = "lang-lua",
    feature = "lang-svelte",
    feature = "lang-php",
    feature = "lang-hack",
    feature = "lang-blade",
    feature = "lang-perl",
    feature = "lang-elixir",
    feature = "lang-tex",
    feature = "lang-latex",
    feature = "lang-haml",
    feature = "lang-astro",
    feature = "lang-shaderlab"
))]
macro_rules! embedded {
    ($variant:ident, $feature:literal) => {{
        #[cfg(feature = $feature)]
        let language = Some(crate::Language::$variant);
        #[cfg(not(feature = $feature))]
        let language = None;
        language
    }};
}
#[cfg(any(
    feature = "lang-ruby",
    feature = "lang-lua",
    feature = "lang-svelte",
    feature = "lang-php",
    feature = "lang-hack",
    feature = "lang-blade",
    feature = "lang-perl",
    feature = "lang-elixir",
    feature = "lang-tex",
    feature = "lang-latex",
    feature = "lang-haml",
    feature = "lang-astro",
    feature = "lang-shaderlab"
))]
pub(crate) use embedded;

/// A halt without context.
/// The patterns an embedding rule lists after the embedded grammar are one.
#[cfg(feature = "_embedding")]
pub(crate) type FnHalt = fn(&[u8], usize) -> bool;

/// What text an embedding rule's end halt reads besides the line
/// ([`Embeds::END_TEXT`]).
#[cfg(feature = "_embedding")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum EndText {
    /// The line's bytes only, through [`Halt::holds`].
    Own,
    /// The begin text of the embedding rule's parent too, through
    /// [`Embeds::end_holds`].
    /// HAML's filters read it: their end repeats the indent their begin
    /// read.
    /// The engine copies that text once per hand-over, into the grammar's
    /// buffer ([`Embeds::parent_text`]).
    #[cfg_attr(
        not(any(test, feature = "lang-haml")),
        expect(
            dead_code,
            reason = "only HAML reads its parent carried; the engine's test grammar does too"
        )
    )]
    ParentBeginText,
}

/// A grammar whose rules embed other languages ([`CarriedState::Embed`]).
/// It marks where the embedding rule takes the line back.
#[cfg(feature = "_embedding")]
pub(crate) trait Embeds: Grammar {
    /// The halt of the embedding rule's end and of the patterns it lists before
    /// the embedded grammar.
    type End: Halt;

    /// Whether another grammar embeds this one with halts
    /// ([`lex_embedding_until`]).
    /// Such a grammar resumes after its own embedded language halting too.
    /// `hand_over` then tests the halts at run time, so the grammar has one
    /// copy of it.
    /// A second copy took `resume_at` and `Lexer::resume_embedding` out of
    /// both.
    /// By default, none does, and the test compiles out.
    const HALTS: bool = false;

    /// Halts for `embedding`.
    /// The first is tried before the embedded patterns at each place.
    /// The second stands for the patterns after the embedded grammar, and is
    /// tried where none of the embedded patterns match.
    /// They may read the begin text of the embedding rule's parent.
    fn embed_halts(embedding: &Embedding, arena: &Arena) -> (Self::End, Option<FnHalt>);

    /// The halts for `embedding` by `rule`, the embedding rule.
    /// A grammar whose embedding rules end differently picks its halts here.
    /// By default, [`Embeds::embed_halts`].
    fn embed_halts_for(
        _rule: Self::Rule,
        embedding: &Embedding,
        arena: &Arena,
    ) -> (Self::End, Option<FnHalt>) {
        Self::embed_halts(embedding, arena)
    }

    /// Whether `rule` lists its own patterns after the language it embeds.
    /// Markdown's `php` fence does: it lists PHP's rules after HTML.
    ///
    /// The embedded language then halts where one of the rule's patterns
    /// starts and none of the language's own patterns match.
    /// That halt takes the place of the second halt of
    /// [`Embeds::embed_halts`].
    /// The engine finds the starts once a line, from where the language
    /// first takes the line, by the search its loop makes.
    /// By default, no rule does.
    fn embeds_after_own(_rule: Self::Rule) -> bool {
        false
    }

    /// What text the end halt reads besides the line ([`EndText`]).
    /// By default, the line's bytes only, and nothing is copied.
    const END_TEXT: EndText = EndText::Own;

    /// Whether `end` holds at `p` of `s`.
    /// `parent_text` is the begin text of the embedding rule's parent.
    /// The engine calls this instead of `end` itself only when
    /// [`Embeds::END_TEXT`] is [`EndText::ParentBeginText`].
    ///
    /// The default body only calls `end.holds`.
    /// A default that called an `#[inline]` function made rustc export it
    /// before (see [`Embeds::lex_embedded_until`]).
    fn end_holds(end: &Self::End, _parent_text: &[u8], s: &[u8], p: usize) -> bool {
        end.holds(s, p)
    }

    /// The buffer for the parent's begin text, in what the matchers
    /// keep from line to line ([`Grammar::Kept`]).
    /// A grammar whose end reads that text ([`EndText::ParentBeginText`])
    /// keeps one there, so
    /// a hand-over allocates nothing once the buffer has room.
    /// Every other grammar keeps none.
    fn parent_text(_kept: &mut Self::Kept) -> Option<&mut Vec<u8>> {
        None
    }

    /// Lexes `line` from `state` in `language`, the language the embedding
    /// rule embeds, when the grammar lexes that language its own way.
    /// `rule` is the embedding rule.
    /// It returns where the language halted, at one of `halts`.
    /// None leaves the line to the language's own
    /// [`crate::Language::lex_until`].
    /// Svelte lexes TypeScript and JavaScript with its `$` prefixes added.
    ///
    /// By default, none, and the body names no function.
    /// A default that called `Language::lex_until`, which is `#[inline]`,
    /// made rustc export the functions that it reaches, such as
    /// `Arena::child`.
    /// Each call to an exported function then went through the GOT.
    fn lex_embedded_until(
        _rule: Self::Rule,
        _language: crate::Language,
        _cx: &mut Context<'_>,
        _line: Line<'_>,
        _state: u32,
        _halts: (&dyn Halt, Option<&dyn Halt>),
    ) -> Option<(Finished, Option<Stopped>)> {
        None
    }

    /// Lexes a deferred capture group with `language`
    /// ([`Grammar::defers_nested`]).
    ///
    /// The group is the text from `line.start` to the end of `line.text`, with
    /// no line feed after it.
    /// `line.text` ends where the group ends.
    /// The group's text sits on the stack `base`.
    /// Its tokens must end inside the group.
    /// None leaves the group as its one placeholder token, as the default
    /// does; its body names no function.
    fn lex_nested(
        _language: Language,
        _cx: &mut Context<'_>,
        _line: Line<'_>,
        _base: u32,
    ) -> Option<Finished> {
        None
    }
}

/// Lexes the capture groups that the last run of the grammar deferred
/// ([`Grammar::defers_nested`]).
///
/// It spends what is left of the line's budget, `steps`, and returns that
/// budget's state.
/// The count starts from `finished`, what the run spent.
#[cfg(feature = "_embedding")]
fn settle_deferred<G: Embeds>(
    cx: &mut Context<'_>,
    line: Line<'_>,
    scratch: fn(&mut crate::scratch::Scratch) -> &mut Scratch<G>,
    finished: Finished,
) -> Finished {
    if !G::DEFERS_NESTED || !has_deferred(scratch(cx.scratch)) {
        return finished;
    }
    let steps = Steps::new(cx.limits, line.text.len());
    let state = steps.take_back(finished);
    lex_deferred(cx, line, scratch, &steps);
    steps.finish(state)
}

/// Lexes each deferred capture group with [`Embeds::lex_nested`], and puts
/// its tokens where its placeholder stood in the output.
///
/// The groups go from the last to the first.
/// Each splice moves only the tokens after it, so the placeholders before it
/// stay where they were.
/// Tokens are never merged or counted as they go out.
/// The spliced ones need no other rule.
///
/// Each group spends from `steps`, the line's one budget.
/// A group whose language left its end uncovered keeps the rest on its
/// placeholder's stack.
#[cfg(feature = "_embedding")]
#[cold]
fn lex_deferred<G: Embeds>(
    cx: &mut Context<'_>,
    line: Line<'_>,
    scratch: fn(&mut crate::scratch::Scratch) -> &mut Scratch<G>,
    steps: &Steps,
) {
    let Some(mut deferrals) = scratch(cx.scratch).deferred().and_then(Option::take) else {
        return;
    };
    let Deferrals { groups, tokens } = &mut *deferrals;
    for group in groups.iter().rev() {
        tokens.clear();
        let lent = steps.lend(cx.limits);
        let mut part_cx = Context::new(
            &mut *cx.scratch,
            &mut *cx.arena,
            &lent,
            &mut *tokens,
            &mut *cx.report,
        );
        let part = Line::new(line.text.get(..group.to).unwrap_or(line.text), group.from);
        let Some(finished) = G::lex_nested(group.language, &mut part_cx, part, group.base) else {
            continue;
        };
        steps.take_back(finished);
        if tokens.last().is_none_or(|token| token.end() < group.to) {
            tokens.push(Token {
                end: group.to,
                stack: Stack(group.base),
            });
        }
        cx.out.splice(group.token..=group.token, tokens.drain(..));
    }
    groups.clear();
    if let Some(slot) = scratch(cx.scratch).deferred() {
        *slot = Some(deferrals);
    }
}

/// Whether the last run deferred a capture group.
#[cfg(feature = "_embedding")]
fn has_deferred<G: Grammar>(scratch: &mut Scratch<G>) -> bool {
    scratch
        .deferred()
        .and_then(|slot| slot.as_ref())
        .is_some_and(|deferrals| !deferrals.groups.is_empty())
}

/// An embedding rule's end halt with the text of the rule's parent carried
/// value, for a grammar whose end reads it ([`EndText::ParentBeginText`]).
#[cfg(feature = "_embedding")]
struct ParentTextEnd<'a, G: Embeds> {
    end: &'a G::End,
    parent_text: &'a [u8],
}

#[cfg(feature = "_embedding")]
impl<G: Embeds> Halt for ParentTextEnd<'_, G> {
    /// Asks the end at character starts only, as a function halt does.
    fn first(&self, s: &[u8], from: usize, to: usize) -> Option<usize> {
        let holds = |s: &[u8], p: usize| G::end_holds(self.end, self.parent_text, s, p);
        holds.first(s, from, to)
    }

    fn holds(&self, s: &[u8], p: usize) -> bool {
        G::end_holds(self.end, self.parent_text, s, p)
    }
}

/// Lexes `line` like [`lex`], for grammar `G` whose rules embed other languages
/// ([`CarriedState::Embed`]). The engine stops where the innermost rule embeds
/// a language, and that language lexes the text from there.
///
/// The language stops where the halts from [`Embeds::embed_halts`] hold, and
/// the engine goes on from there.
/// The rule's carried value keeps the embedded language's state, and a language
/// that cannot halt lexes the rest of the line.
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
        anchored_at_root: line.anchored_at_root,
    };
    let (parts, buffers) = cx.parts();
    let (first, embedding) = Lexer::new(parts, text, state, scratch(buffers)).run_embedding();
    let first = settle_deferred(cx, line, scratch, first);
    match embedding {
        Some(embedding) => hand_over::<G>(cx, line, scratch, None, first, embedding).0,
        None => first,
    }
}

/// Lexes `line` like [`lex_embedding`], for grammar `G` that another grammar
/// embeds in turn.
/// The line also halts where `outer` holds while only `G`'s root rule is
/// open, as in [`lex_until`], and the second value is where.
/// A line with no text from its start leaves the state as it is.
#[cfg(feature = "_halting-embedding")]
pub(crate) fn lex_embedding_until<G: Embeds>(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    scratch: fn(&mut crate::scratch::Scratch) -> &mut Scratch<G>,
    outer: Halts<'_>,
) -> (Finished, Option<Stopped>) {
    if line.text.len() <= line.start {
        return (Finished::unspent(state), None);
    }
    lex_embedding_through(cx, line, state, scratch, outer)
}

/// Lexes `line` like [`lex_embedding_until`], also from the end of a text
/// that is not empty.
/// There only the line feed is left, where an open rule's end, such as a line
/// comment's `\n`, can still match.
/// Blade's PHP resumes there after a Blade rule that ends the line.
#[cfg(feature = "lang-blade")]
pub(crate) fn lex_embedding_until_resuming<G: Embeds>(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    scratch: fn(&mut crate::scratch::Scratch) -> &mut Scratch<G>,
    outer: Halts<'_>,
) -> (Finished, Option<Stopped>) {
    if line.text.is_empty() || line.text.len() < line.start {
        return (Finished::unspent(state), None);
    }
    lex_embedding_through(cx, line, state, scratch, outer)
}

/// The body of [`lex_embedding_until`], for a line it does not skip.
///
/// LLVM then took `settle_deferred` out of line in CoffeeScript and LaTeX.
/// Both effects were measured at one codegen unit.
#[cfg(feature = "_halting-embedding")]
#[expect(
    clippy::inline_always,
    reason = "with `#[inline]`, LLVM split it from `lex_embedding_until`"
)]
#[inline(always)]
fn lex_embedding_through<G: Embeds>(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    scratch: fn(&mut crate::scratch::Scratch) -> &mut Scratch<G>,
    outer: Halts<'_>,
) -> (Finished, Option<Stopped>) {
    let text = Text {
        bytes: line.text.as_bytes(),
        start: line.start,
        anchored: line.embedded,
        anchored_at_root: line.anchored_at_root,
    };
    let (parts, buffers) = cx.parts();
    let (first, stop) = Lexer::new(parts, text, state, scratch(buffers))
        .halting(&outer)
        .run_embedding_halting();
    let first = settle_deferred(cx, line, scratch, first);
    match stop {
        Stop::Line => (first, None),
        Stop::Halted(stopped) => (first, Some(stopped)),
        Stop::Embedding(embedding) => {
            hand_over::<G>(cx, line, scratch, Some(&outer), first, embedding)
        }
    }
}

/// Lexes `line` like [`lex_embedding`], for grammar `G` without the per-step
/// test ([`Grammar::EMBEDS`]).
/// Most lines need none of the embedded languages.
/// The line is lexed without them first.
/// A line that needs them is lexed again with them.
#[cfg(any(feature = "lang-elixir", feature = "lang-lua"))]
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
        anchored_at_root: line.anchored_at_root,
    };
    let first = cx.out.len();
    let (parts, buffers) = cx.parts();
    let (finished, embeds) = Lexer::new(parts, text, state, scratch(buffers)).run_on_demand();
    if embeds {
        if G::DEFERS_NESTED
            && let Some(deferrals) = scratch(cx.scratch)
                .deferred()
                .and_then(|slot| slot.as_mut())
        {
            deferrals.groups.clear();
        }
        return lex_again(cx, line, state, scratch, first, finished);
    }
    settle_deferred(cx, line, scratch, finished)
}

/// Lexes `line` again with [`lex_embedding`].
/// The first pass of [`lex_on_demand`] found that it needs its embedded
/// languages.
///
/// A line that ends inside a rule that embeds drops the frames kept for the
/// next line.
/// That line then opens its rules from the state, which notes the rule.
///
/// Its tokens start at `first` in the output.
/// The second pass gets the work the first pass left.
/// The line spends one budget.
#[cfg(any(feature = "lang-elixir", feature = "lang-lua"))]
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
    let mut again = Context::new(
        &mut *cx.scratch,
        &mut *cx.arena,
        &lent,
        &mut *cx.out,
        &mut *cx.report,
    )
    .with_first_token(cx.first_token);
    let state = lex_embedding::<G>(&mut again, line, state, scratch);
    let state = steps.take_back(state);
    let rule = G::rule_from_code(cx.arena.frame(state).rule);
    if matches!(G::rule_carried_state(rule), CarriedState::Embed(_)) {
        scratch(cx.scratch).forget_resume();
    }
    steps.finish(state)
}

/// The rest of [`lex_embedding`]'s line from where the grammar's `first`
/// lexer stopped for `embedding`.
///
/// The embedded language lexes up to its halt, and the grammar goes on from
/// there.
/// This repeats as often as the line needs.
///
/// One count for the line starts from what `first` spent.
/// Each part is lent what it has left and adds what it spends.
/// So the whole line spends one work budget.
/// Out of line, since most lines embed nothing.
///
/// Where the language halts before it lexed anything, the grammar goes on
/// with `\G` still holding there, if it held where the language started.
/// No match has ended anywhere else since the one the embedding started
/// after, so `\G` holds as it would in a `TextMate` engine.
///
/// At the start of a later line inside the rule, no match has ended on the
/// line, so `\G` does not hold there.
///
/// Each part keeps where the line's rules opened.
/// So a rule that opens at a halt and closes there without advancing stops
/// the line, as in one `TextMate` stack.
/// Svelte's `{#each}` destructuring opens and closes at the text after its
/// `}` so.
///
/// A rule whose begin took the line feed embeds from past the text.
/// No text is left on the line, so the line ends there.
/// The language starts on the next line, from the rule's empty carried value.
#[cfg(feature = "_embedding")]
#[cold]
#[expect(
    clippy::too_many_lines,
    reason = "split, it changed every grammar's hand-over code and the hot path's codegen"
)]
fn hand_over<G: Embeds>(
    cx: &mut Context<'_>,
    line: Line<'_>,
    scratch: fn(&mut crate::scratch::Scratch) -> &mut Scratch<G>,
    outer: Option<&Halts<'_>>,
    first: Finished,
    mut embedding: Embedding,
) -> (Finished, Option<Stopped>) {
    debug_assert!(
        outer.is_none() || G::HALTS,
        "a grammar embedded with halts sets Embeds::HALTS"
    );
    let steps = Steps::new(cx.limits, line.text.len());
    let mut next = steps.take_back(first);
    let mut starts: Option<(u16, Vec<usize>)> = None;
    let (state, halted) = loop {
        if embedding.at > line.text.len() {
            break (next, None);
        }
        let language = embedding.language;
        let rule = G::rule_from_code(cx.arena.frame(next).rule);
        let inner = match embedding.state {
            0 => match G::embed_start(rule) {
                Some(start) => start(cx.arena, embedding.base),
                None => language.start_embedded(cx.arena, embedding.base),
            },
            known => known,
        };
        let part = Line::new(line.text, embedding.at);
        let (end, after) = G::embed_halts_for(rule, &embedding, cx.arena);
        let own = G::embeds_after_own(rule);
        if own && starts.as_ref().map(|&(code, _)| code) != Some(G::rule_code(rule)) {
            let buffer = starts.take().map_or_else(
                || std::mem::take(&mut cx.scratch.rule_starts),
                |(_, buffer)| buffer,
            );
            let at = (next, embedding.at);
            starts = Some((
                G::rule_code(rule),
                own_starts(cx, line, scratch, &steps, at, buffer),
            ));
        }
        let listed =
            crate::language::Places(starts.as_ref().map_or(&[], |(_, buffer)| buffer.as_slice()));
        let own_after: &dyn Halt = &listed;
        let after = if own {
            Some(own_after)
        } else {
            after.as_ref().map(|after| {
                let after: &dyn Halt = after;
                after
            })
        };
        let ends = ends_at_line_end::<G>(cx.arena, line, &end, &embedding);
        let lent = steps.lend(cx.limits);
        let mut part_cx = Context::new(
            &mut *cx.scratch,
            &mut *cx.arena,
            &lent,
            &mut *cx.out,
            &mut *cx.report,
        )
        .with_first_token(cx.first_token);
        let halted = if ends {
            let at = embedding.at;
            Some((
                Finished::unspent(inner),
                Some(Stopped {
                    at,
                    reason: StopReason::Halt,
                }),
            ))
        } else {
            match G::END_TEXT {
                EndText::ParentBeginText => {
                    let at = (&embedding, scratch);
                    lex_reading_parent_text::<G>(rule, &mut part_cx, part, inner, (&end, after), at)
                }
                EndText::Own => {
                    G::lex_embedded_until(rule, language, &mut part_cx, part, inner, (&end, after))
                        .or_else(|| language.lex_until(&mut part_cx, part, inner, &end, after))
                }
            }
        };
        let (inner, stop) = match halted {
            Some(halted) => halted,
            None => (language.lex_embedded(&mut part_cx, part, inner), None),
        };
        let inner = steps.take_back(inner);
        let (at, state) = match resume_at(cx, line, scratch, (next, inner), stop) {
            ControlFlow::Continue(resumed) => resumed,
            ControlFlow::Break(end) => break (end, None),
        };
        let text = Text {
            bytes: line.text.as_bytes(),
            start: at,
            anchored: embedding.anchored && at == embedding.at,
            anchored_at_root: false,
        };
        let lent = steps.lend(cx.limits);
        let parts = Parts {
            arena: &mut *cx.arena,
            limits: &lent,
            out: &mut *cx.out,
            report: &mut *cx.report,
        };
        let lexer = Lexer::new(parts, text, state, scratch(&mut *cx.scratch));
        let (finished, found) = match outer {
            Some(halts) if G::HALTS => resume_halting(lexer, halts),
            Some(_) | None => {
                let (finished, found) = lexer.resume_embedding();
                (finished, found.map_or(Stop::Line, Stop::Embedding))
            }
        };
        let resumed = steps.take_back(finished);
        if G::DEFERS_NESTED && has_deferred(scratch(cx.scratch)) {
            lex_deferred(cx, line, scratch, &steps);
        }
        match found {
            Stop::Line => break (resumed, None),
            Stop::Halted(stopped) => break (resumed, Some(stopped)),
            Stop::Embedding(found) => (next, embedding) = (resumed, found),
        }
    };
    if let Some((_, buffer)) = starts {
        cx.scratch.rule_starts = buffer;
    }
    scratch(cx.scratch).forget_enters();
    (steps.finish(state), halted)
}

/// Whether the hand-over of `embedding` leaves no text on `line`, and the
/// rule's end halt holds at the line's end.
/// The language then lexes nothing, and the rule's end is tried there.
/// vscode-textmate tries it there too, after a match that reaches the line's
/// end.
/// A Ruby line in HAML that ends with `# comment` or a bare `-` ends so.
#[cfg(feature = "_embedding")]
fn ends_at_line_end<G: Embeds>(
    arena: &Arena,
    line: Line<'_>,
    end: &G::End,
    embedding: &Embedding,
) -> bool {
    let len = line.text.len();
    if embedding.at != len {
        return false;
    }
    let parent_text: &[u8] = match G::END_TEXT {
        EndText::ParentBeginText => arena.string(embedding.parent).as_bytes(),
        EndText::Own => &[],
    };
    G::end_holds(end, parent_text, line.text.as_bytes(), len)
}

/// Lexes `part` in the embedded language up to its halts, for a grammar
/// whose end halt reads the begin text of the embedding rule's parent
/// ([`EndText::ParentBeginText`]).
/// The grammar's own way goes first ([`Embeds::lex_embedded_until`]), then
/// the language's.
///
/// The text is copied into the grammar's buffer first
/// ([`Embeds::parent_text`]). The buffer goes back after the line part, so
/// nothing is allocated once it has room.
/// A hand-over nested in the part takes the buffer empty.
#[cfg(feature = "_embedding")]
fn lex_reading_parent_text<G: Embeds>(
    rule: G::Rule,
    cx: &mut Context<'_>,
    part: Line<'_>,
    inner: u32,
    (end, after): (&G::End, Option<&dyn Halt>),
    (embedding, scratch): (
        &Embedding,
        fn(&mut crate::scratch::Scratch) -> &mut Scratch<G>,
    ),
) -> Option<(Finished, Option<Stopped>)> {
    let language = embedding.language;
    let mut parent_text = scratch(cx.scratch)
        .parent_text()
        .map(std::mem::take)
        .unwrap_or_default();
    parent_text.clear();
    parent_text.extend_from_slice(cx.arena.string(embedding.parent).as_bytes());
    let end = ParentTextEnd::<G> {
        end,
        parent_text: &parent_text,
    };
    let halted = G::lex_embedded_until(rule, language, cx, part, inner, (&end, after))
        .or_else(|| language.lex_until(cx, part, inner, &end, after));
    if let Some(buffer) = scratch(cx.scratch).parent_text() {
        *buffer = parent_text;
    }
    halted
}

/// Where the grammar goes on after the language that the rule `embedding`
/// embeds stopped, as `stopped` records.
///
/// Where a halt held, it gives that place and the rule's state, with the
/// language's state `inner` as its carried value.
/// Otherwise it gives the state the line ends in:
///
/// - that state, where the language lexed the rest of the line;
/// - the state below the rule, where it popped the rule.
#[cfg(feature = "_embedding")]
fn resume_at<G: Grammar>(
    cx: &mut Context<'_>,
    line: Line<'_>,
    scratch: fn(&mut crate::scratch::Scratch) -> &mut Scratch<G>,
    (embedding, inner): (u32, u32),
    stopped: Option<Stopped>,
) -> ControlFlow<u32, (usize, u32)> {
    match stopped {
        Some(Stopped {
            at,
            reason: StopReason::Halt,
        }) => ControlFlow::Continue((at, resumed_with(cx, scratch, embedding, inner))),
        Some(Stopped {
            at,
            reason: StopReason::PopHost,
        }) => ControlFlow::Break(pop_host(cx, line, scratch, embedding, at)),
        None => ControlFlow::Break(resumed_with(cx, scratch, embedding, inner)),
    }
}

/// The state of the embedding rule `embedding` with the language's state
/// `inner` as its carried value ([`with_inner`]).
/// The frames the grammar kept for `embedding` move to that state, since
/// only the top frame's carried value differs.
/// Then the next lexer takes them over instead of opening every rule again.
#[cfg(feature = "_embedding")]
fn resumed_with<G: Grammar>(
    cx: &mut Context<'_>,
    scratch: fn(&mut crate::scratch::Scratch) -> &mut Scratch<G>,
    embedding: u32,
    inner: u32,
) -> u32 {
    let state = with_inner(cx, embedding, inner);
    scratch(cx.scratch).move_resume(embedding, state, inner);
    state
}

/// The state of the embedding rule `embedding` with the language's state
/// `inner` as its carried value, or `embedding` where the node limit is
/// reached.
#[cfg(feature = "_embedding")]
fn with_inner(cx: &mut Context<'_>, embedding: u32, inner: u32) -> u32 {
    with_carried(cx.arena, embedding, inner).unwrap_or_else(|| {
        cx.report.record(Limit::Nodes);
        embedding
    })
}

/// Pops the embedding rule on top of `state`, and returns the state below it.
///
/// vscode-textmate pops it where the language it embeds matched an empty text
/// at its root without advancing ([`StopReason::PopHost`]).
/// The rest of the line from `at` sits on that state's content scopes, in
/// place of the tokens the language gave it.
/// The line stops there.
#[cfg(feature = "_embedding")]
#[cold]
fn pop_host<G: Grammar>(
    cx: &mut Context<'_>,
    line: Line<'_>,
    scratch: fn(&mut crate::scratch::Scratch) -> &mut Scratch<G>,
    state: u32,
    at: usize,
) -> u32 {
    cut_tokens(cx.out, cx.first_token, at);
    let parent = cx.arena.frame(state).parent;
    if line.text.len() > at {
        cx.out.push(Token {
            end: line.text.len(),
            stack: Stack(cx.arena.frame(parent).stack),
        });
    }
    scratch(cx.scratch).forget_resume();
    parent
}

/// Fills `starts` with where the top rule's own patterns in
/// `state` start on `line` from `at`.
///
/// It is for a rule that lists them after the language it embeds
/// ([`Embeds::embeds_after_own`]).
/// The search spends from the line's one budget.
#[cfg(feature = "_embedding")]
fn own_starts<G: Embeds>(
    cx: &mut Context<'_>,
    line: Line<'_>,
    scratch: fn(&mut crate::scratch::Scratch) -> &mut Scratch<G>,
    steps: &Steps,
    (state, at): (u32, usize),
    mut starts: Vec<usize>,
) -> Vec<usize> {
    let text = Text {
        bytes: line.text.as_bytes(),
        start: at,
        anchored: true,
        anchored_at_root: false,
    };
    let lent = steps.lend(cx.limits);
    let parts = Parts {
        arena: &mut *cx.arena,
        limits: &lent,
        out: &mut *cx.out,
        report: &mut *cx.report,
    };
    let found =
        Lexer::new(parts, text, state, scratch(&mut *cx.scratch)).pattern_starts(&mut starts);
    steps.take_back(found);
    starts
}

/// Goes on with a halting line where an embedded language halted, halting
/// at `outer`, as the grammar is itself embedded.
#[cfg(feature = "_halting-embedding")]
fn resume_halting<'a, G: Grammar>(lexer: Lexer<'a, G>, outer: &'a Halts<'a>) -> (Finished, Stop) {
    lexer.halting(outer).resume_embedding_halting()
}

/// A grammar that nothing embeds never resumes a halting line.
#[cfg(all(feature = "_embedding", not(feature = "_halting-embedding")))]
fn resume_halting<G: Grammar>(lexer: Lexer<'_, G>, _: &Halts<'_>) -> (Finished, Stop) {
    let (finished, found) = lexer.resume_embedding();
    (finished, found.map_or(Stop::Line, Stop::Embedding))
}

/// The scopes of a pattern's capture groups by group number, and the groups
/// that the grammar tokenizes with patterns of their own.
#[cfg_attr(
    not(nishiki_unstable),
    expect(unreachable_pub, reason = "public only through nishiki::unstable")
)]
pub type Captures<G> = (
    &'static [(u8, Scopes<<G as Grammar>::Scope>)],
    &'static [(u8, <G as Grammar>::Nested)],
);

/// A grammar whose rules are generated tables: the patterns of each rule and
/// the bytes each can start at.
#[cfg(feature = "_rule-tables")]
pub(crate) trait Tabled: Grammar {
    /// Whether a pattern is left out inside words only past the scan's first
    /// position.
    ///
    /// A match inside a word could also start at the word's start, which the
    /// scan has not reached when it begins inside the word.
    /// Then the scan counts its first position inside a word as away from
    /// words ([`Place::InWord`](tables::Place::InWord)).
    /// It compiles that step out where no pattern needs it.
    const WORDS_AT_START: bool;

    /// The bytes `pattern` can start at.
    fn pattern_start(pattern: Self::Pattern) -> Start;
    /// The tables of `rule`, built on first use.
    fn tables(rule: Self::Rule) -> &'static Tables<Self>;

    /// Whether the grammar's `$base` includes are slots.
    /// There the root patterns of the document that embeds the grammar
    /// compete ([`crate::language::Halt::root`]).
    /// `false` compiles the slot out of the scan.
    const BASE_SLOTS: bool = false;

    /// Whether `pattern` is the slot that stands for a `$base` include.
    fn is_base_slot(_pattern: Self::Pattern) -> bool {
        false
    }

    /// The tables of `rule` with its `$base` includes as slots,
    /// built on first use; `None` for a rule with no `$base`.
    /// A line that another document embeds scans with them.
    fn base_tables(_rule: Self::Rule) -> Option<&'static Tables<Self>> {
        None
    }
}

/// What the scan does with the rest of a space and tab run where no
/// pattern matched at the run's first blank.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(
    not(nishiki_unstable),
    expect(unreachable_pub, reason = "public only through nishiki::unstable")
)]
pub enum BlankRuns {
    /// It skips the rest of the run.
    ///
    /// Sound when no pattern can match at a later blank of a run and not at
    /// its first.
    Skip,
    /// It jumps to the run's last blank and tries there only the patterns
    /// that take exactly one blank ([`Grammar::takes_one_blank`]).
    ///
    /// Such a pattern, as Sass's `\s-\s`, requires a character that is not a
    /// blank after its blank.
    /// It can match only at the last blank of a run.
    ///
    /// `ToLast` is sound when every pattern that can match at a later blank
    /// does one of two things:
    ///
    /// - it matches at the first blank too;
    /// - it is such a pattern.
    ///
    /// Then no other pattern can match at the last blank where it failed at
    /// the first.
    ///
    /// A lookbehind on the blank before a pattern can break that.
    /// It can let the pattern fail at the first blank and at the last, and
    /// match between them.
    /// A grammar with such a pattern takes [`BlankRuns::Visit`].
    #[cfg(any(nishiki_unstable, feature = "_blank-runs-to-last"))]
    ToLast,
    /// It tries every blank of the run, one byte at a time.
    ///
    /// Sound for every grammar.
    /// A grammar takes it when a lookbehind on the blank before a pattern
    /// rules out [`BlankRuns::ToLast`], as Objective-C's parameter name does.
    #[cfg(any(nishiki_unstable, feature = "lang-objective-c"))]
    Visit,
}

/// What a rule carries from line to line.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(
    not(nishiki_unstable),
    expect(unreachable_pub, reason = "public only through nishiki::unstable")
)]
pub enum CarriedState {
    /// Nothing.
    None,
    /// The text of a begin capture group, which the end pattern repeats.
    Backref(u8),
    /// The state of `Language`, whose grammar the rule's patterns include.
    ///
    /// A rule that embeds another language keeps that language's state.
    /// The lexer hands the text to it (see `Lexer::run_embedding`).
    Embed(crate::Language),
}

/// A 16-byte `CarriedState`, with a start function in it, cost Ruby 0.3% at one
/// codegen unit.
const _: () = assert!(size_of::<CarriedState>() <= 2);

/// How a line whose grammar embeds other languages stopped.
#[cfg(feature = "_embedding")]
#[derive(Clone, Copy, Debug)]
pub(crate) enum Stop {
    /// At the end of the line.
    Line,
    /// Where and why the line stopped for the grammar that embeds it.
    Halted(Stopped),
    /// Where the innermost rule hands the text to the language it embeds.
    Embedding(Embedding),
}

/// Where a line stopped for an embedded language.
/// It holds the cursor, the language and its state (0 before it lexed
/// anything), and the scope stack the language's text sits on.
/// It also holds the carried value of the embedding rule's parent frame, such
/// as the outer rule's begin text.
#[cfg(feature = "_embedding")]
#[derive(Clone, Copy, Debug)]
pub(crate) struct Embedding {
    pub(crate) at: usize,
    /// Whether `\G` holds at `at`: a match ended there on this line.
    /// At the start of a line that goes on inside the rule, it does not.
    pub(crate) anchored: bool,
    pub(crate) language: crate::Language,
    pub(crate) state: u32,
    pub(crate) base: u32,
    pub(crate) parent: u32,
}

/// A scope whose name ends in text of the line, such as the digits of `2>`:
/// the scope is `prefix`, the text `start..end` spelled as
/// [`Grammar::dyn_name_case`] sets, and `suffix`.
#[derive(Clone, Copy, Debug)]
pub(crate) struct DynName {
    pub(crate) prefix: &'static str,
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) suffix: &'static str,
}

/// How a scope name spells the text it is built from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(
    not(nishiki_unstable),
    expect(unreachable_pub, reason = "public only through nishiki::unstable")
)]
pub enum NameCase {
    /// As the line writes it, as `$1` does.
    AsWritten,
    /// Lowered, as `${1:/downcase}` does with JavaScript's `toLowerCase`:
    /// the Kelvin sign is `k`, but `ſ` and the ligatures stay.
    /// The text lowers into the name's buffer one character at a time, with
    /// no allocation.
    /// Only a text with a `Σ` goes through `str::to_lowercase`, which gives
    /// a final sigma `ς` as JavaScript does and allocates.
    Lowered,
}

/// What a scope of a grammar names.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(
    not(nishiki_unstable),
    expect(unreachable_pub, reason = "public only through nishiki::unstable")
)]
pub enum ScopeName {
    /// A scope of the crate's shared numbering.
    Fixed(Scope),
    /// A scope whose name repeats the text of a capture group of the match,
    /// as `storage.type.$1.cpp` does: `prefix`, the text of group `group`
    /// and `suffix`.
    #[cfg(any(test, nishiki_unstable, feature = "_capture-names"))]
    Capture {
        /// The name's text before the group's.
        prefix: &'static str,
        /// The number of the capture group whose text the name repeats.
        group: u8,
        /// The name's text after the group's.
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
#[cfg_attr(
    not(nishiki_unstable),
    expect(unreachable_pub, reason = "public only through nishiki::unstable")
)]
pub enum Act<G: Grammar> {
    /// Names the text and opens nothing.
    Match(Scopes<G::Scope>),
    /// Opens a rule.
    Begin(G::Rule),
    /// Closes the innermost rule.
    End,
}

impl<G: Grammar> Act<G> {
    /// The same act for `H`, a dialect of `G` with its rules and scopes, such
    /// as the CoffeeScript that Svelte embeds.
    #[cfg(feature = "lang-coffee")]
    pub(crate) fn dialect<H: Grammar<Rule = G::Rule, Scope = G::Scope>>(self) -> Act<H> {
        match self {
            Act::Match(name) => Act::Match(name),
            Act::Begin(rule) => Act::Begin(rule),
            Act::End => Act::End,
        }
    }
}

/// What one pattern does when it matches, and the scopes of its captures.
#[derive(Clone, Copy, Debug)]
#[cfg(feature = "_rule-tables")]
pub(crate) struct PatternDefinition<G: Grammar> {
    pub(crate) act: Act<G>,
    /// The scopes of the match's capture groups, by group number; group 0 is
    /// the whole match.
    pub(crate) captures: &'static [(u8, Scopes<G::Scope>)],
    /// The capture groups whose text the grammar tokenizes with patterns of
    /// their own, by group number.
    pub(crate) nested: &'static [(u8, G::Nested)],
}

/// The scopes and patterns of one rule.
#[derive(Clone, Copy, Debug)]
#[cfg(feature = "_rule-tables")]
pub(crate) struct RuleDefinition<G: Grammar> {
    /// The scopes of its begin and end text and everything between.
    pub(crate) name: Scopes<G::Scope>,
    /// The scopes of what lies between its begin and end text.
    pub(crate) content: Scopes<G::Scope>,
    /// The pattern that closes it, tried before every pattern of the rule.
    ///
    /// A rule whose `L:` injections win a tie with its end lists the end
    /// among its patterns instead, after them, as Svelte's language tags do.
    /// The engine finds an end pattern by its act, not by this field.
    pub(crate) end: Option<G::Pattern>,
    /// The begin capture group that the end pattern repeats, when it does.
    pub(crate) backref: Option<u8>,
    /// The patterns it tries, in order, after `end`.
    pub(crate) patterns: &'static [G::Pattern],
}

/// The bytes a pattern can start at.
#[derive(Clone, Copy, Debug)]
#[cfg(feature = "_rule-tables")]
pub(crate) struct Start {
    /// Bit `b` of word `b / 64` marks that a match can start before byte `b`.
    pub(crate) bytes: [u64; 4],
    /// Where in the line a match can start.
    /// A pattern that opens with `^` matches only at the start of the line.
    /// Any lookbehind that needs a byte keeps a match off the start of the
    /// line. A `\b`, or a lookbehind that fails after a word character, keeps
    /// it out of words: `(?<!\w)` and `(?<=\.)` are such lookbehinds.
    pub(crate) places: PlaceSet,
    /// The pattern can match where no byte follows: past the line feed, or
    /// at the end of a last line without one.
    ///
    /// Such a match is empty, as an anchor such as `$` or `\z`, an optional
    /// atom or a negative lookahead can be.
    /// A lookahead that needs a byte, such as `(?=<<)`, cannot match there.
    /// A backreference counts, since it can repeat an empty capture.
    pub(crate) matches_at_end: bool,
}

/// A line and where lexing it starts.
///
/// Lexing starts at byte `start`, where a Markdown fence's text begins after
/// the indentation of the containers around it.
/// The bytes before it stay in the line, because the grammar looks at them.
/// `^` matches only at 0, so a decorator that follows indentation is not at
/// the start of the line.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Text<'a> {
    pub(crate) bytes: &'a [u8],
    pub(crate) start: usize,
    /// Whether `\G` matches at `start`: set for a Markdown fence line, whose
    /// `while` rule has matched the container prefix right before `start`, as
    /// it does in Shiki.
    pub(crate) anchored: bool,
    /// Whether `\G` matches at `start` while the root rule alone is open
    /// ([`crate::language::Line::anchored_at_root`]).
    pub(crate) anchored_at_root: bool,
}
