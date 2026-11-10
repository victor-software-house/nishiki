//! Compiles `demo.json` to nishiki's grammar bytes in `OUT_DIR`.
//!
//! The program loads the bytes with `include_bytes!`, so it parses no JSON at
//! run time.
//! `DEMO_GRAMMAR` names the file, so the program needs no `concat!` to find
//! it.

use std::error::Error;
use std::io::{self, Write as _};
use std::path::PathBuf;
use std::{env, fs};

use nishiki::Grammar;

fn main() -> Result<(), Box<dyn Error>> {
    let json = fs::read_to_string("demo.json")?;
    let bytes = Grammar::from_json(&json)?.to_bytes();
    let path =
        PathBuf::from(env::var_os("OUT_DIR").ok_or("cargo sets no OUT_DIR")?).join("demo.grammar");
    fs::write(&path, bytes)?;
    let mut out = io::stdout().lock();
    writeln!(out, "cargo::rerun-if-changed=demo.json")?;
    writeln!(out, "cargo::rustc-env=DEMO_GRAMMAR={}", path.display())?;
    Ok(())
}
