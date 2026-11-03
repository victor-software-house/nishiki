use indoc::indoc;

use crate::Language;
use crate::test_support::{self, LeafRow, leaf_in, merge_by_leaf};

/// Lexes the lines of `text` in order from one start state.
fn lex_lines(text: &str) -> Vec<LeafRow> {
    merge_by_leaf(test_support::lex(Language::Lua, text))
}

fn lex(line: &str) -> LeafRow {
    lex_lines(line).remove(0)
}

#[test]
fn a_long_string_closes_only_at_its_own_level() {
    let row = lex("local s = [==[a]]b]==] x");
    assert_eq!(
        leaf_in(&row, "[==["),
        "punctuation.definition.string.begin.lua"
    );
    assert_eq!(leaf_in(&row, "a]]b"), "string.quoted.other.multiline.lua");
    assert_eq!(
        leaf_in(&row, "]==] "),
        "punctuation.definition.string.end.lua"
    );
    assert_eq!(leaf_in(&row, "x"), "variable.other.lua");
}

#[test]
fn a_long_string_with_no_equals_signs_runs_across_lines() {
    let rows = lex_lines(indoc! {"
        x = [[
        a]] y
    "});
    assert_eq!(
        leaf_in(&rows[0], "[["),
        "punctuation.definition.string.begin.lua"
    );
    assert_eq!(leaf_in(&rows[1], "a"), "string.quoted.other.multiline.lua");
    assert_eq!(
        leaf_in(&rows[1], "]] "),
        "punctuation.definition.string.end.lua"
    );
    assert_eq!(leaf_in(&rows[1], "y"), "variable.other.lua");
}

#[test]
fn a_long_comment_closes_only_at_its_own_level() {
    let row = lex("--[==[ c ]] ]==] y");
    assert_eq!(
        leaf_in(&row, "--[==["),
        "punctuation.definition.comment.begin.lua"
    );
    assert_eq!(leaf_in(&row, " c ]] "), "comment.block.lua");
    assert_eq!(
        leaf_in(&row, "]==]"),
        "punctuation.definition.comment.end.lua"
    );
    assert_eq!(leaf_in(&row, "y"), "variable.other.lua");
}

#[test]
fn a_long_comment_with_no_equals_signs_runs_across_lines() {
    let rows = lex_lines(indoc! {"
        --[[ a
        b ]] y
    "});
    assert_eq!(
        leaf_in(&rows[0], "--[["),
        "punctuation.definition.comment.begin.lua"
    );
    assert_eq!(leaf_in(&rows[1], "b "), "comment.block.lua");
    assert_eq!(
        leaf_in(&rows[1], "]]"),
        "punctuation.definition.comment.end.lua"
    );
    assert_eq!(leaf_in(&rows[1], "y"), "variable.other.lua");
}

#[test]
fn a_documentation_comment_names_its_tags_and_types() {
    let row = lex("---@param name? string");
    assert_eq!(leaf_in(&row, "@param"), "storage.type.annotation.lua");
    assert_eq!(leaf_in(&row, "name"), "entity.name.variable.lua");
    assert_eq!(leaf_in(&row, "?"), "keyword.operator.lua");
    assert_eq!(leaf_in(&row, "string"), "support.type.lua");
}

#[test]
fn a_function_head_names_its_parameters() {
    let row = lex("local function f(a, ...) end");
    assert_eq!(leaf_in(&row, "local"), "keyword.local.lua");
    assert_eq!(leaf_in(&row, "f"), "entity.name.function.lua");
    assert_eq!(leaf_in(&row, "a"), "variable.parameter.function.lua");
    assert_eq!(
        leaf_in(&row, ")"),
        "punctuation.definition.parameters.finish.lua"
    );
    assert_eq!(leaf_in(&row, "end"), "keyword.control.lua");
}

#[test]
fn a_binary_exponent_needs_its_sign() {
    let row = lex("print(0x1p4, 3.5e-2, 0xA.)");
    assert_eq!(
        leaf_in(&row, "0x1"),
        "constant.numeric.float.hexadecimal.lua"
    );
    assert_eq!(leaf_in(&row, "p4, "), "source.lua");
    assert_eq!(leaf_in(&row, "3.5e-2"), "constant.numeric.float.lua");
    assert_eq!(
        leaf_in(&row, "0xA."),
        "constant.numeric.float.hexadecimal.lua"
    );
}

#[test]
fn the_standard_library_is_named_unless_it_is_a_member() {
    let row = lex(r#"string.format("%d\n", math.pi)"#);
    assert_eq!(
        leaf_in(&row, "string.format"),
        "support.function.library.lua"
    );
    assert_eq!(leaf_in(&row, r"\n"), "constant.character.escape.lua");
    assert_eq!(leaf_in(&row, "math.pi"), "constant.language.lua");
    let row = lex("t.x = function() end");
    assert_eq!(leaf_in(&row, "x"), "entity.name.function.lua");
}

#[test]
fn labels_and_attributes_are_named() {
    assert_eq!(leaf_in(&lex("goto continue"), "continue"), "string.tag.lua");
    let row = lex("::continue::");
    assert_eq!(
        leaf_in(&row, "::"),
        "punctuation.section.embedded.begin.lua"
    );
    assert_eq!(
        leaf_in(&lex("local x <const> = 1"), "<const>"),
        "storage.type.attribute.lua"
    );
}

/// Shiki lexes the body of `ffi.cdef[[...]]` as C; the body stays the
/// embedding rule's content until the engine can run another lexer inside a
/// rule.
#[test]
fn an_ffi_cdef_body_is_left_unlexed() {
    let row = lex("ffi.cdef[[ int x; ]]");
    assert_eq!(
        leaf_in(&row, "[["),
        "punctuation.definition.string.begin.lua"
    );
    assert_eq!(leaf_in(&row, " int x; "), "meta.embedded.lua");
    assert_eq!(leaf_in(&row, "]]"), "punctuation.definition.string.end.lua");
}
