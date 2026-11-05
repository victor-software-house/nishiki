//! Hostile lines for Objective-C++ under the default limits.
//!
//! A line that would cost more than the default work ends at the work limit.
//! Its tokens still cover the whole line.
//! Scope resolution, destructors and template arguments have no `\b` before
//! them.
//! The long chains here check that no matcher walks a name from each of its
//! bytes.

use nishiki::{Highlighter, Language, Limit};

use super::lex;

#[test]
fn hostile_objective_cpp_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::ObjectiveCpp);
    for line in [
        "\"".repeat(12_000),
        "'".repeat(12_000),
        "(".repeat(12_000),
        "{".repeat(12_000),
        "[".repeat(12_000),
        "<".repeat(12_000),
        "@".repeat(12_000),
        "/*".repeat(6_000),
        "#".repeat(12_000),
        "::".repeat(6_000),
        format!("x = {}", "[a b:".repeat(3_000)),
        format!("template <{}", "typename T, ".repeat(1_000)),
        format!("R\"x({}", "a".repeat(12_000)),
        format!("@interface A{}", " : B".repeat(2_000)),
        format!("#define A{}", "(B)".repeat(3_000)),
        format!("{}x", " ".repeat(12_000)),
        format!("{}x", "\u{a0}".repeat(4_000)),
        "x".repeat(12_000),
        format!("{}x", "a::".repeat(4_000)),
        format!("{}~b(", "a::".repeat(4_000)),
        format!("{}::", "a".repeat(12_000)),
        format!("template <{}>", "a ".repeat(6_000)),
        format!("class A : public B{}", ", private C".repeat(1_000)),
        format!("using namespace {}", "a::".repeat(4_000)),
        format!("R\"{}(", "a".repeat(12_000)),
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
