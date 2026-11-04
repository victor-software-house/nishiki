//! The engine against Shiki: each grammar of the four hand-written languages,
//! run over the fixtures `oracle/generate.ts` wrote, must give every
//! non-whitespace byte the scope stack Shiki gives it.

use std::collections::HashMap;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use super::grammar::{Grammar, NONE};
use super::tokenizer::{self, Scratch, Text};
use crate::arena::Arena;
use crate::limits::{Limits, LineReport};

#[path = "../../tests/engine/javascript.rs"]
mod javascript_tables;
#[path = "../../tests/engine/make.rs"]
mod make_tables;
#[path = "../../tests/engine/markdown.rs"]
mod markdown_tables;
#[path = "../../tests/engine/python.rs"]
mod python_tables;
#[path = "../../tests/engine/rust.rs"]
mod rust_tables;
#[path = "../../tests/engine/shellscript.rs"]
mod shell_tables;
#[path = "../../tests/engine/toml.rs"]
mod toml_tables;
#[path = "../../tests/engine/typescript.rs"]
mod typescript_tables;
#[path = "../../tests/engine/yaml.rs"]
mod yaml_tables;

/// Whether the engine finds where rules start with its scanners, or searches
/// every rule alone, to check that both give one output.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Scanners {
    On,
    Off,
}

/// One source line of a fixture and Shiki's scope stack for each byte.
struct Line {
    text: String,
    stacks: Vec<String>,
}

fn fixtures(language: &str) -> Vec<PathBuf> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/oracle")
        .join(language);
    let mut paths: Vec<PathBuf> = fs::read_dir(&dir)
        .expect("the fixture directory exists")
        .map(|entry| entry.expect("a readable directory entry").path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "txt"))
        .collect();
    paths.sort();
    assert!(!paths.is_empty(), "no fixtures in {}", dir.display());
    paths
}

fn parse(path: &Path) -> Vec<Line> {
    let content = fs::read_to_string(path).expect("a readable fixture");
    let mut lines: Vec<Line> = Vec::new();
    let mut start = 0;
    for record in content.lines() {
        if let Some(text) = record.strip_prefix("L ") {
            lines.push(Line {
                text: text.to_owned(),
                stacks: Vec::new(),
            });
            start = 0;
        } else if let Some(token) = record.strip_prefix("T ") {
            let (end, scopes) = token.split_once(' ').expect("a byte end and scopes");
            let end: usize = end.parse().expect("a byte end");
            let line = lines.last_mut().expect("a token after its line");
            line.stacks.extend((start..end).map(|_| scopes.to_owned()));
            start = end;
        } else {
            panic!("{}: unknown record {record:?}", path.display());
        }
    }
    lines
}

/// Which lines sit in an embedded block, a fence or front matter, of a
/// language the grammar's tables do not hold. The tables hold the languages
/// the oracle had when they were written, so a block of a language added
/// since cannot match, and it is left out rather than failing every new
/// language's pull request. A block is of such a language when Shiki gives a
/// scope the tables never name, or a `meta.embedded.block.*` scope that no
/// rule reachable from the root gives its contents, because the rule that
/// embeds the language was dropped from every list with the language.
fn unknown_fences(grammar: &Grammar, lines: &[Line]) -> Vec<bool> {
    let inside = |line: &Line| {
        line.stacks.iter().any(|stack| {
            stack.contains("markup.fenced_code.block.markdown") || stack.contains(".frontmatter")
        })
    };
    let mut reachable = vec![false; grammar.rules.len()];
    let mut todo = vec![grammar.root];
    while let Some(id) = todo.pop() {
        if std::mem::replace(&mut reachable[id], true) {
            continue;
        }
        let rule = &grammar.rules[id];
        if let Some(list) = grammar.lists.get(rule.list) {
            todo.extend(list.rules);
        }
        todo.extend(
            rule.captures
                .iter()
                .chain(rule.end_captures)
                .map(|cap| cap.retokenize)
                .filter(|&retokenize| retokenize != NONE),
        );
    }
    let embeds = |scope: &str| {
        grammar
            .rules
            .iter()
            .zip(&reachable)
            .any(|(rule, &reached)| {
                reached
                    && rule
                        .content_name
                        .iter()
                        .any(|&name| grammar.names.get(usize::from(name)) == Some(&scope))
            })
    };
    let mut skip = vec![false; lines.len()];
    let mut at = 0;
    while at < lines.len() {
        if !inside(&lines[at]) {
            at += 1;
            continue;
        }
        let end = (at..lines.len())
            .find(|&next| !inside(&lines[next]))
            .unwrap_or(lines.len());
        let unknown = lines[at..end]
            .iter()
            .flat_map(|line| &line.stacks)
            .flat_map(|stack| stack.split(' '))
            .any(|scope| {
                scope != grammar.scope
                    && (!grammar.names.contains(&scope)
                        || (scope.starts_with("meta.embedded.block.") && !embeds(scope)))
            });
        if unknown {
            skip[at..end].fill(true);
        }
        at = end;
    }
    skip
}

/// The bytes of `lines` whose stack differs from Shiki's, with a message for
/// each of the first few.
fn run(grammar: &'static Grammar, lines: &[Line], scanners: Scanners) -> (usize, usize, String) {
    let mut arena = Arena::new(None);
    let limits = Limits::unlimited();
    let mut scratch = Scratch::default();
    scratch.no_scanner = scanners == Scanners::Off;
    let mut state = tokenizer::start(&mut arena, grammar, crate::arena::ROOT);
    let mut names: HashMap<u32, String> = HashMap::new();
    let (mut checked, mut missed) = (0, 0);
    let mut message = String::new();
    let mut shown = 0;
    let skip = unknown_fences(grammar, lines);
    for (number, line) in lines.iter().enumerate() {
        let mut out = Vec::new();
        let mut report = LineReport::default();
        state = tokenizer::line(
            &mut scratch,
            &mut arena,
            &limits,
            Text {
                grammar,
                bytes: line.text.as_bytes(),
                start: 0,
                anchored: false,
            },
            state,
            &mut out,
            &mut report,
        );
        let mut actual: Vec<u32> = Vec::new();
        let mut start = 0;
        for token in &out {
            actual.extend((start..token.end).map(|_| token.stack.0));
            start = token.end;
        }
        if skip[number] {
            continue;
        }
        let mut line_missed = 0;
        for (column, byte) in line.text.bytes().enumerate() {
            if byte.is_ascii_whitespace() {
                continue;
            }
            checked += 1;
            let expected = line.stacks.get(column).map_or("", String::as_str);
            let got = match actual.get(column) {
                Some(&stack) => names.entry(stack).or_insert_with(|| {
                    let mut scopes: Vec<&str> = arena.scopes(stack).collect();
                    scopes.reverse();
                    scopes.join(" ")
                }),
                None => "",
            };
            if expected != got {
                missed += 1;
                line_missed += 1;
                if line_missed == 1 && shown < 8 {
                    shown += 1;
                    let _ = write!(
                        message,
                        "\n  line {} col {}: {:?}",
                        number + 1,
                        column + 1,
                        line.text
                    );
                    let _ = write!(message, "\n    expected {expected}");
                    let _ = write!(message, "\n    actual   {got}");
                }
            }
        }
    }
    (checked, missed, message)
}

fn check(grammar: &'static Grammar, language: &str, scanners: Scanners) {
    let mut failures = Vec::new();
    for path in fixtures(language) {
        let (checked, missed, message) = run(grammar, &parse(&path), scanners);
        if missed > 0 {
            failures.push(format!(
                "{}: {missed} of {checked} bytes differ{message}",
                path.display()
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

#[test]
fn rust_grammar_matches_shiki() {
    check(&rust_tables::GRAMMAR, "rust", Scanners::On);
}

#[test]
fn shell_grammar_matches_shiki() {
    check(&shell_tables::GRAMMAR, "shellscript", Scanners::On);
}

#[test]
fn typescript_grammar_matches_shiki() {
    check(&typescript_tables::GRAMMAR, "typescript", Scanners::On);
}

#[test]
fn markdown_grammar_matches_shiki() {
    check(&markdown_tables::GRAMMAR, "markdown", Scanners::On);
}

#[test]
fn python_grammar_matches_shiki() {
    check(&python_tables::GRAMMAR, "python", Scanners::On);
}

#[test]
fn yaml_grammar_matches_shiki() {
    check(&yaml_tables::GRAMMAR, "yaml", Scanners::On);
}

#[test]
fn make_grammar_matches_shiki() {
    check(&make_tables::GRAMMAR, "make", Scanners::On);
}

#[test]
fn toml_grammar_matches_shiki() {
    check(&toml_tables::GRAMMAR, "toml", Scanners::On);
}

#[test]
fn javascript_grammar_matches_shiki() {
    check(&javascript_tables::GRAMMAR, "javascript", Scanners::On);
}

#[test]
fn every_grammar_matches_shiki_without_the_scanners() {
    check(&rust_tables::GRAMMAR, "rust", Scanners::Off);
    check(&shell_tables::GRAMMAR, "shellscript", Scanners::Off);
    check(&typescript_tables::GRAMMAR, "typescript", Scanners::Off);
    check(&markdown_tables::GRAMMAR, "markdown", Scanners::Off);
    check(&python_tables::GRAMMAR, "python", Scanners::Off);
    check(&yaml_tables::GRAMMAR, "yaml", Scanners::Off);
    check(&make_tables::GRAMMAR, "make", Scanners::Off);
    check(&toml_tables::GRAMMAR, "toml", Scanners::Off);
    check(&javascript_tables::GRAMMAR, "javascript", Scanners::Off);
}

/// Times the engine on the fixtures of one language, in MiB/s: the best of
/// five passes, each with a fresh scratch (a document of its own, whose lazy
/// automata start empty) and the best of five passes that reuse one scratch,
/// and the slowest line's best time in the fresh passes. The first pass is
/// not timed: it builds the scanners, which a program builds once.
/// Run with `cargo test --release --lib engine_speed -- --ignored --nocapture`.
#[expect(
    clippy::cast_precision_loss,
    clippy::as_conversions,
    reason = "a measurement that prints its table"
)]
fn speed(grammar: &'static Grammar, language: &str, limits: &Limits, scanners: Scanners) {
    use std::time::Instant;
    for path in fixtures(language) {
        let lines = parse(&path);
        let bytes: usize = lines.iter().map(|l| l.text.len() + 1).sum();
        let mut worst = vec![f64::MAX; lines.len()];
        let pass = |scratch: &mut Scratch, worst: Option<&mut Vec<f64>>| {
            let mut arena = Arena::new(limits.nodes);
            scratch.no_scanner = scanners == Scanners::Off;
            let mut state = tokenizer::start(&mut arena, grammar, crate::arena::ROOT);
            let mut out = Vec::new();
            let mut worst = worst;
            let begin = Instant::now();
            for (number, line) in lines.iter().enumerate() {
                out.clear();
                let mut report = LineReport::default();
                let t = Instant::now();
                state = tokenizer::line(
                    scratch,
                    &mut arena,
                    limits,
                    Text {
                        grammar,
                        bytes: line.text.as_bytes(),
                        start: 0,
                        anchored: false,
                    },
                    state,
                    &mut out,
                    &mut report,
                );
                if let Some(worst) = worst.as_deref_mut() {
                    worst[number] = worst[number].min(t.elapsed().as_secs_f64());
                }
            }
            begin.elapsed().as_secs_f64()
        };
        let first = pass(&mut Scratch::default(), None);
        eprintln!("  first pass (builds the scanners) {:.1} ms", first * 1e3);
        let cold = (0..5)
            .map(|_| pass(&mut Scratch::default(), Some(&mut worst)))
            .fold(f64::MAX, f64::min);
        let mut scratch = Scratch::default();
        let warm = (0..5)
            .map(|_| pass(&mut scratch, None))
            .fold(f64::MAX, f64::min);
        let mut sorted = worst.clone();
        sorted.sort_by(f64::total_cmp);
        let p99 = sorted[sorted.len() * 99 / 100];
        let max = sorted[sorted.len() - 1];
        let mib = |seconds: f64| bytes as f64 / seconds / 1_048_576.0;
        eprintln!(
            "{language:12} {:32} cold {:7.2}  warm {:7.2} MiB/s  p99 {:7.1} us  max {:7.1} us",
            path.file_stem().and_then(|s| s.to_str()).unwrap_or(""),
            mib(cold),
            mib(warm),
            p99 * 1e6,
            max * 1e6,
        );
    }
}

#[test]
#[ignore = "a measurement, run in release mode"]
fn engine_speed() {
    let limits = Limits::default();
    for scanners in [Scanners::Off, Scanners::On] {
        eprintln!(
            "scanners {}",
            if scanners == Scanners::Off {
                "off"
            } else {
                "on"
            }
        );
        speed(&rust_tables::GRAMMAR, "rust", &limits, scanners);
        speed(&shell_tables::GRAMMAR, "shellscript", &limits, scanners);
        speed(&typescript_tables::GRAMMAR, "typescript", &limits, scanners);
        speed(&markdown_tables::GRAMMAR, "markdown", &limits, scanners);
        speed(&python_tables::GRAMMAR, "python", &limits, scanners);
        speed(&yaml_tables::GRAMMAR, "yaml", &limits, scanners);
    }
}

#[test]
fn an_embedded_line_is_lexed_from_its_start_with_absolute_ends() {
    let grammar: &'static Grammar = &rust_tables::GRAMMAR;
    let mut arena = Arena::new(None);
    let mut scratch = Scratch::default();
    let state = tokenizer::start_embedded(&mut arena, grammar, crate::arena::ROOT);
    let mut out = Vec::new();
    let mut report = LineReport::default();
    let line = b"> let x = 1;";
    tokenizer::line(
        &mut scratch,
        &mut arena,
        &Limits::unlimited(),
        Text {
            grammar,
            bytes: line,
            start: 2,
            anchored: true,
        },
        state,
        &mut out,
        &mut report,
    );
    let ends: Vec<usize> = out.iter().map(|t| t.end).collect();
    assert!(ends.iter().all(|&end| end > 2), "{ends:?}");
    assert_eq!(ends.last(), Some(&line.len()));
    let first = arena.scopes(out[0].stack.0).collect::<Vec<_>>();
    assert_eq!(first.first(), Some(&"storage.type.rust"), "{first:?}");
    assert!(!first.contains(&"source.rust"), "{first:?}");
}

/// The most work any oracle line costs the machine, as a share of the
/// default budget of 64 steps a byte plus 4,096, at 32 machine steps each.
#[test]
#[ignore = "a measurement"]
#[expect(
    clippy::cast_precision_loss,
    clippy::as_conversions,
    reason = "a measurement that prints its table"
)]
fn work_per_line() {
    let grammars: [(&str, &'static Grammar); 6] = [
        ("rust", &rust_tables::GRAMMAR),
        ("shellscript", &shell_tables::GRAMMAR),
        ("typescript", &typescript_tables::GRAMMAR),
        ("markdown", &markdown_tables::GRAMMAR),
        ("python", &python_tables::GRAMMAR),
        ("yaml", &yaml_tables::GRAMMAR),
    ];
    for (language, grammar) in grammars {
        let mut worst = 0.0_f64;
        let mut worst_line = String::new();
        for path in fixtures(language) {
            let lines = parse(&path);
            let mut arena = Arena::new(None);
            let mut scratch = Scratch::default();
            let mut state = tokenizer::start(&mut arena, grammar, crate::arena::ROOT);
            for line in &lines {
                let mut out = Vec::new();
                let mut report = LineReport::default();
                state = tokenizer::line(
                    &mut scratch,
                    &mut arena,
                    &Limits::unlimited(),
                    Text {
                        grammar,
                        bytes: line.text.as_bytes(),
                        start: 0,
                        anchored: false,
                    },
                    state,
                    &mut out,
                    &mut report,
                );
                let budget = (64 * line.text.len() + 4096) * 32;
                let ratio = scratch.spent as f64 / budget as f64;
                if ratio > worst {
                    worst = ratio;
                    worst_line = format!("{} steps: {:?}", scratch.spent, line.text);
                }
            }
        }
        eprintln!(
            "{language:12} worst line {worst:5.2} of the default budget  {}",
            worst_line.chars().take(90).collect::<String>()
        );
    }
}

/// The survey over many languages: grammars read from the JSON tables
/// `oracle/engine-tables.ts --json` writes, run on the fixtures of a
/// directory (`NISHIKI_SURVEY`, holding `<id>.json` and `<id>.txt` for each
/// language). Run with `--features json`; the tables are leaked, as a test
/// may.
#[cfg(feature = "json")]
mod survey {
    use std::sync::OnceLock;

    use serde_json::Value;

    use super::{Grammar, Scanners, fixtures_in, parse, run};
    use crate::textmate::grammar::{Cap, Injection, List, Pattern, Rule, RuleKind};

    fn leak<T>(items: Vec<T>) -> &'static [T] {
        Box::leak(items.into_boxed_slice())
    }

    fn index(value: &Value) -> usize {
        value
            .as_i64()
            .and_then(|n| usize::try_from(n).ok())
            .unwrap_or(usize::MAX)
    }

    fn numbers<T: TryFrom<u64>>(value: &Value) -> Vec<T> {
        value
            .as_array()
            .map(|items| {
                items
                    .iter()
                    .filter_map(|n| n.as_u64().and_then(|n| T::try_from(n).ok()))
                    .collect()
            })
            .unwrap_or_default()
    }

    fn caps(value: &Value) -> &'static [Cap] {
        leak(
            value
                .as_array()
                .map(|items| {
                    items
                        .iter()
                        .map(|c| Cap {
                            index: index(&c["index"]),
                            name: leak(numbers(&c["name"])),
                            content_name: leak(numbers(&c["contentName"])),
                            retokenize: index(&c["retokenize"]),
                        })
                        .collect()
                })
                .unwrap_or_default(),
        )
    }

    fn text(value: &Value) -> &'static str {
        Box::leak(
            value
                .as_str()
                .unwrap_or_default()
                .to_owned()
                .into_boxed_str(),
        )
    }

    pub(super) fn load(json: &str) -> &'static Grammar {
        let v: Value = serde_json::from_str(json).unwrap();
        let rules = v["rules"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| Rule {
                kind: match r["kind"].as_str().unwrap() {
                    "Match" => RuleKind::Match,
                    "BeginEnd" => RuleKind::BeginEnd,
                    "BeginWhile" => RuleKind::BeginWhile,
                    _ => RuleKind::Container,
                },
                name: leak(numbers(&r["name"])),
                content_name: leak(numbers(&r["contentName"])),
                begin: index(&r["begin"]),
                end: index(&r["end"]),
                until: index(&r["until"]),
                captures: caps(&r["captures"]),
                end_captures: caps(&r["endCaptures"]),
                list: index(&r["list"]),
                apply_end_last: r["applyEndLast"].as_bool().unwrap_or(false),
            })
            .collect();
        let lists: Vec<List> = v["lists"]
            .as_array()
            .unwrap()
            .iter()
            .map(|l| List::new(leak(numbers(l))))
            .collect();
        let patterns: Vec<Pattern> = v["patterns"]
            .as_array()
            .unwrap()
            .iter()
            .map(|p| Pattern::new(text(&p["source"])))
            .collect();
        let injections = v["injections"]
            .as_array()
            .map(|items| {
                items
                    .iter()
                    .map(|i| Injection {
                        selector: text(&i["selector"]),
                        rule: index(&i["rule"]),
                    })
                    .collect()
            })
            .unwrap_or_default();
        let names = v["names"].as_array().unwrap().iter().map(text).collect();
        Box::leak(Box::new(Grammar {
            scope: text(&v["scope"]),
            names: leak(names),
            rules: leak(rules),
            lists: leak(lists),
            injections: leak(injections),
            patterns: leak(patterns),
            root: index(&v["root"]),
            cells: OnceLock::new(),
        }))
    }

    #[test]
    #[ignore = "a survey over a scratch directory"]
    fn engine_against_shiki_on_many_languages() {
        let Some(dir) = std::env::var_os("NISHIKI_SURVEY") else {
            return;
        };
        let dir = std::path::PathBuf::from(dir);
        let mut rows: Vec<(String, usize, usize, String)> = Vec::new();
        for fixture in fixtures_in(&dir) {
            let id = fixture.file_stem().unwrap().to_string_lossy().into_owned();
            let Ok(json) = std::fs::read_to_string(dir.join(format!("{id}.json"))) else {
                continue;
            };
            let lines = parse(&fixture);
            let outcome = std::panic::catch_unwind(|| {
                let grammar = load(&json);
                run(
                    grammar,
                    &lines,
                    if std::env::var_os("NISHIKI_NO_SCANNER").is_some() {
                        Scanners::Off
                    } else {
                        Scanners::On
                    },
                )
            });
            match outcome {
                Ok((checked, missed, message)) => {
                    rows.push((
                        id,
                        checked,
                        missed,
                        message
                            .lines()
                            .skip(1)
                            .take(3)
                            .collect::<Vec<_>>()
                            .join("\n"),
                    ));
                }
                Err(_) => rows.push((id, 0, usize::MAX, "PANIC".to_owned())),
            }
        }
        rows.sort_by_key(|r| r.2);
        let exact = rows.iter().filter(|r| r.2 == 0).count();
        eprintln!(
            "{exact} of {} languages match Shiki on every byte of their sample",
            rows.len()
        );
        for (id, checked, missed, first) in &rows {
            if *missed > 0 {
                eprintln!("  {id}: {missed} of {checked} bytes differ\n{first}");
            }
        }
    }
}

#[cfg(feature = "json")]
fn fixtures_in(dir: &Path) -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = fs::read_dir(dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "txt"))
        .collect();
    paths.sort();
    paths
}

/// A grammar of one rule that names its forty-second group, as C++'s
/// function definitions name a group far into their pattern.
mod many_groups {
    use std::sync::OnceLock;

    use super::super::grammar::{Cap, Grammar, List, Pattern, Rule, RuleKind};

    static PATTERNS: [Pattern; 1] = [Pattern::new(
        "()()()()()()()()()()()()()()()()()()()()()()()()()()()()()()()()()()()()()()()()(x)",
    )];
    static LISTS: [List; 2] = [List::new(&[1]), List::new(&[])];
    static RULES: [Rule; 2] = [
        Rule {
            kind: RuleKind::Container,
            name: &[],
            content_name: &[],
            begin: usize::MAX,
            end: usize::MAX,
            until: usize::MAX,
            captures: &[],
            end_captures: &[],
            list: 0,
            apply_end_last: false,
        },
        Rule {
            kind: RuleKind::Match,
            name: &[],
            content_name: &[],
            begin: 0,
            end: usize::MAX,
            until: usize::MAX,
            captures: &[Cap {
                index: 41,
                name: &[0],
                content_name: &[],
                retokenize: usize::MAX,
            }],
            end_captures: &[],
            list: 1,
            apply_end_last: false,
        },
    ];
    pub(super) static GRAMMAR: Grammar = Grammar {
        scope: "source.test",
        names: &["keyword.test"],
        rules: &RULES,
        lists: &LISTS,
        injections: &[],
        patterns: &PATTERNS,
        root: 0,
        cells: OnceLock::new(),
    };
}

#[test]
fn a_group_far_into_a_pattern_is_named() {
    let grammar: &'static Grammar = &many_groups::GRAMMAR;
    let mut arena = Arena::new(None);
    let mut scratch = Scratch::default();
    let state = tokenizer::start(&mut arena, grammar, crate::arena::ROOT);
    let mut out = Vec::new();
    tokenizer::line(
        &mut scratch,
        &mut arena,
        &Limits::unlimited(),
        Text {
            grammar,
            bytes: b"ax",
            start: 0,
            anchored: false,
        },
        state,
        &mut out,
        &mut LineReport::default(),
    );
    let scopes: Vec<Vec<&str>> = out
        .iter()
        .map(|t| arena.scopes(t.stack.0).collect())
        .collect();
    assert_eq!(scopes.len(), 2, "{scopes:?}");
    assert_eq!(scopes[1], ["keyword.test", "source.test"]);
}
