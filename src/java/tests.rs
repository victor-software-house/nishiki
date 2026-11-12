use indoc::indoc;

use crate::test_support::rows::Row;
use crate::{Highlighter, Language, Limit, Limits, State, test_support};

fn lex(text: &str) -> Vec<Row> {
    test_support::rows::lex(Language::Java, text)
}

/// The innermost scope of the first token whose text is `token`.
fn leaf(text: &str, token: &str) -> String {
    test_support::scopes::leaf(Language::Java, text, token)
}

#[test]
fn a_class_with_a_method_and_a_field() {
    let text = indoc! {"
        public class A {
          int x = 1;
          void m(String s) {}
        }
    "};
    assert_eq!(leaf(text, "A"), "entity.name.type.class.java");
    assert_eq!(leaf(text, "m"), "entity.name.function.java");
    assert_eq!(leaf(text, "s"), "variable.parameter.java");
    assert_eq!(leaf(text, "x"), "variable.other.definition.java");
    assert_eq!(leaf(text, "1"), "constant.numeric.decimal.java");
    assert_eq!(leaf(text, "public"), "storage.modifier.java");
}

#[test]
fn a_block_comment_and_a_text_block_continue_across_lines() {
    let text = indoc! {r#"
        /* a
           b */ int
        String s = """
          x\tz "quoted"
        """;
    "#};
    let rows = lex(text);
    assert_eq!(
        rows[1][0].1.last().map(String::as_str),
        Some("comment.block.java")
    );
    let block = "string.quoted.triple.java";
    assert!(
        rows[3]
            .iter()
            .all(|(_, scopes)| scopes.iter().any(|s| s == block))
    );
    assert!(rows[3].iter().any(|(t, scopes)| {
        t == "\\t"
            && scopes
                .last()
                .is_some_and(|s| s == "constant.character.escape.java")
    }));
    assert_eq!(
        rows[4][0].1.last().map(String::as_str),
        Some("punctuation.definition.string.end.java")
    );
}

#[test]
fn a_string_without_its_closing_quote_keeps_the_next_line() {
    let rows = lex("String s = \"a\nb\";");
    assert_eq!(
        rows[1][0].1.last().map(String::as_str),
        Some("string.quoted.double.java")
    );
    assert_eq!(
        rows[1][1].1.last().map(String::as_str),
        Some("punctuation.definition.string.end.java")
    );
}

#[test]
fn javadoc_tags_are_scoped_only_in_a_comment_that_starts_a_line() {
    let rows = lex("/** @param x the x */\nint a; /** @param y */");
    assert!(rows[0].iter().any(|(t, scopes)| {
        t == "@param"
            && scopes
                .last()
                .is_some_and(|s| s == "keyword.other.documentation.javadoc.java")
    }));
    assert!(rows[1].iter().all(|(_, scopes)| {
        scopes
            .last()
            .is_none_or(|s| s != "keyword.other.documentation.javadoc.java")
    }));
}

#[test]
fn a_line_comment_after_blanks_gets_the_leading_space_scope() {
    let rows = lex("  // c\nint a; // d");
    assert_eq!(
        rows[0][0].1.last().map(String::as_str),
        Some("punctuation.whitespace.comment.leading.java")
    );
    assert!(rows[1].iter().all(|(_, scopes)| {
        scopes
            .last()
            .is_none_or(|s| s != "punctuation.whitespace.comment.leading.java")
    }));
}

#[test]
fn a_reserved_word_in_a_package_name_is_invalid() {
    assert_eq!(
        leaf("package a.class.b;", "class"),
        "invalid.illegal.character_not_allowed_here.java"
    );
}

#[test]
fn generics_nest_and_name_their_arguments() {
    let text = "Map<String, List<Integer>> m = null;";
    let rows = lex(text);
    let angles = rows[0]
        .iter()
        .filter(|(_, scopes)| {
            scopes
                .last()
                .is_some_and(|s| s == "punctuation.bracket.angle.java")
        })
        .count();
    assert_eq!(angles, 4);
    assert_eq!(leaf(text, "m"), "variable.other.definition.java");
}

#[test]
fn a_record_has_a_head_and_a_body() {
    let text = indoc! {"
        record R<T>(int a, T b) implements I {
          R {}
        }
    "};
    assert_eq!(leaf(text, "R"), "entity.name.type.record.java");
    assert_eq!(leaf(text, "implements"), "storage.modifier.implements.java");
}

#[test]
fn an_enum_names_its_constants() {
    let text = indoc! {"
        enum E {
          A, B(1);
          int x;
        }
    "};
    assert_eq!(leaf(text, "A"), "constant.other.enum.java");
    assert_eq!(leaf(text, "B"), "constant.other.enum.java");
    assert_eq!(leaf(text, "E"), "entity.name.type.enum.java");
}

#[test]
fn a_package_name_with_a_capital_is_deprecated() {
    let text = "package a.B;";
    assert_eq!(
        leaf(text, "B"),
        "invalid.deprecated.package_name_not_lowercase.java"
    );
}

#[test]
fn numbers_take_their_radix_and_suffix() {
    assert_eq!(leaf("x = 0x1Fp3;", "0x1Fp3"), "constant.numeric.hex.java");
    assert_eq!(
        leaf("x = 0b101L;", "0b101L"),
        "constant.numeric.binary.java"
    );
    assert_eq!(leaf("x = 017;", "017"), "constant.numeric.octal.java");
    assert_eq!(
        leaf("x = 1_000.5e3f;", "1_000.5e3f"),
        "constant.numeric.decimal.java"
    );
}

#[test]
fn the_nesting_limit_keeps_the_text_plain() {
    let limits = Limits {
        depth: Some(3),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Java, limits);
    let mut state = State::default();
    let mut tokens = Vec::new();
    let line = "((((((1))))))";
    let report = highlighter.line(line, &mut state, &mut tokens);
    assert!(report.reached(Limit::Depth));
    assert_eq!(tokens.last().map(|token| token.end()), Some(line.len()));
}

/// Each line lexed with [`super::lex_until`] and the end `%>`, the state
/// carried from line to line: where the line halted and where its last token
/// ends.
fn until(lines: &[&str]) -> Vec<(Option<usize>, usize)> {
    use crate::arena::{Arena, ROOT};
    use crate::language::{Context, Line};
    use crate::scratch::Scratch;
    use crate::{LineReport, Token};

    let mut arena = Arena::new(None);
    let mut scratch = Scratch::default();
    let mut state = super::start_embedded(&mut arena, ROOT);
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
            first_token: 0,
        };
        let line = Line {
            text,
            start: 0,
            embedded: false,
            anchored_at_root: false,
        };
        let (next, halted) = super::lex_until(&mut cx, line, state, &|s: &[u8], p: usize| {
            s.get(p..).is_some_and(|rest| rest.starts_with(b"%>"))
        });
        state = next.state();
        rows.push((
            halted.map(|stopped| stopped.at),
            out.last().map_or(0, |token| token.end()),
        ));
    }
    rows
}

#[test]
fn embedded_java_halts_at_its_end_only_where_no_rule_is_open() {
    for (line, halt) in [
        ("int a = 1; %> b", Some(11)),
        ("%>", Some(0)),
        ("a %= 2 %> b", Some(7)),
        ("String s = \"%>\"; %> b", Some(17)),
        ("/* %> */ %> b", Some(9)),
        ("// %> b %>", None),
        ("int a = 1;", None),
    ] {
        let rows = until(&[line]);
        assert_eq!(rows[0].0, halt, "{line:?}");
        let covered = halt.unwrap_or(line.len());
        assert_eq!(
            rows[0].1, covered,
            "the tokens stop at the halt in {line:?}"
        );
    }
}

#[test]
fn embedded_java_halts_after_the_rules_of_earlier_lines_close() {
    let rows = until(&["if (a) {", "// %> b %>", "} %> c"]);
    assert_eq!(rows[0].0, None);
    assert_eq!(rows[1].0, None);
    assert_eq!(rows[2].0, Some(2));
}

/// The generic names of two records keep their own scopes.
/// Both put a one-scope slice built on the stack on the same base.
/// A cache keyed by the slice's address therefore gave the second record the
/// first one's stack.
#[test]
fn the_generic_names_of_two_records_keep_their_own_scopes() {
    let text = indoc! {"
        record A<extends>() {}
        record B<super>() {}
    "};
    assert_eq!(leaf(text, "extends"), "storage.modifier.extends.java");
    assert_eq!(leaf(text, "super"), "storage.modifier.super.java");
}
