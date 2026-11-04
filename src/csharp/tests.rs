//! Tests of the C# lexer.

use crate::arena::{Arena, ROOT};
use crate::csharp::engine::{Lexer, Scratch};
use crate::csharp::grammar::Pat;
use crate::csharp::matchers::ProbeGroups;
use crate::language::Parts;
use crate::limits::{Limits, LineReport};
use crate::rules::Text;

/// Whether `\G` holds where a matcher is tried.
#[derive(Clone, Copy, Debug)]
enum Anchor {
    G,
    Free,
}

/// What follows the text a matcher is tried on.
#[derive(Clone, Copy, Debug)]
enum After {
    LineFeed,
    Nothing,
}

/// What `pat` answers at `p` in `text`: where its match ends and the span of
/// each capture group, or `None` where it does not match.
fn answer(
    arena: &mut Arena,
    pat: Pat,
    text: &str,
    p: usize,
    anchor: Anchor,
    after: After,
) -> Option<(usize, ProbeGroups)> {
    let limits = Limits::default();
    let mut tokens = Vec::new();
    let mut report = LineReport::default();
    let mut scratch = Scratch::default();
    let parts = Parts {
        arena,
        limits: &limits,
        out: &mut tokens,
        report: &mut report,
    };
    let mut lexer = Lexer::new(
        parts,
        Text {
            bytes: text.as_bytes(),
            start: 0,
            anchored: false,
        },
        ROOT,
        &mut scratch,
    );
    lexer.nl = matches!(after, After::LineFeed);
    lexer.pos = p;
    lexer.g = matches!(anchor, Anchor::G);
    lexer.probe(pat, p)
}

/// The type and the name in `foreach` split the blanks between them as
/// Oniguruma does, which `oracle/onig.ts` gave for each case: the type's
/// last part takes them all and gives back one to `\s+`, so the type's
/// group keeps all but the last.
#[test]
fn a_foreach_type_keeps_all_but_the_last_blank_before_the_name() {
    let mut arena = Arena::new(None);
    let cases = [
        ("int x in xs", (0, 3), (4, 5)),
        ("int  x in xs", (0, 4), (5, 6)),
        ("int\t \tx in xs", (0, 5), (6, 7)),
        ("List<int>   x in xs", (0, 11), (12, 13)),
        ("int?  x in xs", (0, 5), (6, 7)),
        ("int[]  x in xs", (0, 6), (7, 8)),
        ("c  ent in source)", (0, 2), (3, 6)),
    ];
    for (text, ty, name) in cases {
        let (_, caps) = answer(
            &mut arena,
            Pat::ForeachStatementAt1At1,
            text,
            0,
            Anchor::Free,
            After::LineFeed,
        )
        .unwrap_or_else(|| panic!("no match in {text:?}"));
        assert_eq!((caps[3], caps[8]), (Some(ty), Some(name)), "{text:?}");
    }
}

#[cfg(feature = "json")]
mod probe {
    use serde_json::{Value, json};

    use super::{After, Anchor, answer};
    use crate::arena::Arena;
    use crate::csharp::grammar::Pat;

    /// Runs the matchers on the cases in the file that `PROBE_IN` names, one
    /// JSON object per line (`rx`, `text`, `p`, `g`, `nl`), and writes their
    /// answers to the file `PROBE_OUT` names. `oracle/onig.ts` compares them
    /// with Oniguruma's.
    #[test]
    #[ignore = "a tool: it needs a cases file"]
    fn matchers_answer_the_probe_cases() {
        let input = std::env::var("PROBE_IN").expect("PROBE_IN");
        let output = std::env::var("PROBE_OUT").expect("PROBE_OUT");
        let cases = std::fs::read_to_string(input).expect("cases");
        let mut out = String::new();
        let mut arena = Arena::new(None);
        for line in cases.lines() {
            let case: Value = serde_json::from_str(line).expect("a case");
            let name = case["rx"].as_str().expect("rx");
            let pat = Pat::ALL
                .iter()
                .copied()
                .find(|pat| format!("{pat:?}") == name)
                .expect("a known pattern");
            let text = case["text"].as_str().expect("text");
            let p = usize::try_from(case["p"].as_u64().expect("p")).expect("p");
            let anchor = match case["g"].as_bool() {
                Some(true) => Anchor::G,
                _ => Anchor::Free,
            };
            let after = match case["nl"].as_bool() {
                Some(false) => After::Nothing,
                _ => After::LineFeed,
            };
            let found = answer(&mut arena, pat, text, p, anchor, after);
            let value = found.map_or(Value::Null, |(end, caps)| {
                json!({ "end": end, "caps": caps.iter().map(|c| c.map(|(a, b)| [a, b])).collect::<Vec<_>>() })
            });
            out.push_str(&value.to_string());
            out.push('\n');
        }
        std::fs::write(output, out).expect("write");
    }
}

#[test]
#[ignore = "a tool: it reads the C# oracle fixtures"]
fn oracle_diff() {
    use std::collections::BTreeMap;

    use crate::{Highlighter, Language, State};
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/oracle/csharp");
    let only = std::env::var("CSHARP_ONLY").ok();
    let mut groups: BTreeMap<(String, String), (usize, String)> = BTreeMap::new();
    let mut total = (0, 0);
    let mut paths: Vec<_> = std::fs::read_dir(dir)
        .expect("dir")
        .map(|e| e.expect("entry").path())
        .filter(|p| p.extension().is_some_and(|e| e == "txt"))
        .collect();
    paths.sort();
    for path in paths {
        let name = path
            .file_stem()
            .expect("stem")
            .to_string_lossy()
            .into_owned();
        if only.as_ref().is_some_and(|o| !name.contains(o.as_str())) {
            continue;
        }
        let content = std::fs::read_to_string(&path).expect("fixture");
        let mut lines: Vec<(String, Vec<String>)> = Vec::new();
        let mut start = 0;
        for record in content.lines() {
            if let Some(text) = record.strip_prefix("L ") {
                lines.push((text.to_owned(), Vec::new()));
                start = 0;
            } else if let Some(token) = record.strip_prefix("T ") {
                let (end, scopes) = token.split_once(' ').expect("token");
                let end: usize = end.parse().expect("end");
                let line = lines.last_mut().expect("line");
                line.1.extend((start..end).map(|_| scopes.to_owned()));
                start = end;
            }
        }
        let mut highlighter = Highlighter::new(Language::CSharp);
        let mut state = State::default();
        let mut tokens = Vec::new();
        for (number, (text, expected)) in lines.iter().enumerate() {
            tokens.clear();
            let _ = highlighter.line(text, &mut state, &mut tokens);
            let mut actual = Vec::new();
            let mut begin = 0;
            for token in &tokens {
                let mut scopes: Vec<&str> = highlighter.scopes(token.stack()).collect();
                scopes.reverse();
                let joined = scopes.join(" ");
                actual.extend((begin..token.end()).map(|_| joined.clone()));
                begin = token.end();
            }
            for (column, byte) in text.bytes().enumerate() {
                if byte.is_ascii_whitespace() {
                    continue;
                }
                total.0 += 1;
                let want = expected.get(column).map_or("", String::as_str);
                let got = actual.get(column).map_or("", String::as_str);
                if want != got {
                    total.1 += 1;
                    let tail = |s: &str| {
                        let parts: Vec<&str> = s.split(' ').collect();
                        parts[parts.len().saturating_sub(2)..].join(" ")
                    };
                    let entry = groups
                        .entry((tail(want), tail(got)))
                        .or_insert((0, format!("{name}:{}:{} {text:?}", number + 1, column + 1)));
                    entry.0 += 1;
                }
            }
        }
    }
    let mut sorted: Vec<_> = groups.into_iter().collect();
    sorted.sort_by_key(|a| std::cmp::Reverse(a.1.0));
    eprintln!("{} of {} differ", total.1, total.0);
    for ((want, got), (count, example)) in sorted.into_iter().take(40) {
        eprintln!("{count:5} want [{want}] got [{got}]\n        e.g. {example}");
    }
}

/// The engine keeps the indices of open `while` rules only when the flag is
/// set. The flag must say whether any rule has a `while` pattern.
#[test]
fn the_while_flag_matches_the_rules() {
    use crate::csharp::engine::CSharp;
    use crate::csharp::grammar::Rule;
    use crate::rules::Grammar;

    let any = Rule::ALL.iter().any(|rule| rule.while_pat().is_some());
    assert_eq!(Rule::HAS_WHILE, any);
    assert_eq!(CSharp::WHILES, any);
}
