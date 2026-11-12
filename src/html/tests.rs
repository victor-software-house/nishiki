use indoc::indoc;

use crate::language::Halt;
use crate::test_support::rows::lex;
use crate::test_support::scopes::{leaf, scopes_of};
use crate::{Language, test_support};

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
/// blank, a quote, `<`, `=`, `>`, `/` or a control.
/// U+00A0 therefore starts one.
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
/// again.
/// The lexer sees the quote before it, so `<` after the quote and blanks is no
/// tag.
/// No line feed follows the group's end.
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
/// after it.
/// An event handler's code ends before its line's line feed.
/// So an `=` at the end of a quoted value's line is an illegal
/// attribute.
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

/// HTML as another language embeds it.
const EMBEDDED: test_support::embedded::Embedded = test_support::embedded::Embedded {
    start: super::start_embedded,
    lex: super::lex,
    lex_until: super::lex_until,
};

/// The scopes a Ruby here document named for HTML puts under its lines, as
/// Shiki gives them: the grammar's `text.html.basic` adds none of its own.
const RUBY_HEREDOC: [&str; 3] = ["source.ruby", "meta.embedded.block.html", "text.html"];

/// Where each line halts under a Ruby here document's halts.
fn ruby_halts(lines: &[&str]) -> Vec<Option<usize>> {
    test_support::embedded::ruby_heredoc_halts(EMBEDDED, &RUBY_HEREDOC, lines)
}

/// Where each line halts under `end` and `after`.
fn halts(lines: &[&str], end: &dyn Halt, after: Option<&dyn Halt>) -> Vec<Option<usize>> {
    test_support::embedded::lex_embedded(EMBEDDED, &RUBY_HEREDOC, lines, Some((end, after)))
        .into_iter()
        .map(|(halt, _)| halt)
        .collect()
}

/// A `<`, for the order of the halts and the patterns at a tag.
fn at_lt(bytes: &[u8], p: usize) -> bool {
    bytes.get(p) == Some(&b'<')
}

/// The places Shiki hands a line of a Ruby here document named for HTML
/// back to Ruby.
/// They are an interpolation or an escape while no HTML rule is open.
/// Neither counts inside a tag, across lines too, or inside a script.
#[test]
fn ruby_takes_an_html_heredoc_line_back_only_where_no_html_rule_is_open() {
    let rows = test_support::embedded::lex_embedded(
        EMBEDDED,
        &RUBY_HEREDOC,
        &["  x #{a} <b>y</b>"],
        Some(test_support::embedded::RUBY_HALTS),
    );
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

/// `end` is tried before the root's patterns at a place.
/// It therefore takes the line back at a `<` that starts a tag.
/// It also takes it back at the end of the text when nothing matches after
/// it.
#[test]
fn an_html_end_halt_beats_a_tag_at_its_place_and_holds_at_the_end() {
    fn at_end(bytes: &[u8], p: usize) -> bool {
        p == bytes.len()
    }
    assert_eq!(halts(&["x <b>y"], &at_lt, None), [Some(2)]);
    assert_eq!(halts(&["x <b>y</b>"], &at_end, None), [Some(10)]);
}

/// `after` is tried only where no root pattern matches: not at a tag's `<`,
/// but at a bare one.
#[test]
fn an_html_after_halt_yields_to_a_root_match_at_its_place() {
    fn never(_: &[u8], _: usize) -> bool {
        false
    }
    assert_eq!(halts(&["x <b>y</b> < z"], &never, Some(&at_lt)), [Some(11)]);
}

/// Asserts each case's scopes below the root, Shiki's answer for its token
/// on its line.
///
/// `Onig` folds `ß` to `ss`, each Latin ligature to its letters and `ſ` to
/// `s` under `(?i)`.
/// The cases that use it read a character that folds to letters as those
/// letters.
/// The `c` of `ſcript` is written `\u{63}`, so the spelling check passes it.
fn assert_html_scopes(cases: &[(&str, &str, &[&str])]) {
    for &(text, token, scopes) in cases {
        assert_eq!(
            scopes_of(Language::Html, text, token)[1..],
            scopes[..],
            "{text:?}"
        );
    }
}

/// A tag name of the grammar's lists, an `svg` root and an element inside
/// one read a fold as its letters.
#[test]
fn a_case_fold_stands_for_its_letters_in_tag_names() {
    assert_html_scopes(&[
        (
            "<\u{fb01}eldset>x</fieldset>",
            "\u{fb01}eldset",
            &[
                "meta.tag.structure.\u{fb01}eldset.start.html",
                "entity.name.tag.html",
            ],
        ),
        (
            "<addre\u{df}>",
            "addre\u{df}",
            &[
                "meta.tag.structure.addre\u{df}.start.html",
                "entity.name.tag.html",
            ],
        ),
        (
            "<\u{fb06}rong>x</strong>",
            "\u{fb06}rong",
            &[
                "meta.tag.inline.\u{fb06}rong.start.html",
                "entity.name.tag.html",
            ],
        ),
        (
            "<\u{17f}vg><g>x</g></\u{17f}vg>",
            "\u{17f}vg",
            &[
                "meta.element.structure.\u{17f}vg.html",
                "meta.tag.structure.\u{17f}vg.start.html",
                "entity.name.tag.html",
            ],
        ),
    ]);
}

/// An end tag reads a fold as its letters: `</style` and `</script` after
/// the ASCII `</`, and the back-reference that closes an SVG element.
/// A character there that folds to no letters, as `ʀ`, closes nothing.
#[test]
fn a_case_fold_stands_for_its_letters_in_end_tags() {
    assert_html_scopes(&[
        (
            "<style>a{}</\u{fb06}yle>",
            "\u{fb06}yle",
            &[
                "meta.embedded.block.html",
                "meta.tag.metadata.style.end.html",
                "entity.name.tag.html",
            ],
        ),
        (
            "<script>x</\u{17f}\u{63}ript>",
            "\u{17f}\u{63}ript",
            &[
                "meta.embedded.block.html",
                "meta.tag.metadata.script.end.html",
                "entity.name.tag.html",
            ],
        ),
        (
            "<script>x</sc\u{280}ipt>y",
            "y",
            &[
                "meta.embedded.block.html",
                "source.js",
                "variable.other.readwrite.js",
            ],
        ),
        (
            "<svg><\u{fb01}lter>x</filter></svg>",
            "filter",
            &[
                "meta.element.structure.svg.html",
                "meta.element.structure.svg.\u{fb01}lter.html",
                "meta.tag.structure.svg.filter.end.html",
                "entity.name.tag.html",
            ],
        ),
        (
            "<svg><\u{fb01}lter>x</FILTER>y</svg>",
            "y",
            &["meta.element.structure.svg.html"],
        ),
        ("<\u{17f}vg><g>x</g></\u{17f}vg>y", "y", &[]),
    ]);
}

/// The `style` and `script` wrappers open at each fold that can start
/// their names: `ſ`, `ﬆ` and `ﬅ`.
/// The fold stands at the start of the name, where the root looks for a
/// wrapper.
#[test]
fn a_case_fold_stands_for_its_letters_in_wrapper_names() {
    assert_html_scopes(&[
        (
            "<\u{fb06}yle>a{}</style>",
            "a",
            &["meta.embedded.block.html", "source.css"],
        ),
        (
            "<\u{fb05}yle>a{}</style>",
            "a",
            &["meta.embedded.block.html", "source.css"],
        ),
        (
            "<\u{17f}tyle>a{}</style>",
            "a",
            &["meta.embedded.block.html", "source.css"],
        ),
        (
            "<\u{17f}\u{63}ript>x</script>",
            "x",
            &[
                "meta.embedded.block.html",
                "source.js",
                "variable.other.readwrite.js",
            ],
        ),
    ]);
}

/// A script's `type` value reads a fold after an ASCII prefix, as the `ſ`
/// after `text/java`, from the byte where the ASCII comparison stopped.
#[test]
fn a_case_fold_stands_for_its_letters_in_attribute_literals() {
    assert_html_scopes(&[(
        "<script type=\"text/java\u{17f}\u{63}ript\">x</script>",
        "x",
        &[
            "meta.embedded.block.html",
            "source.js",
            "variable.other.readwrite.js",
        ],
    )]);
}

/// A Markdown grammar loaded at run time embeds HTML in Shiki's Markdown
/// order, with or without nishiki's own Markdown lexer.
/// Its document scope is one the arena made, so only its name matches.
#[cfg(feature = "_textmate")]
#[test]
fn html_in_a_loaded_markdown_grammar_reads_derivative_first() {
    let mut arena = crate::arena::Arena::new(None);
    let markdown = arena.named_scope("text.html.markdown").unwrap();
    let base = arena.child(crate::arena::ROOT, markdown).unwrap();
    let state = super::start_embedded(&mut arena, base);
    assert_eq!(
        arena.frame(state).carried,
        super::state::ROOT_DERIVATIVE_FIRST
    );
}

/// HTML in a Svelte grammar loaded at run time reads Svelte's order and
/// Svelte's `$` prefixes, with or without nishiki's own Svelte lexer.
#[cfg(feature = "_textmate")]
#[test]
fn html_in_a_loaded_svelte_grammar_reads_svelte() {
    let mut arena = crate::arena::Arena::new(None);
    let svelte = arena.named_scope("source.svelte").unwrap();
    let base = arena.child(crate::arena::ROOT, svelte).unwrap();
    let state = super::start_embedded(&mut arena, base);
    assert_eq!(arena.frame(state).carried, super::state::ROOT_SVELTE);
}

/// The names in `DERIVATIVE_FIRST` are the document scopes of nishiki's own
/// Markdown and Svelte lexers.
#[cfg(all(feature = "lang-markdown", feature = "lang-svelte"))]
#[test]
fn the_derivative_first_names_are_the_lexers_documents() {
    assert_eq!(
        super::DERIVATIVE_FIRST,
        [
            crate::markdown::DOCUMENT.name(),
            crate::svelte::DOCUMENT.name()
        ]
    );
}
