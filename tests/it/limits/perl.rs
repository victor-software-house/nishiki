//! Hostile lines for Perl under the default limits.
//!
//! A line that would cost more than the default work ends at the work limit,
//! with tokens that cover it, rather than running on.
//! The same holds for the HTML, SQL and other languages a heredoc hands its
//! lines to.

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
        format!("q{}", "{a".repeat(4_000)),
        format!("s{}{{b}}", "{a".repeat(4_000)),
        format!("<<\"{}\";", "L".repeat(12_000)),
        format!("<<~{}", "L".repeat(12_000)),
        "print <<A, <<B, <<'C', <<\"D\";".repeat(1_000),
        format!("tr/{}/b/", "\\/".repeat(4_000)),
        format!("y/{}", "\\".repeat(12_000)),
        "s/".repeat(6_000),
        "&/(/{/|/~/".repeat(1_200),
        "if /unless /x/ ".repeat(800),
        "a/b/ ".repeat(2_400),
        format!("{}/", "m".repeat(12_000)),
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

/// Each heredoc opener lexes the rest of its line again, with the next
/// opener in it.
/// Past 64 such levels the rest keeps the scopes around it, and the line
/// reports the depth limit, where Shiki goes on.
#[test]
fn a_line_of_heredoc_openers_reports_the_depth_limit() {
    let mut highlighter = Highlighter::new(Language::Perl);
    let line = "print <<A, <<B, <<'C', <<\"D\";".repeat(1_000);
    let (rows, _) = lex(&mut highlighter, &[&line]);
    let report = rows[0].1;
    assert!(
        report.limits().any(|limit| matches!(limit, Limit::Depth)),
        "{:?}",
        report.limits().collect::<Vec<_>>()
    );
}
