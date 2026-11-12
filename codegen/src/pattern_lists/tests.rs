use nishiki::Grammar;
use nishiki::unstable::codegen::GenerateError;
use nishiki::unstable::codegen::flatten::flatten;
use nishiki::unstable::codegen::tables::{Act, CaptureLists, RuleTables};
use nishiki::unstable::grammar::unlinked;

/// A language with no respelling, whose capture lists become rules.
const PLAIN: RuleTables = RuleTables {
    language: "demo",
    grammar: "demo",
    key_spellings: &[],
    capture_lists: CaptureLists::Rules,
};

/// Why a test grammar gave no tables.
#[derive(Debug, PartialEq, Eq)]
enum Failure {
    /// The grammar's JSON did not load, with the loader's message.
    Grammar(String),
    /// The walk refused the grammar.
    Generate(GenerateError),
}

/// Each pattern's id and act, in table order, from the grammar `json` read
/// as `language` says.
fn ids(json: &str, language: &RuleTables) -> Result<Vec<(String, Act)>, Failure> {
    let grammar = Grammar::from_json(json).map_err(|error| Failure::Grammar(error.to_string()))?;
    let tables = flatten(unlinked(grammar), language).map_err(Failure::Generate)?;
    Ok(tables
        .patterns
        .into_iter()
        .map(|pattern| (pattern.id, pattern.act))
        .collect())
}

/// The expected ids and acts, as [`ids`] lists them.
fn expected(patterns: &[(&str, Act)]) -> Vec<(String, Act)> {
    patterns
        .iter()
        .map(|&(id, act)| (id.to_owned(), act))
        .collect()
}

/// A repository entry takes its key in Pascal case, an inline pattern its
/// list's id and position, and a begin's end the begin's id and `End`.
#[test]
fn each_pattern_takes_its_id_from_where_the_walk_finds_it() {
    let json = r##"{"scopeName": "source.demo", "patterns": [
        {"include": "#string-quoted"}, {"match": "x"}],
        "repository": {"string-quoted": {"begin": "\"", "end": "\"",
            "patterns": [{"match": "\\\\."}]}}}"##;
    assert_eq!(
        ids(json, &PLAIN),
        Ok(expected(&[
            ("StringQuoted", Act::Begin),
            ("StringQuotedEnd", Act::End),
            ("StringQuotedAt0", Act::Match),
            ("RootAt1", Act::Match),
        ]))
    );
}

/// A capture list that reaches its own pattern leaves that pattern where the
/// list first reached it, as a JavaScript `Map` keeps a key's first place.
#[test]
fn a_pattern_a_capture_list_reaches_first_keeps_that_place() {
    let json = r##"{"scopeName": "source.demo", "patterns": [{"include": "#word"}],
        "repository": {"word": {"match": "(w)",
            "captures": {"1": {"patterns": [{"include": "#word"}, {"match": "v"}]}}}}}"##;
    assert_eq!(
        ids(json, &PLAIN),
        Ok(expected(&[
            ("Word", Act::Match),
            ("Nested0At1", Act::Match)
        ]))
    );
}

/// A language that lexes captures by hand adds no rule for their lists.
#[test]
fn capture_lists_by_hand_add_no_pattern() {
    let json = r#"{"scopeName": "source.demo", "patterns": [{"match": "(w)",
        "captures": {"1": {"patterns": [{"match": "v"}]}}}]}"#;
    let by_hand = RuleTables {
        capture_lists: CaptureLists::ByHand,
        ..PLAIN
    };
    assert_eq!(
        ids(json, &by_hand),
        Ok(expected(&[("RootAt0", Act::Match)]))
    );
}

/// A key's misspelling is respelled before the key becomes an id.
#[test]
fn a_misspelled_key_takes_the_spelling_the_language_gives() {
    let json = r##"{"scopeName": "source.demo", "patterns": [{"include": "#charecter-escape"}],
        "repository": {"charecter-escape": {"match": "\\\\."}}}"##;
    let respelled = RuleTables {
        key_spellings: &[("charecter", "character")],
        ..PLAIN
    };
    assert_eq!(
        ids(json, &respelled),
        Ok(expected(&[("CharacterEscape", Act::Match)]))
    );
}

/// An include of a key the repository lacks names the rule that holds it.
#[test]
fn an_include_of_a_missing_key_is_an_error() {
    let json = r##"{"scopeName": "source.demo", "patterns": [{"include": "#gone"}]}"##;
    assert_eq!(
        ids(json, &PLAIN),
        Err(Failure::Generate(GenerateError::Include {
            rule: "Root".to_owned(),
            include: "#gone".to_owned(),
        }))
    );
}
