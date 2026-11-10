//! A grammar the test defines, run through [`RuleSyntax`].

use std::marker::PhantomData;

use super::RuleSyntax;
use crate::limits::Limits;
use crate::rules::{Act, Captures, Grammar, Lexer, ScopeName};
use crate::scope::{Scope, Scopes, TOML, scopes};
use crate::{Highlighter, State};

/// The scopes the grammar names, borrowed from TOML's table.
/// The test checks their names first, so a reordered table fails there.
const SOURCE: Scope = Scope::new(TOML, 0);
const KEY: Scope = Scope::new(TOML, 5);
const INTEGER: Scope = Scope::new(TOML, 18);

#[derive(Clone, Copy, Debug, PartialEq)]
struct Root;

#[derive(Clone, Copy, Debug)]
enum Pattern {
    Key,
    Integer,
}

#[derive(Clone, Copy, Debug)]
enum Nested {}

/// Lowercase words as keys and runs of digits as integers.
#[derive(Clone, Copy, Debug)]
struct Pairs;

impl Grammar for Pairs {
    type Scope = Scope;
    const NO_SCOPES: Scopes<Scope> = Scope::NO_SCOPES;
    type Rule = Root;
    type Pattern = Pattern;
    type Nested = Nested;
    type Extra = ();
    type Kept = ();

    const NAME: &'static str = "pairs";
    const ROOT: Root = Root;
    const SOURCE: Scope = SOURCE;

    fn scope(scope: Scope) -> ScopeName {
        ScopeName::Fixed(scope)
    }

    fn rule_code(_: Root) -> u16 {
        0
    }

    fn rule_from_code(_: u16) -> Root {
        Root
    }

    fn rule_name(_: Root) -> Scopes<Scope> {
        Scope::NO_SCOPES
    }

    fn rule_content(_: Root) -> Scopes<Scope> {
        Scope::NO_SCOPES
    }

    fn pattern_act(pattern: Pattern) -> Act<Self> {
        match pattern {
            Pattern::Key => Act::Match(scopes![KEY]),
            Pattern::Integer => Act::Match(scopes![INTEGER]),
        }
    }

    fn pattern_captures(_: Pattern) -> Captures<Self> {
        (&[], &[])
    }

    fn scan(lexer: &mut Lexer<'_, Self>) -> Option<(usize, Pattern, usize)> {
        for p in lexer.pos..lexer.lim() {
            for pattern in [Pattern::Key, Pattern::Integer] {
                if let Some(end) = lexer.try_pattern(pattern, p) {
                    return Some((p, pattern, end));
                }
            }
        }
        None
    }

    fn matcher(lexer: &mut Lexer<'_, Self>, pattern: Pattern, p: usize) -> Option<usize> {
        let end = match pattern {
            Pattern::Key => lexer.skip(p, |b| b.is_ascii_lowercase()),
            Pattern::Integer => lexer.skip(p, |b| b.is_ascii_digit()),
        };
        (end > p).then_some(end)
    }

    fn tokenize_nested(_: &mut Lexer<'_, Self>, nested: Nested, _: usize, _: usize, _: u32) {
        match nested {}
    }
}

/// Each token of `line` as its text and its scopes, innermost first.
fn tokens(
    highlighter: &mut Highlighter<RuleSyntax<Pairs>>,
    line: &str,
    state: &mut State,
) -> Vec<(String, Vec<String>)> {
    let mut out = Vec::new();
    let _ = highlighter.line(line, state, &mut out);
    let mut start = 0;
    out.iter()
        .map(|token| {
            let text = line[start..token.end()].to_owned();
            start = token.end();
            let scopes = highlighter
                .scopes(token.stack())
                .map(str::to_owned)
                .collect();
            (text, scopes)
        })
        .collect()
}

#[test]
fn a_grammar_the_test_defines_highlights_lines() {
    assert_eq!(
        [SOURCE.name(), KEY.name(), INTEGER.name()],
        [
            "source.toml",
            "variable.other.key.toml",
            "constant.numeric.integer.toml"
        ]
    );
    let mut highlighter =
        Highlighter::with_lexer(RuleSyntax::<Pairs>(PhantomData), Limits::default());
    let mut state = State::default();
    let row = |text: &str, scopes: &[&str]| {
        (
            text.to_owned(),
            scopes
                .iter()
                .map(|&scope| scope.to_owned())
                .collect::<Vec<_>>(),
        )
    };
    assert_eq!(
        tokens(&mut highlighter, "port = 80", &mut state),
        [
            row("port", &["variable.other.key.toml", "source.toml"]),
            row(" = ", &["source.toml"]),
            row("80", &["constant.numeric.integer.toml", "source.toml"]),
        ]
    );
    assert_eq!(
        tokens(&mut highlighter, "7 up", &mut state),
        [
            row("7", &["constant.numeric.integer.toml", "source.toml"]),
            row(" ", &["source.toml"]),
            row("up", &["variable.other.key.toml", "source.toml"]),
        ]
    );
}
