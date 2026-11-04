//! Hostile lines for HAML, which the `TextMate` engine runs, under the
//! default limits.
//!
//! The engine interprets the grammar's own regular expressions, so a line
//! that would cost it more than the default work ends at the work limit, with
//! tokens that cover it, rather than running on. The same holds for the lines
//! the engine hands to the Ruby, JavaScript, CSS and Markdown lexers.

use nishiki::{Highlighter, Language, Limit};

use super::lex;

#[test]
fn hostile_haml_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Haml);
    for line in [
        "%".repeat(12_000),
        "#{".repeat(6_000),
        "{".repeat(12_000),
        "(".repeat(12_000),
        "[".repeat(12_000),
        "\"".repeat(12_000),
        format!("%a{}", "{:b => ".repeat(1_500)),
        format!("= {}", "f(".repeat(4_000)),
        format!("- {}", "[".repeat(6_000)),
        format!("%p.{}", "a.".repeat(6_000)),
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

#[test]
fn hostile_haml_filters_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Haml);
    for filter in [":javascript", ":css", ":ruby", ":markdown"] {
        for body in ["(".repeat(12_000), "{".repeat(12_000), "\"".repeat(12_000)] {
            let (rows, _) = lex(&mut highlighter, &[filter, &format!("  {body}")]);
            let report = rows[1].1;
            assert!(
                report
                    .limits()
                    .all(|limit| matches!(limit, Limit::Depth | Limit::Work)),
                "{filter} reached {:?}",
                report.limits().collect::<Vec<_>>()
            );
        }
    }
}
