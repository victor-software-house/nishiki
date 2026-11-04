use indoc::indoc;

use crate::{Highlighter, Language, State};

/// One lexed line: each token's text and its scope names, outermost first.
type Row = Vec<(String, Vec<String>)>;

fn lex(text: &str) -> Vec<Row> {
    let mut highlighter = Highlighter::new(Language::CMake);
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
    assert_eq!(Language::from_name("cmake"), Some(Language::CMake));
}

#[test]
fn a_command_takes_the_blanks_before_it() {
    let rows = lex(indoc! {"
        cmake_minimum_required(VERSION 3.20)
          project(Foo LANGUAGES CXX)
    "});
    assert_eq!(rows[0][0].0, "cmake_minimum_required");
    assert_eq!(rows[1][0].0, "  project");
    assert_eq!(
        rows[1][0].1.last().map(String::as_str),
        Some("keyword.cmake")
    );
}

#[test]
fn words_match_without_regard_to_case_and_with_wildcards() {
    let text = "set_target_properties(foo PROPERTIES Foo_POSTFIX d OUTPUT_NAME bar)";
    assert_eq!(leaf(text, "Foo_POSTFIX"), "variable.source.cmake");
    assert_eq!(leaf(text, "OUTPUT_NAME"), "entity.source.cmake");
    let text = "if(WIN32 AND NOT msvc) # comment";
    assert_eq!(leaf(text, "WIN32"), "constant.source.cmake");
    assert_eq!(leaf(text, "AND"), "keyword.cmake");
    assert_eq!(leaf(text, "msvc"), "constant.source.cmake");
    assert_eq!(leaf(text, "# comment"), "comment.source.cmake");
}

#[test]
fn a_string_holds_escapes_but_no_expansions() {
    let text = r#"set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall \"x\"")"#;
    assert_eq!(leaf(text, "CMAKE_CXX_FLAGS"), "variable.source.cmake");
    assert_eq!(
        leaf(text, "${CMAKE_CXX_FLAGS} -Wall "),
        "string.source.cmake"
    );
    assert_eq!(leaf(text, "\\\""), "constant.character.escape");
}

#[test]
fn a_bracket_closes_only_at_its_own_equals_run() {
    let rows = lex(indoc! {"
        #[[ bracket
        comment ]]
        message([=[ raw ]] ]=] $ENV{HOME})
    "});
    assert_eq!(
        rows[1][1].1.last().map(String::as_str),
        Some("comment.source.cmake")
    );
    let text = "message([=[ raw ]] ]=] $ENV{HOME})";
    assert_eq!(leaf(text, " raw ]] "), "argument.source.cmake");
    assert_eq!(leaf(text, "]=]"), "argument.source.cmake");
    assert_eq!(leaf(text, "$ENV{HOME}"), "storage.source.cmake");
}
