//! Q3: what one streamed line costs (openspec bench-methodology, design
//! decision 6).
//!
//! Each document of the corpus is lexed `--passes` times (11 by default),
//! each pass from a fresh highlighter, with every line timed on its own.
//!
//! - The timer's own overhead is measured first and subtracted from every time.
//! - Pass 1 is first contact, while lazy structures such as the `TextMate`
//!   engine's lazy DFA are still being built; its worst line is reported apart.
//! - For the other passes, each line's median is its steady cost. p50, p99 and
//!   the maximum are taken over lines, per language and tier.
//! - A time under four times the timer's overhead is below resolution and
//!   printed as `<` that bound, not as a number.
//!
//! Usage: `latency [--passes N] [--tier small|medium|large|reply] [--tsv]
//! [language...]`; `mise run bench:latency` runs it in release mode.
//! `--tsv` prints tab-separated rows in nanoseconds instead of the table,
//! for `bench/record.ts`.

use std::hint::black_box;
use std::process::ExitCode;
use std::time::{Duration, Instant};

use nishiki::{Highlighter, Language, State};
use nishiki_bench::report::{Align, table_head};
use nishiki_bench::timing::{micros, rank, timer_overhead};
use nishiki_bench::{Document, Tier, corpus};

/// Passes when `--passes` is not given.
const PASSES: usize = 11;

/// The options the command line sets.
struct Options {
    passes: usize,
    tier: Option<Tier>,
    tsv: bool,
    languages: Vec<Language>,
}

fn options() -> Result<Options, String> {
    let mut options = Options {
        passes: PASSES,
        tier: None,
        tsv: false,
        languages: Vec::new(),
    };
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--passes" => {
                let value = args.next().ok_or("--passes needs a number")?;
                options.passes = value
                    .parse()
                    .ok()
                    .filter(|&passes| passes >= 2)
                    .ok_or("--passes needs a number of at least 2")?;
            }
            "--tier" => {
                let value = args.next().ok_or("--tier needs a tier")?;
                let tier = Tier::ALL
                    .into_iter()
                    .find(|tier| tier.name() == value)
                    .ok_or_else(|| format!("unknown tier {value:?}"))?;
                options.tier = Some(tier);
            }
            "--tsv" => options.tsv = true,
            name => {
                let language = Language::from_name(name)
                    .ok_or_else(|| format!("unknown language {name:?}"))?;
                options.languages.push(language);
            }
        }
    }
    Ok(options)
}

/// Each line's time in each pass, overhead subtracted: `times[pass][line]`.
fn passes(document: &Document, passes: usize, overhead: Duration) -> Vec<Vec<Duration>> {
    let lines = document.lines();
    let mut tokens = Vec::new();
    (0..passes)
        .map(|_| {
            let mut highlighter = Highlighter::new(document.language);
            let mut state = State::default();
            lines
                .iter()
                .map(|&line| {
                    tokens.clear();
                    let start = Instant::now();
                    let report = highlighter.line(line, &mut state, &mut tokens);
                    let elapsed = start.elapsed();
                    black_box(&report);
                    elapsed.saturating_sub(overhead)
                })
                .collect()
        })
        .collect()
}

/// Prints the head of the TSV rows, or the timer's figures and the head of the
/// table.
fn print_head(options: &Options, overhead: Duration, resolution: Duration) {
    if options.tsv {
        println!(
            "{}",
            [
                "language",
                "tier",
                "lines",
                "below_resolution",
                "steady_p50_ns",
                "steady_p99_ns",
                "steady_max_ns",
                "first_contact_max_ns",
                "resolution_ns",
            ]
            .join("\t")
        );
    } else {
        print!(
            "timer overhead {:.1} ns, subtracted; ",
            overhead.as_secs_f64() * 1e9
        );
        print!(
            "below {:.3} µs is below resolution; ",
            resolution.as_secs_f64() * 1e6
        );
        println!("{} passes, the first apart", options.passes);
        println!();
        println!(
            "{}",
            table_head(&[
                ("language", Align::Left),
                ("tier", Align::Left),
                ("lines", Align::Right),
                ("below resolution", Align::Right),
                ("steady p50 µs", Align::Right),
                ("steady p99 µs", Align::Right),
                ("steady max µs", Align::Right),
                ("first-contact max µs", Align::Right),
            ])
        );
    }
}

fn main() -> ExitCode {
    let options = match options() {
        Ok(options) => options,
        Err(message) => {
            eprintln!("{message}");
            return ExitCode::FAILURE;
        }
    };
    let documents = match corpus() {
        Ok(documents) => documents,
        Err(error) => {
            eprintln!("cannot read bench/corpus: {error}");
            return ExitCode::FAILURE;
        }
    };
    let overhead = timer_overhead();
    let resolution = overhead * 4;
    print_head(&options, overhead, resolution);

    let selected = documents.iter().filter(|document| {
        options.tier.is_none_or(|tier| document.tier == tier)
            && (options.languages.is_empty() || options.languages.contains(&document.language))
    });
    let mut groups: Vec<(Language, Tier, Vec<Duration>, Duration)> = Vec::new();
    for document in selected {
        let times = passes(document, options.passes, overhead);
        let lines = times.first().map_or(0, Vec::len);
        let first_max = times
            .first()
            .and_then(|first| first.iter().max().copied())
            .unwrap_or_default();
        let steady: Vec<Duration> = (0..lines)
            .map(|line| {
                let mut each: Vec<Duration> = times[1..].iter().map(|pass| pass[line]).collect();
                each.sort_unstable();
                each[each.len() / 2]
            })
            .collect();
        match groups
            .iter_mut()
            .find(|(language, tier, ..)| *language == document.language && *tier == document.tier)
        {
            Some((_, _, all, worst)) => {
                all.extend(steady);
                *worst = (*worst).max(first_max);
            }
            None => groups.push((document.language, document.tier, steady, first_max)),
        }
    }
    for (language, tier, mut steady, first_max) in groups {
        steady.sort_unstable();
        let below = steady.iter().filter(|&&time| time < resolution).count();
        let p50 = rank(&steady, 500);
        let p99 = rank(&steady, 990);
        let max = steady.last().copied().unwrap_or_default();
        if options.tsv {
            println!(
                "{}\t{tier}\t{}\t{below}\t{}\t{}\t{}\t{}\t{}",
                language.id(),
                steady.len(),
                p50.as_nanos(),
                p99.as_nanos(),
                max.as_nanos(),
                first_max.as_nanos(),
                resolution.as_nanos(),
            );
        } else {
            println!(
                "| {} | {tier} | {} | {below} | {} | {} | {} | {} |",
                language.id(),
                steady.len(),
                micros(p50, resolution),
                micros(p99, resolution),
                micros(max, resolution),
                micros(first_max, resolution),
            );
        }
    }
    ExitCode::SUCCESS
}
