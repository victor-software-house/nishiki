//! Hostile lines for the HCL lexer, under the default limits.

use nishiki::{Highlighter, Language, Limit};

use super::lex;

#[test]
fn hostile_hcl_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Hcl);
    // `prefix`, then `unit` `count` times, then `suffix`.
    let line = |prefix: &str, unit: &str, count: usize, suffix: &str| {
        format!("{prefix}{}{suffix}", unit.repeat(count))
    };
    let lines = [
        line("x = ", "[", 100_000, ""),
        line("x = ", "{", 100_000, ""),
        line("x = ", "(", 100_000, ""),
        line("x = ", "f(", 100_000, ""),
        line("x = ", "[for ", 50_000, ""),
        line("x = ", "{for ", 50_000, ""),
        line("x = \"", "${\"", 50_000, ""),
        line("x = \"", "%{", 100_000, ""),
        line("", "a ", 100_000, ""),
        line("", "a \"b\" ", 50_000, ""),
        line("", "a {", 100_000, ""),
        line("", "a-", 100_000, ""),
        line("", "a-", 100_000, "= 1"),
        line("", "a:", 100_000, ""),
        line("x = ", "a:", 100_000, "("),
        line("x = ", "a.", 100_000, ""),
        line("x = ", "a.*", 100_000, ""),
        line("x = ", "...", 100_000, ""),
        line("x = {", "a ", 100_000, ""),
        line("x = {", "a-", 100_000, ""),
        line("x = {", "a = ", 100_000, ""),
        line("x = {", "xfor ", 50_000, ""),
        line("x = [", "elif ", 50_000, ""),
        line("x = [", "a if ", 50_000, ""),
        line("  \"", "\" = ", 100_000, ""),
        line("x = ", "<<a ", 100_000, ""),
        line("x = ", "<<", 100_000, ""),
        line("x = ", "1.", 100_000, ""),
        line("x = ", "1e", 100_000, ""),
        line("x = \"", "\\", 100_000, ""),
        line("x = \"", "\\u12", 60_000, ""),
        line("x = \"", "$", 300_000, ""),
        line("", "#", 300_000, ""),
        line("", "/*", 100_000, ""),
        line("", "x", 300_000, ""),
        line("", "\u{a0}", 100_000, "x"),
        line("", "\u{3000}", 100_000, "{"),
    ];
    for line in lines {
        let (rows, _) = lex(&mut highlighter, &[&line]);
        let report = rows[0].1;
        assert!(
            report.limits().all(|limit| limit == Limit::Depth),
            "{:?} reached {:?}",
            &line[..8.min(line.len())],
            report.limits().collect::<Vec<_>>()
        );
    }
}
