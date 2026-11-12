//! Why a code generator could not build a language's tables.

use std::{error, fmt};

/// Why [`super::flatten::flatten`] could not build a language's tables.
///
/// Each variant names the pattern id or the include that failed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GenerateError {
    /// An include names no rule of the grammar's repository.
    Include {
        /// The id of the rule whose list holds the include.
        rule: String,
        /// The include, such as `#strings` or `source.css`.
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
    },
    /// Two rules with different expressions take one id.
    TwoRulesOneId {
        /// The id.
        id: String,
    },
}

impl fmt::Display for GenerateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GenerateError::Include { rule, include } => {
                write!(f, "{rule}: the include {include} names no rule")
            }
            GenerateError::NoAct { id } => {
                write!(
                    f,
                    "{id}: neither a match, a begin with an end, nor patterns"
                )
            }
            GenerateError::Split { id } => {
                write!(
                    f,
                    "{id}: $1 names a first group that is missing or not plain words"
                )
            }
            GenerateError::TwoRulesOneId { id } => {
                write!(f, "{id}: two rules with different expressions take this id")
            }
        }
    }
}

impl error::Error for GenerateError {}
