use indoc::indoc;

use crate::{Highlighter, Language, State};

/// One lexed line: each token's text and its innermost scope name.
type Row = Vec<(String, String)>;

fn lex_in(highlighter: &mut Highlighter, state: &mut State, line: &str) -> Row {
    let mut tokens = Vec::new();
    let report = highlighter.line(line, state, &mut tokens);
    assert!(report.is_complete(), "{line:?} reached a limit");
    let mut start = 0;
    let mut row = Row::new();
    for token in &tokens {
        let innermost = highlighter
            .scopes(token.stack())
            .next()
            .unwrap_or_default()
            .to_owned();
        let text = &line[start..token.end()];
        match row.last_mut() {
            Some((last, scope)) if *scope == innermost => last.push_str(text),
            _ => row.push((text.to_owned(), innermost)),
        }
        start = token.end();
    }
    assert_eq!(start, line.len(), "the tokens cover the line {line:?}");
    row
}

/// Lexes the lines of `text` in order from one start state.
fn lex_lines(text: &str) -> Vec<Row> {
    let mut highlighter = Highlighter::new(Language::Ruby);
    let mut state = State::default();
    text.lines()
        .map(|line| lex_in(&mut highlighter, &mut state, line))
        .collect()
}

fn lex(line: &str) -> Row {
    lex_lines(line).remove(0)
}

/// The innermost scope of the token that is exactly `text`.
fn scope_of(row: &Row, text: &str) -> String {
    row.iter()
        .find(|(token, _)| token == text)
        .unwrap_or_else(|| panic!("no token {text:?} in {row:?}"))
        .1
        .clone()
}

#[test]
fn an_empty_line_has_no_tokens() {
    assert_eq!(
        lex_in(
            &mut Highlighter::new(Language::Ruby),
            &mut State::default(),
            ""
        ),
        Row::new()
    );
}

#[test]
fn a_class_names_its_type() {
    let row = lex("class LotteryTicket");
    assert_eq!(scope_of(&row, "class"), "keyword.control.class.ruby");
    assert_eq!(
        scope_of(&row, "LotteryTicket"),
        "entity.name.type.class.ruby"
    );
}

/// A point that neither a blank nor a digit follows does not start a
/// fraction.
/// `1..25` is therefore two numbers and a range.
#[test]
fn a_range_of_numbers_is_two_numbers() {
    let row = lex("  NUMERIC_RANGE = 1..25");
    assert_eq!(
        scope_of(&row, "NUMERIC_RANGE"),
        "variable.other.constant.ruby"
    );
    assert_eq!(scope_of(&row, "1"), "constant.numeric.ruby");
    assert_eq!(scope_of(&row, ".."), "punctuation.separator.method.ruby");
    assert_eq!(scope_of(&row, "25"), "constant.numeric.ruby");
}

#[test]
fn a_string_embeds_its_interpolation() {
    let row = lex(r#"raise ArgumentError, "got #{name.inspect}""#);
    assert_eq!(
        scope_of(&row, "#{"),
        "punctuation.section.embedded.begin.ruby"
    );
    assert_eq!(scope_of(&row, "inspect"), "entity.name.function.ruby");
}

#[test]
fn a_word_array_repeats_its_delimiter() {
    let row = lex("ALLOWED_SCHEMES = %w(https http wss ws).freeze");
    assert_eq!(
        scope_of(&row, "%w("),
        "punctuation.section.array.begin.ruby"
    );
    assert_eq!(
        scope_of(&row, "https http wss ws"),
        "string.quoted.other.ruby"
    );
    assert_eq!(scope_of(&row, ")"), "punctuation.section.array.end.ruby");
}

#[test]
fn a_slash_after_a_match_operator_starts_a_regular_expression() {
    let row = lex(r"args.shift if meth && (map[meth] || meth !~ /^\-/)");
    assert_eq!(scope_of(&row, "!~"), "keyword.operator.comparison.ruby");
    assert_eq!(scope_of(&row, r"\-"), "constant.character.escape.ruby");
}

#[test]
fn a_yard_tag_names_its_parameter_and_types() {
    let row = lex("      # @param metadata_overrides [Hash] the hash");
    assert_eq!(
        scope_of(&row, "@"),
        "comment.line.keyword.punctuation.yard.ruby"
    );
    assert_eq!(scope_of(&row, "param"), "comment.line.keyword.yard.ruby");
    assert_eq!(
        scope_of(&row, "metadata_overrides"),
        "comment.line.parameter.yard.ruby"
    );
    assert_eq!(scope_of(&row, "["), "comment.line.punctuation.yard.ruby");
}

#[test]
fn a_heredoc_runs_to_the_line_of_its_name() {
    let rows = lex_lines(indoc! {"
        raise ArgumentError, <<~MSG.squish
          Passing string to define a callback is not supported.
        MSG
        end
    "});
    assert_eq!(scope_of(&rows[0], "<<~MSG"), "string.definition.begin.ruby");
    assert_eq!(scope_of(&rows[0], "."), "punctuation.separator.method.ruby");
    assert_eq!(
        scope_of(
            &rows[1],
            "  Passing string to define a callback is not supported."
        ),
        "string.unquoted.heredoc.ruby"
    );
    assert_eq!(scope_of(&rows[2], "MSG"), "string.definition.end.ruby");
    assert_eq!(scope_of(&rows[3], "end"), "keyword.control.ruby");
}

#[test]
fn an_embedded_document_runs_to_its_end() {
    let rows = lex_lines(indoc! {"
        =begin
        class Hidden
        =end
        class Shown
    "});
    assert_eq!(
        scope_of(&rows[1], "class Hidden"),
        "comment.block.documentation.ruby"
    );
    assert_eq!(scope_of(&rows[3], "Shown"), "entity.name.type.class.ruby");
}

#[test]
fn a_ruby_heredoc_lexes_its_lines_as_ruby() {
    let rows = lex_lines(indoc! {"
        module_eval <<~RUBY, __FILE__, __LINE__ + 1
          x = 1
        RUBY
        end
    "});
    assert_eq!(
        scope_of(&rows[0], "<<~RUBY"),
        "string.definition.begin.ruby"
    );
    assert_eq!(scope_of(&rows[0], "__FILE__"), "variable.language.ruby");
    assert_eq!(scope_of(&rows[1], "="), "keyword.operator.assignment.ruby");
    assert_eq!(scope_of(&rows[2], "RUBY"), "string.definition.end.ruby");
    assert_eq!(scope_of(&rows[3], "end"), "keyword.control.ruby");
}

/// In `expect(<<~RUBY)` the here document takes the `)`.
/// The call's rule therefore stays open after it ends, as in Shiki.
/// A long spec file holds one such rule per call.
/// The default limits keep 300 of them open.
#[test]
fn a_call_whose_paren_a_heredoc_takes_stays_open() {
    let calls = 300;
    let call = indoc! {"
        expect(<<~RUBY)
          x
        RUBY
    "};
    let text = format!("{}y", call.repeat(calls));
    let mut highlighter = Highlighter::new(Language::Ruby);
    let mut state = State::default();
    let mut tokens = Vec::new();
    for line in text.lines() {
        tokens.clear();
        let report = highlighter.line(line, &mut state, &mut tokens);
        assert!(report.is_complete(), "{line:?} reached a limit");
    }
    let last = tokens.last().expect("`y` has a token");
    let open = highlighter
        .scopes(last.stack())
        .filter(|&scope| scope == "meta.function-call.ruby")
        .count();
    assert_eq!(open, calls);
}

/// The engine keeps the indices of open `while` rules only when the flag is
/// set.
/// The flag must say whether any rule has a `while` pattern.
#[test]
fn the_while_flag_matches_the_rules() {
    use crate::ruby::engine::Ruby;
    use crate::ruby::grammar::Rule;
    use crate::rules::Grammar;

    let any = Rule::ALL.iter().any(|rule| rule.while_pat().is_some());
    assert_eq!(Rule::HAS_WHILE, any);
    assert_eq!(Ruby::WHILES, any);
}
