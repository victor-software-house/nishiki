use crate::Language;
use crate::test_support::{self, LeafRow, leaf_in, merge_by_leaf};

fn lex(line: &str) -> LeafRow {
    merge_by_leaf(test_support::lex(Language::ObjectiveCpp, line)).remove(0)
}

#[test]
fn the_name_is_known() {
    assert_eq!(
        Language::from_name("objective-cpp"),
        Some(Language::ObjectiveCpp)
    );
}

#[test]
fn a_class_names_its_bases() {
    let row = lex("class Card : public Base, private Other {");
    assert_eq!(leaf_in(&row, "class"), "storage.type.class.objcpp");
    assert_eq!(leaf_in(&row, "Card"), "entity.name.type.objcpp");
    assert_eq!(
        leaf_in(&row, "public"),
        "storage.type.modifier.access.objcpp"
    );
    assert_eq!(leaf_in(&row, "Base"), "entity.name.type.inherited.objcpp");
    assert_eq!(
        leaf_in(&row, "private"),
        "storage.type.modifier.access.objcpp"
    );
}

#[test]
fn objective_c_expressions_take_objcpp_scopes() {
    let row = lex("@interface Card : NSObject");
    assert_eq!(leaf_in(&row, "interface"), "storage.type.objcpp");
    assert_eq!(
        leaf_in(&row, "NSObject"),
        "entity.other.inherited-class.objcpp"
    );
}

#[test]
fn a_long_raw_string_delimiter_is_invalid() {
    let row = lex(r#"auto s = R"abcdefghijklmnopq(a)abcdefghijklmnopq";"#);
    assert_eq!(
        leaf_in(&row, "abcdefghijklmnopq"),
        "invalid.illegal.delimiter-too-long.objcpp"
    );
}

#[test]
fn a_bracket_after_a_namespace_keeps_the_literal_name() {
    let row = lex("namespace foo) x");
    assert_eq!(leaf_in(&row, ")"), "keyword.control.namespace.$2.objcpp");
}

#[cfg(feature = "json")]
mod probe {
    use crate::objective_cpp::engine::ObjectiveCpp;
    use crate::objective_cpp::grammar::Rx;
    use crate::test_support::probe::{answer_cases, lexer};

    /// Answers the cases of `oracle/onig.ts` with Objective-C++'s own
    /// matchers (see [`answer_cases`]).
    /// The expressions it shares with Objective-C are Objective-C's probe's.
    #[test]
    #[ignore = "a tool: it needs a cases file"]
    fn matchers_answer_the_probe_cases() {
        answer_cases(&Rx::ALL, |arena, rx, case| {
            lexer::<ObjectiveCpp, _>(arena, case, |lexer| {
                lexer.probe_matcher(case.p, lexer.g, |lexer, p| lexer.own_matcher(rx, p))
            })
        });
    }
}
