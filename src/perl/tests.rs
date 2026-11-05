use indoc::indoc;

use crate::{Highlighter, Language, State};

/// One lexed line: each token's text and its scopes after the document's,
/// joined by spaces.
type Row = Vec<(String, String)>;

/// The document's scope, which every token carries.
const SOURCE: &str = "source.perl";

/// Lexes `text` line by line, and checks that the tokens cover each line.
fn lex(text: &str) -> Vec<Row> {
    let mut highlighter = Highlighter::new(Language::Perl);
    let mut state = State::default();
    let mut rows = Vec::new();
    for line in text.split('\n') {
        let mut tokens = Vec::new();
        let report = highlighter.line(line, &mut state, &mut tokens);
        assert!(report.is_complete(), "{line:?} reached a limit");
        let mut start = 0;
        let mut row = Vec::new();
        for token in &tokens {
            let mut scopes = highlighter
                .scopes(token.stack())
                .map(ToString::to_string)
                .filter(|scope| scope != SOURCE)
                .collect::<Vec<_>>();
            scopes.reverse();
            let scopes = scopes.join(" ");
            row.push((line[start..token.end()].to_owned(), scopes));
            start = token.end();
        }
        assert_eq!(start, line.len(), "the tokens cover the line {line:?}");
        rows.push(row);
    }
    rows
}

/// A row from its tokens' texts and scopes.
fn row(tokens: &[(&str, &str)]) -> Row {
    tokens
        .iter()
        .map(|&(t, s)| (t.to_owned(), s.to_owned()))
        .collect()
}

#[test]
fn the_name_is_known() {
    assert_eq!(Language::from_name("perl"), Some(Language::Perl));
}

/// Each case is Shiki's answer.
#[test]
fn a_sub_names_the_blank_before_its_brace() {
    let rows = lex("sub foo {");
    assert_eq!(
        rows[0],
        row(&[
            ("sub", "meta.function.perl storage.type.sub.perl"),
            (" ", "meta.function.perl"),
            ("foo", "meta.function.perl entity.name.function.perl"),
            (" ", "meta.function.perl"),
            ("{", ""),
        ])
    );
}

/// The `~` lets the closing label stand indented, and the indentation keeps
/// only the document's scope.
#[test]
fn an_indented_heredoc_closes_at_its_indented_label() {
    let rows = lex(indoc! {r"
        $x = <<~EOT;
          hi $y
          EOT
        1;
    "});
    assert_eq!(
        rows[2],
        row(&[
            ("  ", ""),
            (
                "EOT",
                "string.unquoted.heredoc.interpolated.perl punctuation.definition.string.end.perl"
            ),
        ])
    );
    assert_eq!(rows[3][0].1, "");
}

#[test]
fn a_heredoc_tagged_html_hands_each_line_to_html() {
    let rows = lex(indoc! {r"
        print <<HTML;
        <b>x</b>
        HTML
    "});
    let html = "meta.embedded.block.html string.unquoted.heredoc.interpolated.perl text.html.basic";
    assert_eq!(
        rows[1][1],
        (
            "b".to_owned(),
            format!("{html} meta.tag.inline.b.start.html entity.name.tag.html")
        )
    );
    assert_eq!(
        rows[2],
        row(&[(
            "HTML",
            "meta.embedded.block.html string.unquoted.heredoc.interpolated.perl punctuation.definition.string.end.perl"
        )])
    );
}

/// The rest of an opener's line lexes again as Perl, where a second opener
/// takes no lines.
#[test]
fn only_the_first_heredoc_of_a_line_takes_lines() {
    let rows = lex(indoc! {r"
        print <<A, <<B;
        a
        A
        b
        B
    "});
    assert_eq!(
        rows[1],
        row(&[("a", "string.unquoted.heredoc.interpolated.perl")])
    );
    assert_eq!(rows[3], row(&[("b", "")]));
    assert_eq!(rows[4], row(&[("B", "")]));
}

#[test]
fn the_data_section_is_a_comment_to_the_end() {
    let rows = lex(indoc! {r"
        __END__
        foo bar
        my $x;
    "});
    assert_eq!(rows[0], row(&[("__END__", "constant.language.perl")]));
    assert_eq!(
        rows[2],
        row(&[("my $x;", "comment.block.documentation.perl")])
    );
}

#[cfg(feature = "json")]
mod probe {
    use crate::perl::engine::Perl;
    use crate::perl::grammar::Pat;
    use crate::test_support::probe::{answer_cases, lexer};

    /// Answers the cases of `oracle/onig.ts` with Perl's matchers (see
    /// [`answer_cases`]).
    #[test]
    #[ignore = "a tool: it needs a cases file"]
    fn matchers_answer_the_probe_cases() {
        answer_cases(&Pat::ALL, |arena, pat, case| {
            lexer::<Perl, _>(arena, case, |lexer| {
                lexer.probe_matcher(case.p, lexer.g, |lexer, p| lexer.matcher(pat, p))
            })
        });
    }
}
