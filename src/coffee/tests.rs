use indoc::indoc;

use crate::Language;
use crate::test_support::{self, LeafRow, leaf_in, merge_by_leaf};

/// Lexes the lines of `text` in order from one start state.
fn lex_lines(text: &str) -> Vec<LeafRow> {
    merge_by_leaf(test_support::lex(Language::CoffeeScript, text))
}

fn lex(line: &str) -> LeafRow {
    lex_lines(line).remove(0)
}

#[test]
fn the_names_are_known() {
    for name in ["coffee", "coffeescript"] {
        assert_eq!(Language::from_name(name), Some(Language::CoffeeScript));
    }
}

#[test]
fn a_function_assignment_names_its_function_and_parameters() {
    let row = lex("square = (x, rest...) -> x * x");
    assert_eq!(leaf_in(&row, "square"), "entity.name.function.coffee");
    assert_eq!(leaf_in(&row, "x"), "variable.parameter.function.coffee");
    assert_eq!(leaf_in(&row, "..."), "keyword.operator.splat.coffee");
    assert_eq!(leaf_in(&row, "->"), "storage.type.function.coffee");
}

#[test]
fn a_class_names_its_parent() {
    let row = lex("class Dog extends Animal");
    assert_eq!(leaf_in(&row, "class"), "storage.type.class.coffee");
    assert_eq!(leaf_in(&row, "Dog"), "entity.name.type.class.coffee");
    assert_eq!(
        leaf_in(&row, "extends"),
        "keyword.control.inheritance.coffee"
    );
    assert_eq!(
        leaf_in(&row, "Animal"),
        "entity.other.inherited-class.coffee"
    );
}

/// A bare call's arguments run to a word that ends them, such as `if`.
/// The method call starts at the `.`, before `log` could read as a console
/// method.
#[test]
fn a_bare_method_call_names_its_method() {
    let row = lex("console.log 1 if ready");
    assert_eq!(leaf_in(&row, "console"), "entity.name.type.object.coffee");
    assert_eq!(leaf_in(&row, "log"), "entity.name.function.coffee");
    assert_eq!(leaf_in(&row, "1"), "constant.numeric.decimal.coffee");
    assert_eq!(leaf_in(&row, "if"), "keyword.control.coffee");
}

/// The backticks hold JavaScript, which the JavaScript lexer lexes.
/// The closing backtick stays CoffeeScript's.
#[test]
fn a_backtick_hands_its_javascript_to_the_javascript_lexer() {
    let row = lex("x = `var raw = 1;`");
    assert_eq!(leaf_in(&row, "var"), "storage.type.js");
    assert_eq!(leaf_in(&row, ";"), "punctuation.terminator.statement.js");
    assert_eq!(
        row.last().map(|(_, scope)| scope.as_str()),
        Some("punctuation.definition.string.end.coffee")
    );
}

/// The JavaScript ends at the last backtick, as if the line ended there.
/// A string it leaves open ends at that cut, as in Shiki.
#[test]
fn the_javascript_is_cut_at_the_last_backtick() {
    let row = lex("y = `var a = \"b`");
    assert_eq!(leaf_in(&row, "b"), "invalid.illegal.newline.js");
    assert_eq!(
        row.last().map(|(_, scope)| scope.as_str()),
        Some("punctuation.definition.string.end.coffee")
    );
}

/// Shiki skips the regular expression rule, whose only pattern includes a
/// grammar it lacks.
#[test]
fn a_regular_expression_is_plain_operators() {
    let row = lex("x = /ab+c/g");
    assert_eq!(leaf_in(&row, "/"), "keyword.operator.coffee");
    assert_eq!(leaf_in(&row, "+"), "keyword.operator.coffee");
}

/// The scan skips the rest of a word where nothing matched at its start.
/// `(new)\s+` needs no `\b`, so it still matches at the end of a word.
/// A character range still starts inside a word.
#[test]
fn a_word_keeps_its_places_for_new_and_a_range() {
    let row = lex("renew\tfor x");
    assert_eq!(leaf_in(&row, "new"), "keyword.operator.new.coffee");
    assert_eq!(leaf_in(&row, "for"), "entity.name.type.instance.coffee");
    let row = lex("r = ///[ab-c]///");
    assert_eq!(
        leaf_in(&row, "b-c"),
        "constant.other.character-class.range.regexp"
    );
}

/// A character range can start at the last blank of a run.
#[test]
fn a_heregex_range_starts_at_the_last_blank() {
    let rows = lex_lines(indoc! {"
        r = ///[
          -> ]///
    "});
    assert_eq!(
        leaf_in(&rows[1], " ->"),
        "constant.other.character-class.range.regexp"
    );
}

#[test]
fn a_block_comment_and_a_heredoc_go_on_across_lines() {
    let rows = lex_lines(indoc! {"
        ###
        a @param b
        ###
        s = '''
        it's
        '''
    "});
    assert_eq!(
        leaf_in(&rows[1], "@param"),
        "storage.type.annotation.coffee"
    );
    assert_eq!(
        leaf_in(&rows[4], "it's"),
        "string.quoted.single.heredoc.coffee"
    );
}

/// Vue hands a `<script lang="coffee">` body to this lexer.
/// `</script>` closes it only while no CoffeeScript rule is open.
#[cfg(feature = "lang-vue")]
#[test]
fn a_vue_script_closes_with_no_coffee_rule_open() {
    let rows = merge_by_leaf(test_support::lex(
        Language::Vue,
        indoc! {r#"
            <script lang="coffee">
            x = `1`
            </script>
        "#},
    ));
    assert_eq!(leaf_in(&rows[1], "x"), "variable.assignment.coffee");
    assert_eq!(leaf_in(&rows[1], "1"), "constant.numeric.decimal.js");
    assert_eq!(
        leaf_in(&rows[2], "</"),
        "punctuation.definition.tag.begin.html.vue"
    );
}

#[cfg(feature = "json")]
mod probe {
    use crate::coffee::engine::Coffee;
    use crate::coffee::grammar::Pat;
    use crate::test_support::probe::{answer_cases, lexer};

    /// Answers the cases of `oracle/onig.ts` with CoffeeScript's matchers
    /// (see [`answer_cases`]).
    #[test]
    #[ignore = "a tool: it needs a cases file"]
    fn matchers_answer_the_probe_cases() {
        answer_cases(&Pat::ALL, |arena, pat, case| {
            lexer::<Coffee, _>(arena, case, |lexer| {
                lexer.probe_matcher(case.p, lexer.g, |lexer, p| lexer.pat_matches(pat, p))
            })
        });
    }
}
