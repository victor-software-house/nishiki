use indoc::indoc;

use crate::{Highlighter, Language, State};

/// One lexed line: each token's text and its scope names, outermost first.
type Row = Vec<(String, Vec<String>)>;

fn lex(text: &str) -> Vec<Row> {
    let mut highlighter = Highlighter::new(Language::Jinja);
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

/// The innermost scope of the first token whose text is `token`.
fn leaf(text: &str, token: &str) -> String {
    lex(text)
        .into_iter()
        .flatten()
        .find(|(t, _)| t == token)
        .unwrap_or_else(|| panic!("no token {token:?} in {text:?}"))
        .1
        .pop()
        .unwrap_or_default()
}

#[test]
fn the_name_is_known() {
    assert_eq!(Language::from_name("jinja"), Some(Language::Jinja));
}

/// The name after `{%` or `{%-` is the tag's keyword.
/// The lookbehind `(?<=\{%-?)` has two lengths, and both hold.
#[test]
fn a_tag_names_its_keyword_after_either_opener() {
    let text = indoc! {"
        {%- for item in items -%}
        {% endfor %}"};
    assert_eq!(leaf(text, "for"), "keyword.control.jinja");
    assert_eq!(leaf(text, "in"), "keyword.control.jinja");
    assert_eq!(leaf(text, "items"), "variable.other.jinja");
    assert_eq!(leaf(text, "endfor"), "keyword.control.jinja");
}

/// A name needs no `\b` before it, so `1abc` holds the name `abc`.
#[test]
fn a_name_may_follow_a_digit() {
    assert_eq!(leaf("{{ 1abc }}", "abc"), "variable.other.jinja");
}

/// The first name of a tag that a `,` follows is no keyword.
#[test]
fn a_comma_keeps_a_name_plain() {
    let text = "{% set x, y = 1, 2 %}";
    assert_eq!(leaf(text, "set"), "keyword.control.jinja");
    assert_eq!(leaf(text, "x"), "variable.other.jinja");
    assert_eq!(leaf(text, "="), "keyword.operator.assignment.jinja");
}

/// The escape `\` before the line feed takes it, and the string goes on.
#[test]
fn an_escaped_line_feed_continues_the_string() {
    let rows = lex("{{ \"a\\\nb\" }}");
    assert_eq!(
        rows[0]
            .last()
            .and_then(|(_, s)| s.last())
            .map(String::as_str),
        Some("constant.character.escape.newline.jinja")
    );
    assert_eq!(
        rows[1][0].1.last().map(String::as_str),
        Some("string.quoted.double.jinja")
    );
}

/// An inner dict's `}` closes before the output's `}}`.
#[test]
fn nested_dicts_close_before_the_output() {
    let rows = lex("{{ {\"k\": {\"j\": 1}} }}");
    let closers = rows[0]
        .iter()
        .filter(|(text, scopes)| {
            text == "}"
                && scopes
                    .last()
                    .is_some_and(|s| s == "punctuation.other.jinja")
        })
        .count();
    assert_eq!(closers, 2);
    assert_eq!(
        rows[0]
            .last()
            .and_then(|(_, s)| s.last())
            .map(String::as_str),
        Some("variable.entity.other.jinja.delimiter")
    );
}

/// A raw block keeps its tags as text.
#[test]
fn a_raw_block_holds_its_tags_as_text() {
    let rows = lex("{% raw %}{{ x }}{% endraw %}");
    assert!(
        rows[0]
            .iter()
            .all(|(_, scopes)| scopes.iter().any(|s| s == "comment.block.jinja.raw")),
        "{:?}",
        rows[0]
    );
}

#[cfg(feature = "json")]
mod probe {
    use crate::jinja::engine::Jinja;
    use crate::jinja::grammar::Pat;
    use crate::test_support::probe::{answer_cases, lexer};

    /// Answers the cases of `oracle/onig.ts` with Jinja's matchers
    /// (see [`answer_cases`]).
    #[test]
    #[ignore = "a tool: it needs a cases file"]
    fn matchers_answer_the_probe_cases() {
        answer_cases(&Pat::ALL, |arena, pat, case| {
            lexer::<Jinja, _>(arena, case, |lexer| {
                lexer.probe_matcher(case.p, lexer.g, |lexer, p| lexer.matcher(pat, p))
            })
        });
    }
}
