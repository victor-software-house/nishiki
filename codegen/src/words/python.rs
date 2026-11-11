//! `src/python/words.rs`: the words of the `python` grammar's big
//! alternations, sorted.

use std::path::Path;

use super::{Cut, Source, bytes, sorted, table};
use crate::File;
use crate::error::Error;
use crate::header::header;

/// The lookbehind some lists open with, so a word after `.` is no builtin.
const NOT_AFTER_DOT: &str = r"(?<!\.)";

/// A list between word boundaries.
const BOUNDED: Cut = Cut::Inside(r"\b", r"\b");

/// Each table: its name, its doc line, the repository entry whose pattern
/// holds it, and the pattern's path under that entry.
const LISTS: &[(&str, &str, &str, &[usize])] = &[
    (
        "BUILTIN_FUNCTIONS",
        "The words of `builtin-functions`.",
        "builtin-functions",
        &[0],
    ),
    (
        "LEGACY_BUILTINS",
        "The legacy builtins.",
        "builtin-functions",
        &[1],
    ),
    (
        "BUILTIN_TYPES",
        "The words of `builtin-types`.",
        "builtin-types",
        &[],
    ),
    (
        "BUILTIN_EXCEPTIONS",
        "The words of `builtin-exceptions`.",
        "builtin-exceptions",
        &[],
    ),
    (
        "MAGIC_FUNCTIONS",
        "The words of `magic-function-names`.",
        "magic-function-names",
        &[],
    ),
    (
        "MAGIC_VARIABLES",
        "The words of `magic-variable-names`.",
        "magic-variable-names",
        &[],
    ),
];

/// Generates the file.
pub(super) fn file(root: &Path) -> Result<File, Error> {
    let source = Source::load(root, "python")?;
    let mut text = header("codegen/src/words/python.rs", &source.path());
    text.push_str("//! The words the grammar's alternations spell out, sorted.\n");
    for &(name, doc, key, path) in LISTS {
        let pattern = source.pattern_at(key, path)?;
        let pattern = pattern.strip_prefix(NOT_AFTER_DOT).unwrap_or(pattern);
        let mut words = sorted(BOUNDED.apply(pattern)?)?;
        words.retain(|word| !word.is_empty());
        let declaration = format!("pub(super) const {name}: &[&[u8]]");
        table(&mut text, &[doc], &declaration, bytes(&words));
    }
    Ok(File {
        path: "src/python/words.rs",
        text,
    })
}
