//! Highlights standard input with a grammar its build script compiled.
//!
//! `build.rs` compiles `demo.json` with `Grammar::from_json`.
//! It writes the result of `Grammar::to_bytes` to `OUT_DIR`.
//! This program loads those bytes with `Grammar::from_bytes`, so it parses no
//! JSON at run time.
//! It prints each token's text and its innermost scope, one token a line.
//!
//! ```text
//! echo 'let n = 1 # one' | cargo run --package nishiki-build-script
//! ```

mod highlight;

use std::error::Error;
use std::io::{self, BufWriter, Read as _, Write as _};

use nishiki::Grammar;

use crate::highlight::highlight;

/// The demo grammar, as `build.rs` wrote it.
static DEMO: &[u8] = include_bytes!(env!("DEMO_GRAMMAR"));

fn main() -> Result<(), Box<dyn Error>> {
    let mut text = String::new();
    io::stdin().read_to_string(&mut text)?;
    let mut out = BufWriter::new(io::stdout().lock());
    highlight(Grammar::from_bytes(DEMO)?, &text, &mut out)?;
    out.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use nishiki::Grammar;

    use super::DEMO;
    use crate::highlight::{DEMO_LINE, DEMO_TOKENS, highlight};

    #[test]
    fn the_grammar_from_the_build_script_highlights_a_line() {
        let mut out = Vec::new();
        highlight(Grammar::from_bytes(DEMO).unwrap(), DEMO_LINE, &mut out).unwrap();
        assert_eq!(String::from_utf8(out).unwrap(), DEMO_TOKENS);
    }
}
