//! Hostile lines for html-derivative, under the default limits.

use nishiki::{Highlighter, Language, Limit};

use super::lex;

#[test]
fn hostile_html_derivative_lines_finish_within_the_default_limits() {
    for line in [
        "<A".repeat(150_000),
        "<A ".repeat(100_000),
        "<A/>".repeat(75_000),
        "</A >".repeat(60_000),
        "<a:b".repeat(75_000),
        "<a:b<".repeat(60_000),
        "<A b=\"".repeat(40_000),
        format!("<{}", "A".repeat(300_000)),
        format!("<A{}", "/".repeat(300_000)),
        format!("<A {}", "b=c ".repeat(75_000)),
        "<".repeat(300_000),
        "x".repeat(300_000),
    ] {
        let mut highlighter = Highlighter::new(Language::HtmlDerivative);
        let (rows, _) = lex(&mut highlighter, &[&line]);
        let report = rows[0].1;
        assert!(
            report.limits().all(|limit| limit == Limit::Depth),
            "{:?}... reached {:?}",
            line.chars().take(24).collect::<String>(),
            report.limits().collect::<Vec<_>>()
        );
    }
}
