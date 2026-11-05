//! Hostile lines and fences for the yaml lexer, under the default limits.

use nishiki::{Highlighter, Language, Limit, Limits, Work};

use super::{lex, steps_spent};

#[test]
fn hostile_yaml_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Yaml);
    for line in [
        "[".repeat(100_000),
        "{a: ".repeat(50_000),
        "{? ".repeat(50_000),
        "? : ".repeat(50_000),
        "&a ".repeat(100_000),
        "!t ".repeat(100_000),
        "- ".repeat(100_000),
        "a: ".repeat(100_000),
        "a:: ".repeat(100_000),
        format!("{}b: c", "a:: ".repeat(100_000)),
        format!("{}\t", "# ".repeat(100_000)),
        "\"\\".repeat(100_000),
        "x ".repeat(150_000),
        " ".repeat(300_000),
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

/// Each `a:` ends in an odd run of colons, so each looks ahead for a later key.
/// The lookahead reads the rest of the line once, not once per scalar.
#[test]
fn a_line_of_unpaired_colons_spends_work_linear_in_its_length() {
    for tail in ["", "b: c"] {
        let short = steps_spent(Language::Yaml, &format!("{}{tail}", "a:: ".repeat(1_000)));
        let long = steps_spent(Language::Yaml, &format!("{}{tail}", "a:: ".repeat(2_000)));
        assert!(
            long <= 2 * short + 16,
            "{tail:?} spent {short} steps at 1,000 scalars and {long} at 2,000"
        );
    }
}

#[test]
fn a_markdown_fence_runs_its_yaml_lines_under_the_same_limits() {
    let limits = Limits {
        work: Some(Work {
            per_byte: 0,
            base: 64,
        }),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Markdown, limits);
    let yaml = format!("a: {}", "&b ".repeat(200));
    let (rows, _) = lex(&mut highlighter, &["```yaml", &yaml, "```"]);
    assert!(rows[1].1.reached(Limit::Work));
    assert!(rows[2].1.is_complete());
}
