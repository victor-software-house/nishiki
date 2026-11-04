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

/// A token's text and full scope stack.
type Row = Vec<(String, Vec<String>)>;

/// The scopes a Ruby here document named for HTML puts under its lines, as
/// Shiki gives them: the grammar's `text.html.basic` adds none of its own.
const RUBY_HEREDOC: [&str; 3] = ["source.ruby", "meta.embedded.block.html", "text.html"];

/// Lexes `lines` in order from [`super::start_embedded`] over
/// [`RUBY_HEREDOC`], each from the start of the line, with
/// [`super::lex_until`] and `halts`, or with [`super::lex`] when there are
/// none, and returns, per line, where it halted and each token's text and
/// full scope stack.
fn embedded(
    lines: &[&str],
    halts: Option<(crate::language::Halt, Option<crate::language::Halt>)>,
) -> Vec<(Option<usize>, Row)> {
    use crate::arena::{Arena, ROOT};
    use crate::language::{Context, Line};
    use crate::scratch::Scratch;
    use crate::{Limits, LineReport, Token};

    let mut arena = Arena::new(None);
    let mut stack = ROOT;
    for name in RUBY_HEREDOC {
        let scope = arena.named_scope(name).expect("room for the base");
        stack = arena.child(stack, scope).expect("room for the base");
    }
    let mut scratch = Scratch::default();
    let mut state = super::start_embedded(&mut arena, stack);
    let limits = Limits::default();
    let mut rows = Vec::new();
    for text in lines {
        let mut out: Vec<Token> = Vec::new();
        let mut report = LineReport::default();
        let mut cx = Context {
            scratch: &mut scratch,
            arena: &mut arena,
            limits: &limits,
            out: &mut out,
            report: &mut report,
        };
        let line = Line {
            text,
            start: 0,
            embedded: false,
        };
        let (next, halt) = match halts {
            Some((end, after)) => super::lex_until(&mut cx, line, state, end, after),
            None => (super::lex(&mut cx, line, state), None),
        };
        state = next;
        let mut from = 0;
        let mut row = Row::new();
        for token in &out {
            let mut scopes: Vec<String> =
                arena.scopes(token.stack().0).map(str::to_owned).collect();
            scopes.reverse();
            row.push((text[from..token.end()].to_owned(), scopes));
            from = token.end();
        }
        assert_eq!(
            from,
            halt.unwrap_or(text.len()),
            "tokens up to the halt: {text:?}"
        );
        rows.push((halt, row));
    }
    rows
}

/// A Ruby interpolation, which a Ruby here document's patterns try before
/// the HTML's.
fn interpolation(bytes: &[u8], p: usize) -> bool {
    bytes.get(p..p + 2) == Some(b"#{")
}

/// A `<`, for the order of the halts and the patterns at a tag.
fn at_lt(bytes: &[u8], p: usize) -> bool {
    bytes.get(p) == Some(&b'<')
}

/// A Ruby escape, which a Ruby here document's patterns try after the
/// HTML's.
fn escape(bytes: &[u8], p: usize) -> bool {
    bytes.get(p) == Some(&b'\\')
}

/// Where each line halts under a Ruby here document's halts, checking that
/// its tokens are those it gets with no halts: all of them on a line that
/// does not halt, and all but the last on one that does, since the halt cuts
/// the last token short.
fn ruby_halts(lines: &[&str]) -> Vec<Option<usize>> {
    let halting = embedded(lines, Some((interpolation, Some(escape))));
    let plain = embedded(lines, None);
    for ((halt, row), (_, whole)) in halting.iter().zip(&plain) {
        if halt.is_none() {
            assert_eq!(row, whole, "no halt, the same tokens");
        } else {
            let before = row.len().saturating_sub(1);
            assert_eq!(row[..before], whole[..before]);
        }
    }
    halting.into_iter().map(|(halt, _)| halt).collect()
}

/// The places Shiki hands a line of a Ruby here document named for HTML
/// back to Ruby: an interpolation or an escape while no HTML rule is open,
/// and neither inside a tag, across lines too, or a script.
#[test]
fn ruby_takes_an_html_heredoc_line_back_only_where_no_html_rule_is_open() {
    let rows = embedded(&["  x #{a} <b>y</b>"], Some((interpolation, Some(escape))));
    assert_eq!(rows[0].0, Some(4));
    assert_eq!(
        rows[0].1,
        [("  x ".to_owned(), RUBY_HEREDOC.map(str::to_owned).to_vec())]
    );
    assert_eq!(ruby_halts(&["  a \\n b < c"]), [Some(4)]);
    assert_eq!(ruby_halts(&["  <a title=\"#{x}\">t</a>"]), [None]);
    assert_eq!(
        ruby_halts(&["  <a href=\"x\"", "     data-y=\"#{y}\">z</a>"]),
        [None, None]
    );
    assert_eq!(ruby_halts(&["  <script>var a = #{x};</script>"]), [None]);
}

/// `end` is tried before the root's patterns at a place, so it takes the
/// line back at a `<` that starts a tag, and at the end of the text when
/// nothing matches after it.
#[test]
fn an_html_end_halt_beats_a_tag_at_its_place_and_holds_at_the_end() {
    fn at_end(bytes: &[u8], p: usize) -> bool {
        p == bytes.len()
    }
    assert_eq!(embedded(&["x <b>y"], Some((at_lt, None)))[0].0, Some(2));
    assert_eq!(
        embedded(&["x <b>y</b>"], Some((at_end, None)))[0].0,
        Some(10)
    );
}

/// `after` is tried only where no root pattern matches: not at a tag's `<`,
/// but at a bare one.
#[test]
fn an_html_after_halt_yields_to_a_root_match_at_its_place() {
    fn never(_: &[u8], _: usize) -> bool {
        false
    }
    assert_eq!(
        embedded(&["x <b>y</b> < z"], Some((never, Some(at_lt))))[0].0,
        Some(11)
    );
}
