//! Hostile lines for Svelte under the default limits.
//!
//! A line that would cost more than the default work ends at the work limit,
//! with tokens that cover it, rather than running on.

use nishiki::{Highlighter, Language, Limit};

use super::lex;

#[test]
fn hostile_svelte_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Svelte);
    for line in [
        "<".repeat(12_000),
        "{".repeat(12_000),
        "}".repeat(12_000),
        "{#if ".repeat(3_000),
        "{:else if ".repeat(1_500),
        "{@const ".repeat(2_000),
        "<!--".repeat(4_000),
        "-->".repeat(4_000),
        "\"".repeat(12_000),
        "'".repeat(12_000),
        "=".repeat(12_000),
        "$".repeat(12_000),
        format!("<div {}", "a=\"".repeat(4_000)),
        format!("<div {}", "on:click|".repeat(1_500)),
        format!("<script lang=\"ts\">{}", "{".repeat(6_000)),
        format!("{{#each {} as x}}", "(".repeat(6_000)),
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
