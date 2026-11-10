use indoc::indoc;

use super::{Grammar, Grammars, Reason, bytes};
use crate::textmate::compile::{Compiled, RawRule, compile};
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
    let mut out = Vec::new();
    let _ = highlighter.line(line, &mut State::default(), &mut out);
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
}

fn highlighter(grammar: Grammar) -> Highlighter<Grammar> {
    Highlighter::for_grammar(grammar, &Grammars::new()).unwrap()
}

fn error(bytes: &[u8]) -> Reason {
    Grammar::from_bytes(bytes).unwrap_err().0
}

/// A grammar whose rules are `rules`, the first of them its one pattern.
fn grammar_of(rules: Vec<RawRule>) -> Compiled {
    Compiled {
        scope: "source.crafted".to_owned(),
        rules,
        patterns: vec![0],
        repository: Vec::new(),
        injections: Vec::new(),
        injection_selector: None,
        inject_to: Vec::new(),
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

#[cfg(feature = "lang-rust")]
#[test]
fn a_hand_off_to_a_language_with_no_grammar_scope_fails() {
    let mut set = Grammars::new();
    set.hand_off(crate::Language::Rust);
    let failed = Highlighter::for_grammar(Grammar::from_json(DEMO).unwrap(), &set);
    assert!(matches!(failed.unwrap_err().0, Reason::Link(_)));
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
    grammar.patterns.push(grammar.rules.len());
    assert!(matches!(
        bytes::read(&bytes::write(&grammar)),
        Err(Reason::Malformed { .. })
    ));
}

#[test]
fn a_rule_named_twice_fails() {
    let mut grammar = compile(DEMO).unwrap();
    let first = grammar.patterns[0];
    grammar.patterns.push(first);
    assert!(bytes::read(&bytes::write(&grammar)).is_err());
}

#[test]
fn a_rule_that_names_itself_fails() {
    let grammar = grammar_of(vec![RawRule {
        patterns: vec![0],
        ..RawRule::default()
    }]);
    assert!(bytes::read(&bytes::write(&grammar)).is_err());
}

#[test]
fn rules_nested_past_the_depth_limit_fail() {
    let chain = |levels: usize| {
        grammar_of(
            (0..levels)
                .map(|id| RawRule {
                    patterns: if id + 1 < levels {
                        vec![id + 1]
                    } else {
                        Vec::new()
                    },
                    ..RawRule::default()
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
