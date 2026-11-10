//! The compiler and the linker on small grammars, and the survey that holds
//! the linker to `oracle/engine-tables.ts` on every grammar Shiki bundles.

use indoc::indoc;

use super::link::{LinkError, link};
use super::{Compiled, compile};
use crate::textmate::grammar::{Grammar, NONE, RuleKind};

/// The grammar of `json`, which must compile.
fn grammar(json: &str) -> Compiled {
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
    let linked = link(&root, &[], None).unwrap();
    let tables = linked.grammar;
    assert_eq!(list_patterns(tables, tables.root), ["a", "b"]);
    assert_eq!(linked.reached, Vec::<usize>::new());
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
    let tables = link(&root, &[], None).unwrap().grammar;
    assert_eq!(list_patterns(tables, tables.root), ["a"]);
}

#[test]
fn nested_repositories_shadow_outer_ones() {
    let root = grammar(indoc! {r##"
        {
          "scopeName": "source.a",
          "patterns": [{"include": "#word"}, {"patterns": [{"include": "#word"}], "repository": {"word": {"match": "inner"}}}],
          "repository": {"word": {"match": "outer"}}
        }
    "##});
    let tables = link(&root, &[], None).unwrap().grammar;
    assert_eq!(list_patterns(tables, tables.root), ["outer", "inner"]);
}

#[test]
fn grammars_that_include_each_other_link_in_either_order() {
    let a = grammar(indoc! {r#"
        {"scopeName": "source.a", "patterns": [{"match": "a"}, {"include": "source.b"}]}
    "#});
    let b = grammar(indoc! {r#"
        {"scopeName": "source.b", "patterns": [{"match": "b"}, {"include": "source.a"}, {"include": "$base"}]}
    "#});
    for set in [[&a, &b], [&b, &a]] {
        let linked = link(&a, &set, None).unwrap();
        let tables = linked.grammar;
        assert_eq!(list_patterns(tables, tables.root), ["a", "b"]);
        assert_eq!(linked.reached.len(), 1);
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
    let tables = link(&root, &[], None).unwrap().grammar;
    assert_eq!(list_patterns(tables, tables.root), ["p"]);
}

#[test]
fn captures_take_the_order_of_their_numbers() {
    let root = grammar(indoc! {r#"
        {
          "scopeName": "source.a",
          "patterns": [{"match": "(a)(b)", "captures": {"2": {"name": "two"}, "1": {"name": "one", "patterns": [{"match": "a"}]}, "x": {"name": "no"}}}]
        }
    "#});
    let tables = link(&root, &[], None).unwrap().grammar;
    let rule = &tables.rules[tables.lists[tables.rules[tables.root].list].rules[0]];
    let indices: Vec<usize> = rule.captures.iter().map(|c| c.index).collect();
    assert_eq!(indices, [1, 2]);
    assert_ne!(rule.captures[0].retokenize, NONE);
    assert_eq!(rule.captures[1].retokenize, NONE);
    assert_eq!(
        tables.rules[rule.captures[0].retokenize].kind,
        RuleKind::Container
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
    let tables = link(&root, &[], None).unwrap().grammar;
    let selectors: Vec<&str> = tables.injections.iter().map(|i| i.selector).collect();
    assert_eq!(selectors, ["R:z", "L:a"]);
}

#[test]
fn a_grammar_injects_itself_where_it_names_the_root() {
    let root = grammar(r#"{"scopeName": "source.a", "patterns": []}"#);
    let injected = grammar(indoc! {r#"
        {"scopeName": "source.i", "injectionSelector": "L:source.a", "injectTo": ["source.a"], "patterns": [{"match": "i"}]}
    "#});
    let linked = link(&root, &[&injected], None).unwrap();
    assert_eq!(linked.grammar.injections.len(), 1);
    assert_eq!(linked.reached, [0]);
}

#[cfg(feature = "lang-css")]
mod hand_offs {
    use indoc::indoc;

    use super::{grammar, list_patterns};
    use crate::language::Language;
    use crate::textmate::compile::link::{LinkError, link};

    /// A grammar whose `style` region includes CSS whole, and one CSS rule.
    fn styled() -> super::Compiled {
        grammar(indoc! {r#"
            {
              "scopeName": "source.a",
              "patterns": [
                {"begin": "<style>", "end": "</style>", "patterns": [{"match": "!"}, {"include": "source.css"}, {"match": "\\?"}]},
                {"include": "source.css#escapes"}
              ]
            }
        "#})
    }

    fn css() -> super::Compiled {
        grammar(indoc! {r#"
            {"scopeName": "source.css", "patterns": [{"match": "css"}], "repository": {"escapes": {"match": "\\\\."}}}
        "#})
    }

    #[test]
    fn a_hand_lexed_grammar_missing_from_the_set_is_handed_off() {
        let linked = link(&styled(), &[], None).unwrap();
        let tables = linked.grammar;
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
        let linked = link(&styled(), &[&css], None).unwrap();
        let tables = linked.grammar;
        assert!(tables.hand_offs.is_empty());
        assert_eq!(list_patterns(tables, tables.root), ["<style>", "\\\\."]);
        assert_eq!(linked.reached, [0]);
    }

    #[test]
    fn a_listed_hand_lexer_wins_and_its_grammar_serves_fragments() {
        let css = css();
        let linked = link(&styled(), &[&css], Some(&[Language::Css])).unwrap();
        let tables = linked.grammar;
        assert_eq!(tables.hand_offs.len(), 1);
        assert_eq!(list_patterns(tables, tables.root), ["<style>", "\\\\."]);
        assert_eq!(linked.reached, [0]);
    }

    #[test]
    fn two_hand_offs_in_one_list_are_refused() {
        let root = grammar(indoc! {r#"
            {"scopeName": "source.a", "patterns": [{"include": "source.css"}, {"include": "text.html.basic"}]}
        "#});
        let languages = [Language::Css, Language::Html];
        #[cfg(feature = "lang-html")]
        assert_eq!(
            link(&root, &[], Some(&languages)).unwrap_err(),
            LinkError::TwoHandOffs
        );
        #[cfg(not(feature = "lang-html"))]
        assert!(link(&root, &[], Some(&languages)).is_err());
    }

    #[test]
    fn a_halt_with_last_match_end_is_refused() {
        let root = grammar(indoc! {r#"
            {"scopeName": "source.a", "patterns": [{"begin": "<", "end": "\\G>", "patterns": [{"include": "source.css"}]}]}
        "#});
        assert!(matches!(
            link(&root, &[], None),
            Err(LinkError::AnchoredHalt(_))
        ));
    }
}

#[cfg(feature = "lang-rust")]
#[test]
fn a_language_no_grammar_hands_off_to_is_refused() {
    let root = grammar(r#"{"scopeName": "source.a", "patterns": []}"#);
    let error = link(&root, &[], Some(&[crate::language::Language::Rust]));
    assert!(matches!(error, Err(LinkError::NotHandLexed(_))));
}

/// The linker against `oracle/engine-tables.ts`, table for table.
///
/// Write the tables and their sources, then point `NISHIKI_LINK_SURVEY` at
/// the directories, joined as `PATH` joins them:
///
/// ```sh
/// bun oracle/engine-tables.ts --json --every <dir>
/// bun oracle/engine-tables.ts --json <other dir> 'haml>ruby>javascript>css>markdown>sass>coffee'
/// NISHIKI_LINK_SURVEY=<dir>:<other dir> cargo test --all-features link_survey -- --ignored --nocapture
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
    use crate::textmate::compile::link::{Linked, link};
    use crate::textmate::compile::{Compiled, compile};
    use crate::textmate::grammar::{Cap, Grammar, NONE, RuleKind};

    /// An index as the JSON tables write it: `-1` for none.
    fn index(n: usize) -> Value {
        if n == NONE { json!(-1) } else { json!(n) }
    }

    fn caps(caps: &[Cap]) -> Value {
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

    fn kind(kind: RuleKind) -> &'static str {
        match kind {
            RuleKind::Match => "Match",
            RuleKind::BeginEnd => "BeginEnd",
            RuleKind::BeginWhile => "BeginWhile",
            RuleKind::Container => "Container",
        }
    }

    /// The tables in the shape `engine-tables.ts --json` writes.
    fn tables(grammar: &Grammar) -> Value {
        json!({
            "scope": grammar.scope,
            "root": grammar.root,
            "names": grammar.names,
            "patterns": grammar.patterns.iter().map(|p| json!({"source": p.source, "backrefs": p.backrefs})).collect::<Vec<_>>(),
            "lists": grammar.lists.iter().map(|l| l.rules).collect::<Vec<_>>(),
            "injections": grammar.injections.iter().map(|i| json!({"selector": i.selector, "rule": i.rule})).collect::<Vec<_>>(),
            "rules": grammar.rules.iter().map(|r| json!({
                "kind": kind(r.kind),
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
            "handOffs": grammar.hand_offs.iter().map(|h| json!({"language": format!("{:?}", h.language), "before": h.before, "after": h.after})).collect::<Vec<_>>(),
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
        let grammars: Vec<Compiled> = lines.map(|line| compile(line).unwrap()).collect();
        let root_scope = header["root"].as_str().unwrap();
        let root = grammars.iter().find(|g| g.scope == root_scope).unwrap();
        let set: Vec<&Compiled> = grammars.iter().collect();
        let handed: Vec<Language> = header["handed"]
            .as_array()
            .unwrap()
            .iter()
            .map(|id| Language::from_name(id.as_str().unwrap()).unwrap())
            .collect();
        let linked: Result<Linked, _> = link(root, &set, Some(&handed));
        let took = started.elapsed();
        let expected = std::fs::read_to_string(dir.join(format!("{base}.json")));
        let refused = std::fs::read_to_string(dir.join(format!("{base}.error")));
        let difference = match (linked, expected, refused) {
            (Ok(linked), Ok(expected), _) => {
                let expected: Value = serde_json::from_str(&expected).unwrap();
                first_difference(&tables(linked.grammar), &expected, "")
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
}
