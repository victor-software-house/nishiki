//! Hostile lines and fences for the java lexer, under the default limits.

use nishiki::{Highlighter, Language, Limit, Limits, Work};

use super::lex;

#[test]
fn hostile_java_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Java);
    // `prefix`, then `unit` `count` times, then `suffix`.
    let line = |prefix: &str, unit: &str, count: usize, suffix: &str| {
        format!("{prefix}{}{suffix}", unit.repeat(count))
    };
    let lines = [
        line("", "{", 100_000, ""),
        line("", "(", 100_000, ""),
        line("", "[", 100_000, ""),
        line("", "<", 100_000, ""),
        line("", "a<", 100_000, ""),
        line("", "A<", 100_000, ">"),
        line("", "}", 100_000, ""),
        line("", ">", 100_000, ""),
        line("", "new ", 50_000, ""),
        line("", "@", 100_000, ""),
        line("", "@A(", 50_000, ""),
        line("", "@A ", 50_000, ""),
        line("", "class ", 50_000, ""),
        line("", "enum ", 50_000, ""),
        line("", "record ", 40_000, ""),
        line("", "public ", 40_000, "class A"),
        line("", "interface ", 30_000, ""),
        line("", "import ", 40_000, ""),
        line("", "package ", 40_000, ""),
        line("", "int ", 50_000, ""),
        line("", "int ", 50_000, "x;"),
        line("", "A ", 100_000, "x;"),
        line("", "A a, ", 50_000, ";"),
        line("", "int[] ", 50_000, "x;"),
        line("", "int x = 1; ", 20_000, ""),
        line("", "A.", 100_000, "x"),
        line("", "a.", 100_000, "b y;"),
        line("", "a.b ", 50_000, ""),
        line("", "A.B<", 50_000, ""),
        line("", "x(", 100_000, ""),
        line("", "x()", 100_000, ""),
        line("", ".x()", 100_000, ""),
        line("", "a b ", 50_000, "("),
        line("", "void m(", 50_000, ""),
        line("", "int a, ", 50_000, ") {"),
        line("", "throws ", 50_000, ""),
        line("", "extends ", 50_000, ""),
        line("", "implements A, ", 30_000, ""),
        line("", "\"", 100_000, ""),
        line("", "'", 100_000, ""),
        line("", "\"\"\"", 100_000, ""),
        line("\"", "\\", 100_000, ""),
        line("\"", "\\\"", 100_000, ""),
        line("\"\"\"", "\\\"\"\"", 50_000, ""),
        line("", "/*", 100_000, ""),
        line("", "/**", 100_000, ""),
        line("", "//", 100_000, ""),
        line("/** ", "{@link A#b(", 30_000, ""),
        line("/** ", "@param ", 40_000, ""),
        line("", "0x", 100_000, ""),
        line("", "0x1p", 50_000, ""),
        line("", "1e", 100_000, ""),
        line("", "1_", 100_000, ""),
        line("", "1.", 100_000, ""),
        line("", ".5", 100_000, ""),
        line("", "1f", 100_000, ""),
        line("", "x", 300_000, ""),
        line("", "$", 300_000, ""),
        line("", "_a", 100_000, ""),
        line("", "->", 100_000, ""),
        line("", "<<", 100_000, "="),
        line("", "?", 300_000, ""),
        line("", ":", 300_000, ""),
        line("", " ", 300_000, ""),
        line("x", " ", 300_000, "y"),
        line("", "\t", 300_000, "//"),
        line("", "\u{a0}", 100_000, ""),
        line("", "日", 100_000, ""),
        line("", "try ", 50_000, ""),
        line("", "catch (", 50_000, ""),
        line("", "for (int i = 0; ", 30_000, ""),
        line("", "case ", 50_000, ""),
        line("", "assert ", 50_000, ""),
        line("", "enum E { A(", 30_000, ""),
        line("", "module m { ", 30_000, ""),
        line("", "open module ", 30_000, ""),
        line("", "static ", 50_000, ""),
        line("", "final ", 50_000, "int a;"),
        line("", "var ", 50_000, "x = 1;"),
        line("", "a=", 100_000, ""),
        line("", "(A) ", 50_000, ""),
        line("", "A::", 100_000, ""),
        line("", "A[]::new ", 30_000, ""),
        line("", "? extends ", 30_000, ""),
        line("", "& ", 100_000, ""),
    ];
    for line in lines {
        let (rows, _) = lex(&mut highlighter, &[&line]);
        let report = rows[0].1;
        assert!(
            report.limits().all(|limit| limit == Limit::Depth),
            "{:?} reached {:?}",
            &line[..6.min(line.len())],
            report.limits().collect::<Vec<_>>()
        );
    }
}

#[test]
fn a_markdown_fence_runs_its_java_lines_under_the_same_limits() {
    let limits = Limits {
        work: Some(Work {
            per_byte: 0,
            base: 64,
        }),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Markdown, limits);
    let java = format!("f({});", "1, ".repeat(200));
    let (rows, _) = lex(&mut highlighter, &["```java", &java, "```"]);
    assert!(rows[1].1.reached(Limit::Work));
    assert!(rows[2].1.is_complete());
}
