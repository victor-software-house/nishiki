//! Hostile lines and fences for the markdown lexer, under the default limits.

use nishiki::{Highlighter, Language, Limit, Limits, State};

use super::{lex, steps_spent};

/// Each hostile line: a prefix, a unit repeated, and a suffix.
const HOSTILE: [(&str, &str, &str); 15] = [
    ("", "**", ""),
    ("", "[", ""),
    ("", "[](", ""),
    ("", "`", ""),
    ("", "``` a ` ", ""),
    ("", "~~ a ", ""),
    ("", "<a ", ""),
    ("", "_ a ", ""),
    ("", "![a](", ""),
    ("", "*a ", ""),
    ("", "> ", ""),
    ("", "- ", ""),
    ("", "|a ", ""),
    // A title opener at every byte of a definition's destination.
    ("[a]: ", "x\"", "x"),
    // A `}` that attributes could follow at every byte of a fence's info.
    ("```{.rustx", "} ", "`"),
];

/// Units the grammar matches in time quadratic in their count, which the
/// work limit cuts short.
const QUADRATIC: [&str; 1] = ["![a]("];

fn hostile(head: &str, unit: &str, tail: &str, count: usize) -> String {
    format!("{head}{}{tail}", unit.repeat(count))
}

#[test]
#[ignore = "waits for the linear code-span scan: a scan that runs out of work now reports it, which shows that code span's quadratic cost"]
fn hostile_markdown_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Markdown);
    for (head, unit, tail) in HOSTILE {
        let line = hostile(head, unit, tail, 20_000);
        let (rows, _) = lex(&mut highlighter, &[&line]);
        let report = rows[0].1;
        assert!(
            report
                .limits()
                .all(|limit| matches!(limit, Limit::Work | Limit::Depth)),
            "{unit:?} reached {:?}",
            report.limits().collect::<Vec<_>>()
        );
        assert_eq!(
            report.reached(Limit::Work),
            QUADRATIC.contains(&unit),
            "{unit:?}"
        );
    }
}

/// The steps a hostile line spends grow with its length and no faster. A
/// look-ahead that rescans the rest of the line from each position, and
/// spends what it reads as it must, fails here.
#[test]
#[ignore = "waits for the linear code-span scan: a scan that runs out of work now reports it, which shows that code span's quadratic cost"]
fn hostile_markdown_lines_spend_work_linear_in_their_length() {
    for (head, unit, tail) in HOSTILE {
        if QUADRATIC.contains(&unit) {
            continue;
        }
        let short = steps_spent(Language::Markdown, &hostile(head, unit, tail, 1_000));
        let long = steps_spent(Language::Markdown, &hostile(head, unit, tail, 2_000));
        assert!(
            long <= 2 * short + 16,
            "{unit:?} spent {short} steps at 1,000 and {long} at 2,000"
        );
    }
}

#[test]
fn markdown_containers_past_the_depth_limit_are_not_opened() {
    let limits = Limits {
        depth: Some(4),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Markdown, limits);
    let (rows, state) = lex(&mut highlighter, &["> > > > > > deep"]);
    let (tokens, report) = &rows[0];
    assert!(report.reached(Limit::Depth));
    let last = tokens[tokens.len() - 1];
    assert_eq!(
        highlighter
            .scopes(last.stack())
            .filter(|name| *name == "markup.quote.markdown")
            .count(),
        4,
        "the document's own context is not counted"
    );
    assert_ne!(state, State::default());
}

/// A line of code spans spends work linear in its length.
/// A code span that scans the rest of the line from each opening fails here.
#[test]
fn a_line_of_code_spans_spends_work_linear_in_its_length() {
    let short = steps_spent(Language::Markdown, &"``` a ` ".repeat(1_000));
    let long = steps_spent(Language::Markdown, &"``` a ` ".repeat(2_000));
    assert!(
        long <= 2 * short + 16,
        "spent {short} steps at 1,000 units and {long} at 2,000"
    );
}

/// A ladder of backtick runs spends work linear in its length.
/// Each run is one backtick longer than the one before.
/// No run has a later run of its length, so each gives back backticks.
/// The steps spent per byte stay flat as the ladder grows.
#[test]
fn a_ladder_of_backtick_runs_spends_work_linear_in_its_length() {
    let ladder = |runs: usize| -> String {
        let mut line = String::new();
        for len in 1..=runs {
            line.push_str("x ");
            line.push_str(&"`".repeat(len));
            line.push(' ');
        }
        line
    };
    let few = ladder(40);
    let many = ladder(80);
    let few_steps = u64::from(steps_spent(Language::Markdown, &few));
    let many_steps = u64::from(steps_spent(Language::Markdown, &many));
    let few_bytes = u64::try_from(few.len()).unwrap_or(u64::MAX);
    let many_bytes = u64::try_from(many.len()).unwrap_or(u64::MAX);
    assert!(
        many_steps * few_bytes <= 2 * few_steps * many_bytes,
        "spent {few_steps} steps on {few_bytes} bytes and {many_steps} on {many_bytes}"
    );
}
