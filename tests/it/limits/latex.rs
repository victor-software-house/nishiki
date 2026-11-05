//! Hostile lines for LaTeX under the default limits.
//!
//! A line that would cost more than the default work ends at the work limit,
//! with tokens that cover it, rather than running on.
//! Several matchers give back a run from its end, such as the groups of a
//! citation or of `\addplot`, so long runs of their pieces check that none
//! walks the rest of the line from each piece.

use nishiki::{Highlighter, Language, Limit};

use super::lex;

#[test]
fn hostile_latex_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Latex);
    for line in [
        "\\".repeat(12_000),
        "{".repeat(12_000),
        "[".repeat(12_000),
        "]".repeat(12_000),
        "$".repeat(12_000),
        "&".repeat(12_000),
        format!("\\{}", "a".repeat(12_000)),
        format!("\\begin{{{}", "a".repeat(12_000)),
        format!("\\section{}", "[a]".repeat(4_000)),
        format!("\\label{}", "[a]]".repeat(3_000)),
        format!("\\cite{}", "{a}".repeat(4_000)),
        format!("\\cite{}", "(a)[b]{c}".repeat(1_300)),
        format!("\\cite{}", "[a]".repeat(4_000)),
        format!("\\addplot{}", "[a]]".repeat(3_000)),
        format!("\\addplot{}", "[a]".repeat(4_000)),
        format!("\\pyinline{}", "[a]".repeat(4_000)),
        format!("\\verb|{}", "a".repeat(12_000)),
        format!("\\l_{}", "a_".repeat(6_000)),
        format!("\\q_{}", "_".repeat(12_000)),
        format!("\\begin{{CacheMeCode}}{{{}", " ".repeat(12_000)),
        format!("\\cacheMeCode[{}", "a".repeat(12_000)),
        format!("\\crefrange{{{}", "a".repeat(12_000)),
        format!("{}x", "\u{a0}".repeat(4_000)),
        " ".repeat(12_000),
    ] {
        let (rows, _) = lex(&mut highlighter, &[&line]);
        let report = rows[0].1;
        assert!(
            report
                .limits()
                .all(|limit| matches!(limit, Limit::Depth | Limit::Work)),
            "{:?} reached {:?}",
            line.chars().take(8).collect::<String>(),
            report.limits().collect::<Vec<_>>()
        );
    }
}

/// Each `\begin{x}` opens an environment that its `\end{x}` closes, so a
/// long run of them stops at the depth limit and each line still finishes.
#[test]
fn nested_environments_stop_at_the_depth_limit() {
    let mut highlighter = Highlighter::new(Language::Latex);
    let lines = vec!["\\begin{itemize}"; 5_000];
    let (rows, _) = lex(&mut highlighter, &lines);
    for (_, report) in rows {
        assert!(
            report
                .limits()
                .all(|limit| matches!(limit, Limit::Depth | Limit::Work)),
            "reached {:?}",
            report.limits().collect::<Vec<_>>()
        );
    }
}
