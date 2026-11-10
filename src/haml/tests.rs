use indoc::indoc;

use crate::{Highlighter, Language, State};

/// One lexed line: each token's text and its scope names, outermost first.
type Row = Vec<(String, Vec<String>)>;

fn lex(text: &str) -> Vec<Row> {
    lex_in(Language::Haml, text)
}

/// Lexes `text` in `language`, line by line, as [`lex`] does.
fn lex_in(language: Language, text: &str) -> Vec<Row> {
    let mut highlighter = Highlighter::new(language);
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

/// The scopes of the first token whose text is `token`.
fn scopes_of(text: &str, token: &str) -> Vec<String> {
    lex(text)
        .into_iter()
        .flatten()
        .find(|(t, _)| t == token)
        .unwrap_or_else(|| panic!("no token {token:?} in {text:?}"))
        .1
}

/// Whether the first token whose text is `token` has a scope that starts
/// with `prefix`.
fn has(text: &str, token: &str, prefix: &str) -> bool {
    scopes_of(text, token)
        .iter()
        .any(|scope| scope.starts_with(prefix))
}

#[test]
fn the_name_is_known() {
    assert_eq!(Language::from_name("haml"), Some(Language::Haml));
}

#[test]
fn a_tag_names_its_parts_and_ends_before_its_text() {
    let text = "%p.lead#intro Hello";
    assert!(has(text, "p", "entity.name.tag.haml"));
    assert!(has(text, ".lead", "entity.other.attribute-name.class"));
    assert!(has(text, "#intro", "entity.other.attribute-name.id"));
    let rows = lex(text);
    let hello = rows[0]
        .iter()
        .find(|(t, _)| t.contains("Hello"))
        .expect("the text");
    assert_eq!(hello.1, ["text.haml"]);
}

#[test]
fn a_ruby_line_hands_its_code_to_ruby_and_keeps_its_comment() {
    let text = "- if user.admin? # check";
    assert!(has(text, "if", "keyword.control"));
    assert!(has(text, "# check", "comment.line.number-sign.ruby"));
}

#[test]
fn an_attribute_hash_is_ruby_up_to_its_closing_brace() {
    let text = "%a{href: url, class: 'x'} link";
    assert!(has(text, "'", "string.quoted.single.ruby"));
    let rows = lex(text);
    let link = rows[0]
        .iter()
        .find(|(t, _)| t.contains("link"))
        .expect("the text");
    assert_eq!(link.1, ["text.haml"]);
}

#[test]
fn an_interpolation_closes_at_the_brace_that_matches_its_own() {
    let rows = lex("== a #{ {b: 1}[:b] } c");
    let closes: Vec<&Vec<String>> = rows[0]
        .iter()
        .filter(|(t, _)| t == "}")
        .map(|(_, scopes)| scopes)
        .collect();
    assert_eq!(closes.len(), 2);
    assert!(
        closes[0]
            .iter()
            .any(|s| s == "punctuation.section.scope.ruby")
    );
    assert!(
        closes[1]
            .iter()
            .any(|s| s == "punctuation.section.embedded.ruby")
    );
    let tail = rows[0].last().expect("the text after the interpolation");
    assert_eq!(tail.1, ["text.haml", "string.quoted.double.ruby"]);
}

#[test]
fn a_ruby_filter_ends_at_the_first_line_indented_no_deeper() {
    let rows = lex(indoc! {"
        :ruby
          x = 1
        %p done
    "});
    assert!(rows[1].iter().any(|(_, scopes)| {
        scopes
            .iter()
            .any(|s| s == "source.ruby.embedded.filter.haml")
    }));
    assert!(rows[2].iter().all(|(_, scopes)| {
        !scopes
            .iter()
            .any(|s| s == "source.ruby.embedded.filter.haml")
    }));
}

#[test]
fn a_filter_reads_a_mixed_indent_exactly() {
    let rows = lex(&[":javascript", "\t  var a = 1;", " \tvar b = 2;", ""].join("\n"));
    assert!(
        rows[1]
            .iter()
            .any(|(_, scopes)| scopes.iter().any(|s| s == "js.haml"))
    );
    assert!(
        rows[2]
            .iter()
            .any(|(_, scopes)| scopes.iter().any(|s| s == "js.haml"))
    );
    let deeper = lex(&["\t:javascript", "\t var a = 1;", " \tvar b = 2;", ""].join("\n"));
    assert!(
        deeper[1]
            .iter()
            .any(|(_, scopes)| scopes.iter().any(|s| s == "js.haml"))
    );
    assert!(
        deeper[2]
            .iter()
            .all(|(_, scopes)| !scopes.iter().any(|s| s == "js.haml"))
    );
}

#[test]
fn a_sass_filter_that_ends_its_line_scopes_only_that_line() {
    let rows = lex(indoc! {"
        :sass
          .a
            color: red
    "});
    assert!(rows[0].iter().any(|(_, scopes)| {
        scopes
            .iter()
            .any(|s| s == "source.sass.embedded.filter.haml")
    }));
    assert!(rows[1].iter().all(|(_, scopes)| {
        !scopes
            .iter()
            .any(|s| s == "source.sass.embedded.filter.haml")
    }));
}

#[test]
fn a_comment_block_lexes_haml_inside_it() {
    let rows = lex(indoc! {"
        -#
          %p inner
        %p outer
    "});
    assert!(
        rows[1]
            .iter()
            .any(|(_, scopes)| scopes.iter().any(|s| s == "comment.block.haml"))
    );
    assert!(
        rows[1]
            .iter()
            .any(|(_, scopes)| scopes.iter().any(|s| s == "entity.name.tag.haml"))
    );
    assert!(
        rows[2]
            .iter()
            .all(|(_, scopes)| !scopes.iter().any(|s| s == "comment.block.haml"))
    );
}

#[test]
fn a_scss_php_or_plain_line_opens_no_filter() {
    for filter in [":scss", ":php", ":plain"] {
        let rows = lex(&[filter, "  .e { f: g; }"].join("\n"));
        assert!(rows[0].iter().all(|(_, scopes)| scopes == &["text.haml"]));
        let class = rows[1].iter().find(|(t, _)| t == ".e").expect("the class");
        assert!(
            class
                .1
                .iter()
                .any(|s| s == "entity.other.attribute-name.class")
        );
    }
}

/// Whether the first token whose text is `p` in `row` names a tag.
fn names_a_tag(row: &Row) -> bool {
    row.iter()
        .find(|(t, _)| t == "p")
        .is_some_and(|(_, scopes)| scopes.iter().any(|s| s == "entity.name.tag.haml"))
}

/// A Ruby line closes at its line's end after its own patterns take the
/// rest of the line: a comment, or a block opener (`|`, `do`).
/// It closes too after Ruby's `.` or `::`, which takes the line feed.
/// The next line is HAML again.
#[test]
fn a_ruby_line_closes_after_a_comment_a_pipe_or_do() {
    for ending in ["- a # c", "- b, |", "- c do", "- d do # e", "- .", "- a::"] {
        let rows = lex(&[ending, "%p x"].join("\n"));
        assert!(names_a_tag(&rows[1]), "{ending:?}: {:?}", rows[1]);
    }
}

/// The same holds in a Ruby here document that embeds HAML.
#[test]
fn a_ruby_line_in_a_ruby_here_document_closes_the_same_way() {
    let rows = lex_in(
        Language::Ruby,
        indoc! {"
            x = <<~HAML
              - a # c
              %p one
              - b, |
              %p two
              - c do
              %p three
              - .
              %p four
              - a::
              %p five
            HAML
        "},
    );
    for row in [2, 4, 6, 8, 10] {
        assert!(names_a_tag(&rows[row]), "line {row}: {:?}", rows[row]);
    }
}

#[cfg(feature = "json")]
mod probe {
    use crate::haml::engine::Haml;
    use crate::haml::grammar::Pattern;
    use crate::test_support::probe::{answer_cases, lexer};

    /// Answers the cases of `oracle/onig.ts` with HAML's matchers
    /// (see [`answer_cases`]).
    #[test]
    #[ignore = "a tool: it needs a cases file"]
    fn matchers_answer_the_probe_cases() {
        answer_cases(&Pattern::ALL, |arena, pattern, case| {
            lexer::<Haml, _>(arena, case, |lexer| {
                lexer.probe_matcher(case.p, lexer.g, |lexer, p| lexer.matcher(pattern, p))
            })
        });
    }
}
