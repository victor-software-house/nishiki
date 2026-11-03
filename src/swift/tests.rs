//! Tests of the Swift lexer.

use indoc::indoc;

use crate::{Highlighter, Language, State};

/// One lexed line: each token's text and its scope names, outermost first.
type Row = Vec<(String, Vec<String>)>;

fn lex(text: &str) -> Vec<Row> {
    let mut highlighter = Highlighter::new(Language::Swift);
    let mut state = State::default();
    let mut rows = Vec::new();
    for line in text.split('\n') {
        let mut tokens = Vec::new();
        let report = highlighter.line(line, &mut state, &mut tokens);
        assert!(report.is_complete(), "{line:?} reached a limit");
        let mut start = 0;
        let mut row = Vec::new();
        for token in &tokens {
            let mut scopes: Vec<String> = highlighter
                .scopes(token.stack())
                .map(ToString::to_string)
                .collect();
            scopes.reverse();
            row.push((line[start..token.end()].to_owned(), scopes));
            start = token.end();
        }
        assert_eq!(start, line.len(), "the tokens cover the line {line:?}");
        rows.push(row);
    }
    rows
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

#[cfg(feature = "json")]
mod probe {
    use serde_json::{Value, json};

    use crate::arena::{Arena, ROOT};
    use crate::limits::{Limits, LineReport};
    use crate::rules::{Grammar, Text};
    use crate::swift::engine::{Lexer, Scratch, Swift};
    use crate::swift::grammar::Pat;

    /// Runs the matchers on the cases in the file that `PROBE_IN` names,
    /// one JSON object per line (`rx`, the name of a pattern, `text`, `p`, the
    /// byte it is tried at, `g`, whether `\G` holds there, and `nl`, whether
    /// the line feed follows the text), and writes their answers to the file
    /// `PROBE_OUT` names: `null` for no match, else the end and the capture
    /// groups. `oracle/onig.ts` compares them with Oniguruma's.
    #[test]
    #[ignore = "a tool: it needs a cases file"]
    fn matchers_answer_the_probe_cases() {
        let input = std::env::var("PROBE_IN").expect("PROBE_IN");
        let output = std::env::var("PROBE_OUT").expect("PROBE_OUT");
        let cases = std::fs::read_to_string(input).expect("cases");
        let mut out = String::new();
        let mut arena = Arena::new(None);
        let limits = Limits::default();
        for line in cases.lines() {
            let case: Value = serde_json::from_str(line).expect("a case");
            let name = case["rx"].as_str().expect("rx");
            let pat = Pat::ALL
                .iter()
                .copied()
                .find(|pat| format!("{pat:?}") == name)
                .expect("a known pattern");
            let text = case["text"].as_str().expect("text");
            let p = usize::try_from(case["p"].as_u64().expect("p")).expect("p");
            let mut tokens = Vec::new();
            let mut report = LineReport::default();
            let mut scratch = Scratch::default();
            let mut lexer = Lexer::new(
                &mut arena,
                &limits,
                Text {
                    bytes: text.as_bytes(),
                    start: 0,
                    anchored: false,
                },
                &mut tokens,
                ROOT,
                &mut report,
                &mut scratch,
            );
            lexer.nl = case["nl"].as_bool().unwrap_or(true);
            lexer.pos = p;
            lexer.g = case["g"].as_bool().unwrap_or(false);
            let end = lexer.probe(pat, p);
            let value = end.map_or(Value::Null, |end| {
                let caps: Vec<Option<[usize; 2]>> = (0..Swift::GROUPS)
                    .map(|group| lexer.cap_span(group).map(|(a, b)| [a, b]))
                    .collect();
                json!({ "end": end, "caps": caps })
            });
            out.push_str(&value.to_string());
            out.push('\n');
        }
        std::fs::write(output, out).expect("write");
    }
}
