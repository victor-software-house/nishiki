//! Assembly on the shared rule engine ([`crate::rules`]): the grammar's types
//! and what its matchers remember about a line.

use std::cell::Cell;

use super::grammar::{Nested, Pat, Rule};
use super::scope::S;
use super::words::LONGEST;
use crate::rules::{self, Act, BlankRuns, Captures, Grammar, Payload, ScopeName, table_scan};
use crate::scope::Scopes;

/// The Assembly lexer for one line.
pub(crate) type Lexer<'a> = rules::Lexer<'a, Asm>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<Asm>;

/// The run of `\w` characters last lowered.
/// It holds where the run starts and ends, the length of the text it was
/// read in, and its letters as `(?i)` reads them.
#[derive(Clone, Copy, Debug)]
pub(super) struct Lowered {
    /// Where the run starts.
    pub(super) start: usize,
    /// Where it ends.
    pub(super) end: usize,
    /// The length of the text.
    pub(super) text: usize,
    /// How many of `letters` it fills.
    /// It is `None` when the run holds a character above ASCII that folds to
    /// no ASCII letters, or is longer than every word.
    pub(super) len: Option<usize>,
    /// The letters, lowercase.
    pub(super) letters: [u8; LONGEST],
    /// Whether the run holds `ß`, `ẞ` or a Latin ligature.
    pub(super) spelling: Spelling,
}

/// How a run of `\w` is spelled, for the word lists that ignore case.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Spelling {
    /// Without `ß`, `ẞ` or a Latin ligature: its lowered letters decide
    /// whether a list holds it.
    Letters,
    /// With one of them.
    /// Oniguruma folds such a character to its letters only inside one
    /// literal string of an expression.
    /// The spellings that the generator checked against Oniguruma therefore
    /// decide.
    Ligature,
}

/// The run of label characters, `[#$.?@_~[:alnum:]]`, last measured: where
/// it starts and ends, and the length of the text it was measured in.
/// A label needs no word boundary before it.
/// The label patterns therefore try every character of a run.
/// Every place in the run shares its end.
#[derive(Clone, Copy, Debug)]
pub(super) struct LabelRun {
    /// Where the run starts.
    pub(super) start: usize,
    /// Where it ends.
    pub(super) end: usize,
    /// The length of the text.
    pub(super) text: usize,
}

/// What the Assembly matchers remember about a line.
#[derive(Debug)]
pub(crate) struct Extra {
    /// The last run of `\w` lowered.
    /// The merged word pattern and six support patterns all test the same
    /// run.
    pub(super) lowered: Cell<Lowered>,
    /// The last run of label characters measured.
    pub(super) label: Cell<LabelRun>,
}

impl Default for Extra {
    fn default() -> Self {
        Extra {
            lowered: Cell::new(Lowered {
                start: usize::MAX,
                end: 0,
                text: 0,
                len: None,
                letters: [0; LONGEST],
                spelling: Spelling::Letters,
            }),
            label: Cell::new(LabelRun {
                start: usize::MAX,
                end: 0,
                text: 0,
            }),
        }
    }
}

/// The `asm` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Asm;

impl Grammar for Asm {
    type Scope = S;
    const NO_SCOPES: Scopes<S> = S::NO_SCOPES;
    type Rule = Rule;
    type Pat = Pat;
    type Nested = Nested;
    type Extra = Extra;
    type Kept = ();

    const NAME: &'static str = "Assembly";
    const ROOT: Rule = Rule::Root;
    const SOURCE: crate::scope::Scope = S::Source.scope();
    const GROUPS: usize = 4;
    /// A comment can start at the blank before its `#`, as `(^|\s)#\s` reads
    /// ([`Pat::takes_one_blank`]).
    const BLANK_RUNS: BlankRuns = BlankRuns::ToLast;

    fn takes_one_blank(pat: Pat) -> bool {
        pat.takes_one_blank()
    }

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

    fn matcher(lexer: &mut Lexer<'_>, pat: Pat, p: usize) -> Option<usize> {
        lexer.matcher(pat, p)
    }

    fn tokenize_nested(_: &mut Lexer<'_>, nested: Nested, _: usize, _: usize, _: u32) {
        match nested {}
    }
}
