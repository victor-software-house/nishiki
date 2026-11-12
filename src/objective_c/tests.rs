use crate::test_support::leaves::{LeafRow, leaf_in, merge_by_leaf};
use crate::{Language, test_support};

fn lex(line: &str) -> LeafRow {
    merge_by_leaf(test_support::rows::lex(Language::ObjectiveC, line)).remove(0)
}

#[test]
fn the_names_are_known() {
    assert_eq!(
        Language::from_name("objective-c"),
        Some(Language::ObjectiveC)
    );
    assert_eq!(Language::from_name("objc"), Some(Language::ObjectiveC));
}

#[test]
fn an_interface_names_its_class_and_superclass() {
    let row = lex("@interface Card : NSObject");
    assert_eq!(leaf_in(&row, "interface"), "storage.type.objc");
    assert_eq!(leaf_in(&row, "Card"), "entity.name.type.objc");
    assert_eq!(
        leaf_in(&row, "NSObject"),
        "entity.other.inherited-class.objc"
    );
}

#[test]
fn a_cocoa_word_takes_its_list_scope() {
    let row = lex("NSString *s = NSStringFromClass(c);");
    assert_eq!(leaf_in(&row, "NSString"), "support.class.cocoa.objc");
    assert_eq!(
        leaf_in(&row, "NSStringFromClass"),
        "support.function.cocoa.objc"
    );
}

#[test]
fn a_directive_name_repeats_its_word() {
    let row = lex("#import <Foundation/Foundation.h>");
    assert_eq!(
        leaf_in(&row, "import"),
        "keyword.control.directive.import.objc"
    );
}

#[cfg(feature = "json")]
mod probe {
    use crate::objective_c::engine::ObjectiveC;
    use crate::objective_c::grammar::Rx;
    use crate::test_support::probe::{answer_cases, lexer};

    /// Answers the cases of `oracle/onig.ts` with Objective-C's matchers (see
    /// [`answer_cases`]).
    #[test]
    #[ignore = "a tool: it needs a cases file"]
    fn matchers_answer_the_probe_cases() {
        answer_cases(&Rx::ALL, |arena, rx, case| {
            lexer::<ObjectiveC, _>(arena, case, |lexer| {
                lexer.probe_matcher(case.p, lexer.g, |lexer, p| lexer.matcher(rx, p))
            })
        });
    }
}
