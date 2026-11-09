//! Hostile lines for ShaderLab under the default limits.
//!
//! A line that would cost more than the default work ends at the work limit,
//! with tokens that cover it, rather than running on.

use nishiki::{Highlighter, Language, Limit};

use super::lex;

#[test]
fn hostile_shaderlab_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::ShaderLab);
    for line in [
        "\"".repeat(12_000),
        format!("\"{}", "a".repeat(12_000)),
        "[".repeat(12_000),
        format!("[{}", "a".repeat(12_000)),
        format!("[{}] {}", "a".repeat(6_000), "b (\"".repeat(1_000)),
        format!("[a]{}b(\"", " ".repeat(12_000)),
        format!("{}(", "f".repeat(12_000)),
        format!("f{}(", " ".repeat(12_000)),
        "on off ".repeat(2_000),
        "1.".repeat(6_000),
        "CGPROGRAM ".repeat(1_500),
        format!("CGPROGRAM {}", "fixed4 ".repeat(2_000)),
        format!("CGPROGRAM {}", "/*".repeat(6_000)),
        "ENDCG ".repeat(2_000),
        format!("HLSLPROGRAM {}ENDHLSL", "UNITY_MATRIX_MVP ".repeat(700)),
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

#[test]
fn an_unterminated_comment_runs_on_across_many_lines_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::ShaderLab);
    let filler = "a line of text with no closer in it";
    let mut lines = vec!["CGPROGRAM /* open"];
    lines.extend(std::iter::repeat_n(filler, 2_000));
    let (rows, _) = lex(&mut highlighter, &lines);
    assert!(rows.iter().all(|(_, report)| report.is_complete()));
}
