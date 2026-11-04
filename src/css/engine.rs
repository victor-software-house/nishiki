//! CSS on the shared rule engine ([`crate::rules`]).
//! The module holds the grammar's types and the answers its lookaheads keep
//! for a line.
//! Those answers keep the scan of a run of the line linear.

use std::cell::Cell;

use super::ahead::Ahead;
use super::grammar::{Nested, Pat, Rule};
use super::scope::S;
use crate::rules::{self, Act, Captures, Grammar, NameCase, Payload, ScopeName, table_scan};
use crate::scope::Scopes;

/// The CSS lexer for one line.
pub(crate) type Lexer<'a> = rules::Lexer<'a, Css>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<Css>;

/// The `css` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Css;

/// The start of a unit's scope name, `keyword.other.unit.${2:/downcase}.css`,
/// which the grammar lowers ([`unit_name_case`]).
pub(crate) const UNIT_PREFIX: &str = "keyword.other.unit.";

/// How the grammar spells the text of a dynamic name with `prefix`.
/// A unit's text is lowered, as `${2:/downcase}` does.
/// Any other text is kept as written.
/// This is CSS's and SCSS's [`Grammar::dyn_name_case`].
pub(crate) fn unit_name_case(prefix: &'static str) -> NameCase {
    if prefix == UNIT_PREFIX {
        NameCase::Lowered
    } else {
        NameCase::AsWritten
    }
}

/// A grammar that runs CSS's matchers on its lines: CSS, and SCSS.
/// SCSS's grammar includes CSS's rules.
/// The grammar's per-line memory holds CSS's.
pub(crate) trait CssLike: Grammar {
    /// The CSS matchers' memory within the grammar's.
    fn css_extra(extra: &Self::Extra) -> &Extra;
    /// The stack `base` with CSS's scopes `scopes` on top.
    fn css_path(lexer: &mut rules::Lexer<'_, Self>, base: u32, scopes: Scopes<S>) -> u32;
    /// The scope that names each of CSS's units, in the order of `UNITS`.
    const UNIT_SCOPES: &'static [Self::Scope];
}

impl CssLike for Css {
    const UNIT_SCOPES: &'static [S] = &super::grammar::UNIT_SCOPES;

    #[inline]
    fn css_extra(extra: &Extra) -> &Extra {
        extra
    }

    fn css_path(lexer: &mut Lexer<'_>, base: u32, scopes: Scopes<S>) -> u32 {
        lexer.path(base, scopes)
    }
}

/// What the CSS matchers remember about a line.
#[derive(Debug)]
pub(crate) struct Extra {
    /// The last run of `[-\w]` characters found: where it starts and ends.
    /// A search from inside the run ends where the run does.
    pub(crate) word_run: Cell<(usize, usize)>,
    /// The last run of identifier characters found.
    /// It is kept the same way when it has no escape.
    /// An escape would make a search from inside the run differ.
    pub(crate) ident_run: Cell<(usize, usize)>,
    /// The last run of white space found, kept the same way.
    pub(crate) space_run: Cell<(usize, usize)>,
    /// The last number found: where the search started, and where the number
    /// ends.
    /// The patterns of its units then ask once.
    pub(crate) number: Cell<(usize, Option<usize>)>,
    /// The last word looked up in the word table: where it starts and ends,
    /// and the bits of the lists that hold it.
    pub(crate) listed: Cell<(usize, usize, u32)>,
    /// The last word after a number looked up among the units: where it
    /// starts and ends, and the unit it spells.
    pub(crate) unit: Cell<(usize, usize, Option<usize>)>,
    /// Where the bytes the lookaheads search for are.
    pub(crate) ahead: Ahead,
}

impl Default for Extra {
    fn default() -> Self {
        Extra {
            word_run: Cell::new((usize::MAX, 0)),
            ident_run: Cell::new((usize::MAX, 0)),
            space_run: Cell::new((usize::MAX, 0)),
            number: Cell::new((usize::MAX, None)),
            listed: Cell::new((usize::MAX, 0, 0)),
            unit: Cell::new((usize::MAX, 0, None)),
            ahead: Ahead::default(),
        }
    }
}

impl Grammar for Css {
    type Scope = S;
    const NO_SCOPES: Scopes<S> = S::NO_SCOPES;
    type Rule = Rule;
    type Pat = Pat;
    type Nested = Nested;
    type Extra = Extra;
    type Kept = ();

    const NAME: &'static str = "CSS";
    const ROOT: Rule = Rule::Root;
    const MARKS_FIRST_LINE: bool = true;
    const SOURCE: crate::scope::Scope = S::Source.scope();
    const GROUPS: usize = 8;
    const CAPTURED_EOL: bool = true;

    fn scope(scope: S) -> ScopeName {
        ScopeName::Fixed(scope.scope())
    }

    fn dyn_name_case(prefix: &'static str) -> NameCase {
        unit_name_case(prefix)
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

    fn rule_payload(rule: Rule) -> Payload {
        rule.data().backref.map_or(Payload::None, Payload::Backref)
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
        lexer.css_matcher(pat, p)
    }

    fn tokenize_nested(lexer: &mut Lexer<'_>, kind: Nested, from: usize, to: usize, base: u32) {
        lexer.css_capture(kind, from, to, base);
    }
}
