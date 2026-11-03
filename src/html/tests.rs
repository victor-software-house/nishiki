use indoc::indoc;

use crate::Language;
use crate::test_support::{leaf, lex, scopes_of};

/// The innermost scope of each token of `row`.
fn leaves(row: &[(String, Vec<String>)]) -> Vec<(&str, &str)> {
    row.iter()
        .map(|(text, scopes)| {
            let innermost = scopes.last().map_or("", String::as_str);
            (text.as_str(), innermost)
        })
        .collect()
}

#[test]
fn the_css_of_a_style_element_ends_at_its_end_tag_in_any_case() {
    let text = "<style>a { color: red }</STYLE>";
    assert_eq!(
        leaf(Language::Html, text, "color"),
        "support.type.property-name.css"
    );
    assert_eq!(leaf(Language::Html, text, "STYLE"), "entity.name.tag.html");
}

/// An end tag that a CSS rule is open around belongs to the CSS: the element
/// ends only where the CSS has no rule open.
#[test]
fn a_style_end_tag_inside_an_open_css_rule_is_css() {
    let rows = lex(
        Language::Html,
        indoc! {"
        <style>a {
        </style> }</style>
    "},
    );
    assert_eq!(
        leaves(&rows[1])[..3],
        [
            ("</", "meta.property-list.css"),
            ("style", "support.type.property-name.css"),
            ("> ", "meta.property-list.css"),
        ]
    );
    assert_eq!(
        leaves(&rows[1]).last(),
        Some(&(">", "punctuation.definition.tag.end.html"))
    );
}

/// The JavaScript of a `<script>` element carries its state from line to line,
/// and the grammar's own `//` rule ends at the end tag.
#[test]
fn the_javascript_of_a_script_element_ends_at_its_end_tag() {
    let rows = lex(
        Language::Html,
        indoc! {"
        <script>
        var a = 1; // x </script>
    "},
    );
    let second = leaves(&rows[1]);
    assert_eq!(second[0], ("var", "storage.type.js"));
    assert!(
        second.contains(&(" x ", "comment.line.double-slash.js")),
        "{second:?}"
    );
    assert_eq!(
        second.last(),
        Some(&(">", "punctuation.definition.tag.end.html"))
    );
    assert!(
        rows[1][0]
            .1
            .iter()
            .filter(|scope| *scope == "source.js")
            .count()
            == 1,
        "{:?}",
        rows[1][0].1
    );
}

/// The injection's search for a `<` inside a start tag that the grammar lexes
/// on its own text must not hide a `<` after the tag.
#[test]
fn a_bare_angle_bracket_after_an_svg_start_tag_is_flagged() {
    let text = "<svg viewBox=\"0 0 1 1\">   </";
    assert_eq!(
        leaf(Language::Html, text, "<").as_str(),
        "punctuation.definition.tag.begin.html"
    );
    let rows = lex(Language::Html, text);
    assert_eq!(
        leaves(&rows[0])[leaves(&rows[0]).len() - 2..],
        [
            ("<", "invalid.illegal.bad-angle-bracket.html"),
            ("/", "meta.element.structure.svg.html")
        ]
    );
}

/// An unrecognized attribute's name may begin with any character but an ASCII
/// blank, a quote, `<`, `=`, `>`, `/` or a control, so U+00A0 starts one.
#[test]
fn a_no_break_space_starts_an_attribute_name() {
    let rows = lex(Language::Html, "<d\u{a0}v id=\"x\">");
    assert_eq!(
        leaves(&rows[0])[2],
        ("\u{a0}v", "entity.other.attribute-name.html")
    );
    assert!(
        rows[0][2]
            .1
            .contains(&"meta.attribute.unrecognized.\u{a0}v.html".to_owned()),
        "{:?}",
        rows[0][2].1
    );
}

/// The JavaScript of a `<script>` element sees the `>` of its tag, as
/// `TextMate` does: a `{` right after it opens an object literal.
#[test]
fn a_brace_right_after_a_script_tag_opens_an_object_literal() {
    assert_eq!(
        scopes_of(Language::Html, "<script>{", "{")[2..],
        [
            "source.js",
            "meta.objectliteral.js",
            "punctuation.definition.block.js"
        ]
    );
}

/// An event handler's code is a capture group that the JavaScript lexer lexes
/// again: it sees the quote before it, so `<` after the quote and blanks is no
/// tag, and no line feed follows the group's end.
#[test]
fn an_event_handler_reads_a_less_than_after_its_quote_as_an_operator() {
    for text in ["<a onclick=\"  <b c=d\">", "<a onclick=\"x <b\">"] {
        assert_eq!(
            leaf(Language::Html, text, "b"),
            "variable.other.readwrite.js",
            "{text}"
        );
    }
}

/// An attribute's `=` in JSX needs a quote, a brace, a comment or a line feed
/// after it. An event handler's code ends before its line's line feed, so an
/// `=` at the end of a quoted value's line is an illegal attribute.
#[test]
fn an_event_handler_line_ending_in_a_jsx_assignment_has_no_line_feed() {
    let rows = lex(
        Language::Html,
        indoc! {r#"
            <a onclick="x
                <meta charset="utf-8" />
        "#},
    );
    let equals = leaves(&rows[1])
        .into_iter()
        .find(|(token, _)| *token == "=")
        .map(|(_, scope)| scope);
    assert_eq!(equals, Some("invalid.illegal.attribute.js"));
}
