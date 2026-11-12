//! Hostile lines for Protobuf under the default limits.
//!
//! Protobuf runs on matchers that `codegen/` writes.
//! The default-matchers design, section 2.4, names three kinds of hostile
//! line for them:
//!
//! - a bound line, about 12,000 bytes, drives a long run with no continuation,
//!   and must end under the work limit;
//! - a retry line, as long as the longest oracle fixture line, drives a pattern
//!   that takes a run and fails at each later start, and must end under the
//!   work limit;
//! - a limit line drives work that grows faster than the line, and must end at
//!   the work limit.
//!
//! Each line ends with tokens that cover it.

use nishiki::{Highlighter, Language, Limit};

use super::lex;

/// The limits that `line` reaches inside a message, after `message A {`.
fn limits_in_a_message(line: &str) -> Vec<Limit> {
    let mut highlighter = Highlighter::new(Language::Proto);
    let (rows, _) = lex(&mut highlighter, &["message A {", line]);
    rows[1].1.limits().collect()
}

/// Bound lines: long runs of one pattern's characters, with no continuation.
/// None reaches the work limit.
#[test]
fn hostile_proto_lines_end_under_the_work_limit() {
    let mut highlighter = Highlighter::new(Language::Proto);
    for line in [
        "\"".repeat(12_000),
        "'".repeat(12_000),
        "{".repeat(12_000),
        "<".repeat(12_000),
        "[".repeat(12_000),
        "/*".repeat(6_000),
        "a.".repeat(6_000),
        "= 1;".repeat(3_000),
        format!("message {}", "A_".repeat(6_000)),
        format!("option ({}", "a.".repeat(6_000)),
        format!("option a{}", ".a".repeat(6_000)),
        format!("option (a{})", ".a".repeat(6_000)),
        format!("a{} =", "\t".repeat(12_000)),
        format!("map<{}", "string, map<".repeat(1_000)),
        format!("{}//", " ".repeat(12_000)),
        format!("{}x", "\u{a0}".repeat(4_000)),
        format!("{}x", "\u{3000}".repeat(4_000)),
        "x".repeat(12_000),
    ] {
        let (rows, _) = lex(&mut highlighter, &[&line]);
        let report = rows[0].1;
        assert!(
            report.limits().all(|limit| limit == Limit::Depth),
            "{:?} reached {:?}",
            line.chars().take(4).collect::<String>(),
            report.limits().collect::<Vec<_>>()
        );
    }
}

/// A retry line: a field whose type runs to the line's end, with no name
/// after it.
/// The field pattern takes the run and fails, and the scanner tries it again
/// at each later start.
///
/// At 101 bytes the line is about as long as the longest Protocol Buffers
/// fixture line, of 102 bytes.
/// The scan skip and the arm drop of `field-pattern-fixes` keep it under the
/// work limit, as the hand matchers did.
#[test]
fn a_failing_field_retried_at_every_start_ends_under_the_work_limit() {
    let line = format!("{} ", "a.".repeat(50));
    let limits = limits_in_a_message(&line);
    assert!(
        limits.iter().all(|&limit| limit == Limit::Depth),
        "reached {limits:?}"
    );
}

/// A bound line of the same field, at 12,001 bytes.
/// After a failure, the scan skip passes every start the failed run proves
/// fails.
/// So the line costs steps linear in its length, and Shiki lexes it uncut
/// too.
#[test]
fn a_long_failing_field_ends_under_the_work_limit() {
    let line = format!("{} ", "a.".repeat(6_000));
    let limits = limits_in_a_message(&line);
    assert!(
        limits.iter().all(|&limit| limit == Limit::Depth),
        "reached {limits:?}"
    );
}

/// A limit line: no-break spaces, each followed by a space.
///
/// The scan passes a run of one blank character in one step, but here each
/// run is one character long.
/// So the scan tries every no-break space.
/// At each, a pattern that begins with `\s*` takes the rest of the line, and
/// the work grows with the square of the line's length.
#[test]
fn no_break_spaces_between_spaces_end_at_the_work_limit() {
    let line = format!("{}x", "\u{a0} ".repeat(4_000));
    let limits = limits_in_a_message(&line);
    assert!(limits.contains(&Limit::Work), "reached {limits:?}");
}
