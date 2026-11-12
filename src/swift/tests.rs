//! Tests of the Swift lexer.

use indoc::indoc;

use crate::arena::Arena;
use crate::rules::Grammar;
use crate::swift::engine::Swift;
use crate::swift::grammar::Pattern;
use crate::test_support::leaves::{leaf_in, merge_by_leaf};
use crate::test_support::probe::{After, Anchor, Case, lexer};
use crate::test_support::rows::Row;
use crate::{Language, test_support};

fn lex(text: &str) -> Vec<Row> {
    test_support::rows::lex(Language::Swift, text)
}

/// The innermost scope of the first token whose text is `token`.
fn leaf(text: &str, token: &str) -> String {
    lex(text)
        .into_iter()
        .flatten()
        .find(|(t, _)| t == token)
        .unwrap_or_else(|| panic!("no token {token:?} in {text:?}"))
        .1
        .pop()
        .unwrap_or_default()
}

/// The innermost scope of the first token of line `line`.
fn first_leaf(rows: &[Row], line: usize) -> &str {
    rows[line][0].1.last().map_or("", String::as_str)
}

#[test]
fn the_name_swift_is_known() {
    assert_eq!(Language::from_name("swift"), Some(Language::Swift));
    assert_eq!(Language::from_name("SWIFT"), Some(Language::Swift));
}

#[test]
fn a_function_declaration_names_its_parts() {
    let text = "public func f(orFail message: String, line: UInt = #line) -> Int {}";
    assert_eq!(
        leaf(text, "public"),
        "keyword.other.declaration-specifier.accessibility.swift"
    );
    assert_eq!(leaf(text, "func"), "storage.type.function.swift");
    assert_eq!(leaf(text, "f"), "entity.name.function.swift");
    assert_eq!(leaf(text, "orFail"), "entity.name.function.swift");
    assert_eq!(leaf(text, "message"), "variable.parameter.function.swift");
    assert_eq!(leaf(text, "String"), "support.type.swift");
    assert_eq!(leaf(text, "#line"), "support.variable.swift");
    assert_eq!(leaf(text, "->"), "keyword.operator.function-result.swift");
}

#[test]
fn an_enum_closes_at_a_brace_that_starts_a_line() {
    let rows = lex(indoc! {"
        enum Mode: String {
            case debug, release
        }
        struct Example {
        }"});
    assert_eq!(
        first_leaf(&rows, 2),
        "punctuation.definition.type.end.swift"
    );
    assert_eq!(first_leaf(&rows, 3), "storage.type.struct.swift");
    assert!(rows[1].iter().any(|(t, scopes)| {
        t == "release" && scopes.iter().any(|s| s == "meta.enum-case.more-cases")
    }));
}

#[test]
fn a_block_string_continues_across_lines_with_interpolations() {
    let rows = lex(indoc! {r#"
        return """
            a \(b) c
            """"#});
    assert_eq!(first_leaf(&rows, 1), "string.quoted.double.block.swift");
    let scope = |text: &str| {
        rows[1]
            .iter()
            .find(|(t, _)| t == text)
            .and_then(|(_, scopes)| scopes.last().cloned())
            .unwrap_or_default()
    };
    assert_eq!(scope("\\("), "punctuation.section.embedded.begin.swift");
    assert_eq!(scope(")"), "punctuation.section.embedded.end.swift");
    assert_eq!(
        rows[2]
            .last()
            .and_then(|(_, scopes)| scopes.last())
            .map(String::as_str),
        Some("punctuation.definition.string.end.swift")
    );
}

#[test]
fn a_raw_string_keeps_its_backslashes() {
    let text = r##"let s = #"\(x)\."#"##;
    assert_eq!(
        leaf(text, "#\""),
        "punctuation.definition.string.begin.raw.swift"
    );
    assert_eq!(
        leaf(text, r"\(x)\."),
        "string.quoted.double.single-line.raw.swift"
    );
    assert_eq!(
        leaf(text, "\"#"),
        "punctuation.definition.string.end.raw.swift"
    );
}

#[test]
fn an_extended_regex_literal_is_lexed_as_a_regex() {
    let text = r"let r = #/(\D+)/#";
    assert_eq!(leaf(text, "#/"), "string.regexp.line.extended.swift");
    assert_eq!(leaf(text, "("), "punctuation.definition.group.regexp");
    assert_eq!(
        leaf(text, r"\D"),
        "constant.character.character-class.regexp"
    );
    assert_eq!(leaf(text, "+"), "keyword.operator.quantifier.regexp");
}

#[test]
fn attributes_and_a_conditional_compilation_block() {
    let text = indoc! {"
        #if canImport(Security)
        @preconcurrency @MainActor
        #endif"};
    assert_eq!(leaf(text, "#"), "punctuation.definition.preprocessor.swift");
    assert_eq!(
        leaf(text, "if"),
        "keyword.control.import.preprocessor.conditional.swift"
    );
    assert_eq!(leaf(text, "canImport"), "keyword.other.condition.swift");
    assert_eq!(leaf(text, "Security"), "entity.name.type.module.swift");
    assert_eq!(leaf(text, "@"), "punctuation.definition.attribute.swift");
    assert_eq!(leaf(text, "MainActor"), "storage.modifier.attribute.swift");
}

#[test]
fn numbers_in_decimal_float_and_hex() {
    for (number, scope) in [
        ("1_000", "constant.numeric.integer.decimal.swift"),
        ("1.0", "constant.numeric.float.decimal.swift"),
        ("0xdeadbeef", "constant.numeric.integer.hexadecimal.swift"),
    ] {
        assert_eq!(leaf(&format!("x = {number}"), number), scope, "{number}");
    }
}

#[test]
fn block_comments_nest() {
    let rows = lex(indoc! {"
        /* a /* b */
        still */ let"});
    assert_eq!(first_leaf(&rows, 1), "comment.block.swift");
    assert_eq!(
        rows[1]
            .last()
            .and_then(|(_, scopes)| scopes.last())
            .map(String::as_str),
        Some("keyword.other.declaration-specifier.swift")
    );
}

#[test]
fn a_shebang_is_a_comment_on_the_first_line_only() {
    let rows = lex(indoc! {"
        #!/usr/bin/env swift
        #!/usr/bin/env swift"});
    assert_eq!(first_leaf(&rows, 0), "punctuation.definition.comment.swift");
    assert!(
        rows[0]
            .iter()
            .all(|(_, scopes)| { scopes.iter().any(|s| s == "comment.line.number-sign.swift") })
    );
    assert!(
        rows[1]
            .iter()
            .all(|(_, scopes)| { scopes.iter().all(|s| s != "comment.line.number-sign.swift") })
    );
}

#[test]
fn an_empty_line_keeps_the_state() {
    let rows = lex(indoc! {"
        /* a

        b */"});
    assert_eq!(rows[1].len(), 0);
    assert_eq!(first_leaf(&rows, 2), "comment.block.swift");
}

/// A capture lexed again has no line feed after it.
/// `\z` therefore holds at its end.
/// An operator there is postfix, and a prefix operator is not.
#[test]
fn an_operator_that_ends_a_capture_lexed_again_is_postfix() {
    let rows = merge_by_leaf(lex(indoc! {r#"
        #sourceLocation(file: "\(a+)
        #sourceLocation(file: "\(a.+)
        #sourceLocation(file: "\(a, -)
    "#}));
    assert_eq!(
        leaf_in(&rows[0], "+"),
        "keyword.operator.custom.postfix.swift"
    );
    assert_eq!(
        leaf_in(&rows[1], ".+"),
        "keyword.operator.custom.postfix.dot.swift"
    );
    assert_eq!(
        leaf_in(&rows[2], "-"),
        "keyword.operator.custom.infix.swift"
    );
}

/// The span of each capture group of a match, `None` for a group that did
/// not take part.
type Groups = Vec<Option<(usize, usize)>>;

/// What `pattern` answers on `case`: where its match ends and the span of each
/// capture group, or `None` where it does not match.
fn answer(arena: &mut Arena, pattern: Pattern, case: &Case<'_>) -> Option<(usize, Groups)> {
    lexer::<Swift, _>(arena, case, |lexer| {
        let end = lexer.probe(pattern, case.p)?;
        let captures = (0..Swift::GROUPS).map(|group| lexer.capture_span(group));
        Some((end, captures.collect()))
    })
}

/// Where the text ends, the matchers answer as Oniguruma does, which
/// `oracle/onig.ts` gave for each case.
/// A capture lexed again has no line feed after it, and a line's scan goes on
/// past its line feed.
#[test]
fn the_matchers_answer_as_oniguruma_where_the_text_ends() {
    use After::{LineFeed, Nothing};
    use Anchor::{Free, G};
    use Pattern::{
        DeclarationsExtensionExtensionAt0 as Name, DeclarationsExtensionExtensionAt0End as NameEnd,
        DeclarationsGenericWhereClauseRequirementListAt3End as SameTypeEnd,
        DeclarationsImportEnd as ImportEnd, LiteralsRegularExpressionLiteralQuoteEnd as QuoteEnd,
        OperatorsAt1At0 as Prefix, OperatorsAt1At1 as Postfix, OperatorsAt1At4 as DotPostfix,
    };
    let mut arena = Arena::new(None);
    let cases = [
        (ImportEnd, "import Foo", 10, Free, Nothing, Some(10)),
        (ImportEnd, "import Foo", 10, Free, LineFeed, Some(11)),
        (QuoteEnd, r"/\Qab/", 6, Free, Nothing, None),
        (QuoteEnd, r"/\Qab/", 6, Free, LineFeed, Some(7)),
        (Prefix, "a, -", 3, G, Nothing, None),
        (Postfix, "a+", 1, G, Nothing, Some(2)),
        (Postfix, "a+", 1, G, LineFeed, Some(2)),
        (DotPostfix, "a.+", 1, G, Nothing, Some(3)),
        (Name, "extension ", 10, G, Nothing, Some(10)),
        (Name, "extension", 10, G, LineFeed, Some(10)),
        (Name, "extension ", 10, G, LineFeed, None),
        (NameEnd, "extension Foo ", 13, Free, Nothing, None),
        (NameEnd, "extension Foo", 14, Free, LineFeed, None),
        (NameEnd, "extension Foo", 13, Free, LineFeed, Some(13)),
        (SameTypeEnd, "T == U", 6, Free, Nothing, None),
        (SameTypeEnd, "T == U", 7, Free, LineFeed, None),
        (SameTypeEnd, "T == U", 6, Free, LineFeed, Some(6)),
        (SameTypeEnd, "T == U {", 6, Free, Nothing, Some(6)),
    ];
    for (pattern, text, p, anchor, after, want) in cases {
        let case = Case {
            text,
            p,
            anchor,
            after,
        };
        let got = answer(&mut arena, pattern, &case).map(|(end, _)| end);
        assert_eq!(
            got, want,
            "{pattern:?} on {text:?} at {p}, {anchor:?}, {after:?}"
        );
    }
}

#[cfg(feature = "json")]
mod probe {
    use super::answer;
    use crate::swift::grammar::Pattern;
    use crate::test_support::probe::answer_cases;

    /// Answers the cases of `oracle/onig.ts` with Swift's matchers (see
    /// [`answer_cases`]).
    #[test]
    #[ignore = "a tool: it needs a cases file"]
    fn matchers_answer_the_probe_cases() {
        answer_cases(&Pattern::ALL, answer);
    }
}
