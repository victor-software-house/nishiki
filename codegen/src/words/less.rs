//! `src/less/words.rs`: the words of the `less` grammar's big alternations,
//! each with its place in the alternation, sorted by word.
//!
//! The grammar follows each list with `\b`.
//! Several words may end where `\b` holds.
//! The matcher takes the one the alternation tries first, as
//! a backtracking matcher does, even when another is longer.
//! For example, `\b(auto|auto-fill)\b` takes the `auto` of `auto-fill`.

use std::collections::BTreeMap;
use std::path::Path;

use super::{Cut, Source, expand, table};
use crate::File;
use crate::error::Error;
use crate::header::header;

/// The longest word, which `LONGEST_WORD` in `src/less/matchers.rs` bounds.
const LONGEST_WORD: usize = 64;

/// A list that is the whole of `\b(…)\b`.
const PLAIN: Cut = Cut::Inside(r"\b(", r")\b");

/// Each table: its name, what its doc line names, and where its list sits.
/// The list is in a repository entry, at a pattern path under it.
const LISTS: &[(&str, &str, &str, &[usize], Cut)] = &[
    (
        "EXTENDED_COLORS",
        "`color-values`, the extended color names",
        "color-values",
        &[5],
        PLAIN,
    ),
    (
        "COUNTER_STYLES",
        "`counter-functions`, the counter styles",
        "counter-functions",
        &[0, 0, 4, 1],
        PLAIN,
    ),
    (
        "VALUE_KEYWORDS",
        "`property-value-constants`, the keywords",
        "property-value-constants",
        &[12],
        PLAIN,
    ),
    (
        "PROPERTY_NAMES",
        "`rule-list-body`, the property names",
        "rule-list-body",
        &[5, 13],
        PLAIN,
    ),
    (
        "TAG_NAMES",
        "`selectors`, the tag names",
        "selectors",
        &[1],
        PLAIN,
    ),
    (
        "PSEUDO_CLASSES",
        "`pseudo-selectors`, the pseudo-classes without arguments",
        "pseudo-selectors",
        &[5],
        Cut::Inside("(:)(", r")\b"),
    ),
    (
        "PSEUDO_ELEMENTS",
        "`pseudo-selectors`, the pseudo-elements",
        "pseudo-selectors",
        &[8],
        Cut::Inside("(::?)(", r")\b"),
    ),
];

/// Generates the file.
pub(super) fn file(root: &Path) -> Result<File, Error> {
    let source = Source::load(root, "less")?;
    let mut text = header("codegen/src/words/less.rs", &source.path());
    for line in [
        "//! The words that the big alternations of the `less` grammar spell out.",
        "//! Each comes with its place in the alternation and whether it ignores case,",
        "//! and they are sorted by word.",
        "",
        "use super::matchers::Word;",
    ] {
        text.push_str(line);
        text.push('\n');
    }
    for &(name, what, key, path, cut) in LISTS {
        let mut first: BTreeMap<String, (usize, bool)> = BTreeMap::new();
        for (order, word) in expand(cut.apply(source.pattern_at(key, path)?)?)?
            .into_iter()
            .enumerate()
        {
            if !word
                .text
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"-.:".contains(&b))
            {
                return Err(Error::Shape(format!(
                    "{name}: {} has a character outside [-.0-9:A-Za-z]",
                    word.text
                )));
            }
            if word.text.len() > LONGEST_WORD {
                return Err(Error::Shape(format!(
                    "{name}: {} is longer than LONGEST_WORD in src/less/matchers.rs",
                    word.text
                )));
            }
            first.entry(word.text).or_insert((order, word.folded));
        }
        let rows = first.iter().map(|(word, (order, folded))| {
            format!("Word {{ text: {word:?}, order: {order}, folded: {folded} }}")
        });
        let doc = format!("The words of {what}.");
        let declaration = format!("pub(super) const {name}: &[Word]");
        table(&mut text, &[&doc], &declaration, rows);
    }
    Ok(File {
        path: "src/less/words.rs".to_owned(),
        text,
    })
}
