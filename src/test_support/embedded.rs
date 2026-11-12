//! Lexing the lines that another language embeds, as the tests of the
//! languages that embed or are embedded do.

use super::Row;
use crate::arena::{Arena, ROOT};
use crate::language::{Context, Halt, Line, StopReason, Stopped};
use crate::limits::Finished;
use crate::scratch::Scratch;
use crate::{Limits, LineReport, Token};

/// A language's lexer for lines another language embeds.
/// It lexes a line from a state up to where `end` or `after` halts it.
/// It returns the state and where it halted.
type LexUntil = fn(
    &mut Context<'_>,
    Line<'_>,
    u32,
    &dyn Halt,
    Option<&dyn Halt>,
) -> (Finished, Option<Stopped>);

/// How a language lexes the lines another language embeds: its start over a
/// base stack, its whole-line lexer, and its `lex_until`.
#[derive(Clone, Copy)]
pub(crate) struct Embedded {
    pub(crate) start: fn(&mut Arena, u32) -> u32,
    pub(crate) lex: fn(&mut Context<'_>, Line<'_>, u32) -> Finished,
    pub(crate) lex_until: LexUntil,
}

/// The halts of a Ruby here document: its interpolation, tried before the
/// embedded language's patterns, and its escapes, tried after them.
#[cfg(any(feature = "lang-html", feature = "lang-xml", feature = "lang-yaml"))]
pub(crate) const RUBY_HALTS: (&dyn Halt, Option<&dyn Halt>) =
    (&ruby_interpolation, Some(&ruby_escape));

/// A Ruby interpolation, as a Ruby here document's patterns start it.
#[cfg(any(feature = "lang-html", feature = "lang-xml", feature = "lang-yaml"))]
fn ruby_interpolation(bytes: &[u8], p: usize) -> bool {
    bytes.get(p..p + 2) == Some(b"#{")
}

/// A Ruby escape, as a Ruby here document's patterns start it.
#[cfg(any(feature = "lang-html", feature = "lang-xml", feature = "lang-yaml"))]
fn ruby_escape(bytes: &[u8], p: usize) -> bool {
    bytes.get(p) == Some(&b'\\')
}

/// Lexes `lines` in order with `language` from its start over the scopes
/// `base`, each from the start of the line.
/// It runs the language's `lex_until` with `halts`, or its whole-line lexer
/// when there are none.
/// Returns, per line, where it halted and each token's text and full scope
/// stack.
#[cfg(any(
    feature = "lang-html",
    feature = "lang-json",
    feature = "lang-postcss",
    feature = "lang-xml",
    feature = "lang-yaml"
))]
pub(crate) fn lex_embedded(
    language: Embedded,
    base: &[&str],
    lines: &[&str],
    halts: Option<(&dyn Halt, Option<&dyn Halt>)>,
) -> Vec<(Option<usize>, Row)> {
    lex_embedded_stops(language, base, lines, halts)
        .into_iter()
        .map(|(stopped, row)| (stopped.map(|stopped| stopped.at), row))
        .collect()
}

/// Lexes `lines` like [`lex_embedded`], and returns, per line, where and why
/// it stopped.
/// A line that halts has tokens up to the halt.
/// One that pops the rule that embeds the language has tokens to its end.
pub(crate) fn lex_embedded_stops(
    language: Embedded,
    base: &[&str],
    lines: &[&str],
    halts: Option<(&dyn Halt, Option<&dyn Halt>)>,
) -> Vec<(Option<Stopped>, Row)> {
    let mut arena = Arena::new(None);
    let mut stack = ROOT;
    for name in base {
        let scope = arena.named_scope(name).expect("room for the base");
        stack = arena.child(stack, scope).expect("room for the base");
    }
    let mut scratch = Scratch::default();
    let mut state = (language.start)(&mut arena, stack);
    let limits = Limits::default();
    let mut rows = Vec::new();
    for text in lines {
        let mut out: Vec<Token> = Vec::new();
        let mut report = LineReport::default();
        scratch.start_line();
        let mut cx = Context {
            scratch: &mut scratch,
            arena: &mut arena,
            limits: &limits,
            out: &mut out,
            report: &mut report,
            first_token: 0,
        };
        let line = Line {
            text,
            start: 0,
            embedded: false,
            anchored_at_root: false,
        };
        let (next, stopped) = match halts {
            Some((end, after)) => (language.lex_until)(&mut cx, line, state, end, after),
            None => ((language.lex)(&mut cx, line, state), None),
        };
        state = next.state();
        let mut from = 0;
        let mut row = Row::new();
        for token in &out {
            let mut scopes: Vec<String> =
                arena.scopes(token.stack().0).map(str::to_owned).collect();
            scopes.reverse();
            row.push((text[from..token.end()].to_owned(), scopes));
            from = token.end();
        }
        let end = match stopped {
            Some(Stopped {
                at,
                reason: StopReason::Halt,
            }) => at,
            Some(Stopped {
                reason: StopReason::PopHost,
                ..
            })
            | None => text.len(),
        };
        assert_eq!(from, end, "tokens up to the stop: {text:?}");
        rows.push((stopped, row));
    }
    rows
}

/// Where each line of `lines` halts under a Ruby here document's halts.
/// It checks that the line's tokens are those it gets with no halts.
/// That is all of them on a line that does not halt.
/// On a line that halts, it is all but the last, since the halt cuts the last
/// token short.
#[cfg(any(feature = "lang-html", feature = "lang-xml", feature = "lang-yaml"))]
pub(crate) fn ruby_heredoc_halts(
    language: Embedded,
    base: &[&str],
    lines: &[&str],
) -> Vec<Option<usize>> {
    let halting = lex_embedded(language, base, lines, Some(RUBY_HALTS));
    let plain = lex_embedded(language, base, lines, None);
    for ((halt, row), (_, whole)) in halting.iter().zip(&plain) {
        if halt.is_none() {
            assert_eq!(row, whole, "no halt, the same tokens");
        } else {
            let before = row.len().saturating_sub(1);
            assert_eq!(row[..before], whole[..before]);
        }
    }
    halting.into_iter().map(|(halt, _)| halt).collect()
}
