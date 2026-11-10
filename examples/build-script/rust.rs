//! Highlights standard input with a grammar its build script wrote as Rust.
//!
//! `build.rs` compiles `demo.json` with `Grammar::from_json`.
//! It writes the result of `Grammar::to_rust` to `OUT_DIR`.
//! This program includes that file, so the grammar is a `static`.
//! Loading it takes no work at run time.
//! It prints each token's text and its innermost scope, one token a line.
//!
//! ```text
//! echo 'let n = 1 # one' | cargo run -p nishiki-build-script --bin nishiki-build-script-rust
//! ```

mod highlight;

use std::error::Error;
use std::io::{self, BufWriter, Read as _, Write as _};

use crate::highlight::highlight;

/// The demo grammar, as `build.rs` wrote it.
#[expect(
    unreachable_pub,
    reason = "the file is written for a library to export"
)]
mod demo {
    include!(env!("DEMO_RUST"));
}

/// A grammar with every field and every list of the model filled in, as
/// `build.rs` wrote it from `full.json`.
#[cfg(test)]
#[expect(
    unreachable_pub,
    reason = "the file is written for a library to export"
)]
mod full {
    include!(env!("FULL_RUST"));
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut text = String::new();
    io::stdin().read_to_string(&mut text)?;
    let mut out = BufWriter::new(io::stdout().lock());
    highlight(demo::GRAMMAR, &text, &mut out)?;
    out.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use nishiki::Grammar;

    use super::{demo, full};
    use crate::highlight::{DEMO_LINE, DEMO_TOKENS, highlight};

    #[test]
    fn the_grammar_written_as_rust_highlights_a_line() {
        let mut out = Vec::new();
        highlight(demo::GRAMMAR, DEMO_LINE, &mut out).unwrap();
        assert_eq!(String::from_utf8(out).unwrap(), DEMO_TOKENS);
    }

    #[test]
    fn a_grammar_written_as_rust_keeps_every_field() {
        for (written, json) in [
            (demo::GRAMMAR, include_str!("demo.json")),
            (full::GRAMMAR, include_str!("full.json")),
        ] {
            assert_eq!(
                written.to_bytes(),
                Grammar::from_json(json).unwrap().to_bytes()
            );
        }
    }
}
