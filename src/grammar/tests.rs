use std::sync::{Arc, PoisonError, mpsc};
use std::thread;
use std::time::Duration;

use indoc::indoc;

use super::{
    Grammar, Grammars, LINKED_TABLES, LinkKey, Reason, bytes, linked_table, static_grammar,
};
use crate::textmate::compile::link::LinkError;
use crate::textmate::compile::{UnlinkedGrammar, UnlinkedRule, compile};
use crate::{Highlighter, State};

/// A grammar with a keyword from its repository and a string with escapes.
const DEMO: &str = indoc! {r##"
    {
      "scopeName": "source.demo",
      "patterns": [
        { "include": "#keywords" },
        {
          "begin": "\"",
          "end": "\"",
          "name": "string.quoted.double.demo",
          "patterns": [{ "match": "\\\\.", "name": "constant.character.escape.demo" }]
        }
      ],
      "repository": {
        "keywords": { "match": "\\b(let|fn)\\b", "name": "keyword.demo" }
      }
    }
"##};

/// Each token of `line`: its text and its scopes, innermost first.
fn tokens(highlighter: &mut Highlighter<Grammar>, line: &str) -> Vec<(String, Vec<String>)> {
    document_tokens(highlighter, &[line]).swap_remove(0)
}

/// The tokens of each of `lines`, lexed in order from the document's start,
/// as [`tokens`] gives them.
fn document_tokens(
    highlighter: &mut Highlighter<Grammar>,
    lines: &[&str],
) -> Vec<Vec<(String, Vec<String>)>> {
    let mut state = State::default();
    lines
        .iter()
        .map(|line| {
            let mut out = Vec::new();
            let _ = highlighter.line(line, &mut state, &mut out);
            let mut start = 0;
            out.iter()
                .map(|token| {
                    let text = line[start..token.end()].to_owned();
                    start = token.end();
                    let scopes = highlighter
                        .scopes(token.stack())
                        .map(str::to_owned)
                        .collect();
                    (text, scopes)
                })
                .collect()
        })
        .collect()
}

fn highlighter(grammar: Grammar) -> Highlighter<Grammar> {
    Highlighter::for_grammar(grammar, &Grammars::new()).unwrap()
}

fn error(bytes: &[u8]) -> Reason {
    Grammar::from_bytes(bytes).unwrap_err().0
}

/// A grammar whose rules are `rules`, the first of them its one pattern.
fn grammar_of(rules: Vec<UnlinkedRule>) -> UnlinkedGrammar {
    UnlinkedGrammar {
        scope: "source.crafted".into(),
        rules: rules.into(),
        patterns: vec![0].into(),
        repository: Vec::new().into(),
        injections: Vec::new().into(),
        injection_selector: None,
        inject_to: Vec::new().into(),
    }
}

#[test]
fn loading_one_grammar_twice_gives_one_handle() {
    let first = Grammar::from_json(DEMO).unwrap();
    let copy = DEMO.to_owned();
    let second = Grammar::from_json(&copy).unwrap();
    assert_eq!(first, second);
    assert!(std::ptr::eq(first.0, second.0));
    assert_eq!(first.scope(), "source.demo");
}

#[test]
fn a_loaded_grammar_highlights_a_line() {
    let mut highlighter = highlighter(Grammar::from_json(DEMO).unwrap());
    let tokens = tokens(&mut highlighter, r#"let s = "a\n";"#);
    assert_eq!(
        tokens[0],
        (
            "let".into(),
            vec!["keyword.demo".into(), "source.demo".into()]
        )
    );
    assert!(tokens.contains(&(
        r"\n".into(),
        vec![
            "constant.character.escape.demo".into(),
            "string.quoted.double.demo".into(),
            "source.demo".into(),
        ]
    )));
}

#[test]
fn a_grammar_from_bytes_equals_the_grammar_from_json() {
    let from_json = Grammar::from_json(DEMO).unwrap();
    let bytes = from_json.to_bytes();
    let from_bytes = Grammar::from_bytes(&bytes).unwrap();
    assert_eq!(from_json.0, from_bytes.0);
    assert_eq!(Grammar::from_bytes(&bytes).unwrap(), from_bytes);
    let line = r#"fn f() { "x\"y" }"#;
    assert_eq!(
        tokens(&mut highlighter(from_json), line),
        tokens(&mut highlighter(from_bytes), line)
    );
}

#[test]
fn one_set_links_once() {
    let grammar = Grammar::from_json(DEMO).unwrap();
    let set: Grammars = [grammar].into_iter().collect();
    let first = Highlighter::for_grammar(grammar, &set).unwrap();
    let second = Highlighter::for_grammar(grammar, &set.clone()).unwrap();
    assert!(std::ptr::eq(first.lexer().table, second.lexer().table));
    assert_eq!(first.grammar(), grammar);
}

/// A grammar whose braces hold `source.inner`.
const OUTER: &str = indoc! {r#"
    {
      "scopeName": "source.outer",
      "patterns": [
        { "match": "\\bouter\\b", "name": "keyword.outer" },
        {
          "begin": "\\{",
          "end": "\\}",
          "name": "meta.block.outer",
          "patterns": [{ "include": "source.inner" }]
        }
      ]
    }
"#};

/// A grammar whose parentheses hold `source.outer`.
const INNER: &str = indoc! {r#"
    {
      "scopeName": "source.inner",
      "patterns": [
        { "match": "\\binner\\b", "name": "keyword.inner" },
        {
          "begin": "\\(",
          "end": "\\)",
          "name": "meta.group.inner",
          "patterns": [{ "include": "source.outer" }]
        }
      ]
    }
"#};

/// Task 2.3 of `openspec/changes/archive/2026-11-10-grammar-api`.
///
/// Each root links twice, once per insertion order, into two tables.
/// Both tables lex its line the same.
/// Each keyword gets its own grammar's scope, so each include resolved.
#[test]
fn two_grammars_that_include_each_other_link_in_either_order() {
    let outer = Grammar::from_json(OUTER).unwrap();
    let inner = Grammar::from_json(INNER).unwrap();
    let in_order: Grammars = [outer, inner].into_iter().collect();
    let reversed: Grammars = [inner, outer].into_iter().collect();
    for (root, line) in [
        (outer, "outer { inner ( outer { inner } ) }"),
        (inner, "inner ( outer { inner ( outer ) } )"),
    ] {
        let mut first = Highlighter::for_grammar(root, &in_order).unwrap();
        let mut second = Highlighter::for_grammar(root, &reversed).unwrap();
        assert!(!std::ptr::eq(first.lexer().table, second.lexer().table));
        let lexed = tokens(&mut first, line);
        assert_eq!(lexed, tokens(&mut second, line), "{line}");
        for word in ["outer", "inner"] {
            let keyword = format!("keyword.{word}");
            let found: Vec<&Vec<String>> = lexed
                .iter()
                .filter(|(text, _)| text == word)
                .map(|(_, scopes)| scopes)
                .collect();
            assert_eq!(found.len(), 2, "{line}: {lexed:?}");
            assert!(
                found.iter().all(|scopes| scopes[0] == keyword),
                "{line}: {lexed:?}"
            );
        }
    }
}

#[test]
fn links_of_one_set_on_many_threads_end_with_one_table() {
    let grammar =
        Grammar::from_json(r#"{ "scopeName": "source.racing", "patterns": [] }"#).unwrap();
    let set = Grammars::new();
    let tables: Vec<_> = thread::scope(|scope| {
        let links: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| linked_table(grammar, &set).unwrap()))
            .collect();
        links.into_iter().map(|link| link.join().unwrap()).collect()
    });
    assert!(tables.iter().all(|&table| std::ptr::eq(table, tables[0])));
}

/// A link in progress for one key, made by holding that key's lock, must
/// not hold up a link of another key.
#[test]
fn a_link_waits_only_for_links_of_its_own_key() {
    let busy = Grammar::from_json(r#"{ "scopeName": "source.busy", "patterns": [] }"#).unwrap();
    let other = Grammar::from_json(r#"{ "scopeName": "source.other", "patterns": [] }"#).unwrap();
    let key = LinkKey {
        root: busy,
        set: Vec::new(),
        handed: None,
    };
    let slot = Arc::clone(
        LINKED_TABLES
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .entry(key)
            .or_default(),
    );
    let held = slot.lock().unwrap();
    let (send, receive) = mpsc::channel();
    thread::scope(|scope| {
        scope.spawn(move || send.send(linked_table(other, &Grammars::new()).is_ok()));
        let linked = receive.recv_timeout(Duration::from_secs(30));
        drop(held);
        assert_eq!(linked, Ok(true));
    });
}

#[test]
fn a_set_keeps_one_grammar_per_scope() {
    let demo = Grammar::from_json(DEMO).unwrap();
    let other = Grammar::from_json(&DEMO.replace("\"keyword.demo\"", "\"storage.demo\"")).unwrap();
    let mut set = Grammars::new();
    assert_eq!(set.insert(demo), None);
    assert_eq!(set.insert(other), Some(demo));
    assert_eq!(set.get("source.demo"), Some(other));
    assert_eq!(set.get("source.other"), None);
}

/// A grammar whose region handed to Rust has an end pattern.
/// Rust's lexer cannot stop for it, so the linker refuses the grammar once
/// the caller names Rust with [`Grammars::hand_off`].
#[cfg(feature = "lang-rust")]
const FAILING: &str = r#"{
    "scopeName": "source.failing",
    "patterns": [{ "begin": "<", "end": ">", "patterns": [{ "include": "source.rust" }] }]
}"#;

#[cfg(feature = "lang-rust")]
#[test]
fn a_link_the_engine_refuses_fails() {
    let mut set = Grammars::new();
    set.hand_off(crate::Language::Rust);
    let failed = Highlighter::for_grammar(Grammar::from_json(FAILING).unwrap(), &set);
    assert!(matches!(failed.unwrap_err().0, Reason::Link(_)));
}

/// A link that fails leaves no entry in [`LINKED_TABLES`] for its key.
///
/// The grammar is [`FAILING`] under a scope name of its own.
/// `a_link_the_engine_refuses_fails` links `FAILING` on another thread.
/// With the same key, its entry could stand in the table when this test
/// reads it.
#[cfg(feature = "lang-rust")]
#[test]
fn a_failed_link_keeps_no_entry() {
    let grammar =
        Grammar::from_json(&FAILING.replace("\"source.failing\"", "\"source.failing.entry\""))
            .unwrap();
    let mut set = Grammars::new();
    set.hand_off(crate::Language::Rust);
    assert!(linked_table(grammar, &set).is_err());
    let tables = LINKED_TABLES.lock().unwrap();
    assert!(!tables.contains_key(&LinkKey::new(grammar, &set)));
}

/// A region handed to CSS, whose end `\G>` matches only where the begin
/// ended, as in Shiki.
///
/// - In `<s>b`, the end matches at once, and `b` is outside the region.
/// - In `<sa { color: red; }>`, the end fails at `a`, so CSS lexes the rest and
///   the region stays open past the last `>`.
#[cfg(feature = "lang-css")]
#[test]
fn an_end_at_the_anchor_matches_only_there() {
    let grammar = Grammar::from_json(indoc! {r#"
        {
          "scopeName": "source.anchored",
          "patterns": [
            {
              "begin": "<s",
              "end": "\\G>",
              "name": "meta.s",
              "patterns": [{ "include": "source.css" }]
            }
          ]
        }
    "#})
    .unwrap();
    let mut highlighter = highlighter(grammar);
    let closed = tokens(&mut highlighter, "<s>b");
    assert_eq!(
        closed.last(),
        Some(&("b".to_owned(), vec!["source.anchored".to_owned()]))
    );
    let open = tokens(&mut highlighter, "<sa { color: red; }>");
    let in_region = |scopes: &[String]| scopes.iter().any(|scope| scope == "meta.s");
    assert!(open.iter().all(|(_, scopes)| in_region(scopes)));
    assert!(open.iter().any(|(_, scopes)| {
        scopes
            .iter()
            .any(|scope| scope == "support.type.property-name.css")
    }));
}

/// The innermost scope of the first `$` in `line`, lexed by a loaded grammar
/// named `source.svelte` that hands its `<script>` to `script`.
/// The grammar holds the injection that adds Svelte's `$` prefixes, with
/// Svelte's selector.
#[cfg(feature = "lang-javascript")]
fn svelte_dollar(script: &str, line: &str) -> Option<String> {
    let json = indoc! {r#"
        {
          "scopeName": "source.svelte",
          "patterns": [
            {
              "begin": "<script>",
              "end": "</script>",
              "contentName": "SCRIPT",
              "patterns": [{ "include": "SCRIPT" }]
            }
          ],
          "injections": {
            "L:(source.ts, source.js, source.coffee)": {
              "patterns": [
                {
                  "match": "\\$(?=[_[:alpha:]])",
                  "name": "punctuation.definition.variable.svelte"
                }
              ]
            }
          }
        }
    "#}
    .replace("SCRIPT", script);
    let mut highlighter = highlighter(Grammar::from_json(&json).unwrap());
    tokens(&mut highlighter, line)
        .into_iter()
        .find(|(text, _)| text == "$")
        .and_then(|(_, scopes)| scopes.into_iter().next())
}

/// JavaScript that a loaded Svelte grammar hands off takes Svelte's `$`
/// prefixes, with or without nishiki's own Svelte lexer.
/// The handed region's rules never run, so the JavaScript lexer applies them.
#[cfg(feature = "lang-javascript")]
#[test]
fn javascript_in_a_loaded_svelte_grammar_takes_the_dollar_prefixes() {
    assert_eq!(
        svelte_dollar("source.js", "<script>let a = $count;</script>").as_deref(),
        Some("punctuation.definition.variable.svelte")
    );
}

/// CoffeeScript that a loaded Svelte grammar hands off takes Svelte's `$`
/// prefixes too, as Svelte's selector names `source.coffee`.
#[cfg(feature = "lang-coffee")]
#[test]
fn coffeescript_in_a_loaded_svelte_grammar_takes_the_dollar_prefixes() {
    assert_eq!(
        svelte_dollar("source.coffee", "<script>a = $count</script>").as_deref(),
        Some("punctuation.definition.variable.svelte")
    );
}

/// Make in a region whose begin took the line feed, with Shiki's scopes.
///
/// The region has no end, because Make's lexer cannot halt.
/// A begin with no end never closes, in Shiki as here.
///
/// - The begin `^<<$\n?` leaves `\G` at the start of each later line while Make
///   has only its root open. So the tab before `FOO = bar` starts an
///   assignment, by `(^ *|\G\s*)`.
/// - Inside `ifdef X`, the conditional is the innermost rule. Its begin stopped
///   before the line feed, so `\G` does not match there, and the tab starts a
///   recipe.
/// - The begin `^<<` stops before the line feed, so it leaves no `\G` on the
///   next line.
#[cfg(feature = "lang-make")]
#[test]
fn make_reads_the_anchor_of_a_begin_that_took_the_line_feed() {
    let lines = ["<<", "\tFOO = bar", "ifdef X", "\tBAR = baz", "endif"];
    let lexed = |begin: &str| {
        let json = indoc! {r#"
            {
              "scopeName": "source.wrap",
              "patterns": [
                {
                  "begin": "BEGIN",
                  "name": "meta.wrap",
                  "patterns": [{ "include": "source.makefile" }]
                }
              ]
            }
        "#}
        .replace("BEGIN", begin);
        document_tokens(&mut highlighter(Grammar::from_json(&json).unwrap()), &lines)
    };
    let innermost = |rows: &[Vec<(String, Vec<String>)>], line: usize, word: &str| {
        rows[line]
            .iter()
            .find(|(text, _)| text.contains(word))
            .and_then(|(_, scopes)| scopes.first().cloned())
    };
    let took_line_feed = lexed(r"^<<$\\n?");
    assert_eq!(
        innermost(&took_line_feed, 1, "FOO").as_deref(),
        Some("variable.other.makefile")
    );
    assert_eq!(
        innermost(&took_line_feed, 3, "BAR").as_deref(),
        Some("meta.scope.recipe.makefile")
    );
    let stopped_before = lexed("^<<");
    assert_eq!(
        innermost(&stopped_before, 1, "FOO").as_deref(),
        Some("meta.scope.recipe.makefile")
    );
}

/// Less pops the rule that embeds it at a `)` at its root.
/// The rest of the line takes the scope below, as in Shiki.
#[cfg(feature = "lang-less")]
#[test]
fn less_pops_the_rule_that_embeds_it() {
    let grammar = Grammar::from_json(indoc! {r#"
        {
          "scopeName": "source.styled",
          "patterns": [
            {
              "begin": "<",
              "end": ">",
              "name": "meta.wrap",
              "patterns": [{ "include": "source.css.less" }]
            }
          ]
        }
    "#})
    .unwrap();
    let scopes = |names: &[&str]| names.iter().map(|&name| name.to_owned()).collect();
    assert_eq!(
        tokens(&mut highlighter(grammar), "<) a> b"),
        vec![
            ("<".to_owned(), scopes(&["meta.wrap", "source.styled"])),
            (") a> b".to_owned(), scopes(&["source.styled"])),
        ]
    );
}

/// When the grammar's root embeds Less, a pop leaves the root open.
/// The line ends there, and Less starts again on the next line, where `a`
/// opens a selector.
#[cfg(feature = "lang-less")]
#[test]
fn less_popping_at_the_root_keeps_the_root() {
    let grammar = Grammar::from_json(indoc! {r#"
        {
          "scopeName": "source.styled",
          "patterns": [{ "include": "source.css.less" }]
        }
    "#})
    .unwrap();
    let mut highlighter = highlighter(grammar);
    let mut state = State::default();
    let mut out = Vec::new();
    let _ = highlighter.line(") a", &mut state, &mut out);
    let popped: Vec<(usize, Vec<String>)> = out
        .iter()
        .map(|token| {
            let scopes = highlighter
                .scopes(token.stack())
                .map(str::to_owned)
                .collect();
            (token.end(), scopes)
        })
        .collect();
    assert_eq!(popped, vec![(3, vec!["source.styled".to_owned()])]);
    out.clear();
    let _ = highlighter.line("a { b: c; }", &mut state, &mut out);
    assert!(out.iter().any(|token| {
        highlighter
            .scopes(token.stack())
            .any(|scope| scope == "meta.selector.less")
    }));
}

#[test]
fn json_without_a_scope_fails() {
    let failed = Grammar::from_json(r#"{ "patterns": [] }"#).unwrap_err();
    assert!(matches!(failed.0, Reason::Compile(_)));
    assert!(Grammar::from_json("not json").is_err());
}

#[test]
fn bytes_of_another_version_fail() {
    let mut bytes = Grammar::from_json(DEMO).unwrap().to_bytes();
    bytes[8] = 2;
    assert!(matches!(
        error(&bytes),
        Reason::Version {
            found: 2,
            expected: 1
        }
    ));
}

#[test]
fn bytes_without_the_magic_fail() {
    assert!(matches!(error(b"not a grammar"), Reason::NotGrammarBytes));
    assert!(matches!(error(b""), Reason::NotGrammarBytes));
}

#[test]
fn every_truncation_fails() {
    let bytes = Grammar::from_json(DEMO).unwrap().to_bytes();
    for end in 0..bytes.len() {
        assert!(bytes::read(&bytes[..end]).is_err(), "cut at {end}");
    }
    let mut longer = bytes.clone();
    longer.push(0);
    assert!(bytes::read(&longer).is_err());
}

#[test]
fn a_rule_index_out_of_range_fails() {
    let mut grammar = compile(DEMO).unwrap();
    let count = grammar.rules.len();
    grammar.patterns.to_mut().push(count);
    assert!(matches!(
        bytes::read(&bytes::write(&grammar)),
        Err(Reason::Malformed { .. })
    ));
}

#[test]
fn a_rule_named_twice_fails() {
    let mut grammar = compile(DEMO).unwrap();
    let first = grammar.patterns[0];
    grammar.patterns.to_mut().push(first);
    assert!(bytes::read(&bytes::write(&grammar)).is_err());
}

#[test]
fn a_repository_out_of_order_fails() {
    let mut grammar = compile(DEMO).unwrap();
    let count = grammar.rules.len();
    grammar.repository.to_mut().push(("a".into(), count));
    grammar.rules.to_mut().push(UnlinkedRule::default());
    assert!(matches!(
        bytes::read(&bytes::write(&grammar)),
        Err(Reason::Malformed { .. })
    ));
    grammar
        .repository
        .to_mut()
        .sort_unstable_by(|(a, _), (b, _)| a.cmp(b));
    assert!(bytes::read(&bytes::write(&grammar)).is_ok());
}

#[test]
fn a_rule_that_names_itself_fails() {
    let grammar = grammar_of(vec![UnlinkedRule {
        patterns: vec![0].into(),
        ..UnlinkedRule::default()
    }]);
    assert!(bytes::read(&bytes::write(&grammar)).is_err());
}

#[test]
fn rules_nested_past_the_depth_limit_fail() {
    let chain = |levels: usize| {
        grammar_of(
            (0..levels)
                .map(|id| UnlinkedRule {
                    patterns: if id + 1 < levels {
                        vec![id + 1].into()
                    } else {
                        Vec::new().into()
                    },
                    ..UnlinkedRule::default()
                })
                .collect(),
        )
    };
    assert!(bytes::read(&bytes::write(&chain(128))).is_ok());
    assert!(bytes::read(&bytes::write(&chain(129))).is_err());
}

#[test]
fn a_count_larger_than_the_bytes_fails_before_it_allocates() {
    let mut bytes = Grammar::from_json(DEMO).unwrap().to_bytes();
    let scope_length = u32::from_le_bytes(bytes[16..20].try_into().unwrap());
    let rule_count_at = 20 + usize::try_from(scope_length).unwrap();
    bytes[rule_count_at..rule_count_at + 4].copy_from_slice(&u32::MAX.to_le_bytes());
    assert!(matches!(error(&bytes), Reason::Malformed { at } if at == rule_count_at));
}

/// Every grammar in `tests/engine/grammars/` writes a Rust file that `syn`
/// reads, so [`Grammar::to_rust`] lays each one out with prettyplease.
#[cfg(feature = "format")]
#[test]
fn every_engine_grammar_writes_rust_that_parses() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/engine/grammars");
    let mut grammars = 0;
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_none_or(|extension| extension != "json")
            || path.file_name().is_some_and(|name| name == "tables.json")
        {
            continue;
        }
        let grammar = Grammar::from_json(&std::fs::read_to_string(&path).unwrap()).unwrap();
        let file = super::tokens::static_file(grammar);
        assert!(syn::parse2::<syn::File>(file).is_ok(), "{}", path.display());
        grammars += 1;
    }
    assert_ne!(grammars, 0);
}

/// A model the compiler did not write, handed to `static_grammar` unchecked,
/// links to a rule that matches nothing or fails to link, and never panics.
#[test]
fn an_unchecked_static_grammar_links_without_a_panic() {
    let past_the_end = static_grammar(Box::leak(Box::new(grammar_of(vec![UnlinkedRule {
        patterns: vec![7].into(),
        ..UnlinkedRule::default()
    }]))));
    let mut highlighter = Highlighter::for_grammar(past_the_end, &Grammars::new()).unwrap();
    assert_eq!(
        tokens(&mut highlighter, "let"),
        [("let".to_owned(), vec!["source.crafted".to_owned()])]
    );
    let names_itself = static_grammar(Box::leak(Box::new(grammar_of(vec![UnlinkedRule {
        patterns: vec![0].into(),
        ..UnlinkedRule::default()
    }]))));
    let failed = Highlighter::for_grammar(names_itself, &Grammars::new());
    assert!(matches!(
        failed.unwrap_err().0,
        Reason::Link(LinkError::TooDeep)
    ));
}
