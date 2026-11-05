use indoc::indoc;

use crate::{Highlighter, Language, State};

/// One lexed line: each token's text and its scope names, outermost first.
type Row = Vec<(String, Vec<String>)>;

fn lex(text: &str) -> Vec<Row> {
    let mut highlighter = Highlighter::new(Language::Vb);
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

/// The scopes of the first token whose text is `token`, innermost last.
fn stack(text: &str, token: &str) -> Vec<String> {
    lex(text)
        .into_iter()
        .flatten()
        .find(|(t, _)| t == token)
        .unwrap_or_else(|| panic!("no token {token:?} in {text:?}"))
        .1
}

#[test]
fn the_name_is_known() {
    assert_eq!(Language::from_name("vb"), Some(Language::Vb));
}

/// Under `(?i)`, the name class `[7A-Z_a-z]` also takes the Kelvin sign,
/// whose fold is `k`.
/// A matcher that reads only ASCII letters loses the name without a sound.
#[test]
fn a_name_may_start_with_the_kelvin_sign() {
    let text = "Dim \u{212a}ey As Integer";
    assert_eq!(leaf(text, "Dim"), "storage.type.asp");
    assert_eq!(leaf(text, "\u{212a}ey"), "variable.other.bfeac.asp");
    assert_eq!(leaf(text, "Integer"), "support.type.vb.asp");
}

/// `ſ` folds to `s`, so `aſ` is the operator `As`, and `ſize` is a name.
#[test]
fn the_long_s_folds_to_s() {
    assert_eq!(
        leaf("If a a\u{17f} b Then", "a\u{17f}"),
        "keyword.operator.asp"
    );
    assert_eq!(leaf("If a a\u{17f} b Then", "b"), "support.type.vb.asp");
    assert_eq!(
        leaf("Dim x As \u{17f}ize", "\u{17f}ize"),
        "support.type.vb.asp"
    );
}

/// `ß` folds to `ss` in a literal, so `Claß` is `Class`.
#[test]
fn a_literal_folds_sharp_s() {
    assert_eq!(leaf("Cla\u{df} Foo", "Cla\u{df} "), "storage.type.asp");
}

/// The alternation tries `Else` before `Else If`, and a blank follows
/// `Else`, so `Else If` is two keywords.
#[test]
fn else_if_is_two_keywords() {
    let rows = lex("Else If x Then");
    let keywords: Vec<&str> = rows[0]
        .iter()
        .filter(|(_, scopes)| scopes.last().is_some_and(|s| s == "keyword.control.asp"))
        .map(|(text, _)| text.as_str())
        .collect();
    assert_eq!(keywords, ["Else", "If", "Then"]);
}

/// After `1.5` the `é` breaks the final `\b`, so the number gives back to
/// `1.`.
#[test]
fn a_number_gives_back_to_its_point() {
    assert_eq!(leaf("x = 1.5\u{e9} + 0xFFL", "1."), "constant.numeric.asp");
    assert_eq!(
        leaf("x = 1.5\u{e9} + 0xFFL", "0xFFL"),
        "constant.numeric.asp"
    );
}

/// The string's end is tried before `""`, so `""` closes the string and
/// opens another.
#[test]
fn a_doubled_quote_ends_the_string() {
    let rows = lex(r#"Dim s = "a""b""#);
    let ends = rows[0]
        .iter()
        .filter(|(_, scopes)| {
            scopes
                .iter()
                .any(|s| s == "punctuation.definition.string.end.asp")
        })
        .count();
    assert_eq!(ends, 2);
}

#[test]
fn a_function_head_and_a_comment() {
    let text = indoc! {"
        Sub Main(args)
          ' note
        End Sub"};
    assert_eq!(leaf(text, "Main"), "entity.name.function.asp");
    assert_eq!(leaf(text, "args"), "variable.parameter.function.asp");
    assert_eq!(
        stack(text, "  ").last().map(String::as_str),
        Some("meta.odd-tab.spaces")
    );
    assert_eq!(leaf(text, "'"), "punctuation.definition.comment.asp");
}
