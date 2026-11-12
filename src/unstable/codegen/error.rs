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
    /// A rule has no `match`, no `begin` with an `end` or a `while`, and no
    /// patterns.
    NoAct {
        /// The id the rule would take.
        id: String,
    },
    /// A match or begin named after a group, such as `$1`, whose group the
    /// split cannot read as words between `|`.
    /// Names that hold several groups are refused too.
    Split {
        /// The id of the pattern.
        id: String,
        /// The pattern's expression.
        regex: String,
    },
    /// A pattern of a split named by its word, as
    /// [`super::tables::SplitIds::Words`] says, whose alternative starts with
    /// no letter.
    NoWord {
        /// The id of the split pattern.
        id: String,
        /// The pattern's expression.
        regex: String,
    },
    /// Two patterns with different expressions or acts take one id, or two
    /// rules do.
    IdTaken {
        /// The id.
        id: String,
    },
    /// An include read as [`super::tables::External::Pattern`] names no
    /// match without capture lists.
    /// Its grammar was not passed, its key is missing, or its entry is
    /// something else.
    ExternalPattern {
        /// The include, such as `source.ruby#escaped_char`.
        include: String,
    },
    /// The other grammar does not inject what the language's
    /// [`super::tables::Injection`] names.
    /// That grammar was not passed, it injects nothing under the selector,
    /// or it injects a number of patterns other than one match per id.
    Injection {
        /// The selector, such as `L:(source.ts, source.js, source.coffee)`.
        selector: String,
    },
    /// The grammars passed hold no list where an
    /// [`super::tables::AddedRule`] points.
    /// The grammar was not passed, it lacks the key, or a position along the
    /// path is missing.
    AddedRule {
        /// The added rule's id, such as `VueStyleAttribute`.
        id: String,
    },
    /// A begin whose own list holds only missing grammars has end captures
    /// that add rules.
    /// The walk drops only a begin, its end and its rule.
    DroppedBeginCaptures {
        /// The begin's id.
        id: String,
    },
    /// A rule embeds another grammar in a body of its own, and lists
    /// patterns too.
    EmbedsBesidePatterns {
        /// The rule's id.
        id: String,
    },
    /// A pattern to merge is no match of plain words between `(?i)\b` and
    /// `\b`, or a capture of it names a scope or lists patterns.
    NotWords {
        /// The pattern's id.
        id: String,
    },
    /// A match of plain words follows the merged matches in `Root`, so the
    /// merge would leave it out.
    WordsFollow {
        /// The match's id.
        id: String,
    },
    /// A rule lists a merged match again, or after the merge's place.
    MergedElsewhere {
        /// The rule's id.
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
                    "{id}: neither a match, a begin with an end or a while, nor patterns"
                )
            }
            GenerateError::Split { id, regex } => {
                write!(
                    f,
                    "{id}: the scope names name no one group of plain words in {regex}"
                )
            }
            GenerateError::NoWord { id, regex } => {
                write!(f, "{id}: an alternative of {regex} starts with no letter")
            }
            GenerateError::IdTaken { id } => {
                write!(
                    f,
                    "{id}: two patterns or rules with different contents take this id"
                )
            }
            GenerateError::ExternalPattern { include } => {
                write!(
                    f,
                    "{include}: no match without capture lists in the grammars passed"
                )
            }
            GenerateError::Injection { selector } => {
                write!(
                    f,
                    "{selector}: the grammars passed inject no match for each id there"
                )
            }
            GenerateError::AddedRule { id } => {
                write!(f, "{id}: the grammars passed hold no list at its path")
            }
            GenerateError::DroppedBeginCaptures { id } => {
                write!(f, "{id}: the begin to drop has end captures that add rules")
            }
            GenerateError::EmbedsBesidePatterns { id } => {
                write!(
                    f,
                    "{id}: the rule embeds a grammar in a body and lists patterns too"
                )
            }
            GenerateError::NotWords { id } => {
                write!(f, "{id}: the pattern to merge is no plain word match")
            }
            GenerateError::WordsFollow { id } => {
                write!(f, "{id}: a plain word match follows the merged ones")
            }
            GenerateError::MergedElsewhere { id } => {
                write!(f, "{id}: the rule lists a merged match outside the merge")
            }
        }
    }
}

impl error::Error for GenerateError {}
