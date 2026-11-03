//! The rule engine that the languages written from a `TextMate` grammar share:
//! Python, Java, CSS, Make, Kotlin, Go and Ruby run on it, and each language
//! supplies a [`Grammar`], the generated tables of its rules and patterns, and
//! the hand-written matchers that decide where each pattern matches.
//!
//! The engine runs the grammar's rules the way a `TextMate` engine does, with
//! those matchers instead of regular expressions: the per-line loop, `\G`,
//! capture groups, `while` rules, the limits and the frames that carry a rule
//! from line to line are here, once.

mod chars;
mod lexer;
#[cfg(any(feature = "lang-go", feature = "lang-ruby"))]
mod runs;
mod tables;

use std::fmt::Debug;

#[cfg(feature = "_rule-tables")]
pub(crate) use chars::Chars;
use lexer::FIRST_LINE;
pub(crate) use lexer::{Lexer, Scratch};
#[cfg(any(feature = "lang-go", feature = "lang-ruby"))]
pub(crate) use runs::{CachedRuns, HasRuns, Runs};
pub(crate) use tables::Tables;
#[cfg(feature = "_rule-tables")]
pub(crate) use tables::table_scan;

use crate::arena::{Arena, ROOT};
use crate::language::{Context, Line};
use crate::scope::Scope;

/// One language's grammar: its rules and patterns, by type, and the
/// hand-written matchers that decide where each pattern matches.
pub(crate) trait Grammar: Sized + Copy + Debug + 'static {
    /// A scope name of the grammar.
    type Scope: Copy + Debug + 'static;
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
    const SOURCE: Self::Scope;

    /// The scope in the crate's shared numbering.
    fn scope(scope: Self::Scope) -> Scope;
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
    fn rule_name(rule: Self::Rule) -> &'static [Self::Scope];
    /// The scopes of what lies between a rule's begin and end text.
    fn rule_content(rule: Self::Rule) -> &'static [Self::Scope];
    /// The `while` pattern of a rule that has one: the rule stays open on a
    /// later line only when the pattern matches at its start.
    fn rule_while(_rule: Self::Rule) -> Option<Self::Pat> {
        None
    }
    /// What a rule carries from line to line.
    fn rule_payload(_rule: Self::Rule) -> Payload {
        Payload::None
    }
    /// The prefix of a content scope that ends in the rule's payload text,
    /// for a rule whose content scope is named by it.
    fn rule_content_delimiter(_rule: Self::Rule) -> Option<&'static str> {
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
    /// cursor, with where its match starts and ends. A language that keeps
    /// the candidate tables of [`Tabled`] calls [`table_scan`].
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
}

/// The state at the start of a document of grammar `G` whose scopes sit on
/// the stack `base`: the root rule with [`Grammar::SOURCE`] on top of it.
pub(crate) fn start<G: Grammar>(arena: &mut Arena, base: u32) -> u32 {
    let stack = arena.child(base, G::scope(G::SOURCE)).unwrap_or(base);
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
) -> u32 {
    if line.text.is_empty() {
        return state;
    }
    let text = Text {
        bytes: line.text.as_bytes(),
        start: line.start,
        anchored: line.embedded,
    };
    let scratch = scratch(cx.scratch);
    Lexer::new(cx.arena, cx.limits, text, cx.out, state, cx.report, scratch).run()
}

/// The scopes of a pattern's capture groups by group number, and the groups
/// that the grammar tokenizes with patterns of their own.
pub(crate) type Captures<G> = (
    &'static [(u8, &'static [<G as Grammar>::Scope])],
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
}

/// A scope whose name ends in text of the line, such as the digits of `2>`:
/// the scope is `prefix`, the text `start..end`, and `suffix`.
#[derive(Clone, Copy, Debug)]
pub(crate) struct DynName {
    pub(crate) prefix: &'static str,
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) suffix: &'static str,
}

/// A run of a match that carries scopes of its own, or is lexed again.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Span<G: Grammar> {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) scopes: &'static [G::Scope],
    /// One more scope, after `scopes`, chosen when the matcher ran.
    pub(crate) single: Option<G::Scope>,
    pub(crate) dynamic: Option<DynName>,
    pub(crate) retok: Option<G::Rule>,
}

/// What a matching pattern does.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Act<G: Grammar> {
    /// Names the text and opens nothing.
    Match(&'static [G::Scope]),
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
    pub(crate) captures: &'static [(u8, &'static [G::Scope])],
    /// The capture groups whose text the grammar tokenizes with patterns of
    /// their own, by group number.
    pub(crate) nested: &'static [(u8, G::Nested)],
}

/// The data of one rule.
#[derive(Clone, Copy, Debug)]
pub(crate) struct RuleData<G: Grammar> {
    /// The scopes of its begin and end text and everything between.
    pub(crate) name: &'static [G::Scope],
    /// The scopes of what lies between its begin and end text.
    pub(crate) content: &'static [G::Scope],
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
