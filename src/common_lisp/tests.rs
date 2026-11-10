use indoc::indoc;

use crate::{Highlighter, Language, State};

/// One lexed line: each token's text and its scope names, outermost first.
type Row = Vec<(String, Vec<String>)>;

fn lex(text: &str) -> Vec<Row> {
    let mut highlighter = Highlighter::new(Language::CommonLisp);
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

/// The innermost scope of the token that holds byte `at` of `text`'s first
/// line.
fn scope_at(text: &str, at: usize) -> String {
    let mut start = 0;
    for (token, mut scopes) in lex(text).swap_remove(0) {
        start += token.len();
        if at < start {
            return scopes.pop().unwrap_or_default();
        }
    }
    panic!("no byte {at} in {text:?}")
}

#[test]
fn the_names_are_known() {
    assert_eq!(
        Language::from_name("common-lisp"),
        Some(Language::CommonLisp)
    );
    assert_eq!(Language::from_name("lisp"), Some(Language::CommonLisp));
}

#[test]
fn words_match_whole_runs_without_regard_to_case() {
    assert_eq!(
        leaf("(CAR x)", "CAR"),
        "support.function.accessor.commonlisp"
    );
    assert_eq!(scope_at("(car-x y)", 1), "source.commonlisp");
    assert_eq!(scope_at("x-car y", 2), "source.commonlisp");
    assert_eq!(
        scope_at("(\u{17f}etq x)", 1),
        "storage.type.function.specialform.commonlisp"
    );
    assert_eq!(
        leaf("(cla\u{df} x)", "cla\u{df}"),
        "support.class.commonlisp"
    );
    assert_eq!(leaf("#'car", "car"), "support.function.accessor.commonlisp");
}

#[test]
fn a_list_name_gives_back_its_last_character() {
    let rows = lex("(defun (xyz) x)");
    let texts: Vec<&str> = rows[0].iter().map(|(text, _)| text.as_str()).collect();
    assert_eq!(texts, ["(", "defun", " (", "xy", "z", ") x)"]);
    assert_eq!(
        scope_at("(defun (setf foo) x)", 13),
        "entity.name.function.commonlisp"
    );
    assert_eq!(
        scope_at("(defun (setf foo) x)", 8),
        "storage.type.function.defname.commonlisp"
    );
}

#[test]
fn a_comma_gives_back_its_dot_when_no_name_follows_it() {
    let text = "(defun ,. nil)";
    assert_eq!(
        scope_at(text, 1),
        "storage.type.function.defname.commonlisp"
    );
    assert_eq!(
        scope_at(text, 7),
        "variable.other.constant.defname.commonlisp"
    );
    assert_eq!(scope_at(text, 8), "entity.name.function.commonlisp");
    assert_eq!(
        scope_at("(defun ,@x)", 9),
        "entity.name.function.commonlisp"
    );
    assert_eq!(
        scope_at("(defun ,@x)", 8),
        "variable.other.constant.defname.commonlisp"
    );
}

#[test]
fn a_package_prefix_is_its_own_token_in_a_defined_name() {
    let text = "(defun pkg:name ()";
    assert_eq!(scope_at(text, 7), "support.type.package.commonlisp");
    assert_eq!(scope_at(text, 10), "entity.name.function.commonlisp");
    assert_eq!(scope_at(text, 11), "entity.name.function.commonlisp");
}

#[test]
fn a_format_parameter_quote_takes_the_string_s_closing_quote() {
    let rows = lex("(format t \"~'\"a\" b)");
    assert_eq!(
        rows[0]
            .iter()
            .find(|(text, _)| text == "a")
            .and_then(|(_, scopes)| scopes.last())
            .map(String::as_str),
        Some("entity.name.variable.commonlisp")
    );
}

#[test]
fn block_comments_nest_across_lines() {
    let rows = lex(indoc! {"
        #| a #| b |# c
        d |# (car x)
    "});
    assert_eq!(
        rows[1][0].1.last().map(String::as_str),
        Some("comment.block.commonlisp")
    );
    assert_eq!(
        leaf("#| a #| b |# c |# (car x)", "car"),
        "support.function.accessor.commonlisp"
    );
}

#[test]
fn a_string_runs_on_to_the_next_line() {
    let rows = lex(indoc! {"
        (print \"open ~
        still~a\")
    "});
    assert_eq!(
        rows[1][0].1.last().map(String::as_str),
        Some("string.quoted.double.commonlisp")
    );
}

#[cfg(feature = "json")]
mod probe {
    use crate::common_lisp::engine::CommonLisp;
    use crate::common_lisp::grammar::Pattern;
    use crate::test_support::probe::{answer_cases, lexer};

    /// Answers the cases of `oracle/onig.ts` with Common Lisp's matchers
    /// (see [`answer_cases`]).
    #[test]
    #[ignore = "a tool: it needs a cases file"]
    fn matchers_answer_the_probe_cases() {
        answer_cases(&Pattern::ALL, |arena, pattern, case| {
            lexer::<CommonLisp, _>(arena, case, |lexer| {
                lexer.probe_matcher(case.p, lexer.g, |lexer, p| lexer.matcher(pattern, p))
            })
        });
    }
}
