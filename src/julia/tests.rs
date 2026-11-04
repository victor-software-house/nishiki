use indoc::indoc;

use crate::{Highlighter, Language, State};

/// One lexed line: each token's text and its scope names, outermost first.
type Row = Vec<(String, Vec<String>)>;

fn lex(text: &str) -> Vec<Row> {
    let mut highlighter = Highlighter::new(Language::Julia);
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

/// The scopes of the first token whose text is `token`, without the
/// document's `source.julia`.
fn scopes(text: &str, token: &str) -> Vec<String> {
    let mut scopes = lex(text)
        .into_iter()
        .flatten()
        .find(|(t, _)| t == token)
        .unwrap_or_else(|| panic!("no token {token:?} in {text:?}"))
        .1;
    scopes.retain(|scope| scope != "source.julia");
    scopes
}

/// The innermost scope of the first token whose text is `token`.
fn leaf(text: &str, token: &str) -> String {
    scopes(text, token).pop().unwrap_or_default()
}

#[test]
fn the_names_are_known() {
    for name in ["julia", "jl"] {
        assert_eq!(Language::from_name(name), Some(Language::Julia));
    }
}

#[test]
fn a_function_head_names_its_function_and_types() {
    let text = "function Base.show(io::IO, x::Vector{T}) where T";
    assert_eq!(leaf(text, "function"), "keyword.other.julia");
    assert_eq!(leaf(text, "."), "keyword.operator.dots.julia");
    assert_eq!(leaf(text, "show"), "entity.name.function.julia");
    assert_eq!(leaf(text, "::"), "keyword.operator.relation.types.julia");
    assert_eq!(leaf(text, "Vector{T}"), "support.type.julia");
    assert_eq!(leaf(text, "where"), "keyword.other.julia");
}

#[test]
fn an_assignment_defines_a_function() {
    let text = "f(x) = x^2";
    assert_eq!(leaf(text, "f"), "entity.name.function.julia");
    assert_eq!(leaf(text, "="), "keyword.operator.update.julia");
    assert_eq!(leaf(text, "^"), "keyword.operator.arithmetic.julia");
    assert_eq!(leaf(text, "2"), "constant.numeric.julia");
}

#[test]
fn a_string_interpolates_names_and_expressions() {
    let text = r#"s = "a $b $(c+1) \n""#;
    assert_eq!(leaf(text, "$b"), "variable.interpolation.julia");
    assert_eq!(
        scopes(text, "+"),
        [
            "string.quoted.double.julia",
            "variable.interpolation.julia",
            "keyword.operator.arithmetic.julia"
        ]
    );
    assert_eq!(leaf(text, r"\n"), "constant.character.escape.julia");
}

#[test]
fn a_python_macro_is_python_up_to_its_end() {
    let text = r#"py"""x = 1 + $y""""#;
    assert_eq!(
        scopes(text, "1"),
        [
            "embed.python.julia",
            "meta.embedded.inline.python",
            "constant.numeric.dec.python"
        ]
    );
    assert_eq!(leaf(text, "$"), "invalid.illegal.operator.python");
    assert_eq!(scopes(text, "y"), ["embed.python.julia"]);
}

#[test]
fn an_sql_macro_goes_back_to_julia_at_an_interpolation() {
    let text = r#"sql"""SELECT a FROM t WHERE b = $c""""#;
    assert_eq!(leaf(text, "SELECT"), "keyword.other.DML.sql");
    assert_eq!(
        scopes(text, "$c"),
        [
            "embed.sql.julia",
            "meta.embedded.inline.sql",
            "variable.interpolation.julia"
        ]
    );
    let row = lex(text).remove(0);
    assert_eq!(
        row.last()
            .and_then(|(_, scopes)| scopes.last())
            .map(String::as_str),
        Some("punctuation.definition.string.end.julia")
    );
}

#[test]
fn an_embedded_language_keeps_its_state_across_lines() {
    let rows = lex(indoc! {r#"
        js"""
        /* a
        b */ let x = $y
        """
    "#});
    let comment = &rows[1][0];
    assert_eq!(comment.0, "/*");
    assert!(
        comment.1.iter().any(|s| s == "comment.block.js"),
        "{comment:?}"
    );
    let name = rows[2].iter().find(|(t, _)| t == "$y").expect("$y");
    assert_eq!(
        name.1.last().map(String::as_str),
        Some("variable.other.readwrite.js")
    );
    assert_eq!(
        rows[3][0].1.last().map(String::as_str),
        Some("punctuation.definition.string.end.julia")
    );
}

#[test]
fn cxx_and_r_macros_stay_plain() {
    assert_eq!(
        scopes(r#"cxx"""int x;""""#, "int x;"),
        ["embed.cxx.julia", "meta.embedded.inline.cpp"]
    );
    assert_eq!(
        scopes(r#"R"""x <- 1""""#, "x <- 1"),
        ["embed.R.julia", "meta.embedded.inline.r"]
    );
}

#[test]
fn numbers_take_their_suffixes() {
    let text = "n = 0x1F + 1_000.5e-3im + Inf32 + 2π";
    for number in ["0x1F", "1_000.5e-3im", "Inf32", "2π"] {
        assert_eq!(leaf(text, number), "constant.numeric.julia", "{number}");
    }
}

#[test]
fn a_for_head_runs_to_a_line_that_ends_without_a_comma() {
    let rows = lex(indoc! {"
        for i in 1:10,
            j in a
          α′ = ∇f'
        end
    "});
    let range = rows[0].iter().find(|(t, _)| t == ":").expect("a colon");
    assert_eq!(
        range.1.last().map(String::as_str),
        Some("keyword.operator.range.julia")
    );
    let prime = rows[2].iter().find(|(t, _)| t == "'").expect("a prime");
    assert_eq!(
        prime.1.last().map(String::as_str),
        Some("keyword.operator.transposed-variable.julia")
    );
    assert_eq!(rows[3][0].0, "end");
}

#[test]
fn a_comment_does_not_end_a_for_head() {
    let rows = lex(indoc! {"
        for i in a # c
        outer
    "});
    assert_eq!(
        rows[1][0].1.last().map(String::as_str),
        Some("keyword.other.julia")
    );
}

#[test]
fn macros_symbols_and_regexes_are_named() {
    let text = r#"@time @. x :sym r"a+"i"#;
    assert_eq!(leaf(text, "@time"), "support.function.macro.julia");
    assert_eq!(leaf(text, "@."), "support.function.macro.julia");
    assert_eq!(leaf(text, ":sym"), "constant.other.symbol.julia");
    assert_eq!(leaf(text, "i"), "keyword.other.option-toggle.regexp.julia");
}

#[cfg(feature = "json")]
mod probe {
    use crate::julia::engine::Julia;
    use crate::julia::grammar::Pat;
    use crate::test_support::probe::{answer_cases, lexer};

    /// Answers the cases of `oracle/onig.ts` with Julia's matchers (see
    /// [`answer_cases`]).
    #[test]
    #[ignore = "a tool: it needs a cases file"]
    fn matchers_answer_the_probe_cases() {
        answer_cases(&Pat::ALL, |arena, pat, case| {
            lexer::<Julia, _>(arena, case, |lexer| {
                lexer.probe_matcher(case.p, lexer.g, |lexer, p| lexer.matcher(pat, p))
            })
        });
    }
}
