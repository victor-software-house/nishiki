//! SCSS on the shared rule engine ([`crate::rules`]): the grammar's types,
//! and what the matchers remember about a line.
//! The patterns that `source.css#…` brings in run on CSS's matchers.
//! A line therefore keeps CSS's memory for them too.

use std::cell::Cell;

use super::grammar::{Nested, Pattern, Rule};
use super::scope::S;
use crate::css::CssLike;
use crate::rules::{self, Act, Captures, Grammar, NameCase, Payload, ScopeName, table_scan};
use crate::scope::Scopes;

/// The SCSS lexer for one line.
pub(crate) type Lexer<'a> = rules::Lexer<'a, Scss>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<Scss>;

/// What the SCSS matchers remember about a line.
#[derive(Debug)]
pub(crate) struct Extra {
    /// CSS's, for the patterns `source.css#…` brings in.
    pub(crate) css: crate::css::Extra,
    /// The last chain of a custom element's parts read: where it was read
    /// from, where it ends, and the last end the lookahead accepted.
    /// A match from a later part of the chain ends there too, when it can.
    pub(crate) custom: Cell<(usize, usize, Option<usize>)>,
    /// The last selector name read: its sigil, where it was read from,
    /// where its units end, and the last end the lookahead accepted.
    pub(crate) name: Cell<(u8, usize, usize, Option<usize>)>,
    /// The last variable name read after `$` or `--`: where it starts and
    /// ends, and the last `\b` in it.
    pub(crate) variable: Cell<(usize, usize, Option<usize>)>,
    /// The last search for a quote that `\s*]` follows: the quote, where
    /// the search started, and what it found.
    pub(crate) quote: Cell<(u8, usize, Option<usize>)>,
}

impl Default for Extra {
    fn default() -> Self {
        Extra {
            css: crate::css::Extra::default(),
            custom: Cell::new((usize::MAX, 0, None)),
            name: Cell::new((0, usize::MAX, 0, None)),
            variable: Cell::new((usize::MAX, 0, None)),
            quote: Cell::new((0, usize::MAX, None)),
        }
    }
}

/// The `scss` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Scss;

impl Grammar for Scss {
    type Scope = S;
    const NO_SCOPES: Scopes<S> = S::NO_SCOPES;
    type Rule = Rule;
    type Pattern = Pattern;
    type Nested = Nested;
    type Extra = Extra;
    type Kept = ();

    const NAME: &'static str = "SCSS";
    const ROOT: Rule = Rule::Root;
    const SOURCE: crate::scope::Scope = S::Source.scope();
    const GROUPS: usize = 14;
    const CAPTURED_EOL: bool = true;

    fn scope(scope: S) -> ScopeName {
        ScopeName::Fixed(scope.scope())
    }

    fn dyn_name_case(prefix: &'static str) -> NameCase {
        crate::css::unit_name_case(prefix)
    }

    fn rule_code(rule: Rule) -> u16 {
        rule.code()
    }

    fn rule_from_code(code: u16) -> Rule {
        Rule::from_code(code)
    }

    fn rule_name(rule: Rule) -> Scopes<S> {
        rule.definition().name
    }

    fn rule_content(rule: Rule) -> Scopes<S> {
        rule.definition().content
    }

    fn rule_payload(rule: Rule) -> Payload {
        rule.definition()
            .backref
            .map_or(Payload::None, Payload::Backref)
    }

    fn pattern_act(pattern: Pattern) -> Act<Self> {
        pattern.definition().act
    }

    fn pattern_captures(pattern: Pattern) -> Captures<Self> {
        let definition = pattern.definition();
        (definition.captures, definition.nested)
    }

    fn pattern_definition(pattern: Pattern) -> (Act<Self>, Captures<Self>) {
        let definition = pattern.definition();
        (definition.act, (definition.captures, definition.nested))
    }

    fn scan(lexer: &mut Lexer<'_>) -> Option<(usize, Pattern, usize)> {
        table_scan(lexer)
    }

    fn matcher(lexer: &mut Lexer<'_>, pattern: Pattern, p: usize) -> Option<usize> {
        match pattern.css() {
            Some(css) => lexer.css_matcher(css, p),
            None => lexer.scss_matcher(pattern, p),
        }
    }

    fn tokenize_nested(lexer: &mut Lexer<'_>, nested: Nested, from: usize, to: usize, base: u32) {
        match nested {
            Nested::Rule(rule) => {
                lexer.retokenise(rule, from, to, base);
                lexer.emit(to, base);
            }
            Nested::Css(css) => lexer.css_capture(css, from, to, base),
        }
    }
}

impl CssLike for Scss {
    const UNIT_SCOPES: &'static [S] = &super::grammar::UNIT_SCOPES;

    #[inline]
    fn css_extra(extra: &Extra) -> &crate::css::Extra {
        &extra.css
    }

    fn css_path(lexer: &mut Lexer<'_>, base: u32, scopes: Scopes<crate::css::S>) -> u32 {
        scopes
            .list
            .iter()
            .fold(base, |stack, scope| lexer.path_fixed(stack, scope.scope()))
    }
}
