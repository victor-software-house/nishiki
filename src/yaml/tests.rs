use indoc::indoc;

use crate::test_support::rows::Row;
use crate::{Highlighter, Language, Limit, Limits, State, test_support};

/// Lexes `text` and returns, per line, each token's text with its scope
/// stack, outermost first, without the `.yaml` suffix.
fn lex(text: &str) -> Vec<Row> {
    let mut rows = test_support::rows::lex(Language::Yaml, text);
    for (_, scopes) in rows.iter_mut().flatten() {
        for scope in scopes {
            scope.truncate(scope.trim_end_matches(".yaml").len());
        }
    }
    rows
}

/// The scopes after `source`, joined, of the first token of `text` that
/// reads `token`.
fn scope_of(text: &str, token: &str) -> String {
    let lines = lex(text);
    let found = lines.iter().flatten().find(|(t, _)| t == token);
    let (_, scopes) = found.unwrap_or_else(|| panic!("no token {token:?} in {lines:?}"));
    scopes[1..].join(" ")
}

#[test]
fn a_key_is_tagged_and_its_value_is_not() {
    let text = "name: value";
    assert_eq!(
        scope_of(text, "name"),
        "string.unquoted.plain.out entity.name.tag"
    );
    assert_eq!(
        scope_of(text, ":"),
        "punctuation.separator.key-value.mapping"
    );
    assert_eq!(scope_of(text, "value"), "string.unquoted.plain.out");
}

#[test]
fn a_constant_key_is_a_constant_not_a_tag() {
    assert_eq!(scope_of("n: x", "n"), "constant.language.boolean");
    assert_eq!(scope_of("1: x", "1"), "constant.numeric.integer");
}

#[test]
fn numbers_follow_the_yaml_1_1_patterns() {
    for (value, scope) in [
        ("0x1F", "constant.numeric.integer"),
        ("012", "constant.numeric.integer"),
        ("1_000", "constant.numeric.integer"),
        ("1:30:00", "constant.numeric.integer"),
        ("1.5.2", "constant.numeric.float"),
        ("-.inf", "constant.numeric.float"),
        ("1.5e+3", "constant.numeric.float"),
        ("2001-12-14", "constant.other.timestamp"),
        ("2001-12-14 21:59:43.10 Z", "constant.other.timestamp"),
        ("~", "constant.language.null"),
        ("Off", "constant.language.boolean"),
        ("<<", "constant.language.merge"),
    ] {
        assert_eq!(scope_of(&format!("k: {value}"), value), scope, "{value}");
    }
}

/// The grammar has no `0o`.
/// An exponent needs its sign.
/// The zone has no blank before `-5`.
#[test]
fn a_scalar_that_nearly_matches_is_plain() {
    for value in [
        "0o17",
        "1e3",
        "1.5e3",
        "2001-12-14 21:59:43.10 -5",
        "true x",
    ] {
        let lines = lex(&format!("k: {value}"));
        assert!(
            lines[0].iter().skip(3).all(|(_, scopes)| scopes
                .last()
                .is_some_and(|s| s == "string.unquoted.plain.out")),
            "{value}: {lines:?}"
        );
    }
}

#[test]
fn a_comment_needs_a_blank_before_it_and_no_tab_in_it() {
    assert_eq!(
        scope_of("a: b #c", "#"),
        "comment.line.number-sign punctuation.definition.comment"
    );
    let lines = lex("a: b#c");
    assert_eq!(lines[0].len(), 4, "{lines:?}");
    assert_eq!(scope_of("a: b #c\td", "#"), "");
}

#[test]
fn an_indented_comment_line_has_a_leading_whitespace_scope() {
    assert_eq!(
        scope_of("  # c", "  "),
        "punctuation.whitespace.comment.leading"
    );
}

/// `z` is indented less than the first line.
/// It is plain text again.
#[test]
fn a_block_scalar_ends_at_the_first_line_indented_less() {
    let text = indoc! {"
        a: |
            x
          z
        b: 1
    "};
    let lines = lex(text);
    let last = |line: usize| {
        lines[line]
            .last()
            .and_then(|(_, scopes)| scopes.last().cloned())
    };
    assert_eq!(last(1).as_deref(), Some("string.unquoted.block"));
    assert_eq!(last(2).as_deref(), Some("string.unquoted.plain.out"));
    assert_eq!(
        scope_of(text, "b"),
        "string.unquoted.plain.out entity.name.tag"
    );
}

#[test]
fn a_block_scalar_without_indented_lines_ends_at_once() {
    let lines = lex("a: |\nb: 1");
    assert_eq!(
        scope_of("a: |\nb: 1", "b"),
        "string.unquoted.plain.out entity.name.tag"
    );
    assert_eq!(lines[1].len(), 4);
}

#[test]
fn a_block_header_takes_one_indicator_and_a_comment() {
    assert_eq!(
        scope_of("a: |2-", "2"),
        "keyword.control.flow.block-scalar.literal constant.numeric.indentation-indicator"
            .replace("keyword.control.flow.block-scalar.literal ", "")
    );
    assert_eq!(
        scope_of("a: >-  # c", "-"),
        "storage.modifier.chomping-indicator"
    );
    assert_eq!(
        scope_of("a: |-2", "2"),
        "invalid.illegal.expected-comment-or-newline"
    );
    assert_eq!(
        scope_of("a: |x", "x"),
        "invalid.illegal.expected-comment-or-newline"
    );
}

/// The `u` is added apart.
/// This source then holds no unicode escape.
/// The letter in `\x4a` ends that escape.
/// `\q` is no escape.
#[test]
fn escapes_in_double_quotes_take_decimal_digits_only() {
    let unicode = format!("\\{}0041", 'u');
    let text = format!(r#"a: "\n\x41\x4a{unicode}\q\p""#);
    for escape in ["\\n", "\\x41", unicode.as_str(), "\\p"] {
        assert_eq!(
            scope_of(&text, escape),
            "string.quoted.double constant.character.escape",
            "{escape}"
        );
    }
    let lines = lex(&text);
    for plain in ["\\x4a", "\\q"] {
        assert!(
            lines[0].iter().any(|(t, s)| t == plain && s.len() == 2),
            "{plain}: {lines:?}"
        );
    }
}

#[test]
fn a_double_quoted_string_and_its_escape_can_cross_lines() {
    let lines = lex("a: \"x\\\ny\"");
    assert_eq!(
        lines[0].last().map(|(_, s)| s.last().cloned()),
        Some(Some(
            "constant.character.escape.double-quoted.newline".to_owned()
        ))
    );
    assert_eq!(
        lines[1][1].1.last().map(String::as_str),
        Some("punctuation.definition.string.end")
    );
}

#[test]
fn a_single_quote_is_doubled_to_be_in_the_string() {
    assert_eq!(
        scope_of("a: 'it''s'", "''"),
        "string.quoted.single constant.character.escape.single-quoted"
    );
}

#[test]
fn a_flow_pair_opens_at_the_colon() {
    let lines = lex("{a: 1}");
    let scopes: Vec<(&str, String)> = lines[0]
        .iter()
        .map(|(text, scopes)| (text.as_str(), scopes[1..].join(" ")))
        .collect();
    assert_eq!(
        scopes,
        [
            (
                "{",
                "meta.flow-mapping punctuation.definition.mapping.begin".to_owned()
            ),
            (
                "a",
                "meta.flow-mapping meta.flow-pair.key string.unquoted.plain.in entity.name.tag"
                    .to_owned()
            ),
            (
                ":",
                "meta.flow-mapping meta.flow-pair punctuation.separator.key-value.mapping"
                    .to_owned()
            ),
            (
                " ",
                "meta.flow-mapping meta.flow-pair meta.flow-pair.value".to_owned()
            ),
            (
                "1",
                "meta.flow-mapping meta.flow-pair meta.flow-pair.value constant.numeric.integer"
                    .to_owned()
            ),
            (
                "}",
                "meta.flow-mapping punctuation.definition.mapping.end".to_owned()
            ),
        ]
    );
}

#[test]
fn a_flow_collection_continues_on_the_next_line() {
    let lines = lex("a: [x,\n  z]");
    assert_eq!(
        lines[1][1].1[1..].join(" "),
        "meta.flow-sequence string.unquoted.plain.in"
    );
}

#[test]
fn a_key_before_a_flow_bracket_is_not_a_pair() {
    assert_eq!(
        scope_of("[a:]", "a"),
        "meta.flow-sequence string.unquoted.plain.in"
    );
    assert_eq!(
        scope_of("{a:1}", "a:1"),
        "meta.flow-mapping string.unquoted.plain.in"
    );
}

#[test]
fn documents_and_directives() {
    assert_eq!(scope_of("---", "---"), "entity.other.document.begin");
    assert_eq!(scope_of("--- text", "text"), "string.unquoted.plain.out");
    assert_eq!(scope_of("...", "..."), "entity.other.document.end");
    assert_eq!(
        scope_of("%YAML 1.2", "1.2"),
        "meta.directive constant.numeric.yaml-version"
    );
    assert_eq!(
        scope_of("%TAG !e! tag:x", "tag:x"),
        "meta.directive support.type.tag-prefix"
    );
    assert_eq!(
        scope_of("%YAML 1.2 x", "x"),
        "meta.directive invalid.illegal.unrecognized"
    );
}

/// The tokens of the one line `line`: each token's text, and its scopes
/// after `source`, joined.
fn tokens_of(line: &str) -> Vec<(String, String)> {
    lex(line)
        .remove(0)
        .into_iter()
        .map(|(text, scopes)| (text, scopes[1..].join(" ")))
        .collect()
}

/// `rows` as `tokens_of` returns them.
fn rows(rows: &[(&str, &str)]) -> Vec<(String, String)> {
    rows.iter()
        .map(|&(text, scopes)| (text.to_owned(), scopes.to_owned()))
        .collect()
}

/// Shiki's grammar spells the blank between a directive's words `[\t ]`.
/// After a no-break space, `1.2` is no version, so `YAML` is a reserved name,
/// and every later word is a mistake.
///
/// Each expected row is Shiki's, without the `.yaml` suffixes.
#[test]
fn a_directive_word_after_a_no_break_space_is_a_mistake() {
    let nbsp = char::from(0xa0_u8);
    let gap = format!(" {nbsp}");
    assert_eq!(
        tokens_of(&format!("%YAML {nbsp}1.2")),
        rows(&[
            ("%", "meta.directive punctuation.definition.directive.begin"),
            ("YAML", "meta.directive support.other.directive.reserved"),
            (&gap, "meta.directive"),
            ("1.2", "meta.directive invalid.illegal.unrecognized"),
        ])
    );
    assert_eq!(
        tokens_of(&format!("%TAG {nbsp}!a! b")),
        rows(&[
            ("%", "meta.directive punctuation.definition.directive.begin"),
            ("TAG", "meta.directive keyword.other.directive.tag"),
            (&gap, "meta.directive"),
            ("!a!", "meta.directive invalid.illegal.unrecognized"),
            (" ", "meta.directive"),
            ("b", "meta.directive invalid.illegal.unrecognized"),
        ])
    );
}

/// The spaces before a directive's comment or line end lie outside the
/// directive, as its end `(?=$|[\t ]+($|#))` says.
/// A no-break space before them stays inside it.
///
/// Each expected row is Shiki's, without the `.yaml` suffixes.
#[test]
fn the_spaces_that_end_a_directive_lie_outside_it() {
    let nbsp = char::from(0xa0_u8).to_string();
    assert_eq!(
        tokens_of("%YAML 1.2  "),
        rows(&[
            ("%", "meta.directive punctuation.definition.directive.begin"),
            ("YAML", "meta.directive keyword.other.directive"),
            (" ", "meta.directive"),
            ("1.2", "meta.directive constant.numeric.yaml-version"),
            ("  ", ""),
        ])
    );
    assert_eq!(
        tokens_of(&format!("%FOO x{nbsp} #c")),
        rows(&[
            ("%", "meta.directive punctuation.definition.directive.begin"),
            ("FOO", "meta.directive support.other.directive.reserved"),
            (" ", "meta.directive"),
            ("x", "meta.directive string.unquoted.directive-name"),
            (&nbsp, "meta.directive"),
            (" ", ""),
            (
                "#",
                "comment.line.number-sign punctuation.definition.comment"
            ),
            ("c", "comment.line.number-sign"),
        ])
    );
}

/// A reserved directive takes a name and one parameter.
/// Its third word is a mistake.
///
/// Each expected row is Shiki's, without the `.yaml` suffixes.
#[test]
fn a_reserved_directive_third_word_is_a_mistake() {
    assert_eq!(
        tokens_of("%FOO x y z"),
        rows(&[
            ("%", "meta.directive punctuation.definition.directive.begin"),
            ("FOO", "meta.directive support.other.directive.reserved"),
            (" ", "meta.directive"),
            ("x", "meta.directive string.unquoted.directive-name"),
            (" ", "meta.directive"),
            ("y", "meta.directive string.unquoted.directive-parameter"),
            (" ", "meta.directive"),
            ("z", "meta.directive invalid.illegal.unrecognized"),
        ])
    );
}

#[test]
fn properties_and_aliases() {
    assert_eq!(
        scope_of("a: &x 1", "x"),
        "meta.property entity.name.type.anchor"
    );
    assert_eq!(
        scope_of("a: !!str 1", "!!str"),
        "meta.property storage.type.tag-handle"
    );
    assert_eq!(scope_of("a: *x", "x"), "variable.other.alias");
    assert_eq!(
        scope_of("a: !a{b 1", "!a{b"),
        "meta.property invalid.illegal.tag-handle"
    );
    assert_eq!(
        scope_of("a: &x/y 1", "/y"),
        "meta.property invalid.illegal.character.anchor"
    );
}

#[test]
fn an_explicit_key_opens_a_block_mapping_until_its_colon() {
    let lines = lex("? a\n: b");
    assert_eq!(lines[0][0].1[1..].join(" "), "meta.block-mapping");
    assert_eq!(
        lines[1][0].1[1..].join(" "),
        "meta.block-mapping punctuation.separator.key-value.mapping"
    );
    assert_eq!(
        lines[1].last().map(|(_, s)| s[1..].join(" ")),
        Some("string.unquoted.plain.out".to_owned())
    );
}

#[test]
fn characters_that_cannot_start_a_scalar_have_no_scope() {
    assert_eq!(scope_of("a: @x", "@"), "");
    assert_eq!(scope_of("a: ]x", "]"), "");
    assert_eq!(scope_of("[|]", "|"), "meta.flow-sequence");
}

#[test]
fn an_empty_line_keeps_the_block_scalar_open() {
    let mut highlighter = Highlighter::new(Language::Yaml);
    let mut state = State::default();
    let mut tokens = Vec::new();
    let _ = highlighter.line("a: |", &mut state, &mut tokens);
    let open = state;
    let _ = highlighter.line("", &mut state, &mut tokens);
    assert_eq!(state, open);
}

#[test]
fn depth_limit_stops_nesting_flow_collections() {
    let limits = Limits {
        depth: Some(8),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Yaml, limits);
    let mut tokens = Vec::new();
    let report = highlighter.line(&"[".repeat(100), &mut State::default(), &mut tokens);
    assert!(report.reached(Limit::Depth));
    assert_eq!(tokens.last().map(|t| t.end()), Some(100));
}

/// A Unicode space separates after the colon of a key and after a sequence
/// dash.
/// It does not separate before a `#`.
/// Only a space or a tab makes a comment.
/// A tag before a Unicode space is not a valid tag.
#[test]
fn unicode_spaces_separate_as_blanks_where_the_grammar_reads_them() {
    for space in ["\u{a0}", "\u{85}", "\u{2003}", "\u{3000}"] {
        let text = format!("a:{space}b");
        assert_eq!(
            scope_of(&text, "a"),
            "string.unquoted.plain.out entity.name.tag",
            "{space:?}"
        );
        assert_eq!(
            scope_of(&format!("-{space}a"), "-"),
            "punctuation.definition.block.sequence.item",
            "{space:?}"
        );
        let text = format!("a: b{space}# c");
        assert!(
            !lex(&text)[0].iter().any(|(_, scopes)| scopes
                .iter()
                .any(|scope| scope == "comment.line.number-sign")),
            "{space:?}"
        );
        assert_eq!(
            scope_of(&format!("a: !!str{space}b"), "!!str"),
            "meta.property invalid.illegal.tag-handle",
            "{space:?}"
        );
    }
}

/// YAML as another language embeds it.
const EMBEDDED: crate::test_support::embedded::Embedded = crate::test_support::embedded::Embedded {
    start: super::start_embedded,
    lex: super::lex,
    lex_until: super::lex_until,
};

/// The scopes a Ruby here document named for YAML puts under its lines, as
/// Shiki gives them.
/// `source.yaml` comes from the rule's `contentName`.
const RUBY_HEREDOC: [&str; 3] = ["source.ruby", "meta.embedded.block.yaml", "source.yaml"];

/// Where each line, lexed alone, halts under a Ruby here document's halts.
fn ruby_halts(lines: &[&str]) -> Vec<Option<usize>> {
    lines
        .iter()
        .flat_map(|line| {
            crate::test_support::embedded::ruby_heredoc_halts(EMBEDDED, &RUBY_HEREDOC, &[line])
        })
        .collect()
}

/// Shiki hands an indented `#{` back to Ruby nowhere.
/// After blanks, the comment rule begins at the blanks before it.
/// Elsewhere, a plain scalar or a string holds it.
#[test]
fn an_indented_ruby_interpolation_in_yaml_stays_yaml() {
    assert_eq!(
        ruby_halts(&[
            "  #{a}: b",
            "  key: #{value}",
            "  key: plain #{value} more",
            "  key: \"q #{x}\" #{y}",
            "    - #{item}",
            "  key: a#{b}",
        ]),
        [None; 6]
    );
}

/// Shiki hands a line back to Ruby where a YAML step begins at the `#{`.
/// The start of an unindented line is such a place.
/// Inside a plain scalar, or at a comment after blanks, it hands nothing back.
#[test]
fn ruby_takes_a_yaml_line_back_where_a_step_begins_at_its_interpolation() {
    assert_eq!(
        ruby_halts(&["#{a}: b", "#{x}", "a#{b}: c", "key: e#{f}", "- #{g}"]),
        [Some(0), Some(0), None, None, None]
    );
}

/// A Ruby escape is tried after YAML's patterns.
/// A plain scalar that starts with it keeps it.
#[test]
fn a_ruby_escape_at_a_yaml_plain_scalar_stays_yaml() {
    assert_eq!(ruby_halts(&["\\n: c", "key: a \\n b"]), [None, None]);
}

/// The colons of `a::` do not pair up with the `:` after them, so the key
/// lookahead reads on.
/// A later `b:` makes `a:` a key, as in Shiki.
/// Without a later key, `a:` stays plain.
#[test]
fn a_later_key_tags_a_key_whose_colons_do_not_pair_up() {
    assert_eq!(
        scope_of("a:: b: c", "a:"),
        "string.unquoted.plain.out entity.name.tag"
    );
    assert_eq!(
        scope_of("YAML Test:: Suite:   x", "YAML Test:"),
        "string.unquoted.plain.out entity.name.tag"
    );
    assert_eq!(scope_of("a:: b", "a:"), "string.unquoted.plain.out");
    assert_eq!(
        scope_of("a:: c:: d:: b: e", "d:"),
        "string.unquoted.plain.out entity.name.tag"
    );
    assert_eq!(scope_of("a:: c:: d::", "d:"), "string.unquoted.plain.out");
    assert_eq!(
        scope_of("{a:: b: c}", "a:"),
        "meta.flow-mapping meta.flow-pair.key string.unquoted.plain.in entity.name.tag"
    );
    assert_eq!(
        scope_of("{a:: b}", "a:"),
        "meta.flow-mapping string.unquoted.plain.in"
    );
}
