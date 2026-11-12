use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use super::{Features, sets};
use crate::error::Error;

/// The features of a small crate: `lang-b` turns on `_engine`.
fn features() -> Features {
    let lists = BTreeMap::from([
        ("_engine".to_owned(), vec![]),
        ("json".to_owned(), vec!["dep:serde_json".to_owned()]),
        ("lang-a".to_owned(), vec![]),
        ("lang-b".to_owned(), vec!["_engine".to_owned()]),
    ]);
    Features::new(&lists)
}

/// The feature sets of a crate whose files are `files`: each a path and its
/// items, one per line.
fn sets_of(files: &[(&str, &[&str])]) -> BTreeSet<BTreeSet<String>> {
    let files = files
        .iter()
        .map(|(path, items)| (Path::new(path).to_path_buf(), items.join("\n")))
        .collect::<BTreeMap<_, _>>();
    let read = |path: &Path| {
        files
            .get(path)
            .cloned()
            .ok_or_else(|| Error::Shape(format!("no file {}", path.display())))
    };
    sets(Path::new("src/lib.rs"), &read, &features()).expect("the crate parses")
}

/// A set of features, from their names.
fn set(names: &[&str]) -> BTreeSet<String> {
    names.iter().map(|name| (*name).to_owned()).collect()
}

/// A probe module in a language's tests takes the language and `json`.
#[test]
fn a_gate_takes_the_gates_of_its_modules() {
    let found = sets_of(&[
        ("src/lib.rs", &[r#"#[cfg(feature = "lang-a")] mod a;"#]),
        (
            "src/a.rs",
            &[r#"#[cfg(test)] mod tests { #[cfg(feature = "json")] mod probe {} }"#],
        ),
    ]);
    assert_eq!(found, BTreeSet::from([set(&["json", "lang-a"])]));
}

/// Code outside the tests gets no build.
#[test]
fn code_outside_the_tests_gets_no_build() {
    let found = sets_of(&[(
        "src/lib.rs",
        &[r#"#[cfg(all(feature = "lang-a", feature = "json"))] fn both() {}"#],
    )]);
    assert_eq!(found, BTreeSet::new());
}

/// A feature that another of the set turns on is dropped, and a set left
/// with one feature gets no build.
/// A private feature is replaced by the public one that turns it on.
#[test]
fn a_feature_another_turns_on_is_dropped() {
    let found = sets_of(&[(
        "src/lib.rs",
        &[
            r#"#[cfg(all(test, feature = "_engine", feature = "lang-b"))] fn one() {}"#,
            r#"#[cfg(all(test, feature = "_engine", feature = "lang-a"))] fn two() {}"#,
        ],
    )]);
    assert_eq!(found, BTreeSet::from([set(&["lang-a", "lang-b"])]));
}

/// Each way to meet an `any` gets its own build.
#[test]
fn each_way_to_meet_an_any_gets_a_build() {
    let found = sets_of(&[(
        "src/lib.rs",
        &[
            r#"#[cfg(all(test, feature = "json", any(feature = "lang-a", feature = "lang-b")))]"#,
            "fn either() {}",
        ],
    )]);
    assert_eq!(
        found,
        BTreeSet::from([set(&["json", "lang-a"]), set(&["json", "lang-b"])])
    );
}

/// A gate on a statement counts with the gates of its function.
#[test]
fn a_gate_inside_a_function_counts_with_its_function() {
    let found = sets_of(&[(
        "src/lib.rs",
        &[
            r#"#[cfg(all(test, feature = "lang-a"))]"#,
            r#"fn lexes() { #[cfg(feature = "lang-b")] let embedded = 1; }"#,
        ],
    )]);
    assert_eq!(found, BTreeSet::from([set(&["lang-a", "lang-b"])]));
}

/// A `not` counts as met.
/// A gate that needs `nishiki_unstable`, or a feature both on and off, gets
/// no build.
#[test]
fn a_not_is_met_and_unstable_code_gets_no_build() {
    let found = sets_of(&[(
        "src/lib.rs",
        &[
            r#"#[cfg(all(test, feature = "lang-a", feature = "json", not(feature = "lang-b")))]"#,
            "fn without_b() {}",
            r#"#[cfg(all(test, nishiki_unstable, feature = "lang-a", feature = "lang-b"))]"#,
            "fn unstable() {}",
            r#"#[cfg(all(test, feature = "lang-a", feature = "json", not(feature = "json")))]"#,
            "fn never() {}",
        ],
    )]);
    assert_eq!(found, BTreeSet::from([set(&["json", "lang-a"])]));
}
