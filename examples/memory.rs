//! Lexes every oracle fixture of one language, line by line from a fresh
//! highlighter each, and prints what that pass allocated: allocations, bytes,
//! and the most bytes held at once by any one document, as
//! `<allocations> <bytes> <peak>`.
//!
//! `mise run callgrind` runs it at a pull request's base and head and shows
//! the counts beside the instruction table, for information. The counts are
//! exact, so any difference comes from the code.
//!
//! A first pass fills what a process builds once, such as each rule's
//! pattern tables, and is not counted. Each document's highlighter and token
//! buffer are made inside the counted pass, because a caller needs both to
//! lex a document. Splitting the fixtures into lines happens before either
//! pass.
//!
//! Usage: `memory <fixtures> <language>`, such as `memory tests/oracle python`.

#![expect(
    clippy::print_stdout,
    clippy::print_stderr,
    reason = "the tool prints its counts and its usage errors"
)]

use std::hint::black_box;
use std::path::Path;
use std::process::ExitCode;
use std::{env, fs};

use nishiki::{Highlighter, Language, State};
use nishiki_alloc_count::{CountingAllocator, measure};

#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let [root, name] = args.as_slice() else {
        eprintln!("usage: memory <fixtures> <language>");
        return ExitCode::FAILURE;
    };
    let Some(language) = Language::from_name(name) else {
        eprintln!("unknown language {name:?}");
        return ExitCode::FAILURE;
    };
    let Ok(files_in) = fs::read_dir(Path::new(root).join(name)) else {
        eprintln!("no fixtures in {root}/{name}");
        return ExitCode::FAILURE;
    };
    // A fixture that cannot be read fails the run: skipping it would change
    // the counts with no sign of why.
    let mut files = Vec::new();
    for entry in files_in {
        match entry {
            Ok(entry) => files.push(entry.path()),
            Err(error) => {
                eprintln!("cannot list {root}/{name}: {error}");
                return ExitCode::FAILURE;
            }
        }
    }
    files.retain(|path| path.extension().is_some_and(|ext| ext == "txt"));
    files.sort();
    let mut texts = Vec::with_capacity(files.len());
    for file in &files {
        match fs::read_to_string(file) {
            Ok(text) => texts.push(text),
            Err(error) => {
                eprintln!("cannot read {}: {error}", file.display());
                return ExitCode::FAILURE;
            }
        }
    }
    let documents: Vec<Vec<&str>> = texts
        .iter()
        .map(|text| {
            text.lines()
                .filter_map(|record| record.strip_prefix("L "))
                .collect()
        })
        .collect();
    for lines in &documents {
        lex(language, lines);
    }
    let (mut allocations, mut bytes, mut peak) = (0_usize, 0_usize, 0_usize);
    for lines in &documents {
        let usage = measure(|| lex(language, lines));
        allocations = allocations.saturating_add(usage.allocations);
        bytes = bytes.saturating_add(usage.bytes);
        peak = peak.max(usage.peak);
    }
    println!("{allocations} {bytes} {peak}");
    ExitCode::SUCCESS
}

/// Lexes one document's lines from a fresh highlighter and token buffer.
fn lex(language: Language, lines: &[&str]) {
    let mut highlighter = Highlighter::new(language);
    let mut state = State::default();
    let mut tokens = Vec::new();
    for &line in lines {
        tokens.clear();
        let report = highlighter.line(line, &mut state, &mut tokens);
        black_box(&report);
    }
    black_box(&tokens);
}
