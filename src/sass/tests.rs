use indoc::indoc;

use crate::Language;
use crate::test_support::{self, LeafRow, leaf_in, merge_by_leaf};

/// Lexes the lines of `text` in order from one start state.
fn lex_lines(text: &str) -> Vec<LeafRow> {
    merge_by_leaf(test_support::lex(Language::Sass, text))
}

fn lex(line: &str) -> LeafRow {
    lex_lines(line).remove(0)
}

#[test]
fn the_name_is_known() {
    assert_eq!(Language::from_name("sass"), Some(Language::Sass));
}

#[test]
fn a_variable_definition_names_its_value() {
    let row = lex("$base: 10px !default");
    assert_eq!(leaf_in(&row, "$base"), "variable.other.name");
    assert_eq!(leaf_in(&row, "10"), "constant.numeric.css.sass");
    assert_eq!(leaf_in(&row, "px"), "keyword.control.unit.css.sass");
    assert_eq!(
        leaf_in(&row, "!default"),
        "keyword.other.important.css.sass"
    );
}

#[test]
fn a_selector_names_its_tag_class_and_pseudo_class() {
    let row = lex("a.btn:hover");
    assert_eq!(leaf_in(&row, "a"), "entity.name.tag.css.sass.symbol");
    assert_eq!(
        leaf_in(&row, ".btn"),
        "entity.other.attribute-name.class.css.sass"
    );
    assert_eq!(
        leaf_in(&row, ":hover"),
        "entity.other.attribute-name.pseudo-class.css.sass"
    );
}

/// A unit needs no word boundary after it, so `1sec` is `1`, `s` and `ec`.
#[test]
fn a_property_names_its_function_and_units() {
    let row = lex("  color: darken($c, 10%)");
    assert_eq!(
        leaf_in(&row, "color"),
        "support.type.property-name.css.sass.prop.name"
    );
    assert_eq!(leaf_in(&row, "darken("), "entity.name.function");
    assert_eq!(leaf_in(&row, "$c"), "variable.other.value");
    assert_eq!(leaf_in(&row, "%"), "keyword.control.unit.css.sass");
    let row = lex("@media screen and (max-width: 1sec)");
    assert_eq!(leaf_in(&row, "s"), "keyword.control.unit.css.sass");
    assert_eq!(
        leaf_in(&row, "ec"),
        "support.constant.property-value.css.sass"
    );
}

#[test]
fn mixins_are_defined_with_equals_and_included_with_plus() {
    let row = lex("=mixin($x)");
    assert_eq!(leaf_in(&row, "="), "keyword.control.at-rule.css.sass");
    assert_eq!(leaf_in(&row, "mixin("), "entity.name.function");
    let row = lex("+mixin(1px)");
    assert_eq!(leaf_in(&row, "+"), "keyword.control.at-rule.css.sass");
    assert_eq!(leaf_in(&row, "mixin"), "support.function.name.sass.library");
}

#[test]
fn an_import_marks_its_alias_invalid() {
    let row = lex("@import foo as bar");
    assert_eq!(leaf_in(&row, "@import"), "keyword.control.at-rule.use");
    assert_eq!(leaf_in(&row, " foo"), "constant.character.css.sass");
    assert_eq!(leaf_in(&row, " as bar"), "invalid");
}

/// The comment goes on while a line starts with a blank and its indent.
#[test]
fn a_block_comment_ends_at_the_first_unindented_line() {
    let rows = lex_lines(indoc! {"
        /* a
           b
        c: d
    "});
    assert_eq!(leaf_in(&rows[0], "/* a"), "comment.block.sass");
    assert_eq!(leaf_in(&rows[1], "   b"), "comment.block.sass");
    assert_eq!(
        leaf_in(&rows[2], "c"),
        "support.type.property-name.css.sass.prop.name"
    );
}

/// Vue hands a `<style lang="sass">` body to this lexer.
/// `</style>` closes it only while no Sass rule is open.
/// A property list stays open to the end of its line and takes a `</style>`
/// inside it, as in Shiki.
#[cfg(feature = "lang-vue")]
#[test]
fn a_vue_style_closes_only_with_no_sass_rule_open() {
    let rows = merge_by_leaf(test_support::lex(
        Language::Vue,
        indoc! {r#"
            <style lang="sass">
            .a
              color: red
            </style>
            <style lang="sass">
              color: red</style>
        "#},
    ));
    assert_eq!(
        leaf_in(&rows[1], ".a"),
        "entity.other.attribute-name.class.css.sass"
    );
    assert_eq!(
        leaf_in(&rows[2], "red"),
        "support.constant.property-value.css.sass"
    );
    assert_eq!(
        leaf_in(&rows[3], "</"),
        "punctuation.definition.tag.begin.html.vue"
    );
    assert_eq!(
        leaf_in(&rows[5], "style"),
        "support.constant.property-value.css.sass"
    );
}

#[cfg(feature = "json")]
mod probe {
    use crate::sass::engine::Sass;
    use crate::sass::grammar::Pat;
    use crate::test_support::probe::{answer_cases, lexer};

    /// Answers the cases of `oracle/onig.ts` with Sass's matchers (see
    /// [`answer_cases`]). The text of a case is the start of a document.
    /// `\A` then holds at its first byte, as it does for Oniguruma.
    #[test]
    #[ignore = "a tool: it needs a cases file"]
    fn matchers_answer_the_probe_cases() {
        answer_cases(&Pat::ALL, |arena, pat, case| {
            lexer::<Sass, _>(arena, case, |lexer| {
                lexer.first_line = true;
                lexer.probe_matcher(case.p, lexer.g, |lexer, p| lexer.matcher(pat, p))
            })
        });
    }
}

/// `.(?=\))` ends the content of `url(` at the blank right before `)`.
/// It takes one blank, so the scan tries it at the last blank of a run.
#[test]
fn url_content_ends_at_the_blank_before_its_parenthesis() {
    let rows = lex_lines(indoc! {"
        .a
          b: url(x  )"});
    assert_eq!(leaf_in(&rows[1], ")"), "entity.name.function.close");
}
