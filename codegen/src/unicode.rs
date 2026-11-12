//! The Unicode set tables: `src/textmate/regex/unicode.rs`.
//!
//! `oracle/unicode-tables.ts` asks Oniguruma, as Shiki bundles it, which code
//! points each `\p{..}` and `[[:..:]]` set holds, and records the sets in
//! `oracle/facts/unicode-sets.json`.
//! This module writes one table per set, and the sorted list of the names a
//! pattern looks the sets up by.
//! `ast::property_named` searches that list by hand.

use std::path::Path;

use crate::File;
use crate::error::Error;
use crate::facts::{Range, array, ranges, read, string};
use crate::header::header;
use crate::numbers::fill;

/// The file this module writes, from the repository root.
const PATH: &str = "src/textmate/regex/unicode.rs";

/// The facts it reads.
const FACTS: &str = "oracle/facts/unicode-sets.json";

/// The prefix of a POSIX bracket's set key, which its name drops.
const POSIX: &str = "posix_";

/// The Unicode set file, generated from the facts under `root`.
pub(crate) fn file(root: &Path) -> Result<File, Error> {
    let value = read(root, FACTS)?;
    let mut sets: Vec<(String, Vec<Range>)> = Vec::new();
    for set in array(FACTS, &value, "sets")? {
        let key = set
            .get("key")
            .ok_or_else(|| Error::Shape(format!("{FACTS}: a set has no key")))?;
        sets.push((
            string(FACTS, key)?.to_owned(),
            ranges(FACTS, array(FACTS, set, "ranges")?)?,
        ));
    }
    let names = names(sets.iter().map(|(key, _)| key.as_str()))?;
    let mut text = header("codegen/src/unicode.rs", FACTS);
    for line in [
        "//! Character sets, as Oniguruma measures them, and the names a pattern",
        "//! looks them up by.",
        "//! `oracle/unicode-tables.ts` measures them on Oniguruma as Shiki bundles it.",
        "",
        "#![cfg_attr(rustfmt, rustfmt::skip)]",
        "#![expect(clippy::unreadable_literal, reason = \"generated tables of code points\")]",
    ] {
        text.push_str(line);
        text.push('\n');
    }
    for (key, ranges) in &sets {
        text.push('\n');
        let opening = format!("static {}: &[(u32, u32)] = &[", ident(key));
        text.push_str(&opening);
        text.push('\n');
        fill(
            &mut text,
            ranges.iter().map(|&(lo, hi)| format!("({lo:#x}, {hi:#x})")),
        );
        text.push_str("];\n");
    }
    text.push('\n');
    for line in [
        "/// Each name a pattern can give a set, with the set, sorted by name.",
        "/// A name is lower case, without `_`, `-` or spaces.",
        "pub(super) static NAMES: &[(&str, &[(u32, u32)])] = &[",
    ] {
        text.push_str(line);
        text.push('\n');
    }
    fill(
        &mut text,
        names
            .iter()
            .map(|(name, key)| format!("({name:?}, {})", ident(key))),
    );
    text.push_str("];\n");
    Ok(File {
        path: PATH.to_owned(),
        text,
    })
}

/// The name of each set, with its key, sorted by name.
/// A POSIX bracket's set is named without its prefix, as `[[:alpha:]]` names
/// `posix_alpha`.
/// Two sets with one name are an error.
fn names<'a>(keys: impl Iterator<Item = &'a str>) -> Result<Vec<(&'a str, &'a str)>, Error> {
    let mut names: Vec<(&str, &str)> = keys
        .map(|key| (key.strip_prefix(POSIX).unwrap_or(key), key))
        .collect();
    names.sort_unstable();
    let repeated = names.windows(2).find_map(|pair| match pair {
        [(first, _), (second, _)] if first == second => Some(*first),
        _ => None,
    });
    if let Some(name) = repeated {
        return Err(Error::Shape(format!("{FACTS}: two sets are named {name}")));
    }
    Ok(names)
}

/// The Rust name of the table of the set `key`, as `SET_POSIX_ALPHA`.
fn ident(key: &str) -> String {
    format!("SET_{}", key.to_ascii_uppercase())
}

#[cfg(test)]
mod tests;
