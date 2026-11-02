use indoc::indoc;

use crate::{Highlighter, Language, State};

/// One lexed line: each token's text and its scope names, outermost first.
type Row = Vec<(String, Vec<String>)>;

fn lex_as(language: Language, text: &str) -> Vec<Row> {
    let mut highlighter = Highlighter::new(language);
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

fn lex(text: &str) -> Vec<Row> {
    lex_as(Language::Json, text)
}

/// The scopes of the first token whose text is `token`, outermost first.
fn scopes_of(text: &str, token: &str) -> Vec<String> {
    lex(text)
        .into_iter()
        .flatten()
        .find(|(t, _)| t == token)
        .unwrap_or_else(|| panic!("no token {token:?} in {text:?}"))
        .1
}

#[test]
fn a_property_and_its_value() {
    assert_eq!(
        scopes_of(r#"{"a": 1}"#, "1"),
        [
            "source.json",
            "meta.structure.dictionary.json",
            "meta.structure.dictionary.value.json",
            "constant.numeric.json"
        ]
    );
    assert_eq!(
        scopes_of(r#"{"a": 1}"#, "a"),
        [
            "source.json",
            "meta.structure.dictionary.json",
            "string.json",
            "support.type.property-name.json"
        ]
    );
}

#[test]
fn a_string_continues_across_lines() {
    let rows = lex("[\"a\nb\"]");
    assert_eq!(
        rows[1][0].1.last().map(String::as_str),
        Some("string.quoted.double.json")
    );
}

#[test]
fn the_dialects_spell_their_scopes_alike_but_for_the_suffix() {
    for (language, suffix) in [
        (Language::Jsonc, ".json.comments"),
        (Language::Jsonl, ".json.lines"),
    ] {
        let rows = lex_as(language, "[null]");
        assert_eq!(
            rows[0][1].1,
            [
                format!("source{suffix}"),
                format!("meta.structure.array{suffix}"),
                format!("constant.language{suffix}"),
            ]
        );
    }
}

#[test]
fn escapes_and_invalid_escapes() {
    let rows = lex(r#"["\n\u00e9\q\u12"]"#);
    let leaves: Vec<(&str, &str)> = rows[0]
        .iter()
        .filter(|(_, scopes)| scopes.len() > 3)
        .map(|(t, scopes)| (t.as_str(), scopes[scopes.len() - 1].as_str()))
        .collect();
    assert!(leaves.contains(&("\\n", "constant.character.escape.json")));
    assert!(leaves.contains(&("\\u00e9", "constant.character.escape.json")));
    assert!(leaves.contains(&("\\q", "invalid.illegal.unrecognized-string-escape.json")));
    assert!(leaves.contains(&("\\u", "invalid.illegal.unrecognized-string-escape.json")));
}

#[test]
fn a_leading_zero_is_two_numbers() {
    let rows = lex("[01]");
    let numbers: Vec<&str> = rows[0]
        .iter()
        .filter(|(_, scopes)| scopes.last().is_some_and(|s| s == "constant.numeric.json"))
        .map(|(t, _)| t.as_str())
        .collect();
    assert_eq!(numbers, ["0", "1"]);
}

#[test]
fn a_constant_needs_word_boundaries() {
    assert_eq!(
        scopes_of("[true]", "true").last().map(String::as_str),
        Some("constant.language.json")
    );
    let rows = lex("[xtrue, trues]");
    assert!(
        rows[0]
            .iter()
            .all(|(_, scopes)| scopes.last().is_none_or(|s| s != "constant.language.json"))
    );
}

#[test]
fn a_stray_character_is_invalid_where_the_grammar_says() {
    assert_eq!(
        scopes_of("[a]", "a").last().map(String::as_str),
        Some("invalid.illegal.expected-array-separator.json")
    );
    assert_eq!(
        scopes_of(r#"{"a" 1}"#, "1").last().map(String::as_str),
        Some("invalid.illegal.expected-dictionary-separator.json")
    );
}

#[test]
fn comments_are_scoped_in_every_dialect() {
    let text = indoc! {"
        // line
        /* block
        more */ /** doc */ 1
    "};
    let rows = lex(text);
    assert_eq!(
        rows[0][0],
        (
            "//".to_owned(),
            [
                "source.json",
                "comment.line.double-slash.js",
                "punctuation.definition.comment.json"
            ]
            .map(str::to_owned)
            .to_vec()
        )
    );
    assert_eq!(rows[1][1].1, ["source.json", "comment.block.json"]);
    assert_eq!(rows[2][0].1, ["source.json", "comment.block.json"]);
}

#[test]
fn a_value_ends_at_the_brace_that_closes_its_object() {
    let rows = lex(r#"{"a": 1 }"#);
    let closing = rows[0].last().map(|(t, s)| (t.as_str(), s.clone()));
    assert_eq!(
        closing,
        Some((
            "}",
            [
                "source.json",
                "meta.structure.dictionary.json",
                "punctuation.definition.dictionary.end.json"
            ]
            .map(str::to_owned)
            .to_vec()
        ))
    );
}
