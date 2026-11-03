//! Hostile lines and fences for the shell lexer, under the default limits.

use nishiki::{Highlighter, Language, Limit, Limits, Work};

use super::lex;

#[test]
fn hostile_shell_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Shell);
    for line in [
        "$(".repeat(100_000),
        "{ ".repeat(100_000),
        "x=1 ".repeat(100_000),
        "case ".repeat(100_000),
        "alias -a ".repeat(50_000),
        format!("{}x=1", "alias -a ".repeat(50_000)),
        format!("select {}\\", "x".repeat(100_000)),
        " ".repeat(300_000),
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
fn a_markdown_fence_runs_its_shell_lines_under_the_same_limits() {
    let limits = Limits {
        work: Some(Work {
            per_byte: 0,
            base: 64,
        }),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Markdown, limits);
    let sh = format!("echo {}", "$(".repeat(200));
    let (rows, _) = lex(&mut highlighter, &["```sh", &sh, "```"]);
    assert!(rows[1].1.reached(Limit::Work));
    assert!(rows[2].1.is_complete());
}
