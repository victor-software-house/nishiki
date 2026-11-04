//! Less on the shared rule engine ([`crate::rules`]): the grammar's types,
//! and what the matchers remember about a line.

use std::cell::{Cell, OnceCell};

use super::grammar::{Pat, Rule};
use super::matchers::{Class, Escape};
use super::scope::S;
use crate::rules::{self, Act, Captures, Grammar, Payload, ScopeName, table_scan};
use crate::scope::Scopes;

/// The Less lexer for one line.
pub(crate) type Lexer<'a> = rules::Lexer<'a, Less>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<Less>;

/// What the Less matchers remember about a line, so that a matcher tried at
/// each place of a long run does not read the run again each time.
#[derive(Debug)]
pub(crate) struct Extra {
    /// The last run of a name's characters read without an escape: its
    /// class and escape, where it was read from and where it ends.
    pub(in crate::less) units: Cell<(Class, Escape, usize, usize)>,
    /// The last run of white space read: where it was read from and where
    /// it ends.
    pub(in crate::less) spaces: Cell<(usize, usize)>,
    /// The last run of `\d` read: where it was read from and where it ends.
    pub(in crate::less) digits: Cell<(usize, usize)>,
    /// The last run of `[0-9]` read: where it was read from and where it ends.
    pub(in crate::less) ascii_digits: Cell<(usize, usize)>,
    /// The last run of `[-\w]` read: where it was read from and where it ends.
    pub(in crate::less) dash_words: Cell<(usize, usize)>,
    /// The last custom element's run read: where it was read from, where it
    /// ends, the last end in it where `\b` holds, and the last `-` before
    /// that end.
    pub(in crate::less) custom: Cell<(usize, usize, Option<usize>, Option<usize>)>,
    /// The last `{` of the line, once looked for.
    pub(in crate::less) brace: OnceCell<Option<usize>>,
    /// The last `}` of the line that an interpolated property's lookahead
    /// accepts, once looked for.
    pub(in crate::less) interpolated: OnceCell<Option<usize>>,
}

impl Default for Extra {
    fn default() -> Self {
        Extra {
            units: Cell::new((Class::DashWord, Escape::Hex, usize::MAX, 0)),
            spaces: Cell::new((usize::MAX, 0)),
            digits: Cell::new((usize::MAX, 0)),
            ascii_digits: Cell::new((usize::MAX, 0)),
            dash_words: Cell::new((usize::MAX, 0)),
            custom: Cell::new((usize::MAX, 0, None, None)),
            brace: OnceCell::new(),
            interpolated: OnceCell::new(),
        }
    }
}

/// The `less` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Less;

impl Grammar for Less {
    type Scope = S;
    const NO_SCOPES: Scopes<S> = S::NO_SCOPES;
    type Rule = Rule;
    type Pat = Pat;
    type Nested = Rule;
    type Extra = Extra;
    type Kept = ();

    const NAME: &'static str = "Less";
    const ROOT: Rule = Rule::Root;
    const SOURCE: crate::scope::Scope = S::SourceCss.scope();
    const GROUPS: usize = 5;
    const CAPTURED_EOL: bool = true;

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

    #[inline]
    fn matcher(lexer: &mut Lexer<'_>, pat: Pat, p: usize) -> Option<usize> {
        lexer.less_matcher(pat, p)
    }

    fn tokenize_nested(lexer: &mut Lexer<'_>, rule: Rule, from: usize, to: usize, base: u32) {
        lexer.retokenise(rule, from, to, base);
        lexer.emit(to, base);
    }
}
