//! Hostile lines and fences for the dockerfile lexer, under the default limits.

use nishiki::{Highlighter, Language, Limit, Limits, Work};

use super::lex;

#[test]
fn hostile_dockerfile_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Dockerfile);
    for line in [
        "\"".repeat(300_000),
        "'".repeat(300_000),
        "\\".repeat(150_000),
        "\\\"".repeat(100_000),
        "\"a\\'".repeat(60_000),
        format!("FROM {}", "a ".repeat(100_000)),
        format!("FROM {}", "a".repeat(300_000)),
        format!("FROM {}as", "a ".repeat(100_000)),
        format!("FROM a {}", "AS".repeat(100_000)),
        format!("ONBUILD{}x", " ".repeat(300_000)),
        format!("{}RUN", " ".repeat(300_000)),
        format!("{}x", "\u{a0}".repeat(100_000)),
        format!("{}#", "\t".repeat(300_000)),
        format!("RUN {}", "\u{17f}".repeat(100_000)),
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

#[test]
fn a_markdown_fence_runs_its_dockerfile_lines_under_the_same_limits() {
    let limits = Limits {
        work: Some(Work {
            per_byte: 0,
            base: 64,
        }),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Markdown, limits);
    let dockerfile = format!("RUN {}", "\"a\" ".repeat(100));
    let (rows, _) = lex(&mut highlighter, &["```dockerfile", &dockerfile, "```"]);
    assert!(rows[1].1.reached(Limit::Work));
    assert!(rows[2].1.is_complete());
}
