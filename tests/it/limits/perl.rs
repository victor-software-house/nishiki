//! Hostile lines for Perl, which the `TextMate` engine runs, under the default
//! limits.
//!
//! The engine interprets the grammar's own regular expressions, so a line
//! that would cost it more than the default work ends at the work limit, with
//! tokens that cover it, rather than running on. The same holds for the HTML,
//! SQL and other languages the engine hands heredocs to.

use nishiki::{Highlighter, Language, Limit};

use super::lex;

#[test]
fn hostile_perl_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Perl);
    for line in [
        "\"".repeat(12_000),
        "'".repeat(12_000),
        "(".repeat(12_000),
        "{".repeat(12_000),
        "[".repeat(12_000),
        "$".repeat(12_000),
        "<<HTML ".repeat(2_000),
        "qw(".repeat(4_000),
        "=pod ".repeat(2_000),
        format!("$a = {}", "f(".repeat(4_000)),
        format!("\"{}\"", "@{[ ".repeat(3_000)),
        format!("s/{}//", "(a)".repeat(4_000)),
        format!("sub a {}", "{ ".repeat(3_000)),
        format!("{}#", " ".repeat(12_000)),
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
