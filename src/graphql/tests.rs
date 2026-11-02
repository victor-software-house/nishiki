use crate::{Highlighter, Language, Limits, State, Work};

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
        &mut Highlighter::new(Language::GraphQl),
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

const SELECTION: &str = "meta.selectionset.graphql";
const ARGUMENTS: &str = "meta.arguments.graphql";
const ENUM_BODY: &str = "meta.type.object.graphql";

#[test]
fn an_empty_line_has_no_tokens() {
    assert_eq!(lex(""), Row::new());
}

#[test]
fn a_string_after_white_space_is_a_string_and_otherwise_a_comment() {
    assert_eq!(
        lex("query { a(b: \"x\", c:\"y\") }"),
        row(&[
            ("query", "keyword.operation.graphql"),
            (" ", SELECTION),
            ("{", "punctuation.operation.graphql"),
            (" ", SELECTION),
            ("a", "variable.graphql"),
            ("(", "meta.brace.round.directive.graphql"),
            ("b", "variable.parameter.graphql"),
            (":", "punctuation.colon.graphql"),
            (" ", ARGUMENTS),
            ("\"", "punctuation.definition.string.begin.graphql"),
            ("x", "string.quoted.double.graphql"),
            ("\"", "punctuation.definition.string.end.graphql"),
            (",", "punctuation.comma.graphql"),
            (" ", ARGUMENTS),
            ("c", "variable.parameter.graphql"),
            (":", "punctuation.colon.graphql"),
            ("\"", "punctuation.whitespace.comment.leading.graphql"),
            ("y\"", "comment.line.graphql.js"),
            (")", "meta.brace.round.directive.graphql"),
            (" ", SELECTION),
            ("}", "punctuation.operation.graphql"),
        ])
    );
}

#[test]
fn an_enum_value_keeps_the_white_space_before_it() {
    let mut highlighter = Highlighter::new(Language::GraphQl);
    let mut state = State::default();
    let enum_value = "constant.character.enum.graphql";
    assert_eq!(
        lex_in(&mut highlighter, &mut state, "enum E { A"),
        row(&[
            ("enum", "keyword.enum.graphql"),
            (" ", "meta.enum.graphql"),
            ("E", "support.type.enum.graphql"),
            (" ", ENUM_BODY),
            ("{", "punctuation.operation.graphql"),
            (" A", enum_value),
        ])
    );
    assert_eq!(
        lex_in(&mut highlighter, &mut state, " B }"),
        row(&[
            (" B", enum_value),
            (" ", ENUM_BODY),
            ("}", "punctuation.operation.graphql"),
        ])
    );
}

#[test]
fn a_match_that_does_not_advance_closes_the_rule_it_sits_in() {
    assert_eq!(
        lex("type A x"),
        row(&[
            ("type", "keyword.type.graphql"),
            (" ", "meta.type.interface.graphql"),
            ("A", "support.type.graphql"),
            (" ", "meta.type.interface.graphql"),
            ("x", "source.graphql"),
        ])
    );
}

#[test]
fn a_string_with_no_closing_quote_runs_on_to_the_next_line() {
    let mut highlighter = Highlighter::new(Language::GraphQl);
    let mut state = State::default();
    lex_in(&mut highlighter, &mut state, "{ a(b: \"x");
    assert_eq!(
        lex_in(&mut highlighter, &mut state, "y # z"),
        row(&[("y # z", "string.quoted.double.graphql")])
    );
}

#[test]
fn an_interpolation_is_javascript_up_to_its_closing_brace() {
    assert_eq!(
        lex("{ ${a.b(1)} c }"),
        row(&[
            ("{", "punctuation.operation.graphql"),
            (" ", "native.interpolation"),
            ("${", "keyword.other.substitution.begin"),
            ("a", "variable.other.object.js"),
            (".", "punctuation.accessor.js"),
            ("b", "entity.name.function.js"),
            ("(", "meta.brace.round.js"),
            ("1", "constant.numeric.decimal.js"),
            (")", "meta.brace.round.js"),
            ("}", "keyword.other.substitution.end"),
            (" ", SELECTION),
            ("c", "variable.graphql"),
            (" ", SELECTION),
            ("}", "punctuation.operation.graphql"),
        ])
    );
}

#[test]
fn an_interpolation_goes_on_across_lines_until_its_closing_brace() {
    let mut highlighter = Highlighter::new(Language::GraphQl);
    let mut state = State::default();
    lex_in(&mut highlighter, &mut state, "{ ${ f(");
    let middle = lex_in(&mut highlighter, &mut state, "  1) + {");
    assert!(
        middle.iter().all(|(_, scope)| scope != SELECTION),
        "{middle:?}"
    );
    let last = lex_in(&mut highlighter, &mut state, "  a: 2 }.a } c }");
    assert!(last.contains(&("}".to_owned(), "keyword.other.substitution.end".to_owned())));
    assert!(last.contains(&("c".to_owned(), "variable.graphql".to_owned())));
}

#[test]
fn a_depth_limit_keeps_a_rule_closed() {
    let limits = Limits {
        depth: Some(2),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::GraphQl, limits);
    let mut state = State::default();
    let mut tokens = Vec::new();
    let report = highlighter.line("{ a { b { c } } }", &mut state, &mut tokens);
    assert!(report.reached(crate::Limit::Depth));
    assert_eq!(tokens.last().map(|token| token.end()), Some(17));
}

#[test]
fn a_spent_work_budget_ends_the_line() {
    let limits = Limits {
        work: Some(Work {
            per_byte: 0,
            base: 8,
        }),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::GraphQl, limits);
    let mut state = State::default();
    let mut tokens = Vec::new();
    let line = "{ a ".repeat(50);
    let report = highlighter.line(&line, &mut state, &mut tokens);
    assert!(report.reached(crate::Limit::Work));
    assert_eq!(tokens.last().map(|token| token.end()), Some(line.len()));
}
