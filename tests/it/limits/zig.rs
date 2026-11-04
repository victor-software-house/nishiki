//! Hostile lines for Zig under the default limits.
//!
//! A line that would cost more than the default work ends at the work limit,
//! with tokens that cover it, rather than running on.

use nishiki::{Highlighter, Language, Limit};

use super::lex;

#[test]
fn hostile_zig_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Zig);
    for line in [
        "\"".repeat(12_000),
        "'".repeat(12_000),
        "\\\\".repeat(6_000),
        "@\"".repeat(6_000),
        "(".repeat(12_000),
        "{".repeat(12_000),
        "[".repeat(12_000),
        ".{".repeat(6_000),
        "a.".repeat(6_000),
        "a: ".repeat(4_000),
        "!".repeat(12_000),
        "0x".repeat(6_000),
        format!("fn {}", "a_".repeat(6_000)),
        format!("const a = {}", "struct {".repeat(1_500)),
        format!("{}//", " ".repeat(12_000)),
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
            &line[..4],
            report.limits().collect::<Vec<_>>()
        );
    }
}
