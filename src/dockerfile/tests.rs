use crate::test_support::leaves::{LeafRow, leaf_row, merge_by_leaf};
use crate::{Highlighter, Language, Limits, State, Work, test_support};

/// Lexes `lines` in order from one start state.
fn lex_lines(lines: &[&str]) -> Vec<LeafRow> {
    merge_by_leaf(test_support::rows::lex(
        Language::Dockerfile,
        &lines.join("\n"),
    ))
}

fn lex(line: &str) -> LeafRow {
    lex_lines(&[line]).remove(0)
}

const SPECIAL: &str = "keyword.other.special-method.dockerfile";
const CONTROL: &str = "keyword.control.dockerfile";
const SOURCE: &str = "source.dockerfile";
const DOUBLE: &str = "string.quoted.double.dockerfile";
const SINGLE: &str = "string.quoted.single.dockerfile";
const ESCAPE: &str = "constant.character.escaped.dockerfile";
const COMMENT: &str = "comment.line.number-sign.dockerfile";

#[test]
fn an_empty_line_has_no_tokens() {
    assert_eq!(lex(""), LeafRow::new());
}

#[test]
fn an_instruction_is_a_keyword_when_a_blank_follows() {
    assert_eq!(
        lex("run echo"),
        leaf_row(&[("run", SPECIAL), (" echo", SOURCE)])
    );
    assert_eq!(lex("RUN"), leaf_row(&[("RUN", SPECIAL)]));
    assert_eq!(lex("RUNS x"), leaf_row(&[("RUNS x", SOURCE)]));
}

#[test]
fn onbuild_is_a_control_keyword_before_the_instruction() {
    assert_eq!(
        lex("ONBUILD ADD . /app"),
        leaf_row(&[
            ("ONBUILD", CONTROL),
            (" ", SOURCE),
            ("ADD", SPECIAL),
            (" . /app", SOURCE),
        ])
    );
    assert_eq!(lex("ONBUILD x"), leaf_row(&[("ONBUILD x", SOURCE)]));
}

#[test]
fn from_with_as_marks_both_words() {
    assert_eq!(
        lex("FROM alpine AS build"),
        leaf_row(&[
            ("FROM", SPECIAL),
            (" alpine ", SOURCE),
            ("AS", SPECIAL),
            (" build", SOURCE),
        ])
    );
    assert_eq!(
        lex("from a-as-b"),
        leaf_row(&[
            ("from", SPECIAL),
            (" a-", SOURCE),
            ("as", SPECIAL),
            ("-b", SOURCE)
        ])
    );
    assert_eq!(
        lex("FROM xas ASX"),
        leaf_row(&[("FROM", SPECIAL), (" xas ASX", SOURCE)])
    );
}

#[test]
fn a_comment_is_read_only_at_the_start_of_a_line() {
    assert_eq!(
        lex("  # note"),
        leaf_row(&[
            ("  ", "punctuation.whitespace.comment.leading.dockerfile"),
            ("#", "punctuation.definition.comment.dockerfile"),
            (" note", COMMENT),
        ])
    );
    assert_eq!(
        lex("RUN a # b"),
        leaf_row(&[("RUN", SPECIAL), (" a # b", SOURCE)])
    );
}

#[test]
fn a_string_runs_across_lines_and_hides_comments() {
    let rows = lex_lines(&["RUN echo \"a", "# FROM x AS y \\q", "b\" 'c"]);
    assert_eq!(
        rows[0],
        leaf_row(&[("RUN", SPECIAL), (" echo ", SOURCE), ("\"a", DOUBLE)])
    );
    assert_eq!(
        rows[1],
        leaf_row(&[("# FROM x AS y ", DOUBLE), ("\\q", ESCAPE)])
    );
    assert_eq!(
        rows[2],
        leaf_row(&[("b\"", DOUBLE), (" ", SOURCE), ("'c", SINGLE)])
    );
}

#[test]
fn a_backslash_escapes_one_character_and_not_the_line_feed() {
    assert_eq!(
        lex("a\\\"b\\é\\"),
        leaf_row(&[
            ("a", SOURCE),
            ("\\\"", ESCAPE),
            ("b", SOURCE),
            ("\\é", ESCAPE),
            ("\\", SOURCE),
        ])
    );
}

#[test]
fn the_folds_of_case_insensitive_matching_apply() {
    assert_eq!(
        lex("\u{fb05}opsignal x"),
        leaf_row(&[("\u{fb05}opsignal", SPECIAL), (" x", SOURCE)])
    );
    assert_eq!(
        lex("wor\u{212a}dir x"),
        leaf_row(&[("wor\u{212a}dir", SPECIAL), (" x", SOURCE)])
    );
    assert_eq!(
        lex("FROM a \u{17f}"),
        leaf_row(&[("FROM", SPECIAL), (" a \u{17f}", SOURCE)])
    );
}

#[test]
#[cfg(feature = "lang-markdown")]
fn a_fence_inside_a_quote_loses_its_line_start_anchors() {
    let rows = merge_by_leaf(test_support::rows::lex(
        Language::Markdown,
        &["> ```dockerfile", "> RUN x"].join("\n"),
    ));
    assert_eq!(
        rows[1],
        leaf_row(&[
            (">", "punctuation.definition.quote.begin.markdown"),
            (" ", "markup.quote.markdown"),
            ("RUN x", "meta.embedded.block.dockerfile"),
        ])
    );
}

#[test]
fn a_depth_limit_keeps_a_string_closed() {
    let limits = Limits {
        depth: Some(0),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Dockerfile, limits);
    let mut state = State::default();
    let mut tokens = Vec::new();
    let report = highlighter.line("RUN \"a", &mut state, &mut tokens);
    assert!(report.reached(crate::Limit::Depth));
    assert_eq!(tokens.last().map(|token| token.end()), Some(6));
}

#[test]
fn a_spent_work_budget_ends_the_line() {
    let limits = Limits {
        work: Some(Work {
            per_byte: 0,
            base: 4,
        }),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Dockerfile, limits);
    let mut state = State::default();
    let mut tokens = Vec::new();
    let line = "'a ".repeat(50);
    let report = highlighter.line(&line, &mut state, &mut tokens);
    assert!(report.reached(crate::Limit::Work));
    assert_eq!(tokens.last().map(|token| token.end()), Some(line.len()));
}
