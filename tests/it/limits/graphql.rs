//! Hostile lines and fences for the graphql lexer, under the default limits.

use nishiki::{Highlighter, Language, Limit, Limits, Work};

use super::lex;

#[test]
fn hostile_graphql_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::GraphQl);
    for line in [
        "{".repeat(300_000),
        "{ a ".repeat(75_000),
        "(".repeat(300_000),
        "[".repeat(300_000),
        "\"".repeat(300_000),
        "\"\"\"".repeat(100_000),
        "#".repeat(300_000),
        "...".repeat(100_000),
        "... on ".repeat(40_000),
        "$a ".repeat(100_000),
        "@a(b: ".repeat(40_000),
        "query ".repeat(50_000),
        format!("type {}", "a ".repeat(100_000)),
        format!("type A implements {}", "& B ".repeat(75_000)),
        format!("a: {}", "[".repeat(300_000)),
        format!("{{ {}a", " ".repeat(300_000)),
        format!("{{ ${{ {}", "a ".repeat(100_000)),
        "${".repeat(150_000),
        "${a}".repeat(75_000),
        format!("{}x", "\u{a0}".repeat(100_000)),
        format!("{{ a{}", ":".repeat(300_000)),
        "x".repeat(300_000),
    ] {
        let (rows, _) = lex(&mut highlighter, &[&line]);
        let report = rows[0].1;
        assert!(
            report.limits().all(|limit| limit == Limit::Depth),
            "{:?} reached {:?}",
            &line[..4],
            report.limits().collect::<Vec<_>>()
        );
    }
}

/// A pinned depth counts the contexts open above GraphQL's root: at one,
/// the first selection opens and the one inside it stays closed.
#[test]
fn a_pinned_graphql_depth_opens_that_many_selections() {
    let limits = Limits {
        depth: Some(1),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::GraphQl, limits);
    let line = "{ a { b } }";
    let (rows, _) = lex(&mut highlighter, &[line]);
    let (tokens, report) = &rows[0];
    assert!(report.reached(Limit::Depth));
    let b = line.find('b').unwrap_or_default();
    let token = tokens
        .iter()
        .find(|token| token.end() > b)
        .copied()
        .unwrap_or_else(|| panic!("no token reaches {b}"));
    let selections = highlighter
        .scopes(token.stack())
        .filter(|&scope| scope == "meta.selectionset.graphql")
        .count();
    assert_eq!(selections, 1, "one selection open, the second kept closed");
}

/// Each interpolation's JavaScript spends the line's one work budget, so a
/// line of many runs out of work and its tokens still cover it.
#[test]
fn a_graphql_line_of_many_interpolations_runs_out_of_work() {
    let limits = Limits {
        work: Some(Work {
            per_byte: 0,
            base: 64,
        }),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::GraphQl, limits);
    let line = format!("{{ {}", "${a}".repeat(200));
    let (rows, _) = lex(&mut highlighter, &[&line]);
    assert!(rows[0].1.reached(Limit::Work));
}
