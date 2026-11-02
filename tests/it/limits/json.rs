//! Hostile lines and fences for the json lexer, under the default limits.

use nishiki::{Highlighter, Language, Limit, Limits, Work};

use super::lex;

#[test]
fn hostile_json_lines_finish_within_the_default_limits() {
    let long_string = format!("\"{}\"", "x".repeat(300_000));
    for language in [Language::Json, Language::Jsonc, Language::Jsonl] {
        let mut highlighter = Highlighter::new(language);
        for line in [
            "[".repeat(100_000),
            "{\"a\":".repeat(100_000),
            "{[".repeat(50_000),
            "]}".repeat(50_000),
            format!("\"{}", "\\n".repeat(100_000)),
            format!("\"{}", "\\u12".repeat(60_000)),
            format!("\"{}", "\\".repeat(100_000)),
            long_string.clone(),
            "/*".repeat(100_000),
            "/**".repeat(100_000),
            "//".repeat(100_000),
            "-".repeat(300_000),
            "1e".repeat(100_000),
            "0.".repeat(100_000),
            "tr".repeat(100_000),
            "\u{a0}".repeat(100_000),
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
}

#[test]
fn a_markdown_fence_runs_its_json_lines_under_the_same_limits() {
    let limits = Limits {
        work: Some(Work {
            per_byte: 0,
            base: 64,
        }),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Markdown, limits);
    let json = format!("[{}]", "1,".repeat(200));
    let (rows, _) = lex(&mut highlighter, &["```json", &json, "```"]);
    assert!(rows[1].1.reached(Limit::Work));
    assert!(rows[2].1.is_complete());
}
