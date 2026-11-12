//! Lexing a text line by line as one document, into each token's text and
//! scope names.

/// One lexed line: each token's text and its scope names, outermost first.
pub(crate) type Row = Vec<(String, Vec<String>)>;

/// Lexes `text` line by line with `highlighter` as one document from its
/// start.
/// It asserts that no line reaches a limit and that each line's tokens cover
/// it.
pub(crate) fn lex_with(highlighter: &mut crate::Highlighter, text: &str) -> Vec<Row> {
    let mut state = crate::State::default();
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
pub(crate) fn lex(language: crate::Language, text: &str) -> Vec<Row> {
    lex_with(&mut crate::Highlighter::new(language), text)
}

/// `rows` without each token's outermost scope, the document's root.
#[cfg(any(feature = "lang-make", feature = "lang-sql"))]
pub(crate) fn without_root(rows: Vec<Row>) -> Vec<Row> {
    rows.into_iter()
        .map(|row| {
            row.into_iter()
                .map(|(text, scopes)| (text, scopes.into_iter().skip(1).collect()))
                .collect()
        })
        .collect()
}
