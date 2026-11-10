//! Svelte on the shared rule engine ([`crate::rules`]): the grammar's types,
//! and where each embedding rule takes its line back.

use super::grammar::{Pattern, Rule};
use super::scope::S;
use crate::Language;
use crate::arena::Arena;
use crate::language::{Context, Halt, Line, Stopped};
use crate::limits::Finished;
use crate::rules::{
    self, Act, Captures, Deferrals, Embedding, Embeds, FnHalt, Grammar, Payload, ScopeName,
    table_scan,
};
use crate::scope::Scopes;
use crate::typescript::Dialect;

/// The Svelte lexer for one line.
pub(crate) type Lexer<'a> = rules::Lexer<'a, Svelte>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<Svelte>;

/// The `svelte` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Svelte;

impl Grammar for Svelte {
    type Scope = S;
    const NO_SCOPES: Scopes<S> = S::NO_SCOPES;
    type Rule = Rule;
    type Pattern = Pattern;
    type Nested = Rule;
    /// The capture groups the line defers ([`Grammar::deferred`]).
    type Extra = Option<Box<Deferrals>>;
    /// The same groups between the run and the engine's splice, and their
    /// buffers from line to line.
    type Kept = Option<Box<Deferrals>>;

    const NAME: &'static str = "Svelte";
    const ROOT: Rule = Rule::Root;
    const SOURCE: crate::scope::Scope = S::Source.scope();
    const GROUPS: usize = 7;
    /// An interpolation's object literal hands TypeScript back to the
    /// interpolation mid-line, which then embeds TypeScript again.
    const EMBEDS: bool = true;
    const WHILES: bool = true;

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

    fn rule_while(rule: Rule) -> Option<Pattern> {
        rule.while_pattern()
    }

    fn rule_payload(rule: Rule) -> Payload {
        rule.payload()
    }

    fn rule_name_delimiter(rule: Rule) -> Option<&'static str> {
        rule.name_delimiter()
    }

    fn rule_name_suffix(rule: Rule) -> &'static str {
        rule.name_suffix()
    }

    fn rule_name_after(rule: Rule) -> Scopes<S> {
        rule.name_after()
    }

    fn embed_start(rule: Rule) -> Option<fn(&mut Arena, u32) -> u32> {
        rule.embed_start()
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

    /// A capture group with patterns of its own: the patterns of its nested
    /// rule, run over the group.
    fn tokenize_nested(lexer: &mut Lexer<'_>, nested: Rule, from: usize, to: usize, base: u32) {
        lexer.retokenise(nested, from, to, base);
        lexer.emit(to, base);
    }

    /// TypeScript and Markdown lex some capture groups again
    /// ([`Embeds::lex_nested`]).
    const DEFERS_NESTED: bool = true;

    fn defers_nested(nested: Rule) -> Option<Language> {
        if nested == Rule::NestedTypeScript {
            Some(Language::TypeScript)
        } else if nested == Rule::NestedMarkdown {
            crate::rules::embedded!(Markdown, "lang-markdown")
        } else {
            None
        }
    }

    fn extra(kept: Self::Kept) -> Self::Extra {
        kept
    }

    fn keep(extra: Self::Extra) -> Self::Kept {
        extra
    }

    fn deferred(extra: &mut Self::Extra) -> Option<&mut Option<Box<Deferrals>>> {
        Some(extra)
    }

    fn deferred_kept(kept: &mut Self::Kept) -> Option<&mut Option<Box<Deferrals>>> {
        Some(kept)
    }
}

impl Embeds for Svelte {
    type End = End;

    /// Every embedding rule picks its halts by the rule
    /// ([`Embeds::embed_halts_for`]); this one never holds.
    fn embed_halts(embedding: &Embedding, _: &Arena) -> (End, Option<FnHalt>) {
        (
            End {
                rule: Rule::Root,
                g: embedding.anchored.then_some(embedding.at),
            },
            None,
        )
    }

    /// TypeScript, JavaScript and CoffeeScript with Svelte's `$` prefixes
    /// injected (`L:(source.ts, source.js, source.coffee)`).
    /// Every other language lexes itself.
    /// So does the TypeScript of a `{@const}` tag's type annotation.
    /// Its scope, `meta.type.annotation.svelte`, has no `source.ts` for the
    /// injection to select.
    fn lex_embedded_until(
        rule: Rule,
        language: Language,
        cx: &mut Context<'_>,
        line: Line<'_>,
        state: u32,
        (end, after): (&dyn Halt, Option<&dyn Halt>),
    ) -> Option<(Finished, Option<Stopped>)> {
        if rule == Rule::SpecialTagsModesAt2At2 {
            return None;
        }
        let dialect = if language == Language::TypeScript {
            Dialect::TypeScript
        } else if language == Language::JavaScript {
            Dialect::JavaScript
        } else {
            #[cfg(feature = "lang-coffee")]
            if matches!(language, Language::CoffeeScript) {
                return Some(coffee_until(cx, line, state, (end, after)));
            }
            return None;
        };
        Some(crate::typescript::lex_until_svelte(
            cx, dialect, line, state, end, after,
        ))
    }

    /// A `{@debug}` name or an `{#each}` index goes to TypeScript, with
    /// Svelte's `$` prefixes.
    /// The text of an `@component` comment's line goes to Markdown, the only
    /// other language [`Grammar::defers_nested`] names, when the text starts
    /// the line.
    /// Elsewhere it stays plain, as in Shiki: Markdown's block patterns need
    /// `^` or `\G`, and neither holds inside the line.
    fn lex_nested(
        language: Language,
        cx: &mut Context<'_>,
        line: Line<'_>,
        base: u32,
    ) -> Option<Finished> {
        if language == Language::TypeScript {
            Some(crate::typescript::lex_capture_svelte(cx, line, base))
        } else if line.start == 0 {
            let state = language.start_embedded(cx.arena, base);
            Some(language.lex_embedded(cx, line, state))
        } else {
            None
        }
    }

    /// The end of the embedding rule, and the patterns it lists before the
    /// embedded grammar. No embedding rule lists patterns after it.
    fn embed_halts_for(rule: Rule, embedding: &Embedding, _: &Arena) -> (End, Option<FnHalt>) {
        (
            End {
                rule,
                g: embedding.anchored.then_some(embedding.at),
            },
            None,
        )
    }
}

/// A CoffeeScript body with Svelte's `$` prefixes.
/// Out of line and cold, so the TypeScript path of
/// [`Embeds::lex_embedded_until`] keeps the shape it had.
/// With the call inside it, it left `hand_over` and cost Svelte 0.38%.
#[cfg(feature = "lang-coffee")]
#[cold]
#[inline(never)]
fn coffee_until(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    (end, after): (&dyn Halt, Option<&dyn Halt>),
) -> (Finished, Option<Stopped>) {
    crate::coffee::lex_until_svelte(cx, line, state, end, after)
}

/// Where an embedding rule takes its line back from the language it embeds:
/// its end pattern, or a pattern it lists before the embedded grammar.
/// `g` is where `\G` holds: where the embedded text starts, when a match
/// ended there on this line.
/// At the start of a line that goes on inside the rule, `\G` holds nowhere.
#[derive(Clone, Copy, Debug)]
pub(crate) struct End {
    rule: Rule,
    g: Option<usize>,
}

impl Halt for End {
    fn first(&self, s: &[u8], from: usize, to: usize) -> Option<usize> {
        (from..=to).find(|&p| self.holds_at(s, p))
    }
}

impl End {
    /// Whether the rule's end, or a pattern before its embedded grammar,
    /// matches at `p`.
    fn holds_at(self, s: &[u8], p: usize) -> bool {
        let rest = s.get(p..).unwrap_or_default();
        let after = |text: &[u8]| rest.starts_with(text);
        match self.rule {
            Rule::BodyJavaScript
            | Rule::BodyCoffeeScript
            | Rule::BodyCss
            | Rule::BodyPostCss
            | Rule::BodyScss
            | Rule::BodyLess
            | Rule::BodySass => after(b"</"),
            Rule::CommentsAt0 => rest.windows(3).any(|w| w == b"-->"),
            Rule::BodyTypeScriptLine => closing_tag(rest),
            Rule::SpecialTagsModesAt0
            | Rule::SpecialTagsModesAt1At2
            | Rule::SpecialTagsModesAt1At3
            | Rule::SpecialTagsModesAt2At3
            | Rule::SpecialTagsModesAt4At1
            | Rule::AttributesAttach
            | Rule::AttributesInterpolated => after(b"}"),
            Rule::SpecialTagsModesAt1At1 => after(b">"),
            Rule::SpecialTagsModesAt2At2 => after(b"="),
            Rule::SpecialTagsModesAt3At0 => each_expression_end(s, p),
            Rule::SpecialTagsModesAt3At1At1 => after(b")") || after(b"}"),
            Rule::SpecialTagsModesAt4At0 => await_expression_end(rest),
            Rule::DestructuringAt0 | Rule::DestructuringConstAt0 | Rule::InterpolationAt0At0 => {
                p > 0 && s.get(p - 1) == Some(&b'}')
            }
            Rule::DestructuringAt1 | Rule::DestructuringConstAt1 => {
                p > 0 && s.get(p - 1) == Some(&b']')
            }
            Rule::InterpolationAt0 => after(b"}") || (self.g == Some(p) && object_ahead(rest)),
            Rule::GenericsDoubleQuoted => after(b"\""),
            Rule::GenericsSingleQuoted => after(b"'"),
            Rule::Root
            | Rule::BodyTypeScriptBlock
            | Rule::BodyTemplate
            | Rule::Comments
            | Rule::NestedSpecialTagsKeywords
            | Rule::NestedSpecialTagsKeywordsGroup2
            | Rule::SpecialTagsVoid
            | Rule::SpecialTagsModesAt1
            | Rule::SpecialTagsModesAt2
            | Rule::SpecialTagsModesAt2At1
            | Rule::SpecialTagsModesAt3
            | Rule::SpecialTagsModesAt3At1
            | Rule::NestedMarkdown
            | Rule::NestedTypeScript
            | Rule::SpecialTagsModesAt4
            | Rule::SpecialTagsModesAt5
            | Rule::SpecialTagsBlockBegin
            | Rule::SpecialTagsBlockEnd
            | Rule::NestedTagsStartNode
            | Rule::NestedTagsName
            | Rule::NestedTagsNameAt2Group1
            | Rule::TagsLang
            | Rule::NestedTagsEndNode
            | Rule::TagsLangAt0
            | Rule::TagsLangStartAttributes
            | Rule::AttributesCommentsAt1
            | Rule::AttributesDirectives
            | Rule::NestedAttributesDirectivesKeywords
            | Rule::NestedAttributesDirectivesTypesAssigned
            | Rule::NestedAttributesDirectivesTypes
            | Rule::NestedAttributesDirectivesGroup5
            | Rule::AttributesDirectivesAt0
            | Rule::AttributesValueAt3
            | Rule::AttributesKeyvalue
            | Rule::NestedAttributesKeyvalueGroup0
            | Rule::AttributesKeyvalueAt0
            | Rule::TagsVoid
            | Rule::TagsGeneralEnd
            | Rule::TagsGeneralStart
            | Rule::TagsStartAttributes
            | Rule::ScopeAt4 => false,
        }
    }
}

/// `</script` or `</style`, then `>` or a blank.
/// The line's end stands for its line feed.
pub(super) fn closing_tag(rest: &[u8]) -> bool {
    let name = if rest.starts_with(b"</script") {
        8
    } else if rest.starts_with(b"</style") {
        7
    } else {
        return false;
    };
    rest.get(name)
        .is_none_or(|&b| b == b'>' || b.is_ascii_whitespace())
}

/// `\s*(?=\{)`: blanks, then a `{`.
fn object_ahead(rest: &[u8]) -> bool {
    let blanks = rest.iter().take_while(|b| b.is_ascii_whitespace()).count();
    rest.get(blanks) == Some(&b'{')
}

/// The end of an `{#each}` expression: `(?=(?:^\s*|\s+)(as)|\s*([,}]))`.
/// `as` needs no word boundary after it, as in the grammar.
fn each_expression_end(s: &[u8], p: usize) -> bool {
    let rest = s.get(p..).unwrap_or_default();
    let blanks = rest.iter().take_while(|b| b.is_ascii_whitespace()).count();
    let next = rest.get(blanks..).unwrap_or_default();
    ((blanks > 0 || p == 0) && next.starts_with(b"as"))
        || next.first().is_some_and(|&b| b == b',' || b == b'}')
}

/// The end of an `{#await}` expression: `\s+(then)|(?=})`.
fn await_expression_end(rest: &[u8]) -> bool {
    let blanks = rest.iter().take_while(|b| b.is_ascii_whitespace()).count();
    (blanks > 0
        && rest
            .get(blanks..)
            .is_some_and(|next| next.starts_with(b"then")))
        || rest.first() == Some(&b'}')
}
