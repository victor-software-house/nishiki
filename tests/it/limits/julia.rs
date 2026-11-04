//! Hostile lines and fences for Julia under the default limits.
//!
//! A line that would cost more than the default work ends at the work limit,
//! with tokens that cover it, rather than running on.

use nishiki::{Highlighter, Language, Limit, Limits, Work};

use super::lex;

#[test]
fn hostile_julia_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Julia);
    for line in [
        "\"".repeat(12_000),
        "\"\"\"".repeat(4_000),
        "$(".repeat(6_000),
        "#=".repeat(6_000),
        "(".repeat(12_000),
        "[".repeat(12_000),
        "{".repeat(12_000),
        "'".repeat(12_000),
        "r\"".repeat(6_000),
        "a.".repeat(6_000),
        "@a ".repeat(4_000),
        "py\"\"\"".repeat(2_000),
        "sql\"\"\"".repeat(2_000),
        format!("function f({}", "a::T, ".repeat(2_000)),
        format!("{}#", " ".repeat(12_000)),
        format!("{}x", "\u{a0}".repeat(4_000)),
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

#[test]
fn a_markdown_fence_runs_its_julia_lines_under_the_same_limits() {
    let limits = Limits {
        work: Some(Work {
            per_byte: 0,
            base: 64,
        }),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Markdown, limits);
    let text = "f(x) = [y^2 for y in x if y > 0]; ".repeat(100);
    let (rows, _) = lex(&mut highlighter, &["```julia", &text, "```"]);
    assert!(rows[1].1.reached(Limit::Work));
    assert!(rows[2].1.is_complete());
}
