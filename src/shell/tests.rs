use indoc::indoc;

use crate::{Highlighter, Language, State};

/// One lexed line: each token's text and its scope names, outermost first.
type Row = Vec<(String, Vec<String>)>;

/// Lexes `text`, line by line, and returns the tokens of each line and the
/// state after each.
fn lex_with_states(text: &str) -> (Vec<Row>, Vec<State>) {
    let mut highlighter = Highlighter::new(Language::Shell);
    let mut state = State::default();
    let (mut rows, mut states) = (Vec::new(), Vec::new());
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
        states.push(state);
    }
    (rows, states)
}

fn lex(text: &str) -> Vec<Row> {
    lex_with_states(text).0
}

/// The scopes of the first token whose text is `token`, outermost first.
fn scopes_of(text: &str, token: &str) -> Vec<String> {
    lex(text)
        .into_iter()
        .flatten()
        .find(|(t, _)| t == token)
        .unwrap_or_else(|| panic!("no token {token:?} in {text:?}"))
        .1
}

#[test]
fn a_command_name_and_an_argument() {
    let rows = lex("echo \"$HOME\"");
    let texts: Vec<&str> = rows[0].iter().map(|(t, _)| t.as_str()).collect();
    assert_eq!(texts, ["echo", " ", "\"", "$", "HOME", "\""]);
    assert_eq!(
        scopes_of("echo hi", "echo"),
        [
            "source.shell",
            "meta.statement.shell",
            "meta.statement.command.shell",
            "meta.statement.command.name.shell",
            "entity.name.function.call.shell",
            "entity.name.command.shell",
            "support.function.builtin.shell",
        ]
    );
}

#[test]
fn a_here_document_spans_lines_and_names_its_delimiter() {
    let text = indoc! {"
        cat <<EOF
        body $x
        EOF
        echo done
    "};
    let (rows, states) = lex_with_states(text);
    assert_ne!(states[0], State::default(), "open after the first line");
    assert_ne!(states[1], State::default(), "open after the body");
    assert_eq!(states[2], State::default(), "closed by the delimiter");
    let body = &rows[1][0];
    assert_eq!(body.0, "body ");
    assert_eq!(
        body.1.last().map(String::as_str),
        Some("string.unquoted.heredoc.no-indent.EOF")
    );
    assert_eq!(
        rows[2][0].1.last().map(String::as_str),
        Some("punctuation.definition.string.heredoc.delimiter.shell")
    );
}

#[test]
fn a_quoted_indented_here_document_ends_at_a_tabbed_delimiter() {
    let text = "cat <<-'END'\n\traw $x\n\tEND\necho a";
    let (rows, states) = lex_with_states(text);
    assert_eq!(rows[1][0].0, "\traw $x");
    assert_eq!(
        rows[1][0].1.last().map(String::as_str),
        Some("string.quoted.heredoc.indent.END")
    );
    assert_eq!(
        rows[2][0].1.last().map(String::as_str),
        Some("punctuation.definition.string.heredoc.\tEND.shell")
    );
    assert_eq!(states[2], State::default());
}

#[test]
fn leading_dots_of_a_captured_name_are_dropped() {
    let rows = lex("cat <<.x\nbody\n.x");
    assert_eq!(
        rows[1][0].1.last().map(String::as_str),
        Some("string.unquoted.heredoc.no-indent.x")
    );
}

#[test]
fn an_unterminated_string_keeps_later_lines() {
    let (rows, states) = lex_with_states("echo \"open\nstill if then fi");
    assert_ne!(states[0], State::default());
    assert_eq!(rows[1].len(), 1, "one token inside the string");
    assert_eq!(
        rows[1][0].1.last().map(String::as_str),
        Some("string.quoted.double.shell")
    );
}

#[test]
fn an_empty_line_leaves_the_state_alone() {
    // A statement left open at the end of a line stays open over blank lines,
    // as it does in Shiki, so `*a*` is a command name and not a glob.
    let (rows, _) = lex_with_states("x;\n\n*a*)");
    assert_eq!(rows[1], Vec::new());
    assert_eq!(
        rows[2][0].1.last().map(String::as_str),
        Some("entity.name.command.shell")
    );
}

#[test]
fn an_argument_gives_back_a_character_before_a_redirection() {
    let rows = lex("echo ab>c");
    let texts: Vec<&str> = rows[0].iter().map(|(t, _)| t.as_str()).collect();
    assert_eq!(texts, ["echo", " ", "a", "b", ">", "c"]);
}

#[test]
fn a_symbol_is_not_a_word_character() {
    // `✓` ends the variable name inside `${…}` as the `\w` of the grammar's
    // regular expressions does not match it.
    let scopes = scopes_of("echo \"${GRE✓ x}\"", "GRE");
    assert_eq!(
        scopes.last().map(String::as_str),
        Some("variable.other.normal.shell")
    );
}

#[test]
fn a_long_delimiter_is_a_here_document() {
    let delimiter = "x".repeat(200);
    let (_, states) = lex_with_states(&format!("cat <<'{delimiter}'\nbody"));
    assert_ne!(states[0], State::default());
    assert_ne!(states[1], State::default());
    let (_, states) = lex_with_states(&format!("cat <<'{delimiter}'\n{delimiter}"));
    assert_eq!(states[1], State::default());
}

/// Each line lexed again from its state gives the same tokens and state.
/// The lines run backwards first, so that each starts from a state the last
/// line did not end in and the open rules are rebuilt from the arena, and
/// then forwards, so that each resumes the rules the line before left.
#[test]
fn a_state_resumes_the_line_after_it() {
    let text = indoc! {"
        if [ -n \"$a\" ]; then
          cat <<EOF
          $b
        EOF
        fi
    "};
    let mut highlighter = Highlighter::new(Language::Shell);
    let mut state = State::default();
    let mut lines = Vec::new();
    for line in text.split('\n') {
        let mut tokens = Vec::new();
        let before = state;
        let _ = highlighter.line(line, &mut state, &mut tokens);
        lines.push((line, before, tokens, state));
    }
    for (line, before, tokens, after) in lines.iter().rev().chain(&lines) {
        let mut again = *before;
        let mut second = Vec::new();
        let _ = highlighter.line(line, &mut again, &mut second);
        assert_eq!(&second, tokens, "{line:?}");
        assert_eq!(again, *after, "{line:?}");
    }
}

#[test]
fn a_line_of_blanks_is_linear() {
    let line = " ".repeat(200_000);
    let (rows, _) = lex_with_states(&line);
    assert_eq!(rows[0].len(), 1);
}

#[test]
fn lazy_expressions_are_found_once() {
    let line = "case ".repeat(20_000);
    let (rows, _) = lex_with_states(&line);
    assert_eq!(rows.len(), 1);
}

#[test]
fn a_rule_opened_in_a_captured_word_keeps_the_words_scopes() {
    // The first line opens a double-quoted string under an argument; the
    // second opens one inside the here-string's word, which has the same
    // state but another scope stack.
    let rows = lex("echo \"a\"\necho <<<x\"b");
    let quote = &rows[1][rows[1].len() - 2];
    assert_eq!(quote.0, "\"");
    assert!(
        quote
            .1
            .contains(&"string.unquoted.herestring.shell".to_owned()),
        "{quote:?}"
    );
}

#[test]
fn an_alias_takes_any_number_of_options() {
    let text = format!("alias {}name=1", "-a ".repeat(20));
    assert_eq!(
        scopes_of(&text, "alias")[2],
        "storage.type.alias.shell",
        "{text}"
    );
}
