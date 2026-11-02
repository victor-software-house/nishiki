//! Hostile lines and fences for the typescript lexer, under the default limits.

use nishiki::{Highlighter, Language, Limit, Limits, Work};

use super::lex;

#[test]
fn a_markdown_fence_runs_its_typescript_lines_under_the_same_limits() {
    let limits = Limits {
        work: Some(Work {
            per_byte: 0,
            base: 64,
        }),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Markdown, limits);
    let ts = format!("let s = \"{}\"; let t = 1;", "x".repeat(200));
    let (rows, _) = lex(&mut highlighter, &["```ts", &ts, "```"]);
    assert!(rows[1].1.reached(Limit::Work));
    assert!(rows[2].1.is_complete());
}

#[test]
fn hostile_typescript_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::TypeScript);
    for line in [
        "`${".repeat(100_000),
        "${".repeat(100_000),
        "(".repeat(100_000),
        "{[(".repeat(50_000),
        ")}]".repeat(50_000),
        "a<".repeat(100_000),
        "f<(a, ".repeat(50_000),
        "x = (a, b: ".repeat(50_000),
        "/*".repeat(100_000),
        "/**@param{".repeat(50_000),
    ] {
        let (rows, _) = lex(&mut highlighter, &[&line]);
        let report = rows[0].1;
        assert!(
            report
                .limits()
                .all(|limit| matches!(limit, Limit::Depth | Limit::Work)),
            "{:?} reached {:?}",
            &line[..8],
            report.limits().collect::<Vec<_>>()
        );
    }
}

#[test]
fn hostile_jsx_lines_finish_within_the_default_limits() {
    for language in [Language::JavaScript, Language::Jsx, Language::Tsx] {
        let mut highlighter = Highlighter::new(language);
        for line in [
            "x = <a>".repeat(50_000),
            "<a><b>".repeat(50_000),
            "x = <a b={".repeat(30_000),
            "x = <a b=\"".repeat(30_000),
            "x = <a b='".repeat(30_000),
            "</a>".repeat(100_000),
            "(<".repeat(100_000),
            "x = <a:b.c-d ".repeat(30_000),
            "<>{<>".repeat(50_000),
            "<a {...b} ".repeat(30_000),
            "&amp".repeat(100_000),
            "x = <T extends ".repeat(30_000),
        ] {
            let (rows, _) = lex(&mut highlighter, &[&line]);
            let report = rows[0].1;
            assert!(
                report
                    .limits()
                    .all(|limit| matches!(limit, Limit::Depth | Limit::Work)),
                "{language:?} {:?} reached {:?}",
                &line[..8],
                report.limits().collect::<Vec<_>>()
            );
        }
    }
}
