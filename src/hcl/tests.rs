//! Each expected scope is Shiki 4.6.0's on the same text.

use indoc::indoc;

use crate::{Highlighter, Language, State};

/// One lexed line: each token's text and its scope names, outermost first,
/// without `source.hcl`.
type Row = Vec<(String, Vec<String>)>;

fn lex(text: &str) -> Vec<Row> {
    let mut highlighter = Highlighter::new(Language::Hcl);
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
                .filter(|scope| scope != "source.hcl")
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

/// The scopes of the first token whose text is `token` on line `row`,
/// joined by spaces.
fn scopes(text: &str, row: usize, token: &str) -> String {
    lex(text)[row]
        .iter()
        .find(|(t, _)| t == token)
        .unwrap_or_else(|| panic!("no token {token:?} on line {row} of {text:?}"))
        .1
        .join(" ")
}

#[test]
fn an_attribute_definition() {
    let text = "x = 1";
    assert_eq!(
        scopes(text, 0, "x"),
        "variable.declaration.hcl variable.other.readwrite.hcl"
    );
    assert_eq!(
        scopes(text, 0, "="),
        "variable.declaration.hcl keyword.operator.assignment.hcl"
    );
    assert_eq!(scopes(text, 0, "1"), "constant.numeric.integer.hcl");
}

#[test]
fn a_block_header_with_its_labels() {
    let text = "resource \"aws_instance\" web nullable {\n}";
    assert_eq!(
        scopes(text, 0, "resource"),
        "meta.block.hcl entity.name.type.hcl"
    );
    assert_eq!(
        scopes(text, 0, "\"aws_instance\""),
        "meta.block.hcl variable.other.enummember.hcl"
    );
    assert_eq!(
        scopes(text, 0, "nullable"),
        "meta.block.hcl variable.other.enummember.hcl"
    );
    assert_eq!(
        scopes(text, 1, "}"),
        "meta.block.hcl punctuation.section.block.end.hcl"
    );
}

#[test]
fn a_block_type_loses_its_scope_up_to_its_last_dash() {
    let text = indoc! {"
        null-x {
        }
        1abc _x {
        }
    "};
    assert_eq!(scopes(text, 0, "null-"), "meta.block.hcl");
    assert_eq!(scopes(text, 0, "x"), "meta.block.hcl entity.name.type.hcl");
    assert_eq!(scopes(text, 2, "1abc _"), "meta.block.hcl");
    assert_eq!(
        scopes(text, 2, "x"),
        "meta.block.hcl variable.other.enummember.hcl"
    );
}

#[test]
fn a_type_keyword_in_a_block_is_not_a_nested_block() {
    let text = indoc! {"
        a {
          map {
          }
        }
    "};
    assert_eq!(scopes(text, 1, "map"), "meta.block.hcl storage.type.hcl");
    assert_eq!(
        scopes(text, 1, "{"),
        "meta.block.hcl meta.braces.hcl punctuation.section.braces.begin.hcl"
    );
}

#[test]
fn a_member_that_starts_with_a_constant_loses_one_character() {
    let rows = lex("x = a.nullable.0.b[*].c.*.d");
    let row: Vec<(&str, &str)> = rows[0]
        .iter()
        .map(|(t, s)| (t.as_str(), s.last().map_or("", String::as_str)))
        .collect();
    assert!(row.contains(&("n", "")));
    assert!(row.contains(&("ullable", "variable.other.member.hcl")));
    assert!(row.contains(&("0", "constant.numeric.integer.hcl")));
    assert!(row.contains(&("*", "keyword.operator.splat.hcl")));
}

#[test]
fn object_keys() {
    let text = indoc! {"
        x = {
          a = 1
          \"b c\" = 2
          (k.v) = 3
          nullable = 4
        }
    "};
    assert_eq!(
        scopes(text, 1, "a"),
        "meta.braces.hcl meta.mapping.key.hcl variable.other.readwrite.hcl"
    );
    assert_eq!(
        scopes(text, 2, "b c"),
        "meta.braces.hcl meta.mapping.key.hcl string.quoted.double.hcl"
    );
    assert_eq!(scopes(text, 2, "="), "meta.braces.hcl keyword.operator.hcl");
    assert_eq!(
        scopes(text, 3, "v"),
        "meta.braces.hcl meta.mapping.key.hcl variable.other.member.hcl"
    );
    assert_eq!(scopes(text, 4, "  nullable = "), "meta.braces.hcl");
}

#[test]
fn an_inline_for_in_an_object_takes_the_rest_of_its_line() {
    let text = indoc! {"
        x = {
          a = { for k in m : k => 1 } }
          b = 2
        }
        c = 3
    "};
    assert_eq!(
        scopes(text, 1, "=>"),
        "meta.braces.hcl meta.braces.hcl storage.type.function.hcl"
    );
    assert_eq!(scopes(text, 1, " } }"), "meta.braces.hcl meta.braces.hcl");
    assert_eq!(
        scopes(text, 3, "}"),
        "meta.braces.hcl meta.braces.hcl punctuation.section.braces.end.hcl"
    );
    assert_eq!(
        scopes(text, 4, "c"),
        "meta.braces.hcl meta.mapping.key.hcl variable.other.readwrite.hcl"
    );
}

#[test]
fn an_inline_for_or_if_starts_inside_a_word_in_an_object() {
    let text = indoc! {"
        x = {
          xfor y
          elif z
        }
    "};
    assert_eq!(scopes(text, 1, "  x"), "meta.braces.hcl");
    assert_eq!(
        scopes(text, 1, "for"),
        "meta.braces.hcl keyword.control.hcl"
    );
    assert_eq!(
        scopes(text, 2, "if"),
        "meta.braces.hcl keyword.control.conditional.hcl"
    );
}

#[test]
fn a_line_comment_keeps_an_inline_if_open() {
    let text = indoc! {"
        x = [
          a if b # c
        ]
        ]
    "};
    assert_eq!(lex(text)[2], [("]".to_owned(), Vec::<String>::new())]);
    assert_eq!(scopes(text, 3, "]"), "punctuation.section.brackets.end.hcl");
    let closed = indoc! {"
        x = [
          a if b
        ]
    "};
    assert_eq!(
        scopes(closed, 2, "]"),
        "punctuation.section.brackets.end.hcl"
    );
}

#[test]
fn a_for_body_reads_slashes_as_operators() {
    let text = "x = [for a in b : a // c\n]";
    assert_eq!(scopes(text, 0, "in"), "keyword.operator.word.hcl");
    assert_eq!(scopes(text, 0, ":"), "keyword.operator.hcl");
    assert_eq!(scopes(text, 0, "/"), "keyword.operator.arithmetic.hcl");
}

#[test]
fn a_heredoc_in_an_inline_for_ends_with_its_line() {
    let text = indoc! {"
        x = [
          for a in <<EOT
          b
        EOT
        ]
    "};
    assert_eq!(
        scopes(text, 1, "EOT"),
        "string.unquoted.heredoc.hcl keyword.control.heredoc.hcl"
    );
    assert_eq!(scopes(text, 3, "EOT"), "variable.other.readwrite.hcl");
}

#[test]
fn a_heredoc_with_interpolations_and_trim_markers() {
    let text = indoc! {"
        y = <<-EOT
        %{ for x in y ~
        }
          EOT
        z = 1
    "};
    assert_eq!(
        scopes(text, 0, "<<-"),
        "string.unquoted.heredoc.hcl keyword.operator.heredoc.hcl"
    );
    assert_eq!(
        scopes(text, 1, "in"),
        "string.unquoted.heredoc.hcl meta.interpolation.hcl keyword.control.hcl"
    );
    assert_eq!(
        scopes(text, 1, " ~"),
        "string.unquoted.heredoc.hcl meta.interpolation.hcl keyword.operator.template.right.trim.hcl"
    );
    assert_eq!(
        scopes(text, 3, "  EOT"),
        "string.unquoted.heredoc.hcl keyword.control.heredoc.hcl"
    );
    assert_eq!(
        scopes(text, 4, "z"),
        "variable.declaration.hcl variable.other.readwrite.hcl"
    );
}

#[test]
fn escapes_and_doubled_markers_in_a_string() {
    let text = "x = \"a\\nb\\x\\u00e9\\U0001F600 $${x}\"";
    assert_eq!(
        scopes(text, 0, "\\n"),
        "string.quoted.double.hcl constant.character.escape.hcl"
    );
    assert_eq!(scopes(text, 0, "b\\x"), "string.quoted.double.hcl");
    assert_eq!(
        scopes(text, 0, "\\u00e9"),
        "string.quoted.double.hcl constant.character.escape.hcl"
    );
    assert_eq!(
        scopes(text, 0, "\\U0001F600 $${x}"),
        "string.quoted.double.hcl"
    );
}

#[test]
fn function_names() {
    let text = "x = a::b::c::d(1) + b-(2) + list(1) + 1max(2)";
    assert_eq!(
        scopes(text, 0, "a::b::c"),
        "meta.function-call.hcl support.function.namespaced.hcl"
    );
    assert_eq!(scopes(text, 0, "::"), "meta.function-call.hcl");
    assert_eq!(
        scopes(text, 0, "d"),
        "meta.function-call.hcl support.function.builtin.hcl"
    );
    assert_eq!(scopes(text, 0, "-"), "meta.function-call.hcl");
    assert_eq!(scopes(text, 0, "list"), "storage.type.hcl");
    assert_eq!(scopes(text, 0, "1max"), "meta.function-call.hcl");
}

#[test]
fn numbers() {
    let text = "x = 1e5 + 1.5e-3 + 12abc + 0x1";
    assert_eq!(scopes(text, 0, "1"), "constant.numeric.float.hcl");
    assert_eq!(
        scopes(text, 0, "e"),
        "constant.numeric.float.hcl punctuation.separator.exponent.hcl"
    );
    assert_eq!(
        scopes(text, 0, "."),
        "constant.numeric.float.hcl punctuation.separator.decimal.hcl"
    );
    assert!(!lex(text)[0].iter().any(|(t, _)| t == "12"));
}

#[test]
fn names_drop_trailing_dashes() {
    let text = "x = [a-, b--c, null-a]";
    assert_eq!(scopes(text, 0, "a"), "variable.other.readwrite.hcl");
    assert_eq!(scopes(text, 0, "b--c"), "variable.other.readwrite.hcl");
    assert_eq!(scopes(text, 0, "null"), "constant.language.hcl");
}

#[test]
fn a_block_comment_spans_lines() {
    let text = "/* e\nf */ x = 1 # g";
    assert_eq!(scopes(text, 1, "f "), "comment.block.hcl");
    assert_eq!(
        scopes(text, 1, "x"),
        "variable.declaration.hcl variable.other.readwrite.hcl"
    );
    assert_eq!(scopes(text, 1, " g"), "comment.line.number-sign.hcl");
}
