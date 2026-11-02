//! Hostile lines and fences for the python lexer, under the default limits.

use nishiki::{Highlighter, Language, Limit, Limits, Work};

use super::lex;

#[test]
fn hostile_python_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Python);
    let lines = [
        "[".repeat(100_000),
        "(".repeat(100_000),
        "{".repeat(100_000),
        "x[".repeat(50_000),
        "f(".repeat(50_000),
        "a.".repeat(100_000),
        "lambda ".repeat(50_000),
        "class ".repeat(50_000),
        "def ".repeat(50_000),
        "@".repeat(100_000),
        format!("@d(x){}x", " ".repeat(100_000)),
        " ".repeat(300_000),
        format!("x{}", " ".repeat(300_000)),
        format!("\"{}", "{".repeat(100_000)),
        format!("\"{}", "{ ".repeat(50_000)),
        format!("\"{}", "{{}".repeat(50_000)),
        format!("\"{}", "{%".repeat(50_000)),
        format!("\"{}", "%(".repeat(50_000)),
        format!("\"{}", "\\N{{".repeat(50_000)),
        format!("f\"{}", "{".repeat(100_000)),
        format!("f\"{}", "{:".repeat(50_000)),
        format!("f\"{}\"", "{:{".repeat(50_000)),
        format!("f\"{}", "{x!r:>".repeat(20_000)),
        format!("\"{}", "{:0".repeat(50_000)),
        format!("\"{}", "\\".repeat(100_000)),
        format!("\"\"\"{}", "\\\"".repeat(50_000)),
        format!("r\"{}", "(?".repeat(50_000)),
        format!("r\"{}", "[".repeat(100_000)),
        format!("r\"{}", "(?P<a ".repeat(30_000)),
        format!("r\"{}", "(?(".repeat(30_000)),
        format!("r\"{}", "{1,".repeat(50_000)),
        format!("r\"{}", "\\1".repeat(50_000)),
        "#".repeat(300_000),
        "# type: ".repeat(30_000),
        "1_".repeat(100_000),
        "0x".repeat(100_000),
        "1e".repeat(100_000),
        "0.".repeat(100_000),
        "x".repeat(300_000),
        "_A".repeat(100_000),
        "\\".repeat(100_000),
        "if ".repeat(60_000),
        "yield from ".repeat(30_000),
        "#!".repeat(100_000),
    ];
    for line in lines {
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
fn a_markdown_fence_runs_its_python_lines_under_the_same_limits() {
    let limits = Limits {
        work: Some(Work {
            per_byte: 0,
            base: 64,
        }),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Markdown, limits);
    let py = format!("x = [{}]", "1, ".repeat(200));
    let (rows, _) = lex(&mut highlighter, &["```py", &py, "```"]);
    assert!(rows[1].1.reached(Limit::Work));
    assert!(rows[2].1.is_complete());
}
