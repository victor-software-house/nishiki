//! Hostile lines and fences for the toml lexer, under the default limits.

use nishiki::{Highlighter, Language, Limit};

use super::lex;

#[test]
fn hostile_toml_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Toml);
    // `prefix`, then `unit` `count` times, then `suffix`.
    let line = |prefix: &str, unit: &str, count: usize, suffix: &str| {
        format!("{prefix}{}{suffix}", unit.repeat(count))
    };
    let lines = [
        line("a = ", "[", 100_000, ""),
        line("a = ", "{", 100_000, ""),
        line("a = ", "{a=", 50_000, ""),
        line("a = ", "[{", 50_000, ""),
        line("", "[", 100_000, ""),
        line("", "[[", 100_000, ""),
        line("", "]", 100_000, ""),
        line("[a", ".a", 100_000, ""),
        line("[", "a.", 100_000, "]"),
        line("", "a.", 100_000, ""),
        line("", "a.", 100_000, "= 1"),
        line("", "a . ", 50_000, "x"),
        line("", "\"a\".", 50_000, "x"),
        line("{", "a.", 100_000, "x"),
        line("a = {", "\"a\".", 50_000, "x"),
        line("a = {", "\"a\".", 50_000, "x y = 1"),
        line("a = {", "a.", 100_000, "x y = 1"),
        line("a = {", "\"a\" = 1,", 30_000, ""),
        line("a = {", "a.b = 1, ", 30_000, ""),
        line("", "\"", 100_000, ""),
        line("", "'", 100_000, ""),
        line("", "\"\" ", 100_000, ""),
        line("", "\"a\" ", 60_000, "= 1"),
        line("a = ", "= ", 100_000, ""),
        line("", "=", 300_000, ""),
        line("a = ", "=", 300_000, ""),
        line("", "a ", 100_000, ""),
        line("a = \"", "\\", 100_000, ""),
        line("a = \"", "\\u12", 60_000, ""),
        line("a = \"\"\"", "\"", 100_000, ""),
        line("a = \"\"\"", "\"\"", 100_000, ""),
        line("a = '''", "''", 100_000, ""),
        line("a = \"", "x", 300_000, "\""),
        line("", "#", 300_000, ""),
        line("a = 1 ", "# ", 100_000, ""),
        line("a = ", "1_", 100_000, ""),
        line("a = ", "1e", 100_000, ""),
        line("a = ", "0.", 100_000, ""),
        line("a = ", "1979-05-27T", 30_000, ""),
        line("a = ", "07:32:00.", 30_000, ""),
        line("a = ", "0x", 100_000, ""),
        line("a = [", "1 ", 100_000, ""),
        line("a = [", ",", 100_000, ""),
        line("a = [", "x ", 100_000, ""),
        line("", "x", 300_000, ""),
        line("", "\u{a0}", 100_000, ""),
        line("a = 1", "\u{a0}", 100_000, "x"),
        line("", "\u{3000}", 100_000, "[a]"),
    ];
    for line in lines {
        let (rows, _) = lex(&mut highlighter, &[&line]);
        let report = rows[0].1;
        assert!(
            report.limits().all(|limit| limit == Limit::Depth),
            "{:?} reached {:?}",
            &line[..4.min(line.len())],
            report.limits().collect::<Vec<_>>()
        );
    }
}
