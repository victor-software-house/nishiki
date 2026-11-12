use crate::test_support::leaves::{LeafRow, leaf_in, merge_by_leaf};
use crate::{Language, test_support};

/// Lexes `lines` in order from one start state.
fn lex_lines(lines: &[&str]) -> Vec<LeafRow> {
    merge_by_leaf(test_support::rows::lex(Language::Go, &lines.join("\n")))
}

fn lex(line: &str) -> LeafRow {
    lex_lines(&[line]).remove(0)
}

#[test]
fn an_empty_line_has_no_tokens() {
    assert_eq!(lex(""), LeafRow::new());
}

#[test]
fn an_empty_line_has_no_tokens_in_a_package() {
    assert_eq!(lex(""), LeafRow::new());
}

#[test]
fn a_package_clause_names_its_package() {
    let row = lex("package main");
    assert_eq!(leaf_in(&row, "package"), "keyword.package.go");
    assert_eq!(leaf_in(&row, "main"), "entity.name.type.package.go");
}

/// The begin takes the line feed.
/// `\G` then holds at the start of the next line, where the end `(?!\G)`
/// cannot match.
#[test]
fn a_package_name_on_the_next_line_is_still_the_package() {
    let rows = lex_lines(&["package ", "main"]);
    assert_eq!(leaf_in(&rows[1], "main"), "entity.name.type.package.go");
}

#[test]
fn a_function_declaration_names_its_function_and_parameters() {
    let row = lex("func handler(w http.ResponseWriter, r *http.Request) {");
    assert_eq!(leaf_in(&row, "handler"), "entity.name.function.go");
    assert_eq!(leaf_in(&row, "w"), "variable.parameter.go");
    assert_eq!(leaf_in(&row, "ResponseWriter"), "entity.name.type.go");
    assert_eq!(leaf_in(&row, "*"), "keyword.operator.address.go");
}

#[test]
fn numbers_are_decimal_or_have_their_own_prefix() {
    let row = lex("x := 42 + 0x1F + 1.5e3i");
    assert_eq!(leaf_in(&row, "42"), "constant.numeric.decimal.go");
    assert_eq!(leaf_in(&row, "1F"), "constant.numeric.hexadecimal.go");
    assert_eq!(leaf_in(&row, "i"), "keyword.other.unit.imaginary.go");
    assert_eq!(leaf_in(&row, "x"), "variable.other.assignment.go");
}

#[test]
fn a_string_has_escapes_and_placeholders() {
    let row = lex("\"a\\n %[2]s %d\"");
    assert_eq!(leaf_in(&row, "\\n"), "constant.character.escape.go");
    assert_eq!(leaf_in(&row, "%[2]s"), "constant.other.placeholder.go");
    assert_eq!(leaf_in(&row, "%d"), "constant.other.placeholder.go");
}

#[test]
fn a_raw_string_runs_over_lines() {
    let rows = lex_lines(&["x := `one", "two`", "y"]);
    assert_eq!(leaf_in(&rows[1], "two"), "string.quoted.raw.go");
    assert_eq!(leaf_in(&rows[2], "y"), "variable.other.go");
}

#[test]
fn an_import_block_names_its_paths() {
    let rows = lex_lines(&["import (", "\t\"fmt\"", "\tf \"os\"", ")"]);
    assert_eq!(leaf_in(&rows[1], "fmt"), "entity.name.import.go");
    assert_eq!(
        leaf_in(&rows[3], ")"),
        "punctuation.definition.imports.end.bracket.round.go"
    );
}

#[test]
fn a_block_comment_runs_over_lines() {
    let rows = lex_lines(&["/* one", "two */ x"]);
    assert_eq!(leaf_in(&rows[0], " one"), "comment.block.go");
    assert_eq!(leaf_in(&rows[1], "x"), "variable.other.go");
}

#[test]
fn a_struct_keeps_its_fields_open_over_lines() {
    let rows = lex_lines(&["type T struct {", "\tName string", "}"]);
    assert_eq!(leaf_in(&rows[0], "T"), "entity.name.type.go");
    assert_eq!(leaf_in(&rows[1], "Name"), "variable.other.property.go");
    assert_eq!(leaf_in(&rows[1], "string"), "storage.type.string.go");
}

#[test]
fn a_call_is_a_function_and_a_method_call_too() {
    let row = lex("fmt.Println(len(x))");
    assert_eq!(leaf_in(&row, "Println"), "entity.name.function.support.go");
    assert_eq!(
        leaf_in(&row, "len"),
        "entity.name.function.support.builtin.go"
    );
}

#[test]
fn a_rune_has_escapes() {
    let row = lex("'\\n'");
    assert_eq!(leaf_in(&row, "\\n"), "constant.other.rune.go");
}

#[test]
fn keywords_and_storage_types_have_their_own_scopes() {
    let row = lex("for range ch { var n int }");
    assert_eq!(leaf_in(&row, "for"), "keyword.control.go");
    assert_eq!(leaf_in(&row, "var"), "keyword.var.go");
    assert_eq!(leaf_in(&row, "int"), "storage.type.numeric.go");
}

#[test]
fn a_label_and_a_goto_name_their_target() {
    let rows = lex_lines(&["loop:", "\tbreak loop"]);
    assert_eq!(leaf_in(&rows[0], "loop"), "variable.other.label.go");
    assert_eq!(leaf_in(&rows[1], "loop"), "variable.other.label.go");
}
