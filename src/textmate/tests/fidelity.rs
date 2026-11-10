//! The Shiki fidelity check, task 3.1 of `openspec/changes/grammar-api`.
//!
//! Every language Shiki 4.6.0 bundles runs on its input: Shiki's sample for
//! it, or a stand-in where Shiki has none.
//! A few languages run on extra inputs too, which reach rules their samples
//! miss.
//! Each non-whitespace byte must get the scope stack Shiki gives it.
//! Every line is compared, with no block left out.
//!
//! Each language loads through the public API, as a caller's would:
//! [`Grammar::from_json`] on every grammar Shiki loads for it, then
//! [`Highlighter::for_grammar_with_limits`].
//!
//! `oracle/fidelity.ts` writes the directory that `NISHIKI_FIDELITY` names.
//! The check writes `report.md` there, one row per input.

use std::fmt::Write as _;
use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;

use indoc::formatdoc;
use serde_json::Value;

use super::{Comparison, Sketch, compare, parse};
use crate::{Grammar, GrammarError, Grammars, Highlighter, Limits};

/// How long one input may take before it counts as stalled.
const STALL: Duration = Duration::from_secs(120);

/// The stack of the thread that lexes an input.
/// A grammar that nests deeply recurses deeply, and an overflow aborts every
/// check.
const STACK: usize = 256 << 20;

/// A highlighter for the grammar whose root scope is `scope`, from the
/// directory `oracle/fidelity.ts` wrote.
///
/// Each of `names` is a grammar Shiki loads for the language, in
/// `grammars/<name>.json`.
/// All of them go into the set the root's includes resolve in.
/// No line limit applies, so a difference is never a limit's.
fn highlighter(
    dir: &Path,
    names: &[String],
    scope: &str,
) -> Result<Highlighter<Grammar>, GrammarError> {
    let mut grammars = Grammars::new();
    for name in names {
        let path = dir.join("grammars").join(format!("{name}.json"));
        grammars.insert(Grammar::from_json(&std::fs::read_to_string(path).unwrap())?);
    }
    let root = grammars.get(scope).unwrap();
    Highlighter::for_grammar_with_limits(root, &grammars, Limits::unlimited())
}

/// What the check found for one language.
enum Outcome {
    /// The grammar did not load or link, for the reason given.
    NoGrammar(String),
    Panicked(String),
    Stalled,
    Compared(Comparison),
}

/// The patterns the regex compiler refuses, by language.
/// Each is a gap in task 3.2 of `openspec/changes/grammar-api`, even where the
/// input still matches.
/// The list is empty.
///
/// The list only shrinks.
/// The check fails on a refusal the list lacks, and on an entry no grammar
/// refuses any more.
const KNOWN_REFUSALS: &[(&str, &str)] = &[];

/// Each pattern of the linked `table` the regex compiler refuses, with its
/// reason.
/// [`Grammar::from_json`] does not fail on such a pattern: it never matches.
fn refused(table: &super::Grammar) -> Vec<(&'static str, String)> {
    table
        .patterns
        .iter()
        .filter_map(|pattern| {
            Sketch::for_grammar(pattern.source)
                .err()
                .map(|error| (pattern.source, format!("{error:?}")))
        })
        .collect()
}

/// Loads the language of `entry`, a row of `languages.json`, and compares its
/// input on a thread of its own.
/// A panic then ends only that input's check.
/// A stall leaves the thread running, so the caller stops the run.
fn outcome(dir: &Path, entry: &Value) -> (Outcome, Vec<(&'static str, String)>) {
    let (send, receive) = mpsc::channel();
    let dir = dir.to_owned();
    let names: Vec<String> = entry["grammars"]
        .as_array()
        .unwrap()
        .iter()
        .map(|name| name.as_str().unwrap().to_owned())
        .collect();
    let scope = entry["scope"].as_str().unwrap().to_owned();
    let sample = entry["sample"].as_str().unwrap().to_owned();
    std::thread::Builder::new()
        .stack_size(STACK)
        .spawn(move || {
            let found = std::panic::catch_unwind(|| {
                let lines = parse(&dir.join("samples").join(format!("{sample}.txt")));
                match highlighter(&dir, &names, &scope) {
                    Ok(highlighter) => {
                        let refusals = refused(highlighter.lexer().table);
                        (
                            Outcome::Compared(compare(highlighter, &lines, &[])),
                            refusals,
                        )
                    }
                    Err(error) => (Outcome::NoGrammar(error.to_string()), Vec::new()),
                }
            });
            let _ = send.send(found);
        })
        .unwrap();
    match receive.recv_timeout(STALL) {
        Ok(Ok(found)) => found,
        Ok(Err(panic)) => {
            let text = panic
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| panic.downcast_ref::<&str>().map(|text| (*text).to_owned()))
                .unwrap_or_default();
            (Outcome::Panicked(text), Vec::new())
        }
        Err(_) => (Outcome::Stalled, Vec::new()),
    }
}

/// `text` fit for a Markdown table cell.
fn cell(text: &str) -> String {
    text.replace('|', "\\|")
}

/// The report's row cells after the input: result, bytes, first
/// difference, Shiki's stack and nishiki's stack.
/// A difference adds its lines to `details`.
fn result(sample: &str, found: &Outcome, details: &mut String) -> String {
    match found {
        Outcome::NoGrammar(reason) => format!("no grammar: {} | | | |", cell(reason)),
        Outcome::Panicked(text) => format!("panic: {} | | | |", cell(text)),
        Outcome::Stalled => format!("stalled past {} s | | | |", STALL.as_secs()),
        Outcome::Compared(comparison) => match &comparison.first {
            None => format!("equal | 0 of {} | | |", comparison.checked),
            Some(first) => {
                let _ = writeln!(
                    details,
                    "{sample}: {} of {} bytes differ{}\n",
                    comparison.missed, comparison.checked, comparison.message
                );
                format!(
                    "differs | {} of {} | {}:{} | `{}` | `{}`",
                    comparison.missed,
                    comparison.checked,
                    first.line,
                    first.column,
                    cell(&first.expected),
                    cell(&first.actual)
                )
            }
        },
    }
}

/// Writes `report.md` in `dir`: `summary`, then the table's `rows`.
fn write_report(dir: &Path, summary: &str, rows: &str) {
    let page = formatdoc! {"
        # Shiki fidelity

        {summary}

        | Input | Result | Bytes that differ | First difference | Shiki | nishiki | Refused patterns | Limits reached |
        |:--|:--|--:|:--|:--|:--|:--|:--|
        {rows}"};
    std::fs::write(dir.join("report.md"), page).unwrap();
}

/// Fails on a difference, a panic, a grammar that does not load or link, or a
/// refusal outside [`KNOWN_REFUSALS`].
/// It also fails on an entry of that list no grammar refuses any more, so the
/// list only shrinks.
///
/// A language with several inputs counts each of its refusals once.
///
/// A stall stops the run at once: its thread still runs, and would slow every
/// language after it.
#[test]
#[ignore = "needs the directory oracle/fidelity.ts writes"]
fn every_shiki_grammar_matches_shiki() {
    let Some(dir) = std::env::var_os("NISHIKI_FIDELITY") else {
        return;
    };
    let dir = std::path::PathBuf::from(dir);
    let languages: Value =
        serde_json::from_str(&std::fs::read_to_string(dir.join("languages.json")).unwrap())
            .unwrap();
    let mut rows = String::new();
    let mut details = String::new();
    let (mut equal, mut measured) = (0, 0);
    let mut unknown: Vec<String> = Vec::new();
    let mut seen: Vec<(&str, &str)> = Vec::new();
    for language in languages.as_array().unwrap() {
        let id = language["id"].as_str().unwrap();
        let sample = language["sample"].as_str().unwrap();
        let (found, refusals) = outcome(&dir, language);
        measured += 1;
        let row = result(sample, &found, &mut details);
        if matches!(found, Outcome::Compared(Comparison { first: None, .. })) {
            equal += 1;
        }
        for &(pattern, ref reason) in &refusals {
            if let Some(&known) = KNOWN_REFUSALS.iter().find(|&&known| known == (id, pattern)) {
                seen.push(known);
            } else {
                let refusal = format!("{id}: {reason} in {pattern}");
                if !unknown.contains(&refusal) {
                    unknown.push(refusal);
                }
            }
        }
        let refusal = refusals
            .first()
            .map_or_else(String::new, |(pattern, reason)| {
                format!("{}: {reason} in {}", refusals.len(), cell(pattern))
            });
        let limits = match &found {
            Outcome::Compared(comparison) => comparison
                .reached
                .iter()
                .map(|limit| format!("{limit:?}"))
                .collect::<Vec<_>>()
                .join(", "),
            Outcome::NoGrammar(_) | Outcome::Panicked(_) | Outcome::Stalled => String::new(),
        };
        let _ = writeln!(rows, "| {sample} | {row} | {refusal} | {limits} |");
        if matches!(found, Outcome::Stalled) {
            let summary = format!(
                "The run stopped at {sample}, which stalled past {} s.",
                STALL.as_secs()
            );
            write_report(&dir, &summary, &rows);
            panic!("{summary} See report.md.");
        }
    }
    let stale: Vec<String> = KNOWN_REFUSALS
        .iter()
        .filter(|&&known| !seen.contains(&known))
        .map(|(id, pattern)| format!("{id}: {pattern}"))
        .collect();
    let summary = format!(
        "{equal} of {measured} inputs match Shiki on every non-whitespace byte.\n\
         {} patterns are refused outside the known list, and {} known refusals no longer occur.",
        unknown.len(),
        stale.len()
    );
    write_report(&dir, &summary, &rows);
    eprintln!("{details}{summary}");
    assert_eq!(equal, measured, "{summary} See report.md.");
    assert!(
        unknown.is_empty(),
        "refusals outside KNOWN_REFUSALS:\n{}",
        unknown.join("\n")
    );
    assert!(
        stale.is_empty(),
        "remove these from KNOWN_REFUSALS, which no grammar refuses any more:\n{}",
        stale.join("\n")
    );
}
