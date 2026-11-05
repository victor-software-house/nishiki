//! Hostile lines for Common Lisp under the default limits.
//!
//! A line that would cost more than the default work ends at the work limit,
//! with tokens that cover it, rather than running on.

use nishiki::{Highlighter, Language, Limit};

use super::{lex, steps_spent};

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
        format!("\"{}", "~".repeat(12_000)),
        format!("\"{}", "~'".repeat(6_000)),
        format!("\"{}", "~(".repeat(6_000)),
        format!("\"{}\"", "~(~'~".repeat(3_000)),
        format!("\"~{}\"", "'~".repeat(6_000)),
        format!("\"~{}\"", "1,".repeat(6_000)),
        format!("\"~/{}\"", "a".repeat(12_000)),
        format!("\"{}\"", "~/a".repeat(4_000)),
        ",a".repeat(6_000),
        ",'a".repeat(4_000),
        format!("'{}", "a:".repeat(6_000)),
        format!("'{}", ":+a".repeat(4_000)),
        "(defun (".repeat(1_500),
        "(defun ".repeat(2_000),
        "#1#".repeat(4_000),
        "+a".repeat(6_000),
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

/// Hostile lines whose cost should grow with their length and no faster:
/// a head, a unit repeated, and a tail.
/// A matcher that searched the rest of the line again from each place fails
/// here.
const LINEAR: &[(&str, &str, &str)] = &[
    ("\"", "~", "\""),
    ("\"", "~'", "\""),
    ("\"~", "'~", "\""),
    ("\"", "~(~'~", "\""),
    ("\"~", "1,", "\""),
    ("\"", "~/a", "\""),
    ("", ",a", ""),
    ("", ",'a", ""),
    ("'", "a:", ""),
    ("'", ":+a", ""),
    ("", "(defun (", ""),
    ("", "+a", ""),
];

/// The line of `count` units between `head` and `tail`.
fn hostile(head: &str, unit: &str, tail: &str, count: usize) -> String {
    format!("{head}{}{tail}", unit.repeat(count))
}

#[test]
fn hostile_common_lisp_lines_spend_work_linear_in_their_length() {
    for &(head, unit, tail) in LINEAR {
        let short = steps_spent(Language::CommonLisp, &hostile(head, unit, tail, 1_000));
        let long = steps_spent(Language::CommonLisp, &hostile(head, unit, tail, 2_000));
        assert!(
            long <= 2 * short + 64,
            "{unit:?} spent {short} steps at 1,000 and {long} at 2,000"
        );
    }
}

/// Block comments nest, each level a context of its own.
/// Nested close to the default depth, they open and close without reaching
/// a limit, and the line after them is code again.
#[test]
fn deeply_nested_block_comments_close_within_the_depth_limit() {
    let mut highlighter = Highlighter::new(Language::CommonLisp);
    let open = "#|".repeat(4_000);
    let close = "|#".repeat(4_000);
    let (rows, _) = lex(&mut highlighter, &[&open, &close, "(car x)"]);
    for (_, report) in &rows {
        assert!(
            report.is_complete(),
            "{:?}",
            report.limits().collect::<Vec<_>>()
        );
    }
    let code = &rows[2].0;
    let scopes: Vec<String> = highlighter
        .scopes(code[1].stack())
        .map(ToString::to_string)
        .collect();
    assert_eq!(
        scopes,
        ["support.function.accessor.commonlisp", "source.commonlisp"]
    );
}
