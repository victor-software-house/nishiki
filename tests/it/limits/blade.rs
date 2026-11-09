//! Hostile lines for Blade under the default limits.
//!
//! A line that would cost more than the default work ends at the work limit,
//! with tokens that cover it, rather than running on. The same holds for the
//! PHP of Blade's echoes and directives, and for the echoes that open inside
//! that PHP.

use nishiki::{Highlighter, Language, Limit};

use super::lex;

#[test]
fn hostile_blade_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Blade);
    for line in [
        "{{".repeat(12_000),
        "{!!".repeat(8_000),
        "{{--".repeat(6_000),
        "@".repeat(12_000),
        "@if(".repeat(6_000),
        "@x(".repeat(8_000),
        "(".repeat(12_000),
        "<?php ".repeat(4_000),
        format!("{{{{ {}", "\"{{ ".repeat(4_000)),
        format!("{{{{ {}", "f(".repeat(6_000)),
        format!("@if({}", "'@if(".repeat(4_000)),
        format!("<div {}", ":a=\"{{ ".repeat(3_000)),
        format!("@section({}", "(".repeat(8_000)),
        format!("@php function f(array $a = {}", "[".repeat(200_000)),
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

/// A heredoc line of tags and no Blade looks for Blade's own rules once, not
/// once a tag. It therefore stays within the default work.
#[test]
fn a_heredoc_line_of_tags_finishes_within_the_default_work() {
    let mut highlighter = Highlighter::new(Language::Blade);
    let tags = "<b>x</b>".repeat(3_000);
    let (rows, _) = lex(
        &mut highlighter,
        &["@php", "$h = <<<BLADE", &tags, "BLADE;", "@endphp"],
    );
    let report = rows[2].1;
    assert_eq!(
        report.limits().collect::<Vec<_>>(),
        [],
        "the heredoc line reached a limit"
    );
}
