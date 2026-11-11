use super::{ident, names};

/// A POSIX bracket's set is named without its prefix, and the names sort.
#[test]
fn the_names_drop_the_posix_prefix_and_sort() {
    let keys = ["lu", "posix_alpha", "alphabetic", "l"];
    let found = names(keys.into_iter()).ok();
    assert_eq!(
        found,
        Some(vec![
            ("alpha", "posix_alpha"),
            ("alphabetic", "alphabetic"),
            ("l", "l"),
            ("lu", "lu"),
        ])
    );
}

/// Two sets with one name are refused.
#[test]
fn two_sets_with_one_name_are_refused() {
    assert!(names(["alpha", "posix_alpha"].into_iter()).is_err());
}

/// A table is named for its set's key, in upper case.
#[test]
fn a_table_is_named_for_its_key() {
    assert_eq!(ident("posix_alpha"), "SET_POSIX_ALPHA");
}
