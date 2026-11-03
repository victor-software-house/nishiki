//! The rule engine that the languages written from a `TextMate` grammar share:
//! Python and Java run on it, and each language supplies a [`Grammar`], the
//! generated tables of its rules and patterns, and the hand-written matchers
//! that decide where each pattern matches.
//!
//! The engine runs the grammar's rules the way a `TextMate` engine does, with
//! those matchers instead of regular expressions: the per-line loop, `\G`,
//! capture groups, the limits and the frames that carry a rule from line to
//! line are here, once.

mod chars;
mod lexer;
mod tables;

use std::fmt::Debug;

pub(crate) use chars::Chars;
#[cfg(feature = "lang-css")]
pub(crate) use lexer::FIRST_LINE;
pub(crate) use lexer::{Lexer, Scratch};
pub(crate) use tables::{Place, Tables};

use crate::scope::Scope;

/// The most capture groups a pattern of a grammar has.
pub(crate) const MAX_GROUPS: usize = 12;

/// One language's grammar: its tables and matchers, by type.
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

    /// The language's name, for diagnostics.
    const NAME: &'static str;
    /// The rule that is open at the start of a document.
    const ROOT: Self::Rule;

    /// The scope in the crate's shared numbering.
    fn scope(scope: Self::Scope) -> Scope;
    /// The rule as a frame stores it.
    fn rule_code(rule: Self::Rule) -> u16;
    /// The rule a frame's code names; [`Grammar::ROOT`] for a code that is
    /// not the language's.
    fn rule_from_code(code: u16) -> Self::Rule;
    /// The data of `rule`.
    fn rule_data(rule: Self::Rule) -> RuleData<Self>;
    /// The data of `pat`.
    fn pat_data(pat: Self::Pat) -> PatData<Self>;
    /// The bytes `pat` can start at.
    fn pat_start(pat: Self::Pat) -> Start;
    /// The tables of `rule`, built on first use.
    fn tables(rule: Self::Rule) -> &'static Tables<Self>;
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
///
/// `anchored` is set for a Markdown fence line: the fence's `while` rule has
/// matched the container prefix just before `start`, which is where `\G`
/// stays for the line, as it does in Shiki.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Text<'a> {
    pub(crate) bytes: &'a [u8],
    pub(crate) start: usize,
    pub(crate) anchored: bool,
}
