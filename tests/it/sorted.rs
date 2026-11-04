//! Every list that a new language adds a row to is sorted by language, so two
//! pull requests that add different languages edit different lines of it.
//! Each test reads one file and names the first pair out of order.

use std::fmt::Debug;
use std::fs;
use std::path::Path;

/// The text of `path`, relative to the repository root.
fn read(path: &str) -> String {
    fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join(path))
        .unwrap_or_else(|error| panic!("{path}: {error}"))
}

/// The lines of `text` between the line `open` and the next line `close`.
fn block<'a>(text: &'a str, open: &str, close: &str) -> Vec<&'a str> {
    let mut lines = text.lines().skip_while(|line| *line != open);
    assert!(lines.next().is_some(), "no line {open:?}");
    lines.take_while(|line| *line != close).collect()
}

/// The text between the first `open` and the next `close` in `line`.
fn between<'a>(line: &'a str, open: &str, close: &str) -> &'a str {
    let (_, rest) = line
        .split_once(open)
        .unwrap_or_else(|| panic!("no {open:?} in {line:?}"));
    rest.split_once(close).map_or(rest, |(inside, _)| inside)
}

/// Fails when a key of `list` is not after the one before it.
fn assert_sorted<K: Ord + Debug>(list: &str, keys: &[K]) {
    assert!(keys.len() > 1, "{list} has fewer than two entries");
    if let Some([before, after, ..]) = keys.windows(2).find(|pair| pair[0] >= pair[1]) {
        panic!("{list} is not sorted: {before:?} comes before {after:?}");
    }
}

/// The quoted names of the TOML array `list`, which taplo writes on one line
/// until it outgrows the line, and one name per line after that.
fn toml_names(text: &str, list: &str) -> Vec<String> {
    let open = format!("{list} = [");
    let line = text
        .lines()
        .find(|line| line.starts_with(&open))
        .unwrap_or_else(|| panic!("no {list} array"));
    let lines = if line == open {
        block(text, &open, "]")
    } else {
        vec![between(line, "[", "]")]
    };
    lines
        .iter()
        .flat_map(|line| line.split(','))
        .map(|name| name.trim().trim_matches('"'))
        .filter(|name| !name.is_empty())
        .map(str::to_owned)
        .collect()
}

#[test]
fn cargo_features_are_sorted() {
    let text = read("Cargo.toml");
    for list in ["all", "config", "docs", "systems", "textmate", "web"] {
        assert_sorted(&format!("Cargo.toml's {list}"), &toml_names(&text, list));
    }
    let features: Vec<&str> = text
        .lines()
        .filter(|line| line.starts_with("lang-"))
        .filter_map(|line| line.split_once(" = ").map(|(feature, _)| feature))
        .collect();
    assert_sorted("Cargo.toml's lang-* features", &features);
}

#[test]
fn the_readme_language_table_is_sorted() {
    let text = read("README.md");
    let variants: Vec<String> = text
        .lines()
        .skip_while(|line| !line.starts_with("| Language | Variant |"))
        .skip(2)
        .take_while(|line| line.starts_with('|'))
        .map(|row| between(row, "`Language::", "`").to_ascii_lowercase())
        .collect();
    assert_sorted("the README's language table", &variants);
}

#[test]
fn the_feature_checks_are_sorted() {
    let text = read("mise-tasks/check/features.sh");
    let languages: Vec<&str> = text
        .lines()
        .filter(|line| line.contains("--features lang-"))
        .collect();
    assert_sorted("check/features.sh's languages", &languages);
    let tiers: Vec<&str> = text
        .lines()
        .filter(|line| line.starts_with("cargo check"))
        .filter_map(|line| line.rsplit_once("--features ").map(|(_, set)| set))
        .filter(|set| ["config", "docs", "systems", "textmate", "web"].contains(set))
        .collect();
    assert_sorted("check/features.sh's tiers", &tiers);
}

#[test]
fn the_frame_tags_are_sorted() {
    let text = read("src/tag.rs");
    let tags: Vec<&str> = block(&text, "tags! {", "}")
        .into_iter()
        .filter_map(|line| line.strip_prefix("    "))
        .filter_map(|line| line.split_once(" = 0x").map(|(tag, _)| tag))
        .collect();
    assert_sorted("src/tag.rs's tags", &tags);
}

#[test]
fn the_scope_tables_are_sorted() {
    let text = read("src/scope.rs");
    let tables: Vec<&str> = block(&text, "tables! {", "}")
        .into_iter()
        .filter(|line| line.ends_with(';'))
        .filter_map(|line| line.trim().split_once(" = ").map(|(table, _)| table))
        .collect();
    assert_sorted("src/scope.rs's tables", &tables);
}

#[test]
fn the_scratch_fields_are_sorted() {
    let text = read("src/scratch.rs");
    let fields: Vec<&str> = block(&text, "pub(crate) struct Scratch {", "}")
        .into_iter()
        .filter_map(|line| line.strip_prefix("    pub(crate) "))
        .filter_map(|line| line.split_once(':').map(|(field, _)| field))
        .collect();
    assert_sorted("src/scratch.rs's fields", &fields);
}

#[test]
fn the_oracle_inputs_are_sorted() {
    let text = read("oracle/inputs.ts");
    let commits: Vec<String> = text
        .lines()
        .filter_map(|line| line.strip_prefix("const "))
        .filter_map(|line| line.split_once(' ').map(|(name, _)| name))
        .map(str::to_ascii_lowercase)
        .collect();
    assert_sorted("oracle/inputs.ts's commits", &commits);
    let inputs = block(&text, "export const inputs: Input[] = [", "];").join("\n");
    let keys: Vec<(&str, &str)> = inputs
        .split("lang: \"")
        .skip(1)
        .map(|input| {
            let (lang, rest) = input.split_once('"').expect("a closing quote");
            (lang, between(rest, "name: \"", "\""))
        })
        .collect();
    assert_sorted("oracle/inputs.ts's inputs", &keys);
}

#[test]
fn the_oracle_sources_are_sorted() {
    let text = read("tests/oracle/SOURCES.md");
    let fixtures: Vec<(&str, &str)> = text
        .lines()
        .skip_while(|line| !line.starts_with("| Fixture |"))
        .skip(2)
        .take_while(|line| line.starts_with('|'))
        .map(|row| {
            let fixture = between(row, "`", "`");
            fixture.split_once('/').expect("a language and a name")
        })
        .collect();
    assert_sorted("tests/oracle/SOURCES.md's fixtures", &fixtures);
    let labels: Vec<&str> = text
        .lines()
        .filter(|line| line.starts_with('[') && line.contains("]: "))
        .map(|line| between(line, "[", "]"))
        .collect();
    assert_sorted("tests/oracle/SOURCES.md's links", &labels);
}
