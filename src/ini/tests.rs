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
        &mut Highlighter::new(Language::Ini),
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

const SOURCE: &str = "source.ini";
const KEY: &str = "keyword.other.definition.ini";
const EQUALS: &str = "punctuation.separator.key-value.ini";
const SECTION: &str = "entity.name.section.group-title.ini";
const ENTITY: &str = "punctuation.definition.entity.ini";
const HASH: &str = "comment.line.number-sign.ini";
const SEMICOLON: &str = "comment.line.semicolon.ini";
const MARK: &str = "punctuation.definition.comment.ini";
const LEADING: &str = "punctuation.whitespace.comment.leading.ini";
const SINGLE: &str = "string.quoted.single.ini";
const DOUBLE: &str = "string.quoted.double.ini";
const BEGIN: &str = "punctuation.definition.string.begin.ini";
const END: &str = "punctuation.definition.string.end.ini";
const ESCAPE: &str = "constant.character.escape.ini";

#[test]
fn an_empty_line_has_no_tokens() {
    assert_eq!(lex(""), Row::new());
}

#[test]
fn a_key_and_its_equals_sign_are_found_anywhere() {
    assert_eq!(
        lex("name = value"),
        row(&[
            ("name", KEY),
            (" ", SOURCE),
            ("=", EQUALS),
            (" value", SOURCE),
        ])
    );
    assert_eq!(
        lex("a=b=c"),
        row(&[
            ("a", KEY),
            ("=", EQUALS),
            ("b", KEY),
            ("=", EQUALS),
            ("c", SOURCE),
        ])
    );
}

#[test]
fn a_key_needs_a_word_character_last_and_a_boundary_first() {
    assert_eq!(lex("a-=1"), row(&[("a-=1", SOURCE)]));
    assert_eq!(
        lex("-x=1"),
        row(&[("-", SOURCE), ("x", KEY), ("=", EQUALS), ("1", SOURCE)])
    );
    assert_eq!(lex("\u{e9}a=1"), row(&[("\u{e9}a=1", SOURCE)]));
    assert_eq!(
        lex("\u{e9}-x=1"),
        row(&[
            ("\u{e9}", SOURCE),
            ("-x", KEY),
            ("=", EQUALS),
            ("1", SOURCE)
        ])
    );
}

#[test]
fn a_section_is_read_only_at_the_start_of_a_line() {
    assert_eq!(
        lex("[core] ; x"),
        row(&[
            ("[", ENTITY),
            ("core", SECTION),
            ("]", ENTITY),
            (" ", SOURCE),
            (";", MARK),
            (" x", SEMICOLON),
        ])
    );
    assert_eq!(lex(" [core]"), row(&[(" [core]", SOURCE)]));
    assert_eq!(lex("[core"), row(&[("[core", SOURCE)]));
}

#[test]
fn a_comment_starts_at_its_mark_anywhere_and_keeps_leading_blanks() {
    assert_eq!(
        lex("\t# note"),
        row(&[("\t", LEADING), ("#", MARK), (" note", HASH)])
    );
    assert_eq!(
        lex("a = b # c"),
        row(&[
            ("a", KEY),
            (" ", SOURCE),
            ("=", EQUALS),
            (" b ", SOURCE),
            ("#", MARK),
            (" c", HASH),
        ])
    );
    assert_eq!(
        lex("x ; a # b"),
        row(&[("x ", SOURCE), (";", MARK), (" a # b", SEMICOLON)])
    );
}

#[test]
fn a_string_runs_across_lines_and_hides_comments_and_keys() {
    let mut highlighter = Highlighter::new(Language::Ini);
    let mut state = State::default();
    assert_eq!(
        lex_in(&mut highlighter, &mut state, "a = \"x"),
        row(&[
            ("a", KEY),
            (" ", SOURCE),
            ("=", EQUALS),
            (" ", SOURCE),
            ("\"", BEGIN),
            ("x", DOUBLE),
        ])
    );
    assert_eq!(
        lex_in(&mut highlighter, &mut state, "# b = c \\\" \" d"),
        row(&[
            ("# b = c \\", DOUBLE),
            ("\"", END),
            (" ", SOURCE),
            ("\"", BEGIN),
            (" d", DOUBLE),
        ])
    );
}

#[test]
fn only_a_single_quoted_string_has_escapes() {
    assert_eq!(
        lex("'a\\'b' \"c\\\" d"),
        row(&[
            ("'", BEGIN),
            ("a", SINGLE),
            ("\\'", ESCAPE),
            ("b", SINGLE),
            ("'", END),
            (" ", SOURCE),
            ("\"", BEGIN),
            ("c\\", DOUBLE),
            ("\"", END),
            (" d", SOURCE),
        ])
    );
}

#[test]
fn a_fence_inside_a_quote_loses_its_line_start_anchors() {
    let mut highlighter = Highlighter::new(Language::Markdown);
    let mut state = State::default();
    lex_in(&mut highlighter, &mut state, "> ```ini");
    assert_eq!(
        lex_in(&mut highlighter, &mut state, ">   [a]"),
        row(&[
            (">", "punctuation.definition.quote.begin.markdown"),
            (" ", "markup.quote.markdown"),
            ("  [a]", "meta.embedded.block.ini"),
        ])
    );
}

#[test]
fn a_depth_limit_keeps_a_string_closed() {
    let limits = Limits {
        depth: Some(1),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Ini, limits);
    let mut state = State::default();
    let mut tokens = Vec::new();
    let report = highlighter.line("a = \"b", &mut state, &mut tokens);
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
    let mut highlighter = Highlighter::new_with_limits(Language::Ini, limits);
    let mut state = State::default();
    let mut tokens = Vec::new();
    let line = "a=b ".repeat(50);
    let report = highlighter.line(&line, &mut state, &mut tokens);
    assert!(report.reached(crate::Limit::Work));
    assert_eq!(tokens.last().map(|token| token.end()), Some(line.len()));
}
