//! What one delta costs while a document streams in (openspec streaming,
//! design decision 10).
//!
//! Each trace of `bench/traces/` is replayed `--passes` times (11 by default),
//! each pass into a fresh `Document`, with every delta timed on its own.
//!
//! - A delta's time is what a renderer pays: the push or the replacement, `lex`
//!   to the end, and styling every token of the lines `take_changes` names,
//!   with Shiki's github-dark.
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
//!   column says so, and the program fails otherwise.
//!
//! Usage: `stream [--passes N] [filter...]`; a filter keeps the traces whose
//! name holds it. `mise run bench:stream` runs it in release mode.

use std::hint::black_box;
use std::process::ExitCode;
use std::time::{Duration, Instant};

use nishiki::{Document, Highlighter, State, Theme, Until};
use nishiki_bench::GITHUB_DARK;
use nishiki_bench::timing::{micros, rank, timer_overhead};
use nishiki_bench::traces::{Kind, Step, Trace, traces};

/// Passes when `--passes` is not given.
const PASSES: usize = 11;

/// What the command line asks for.
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

/// One pass over a trace: setup, each delta's time, and whether the final
/// lines equal lexing the text line by line.
struct Pass {
    setup: Duration,
    deltas: Vec<Duration>,
    exact: bool,
}

/// Styles every token of the lines that changed since the last call, as a
/// renderer would.
fn style_changes(document: &mut Document, theme: &Theme) {
    let Some(changes) = document.take_changes() else {
        return;
    };
    for index in changes.start..changes.new_end {
        if let Some(line) = document.line(index) {
            for token in line.tokens() {
                black_box(theme.style(document.highlighter().scopes(token.stack())));
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
fn replay(trace: &Trace, theme: &Theme, overhead: Duration) -> Pass {
    let owned = theme.clone();
    let start = Instant::now();
    let mut document = Document::new(trace.language, owned);
    let setup = start.elapsed().saturating_sub(overhead);
    if trace.kind == Kind::Edits {
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
                    };
                }
            }
        }
        document.lex(Until::End);
        style_changes(&mut document, theme);
        deltas.push(start.elapsed().saturating_sub(overhead));
    }
    document.finish();
    document.lex(Until::End);
    Pass {
        setup,
        deltas,
        exact: exact(&document),
    }
}

/// The whole text of an append trace pushed as one delta, lexed and styled.
fn whole(trace: &Trace, theme: &Theme, overhead: Duration) -> Duration {
    let mut document = Document::new(trace.language, theme.clone());
    let start = Instant::now();
    document.push_str(&trace.text);
    document.lex(Until::End);
    style_changes(&mut document, theme);
    start.elapsed().saturating_sub(overhead)
}

fn median(mut values: Vec<Duration>) -> Duration {
    values.sort_unstable();
    values.get(values.len() / 2).copied().unwrap_or_default()
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
    println!(
        "timer overhead {:.1} ns, subtracted; below {:.3} µs is below resolution; {} passes, the first apart",
        overhead.as_secs_f64() * 1e9,
        resolution.as_secs_f64() * 1e6,
        options.passes
    );
    println!();
    println!(
        "| trace | deltas | setup µs | first delta µs | steady p50 µs | steady p99 µs | steady max µs | whole file µs | exact |"
    );
    println!("|:--|--:|--:|--:|--:|--:|--:|--:|:--|");
    let mut all_exact = true;
    for trace in traces.iter().filter(|trace| {
        options.filters.is_empty()
            || options
                .filters
                .iter()
                .any(|filter| trace.name.contains(filter.as_str()))
    }) {
        let passes: Vec<Pass> = (0..options.passes)
            .map(|_| replay(trace, &theme, overhead))
            .collect();
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
        let whole_file = match trace.kind {
            Kind::ToolWrite => micros(
                median(
                    (0..options.passes)
                        .map(|_| whole(trace, &theme, overhead))
                        .collect(),
                ),
                resolution,
            ),
            Kind::Reply | Kind::Edits => String::new(),
        };
        println!(
            "| {} | {count} | {} | {} | {} | {} | {} | {whole_file} | {} |",
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
