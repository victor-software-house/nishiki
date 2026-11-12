use indoc::indoc;

use crate::test_support::leaves::{LeafRow, leaf_in, merge_by_leaf};
use crate::{Language, test_support};

fn lex(text: &str) -> Vec<LeafRow> {
    merge_by_leaf(test_support::rows::lex(Language::Latex, text))
}

#[test]
fn the_name_is_known() {
    assert_eq!(Language::from_name("latex"), Some(Language::Latex));
}

#[test]
fn a_section_names_its_title() {
    let rows = lex(r"\section{Intro}");
    assert_eq!(
        leaf_in(&rows[0], "section"),
        "support.function.section.latex"
    );
    assert_eq!(leaf_in(&rows[0], "Intro"), "entity.name.section.latex");
}

#[test]
fn verb_text_runs_to_its_delimiter() {
    let rows = lex(r"\verb|a{b| c");
    assert_eq!(leaf_in(&rows[0], "a{b"), "markup.raw.verb.latex");
}

/// The TeX rules the grammar includes answer what LaTeX's own do not.
#[test]
fn tex_math_reads_inside_latex() {
    let rows = lex(r"$\alpha$");
    assert_eq!(leaf_in(&rows[0], "alpha"), "constant.character.math.tex");
}

#[test]
fn a_minted_environment_hands_its_body_to_the_language() {
    let rows = lex(indoc! {r"
        \begin{minted}{python}
        def f(): pass
        \end{minted}
        x
    "});
    assert!(
        leaf_in(&rows[1], "def").ends_with(".python"),
        "{:?}",
        rows[1]
    );
    assert!(
        !leaf_in(&rows[3], "x").ends_with(".python"),
        "{:?}",
        rows[3]
    );
}

#[test]
fn a_pycode_environment_ends_at_its_end() {
    let rows = lex(indoc! {r"
        \begin{pycode}
        import os
        \end{pycode}
    "});
    assert!(
        leaf_in(&rows[1], "import").ends_with(".python"),
        "{:?}",
        rows[1]
    );
    assert!(
        !leaf_in(&rows[2], "end").ends_with(".python"),
        "{:?}",
        rows[2]
    );
}

/// Shiki loads no gnuplot grammar with LaTeX, so the code stays plain.
#[test]
fn gnuplot_code_is_plain() {
    let rows = lex(indoc! {r"
        \begin{gnuplot}
        plot sin(x)
        \end{gnuplot}
    "});
    assert!(
        leaf_in(&rows[1], "plot sin(x)") == "text.tex.latex",
        "{:?}",
        rows[1]
    );
}

#[cfg(feature = "json")]
mod probe {
    use crate::latex::engine::Latex;
    use crate::latex::grammar::Rx;
    use crate::test_support::probe::{answer_cases, lexer};

    /// Answers the cases of `oracle/onig.ts` with LaTeX's own matchers (see
    /// [`answer_cases`]).
    /// TeX's probe answers the expressions of the TeX rules LaTeX includes.
    #[test]
    #[ignore = "a tool: it needs a cases file"]
    fn matchers_answer_the_probe_cases() {
        answer_cases(&Rx::ALL, |arena, rx, case| {
            lexer::<Latex, _>(arena, case, |lexer| {
                lexer.probe_matcher(case.p, lexer.g, |lexer, p| lexer.matcher(rx, p))
            })
        });
    }
}
