//! Lexes every oracle fixture of one language once, line by line from a
//! fresh highlighter, and prints nothing: the program `mise run callgrind`
//! counts instructions of, at a pull request's base and head.
//!
//! Usage: `lex <fixtures> <language>`, such as `lex tests/oracle python`.

#![expect(clippy::print_stderr, reason = "the tool prints its usage errors")]

use std::hint::black_box;
use std::path::Path;
use std::process::ExitCode;
use std::{env, fs};

use nishiki::{Highlighter, Language, State};

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
    let mut tokens = Vec::new();
    for file in files {
        let Ok(content) = fs::read_to_string(&file) else {
            continue;
        };
        let mut highlighter = Highlighter::new(language);
        let mut state = State::default();
        for line in content
            .lines()
            .filter_map(|record| record.strip_prefix("L "))
        {
            tokens.clear();
            let report = highlighter.line(line, &mut state, &mut tokens);
            black_box(&report);
        }
    }
    black_box(&tokens);
    ExitCode::SUCCESS
}
