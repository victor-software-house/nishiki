//! Highlights standard input with every language the build turns on, and
//! prints the number of tokens.
//!
//! `mise run check:size` builds this program once per tier in
//! `[workspace.metadata.size]` of the root `Cargo.toml`.
//! It holds the size of each build's sections to the tier's budgets.
//!
//! The program reads its text at run time and styles every token.
//! The linker then keeps every lexer, its tables and the styling code.
//!
//! ```text
//! cargo build --release --package nishiki-examples --example size \
//!   --no-default-features --features nishiki/lang-json
//! ```

use std::hint::black_box;
use std::io::{self, Read as _, Write as _};
use std::process::ExitCode;

use nishiki::{Highlighter, Language, State, Theme, ThemeSource};

fn main() -> ExitCode {
    let mut text = String::new();
    if io::stdin().read_to_string(&mut text).is_err() {
        return ExitCode::FAILURE;
    }
    let theme = Theme::new(ThemeSource::default());
    let mut tokens = Vec::new();
    let mut count = 0usize;
    for &language in Language::ALL {
        let mut highlighter = Highlighter::new(language);
        let mut state = State::default();
        for line in text.lines() {
            tokens.clear();
            let report = highlighter.line(line, &mut state, &mut tokens);
            black_box(&report);
            for token in &tokens {
                black_box(highlighter.style(&theme, token.stack()));
            }
            count += tokens.len();
        }
    }
    writeln!(io::stdout(), "{count}").map_or(ExitCode::FAILURE, |()| ExitCode::SUCCESS)
}
