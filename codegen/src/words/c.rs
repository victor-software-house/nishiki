//! `src/c/words.rs`: the words of the `c` grammar's big alternations, sorted.

use std::path::Path;

use super::{Cut, Source, quoted, sorted, table};
use crate::File;
use crate::error::Error;
use crate::header::header;

/// A list that is the whole of `\b(…)\b`.
const PLAIN: Cut = Cut::Inside(r"\b(", r")\b");

/// The types of a storage rule, which end the pattern in `(?!\w)`.
const STORAGE: Cut = Cut::After("(?:", r")(?!\w))");

/// The words a rule excludes before a call's `(`.
const EXCLUSION: Cut = Cut::Until("(?!(?:", r")\s*\()");

/// Each table: its name, the repository entry whose pattern holds it, the
/// path to that pattern under the entry, and where the list sits.
/// An empty path names the entry itself.
const LISTS: &[(&str, &str, &[usize], Cut)] = &[
    ("CONTROL", "anon_pattern_1", &[], PLAIN),
    ("MODIFIER", "anon_pattern_3", &[], PLAIN),
    ("CONSTANT", "anon_pattern_7", &[], PLAIN),
    ("SYS_TYPES", "anon_pattern_8", &[], PLAIN),
    ("MAC_CONSTANT", "anon_pattern_11", &[], PLAIN),
    ("MAC_TYPE", "anon_pattern_12", &[], PLAIN),
    ("PRIMITIVE", "storage_types", &[0], STORAGE),
    ("BUILTIN", "storage_types", &[1], STORAGE),
    ("STDINT", "anon_pattern_10", &[], PLAIN),
    ("PTHREAD", "anon_pattern_9", &[], PLAIN),
    ("PREDEF", "predefined_macros", &[0], PLAIN),
    (
        "FUNCDEF_EXCL",
        "anon_pattern_range_7",
        &[],
        Cut::Until(r"(?<!\w)(?!\s*(?:", r")\s*\()"),
    ),
    (
        "MEMBER_EXCL",
        "member_access",
        &[],
        Cut::After("(?!(?:", r")\b)[A-Z_a-z]"),
    ),
    ("CALL_EXCL", "c_function_call", &[], EXCLUSION),
    (
        "DEFINE_EXCL",
        "preprocessor-rule-define-line-contents",
        &[4],
        EXCLUSION,
    ),
];

/// Generates the file.
pub(super) fn file(root: &Path) -> Result<File, Error> {
    let source = Source::load(root, "c")?;
    let mut text = header("codegen/src/words/c.rs", &source.path());
    text.push_str("//! The words the grammar's alternations spell out, sorted.\n");
    for &(name, key, path, cut) in LISTS {
        let words = sorted(cut.apply(source.pattern_at(key, path)?)?)?;
        let declaration = format!("pub(super) const {name}: &[&str]");
        table(&mut text, &[], &declaration, quoted(&words));
    }
    Ok(File {
        path: "src/c/words.rs".to_owned(),
        text,
    })
}
