//! What the languages' unit tests share: lexing text line by line as one
//! document, and reading back each token's text and scope names.

/// One lexed line: each token's text and its scope names, outermost first.
#[cfg(any(
    feature = "lang-c",
    feature = "lang-coffee",
    feature = "lang-cpp",
    feature = "lang-css",
    feature = "lang-dockerfile",
    feature = "lang-go",
    feature = "lang-ini",
    feature = "lang-java",
    feature = "lang-json",
    feature = "lang-less",
    feature = "lang-lua",
    feature = "lang-make",
    feature = "lang-matlab",
    feature = "lang-nix",
    feature = "lang-objective-c",
    feature = "lang-postcss",
    feature = "lang-proto",
    feature = "lang-python",
    feature = "lang-sass",
    feature = "lang-sql",
    feature = "lang-swift",
    feature = "lang-tex",
    feature = "lang-yaml",
    feature = "lang-zig"
))]
pub(crate) type Row = Vec<(String, Vec<String>)>;

/// Lexes `text` line by line with `highlighter` as one document from its
/// start.
/// It asserts that no line reaches a limit and that each line's tokens cover
/// it.
#[cfg(any(
    feature = "lang-c",
    feature = "lang-coffee",
    feature = "lang-cpp",
    feature = "lang-css",
    feature = "lang-dockerfile",
    feature = "lang-go",
    feature = "lang-ini",
    feature = "lang-java",
    feature = "lang-json",
    feature = "lang-less",
    feature = "lang-lua",
    feature = "lang-make",
    feature = "lang-matlab",
    feature = "lang-nix",
    feature = "lang-objective-c",
    feature = "lang-postcss",
    feature = "lang-proto",
    feature = "lang-python",
    feature = "lang-sass",
    feature = "lang-sql",
    feature = "lang-tex",
    feature = "lang-zig"
))]
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
#[cfg(any(
    feature = "lang-c",
    feature = "lang-coffee",
    feature = "lang-cpp",
    feature = "lang-css",
    feature = "lang-dockerfile",
    feature = "lang-go",
    feature = "lang-ini",
    feature = "lang-java",
    feature = "lang-json",
    feature = "lang-less",
    feature = "lang-lua",
    feature = "lang-matlab",
    feature = "lang-nix",
    feature = "lang-objective-c",
    feature = "lang-postcss",
    feature = "lang-proto",
    feature = "lang-python",
    feature = "lang-sass",
    feature = "lang-tex",
    feature = "lang-zig"
))]
pub(crate) fn lex(language: crate::Language, text: &str) -> Vec<Row> {
    lex_with(&mut crate::Highlighter::new(language), text)
}

/// `rows` without each token's outermost scope, the document's root.
#[cfg(any(feature = "lang-c", feature = "lang-make", feature = "lang-sql"))]
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
#[cfg(any(
    feature = "lang-c",
    feature = "lang-cpp",
    feature = "lang-html",
    feature = "lang-java",
    feature = "lang-json",
    feature = "lang-less",
    feature = "lang-make",
    feature = "lang-postcss",
    feature = "lang-python",
    feature = "lang-sql"
))]
pub(crate) fn scopes_in(rows: Vec<Row>, text: &str, token: &str) -> Vec<String> {
    rows.into_iter()
        .flatten()
        .find(|(t, _)| t == token)
        .unwrap_or_else(|| panic!("no token {token:?} in {text:?}"))
        .1
}

/// The scopes of the first token whose text is `token` when `text` is lexed as
/// `language`, outermost first.
#[cfg(any(
    feature = "lang-cpp",
    feature = "lang-html",
    feature = "lang-java",
    feature = "lang-json",
    feature = "lang-less",
    feature = "lang-postcss",
    feature = "lang-python"
))]
pub(crate) fn scopes_of(language: crate::Language, text: &str, token: &str) -> Vec<String> {
    scopes_in(lex(language, text), text, token)
}

/// The innermost scope of the first token whose text is `token` when `text`
/// is lexed as `language`.
#[cfg(any(feature = "lang-astro", feature = "lang-html", feature = "lang-java"))]
pub(crate) fn leaf(language: crate::Language, text: &str, token: &str) -> String {
    scopes_of(language, text, token).pop().unwrap_or_default()
}

/// One lexed line with each token cut to its innermost scope, neighbouring
/// tokens of the same scope merged: what a theme colours.
#[cfg(any(
    feature = "lang-coffee",
    feature = "lang-css",
    feature = "lang-dockerfile",
    feature = "lang-go",
    feature = "lang-ini",
    feature = "lang-less",
    feature = "lang-lua",
    feature = "lang-matlab",
    feature = "lang-nix",
    feature = "lang-objective-c",
    feature = "lang-postcss",
    feature = "lang-proto",
    feature = "lang-sass",
    feature = "lang-swift",
    feature = "lang-tex",
    feature = "lang-xml",
    feature = "lang-zig"
))]
pub(crate) type LeafRow = Vec<(String, String)>;

/// `rows` with each token cut to its innermost scope and neighbouring tokens
/// of the same innermost scope merged into one.
#[cfg(any(
    feature = "lang-coffee",
    feature = "lang-css",
    feature = "lang-dockerfile",
    feature = "lang-go",
    feature = "lang-ini",
    feature = "lang-less",
    feature = "lang-lua",
    feature = "lang-matlab",
    feature = "lang-nix",
    feature = "lang-objective-c",
    feature = "lang-postcss",
    feature = "lang-proto",
    feature = "lang-sass",
    feature = "lang-swift",
    feature = "lang-tex",
    feature = "lang-xml",
    feature = "lang-zig"
))]
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
#[cfg(any(
    feature = "lang-dockerfile",
    feature = "lang-ini",
    feature = "lang-xml"
))]
pub(crate) fn leaf_row(parts: &[(&str, &str)]) -> LeafRow {
    parts
        .iter()
        .map(|&(text, scope)| (text.to_owned(), scope.to_owned()))
        .collect()
}

/// The innermost scope of the merged token in `row` whose text is `token`.
#[cfg(any(
    feature = "lang-coffee",
    feature = "lang-css",
    feature = "lang-go",
    feature = "lang-less",
    feature = "lang-lua",
    feature = "lang-matlab",
    feature = "lang-nix",
    feature = "lang-objective-c",
    feature = "lang-postcss",
    feature = "lang-proto",
    feature = "lang-sass",
    feature = "lang-swift",
    feature = "lang-tex",
    feature = "lang-zig"
))]
pub(crate) fn leaf_in(row: &LeafRow, token: &str) -> String {
    row.iter()
        .find(|(text, _)| text == token)
        .unwrap_or_else(|| panic!("no token {token:?} in {row:?}"))
        .1
        .clone()
}

/// Lexing the lines another language embeds.
#[cfg(any(
    feature = "lang-html",
    feature = "lang-json",
    feature = "lang-less",
    feature = "lang-postcss",
    feature = "lang-xml",
    feature = "lang-yaml"
))]
pub(crate) mod embedded;

/// The tests of the languages whose matchers `codegen/` writes.
#[cfg(feature = "lang-proto")]
pub(crate) mod differential;
#[cfg(feature = "_rule-engine")]
#[cfg(any(
    feature = "json",
    feature = "lang-csharp",
    feature = "lang-proto",
    feature = "lang-swift"
))]
pub(crate) mod probe;

/// The text of every oracle fixture, with its language.
#[cfg(any(feature = "_rule-engine", feature = "lang-shell", feature = "all"))]
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
