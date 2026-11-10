//! What the languages' unit tests share: lexing text line by line as one
//! document, and reading back each token's text and scope names.

use crate::arena::{Arena, ROOT};
use crate::language::{Context, Halt, Line, StopReason, Stopped};
use crate::limits::Finished;
use crate::scratch::Scratch;
use crate::{Highlighter, Language, Limits, LineReport, State, Token};

/// One lexed line: each token's text and its scope names, outermost first.
pub(crate) type Row = Vec<(String, Vec<String>)>;

/// Lexes `text` line by line with `highlighter` as one document from its
/// start.
/// It asserts that no line reaches a limit and that each line's tokens cover
/// it.
pub(crate) fn lex_with(highlighter: &mut Highlighter, text: &str) -> Vec<Row> {
    let mut state = State::default();
    let mut rows = Vec::new();
    for line in text.split('\n') {
        let mut tokens = Vec::new();
        let report = highlighter.line(line, &mut state, &mut tokens);
        assert!(report.is_complete(), "{line:?} reached a limit");
        let mut start = 0;
        let mut row = Vec::new();
        for token in &tokens {
            let mut scopes: Vec<String> = highlighter
                .scopes(token.stack())
                .map(ToString::to_string)
                .collect();
            scopes.reverse();
            row.push((line[start..token.end()].to_owned(), scopes));
            start = token.end();
        }
        assert_eq!(start, line.len(), "the tokens cover the line {line:?}");
        rows.push(row);
    }
    rows
}

/// Lexes `text` as `language` with a new highlighter, as [`lex_with`] does.
pub(crate) fn lex(language: Language, text: &str) -> Vec<Row> {
    lex_with(&mut Highlighter::new(language), text)
}

/// `rows` without each token's outermost scope, the document's root.
pub(crate) fn without_root(rows: Vec<Row>) -> Vec<Row> {
    rows.into_iter()
        .map(|row| {
            row.into_iter()
                .map(|(text, scopes)| (text, scopes.into_iter().skip(1).collect()))
                .collect()
        })
        .collect()
}

/// The scopes of the first token of `rows`, lexed from `text`, whose text is
/// `token`, outermost first.
pub(crate) fn scopes_in(rows: Vec<Row>, text: &str, token: &str) -> Vec<String> {
    rows.into_iter()
        .flatten()
        .find(|(t, _)| t == token)
        .unwrap_or_else(|| panic!("no token {token:?} in {text:?}"))
        .1
}

/// The scopes of the first token whose text is `token` when `text` is lexed as
/// `language`, outermost first.
pub(crate) fn scopes_of(language: Language, text: &str, token: &str) -> Vec<String> {
    scopes_in(lex(language, text), text, token)
}

/// The innermost scope of the first token whose text is `token` when `text`
/// is lexed as `language`.
pub(crate) fn leaf(language: Language, text: &str, token: &str) -> String {
    scopes_of(language, text, token).pop().unwrap_or_default()
}

/// One lexed line with each token cut to its innermost scope, neighbouring
/// tokens of the same scope merged: what a theme colours.
pub(crate) type LeafRow = Vec<(String, String)>;

/// `rows` with each token cut to its innermost scope and neighbouring tokens
/// of the same innermost scope merged into one.
pub(crate) fn merge_by_leaf(rows: Vec<Row>) -> Vec<LeafRow> {
    rows.into_iter()
        .map(|row| {
            let mut merged = LeafRow::new();
            for (text, mut scopes) in row {
                let leaf = scopes.pop().unwrap_or_default();
                match merged.last_mut() {
                    Some((last, scope)) if *scope == leaf => last.push_str(&text),
                    _ => merged.push((text, leaf)),
                }
            }
            merged
        })
        .collect()
}

/// A [`LeafRow`] from `(text, innermost scope)` pairs, to compare a lexed one
/// with.
pub(crate) fn leaf_row(parts: &[(&str, &str)]) -> LeafRow {
    parts
        .iter()
        .map(|&(text, scope)| (text.to_owned(), scope.to_owned()))
        .collect()
}

/// The innermost scope of the merged token in `row` whose text is `token`.
pub(crate) fn leaf_in(row: &LeafRow, token: &str) -> String {
    row.iter()
        .find(|(text, _)| text == token)
        .unwrap_or_else(|| panic!("no token {token:?} in {row:?}"))
        .1
        .clone()
}

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
pub(crate) const RUBY_HALTS: (&dyn Halt, Option<&dyn Halt>) =
    (&ruby_interpolation, Some(&ruby_escape));

/// A Ruby interpolation, as a Ruby here document's patterns start it.
fn ruby_interpolation(bytes: &[u8], p: usize) -> bool {
    bytes.get(p..p + 2) == Some(b"#{")
}

/// A Ruby escape, as a Ruby here document's patterns start it.
fn ruby_escape(bytes: &[u8], p: usize) -> bool {
    bytes.get(p) == Some(&b'\\')
}

/// Lexes `lines` in order with `language` from its start over the scopes
/// `base`, each from the start of the line.
/// It runs the language's `lex_until` with `halts`, or its whole-line lexer
/// when there are none.
/// Returns, per line, where it halted and each token's text and full scope
/// stack.
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

#[cfg(feature = "_rule-engine")]
pub(crate) mod probe;

/// The text of every oracle fixture, with its language.
pub(crate) fn oracle_documents() -> Vec<(crate::Language, String)> {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/oracle");
    let mut dirs: Vec<_> = std::fs::read_dir(&root)
        .expect("tests/oracle is readable")
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect();
    dirs.sort();
    let mut documents = Vec::new();
    for dir in dirs {
        let Some(language) = dir
            .file_name()
            .and_then(|name| name.to_str())
            .and_then(crate::Language::from_name)
        else {
            continue;
        };
        let mut files: Vec<_> = std::fs::read_dir(&dir)
            .expect("a fixture directory is readable")
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| path.extension().is_some_and(|ext| ext == "txt"))
            .collect();
        files.sort();
        for file in files {
            let text = std::fs::read_to_string(&file).expect("a fixture is readable");
            let lines: Vec<&str> = text
                .lines()
                .filter_map(|record| record.strip_prefix("L ").or((record == "L").then_some("")))
                .collect();
            documents.push((language, lines.join("\n")));
        }
    }
    documents
}
