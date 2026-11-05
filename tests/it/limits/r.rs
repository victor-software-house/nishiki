//! Hostile lines for R under the default limits.
//!
//! A line that would cost more than the default work ends at the work limit,
//! with tokens that cover it, rather than running on.

use nishiki::{Highlighter, Language, Limit};

use super::lex;

#[test]
fn hostile_r_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::R);
    for line in [
        "(".repeat(12_000),
        "{".repeat(12_000),
        "[".repeat(12_000),
        "f(".repeat(6_000),
        "\"".repeat(12_000),
        "'".repeat(12_000),
        "`".repeat(12_000),
        format!("r\"{}(", "-".repeat(12_000)),
        format!("r\"{}(x)-\"", "-".repeat(12_000)),
        "r\"(".repeat(4_000),
        "%".repeat(12_000),
        "%in% ".repeat(2_400),
        "\\".repeat(12_000),
        "\\\\x".repeat(4_000),
        format!("#' {}", "_a ".repeat(4_000)),
        format!("#' {}", "*a ".repeat(4_000)),
        format!("#' {}", "__a ".repeat(3_000)),
        format!("#' {}", "**a ".repeat(3_000)),
        format!("#' {}", "[a::".repeat(3_000)),
        format!("#' {}", "`a\\".repeat(3_000)),
        format!("#' {}", "<a".repeat(6_000)),
        format!("#' @param {}", "a,".repeat(6_000)),
        format!("#' {}", "@".repeat(12_000)),
        format!("#' @examplesIf {}", "f(".repeat(4_000)),
        "é".repeat(6_000),
        "x\u{301}".repeat(4_000),
        "pkg::".repeat(2_400),
        format!("{}=", "a ".repeat(6_000)),
        format!("function({}", "a,".repeat(6_000)),
        format!("{}#", " ".repeat(12_000)),
        "1e".repeat(6_000),
        "0x".repeat(6_000),
        "x".repeat(12_000),
    ] {
        let (rows, _) = lex(&mut highlighter, &[&line]);
        let report = rows[0].1;
        assert!(
            report
                .limits()
                .all(|limit| matches!(limit, Limit::Depth | Limit::Work)),
            "{:?} reached {:?}",
            line.chars().take(4).collect::<String>(),
            report.limits().collect::<Vec<_>>()
        );
    }
}

/// A fence that a roxygen comment never closes holds every line after it.
#[test]
fn an_unclosed_roxygen_fence_runs_on_across_many_lines_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::R);
    let filler = "x <- f(a, b) + 1";
    let mut lines = vec!["#' ```r"];
    lines.extend(std::iter::repeat_n(filler, 2_000));
    let (rows, _) = lex(&mut highlighter, &lines);
    assert!(rows.iter().all(|(_, report)| report.is_complete()));
}
