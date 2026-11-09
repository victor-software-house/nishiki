//! Hostile lines for MATLAB under the default limits.
//!
//! A line that would cost more than the default work ends at the work limit.
//! Its tokens still cover the whole line.
//!
//! Command syntax is tried at the start of every line, and indexing braces
//! and validators have no `\b` before their name.
//! The long names, argument lists and blank lines here check that no matcher
//! walks a run from each of its bytes.

use nishiki::{Highlighter, Language, Limit};

use super::lex;

#[test]
fn hostile_matlab_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Matlab);
    for line in [
        "\"".repeat(12_000),
        "'".repeat(12_000),
        "(".repeat(12_000),
        "[".repeat(12_000),
        "{".repeat(12_000),
        "%".repeat(12_000),
        "...".repeat(4_000),
        format!("a = {}", "f(".repeat(4_000)),
        format!("x = {}", "a'".repeat(6_000)),
        format!("if a {}", "if b ".repeat(2_000)),
        format!("{}%", " ".repeat(12_000)),
        format!("{}x", "\u{a0}".repeat(4_000)),
        "x".repeat(12_000),
        format!("{} x", "a".repeat(12_000)),
        format!("hold {}", "on ".repeat(4_000)),
        format!("disp .{}", "a ".repeat(6_000)),
        format!("disp .x{}", "'a' ".repeat(3_000)),
        " ".repeat(12_000),
        format!("{}{{", "a.".repeat(6_000)),
        format!("x = {}", "1e5".repeat(4_000)),
        format!("x = '{}'", "%d".repeat(6_000)),
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

/// Nested block comments open a rule per `%{`, so they stop at the depth
/// limit and each line still finishes.
#[test]
fn deeply_nested_block_comments_stop_at_the_depth_limit() {
    let mut highlighter = Highlighter::new(Language::Matlab);
    let lines = vec!["%{"; 5_000];
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
