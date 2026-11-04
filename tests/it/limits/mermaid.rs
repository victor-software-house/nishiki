//! Hostile lines for Mermaid under the default limits.
//!
//! A line that would cost more than the default work ends at the work limit,
//! with tokens that cover it, rather than running on.

use nishiki::{Highlighter, Language, Limit};

use super::lex;

#[test]
fn hostile_mermaid_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Mermaid);
    for line in [
        "\"".repeat(12_000),
        "-->".repeat(4_000),
        "[".repeat(12_000),
        "(".repeat(12_000),
        "{".repeat(12_000),
        "%%".repeat(6_000),
        "A-->B;".repeat(2_000),
        "|a|".repeat(4_000),
        format!("graph TD {}", "A[".repeat(4_000)),
        format!("sequenceDiagram {}", "A->>B: ".repeat(1_500)),
        format!("{}%%", " ".repeat(12_000)),
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
