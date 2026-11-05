//! Hostile lines for HAML under the default limits.
//!
//! A line that would cost more than the default work ends at the work limit,
//! with tokens that cover it, rather than running on.
//! The same holds for the lines HAML hands to the Ruby, JavaScript, CSS and
//! Markdown lexers.

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
        format!("%p= \"{}\"", "#{".repeat(3_000)),
        format!("%p= \"#{{{}}}\"", "{".repeat(6_000)),
        format!("%a{{{}}}", "b: 1, ".repeat(2_000)),
        format!("%a[{}]", "b, ".repeat(3_000)),
        format!("{}-#", " ".repeat(12_000)),
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

#[test]
fn a_long_filter_body_and_nested_comment_blocks_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Haml);
    let body = "  var a = 1;";
    let mut lines = vec![":javascript"];
    lines.extend(std::iter::repeat_n(body, 2_000));
    let (rows, _) = lex(&mut highlighter, &lines);
    assert!(rows.iter().all(|(_, report)| report.is_complete()));
    let nested: Vec<String> = (0..500)
        .map(|depth| format!("{}-#", " ".repeat(depth)))
        .collect();
    let nested: Vec<&str> = nested.iter().map(String::as_str).collect();
    let (rows, _) = lex(&mut highlighter, &nested);
    assert!(rows.iter().all(|(_, report)| {
        report
            .limits()
            .all(|limit| matches!(limit, Limit::Depth | Limit::Work))
    }));
}
