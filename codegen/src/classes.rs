//! The character class tables: `src/classes/table.rs`.
//!
//! Five tables hold what Oniguruma's `\d`, `\w`, `\s`, `[[:alpha:]]` and
//! `\p{upper}` match beyond ASCII, as `oracle/classes.ts` measures them
//! through Shiki's grammars, in `oracle/facts/classes.json`.
//! The other twelve hold Unicode categories and unions of them, as
//! `oracle/unicode-tables.ts` measures them on Oniguruma, in
//! `oracle/facts/unicode-sets.json`.
//! Each table holds its set's characters among the points `classes.json`
//! lists, as sorted inclusive ranges, one to a line.
//!
//! The file holds data only.
//! Each lookup is one call of `contains`, which `src/classes.rs` holds by
//! hand.

use std::fs;
use std::path::Path;

use serde_json::Value;

use crate::File;
use crate::error::Error;
use crate::header::header;

/// The file this module writes, from the repository root.
const PATH: &str = "src/classes/table.rs";

/// The measured sets and the points they cover.
const CLASSES: &str = "oracle/facts/classes.json";

/// The Unicode categories, over every code point.
const UNICODE_SETS: &str = "oracle/facts/unicode-sets.json";

/// The file's own docs, after its header.
const DOCS: &[&str] = &[
    "The characters Oniguruma's `\\d`, `\\w`, `\\s`, `[[:alpha:]]`, `\\p{upper}`,",
    "`\\p{L}`, `\\p{N}`, `\\p{M}`, `[\\p{Lt}\\p{Lu}]`, `[\\p{Lo}\\p{Nl}\\p{Ll}]`,",
    "`[\\p{Sm}\\p{So}]`, `\\p{Ll}`, `[\\p{S}\\p{P}]`, `\\p{Nl}`, `[\\p{Mn}\\p{Mc}]` and",
    "`\\p{Pc}` match beyond ASCII, as Shiki 4.6.0 reads them.",
];

/// Where a table's characters come from.
#[derive(Clone, Copy, Debug)]
enum Members {
    /// The set of `classes.json` with this key.
    Measured(&'static str),
    /// The union of the sets of `unicode-sets.json` with these keys.
    Categories(&'static [&'static str]),
}

/// One table and the lookup that reads it.
#[derive(Clone, Copy, Debug)]
struct Class {
    /// The table's name, such as `DIGITS`.
    table: &'static str,
    /// The lookup's name, such as `is_digit`.
    lookup: &'static str,
    /// The regex the table answers for, as its docs spell it.
    regex: &'static str,
    /// Where its characters come from.
    members: Members,
    /// The features of which a build needs one to keep the table and its
    /// lookup, or none when every build keeps them.
    features: &'static [&'static str],
}

/// The languages that read `[[:alpha:]]` beyond ASCII.
const ALPHA: &[&str] = &[
    "lang-asm",
    "lang-astro",
    "lang-coffee",
    "lang-gdscript",
    "lang-hcl",
    "lang-julia",
    "lang-latex",
    "lang-python",
    "lang-svelte",
    "lang-tex",
    "lang-javascript",
    "lang-jsx",
    "lang-tsx",
    "lang-typescript",
];

/// The languages that read `\p{L}` beyond ASCII.
const LETTER: &[&str] = &[
    "lang-hack",
    "lang-julia",
    "lang-mermaid",
    "lang-powershell",
    "lang-r",
    "lang-swift",
];

/// The languages that read `\p{N}` beyond ASCII.
const NUMBER: &[&str] = &["lang-latex", "lang-powershell", "lang-swift"];

/// The tables Scala reads.
const SCALA: &[&str] = &["lang-scala"];

/// The tables Haskell reads.
const HASKELL: &[&str] = &["lang-haskell"];

/// The tables the identifiers of `r` and Julia read, behind a private feature.
const IDENTIFIER: &[&str] = &["_identifier-classes"];

/// Every table, in the file's order.
const TABLES: &[Class] = &[
    Class {
        table: "DIGITS",
        lookup: "is_digit",
        regex: "\\d",
        members: Members::Measured("digits"),
        features: &[],
    },
    Class {
        table: "WORDS",
        lookup: "is_word",
        regex: "\\w",
        members: Members::Measured("words"),
        features: &[],
    },
    Class {
        table: "SPACES",
        lookup: "is_space",
        regex: "\\s",
        members: Members::Measured("spaces"),
        features: &[],
    },
    Class {
        table: "ALPHAS",
        lookup: "is_alpha",
        regex: "[[:alpha:]]",
        members: Members::Measured("alphas"),
        features: ALPHA,
    },
    Class {
        table: "UPPERS",
        lookup: "is_upper",
        regex: "\\p{upper}",
        members: Members::Measured("uppers"),
        features: &["lang-python"],
    },
    Class {
        table: "LETTERS",
        lookup: "is_letter",
        regex: "\\p{L}",
        members: Members::Categories(&["l"]),
        features: LETTER,
    },
    Class {
        table: "NUMBERS",
        lookup: "is_number",
        regex: "\\p{N}",
        members: Members::Categories(&["n"]),
        features: NUMBER,
    },
    Class {
        table: "MARKS",
        lookup: "is_mark",
        regex: "\\p{M}",
        members: Members::Categories(&["m"]),
        features: &["lang-swift"],
    },
    Class {
        table: "TITLE_UPPERS",
        lookup: "is_title_or_upper",
        regex: "[\\p{Lt}\\p{Lu}]",
        members: Members::Categories(&["lt", "lu"]),
        features: &["lang-scala", "lang-haskell"],
    },
    Class {
        table: "LOWER_OTHERS",
        lookup: "is_lower_or_other_letter",
        regex: "[\\p{Lo}\\p{Nl}\\p{Ll}]",
        members: Members::Categories(&["lo", "nl", "ll"]),
        features: SCALA,
    },
    Class {
        table: "SYMBOLS",
        lookup: "is_symbol",
        regex: "[\\p{Sm}\\p{So}]",
        members: Members::Categories(&["sm", "so"]),
        features: SCALA,
    },
    Class {
        table: "LOWERS",
        lookup: "is_lower",
        regex: "\\p{Ll}",
        members: Members::Categories(&["ll"]),
        features: HASKELL,
    },
    Class {
        table: "TITLES",
        lookup: "is_title",
        regex: "\\p{Lt}",
        members: Members::Categories(&["lt"]),
        features: HASKELL,
    },
    Class {
        table: "SYMBOLS_OR_PUNCTUATION",
        lookup: "is_symbol_or_punctuation",
        regex: "[\\p{S}\\p{P}]",
        members: Members::Categories(&["s", "p"]),
        features: HASKELL,
    },
    Class {
        table: "LETTER_NUMBERS",
        lookup: "is_letter_number",
        regex: "\\p{Nl}",
        members: Members::Categories(&["nl"]),
        features: IDENTIFIER,
    },
    Class {
        table: "NONENCLOSING_MARKS",
        lookup: "is_nonenclosing_mark",
        regex: "[\\p{Mn}\\p{Mc}]",
        members: Members::Categories(&["mn", "mc"]),
        features: IDENTIFIER,
    },
    Class {
        table: "CONNECTORS",
        lookup: "is_connector",
        regex: "\\p{Pc}",
        members: Members::Categories(&["pc"]),
        features: IDENTIFIER,
    },
];

/// An inclusive range of code points.
type Range = (u32, u32);

/// The sets of a facts file, by key, as its `sets` array holds them.
struct Sets(Vec<(String, Vec<Range>)>);

impl Sets {
    /// The sets of the facts file `value`, read from `path`.
    fn read(path: &str, value: &Value) -> Result<Sets, Error> {
        let mut sets = Vec::new();
        for set in array(path, value, "sets")? {
            let key = set
                .get("key")
                .and_then(Value::as_str)
                .ok_or_else(|| Error::Shape(format!("{path}: a set has no key")))?;
            let ranges = ranges(path, array(path, set, "ranges")?)?;
            sets.push((key.to_owned(), ranges));
        }
        Ok(Sets(sets))
    }

    /// The ranges of the set with `key`.
    fn get(&self, path: &str, key: &str) -> Result<&[Range], Error> {
        self.0
            .iter()
            .find(|(name, _)| name == key)
            .map(|(_, ranges)| ranges.as_slice())
            .ok_or_else(|| Error::Shape(format!("{path}: no set {key}")))
    }
}

/// The table file, generated from the facts under `root`.
pub(crate) fn file(root: &Path) -> Result<File, Error> {
    let classes = read(root, CLASSES)?;
    let points = ranges(CLASSES, array(CLASSES, &classes, "points")?)?;
    let measured = Sets::read(CLASSES, &classes)?;
    let categories = Sets::read(UNICODE_SETS, &read(root, UNICODE_SETS)?)?;
    let mut lines: Vec<String> = DOCS.iter().map(|line| format!("//! {line}")).collect();
    lines.push(String::new());
    lines.push("use super::contains;".to_owned());
    for class in TABLES {
        let members = match class.members {
            Members::Measured(key) => measured.get(CLASSES, key)?.to_vec(),
            Members::Categories(keys) => {
                let mut union = Vec::new();
                for key in keys {
                    union.extend_from_slice(categories.get(UNICODE_SETS, key)?);
                }
                union
            }
        };
        lines.push(String::new());
        lines.push(format!(
            "/// The non-ASCII characters `{}` matches.",
            class.regex
        ));
        lines.extend(gate(class.features));
        lines.push(format!("const {}: &[(u32, u32)] = &[", class.table));
        for (lo, hi) in within(members, &points) {
            lines.push(format!("    (0x{lo:x}, 0x{hi:x}),"));
        }
        lines.push("];".to_owned());
    }
    for class in TABLES {
        lines.push(String::new());
        lines.push(format!(
            "/// The regex `{}` for a non-ASCII character.",
            class.regex
        ));
        lines.extend(gate(class.features));
        lines.push(format!(
            "pub(super) fn {}(c: char) -> bool {{",
            class.lookup
        ));
        lines.push(format!("    contains({}, c)", class.table));
        lines.push("}".to_owned());
    }
    let mut text = header("codegen/src/classes.rs", "oracle/facts/");
    for line in lines {
        text.push_str(&line);
        text.push('\n');
    }
    Ok(File { path: PATH, text })
}

/// The lines of a `cfg` attribute that requires one of `features`.
/// An empty `features` gives no lines.
/// More than two features go one to a line.
fn gate(features: &[&str]) -> Vec<String> {
    let named: Vec<String> = features
        .iter()
        .map(|feature| format!("feature = \"{feature}\""))
        .collect();
    match named.as_slice() {
        [] => Vec::new(),
        [one] => vec![format!("#[cfg({one})]")],
        [first, second] => vec![format!("#[cfg(any({first}, {second}))]")],
        many => {
            let mut lines = vec!["#[cfg(any(".to_owned()];
            let last = many.len() - 1;
            for (index, item) in many.iter().enumerate() {
                let comma = if index == last { "" } else { "," };
                lines.push(format!("    {item}{comma}"));
            }
            lines.push("))]".to_owned());
            lines
        }
    }
}

/// The characters of `members` that `points` hold, as sorted ranges, with
/// overlapping and adjacent ranges joined.
fn within(members: Vec<Range>, points: &[Range]) -> Vec<Range> {
    let mut clipped: Vec<Range> = members
        .into_iter()
        .flat_map(|(lo, hi)| {
            points.iter().filter_map(move |&(start, end)| {
                let (from, to) = (lo.max(start), hi.min(end));
                (from <= to).then_some((from, to))
            })
        })
        .collect();
    clipped.sort_unstable();
    let mut joined: Vec<Range> = Vec::with_capacity(clipped.len());
    for (lo, hi) in clipped {
        match joined.last_mut() {
            Some(last) if lo <= last.1.saturating_add(1) => last.1 = last.1.max(hi),
            Some(_) | None => joined.push((lo, hi)),
        }
    }
    joined
}

/// The facts file at `path` under `root`, read as JSON.
fn read(root: &Path, path: &str) -> Result<Value, Error> {
    let file = root.join(path);
    let text = fs::read_to_string(&file).map_err(|source| Error::Io {
        path: file.clone(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| Error::Json { path: file, source })
}

/// The array `value` holds at `key`.
fn array<'a>(path: &str, value: &'a Value, key: &str) -> Result<&'a [Value], Error> {
    value
        .get(key)
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .ok_or_else(|| Error::Shape(format!("{path}: no array {key}")))
}

/// The ranges that `values` spell, each as `0660..0669` or `00AA`, in hex.
fn ranges(path: &str, values: &[Value]) -> Result<Vec<Range>, Error> {
    values
        .iter()
        .map(|value| {
            let text = value
                .as_str()
                .ok_or_else(|| Error::Shape(format!("{path}: a range is not a string")))?;
            let (lo, hi) = text.split_once("..").unwrap_or((text, text));
            let point = |hex: &str| {
                u32::from_str_radix(hex, 16).map_err(|error| {
                    Error::Shape(format!(
                        "{path}: {text} is not a range of hex points: {error}"
                    ))
                })
            };
            Ok((point(lo)?, point(hi)?))
        })
        .collect()
}

#[cfg(test)]
mod tests;
