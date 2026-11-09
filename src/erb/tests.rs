use indoc::indoc;

use crate::Language;
use crate::test_support::{leaf, lex, scopes_of};

/// `<%=` opens code that the line closes, whose Ruby ends at `%>`: the `%`
/// is Ruby too, as the end's first group names it.
#[test]
fn code_that_its_line_closes_is_a_line_of_ruby() {
    let text = "<p><%= @x %></p>";
    assert_eq!(
        scopes_of(Language::Erb, text, "<%="),
        [
            "text.html.erb",
            "meta.embedded.line.erb",
            "punctuation.section.embedded.begin.erb"
        ]
    );
    assert_eq!(
        scopes_of(Language::Erb, text, "%"),
        [
            "text.html.erb",
            "meta.embedded.line.erb",
            "punctuation.section.embedded.end.erb",
            "source.ruby"
        ]
    );
    assert_eq!(leaf(Language::Erb, text, "p"), "entity.name.tag.html");
}

/// Where no HTML rule matches, as inside a quoted value, ERB's injection
/// opens code.
#[test]
fn code_opens_inside_a_quoted_value() {
    let scopes = scopes_of(Language::Erb, "<a href=\"<%= url %>\">", "<%=");
    assert!(
        scopes.contains(&"string.quoted.double.html".to_owned())
            && scopes.contains(&"meta.embedded.line.erb".to_owned()),
        "{scopes:?}"
    );
}

/// ERB's injection replaces the HTML grammar's, so a bare `<` is text.
#[test]
fn a_bare_less_than_is_text() {
    let rows = lex(Language::Erb, "a < b");
    assert!(
        rows[0]
            .iter()
            .all(|(_, scopes)| scopes == &["text.html.erb"]),
        "{rows:?}"
    );
}

/// In JavaScript the `<` of `<%=` is an operator, which matches where the
/// injection would and so wins: the `%=` after it is JavaScript too.
#[test]
fn javascript_code_takes_the_less_than_first() {
    assert_eq!(
        leaf(Language::Erb, "<script>var a = <%= j %>;</script>", "%="),
        "keyword.operator.assignment.compound.js"
    );
}

/// At the top level of a `<style>` element's CSS, no CSS rule matches a `<`,
/// so ERB opens there.
#[test]
fn code_opens_between_css_rules() {
    let text = indoc! {"
        <style>
        <% if a %>
        b {}
        <% end %>
        </style>
    "};
    let scopes = scopes_of(Language::Erb, text, "if");
    assert_eq!(
        scopes[scopes.len() - 4..],
        [
            "source.css",
            "meta.embedded.line.erb",
            "source.ruby",
            "keyword.control.ruby"
        ]
    );
}

/// ERB's injection leaves comments alone.
#[test]
fn an_html_comment_holds_no_code() {
    let rows = lex(Language::Erb, "<!-- <%= x %> -->");
    assert!(rows[0].iter().all(|(text, _)| text != "<%="), "{rows:?}");
}

/// `<%#` is a comment up to `%>`; `<%%` is no escape but code whose Ruby
/// starts with `%`.
#[test]
fn a_hash_after_the_percent_makes_a_comment() {
    let text = "<%# note %> <%% raw %>";
    assert_eq!(
        leaf(Language::Erb, text, "<%#"),
        "punctuation.definition.comment.begin.erb"
    );
    assert_eq!(
        leaf(Language::Erb, text, "%>"),
        "punctuation.definition.comment.end.erb"
    );
    assert_eq!(
        scopes_of(Language::Erb, text, "<%")[1],
        "meta.embedded.block.erb"
    );
}

/// A `#` in code that its line closes is ERB's comment up to the end.
/// In code that runs on, it is Ruby's own comment.
#[test]
fn a_hash_in_code_is_erb_comment_only_before_its_end() {
    assert_eq!(
        scopes_of(Language::Erb, "<% x # note %>", "#")[3..],
        [
            "comment.line.number-sign.erb",
            "punctuation.definition.comment.erb"
        ]
    );
    assert_eq!(
        leaf(Language::Erb, "<% x # note", "#"),
        "punctuation.definition.comment.ruby"
    );
}

/// The end `(-?%)>` takes a `-`, and a `%>` inside a Ruby string ends
/// nothing.
#[test]
fn the_end_takes_a_dash_and_skips_strings() {
    assert_eq!(
        scopes_of(Language::Erb, "<% if a -%>", "-%")[2..],
        ["punctuation.section.embedded.end.erb", "source.ruby"]
    );
    let scopes = scopes_of(Language::Erb, "<%= \"%>\" %> y", "%>");
    assert!(
        scopes
            .iter()
            .any(|scope| scope.starts_with("string.quoted")),
        "{scopes:?}"
    );
}

/// Code that its line does not close runs on to the next line.
/// The blanks before and after it at the ends of its lines get scopes of
/// their own.
#[test]
fn code_across_lines_has_its_blanks_named() {
    let text = ["<html>", "  <%# a", "  b %>", "  <% if x", "  y %>  "].join("\n");
    let rows = lex(Language::Erb, &text);
    assert_eq!(
        rows[1][0],
        (
            "  ".to_owned(),
            vec![
                "text.html.erb".to_owned(),
                "punctuation.whitespace.comment.leading.erb".to_owned()
            ]
        )
    );
    assert_eq!(
        rows[3][0].1[1],
        "punctuation.whitespace.embedded.leading.erb"
    );
    assert_eq!(rows[3][1].1[1], "meta.embedded.block.erb");
    let last = rows[4].last().map(|(_, scopes)| scopes.clone());
    assert_eq!(
        last.as_deref()
            .and_then(<[String]>::last)
            .map(String::as_str),
        Some("punctuation.whitespace.embedded.trailing.erb")
    );
}

/// Code that its line does not close starts at the line's blanks, before
/// any JavaScript rule.
/// It therefore opens in a `<script>` element's code, under the `source.js`
/// that the code's lexer adds.
#[test]
fn code_that_runs_on_opens_in_script_code_under_source_js() {
    let text = indoc! {"
        <script>
          <%%= x
        %>
        </script>
    "};
    assert_eq!(
        scopes_of(Language::Erb, text, "<%%=")[1..],
        [
            "meta.embedded.block.html",
            "source.js",
            "meta.embedded.block.erb",
            "punctuation.section.embedded.begin.erb"
        ]
    );
}
