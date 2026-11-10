//! Compiles the example grammars and writes them to `OUT_DIR`.
//!
//! - `demo.grammar` holds `Grammar::to_bytes` of `demo.json`.
//!   `nishiki-build-script` loads it with `include_bytes!`.
//! - `demo.rs` and `full.rs` hold `Grammar::to_rust` of `demo.json` and
//!   `full.json`. `nishiki-build-script-rust` includes them with `include!`.
//!
//! Neither program parses JSON at run time.
//! An environment variable names each file, so the programs need no `concat!`
//! to find it.

use std::error::Error;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

use nishiki::Grammar;

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").ok_or("cargo sets no OUT_DIR")?);
    let mut out = io::stdout().lock();
    let demo = grammar(&mut out, "demo.json")?;
    write(
        &mut out,
        &out_dir.join("demo.grammar"),
        "DEMO_GRAMMAR",
        demo.to_bytes(),
    )?;
    write(
        &mut out,
        &out_dir.join("demo.rs"),
        "DEMO_RUST",
        demo.to_rust(),
    )?;
    let full = grammar(&mut out, "full.json")?;
    write(
        &mut out,
        &out_dir.join("full.rs"),
        "FULL_RUST",
        full.to_rust(),
    )?;
    Ok(())
}

/// Compiles the grammar in `file`, and asks cargo to rerun when it changes.
fn grammar(out: &mut impl io::Write, file: &str) -> Result<Grammar, Box<dyn Error>> {
    writeln!(out, "cargo::rerun-if-changed={file}")?;
    Ok(Grammar::from_json(&fs::read_to_string(file)?)?)
}

/// Writes `contents` to `path`, and names the path in the variable `name`.
fn write(
    out: &mut impl io::Write,
    path: &Path,
    name: &str,
    contents: impl AsRef<[u8]>,
) -> Result<(), Box<dyn Error>> {
    fs::write(path, contents)?;
    writeln!(out, "cargo::rustc-env={name}={}", path.display())?;
    Ok(())
}
