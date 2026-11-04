use indoc::indoc;

use crate::{Highlighter, Language, State};

/// One lexed line: each token's text and its scope names, outermost first.
type Row = Vec<(String, Vec<String>)>;

fn lex(text: &str) -> Vec<Row> {
    let mut highlighter = Highlighter::new(Language::Dart);
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
fn the_name_is_known() {
    assert_eq!(Language::from_name("dart"), Some(Language::Dart));
}

#[test]
fn an_import_names_its_keywords_and_ends_at_the_semicolon() {
    let text = r#"import "a.dart" as b show C;"#;
    assert_eq!(leaf(text, "import"), "keyword.other.import.dart");
    assert_eq!(leaf(text, "as"), "keyword.other.import.dart");
    assert_eq!(leaf(text, "show"), "keyword.other.import.dart");
    assert_eq!(leaf(text, " C"), "meta.declaration.dart");
    assert_eq!(leaf(text, ";"), "punctuation.terminator.dart");
}

#[test]
fn a_generic_call_lexes_its_type_arguments() {
    let text = "  final x = foo<List<int>>(1);";
    let row = &lex(text)[0];
    let leaves: Vec<(&str, &str)> = row
        .iter()
        .map(|(t, s)| (t.as_str(), s.last().map_or("", String::as_str)))
        .collect();
    assert_eq!(
        leaves[5..12],
        [
            ("foo", "entity.name.function.dart"),
            ("<", "other.source.dart"),
            ("List", "support.class.dart"),
            ("<", "other.source.dart"),
            ("int", "support.class.dart"),
            (">", "other.source.dart"),
            (">", "other.source.dart"),
        ]
    );
}

#[test]
fn a_star_after_async_is_an_operator_before_a_blank() {
    let text = "main() async* {";
    assert_eq!(leaf(text, "async"), "keyword.control.dart");
    assert_eq!(leaf(text, "*"), "keyword.operator.arithmetic.dart");
}

#[test]
fn is_takes_its_bang_only_before_a_word() {
    let text = "is! x; a is!b";
    assert_eq!(leaf(text, "is"), "keyword.operator.dart");
    assert_eq!(leaf(text, "!"), "keyword.operator.logical.dart");
    assert_eq!(leaf(text, "is!"), "keyword.operator.dart");
}

#[test]
fn a_number_before_a_letter_ends_at_its_dot() {
    let text = "f(1.5e3, 0x1F, 1.x);";
    assert_eq!(leaf(text, "1.5e3"), "constant.numeric.dart");
    assert_eq!(leaf(text, "0x1F"), "constant.numeric.dart");
    assert_eq!(leaf(text, "1."), "constant.numeric.dart");
}

#[test]
fn an_interpolation_holds_names() {
    let text = r#"var s = "a $b ${c.d} \n";"#;
    assert_eq!(leaf(text, "b"), "variable.parameter.dart");
    assert_eq!(leaf(text, "${"), "meta.embedded.expression.dart");
    assert_eq!(leaf(text, "d"), "variable.parameter.dart");
    assert_eq!(leaf(text, "\\n"), "constant.character.escape.dart");
}

#[test]
fn extension_type_is_one_keyword_and_new_after_a_dot_a_function() {
    let text = "T.new; extension type E(int i) {}";
    assert_eq!(leaf(text, "new"), "entity.name.function.dart");
    assert_eq!(leaf(text, "extension type"), "keyword.declaration.dart");
    assert_eq!(leaf(text, "E"), "support.class.dart");
}

#[test]
fn a_documentation_comment_runs_over_its_lines() {
    let text = indoc! {"
        /// See [Foo] and `bar`.
        ///     code
        int get value => 1;
    "};
    let rows = lex(text);
    assert_eq!(leaf(text, "[Foo]"), "variable.name.source.dart");
    assert_eq!(leaf(text, "`bar`"), "variable.other.source.dart");
    assert_eq!(leaf(text, "code"), "variable.other.source.dart");
    assert_eq!(
        rows[1][1],
        (
            "//     ".to_owned(),
            vec![
                "source.dart".to_owned(),
                "comment.block.documentation.dart".to_owned()
            ]
        )
    );
    assert_eq!(leaf(text, "get"), "keyword.declaration.dart");
    assert_eq!(leaf(text, "=>"), "keyword.operator.closure.dart");
}

#[test]
fn an_empty_line_keeps_the_state() {
    let rows = lex(indoc! {"
        /* a

        b */ c"});
    assert_eq!(
        rows[2][0].1.last().map(String::as_str),
        Some("comment.block.dart")
    );
}
