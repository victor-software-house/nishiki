//! Lexes every oracle fixture of one language twice, line by line from a
//! fresh highlighter each time, and prints nothing: the program
//! `mise run callgrind` counts instructions of, at a pull request's base and
//! head.
//!
//! The first pass fills what a process builds once, such as each rule's
//! pattern tables; callgrind counts only [`measured`], the second pass, so
//! a change to that one-time work does not read as a change to lexing.
//! Each fixture is split into its lines before either pass, so the count
//! holds lexing alone.
//!
//! Usage: `lex <fixtures> <language>`, such as `lex tests/oracle python`.

#![expect(clippy::print_stderr, reason = "the tool prints its usage errors")]

use std::hint::black_box;
use std::path::Path;
use std::process::ExitCode;
use std::{env, fs};

use nishiki::{Highlighter, Language, State, Token};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let [root, name] = args.as_slice() else {
        eprintln!("usage: lex <fixtures> <language>");
        return ExitCode::FAILURE;
    };
    let Some(language) = Language::from_name(name) else {
        eprintln!("unknown language {name:?}");
        return ExitCode::FAILURE;
    };
    let Ok(files) = fs::read_dir(Path::new(root).join(name)) else {
        eprintln!("no fixtures in {root}/{name}");
        return ExitCode::FAILURE;
    };
    let mut files: Vec<_> = files
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "txt"))
        .collect();
    files.sort();
    let texts: Vec<String> = files
        .iter()
        .filter_map(|file| fs::read_to_string(file).ok())
        .collect();
    let documents: Vec<Vec<&str>> = texts
        .iter()
        .map(|text| {
            text.lines()
                .filter_map(|record| record.strip_prefix("L "))
                .collect()
        })
        .collect();
    let mut tokens = Vec::new();
    lex(language, &documents, &mut tokens);
    measured(language, &documents, &mut tokens);
    black_box(&tokens);
    ExitCode::SUCCESS
}

/// The pass callgrind counts (`--toggle-collect`); kept out of line so that
/// it has a symbol of its own.
#[inline(never)]
fn measured(language: Language, documents: &[Vec<&str>], tokens: &mut Vec<Token>) {
    lex(language, documents, tokens);
}

/// Lexes each document's lines from a fresh highlighter.
fn lex(language: Language, documents: &[Vec<&str>], tokens: &mut Vec<Token>) {
    for lines in documents {
        let mut highlighter = Highlighter::new(language);
        let mut state = State::default();
        for &line in lines {
            tokens.clear();
            let report = highlighter.line(line, &mut state, tokens);
            black_box(&report);
        }
    }
}
