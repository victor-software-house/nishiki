//! Hostile lines for TeX under the default limits.
//!
//! A line that would cost more than the default work ends at the work limit,
//! with tokens that cover it, rather than running on.
//! Command names are measured as a run of letters.
//! Long names and long runs of backslashes check that no matcher walks a run
//! from each of its bytes.

use nishiki::{Highlighter, Language, Limit};

use super::lex;

#[test]
fn hostile_tex_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Tex);
    for line in [
        "\\".repeat(12_000),
        "$".repeat(12_000),
        "{".repeat(12_000),
        "}".repeat(12_000),
        "%".repeat(12_000),
        format!("\\{}", "a".repeat(12_000)),
        format!("\\{}:n", "a_".repeat(6_000)),
        format!("${}$", "\\alpha".repeat(2_000)),
        format!("${}", "\\left(".repeat(2_000)),
        format!("${}", "1.".repeat(6_000)),
        format!("{}\\iffalse", " ".repeat(12_000)),
        format!("\\iffalse{}", " ".repeat(12_000)),
        format!("\\catcode`{}", "\\".repeat(12_000)),
        format!("{}x", "\u{a0}".repeat(4_000)),
        " ".repeat(12_000),
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

/// Each `{` opens a rule that a `}` closes, so a long run of open braces
/// stops at the depth limit and each line still finishes.
#[test]
fn nested_braces_stop_at_the_depth_limit() {
    let mut highlighter = Highlighter::new(Language::Tex);
    let lines = vec!["{"; 5_000];
    let (rows, _) = lex(&mut highlighter, &lines);
    for (_, report) in rows {
        assert!(
            report
                .limits()
                .all(|limit| matches!(limit, Limit::Depth | Limit::Work)),
            "reached {:?}",
            report.limits().collect::<Vec<_>>()
        );
    }
}
