//! Hostile lines for the XML lexer, under the default limits, and its
//! pinned depth.

use nishiki::{Highlighter, Language, Limit, Limits};

use super::lex;

#[test]
fn hostile_xml_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Xml);
    for line in [
        "<".repeat(300_000),
        "<a ".repeat(100_000),
        "<a b=".repeat(60_000),
        "<a b=\"".repeat(40_000),
        "<a ></a>".repeat(40_000),
        format!("{}></a>", "<a ".repeat(50_000)),
        "<a:b:c ".repeat(40_000),
        "<?a ".repeat(75_000),
        "<?a b ".repeat(50_000),
        "<!DOCTYPE a ".repeat(25_000),
        "<!DOCTYPE a [<!ENTITY ".repeat(14_000),
        "<!ENTITY % ".repeat(30_000),
        "<!--".repeat(75_000),
        "<!-- -- ".repeat(40_000),
        "<![CDATA[".repeat(35_000),
        "<%".repeat(150_000),
        "<%@".repeat(100_000),
        "<% %>".repeat(60_000),
        "<% \"%>".repeat(50_000),
        "<% /* %>".repeat(40_000),
        "&".repeat(300_000),
        "&a".repeat(150_000),
        "&#x".repeat(100_000),
        "\"".repeat(300_000),
        "'".repeat(300_000),
        format!("<a{}b=c", " ".repeat(300_000)),
        format!("<a{}x", "\u{a0}".repeat(100_000)),
        format!("<a{}x", " b".repeat(150_000)),
        format!("<a {}", "b = ".repeat(75_000)),
        format!("<{}", "a".repeat(300_000)),
        format!("<{}", "\u{e9}".repeat(150_000)),
        format!("<a {}", "\u{e9}".repeat(150_000)),
        format!("<a {}>", "\u{e9}=".repeat(100_000)),
        format!("<a{}></a", " x".repeat(100_000)),
        format!("<a>{}", "></a".repeat(75_000)),
        format!("<{}:", "a".repeat(300_000)),
        "x".repeat(300_000),
    ] {
        let (rows, _) = lex(&mut highlighter, &[&line]);
        let report = rows[0].1;
        assert!(
            report.limits().all(|limit| limit == Limit::Depth),
            "{:?} reached {:?}",
            line.chars().take(8).collect::<String>(),
            report.limits().collect::<Vec<_>>()
        );
        highlighter = Highlighter::new(Language::Xml);
    }
}

/// A pinned depth counts the rules open above the document's root, as in every
/// language.
///
/// A depth of one opens a tag and keeps its quoted value plain. A depth of two
/// opens both.
#[test]
fn xml_rules_past_the_depth_limit_are_not_opened() {
    for (depth, string_opens) in [(1, false), (2, true)] {
        let limits = Limits {
            depth: Some(depth),
            ..Limits::default()
        };
        let mut highlighter = Highlighter::new_with_limits(Language::Xml, limits);
        let (rows, _) = lex(&mut highlighter, &["<a b=\"c"]);
        let (tokens, report) = &rows[0];
        let has = |name: &str| {
            tokens
                .iter()
                .any(|token| highlighter.scopes(token.stack()).any(|scope| scope == name))
        };
        assert!(has("meta.tag.xml"), "depth {depth}");
        assert_eq!(
            has("string.quoted.double.xml"),
            string_opens,
            "depth {depth}"
        );
        assert_eq!(report.reached(Limit::Depth), !string_opens, "depth {depth}");
    }
}
