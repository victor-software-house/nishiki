//! Writes nishiki's generated Rust files, and checks them.
//!
//! `mise run codegen` writes each file whose bytes would change.
//! `mise run codegen -- --check` writes nothing, and fails when a file would
//! change. The writers read the grammars in `tests/engine/grammars/`, which
//! `oracle/engine-grammars.ts` writes.
//! They read each grammar through `nishiki::unstable`, the surface a caller has
//! (openspec rust-codegen, design section 3).

mod error;
mod header;
mod numbers;
mod words;

use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::{env, fs};

use error::Error;

/// A generated file: its path from the repository root, and its text.
#[derive(Debug)]
struct File {
    /// The path, such as `src/c/words.rs`.
    path: &'static str,
    /// The whole text the file holds.
    text: String,
}

/// What a run does with the files it generates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mode {
    /// Write each file whose bytes differ.
    Write,
    /// Write nothing, and report each file whose bytes differ.
    Check,
}

fn main() -> ExitCode {
    let mode = match env::args().skip(1).collect::<Vec<_>>().as_slice() {
        [] => Mode::Write,
        [flag] if flag == "--check" => Mode::Check,
        other => {
            eprintln!("codegen takes no argument or --check, not {other:?}");
            return ExitCode::FAILURE;
        }
    };
    match run(mode) {
        Ok(stale) if stale.is_empty() => ExitCode::SUCCESS,
        Ok(stale) => {
            for path in stale {
                eprintln!("{path} differs from what codegen/ writes");
            }
            eprintln!("run `mise run codegen` and commit the result");
            ExitCode::FAILURE
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

/// Generates every file, then writes or checks each.
/// Returns the files a check found stale; a write leaves none.
fn run(mode: Mode) -> Result<Vec<&'static str>, Error> {
    let root = root()?;
    let mut stale = Vec::new();
    for file in words::files(&root)? {
        let path = root.join(file.path);
        let current = fs::read(&path).ok();
        if current.as_deref() == Some(file.text.as_bytes()) {
            continue;
        }
        match mode {
            Mode::Check => stale.push(file.path),
            Mode::Write => {
                fs::write(&path, &file.text).map_err(|source| Error::Io { path, source })?;
                println!("wrote {}", file.path);
            }
        }
    }
    Ok(stale)
}

/// The repository root: the directory that holds `codegen/`.
fn root() -> Result<PathBuf, Error> {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest
        .parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| Error::Shape(format!("{} has no parent", manifest.display())))
}
