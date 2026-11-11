//! `src/css/words.rs`: the words of the `css` grammar's big alternations, in
//! lower case, in one sorted table.
//!
//! Every list is matched without regard to case.
//! Each word carries the bits of the lists that hold it, so a matcher looks a
//! word up once for every list.

use std::collections::BTreeMap;
use std::path::Path;

use super::{Cut, Source, sorted};
use crate::File;
use crate::error::Error;
use crate::header::header;
use crate::numbers::fill;

/// A list after `(?<![-\w])(`, which ends before `(?![-\w])`.
const KEYWORDS: Cut = Cut::After(r"(?<![-\w])(", r")(?![-\w])");

/// A function's name after `(?<![-\w])(`, which ends before its `(`.
const FUNCTION: Cut = Cut::After(r"(?<![-\w])(", r")(\()");

/// Each list: its name, the repository entry whose pattern holds it, the
/// pattern's path under that entry, and where the list sits.
/// A list's bit is `1 << ` its place here.
const LISTS: &[(&str, &str, &[usize], Cut)] = &[
    (
        "PROPERTY_NAMES",
        "property-names",
        &[0],
        Cut::After(r"(?<![-\w])(?:", r")(?![-\w])"),
    ),
    ("PROPERTY_KEYWORDS", "property-keywords", &[0], KEYWORDS),
    ("COUNTER_STYLES", "property-keywords", &[1], KEYWORDS),
    (
        "FONT_FAMILIES",
        "property-keywords",
        &[3],
        Cut::After(r"(?<![-\w])(?i:", r")(?![-\w])"),
    ),
    ("COLORS_STANDARD", "color-keywords", &[0], KEYWORDS),
    ("COLORS_EXTENDED", "color-keywords", &[1], KEYWORDS),
    ("COLORS_SYSTEM", "color-keywords", &[3], KEYWORDS),
    (
        "TAG_NAMES",
        "tag-names",
        &[],
        Cut::After(r"(?<![-:\w])(?:", ")(?=[#"),
    ),
    ("FUNCTIONS", "functions", &[4], FUNCTION),
    ("FUNCTION_COLORS", "functions", &[1], FUNCTION),
    ("FUNCTION_GRADIENTS", "functions", &[2], FUNCTION),
    ("FUNCTION_TRANSFORMS", "functions", &[7], FUNCTION),
    (
        "PSEUDO_CLASSES",
        "pseudo-classes",
        &[],
        Cut::After("(:)(:*)(?:", r")(?![-\w]|\s*[;}])"),
    ),
    (
        "PSEUDO_ELEMENTS",
        "pseudo-elements",
        &[],
        Cut::After("(?:(::?)(?:", "|(?:-(?:"),
    ),
    (
        "PSEUDO_ELEMENTS_DOUBLE",
        "pseudo-elements",
        &[],
        Cut::After("|(::)(?:", r"))(?![-\w]|\s*[;}])"),
    ),
    (
        "VENDORS",
        "pseudo-elements",
        &[],
        Cut::After("|(?:-(?:", ")|(?:mso|prince))-"),
    ),
];

/// Generates the file.
pub(super) fn file(root: &Path) -> Result<File, Error> {
    let source = Source::load(root, "css")?;
    let mut bits: BTreeMap<String, u32> = BTreeMap::new();
    for (index, &(_, key, path, cut)) in (0u32..).zip(LISTS) {
        for word in sorted(cut.apply(source.pattern_at(key, path)?)?)? {
            *bits.entry(word.to_ascii_lowercase()).or_default() |= 1 << index;
        }
    }
    let mut text = header("codegen/src/words/css.rs", &source.path());
    for line in [
        "//! The words that the big alternations of the `css` grammar spell out, lower",
        "//! case and sorted, in one table with the lists that hold each.",
        "",
        "/// A word list of the grammar.",
        "#[derive(Clone, Copy, Debug, PartialEq, Eq)]",
        "pub(in crate::css) enum List {",
    ] {
        text.push_str(line);
        text.push('\n');
    }
    for &(name, ..) in LISTS {
        text.push_str("    ");
        text.push_str(&variant(name));
        text.push_str(",\n");
    }
    for line in [
        "}",
        "",
        "impl List {",
        "    /// The bit of the list in [`WORDS`].",
        "    pub(in crate::css) const fn bit(self) -> u32 {",
        "        match self {",
    ] {
        text.push_str(line);
        text.push('\n');
    }
    for (index, &(name, ..)) in LISTS.iter().enumerate() {
        text.push_str("            List::");
        text.push_str(&variant(name));
        text.push_str(" => 1 << ");
        text.push_str(&index.to_string());
        text.push_str(",\n");
    }
    for line in [
        "        }",
        "    }",
        "}",
        "",
        "/// Every word of the lists, sorted, with the bits of the lists that hold it.",
        "pub(in crate::css) const WORDS: &[(&str, u32)] = &[",
    ] {
        text.push_str(line);
        text.push('\n');
    }
    fill(
        &mut text,
        bits.iter()
            .map(|(word, mask)| format!("({word:?}, {mask})")),
    );
    text.push_str("];\n");
    Ok(File {
        path: "src/css/words.rs",
        text,
    })
}

/// The enum variant of the list named `name`, such as `PropertyNames` for
/// `PROPERTY_NAMES`.
fn variant(name: &str) -> String {
    name.split('_')
        .map(|part| {
            let lower = part.to_ascii_lowercase();
            let mut chars = lower.chars();
            let first = chars.next().map(|c| c.to_ascii_uppercase());
            first.into_iter().chain(chars).collect::<String>()
        })
        .collect()
}
