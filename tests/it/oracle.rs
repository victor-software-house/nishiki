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
fn check<L: FnMut(&str) -> Tokens>(language: &str, new_lexer: impl Fn() -> L) {
    let mut failures = Vec::new();
    for path in fixtures(language) {
        let (checked, misses) = compare(&parse(&path), new_lexer());
        if !misses.is_empty() {
            failures.push(report(&path, checked, &misses));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
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

#[test]
fn rust_matches_shiki() {
    check("rust", || lexer(Language::Rust));
}

#[test]
fn shell_matches_shiki() {
    check("shellscript", || lexer(Language::Shell));
}

#[test]
fn json_matches_shiki() {
    check("json", || lexer(Language::Json));
}

#[test]
fn jsonc_matches_shiki() {
    check("jsonc", || lexer(Language::Jsonc));
}

#[test]
fn jsonl_matches_shiki() {
    check("jsonl", || lexer(Language::Jsonl));
}

#[test]
fn python_matches_shiki() {
    check("python", || lexer(Language::Python));
}

#[test]
fn yaml_matches_shiki() {
    check("yaml", || lexer(Language::Yaml));
}

#[test]
fn toml_matches_shiki() {
    check("toml", || lexer(Language::Toml));
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
fn check_colours(language: Language, dir: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/oracle");
    let mut failures = Vec::new();
    for fixture in fixtures(dir) {
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
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

#[test]
fn make_matches_shiki() {
    check("make", || lexer(Language::Make));
}

#[cfg(feature = "json")]
#[test]
fn make_colours_match_shiki() {
    check_colours(Language::Make, "make");
}

#[cfg(feature = "json")]
#[test]
fn rust_colours_match_shiki() {
    check_colours(Language::Rust, "rust");
}

#[test]
fn markdown_matches_shiki() {
    check("markdown", || lexer(Language::Markdown));
}

#[cfg(feature = "json")]
#[test]
fn markdown_colours_match_shiki() {
    check_colours(Language::Markdown, "markdown");
}

#[test]
fn typescript_matches_shiki() {
    check("typescript", || lexer(Language::TypeScript));
}

#[cfg(feature = "json")]
#[test]
fn typescript_colours_match_shiki() {
    check_colours(Language::TypeScript, "typescript");
}

#[cfg(feature = "json")]
#[test]
fn shell_colours_match_shiki() {
    check_colours(Language::Shell, "shellscript");
}

#[cfg(feature = "json")]
#[test]
fn json_colours_match_shiki() {
    check_colours(Language::Json, "json");
}

#[cfg(feature = "json")]
#[test]
fn jsonc_colours_match_shiki() {
    check_colours(Language::Jsonc, "jsonc");
}

#[cfg(feature = "json")]
#[test]
fn jsonl_colours_match_shiki() {
    check_colours(Language::Jsonl, "jsonl");
}

#[cfg(feature = "json")]
#[test]
fn python_colours_match_shiki() {
    check_colours(Language::Python, "python");
}

#[cfg(feature = "json")]
#[test]
fn yaml_colours_match_shiki() {
    check_colours(Language::Yaml, "yaml");
}

#[cfg(feature = "json")]
#[test]
fn toml_colours_match_shiki() {
    check_colours(Language::Toml, "toml");
}
