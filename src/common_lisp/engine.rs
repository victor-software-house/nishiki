//! Common Lisp on the shared rule engine ([`crate::rules`]): the grammar's
//! types and what its matchers remember about a line.

use std::cell::Cell;

use super::grammar::{Nested, Pat, Rule};
use super::scope::S;
use super::words::LONGEST;
use crate::rules::{self, Act, Captures, Grammar, Payload, ScopeName, table_scan};
use crate::scope::Scopes;

/// The Common Lisp lexer for one line.
pub(crate) type Lexer<'a> = rules::Lexer<'a, CommonLisp>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<CommonLisp>;

/// The run of characters before the next delimiter, `(`, `)` or `\s`, last
/// measured.
/// It holds where the run starts and ends, and the length of the text it was
/// measured in.
/// A capture group lexed again has a shorter text, so the length tells the
/// two apart.
#[derive(Clone, Copy, Debug)]
pub(super) struct Run {
    /// Where the run starts.
    pub(super) start: usize,
    /// Where the next delimiter stands, or the end of the text.
    pub(super) end: usize,
    /// The length of the text.
    pub(super) text: usize,
}

/// A run's characters as `(?i)` reads them, lowercase, for the word lists.
#[derive(Clone, Copy, Debug)]
pub(super) struct Lowered {
    /// Where the run starts.
    pub(super) start: usize,
    /// The length of the text it was lowered in.
    pub(super) text: usize,
    /// How many of `letters` it fills.
    /// It is `None` when the run holds a character above ASCII that folds to
    /// no ASCII letters, or is longer than every word.
    pub(super) len: Option<usize>,
    /// The letters.
    pub(super) letters: [u8; LONGEST],
}

/// The end of a failed search for a format directive's parameters.
/// The search from a `~` follows one path through the text.
/// A `~` inside that path is the character of a `'.` parameter, so a search
/// from it rejoins the path and fails where the first did.
#[derive(Clone, Copy, Debug)]
pub(super) struct Chain {
    /// The `~` the failed search started at.
    pub(super) tilde: usize,
    /// Where it failed.
    pub(super) fail: usize,
    /// The length of the text.
    pub(super) text: usize,
}

/// A search for the package and earmuffs of `'pkg:*name*` after a quote.
/// The first place in a run of `\S` where the package ends and the earmuffs
/// match does not depend on where the run starts.
#[derive(Clone, Copy, Debug)]
pub(super) struct Muffs {
    /// The first place the search tried.
    pub(super) from: usize,
    /// The end of the run of `\S` it searched.
    pub(super) to: usize,
    /// The length of the text.
    pub(super) text: usize,
    /// The first match found: where the colons start, where the earmuffs
    /// start, and where they end.
    pub(super) found: Option<(usize, usize, usize)>,
}

/// What the Common Lisp matchers remember about a line.
/// Each cache answers one measure that several patterns, or several places,
/// ask for.
#[derive(Debug)]
pub(crate) struct Extra {
    /// The last run measured.
    pub(super) run: Cell<Run>,
    /// The last run lowered.
    pub(super) lowered: Cell<Lowered>,
    /// The last failed parameter search of each format directive pattern:
    /// `StringAt1`, `StringAt2` and `StringAt3`.
    pub(super) chains: Cell<[Chain; 3]>,
    /// The last search for a quoted package's earmuffs.
    pub(super) muffs: Cell<Muffs>,
}

impl Default for Extra {
    fn default() -> Self {
        let chain = Chain {
            tilde: usize::MAX,
            fail: 0,
            text: 0,
        };
        Extra {
            run: Cell::new(Run {
                start: usize::MAX,
                end: 0,
                text: 0,
            }),
            lowered: Cell::new(Lowered {
                start: usize::MAX,
                text: 0,
                len: None,
                letters: [0; LONGEST],
            }),
            chains: Cell::new([chain; 3]),
            muffs: Cell::new(Muffs {
                from: usize::MAX,
                to: 0,
                text: 0,
                found: None,
            }),
        }
    }
}

/// The `common-lisp` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct CommonLisp;

impl Grammar for CommonLisp {
    type Scope = S;
    const NO_SCOPES: Scopes<S> = S::NO_SCOPES;
    type Rule = Rule;
    type Pat = Pat;
    type Nested = Nested;
    type Extra = Extra;
    type Kept = ();

    const NAME: &'static str = "Common Lisp";
    const ROOT: Rule = Rule::Root;
    const SOURCE: crate::scope::Scope = S::Source.scope();
    const GROUPS: usize = 13;
    const SKIPS_PLACES: bool = true;

    fn scope(scope: S) -> ScopeName {
        ScopeName::Fixed(scope.scope())
    }

    fn rule_code(rule: Rule) -> u16 {
        rule.code()
    }

    fn rule_from_code(code: u16) -> Rule {
        Rule::from_code(code)
    }

    fn rule_name(rule: Rule) -> Scopes<S> {
        rule.data().name
    }

    fn rule_content(rule: Rule) -> Scopes<S> {
        rule.data().content
    }

    fn rule_payload(_: Rule) -> Payload {
        Payload::None
    }

    fn pat_act(pat: Pat) -> Act<Self> {
        pat.data().act
    }

    fn pat_captures(pat: Pat) -> Captures<Self> {
        let data = pat.data();
        (data.captures, data.nested)
    }

    fn pat_data(pat: Pat) -> (Act<Self>, Captures<Self>) {
        let data = pat.data();
        (data.act, (data.captures, data.nested))
    }

    fn scan(lexer: &mut Lexer<'_>) -> Option<(usize, Pat, usize)> {
        table_scan(lexer)
    }

    fn next_place(lexer: &Lexer<'_>, p: usize) -> usize {
        lexer.after_miss(p)
    }

    fn matcher(lexer: &mut Lexer<'_>, pat: Pat, p: usize) -> Option<usize> {
        lexer.matcher(pat, p)
    }

    fn tokenize_nested(lexer: &mut Lexer<'_>, nested: Nested, from: usize, to: usize, base: u32) {
        lexer.retokenise(nested, from, to, base);
    }
}
