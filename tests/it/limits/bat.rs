//! Hostile lines for Batch under the default limits.
//!
//! A line that would cost more than the default work ends at the work limit,
//! with tokens that cover it, rather than running on.

use nishiki::{Highlighter, Language, Limit};

use super::lex;

#[test]
fn hostile_bat_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Bat);
    for line in [
        "\"".repeat(12_000),
        "%".repeat(12_000),
        "%%a".repeat(4_000),
        "!".repeat(12_000),
        "(".repeat(12_000),
        "^".repeat(12_000),
        "%~dp0".repeat(2_400),
        "set /a ".repeat(2_000),
        format!("if {}", "not exist a ".repeat(1_000)),
        format!("echo {}", "%a:~1,2%".repeat(1_500)),
        format!("for %%i in ({}", "a ".repeat(6_000)),
        format!("{}rem", " ".repeat(12_000)),
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
