//! Hostile lines for Vue, under the default limits.

use nishiki::{Highlighter, Language, Limit};

use super::lex;

#[test]
fn hostile_vue_lines_finish_within_the_default_limits() {
    for line in [
        "<".repeat(300_000),
        "<a".repeat(150_000),
        "<a lang=ts ".repeat(30_000),
        "<a lang=\"md\"".repeat(25_000),
        "<a/".repeat(100_000),
        "<a />".repeat(60_000),
        "<!--".repeat(75_000),
        "<!-- @a ".repeat(40_000),
        "<template>".repeat(30_000),
        "<template><Comp".repeat(20_000),
        "<template><Comp v-if=\"".repeat(15_000),
        "<template><template v-for=a>".repeat(10_000),
        "<template><A :b=\"c\" @d.e.f='g' #h=`i`>".repeat(8_000),
        "<template><A :[b]=c v-x:y.z=w>".repeat(10_000),
        "<script lang=\"ts\">".repeat(20_000),
        "<script>".repeat(30_000),
        "<style>".repeat(40_000),
        "<style lang=\"scss\">".repeat(15_000),
        "<i18n lang=\"yaml\">".repeat(15_000),
        "<x lang=\"pug\">".repeat(20_000),
        format!("<a{}", " lang=\"js\"".repeat(30_000)),
        format!("<template><A{}", " v-if=\"x\"".repeat(30_000)),
        format!("<template><A v-x{}", ".m".repeat(100_000)),
        format!("<a{}", "-".repeat(300_000)),
    ] {
        let mut highlighter = Highlighter::new(Language::Vue);
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
