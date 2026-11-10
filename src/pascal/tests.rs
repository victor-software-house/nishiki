use indoc::indoc;

use crate::{Highlighter, Language, State};

/// One lexed line: each token's text and its scope names, outermost first.
type Row = Vec<(String, Vec<String>)>;

fn lex(text: &str) -> Vec<Row> {
    let mut highlighter = Highlighter::new(Language::Pascal);
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
    assert_eq!(Language::from_name("pascal"), Some(Language::Pascal));
}

#[test]
fn the_keyword_list_shadows_the_later_word_patterns() {
    let text = "property Name: string read FName write SetName; result := self; f: file";
    assert_eq!(leaf(text, "property"), "storage.type.function.pascal");
    assert_eq!(leaf(text, "Name"), "entity.name.function.pascal");
    assert_eq!(leaf(text, "read"), "keyword.pascal");
    assert_eq!(leaf(text, "write"), "keyword.pascal");
    assert_eq!(leaf(text, "result"), "keyword.pascal");
    assert_eq!(leaf(text, "self"), "token.variable");
    assert_eq!(leaf(text, "file"), "keyword.pascal");
    assert_eq!(leaf(text, "string"), "storage.support.type.pascal");
}

#[test]
fn words_match_without_regard_to_case_and_lazy_optionals_match_both_forms() {
    let text = "X AS Y; A ASC; EXPORTS e; Requires r; BEGIN End";
    for word in ["AS", "ASC", "EXPORTS", "Requires", "BEGIN", "End"] {
        assert_eq!(leaf(text, word), "keyword.pascal", "{word}");
    }
    assert_eq!(leaf("x := ſelf", "ſelf"), "token.variable");
}

#[test]
fn a_prototype_needs_a_directive_after_its_semicolon() {
    let rows = lex(indoc! {"
        procedure Foo(a: Integer; b: (x)); forward;
        function Bar: Integer; external 'lib';
        function Baz(a: Integer): Integer;
    "});
    assert_eq!(rows[0][0].0, "procedure");
    assert_eq!(
        rows[0][0].1.last().map(String::as_str),
        Some("storage.type.prototype.pascal")
    );
    assert_eq!(
        leaf("procedure Foo(a: Integer; b: (x)); forward;", "Foo"),
        "entity.name.function.prototype.pascal"
    );
    assert_eq!(
        leaf("function Bar: Integer; external 'lib';", "Bar"),
        "entity.name.function.pascal"
    );
    assert_eq!(
        leaf("function Baz(a: Integer): Integer;", "Baz"),
        "entity.name.function.pascal"
    );
    assert_eq!(
        leaf("function TFoo.Bar; forward;", "TFoo.Bar"),
        "entity.name.function.prototype.pascal"
    );
}

#[test]
fn numbers_follow_the_grammar_s_alternation() {
    assert_eq!(leaf("x := 12ab", "12"), "constant.numeric.pascal");
    assert_eq!(leaf("x := y1.5", "1.5"), "constant.numeric.pascal");
    assert_eq!(leaf("x := 2.5e-3", "2"), "constant.numeric.pascal");
    assert_eq!(leaf("x := 2.5e-3", ".5e-3"), "constant.numeric.pascal");
    assert_eq!(leaf("x := $FF", "$FF"), "constant.numeric.hex.pascal");
    assert_eq!(leaf("c := #13#10", "#13"), "string.other.pascal");
}

#[test]
fn comments_end_at_their_first_closer_and_do_not_nest() {
    let rows = lex(indoc! {"
        { a (* b } c
        (* d { e *) f
        {$IFDEF X} g
          // h
    "});
    assert_eq!(rows[0][3].0, " c");
    assert_eq!(
        rows[0][1].1.last().map(String::as_str),
        Some("comment.block.pascal.two")
    );
    assert_eq!(rows[1][3].0, " f");
    assert_eq!(
        rows[2][0].1.last().map(String::as_str),
        Some("string.regexp")
    );
    assert_eq!(rows[3][0].0, "  ");
    assert_eq!(
        rows[3][0].1.last().map(String::as_str),
        Some("punctuation.whitespace.comment.leading.pascal")
    );
}

#[test]
fn a_doubled_quote_ends_the_string_and_starts_another() {
    let rows = lex("s := 'it''s'");
    let quotes: Vec<&str> = rows[0]
        .iter()
        .filter(|(text, _)| text == "'")
        .filter_map(|(_, scopes)| scopes.last().map(String::as_str))
        .collect();
    assert_eq!(
        quotes,
        [
            "punctuation.definition.string.begin.pascal",
            "punctuation.definition.string.end.pascal",
            "punctuation.definition.string.begin.pascal",
            "punctuation.definition.string.end.pascal",
        ]
    );
}

#[test]
fn an_open_string_and_comments_run_on_to_the_next_line() {
    let rows = lex(indoc! {"
        s := 'open
        still string' x
        { open
        still comment } y
    "});
    assert_eq!(
        rows[1][0].1.last().map(String::as_str),
        Some("string.quoted.single.pascal")
    );
    assert_eq!(
        rows[3][0].1.last().map(String::as_str),
        Some("comment.block.pascal.two")
    );
}

#[cfg(feature = "json")]
mod probe {
    use crate::pascal::engine::Pascal;
    use crate::pascal::grammar::Pattern;
    use crate::test_support::probe::{answer_cases, lexer};

    /// Answers the cases of `oracle/onig.ts` with Pascal's matchers
    /// (see [`answer_cases`]).
    #[test]
    #[ignore = "a tool: it needs a cases file"]
    fn matchers_answer_the_probe_cases() {
        answer_cases(&Pattern::ALL, |arena, pattern, case| {
            lexer::<Pascal, _>(arena, case, |lexer| {
                lexer.probe_matcher(case.p, lexer.g, |lexer, p| lexer.matcher(pattern, p))
            })
        });
    }
}
