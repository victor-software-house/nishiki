//! `THIRD-PARTY-NOTICES.md` ships in the crate, and every grammar that lands
//! appends an entry to it. These tests fail when an entry lands inside
//! another's licence block, or when a grammar the `TextMate` engine may bundle
//! has no entry.

use std::fs;
use std::path::Path;

/// The text of `path`, relative to the repository root.
fn read(path: &str) -> String {
    fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join(path))
        .unwrap_or_else(|error| panic!("{path}: {error}"))
}

/// Where a line of the notices sits.
#[derive(Clone, Copy)]
enum Place {
    /// In the prose between licence blocks.
    Prose,
    /// In a licence block, which opened on this line number.
    Block(usize),
}

#[test]
fn every_licence_block_closes_before_the_next_entry() {
    let text = read("THIRD-PARTY-NOTICES.md");
    let mut place = Place::Prose;
    for (index, line) in text.lines().enumerate() {
        let number = index + 1;
        place = match (place, line.strip_prefix("```")) {
            (Place::Prose, Some(info)) => {
                assert!(
                    !info.is_empty(),
                    "line {number} closes a block that never opened"
                );
                Place::Block(number)
            }
            (Place::Block(opened), Some(info)) => {
                assert!(
                    info.is_empty(),
                    "line {number} opens a block inside the one opened on line {opened}"
                );
                Place::Prose
            }
            (Place::Block(opened), None) => {
                assert!(
                    !line.starts_with("Its "),
                    "line {number} starts an entry inside the block opened on line {opened}"
                );
                place
            }
            (Place::Prose, None) => place,
        };
    }
    if let Place::Block(opened) = place {
        panic!("the block opened on line {opened} never closes");
    }
}

#[test]
fn every_allowed_grammar_has_its_origin_in_the_notices() {
    let licences = read("oracle/licences.ts");
    let notices = read("THIRD-PARTY-NOTICES.md");
    let origins: Vec<&str> = licences
        .lines()
        .skip_while(|line| !line.starts_with("export const allowed"))
        .take_while(|line| *line != "};")
        .filter_map(|line| line.trim().strip_prefix("origin: \""))
        .filter_map(|rest| rest.strip_suffix("\","))
        .collect();
    assert!(!origins.is_empty(), "oracle/licences.ts lists no origins");
    let missing: Vec<&str> = origins
        .into_iter()
        .filter(|origin| !notices.contains(&format!("https://github.com/{origin}")))
        .collect();
    assert!(
        missing.is_empty(),
        "THIRD-PARTY-NOTICES.md names no entry for {missing:?}"
    );
}
