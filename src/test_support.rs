//! What the languages' unit tests share: lexing text line by line as one
//! document, and reading back each token's text and scope names.
//!
//! Each module compiles with the languages whose tests call it.
//! A group of many languages is a private `_test-…` feature in `Cargo.toml`,
//! which those languages' features enable.

#[cfg(feature = "_test-rows")]
pub(crate) mod rows;

#[cfg(feature = "_test-scopes")]
pub(crate) mod scopes;

#[cfg(any(
    feature = "_test-leaves",
    feature = "lang-dockerfile",
    feature = "lang-ini",
    feature = "lang-xml"
))]
pub(crate) mod leaves;

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
