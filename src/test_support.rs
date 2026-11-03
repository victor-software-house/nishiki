//! What the languages' unit tests share: lexing text line by line as one
//! document, and reading back each token's text and scope names.

use crate::{Highlighter, Language, State};

/// One lexed line: each token's text and its scope names, outermost first.
pub(crate) type Row = Vec<(String, Vec<String>)>;

/// Lexes `text` line by line with `highlighter` as one document from its
/// start, asserting that no line reaches a limit and that each line's tokens
/// cover it.
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

/// The innermost scope of the merged token in `row` whose text is `token`.
pub(crate) fn leaf_in(row: &LeafRow, token: &str) -> String {
    row.iter()
        .find(|(text, _)| text == token)
        .unwrap_or_else(|| panic!("no token {token:?} in {row:?}"))
        .1
        .clone()
}
