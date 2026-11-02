use crate::{Highlighter, Language, State};

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
        &mut Highlighter::new(Language::Css),
        &mut State::default(),
        line,
    )
}

#[test]
fn an_empty_line_has_no_tokens() {
    assert_eq!(lex(""), Row::new());
}

/// The innermost scope of the token that is exactly `text`.
fn scope_of(row: &Row, text: &str) -> String {
    row.iter()
        .find(|(token, _)| token == text)
        .unwrap_or_else(|| panic!("no token {text:?} in {row:?}"))
        .1
        .clone()
}

/// Lexes `lines` in order from one start state.
fn lex_lines(lines: &[&str]) -> Vec<Row> {
    let mut highlighter = Highlighter::new(Language::Css);
    let mut state = State::default();
    lines
        .iter()
        .map(|line| lex_in(&mut highlighter, &mut state, line))
        .collect()
}

#[test]
fn a_rule_names_its_selector_property_and_value() {
    let row = lex("a:hover { color: red; }");
    assert_eq!(scope_of(&row, "a"), "entity.name.tag.css");
    assert_eq!(
        scope_of(&row, "hover"),
        "entity.other.attribute-name.pseudo-class.css"
    );
    assert_eq!(scope_of(&row, "color"), "support.type.property-name.css");
    assert_eq!(
        scope_of(&row, "red"),
        "support.constant.color.w3c-standard-color-name.css"
    );
    assert_eq!(scope_of(&row, ";"), "punctuation.terminator.rule.css");
}

#[test]
fn a_number_and_its_unit_are_two_scopes() {
    let row = lex("a{margin:1.5em 50% -.5}");
    assert_eq!(scope_of(&row, "1.5"), "constant.numeric.css");
    assert_eq!(scope_of(&row, "em"), "keyword.other.unit.em.css");
    assert_eq!(scope_of(&row, "%"), "keyword.other.unit.percentage.css");
    assert_eq!(scope_of(&row, "-.5"), "constant.numeric.css");
}

#[test]
fn important_and_hex_colours_are_values() {
    let row = lex("a{color:#fff!important}");
    assert_eq!(
        scope_of(&row, "fff"),
        "constant.other.color.rgb-value.hex.css"
    );
    assert_eq!(scope_of(&row, "!important"), "keyword.other.important.css");
}

#[test]
fn a_comment_runs_over_lines() {
    let rows = lex_lines(&["a { /* one", "two */ color: red }"]);
    assert_eq!(scope_of(&rows[0], " one"), "comment.block.css");
    assert_eq!(scope_of(&rows[1], "two "), "comment.block.css");
    assert_eq!(
        scope_of(&rows[1], "*/"),
        "punctuation.definition.comment.end.css"
    );
    assert_eq!(
        scope_of(&rows[1], "color"),
        "support.type.property-name.css"
    );
}

#[test]
fn a_media_query_has_a_header_and_a_body() {
    let rows = lex_lines(&["@media (min-width: 10px) {", "  a { top: 0 }", "}"]);
    assert_eq!(
        scope_of(&rows[0], "min-width"),
        "support.type.property-name.media.css"
    );
    assert_eq!(scope_of(&rows[1], "a"), "entity.name.tag.css");
    assert_eq!(
        scope_of(&rows[2], "}"),
        "punctuation.section.media.end.bracket.curly.css"
    );
}

#[test]
fn a_rule_list_stays_open_over_lines() {
    let rows = lex_lines(&["a {", "  color: red;", "  margin: 0", "}"]);
    assert_eq!(
        scope_of(&rows[1], "color"),
        "support.type.property-name.css"
    );
    assert_eq!(
        scope_of(&rows[2], "margin"),
        "support.type.property-name.css"
    );
    assert_eq!(
        scope_of(&rows[3], "}"),
        "punctuation.section.property-list.end.bracket.curly.css"
    );
}

#[test]
fn a_backslash_at_the_end_of_a_line_holds_the_next_line() {
    let rows = lex_lines(&["a {", "  color: red \\", "x", "}"]);
    assert_eq!(
        scope_of(&rows[2], "x"),
        "constant.character.escape.newline.css"
    );
    // The rule keeps `\G` at the start of every line that follows, so its
    // end `^(?<!\G)` never matches again.
    assert_eq!(
        scope_of(&rows[3], "}"),
        "constant.character.escape.newline.css"
    );
}

#[test]
fn charset_opens_a_rule_that_wants_double_quotes() {
    let row = lex("@charset \"UTF-8\";");
    assert_eq!(
        scope_of(&row, "charset"),
        "keyword.control.at-rule.charset.css"
    );
    assert_eq!(scope_of(&row, "UTF-8"), "string.quoted.double.css");
    let row = lex("@charset 'UTF-8';");
    assert_eq!(
        scope_of(&row, "'UTF-8'"),
        "invalid.illegal.not-double-quoted.charset.css"
    );
}

#[test]
fn an_unclosed_string_is_illegal_to_the_end_of_the_line() {
    let row = lex("a { content: \"x");
    assert_eq!(scope_of(&row, "x"), "invalid.illegal.unclosed.string.css");
}

#[test]
fn url_takes_a_bare_argument_or_a_string() {
    let row = lex("a { b: url(x.png) url(\"y.png\") }");
    assert_eq!(scope_of(&row, "x.png"), "variable.parameter.url.css");
    assert_eq!(scope_of(&row, "y.png"), "string.quoted.double.css");
}

#[test]
fn a_selector_can_be_a_class_followed_by_a_space_that_is_not_ascii() {
    // The class gives back the no-break space, which is also an identifier
    // character, so that a stop can follow it.
    let row = lex("e.t\u{a0}*");
    assert_eq!(scope_of(&row, "t"), "entity.other.attribute-name.class.css");
    assert_eq!(scope_of(&row, "*"), "entity.name.tag.wildcard.css");
}

#[test]
fn the_long_s_is_an_s_where_the_grammar_ignores_case() {
    let row = lex("\u{17f}{}");
    assert_eq!(scope_of(&row, "\u{17f}"), "entity.name.tag.css");
}

#[test]
fn a_custom_property_name_is_a_variable() {
    let row = lex("a { --x: 1px }");
    assert_eq!(scope_of(&row, "--x"), "variable.css");
}

#[test]
fn charset_is_the_charset_rule_on_the_first_line_only() {
    let rows = lex_lines(&["@charset \"a\";", "@charset \"b\";"]);
    assert_eq!(
        scope_of(&rows[0], "charset"),
        "keyword.control.at-rule.charset.css"
    );
    assert_eq!(scope_of(&rows[1], "charset"), "keyword.control.at-rule.css");
}
