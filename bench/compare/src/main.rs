//! Compares nishiki with other highlighters on the benchmark corpus
//! (openspec bench-methodology, design decision 13; bench/compare/README.md).
//!
//! For each file and engine it reports:
//!
//! - setup: a fresh process loading the engine's grammars and theme;
//! - first document: the same fresh process colouring the file once;
//! - warm throughput: the median over rounds of colouring the file again and
//!   again, the engine order rotating every round;
//! - tokens: what the engine made, since fewer tokens is less work;
//! - fidelity: the share of non-whitespace bytes whose colour matches Shiki's,
//!   from `bun bench/compare/shiki.ts reference`.
//!
//! Usage: `compare [--rounds N] [--cold N] [--tier T] [--tsv] [language...]`,
//! by default 5 rounds, 11 cold processes, the medium tier of every language.
//! `--tsv` prints one tab-separated row per file and engine, with every
//! cold sample and every round, for `bench/compare/record.ts`.
//! A cold sample is `compare --sample <engine> <file>`.
//! `compare --pierre-theme <path>` writes github-dark as a Zed theme for
//! `@pierre/highlights`, which `shiki.ts time` reads.

mod engines;
#[cfg(feature = "giallo")]
mod giallo;

use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use std::time::{Duration, Instant};

use nishiki::{Highlighter, Language, State};
use nishiki_bench::{Document, Tier, corpus};

use crate::engines::{Colours, NAMES, lines, pierre_theme, setup};

/// Time each round spends on one engine and file.
const ROUND: Duration = Duration::from_millis(300);

/// How the runner prints its rows.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Output {
    /// A Markdown table of medians, for a reader.
    Table,
    /// Tab-separated rows with every sample, for `bench/compare/record.ts`.
    Tsv,
}

/// What the runner measures, from the command line.
struct Options {
    rounds: usize,
    cold: usize,
    tier: Tier,
    output: Output,
    languages: Vec<Language>,
}

fn options(args: &[String]) -> Result<Options, String> {
    let mut options = Options {
        rounds: 5,
        cold: 11,
        tier: Tier::Medium,
        output: Output::Table,
        languages: Vec::new(),
    };
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        let mut number = |what: &str, low: usize| {
            args.next()
                .and_then(|value| value.parse::<usize>().ok())
                .filter(|&n| n >= low)
                .ok_or_else(|| format!("{what} needs a number of at least {low}"))
        };
        match arg.as_str() {
            "--rounds" => options.rounds = number("--rounds", 5)?,
            "--cold" => options.cold = number("--cold", 3)?,
            "--tsv" => options.output = Output::Tsv,
            "--tier" => {
                let value = args.next().ok_or("--tier needs a tier")?;
                options.tier = Tier::ALL
                    .into_iter()
                    .find(|tier| tier.name() == value)
                    .ok_or_else(|| format!("unknown tier {value:?}"))?;
            }
            name => options.languages.push(
                Language::from_name(name).ok_or_else(|| format!("unknown language {name:?}"))?,
            ),
        }
    }
    Ok(options)
}

fn corpus_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../corpus")
}

fn extension(name: &str) -> &str {
    name.rsplit_once('.').map_or("", |(_, extension)| extension)
}

/// One cold sample: set up `engine` for the file's language and colour the
/// file once, printing both times in ms.
fn sample(engine: &str, name: &str) -> Result<(), String> {
    let language = name
        .split('/')
        .next()
        .and_then(Language::from_name)
        .ok_or_else(|| format!("{name} is not a corpus path"))?;
    let text = std::fs::read_to_string(corpus_root().join(name))
        .map_err(|error| format!("{name}: {error}"))?;
    let start = Instant::now();
    let mut ready = setup(engine, language, extension(name))
        .ok_or_else(|| format!("{engine} has no {name}"))?;
    let set_up = start.elapsed();
    ready.run(&text);
    let first = start.elapsed().saturating_sub(set_up);
    println!(
        "{} {}",
        set_up.as_secs_f64() * 1e3,
        first.as_secs_f64() * 1e3
    );
    Ok(())
}

/// Runs this program for one cold sample; returns setup and first ms.
fn cold_sample(engine: &str, name: &str) -> Result<(f64, f64), String> {
    let exe = std::env::current_exe().map_err(|error| error.to_string())?;
    let output = Command::new(exe)
        .args(["--sample", engine, name])
        .output()
        .map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into_owned());
    }
    let text = String::from_utf8_lossy(&output.stdout);
    let mut numbers = text.split_whitespace().map(str::parse::<f64>);
    match (numbers.next(), numbers.next()) {
        (Some(Ok(setup)), Some(Ok(first))) => Ok((setup, first)),
        _ => Err(format!("unreadable sample: {text:?}")),
    }
}

fn median(values: &mut [f64]) -> f64 {
    values.sort_by(f64::total_cmp);
    values.get(values.len() / 2).copied().unwrap_or(f64::NAN)
}

/// Shiki's colours for `name`, written by `shiki.ts reference`.
fn reference(name: &str) -> Option<Colours> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(format!("../../target/compare/shiki/{name}.colors"));
    let text = std::fs::read_to_string(path).ok()?;
    let colours = text
        .lines()
        .map(|row| {
            row.split_whitespace()
                .filter_map(|span| {
                    let (end, colour) = span.split_once(':')?;
                    Some((end.parse().ok()?, colour.to_lowercase()))
                })
                .collect()
        })
        .collect();
    Some(colours)
}

/// The colour of each byte of a line, from its spans.
fn per_byte(spans: &[(usize, String)], len: usize) -> Vec<&str> {
    let mut colours = vec![""; len];
    let mut start = 0;
    for (end, colour) in spans {
        let end = (*end).min(len);
        for slot in colours.get_mut(start..end).into_iter().flatten() {
            *slot = colour;
        }
        start = end.max(start);
    }
    colours
}

/// The share of `text`'s non-whitespace bytes that `colours` gives Shiki's
/// colour, in percent.
fn fidelity(text: &str, colours: &Colours, shiki: &Colours) -> f64 {
    let (mut same, mut total) = (0_u64, 0_u64);
    for (index, line) in lines(text).into_iter().enumerate() {
        let theirs = per_byte(shiki.get(index).map_or(&[], Vec::as_slice), line.len());
        let ours = per_byte(colours.get(index).map_or(&[], Vec::as_slice), line.len());
        for (at, byte) in line.bytes().enumerate() {
            if byte == b' ' || byte == b'\t' {
                continue;
            }
            total += 1;
            if theirs[at] == ours[at] {
                same += 1;
            }
        }
    }
    if total == 0 {
        return 100.0;
    }
    f64::from(u32::try_from(same).unwrap_or(u32::MAX)) * 100.0
        / f64::from(u32::try_from(total).unwrap_or(u32::MAX))
}

/// Whether nishiki lexes every line of `document` in full under its
/// default limits (fairness rule 4: every engine does the full work).
fn nishiki_complete(document: &Document) -> bool {
    let mut highlighter = Highlighter::new(document.language);
    let mut state = State::default();
    let mut tokens = Vec::new();
    document.lines().into_iter().all(|line| {
        tokens.clear();
        highlighter
            .line(line, &mut state, &mut tokens)
            .is_complete()
    })
}

/// One engine's row for one file: every cold sample and every round.
struct Row {
    engine: &'static str,
    setup_ms: Vec<f64>,
    first_ms: Vec<f64>,
    mib_per_s: Vec<f64>,
    tokens: usize,
    fidelity: Option<f64>,
}

impl Row {
    /// The row as the Markdown table prints it: medians.
    fn table_line(&self, file: &str) -> String {
        format!(
            "| {file} | {} | {:.2} | {:.2} | {:.1} | {} | {} |",
            self.engine,
            median(&mut self.setup_ms.clone()),
            median(&mut self.first_ms.clone()),
            median(&mut self.mib_per_s.clone()),
            self.tokens,
            self.fidelity
                .map_or_else(|| "n/a".to_owned(), |share| format!("{share:.2}")),
        )
    }

    /// The row as `--tsv` prints it: each list of samples joined by commas.
    fn tsv_line(&self, file: &str) -> String {
        let list = |values: &[f64]| {
            values
                .iter()
                .map(f64::to_string)
                .collect::<Vec<_>>()
                .join(",")
        };
        format!(
            "{file}\t{}\t{}\t{}\t{}\t{}\t{}",
            self.engine,
            list(&self.setup_ms),
            list(&self.first_ms),
            list(&self.mib_per_s),
            self.tokens,
            self.fidelity
                .map_or_else(|| "n/a".to_owned(), |share| share.to_string()),
        )
    }
}

fn runner(options: &Options) -> Result<(), String> {
    let documents = corpus().map_err(|error| format!("cannot read bench/corpus: {error}"))?;
    let selected: Vec<&Document> = documents
        .iter()
        .filter(|document| {
            document.tier == options.tier
                && (options.languages.is_empty() || options.languages.contains(&document.language))
        })
        .collect();
    match options.output {
        Output::Table => {
            println!(
                "{} rounds of {} ms per engine and file, the order rotating; {} fresh processes for setup and first document; colours against Shiki 4.6.0, github-dark",
                options.rounds,
                ROUND.as_millis(),
                options.cold
            );
            println!();
            println!(
                "| file | engine | setup ms | first document ms | warm MiB/s | tokens | fidelity % |"
            );
            println!("|:--|:--|--:|--:|--:|--:|--:|");
        }
        Output::Tsv => println!("file\tengine\tsetup_ms\tfirst_ms\tmib_per_s\ttokens\tfidelity"),
    }
    for document in selected {
        if !nishiki_complete(document) {
            return Err(format!(
                "{}: nishiki reached a limit, so it did less work than the others",
                document.name
            ));
        }
        let text = std::fs::read_to_string(corpus_root().join(&document.name))
            .map_err(|error| error.to_string())?;
        let bytes = f64::from(u32::try_from(text.len()).unwrap_or(u32::MAX));
        let shiki = reference(&document.name);
        let mut ready: Vec<(&'static str, Box<dyn engines::Engine>)> = NAMES
            .iter()
            .filter_map(|&name| {
                Some((
                    name,
                    setup(name, document.language, extension(&document.name))?,
                ))
            })
            .collect();
        let mut speeds: Vec<Vec<f64>> = ready.iter().map(|_| Vec::new()).collect();
        for (_, engine) in &mut ready {
            engine.run(&text);
        }
        for round in 0..options.rounds {
            for offset in 0..ready.len() {
                let index = (offset + round) % ready.len();
                let engine = &mut ready[index].1;
                let start = Instant::now();
                let mut iterations = 0_u32;
                while start.elapsed() < ROUND {
                    engine.run(&text);
                    iterations += 1;
                }
                let seconds = start.elapsed().as_secs_f64();
                speeds[index]
                    .push(bytes * f64::from(iterations) / seconds / f64::from(1_u32 << 20));
            }
        }
        let mut rows = Vec::new();
        for ((engine, ready), speed) in ready.iter_mut().zip(speeds) {
            let mut setups = Vec::new();
            let mut firsts = Vec::new();
            for _ in 0..options.cold {
                let (setup_ms, first_ms) = cold_sample(engine, &document.name)?;
                setups.push(setup_ms);
                firsts.push(first_ms);
            }
            let tokens = ready.run(&text);
            let fidelity = shiki
                .as_ref()
                .map(|shiki| fidelity(&text, &ready.colours(&text), shiki));
            rows.push(Row {
                engine,
                setup_ms: setups,
                first_ms: firsts,
                mib_per_s: speed,
                tokens,
                fidelity,
            });
        }
        for row in rows {
            let line = match options.output {
                Output::Table => row.table_line(&document.name),
                Output::Tsv => row.tsv_line(&document.name),
            };
            println!("{line}");
        }
    }
    Ok(())
}

/// Writes [`pierre_theme`] to `path`.
fn write_pierre_theme(path: &str) -> Result<(), String> {
    let json = pierre_theme().ok_or("github-dark did not load")?;
    if let Some(parent) = Path::new(path).parent() {
        std::fs::create_dir_all(parent).map_err(|error| format!("{path}: {error}"))?;
    }
    std::fs::write(path, json).map_err(|error| format!("{path}: {error}"))
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = match args.as_slice() {
        [flag, engine, name] if flag == "--sample" => sample(engine, name),
        [flag, path] if flag == "--pierre-theme" => write_pierre_theme(path),
        _ => options(&args).and_then(|options| runner(&options)),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}
