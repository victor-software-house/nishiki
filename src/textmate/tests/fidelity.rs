//! The Shiki fidelity check, task 3.1 of `openspec/changes/grammar-api`.
//!
//! Every language Shiki 4.6.0 bundles runs on its input: Shiki's sample for
//! it, or a stand-in where Shiki has none.
//! Each non-whitespace byte must get the scope stack Shiki gives it.
//! Every line is compared, with no block left out.
//!
//! `oracle/fidelity.ts` writes the directory that `NISHIKI_FIDELITY` names.
//! The check writes `report.md` there, one row per language.

use std::fmt::Write as _;
use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;

use indoc::formatdoc;
use serde_json::Value;

use super::{Comparison, Grammar, Scanners, Sketch, compare, parse, survey};

/// How long one input may take before it counts as stalled.
const STALL: Duration = Duration::from_secs(120);

/// The stack of the thread that lexes an input.
/// A grammar that nests deeply recurses deeply, and an overflow aborts every
/// check.
const STACK: usize = 256 << 20;

/// Language `id`'s grammar, from the directory `oracle/fidelity.ts` wrote.
///
/// It reads the tables `oracle/engine-tables.ts --json` wrote, through the
/// survey's loader. The public API replaces it: `Grammar::from_json` on the
/// files in `grammars/`.
fn grammar(dir: &Path, id: &str) -> &'static Grammar {
    let path = dir.join("tables").join(format!("{id}.json"));
    survey::load(&std::fs::read_to_string(path).unwrap())
}

/// What the check found for one language.
enum Outcome {
    /// `engine-tables.ts` failed, for the reason it gave.
    NoGrammar(String),
    Panicked(String),
    Stalled,
    Compared(Comparison),
}

/// The patterns of `grammar` the regex compiler refuses, and its reason for the
/// first.
fn refused(grammar: &Grammar) -> (usize, Option<String>) {
    let mut count = 0;
    let mut first = None;
    for pattern in grammar.patterns {
        if let Err(error) = Sketch::for_grammar(pattern.source) {
            count += 1;
            first.get_or_insert_with(|| format!("{error:?} in {}", pattern.source));
        }
    }
    (count, first)
}

/// Loads language `id` and compares its input on a thread of its own.
/// A panic or a stall then ends only that language's check.
fn outcome(dir: &Path, id: &str) -> (Outcome, (usize, Option<String>)) {
    let (send, receive) = mpsc::channel();
    let dir = dir.to_owned();
    let id = id.to_owned();
    std::thread::Builder::new()
        .stack_size(STACK)
        .spawn(move || {
            let found = std::panic::catch_unwind(|| {
                let loaded = grammar(&dir, &id);
                let lines = parse(&dir.join("samples").join(format!("{id}.txt")));
                (compare(loaded, &lines, Scanners::On, &[]), refused(loaded))
            });
            let _ = send.send(found);
        })
        .unwrap();
    match receive.recv_timeout(STALL) {
        Ok(Ok((found, refusals))) => (Outcome::Compared(found), refusals),
        Ok(Err(panic)) => {
            let text = panic
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| panic.downcast_ref::<&str>().map(|text| (*text).to_owned()))
                .unwrap_or_default();
            (Outcome::Panicked(text), (0, None))
        }
        Err(_) => (Outcome::Stalled, (0, None)),
    }
}

/// `text` fit for a Markdown table cell.
fn cell(text: &str) -> String {
    text.replace('|', "\\|")
}

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
    let mut report = String::new();
    let mut details = String::new();
    let (mut equal, mut measured) = (0, 0);
    for language in languages.as_array().unwrap() {
        let id = language["id"].as_str().unwrap();
        let (found, (refusals, first_refusal)) = if let Some(reason) = language["tables"].as_str() {
            (Outcome::NoGrammar(reason.to_owned()), (0, None))
        } else {
            outcome(&dir, id)
        };
        let limits = match &found {
            Outcome::Compared(comparison) => comparison
                .reached
                .iter()
                .map(|limit| format!("{limit:?}"))
                .collect::<Vec<_>>()
                .join(", "),
            Outcome::NoGrammar(_) | Outcome::Panicked(_) | Outcome::Stalled => String::new(),
        };
        let row = match &found {
            Outcome::NoGrammar(reason) => format!("no grammar: {} | | | |", cell(reason)),
            Outcome::Panicked(text) => format!("panic: {} | | | |", cell(text)),
            Outcome::Stalled => format!("stalled past {} s | | | |", STALL.as_secs()),
            Outcome::Compared(comparison) => match &comparison.first {
                None => {
                    equal += 1;
                    format!("equal | 0 of {} | | |", comparison.checked)
                }
                Some(first) => {
                    let _ = writeln!(
                        details,
                        "{id}: {} of {} bytes differ{}\n",
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
        };
        measured += 1;
        let refusal = first_refusal.map_or_else(String::new, |first| {
            format!(
                "{refusals}: {}",
                cell(&first.chars().take(120).collect::<String>())
            )
        });
        let _ = writeln!(report, "| {id} | {row} | {refusal} | {limits} |");
    }
    let summary = format!(
        "{equal} of {measured} languages match Shiki on every non-whitespace byte of their input."
    );
    let page = formatdoc! {"
        # Shiki fidelity

        {summary}

        | Language | Result | Bytes that differ | First difference | Shiki | nishiki | Refused patterns | Limits reached |
        |:--|:--|--:|:--|:--|:--|:--|:--|
        {report}"};
    std::fs::write(dir.join("report.md"), page).unwrap();
    eprintln!("{details}{summary}");
    assert_eq!(equal, measured, "{summary} See report.md.");
}
