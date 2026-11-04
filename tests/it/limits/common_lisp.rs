//! Hostile lines for Common Lisp, under the default limits.
//! The `TextMate` engine runs the language.
//!
//! The engine interprets the grammar's own regular expressions.
//! A line that would cost more than the default work ends at the work limit.
//! Its tokens still cover the whole line.

use nishiki::{Highlighter, Language, Limit};

use super::lex;

#[test]
fn hostile_common_lisp_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::CommonLisp);
    for line in [
        "\"".repeat(12_000),
        "|".repeat(12_000),
        "(".repeat(12_000),
        ")".repeat(12_000),
        "#|".repeat(6_000),
        ";".repeat(12_000),
        "'".repeat(12_000),
        "`".repeat(12_000),
        ",@".repeat(6_000),
        "#'".repeat(6_000),
        "#\\".repeat(6_000),
        format!("(defun f ({}", "&optional ".repeat(1_000)),
        format!("(format t \"{}\")", "~a~%".repeat(3_000)),
        format!("{}x", ":".repeat(12_000)),
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
