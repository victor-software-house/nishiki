use indoc::indoc;

use crate::{Highlighter, Language, State};

/// One lexed line: each token's text and its scope names, outermost first.
type Row = Vec<(String, Vec<String>)>;

fn lex(text: &str) -> Vec<Row> {
    let mut highlighter = Highlighter::new(Language::Elixir);
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
    assert_eq!(Language::from_name("elixir"), Some(Language::Elixir));
}

#[test]
fn a_module_with_a_function() {
    let text = indoc! {"
        defmodule Math do
          def sum(a, b \\\\ 0) when is_integer(a) do
            a + b
          end
        end"};
    assert_eq!(leaf(text, "defmodule"), "keyword.control.module.elixir");
    assert_eq!(leaf(text, "Math"), "entity.name.type.class.elixir");
    assert_eq!(leaf(text, "sum"), "entity.name.function.public.elixir");
    assert_eq!(leaf(text, "\\\\"), "keyword.operator.other.elixir");
    assert_eq!(leaf(text, "is_integer"), "entity.name.function.elixir");
}

#[test]
fn atoms_and_keys_name_their_colon() {
    assert_eq!(leaf(":ok", ":"), "punctuation.definition.constant.elixir");
    assert_eq!(
        leaf("[name: 1]", ":"),
        "punctuation.definition.constant.elixir"
    );
}

/// `fn` with a `->` later on the line opens an anonymous function's head.
/// A line of many `fn` asks for the line's last `->` once.
#[test]
fn fn_reads_the_arrow_after_it() {
    assert_eq!(leaf("f = fn x -> x end", "fn"), "keyword.control.elixir");
    assert_eq!(
        leaf("f = fn x -> x end", "->"),
        "keyword.operator.other.elixir"
    );
    assert_eq!(leaf("fn", "fn"), "keyword.control.elixir");
}

/// `~s` at the end of a line takes the line feed as its delimiter.
/// The sigil then runs to the end of the next line.
#[test]
fn a_sigil_opened_by_the_line_feed_ends_at_the_next() {
    let rows = lex(indoc! {"
        x = ~s
        abc
        y = 1"});
    assert_eq!(
        rows[1][0].1.last().map(String::as_str),
        Some("string.quoted.other.sigil.elixir")
    );
    assert!(
        rows[2]
            .iter()
            .all(|(_, scopes)| scopes.iter().all(|s| !s.starts_with("string"))),
        "{:?}",
        rows[2]
    );
}

/// A `~H` sigil hands its text to HTML up to `"""`.
#[cfg(feature = "lang-html")]
#[test]
fn a_heex_sigil_holds_html() {
    let text = indoc! {r#"
        x = ~H"""
        <div id="a">b</div>
        """
        y = 1"#};
    assert_eq!(leaf(text, "div"), "entity.name.tag.html");
    assert_eq!(leaf(text, "1"), "constant.numeric.integer.elixir");
}
