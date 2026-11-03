//! Hostile lines for the less lexer, under the default limits.

use nishiki::{Highlighter, Language, Limit};

use super::lex;

#[test]
fn hostile_less_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Less);
    // `prefix`, then `unit` `count` times, then `suffix`.
    let line = |prefix: &str, unit: &str, count: usize, suffix: &str| {
        format!("{prefix}{}{suffix}", unit.repeat(count))
    };
    let lines = [
        line("", "\"", 300_000, ""),
        line("", "'", 300_000, ""),
        line("", "~\"", 150_000, ""),
        line("", "\\", 300_000, ""),
        line("", "/*", 150_000, ""),
        line("", "//", 150_000, ""),
        line("", "(", 300_000, ""),
        line("", "[", 300_000, ""),
        line("", "{", 300_000, ""),
        line("", "}", 300_000, ""),
        line("", "@{", 150_000, ""),
        line("", "@{a}", 75_000, ""),
        line("a@{b}", "}", 150_000, ":x"),
        line("", "{a:", 100_000, ""),
        line("", "@a:", 100_000, ""),
        line("", "@a", 150_000, ""),
        line("", "@@a", 100_000, ""),
        line("", "-", 300_000, ""),
        line("", "--", 150_000, "x"),
        line("", "a-", 150_000, "^"),
        line("", "-a", 150_000, "^"),
        line("", "a-", 150_000, "A"),
        line("", "#a", 150_000, "^"),
        line("", ".a", 150_000, "^"),
        line("", "&a", 150_000, ""),
        line("", ">", 300_000, ""),
        line("", "+_", 150_000, ""),
        line("", ".m() when ", 30_000, ""),
        line("", "[a=\"", 75_000, ""),
        line("", "[a='", 75_000, ""),
        line("", "[a$=", 75_000, ""),
        line("", "@import ", 30_000, ""),
        line("", "@media ", 40_000, ""),
        line("", "@media (", 40_000, ""),
        line("", "@keyframes ", 25_000, ""),
        line("@keyframes x {", "1", 300_000, ""),
        line("@media (a: ", "1", 300_000, ""),
        line("", "@counter-style ", 20_000, ""),
        line("", "@page ", 40_000, ""),
        line("", ":nth-child(", 30_000, ""),
        line(":nth-child(", "n", 300_000, ""),
        line("", "::-webkit-", 30_000, ""),
        line("a{b:", "a(", 100_000, ""),
        line("a{b:", "rgba(", 60_000, ""),
        line("a{b:", "1px ", 75_000, ""),
        line("a{b:", "1.5e5", 60_000, "deg"),
        line("a{b:", "1", 300_000, "x"),
        line("a{b:", "#", 300_000, ""),
        line("a{b:", "u+", 150_000, ""),
        line("a{b:", "- ", 150_000, ""),
        line("a{b:", "auto-", 60_000, ""),
        line("a{", "-webkit-", 30_000, "x"),
        line("a{", "border-", 40_000, ""),
        line("a{", "font ", 50_000, ""),
        line("a{", " ", 300_000, "x"),
        line("a{font-family:", "a ", 150_000, ""),
        line("a{transition:", "a ", 150_000, ""),
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
        highlighter = Highlighter::new(Language::Less);
    }
}
