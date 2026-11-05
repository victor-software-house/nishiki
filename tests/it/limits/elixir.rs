//! Hostile lines for Elixir under the default limits.
//!
//! A line that would cost more than the default work ends at the work limit,
//! with tokens that cover it, rather than running on.
//! The same holds for the HTML that `~H` and `~L` sigils hand over.

use nishiki::{Highlighter, Language, Limit};

use super::lex;

#[test]
fn hostile_elixir_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Elixir);
    for line in [
        "\"".repeat(12_000),
        "'".repeat(12_000),
        "(".repeat(12_000),
        "%{".repeat(6_000),
        "[".repeat(12_000),
        "@".repeat(12_000),
        "~H\"\"\" ".repeat(2_000),
        "~r/".repeat(4_000),
        "<<".repeat(6_000),
        format!("a = {}", "f(".repeat(4_000)),
        format!("\"{}\"", "#{ ".repeat(3_000)),
        format!("fn -> {}", "fn -> ".repeat(2_000)),
        "fn ".repeat(4_000),
        format!("{}->", "fn ".repeat(4_000)),
        format!("a {}", "(\"->' ".repeat(2_000)),
        format!("\"{}\"", "#{\"".repeat(3_000)),
        "~s{".repeat(4_000),
        "~a|".repeat(4_000),
        format!("def a do {}", "do ".repeat(3_000)),
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
