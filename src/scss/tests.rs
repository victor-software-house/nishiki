use crate::Language;
use crate::test_support::{self, LeafRow, leaf_in, merge_by_leaf};

fn lex(line: &str) -> LeafRow {
    merge_by_leaf(test_support::lex(Language::Scss, line)).remove(0)
}

#[test]
fn an_empty_line_has_no_tokens() {
    assert_eq!(lex(""), LeafRow::new());
}

#[test]
fn a_variable_setting_names_its_value() {
    let row = lex("$a: 1px !default;");
    assert_eq!(leaf_in(&row, "$a"), "variable.scss");
    assert_eq!(leaf_in(&row, "px"), "keyword.other.unit.px.css");
    assert_eq!(leaf_in(&row, "!default"), "keyword.other.default.scss");
    assert_eq!(leaf_in(&row, ";"), "punctuation.terminator.rule.scss");
}

#[test]
fn a_parent_suffix_takes_the_name_after_the_ampersand() {
    let row = lex(".a { &-b > p:hover { color: red; } }");
    assert_eq!(leaf_in(&row, "&"), "entity.name.tag.reference.scss");
    assert_eq!(leaf_in(&row, "-b"), "punctuation.definition.entity.css");
    assert_eq!(leaf_in(&row, "p"), "entity.name.tag.css");
    assert_eq!(
        leaf_in(&row, "hover"),
        "entity.other.attribute-name.pseudo-class.css"
    );
    assert_eq!(leaf_in(&row, "color"), "support.type.property-name.css");
    assert_eq!(
        leaf_in(&row, "red"),
        "support.constant.color.w3c-standard-color-name.css"
    );
}

#[test]
fn an_include_names_its_module_and_mixin() {
    let row = lex("@include m.foo($x, 2);");
    assert_eq!(
        leaf_in(&row, "include"),
        "keyword.control.at-rule.include.scss"
    );
    assert_eq!(leaf_in(&row, "m"), "variable.scss");
    assert_eq!(leaf_in(&row, "foo"), "entity.name.function.scss");
    assert_eq!(leaf_in(&row, "$x"), "variable.scss");
    assert_eq!(leaf_in(&row, ";"), "punctuation.terminator.rule.css");
}

#[test]
fn an_interpolation_in_a_class_name_is_lexed_again() {
    let row = lex("@each $k, $v in $map { .x-#{$k} { a: $v } }");
    assert_eq!(leaf_in(&row, "in"), "keyword.control.operator");
    assert_eq!(leaf_in(&row, "x-"), "entity.other.attribute-name.class.css");
    assert_eq!(
        leaf_in(&row, "#{"),
        "punctuation.definition.interpolation.begin.bracket.curly.scss"
    );
}

#[test]
fn an_attribute_selector_takes_its_operator_and_quoted_value() {
    let row = lex("[data-x$=\"y\"] { }");
    assert_eq!(
        leaf_in(&row, "data-x"),
        "entity.other.attribute-name.attribute.scss"
    );
    assert_eq!(leaf_in(&row, "$="), "keyword.operator.scss");
    assert_eq!(leaf_in(&row, "y"), "meta.attribute-selector.scss");
    assert_eq!(
        leaf_in(&row, "]"),
        "punctuation.definition.attribute-selector.end.bracket.square.scss"
    );
}

#[test]
fn a_custom_element_is_named_before_a_pseudo_class() {
    let row = lex("my-el:hover { }");
    assert_eq!(leaf_in(&row, "my-el"), "entity.name.tag.custom.scss");
}

#[test]
fn a_media_query_reads_css_feature_names() {
    let row = lex("@media (min-width: 1px) { }");
    assert_eq!(leaf_in(&row, "media"), "keyword.control.at-rule.media.scss");
    assert_eq!(
        leaf_in(&row, "min-width"),
        "support.type.property-name.media.css"
    );
}

#[test]
fn a_line_comment_runs_to_the_end_of_the_line() {
    let row = lex("// c");
    assert_eq!(leaf_in(&row, " c"), "comment.line.scss");
}
