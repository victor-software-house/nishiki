//! Each limit, reached and not reached, through the public API.
//! Each language's hostile lines are in `limits/<language>.rs`.

mod c;
mod cpp;
mod csharp;
mod css;
mod diff;
mod dockerfile;
mod go;
mod graphql;
mod html;
mod ini;
mod java;
mod json;
mod kotlin;
mod make;
mod markdown;
#[cfg(feature = "lang-powershell")]
mod powershell;
mod python;
mod ruby;
mod rust;
#[cfg(feature = "lang-scala")]
mod scala;
mod shell;
mod sql;
mod swift;
mod toml;
mod typescript;
mod xml;
mod yaml;

use nishiki::{Highlighter, Language, Limit, Limits, LineReport, State, Token, Work};

/// Lexes `lines` in order and returns each line's tokens and report, and
/// the final state.
fn lex(highlighter: &mut Highlighter, lines: &[&str]) -> (Vec<(Vec<Token>, LineReport)>, State) {
    let mut state = State::default();
    let rows = lines
        .iter()
        .map(|line| {
            let mut tokens = Vec::new();
            let report = highlighter.line(line, &mut state, &mut tokens);
            assert_eq!(
                tokens.last().map_or(0, |token| token.end()),
                line.len(),
                "tokens cover {line:?}"
            );
            (tokens, report)
        })
        .collect();
    (rows, state)
}

fn innermost(highlighter: &Highlighter, token: Token) -> &str {
    highlighter.scopes(token.stack()).next().unwrap_or_default()
}

#[test]
fn a_long_line_is_one_token_and_keeps_the_state() {
    let limits = Limits {
        line_bytes: Some(8),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Rust, limits);
    let (rows, state) = lex(&mut highlighter, &["let s = \"open", "short"]);
    let (tokens, report) = &rows[0];
    assert!(report.reached(Limit::LineBytes));
    assert_eq!(tokens.len(), 1);
    assert_eq!(innermost(&highlighter, tokens[0]), "source.rust");
    assert!(
        rows[1].1.is_complete(),
        "the next line is lexed from the old state"
    );
    assert_eq!(state, State::default());
}

#[test]
fn running_out_of_work_ends_the_line_and_keeps_its_state() {
    let limits = Limits {
        work: Some(Work {
            per_byte: 0,
            base: 64,
        }),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Rust, limits);
    let line = format!("let s = \"{}\"; let t = 1;", r"\n".repeat(200));
    let (rows, _) = lex(&mut highlighter, &[&line, "x"]);
    let (tokens, report) = &rows[0];
    assert!(report.reached(Limit::Work));
    let last = tokens[tokens.len() - 1];
    assert_eq!(
        innermost(&highlighter, last),
        "string.quoted.double.rust",
        "the rest of the line keeps the string's stack"
    );
    assert_eq!(
        innermost(&highlighter, rows[1].0[0]),
        "string.quoted.double.rust",
        "the next line starts inside the string"
    );
}

#[test]
fn contexts_past_the_depth_limit_are_not_opened() {
    let limits = Limits {
        depth: Some(4),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Rust, limits);
    let (rows, state) = lex(&mut highlighter, &["f(f(f(f(f(f(x))))))"]);
    assert!(rows[0].1.reached(Limit::Depth));
    assert_eq!(state, State::default());
}

#[test]
fn depth_can_be_raised_without_bound() {
    let mut highlighter = Highlighter::new_with_limits(Language::Rust, Limits::unlimited());
    let deep = "/*".repeat(10_000);
    let close = "*/".repeat(10_000);
    let (rows, state) = lex(&mut highlighter, &[&deep, &close]);
    assert!(rows.iter().all(|(_, report)| report.is_complete()));
    assert_eq!(innermost(&highlighter, rows[1].0[0]), "comment.block.rust");
    assert_eq!(state, State::default(), "every comment closed");
}

#[test]
fn the_node_limit_falls_back_to_the_context_stack() {
    let limits = Limits {
        nodes: Some(4),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Rust, limits);
    let (rows, _) = lex(&mut highlighter, &["fn main() { let x = \"s\"; }"]);
    assert!(rows[0].1.reached(Limit::Nodes));
}
