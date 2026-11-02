//! Hostile lines and fences for the sql lexer, under the default limits.

use nishiki::{Highlighter, Language, Limit, Limits, Work};

use super::lex;

#[test]
fn hostile_sql_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Sql);
    for line in [
        "'".repeat(100_000),
        "''".repeat(100_000),
        "\"".repeat(100_000),
        "\"#{".repeat(60_000),
        "`".repeat(100_000),
        "[".repeat(100_000),
        "]]".repeat(100_000),
        "/*".repeat(100_000),
        "*/".repeat(100_000),
        "--".repeat(100_000),
        "#".repeat(300_000),
        "%{".repeat(100_000),
        "%r{#{".repeat(50_000),
        "#{".repeat(100_000),
        "N'".repeat(100_000),
        "'a' ".repeat(75_000),
        "@@".repeat(100_000),
        "@a ".repeat(100_000),
        "a.".repeat(100_000),
        "a.b ".repeat(75_000),
        "()".repeat(100_000),
        "count(".repeat(50_000),
        "drop table ".repeat(30_000),
        format!("{}drop table x", " ".repeat(300_000)),
        format!("{}x", "\u{a0}".repeat(100_000)),
        "create ".repeat(40_000),
        "create or replace ".repeat(20_000),
        format!("create table {}", "'".repeat(200_000)),
        "select all ".repeat(30_000),
        "left outer ".repeat(30_000),
        "natural left outer ".repeat(20_000),
        "timestamp(1) with ".repeat(20_000),
        "double ".repeat(40_000),
        "primary ".repeat(40_000),
        "\\".repeat(150_000),
        "1".repeat(300_000),
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
fn a_markdown_fence_runs_its_sql_lines_under_the_same_limits() {
    let limits = Limits {
        work: Some(Work {
            per_byte: 0,
            base: 64,
        }),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Markdown, limits);
    let sql = format!("select {}", "a.b, ".repeat(100));
    let (rows, _) = lex(&mut highlighter, &["```sql", &sql, "```"]);
    assert!(rows[1].1.reached(Limit::Work));
    assert!(rows[2].1.is_complete());
}
