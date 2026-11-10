//! Highlighting shared by both programs: each token of standard input with
//! its innermost scope, one token a line.

use std::error::Error;
use std::io::Write;

use nishiki::{Grammar, Grammars, Highlighter, State};

/// A line of the demo language with every scope `demo.json` names.
#[cfg(test)]
pub(crate) const DEMO_LINE: &str = r#"let s = "a\n" # 1"#;

/// What [`highlight`] writes for [`DEMO_LINE`] with the demo grammar.
#[cfg(test)]
pub(crate) const DEMO_TOKENS: &str = indoc::indoc! {r##"
    "let" keyword.control.demo
    " s = " source.demo
    "\"a" string.quoted.double.demo
    "\\n" constant.character.escape.demo
    "\"" string.quoted.double.demo
    " " source.demo
    "# 1" comment.line.number-sign.demo
"##};

/// Writes each token of `text` with its innermost scope to `out`.
pub(crate) fn highlight(
    grammar: Grammar,
    text: &str,
    out: &mut impl Write,
) -> Result<(), Box<dyn Error>> {
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
