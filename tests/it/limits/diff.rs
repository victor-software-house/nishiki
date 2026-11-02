//! Hostile lines and fences for the diff lexer, under the default limits.

use nishiki::{Highlighter, Language, Limit, Limits, Work};

use super::lex;

#[test]
fn hostile_diff_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Diff);
    for line in [
        "@@ ".repeat(100_000),
        format!("@@{}", " ".repeat(300_000)),
        format!("@@ {}", "@ ".repeat(100_000)),
        "@@\u{a0}".repeat(100_000),
        format!("==== {}", " - ".repeat(100_000)),
        format!("{} ====", " - ".repeat(100_000)),
        format!("{}====", " ".repeat(300_000)),
        format!("diff {}", "a ".repeat(100_000)),
        format!("diff {}", "a".repeat(300_000)),
        format!("index {}", "a".repeat(300_000)),
        format!("Only in {}", ":".repeat(300_000)),
        "1,".repeat(100_000),
        "\u{663}".repeat(100_000),
        format!("*** {} ****", "x".repeat(300_000)),
        "-".repeat(300_000),
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
fn a_markdown_fence_runs_its_diff_lines_under_the_same_limits() {
    let limits = Limits {
        work: Some(Work {
            per_byte: 0,
            base: 64,
        }),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Markdown, limits);
    let diff = format!("@@ {} @@", "x".repeat(200));
    let (rows, _) = lex(&mut highlighter, &["```diff", &diff, "```"]);
    assert!(rows[1].1.reached(Limit::Work));
    assert!(rows[2].1.is_complete());
}
