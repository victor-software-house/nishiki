//! Why codegen could not generate a file.

use std::path::PathBuf;
use std::{error, fmt, io};

use nishiki::GrammarError;
use nishiki::unstable::codegen::GenerateError;
use nishiki::unstable::regex;

/// Why codegen could not generate a file.
#[derive(Debug)]
pub(crate) enum Error {
    /// A file could not be read or written.
    Io {
        /// The file.
        path: PathBuf,
        /// What the system reported.
        source: io::Error,
    },
    /// A grammar file holds no grammar nishiki reads.
    Grammar {
        /// The grammar file.
        path: PathBuf,
        /// What nishiki reported.
        source: GrammarError,
    },
    /// A facts file holds no JSON.
    Json {
        /// The facts file.
        path: PathBuf,
        /// What the parser reported.
        source: serde_json::Error,
    },
    /// A word list does not parse as a pattern.
    Pattern {
        /// The word list's text.
        pattern: String,
        /// What the parser reported.
        source: regex::Error,
    },
    /// A grammar lacks a rule, a text or a shape that a writer reads.
    Shape(String),
    /// A check found the repository out of step with its grammars.
    Check(String),
    /// A language's tables could not be built from its grammar.
    Generate {
        /// The language, such as `go`.
        language: &'static str,
        /// What `nishiki::unstable::codegen` reported.
        source: GenerateError,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io { path, source } => write!(f, "{}: {source}", path.display()),
            Error::Grammar { path, source } => write!(f, "{}: {source}", path.display()),
            Error::Json { path, source } => write!(f, "{}: {source}", path.display()),
            Error::Pattern { pattern, source } => write!(f, "{pattern}: {source}"),
            Error::Shape(message) | Error::Check(message) => f.write_str(message),
            Error::Generate { language, source } => write!(f, "{language}: {source}"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Io { source, .. } => Some(source),
            Error::Grammar { source, .. } => Some(source),
            Error::Json { source, .. } => Some(source),
            Error::Pattern { source, .. } => Some(source),
            Error::Generate { source, .. } => Some(source),
            Error::Shape(_) | Error::Check(_) => None,
        }
    }
}
