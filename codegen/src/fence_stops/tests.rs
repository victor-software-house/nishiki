use super::{can_match_empty, check, compare, kebab};
use crate::root;

/// The committed grammars hold one fence grammar that can stop at its root,
/// Less's, and `pops_host` names it.
#[test]
fn the_repository_passes_its_check() {
    let stopping = root().and_then(|root| check(&root));
    assert!(matches!(stopping.as_deref(), Ok(["less"])), "{stopping:?}");
}

/// The comparison fails when the lists differ, and passes when they match.
#[test]
fn compare_fails_on_a_different_list() {
    let named = ["less".to_owned()];
    assert!(compare(&["less"], &named).is_ok());
    assert!(compare(&["less", "pug"], &named).is_err());
    assert!(compare(&[], &named).is_err());
}

/// The cases of `oracle/rule-grammar.test.ts` for `canMatchEmpty`.
#[test]
fn a_pattern_matches_the_empty_text_when_a_branch_can() {
    assert!(can_match_empty("(;)|(?=[)}])"));
    assert!(can_match_empty("a?"));
    assert!(!can_match_empty("(;)|[)}]"));
    assert!(!can_match_empty(r"\b\w+"));
}

/// A variant's name becomes the grammar's name.
#[test]
fn a_variant_is_named_in_kebab_case() {
    assert_eq!(kebab("Less"), "less");
    assert_eq!(kebab("ObjectiveCpp"), "objective-cpp");
    assert_eq!(kebab("Css"), "css");
}
