use crate::{Highlighter, Language, State};

/// One lexed line: each token's text and its innermost scope name.
type Row = Vec<(String, String)>;

fn lex_in(highlighter: &mut Highlighter, state: &mut State, line: &str) -> Row {
    let mut tokens = Vec::new();
    let report = highlighter.line(line, state, &mut tokens);
    assert!(report.is_complete(), "{line:?} reached a limit");
    let mut start = 0;
    let mut row = Row::new();
    for token in &tokens {
        let innermost = highlighter
            .scopes(token.stack())
            .next()
            .unwrap_or_default()
            .to_owned();
        let text = &line[start..token.end()];
        match row.last_mut() {
            Some((last, scope)) if *scope == innermost => last.push_str(text),
            _ => row.push((text.to_owned(), innermost)),
        }
        start = token.end();
    }
    assert_eq!(start, line.len(), "the tokens cover the line {line:?}");
    row
}

fn lex(line: &str) -> Row {
    lex_in(
        &mut Highlighter::new(Language::Go),
        &mut State::default(),
        line,
    )
}

#[test]
fn an_empty_line_has_no_tokens() {
    assert_eq!(lex(""), Row::new());
}

#[test]
fn an_empty_line_has_no_tokens_in_a_package() {
    assert_eq!(lex(""), Row::new());
}

/// The innermost scope of the token that is exactly `text`.
fn scope_of(row: &Row, text: &str) -> String {
    row.iter()
        .find(|(token, _)| token == text)
        .unwrap_or_else(|| panic!("no token {text:?} in {row:?}"))
        .1
        .clone()
}

/// Lexes `lines` in order from one start state.
fn lex_lines(lines: &[&str]) -> Vec<Row> {
    let mut highlighter = Highlighter::new(Language::Go);
    let mut state = State::default();
    lines
        .iter()
        .map(|line| lex_in(&mut highlighter, &mut state, line))
        .collect()
}

#[test]
fn a_package_clause_names_its_package() {
    let row = lex("package main");
    assert_eq!(scope_of(&row, "package"), "keyword.package.go");
    assert_eq!(scope_of(&row, "main"), "entity.name.type.package.go");
}

/// The begin takes the line feed, so `\G` holds at the start of the next
/// line, where the end `(?!\G)` cannot match.
#[test]
fn a_package_name_on_the_next_line_is_still_the_package() {
    let rows = lex_lines(&["package ", "main"]);
    assert_eq!(scope_of(&rows[1], "main"), "entity.name.type.package.go");
}

#[test]
fn a_function_declaration_names_its_function_and_parameters() {
    let row = lex("func handler(w http.ResponseWriter, r *http.Request) {");
    assert_eq!(scope_of(&row, "handler"), "entity.name.function.go");
    assert_eq!(scope_of(&row, "w"), "variable.parameter.go");
    assert_eq!(scope_of(&row, "ResponseWriter"), "entity.name.type.go");
    assert_eq!(scope_of(&row, "*"), "keyword.operator.address.go");
}

#[test]
fn numbers_are_decimal_or_have_their_own_prefix() {
    let row = lex("x := 42 + 0x1F + 1.5e3i");
    assert_eq!(scope_of(&row, "42"), "constant.numeric.decimal.go");
    assert_eq!(scope_of(&row, "1F"), "constant.numeric.hexadecimal.go");
    assert_eq!(scope_of(&row, "i"), "keyword.other.unit.imaginary.go");
    assert_eq!(scope_of(&row, "x"), "variable.other.assignment.go");
}

#[test]
fn a_string_has_escapes_and_placeholders() {
    let row = lex("\"a\\n %[2]s %d\"");
    assert_eq!(scope_of(&row, "\\n"), "constant.character.escape.go");
    assert_eq!(scope_of(&row, "%[2]s"), "constant.other.placeholder.go");
    assert_eq!(scope_of(&row, "%d"), "constant.other.placeholder.go");
}

#[test]
fn a_raw_string_runs_over_lines() {
    let rows = lex_lines(&["x := `one", "two`", "y"]);
    assert_eq!(scope_of(&rows[1], "two"), "string.quoted.raw.go");
    assert_eq!(scope_of(&rows[2], "y"), "variable.other.go");
}

#[test]
fn an_import_block_names_its_paths() {
    let rows = lex_lines(&["import (", "\t\"fmt\"", "\tf \"os\"", ")"]);
    assert_eq!(scope_of(&rows[1], "fmt"), "entity.name.import.go");
    assert_eq!(
        scope_of(&rows[3], ")"),
        "punctuation.definition.imports.end.bracket.round.go"
    );
}

#[test]
fn a_block_comment_runs_over_lines() {
    let rows = lex_lines(&["/* one", "two */ x"]);
    assert_eq!(scope_of(&rows[0], " one"), "comment.block.go");
    assert_eq!(scope_of(&rows[1], "x"), "variable.other.go");
}

#[test]
fn a_struct_keeps_its_fields_open_over_lines() {
    let rows = lex_lines(&["type T struct {", "\tName string", "}"]);
    assert_eq!(scope_of(&rows[0], "T"), "entity.name.type.go");
    assert_eq!(scope_of(&rows[1], "Name"), "variable.other.property.go");
    assert_eq!(scope_of(&rows[1], "string"), "storage.type.string.go");
}

#[test]
fn a_call_is_a_function_and_a_method_call_too() {
    let row = lex("fmt.Println(len(x))");
    assert_eq!(scope_of(&row, "Println"), "entity.name.function.support.go");
    assert_eq!(
        scope_of(&row, "len"),
        "entity.name.function.support.builtin.go"
    );
}

#[test]
fn a_rune_has_escapes() {
    let row = lex("'\\n'");
    assert_eq!(scope_of(&row, "\\n"), "constant.other.rune.go");
}

#[test]
fn keywords_and_storage_types_have_their_own_scopes() {
    let row = lex("for range ch { var n int }");
    assert_eq!(scope_of(&row, "for"), "keyword.control.go");
    assert_eq!(scope_of(&row, "var"), "keyword.var.go");
    assert_eq!(scope_of(&row, "int"), "storage.type.numeric.go");
}

#[test]
fn a_label_and_a_goto_name_their_target() {
    let rows = lex_lines(&["loop:", "\tbreak loop"]);
    assert_eq!(scope_of(&rows[0], "loop"), "variable.other.label.go");
    assert_eq!(scope_of(&rows[1], "loop"), "variable.other.label.go");
}
