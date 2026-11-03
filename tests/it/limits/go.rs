//! Hostile lines for the go lexer, under the default limits.

use nishiki::{Highlighter, Language, Limit};

use super::lex;

#[test]
fn hostile_go_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Go);
    // `prefix`, then `unit` `count` times, then `suffix`.
    let line = |prefix: &str, unit: &str, count: usize, suffix: &str| {
        format!("{prefix}{}{suffix}", unit.repeat(count))
    };
    let lines = [
        line("", "\"", 300_000, ""),
        line("", "'", 300_000, ""),
        line("", "`", 300_000, ""),
        line("", "\\", 150_000, ""),
        line("", "/*", 150_000, ""),
        line("", "(", 300_000, ""),
        line("", "[", 300_000, ""),
        line("", "{", 300_000, ""),
        line("", "func(", 60_000, ""),
        line("", "func (", 50_000, ""),
        line("", "a,", 150_000, ""),
        line("", "a, ", 100_000, ""),
        line("", "a.", 150_000, ""),
        line("", "a b ", 80_000, ""),
        line("", "x := ", 60_000, ""),
        line("", "interface{", 30_000, ""),
        line("", "map[", 60_000, ""),
        line("", "<-chan ", 40_000, ""),
        line("", "[]*", 100_000, ""),
        line("", "chan ", 60_000, ""),
        line("var a ", "chan ", 17, ""),
        line("var a ", "chan ", 257, ""),
        line("var a ", "chan <- ", 5_000, ""),
        line("", "chan <- ", 40_000, ""),
        line("", "*[]chan<- ", 30_000, ""),
        line("", "1.5e+", 60_000, ""),
        line("", "0x1_", 70_000, ""),
        line("", "%[1]*.[2]", 40_000, ""),
        line("\"", "%", 300_000, ""),
        line("case ", "a,", 100_000, ""),
        line("case ", "a:", 100_000, ""),
        line("switch ", "a;", 100_000, ""),
        line("switch ", "a == ", 60_000, "{"),
        line("import ", "a ", 100_000, ""),
        line("var ", "a,", 100_000, ""),
        line("const ", "a,", 100_000, ""),
        line("const ", "chan ", 60_000, "struct"),
        line("a[", "1:", 100_000, ""),
        line("make(", "chan ", 60_000, ""),
        line("make(T[", "a,", 100_000, ""),
        line("type T[", "a], ", 17, ""),
        line("type T[", "a], ", 50_000, ""),
        line("func f() ", "chan ", 60_000, ""),
        line("x.(", "chan ", 60_000, ")"),
        line("", " ", 300_000, "x"),
        line("", "\u{e9}", 150_000, ""),
        line("", "x", 300_000, ""),
    ];
    // A struct or interface that opens many times on one line is scanned from
    // each opener, as Oniguruma scans it, and may run into the work limit.
    let repeated = [
        line("", "type T struct {", 20_000, ""),
        line("", "struct {", 40_000, ""),
        line("{", "a b ", 60_000, "}"),
        line("", "\u{a0}", 100_000, "x"),
    ];
    let mut failures = Vec::new();
    for line in repeated {
        let (rows, _) = lex(&mut highlighter, &[&line]);
        let report = rows[0].1;
        if !report
            .limits()
            .all(|limit| matches!(limit, Limit::Depth | Limit::Work))
        {
            failures.push(format!(
                "{:?} reached {:?}",
                &line[..12.min(line.len())],
                report.limits().collect::<Vec<_>>()
            ));
        }
    }
    for line in lines {
        let (rows, _) = lex(&mut highlighter, &[&line]);
        let report = rows[0].1;
        if !report.limits().all(|limit| limit == Limit::Depth) {
            failures.push(format!(
                "{:?} reached {:?}",
                &line[..12.min(line.len())],
                report.limits().collect::<Vec<_>>()
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
