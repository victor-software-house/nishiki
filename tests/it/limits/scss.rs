//! Hostile lines for the scss lexer, under the default limits.

use nishiki::{Highlighter, Language, Limit};

use super::lex;

#[test]
fn hostile_scss_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Scss);
    // `prefix`, then `unit` `count` times, then `suffix`.
    let line = |prefix: &str, unit: &str, count: usize, suffix: &str| {
        format!("{prefix}{}{suffix}", unit.repeat(count))
    };
    let lines = [
        line("", "\"", 300_000, ""),
        line("", "'", 300_000, ""),
        line("", "\\", 300_000, ""),
        line("", "/*", 150_000, ""),
        line("", "//", 150_000, ""),
        line("", "(", 300_000, ""),
        line("", "[", 300_000, ""),
        line("", "{", 300_000, ""),
        line("", "#{", 150_000, ""),
        line("", "{a:", 100_000, ""),
        line("", "$a:", 100_000, ""),
        line("", "$a", 150_000, ""),
        line("", "-", 300_000, ""),
        line("", "--", 150_000, "x"),
        line("", "a-", 150_000, "^"),
        line("", ".$", 150_000, "^"),
        line("", "#a", 150_000, "^"),
        line("", "%a", 150_000, "^"),
        line("", "&a", 150_000, ""),
        line("", "[a=\"", 75_000, ""),
        line("", "[a='", 75_000, ""),
        line("", "[a$=", 75_000, ""),
        line("", "@include ", 30_000, ""),
        line("", "@mixin ", 40_000, ""),
        line("", "@media ", 40_000, ""),
        line("", "@if ", 60_000, ""),
        line("", "@else ", 50_000, ""),
        line("", "@each $a in ", 20_000, ""),
        line("", "@keyframes ", 25_000, ""),
        line("", "nth-child(", 30_000, ""),
        line(":nth-child(", "n", 300_000, ""),
        line("a{b:", "a(", 100_000, ""),
        line("a{b:", "a.b", 100_000, ""),
        line("a{b:", "1px ", 75_000, ""),
        line("a{b:", "- ", 150_000, ""),
        line("a{b:", "rgba", 75_000, ""),
        line("a{", " ", 300_000, "x"),
        line("", "\u{a0}", 100_000, "x{"),
        line("", "x", 300_000, ""),
        line("", "\u{e9}", 150_000, ""),
        line(".", "\u{e9}", 150_000, "^"),
        line("[", "\u{e9}", 150_000, ""),
        line("a{b:", "\u{e9}", 150_000, ""),
    ];
    for text in lines {
        let (rows, _) = lex(&mut highlighter, &[&text]);
        let report = rows[0].1;
        assert!(
            report.limits().all(|limit| limit == Limit::Depth),
            "{:?} reached {:?}",
            text.chars().take(6).collect::<String>(),
            report.limits().collect::<Vec<_>>()
        );
        highlighter = Highlighter::new(Language::Scss);
    }
}
