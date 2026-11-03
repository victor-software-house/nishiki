//! Hostile lines for the ruby lexer, under the default limits.

use nishiki::{Highlighter, Language, Limit, Limits, Work};

use super::lex;

#[test]
fn hostile_ruby_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Ruby);
    // `prefix`, then `unit` `count` times, then `suffix`.
    let line = |prefix: &str, unit: &str, count: usize, suffix: &str| {
        format!("{prefix}{}{suffix}", unit.repeat(count))
    };
    let lines = [
        line("", "\"", 300_000, ""),
        line("", "'", 300_000, ""),
        line("", "`", 300_000, ""),
        line("", "\\", 150_000, ""),
        line("", "/", 300_000, ""),
        line("", " /a", 100_000, ""),
        line("", "/\\", 150_000, ""),
        line("x = ", "/", 300_000, ""),
        line("", "(", 300_000, ""),
        line("", "[", 300_000, ""),
        line("", "{", 300_000, ""),
        line("", "{ |", 100_000, ""),
        line("", "do |", 60_000, ""),
        line("", "|a, ", 60_000, ""),
        line("", "#{", 150_000, ""),
        line("\"", "#{", 150_000, ""),
        line("\"", "#@a", 100_000, ""),
        line("", "%w(", 100_000, ""),
        line("", "%r{", 100_000, ""),
        line("", "%q|", 100_000, ""),
        line("", "%", 300_000, ""),
        line("", "?", 300_000, ""),
        line("", "?\\C-", 60_000, ""),
        line("", ":", 300_000, ""),
        line("", ":a", 150_000, ""),
        line("", "a:", 150_000, ""),
        line("", "@", 300_000, ""),
        line("", "$", 300_000, ""),
        line("", "<<", 150_000, ""),
        line("", "<<A,", 60_000, ""),
        line("x = ", "<<A, ", 50_000, "<<B"),
        line("", "def ", 60_000, ""),
        line("def a ", "b, ", 60_000, ""),
        line("def a(", "b, ", 60_000, ""),
        line("", "->(", 60_000, ""),
        line("", "class ", 50_000, ""),
        line("class A", "::B", 50_000, ""),
        line("", "a.", 150_000, ""),
        line("", "a::", 100_000, ""),
        line("", "&.", 150_000, ""),
        line("", "1_", 150_000, ""),
        line("", "1.", 150_000, ""),
        line("", "0x1_", 70_000, ""),
        line("", "1e+", 70_000, ""),
        line("if ", "(", 100_000, "a = 1"),
        line("", "a = ", 60_000, ""),
        line("", "# @param ", 30_000, ""),
        line("# @option ", "a ", 60_000, ""),
        line("# @see ", " ", 100_000, ""),
        line("# @param a [", "]", 100_000, ""),
        line("# @!macro [", "] ", 60_000, ""),
        line("", " ", 300_000, "x"),
        line("", "\u{e9}", 150_000, ""),
        line("", "x", 300_000, ""),
        line("", "x", 300_000, "("),
        line("", "x", 300_000, ":"),
    ];
    let mut failures = Vec::new();
    for line in lines {
        let (rows, _) = lex(&mut highlighter, &[&line]);
        let report = rows[0].1;
        if !report.limits().all(|limit| limit == Limit::Depth) {
            failures.push(format!(
                "{:?} reached {:?}",
                &line[..12.min(line.len())],
                report.limits().collect::<Vec<_>>()
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn a_sql_heredoc_line_of_many_interpolations_runs_out_of_work() {
    let limits = Limits {
        work: Some(Work {
            per_byte: 0,
            base: 64,
        }),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Ruby, limits);
    let body = format!("select {}", "#{}".repeat(5_000));
    let (rows, _) = lex(&mut highlighter, &["x = <<~SQL", &body, "SQL"]);
    assert!(rows[1].1.reached(Limit::Work));
}
