use indoc::indoc;

use crate::{Highlighter, Language, State};

/// One lexed line: each token's text and its innermost scope, or an empty
/// string where the token has only the document's scope.
type Row = Vec<(String, String)>;

/// The document's scope, which every token carries.
const SOURCE: &str = "source.php";

fn lex(text: &str) -> Vec<Row> {
    let mut highlighter = Highlighter::new(Language::Php);
    let mut state = State::default();
    let mut rows = Vec::new();
    for line in text.split('\n') {
        let mut tokens = Vec::new();
        let report = highlighter.line(line, &mut state, &mut tokens);
        assert!(report.is_complete(), "{line:?} reached a limit");
        let mut start = 0;
        let mut row = Vec::new();
        for token in &tokens {
            let leaf = highlighter
                .scopes(token.stack())
                .map(ToString::to_string)
                .find(|scope| scope != SOURCE)
                .unwrap_or_default();
            row.push((line[start..token.end()].to_owned(), leaf));
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
}

#[test]
fn the_name_is_known() {
    assert_eq!(Language::from_name("php"), Some(Language::Php));
}

#[test]
fn a_word_list_names_the_list_that_holds_the_word_first() {
    let text = indoc! {r"
        json_encode($a);
        \PHP_EOL;
        new \ArrayIterator();
    "};
    assert_eq!(leaf(text, "json_encode"), "support.function.json.php");
    assert_eq!(leaf(text, "PHP_EOL"), "support.constant.core.php");
    assert_eq!(leaf(text, "ArrayIterator"), "support.class.builtin.php");
}

#[test]
fn a_heredoc_named_for_a_language_hands_its_text_over_up_to_its_label() {
    let text = indoc! {r"
        $a = <<<SQL
        SELECT 1 FROM t WHERE x = {$b}
        SQL;
    "};
    let rows = lex(text);
    assert!(
        rows[1]
            .iter()
            .any(|(t, s)| t == "SELECT" && s.contains(".sql")),
        "{:?}",
        rows[1]
    );
    assert!(
        rows[1]
            .iter()
            .any(|(t, s)| t == "b" && s == "variable.other.php"),
        "{:?}",
        rows[1]
    );
    assert_eq!(rows[2][0].0, "SQL");
}

#[cfg(feature = "json")]
mod probe {
    use crate::php::engine::Php;
    use crate::php::grammar::Pat;
    use crate::test_support::probe::{answer_cases, lexer};

    /// Answers the cases of `oracle/onig.ts` with PHP's matchers (see
    /// [`answer_cases`]).
    #[test]
    #[ignore = "a tool: it needs a cases file"]
    fn matchers_answer_the_probe_cases() {
        answer_cases(&Pat::ALL, |arena, pat, case| {
            lexer::<Php, _>(arena, case, |lexer| {
                lexer.probe_matcher(case.p, lexer.g, |lexer, p| lexer.matcher(pat, p))
            })
        });
    }
}
