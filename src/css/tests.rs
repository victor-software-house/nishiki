use super::matchers::{LONGEST_UNIT, UNITS};
use crate::Language;
use crate::test_support::{self, LeafRow, leaf_in, merge_by_leaf};

/// Lexes `lines` in order from one start state.
fn lex_lines(lines: &[&str]) -> Vec<LeafRow> {
    merge_by_leaf(test_support::lex(Language::Css, &lines.join("\n")))
}

fn lex(line: &str) -> LeafRow {
    lex_lines(&[line]).remove(0)
}

#[test]
fn an_empty_line_has_no_tokens() {
    assert_eq!(lex(""), LeafRow::new());
}

#[test]
fn a_rule_names_its_selector_property_and_value() {
    let row = lex("a:hover { color: red; }");
    assert_eq!(leaf_in(&row, "a"), "entity.name.tag.css");
    assert_eq!(
        leaf_in(&row, "hover"),
        "entity.other.attribute-name.pseudo-class.css"
    );
    assert_eq!(leaf_in(&row, "color"), "support.type.property-name.css");
    assert_eq!(
        leaf_in(&row, "red"),
        "support.constant.color.w3c-standard-color-name.css"
    );
    assert_eq!(leaf_in(&row, ";"), "punctuation.terminator.rule.css");
}

#[test]
fn a_number_and_its_unit_are_two_scopes() {
    let row = lex("a{margin:1.5em 50% -.5}");
    assert_eq!(leaf_in(&row, "1.5"), "constant.numeric.css");
    assert_eq!(leaf_in(&row, "em"), "keyword.other.unit.em.css");
    assert_eq!(leaf_in(&row, "%"), "keyword.other.unit.percentage.css");
    assert_eq!(leaf_in(&row, "-.5"), "constant.numeric.css");
}

#[test]
fn important_and_hex_colours_are_values() {
    let row = lex("a{color:#fff!important}");
    assert_eq!(
        leaf_in(&row, "fff"),
        "constant.other.color.rgb-value.hex.css"
    );
    assert_eq!(leaf_in(&row, "!important"), "keyword.other.important.css");
}

#[test]
fn a_comment_runs_over_lines() {
    let rows = lex_lines(&["a { /* one", "two */ color: red }"]);
    assert_eq!(leaf_in(&rows[0], " one"), "comment.block.css");
    assert_eq!(leaf_in(&rows[1], "two "), "comment.block.css");
    assert_eq!(
        leaf_in(&rows[1], "*/"),
        "punctuation.definition.comment.end.css"
    );
    assert_eq!(leaf_in(&rows[1], "color"), "support.type.property-name.css");
}

#[test]
fn a_media_query_has_a_header_and_a_body() {
    let rows = lex_lines(&["@media (min-width: 10px) {", "  a { top: 0 }", "}"]);
    assert_eq!(
        leaf_in(&rows[0], "min-width"),
        "support.type.property-name.media.css"
    );
    assert_eq!(leaf_in(&rows[1], "a"), "entity.name.tag.css");
    assert_eq!(
        leaf_in(&rows[2], "}"),
        "punctuation.section.media.end.bracket.curly.css"
    );
}

#[test]
fn a_rule_list_stays_open_over_lines() {
    let rows = lex_lines(&["a {", "  color: red;", "  margin: 0", "}"]);
    assert_eq!(leaf_in(&rows[1], "color"), "support.type.property-name.css");
    assert_eq!(
        leaf_in(&rows[2], "margin"),
        "support.type.property-name.css"
    );
    assert_eq!(
        leaf_in(&rows[3], "}"),
        "punctuation.section.property-list.end.bracket.curly.css"
    );
}

#[test]
fn a_backslash_at_the_end_of_a_line_holds_the_next_line() {
    let rows = lex_lines(&["a {", "  color: red \\", "x", "}"]);
    assert_eq!(
        leaf_in(&rows[2], "x"),
        "constant.character.escape.newline.css"
    );
    // The rule keeps `\G` at the start of every line that follows, so its
    // end `^(?<!\G)` never matches again.
    assert_eq!(
        leaf_in(&rows[3], "}"),
        "constant.character.escape.newline.css"
    );
}

#[test]
fn charset_opens_a_rule_that_wants_double_quotes() {
    let row = lex("@charset \"UTF-8\";");
    assert_eq!(
        leaf_in(&row, "charset"),
        "keyword.control.at-rule.charset.css"
    );
    assert_eq!(leaf_in(&row, "UTF-8"), "string.quoted.double.css");
    let row = lex("@charset 'UTF-8';");
    assert_eq!(
        leaf_in(&row, "'UTF-8'"),
        "invalid.illegal.not-double-quoted.charset.css"
    );
}

#[test]
fn an_unclosed_string_is_illegal_to_the_end_of_the_line() {
    let row = lex("a { content: \"x");
    assert_eq!(leaf_in(&row, "x"), "invalid.illegal.unclosed.string.css");
}

#[test]
fn url_takes_a_bare_argument_or_a_string() {
    let row = lex("a { b: url(x.png) url(\"y.png\") }");
    assert_eq!(leaf_in(&row, "x.png"), "variable.parameter.url.css");
    assert_eq!(leaf_in(&row, "y.png"), "string.quoted.double.css");
}

#[test]
fn a_selector_can_be_a_class_followed_by_a_space_that_is_not_ascii() {
    // The class gives back the no-break space, which is also an identifier
    // character, so that a stop can follow it.
    let row = lex("e.t\u{a0}*");
    assert_eq!(leaf_in(&row, "t"), "entity.other.attribute-name.class.css");
    assert_eq!(leaf_in(&row, "*"), "entity.name.tag.wildcard.css");
}

#[test]
fn the_long_s_is_an_s_where_the_grammar_ignores_case() {
    let row = lex("\u{17f}{}");
    assert_eq!(leaf_in(&row, "\u{17f}"), "entity.name.tag.css");
}

/// `Onig` folds `ß` and `ẞ` to `ss`, and each Latin ligature to its letters,
/// so under `(?i)` one character matches two or three letters of a word of
/// the grammar's lists. Each case is Shiki's answer on the line.
#[test]
fn a_multi_letter_case_fold_stands_for_its_letters() {
    for (line, token, scope) in [
        ("\u{fb01}eldset {}", "\u{fb01}eldset", "entity.name.tag.css"),
        ("PROGRE\u{1e9e} {}", "PROGRE\u{1e9e}", "entity.name.tag.css"),
        (
            "a:ho\u{fb06} {}",
            "ho\u{fb06}",
            "entity.other.attribute-name.pseudo-class.css",
        ),
        (
            "a:nth-la\u{fb06}-child(2) {}",
            "nth-la\u{fb06}-child",
            "entity.other.attribute-name.pseudo-class.css",
        ),
        (
            "a::\u{fb01}rst-letter {}",
            "\u{fb01}rst-letter",
            "entity.other.attribute-name.pseudo-element.css",
        ),
        (
            "a{color:corn\u{fb02}owerblue}",
            "corn\u{fb02}owerblue",
            "support.constant.color.w3c-extended-color-name.css",
        ),
        (
            "a{su\u{fb03}x:1}",
            "su\u{fb03}x",
            "support.type.property-name.css",
        ),
        (
            "a{b:di\u{fb00}erence}",
            "di\u{fb00}erence",
            "support.constant.property-value.css",
        ),
        (
            "a{b:\u{fb05}atic}",
            "\u{fb05}atic",
            "support.constant.property-value.css",
        ),
        (
            "a{font-family:sy\u{fb06}em-ui}",
            "sy\u{fb06}em-ui",
            "support.constant.font-name.css",
        ),
        (
            "@media embo\u{df}ed {}",
            "embo\u{df}ed",
            "invalid.deprecated.constant.media.css",
        ),
        (
            "a{b:\u{fb06}eps(1)}",
            "\u{fb06}eps",
            "support.function.timing-function.css",
        ),
        (
            "@counter-\u{fb06}yle x {}",
            "counter-\u{fb06}yle",
            "keyword.control.at-rule.counter-style.css",
        ),
    ] {
        assert_eq!(leaf_in(&lex(line), token), scope, "{line:?}");
    }
    for (line, token) in [
        ("a{b:di\u{fb03}erence}", "di\u{fb03}erence"),
        ("a{b:foo\u{fb01}}", "foo\u{fb01}"),
    ] {
        assert_eq!(
            leaf_in(&lex(line), token),
            "meta.property-value.css",
            "{line:?}: the fold's letters must all match"
        );
    }
}

#[test]
fn a_custom_property_name_is_a_variable() {
    let row = lex("a { --x: 1px }");
    assert_eq!(leaf_in(&row, "--x"), "variable.css");
}

#[test]
fn charset_is_the_charset_rule_on_the_first_line_only() {
    let rows = lex_lines(&["@charset \"a\";", "@charset \"b\";"]);
    assert_eq!(
        leaf_in(&rows[0], "charset"),
        "keyword.control.at-rule.charset.css"
    );
    assert_eq!(leaf_in(&rows[1], "charset"), "keyword.control.at-rule.css");
}

/// The grammar ignores case in the lookahead that starts a selector at a name
/// that could be a property, such as `mark`, `font` or `mask`, so an
/// upper-case `Mark` that nothing on its line ends as a declaration starts
/// one too.
#[test]
fn a_shared_name_in_upper_case_starts_a_selector() {
    for name in ["mark", "Mark", "MARK", "Font", "Mask"] {
        let rows = lex_lines(&["a {", &format!("  {name} it"), "}"]);
        assert_eq!(leaf_in(&rows[1], name), "entity.name.tag.css", "{name}");
    }
}

/// The unit table is sorted for its binary search, and `LONGEST_UNIT` is the
/// length of its longest unit, which bounds the lowering buffer.
#[test]
fn the_unit_table_is_sorted_and_bounds_its_longest_unit() {
    assert!(UNITS.is_sorted(), "{UNITS:?}");
    assert_eq!(
        UNITS.iter().map(|unit| unit.len()).max(),
        Some(LONGEST_UNIT)
    );
}

/// Every unit of the table is one the grammar's patterns name, so a
/// number followed by it is named `keyword.other.unit.<unit>.css`.
#[test]
fn every_unit_of_the_table_is_named() {
    for unit in UNITS {
        let line = format!("a{{b:1{unit}}}");
        assert_eq!(
            leaf_in(&lex(&line), unit),
            format!("keyword.other.unit.{unit}.css"),
            "{line:?}"
        );
    }
}
