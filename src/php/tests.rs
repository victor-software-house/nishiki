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

/// U+00A0 is both a name character and \s, so a type gives back only its
/// last one for the \s+ before the variable.
#[test]
fn a_type_gives_back_its_last_unicode_space_for_the_variable() {
    let text = "function f(Q\u{a0}$h, S\u{a0}\u{a0}$j) {}";
    assert_eq!(leaf(text, "Q"), "support.class.php");
    assert_eq!(
        leaf(text, "\u{a0}"),
        "meta.function.parameter.typehinted.php"
    );
    assert_eq!(leaf(text, "S\u{a0}"), "support.class.php");
}

/// A space past ASCII is both a name character and \s, so a \s* or a \s+
/// before a name gives its last such space back when no name starts after
/// it, as Oniguruma backtracks.
#[test]
fn white_space_gives_its_last_unicode_space_to_a_name() {
    let lines = [
        "x;",
        "\u{3000}\u{3000}:",
        "\u{a0}\u{a0}:",
        "A::\u{a0}();",
        "$a->\u{3000}();",
        "class\u{a0}\u{a0}{}",
        "goto\u{3000}\u{3000};",
    ];
    let rows = lex(&lines.join("\n"));
    let expected: [(usize, &[(&str, &str)]); 6] = [
        (
            1,
            &[
                ("\u{3000}", ""),
                ("\u{3000}", "entity.name.goto-label.php"),
                (":", ""),
            ],
        ),
        (
            2,
            &[
                ("\u{a0}", ""),
                ("\u{a0}", "entity.name.goto-label.php"),
                (":", ""),
            ],
        ),
        (
            3,
            &[
                ("A", "support.class.php"),
                ("::", "keyword.operator.class.php"),
                ("\u{a0}", "entity.name.function.php"),
                (
                    "(",
                    "punctuation.definition.arguments.begin.bracket.round.php",
                ),
                (
                    ")",
                    "punctuation.definition.arguments.end.bracket.round.php",
                ),
                (";", "punctuation.terminator.expression.php"),
            ],
        ),
        (
            4,
            &[
                ("$", "punctuation.definition.variable.php"),
                ("a", "variable.other.php"),
                ("->", "keyword.operator.class.php"),
                ("\u{3000}", "entity.name.function.php"),
                (
                    "(",
                    "punctuation.definition.arguments.begin.bracket.round.php",
                ),
                (
                    ")",
                    "punctuation.definition.arguments.end.bracket.round.php",
                ),
                (";", "punctuation.terminator.expression.php"),
            ],
        ),
        (
            5,
            &[
                ("class", "storage.type.class.php"),
                ("\u{a0}", "meta.class.php"),
                ("\u{a0}", "entity.name.type.class.php"),
                ("{", "punctuation.definition.class.begin.bracket.curly.php"),
                ("}", "punctuation.definition.class.end.bracket.curly.php"),
            ],
        ),
        (
            6,
            &[
                ("goto", "keyword.control.goto.php"),
                ("\u{3000}", ""),
                ("\u{3000}", "support.other.php"),
                (";", "punctuation.terminator.expression.php"),
            ],
        ),
    ];
    for (row, tokens) in expected {
        let got: Vec<(&str, &str)> = rows[row]
            .iter()
            .map(|(text, leaf)| (text.as_str(), leaf.as_str()))
            .collect();
        assert_eq!(got, tokens, "line {row}");
    }
}

/// A \s* or a \s+ before a name gives back each space until a name starts,
/// past an ASCII space at the end of the run.
#[test]
fn white_space_gives_back_spaces_until_a_name_starts() {
    let lines = [
        "x;",
        "\u{a0} :",
        "A::\u{a0} ();",
        "$a->\u{3000} ();",
        "goto\u{a0} ;",
        "class\u{a0} {}",
    ];
    let rows = lex(&lines.join("\n"));
    let expected: [(usize, &[(&str, &str)]); 5] = [
        (1, &[("\u{a0}", "entity.name.goto-label.php"), (" :", "")]),
        (
            2,
            &[
                ("A", "support.class.php"),
                ("::", "keyword.operator.class.php"),
                ("\u{a0}", "entity.name.function.php"),
                (" ", "meta.method-call.static.php"),
                (
                    "(",
                    "punctuation.definition.arguments.begin.bracket.round.php",
                ),
                (
                    ")",
                    "punctuation.definition.arguments.end.bracket.round.php",
                ),
                (";", "punctuation.terminator.expression.php"),
            ],
        ),
        (
            3,
            &[
                ("$", "punctuation.definition.variable.php"),
                ("a", "variable.other.php"),
                ("->", "keyword.operator.class.php"),
                ("\u{3000}", "entity.name.function.php"),
                (" ", "meta.method-call.php"),
                (
                    "(",
                    "punctuation.definition.arguments.begin.bracket.round.php",
                ),
                (
                    ")",
                    "punctuation.definition.arguments.end.bracket.round.php",
                ),
                (";", "punctuation.terminator.expression.php"),
            ],
        ),
        (
            4,
            &[
                ("goto\u{a0}", "constant.other.php"),
                (" ", ""),
                (";", "punctuation.terminator.expression.php"),
            ],
        ),
        (
            5,
            &[
                ("class", "storage.type.php"),
                ("\u{a0}", "constant.other.php"),
                (" ", ""),
                ("{", "punctuation.definition.begin.bracket.curly.php"),
                ("}", "punctuation.definition.end.bracket.curly.php"),
            ],
        ),
    ];
    for (row, tokens) in expected {
        let got: Vec<(&str, &str)> = rows[row]
            .iter()
            .map(|(text, leaf)| (text.as_str(), leaf.as_str()))
            .collect();
        assert_eq!(got, tokens, "line {row}");
    }
}

#[test]
fn a_heredoc_label_after_unicode_white_space_closes_it() {
    let lines = [
        "$x = <<<EOT",
        "hi",
        "\u{a0}EOT;",
        "$y = <<<'N'",
        "no",
        "\u{3000}N;",
        "$q = <<<SQL",
        "SELECT 1",
        "\u{a0}SQL;",
    ];
    let rows = lex(&lines.join("\n"));
    let labels = [
        (2, "EOT", "heredoc"),
        (5, "N", "nowdoc"),
        (8, "SQL", "heredoc"),
    ];
    for (row, label, kind) in labels {
        let closer = [
            (label.to_owned(), format!("keyword.operator.{kind}.php")),
            (
                ";".to_owned(),
                "punctuation.terminator.expression.php".to_owned(),
            ),
        ];
        assert_eq!(rows[row][1..], closer, "row {row}");
    }
}

#[cfg(feature = "json")]
mod probe {
    use crate::php::engine::Php;
    use crate::php::grammar::Pattern;
    use crate::test_support::probe::{answer_cases, lexer};

    /// Answers the cases of `oracle/onig.ts` with PHP's matchers (see
    /// [`answer_cases`]).
    #[test]
    #[ignore = "a tool: it needs a cases file"]
    fn matchers_answer_the_probe_cases() {
        answer_cases(&Pattern::ALL, |arena, pattern, case| {
            lexer::<Php, _>(arena, case, |lexer| {
                lexer.probe_matcher(case.p, lexer.g, |lexer, p| lexer.matcher(pattern, p))
            })
        });
    }
}
