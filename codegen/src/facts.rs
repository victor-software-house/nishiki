//! The facts files of `oracle/facts/`, as the writers read them.
//!
//! The Bun scripts of `oracle/` measure Shiki and Oniguruma and record what
//! they find there as JSON.
//! A code point is spelled in hex, as `00AA`, and a range of them as
//! `0660..0669`.

use std::fs;
use std::path::Path;

use serde_json::Value;

use crate::error::Error;

/// An inclusive range of code points.
pub(crate) type Range = (u32, u32);

/// The facts file at `path` under `root`, read as JSON.
pub(crate) fn read(root: &Path, path: &str) -> Result<Value, Error> {
    let file = root.join(path);
    let text = fs::read_to_string(&file).map_err(|source| Error::Io {
        path: file.clone(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| Error::Json { path: file, source })
}

/// The array `value` holds at `key`.
pub(crate) fn array<'a>(path: &str, value: &'a Value, key: &str) -> Result<&'a [Value], Error> {
    value
        .get(key)
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .ok_or_else(|| Error::Shape(format!("{path}: no array {key}")))
}

/// The ranges that `values` spell.
pub(crate) fn ranges(path: &str, values: &[Value]) -> Result<Vec<Range>, Error> {
    values
        .iter()
        .map(|value| {
            let text = string(path, value)?;
            let (lo, hi) = text.split_once("..").unwrap_or((text, text));
            Ok((point(path, lo)?, point(path, hi)?))
        })
        .collect()
}

/// The code points that `values` spell, one each.
pub(crate) fn points(path: &str, values: &[Value]) -> Result<Vec<u32>, Error> {
    values
        .iter()
        .map(|value| point(path, string(path, value)?))
        .collect()
}

/// The text of `value`, which a facts file holds as a string.
pub(crate) fn string<'a>(path: &str, value: &'a Value) -> Result<&'a str, Error> {
    value
        .as_str()
        .ok_or_else(|| Error::Shape(format!("{path}: {value} is not a string")))
}

/// The code point that `hex` spells.
fn point(path: &str, hex: &str) -> Result<u32, Error> {
    u32::from_str_radix(hex, 16)
        .map_err(|error| Error::Shape(format!("{path}: {hex} is not a hex code point: {error}")))
}
