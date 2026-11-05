//! Hostile lines for Pascal under the default limits.
//!
//! A line that would cost more than the default work ends at the work limit,
//! with tokens that cover it, rather than running on.

use nishiki::{Highlighter, Language, Limit};

use super::lex;

#[test]
fn hostile_pascal_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Pascal);
    for line in [
        "'".repeat(12_000),
        format!("'{}'", "''".repeat(6_000)),
        format!("x := '{}", "a".repeat(12_000)),
        format!("{{{}", "comment ".repeat(1_500)),
        format!("(*{}", "comment ".repeat(1_500)),
        "{".repeat(12_000),
        "{$".repeat(6_000),
        "(*".repeat(6_000),
        "/".repeat(12_000),
        "#".repeat(12_000),
        "$".repeat(12_000),
        format!("procedure {}", "a.".repeat(6_000)),
        format!("function f{};", "(".repeat(12_000)),
        format!("function f({}); forward", "a".repeat(12_000)),
        format!("{}//", " ".repeat(12_000)),
        format!("x := {}", "1.".repeat(6_000)),
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
fn an_unterminated_string_runs_on_across_many_lines_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Pascal);
    let filler = "a line of text with no quote in it";
    let mut lines = vec!["s := 'open"];
    lines.extend(std::iter::repeat_n(filler, 2_000));
    let (rows, _) = lex(&mut highlighter, &lines);
    assert!(rows.iter().all(|(_, report)| report.is_complete()));
}
