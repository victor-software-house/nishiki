//! HAML on the shared rule engine ([`crate::rules`]): the grammar's types, and
//! where an embedded language hands the line back.

use super::grammar::{Nested, Pattern, Rule};
use super::matchers::halts;
use super::scope::S;
use crate::arena::Arena;
use crate::rules::{self, Act, Captures, CarriedState, Grammar, ScopeName, table_scan};
use crate::scope::Scopes;

/// The HAML lexer for one line.
pub(crate) type Lexer<'a> = rules::Lexer<'a, Haml>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<Haml>;

/// The `haml` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Haml;

impl Grammar for Haml {
    type Scope = S;
    const NO_SCOPES: Scopes<S> = S::NO_SCOPES;
    type Rule = Rule;
    type Pattern = Pattern;
    type Nested = Nested;
    /// The buffer for a filter's indent, which the engine fills at each
    /// hand-over ([`rules::Embeds::parent_text`]).
    type Extra = Vec<u8>;
    /// The same buffer, kept for its room.
    type Kept = Vec<u8>;

    const NAME: &'static str = "HAML";
    const ROOT: Rule = Rule::Root;
    const SOURCE: crate::scope::Scope = S::Text.scope();
    const GROUPS: usize = 4;

    fn extra(kept: Vec<u8>) -> Vec<u8> {
        kept
    }

    fn keep(extra: Vec<u8>) -> Vec<u8> {
        extra
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
        rule.definition().name
    }

    fn rule_content(rule: Rule) -> Scopes<S> {
        rule.definition().content
    }

    /// An interpolation can close inside an attribute hash or a Ruby line, and
    /// Ruby goes on after it.
    const EMBEDS: bool = true;

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
        lexer.matcher(pattern, p)
    }

    fn tokenize_nested(_: &mut Lexer<'_>, nested: Nested, _: usize, _: usize, _: u32) {
        match nested {}
    }
}

/// How a filter's end reads the indent its begin captured, `\1`.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Filter {
    /// `^(?!\1\s+|$\n*)`: a line that is neither empty nor indented past the
    /// filter.
    Unindented,
    /// `^(?=\1\s+|$\n*)`: a line that is empty or indented past the filter.
    /// The Sass and plain filters end so, at their first body line.
    Indented,
    /// `(?m:(?<=\n)(?!\1\s+|$\n*))`: never, since only the line feed's end
    /// follows a line feed, and `$` holds there.
    AfterLineFeed,
}

/// Where an embedding rule takes the line back from the language it embeds:
/// its end, and the patterns it lists before that language.
#[derive(Clone, Copy, Debug)]
pub(crate) enum End {
    /// A filter's body: the filter's end, which repeats its indent.
    Filter(Filter),
    /// A Ruby line: its end, and the PHP keywords, the block opener and the
    /// comment it lists before Ruby.
    RubyLine,
    /// A tag's attribute hash: its closing `}`.
    Attributes,
    /// A tag's object: its last `]`.
    Object,
    /// An interpolation: its closing `}`, or a `{` that opens a nested pair.
    Interpolation,
    /// A nested pair of braces in an interpolation: its `}`, or another `{`.
    Curly,
    /// A rule that embeds nothing.
    Never,
}

impl End {
    /// Whether the end holds at `p` of `s`, for a filter whose indent is
    /// `indent`.
    pub(crate) fn holds_at(self, indent: &[u8], s: &[u8], p: usize) -> bool {
        match self {
            End::Filter(Filter::Unindented) => p == 0 && !halts::stays_in_filter(indent, s),
            End::Filter(Filter::Indented) => p == 0 && halts::stays_in_filter(indent, s),
            End::Filter(Filter::AfterLineFeed) | End::Never => false,
            End::RubyLine => halts::ruby_line(s, p),
            End::Attributes => halts::attributes_end(s, p),
            End::Object => halts::object_end(s, p),
            End::Interpolation | End::Curly => matches!(s.get(p), Some(b'{' | b'}')),
        }
    }
}

/// The engine calls [`rules::Embeds::end_holds`] instead, with the filter's
/// indent ([`rules::EndText::ParentBeginText`]).
/// Asked directly, the end reads an empty indent.
impl crate::language::Halt for End {
    fn first(&self, s: &[u8], from: usize, to: usize) -> Option<usize> {
        (from..=to).find(|&p| self.holds_at(&[], s, p))
    }
}

impl rules::Embeds for Haml {
    type End = End;
    const HALTS: bool = true;

    fn embed_halts(_: &rules::Embedding, _: &Arena) -> (End, Option<rules::FnHalt>) {
        (End::Never, None)
    }

    fn embed_halts_for(
        rule: Rule,
        _: &rules::Embedding,
        _: &Arena,
    ) -> (End, Option<rules::FnHalt>) {
        (rule.end_halt(), None)
    }

    fn embeds_after_own(rule: Rule) -> bool {
        rule.embeds_after_own()
    }

    /// A filter's end repeats the indent its begin captured, which the
    /// filter's frame, the parent of the body that embeds, carries.
    const END_TEXT: rules::EndText = rules::EndText::ParentBeginText;

    fn end_holds(end: &End, parent_text: &[u8], s: &[u8], p: usize) -> bool {
        end.holds_at(parent_text, s, p)
    }

    fn parent_text(kept: &mut Vec<u8>) -> Option<&mut Vec<u8>> {
        Some(kept)
    }
}
