//! Tests of the C# lexer.

use crate::arena::Arena;
use crate::csharp::engine::CSharp;
use crate::csharp::grammar::Pattern;
use crate::csharp::matchers::ProbeGroups;
use crate::test_support::probe::{After, Anchor, Case, lexer};

/// What `pattern` answers on `case`: where its match ends and the span of each
/// capture group, or `None` where it does not match.
fn answer(arena: &mut Arena, pattern: Pattern, case: &Case<'_>) -> Option<(usize, ProbeGroups)> {
    lexer::<CSharp, _>(arena, case, |lexer| lexer.probe(pattern, case.p))
}

/// The type and the name in `foreach` split the blanks between them as
/// Oniguruma does, which `oracle/onig.ts` gave for each case.
/// The type's last part takes them all and gives back one to `\s+`.
/// The type's group thus keeps all but the last.
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
        let case = Case {
            text,
            p: 0,
            anchor: Anchor::Free,
            after: After::LineFeed,
        };
        let (_, captures) = answer(&mut arena, Pattern::ForeachStatementAt1At1, &case)
            .unwrap_or_else(|| panic!("no match in {text:?}"));
        assert_eq!(
            (captures[3], captures[8]),
            (Some(ty), Some(name)),
            "{text:?}"
        );
    }
}

#[cfg(feature = "json")]
mod probe {
    use super::answer;
    use crate::csharp::grammar::Pattern;
    use crate::test_support::probe::answer_cases;

    /// Answers the cases of `oracle/onig.ts` with C#'s matchers (see
    /// [`answer_cases`]).
    #[test]
    #[ignore = "a tool: it needs a cases file"]
    fn matchers_answer_the_probe_cases() {
        answer_cases(&Pattern::ALL, |arena, pattern, case| {
            answer(arena, pattern, case).map(|(end, captures)| (end, captures.to_vec()))
        });
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

    let any = Rule::ALL.iter().any(|rule| rule.while_pattern().is_some());
    assert_eq!(Rule::HAS_WHILE, any);
    assert_eq!(CSharp::WHILES, any);
}
