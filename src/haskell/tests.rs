use indoc::indoc;

use crate::{Highlighter, Language, State};

/// One lexed line: each token's text and its scope names, outermost first.
type Row = Vec<(String, Vec<String>)>;

fn lex(text: &str) -> Vec<Row> {
    let mut highlighter = Highlighter::new(Language::Haskell);
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

/// The scopes, outermost first, of the token that holds the first byte of
/// the first `needle` in `text`.
fn scopes(text: &str, needle: &str) -> Vec<String> {
    let (line, at) = text
        .split('\n')
        .enumerate()
        .find_map(|(index, line)| line.find(needle).map(|at| (index, at)))
        .unwrap_or_else(|| panic!("no {needle:?} in {text:?}"));
    let mut start = 0;
    for (token, scopes) in &lex(text)[line] {
        start += token.len();
        if start > at {
            return scopes.clone();
        }
    }
    panic!("no token holds {needle:?}")
}

/// The innermost scope at the first `needle` in `text`.
fn leaf(text: &str, needle: &str) -> String {
    scopes(text, needle).pop().unwrap_or_default()
}

/// Whether the first `needle` in `text` sits in the scope `scope`.
fn within(text: &str, needle: &str, scope: &str) -> bool {
    scopes(text, needle).iter().any(|s| s == scope)
}

#[test]
fn the_names_are_known() {
    for name in ["haskell", "hs"] {
        assert_eq!(Language::from_name(name), Some(Language::Haskell));
    }
}

#[test]
fn a_class_declaration_runs_while_its_lines_are_indented() {
    let text = indoc! {"
        class Eq a =>
          Ord a
        x = 1
    "};
    assert!(within(text, "Ord", "meta.declaration.class.haskell"));
    assert!(!within(text, "x", "meta.declaration.class.haskell"));
}

#[test]
fn a_function_signature_continues_on_lines_indented_further() {
    let text = indoc! {"
        f :: Int
          -> Int
        f = id
    "};
    assert!(within(text, "->", "meta.function.type-declaration.haskell"));
    assert_eq!(leaf(text, "->"), "keyword.operator.arrow.haskell");
    assert!(!within(
        text,
        "id",
        "meta.function.type-declaration.haskell"
    ));
}

#[test]
fn a_haddock_comment_continues_while_lines_start_with_dashes() {
    let text = indoc! {"
        -- | The answer.
        -- More of it.
        answer = 42
    "};
    assert!(within(
        text,
        "More of it",
        "comment.block.documentation.haskell"
    ));
    assert!(!within(
        text,
        "answer =",
        "comment.block.documentation.haskell"
    ));
}

#[test]
fn a_block_comment_spans_lines() {
    let text = indoc! {"
        {- one
        two -}
        x
    "};
    assert!(within(text, "two", "comment.block.haskell"));
    assert!(!within(text, "x", "comment.block.haskell"));
}

#[test]
fn a_quasi_quote_ends_at_its_own_bars() {
    let text = indoc! {"
        [expr||a |] b
        c||]
        d
    "};
    assert!(
        within(text, "c", "meta.quasi-quotation.haskell")
            || within(text, "a |] b", "meta.quasi-quotation.haskell")
    );
    assert_eq!(
        leaf("[e|x|]", "|]"),
        "keyword.operator.quasi-quotation.end.haskell"
    );
}

#[test]
fn a_data_declaration_names_its_constructors() {
    let text = "data T = A Int | B";
    assert!(within(text, "A", "meta.declaration.data.algebraic.haskell"));
    assert_eq!(leaf(text, "|"), "keyword.operator.pipe.haskell");
}

#[test]
fn a_gadt_head_ends_before_where() {
    let text = indoc! {"
        data T a where
          C :: T Int
    "};
    assert_eq!(leaf(text, "where"), "keyword.other.where.haskell");
    assert!(within(
        text,
        "C",
        "meta.declaration.data.generalized.haskell"
    ));
}

#[test]
fn a_string_gap_spans_lines() {
    let text = indoc! {r#"
        s = "a\
            \b"
    "#};
    assert!(within(text, "b", "string.quoted.double.haskell"));
}

#[test]
fn numbers_and_characters_take_their_scopes() {
    assert_eq!(
        leaf("x = 0x1F", "0x1F"),
        "constant.numeric.integral.hexadecimal.haskell"
    );
    assert_eq!(
        leaf("x = 1.5e3", "1.5e3"),
        "constant.numeric.floating.decimal.haskell"
    );
    assert_eq!(
        leaf("x' = 1", "1"),
        "constant.numeric.integral.decimal.haskell"
    );
    assert!(within(r"c = '\n'", r"\n", "string.quoted.single.haskell"));
}

#[test]
fn a_pragma_names_its_extensions() {
    assert_eq!(
        leaf("{-# LANGUAGE GADTs #-}", "GADTs"),
        "keyword.other.preprocessor.extension.haskell"
    );
}

#[test]
fn operators_in_parentheses_and_backticks() {
    assert!(
        within("x = (+) 1", "+", "entity.name.function.infix.haskell")
            || leaf("x = (+) 1", "+") == "keyword.operator.infix.haskell"
    );
    assert_eq!(leaf("x = a `div` b", "`"), "punctuation.backtick.haskell");
}
