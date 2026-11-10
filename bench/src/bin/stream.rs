//! What one delta costs while a document streams in (openspec streaming,
//! design decision 10).
//!
//! Each trace of `bench/traces/` is replayed `--passes` times (11 by default),
//! each pass into a fresh `Document`, with every delta timed on its own.
//!
//! - A delta's time is what a renderer pays:
//!   - the push or the replacement;
//!   - `lex` to the end, which styles what it lexes with Shiki's github-dark;
//!   - reading every styled token of the lines `take_changes` names.
//! - Setup is `Document::new`, its median over the passes.
//! - Pass 1's first delta is first contact, while lazy tables are still built.
//! - For the other passes, each delta's median is its steady cost. p50, p99 and
//!   the maximum are taken over the deltas.
//! - A tool-write trace also times its whole file pushed as one delta, the
//!   median over the passes.
//! - An edit trace loads its first text before timing; each replacement is a
//!   delta.
//! - After each pass the document is finished and lexed. Its lines must equal
//!   lexing the text line by line, which the oracle holds to Shiki. The `exact`
//!   column shows whether they do, and the program fails when they do not.
//! - `fidelity` is the share of the final text's non-blank bytes coloured as
//!   Shiki colours them, from the references `bun bench/compare/shiki.ts
//!   stream-reference` writes. It reads `n/a` when they are missing.
//!
//! Usage: `stream [--passes N] [filter...]`; a filter keeps the traces whose
//! name holds it. `mise run bench:stream` runs it in release mode.

use std::hint::black_box;
use std::process::ExitCode;
use std::time::{Duration, Instant};

use nishiki::{Document, Highlighter, State, Theme, Until};
use nishiki_bench::GITHUB_DARK;
use nishiki_bench::timing::{micros, rank, timer_overhead};
use nishiki_bench::traces::{Origin, Step, Trace, traces};

/// Passes when `--passes` is not given.
const PASSES: usize = 11;

/// The options the command line sets.
struct Options {
    passes: usize,
    filters: Vec<String>,
}

fn options() -> Result<Options, String> {
    let mut options = Options {
        passes: PASSES,
        filters: Vec::new(),
    };
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--passes" {
            let value = args.next().ok_or("--passes needs a number")?;
            options.passes = value
                .parse()
                .ok()
                .filter(|&passes| passes >= 2)
                .ok_or("--passes needs a number of at least 2")?;
        } else {
            options.filters.push(arg);
        }
    }
    Ok(options)
}

/// One pass over a trace.
///
/// It holds the setup, each delta's time, whether the final lines equal
/// lexing the text line by line, and their fidelity to Shiki.
struct Pass {
    setup: Duration,
    deltas: Vec<Duration>,
    exact: bool,
    fidelity: Option<f64>,
}

/// Shiki's colours for a trace's final text: per line, each span's byte end
/// and colour.
type Colours = Vec<Vec<(usize, String)>>;

/// The references `shiki.ts stream-reference` wrote for `name`, if any.
fn reference(name: &str) -> Option<Colours> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../target/compare/shiki/traces")
        .join(format!("{name}.colors"));
    let text = std::fs::read_to_string(path).ok()?;
    Some(
        text.lines()
            .map(|row| {
                row.split(' ')
                    .filter_map(|span| {
                        let (end, colour) = span.split_once(':')?;
                        Some((end.parse().ok()?, colour.to_ascii_lowercase()))
                    })
                    .collect()
            })
            .collect(),
    )
}

/// The colour of each byte of a line of `length` bytes, from its spans.
fn per_byte(spans: &[(usize, String)], length: usize) -> Vec<&str> {
    let mut colours = vec![""; length];
    let mut start = 0;
    for (end, colour) in spans {
        let end = (*end).min(length);
        for slot in colours.iter_mut().take(end).skip(start) {
            *slot = colour;
        }
        start = start.max(end);
    }
    colours
}

/// The share of `document`'s non-blank bytes that `theme` colours as Shiki
/// does, in percent.
fn fidelity(document: &Document, theme: &Theme, shiki: &Colours) -> f64 {
    let mut same = 0_u32;
    let mut total = 0_u32;
    for index in 0..document.line_count() {
        let Some(line) = document.line(index) else {
            continue;
        };
        let ours: Vec<(usize, String)> = line
            .tokens()
            .iter()
            .map(|token| {
                let style = theme.style(document.highlighter().scopes(token.stack()));
                (
                    token.end(),
                    style.foreground.to_string().to_ascii_lowercase(),
                )
            })
            .collect();
        let text = line.text().as_bytes();
        let want = per_byte(shiki.get(index).map_or(&[][..], Vec::as_slice), text.len());
        let have = per_byte(&ours, text.len());
        for (at, byte) in text.iter().enumerate() {
            if *byte == b' ' || *byte == b'\t' {
                continue;
            }
            total += 1;
            if want[at] == have[at] {
                same += 1;
            }
        }
    }
    if total == 0 {
        100.0
    } else {
        f64::from(same) * 100.0 / f64::from(total)
    }
}

/// Reads every styled token of the lines that changed since the last call,
/// as a renderer would.
fn style_changes(document: &mut Document) {
    let Some(changes) = document.take_changes() else {
        return;
    };
    for index in changes.start..changes.new_end {
        if let Some(line) = document.line(index) {
            for styled in line.styled() {
                black_box(styled);
            }
        }
    }
}

/// Whether every line of `document` has the tokens lexing its text line by
/// line gives, scope names compared.
fn exact(document: &Document) -> bool {
    let mut highlighter = Highlighter::new(document.highlighter().language());
    let mut state = State::default();
    let mut tokens = Vec::new();
    let lines: Vec<&str> = document
        .text()
        .split('\n')
        .map(|line| line.strip_suffix('\r').unwrap_or(line))
        .collect();
    lines.len() == document.line_count()
        && lines.iter().enumerate().all(|(index, text)| {
            tokens.clear();
            let _ = highlighter.line(text, &mut state, &mut tokens);
            let Some(line) = document.line(index) else {
                return false;
            };
            line.tokens().len() == tokens.len()
                && line.tokens().iter().zip(&tokens).all(|(ours, theirs)| {
                    ours.end() == theirs.end()
                        && document
                            .highlighter()
                            .scopes(ours.stack())
                            .eq(highlighter.scopes(theirs.stack()))
                })
        })
}

/// Replays `trace` once into a fresh document.
fn replay(trace: &Trace, theme: &Theme, overhead: Duration, shiki: Option<&Colours>) -> Pass {
    let owned = theme.clone();
    let start = Instant::now();
    let mut document = Document::new(trace.language, owned);
    let setup = start.elapsed().saturating_sub(overhead);
    if trace.origin == Origin::Edits {
        document.push_str(&trace.text);
        document.finish();
        document.lex(Until::End);
        let _ = document.take_changes();
    }
    let mut deltas = Vec::with_capacity(trace.steps.len());
    for step in &trace.steps {
        let start = Instant::now();
        match step {
            Step::Push(range) => {
                document.push_str(trace.text.get(range.clone()).unwrap_or_default());
            }
            Step::Replace(range, text) => {
                if document.replace(range.clone(), text).is_err() {
                    return Pass {
                        setup,
                        deltas,
                        exact: false,
                        fidelity: None,
                    };
                }
            }
        }
        document.lex(Until::End);
        style_changes(&mut document);
        deltas.push(start.elapsed().saturating_sub(overhead));
    }
    document.finish();
    document.lex(Until::End);
    Pass {
        setup,
        deltas,
        exact: exact(&document),
        fidelity: shiki.map(|shiki| fidelity(&document, theme, shiki)),
    }
}

/// The whole text of an append trace pushed as one delta, lexed and styled.
fn whole(trace: &Trace, theme: &Theme, overhead: Duration) -> Duration {
    let mut document = Document::new(trace.language, theme.clone());
    let start = Instant::now();
    document.push_str(&trace.text);
    document.lex(Until::End);
    style_changes(&mut document);
    start.elapsed().saturating_sub(overhead)
}

fn median(mut values: Vec<Duration>) -> Duration {
    values.sort_unstable();
    values.get(values.len() / 2).copied().unwrap_or_default()
}

/// Prints the timer's figures and the head of the table of traces.
fn print_head(options: &Options, overhead: Duration, resolution: Duration) {
    println!(
        "timer overhead {:.1} ns, subtracted; below {:.3} µs is below resolution; {} passes, the first apart",
        overhead.as_secs_f64() * 1e9,
        resolution.as_secs_f64() * 1e6,
        options.passes
    );
    println!();
    println!(
        "| trace | deltas | setup µs | first delta µs | steady p50 µs | steady p99 µs | steady max µs | whole file µs | exact | fidelity |"
    );
    println!("|:--|--:|--:|--:|--:|--:|--:|--:|:--|--:|");
}

fn main() -> ExitCode {
    let options = match options() {
        Ok(options) => options,
        Err(message) => {
            eprintln!("{message}");
            return ExitCode::FAILURE;
        }
    };
    let traces = match traces() {
        Ok(traces) => traces,
        Err(error) => {
            eprintln!("cannot read bench/traces: {error}");
            return ExitCode::FAILURE;
        }
    };
    let theme = match Theme::from_json(GITHUB_DARK) {
        Ok(theme) => theme,
        Err(error) => {
            eprintln!("cannot read github-dark: {error}");
            return ExitCode::FAILURE;
        }
    };
    let overhead = timer_overhead();
    let resolution = overhead * 4;
    print_head(&options, overhead, resolution);
    let mut all_exact = true;
    for trace in traces.iter().filter(|trace| {
        options.filters.is_empty()
            || options
                .filters
                .iter()
                .any(|filter| trace.name.contains(filter.as_str()))
    }) {
        let shiki = reference(&trace.name);
        let passes: Vec<Pass> = (0..options.passes)
            .map(|_| replay(trace, &theme, overhead, shiki.as_ref()))
            .collect();
        let score = passes
            .first()
            .and_then(|pass| pass.fidelity)
            .map_or_else(|| "n/a".to_owned(), |score| format!("{score:.2}"));
        let exact = passes.iter().all(|pass| pass.exact);
        all_exact &= exact;
        let first = passes
            .first()
            .and_then(|pass| pass.deltas.first().copied())
            .unwrap_or_default();
        let count = passes.first().map_or(0, |pass| pass.deltas.len());
        let mut steady: Vec<Duration> = (0..count)
            .map(|delta| {
                median(
                    passes[1..]
                        .iter()
                        .filter_map(|pass| pass.deltas.get(delta).copied())
                        .collect(),
                )
            })
            .collect();
        steady.sort_unstable();
        let setup = median(passes.iter().map(|pass| pass.setup).collect());
        let whole_file = match trace.origin {
            Origin::ToolWrite => micros(
                median(
                    (0..options.passes)
                        .map(|_| whole(trace, &theme, overhead))
                        .collect(),
                ),
                resolution,
            ),
            Origin::Reply | Origin::Edits => String::new(),
        };
        println!(
            "| {} | {count} | {} | {} | {} | {} | {} | {whole_file} | {} | {score} |",
            trace.name,
            micros(setup, resolution),
            micros(first, resolution),
            micros(rank(&steady, 500), resolution),
            micros(rank(&steady, 990), resolution),
            micros(steady.last().copied().unwrap_or_default(), resolution),
            if exact { "yes" } else { "no" },
        );
    }
    if all_exact {
        ExitCode::SUCCESS
    } else {
        eprintln!("a document's final lines differ from lexing its text line by line");
        ExitCode::FAILURE
    }
}
