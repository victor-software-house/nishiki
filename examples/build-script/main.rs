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

use std::error::Error;
use std::io::{self, BufWriter, Read as _, Write};

use nishiki::{Grammar, Grammars, Highlighter, State};

/// The demo grammar, as `build.rs` wrote it.
static DEMO: &[u8] = include_bytes!(env!("DEMO_GRAMMAR"));

fn main() -> Result<(), Box<dyn Error>> {
    let mut text = String::new();
    io::stdin().read_to_string(&mut text)?;
    let mut out = BufWriter::new(io::stdout().lock());
    highlight(&text, &mut out)?;
    out.flush()?;
    Ok(())
}

/// Writes each token of `text` with its innermost scope to `out`.
fn highlight(text: &str, out: &mut impl Write) -> Result<(), Box<dyn Error>> {
    let grammar = Grammar::from_bytes(DEMO)?;
    let mut highlighter = Highlighter::for_grammar(grammar, &Grammars::new())?;
    let mut state = State::default();
    let mut tokens = Vec::new();
    for line in text.lines() {
        tokens.clear();
        let _ = highlighter.line(line, &mut state, &mut tokens);
        let mut start = 0;
        for token in &tokens {
            let scope = highlighter.scopes(token.stack()).next().unwrap_or_default();
            writeln!(out, "{:?} {scope}", &line[start..token.end()])?;
            start = token.end();
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::highlight;

    #[test]
    fn the_grammar_from_the_build_script_highlights_a_line() {
        let mut out = Vec::new();
        highlight(r#"let s = "a\n" # 1"#, &mut out).unwrap();
        assert_eq!(
            String::from_utf8(out).unwrap(),
            indoc! {r##"
                "let" keyword.control.demo
                " s = " source.demo
                "\"a" string.quoted.double.demo
                "\\n" constant.character.escape.demo
                "\"" string.quoted.double.demo
                " " source.demo
                "# 1" comment.line.number-sign.demo
            "##}
        );
    }
}
