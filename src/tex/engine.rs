//! TeX on the shared rule engine ([`crate::rules`]): the grammar's types,
//! and where R, which `\Sexpr{…}` embeds, hands the line back.

use super::grammar::{Nested, Pattern, Rule, Sc};
use super::scope::S;
use crate::arena::Arena;
use crate::rules::{self, Act, Captures, CarriedState, FnHalt, Grammar, ScopeName, table_scan};
use crate::scope::Scopes;

/// The TeX lexer for one line.
pub(crate) type Lexer<'a> = rules::Lexer<'a, Tex>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<Tex>;

/// The `tex` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Tex;

impl Grammar for Tex {
    type Scope = Sc;
    const NO_SCOPES: Scopes<Sc> = Sc::NO_SCOPES;
    type Rule = Rule;
    type Pattern = Pattern;
    type Nested = Nested;
    type Extra = ();
    type Kept = ();

    const NAME: &'static str = "TeX";
    const ROOT: Rule = Rule::Root;
    const SOURCE: crate::scope::Scope = S::Source.scope();
    const GROUPS: usize = 5;

    fn scope(scope: Sc) -> ScopeName {
        match scope {
            Sc::S(scope) => ScopeName::Fixed(scope.scope()),
        }
    }

    fn rule_code(rule: Rule) -> u16 {
        rule.code()
    }

    fn rule_from_code(code: u16) -> Rule {
        Rule::from_code(code)
    }

    fn rule_name(rule: Rule) -> Scopes<Sc> {
        rule.definition().name
    }

    fn rule_content(rule: Rule) -> Scopes<Sc> {
        rule.definition().content
    }

    /// Math carries the `$` or `$$` that opened it, which its end repeats.
    /// `\Sexpr{…}`'s inner rule carries R.
    fn rule_carried_state(rule: Rule) -> CarriedState {
        rule.carried_state()
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
        lexer.tex_matcher(pattern.rx(), p)
    }

    /// Lexes the group again with its rule, under the stack the engine gives
    /// and the group's own scopes.
    /// TeX's tables name no such group, so the engine never calls it.
    fn tokenize_nested(lexer: &mut Lexer<'_>, nested: Nested, from: usize, to: usize, base: u32) {
        let inner = lexer.path(base, nested.scopes);
        lexer.retokenise(nested.rule, from, to, inner);
    }
}

/// `\Sexpr{…}` hands its text to R, which gives it back at the `}`.
impl rules::Embeds for Tex {
    type End = FnHalt;

    /// The one embedding rule picks its halt by the rule
    /// ([`rules::Embeds::embed_halts_for`]); this one never holds.
    fn embed_halts(_: &rules::Embedding, _: &Arena) -> (FnHalt, Option<FnHalt>) {
        (never, None)
    }

    fn embed_halts_for(rule: Rule, _: &rules::Embedding, _: &Arena) -> (FnHalt, Option<FnHalt>) {
        (rule.end_halt(), None)
    }
}

/// A halt that never holds: the generated `Rule::end_halt` of a rule that
/// embeds no language.
pub(super) fn never(_: &[u8], _: usize) -> bool {
    false
}

/// `}`: the end of `\Sexpr{…}`.
pub(super) fn closes_brace(s: &[u8], p: usize) -> bool {
    s.get(p) == Some(&b'}')
}
