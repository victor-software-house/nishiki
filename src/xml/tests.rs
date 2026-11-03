use crate::{Highlighter, Language, Limits, State};

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
        &mut Highlighter::new(Language::Xml),
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

const TAG: &str = "punctuation.definition.tag.xml";
const META_TAG: &str = "meta.tag.xml";

#[test]
fn an_empty_line_leaves_the_state_alone() {
    let mut highlighter = Highlighter::new(Language::Xml);
    let mut state = State::default();
    lex_in(&mut highlighter, &mut state, "<!-- a");
    assert_eq!(lex_in(&mut highlighter, &mut state, ""), Row::new());
    assert_eq!(
        lex_in(&mut highlighter, &mut state, "b"),
        row(&[("b", "comment.block.xml")])
    );
}

#[test]
fn a_tag_has_a_name_attributes_and_strings() {
    assert_eq!(
        lex("<a:b c:d='e&amp;'/>"),
        row(&[
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
        row(&[
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
        row(&[
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
    let mut highlighter = Highlighter::new(Language::Xml);
    let mut state = State::default();
    assert_eq!(
        lex_in(&mut highlighter, &mut state, "<!-- a -- b -->"),
        row(&[
            ("<!--", "punctuation.definition.comment.xml"),
            (" a ", "comment.block.xml"),
            ("--", "invalid.illegal.bad-comments-or-CDATA.xml"),
            (" b -->", "comment.block.xml"),
        ])
    );
    assert_eq!(
        lex_in(&mut highlighter, &mut state, "<a>"),
        row(&[("<a>", "comment.block.xml")])
    );
}

#[test]
fn a_jsp_comment_has_no_end_and_no_scope() {
    let mut highlighter = Highlighter::new(Language::Xml);
    let mut state = State::default();
    assert_eq!(
        lex_in(&mut highlighter, &mut state, "<%-- a --%>"),
        row(&[
            ("<%--", "punctuation.definition.comment.xml"),
            (" a --%>", "text.xml"),
        ])
    );
    assert_eq!(
        lex_in(&mut highlighter, &mut state, "<a>"),
        row(&[("<a>", "text.xml")])
    );
}

#[test]
fn a_cdata_section_runs_across_lines() {
    let mut highlighter = Highlighter::new(Language::Xml);
    let mut state = State::default();
    let cdata = "string.unquoted.cdata.xml";
    assert_eq!(
        lex_in(&mut highlighter, &mut state, "<![CDATA[ <a> &"),
        row(&[
            ("<![CDATA[", "punctuation.definition.string.begin.xml"),
            (" <a> &", cdata),
        ])
    );
    assert_eq!(
        lex_in(&mut highlighter, &mut state, "]] ]]>x"),
        row(&[
            ("]] ", cdata),
            ("]]>", "punctuation.definition.string.end.xml"),
            ("x", "text.xml"),
        ])
    );
}

#[test]
fn a_doctype_holds_an_internal_subset_across_lines() {
    let mut highlighter = Highlighter::new(Language::Xml);
    let mut state = State::default();
    let doctype = "meta.tag.sgml.doctype.xml";
    assert_eq!(
        lex_in(&mut highlighter, &mut state, "<!DOCTYPE a ["),
        row(&[
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
        lex_in(
            &mut highlighter,
            &mut state,
            "<!ENTITY % e SYSTEM \"u\"> %p;"
        ),
        row(&[
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
        lex_in(&mut highlighter, &mut state, "]> <b>"),
        row(&[
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
    let mut highlighter = Highlighter::new(Language::Xml);
    let mut state = State::default();
    lex_in(&mut highlighter, &mut state, "<a b=\"c");
    assert_eq!(
        lex_in(&mut highlighter, &mut state, "d\" e='f'>"),
        row(&[
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
        row(&[
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
        row(&[
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
    let mut highlighter = Highlighter::new(Language::Xml);
    let mut state = State::default();
    let first = lex_in(&mut highlighter, &mut state, "<% if (a) { %> b");
    assert!(
        first.iter().all(|(_, scope)| scope != "text.xml"),
        "{first:?}"
    );
    let second = lex_in(&mut highlighter, &mut state, "} %> c");
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
        let mut highlighter = Highlighter::new(Language::Xml);
        let mut state = State::default();
        let mut rows = Vec::new();
        for line in lines {
            rows.push(lex_in(&mut highlighter, &mut state, line));
        }
        let ends: usize = rows
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
        row(&[
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
