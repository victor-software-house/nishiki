use indoc::indoc;

use crate::Language;
use crate::test_support::{self, LeafRow, leaf_in, merge_by_leaf};

/// Lexes the lines of `text` in order from one start state.
fn lex_lines(text: &str) -> Vec<LeafRow> {
    merge_by_leaf(test_support::lex(Language::Nix, text))
}

fn lex(line: &str) -> LeafRow {
    lex_lines(line).remove(0)
}

#[test]
fn the_name_is_known() {
    assert_eq!(Language::from_name("nix"), Some(Language::Nix));
}

#[test]
fn a_let_binds_names_until_its_in() {
    let row = lex("let a = 1; in a");
    assert_eq!(leaf_in(&row, "let"), "keyword.other.nix");
    assert_eq!(
        leaf_in(&row, "a"),
        "entity.other.attribute-name.multipart.nix"
    );
    assert_eq!(leaf_in(&row, "="), "keyword.operator.bind.nix");
    assert_eq!(leaf_in(&row, ";"), "punctuation.terminator.bind.nix");
    assert_eq!(leaf_in(&row, "in"), "keyword.other.nix");
    assert_eq!(
        row.last().map(|(_, scope)| scope.as_str()),
        Some("variable.parameter.name.nix")
    );
}

/// A brace whose content holds a `,` or a `?` opens a function head.
/// The blank after `...` stays in the operator.
#[test]
fn a_brace_with_parameters_opens_a_function_head() {
    let row = lex("{ x, y ? 2, ... }: x");
    assert_eq!(
        leaf_in(&row, "{"),
        "punctuation.definition.entity.function.2.nix"
    );
    assert_eq!(leaf_in(&row, "x"), "variable.parameter.function.1.nix");
    assert_eq!(leaf_in(&row, "?"), "keyword.operator.nix");
    assert_eq!(leaf_in(&row, "... "), "keyword.operator.nix");
    assert_eq!(
        leaf_in(&row, "}"),
        "punctuation.definition.entity.function.nix"
    );
    assert_eq!(leaf_in(&row, ":"), "punctuation.definition.function.nix");
}

#[test]
fn a_brace_with_bindings_opens_an_attribute_set() {
    let row = lex("rec { a.b = 1; inherit (x) y; }");
    assert_eq!(leaf_in(&row, "rec"), "keyword.other.nix");
    assert_eq!(leaf_in(&row, "{"), "punctuation.definition.attrset.nix");
    assert_eq!(leaf_in(&row, "inherit"), "keyword.other.inherit.nix");
    assert_eq!(
        leaf_in(&row, "("),
        "punctuation.section.function.arguments.nix"
    );
    assert_eq!(leaf_in(&row, "y"), "entity.other.attribute-name.single.nix");
    assert_eq!(leaf_in(&row, ";"), "punctuation.terminator.bind.nix");
    assert_eq!(leaf_in(&row, "}"), "punctuation.definition.attrset.nix");
}

/// The grammar splits `then` and `else` into two tokens each.
/// Both halves share one scope, so they merge here.
#[test]
fn a_conditional_names_its_keywords() {
    let row = lex("if a then b else c");
    assert_eq!(leaf_in(&row, "if"), "keyword.other.nix");
    assert_eq!(leaf_in(&row, "then"), "keyword.other.nix");
    assert_eq!(leaf_in(&row, "else"), "keyword.other.nix");
}

#[test]
fn an_indented_string_runs_across_lines_and_keeps_its_escapes() {
    let rows = lex_lines(indoc! {"
        ''
          a ''${b}
        '' x
    "});
    assert_eq!(
        leaf_in(&rows[0], "''"),
        "punctuation.definition.string.other.start.nix"
    );
    assert_eq!(leaf_in(&rows[1], "  a "), "string.quoted.other.nix");
    assert_eq!(leaf_in(&rows[1], "''$"), "constant.character.escape.nix");
    assert_eq!(leaf_in(&rows[1], "{b}"), "string.quoted.other.nix");
    assert_eq!(
        leaf_in(&rows[2], "''"),
        "punctuation.definition.string.other.end.nix"
    );
    assert_eq!(leaf_in(&rows[2], "x"), "variable.parameter.name.nix");
}

#[test]
fn a_quoted_string_holds_interpolations_and_escapes() {
    let row = lex(r#""a ${b} \" c" d"#);
    assert_eq!(
        leaf_in(&row, "${"),
        "punctuation.section.embedded.begin.nix"
    );
    assert_eq!(leaf_in(&row, "b"), "variable.parameter.name.nix");
    assert_eq!(leaf_in(&row, r#"\""#), "constant.character.escape.nix");
    assert_eq!(leaf_in(&row, "d"), "variable.parameter.name.nix");
}

#[test]
fn paths_search_paths_and_urls_are_unquoted_strings() {
    let row = lex("x: import ./a/b.nix <nixpkgs> https://x.org/a");
    assert_eq!(leaf_in(&row, "x"), "variable.parameter.function.4.nix");
    assert_eq!(leaf_in(&row, "import"), "support.function.nix");
    assert_eq!(leaf_in(&row, "./a/b.nix"), "string.unquoted.path.nix");
    assert_eq!(leaf_in(&row, "<nixpkgs>"), "string.unquoted.spath.nix");
    assert_eq!(leaf_in(&row, "https://x.org/a"), "string.unquoted.url.nix");
}

/// The comment's first match runs over `**/`, as Shiki's does.
/// The comment then ends at the next `*/`.
#[test]
fn a_block_comment_runs_past_a_doubled_star() {
    let row = lex("/* a **/ b */ c # d");
    assert_eq!(leaf_in(&row, "/* a **/ b */"), "comment.block.nix");
    assert_eq!(leaf_in(&row, "c"), "variable.parameter.name.nix");
    assert_eq!(leaf_in(&row, "# d"), "comment.line.number-sign.nix");
}

#[test]
fn builtins_and_operators_are_named() {
    let row = lex("with pkgs; builtins.map f [ 1 ]");
    assert_eq!(leaf_in(&row, "with"), "keyword.other.nix");
    assert_eq!(leaf_in(&row, "builtins"), "constant.language.nix");
    assert_eq!(leaf_in(&row, "."), "keyword.operator.nix");
    assert_eq!(leaf_in(&row, "map"), "support.function.nix");
    assert_eq!(leaf_in(&row, "["), "punctuation.definition.list.nix");
    assert_eq!(leaf_in(&lex("a.b or c"), "or"), "keyword.operator.nix");
}

#[cfg(feature = "json")]
mod probe {
    use crate::nix::engine::Nix;
    use crate::nix::grammar::Pat;
    use crate::test_support::probe::{answer_cases, lexer};

    /// Answers the cases of `oracle/onig.ts` with Nix's matchers (see
    /// [`answer_cases`]). The text of a case is the start of a document.
    /// `\A` then holds at its first byte, as it does for Oniguruma.
    #[test]
    #[ignore = "a tool: it needs a cases file"]
    fn matchers_answer_the_probe_cases() {
        answer_cases(&Pat::ALL, |arena, pat, case| {
            lexer::<Nix, _>(arena, case, |lexer| {
                lexer.first_line = true;
                lexer.probe_matcher(case.p, lexer.g, |lexer, p| lexer.matcher(pat, p))
            })
        });
    }
}
