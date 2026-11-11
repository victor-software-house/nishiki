use crate::Language;
use crate::test_support::{self, LeafRow, leaf_in, merge_by_leaf};

fn lex(line: &str) -> LeafRow {
    merge_by_leaf(test_support::lex(Language::Matlab, line)).remove(0)
}

#[test]
fn the_name_is_known() {
    assert_eq!(Language::from_name("matlab"), Some(Language::Matlab));
}

#[test]
fn a_quote_after_a_name_is_a_transpose() {
    let row = lex("x = a';");
    assert_eq!(leaf_in(&row, "'"), "keyword.operator.transpose.matlab");
}

#[test]
fn a_quote_after_a_blank_opens_a_string() {
    let row = lex("y = [a ' str'];");
    assert_eq!(leaf_in(&row, " str"), "string.quoted.single.matlab");
}

#[test]
fn a_command_names_its_call_and_comment() {
    let row = lex("hold on % c");
    assert_eq!(leaf_in(&row, "hold"), "variable.other.command.matlab");
    assert_eq!(leaf_in(&row, "% c"), "comment.line.percentage.matlab");
}

/// The indexing braces end at the last of two blanks, before the line feed.
/// The next line is then a command again.
#[test]
fn indexing_braces_end_at_the_last_trailing_blank() {
    let text = ["a {", "y ,  ", "format on"].join("\n");
    let rows = merge_by_leaf(test_support::lex(Language::Matlab, &text));
    assert_eq!(leaf_in(&rows[2], "format"), "variable.other.command.matlab");
}

#[test]
fn a_function_names_its_outputs() {
    let row = lex("function [a, b] = f(x)");
    assert_eq!(leaf_in(&row, "b"), "variable.parameter.output.matlab");
    assert_eq!(leaf_in(&row, "f"), "entity.name.function.matlab");
}

#[cfg(feature = "json")]
mod probe {
    use crate::matlab::engine::Matlab;
    use crate::matlab::grammar::Rx;
    use crate::test_support::probe::{answer_cases, lexer};

    /// Answers the cases of `oracle/onig.ts` with MATLAB's matchers (see
    /// [`answer_cases`]).
    #[test]
    #[ignore = "a tool: it needs a cases file"]
    fn matchers_answer_the_probe_cases() {
        answer_cases(&Rx::ALL, |arena, rx, case| {
            lexer::<Matlab, _>(arena, case, |lexer| {
                lexer.probe_matcher(case.p, lexer.g, |lexer, p| lexer.matcher(rx, p))
            })
        });
    }
}
