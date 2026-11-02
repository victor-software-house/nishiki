//! Hostile lines and fences for the graphql lexer, under the default limits.

use nishiki::{Highlighter, Language, Limit};

use super::lex;

#[test]
fn hostile_graphql_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::GraphQl);
    for line in [
        "{".repeat(300_000),
        "{ a ".repeat(75_000),
        "(".repeat(300_000),
        "[".repeat(300_000),
        "\"".repeat(300_000),
        "\"\"\"".repeat(100_000),
        "#".repeat(300_000),
        "...".repeat(100_000),
        "... on ".repeat(40_000),
        "$a ".repeat(100_000),
        "@a(b: ".repeat(40_000),
        "query ".repeat(50_000),
        format!("type {}", "a ".repeat(100_000)),
        format!("type A implements {}", "& B ".repeat(75_000)),
        format!("a: {}", "[".repeat(300_000)),
        format!("{{ {}a", " ".repeat(300_000)),
        format!("{{ ${{ {}", "a ".repeat(100_000)),
        "${".repeat(150_000),
        "${a}".repeat(75_000),
        format!("{}x", "\u{a0}".repeat(100_000)),
        format!("{{ a{}", ":".repeat(300_000)),
        "x".repeat(300_000),
    ] {
        let (rows, _) = lex(&mut highlighter, &[&line]);
        let report = rows[0].1;
        assert!(
            report.limits().all(|limit| limit == Limit::Depth),
            "{:?} reached {:?}",
            &line[..4],
            report.limits().collect::<Vec<_>>()
        );
    }
}
