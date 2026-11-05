//! Lexes every oracle fixture of one language in three passes, line by line
//! from a fresh highlighter for each document, and prints nothing: the
//! program `mise run callgrind` counts instructions of, at a pull request's
//! base and head.
//!
//! Callgrind counts each pass on its own:
//!
//! - [`first_pass`] lexes and styles every token in a fresh process.
//! - [`measured`] lexes again, and styles nothing.
//! - [`colour_pass`] lexes and styles every token with github-dark, as a
//!   renderer does.
//!
//! The first pass holds the work a process does once, such as building each
//! rule's pattern tables.
//! That work is done before [`measured`], which holds lexing alone.
//!
//! Each fixture is split into its lines before any pass, so no count holds
//! the splitting.
//!
//! Usage: `lex <fixtures> <language>`, such as `lex tests/oracle python`.

#![expect(clippy::print_stderr, reason = "the tool prints its usage errors")]

use std::hint::black_box;
use std::path::Path;
use std::process::ExitCode;
use std::{env, fs};

use nishiki::{Highlighter, Language, State, Theme, Token};

/// Shiki's github-dark, as the oracle pins it.
const GITHUB_DARK: &str = include_str!("../tests/oracle/themes/github-dark.json");

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
    let theme = match Theme::from_json(GITHUB_DARK) {
        Ok(theme) => theme,
        Err(error) => {
            eprintln!("github-dark does not load: {error}");
            return ExitCode::FAILURE;
        }
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
    black_box(first_pass(language, &theme, &documents, &mut tokens));
    measured(language, &documents, &mut tokens);
    colour_pass(language, &theme, &documents, &mut tokens);
    black_box(&tokens);
    ExitCode::SUCCESS
}

/// The first pass of the process, which callgrind counts as the first
/// document; kept out of line so that it has a symbol of its own.
///
/// It returns the last line's token count.
/// That keeps its body apart from [`colour_pass`], whose symbol it would
/// otherwise share.
#[inline(never)]
fn first_pass(
    language: Language,
    theme: &Theme,
    documents: &[Vec<&str>],
    tokens: &mut Vec<Token>,
) -> usize {
    colour(language, theme, documents, tokens);
    tokens.len()
}

/// The pass callgrind counts as lexing (`--toggle-collect`); kept out of
/// line so that it has a symbol of its own.
#[inline(never)]
fn measured(language: Language, documents: &[Vec<&str>], tokens: &mut Vec<Token>) {
    lex(language, documents, tokens);
}

/// The pass callgrind counts as colouring; kept out of line so that it has a
/// symbol of its own.
#[inline(never)]
fn colour_pass(
    language: Language,
    theme: &Theme,
    documents: &[Vec<&str>],
    tokens: &mut Vec<Token>,
) {
    colour(language, theme, documents, tokens);
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

/// Lexes each document's lines from a fresh highlighter and styles every
/// token with `theme`.
fn colour(language: Language, theme: &Theme, documents: &[Vec<&str>], tokens: &mut Vec<Token>) {
    for lines in documents {
        let mut highlighter = Highlighter::new(language);
        let mut state = State::default();
        for &line in lines {
            tokens.clear();
            let report = highlighter.line(line, &mut state, tokens);
            black_box(&report);
            for token in tokens.iter() {
                black_box(theme.style(highlighter.scopes(token.stack())));
            }
        }
    }
}
