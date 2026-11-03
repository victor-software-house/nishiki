//! Hostile lines for ERB, under the default limits.

use nishiki::{Highlighter, Language, Limit};

use super::lex;

#[test]
fn hostile_erb_lines_finish_within_the_default_limits() {
    for line in [
        "<%".repeat(150_000),
        "%>".repeat(150_000),
        "<%%".repeat(100_000),
        "<%=".repeat(100_000),
        "<%#".repeat(100_000),
        "<% %>".repeat(60_000),
        "<%= x %>".repeat(40_000),
        "<%# x %>".repeat(40_000),
        "<% x -%>".repeat(40_000),
        "<% # %>".repeat(40_000),
        "<% \"%>\" %>".repeat(30_000),
        format!("<%{}", "%".repeat(300_000)),
        format!("<%{}>", "%".repeat(300_000)),
        format!("<% {}", "#".repeat(300_000)),
        format!("<% #{}", "%".repeat(300_000)),
        format!("{}<%", " ".repeat(300_000)),
        format!("{}<%#", " ".repeat(300_000)),
        "<a href=\"<%= x %>\">".repeat(15_000),
        "<a href=\"<%".repeat(25_000),
        "<style><%".repeat(25_000),
        "<style><% x %>".repeat(20_000),
        "<script><%= x %>".repeat(15_000),
        "<!-- <% -->".repeat(25_000),
        "<".repeat(300_000),
    ] {
        let mut highlighter = Highlighter::new(Language::Erb);
        let (rows, _) = lex(&mut highlighter, &[&line]);
        let report = rows[0].1;
        assert!(
            report.limits().all(|limit| limit == Limit::Depth),
            "{:?}... reached {:?}",
            line.chars().take(24).collect::<String>(),
            report.limits().collect::<Vec<_>>()
        );
    }
}
