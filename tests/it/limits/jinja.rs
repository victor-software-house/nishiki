//! Hostile lines for Jinja under the default limits.
//!
//! A line that would cost more than the default work ends at the work limit,
//! with tokens that cover it, rather than running on.

use nishiki::{Highlighter, Language, Limit};

use super::lex;

#[test]
fn hostile_jinja_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Jinja);
    for line in [
        "{{".repeat(6_000),
        "{%".repeat(6_000),
        "{#".repeat(6_000),
        format!("{}x", "{# ".repeat(4_000)),
        format!("{}{}", "{# ".repeat(4_000), "#} ".repeat(4_000)),
        format!("{{{{ {}", "{".repeat(12_000)),
        format!("{{{{ {} }}}}", "{\"k\": ".repeat(3_000)),
        format!("{{{{ {}", "{\"a\": 1} ".repeat(2_000)),
        format!("{{{{ {}", "[".repeat(12_000)),
        format!("{{{{ {}", "(".repeat(12_000)),
        format!("{{{{ \"{}", "\\n".repeat(6_000)),
        format!("{{{{ '{}", "\\x4".repeat(4_000)),
        format!("{{{{ \"{}", "\\N{a".repeat(3_000)),
        format!("{{{{ {}", "@/".repeat(6_000)),
        format!("{{% {}", "with ".repeat(3_000)),
        format!("{{% {}", "block ".repeat(3_000)),
        format!("{{%- {}x", " ".repeat(12_000)),
        format!("{}{{% raw %}}", "{% raw ".repeat(2_000)),
        format!("{{{{ {}\u{e9}", "a".repeat(12_000)),
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
