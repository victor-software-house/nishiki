//! Why a code generator could not build a language's tables.

use std::{error, fmt};

/// Why [`super::flatten::flatten`] could not build a language's tables.
///
/// Each variant names the pattern id or the include that failed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GenerateError {
    /// An include names another grammar.
    Include {
        /// The id of the rule whose list holds the include.
        rule: String,
        /// The include, such as `source.css`.
        include: String,
    },
    /// A rule has no `match`, no `begin` with an `end`, and no patterns.
    NoAct {
        /// The id the rule would take.
        id: String,
    },
    /// A match named after its first group, `$1`, whose group the split
    /// cannot read as words between `|`.
    Split {
        /// The id of the match.
        id: String,
        /// The match's expression.
        regex: String,
    },
    /// Two patterns with different expressions or acts take one id, or two
    /// rules do.
    IdTaken {
        /// The id.
        id: String,
    },
}

impl fmt::Display for GenerateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GenerateError::Include { rule, include } => {
                write!(f, "{rule}: the include {include} names another grammar")
            }
            GenerateError::NoAct { id } => {
                write!(
                    f,
                    "{id}: neither a match, a begin with an end, nor patterns"
                )
            }
            GenerateError::Split { id, regex } => {
                write!(
                    f,
                    "{id}: $1 names a first group that is missing or not plain words in {regex}"
                )
            }
            GenerateError::IdTaken { id } => {
                write!(
                    f,
                    "{id}: two patterns or rules with different contents take this id"
                )
            }
        }
    }
}

impl error::Error for GenerateError {}
