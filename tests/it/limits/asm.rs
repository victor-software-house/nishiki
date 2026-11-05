//! Hostile lines for x86 assembly under the default limits.
//!
//! A line that would cost more than the default work ends at the work limit,
//! with tokens that cover it, rather than running on.

use nishiki::{Highlighter, Language, Limit};

use super::{lex, steps_spent};

#[test]
fn hostile_asm_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Asm);
    for line in [
        "\"".repeat(12_000),
        "'".repeat(12_000),
        "[".repeat(12_000),
        "%".repeat(12_000),
        "$".repeat(12_000),
        ";".repeat(12_000),
        "mov ".repeat(3_000),
        "0x".repeat(6_000),
        format!("mov rax, {}", "[rbx+".repeat(2_000)),
        format!("db {}", "0, ".repeat(4_000)),
        format!("label{}:", "_a".repeat(4_000)),
        format!("%macro {}", "%% ".repeat(3_000)),
        format!("{};", " ".repeat(12_000)),
        format!("{}x", "\u{a0}".repeat(4_000)),
        "x".repeat(12_000),
        "a.".repeat(6_000),
        "__?".repeat(4_000),
        "..@a".repeat(3_000),
        "$a$".repeat(4_000),
        "0b1.".repeat(3_000),
        "1_".repeat(6_000),
        format!("%define X {}", "%strlen(".repeat(1_500)),
        format!("%define X {}\\", " ".repeat(12_000)),
        format!("{}#", " ".repeat(12_000)),
        " # ".repeat(4_000),
        "/*".repeat(6_000),
        "\u{fb01}ld ".repeat(2_000),
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
    ("", "x", ""),
    ("", "a.", ""),
    ("", "__?", ""),
    ("", "..@a", ""),
    ("", "$a$", ""),
    ("", "0b1.", ""),
    ("", "1_", ""),
    ("", "mov ", ""),
    ("%define X ", "%strlen(", ""),
    ("", " # ", ""),
    ("", "\u{fb01}ld ", ""),
];

/// The line of `count` units between `head` and `tail`.
fn hostile(head: &str, unit: &str, tail: &str, count: usize) -> String {
    format!("{head}{}{tail}", unit.repeat(count))
}

/// The steps a line's last units may miss, since no unit follows them.
/// It is a fixed term per line.
const LINE_END_SLACK: u32 = 64;

/// The number of the grammar's `__` word lists, such as the one that holds
/// `__?FILE?__`, which read on past each `?` that ends a run of `\w`.
const MARKED_LISTS: u32 = 14;

/// The longest word of those lists, `___?NASM_PATCHLEVEL?__`.
const LONGEST_MARKED_WORD: u32 = 22;

/// The steps a line's last places miss in those lists.
/// Each list reads at most its longest word past a place, so a place within
/// that many bytes of the line's end reads less than the rest.
/// That shortfall is a fixed term per line.
const NEAR_END_SHORTFALL: u32 = MARKED_LISTS * LONGEST_MARKED_WORD * LONGEST_MARKED_WORD;

#[test]
fn hostile_asm_lines_spend_work_linear_in_their_length() {
    for &(head, unit, tail) in LINEAR {
        let slack = if unit == "__?" {
            NEAR_END_SHORTFALL
        } else {
            LINE_END_SLACK
        };
        let short = steps_spent(Language::Asm, &hostile(head, unit, tail, 1_000));
        let long = steps_spent(Language::Asm, &hostile(head, unit, tail, 2_000));
        assert!(
            long <= 2 * short + slack,
            "{unit:?} spent {short} steps at 1,000 and {long} at 2,000"
        );
    }
}
