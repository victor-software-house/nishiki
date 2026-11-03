//! Hostile lines for Haskell, which the `TextMate` engine runs, under the
//! default limits.
//!
//! The engine interprets the grammar's own regular expressions, so a line
//! that would cost it more than the default work ends at the work limit, with
//! tokens that cover it, rather than running on.

use nishiki::{Highlighter, Language, Limit};

use super::lex;

#[test]
fn hostile_haskell_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Haskell);
    for line in [
        "\"".repeat(12_000),
        "'".repeat(12_000),
        "{-".repeat(6_000),
        "(".repeat(12_000),
        "[".repeat(12_000),
        "`".repeat(12_000),
        "a.".repeat(6_000),
        "a :: ".repeat(2_400),
        "\\x -> ".repeat(2_000),
        format!("data A = {}", "A | ".repeat(3_000)),
        format!("f {}", "(a, ".repeat(3_000)),
        format!("{}--", " ".repeat(12_000)),
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
