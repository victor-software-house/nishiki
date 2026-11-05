//! Each expected scope is Shiki 4.6.0's on the same text.

use indoc::indoc;

use crate::{Highlighter, Language, State};

/// One lexed line: each token's text and its innermost scope, or an empty
/// string where the token has only the document's scope.
type Row = Vec<(String, String)>;

/// The document's scope, which every token carries.
const SOURCE: &str = "text.html.php.blade";

fn lex(text: &str) -> Vec<Row> {
    let mut highlighter = Highlighter::new(Language::Blade);
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
    assert_eq!(Language::from_name("blade"), Some(Language::Blade));
}

#[test]
fn an_echo_holds_php() {
    let text = "{{ $a }}";
    assert_eq!(
        leaf(text, 0, "{{"),
        "support.function.construct.begin.blade"
    );
    assert_eq!(leaf(text, 0, "a"), "variable.other.php");
    assert_eq!(leaf(text, 0, "}}"), "support.function.construct.end.blade");
}

#[test]
fn a_raw_and_an_escaped_echo() {
    let text = indoc! {"
        {!! $b !!}
        @{{ x }}
    "};
    assert_eq!(leaf(text, 0, "!!}"), "support.function.construct.end.blade");
    assert_eq!(leaf(text, 1, "@"), "variable.other.index.php");
    assert_eq!(leaf(text, 1, "x"), "constant.other.php");
    assert_eq!(leaf(text, 1, "}}"), "end.bracket.round.blade");
}

#[test]
fn a_comment() {
    let text = "{{-- c --}}";
    assert_eq!(
        leaf(text, 0, "{{--"),
        "punctuation.definition.comment.begin.blade"
    );
    assert_eq!(leaf(text, 0, " c "), "comment.block.blade");
    assert_eq!(
        leaf(text, 0, "--}}"),
        "punctuation.definition.comment.end.blade"
    );
}

#[test]
fn a_directive_holds_php() {
    let text = "@section('t')";
    assert_eq!(leaf(text, 0, "@section"), "keyword.blade");
    assert_eq!(leaf(text, 0, "("), "begin.bracket.round.blade.php");
    assert_eq!(leaf(text, 0, "t"), "string.quoted.single.php");
    assert_eq!(leaf(text, 0, ")"), "end.bracket.round.blade.php");
}

#[test]
fn php_wins_a_tie_between_tags() {
    let text = "@if(@isset($x))";
    assert_eq!(leaf(text, 0, "@"), "keyword.operator.error-control.php");
    assert_eq!(leaf(text, 0, "isset"), "support.function.construct.php");
}

#[test]
fn an_echo_opens_inside_a_php_string() {
    let text = "{{ $a ?? \"x {{ y }} z\" }}";
    let row = &lex(text)[0];
    let inner: Vec<_> = row
        .iter()
        .filter(|(t, _)| t == "{{")
        .map(|(_, scope)| scope.as_str())
        .collect();
    assert_eq!(inner, ["support.function.construct.begin.blade"; 2]);
    assert_eq!(leaf(text, 0, " z"), "string.quoted.double.php");
}

#[test]
fn a_directive_opens_inside_a_php_string() {
    let text = "{{ \"a {$b} @c d\" }}";
    assert_eq!(leaf(text, 0, "@c"), "entity.name.function.blade");
}

#[test]
fn blade_leads_inside_a_tag() {
    let text = "<div class=\"{{ $a }}\" @click=\"x\">";
    assert_eq!(
        leaf(text, 0, "{{"),
        "support.function.construct.begin.blade"
    );
    assert_eq!(leaf(text, 0, "@click"), "entity.name.function.blade");
}

#[test]
fn a_php_section_spans_lines() {
    let text = indoc! {"
        @php
        $a = 1;
        @endphp
    "};
    assert_eq!(
        leaf(text, 0, "@php"),
        "punctuation.section.embedded.begin.php"
    );
    assert_eq!(leaf(text, 1, "a"), "variable.other.php");
    assert_eq!(
        leaf(text, 2, "@endphp"),
        "punctuation.section.embedded.end.php"
    );
}

#[cfg(feature = "json")]
mod probe {
    use crate::blade::engine::Blade;
    use crate::blade::grammar::Pat;
    use crate::test_support::probe::{answer_cases, lexer};

    /// Answers the cases of `oracle/onig.ts` with Blade's matchers (see
    /// [`answer_cases`]).
    #[test]
    #[ignore = "a tool: it needs a cases file"]
    fn matchers_answer_the_probe_cases() {
        answer_cases(&Pat::ALL, |arena, pat, case| {
            lexer::<Blade, _>(arena, case, |lexer| {
                lexer.probe_matcher(case.p, lexer.g, |lexer, p| lexer.matcher(pat, p))
            })
        });
    }
}
