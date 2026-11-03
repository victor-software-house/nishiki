//! Hostile lines and fences for the css lexer, under the default limits.

use nishiki::{Highlighter, Language, Limit};

use super::lex;

#[test]
fn hostile_css_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Css);
    for line in [
        "\"".repeat(300_000),
        "'".repeat(300_000),
        "\\".repeat(150_000),
        "/*".repeat(150_000),
        "(".repeat(300_000),
        "[".repeat(300_000),
        "{".repeat(300_000),
        "{a:".repeat(100_000),
        "a:not(".repeat(50_000),
        "@media ".repeat(50_000),
        "@media a".repeat(40_000),
        "@import url(".repeat(25_000),
        format!("@charset {}", "\"a".repeat(100_000)),
        format!("@namespace {}", "/* ".repeat(100_000)),
        format!("@media {}", "(-webkit-a /* ".repeat(20_000)),
        format!("a{{b:{}", "1.5e+".repeat(60_000)),
        format!("a{{b:{}", "1".repeat(300_000)),
        format!("a{{b:{}", "u+".repeat(100_000)),
        format!("a{{b:{}", "url(".repeat(60_000)),
        format!("a{{b:{}", "var(--".repeat(50_000)),
        format!("a{{{}x", " ".repeat(300_000)),
        format!("{}x{{", "\u{a0}".repeat(100_000)),
        format!("a{{b:{}", "\\0000e9".repeat(40_000)),
        format!("a{{b:{}", "!important ".repeat(30_000)),
        "a ".repeat(150_000),
        "a-".repeat(150_000),
        "x".repeat(300_000),
        "\u{e9}".repeat(150_000),
        "\u{e9}|".repeat(100_000),
        ".\u{a0}".repeat(100_000),
        "#\u{e9}".repeat(100_000),
        "\u{e9} ".repeat(100_000),
        format!("a{{b:{}", "\u{e9}".repeat(150_000)),
        format!("a{{b:{}", "\u{17f}".repeat(100_000)),
        format!("a{{b:{}", "-\u{e9}".repeat(100_000)),
        format!("a{{{}", "\u{e9}:".repeat(100_000)),
        format!("@{}", "\u{e9}".repeat(150_000)),
        format!("[{}", "\u{e9}".repeat(150_000)),
        format!(":{}", "\u{e9}".repeat(150_000)),
    ] {
        let (rows, _) = lex(&mut highlighter, &[&line]);
        let report = rows[0].1;
        assert!(
            report.limits().all(|limit| limit == Limit::Depth),
            "{:?} reached {:?}",
            line.chars().take(4).collect::<String>(),
            report.limits().collect::<Vec<_>>()
        );
        highlighter = Highlighter::new(Language::Css);
    }
}
