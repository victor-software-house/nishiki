use indoc::indoc;

use crate::Language;
use crate::test_support::leaf;

#[test]
fn a_script_body_follows_its_lang() {
    let ts = "<script lang=\"ts\">let x: number</script>";
    assert_eq!(
        leaf(Language::Astro, ts, "number"),
        "support.type.primitive.ts"
    );
    let js = "<script>let x = 1</script>";
    assert_eq!(leaf(Language::Astro, js, "let"), "storage.type.js");
}

#[test]
fn a_stylus_body_is_plain() {
    let text = indoc! {r#"
        <style lang="stylus">a
          color red</style>"#};
    assert_eq!(
        leaf(Language::Astro, text, "  color red"),
        "meta.lang.stylus.astro"
    );
}

#[test]
fn a_body_opens_after_a_bracket_in_a_quoted_value() {
    let text = "<script lang=\"ts\" x=\"a>b\">";
    assert_eq!(
        leaf(Language::Astro, text, "b"),
        "variable.other.readwrite.ts"
    );
}

#[test]
fn a_body_opens_after_a_bracket_in_a_backtick_value() {
    let text = "<script lang=\"ts\" x=`a>b ${c}`>";
    assert_eq!(
        leaf(Language::Astro, text, "b"),
        "variable.other.readwrite.ts"
    );
}

#[test]
fn a_script_tag_after_a_bracket_opens_its_body() {
    let text = "<p></p><script>a</script>";
    assert_eq!(
        leaf(Language::Astro, text, "script"),
        "variable.other.readwrite.js"
    );
}

#[test]
fn an_event_name_takes_a_following_x() {
    let text = "<a onclickx=\"go()\">";
    assert_eq!(
        leaf(Language::Astro, text, "onclick"),
        "entity.other.attribute-name.astro"
    );
}

#[test]
fn an_expression_is_tsx() {
    let text = "<p>{items.map((i) => <li>{i}</li>)}</p>";
    assert_eq!(leaf(Language::Astro, text, "li"), "entity.name.tag.tsx");
}

#[test]
fn the_frontmatter_is_typescript() {
    let text = indoc! {r#"
        ---
        const a: string = "x"
        ---
        <p>{a}</p>"#};
    assert_eq!(
        leaf(Language::Astro, text, "string"),
        "support.type.primitive.ts"
    );
}

#[test]
fn a_named_entity_spells_its_name() {
    let text = "<p>&amp;</p>";
    assert_eq!(
        leaf(Language::Astro, text, "amp"),
        "constant.character.entity.named.amp.astro"
    );
}

#[cfg(feature = "json")]
mod probe {
    use crate::astro::engine::Astro;
    use crate::astro::grammar::Pattern;
    use crate::test_support::probe::{answer_cases, lexer};

    /// Answers the cases of `oracle/onig.ts` with Astro's matchers
    /// (see [`answer_cases`]).
    /// A case has no open rules, so no body opens.
    /// No line is the document's first, so the frontmatter never opens.
    #[test]
    #[ignore = "a tool: it needs a cases file"]
    fn matchers_answer_the_probe_cases() {
        answer_cases(&Pattern::ALL, |arena, pattern, case| {
            lexer::<Astro, _>(arena, case, |lexer| {
                lexer.probe_matcher(case.p, lexer.g, |lexer, p| lexer.matcher(pattern, p))
            })
        });
    }
}
