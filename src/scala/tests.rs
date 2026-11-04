use indoc::indoc;

use crate::{Highlighter, Language, State};

/// One lexed line: each token's text and its scope names, outermost first.
type Row = Vec<(String, Vec<String>)>;

fn lex(text: &str) -> Vec<Row> {
    let mut highlighter = Highlighter::new(Language::Scala);
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

#[test]
fn the_name_is_known() {
    assert_eq!(Language::from_name("scala"), Some(Language::Scala));
}

#[test]
fn a_class_with_a_method_and_a_value() {
    let text = indoc! {"
        case class Point(x: Int, y: Int) {
          def norm: Double = math.sqrt(x * x + y * y)
          val zero = 0
        }"};
    assert_eq!(leaf(text, "case"), "keyword.declaration.scala");
    assert_eq!(leaf(text, "Point"), "entity.name.class.declaration");
    assert_eq!(leaf(text, "x"), "variable.parameter.scala");
    assert_eq!(leaf(text, "norm"), "entity.name.function.declaration");
    assert_eq!(leaf(text, "zero"), "variable.stable.declaration.scala");
    assert_eq!(leaf(text, "0"), "constant.numeric.scala");
}

#[test]
fn an_import_ends_at_its_line() {
    let rows = lex("import scala.collection.{Map => M, given}\nval x = 1");
    let names: Vec<&str> = rows[0].iter().map(|(t, _)| t.as_str()).collect();
    assert!(names.contains(&"Map"), "{names:?}");
    assert!(
        rows[1][0].1.iter().all(|s| s != "meta.import.scala"),
        "{:?}",
        rows[1]
    );
}

#[test]
fn operators_take_their_kind() {
    assert_eq!(leaf("a && b", "&&"), "keyword.operator.logical.scala");
    assert_eq!(leaf("a <= b", "<="), "keyword.operator.comparison.scala");
    assert_eq!(leaf("a + b", "+"), "keyword.operator.arithmetic.scala");
    assert_eq!(leaf("a ::: b", ":::"), "keyword.operator.scala");
}

/// The interpolated string lists `\$["$]` before the interpolation patterns,
/// so its `$$` is a plain escape, as in Shiki.
#[test]
fn an_interpolated_string_names_its_parts() {
    let text = r#"val t = s"x $name ${a + 1} $$""#;
    assert_eq!(leaf(text, "s"), "keyword.interpolation.scala");
    assert_eq!(leaf(text, "name"), "meta.template.expression.scala");
    assert_eq!(leaf(text, "$$"), "constant.character.escape.scala");
}

#[test]
fn a_block_comment_runs_across_lines() {
    let rows = lex(indoc! {"
        /** Docs
          * @param x the x
          */
        val y = 2"});
    assert!(rows[1].iter().any(|(t, s)| {
        t == "@param"
            && s.last()
                .is_some_and(|l| l == "keyword.other.documentation.scaladoc.scala")
    }));
    assert!(
        rows[3]
            .iter()
            .all(|(_, s)| s.iter().all(|l| !l.starts_with("comment")))
    );
}

#[test]
fn names_beyond_ascii_use_the_measured_classes() {
    assert_eq!(
        leaf("val café = 1", "café"),
        "variable.stable.declaration.scala"
    );
    assert_eq!(leaf("a ∘ b", "∘"), "keyword.operator.scala");
}

#[test]
fn an_end_marker_names_what_it_ends() {
    assert_eq!(leaf("end if", "end if"), "keyword.control.flow.end.scala");
    let rows = lex("end Point");
    assert!(rows[0].iter().any(|(t, s)| {
        t == "Point"
            && s.last()
                .is_some_and(|l| l == "entity.name.type.declaration")
    }));
}
