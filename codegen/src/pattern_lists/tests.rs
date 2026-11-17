use nishiki::Grammar;
use nishiki::unstable::codegen::GenerateError;
use nishiki::unstable::codegen::flatten::flatten;
use nishiki::unstable::codegen::tables::{
    Act, AddedRule, CaptureLists, CaptureNames, External, Identity, Injection, LonePattern,
    MergedWords, Reentry, RuleTables, Split, Tables,
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
    capture_names: CaptureNames::Kept,
    lone_pattern: LonePattern::OwnId,
    identity: Identity::Id,
    reentry: Reentry::WithinRule,
    externals: &[],
    injected: None,
    added_rules: &[],
    merged_words: None,
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
    flatten(unlinked(grammar), &[], language).map_err(Failure::Generate)
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

/// An include of a key the repository lacks adds nothing.
#[test]
fn an_include_of_a_missing_key_adds_nothing() {
    let json =
        r##"{"scopeName": "source.demo", "patterns": [{"include": "#gone"}, {"match": "x"}]}"##;
    assert_eq!(ids(json, &PLAIN), Ok(expected(&[("RootAt1", Act::Match)])));
}

/// An include of another grammar names the rule that holds it.
#[test]
fn an_include_of_another_grammar_is_an_error() {
    let json = r#"{"scopeName": "source.demo", "patterns": [{"include": "source.gone"}]}"#;
    assert_eq!(
        ids(json, &PLAIN),
        Err(Failure::Generate(GenerateError::Include {
            rule: "Root".to_owned(),
            include: "source.gone".to_owned(),
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

/// A repository entry whose begin is named after a later group, in lower
/// case, becomes one begin per alternative.
/// Each takes the entry's id and its position, and lists its own patterns.
#[test]
fn an_entry_begin_named_after_a_group_becomes_one_begin_per_alternative() {
    let json = r##"{"scopeName": "source.demo", "patterns": [{"include": "#at-rule"}],
        "repository": {"at-rule": {"begin": "((@)(page|font))\\b", "end": ";",
            "name": "meta.at-rule.${3:/downcase}.demo", "patterns": [{"match": "x"}]}}}"##;
    let begins = tables(json, &PLAIN).map(|tables| {
        tables
            .patterns
            .into_iter()
            .filter(|pattern| pattern.act == Act::Begin)
            .map(|pattern| (pattern.id, pattern.regex))
            .collect::<Vec<_>>()
    });
    assert_eq!(
        begins,
        Ok(vec![
            ("AtRuleAt0".to_owned(), r"((@)(page))\b".to_owned()),
            ("AtRuleAt1".to_owned(), r"((@)(font))\b".to_owned()),
        ])
    );
    assert_eq!(
        tried(json, &PLAIN, "Root"),
        Ok(Some(vec!["AtRuleAt0".to_owned(), "AtRuleAt1".to_owned()]))
    );
    assert_eq!(
        tried(json, &PLAIN, "AtRuleAt1"),
        Ok(Some(vec!["AtRuleAt1At0".to_owned()]))
    );
}

/// Names that hold the text of two groups give no split.
#[test]
fn names_of_two_groups_do_not_split() {
    let json = r#"{"scopeName": "source.demo", "patterns": [
        {"match": "(a|b)(c|d)", "name": "keyword.$1.$2.demo"}]}"#;
    assert_eq!(
        ids(json, &PLAIN),
        Err(Failure::Generate(GenerateError::Split {
            id: "RootAt0".to_owned(),
            regex: "(a|b)(c|d)".to_owned(),
        }))
    );
}

/// A match whose capture name alone holds a group's text stays one pattern,
/// or splits the optional group around it, as `capture_names` sets.
#[test]
fn a_capture_named_after_a_group_splits_its_optional_group() {
    let json = r#"{"scopeName": "source.demo", "patterns": [
        {"match": "1(?:(%)|(px|em)\\b)?",
            "captures": {"2": {"name": "unit.${2:/downcase}.demo"}}},
        {"match": "x"}]}"#;
    let regexes = |capture_names| {
        tables(
            json,
            &RuleTables {
                capture_names,
                ..PLAIN
            },
        )
        .map(|tables| {
            tables
                .patterns
                .into_iter()
                .map(|pattern| pattern.regex)
                .collect::<Vec<_>>()
        })
    };
    assert_eq!(
        regexes(CaptureNames::Kept),
        Ok(vec![r"1(?:(%)|(px|em)\b)?".to_owned(), "x".to_owned()])
    );
    assert_eq!(
        regexes(CaptureNames::SplitsOptionalGroup),
        Ok(vec![
            "1(?:(%))".to_owned(),
            r"1(?:(px|em)\b)".to_owned(),
            "1".to_owned(),
            "x".to_owned(),
        ])
    );
}

/// A capture named after a group that no optional group holds gives no
/// split.
#[test]
fn a_capture_named_after_a_required_group_does_not_split() {
    let json = r#"{"scopeName": "source.demo", "patterns": [{"match": "1(px|em)",
        "captures": {"1": {"name": "unit.$1.demo"}}}]}"#;
    let splits = RuleTables {
        capture_names: CaptureNames::SplitsOptionalGroup,
        ..PLAIN
    };
    assert_eq!(
        ids(json, &splits),
        Err(Failure::Generate(GenerateError::Split {
            id: "RootAt0".to_owned(),
            regex: "1(px|em)".to_owned(),
        }))
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

/// A match named after its first group fails when it has no group.
#[test]
fn a_name_of_a_missing_group_does_not_split() {
    let json = r#"{"scopeName": "source.demo", "patterns": [
        {"match": "\\bfoo\\b", "name": "keyword.$1.demo"}]}"#;
    assert_eq!(
        ids(json, &PLAIN),
        Err(Failure::Generate(GenerateError::Split {
            id: "RootAt0".to_owned(),
            regex: r"\bfoo\b".to_owned(),
        }))
    );
}

/// A group splits at each `|` at its own level.
/// A `|` or `)` in an inner group, after a `\`, or in a character class
/// splits nothing and ends nothing.
#[test]
fn a_group_splits_at_its_own_level() {
    for (regex, split) in [
        (
            r"\b(foo(?:bar|qux)?|baz)\b",
            [r"\b(foo(?:bar|qux)?)\b", r"\b(baz)\b"],
        ),
        (r"(a\)|b)c", [r"(a\))c", "(b)c"]),
        (r"([)|]|b)c", ["([)|])c", "(b)c"]),
        (r"([_[:alpha:]]|b)c", ["([_[:alpha:]])c", "(b)c"]),
    ] {
        let json = serde_json::json!({"scopeName": "source.demo", "patterns": [
            {"match": regex, "name": "keyword.$1.demo"}]});
        assert_eq!(
            regexes(&json.to_string(), &PLAIN),
            Ok(split.map(str::to_owned).to_vec()),
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

/// Under `Reentry::WithinWalk`, a rule a begin opens inside a list that
/// reaches itself gets its full list from the second pass.
#[test]
fn the_second_pass_lists_what_the_walk_cut_short() {
    let json = r##"{"scopeName": "source.demo", "patterns": [{"include": "#list"}],
        "repository": {"list": {"patterns": [
            {"begin": "b", "end": "e", "patterns": [{"include": "#list"}]},
            {"match": "m"}]}}}"##;
    let walk_wide = RuleTables {
        reentry: Reentry::WithinWalk,
        ..PLAIN
    };
    assert_eq!(
        tried(json, &walk_wide, "ListAt0"),
        Ok(Some(vec!["ListAt0".to_owned(), "ListAt1".to_owned()]))
    );
}

/// A list whose key takes a begin's id in Pascal case gives its patterns ids
/// from that context, and takes no pattern id itself.
#[test]
fn a_list_takes_no_pattern_id() {
    let json = r##"{"scopeName": "source.demo",
        "patterns": [{"include": "#foo"}, {"include": "#foo-"}],
        "repository": {"foo": {"begin": "b", "end": "e"},
            "foo-": {"patterns": [{"match": "m"}]}}}"##;
    assert_eq!(
        ids(json, &PLAIN),
        Ok(expected(&[
            ("Foo", Act::Begin),
            ("FooEnd", Act::End),
            ("FooAt0", Act::Match),
        ]))
    );
}

/// An include of another grammar that the language lists as adding nothing
/// adds nothing, and the patterns after it keep their places.
#[test]
fn an_external_that_adds_nothing_keeps_the_places_after_it() {
    let json = r#"{"scopeName": "source.demo", "patterns": [
        {"include": "source.gone"}, {"match": "a"}]}"#;
    let gone = RuleTables {
        externals: &[("source.gone", External::AddsNothing)],
        ..PLAIN
    };
    assert_eq!(ids(json, &gone), Ok(expected(&[("RootAt1", Act::Match)])));
}

/// An include of another grammar that the language replaces with a begin
/// gives a begin with the include's inline id, its end and an empty rule.
#[test]
fn an_external_begin_takes_the_include_id() {
    let json = r##"{"scopeName": "source.demo", "patterns": [{"include": "#block"}],
        "repository": {"block": {"begin": "b", "end": "e",
            "patterns": [{"match": "m"}, {"include": "source.c"}]}}}"##;
    let made = RuleTables {
        externals: &[(
            "source.c",
            External::Begin {
                begin: "\\G",
                end: "x",
            },
        )],
        ..PLAIN
    };
    assert_eq!(
        ids(json, &made),
        Ok(expected(&[
            ("Block", Act::Begin),
            ("BlockEnd", Act::End),
            ("BlockAt0", Act::Match),
            ("BlockAt1", Act::Begin),
            ("BlockAt1End", Act::End),
        ]))
    );
    assert_eq!(tried(json, &made, "BlockAt1"), Ok(Some(Vec::new())));
}

/// A repository key `root`, whose list takes the id `Root`, still adds its
/// patterns to the top-level list.
#[test]
fn a_list_under_the_key_root_adds_its_patterns() {
    let json = r##"{"scopeName": "source.demo", "patterns": [{"include": "#root"}],
        "repository": {"root": {"patterns": [{"match": "r"}]}}}"##;
    assert_eq!(
        tried(json, &PLAIN, "Root"),
        Ok(Some(vec!["RootAt0".to_owned()]))
    );
}

/// A repository key `$self`, which Pascal case leaves as it is, still adds
/// its patterns to the top-level list.
#[test]
fn a_list_under_the_key_self_adds_its_patterns() {
    let json = r##"{"scopeName": "source.demo", "patterns": [{"include": "#$self"}],
        "repository": {"$self": {"patterns": [{"match": "s"}]}}}"##;
    assert_eq!(
        tried(json, &PLAIN, "Root"),
        Ok(Some(vec!["$selfAt0".to_owned()]))
    );
}

/// Under content identity, a pattern reached again with the same contents
/// keeps its first id, though its new place names it otherwise.
#[test]
fn content_identity_keeps_the_first_id_of_equal_patterns() {
    let json = r##"{"scopeName": "source.demo", "patterns": [
        {"match": "a"}, {"include": "#again"}],
        "repository": {"again": {"match": "a"}}}"##;
    let by_content = RuleTables {
        identity: Identity::Content,
        ..PLAIN
    };
    assert_eq!(
        ids(json, &by_content),
        Ok(expected(&[("RootAt0", Act::Match)]))
    );
    assert_eq!(
        tried(json, &by_content, "Root"),
        Ok(Some(vec!["RootAt0".to_owned(), "RootAt0".to_owned()]))
    );
}

/// A begin whose own list holds only a missing grammar is dropped, with its
/// end and rule, and stays dropped when reached again.
#[test]
fn a_begin_with_only_a_missing_grammar_is_dropped() {
    let json = r##"{"scopeName": "source.demo", "patterns": [
        {"include": "#plain"}, {"match": "a"}, {"include": "#plain"}],
        "repository": {"plain": {"begin": "b", "end": "e",
            "patterns": [{"include": "text.plain"}]}}}"##;
    let missing = RuleTables {
        externals: &[("text.plain", External::Missing)],
        identity: Identity::Content,
        ..PLAIN
    };
    assert_eq!(
        ids(json, &missing),
        Ok(expected(&[("RootAt1", Act::Match)]))
    );
    assert_eq!(rule_ids(json, &missing), Ok(vec!["Root".to_owned()]));
}

/// A rule that embeds a grammar and whose end repeats a group of its begin
/// lists one made-up begin, its body, which ends where the rule would.
#[test]
fn an_embedding_rule_with_a_backreference_lists_its_body() {
    let json = r#"{"scopeName": "source.demo", "patterns": [
        {"begin": "^(\\s*):ruby", "end": "^(?!\\1\\s+)",
            "patterns": [{"include": "source.ruby"}]}]}"#;
    let embeds = RuleTables {
        externals: &[("source.ruby", External::Embeds)],
        ..PLAIN
    };
    assert_eq!(
        ids(json, &embeds),
        Ok(expected(&[
            ("RootAt0", Act::Begin),
            ("RootAt0End", Act::End),
            ("RootAt0Body", Act::Begin),
            ("RootAt0BodyEnd", Act::End),
        ]))
    );
    assert_eq!(
        tried(json, &embeds, "RootAt0"),
        Ok(Some(vec!["RootAt0Body".to_owned()]))
    );
    assert_eq!(tried(json, &embeds, "RootAt0Body"), Ok(Some(Vec::new())));
}

/// A rule that embeds a grammar in a body cannot list patterns of its own.
#[test]
fn an_embedding_body_beside_patterns_is_an_error() {
    let json = r#"{"scopeName": "source.demo", "patterns": [
        {"begin": "(a)", "end": "\\1",
            "patterns": [{"match": "m"}, {"include": "source.ruby"}]}]}"#;
    let embeds = RuleTables {
        externals: &[("source.ruby", External::Embeds)],
        ..PLAIN
    };
    assert_eq!(
        ids(json, &embeds),
        Err(Failure::Generate(GenerateError::EmbedsBesidePatterns {
            id: "RootAt0".to_owned()
        }))
    );
}

/// A language that reads a match from another grammar's repository.
const READS_RUBY: RuleTables = RuleTables {
    externals: &[(
        "source.ruby#escaped_char",
        External::Pattern {
            grammar: "ruby",
            key: "escaped_char",
        },
    )],
    ..PLAIN
};

/// The tables of the grammar `json`, read as [`READS_RUBY`] says, with
/// `ruby` as the grammar `ruby`.
fn with_ruby(json: &str, ruby: &str) -> Result<Tables, Failure> {
    let load = |json| {
        Grammar::from_json(json)
            .map(unlinked)
            .map_err(|error| Failure::Grammar(error.to_string()))
    };
    flatten(load(json)?, &[("ruby", load(ruby)?)], &READS_RUBY).map_err(Failure::Generate)
}

/// A match of another grammar takes that grammar's name and its key in
/// Pascal case, and its place the first time the walk reaches it.
#[test]
fn an_external_pattern_takes_the_grammar_and_key_once() {
    let json = r#"{"scopeName": "source.demo", "patterns": [
        {"include": "source.ruby#escaped_char"}, {"match": "a"},
        {"include": "source.ruby#escaped_char"}]}"#;
    let ruby = r#"{"scopeName": "source.ruby", "patterns": [],
        "repository": {"escaped_char": {"match": "\\\\."}}}"#;
    let tables = with_ruby(json, ruby);
    let patterns = tables.map(|tables| {
        tables
            .patterns
            .into_iter()
            .map(|pattern| (pattern.id, pattern.regex))
            .collect::<Vec<_>>()
    });
    assert_eq!(
        patterns,
        Ok(vec![
            ("RubyEscapedChar".to_owned(), "\\\\.".to_owned()),
            ("RootAt1".to_owned(), "a".to_owned()),
        ])
    );
}

/// An external pattern whose entry is not a plain match is an error.
#[test]
fn an_external_pattern_that_is_no_match_is_an_error() {
    let json = r#"{"scopeName": "source.demo", "patterns": [
        {"include": "source.ruby#escaped_char"}]}"#;
    let ruby = r#"{"scopeName": "source.ruby", "patterns": [],
        "repository": {"escaped_char": {"begin": "b", "end": "e"}}}"#;
    assert_eq!(
        with_ruby(json, ruby).map(|tables| tables.patterns.len()),
        Err(Failure::Generate(GenerateError::ExternalPattern {
            include: "source.ruby#escaped_char".to_owned()
        }))
    );
}

/// A list entry whose own patterns are one dropped include is left out, and
/// the entries after it take its place.
#[test]
fn an_entry_of_one_dropped_include_takes_no_place() {
    let json = r#"{"scopeName": "source.demo", "patterns": [
        {"begin": "/", "end": "/", "patterns": [{"include": "source.js.regexp"}]},
        {"match": "a"}]}"#;
    let drops = RuleTables {
        externals: &[("source.js.regexp", External::DropsItsEntry)],
        ..PLAIN
    };
    assert_eq!(ids(json, &drops), Ok(expected(&[("RootAt0", Act::Match)])));
}

/// Under named capture rules, a capture list of one repository include takes
/// the key's id, a list of one capture-rule include takes that rule's id,
/// and any other list takes `Nested` and a number.
#[test]
fn named_capture_rules_take_the_key_or_the_external_id() {
    let json = r##"{"scopeName": "source.demo", "patterns": [{"match": "(a)(b)(c)",
        "captures": {"1": {"patterns": [{"include": "#function_names"}]},
            "2": {"patterns": [{"include": "source.js"}]},
            "3": {"patterns": [{"match": "c"}]}}}],
        "repository": {"function_names": {"patterns": [{"match": "f"}]}}}"##;
    let named = RuleTables {
        capture_lists: CaptureLists::NamedRules,
        externals: &[("source.js", External::CaptureRule { id: "Javascript" })],
        ..PLAIN
    };
    assert_eq!(
        rule_ids(json, &named),
        Ok(vec![
            "FunctionNames".to_owned(),
            "Javascript".to_owned(),
            "Nested2".to_owned(),
            "Root".to_owned(),
        ])
    );
    assert_eq!(tried(json, &named, "Javascript"), Ok(Some(Vec::new())));
}

/// A language that takes the matches Svelte injects into it.
const INJECTED: RuleTables = RuleTables {
    injected: Some(Injection {
        grammar: "svelte",
        selector: "L:source.demo",
        ids: &["SvelteVariable", "SvelteProps"],
    }),
    ..PLAIN
};

/// The tables of the grammar `json`, read as [`INJECTED`] says, with
/// `svelte` as the grammar `svelte`.
fn with_svelte(json: &str, svelte: &str) -> Result<Tables, Failure> {
    let load = |json| {
        Grammar::from_json(json)
            .map(unlinked)
            .map_err(|error| Failure::Grammar(error.to_string()))
    };
    flatten(load(json)?, &[("svelte", load(svelte)?)], &INJECTED).map_err(Failure::Generate)
}

/// Injected matches follow every other pattern, and no rule lists them.
#[test]
fn injected_matches_follow_every_pattern_and_no_rule_lists_them() {
    let json = r#"{"scopeName": "source.demo", "patterns": [{"match": "a"}]}"#;
    let svelte = r#"{"scopeName": "source.svelte", "patterns": [],
        "injections": {"L:source.demo": {"patterns": [{"match": "v"}, {"match": "p"}]}}}"#;
    let tables = with_svelte(json, svelte);
    assert_eq!(
        tables.as_ref().map(|tables| tables
            .patterns
            .iter()
            .map(|pattern| pattern.id.as_str())
            .collect::<Vec<_>>()),
        Ok(vec!["RootAt0", "SvelteVariable", "SvelteProps"])
    );
    assert_eq!(
        tables.map(|tables| tables
            .rules
            .into_iter()
            .map(|rule| rule.patterns)
            .collect::<Vec<_>>()),
        Ok(vec![vec![0]])
    );
}

/// An injection with another number of patterns than ids is an error.
#[test]
fn an_injection_of_another_length_is_an_error() {
    let json = r#"{"scopeName": "source.demo", "patterns": []}"#;
    let svelte = r#"{"scopeName": "source.svelte", "patterns": [],
        "injections": {"L:source.demo": {"patterns": [{"match": "v"}]}}}"#;
    assert_eq!(
        with_svelte(json, svelte).map(|tables| tables.patterns.len()),
        Err(Failure::Generate(GenerateError::Injection {
            selector: "L:source.demo".to_owned()
        }))
    );
}

/// A backslash before a backslash and a digit still repeats a group, as in
/// vscode-textmate, so an embedding rule whose end holds `\\1` lists a
/// body.
#[test]
fn a_backslash_before_a_backreference_still_repeats_its_group() {
    let json = r#"{"scopeName": "source.demo", "patterns": [
        {"begin": "a", "end": "\\\\1", "patterns": [{"include": "source.ruby"}]}]}"#;
    let embeds = RuleTables {
        externals: &[("source.ruby", External::Embeds)],
        ..PLAIN
    };
    assert_eq!(
        ids(json, &embeds),
        Ok(expected(&[
            ("RootAt0", Act::Begin),
            ("RootAt0End", Act::End),
            ("RootAt0Body", Act::Begin),
            ("RootAt0BodyEnd", Act::End)
        ]))
    );
}

/// A group number runs over every digit, so `\1` and `\10` are two groups,
/// and an embedding rule whose end holds both lists no body.
#[test]
fn a_two_digit_backreference_is_its_own_group() {
    let json = r#"{"scopeName": "source.demo", "patterns": [
        {"begin": "a", "end": "\\1\\10", "patterns": [{"include": "source.ruby"}]}]}"#;
    let embeds = RuleTables {
        externals: &[("source.ruby", External::Embeds)],
        ..PLAIN
    };
    assert_eq!(
        ids(json, &embeds),
        Ok(expected(&[
            ("RootAt0", Act::Begin),
            ("RootAt0End", Act::End)
        ]))
    );
}

/// A named backreference, `\k<name>`, repeats a group of the end itself.
/// An end that holds it beside `\1` repeats one group of the begin, and an
/// embedding rule lists a body.
#[test]
fn a_named_backreference_repeats_no_begin_group() {
    let json = r#"{"scopeName": "source.demo", "patterns": [
        {"begin": "(a)", "end": "(?<q>e)\\k<q>\\1",
            "patterns": [{"include": "source.ruby"}]}]}"#;
    let embeds = RuleTables {
        externals: &[("source.ruby", External::Embeds)],
        ..PLAIN
    };
    assert_eq!(
        ids(json, &embeds),
        Ok(expected(&[
            ("RootAt0", Act::Begin),
            ("RootAt0End", Act::End),
            ("RootAt0Body", Act::Begin),
            ("RootAt0BodyEnd", Act::End)
        ]))
    );
}

/// A begin to drop whose end captures add rules is an error, as the walk
/// drops only the begin, its end and its rule.
#[test]
fn a_dropped_begin_with_end_capture_rules_is_an_error() {
    let json = r#"{"scopeName": "source.demo", "patterns": [
        {"begin": "b", "end": "(e)", "endCaptures": {"1": {"patterns": [{"match": "e"}]}},
            "patterns": [{"include": "text.plain"}]}]}"#;
    let missing = RuleTables {
        externals: &[("text.plain", External::Missing)],
        ..PLAIN
    };
    assert_eq!(
        ids(json, &missing),
        Err(Failure::Generate(GenerateError::DroppedBeginCaptures {
            id: "RootAt0".to_owned()
        }))
    );
}

/// A language that adds a rule from a list in the grammar `host`.
const ADDS_STYLE: RuleTables = RuleTables {
    added_rules: &[AddedRule {
        id: "Style",
        grammar: "host",
        key: "style-attr",
        path: &[0],
    }],
    ..PLAIN
};

/// The grammar `host`, whose `style-attr` entry holds a string begin.
/// The string's list includes an entry of the demo grammar.
const HOST: &str = r#"{"scopeName": "text.host", "patterns": [],
    "repository": {"style-attr": {"begin": "s=", "end": "$", "patterns": [
        {"begin": "'", "end": "'", "patterns": [
            {"include": "source.demo#escapes"}, {"match": "--\\w+"}]}]}}}"#;

/// The tables of the grammar `json`, read as `language` says, with `host`
/// as the grammar `host`.
fn with_host(json: &str, host: &str, language: &RuleTables) -> Result<Tables, Failure> {
    let load = |json| {
        Grammar::from_json(json)
            .map(unlinked)
            .map_err(|error| Failure::Grammar(error.to_string()))
    };
    flatten(load(json)?, &[("host", load(host)?)], language).map_err(Failure::Generate)
}

/// An added rule lists the patterns of the list its path reaches, after
/// `Root`'s.
/// An include of the language's own scope with a key adds its own entry.
#[test]
fn an_added_rule_lists_another_grammars_list() {
    let json = r#"{"scopeName": "source.demo", "patterns": [{"match": "a"}],
        "repository": {"escapes": {"match": "\\\\."}}}"#;
    let tables = with_host(json, HOST, &ADDS_STYLE).map(|tables| {
        let patterns = tables
            .patterns
            .iter()
            .map(|pattern| (pattern.id.clone(), pattern.regex.clone()))
            .collect::<Vec<_>>();
        let style = tables
            .rules
            .iter()
            .find(|rule| rule.id == "Style")
            .map(|rule| rule.patterns.clone());
        (patterns, style)
    });
    assert_eq!(
        tables,
        Ok((
            vec![
                ("RootAt0".to_owned(), "a".to_owned()),
                ("Escapes".to_owned(), "\\\\.".to_owned()),
                ("StyleAt1".to_owned(), "--\\w+".to_owned()),
            ],
            Some(vec![1, 2]),
        ))
    );
}

/// An added rule whose path reaches no list is an error.
#[test]
fn an_added_rule_past_the_list_is_an_error() {
    let json = r#"{"scopeName": "source.demo", "patterns": []}"#;
    let past = RuleTables {
        added_rules: &[AddedRule {
            id: "Style",
            grammar: "host",
            key: "style-attr",
            path: &[0, 5],
        }],
        ..PLAIN
    };
    assert_eq!(
        with_host(json, HOST, &past).map(|tables| tables.patterns.len()),
        Err(Failure::Generate(GenerateError::AddedRule {
            id: "Style".to_owned()
        }))
    );
}

/// A language that merges the matches of `Root`'s first entry into `Word`.
const MERGES_ONE: RuleTables = RuleTables {
    merged_words: Some(MergedWords {
        id: "Word",
        entries: 1,
    }),
    ..PLAIN
};

/// The tables after a merge, as the merge tests compare them.
#[derive(Debug, PartialEq, Eq)]
struct Merged {
    /// Each pattern's id and expression, in table order.
    patterns: Vec<(String, String)>,
    /// The ids of the patterns `Root` tries, in its order.
    root: Vec<String>,
}

/// The tables of the grammar `json` read as `language` says, as
/// [`Merged`] holds them.
fn merged(json: &str, language: &RuleTables) -> Result<Merged, Failure> {
    let tables = tables(json, language)?;
    let root = tried(json, language, "Root")?.unwrap_or_default();
    let patterns = tables
        .patterns
        .into_iter()
        .map(|pattern| (pattern.id, pattern.regex))
        .collect();
    Ok(Merged { patterns, root })
}

/// The matches of `Root`'s first entries merge into one match of their
/// words, in their order.
/// It follows every other pattern, and `Root` tries it first.
#[test]
fn the_first_entries_matches_merge_into_one() {
    let json = r##"{"scopeName": "source.demo",
        "patterns": [{"include": "#a"}, {"include": "#b"}, {"match": "x+"}],
        "repository": {
            "a": {"patterns": [{"match": "(?i)\\b(ax|bx)\\b", "name": "r"}]},
            "b": {"patterns": [{"match": "(?i)\\b(mov|add)\\b", "name": "m"},
                {"match": "(?i)\\bnop\\b"}]}}}"##;
    let merges = RuleTables {
        merged_words: Some(MergedWords {
            id: "Word",
            entries: 2,
        }),
        ..PLAIN
    };
    assert_eq!(
        merged(json, &merges),
        Ok(Merged {
            patterns: vec![
                ("RootAt2".to_owned(), "x+".to_owned()),
                (
                    "Word".to_owned(),
                    r"(?i)\b(?:(ax|bx)|(mov|add)|nop)\b".to_owned()
                ),
            ],
            root: vec!["Word".to_owned(), "RootAt2".to_owned()],
        })
    );
}

/// A match to merge whose capture names a scope is an error, as the merged
/// match keeps no capture.
#[test]
fn a_named_capture_does_not_merge() {
    let json = r##"{"scopeName": "source.demo", "patterns": [{"include": "#a"}],
        "repository": {"a": {"patterns": [{"match": "(?i)\\b(ax)\\b",
            "captures": {"1": {"name": "r"}}}]}}}"##;
    assert_eq!(
        merged(json, &MERGES_ONE),
        Err(Failure::Generate(GenerateError::NotWords {
            id: "AAt0".to_owned()
        }))
    );
}

/// A match of plain words right after the merged ones is an error, since
/// the merge would leave it out.
#[test]
fn a_word_match_after_the_merge_is_an_error() {
    let json = r##"{"scopeName": "source.demo",
        "patterns": [{"include": "#a"}, {"match": "(?i)\\b(cx)\\b"}],
        "repository": {"a": {"patterns": [{"match": "(?i)\\b(ax)\\b"}]}}}"##;
    assert_eq!(
        merged(json, &MERGES_ONE),
        Err(Failure::Generate(GenerateError::WordsFollow {
            id: "RootAt1".to_owned()
        }))
    );
}

/// A rule that lists a merged match too is an error, since the match leaves
/// the tables.
#[test]
fn a_merged_match_in_another_rule_is_an_error() {
    let json = r##"{"scopeName": "source.demo",
        "patterns": [{"include": "#a"}, {"begin": "<", "end": ">",
            "patterns": [{"include": "#a"}]}],
        "repository": {"a": {"patterns": [{"match": "(?i)\\b(ax)\\b"}]}}}"##;
    assert_eq!(
        merged(json, &MERGES_ONE),
        Err(Failure::Generate(GenerateError::MergedElsewhere {
            id: "RootAt1".to_owned()
        }))
    );
}

/// Each pattern's expression, in table order, from the grammar `json` read
/// as `language` says.
fn regexes(json: &str, language: &RuleTables) -> Result<Vec<String>, Failure> {
    Ok(tables(json, language)?
        .patterns
        .into_iter()
        .map(|pattern| pattern.regex)
        .collect())
}

/// `$0` names the whole match.
/// No group holds it, so the pattern does not split.
#[test]
fn a_name_of_the_whole_match_does_not_split() {
    let json = r#"{"scopeName": "source.demo", "patterns": [
        {"match": "\\b(a|b)\\b", "name": "keyword.$0.demo"}]}"#;
    assert_eq!(regexes(json, &PLAIN), Ok(vec![r"\b(a|b)\b".to_owned()]));
}

/// A named group takes a number, as Shiki's Oniguruma gives it one.
/// A lookbehind takes none.
#[test]
fn a_named_group_counts_and_a_lookbehind_does_not() {
    let json = r#"{"scopeName": "source.demo", "patterns": [
        {"match": "(?<=y)(?<q>x)(a|b)", "name": "keyword.$2.demo"}]}"#;
    assert_eq!(
        regexes(json, &PLAIN),
        Ok(vec![
            "(?<=y)(?<q>x)(a)".to_owned(),
            "(?<=y)(?<q>x)(b)".to_owned()
        ])
    );
}

/// A name that holds a named group's text does not split, as the split
/// reads only a plain group as words.
#[test]
fn a_name_of_a_named_group_is_an_error() {
    let json = r#"{"scopeName": "source.demo", "patterns": [
        {"match": "(?<q>a|b)", "name": "keyword.$1.demo"}]}"#;
    assert_eq!(
        regexes(json, &PLAIN),
        Err(Failure::Generate(GenerateError::Split {
            id: "RootAt0".to_owned(),
            regex: "(?<q>a|b)".to_owned(),
        }))
    );
}

/// Dropping a group that a later group follows would change the later
/// group's number, so the split refuses it.
#[test]
fn dropping_a_group_before_another_is_an_error() {
    let json = r#"{"scopeName": "source.demo", "patterns": [
        {"match": "(a|b)(c)", "name": "keyword.$1.demo"}]}"#;
    let drops = RuleTables {
        split: Split::DropsGroup,
        ..PLAIN
    };
    assert_eq!(
        regexes(json, &drops),
        Err(Failure::Generate(GenerateError::Split {
            id: "RootAt0".to_owned(),
            regex: "(a|b)(c)".to_owned(),
        }))
    );
}

/// The optional group that splits is the innermost optional one, past any
/// plain group that lies between it and the named group.
#[test]
fn the_innermost_optional_group_splits_past_a_plain_one() {
    let json = r#"{"scopeName": "source.demo", "patterns": [
        {"match": "1(?:(%)|((px|em))\\b)?",
            "captures": {"3": {"name": "unit.$3.demo"}}}]}"#;
    let splits = RuleTables {
        capture_names: CaptureNames::SplitsOptionalGroup,
        ..PLAIN
    };
    assert_eq!(
        regexes(json, &splits),
        Ok(vec![
            "1(?:(%))".to_owned(),
            r"1(?:((px|em))\b)".to_owned(),
            "1".to_owned(),
        ])
    );
}

/// A lazy or possessive optional group is no plain `?`, so it does not
/// split.
#[test]
fn a_lazy_optional_group_is_an_error() {
    let json = r#"{"scopeName": "source.demo", "patterns": [
        {"match": "1(?:(%)|(px))??",
            "captures": {"2": {"name": "unit.$2.demo"}}}]}"#;
    let splits = RuleTables {
        capture_names: CaptureNames::SplitsOptionalGroup,
        ..PLAIN
    };
    assert_eq!(
        regexes(json, &splits),
        Err(Failure::Generate(GenerateError::Split {
            id: "RootAt0".to_owned(),
            regex: "1(?:(%)|(px))??".to_owned(),
        }))
    );
}

/// `\01` and `\1` both repeat group 1, as vscode-textmate reads them, so an
/// embedding rule whose end holds both lists a body.
#[test]
fn a_leading_zero_names_the_same_group() {
    let json = r#"{"scopeName": "source.demo", "patterns": [
        {"begin": "(a)", "end": "\\1\\01", "patterns": [{"include": "source.ruby"}]}]}"#;
    let embeds = RuleTables {
        externals: &[("source.ruby", External::Embeds)],
        ..PLAIN
    };
    assert_eq!(
        ids(json, &embeds),
        Ok(expected(&[
            ("RootAt0", Act::Begin),
            ("RootAt0End", Act::End),
            ("RootAt0Body", Act::Begin),
            ("RootAt0BodyEnd", Act::End)
        ]))
    );
}
