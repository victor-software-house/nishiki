use nishiki::Grammar;
use nishiki::unstable::codegen::GenerateError;
use nishiki::unstable::codegen::flatten::flatten;
use nishiki::unstable::codegen::tables::{
    Act, CaptureLists, LonePattern, RuleTables, Split, Tables,
};
use nishiki::unstable::grammar::unlinked;

/// A language with no respelling, whose capture lists become rules.
const PLAIN: RuleTables = RuleTables {
    language: "demo",
    grammar: "demo",
    key_spellings: &[],
    capture_lists: CaptureLists::Rules,
    pattern_names: &[],
    split: Split::KeepsGroup,
    lone_pattern: LonePattern::OwnId,
};

/// Why a test grammar gave no tables.
#[derive(Debug, PartialEq, Eq)]
enum Failure {
    /// The grammar's JSON did not load, with the loader's message.
    Grammar(String),
    /// The walk refused the grammar.
    Generate(GenerateError),
}

/// The tables of the grammar `json`, read as `language` says.
fn tables(json: &str, language: &RuleTables) -> Result<Tables, Failure> {
    let grammar = Grammar::from_json(json).map_err(|error| Failure::Grammar(error.to_string()))?;
    flatten(unlinked(grammar), language).map_err(Failure::Generate)
}

/// The ids of the patterns the rule `id` tries, in its order, or `None`
/// when the tables hold no such rule.
fn tried(json: &str, language: &RuleTables, id: &str) -> Result<Option<Vec<String>>, Failure> {
    let tables = tables(json, language)?;
    Ok(tables.rules.iter().find(|rule| rule.id == id).map(|rule| {
        rule.patterns
            .iter()
            .map(|&index| tables.patterns[index].id.clone())
            .collect()
    }))
}

/// Each rule's id, in table order, from the grammar `json` read as
/// `language` says.
fn rule_ids(json: &str, language: &RuleTables) -> Result<Vec<String>, Failure> {
    Ok(tables(json, language)?
        .rules
        .into_iter()
        .map(|rule| rule.id)
        .collect())
}

/// Each pattern's id and act, in table order, from the grammar `json` read
/// as `language` says.
fn ids(json: &str, language: &RuleTables) -> Result<Vec<(String, Act)>, Failure> {
    Ok(tables(json, language)?
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

/// An include of the grammar's own scope name adds the top-level patterns,
/// as `$self` does.
#[test]
fn an_include_of_the_own_scope_name_adds_the_top_level_patterns() {
    let json = r##"{"scopeName": "source.demo", "patterns": [{"match": "a"}, {"include": "#block"}],
        "repository": {"block": {"begin": "b", "end": "c",
            "patterns": [{"include": "source.demo"}]}}}"##;
    assert_eq!(
        tried(json, &PLAIN, "Block"),
        Ok(Some(vec!["RootAt0".to_owned(), "Block".to_owned()]))
    );
}

/// A list that includes itself adds nothing where it reaches itself again.
#[test]
fn a_list_that_reaches_itself_adds_nothing_there() {
    let json = r##"{"scopeName": "source.demo", "patterns": [{"include": "#words"}],
        "repository": {"words": {"patterns": [{"match": "w"}, {"include": "#words"}]}}}"##;
    assert_eq!(ids(json, &PLAIN), Ok(expected(&[("WordsAt0", Act::Match)])));
}

/// A rule a begin opens inside a list lists that list in full, though the
/// list is still being expanded around it.
#[test]
fn a_rule_inside_a_list_lists_that_list_in_full() {
    let json = r##"{"scopeName": "source.demo", "patterns": [{"include": "#list"}],
        "repository": {"list": {"patterns": [
            {"begin": "b", "end": "e", "patterns": [{"include": "#list"}]},
            {"match": "m"}]}}}"##;
    assert_eq!(
        tried(json, &PLAIN, "ListAt0"),
        Ok(Some(vec!["ListAt0".to_owned(), "ListAt1".to_owned()]))
    );
}

/// A match named after its first group becomes one pattern per alternative,
/// each with its own place, in or out of the group as `split` sets.
#[test]
fn a_match_named_after_its_group_becomes_one_pattern_per_alternative() {
    let json = r#"{"scopeName": "source.demo", "patterns": [
        {"match": "\\b(extends|super)\\b", "name": "storage.$1.demo"}, {"match": "x"}]}"#;
    let regexes = |split| {
        tables(json, &RuleTables { split, ..PLAIN }).map(|tables| {
            tables
                .patterns
                .into_iter()
                .map(|pattern| (pattern.id, pattern.regex))
                .collect::<Vec<_>>()
        })
    };
    let pairs = |list: [(&str, &str); 3]| {
        Ok(list
            .map(|(id, regex)| (id.to_owned(), regex.to_owned()))
            .to_vec())
    };
    assert_eq!(
        regexes(Split::KeepsGroup),
        pairs([
            ("RootAt0", "\\b(extends)\\b"),
            ("RootAt1", "\\b(super)\\b"),
            ("RootAt2", "x")
        ])
    );
    assert_eq!(
        regexes(Split::DropsGroup),
        pairs([
            ("RootAt0", "\\bextends\\b"),
            ("RootAt1", "\\bsuper\\b"),
            ("RootAt2", "x")
        ])
    );
}

/// A language's name for an inline pattern replaces its derived id, and the
/// patterns inside it take their ids from that name.
#[test]
fn a_pattern_name_replaces_the_derived_id() {
    let json = r#"{"scopeName": "source.demo", "patterns": [
        {"begin": "b", "end": "e", "patterns": [{"match": "m"}]}]}"#;
    let named = RuleTables {
        pattern_names: &[("RootAt0", "Block")],
        ..PLAIN
    };
    assert_eq!(
        ids(json, &named),
        Ok(expected(&[
            ("Block", Act::Begin),
            ("BlockEnd", Act::End),
            ("BlockAt0", Act::Match),
        ]))
    );
}

/// An entry that holds one inline pattern gives it the entry's id under
/// `LonePattern::ListId`.
#[test]
fn a_lone_pattern_takes_the_entry_id_under_list_id() {
    let json = r##"{"scopeName": "source.demo", "patterns": [{"include": "#word"}],
        "repository": {"word": {"patterns": [{"match": "w"}]}}}"##;
    let lone = RuleTables {
        lone_pattern: LonePattern::ListId,
        ..PLAIN
    };
    assert_eq!(ids(json, &PLAIN), Ok(expected(&[("WordAt0", Act::Match)])));
    assert_eq!(ids(json, &lone), Ok(expected(&[("Word", Act::Match)])));
}

/// A begin's end id that a repository entry also takes is an error.
#[test]
fn an_end_id_another_pattern_takes_is_an_error() {
    let json = r##"{"scopeName": "source.demo",
        "patterns": [{"include": "#foo-end"}, {"include": "#foo"}],
        "repository": {"foo": {"begin": "b", "end": "e"}, "foo-end": {"match": "f"}}}"##;
    assert_eq!(
        ids(json, &PLAIN),
        Err(Failure::Generate(GenerateError::IdTaken {
            id: "FooEnd".to_owned(),
        }))
    );
}

/// A begin's end id that a match with the same expression takes is an error,
/// since the two patterns differ in their act.
#[test]
fn an_end_id_a_match_with_its_expression_takes_is_an_error() {
    let json = r##"{"scopeName": "source.demo",
        "patterns": [{"include": "#foo"}, {"include": "#foo-end"}],
        "repository": {"foo": {"begin": "b", "end": "e"}, "foo-end": {"match": "e"}}}"##;
    assert_eq!(
        ids(json, &PLAIN),
        Err(Failure::Generate(GenerateError::IdTaken {
            id: "FooEnd".to_owned(),
        }))
    );
}

/// A begin's id that a capture list's rule already has is an error.
#[test]
fn a_rule_id_a_capture_list_takes_is_an_error() {
    let json = r##"{"scopeName": "source.demo", "patterns": [
        {"match": "(w)", "captures": {"1": {"patterns": [{"match": "v"}]}}},
        {"include": "#nested0"}],
        "repository": {"nested0": {"begin": "b", "end": "e"}}}"##;
    assert_eq!(
        ids(json, &PLAIN),
        Err(Failure::Generate(GenerateError::IdTaken {
            id: "Nested0".to_owned(),
        }))
    );
}

/// A begin that its own capture list reaches is written once, with one rule.
#[test]
fn a_begin_its_capture_list_reaches_is_written_once() {
    let json = r##"{"scopeName": "source.demo", "patterns": [{"include": "#foo"}],
        "repository": {"foo": {"begin": "(b)", "end": "e",
            "beginCaptures": {"1": {"patterns": [{"include": "#foo"}]}}}}}"##;
    assert_eq!(
        ids(json, &PLAIN),
        Ok(expected(&[("Foo", Act::Begin), ("FooEnd", Act::End)]))
    );
    assert_eq!(
        rule_ids(json, &PLAIN),
        Ok(["Nested0", "Foo", "Root"].map(str::to_owned).to_vec())
    );
    assert_eq!(
        tried(json, &PLAIN, "Nested0"),
        Ok(Some(vec!["Foo".to_owned()]))
    );
}

/// A `$self` in the top-level list adds nothing, so no pattern repeats in
/// `Root`.
#[test]
fn a_self_in_the_top_level_list_adds_nothing() {
    let json = r#"{"scopeName": "source.demo", "patterns": [
        {"match": "x"}, {"include": "$self"}]}"#;
    assert_eq!(
        tried(json, &PLAIN, "Root"),
        Ok(Some(vec!["RootAt0".to_owned()]))
    );
}

/// A match named after its first group fails when it has no group, or when
/// the group holds more than words between `|`.
#[test]
fn a_group_that_is_not_plain_words_does_not_split() {
    for regex in [r"\b(foo(?:bar)?|baz)\b", r"(a\)|b)", r"([)]|b)", r"\bfoo\b"] {
        let json = serde_json::json!({"scopeName": "source.demo", "patterns": [
            {"match": regex, "name": "keyword.$1.demo"}]});
        assert_eq!(
            ids(&json.to_string(), &PLAIN),
            Err(Failure::Generate(GenerateError::Split {
                id: "RootAt0".to_owned(),
                regex: regex.to_owned(),
            })),
            "{regex}"
        );
    }
}

/// Under `LonePattern::ListId`, an entry whose one pattern is itself a list
/// expands that list, whose own lone match takes the list's id.
#[test]
fn a_lone_list_still_adds_its_patterns() {
    let json = r##"{"scopeName": "source.demo", "patterns": [{"include": "#outer"}],
        "repository": {"outer": {"patterns": [{"patterns": [{"match": "w"}]}]}}}"##;
    let lone = RuleTables {
        lone_pattern: LonePattern::ListId,
        ..PLAIN
    };
    assert_eq!(ids(json, &lone), Ok(expected(&[("OuterAt0", Act::Match)])));
}
