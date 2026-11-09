//! The delta traces of `bench/traces/`, as the stream program replays them.
//! `bench/traces/README.md` has the format.

use std::fs;
use std::io::{Error, ErrorKind, Result};
use std::ops::Range;
use std::path::Path;

use nishiki::Language;

/// What a trace stands for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// A model's reply, in token-sized deltas.
    Reply,
    /// A file a tool writes, in token-sized deltas.
    ToolWrite,
    /// A file's history, one replacement per hunk.
    Edits,
}

impl Kind {
    /// The kind's folder name.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Kind::Reply => "reply",
            Kind::ToolWrite => "tool-write",
            Kind::Edits => "edits",
        }
    }
}

/// One step of a trace.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Step {
    /// Append these bytes of the trace's text.
    Push(Range<usize>),
    /// Replace this byte range of the document with the text.
    Replace(Range<usize>, String),
}

/// One trace: what it stands for, its language, its text, and its steps.
/// An append trace pushes slices of its text.
/// An edit trace starts from its text and replaces ranges.
#[derive(Debug)]
pub struct Trace {
    /// `<kind>/<file>`.
    pub name: String,
    /// What the trace stands for.
    pub kind: Kind,
    /// The language its header names.
    pub language: Language,
    /// The whole text an append trace pushes, or an edit trace's first text.
    pub text: String,
    /// The steps, in order.
    pub steps: Vec<Step>,
}

fn invalid(path: &Path, message: &str) -> Error {
    Error::new(
        ErrorKind::InvalidData,
        format!("{}: {message}", path.display()),
    )
}

/// Every trace of `bench/traces/`, sorted by kind folder and file name.
///
/// # Errors
///
/// When a trace or its source cannot be read, or a trace is malformed.
pub fn traces() -> Result<Vec<Trace>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut traces = Vec::new();
    for kind in [Kind::Reply, Kind::ToolWrite, Kind::Edits] {
        let mut files: Vec<_> = fs::read_dir(root.join("traces").join(kind.name()))?
            .map(|entry| entry.map(|entry| entry.path()))
            .collect::<Result<_>>()?;
        files.sort();
        for path in files {
            traces.push(read(root, &path, kind)?);
        }
    }
    Ok(traces)
}

/// Reads one trace.
fn read(root: &Path, path: &Path, kind: Kind) -> Result<Trace> {
    let bytes = fs::read(path)?;
    let mut at = 0;
    let mut language = None;
    let mut source = None;
    let mut body = None;
    while at < bytes.len() {
        let end = at
            + bytes[at..]
                .iter()
                .position(|&byte| byte == b'\n')
                .ok_or_else(|| invalid(path, "a header line has no line feed"))?;
        let row = std::str::from_utf8(&bytes[at..end])
            .map_err(|error| Error::new(ErrorKind::InvalidData, error))?;
        at = end + 1;
        let (key, value) = row.split_once(' ').unwrap_or((row, ""));
        match key {
            "language" => language = Language::from_name(value),
            "source" => source = Some(value.to_owned()),
            "deltas" | "base" => {
                body = Some((key == "deltas", value.to_owned()));
                break;
            }
            _ => {}
        }
    }
    let language = language.ok_or_else(|| invalid(path, "no language nishiki lexes"))?;
    let (appends, value) = body.ok_or_else(|| invalid(path, "no body"))?;
    let name = format!(
        "{}/{}",
        kind.name(),
        path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default()
    );
    let rest = &bytes[at..];
    if appends {
        let source = source.ok_or_else(|| invalid(path, "no source"))?;
        let text = fs::read_to_string(root.join("..").join(&source))?;
        let mut steps = Vec::new();
        let mut from = 0;
        for length in std::str::from_utf8(rest)
            .map_err(|error| Error::new(ErrorKind::InvalidData, error))?
            .split_ascii_whitespace()
        {
            let length: usize = length
                .parse()
                .map_err(|_| invalid(path, "a delta is not a number"))?;
            steps.push(Step::Push(from..from + length));
            from += length;
        }
        if from != text.len() {
            return Err(invalid(path, "the deltas do not add up to the source"));
        }
        return Ok(Trace {
            name,
            kind,
            language,
            text,
            steps,
        });
    }
    let length: usize = value
        .parse()
        .map_err(|_| invalid(path, "the base length is not a number"))?;
    let mut at = 0;
    let base = blob(path, rest, &mut at, length)?;
    let mut steps = Vec::new();
    while at < rest.len() {
        let end = at
            + rest[at..]
                .iter()
                .position(|&byte| byte == b'\n')
                .ok_or_else(|| invalid(path, "a replace line has no line feed"))?;
        let row = std::str::from_utf8(&rest[at..end])
            .map_err(|error| Error::new(ErrorKind::InvalidData, error))?;
        at = end + 1;
        let fields: Vec<usize> = row
            .split(' ')
            .skip(1)
            .map(str::parse)
            .collect::<std::result::Result<_, _>>()
            .map_err(|_| invalid(path, "a replace line is not numbers"))?;
        let [start, end, length] = fields[..] else {
            return Err(invalid(path, "a replace line needs three numbers"));
        };
        let text = blob(path, rest, &mut at, length)?;
        steps.push(Step::Replace(start..end, text));
    }
    Ok(Trace {
        name,
        kind,
        language,
        text: base,
        steps,
    })
}

/// The `length` bytes at `at` as text, and the line feed after them.
fn blob(path: &Path, bytes: &[u8], at: &mut usize, length: usize) -> Result<String> {
    let text = bytes
        .get(*at..*at + length)
        .ok_or_else(|| invalid(path, "a text runs past the end"))?;
    if bytes.get(*at + length) != Some(&b'\n') {
        return Err(invalid(path, "a text does not end with a line feed"));
    }
    *at += length + 1;
    String::from_utf8(text.to_vec()).map_err(|error| Error::new(ErrorKind::InvalidData, error))
}
