//! Measures nishiki on the oracle fixtures: throughput, streaming latency per
//! line, cold start, and theme styling.
//!
//! Run with `mise run bench`, in release mode. Each fixture's text is lexed
//! line by line from a fresh highlighter, as an editor or a streamed reply
//! lexes it. The figures are this machine's; compare runs on one machine.
//!
//! Language names as arguments, `mise run bench python java`, measure only
//! those languages' fixtures, so a change to one language is compared in
//! seconds; the theme figure then comes from their largest fixture.

#![expect(
    clippy::print_stdout,
    clippy::print_stderr,
    reason = "a benchmark prints its results and its usage errors"
)]

use std::hint::black_box;
use std::path::Path;
use std::process::ExitCode;
use std::time::{Duration, Instant};
use std::{env, fs};

use nishiki::{Highlighter, Language, State, Theme, ThemeSource};

/// Whole passes over a document for the throughput figure.
const PASSES: u32 = 20;

/// One fixture's source lines.
struct Document {
    name: String,
    language: Language,
    lines: Vec<String>,
}

impl Document {
    fn bytes(&self) -> usize {
        self.lines.iter().map(|line| line.len() + 1).sum()
    }
}

fn documents(root: &Path) -> Vec<Document> {
    let mut documents = Vec::new();
    let Ok(dirs) = fs::read_dir(root) else {
        return documents;
    };
    let mut dirs: Vec<_> = dirs
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect();
    dirs.sort();
    for dir in dirs {
        let Some(language) = dir
            .file_name()
            .and_then(|name| name.to_str())
            .and_then(Language::from_name)
        else {
            continue;
        };
        let Ok(files) = fs::read_dir(&dir) else {
            continue;
        };
        let mut files: Vec<_> = files
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| path.extension().is_some_and(|ext| ext == "txt"))
            .collect();
        files.sort();
        for file in files {
            let Ok(content) = fs::read_to_string(&file) else {
                continue;
            };
            let lines = content
                .lines()
                .filter_map(|record| record.strip_prefix("L "))
                .map(ToOwned::to_owned)
                .collect();
            let name = file
                .file_stem()
                .and_then(|stem| stem.to_str())
                .unwrap_or_default();
            documents.push(Document {
                name: format!("{}/{name}", language.id()),
                language,
                lines,
            });
        }
    }
    documents
}

/// Lexes `document` once from a fresh highlighter; returns the tokens made.
fn lex(document: &Document) -> usize {
    let mut highlighter = Highlighter::new(document.language);
    let mut state = State::default();
    let mut tokens = Vec::new();
    let mut count = 0;
    for line in &document.lines {
        tokens.clear();
        let _ = highlighter.line(line, &mut state, &mut tokens);
        count += tokens.len();
    }
    black_box(count)
}

fn mib_per_second(bytes: usize, elapsed: Duration) -> f64 {
    let bytes = f64::from(u32::try_from(bytes).unwrap_or(u32::MAX));
    bytes / elapsed.as_secs_f64() / f64::from(1 << 20)
}

/// The value at `quantile` (0 to 1) of sorted `values`.
fn percentile(sorted: &[Duration], quantile: f64) -> Duration {
    let last = sorted.len().saturating_sub(1);
    let rank = (f64::from(u32::try_from(last).unwrap_or(u32::MAX)) * quantile).round();
    let index = format!("{rank:.0}")
        .parse::<usize>()
        .unwrap_or(last)
        .min(last);
    sorted.get(index).copied().unwrap_or_default()
}

fn micros(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1e6
}

fn main() -> ExitCode {
    let mut only = Vec::new();
    for name in env::args().skip(1) {
        let Some(language) = Language::from_name(&name) else {
            eprintln!("unknown language: {name}");
            return ExitCode::FAILURE;
        };
        only.push(language);
    }
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/oracle");
    let mut documents = documents(&root);
    if !only.is_empty() {
        documents.retain(|document| only.contains(&document.language));
    }
    println!(
        "{:<28} {:>8} {:>10} {:>9} {:>9} {:>9} {:>10}",
        "fixture", "KiB", "MiB/s", "p50 µs", "p99 µs", "max µs", "first µs"
    );
    for document in &documents {
        lex(document);
        let start = Instant::now();
        for _ in 0..PASSES {
            lex(document);
        }
        let throughput = mib_per_second(
            document.bytes() * usize::try_from(PASSES).unwrap_or(1),
            start.elapsed(),
        );

        let mut highlighter = Highlighter::new(document.language);
        let mut state = State::default();
        let mut tokens = Vec::new();
        let mut times = Vec::with_capacity(document.lines.len());
        for line in &document.lines {
            tokens.clear();
            let start = Instant::now();
            let _ = highlighter.line(line, &mut state, &mut tokens);
            times.push(start.elapsed());
        }
        times.sort();

        let cold = Instant::now();
        let mut fresh = Highlighter::new(document.language);
        tokens.clear();
        let first = document.lines.first().map_or("", String::as_str);
        let _ = fresh.line(first, &mut State::default(), &mut tokens);
        let first_line = cold.elapsed();

        println!(
            "{:<28} {:>8.1} {:>10.1} {:>9.2} {:>9.2} {:>9.1} {:>10.1}",
            document.name,
            f64::from(u32::try_from(document.bytes()).unwrap_or(u32::MAX)) / 1024.0,
            throughput,
            micros(percentile(&times, 0.5)),
            micros(percentile(&times, 0.99)),
            micros(times.last().copied().unwrap_or_default()),
            micros(first_line),
        );
    }

    let theme = Theme::new(ThemeSource::default());
    if let Some(document) = documents.iter().max_by_key(|document| document.bytes()) {
        let mut highlighter = Highlighter::new(document.language);
        let mut state = State::default();
        let mut tokens = Vec::new();
        let mut all = Vec::new();
        for line in &document.lines {
            tokens.clear();
            let _ = highlighter.line(line, &mut state, &mut tokens);
            all.extend_from_slice(&tokens);
        }
        let start = Instant::now();
        for token in &all {
            black_box(theme.style(highlighter.scopes(token.stack())));
        }
        let elapsed = start.elapsed();
        println!(
            "\ntheme styling, uncached: {} tokens of {} in {:.1} ms, {:.0} ns per token",
            all.len(),
            document.name,
            elapsed.as_secs_f64() * 1e3,
            elapsed.as_secs_f64() * 1e9
                / f64::from(u32::try_from(all.len().max(1)).unwrap_or(u32::MAX)),
        );
    }
    ExitCode::SUCCESS
}
