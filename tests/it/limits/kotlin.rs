//! Hostile lines for the kotlin lexer, under the default limits.

use nishiki::{Highlighter, Language, Limit};

use super::lex;

#[test]
fn hostile_kotlin_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Kotlin);
    // `prefix`, then `unit` `count` times, then `suffix`.
    let line = |prefix: &str, unit: &str, count: usize, suffix: &str| {
        format!("{prefix}{}{suffix}", unit.repeat(count))
    };
    let lines = [
        line("", "(", 100_000, ""),
        line("", "[", 100_000, ""),
        line("", "{", 100_000, ""),
        line("", "}", 100_000, ""),
        line("", "<", 100_000, ""),
        line("", ">", 100_000, ""),
        line("", "<>", 100_000, ""),
        line("", "a<", 100_000, ""),
        line("", "x<", 50_000, ""),
        line("", "x<a>", 50_000, "("),
        line("", "x <", 50_000, ""),
        line("", "x(", 100_000, ""),
        line("", "x()", 100_000, ""),
        line("", ".x()", 100_000, ""),
        line("", "?.x ", 50_000, "("),
        line("", "a b ", 50_000, "("),
        line("", "::", 100_000, ""),
        line("", "a::", 50_000, ""),
        line("", "`", 100_000, ""),
        line("", "`a", 100_000, ""),
        line("", "@", 100_000, ""),
        line("", "@A(", 50_000, ""),
        line("", "@A ", 50_000, ""),
        line("", "@A:", 50_000, ""),
        line("", "@a:[", 50_000, ""),
        line("", "@file: ", 50_000, ""),
        line("", "@a.b.", 50_000, ":"),
        line("", "class ", 50_000, ""),
        line("", "class A<", 30_000, ""),
        line("", "fun interface ", 30_000, ""),
        line("", "interface ", 30_000, ""),
        line("", "typealias ", 40_000, ""),
        line("", "typealias A<", 30_000, ""),
        line("", "object ", 50_000, ""),
        line("", "fun ", 50_000, ""),
        line("", "fun <", 30_000, ""),
        line("", "fun a.", 50_000, ""),
        line("", "fun A.b ", 30_000, ""),
        line("", "val ", 50_000, "x"),
        line("", "val <", 30_000, ""),
        line("", "var ", 50_000, ""),
        line("", "import ", 40_000, ""),
        line("", "package ", 40_000, ""),
        line("", ": ", 100_000, ""),
        line("", ":(", 50_000, ""),
        line("", ": (", 50_000, ""),
        line("", ": <", 50_000, ""),
        line("", ": A<", 30_000, ""),
        line("", ": (A, ", 30_000, ""),
        line("", ": a ", 50_000, ""),
        line("", ":a", 100_000, ""),
        line("", "?:", 100_000, ""),
        line("", "a: A ", 30_000, ""),
        line("", "\"", 100_000, ""),
        line("", "\"\"", 100_000, ""),
        line("", "\"\"\"", 100_000, ""),
        line("", "'", 100_000, ""),
        line("\"", "\\", 100_000, ""),
        line("\"", "\\\"", 100_000, ""),
        line("\"", "${", 100_000, ""),
        line("\"\"\"", "${", 100_000, ""),
        line("\"", "$a", 100_000, ""),
        line("\"", "${x}", 50_000, ""),
        line("", "\"${", 50_000, ""),
        line("", "/*", 100_000, ""),
        line("", "/**", 100_000, ""),
        line("", "//", 100_000, ""),
        line("/** ", "{@link a#b(", 30_000, ""),
        line("/** ", "{@link ", 40_000, "#a(b)}"),
        line("/** ", "{@link a#", 40_000, ""),
        line("/** ", "@param ", 40_000, ""),
        line("/** ", "@param[", 40_000, ""),
        line("/** ", "@param[a]", 40_000, ""),
        line("", "0x", 100_000, ""),
        line("", "0b", 100_000, ""),
        line("", "1e", 100_000, ""),
        line("", "1_", 100_000, ""),
        line("", "1.", 100_000, ""),
        line("", "1.5e", 50_000, ""),
        line("", "1.5eu", 50_000, ""),
        line("", "1f", 100_000, ""),
        line("", "1", 300_000, "x"),
        line("", "x", 300_000, ""),
        line("", "日", 100_000, ""),
        line("", "\u{a0}", 100_000, ""),
        line("", "$", 300_000, ""),
        line("", "_a", 100_000, ""),
        line("", "->", 100_000, ""),
        line("", "==", 100_000, "="),
        line("", "!=", 100_000, "="),
        line("", "a=", 100_000, ""),
        line("", "a= ", 100_000, ""),
        line("", "this@", 50_000, ""),
        line("", "super@a ", 50_000, ""),
        line("", ".", 300_000, ""),
        line("", "..", 100_000, ""),
        line("", " ", 300_000, ""),
        line("x", " ", 300_000, "y"),
        line("", "\t", 300_000, "//"),
        line("", "if ", 50_000, ""),
        line("", "when ", 50_000, ""),
        line("", "private ", 50_000, ""),
        line("", "is ", 50_000, ""),
    ];
    // A group that opens a hundred thousand times and closes once is scanned
    // from each opener, as Oniguruma scans it. It runs into the work limit.
    let nested = [line("", "A<", 100_000, ">"), line("", ": (", 50_000, ")")];
    let mut failures = Vec::new();
    for line in nested {
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
