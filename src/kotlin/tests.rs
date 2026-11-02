use indoc::indoc;

use crate::{Highlighter, Language, State};

/// One lexed line: each token's text and its scope names, outermost first.
type Row = Vec<(String, Vec<String>)>;

fn lex(text: &str) -> Vec<Row> {
    let mut highlighter = Highlighter::new(Language::Kotlin);
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

/// The innermost scope of the first token of line `line`.
fn first_leaf(rows: &[Row], line: usize) -> &str {
    rows[line][0].1.last().map_or("", String::as_str)
}

#[test]
fn names_are_known_under_their_aliases() {
    for name in ["kotlin", "kt", "KTS"] {
        assert_eq!(Language::from_name(name), Some(Language::Kotlin));
    }
}

#[test]
fn a_class_with_a_function_and_a_property() {
    let text = indoc! {"
        class Box<T>(val value: T) {
            fun get(): T = value
        }
    "};
    assert_eq!(leaf(text, "class"), "keyword.hard.class.kotlin");
    assert_eq!(leaf(text, "Box"), "entity.name.type.class.kotlin");
    assert_eq!(leaf(text, "T"), "entity.name.type.kotlin");
    assert_eq!(leaf(text, "val"), "keyword.hard.kotlin");
    assert_eq!(leaf(text, "fun"), "keyword.hard.fun.kotlin");
    assert_eq!(leaf(text, "get"), "entity.name.function.declaration.kotlin");
}

#[test]
fn an_extension_function_names_its_receiver() {
    let text = "fun String.shout() = this";
    assert_eq!(
        leaf(text, "String"),
        "entity.name.type.class.extension.kotlin"
    );
    assert_eq!(
        leaf(text, "shout"),
        "entity.name.function.declaration.kotlin"
    );
    assert_eq!(leaf(text, "this"), "variable.language.this.kotlin");
}

#[test]
fn a_receiver_without_a_name_after_its_dot_is_the_name() {
    let text = "fun Foo.";
    assert_eq!(leaf(text, "Foo"), "entity.name.function.declaration.kotlin");
}

#[test]
fn a_call_a_reference_and_a_type_annotation() {
    let text = "val f: Map<K, V> = run<Int>(::g)";
    assert_eq!(leaf(text, "K"), "entity.name.type.kotlin");
    assert_eq!(leaf(text, "run"), "entity.name.function.call.kotlin");
    assert_eq!(leaf(text, "Int"), "entity.name.type.kotlin");
    assert_eq!(leaf(text, "g"), "entity.name.function.reference.kotlin");
}

#[test]
fn a_block_comment_and_a_raw_string_continue_across_lines() {
    let text = indoc! {r#"
        /* a
           b */ val s = """
          x ${y} "q"
        """
    "#};
    let rows = lex(text);
    assert_eq!(first_leaf(&rows, 1), "comment.block.kotlin");
    assert!(
        rows[2]
            .iter()
            .all(|(_, scopes)| { scopes.iter().any(|s| s == "string.quoted.double.kotlin") })
    );
    assert!(rows[2].iter().any(|(t, scopes)| {
        t == "${"
            && scopes
                .last()
                .is_some_and(|s| s == "punctuation.definition.template-expression.begin")
    }));
    assert_eq!(first_leaf(&rows, 3), "string.quoted.double.kotlin");
}

#[test]
fn a_template_expression_holds_code() {
    let text = r#"val s = "a ${b + 1} c $d""#;
    assert_eq!(leaf(text, "1"), "constant.numeric.decimal.kotlin");
    assert_eq!(leaf(text, "+"), "keyword.operator.arithmetic.kotlin");
    assert_eq!(leaf(text, "$d"), "variable.string-escape.kotlin");
    assert_eq!(
        leaf(text, "}"),
        "punctuation.definition.template-expression.end"
    );
}

#[test]
fn a_template_expression_ends_at_the_first_closing_brace() {
    let rows = lex(r#""${ f { x } y }""#);
    let after: Vec<_> = rows[0]
        .iter()
        .skip_while(|(t, _)| t != "}")
        .map(|(t, _)| t.as_str())
        .collect();
    assert!(
        after.len() > 1,
        "the text after the first brace is outside the expression"
    );
}

#[test]
fn kdoc_tags_are_keywords_and_their_names_parameters() {
    let rows = lex(indoc! {"
        /** @param x the x
         * @see y
         */"});
    let tag = |row: usize, text: &str| {
        rows[row]
            .iter()
            .find(|(t, _)| t == text)
            .and_then(|(_, scopes)| scopes.last().cloned())
            .unwrap_or_default()
    };
    assert_eq!(
        tag(0, "@param"),
        "keyword.other.documentation.javadoc.kotlin"
    );
    assert_eq!(tag(0, "x"), "variable.parameter.kotlin");
    assert_eq!(tag(1, "@see"), "keyword.other.documentation.javadoc.kotlin");
}

#[test]
fn a_link_in_kdoc_names_its_type_and_member() {
    let rows = lex("/** {@link a.B#c(d)} done */");
    let scope = |text: &str| {
        rows[0]
            .iter()
            .find(|(t, _)| t == text)
            .and_then(|(_, scopes)| scopes.last().cloned())
            .unwrap_or_default()
    };
    assert_eq!(scope("@link"), "keyword.other.documentation.javadoc.kotlin");
    assert_eq!(scope("a.B"), "entity.name.type.class.kotlin");
    assert_eq!(scope("c(d)"), "variable.parameter.kotlin");
}

#[test]
fn an_empty_kdoc_looking_comment_stays_open() {
    let rows = lex("/**/ x\ny */");
    assert_eq!(first_leaf(&rows, 1), "comment.block.javadoc.kotlin");
}

#[test]
fn a_use_site_target_and_an_annotation() {
    assert_eq!(
        leaf("@file:JvmName(\"A\")", "@file"),
        "entity.name.type.annotation-site.kotlin"
    );
    assert_eq!(
        leaf("@get:Json fun f()", "Json"),
        "entity.name.type.annotation.kotlin"
    );
    assert_eq!(
        leaf("@Deprecated fun f()", "@Deprecated"),
        "entity.name.type.annotation.kotlin"
    );
}

#[test]
fn numbers_in_every_base() {
    for (number, scope) in [
        ("1_000L", "constant.numeric.decimal.kotlin"),
        ("1.5f", "constant.numeric.decimal.kotlin"),
        ("2e10", "constant.numeric.decimal.kotlin"),
        ("0xFFu", "constant.numeric.hex.kotlin"),
        ("0b1010", "constant.numeric.binary.kotlin"),
    ] {
        assert_eq!(leaf(&format!("x = {number}"), number), scope, "{number}");
    }
}

#[test]
fn an_unterminated_exponent_is_not_a_number() {
    let rows = lex("x = 1e");
    assert!(rows[0].iter().all(|(_, scopes)| {
        scopes
            .last()
            .is_none_or(|s| s != "constant.numeric.decimal.kotlin")
    }));
}

#[test]
fn an_import_ends_at_the_line_end() {
    let rows = lex("import a.b.*\nval x = 1");
    let package = rows[0]
        .iter()
        .find(|(t, _)| t == "a.b.")
        .and_then(|(_, scopes)| scopes.last().cloned());
    assert_eq!(package.as_deref(), Some("entity.name.package.kotlin"));
    assert_eq!(first_leaf(&rows, 1), "keyword.hard.kotlin");
}

#[test]
fn a_one_letter_key_before_two_equals_signs_is_a_parameter() {
    assert_eq!(leaf("a==b", "a="), "variable.parameter.kotlin");
}

#[test]
fn an_empty_line_keeps_the_state() {
    let rows = lex(indoc! {"
        /* a

        b */"});
    assert_eq!(rows[1].len(), 0);
    assert_eq!(first_leaf(&rows, 2), "comment.block.kotlin");
}
