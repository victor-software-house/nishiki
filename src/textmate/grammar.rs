//! The compiled form of a `TextMate` grammar: static tables that
//! `oracle/engine-tables.ts` writes, with the includes already followed.
//!
//! Nothing is parsed at run time. A pattern's text is compiled to a regular
//! expression the first time a search needs it.

use std::sync::OnceLock;

use super::regex::Regex;

/// The index that stands for no rule, pattern or name.
pub(crate) const NONE: usize = usize::MAX;

/// A scope-name reference with this bit set names a scope built from the
/// capture groups of a match, such as `keyword.$1`.
pub(crate) const DYNAMIC_NAME: u16 = 0x8000;

/// What a rule does when its pattern matches.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RuleKind {
    /// One pattern that scopes its text.
    Match,
    /// A `begin` pattern that opens a context and an `end` pattern that closes
    /// it.
    BeginEnd,
    /// A `begin` pattern that opens a context that lasts while a `while`
    /// pattern keeps matching at the start of each line.
    BeginWhile,
    /// Only a list of patterns: the root of a grammar, or a capture's rules.
    Container,
}

/// What a capture group of a match does.
#[derive(Debug)]
pub(crate) struct Cap {
    /// The group's number.
    pub(crate) index: usize,
    /// The scopes its text gets.
    pub(crate) name: &'static [u16],
    /// The scopes the rules inside it get, when it holds some.
    pub(crate) content_name: &'static [u16],
    /// The rule that lexes its text again, or [`NONE`].
    pub(crate) retokenize: usize,
}

/// One rule of a grammar.
#[derive(Debug)]
pub(crate) struct Rule {
    /// What the rule does.
    pub(crate) kind: RuleKind,
    /// The scopes the rule's text gets.
    pub(crate) name: &'static [u16],
    /// The scopes the rules inside a context get.
    pub(crate) content_name: &'static [u16],
    /// The pattern that opens the rule: the only one of a `Match`.
    pub(crate) begin: usize,
    /// The pattern that closes a `BeginEnd`.
    pub(crate) end: usize,
    /// The pattern a `BeginWhile` checks at the start of each line.
    pub(crate) until: usize,
    /// The groups of the `begin` or `match` pattern.
    pub(crate) captures: &'static [Cap],
    /// The groups of the `end` or `while` pattern.
    pub(crate) end_captures: &'static [Cap],
    /// The rules that can start inside the rule, in order, includes followed.
    pub(crate) patterns: &'static [usize],
    /// Whether the `end` pattern is tried after the patterns, not before.
    pub(crate) apply_end_last: bool,
}

/// Whether `source` holds a back-reference `\1`..`\9`, which an `end` or
/// `while` pattern fills in from the `begin` match.
const fn has_backrefs(source: &str) -> bool {
    let bytes = source.as_bytes();
    let mut i = 0;
    while i + 1 < bytes.len() {
        if bytes[i] == b'\\' {
            if bytes[i + 1] >= b'1' && bytes[i + 1] <= b'9' {
                return true;
            }
            i += 1;
        }
        i += 1;
    }
    false
}

/// A pattern's text and, once a search needs it, its compiled form.
#[derive(Debug)]
pub(crate) struct PatternSlot {
    /// The pattern as the grammar spells it.
    pub(crate) source: &'static str,
    /// Whether it holds `\1`..`\9`, so an `end` pattern is built per match.
    pub(crate) backrefs: bool,
    regex: OnceLock<Option<Regex>>,
}

impl PatternSlot {
    /// A slot for `source`.
    pub(crate) const fn new(source: &'static str) -> PatternSlot {
        PatternSlot {
            source,
            backrefs: has_backrefs(source),
            regex: OnceLock::new(),
        }
    }

    /// The compiled pattern, or `None` for text the engine cannot compile,
    /// which never matches.
    pub(crate) fn regex(&self) -> Option<&Regex> {
        self.regex
            .get_or_init(|| Regex::new(self.source).ok())
            .as_ref()
    }
}

/// A compiled grammar.
#[derive(Debug)]
pub(crate) struct Grammar {
    /// The root scope, such as `source.rust`.
    pub(crate) scope: &'static str,
    /// Every scope name the rules use.
    pub(crate) names: &'static [&'static str],
    /// The rules.
    pub(crate) rules: &'static [Rule],
    /// The patterns.
    pub(crate) patterns: &'static [PatternSlot],
    /// The rule of the whole grammar.
    pub(crate) root: usize,
}
