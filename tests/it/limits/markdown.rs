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
