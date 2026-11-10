use indoc::indoc;

use crate::{Highlighter, Language, State};

/// One lexed line: each run of text and its scopes after the document's,
/// joined by spaces.
/// Neighbouring tokens with the same scopes are one run, as Shiki may split
/// them where nishiki does not.
type Row = Vec<(String, String)>;

/// The document's scope, which every token carries.
const SOURCE: &str = "source.r";

/// Lexes `text` line by line, and checks that the tokens cover each line.
fn lex(text: &str) -> Vec<Row> {
    let mut highlighter = Highlighter::new(Language::R);
    let mut state = State::default();
    let mut rows = Vec::new();
    for line in text.split('\n') {
        let mut tokens = Vec::new();
        let report = highlighter.line(line, &mut state, &mut tokens);
        assert!(report.is_complete(), "{line:?} reached a limit");
        let mut start = 0;
        let mut row: Row = Vec::new();
        for token in &tokens {
            let mut scopes = highlighter
                .scopes(token.stack())
                .map(ToString::to_string)
                .filter(|scope| scope != SOURCE)
                .collect::<Vec<_>>();
            scopes.reverse();
            let scopes = scopes.join(" ");
            let text = &line[start..token.end()];
            match row.last_mut() {
                Some(last) if last.1 == scopes => last.0.push_str(text),
                Some(_) | None => row.push((text.to_owned(), scopes)),
            }
            start = token.end();
        }
        assert_eq!(start, line.len(), "the tokens cover the line {line:?}");
        rows.push(row);
    }
    rows
}

/// A row from its runs' texts and scopes.
fn row(runs: &[(&str, &str)]) -> Row {
    runs.iter()
        .map(|&(t, s)| (t.to_owned(), s.to_owned()))
        .collect()
}

#[test]
fn the_name_is_known() {
    assert_eq!(Language::from_name("r"), Some(Language::R));
}

/// Each case is Shiki's answer.
/// The end repeats the start's dashes, so `)"` does not close it.
#[test]
fn a_raw_string_ends_at_its_dashes() {
    let rows = lex(r#"x <- r"-(a)"b)-""#);
    assert_eq!(
        rows[0],
        row(&[
            ("x", "variable.object"),
            (" ", ""),
            ("<-", "keyword.operator"),
            (" ", ""),
            (r#"r"-(a)"b)-""#, "string.quoted.other"),
        ])
    );
}

/// A fence that a roxygen comment opens holds the lines after it as plain
/// text, R code included, until its backticks come again.
#[test]
fn a_roxygen_fence_holds_the_lines_after_it() {
    let rows = lex(indoc! {"
        #' ```r
        x <- 1
        #' ```
        y
    "});
    assert_eq!(
        rows[0],
        row(&[
            ("#'", "comment.line.roxygen"),
            (" ```", "comment.line"),
            ("r", "entity.name.section"),
        ])
    );
    assert_eq!(rows[1], row(&[("x <- 1", "")]));
    assert_eq!(
        rows[2],
        row(&[("#'", "comment.line"), (" ", ""), ("```", "comment.line")])
    );
    assert_eq!(rows[3], row(&[("y", "variable.object")]));
}

/// The lines after `@examples` are R, until a line that does not start with
/// `#'`.
#[test]
fn an_examples_block_holds_r_code() {
    let rows = lex(indoc! {"
        #' @examples
        #' f(1)
        x
    "});
    assert_eq!(
        rows[1],
        row(&[
            ("#'", "comment.line"),
            (" ", ""),
            ("f", "meta.function-call entity.name.function"),
            ("(", "meta.bracket"),
            ("1", "constant.numeric"),
            (")", "meta.bracket"),
        ])
    );
    assert_eq!(rows[2], row(&[("x", "variable.object")]));
}

/// An indented `#'` line ends the block one blank before the `#`, and the
/// rest is an ordinary comment.
#[test]
fn an_indented_roxygen_line_ends_an_examples_block() {
    let rows = lex(indoc! {"
        #' @examples
          #' f(1)
    "});
    assert_eq!(rows[1], row(&[("  ", ""), ("#' f(1)", "comment.line")]));
}

/// A name may start with a dot, and names come before numbers.
#[test]
fn a_dot_and_digits_are_a_name() {
    let rows = lex(".5 + 1e");
    assert_eq!(
        rows[0],
        row(&[
            (".5", "variable.object"),
            (" ", ""),
            ("+", "keyword.operator"),
            (" ", ""),
            ("1e", "constant.numeric"),
        ])
    );
}

/// `return` is a keyword only before its arguments.
#[test]
fn a_builtin_is_a_keyword_before_a_parenthesis() {
    let rows = lex("return(x); return");
    assert_eq!(
        rows[0],
        row(&[
            ("return", "keyword.other"),
            ("(", "meta.bracket"),
            ("x", "meta.bracket variable.object"),
            (")", "meta.bracket"),
            ("; ", ""),
            ("return", "variable.object"),
        ])
    );
}

#[cfg(feature = "json")]
mod probe {
    use crate::r::engine::R;
    use crate::r::grammar::Pattern;
    use crate::test_support::probe::{answer_cases, lexer};

    /// Answers the cases of `oracle/onig.ts` with R's matchers (see
    /// [`answer_cases`]).
    #[test]
    #[ignore = "a tool: it needs a cases file"]
    fn matchers_answer_the_probe_cases() {
        answer_cases(&Pattern::ALL, |arena, pattern, case| {
            lexer::<R, _>(arena, case, |lexer| {
                lexer.probe_matcher(case.p, lexer.g, |lexer, p| lexer.matcher(pattern, p))
            })
        });
    }
}
