//! Hostile lines for Astro under the default limits.
//!
//! A line that would cost more than the default work ends at the work limit,
//! with tokens that cover it, rather than running on.

use nishiki::{Highlighter, Language, Limit};

use super::lex;

#[test]
fn hostile_astro_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Astro);
    for line in [
        "{".repeat(12_000),
        format!("{{ {}", "{a: ".repeat(3_000)),
        "<".repeat(12_000),
        "<a ".repeat(4_000),
        "</".repeat(6_000),
        "<!--".repeat(3_000),
        format!("<div {}>", "a=\"b\" ".repeat(2_000)),
        format!("<div {}>", "onclick=\"x\" ".repeat(1_000)),
        format!("<div onclick=\"{}\">", "a /* b */ ".repeat(1_000)),
        format!("<div onclick='{}'>", "// ".repeat(4_000)),
        format!("<div title=`{}`>", "${a} \\n ".repeat(1_000)),
        format!("<div title=`{}`>", "${".repeat(6_000)),
        format!("<script lang=\"ts\" {}>", "x=\"a>b\" ".repeat(1_000)),
        format!("<script {}>", "type=\"x\" ".repeat(1_000)),
        format!("<pre {}is:raw>", "a ".repeat(6_000)),
        format!("<{}is:raw>", "a".repeat(12_000)),
        "&amp;".repeat(2_000),
        format!("&{};", "a".repeat(12_000)),
        "&#1".repeat(4_000),
        format!("{}\u{e9}", "onclick".repeat(2_000)),
        format!("{}x", "\u{a0}".repeat(4_000)),
        "x".repeat(12_000),
    ] {
        let (rows, _) = lex(&mut highlighter, &[&line]);
        let report = rows[0].1;
        assert!(
            report
                .limits()
                .all(|limit| matches!(limit, Limit::Depth | Limit::Work)),
            "{:?} reached {:?}",
            line.chars().take(4).collect::<String>(),
            report.limits().collect::<Vec<_>>()
        );
    }
}
