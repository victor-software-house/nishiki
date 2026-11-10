//! Astro on the shared rule engine ([`crate::rules`]): the grammar's types,
//! and where each embedding rule takes its line back.

use super::grammar::{Pattern, Rule, Sc};
use super::matchers::{Body, body_of};
use super::scope::S;
use crate::Language;
use crate::arena::Arena;
use crate::language::{Context, Halt, Line};
use crate::limits::Finished;
use crate::rules::{
    self, Act, Captures, Deferrals, Embedding, Embeds, FnHalt, Grammar, Payload, ScopeName,
    table_scan,
};
use crate::scope::Scopes;

/// The Astro lexer for one line.
pub(crate) type Lexer<'a> = rules::Lexer<'a, Astro>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<Astro>;

/// The `astro` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Astro;

impl Grammar for Astro {
    type Scope = Sc;
    const NO_SCOPES: Scopes<Sc> = Sc::NO_SCOPES;
    type Rule = Rule;
    type Pattern = Pattern;
    type Nested = Rule;
    /// The capture groups the line defers ([`Grammar::deferred`]).
    type Extra = Option<Box<Deferrals>>;
    /// The same groups between the run and the engine's splice, and their
    /// buffers from line to line.
    type Kept = Option<Box<Deferrals>>;

    const NAME: &'static str = "Astro";
    const ROOT: Rule = Rule::Root;
    const SOURCE: crate::scope::Scope = S::Source.scope();
    const GROUPS: usize = 5;
    /// A `//` comment in a quoted event handler value ends at the line
    /// feed, `(?=")|\n`, and takes it.
    const CAPTURED_EOL: bool = true;
    /// An expression's object literal hands TSX back to the expression
    /// mid-line, which then embeds TSX again.
    const EMBEDS: bool = true;
    /// The frontmatter opens only at `\A`, on the document's first line.
    const MARKS_FIRST_LINE: bool = true;

    fn scope(scope: Sc) -> ScopeName {
        match scope {
            Sc::S(scope) => ScopeName::Fixed(scope.scope()),
            Sc::Group(prefix, group, suffix) => ScopeName::Capture {
                prefix,
                group,
                suffix,
            },
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

    fn rule_payload(rule: Rule) -> Payload {
        rule.payload()
    }

    fn rule_name_delimiter(rule: Rule) -> Option<&'static str> {
        rule.name_delimiter()
    }

    fn rule_name_suffix(rule: Rule) -> &'static str {
        rule.name_suffix()
    }

    fn rule_name_after(rule: Rule) -> Scopes<Sc> {
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

    /// JavaScript lexes the event handler values again
    /// ([`Embeds::lex_nested`]).
    const DEFERS_NESTED: bool = true;

    fn defers_nested(nested: Rule) -> Option<Language> {
        (nested == Rule::NestedJavaScript).then_some(Language::JavaScript)
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

impl Embeds for Astro {
    type End = End;

    /// Every embedding rule picks its halts by the rule
    /// ([`Embeds::embed_halts_for`]); this one never holds.
    fn embed_halts(embedding: &Embedding, _: &Arena) -> (End, Option<FnHalt>) {
        (
            End {
                rule: Rule::Root,
                g: embedding.anchored.then_some(embedding.at),
                body: false,
            },
            None,
        )
    }

    /// An event handler value goes to JavaScript, from a fresh state on the
    /// group's stack, as `TextMate` lexes a capture group again.
    fn lex_nested(
        _language: Language,
        cx: &mut Context<'_>,
        line: Line<'_>,
        base: u32,
    ) -> Option<Finished> {
        let state = crate::typescript::start_embedded(cx.arena, base);
        Some(crate::typescript::lex_capture(cx, line, state))
    }

    /// The end of the embedding rule, and the patterns it lists before the
    /// embedded grammar. No embedding rule lists patterns after it.
    /// A backtick attribute value lists the bodies first: in a `<script>` or
    /// `<style>` start tag, a body opens after a `>` in it.
    fn embed_halts_for(rule: Rule, embedding: &Embedding, arena: &Arena) -> (End, Option<FnHalt>) {
        (
            End {
                rule,
                g: embedding.anchored.then_some(embedding.at),
                body: rule == Rule::AttributeLiteral && opens_body(arena, embedding.base),
            },
            None,
        )
    }
}

/// Where an embedding rule takes its line back from the language it embeds:
/// its end pattern, or a pattern it lists before the embedded grammar.
/// `g` is where `\G` holds: where the embedded text starts, when a match
/// ended there on this line.
///
/// At the start of a line that goes on inside the rule, `\G` holds nowhere.
/// `body` says whether an injection's selector holds on the rule's text, so
/// that a body opens after a `>`.
#[derive(Clone, Copy, Debug)]
pub(crate) struct End {
    rule: Rule,
    g: Option<usize>,
    body: bool,
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
            | Rule::BodyJson
            | Rule::BodyTypeScript
            | Rule::BodyCss
            | Rule::BodyLess
            | Rule::BodyPostCss
            | Rule::BodySass
            | Rule::BodyScss => after(b"</"),
            Rule::Frontmatter => {
                ((p == 0 || self.g == Some(p)) && after(b"---"))
                    || (after(b"...") && blank_to_end(rest.get(3..).unwrap_or_default()))
            }
            Rule::InterpolationAt0 => after(b"}") || (self.g == Some(p) && object_ahead(rest)),
            Rule::InterpolationAt0At0 => p > 0 && s.get(p - 1) == Some(&b'}'),
            Rule::AttributesInterpolated => after(b"}"),
            Rule::AttributeLiteral => {
                after(b"`") || (self.body && p > 0 && s.get(p - 1) == Some(&b'>') && !after(b"</"))
            }
            Rule::Comments
            | Rule::NestedTagsStartNode
            | Rule::NestedScriptStart
            | Rule::NestedStyleStart
            | Rule::NestedTagsName
            | Rule::NestedTagsEndNode
            | Rule::TagsRaw
            | Rule::TagsLangStartAttributes
            | Rule::NestedAttributeName
            | Rule::AttributesEvents
            | Rule::EventEquals
            | Rule::EventScript
            | Rule::NestedJavaScript
            | Rule::EventScriptDouble
            | Rule::EventScriptDoubleLineComment
            | Rule::EventScriptDoubleBlockComment
            | Rule::EventScriptSingle
            | Rule::EventScriptSingleLineComment
            | Rule::EventScriptSingleBlockComment
            | Rule::AttributesKeyvalue
            | Rule::KeyvalueEquals
            | Rule::ValueQuoted
            | Rule::TagsLangScript
            | Rule::LangJson
            | Rule::LangModule
            | Rule::LangNamed
            | Rule::TagsLangStyle
            | Rule::TagsVoid
            | Rule::TagsGeneralEnd
            | Rule::TagsGeneralStart
            | Rule::TagsStartAttributes
            | Rule::Text
            | Rule::Root => false,
        }
    }
}

/// Whether a body's injection holds on text whose scopes are the stack
/// `base`.
/// A `meta.script.astro` or `meta.style.astro` scope names a body, with the
/// `meta.lang.<lang>.astro` inside it when there is one.
/// No `source` scope inside the tag's may hide it (`- (meta source)`).
fn opens_body(arena: &Arena, base: u32) -> bool {
    let mut lang = None;
    for scope in arena.scopes(base) {
        if scope == "meta.script.astro" || scope == "meta.style.astro" {
            return body_of(scope == "meta.script.astro", lang) != Body::Plain;
        }
        if scope.starts_with("source.") {
            return false;
        }
        if lang.is_none() {
            lang = scope
                .strip_prefix("meta.lang.")
                .and_then(|rest| rest.strip_suffix(".astro"));
        }
    }
    false
}

/// `\s*$`: blanks to the line's end.
fn blank_to_end(rest: &[u8]) -> bool {
    std::str::from_utf8(rest).is_ok_and(|text| text.chars().all(crate::classes::is_space))
}

/// `\s*(?=\{)`: blanks, then a `{`.
fn object_ahead(rest: &[u8]) -> bool {
    let blanks = rest.iter().take_while(|b| b.is_ascii_whitespace()).count();
    rest.get(blanks) == Some(&b'{')
}
