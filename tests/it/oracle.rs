//! Compares nishiki's scopes with Shiki's on every fixture `oracle/generate.ts`
//! wrote under `tests/oracle/<language>/`.
//!
//! Each non-whitespace byte must carry the same scope stack as Shiki gives it,
//! the measure the research note used. Whitespace is left out because Shiki
//! and nishiki may split a run of spaces between tokens differently without
//! any colour changing.

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use nishiki::{Highlighter, Language, State};

/// One source line of a fixture and Shiki's scope stack for each of its bytes.
struct Line {
    text: String,
    stacks: Vec<String>,
}

/// A byte whose scope stack differs from Shiki's.
struct Miss {
    line: usize,
    column: usize,
    text: String,
    expected: String,
    actual: String,
}

/// Mismatches listed in a failure message; the rest are counted.
const SHOWN: usize = 20;

#[expect(
    clippy::expect_used,
    reason = "a test helper; a bad fixture fails the test"
)]
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

#[expect(
    clippy::expect_used,
    reason = "a test helper; a bad fixture fails the test"
)]
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

/// One line's tokens: each token's byte end and its scope stack, joined by
/// spaces. A lexer under test is a closure that keeps its own state between
/// calls, so each language plugs in without a shared trait.
type Tokens = Vec<(usize, String)>;

fn compare(lines: &[Line], mut lex: impl FnMut(&str) -> Tokens) -> (usize, Vec<Miss>) {
    let mut checked = 0;
    let mut misses = Vec::new();
    for (number, line) in lines.iter().enumerate() {
        let mut actual = Vec::with_capacity(line.text.len());
        let mut start = 0;
        for (end, scopes) in lex(&line.text) {
            actual.extend((start..end).map(|_| scopes.clone()));
            start = end;
        }
        for (column, byte) in line.text.bytes().enumerate() {
            if byte.is_ascii_whitespace() {
                continue;
            }
            checked += 1;
            let expected = line.stacks.get(column).map_or("", String::as_str);
            let got = actual.get(column).map_or("", String::as_str);
            if expected != got {
                misses.push(Miss {
                    line: number + 1,
                    column: column + 1,
                    text: line.text.clone(),
                    expected: expected.to_owned(),
                    actual: got.to_owned(),
                });
            }
        }
    }
    (checked, misses)
}

/// Checks every fixture of `language`, with a fresh lexer from `new_lexer`
/// for each, and fails with every fixture that differs from Shiki.
/// The fixtures of `language` whose scopes differ from Shiki's, as reports.
fn scope_failures(language: Language) -> Vec<String> {
    let mut failures = Vec::new();
    for path in fixtures(language.id()) {
        let (checked, misses) = compare(&parse(&path), lexer(language));
        if !misses.is_empty() {
            failures.push(report(&path, checked, &misses));
        }
    }
    failures
}

/// The failure message: the share that matched, then the first mismatches,
/// one per line of source.
fn report(path: &Path, checked: usize, misses: &[Miss]) -> String {
    let mut message = format!(
        "{}: {} of {checked} non-whitespace bytes differ from Shiki",
        path.display(),
        misses.len()
    );
    let mut last_line = 0;
    let mut shown = 0;
    for miss in misses {
        if miss.line == last_line || shown == SHOWN {
            continue;
        }
        last_line = miss.line;
        shown += 1;
        let _ = write!(
            message,
            "\n  line {} column {}: {:?}\n    shiki:   {}\n    nishiki: {}",
            miss.line, miss.column, miss.text, miss.expected, miss.actual
        );
    }
    message
}

/// A lexer under test: `language`'s highlighter with default limits, whose
/// every line must be lexed in full.
fn lexer(language: Language) -> impl FnMut(&str) -> Tokens {
    let mut highlighter = Highlighter::new(language);
    let mut state = State::default();
    let mut tokens = Vec::new();
    move |line: &str| {
        tokens.clear();
        let report = highlighter.line(line, &mut state, &mut tokens);
        assert!(
            report.is_complete(),
            "{line:?} reached {:?}",
            report.limits().collect::<Vec<_>>()
        );
        tokens
            .iter()
            .map(|token| {
                let mut scopes: Vec<&str> = highlighter.scopes(token.stack()).collect();
                scopes.reverse();
                (token.end(), scopes.join(" "))
            })
            .collect()
    }
}

/// Shiki's colours for one fixture under one theme: the fixture's lines,
/// each byte's `COLOUR:font style bits`, colours as nishiki prints them.
#[cfg(feature = "json")]
#[expect(
    clippy::expect_used,
    reason = "a test helper; a bad fixture fails the test"
)]
fn parse_colours(colours: &Path, fixture: &Path) -> Vec<Line> {
    let content = fs::read_to_string(colours).expect("a readable colour fixture");
    let lines = parse(fixture);
    lines
        .into_iter()
        .zip(content.lines())
        .map(|(line, row)| {
            let mut stacks = Vec::with_capacity(line.text.len());
            let mut start = 0;
            for token in row.split(' ').filter(|token| !token.is_empty()) {
                let mut fields = token.splitn(3, ':');
                let end: usize = fields
                    .next()
                    .and_then(|end| end.parse().ok())
                    .expect("a byte end");
                let colour = fields
                    .next()
                    .and_then(nishiki::Color::from_hex)
                    .expect("a hex colour");
                let font: u8 = fields
                    .next()
                    .and_then(|font| font.parse().ok())
                    .expect("font bits");
                stacks.extend((start..end).map(|_| format!("{colour}:{font}")));
                start = end;
            }
            Line {
                text: line.text,
                stacks,
            }
        })
        .collect()
}

/// Checks nishiki's colours against Shiki's for every colour fixture of
/// `language`, under each theme in `tests/oracle/themes/`.
#[cfg(feature = "json")]
#[expect(
    clippy::expect_used,
    reason = "a test helper; a bad fixture fails the test"
)]
/// The fixture and theme pairs of `language` whose colours differ from
/// Shiki's, as reports.
fn colour_failures(language: Language) -> Vec<String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/oracle");
    let mut failures = Vec::new();
    for fixture in fixtures(language.id()) {
        let stem = fixture
            .file_stem()
            .and_then(|stem| stem.to_str())
            .expect("a UTF-8 name");
        for theme_path in fs::read_dir(root.join("themes")).expect("the themes directory exists") {
            let theme_path = theme_path.expect("a readable directory entry").path();
            let name = theme_path
                .file_stem()
                .and_then(|name| name.to_str())
                .expect("a UTF-8 name");
            let colours = fixture.with_file_name(format!("{stem}.{name}.colors"));
            let json = fs::read_to_string(&theme_path).expect("a readable theme");
            let theme = nishiki::Theme::from_json(&json).expect("a theme Shiki loads");
            let lines = parse_colours(&colours, &fixture);
            let mut highlighter = Highlighter::new(language);
            let mut state = State::default();
            let mut tokens = Vec::new();
            let (checked, misses) = compare(&lines, |line: &str| {
                tokens.clear();
                let _ = highlighter.line(line, &mut state, &mut tokens);
                tokens
                    .iter()
                    .map(|token| {
                        let style = theme.style(highlighter.scopes(token.stack()));
                        (
                            token.end(),
                            format!("{}:{}", style.foreground, style.font.bits()),
                        )
                    })
                    .collect()
            });
            if !misses.is_empty() {
                failures.push(report(&colours, checked, &misses));
            }
        }
    }
    failures
}

/// Runs `failures` for every language at once and fails with every report.
#[expect(
    clippy::expect_used,
    reason = "a test helper; a panicking language fails the test"
)]
fn every_language(failures: fn(Language) -> Vec<String>) {
    let reports: Vec<String> = std::thread::scope(|scope| {
        let runs: Vec<_> = Language::ALL
            .iter()
            .map(|&language| scope.spawn(move || failures(language)))
            .collect();
        runs.into_iter()
            .flat_map(|run| run.join().expect("a language's check does not panic"))
            .collect()
    });
    assert!(reports.is_empty(), "{}", reports.join("\n\n"));
}

#[test]
fn every_language_matches_shiki() {
    every_language(scope_failures);
}

#[cfg(feature = "json")]
#[test]
fn every_language_colours_match_shiki() {
    every_language(colour_failures);
}
