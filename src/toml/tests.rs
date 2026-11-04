use indoc::indoc;

use crate::{Highlighter, Language, Limits, State};

/// One lexed line: each token's text and its scope names, outermost first.
type Row = Vec<(String, Vec<String>)>;

fn lex(text: &str) -> Vec<Row> {
    let mut highlighter = Highlighter::new(Language::Toml);
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

/// The innermost scope of the last token of the line `row` of `text`.
fn last_leaf(text: &str, row: usize) -> String {
    lex(text)[row]
        .last()
        .and_then(|(_, scopes)| scopes.last().cloned())
        .unwrap_or_default()
}

const INVALID: &str = "invalid.illegal.not-allowed-here.toml";

#[test]
fn a_key_and_its_value() {
    assert_eq!(leaf("a = 1", "a"), "variable.other.key.toml");
    assert_eq!(leaf("a = 1", "="), "punctuation.separator.key-value.toml");
    assert_eq!(leaf("a = 1", "1"), "constant.numeric.integer.toml");
    assert_eq!(leaf("a = 1.5", "1.5"), "constant.numeric.float.toml");
    assert_eq!(leaf("a = 0xff", "0xff"), "constant.numeric.hex.toml");
    assert_eq!(
        leaf("a = 1979-05-27", "1979-05-27"),
        "constant.other.date.toml"
    );
    assert_eq!(leaf("a = 07:32:00", "07:32:00"), "constant.other.time.toml");
    assert_eq!(leaf("a = true", "true"), "constant.language.boolean.toml");
    assert_eq!(leaf("a = -inf", "-inf"), "constant.numeric.toml");
}

#[test]
fn a_number_with_a_leading_zero_is_a_zero_and_text_after_it() {
    assert_eq!(leaf("a = 01", "0"), "constant.numeric.integer.toml");
    assert_eq!(leaf("a = 01", "1"), INVALID);
    assert_eq!(leaf("a = 1.", "1"), "constant.numeric.float.toml");
    assert_eq!(leaf("a = 1.", "."), INVALID);
}

#[test]
fn a_date_is_read_only_when_every_part_is_in_range() {
    assert_eq!(
        leaf("a = 1979-13-01", "1979"),
        "constant.numeric.integer.toml"
    );
    assert_eq!(
        leaf("a = 1979-01-01T24:00:00+24:00", "1979-01-01T24:00:00+24:00"),
        "constant.other.date.toml"
    );
    assert_eq!(
        leaf("a = 1979-01-01T25:00:00", "1979-01-01"),
        "constant.other.date.toml"
    );
    assert_eq!(leaf("a = 1979-01-01T25:00:00", "T25:00:00"), INVALID);
}

#[test]
fn a_table_header_names_each_dotted_part() {
    let rows = lex("[a.\"b c\"]");
    let names: Vec<_> = rows[0]
        .iter()
        .filter(|(_, scopes)| {
            scopes
                .last()
                .is_some_and(|s| s == "entity.name.section.toml")
        })
        .map(|(text, _)| text.as_str())
        .collect();
    assert_eq!(names, ["a", "\"b", "c\""]);
    assert_eq!(
        rows[0][0].1,
        [
            "source.toml",
            "meta.group.toml",
            "punctuation.definition.section.begin.toml"
        ]
    );
    assert_eq!(lex("[[a]]")[0][0].1[1], "meta.group.double.toml");
}

#[test]
fn a_table_header_is_read_only_at_the_start_of_a_line() {
    assert_eq!(
        last_leaf("  [a]", 0),
        "punctuation.definition.section.begin.toml"
    );
    assert_eq!(last_leaf("a = 1 [b]", 0), INVALID);
    assert_eq!(last_leaf("[a b", 0), INVALID);
    assert_eq!(last_leaf("[a]]", 0), INVALID);
}

#[test]
fn a_comment_after_blanks_has_the_leading_space_scope() {
    let rows = lex("  # c\n\u{a0}# c");
    assert_eq!(
        rows[0][0].1.last().map(String::as_str),
        Some("punctuation.whitespace.comment.leading.toml")
    );
    assert_eq!(rows[1][0].1, ["source.toml"]);
}

#[test]
fn text_that_is_no_rule_is_invalid_with_the_white_space_that_follows_it() {
    let rows = lex("x y\nz  ");
    assert_eq!(rows[0][0].0, "x ");
    assert_eq!(rows[0][0].1.last().map(String::as_str), Some(INVALID));
    assert_eq!(rows[1][0].0, "z");
}

#[test]
fn what_follows_an_equals_that_is_no_value_is_plain_for_one_character() {
    let rows = lex("a = ??");
    let texts: Vec<_> = rows[0].iter().map(|(t, _)| t.as_str()).collect();
    assert_eq!(texts, ["a", " ", "=", " ?", "?"]);
    assert_eq!(rows[0][3].1, ["source.toml"]);
    assert_eq!(rows[0][4].1.last().map(String::as_str), Some(INVALID));
    let rows = lex("a == 1");
    assert_eq!(rows[0].last().map(|(t, _)| t.as_str()), Some("= 1"));
}

/// `3` follows `2` with no comma between them.
/// The comma after `3` is a separator.
/// `4` after that is a value again.
#[test]
fn a_value_after_a_value_in_an_array_needs_a_comma_across_lines() {
    let text = indoc! {"
        a = [
          1,
          2
          3 ,
          4
        ]
    "};
    let rows = lex(text);
    let integer = Some("constant.numeric.integer.toml");
    let leaf_of = |row: usize, token: usize| rows[row][token].1.last().map(String::as_str);
    assert_eq!(leaf_of(1, 1), integer);
    assert_eq!(leaf_of(2, 1), integer);
    assert_eq!(leaf_of(3, 1), Some(INVALID));
    assert_eq!(leaf_of(3, 2), Some("punctuation.separator.array.toml"));
    assert_eq!(leaf_of(4, 1), integer);
    assert_eq!(leaf_of(5, 0), Some("punctuation.definition.array.end.toml"));
}

#[test]
fn an_inline_table_has_no_invalid_text_and_no_comments() {
    let rows = lex("a = { b, c = 1 # d\n}");
    assert!(
        rows.iter()
            .flatten()
            .all(|(_, scopes)| scopes.last().is_none_or(|s| s != INVALID))
    );
    let comment = rows[0]
        .iter()
        .any(|(_, scopes)| scopes.iter().any(|s| s == "comment.line.number-sign.toml"));
    assert!(!comment);
    assert_eq!(
        rows[1][0].1.last().map(String::as_str),
        Some("punctuation.definition.inline-table.end.toml")
    );
}

#[test]
fn a_key_that_begins_with_a_quote_ends_at_the_first_quote_before_an_equals() {
    let rows = lex("\"a\".\"b\" = 1");
    let invalid: Vec<_> = rows[0]
        .iter()
        .filter(|(_, scopes)| scopes.last().is_some_and(|s| s == INVALID))
        .map(|(t, _)| t.as_str())
        .collect();
    assert_eq!(invalid, ["\"", "\""]);
    let rows = lex("\"a\" = \"b\", \"c\" = 1");
    assert_eq!(rows[0].iter().filter(|(t, _)| t == "=").count(), 2);
}

#[test]
fn strings_continue_across_lines_and_close_with_up_to_five_quotes() {
    let text = indoc! {r#"
        a = """x
        y\
        z""""" b
        c = 1
    "#};
    let rows = lex(text);
    let triple = "string.quoted.triple.double.toml";
    assert!(
        rows[1]
            .iter()
            .all(|(_, scopes)| scopes.iter().any(|s| s == triple))
    );
    assert_eq!(rows[2][1].0, "\"\"\"\"\"");
    assert_eq!(
        rows[2][1].1.last().map(String::as_str),
        Some("punctuation.definition.string.end.toml")
    );
    assert_eq!(
        rows[2]
            .last()
            .and_then(|(_, scopes)| scopes.last().map(String::as_str)),
        Some(INVALID)
    );
    assert_eq!(
        rows[3][0].1.last().map(String::as_str),
        Some("variable.other.key.toml")
    );
}

/// The backslashes and the `u`s sit in separate pieces of the text.
/// No tool then reads them as one escape.
#[test]
fn escapes_are_valid_or_invalid_by_their_digits() {
    let text = ["a = \"", "\\n", "\\", "u00e9", "\\", "u00e ", "\\q", "\""].concat();
    let rows = lex(&text);
    let scoped: Vec<_> = rows[0]
        .iter()
        .filter(|(_, scopes)| {
            scopes.last().is_some_and(|s| {
                s.starts_with("constant.character") || s.starts_with("invalid.illegal.escape")
            })
        })
        .map(|(t, s)| {
            (
                t.as_str(),
                s.last()
                    .map(String::as_str)
                    .unwrap_or_default()
                    .starts_with("constant"),
            )
        })
        .collect();
    assert_eq!(
        scoped,
        [
            ("\\n", true),
            ("\\u00e9", true),
            ("\\u", false),
            ("\\q", false)
        ]
    );
}

#[test]
fn a_backslash_ends_a_basic_string_line_as_invalid_and_a_multi_line_one_as_text() {
    assert_eq!(last_leaf("a = \"x\\", 0), "invalid.illegal.escape.toml");
    assert_eq!(
        last_leaf("a = \"\"\"x\\", 0),
        "string.quoted.triple.double.toml"
    );
}

#[test]
fn the_nesting_limit_keeps_the_text_plain() {
    let limits = Limits {
        depth: Some(3),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Toml, limits);
    let mut state = State::default();
    let mut tokens = Vec::new();
    let line = "a = [[[[[1]]]]]";
    let report = highlighter.line(line, &mut state, &mut tokens);
    assert!(report.reached(crate::Limit::Depth));
    assert_eq!(tokens.last().map(|token| token.end()), Some(line.len()));
}
