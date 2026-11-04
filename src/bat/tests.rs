use indoc::indoc;

use crate::{Highlighter, Language, State};

/// One lexed line: each token's text and its scope names, outermost first.
type Row = Vec<(String, Vec<String>)>;

fn lex(text: &str) -> Vec<Row> {
    let mut highlighter = Highlighter::new(Language::Bat);
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
    for name in ["bat", "batch", "cmd"] {
        assert_eq!(Language::from_name(name), Some(Language::Bat));
    }
}

#[test]
fn commands_and_their_pairs_are_keywords() {
    assert_eq!(leaf("copy a b", "copy"), "keyword.command.batchfile");
    assert_eq!(
        leaf("net user bob", "net user"),
        "keyword.command.batchfile"
    );
    assert_eq!(leaf("net use x", "net use"), "keyword.command.batchfile");
    assert_eq!(leaf("@XCOPY a b", "XCOPY"), "keyword.command.batchfile");
    assert_ne!(leaf("copyx a", "copyx a"), "keyword.command.batchfile");
}

#[test]
fn echo_takes_its_state_only_at_the_end_of_the_line() {
    assert_eq!(
        leaf("echo off", "off"),
        "keyword.other.special-method.batchfile"
    );
    assert_eq!(leaf("@echo off", "echo"), "keyword.command.batchfile");
    assert_ne!(
        leaf("echo off now", "off now"),
        "keyword.other.special-method.batchfile"
    );
}

#[test]
fn comments_take_the_rest_of_the_line() {
    assert_eq!(
        leaf("rem a | b", "|"),
        "invalid.illegal.unexpected-character.batchfile"
    );
    assert_eq!(
        leaf(":: note", "::"),
        "punctuation.definition.comment.batchfile"
    );
    assert_eq!(
        leaf("x & :: note", "&"),
        "keyword.operator.conditional.batchfile"
    );
}

#[test]
fn a_for_line_reads_its_parameter_before_any_other_rule() {
    let rows = lex("for %%i in (*.txt) do echo %%~nxi");
    let row = &rows[0];
    let parameter = |text: &str| {
        row.iter()
            .filter(|(t, _)| t == text)
            .map(|(_, s)| s.last().cloned().unwrap_or_default())
            .collect::<Vec<_>>()
    };
    assert_eq!(
        parameter("%%"),
        ["punctuation.definition.variable.batchfile"; 2]
    );
    assert!(
        row.iter()
            .all(|(_, s)| s.iter().any(|name| name == "meta.block.repeat.batchfile")),
        "{row:?}"
    );
    assert_eq!(
        leaf("echo %%i", "%%"),
        "constant.character.escape.batchfile"
    );
}

#[test]
fn an_in_clause_ends_at_the_line_feed_only_after_a_parenthesis() {
    let rows = lex(indoc! {"
        for %%i in (a) x
        set /a x
        goto y"});
    assert!(
        rows[2][0]
            .1
            .iter()
            .any(|name| name == "meta.block.repeat.batchfile"),
        "{:?}",
        rows[2]
    );
    let rows = lex(indoc! {"
        for %%i in (a)
        x
        goto y"});
    assert!(
        rows[2][0]
            .1
            .iter()
            .all(|name| name != "meta.block.repeat.batchfile"),
        "{:?}",
        rows[2]
    );
}

#[test]
fn an_expansion_names_its_substring() {
    let text = "echo %path:~1,-2%";
    assert_eq!(leaf(text, "path"), "variable.other.readwrite.batchfile");
    assert_eq!(leaf(text, "1"), "constant.numeric.batchfile");
    assert_eq!(leaf(text, "-2"), "constant.numeric.batchfile");
    assert_eq!(leaf(text, ","), "punctuation.separator.batchfile");
}

#[test]
fn set_names_its_variable_and_its_arithmetic() {
    assert_eq!(
        leaf("set name=value", "name"),
        "variable.other.readwrite.batchfile"
    );
    assert_eq!(
        leaf("set /a x+=1", "+="),
        "keyword.operator.assignment.augmented.batchfile"
    );
    assert_eq!(
        leaf("set /a x+=1", "x"),
        "variable.other.readwrite.batchfile"
    );
}

#[test]
fn labels_follow_goto_and_the_start_of_a_line() {
    assert_eq!(
        leaf(":end", "end"),
        "keyword.other.special-method.batchfile"
    );
    assert_eq!(
        leaf("goto :end", "end"),
        "keyword.other.special-method.batchfile"
    );
}

#[test]
fn case_folds_stand_for_their_letters() {
    assert_eq!(
        leaf("\u{17f}et x=1", "\u{17f}et"),
        "keyword.command.batchfile"
    );
    assert_eq!(
        leaf("if a L\u{df} b", "L\u{df}"),
        "keyword.operator.comparison.batchfile"
    );
}
