//! Each expected scope is Shiki 4.6.0's on the same text.

use indoc::indoc;

use crate::{Highlighter, Language, State};

/// One lexed line: each token's text and its innermost scope, or an empty
/// string where the token has only the document's scope.
type Row = Vec<(String, String)>;

/// The document's scope, which every token carries.
const SOURCE: &str = "source.hack";

fn lex(text: &str) -> Vec<Row> {
    let mut highlighter = Highlighter::new(Language::Hack);
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

/// The innermost scope of the first token on line `row` whose text is
/// `token`.
fn leaf(text: &str, row: usize, token: &str) -> String {
    lex(text)[row]
        .iter()
        .find(|(t, _)| t == token)
        .unwrap_or_else(|| panic!("no token {token:?} on line {row} of {text:?}"))
        .1
        .clone()
}

#[test]
fn the_name_is_known() {
    assert_eq!(Language::from_name("hack"), Some(Language::Hack));
}

#[test]
fn the_opening_tag_reads_as_an_operator_and_a_constant() {
    let text = "<?hh";
    assert_eq!(leaf(text, 0, "<"), "keyword.operator.comparison.php");
    assert_eq!(leaf(text, 0, "hh"), "constant.other.php");
}

#[test]
fn a_function_and_its_parameters() {
    let text = "function f(int $x): void {";
    assert_eq!(leaf(text, 0, "function"), "storage.type.function.php");
    assert_eq!(leaf(text, 0, "f"), "entity.name.function.php");
    assert_eq!(leaf(text, 0, "int"), "support.type.php");
    assert_eq!(leaf(text, 0, "x"), "variable.other.php");
}

#[test]
fn a_string_that_starts_with_a_sql_keyword_is_sql() {
    let text = "$q = \"SELECT a FROM t\";";
    assert_eq!(leaf(text, 0, "SELECT"), "keyword.other.DML.sql");
    assert_eq!(leaf(text, 0, "FROM"), "keyword.other.DML.sql");
}

#[test]
fn xhp_in_an_expression() {
    let text = "$e = <div class=\"a\">{$x}</div>;";
    assert_eq!(leaf(text, 0, "div"), "entity.name.tag.open.xhp");
    assert_eq!(leaf(text, 0, "class"), "entity.other.attribute-name.xhp");
}

#[test]
fn html_at_the_root_and_hack_between_tags() {
    let text = indoc! {"
        <div>html</div>
        <!-- c -->
    "};
    assert_eq!(leaf(text, 0, "<"), "punctuation.definition.tag.begin.html");
    assert_eq!(leaf(text, 0, "html"), "constant.other.php");
    assert_eq!(leaf(text, 1, "<!--"), "punctuation.definition.comment.html");
}

#[cfg(feature = "json")]
mod probe {
    use crate::hack::engine::Hack;
    use crate::hack::grammar::Pattern;
    use crate::test_support::probe::{answer_cases, lexer};

    /// Answers the cases of `oracle/onig.ts` with Hack's matchers (see
    /// [`answer_cases`]).
    #[test]
    #[ignore = "a tool: it needs a cases file"]
    fn matchers_answer_the_probe_cases() {
        answer_cases(&Pattern::ALL, |arena, pattern, case| {
            lexer::<Hack, _>(arena, case, |lexer| {
                lexer.probe_matcher(case.p, lexer.g, |lexer, p| lexer.matcher(pattern, p))
            })
        });
    }
}
