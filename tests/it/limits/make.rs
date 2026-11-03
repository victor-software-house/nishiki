//! Hostile lines and fences for the make lexer, under the default limits.

use nishiki::{Highlighter, Language, Limit, Limits, Work};

use super::lex;

#[test]
fn hostile_make_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Make);
    for line in [
        "$(".repeat(100_000),
        "${".repeat(100_000),
        "$($".repeat(100_000),
        "$(a ".repeat(100_000),
        "$(shell ".repeat(50_000),
        "$(origin ".repeat(50_000),
        format!("$(origin {}", "a".repeat(300_000)),
        format!("$(origin {} ", " ".repeat(300_000)),
        format!("$(MAKE{}", " ".repeat(300_000)),
        "$(a,(".repeat(50_000),
        "$(call (".repeat(50_000),
        "$(call {".repeat(50_000),
        "\\\\".repeat(100_000),
        "#".repeat(300_000),
        " #".repeat(100_000),
        "a:".repeat(100_000),
        "a: ".repeat(100_000),
        "a:=".repeat(100_000),
        format!("{} = x", "a ".repeat(100_000)),
        format!("{} = x", "a+".repeat(100_000)),
        format!("\t{}", "@-+".repeat(100_000)),
        format!("define {}", "x ".repeat(100_000)),
        format!("else {}", "ifeq".repeat(75_000)),
        format!("else {} x", " ".repeat(300_000)),
        "\u{a0}".repeat(100_000),
        "$".repeat(300_000),
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

#[test]
fn a_markdown_fence_runs_its_makefile_lines_under_the_same_limits() {
    let limits = Limits {
        work: Some(Work {
            per_byte: 0,
            base: 64,
        }),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Markdown, limits);
    let make = format!("x = {}", "$(a) ".repeat(100));
    let (rows, _) = lex(&mut highlighter, &["```makefile", &make, "```"]);
    assert!(rows[1].1.reached(Limit::Work));
    assert!(rows[2].1.is_complete());
}

/// Each reference opens two contexts, the `$(` and the name inside it, so
/// three open contexts above the document hold one reference and the name
/// of a second.
#[test]
fn make_references_past_the_depth_limit_are_not_opened() {
    let limits = Limits {
        depth: Some(3),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Make, limits);
    let (rows, _) = lex(&mut highlighter, &["$(a $(b $(c $(d x"]);
    let (tokens, report) = &rows[0];
    assert!(report.reached(Limit::Depth));
    let last = tokens[tokens.len() - 1];
    assert_eq!(
        highlighter
            .scopes(last.stack())
            .filter(|name| *name == "string.interpolated.makefile")
            .count(),
        2,
        "the document's own context is not counted"
    );
}
