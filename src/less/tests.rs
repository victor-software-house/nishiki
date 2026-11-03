use crate::Language;
use crate::test_support::{self, LeafRow, leaf_in, merge_by_leaf, scopes_of};

fn lex(line: &str) -> LeafRow {
    merge_by_leaf(test_support::lex(Language::Less, line)).remove(0)
}

#[test]
fn an_empty_line_has_no_tokens() {
    assert_eq!(lex(""), LeafRow::new());
}

#[test]
fn a_variable_assignment_names_its_value() {
    let row = lex("@a: 1px;");
    assert_eq!(leaf_in(&row, "@"), "punctuation.definition.variable.less");
    assert_eq!(leaf_in(&row, "a"), "support.other.variable.less");
    assert_eq!(leaf_in(&row, "px"), "keyword.other.unit.less");
    assert_eq!(leaf_in(&row, ";"), "punctuation.terminator.rule.less");
}

#[test]
fn a_guard_reads_its_condition() {
    let row = lex(".m() when (@a > 0) { color: red; }");
    assert_eq!(leaf_in(&row, "when"), "keyword.control.conditional.less");
    assert_eq!(leaf_in(&row, ">"), "keyword.operator.logical.less");
    assert_eq!(leaf_in(&row, "color"), "support.type.property-name.less");
    assert_eq!(
        leaf_in(&row, "red"),
        "support.constant.color.w3c-standard-color-name.less"
    );
}

#[test]
fn a_media_query_reads_its_feature() {
    let row = lex("@media (min-width: 1px) { .a { b: c } }");
    assert_eq!(leaf_in(&row, "media"), "keyword.control.at-rule.media.less");
    assert_eq!(
        leaf_in(&row, "min-width"),
        "support.type.property-name.media.less"
    );
    assert_eq!(leaf_in(&row, "b"), "entity.name.tag.less");
}

#[test]
fn extend_and_an_escaped_string_get_their_scopes() {
    let row = lex(r#".a:extend(.b all) { &-x { width: ~"calc(1px)"; } }"#);
    assert_eq!(
        leaf_in(&row, "extend"),
        "entity.other.attribute-name.pseudo-class.extend.less"
    );
    assert_eq!(leaf_in(&row, "all"), "constant.language.all.less");
    assert_eq!(leaf_in(&row, "~"), "constant.character.escape.less");
    assert_eq!(leaf_in(&row, "calc(1px)"), "markup.raw.inline.less");
}

#[test]
fn a_line_comment_runs_to_the_end_of_the_line() {
    let row = lex("// c");
    assert_eq!(leaf_in(&row, " c"), "comment.line.double-slash.less");
}

#[test]
fn a_keyframe_percentage_holds_its_percent_sign() {
    let text = "@keyframes x { 50.5% { top: 0; } }";
    assert_eq!(
        scopes_of(Language::Less, text, "50.5"),
        ["source.css.less", "constant.numeric.less"]
    );
    assert_eq!(
        scopes_of(Language::Less, text, "%"),
        [
            "source.css.less",
            "constant.numeric.less",
            "keyword.other.unit.less"
        ]
    );
}
