use crate::test_support::{self, LeafRow, leaf_row, merge_by_leaf};
use crate::{Highlighter, Language, Limits, State};

/// Lexes `lines` in order from one start state.
fn lex_lines(lines: &[&str]) -> Vec<LeafRow> {
    merge_by_leaf(test_support::lex(Language::Xml, &lines.join("\n")))
}

fn lex(line: &str) -> LeafRow {
    lex_lines(&[line]).remove(0)
}

const TAG: &str = "punctuation.definition.tag.xml";
const META_TAG: &str = "meta.tag.xml";

#[test]
fn an_empty_line_leaves_the_state_alone() {
    let rows = lex_lines(&["<!-- a", "", "b"]);
    assert_eq!(rows[1], LeafRow::new());
    assert_eq!(rows[2], leaf_row(&[("b", "comment.block.xml")]));
}

#[test]
fn a_tag_has_a_name_attributes_and_strings() {
    assert_eq!(
        lex("<a:b c:d='e&amp;'/>"),
        leaf_row(&[
            ("<", TAG),
            ("a", "entity.name.tag.namespace.xml"),
            (":", "punctuation.separator.namespace.xml"),
            ("b", "entity.name.tag.localname.xml"),
            (" ", META_TAG),
            ("c", "entity.other.attribute-name.namespace.xml"),
            (":", "punctuation.separator.namespace.xml"),
            ("d", "entity.other.attribute-name.localname.xml"),
            ("=", META_TAG),
            ("'", "punctuation.definition.string.begin.xml"),
            ("e", "string.quoted.single.xml"),
            ("&", "punctuation.definition.constant.xml"),
            ("amp", "constant.character.entity.xml"),
            (";", "punctuation.definition.constant.xml"),
            ("'", "punctuation.definition.string.end.xml"),
            ("/>", TAG),
        ])
    );
}

#[test]
fn an_attribute_needs_white_space_before_it() {
    assert_eq!(
        lex("<a b=\"c\"d=\"e\">"),
        leaf_row(&[
            ("<", TAG),
            ("a", "entity.name.tag.localname.xml"),
            (" ", META_TAG),
            ("b", "entity.other.attribute-name.localname.xml"),
            ("=", META_TAG),
            ("\"", "punctuation.definition.string.begin.xml"),
            ("c", "string.quoted.double.xml"),
            ("\"", "punctuation.definition.string.end.xml"),
            ("d=", META_TAG),
            ("\"", "punctuation.definition.string.begin.xml"),
            ("e", "string.quoted.double.xml"),
            ("\"", "punctuation.definition.string.end.xml"),
            (">", TAG),
        ])
    );
}

/// A tag whose line holds its closing tag is one rule, and the closing name
/// must repeat the opening one.
#[test]
fn an_element_with_no_content_is_one_rule() {
    let no_content = "meta.tag.no-content.xml";
    assert_eq!(
        lex("<a b=\"c\"></a>"),
        leaf_row(&[
            ("<", TAG),
            ("a", "entity.name.tag.localname.xml"),
            (" ", no_content),
            ("b", "entity.other.attribute-name.localname.xml"),
            ("=", no_content),
            ("\"", "punctuation.definition.string.begin.xml"),
            ("c", "string.quoted.double.xml"),
            ("\"", "punctuation.definition.string.end.xml"),
            ("></", TAG),
            ("a", "entity.name.tag.localname.xml"),
            (">", TAG),
        ])
    );
    assert_eq!(lex("<a></b>")[0], ("<".to_owned(), TAG.to_owned()));
    assert_eq!(lex("<a></b>")[1].1, "entity.name.tag.localname.xml");
}

#[test]
fn a_dash_in_a_comment_opens_a_rule_that_never_ends() {
    let rows = lex_lines(&["<!-- a -- b -->", "<a>"]);
    assert_eq!(
        rows[0],
        leaf_row(&[
            ("<!--", "punctuation.definition.comment.xml"),
            (" a ", "comment.block.xml"),
            ("--", "invalid.illegal.bad-comments-or-CDATA.xml"),
            (" b -->", "comment.block.xml"),
        ])
    );
    assert_eq!(rows[1], leaf_row(&[("<a>", "comment.block.xml")]));
}

#[test]
fn a_jsp_comment_has_no_end_and_no_scope() {
    let rows = lex_lines(&["<%-- a --%>", "<a>"]);
    assert_eq!(
        rows[0],
        leaf_row(&[
            ("<%--", "punctuation.definition.comment.xml"),
            (" a --%>", "text.xml"),
        ])
    );
    assert_eq!(rows[1], leaf_row(&[("<a>", "text.xml")]));
}

#[test]
fn a_cdata_section_runs_across_lines() {
    let rows = lex_lines(&["<![CDATA[ <a> &", "]] ]]>x"]);
    let cdata = "string.unquoted.cdata.xml";
    assert_eq!(
        rows[0],
        leaf_row(&[
            ("<![CDATA[", "punctuation.definition.string.begin.xml"),
            (" <a> &", cdata),
        ])
    );
    assert_eq!(
        rows[1],
        leaf_row(&[
            ("]] ", cdata),
            ("]]>", "punctuation.definition.string.end.xml"),
            ("x", "text.xml"),
        ])
    );
}

#[test]
fn a_doctype_holds_an_internal_subset_across_lines() {
    let rows = lex_lines(&["<!DOCTYPE a [", "<!ENTITY % e SYSTEM \"u\"> %p;", "]> <b>"]);
    let doctype = "meta.tag.sgml.doctype.xml";
    assert_eq!(
        rows[0],
        leaf_row(&[
            ("<!", TAG),
            ("DOCTYPE", "keyword.other.doctype.xml"),
            (" ", doctype),
            ("a", "variable.language.documentroot.xml"),
            (" ", doctype),
            ("[", "punctuation.definition.constant.xml"),
        ])
    );
    let subset = "meta.internalsubset.xml";
    assert_eq!(
        rows[1],
        leaf_row(&[
            ("<!", TAG),
            ("ENTITY", "keyword.other.entity.xml"),
            (" ", subset),
            ("% ", "punctuation.definition.entity.xml"),
            ("e", "variable.language.entity.xml"),
            (" SYSTEM ", "keyword.other.entitytype.xml"),
            ("\"", "punctuation.definition.string.begin.xml"),
            ("u", "string.quoted.double.xml"),
            ("\"", "punctuation.definition.string.end.xml"),
            (">", TAG),
            (" ", subset),
            ("%", "punctuation.definition.constant.xml"),
            ("p", "constant.character.parameter-entity.xml"),
            (";", "punctuation.definition.constant.xml"),
        ])
    );
    assert_eq!(
        rows[2],
        leaf_row(&[
            ("]", "punctuation.definition.constant.xml"),
            (">", TAG),
            (" ", "text.xml"),
            ("<", TAG),
            ("b", "entity.name.tag.localname.xml"),
            (">", TAG),
        ])
    );
}

#[test]
fn a_string_runs_across_lines() {
    let rows = lex_lines(&["<a b=\"c", "d\" e='f'>"]);
    assert_eq!(
        rows[1],
        leaf_row(&[
            ("d", "string.quoted.double.xml"),
            ("\"", "punctuation.definition.string.end.xml"),
            (" ", META_TAG),
            ("e", "entity.other.attribute-name.localname.xml"),
            ("=", META_TAG),
            ("'", "punctuation.definition.string.begin.xml"),
            ("f", "string.quoted.single.xml"),
            ("'", "punctuation.definition.string.end.xml"),
            (">", TAG),
        ])
    );
}

#[test]
fn an_ampersand_that_starts_no_entity_is_illegal() {
    assert_eq!(
        lex("a & &b &#x1G; &#9;"),
        leaf_row(&[
            ("a ", "text.xml"),
            ("&", "invalid.illegal.bad-ampersand.xml"),
            (" ", "text.xml"),
            ("&", "invalid.illegal.bad-ampersand.xml"),
            ("b ", "text.xml"),
            ("&", "invalid.illegal.bad-ampersand.xml"),
            ("#x1G; ", "text.xml"),
            ("&", "punctuation.definition.constant.xml"),
            ("#9", "constant.character.entity.xml"),
            (";", "punctuation.definition.constant.xml"),
        ])
    );
}

#[test]
fn a_unicode_space_separates_an_attribute() {
    let row = lex("<a\u{a0}b='c'>");
    assert!(
        row.iter().any(
            |(text, scope)| text == "b" && scope == "entity.other.attribute-name.localname.xml"
        ),
        "{row:?}"
    );
}

#[test]
fn a_processing_instruction_reads_names_after_a_space() {
    let pi = "meta.tag.preprocessor.xml";
    assert_eq!(
        lex("<?xml version='1' x-y?>"),
        leaf_row(&[
            ("<?", TAG),
            ("xml", "entity.name.tag.xml"),
            (" version", "entity.other.attribute-name.xml"),
            ("=", pi),
            ("'", "punctuation.definition.string.begin.xml"),
            ("1", "string.quoted.single.xml"),
            ("'", "punctuation.definition.string.end.xml"),
            (" x-y", "entity.other.attribute-name.xml"),
            ("?>", TAG),
        ])
    );
}

/// While the Java of the first line has a block open, its `%>` belongs to
/// Java and so does the text after it.
#[test]
fn java_in_a_page_runs_up_to_a_closing_mark_that_java_is_not_inside() {
    let rows = lex_lines(&["<% if (a) { %> b", "} %> c"]);
    let [first, second] = &rows[..] else {
        panic!("two rows, not {rows:?}");
    };
    assert!(
        first.iter().all(|(_, scope)| scope != "text.xml"),
        "{first:?}"
    );
    assert_eq!(
        second.last(),
        Some(&(" c".to_owned(), "text.xml".to_owned()))
    );
    assert!(
        second
            .iter()
            .any(|(text, scope)| text == "%>" && scope == "punctuation.section.embedded.end.xml"),
        "{second:?}"
    );
    assert!(
        second
            .iter()
            .any(|(_, scope)| scope.rsplit('.').next() == Some("java"))
    );
}

#[test]
fn a_closing_mark_inside_a_java_string_or_line_comment_belongs_to_java() {
    for (lines, want) in [
        (&["<% String s = \"%>\"; %> x"][..], 1),
        (&["<% // %> x", "y"][..], 0),
        (&["<% if (a) { %>", "// %> x <% } %> y"][..], 0),
    ] {
        let ends: usize = lex_lines(lines)
            .iter()
            .flatten()
            .filter(|(_, scope)| scope == "punctuation.section.embedded.end.xml")
            .count();
        assert_eq!(ends, want, "{lines:?}");
    }
}

#[test]
fn a_page_directive_names_its_properties() {
    assert_eq!(
        lex("<%@ page lang %>"),
        leaf_row(&[
            ("<%@", "punctuation.section.embedded.begin.xml"),
            (" ", "source.java-props.embedded.xml"),
            ("page", "keyword.other.page-props.xml"),
            (" lang ", "source.java-props.embedded.xml"),
            ("%>", "punctuation.section.embedded.end.xml"),
        ])
    );
}

#[test]
fn a_context_deeper_than_the_depth_limit_is_not_opened() {
    let limits = Limits {
        depth: Some(1),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Xml, limits);
    let mut tokens = Vec::new();
    let mut state = State::default();
    let report = highlighter.line("<a b='c'>", &mut state, &mut tokens);
    assert!(report.reached(crate::Limit::Depth));
    assert_eq!(tokens.last().map(|token| token.end()), Some(9));
}

#[test]
fn java_does_not_read_into_the_middle_of_a_character_after_its_end() {
    let line = format!("<% a %>{}", "\u{e9}".repeat(100));
    let row = lex(&line);
    assert_eq!(
        row.last().map(|(_, scope)| scope.as_str()),
        Some("text.xml")
    );
}
