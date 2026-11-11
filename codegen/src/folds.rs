//! The case fold tables: `src/textmate/regex/folds.rs`.
//!
//! `oracle/case-folds.ts` asks Oniguruma, as Shiki bundles it, which
//! characters `(?i)` takes as one, and records each orbit and each fold of
//! more than one character in `oracle/facts/case-folds.json`.
//! This module derives the tables the regex engine reads from them.
//! A character's canonical form is the smallest member of its orbit.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::Path;

use crate::File;
use crate::error::Error;
use crate::facts::{array, points, read};
use crate::header::header;
use crate::numbers::{INDENT, WIDTH, fill, fill_at};

/// The file this module writes, from the repository root.
const PATH: &str = "src/textmate/regex/folds.rs";

/// The facts it reads.
const FACTS: &str = "oracle/facts/case-folds.json";

/// What `case-folds.json` records.
#[derive(Debug)]
struct Folds {
    /// Each orbit of more than one character, its members ascending.
    orbits: Vec<Vec<u32>>,
    /// Each character whose fold is more than one character, with that fold,
    /// by character.
    folds: Vec<(u32, Vec<u32>)>,
}

/// The tables the file holds.
#[derive(Debug, PartialEq, Eq)]
struct Tables {
    /// The most members an orbit has.
    largest: usize,
    /// Each member of an orbit, with the next member, by member.
    next: Vec<(u32, u32)>,
    /// Each character whose fold is more than one character, with that fold
    /// in canonical characters, by character.
    multi: Vec<(u32, Vec<u32>)>,
    /// Each canonical fold of more than one character, with the characters
    /// that fold to it, by fold.
    unfold: Vec<(Vec<u32>, Vec<u32>)>,
    /// Per byte, one more than the index in `wide_tails` of the characters
    /// whose UTF-8 form it starts, or 0.
    wide_lead: Vec<u8>,
    /// Per first byte, the second bytes its characters can have, as a bit per
    /// value of the low six bits.
    wide_second: Vec<u64>,
    /// Per first byte, the bytes after it of each character, ascending.
    wide_tails: Vec<Vec<u16>>,
}

/// The folds file, generated from the facts under `root`.
pub(crate) fn file(root: &Path) -> Result<File, Error> {
    let folds = Folds::read(root)?;
    let tables = tables(&folds)?;
    let mut text = header("codegen/src/folds.rs", FACTS);
    for line in [
        "//! The characters `(?i)` takes as one, and the folds of more than one",
        "//! character.",
        "//! `oracle/case-folds.ts` measures them on Oniguruma as Shiki bundles it.",
        "",
        "#![cfg_attr(rustfmt, rustfmt::skip)]",
        "",
        "/// The most members an orbit has.",
    ] {
        text.push_str(line);
        text.push('\n');
    }
    text.push_str("pub(super) const LARGEST_ORBIT: usize = ");
    text.push_str(&tables.largest.to_string());
    text.push_str(";\n");
    table(
        &mut text,
        &[
            "Every character that `(?i)` takes as another, and the next member of its",
            "orbit, by character.",
            "Following `next` from a member visits its whole orbit and comes back.",
        ],
        "pub(super) static NEXT: &[(u32, u32)] = &[",
        tables
            .next
            .iter()
            .map(|&(a, b)| format!("({a:#x}, {b:#x})")),
    );
    table(
        &mut text,
        &[
            "Every character whose fold is more than one character, with that fold, by",
            "character.",
            "A fold's characters are canonical.",
            "Each is the smallest member of its orbit.",
        ],
        "pub(super) static MULTI: &[(u32, &[u32])] = &[",
        tables
            .multi
            .iter()
            .map(|(c, fold)| format!("({c:#x}, {})", list(fold))),
    );
    table(
        &mut text,
        &[
            "Every fold of more than one character, with the characters that fold to",
            "it, by fold.",
        ],
        "pub(super) static UNFOLD: &[(&[u32], &[u32])] = &[",
        tables
            .unfold
            .iter()
            .map(|(fold, chars)| format!("({}, {})", list(fold), list(chars))),
    );
    table(
        &mut text,
        &[
            "A value per byte.",
            "Some bytes start the UTF-8 form of a character beyond ASCII that a fold of",
            "more than one character involves.",
            "Such a byte holds one more than the index of its group in [`WIDE_TAILS`].",
            "Every other byte holds 0.",
        ],
        "pub(super) static WIDE_LEAD: [u8; 256] = [",
        tables.wide_lead.iter().map(u8::to_string),
    );
    table(
        &mut text,
        &[
            "Per first byte of [`WIDE_LEAD`], the second bytes its characters can",
            "have, as a bit per value of the second byte's low six bits.",
        ],
        "pub(super) static WIDE_SECOND: &[u64] = &[",
        tables.wide_second.iter().map(|&mask| grouped(mask)),
    );
    head(
        &mut text,
        &[
            "Per first byte of [`WIDE_LEAD`], the bytes after it of each such",
            "character, ascending: the second byte of a two-byte form, or the second",
            "and third of a three-byte form as one number, the second high.",
        ],
        "pub(super) static WIDE_TAILS: &[&[u16]] = &[",
    );
    for tails in &tables.wide_tails {
        groups(&mut text, tails);
    }
    text.push_str("];\n");
    Ok(File { path: PATH, text })
}

impl Folds {
    /// The orbits and folds of `case-folds.json` under `root`.
    fn read(root: &Path) -> Result<Folds, Error> {
        let value = read(root, FACTS)?;
        let orbits = array(FACTS, &value, "orbits")?
            .iter()
            .map(|orbit| {
                let members = orbit
                    .as_array()
                    .ok_or_else(|| Error::Shape(format!("{FACTS}: an orbit is not an array")))?;
                points(FACTS, members)
            })
            .collect::<Result<_, _>>()?;
        let folds = array(FACTS, &value, "folds")?
            .iter()
            .map(|fold| {
                let char = fold
                    .get("char")
                    .ok_or_else(|| Error::Shape(format!("{FACTS}: a fold has no char")))?;
                let [c] = points(FACTS, std::slice::from_ref(char))?[..] else {
                    return Err(Error::Shape(format!("{FACTS}: a fold's char is one point")));
                };
                Ok((c, points(FACTS, array(FACTS, fold, "fold")?)?))
            })
            .collect::<Result<_, _>>()?;
        Ok(Folds { orbits, folds })
    }
}

/// The tables of `folds`, as `oracle/case-folds.ts` derived them.
fn tables(folds: &Folds) -> Result<Tables, Error> {
    let mut orbit_of: HashMap<u32, &[u32]> = HashMap::new();
    for orbit in &folds.orbits {
        for &member in orbit {
            orbit_of.insert(member, orbit);
        }
    }
    let mut next = Vec::new();
    for orbit in &folds.orbits {
        for (index, &member) in orbit.iter().enumerate() {
            let following = orbit.get((index + 1) % orbit.len()).copied();
            next.push((member, following.unwrap_or(member)));
        }
    }
    next.sort_unstable();
    let largest = folds.orbits.iter().map(Vec::len).max().unwrap_or(0);
    let canonical = |c: u32| {
        orbit_of
            .get(&c)
            .and_then(|orbit| orbit.first())
            .copied()
            .unwrap_or(c)
    };
    let mut multi: Vec<(u32, Vec<u32>)> = folds
        .folds
        .iter()
        .map(|(c, fold)| (*c, fold.iter().map(|&f| canonical(f)).collect()))
        .collect();
    multi.sort_unstable();
    let mut unfold: BTreeMap<Vec<u32>, Vec<u32>> = BTreeMap::new();
    for (c, fold) in &multi {
        unfold.entry(fold.clone()).or_default().push(*c);
    }
    let unfold = unfold
        .into_iter()
        .map(|(fold, mut chars)| {
            chars.sort_unstable();
            (fold, chars)
        })
        .collect();
    let mut wide = BTreeSet::new();
    for (c, fold) in &multi {
        for each in std::iter::once(c).chain(fold) {
            let members = orbit_of
                .get(each)
                .copied()
                .unwrap_or(std::slice::from_ref(each));
            wide.extend(members.iter().copied().filter(|&member| member > 0x7f));
        }
    }
    let mut tails_of: BTreeMap<u8, Vec<u16>> = BTreeMap::new();
    for c in wide {
        let (lead, tail) = utf8_split(c)?;
        tails_of.entry(lead).or_default().push(tail);
    }
    let mut wide_lead = vec![0u8; 256];
    let mut wide_second = Vec::new();
    let mut wide_tails = Vec::new();
    for (index, (lead, mut tails)) in tails_of.into_iter().enumerate() {
        let slot = wide_lead
            .get_mut(usize::from(lead))
            .ok_or_else(|| Error::Shape(format!("no lead byte {lead}")))?;
        *slot = u8::try_from(index + 1)
            .map_err(|error| Error::Shape(format!("{index} lead bytes: {error}")))?;
        tails.sort_unstable();
        let mask = tails.iter().fold(0u64, |mask, &tail| {
            let second = if tail > 0xff { tail >> 8 } else { tail };
            mask | (1u64 << (second & 0x3f))
        });
        wide_second.push(mask);
        wide_tails.push(tails);
    }
    Ok(Tables {
        largest,
        next,
        multi,
        unfold,
        wide_lead,
        wide_second,
        wide_tails,
    })
}

/// The first byte of `c`'s UTF-8 form, and the bytes after it as one number.
/// A two-byte form gives its second byte.
/// A three-byte form gives its second and third, the second high.
fn utf8_split(c: u32) -> Result<(u8, u16), Error> {
    let character = char::from_u32(c).ok_or_else(|| Error::Shape(format!("{c:#x} is no char")))?;
    let mut buffer = [0u8; 4];
    match *character.encode_utf8(&mut buffer).as_bytes() {
        [lead, second] => Ok((lead, u16::from(second))),
        [lead, second, third] => Ok((lead, (u16::from(second) << 8) | u16::from(third))),
        _ => Err(Error::Shape(format!(
            "U+{c:X} is not two or three bytes of UTF-8"
        ))),
    }
}

/// Appends a blank line, `doc` as doc lines, then `declaration`, `items` filled
/// into lines, and the closing `];`.
fn table(
    text: &mut String,
    doc: &[&str],
    declaration: &str,
    items: impl IntoIterator<Item = String>,
) {
    head(text, doc, declaration);
    fill(text, items);
    text.push_str("];\n");
}

/// Appends a blank line, `doc` as doc lines, then `declaration` on a line.
fn head(text: &mut String, doc: &[&str], declaration: &str) {
    text.push('\n');
    for line in doc {
        text.push_str("/// ");
        text.push_str(line);
        text.push('\n');
    }
    text.push_str(declaration);
    text.push('\n');
}

/// Appends the group `tails` of `WIDE_TAILS` on a line of its own.
/// A group wider than a line opens a block of filled lines.
fn groups(text: &mut String, tails: &[u16]) {
    let line = format!("{INDENT}{},", list(tails));
    if line.chars().count() <= WIDTH {
        text.push_str(&line);
        text.push('\n');
        return;
    }
    text.push_str(INDENT);
    text.push_str("&[\n");
    fill_at(
        text,
        tails.iter().map(|tail| format!("{tail:#x}")),
        "        ",
    );
    text.push_str(INDENT);
    text.push_str("],\n");
}

/// `values` as a slice literal of hex numbers, such as `&[0x53, 0x53]`.
fn list<T: Into<u32> + Copy>(values: &[T]) -> String {
    let items: Vec<String> = values
        .iter()
        .map(|&value| format!("{:#x}", value.into()))
        .collect();
    format!("&[{}]", items.join(", "))
}

/// `mask` in hex, its digits in groups of four from the right, as
/// `0x8000_0000`.
fn grouped(mask: u64) -> String {
    let digits = format!("{mask:x}");
    let mut out = String::new();
    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index) % 4 == 0 {
            out.push('_');
        }
        out.push(digit);
    }
    format!("0x{out}")
}

#[cfg(test)]
mod tests;
