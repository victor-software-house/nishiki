//! Q4: how soon the first line is coloured in a fresh process (openspec
//! bench-methodology, design decision 7).
//!
//! The runner starts this program once per sample. Every sample begins with
//! empty lazy state. No pattern tables, decoded grammars or lazily built
//! automata survive from an earlier sample. The binary is in the page cache.
//! The figure is a fresh process, not a cold disk.
//!
//! - A sample creates a highlighter, lexes the document's first line, then the
//!   rest. It prints the three times, counted from just before the highlighter
//!   is created. Reading the file comes before that.
//! - A theme sample times `Theme::from_json` on Shiki's github-dark.
//! - A grammar sample times three steps on TypeScript's grammar from the
//!   corpus: `Grammar::from_json`, `Grammar::from_bytes` on the bytes
//!   `Grammar::to_bytes` wrote, and the first `Highlighter::for_grammar`, which
//!   links it. nishiki keeps every grammar and link for the rest of the
//!   process, so each runs once per process, as a caller's does.
//! - The runner starts `--processes` samples per file, theme and grammar (31 by
//!   default). It interleaves them round by round and drops the first three
//!   rounds. Then it prints the median and p90 of each time.
//!
//! Usage: `cold [--processes N] [--tier small|medium|large|reply] [--tsv]
//! [language...]`, by default the small tier of every language;
//! `mise run bench:cold` runs it in release mode. A sample is
//! `cold --sample <file>`, `cold --theme` or `cold --grammar`. `--tsv` prints
//! tab-separated rows, one per file, theme or grammar and measure, instead of
//! the table, for `bench/record.ts`.

use std::hint::black_box;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use std::time::Instant;

use nishiki::{Grammar, Grammars, Highlighter, Language, State, Theme};
use nishiki_bench::report::{Align, table_head};
use nishiki_bench::{GITHUB_DARK, Tier, corpus};

/// Samples per file when `--processes` is not given.
const PROCESSES: usize = 31;

/// Rounds dropped before any sample counts.
const DROPPED: usize = 3;

/// One sample: lexes `path` in the language its folder names.
///
/// It prints the µs to create the highlighter, to lex the first line, and to
/// lex the whole document.
fn sample(path: &Path) -> Result<(), String> {
    let language = path
        .parent()
        .and_then(Path::parent)
        .and_then(Path::file_name)
        .and_then(|name| name.to_str())
        .and_then(Language::from_name)
        .ok_or_else(|| format!("{} is not in a language folder", path.display()))?;
    let text =
        std::fs::read_to_string(path).map_err(|error| format!("{}: {error}", path.display()))?;
    let lines: Vec<&str> = text
        .lines()
        .map(|line| line.strip_suffix('\r').unwrap_or(line))
        .collect();
    let start = Instant::now();
    let mut highlighter = Highlighter::new(language);
    let created = start.elapsed();
    let mut state = State::default();
    let mut tokens = Vec::new();
    let mut first = created;
    for (index, &line) in lines.iter().enumerate() {
        tokens.clear();
        let report = highlighter.line(line, &mut state, &mut tokens);
        black_box(&report);
        if index == 0 {
            first = start.elapsed();
        }
    }
    let whole = start.elapsed();
    println!(
        "{} {} {}",
        created.as_secs_f64() * 1e6,
        first.as_secs_f64() * 1e6,
        whole.as_secs_f64() * 1e6
    );
    Ok(())
}

/// One theme sample: the µs `Theme::from_json` takes on github-dark.
fn theme_sample() -> Result<(), String> {
    let start = Instant::now();
    let theme = Theme::from_json(GITHUB_DARK).map_err(|error| error.to_string())?;
    let elapsed = start.elapsed();
    black_box(theme);
    println!("{}", elapsed.as_secs_f64() * 1e6);
    Ok(())
}

/// One grammar sample: the µs `Grammar::from_json`, `Grammar::from_bytes` and
/// the first `Highlighter::for_grammar` take on TypeScript's grammar.
///
/// Writing the bytes is not timed.
fn grammar_sample() -> Result<(), String> {
    let json = nishiki_bench::grammar(Language::TypeScript)
        .map_err(|error| format!("TypeScript's grammar: {error}"))?;
    let start = Instant::now();
    let grammar = Grammar::from_json(&json).map_err(|error| error.to_string())?;
    let compiled = start.elapsed();
    let bytes = grammar.to_bytes();
    let start = Instant::now();
    let loaded = Grammar::from_bytes(&bytes).map_err(|error| error.to_string())?;
    let read = start.elapsed();
    black_box(loaded);
    let start = Instant::now();
    let highlighter =
        Highlighter::for_grammar(grammar, &Grammars::new()).map_err(|error| error.to_string())?;
    let linked = start.elapsed();
    black_box(highlighter);
    println!(
        "{} {} {}",
        compiled.as_secs_f64() * 1e6,
        read.as_secs_f64() * 1e6,
        linked.as_secs_f64() * 1e6
    );
    Ok(())
}

/// Runs this program with `args` and returns the numbers it printed.
fn run(args: &[&std::ffi::OsStr]) -> Result<Vec<f64>, String> {
    let exe = std::env::current_exe().map_err(|error| error.to_string())?;
    let output = Command::new(exe)
        .args(args)
        .output()
        .map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into_owned());
    }
    String::from_utf8_lossy(&output.stdout)
        .split_whitespace()
        .map(|number| {
            number
                .parse()
                .map_err(|_| format!("not a number: {number:?}"))
        })
        .collect()
}

/// The value at nearest rank `percent` / 100 of `values`, which it sorts.
fn rank(values: &mut [f64], percent: usize) -> f64 {
    values.sort_by(f64::total_cmp);
    let index = (values.len() * percent).div_ceil(100).saturating_sub(1);
    values.get(index).copied().unwrap_or_default()
}

/// What the runner measures, from the command line.
struct Options {
    processes: usize,
    tier: Tier,
    tsv: bool,
    languages: Vec<Language>,
}

fn options(args: &[String]) -> Result<Options, String> {
    let mut options = Options {
        processes: PROCESSES,
        tier: Tier::Small,
        tsv: false,
        languages: Vec::new(),
    };
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--processes" => {
                options.processes = args
                    .next()
                    .and_then(|value| value.parse().ok())
                    .filter(|&processes| processes > DROPPED)
                    .ok_or_else(|| format!("--processes needs a number above {DROPPED}"))?;
            }
            "--tier" => {
                let value = args.next().ok_or("--tier needs a tier")?;
                options.tier = Tier::ALL
                    .into_iter()
                    .find(|tier| tier.name() == value)
                    .ok_or_else(|| format!("unknown tier {value:?}"))?;
            }
            "--tsv" => options.tsv = true,
            name => options.languages.push(
                Language::from_name(name).ok_or_else(|| format!("unknown language {name:?}"))?,
            ),
        }
    }
    Ok(options)
}

/// The times of every sample that counts.
struct Timings {
    /// Each file's name.
    names: Vec<String>,
    /// Per file, the times to create the highlighter, to lex the first line
    /// and to lex the document.
    times: Vec<[Vec<f64>; 3]>,
    /// The times of `Theme::from_json`.
    theme: Vec<f64>,
    /// The times of the three grammar steps.
    grammar: [Vec<f64>; 3],
}

/// Runs the samples, then prints what they measured as `--tsv` asks.
fn runner(options: &Options) -> Result<(), String> {
    let timings = measure(options)?;
    if options.tsv {
        print_tsv(timings);
    } else {
        print_table(options, timings);
    }
    Ok(())
}

/// Starts every sample, round by round, and keeps those after the dropped
/// rounds.
fn measure(options: &Options) -> Result<Timings, String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("corpus");
    let documents = corpus().map_err(|error| format!("cannot read bench/corpus: {error}"))?;
    let files: Vec<(String, PathBuf)> = documents
        .iter()
        .filter(|document| {
            document.tier == options.tier
                && (options.languages.is_empty() || options.languages.contains(&document.language))
        })
        .map(|document| (document.name.clone(), root.join(&document.name)))
        .collect();
    let mut times: Vec<[Vec<f64>; 3]> = files.iter().map(|_| Default::default()).collect();
    let mut theme = Vec::new();
    let mut grammar: [Vec<f64>; 3] = Default::default();
    for round in 0..options.processes {
        for ((_, path), times) in files.iter().zip(&mut times) {
            let numbers = run(&["--sample".as_ref(), path.as_os_str()])?;
            if round >= DROPPED {
                for (series, number) in times.iter_mut().zip(numbers) {
                    series.push(number);
                }
            }
        }
        let numbers = run(&["--theme".as_ref()])?;
        if round >= DROPPED {
            theme.extend(numbers);
        }
        let numbers = run(&["--grammar".as_ref()])?;
        if round >= DROPPED {
            for (series, number) in grammar.iter_mut().zip(numbers) {
                series.push(number);
            }
        }
    }
    Ok(Timings {
        names: files.into_iter().map(|(name, _)| name).collect(),
        times,
        theme,
        grammar,
    })
}

/// Prints the tab-separated rows `bench/record.ts` reads.
fn print_tsv(timings: Timings) {
    let Timings {
        names,
        mut times,
        mut theme,
        mut grammar,
    } = timings;
    println!("kind\tname\tmeasure\tmedian_us\tp90_us");
    for (name, [created, first, whole]) in names.iter().zip(&mut times) {
        for (step, series) in [("new", created), ("first_line", first), ("document", whole)] {
            println!(
                "file\t{name}\t{step}\t{:.1}\t{:.1}",
                rank(series, 50),
                rank(series, 90)
            );
        }
    }
    println!(
        "theme\tgithub-dark\tfrom_json\t{:.1}\t{:.1}",
        rank(&mut theme, 50),
        rank(&mut theme, 90)
    );
    for (step, series) in ["from_json", "from_bytes", "link"].iter().zip(&mut grammar) {
        println!(
            "grammar\ttypescript\t{step}\t{:.1}\t{:.1}",
            rank(series, 50),
            rank(series, 90)
        );
    }
}

/// Prints the table, with the theme and the grammar below it.
fn print_table(options: &Options, timings: Timings) {
    let Timings {
        names,
        mut times,
        mut theme,
        mut grammar,
    } = timings;
    println!(
        "{} fresh processes per file, the first {DROPPED} rounds dropped; \
         µs from just before the highlighter is created",
        options.processes
    );
    println!();
    println!(
        "{}",
        table_head(&[
            ("file", Align::Left),
            ("new, median", Align::Right),
            ("new, p90", Align::Right),
            ("first line, median", Align::Right),
            ("first line, p90", Align::Right),
            ("document, median", Align::Right),
            ("document, p90", Align::Right),
        ])
    );
    for (name, [created, first, whole]) in names.iter().zip(&mut times) {
        println!(
            "| {name} | {:.1} | {:.1} | {:.1} | {:.1} | {:.1} | {:.1} |",
            rank(created, 50),
            rank(created, 90),
            rank(first, 50),
            rank(first, 90),
            rank(whole, 50),
            rank(whole, 90),
        );
    }
    println!();
    println!(
        "github-dark from JSON: median {:.1} µs, p90 {:.1} µs",
        rank(&mut theme, 50),
        rank(&mut theme, 90)
    );
    let [compiled, read, linked] = &mut grammar;
    println!(
        "TypeScript's grammar: \
         from JSON median {:.1} µs, from bytes median {:.1} µs, first link median {:.1} µs",
        rank(compiled, 50),
        rank(read, 50),
        rank(linked, 50)
    );
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = match args.as_slice() {
        [flag, path] if flag == "--sample" => sample(Path::new(path)),
        [flag] if flag == "--theme" => theme_sample(),
        [flag] if flag == "--grammar" => grammar_sample(),
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
