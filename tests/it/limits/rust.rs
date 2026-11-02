//! Hostile lines and fences for the rust lexer, under the default limits.

use nishiki::{Highlighter, Language, Limit, Limits, State, Work};

use super::lex;

#[test]
fn hostile_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Rust);
    for line in [
        "/*".repeat(100_000),
        "f(".repeat(100_000),
        "a::<".repeat(100_000),
    ] {
        let (rows, _) = lex(&mut highlighter, &[&line]);
        let report = rows[0].1;
        assert!(
            report.limits().all(|limit| limit == Limit::Depth),
            "{:?} reached {:?}",
            &line[..8],
            report.limits().collect::<Vec<_>>()
        );
    }
}

#[test]
fn a_raw_string_takes_any_number_of_hashes() {
    let hashes = "#".repeat(300);
    let line = format!("r{hashes}\"x\"{hashes};");
    let mut highlighter = Highlighter::new(Language::Rust);
    let (rows, state) = lex(&mut highlighter, &[&line]);
    assert!(rows[0].1.is_complete());
    assert_eq!(state, State::default());
}

#[test]
fn a_markdown_fence_runs_its_rust_lines_under_the_same_limits() {
    let limits = Limits {
        work: Some(Work {
            per_byte: 0,
            base: 64,
        }),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Markdown, limits);
    let rust = format!("let s = \"{}\"; let t = 1;", r"\n".repeat(200));
    let (rows, _) = lex(&mut highlighter, &["```rust", &rust, "```"]);
    assert!(rows[1].1.reached(Limit::Work));
    assert!(rows[2].1.is_complete());
}
