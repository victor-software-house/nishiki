//! Hostile lines and fences for the HTML lexer, under the default limits.

use nishiki::{Highlighter, Language, Limit};

use super::{lex, steps_spent};

#[test]
fn hostile_html_lines_finish_within_the_default_limits() {
    for line in [
        "<".repeat(300_000),
        "<a ".repeat(100_000),
        "<a b=".repeat(60_000),
        "<a b=\"".repeat(40_000),
        "<a b='".repeat(40_000),
        "<a b=\"<".repeat(40_000),
        "<a ></a>".repeat(40_000),
        "<div ></div>".repeat(25_000),
        "</".repeat(150_000),
        "</a ".repeat(75_000),
        format!("<a{}b=c", " ".repeat(300_000)),
        format!("<a b{}", " ".repeat(300_000)),
        format!("<a b={}", " ".repeat(300_000)),
        format!("<a b={}x", "\u{a0}".repeat(100_000)),
        format!("<a{}x", " b".repeat(150_000)),
        format!("<a {}", "b = ".repeat(75_000)),
        format!("<a {}", "b=c ".repeat(75_000)),
        format!("<{}", "a".repeat(300_000)),
        format!("<a {}", "\u{e9}".repeat(150_000)),
        format!("<a-{}", "b".repeat(300_000)),
        "<a-b ".repeat(60_000),
        "<a b=\"&amp;\" ".repeat(25_000),
        "&".repeat(300_000),
        "&a".repeat(150_000),
        "&amp".repeat(75_000),
        "&amp;".repeat(60_000),
        "&#x".repeat(100_000),
        "&#1".repeat(100_000),
        "<!--".repeat(75_000),
        "<!-- -- ".repeat(40_000),
        "<!---->".repeat(40_000),
        "--!>".repeat(75_000),
        "<![CDATA[".repeat(35_000),
        "]]>".repeat(100_000),
        "<!DOCTYPE ".repeat(30_000),
        "<!DOCTYPE a \"".repeat(25_000),
        "<?xml ".repeat(50_000),
        "<?xml a=b ?>".repeat(25_000),
        "<style>".repeat(40_000),
        "<style>a{".repeat(30_000),
        "<style></style>".repeat(20_000),
        "<style>a{}</style>".repeat(20_000),
        "  <style>".repeat(30_000),
        "<script>".repeat(40_000),
        "<script>{".repeat(30_000),
        "<script></script>".repeat(20_000),
        "<script src=x></script>".repeat(12_000),
        "<script type=text/html>".repeat(12_000),
        "<script type=json>".repeat(15_000),
        "<script type=".repeat(20_000),
        "<script a b c ".repeat(20_000),
        "<script>//</script>".repeat(15_000),
        "<script>/*</script>".repeat(15_000),
        "<svg ".repeat(60_000),
        "<svg><g ".repeat(30_000),
        "<svg><g a=\"".repeat(25_000),
        "<svg><g/>".repeat(30_000),
        "<svg><g></g>".repeat(25_000),
        "<svg><foo ".repeat(30_000),
        "<svg>".repeat(60_000),
        "<math><mi>".repeat(30_000),
        format!("<svg>{}", "<g ".repeat(100_000)),
        format!("<svg>{}", "<g a=\"b\" ".repeat(30_000)),
        "<a onclick=\"".repeat(25_000),
        "<a onclick='//".repeat(25_000),
        "<a onclick=\"/*".repeat(25_000),
        "<a onclick=\"f(\" ".repeat(20_000),
        "<a style=\"".repeat(30_000),
        "<a style=x ".repeat(30_000),
        "<a onclick=x ".repeat(30_000),
        "<a onclick=".repeat(30_000),
        "<a data-x=\"".repeat(30_000),
        "<p>".repeat(100_000),
        "<p ".repeat(100_000),
        "x".repeat(300_000),
        " ".repeat(300_000),
        "\"".repeat(300_000),
        "'".repeat(300_000),
        "=".repeat(300_000),
        ">".repeat(300_000),
        "/".repeat(300_000),
    ] {
        let mut highlighter = Highlighter::new(Language::Html);
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

/// The steps a line of event handlers spends grow with its length and no
/// faster. Each handler is a JavaScript text of its own that ends where the
/// handler does. A search per handler would read the line up to each
/// handler.
#[test]
fn a_line_of_event_handlers_spends_work_linear_in_its_length() {
    let short = steps_spent(Language::Html, &"<a onclick=\"".repeat(1_000));
    let long = steps_spent(Language::Html, &"<a onclick=\"".repeat(2_000));
    assert!(
        long <= 2 * short + 16,
        "spent {short} steps at 1,000 handlers and {long} at 2,000"
    );
}
