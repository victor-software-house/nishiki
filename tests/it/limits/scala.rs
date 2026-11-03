//! Hostile lines and fences for Scala, which the `TextMate` engine runs,
//! under the default limits.
//!
//! The engine interprets the grammar's own regular expressions, so a line
//! that would cost it more than the default work ends at the work limit, with
//! tokens that cover it, rather than running on.

use nishiki::{Highlighter, Language, Limit, Limits, Work};

use super::lex;

#[test]
fn hostile_scala_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Scala);
    for line in [
        "\"".repeat(12_000),
        "'".repeat(12_000),
        "`".repeat(12_000),
        "\"\"\"".repeat(4_000),
        "s\"${".repeat(3_000),
        "/*".repeat(6_000),
        "(".repeat(12_000),
        "{".repeat(12_000),
        "[".repeat(12_000),
        "<a ".repeat(4_000),
        "a.".repeat(6_000),
        "a: ".repeat(4_000),
        "=> ".repeat(4_000),
        format!("def {}", "a_".repeat(6_000)),
        format!("class A[{}", "B <: ".repeat(2_400)),
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

#[test]
fn a_markdown_fence_runs_its_scala_lines_under_the_same_limits() {
    let limits = Limits {
        work: Some(Work {
            per_byte: 0,
            base: 64,
        }),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Markdown, limits);
    let scala = "val a: List[Int] = xs.map(_ + 1); ".repeat(100);
    let (rows, _) = lex(&mut highlighter, &["```scala", &scala, "```"]);
    assert!(rows[1].1.reached(Limit::Work));
    assert!(rows[2].1.is_complete());
}
