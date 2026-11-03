use crate::{Highlighter, Language, Limits, State, Work};

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
        &mut Highlighter::new(Language::Dockerfile),
        &mut State::default(),
        line,
    )
}

/// A row from `(text, innermost scope)` pairs.
fn row(parts: &[(&str, &str)]) -> Row {
    parts
        .iter()
        .map(|&(text, scope)| (text.to_owned(), scope.to_owned()))
        .collect()
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
    assert_eq!(lex(""), Row::new());
}

#[test]
fn an_instruction_is_a_keyword_when_a_blank_follows() {
    assert_eq!(lex("run echo"), row(&[("run", SPECIAL), (" echo", SOURCE)]));
    assert_eq!(lex("RUN"), row(&[("RUN", SPECIAL)]));
    assert_eq!(lex("RUNS x"), row(&[("RUNS x", SOURCE)]));
}

#[test]
fn onbuild_is_a_control_keyword_before_the_instruction() {
    assert_eq!(
        lex("ONBUILD ADD . /app"),
        row(&[
            ("ONBUILD", CONTROL),
            (" ", SOURCE),
            ("ADD", SPECIAL),
            (" . /app", SOURCE),
        ])
    );
    assert_eq!(lex("ONBUILD x"), row(&[("ONBUILD x", SOURCE)]));
}

#[test]
fn from_with_as_marks_both_words() {
    assert_eq!(
        lex("FROM alpine AS build"),
        row(&[
            ("FROM", SPECIAL),
            (" alpine ", SOURCE),
            ("AS", SPECIAL),
            (" build", SOURCE),
        ])
    );
    assert_eq!(
        lex("from a-as-b"),
        row(&[
            ("from", SPECIAL),
            (" a-", SOURCE),
            ("as", SPECIAL),
            ("-b", SOURCE)
        ])
    );
    assert_eq!(
        lex("FROM xas ASX"),
        row(&[("FROM", SPECIAL), (" xas ASX", SOURCE)])
    );
}

#[test]
fn a_comment_is_read_only_at_the_start_of_a_line() {
    assert_eq!(
        lex("  # note"),
        row(&[
            ("  ", "punctuation.whitespace.comment.leading.dockerfile"),
            ("#", "punctuation.definition.comment.dockerfile"),
            (" note", COMMENT),
        ])
    );
    assert_eq!(
        lex("RUN a # b"),
        row(&[("RUN", SPECIAL), (" a # b", SOURCE)])
    );
}

#[test]
fn a_string_runs_across_lines_and_hides_comments() {
    let mut highlighter = Highlighter::new(Language::Dockerfile);
    let mut state = State::default();
    assert_eq!(
        lex_in(&mut highlighter, &mut state, "RUN echo \"a"),
        row(&[("RUN", SPECIAL), (" echo ", SOURCE), ("\"a", DOUBLE)])
    );
    assert_eq!(
        lex_in(&mut highlighter, &mut state, "# FROM x AS y \\q"),
        row(&[("# FROM x AS y ", DOUBLE), ("\\q", ESCAPE)])
    );
    assert_eq!(
        lex_in(&mut highlighter, &mut state, "b\" 'c"),
        row(&[("b\"", DOUBLE), (" ", SOURCE), ("'c", SINGLE)])
    );
}

#[test]
fn a_backslash_escapes_one_character_and_not_the_line_feed() {
    assert_eq!(
        lex("a\\\"b\\é\\"),
        row(&[
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
        row(&[("\u{fb05}opsignal", SPECIAL), (" x", SOURCE)])
    );
    assert_eq!(
        lex("wor\u{212a}dir x"),
        row(&[("wor\u{212a}dir", SPECIAL), (" x", SOURCE)])
    );
    assert_eq!(
        lex("FROM a \u{17f}"),
        row(&[("FROM", SPECIAL), (" a \u{17f}", SOURCE)])
    );
}

#[test]
#[cfg(feature = "lang-markdown")]
fn a_fence_inside_a_quote_loses_its_line_start_anchors() {
    let mut highlighter = Highlighter::new(Language::Markdown);
    let mut state = State::default();
    lex_in(&mut highlighter, &mut state, "> ```dockerfile");
    assert_eq!(
        lex_in(&mut highlighter, &mut state, "> RUN x"),
        row(&[
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
