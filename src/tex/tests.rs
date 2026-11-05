use crate::Language;
use crate::test_support::{self, LeafRow, leaf_in, merge_by_leaf};

fn lex(line: &str) -> LeafRow {
    merge_by_leaf(test_support::lex(Language::Tex, line)).remove(0)
}

#[test]
fn the_name_is_known() {
    assert_eq!(Language::from_name("tex"), Some(Language::Tex));
}

#[test]
fn a_control_word_is_a_keyword() {
    let row = lex(r"\ifx\a\b \else \fi");
    assert_eq!(leaf_in(&row, "else"), "keyword.control.tex");
}

#[test]
fn math_closes_with_the_dollars_that_opened_it() {
    let row = lex(r"$$a$$ c");
    assert_eq!(leaf_in(&row, "a"), "support.class.math.block.tex");
    assert_eq!(leaf_in(&row, " c"), "text.tex");
}

#[test]
fn a_math_symbol_is_a_constant() {
    let row = lex(r"$\alpha + \leq$");
    assert_eq!(leaf_in(&row, "alpha"), "constant.character.math.tex");
}

/// The grammar's operator list spells `\bigoplus` as `\bogoplus`, and Shiki
/// keeps the typo.
#[test]
fn the_operator_typo_is_a_math_constant() {
    let row = lex(r"$\bogoplus x$");
    assert_eq!(leaf_in(&row, "bogoplus"), "constant.character.math.tex");
}

/// `%:?` matches at the `%` of a magic comment before its own rule can.
#[test]
fn a_magic_comment_is_an_ordinary_comment() {
    let row = lex("%!TEX root = main.tex");
    assert_eq!(
        leaf_in(&row, "!TEX root = main.tex"),
        "comment.line.percentage.tex"
    );
}

/// `\Sexpr{…}` hands its text to R, which gives the line back at the `}`.
#[cfg(feature = "lang-r")]
#[test]
fn sexpr_text_is_r() {
    let row = lex(r"$\Sexpr{mean(x)} y$");
    assert_eq!(leaf_in(&row, "mean"), "entity.name.function");
    assert_eq!(leaf_in(&row, " y"), "support.class.math.block.tex");
}

#[test]
fn a_comment_runs_to_the_line_end() {
    let row = lex("x % note");
    assert_eq!(leaf_in(&row, " note"), "comment.line.percentage.tex");
}

#[cfg(feature = "json")]
mod probe {
    use crate::test_support::probe::{answer_cases, lexer};
    use crate::tex::engine::Tex;
    use crate::tex::grammar::Rx;

    /// Answers the cases of `oracle/onig.ts` with TeX's matchers (see
    /// [`answer_cases`]).
    #[test]
    #[ignore = "a tool: it needs a cases file"]
    fn matchers_answer_the_probe_cases() {
        answer_cases(&Rx::ALL, |arena, rx, case| {
            lexer::<Tex, _>(arena, case, |lexer| {
                lexer.probe_matcher(case.p, lexer.g, |lexer, p| lexer.tex_matcher(rx, p))
            })
        });
    }
}
