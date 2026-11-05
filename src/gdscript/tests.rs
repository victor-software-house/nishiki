use indoc::indoc;

use crate::{Highlighter, Language, State};

/// One lexed line: each token's text and its scope names, outermost first.
type Row = Vec<(String, Vec<String>)>;

fn lex(text: &str) -> Vec<Row> {
    let mut highlighter = Highlighter::new(Language::GdScript);
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

#[test]
fn the_names_are_known() {
    assert_eq!(Language::from_name("gdscript"), Some(Language::GdScript));
    assert_eq!(Language::from_name("gd"), Some(Language::GdScript));
}

#[test]
fn a_class_with_a_function() {
    let text = indoc! {"
        extends Node2D
        class_name Player

        func _ready() -> void:
        \tvar path = $Sprite/Body"};
    assert_eq!(leaf(text, "extends"), "keyword.language.gdscript");
    assert_eq!(leaf(text, "Player"), "entity.name.type.class.gdscript");
    assert_eq!(leaf(text, "_ready"), "entity.name.function.gdscript");
    assert_eq!(
        leaf(text, "void"),
        "entity.name.type.class.builtin.gdscript"
    );
    assert_eq!(leaf(text, "$"), "keyword.control.flow.gdscript");
    assert_eq!(leaf(text, "Body"), "constant.character.escape.gdscript");
}

/// The grammar's `(?<!/\s*)` keeps a bare node path from starting after a
/// `/`, but Shiki still reads `$Node` after `a / ` as a node path of
/// several names.
#[test]
fn a_node_path_after_a_slash() {
    let rows = lex("x = a / $Node");
    let node = rows[0]
        .iter()
        .find(|(t, _)| t == "Node")
        .map(|(_, s)| s.clone());
    assert_eq!(
        node,
        Some(vec![
            "source.gdscript".to_owned(),
            "meta.literal.nodepath.bare.gdscript".to_owned(),
            "constant.character.escape.gdscript".to_owned(),
        ])
    );
}

/// A line continuation goes on while the next line opens a string, and
/// ends before a line of plain code.
#[test]
fn a_line_continuation_reaches_a_string_only() {
    let text = indoc! {r#"
    var s = "x" \
        	"y {name!r:>10}"
        y = \
    z = 1"#};
    let rows = lex(text);
    assert_eq!(
        rows[0]
            .last()
            .and_then(|(_, s)| s.last())
            .map(String::as_str),
        Some("punctuation.separator.continuation.line.gdscript")
    );
    assert!(
        rows[1][0]
            .1
            .iter()
            .any(|s| s == "meta.variable.declaration.gdscript")
    );
    assert_eq!(leaf(text, "!r"), "storage.type.format.gdscript");
    assert_eq!(leaf(text, ":>10"), "storage.type.format.gdscript");
    assert_eq!(rows[3][0].1, ["source.gdscript", "variable.other.gdscript"]);
}

#[test]
fn is_not_takes_two_blanks() {
    let text = "if x is not  Node: pass";
    assert_eq!(leaf(text, "is"), "storage.type.is.gdscript");
    assert_eq!(leaf(text, "not"), "storage.type.not.gdscript");
    assert_eq!(leaf(text, "Node"), "entity.name.type.class.gdscript");
}

/// A call comes before `\b(\w+).(new)\(` in the grammar, so `Foonew(` is
/// one function name.
#[test]
fn foonew_is_a_call() {
    assert_eq!(
        leaf("var n = Foonew(1)", "Foonew"),
        "entity.name.function.gdscript"
    );
}
