use indoc::indoc;

use crate::test_support::{self, Row};
use crate::{Highlighter, Language, Limit, Limits, State};

fn lex_as(language: Language, text: &str) -> Vec<Row> {
    test_support::lex(language, text)
}

fn lex(text: &str) -> Vec<Row> {
    lex_as(Language::C, text)
}

/// The scopes of the first token whose text is `token`, outermost first,
/// without the document's `source.c`.
fn scopes_of(text: &str, token: &str) -> Vec<String> {
    test_support::scopes_in(test_support::without_root(lex(text)), text, token)
}

#[test]
fn a_function_definition_and_its_parameters() {
    let text = "int main(void) { return 0; }";
    assert_eq!(
        scopes_of(text, "main"),
        [
            "meta.function.c",
            "meta.function.definition.parameters.c",
            "entity.name.function.c"
        ]
    );
    assert_eq!(
        scopes_of(text, "return"),
        ["meta.block.c", "keyword.control.c"]
    );
}

#[test]
fn numbers_are_lexed_again_by_their_notation() {
    let text = "x = 0x1F + 1.5e-3f + 1'000 + 0b101 + 08 + 0x;";
    assert_eq!(scopes_of(text, "0x"), ["keyword.other.unit.hexadecimal.c"]);
    assert_eq!(scopes_of(text, "1F"), ["constant.numeric.hexadecimal.c"]);
    assert_eq!(
        scopes_of(text, "e"),
        ["keyword.other.unit.exponent.decimal.c"]
    );
    assert_eq!(
        scopes_of(text, "f"),
        ["keyword.other.unit.suffix.floating-point.c"]
    );
    assert_eq!(
        scopes_of(text, "'"),
        [
            "constant.numeric.decimal.c",
            "punctuation.separator.constant.numeric"
        ]
    );
    assert_eq!(scopes_of(text, "08"), ["constant.numeric.decimal.c"]);
    let row = &lex(text)[0];
    let last = row.iter().rev().find(|(t, _)| t == "0x").map(|(_, s)| s);
    assert_eq!(
        last.and_then(|scopes| scopes.last()).map(String::as_str),
        Some("invalid.illegal.constant.numeric"),
        "a prefix with no digits is not a number"
    );
}

#[test]
fn a_string_runs_across_lines_and_marks_its_escapes() {
    let rows = lex(indoc! {"
        char *s = \"a\\n\\q
        b\";
        int y;"});
    let scopes = |row: usize, text: &str| {
        rows[row]
            .iter()
            .find(|(t, _)| t == text)
            .map(|(_, scopes)| scopes.last().cloned().unwrap_or_default())
    };
    assert_eq!(
        scopes(0, "\\n").as_deref(),
        Some("constant.character.escape.c")
    );
    assert_eq!(
        scopes(0, "\\q").as_deref(),
        Some("invalid.illegal.unknown-escape.c")
    );
    assert_eq!(scopes(1, "b").as_deref(), Some("string.quoted.double.c"));
    assert_eq!(
        scopes(2, "int").as_deref(),
        Some("storage.type.built-in.primitive.c")
    );
}

#[test]
fn a_macro_continues_over_a_backslash() {
    let rows = lex(indoc! {"
        #define X(a) \\
          a + 1
        int y;
    "});
    assert_eq!(
        rows[0]
            .last()
            .map(|(_, scopes)| scopes.last().map(String::as_str)),
        Some(Some("constant.character.escape.line-continuation.c"))
    );
    assert_eq!(
        rows[1][0].1,
        ["source.c", "meta.preprocessor.macro.c"],
        "the next line is still the macro"
    );
    assert_eq!(
        rows[2][0].1,
        ["source.c", "storage.type.built-in.primitive.c"]
    );
}

#[test]
fn a_dead_branch_is_a_comment_and_the_other_is_code() {
    let rows = lex(indoc! {"
        #if 0
        int dead;
        #else
        int live;
        #endif
    "});
    assert_eq!(
        rows[1][0].1,
        ["source.c", "comment.block.preprocessor.if-branch.c"]
    );
    assert_eq!(
        rows[3][0].1,
        ["source.c", "storage.type.built-in.primitive.c"]
    );
}

#[test]
fn a_line_comment_continues_over_a_backslash() {
    let rows = lex(indoc! {"
        // c \x5c
        still
        int x;"});
    assert_eq!(
        rows[1][0].1,
        ["source.c", "comment.line.double-slash.c"],
        "the backslash continues the comment"
    );
    assert_eq!(
        rows[2][0].1,
        ["source.c", "storage.type.built-in.primitive.c"]
    );
}

#[test]
fn white_space_before_a_comment_belongs_to_the_comment() {
    assert_eq!(scopes_of("x;  /* c */", "  "), ["comment.block.c"]);
    assert_eq!(scopes_of("x;  // c", "  "), ["comment.line.double-slash.c"]);
}

#[test]
fn doxygen_commands_in_a_documentation_comment() {
    let text = "/** @param[in] x the \\brief x */";
    assert_eq!(
        scopes_of(text, "@param"),
        [
            "comment.block.documentation.c",
            "storage.type.class.doxygen.c"
        ]
    );
    assert_eq!(
        scopes_of(text, "in"),
        [
            "comment.block.documentation.c",
            "keyword.other.parameter.direction.in.c"
        ]
    );
    assert_eq!(
        scopes_of(text, "x"),
        ["comment.block.documentation.c", "variable.parameter.c"]
    );
}

#[test]
fn member_access_reads_what_the_grammar_reads() {
    let text = "void f(void) { p->q.r (1); s.t.u(2); }";
    assert_eq!(
        scopes_of(text, "r"),
        ["meta.block.c", "variable.other.member.c"]
    );
    assert_eq!(
        scopes_of(text, "u"),
        ["meta.block.c", "entity.name.function.member.c"]
    );
    assert_eq!(
        scopes_of(text, "2"),
        [
            "meta.block.c",
            "meta.function-call.member.c",
            "constant.numeric.decimal.c"
        ]
    );
}

#[test]
fn a_declaration_with_a_word_that_starts_with_or_is_an_initialization() {
    let text = "void f(void) { int order(int y); }";
    assert_eq!(
        scopes_of(text, "or"),
        ["meta.block.c", "meta.initialization.c", "variable.other.c"]
    );
}

#[test]
fn a_switch_with_cases() {
    let rows = lex(indoc! {"
        switch (x) {
          case 1:
            break;
          default:
            break;
        }
    "});
    assert_eq!(
        rows[0][0].1,
        [
            "source.c",
            "meta.block.switch.c",
            "meta.head.switch.c",
            "keyword.control.switch.c"
        ]
    );
    assert_eq!(
        rows[1][1].1,
        [
            "source.c",
            "meta.block.switch.c",
            "meta.body.switch.c",
            "meta.conditional.case.c",
            "keyword.control.case.c"
        ]
    );
    assert_eq!(
        rows[2][1].1,
        [
            "source.c",
            "meta.block.switch.c",
            "meta.body.switch.c",
            "keyword.control.c"
        ],
        "the colon ended the case"
    );
}

#[test]
fn a_begin_that_runs_to_the_end_of_the_line_anchors_the_next_one() {
    let rows = lex("asm \n/% x");
    assert_eq!(
        rows[1][0].1,
        ["source.c", "meta.asm.c"],
        "`\\G` matches at the start of the line after a begin that took its line feed"
    );
    assert_eq!(rows[1][1].1, ["source.c", "keyword.operator.c"]);
}

#[test]
fn a_directive_is_one_only_at_the_start_of_the_line() {
    let rows = lex("  #if 0\nx");
    assert_eq!(
        rows[1][0].1,
        ["source.c", "comment.block.preprocessor.if-branch.c"]
    );
    let rows = lex("x #if 0\ny");
    assert_ne!(
        rows[1][0].1,
        ["source.c", "comment.block.preprocessor.if-branch.c"],
        "a `#if` after other text starts nothing"
    );
}

#[test]
#[cfg(feature = "lang-markdown")]
fn a_fence_in_a_list_item_does_not_start_its_directives() {
    let rows = lex_as(
        Language::Markdown,
        indoc! {"
            - item
              ```c
              #include <x.h>
              int y;
              ```"},
    );
    let include = &rows[2][1];
    assert_eq!(include.0, "#include ");
    assert_eq!(
        include.1.last().map(String::as_str),
        Some("meta.embedded.block.c"),
        "the directive follows the indentation, so `^` does not match"
    );
    assert_eq!(
        rows[3][1].1.last().map(String::as_str),
        Some("storage.type.built-in.primitive.c")
    );
}

#[test]
#[cfg(feature = "lang-markdown")]
fn a_fence_line_of_containers_alone_still_ends_a_macro() {
    let rows = lex_as(
        Language::Markdown,
        indoc! {"
            > ```c
            > #define X \\
            >\x20
            > int y;
            > ```"},
    );
    assert_eq!(
        rows[3][2].1.last().map(String::as_str),
        Some("storage.type.built-in.primitive.c"),
        "the line with nothing after `> ` ended the macro"
    );
}

#[test]
fn contexts_past_the_depth_limit_are_not_opened() {
    let limits = Limits {
        depth: Some(4),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::C, limits);
    let mut state = State::default();
    let mut tokens = Vec::new();
    let line = "((((((x))))))";
    let report = highlighter.line(line, &mut state, &mut tokens);
    assert!(report.reached(Limit::Depth));
    assert_eq!(
        tokens.last().map(|token| token.end()),
        Some(line.len()),
        "the tokens cover the line"
    );
    tokens.clear();
    let next = "int y;";
    let report = highlighter.line(next, &mut state, &mut tokens);
    assert!(report.is_complete(), "the state after the limit lexes on");
    assert_eq!(tokens.last().map(|token| token.end()), Some(next.len()));
}

/// A line of spaces inside an assembly statement lexes within its text.
/// The line-end pattern's `\s+` takes the line feed with the spaces.
/// Shiki 4.6.0 scopes the spaces `meta.asm.c` after `asm`, and plain
/// `source.c` once a comment line has closed the statement.
#[test]
fn a_line_of_spaces_inside_an_assembly_statement() {
    let rows = lex(&["asm", "   ", "(\"nop\");"].join("\n"));
    assert_eq!(
        rows[1],
        [(
            "   ".to_owned(),
            vec!["source.c".to_owned(), "meta.asm.c".to_owned()]
        )]
    );
    let rows = lex(&["asm", "/* c */", "  ", "(\"nop\");"].join("\n"));
    assert_eq!(rows[2], [("  ".to_owned(), vec!["source.c".to_owned()])]);
}
