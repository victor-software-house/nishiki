//! Hostile lines for Vue, under the default limits.

use nishiki::{Highlighter, Language, Limit, Limits, State, Token};

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

/// Each block in `lang="vue"` opens its component on the outer stack.
/// Two thousand levels lex on the test thread, with no limits.
/// The text inside has one `text.html.vue` per level.
/// Every level closes.
#[test]
fn nested_vue_blocks_share_one_stack() {
    let levels = 2_000;
    let open = format!("{}x", "<a lang=\"vue\">".repeat(levels));
    let close = "</a>".repeat(levels);
    let mut highlighter = Highlighter::new_with_limits(Language::Vue, Limits::unlimited());
    let (rows, state) = lex(&mut highlighter, &[&open, &close]);
    assert!(rows.iter().all(|(_, report)| report.is_complete()));
    let text = rows[0].0.last().copied().map(Token::stack);
    let components = text.map_or(0, |stack| {
        highlighter
            .scopes(stack)
            .filter(|&scope| scope == "text.html.vue")
            .count()
    });
    assert_eq!(
        components,
        levels + 1,
        "the root and one component per level"
    );
    assert_eq!(state, State::default(), "every block closed");
}
