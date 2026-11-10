//! The compiler and the linker on small grammars, and the survey that holds
//! the linker to `oracle/engine-tables.ts` on every grammar Shiki bundles.

use indoc::{formatdoc, indoc};

use super::link::{LinkError, link};
use super::{UnlinkedGrammar, compile};
use crate::textmate::grammar::{Grammar, Matching, NONE};

/// The grammar of `json`, which must compile.
fn grammar(json: &str) -> UnlinkedGrammar {
    compile(json).unwrap()
}

/// The patterns of the rules in the list of `rule`.
fn list_patterns(grammar: &Grammar, rule: usize) -> Vec<&'static str> {
    let list = grammar.rules[rule].list;
    grammar.lists[list]
        .rules
        .iter()
        .map(|&id| grammar.patterns[grammar.rules[id].begin].source)
        .collect()
}

#[test]
fn a_repeated_key_keeps_its_first_place_and_its_last_value() {
    use super::json::{Json, Number};
    let parsed = Json::parse(r#"{"a": 1, "b": 2, "a": 3}"#).unwrap();
    assert_eq!(
        parsed,
        Json::Object(vec![
            ("a".to_owned(), Json::Number(Number::Unsigned(3))),
            ("b".to_owned(), Json::Number(Number::Unsigned(2))),
        ])
    );
}

#[test]
fn a_grammar_without_a_scope_does_not_compile() {
    assert!(compile(r#"{"patterns": []}"#).is_err());
    assert!(compile("[").is_err());
}

#[test]
fn includes_inside_a_grammar_are_followed() {
    let root = grammar(indoc! {r##"
        {
          "scopeName": "source.a",
          "patterns": [{"include": "#word"}, {"include": "$self"}, {"match": "b"}],
          "repository": {
            "word": {"patterns": [{"match": "a", "name": "keyword.a"}, {"include": "#word"}]}
          }
        }
    "##});
    let tables = link(&root, &[], None).unwrap();
    assert_eq!(list_patterns(tables, tables.root), ["a", "b"]);
}

#[test]
fn a_rule_with_only_an_include_holds_it() {
    let root = grammar(indoc! {r##"
        {
          "scopeName": "source.a",
          "patterns": [{"include": "#alias"}],
          "repository": {
            "alias": {"include": "#word"},
            "word": {"match": "a"}
          }
        }
    "##});
    let tables = link(&root, &[], None).unwrap();
    assert_eq!(list_patterns(tables, tables.root), ["a"]);
}

#[test]
fn nested_repositories_shadow_outer_ones() {
    let root = grammar(indoc! {r##"
        {
          "scopeName": "source.a",
          "patterns": [
            {"include": "#word"},
            {"patterns": [{"include": "#word"}], "repository": {"word": {"match": "inner"}}}
          ],
          "repository": {"word": {"match": "outer"}}
        }
    "##});
    let tables = link(&root, &[], None).unwrap();
    assert_eq!(list_patterns(tables, tables.root), ["outer", "inner"]);
}

#[test]
fn grammars_that_include_each_other_link_in_either_order() {
    let a = grammar(indoc! {r#"
        {"scopeName": "source.a", "patterns": [{"match": "a"}, {"include": "source.b"}]}
    "#});
    let b = grammar(indoc! {r#"
        {
          "scopeName": "source.b",
          "patterns": [{"match": "b"}, {"include": "source.a"}, {"include": "$base"}]
        }
    "#});
    for set in [[&a, &b], [&b, &a]] {
        let tables = link(&a, &set, None).unwrap();
        assert_eq!(list_patterns(tables, tables.root), ["a", "b"]);
    }
}

#[test]
fn an_include_of_a_missing_grammar_drops_its_rule() {
    let root = grammar(indoc! {r#"
        {
          "scopeName": "source.a",
          "patterns": [
            {"begin": "x", "end": "y", "patterns": [{"include": "source.missing"}]},
            {"begin": "p", "end": "q", "patterns": [{"include": "source.missing"}, {"match": "r"}]}
          ]
        }
    "#});
    let tables = link(&root, &[], None).unwrap();
    assert_eq!(list_patterns(tables, tables.root), ["p"]);
}

#[test]
fn captures_take_the_order_of_their_numbers() {
    let root = grammar(indoc! {r#"
        {
          "scopeName": "source.a",
          "patterns": [
            {
              "match": "(a)(b)",
              "captures": {
                "2": {"name": "two"},
                "1": {"name": "one", "patterns": [{"match": "a"}]},
                "x": {"name": "no"}
              }
            }
          ]
        }
    "#});
    let tables = link(&root, &[], None).unwrap();
    let rule = &tables.rules[tables.lists[tables.rules[tables.root].list].rules[0]];
    let indices: Vec<usize> = rule.captures.iter().map(|c| c.index).collect();
    assert_eq!(indices, [1, 2]);
    assert_ne!(rule.captures[0].retokenize, NONE);
    assert_eq!(rule.captures[1].retokenize, NONE);
    assert_eq!(
        tables.rules[rule.captures[0].retokenize].matching,
        Matching::Container
    );
}

#[test]
fn injections_keep_the_order_of_the_text() {
    let root = grammar(indoc! {r#"
        {
          "scopeName": "source.a",
          "patterns": [],
          "injections": {"R:z": {"match": "z"}, "L:a": {"match": "a"}}
        }
    "#});
    let tables = link(&root, &[], None).unwrap();
    let selectors: Vec<&str> = tables.injections.iter().map(|i| i.selector).collect();
    assert_eq!(selectors, ["R:z", "L:a"]);
}

#[test]
fn a_grammar_injects_itself_where_it_names_the_root() {
    let root = grammar(r#"{"scopeName": "source.a", "patterns": []}"#);
    let injected = grammar(indoc! {r#"
        {
          "scopeName": "source.i",
          "injectionSelector": "L:source.a",
          "injectTo": ["source.a"],
          "patterns": [{"match": "i"}]
        }
    "#});
    let tables = link(&root, &[&injected], None).unwrap();
    assert_eq!(tables.injections.len(), 1);
}

/// A repository of `entries` rules, each including the next, and a root
/// that includes the first.
fn chain(entries: usize) -> String {
    chain_with_root(entries, r##"{"include": "#r0"}"##)
}

/// The chain of [`chain`], with a root that includes every entry from the
/// far end.
/// Each entry then compiles after the one it includes, so the link never
/// nests deep while it compiles.
fn chain_from_far_end(entries: usize) -> String {
    let includes: Vec<String> = (0..entries)
        .rev()
        .map(|i| format!(r##"{{"include": "#r{i}"}}"##))
        .collect();
    chain_with_root(entries, &includes.join(", "))
}

/// A repository of `entries` rules, each including the next, and a root
/// whose patterns are `root`.
fn chain_with_root(entries: usize, root: &str) -> String {
    let repository: Vec<String> = (0..entries)
        .map(|i| {
            format!(
                r##""r{i}": {{"patterns": [{{"include": "#r{}"}}, {{"match": "m{i}"}}]}}"##,
                i + 1
            )
        })
        .collect();
    format!(
        r#"{{"scopeName": "source.a", "patterns": [{root}], "repository": {{{}}}}}"#,
        repository.join(", ")
    )
}

#[test]
fn a_chain_of_includes_past_the_depth_bound_is_refused() {
    let root = grammar(&chain(100_000));
    assert_eq!(link(&root, &[], None).unwrap_err(), LinkError::TooDeep);
}

#[test]
fn a_chain_of_includes_within_the_depth_bound_links() {
    let root = grammar(&chain(200));
    let tables = link(&root, &[], None).unwrap();
    assert_eq!(list_patterns(tables, tables.root).len(), 200);
}

#[test]
fn a_long_chain_compiled_from_its_far_end_is_refused() {
    let root = grammar(&chain_from_far_end(100_000));
    assert_eq!(link(&root, &[], None).unwrap_err(), LinkError::TooLarge);
}

#[test]
fn a_short_chain_compiled_from_its_far_end_links() {
    let root = grammar(&chain_from_far_end(1_000));
    let tables = link(&root, &[], None).unwrap();
    assert_eq!(list_patterns(tables, tables.root).len(), 1_000);
}

#[cfg(feature = "lang-css")]
mod hand_offs {
    use indoc::indoc;

    use super::{grammar, list_patterns};
    use crate::language::Language;
    use crate::textmate::compile::link::link;

    /// A grammar whose `style` region includes CSS whole, and one CSS rule.
    fn styled() -> super::UnlinkedGrammar {
        grammar(indoc! {r#"
            {
              "scopeName": "source.a",
              "patterns": [
                {
                  "begin": "<style>",
                  "end": "</style>",
                  "patterns": [{"match": "!"}, {"include": "source.css"}, {"match": "\\?"}]
                },
                {"include": "source.css#escapes"}
              ]
            }
        "#})
    }

    fn css() -> super::UnlinkedGrammar {
        grammar(indoc! {r#"
            {
              "scopeName": "source.css",
              "patterns": [{"match": "css"}],
              "repository": {"escapes": {"match": "\\\\."}}
            }
        "#})
    }

    #[test]
    fn a_hand_lexed_grammar_missing_from_the_set_is_handed_off() {
        let tables = link(&styled(), &[], None).unwrap();
        assert_eq!(tables.hand_offs.len(), 1);
        assert_eq!(tables.hand_offs[0].language, Language::Css);
        assert_eq!(list_patterns(tables, tables.root), ["<style>"]);
        let style = tables.lists[tables.rules[tables.root].list].rules[0];
        assert_eq!(list_patterns(tables, style), ["!", "\\?"]);
        let hand_off = tables.hand_off(tables.rules[style].list).unwrap();
        assert_eq!(tables.lists[hand_off.before].rules.len(), 1);
        assert_eq!(tables.lists[hand_off.after].rules.len(), 1);
    }

    #[test]
    fn a_grammar_in_the_set_wins_over_its_hand_lexer() {
        let css = css();
        let tables = link(&styled(), &[&css], None).unwrap();
        assert!(tables.hand_offs.is_empty());
        assert_eq!(list_patterns(tables, tables.root), ["<style>", "\\\\."]);
    }

    #[test]
    fn a_listed_hand_lexer_wins_and_its_grammar_serves_fragments() {
        let css = css();
        let tables = link(&styled(), &[&css], Some(&[Language::Css])).unwrap();
        assert_eq!(tables.hand_offs.len(), 1);
        assert_eq!(list_patterns(tables, tables.root), ["<style>", "\\\\."]);
    }

    /// Neither CSS nor HTML reads the other, so the first include takes the
    /// region.
    #[cfg(feature = "lang-html")]
    #[test]
    fn the_first_of_several_hand_offs_in_one_list_takes_the_region() {
        let root = grammar(indoc! {r#"
            {
              "scopeName": "source.a",
              "patterns": [{ "include": "source.css" }, { "include": "text.html.basic" }]
            }
        "#});
        let tables = link(&root, &[], Some(&[Language::Css, Language::Html])).unwrap();
        let languages: Vec<Language> = tables.hand_offs.iter().map(|h| h.language).collect();
        assert_eq!(languages, [Language::Css]);
    }

    /// PHP's lexer reads the HTML around PHP's code, as a Markdown `php` fence
    /// includes both grammars.
    #[cfg(feature = "lang-php")]
    #[test]
    fn the_language_that_reads_the_others_takes_the_region() {
        let root = grammar(indoc! {r#"
            {
              "scopeName": "source.a",
              "patterns": [{ "include": "text.html.basic" }, { "include": "source.php" }]
            }
        "#});
        let tables = link(&root, &[], None).unwrap();
        let languages: Vec<Language> = tables.hand_offs.iter().map(|h| h.language).collect();
        assert_eq!(languages, [Language::Php]);
    }

    /// `\G` in a halt matches only where the embedding rule's begin ended.
    /// Hand-lexed tokens never move that place, so the starts found before
    /// the language runs are the ones Shiki finds.
    #[test]
    fn a_halt_with_last_match_end_hands_off() {
        let root = grammar(indoc! {r#"
            {
              "scopeName": "source.a",
              "patterns": [
                { "begin": "<", "end": "\\G>", "patterns": [{ "include": "source.css" }] }
              ]
            }
        "#});
        for handed in [None, Some([Language::Css].as_slice())] {
            let tables = link(&root, &[], handed).unwrap();
            let languages: Vec<Language> = tables.hand_offs.iter().map(|h| h.language).collect();
            assert_eq!(languages, [Language::Css]);
        }
    }

    /// A region with an end, which Rust's lexer cannot halt for.
    #[cfg(feature = "lang-rust")]
    fn ended_rust() -> super::UnlinkedGrammar {
        grammar(indoc! {r#"
            {
              "scopeName": "source.a",
              "patterns": [
                { "begin": "<", "end": ">", "patterns": [{ "include": "source.rust" }] }
              ]
            }
        "#})
    }

    /// The include of Rust matches nothing.
    #[cfg(feature = "lang-rust")]
    #[test]
    fn a_lexer_that_cannot_halt_takes_no_region_with_an_end() {
        let tables = link(&ended_rust(), &[], None).unwrap();
        assert!(tables.hand_offs.is_empty());
    }

    #[cfg(feature = "lang-rust")]
    #[test]
    fn a_named_lexer_that_cannot_halt_refuses_a_region_with_an_end() {
        assert_eq!(
            link(&ended_rust(), &[], Some(&[Language::Rust])).err(),
            Some(crate::textmate::compile::link::LinkError::NoHalt(
                Language::Rust
            ))
        );
    }
}

/// Every language nishiki lexes takes a hand-off.
/// A grammar that includes the language's grammar scope, without that grammar
/// in the set, hands the region to the language's lexer.
/// The region is a `while` rule's, so it needs no halt.
#[test]
fn every_language_takes_the_hand_off_of_its_grammar_scope() {
    for &language in crate::language::Language::ALL {
        let scope = language.grammar_scope();
        let root = grammar(&formatdoc! {r#"
            {{
              "scopeName": "source.a",
              "patterns": [
                {{ "begin": "<", "while": "^>", "patterns": [{{ "include": "{scope}" }}] }}
              ]
            }}
        "#});
        let tables = link(&root, &[], None).unwrap();
        let handed: Vec<_> = tables
            .hand_offs
            .iter()
            .map(|hand_off| hand_off.language)
            .collect();
        assert_eq!(handed, [language], "{}", language.id());
    }
}

/// The linker against `oracle/engine-tables.ts`, table for table.
///
/// Write the tables and their sources, then point `NISHIKI_LINK_SURVEY` at
/// the directories, joined as `PATH` joins them:
///
/// ```sh
/// bun oracle/engine-tables.ts --every <dir>
/// bun oracle/engine-tables.ts <other dir> 'haml>ruby>javascript>css>markdown>sass>coffee'
/// NISHIKI_LINK_SURVEY=<dir>:<other dir> \
///   cargo test --all-features link_survey -- --ignored --nocapture
/// ```
///
/// The files are named for a spec's first language.
/// A spec with hand-offs therefore goes in a directory of its own.
mod survey {
    use std::fmt::Write as _;
    use std::path::Path;
    use std::time::{Duration, Instant};

    use serde_json::{Value, json};

    use crate::language::Language;
    use crate::textmate::compile::link::link;
    use crate::textmate::compile::{UnlinkedGrammar, compile};
    use crate::textmate::grammar::{Capture, Grammar, Matching, NONE};

    /// An index as the JSON tables write it: `-1` for none.
    fn index(n: usize) -> Value {
        if n == NONE { json!(-1) } else { json!(n) }
    }

    fn caps(caps: &[Capture]) -> Value {
        caps.iter()
            .map(|c| {
                json!({
                    "index": c.index,
                    "name": c.name,
                    "contentName": c.content_name,
                    "retokenize": index(c.retokenize),
                })
            })
            .collect()
    }

    fn matching(matching: Matching) -> &'static str {
        match matching {
            Matching::Match => "Match",
            Matching::BeginEnd => "BeginEnd",
            Matching::BeginWhile => "BeginWhile",
            Matching::Container => "Container",
        }
    }

    /// The tables in the shape `engine-tables.ts` writes.
    fn tables(grammar: &Grammar) -> Value {
        json!({
            "scope": grammar.scope,
            "root": grammar.root,
            "names": grammar.names,
            "patterns": grammar
                .patterns
                .iter()
                .map(|p| json!({"source": p.source, "backrefs": p.backrefs}))
                .collect::<Vec<_>>(),
            "lists": grammar.lists.iter().map(|l| l.rules).collect::<Vec<_>>(),
            "injections": grammar
                .injections
                .iter()
                .map(|i| json!({"selector": i.selector, "rule": i.rule}))
                .collect::<Vec<_>>(),
            "rules": grammar.rules.iter().map(|r| json!({
                "matching": matching(r.matching),
                "name": r.name,
                "contentName": r.content_name,
                "begin": index(r.begin),
                "end": index(r.end),
                "until": index(r.until),
                "captures": caps(r.captures),
                "endCaptures": caps(r.end_captures),
                "list": r.list,
                "applyEndLast": r.apply_end_last,
            })).collect::<Vec<_>>(),
            "handOffs": grammar
                .hand_offs
                .iter()
                .map(|h| json!({
                    "language": format!("{:?}", h.language),
                    "before": h.before,
                    "after": h.after,
                }))
                .collect::<Vec<_>>(),
            "listHandOffs": grammar.list_hand_offs,
            "handingCaptures": grammar.handing_captures,
            "dfaCache": grammar.dfa_cache,
        })
    }

    /// The path of the first place where `a` and `b` differ.
    fn first_difference(a: &Value, b: &Value, path: &str) -> Option<String> {
        match (a, b) {
            (Value::Object(x), Value::Object(y)) => {
                let keys: std::collections::BTreeSet<&String> = x.keys().chain(y.keys()).collect();
                keys.into_iter().find_map(|key| {
                    first_difference(
                        x.get(key).unwrap_or(&Value::Null),
                        y.get(key).unwrap_or(&Value::Null),
                        &format!("{path}.{key}"),
                    )
                })
            }
            (Value::Array(x), Value::Array(y)) => {
                if x.len() != y.len() {
                    return Some(format!("{path}: {} entries, not {}", x.len(), y.len()));
                }
                x.iter()
                    .zip(y)
                    .enumerate()
                    .find_map(|(i, (x, y))| first_difference(x, y, &format!("{path}[{i}]")))
            }
            _ if a == b => None,
            _ => Some(format!("{path}: {a} against {b}")),
        }
    }

    /// Links the grammars of `sources` and compares the tables with the JSON
    /// beside it. Returns the time compiling and linking took, and the
    /// difference if any.
    fn check(dir: &Path, base: &str) -> (Duration, Option<String>) {
        let sources = std::fs::read_to_string(dir.join(format!("{base}.sources.jsonl"))).unwrap();
        let mut lines = sources.lines();
        let header: Value = serde_json::from_str(lines.next().unwrap()).unwrap();
        let started = Instant::now();
        let grammars: Vec<UnlinkedGrammar> = lines.map(|line| compile(line).unwrap()).collect();
        let root_scope = header["root"].as_str().unwrap();
        let root = grammars.iter().find(|g| g.scope == root_scope).unwrap();
        let set: Vec<&UnlinkedGrammar> = grammars.iter().collect();
        let handed: Vec<Language> = header["handed"]
            .as_array()
            .unwrap()
            .iter()
            .map(|id| Language::from_name(id.as_str().unwrap()).unwrap())
            .collect();
        let linked = link(root, &set, Some(&handed));
        let took = started.elapsed();
        let expected = std::fs::read_to_string(dir.join(format!("{base}.json")));
        let refused = std::fs::read_to_string(dir.join(format!("{base}.error")));
        let difference = match (linked, expected, refused) {
            (Ok(linked), Ok(expected), _) => {
                let expected: Value = serde_json::from_str(&expected).unwrap();
                first_difference(&tables(linked), &expected, "")
            }
            (Err(_), Err(_), Ok(_)) => None,
            (Ok(_), Err(_), Ok(refused)) => Some(format!("engine-tables.ts refused it: {refused}")),
            (Err(error), Ok(_), _) => Some(format!("the linker refused it: {error}")),
            (Ok(_) | Err(_), Err(_), Err(_)) => Some("no tables and no error".to_owned()),
        };
        (took, difference)
    }

    #[test]
    #[ignore = "a survey over scratch directories"]
    fn link_survey() {
        let Some(dirs) = std::env::var_os("NISHIKI_LINK_SURVEY") else {
            return;
        };
        let mut specs: Vec<(std::path::PathBuf, String)> = Vec::new();
        for dir in std::env::split_paths(&dirs) {
            for entry in std::fs::read_dir(&dir).unwrap() {
                let name = entry.unwrap().file_name().into_string().unwrap();
                if let Some(base) = name.strip_suffix(".sources.jsonl") {
                    specs.push((dir.clone(), base.to_owned()));
                }
            }
        }
        specs.sort();
        let mut report = String::new();
        let mut total = Duration::ZERO;
        let mut slowest = (Duration::ZERO, String::new());
        for (dir, base) in &specs {
            let (took, difference) = check(dir, base);
            total += took;
            if took > slowest.0 {
                slowest = (took, base.clone());
            }
            if let Some(difference) = difference {
                writeln!(report, "  {}/{base}: {difference}", dir.display()).unwrap();
            }
        }
        eprintln!(
            "{} specs, compiled and linked in {total:?}; slowest {} in {:?}",
            specs.len(),
            slowest.1,
            slowest.0
        );
        assert!(report.is_empty(), "tables differ:\n{report}");
    }

    /// Links every grammar of the survey directories with no language
    /// named, so each hand lexer built takes the includes of its grammar.
    /// Each grammar links twice: with the grammars Shiki loads for it, and
    /// alone.
    /// `bun oracle/engine-tables.ts --every <dir>` writes a directory.
    /// Every grammar must link both ways.
    #[test]
    #[ignore = "a survey over scratch directories"]
    fn every_grammar_links_with_every_hand_lexer() {
        let Some(dirs) = std::env::var_os("NISHIKI_LINK_SURVEY") else {
            return;
        };
        let mut failures = Vec::new();
        let mut linked = [0usize; 2];
        let mut handing = [0usize; 2];
        for dir in std::env::split_paths(&dirs) {
            let mut bases: Vec<String> = std::fs::read_dir(&dir)
                .unwrap()
                .map(|entry| entry.unwrap().file_name().into_string().unwrap())
                .filter_map(|name| name.strip_suffix(".sources.jsonl").map(str::to_owned))
                .collect();
            bases.sort();
            for base in bases {
                let sources =
                    std::fs::read_to_string(dir.join(format!("{base}.sources.jsonl"))).unwrap();
                let mut lines = sources.lines();
                let header: Value = serde_json::from_str(lines.next().unwrap()).unwrap();
                let grammars: Vec<UnlinkedGrammar> =
                    lines.map(|line| compile(line).unwrap()).collect();
                let root_scope = header["root"].as_str().unwrap();
                let root = grammars.iter().find(|g| g.scope == root_scope).unwrap();
                let set: Vec<&UnlinkedGrammar> = grammars.iter().collect();
                for (way, (name, set)) in [("full", &set[..]), ("alone", &[][..])]
                    .into_iter()
                    .enumerate()
                {
                    match link(root, set, None) {
                        Ok(tables) => {
                            linked[way] += 1;
                            handing[way] += usize::from(!tables.hand_offs.is_empty());
                        }
                        Err(error) => failures.push(format!("{base} {name}: {error}")),
                    }
                }
            }
        }
        eprintln!(
            "with their grammars: {} linked, {} hand off; alone: {} linked, {} hand off",
            linked[0], handing[0], linked[1], handing[1]
        );
        assert!(
            failures.is_empty(),
            "grammars that did not link: {failures:#?}"
        );
    }
}
