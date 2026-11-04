//! The C4 diagrams' elements, relations and boundaries.
//!
//! Each element is `(?i)^\s*(WORD)\b` and, in its long form, its arguments.
//! An argument list is a fixed sequence of parts, each required or optional.
//! Every part starts with a character no part before it can take: `(`, a
//! word character, `)`, `,` or `"`.
//! So each optional part either matches or is skipped, and nothing
//! backtracks.

use super::super::engine::Lexer;
use crate::classes::{is_digit, is_word};
use crate::rules::Chars;

/// `Person(?:|_Ext)`.
pub(super) const PERSON: &[&[u8]] = &[b"Person", b"Person_Ext"];
/// `System(?:|_Ext|Db|Db_Ext|Queue|Queue_Ext)`.
pub(super) const SYSTEM: &[&[u8]] = &[
    b"System",
    b"System_Ext",
    b"SystemDb",
    b"SystemDb_Ext",
    b"SystemQueue",
    b"SystemQueue_Ext",
];
/// The relations.
pub(super) const REL: &[&[u8]] = &[
    b"Rel",
    b"BiRel",
    b"Rel_U",
    b"Rel_D",
    b"Rel_L",
    b"Rel_R",
    b"Rel_Up",
    b"Rel_Down",
    b"Rel_Left",
    b"Rel_Right",
];
/// `(?:|Enterprise_|System_)Boundary`.
pub(super) const BOUNDARY: &[&[u8]] = &[b"Boundary", b"Enterprise_Boundary", b"System_Boundary"];
/// `(?:|Enterprise_|System_|Container_)Boundary`.
pub(super) const CONTAINER_BOUNDARY: &[&[u8]] = &[
    b"Boundary",
    b"Enterprise_Boundary",
    b"System_Boundary",
    b"Container_Boundary",
];
/// The layout statements.
pub(super) const LAYOUT: &[&[u8]] = &[
    b"LAYOUT_TOP_DOWN",
    b"LAYOUT_LEFT_RIGHT",
    b"LAYOUT_LANDSCAPE",
    b"LAYOUT_WITH_LEGEND",
    b"SHOW_LEGEND",
    b"HIDE_STEREOTYPE",
];
/// `Add(?:Element|Rel|Boundary|Person|System)Tag`.
pub(super) const TAGS: &[&[u8]] = &[
    b"AddElementTag",
    b"AddRelTag",
    b"AddBoundaryTag",
    b"AddPersonTag",
    b"AddSystemTag",
];
/// `Component(?:|Db|Queue|_Ext|Db_Ext|Queue_Ext)`.
pub(super) const COMPONENT: &[&[u8]] = &[
    b"Component",
    b"ComponentDb",
    b"ComponentQueue",
    b"Component_Ext",
    b"ComponentDb_Ext",
    b"ComponentQueue_Ext",
];
/// `Container(?:|_Ext|Db|Db_Ext|Queue|Queue_Ext)`.
pub(super) const CONTAINER: &[&[u8]] = &[
    b"Container",
    b"Container_Ext",
    b"ContainerDb",
    b"ContainerDb_Ext",
    b"ContainerQueue",
    b"ContainerQueue_Ext",
];
/// `Deployment_Node|Node|Node_L|Node_R`.
pub(super) const NODE: &[&[u8]] = &[b"Deployment_Node", b"Node", b"Node_L", b"Node_R"];

/// One part of an argument list.
#[derive(Clone, Copy, Debug)]
pub(super) enum Part {
    /// `(\()`.
    Open,
    /// `(\))`.
    Close,
    /// `(\w+)`.
    Word,
    /// `(\d+)`.
    Digits,
    /// `(,\s*)`.
    Comma,
    /// `("(?:[^"\\]|\\.)*")`.
    Str,
    /// `\s*`, which is no group.
    Spaces,
}

/// Whether a part must match.
#[derive(Clone, Copy, Debug)]
pub(super) enum Need {
    /// The part must match.
    Must,
    /// The part may be skipped: `(…)?`.
    May,
}

use Need::{May, Must};
use Part::{Close, Comma, Digits, Open, Spaces, Str, Word};

/// `(\()?(\w+)(\))?(,\s*)("…")(,\s*)?("…")?(\))?`: a person's or a system's
/// arguments.
pub(super) const PERSON_ARGS: &[(Part, Need)] = &[
    (Open, May),
    (Word, Must),
    (Close, May),
    (Comma, Must),
    (Str, Must),
    (Comma, May),
    (Str, May),
    (Close, May),
];
/// `(\()?\s*(\w+)?\s*(\))?(,\s*)(\w+)(,\s*)("…")(,\s*)("…")(\))?`: a
/// relation's arguments.
pub(super) const REL_ARGS: &[(Part, Need)] = &[
    (Open, May),
    (Spaces, Must),
    (Word, May),
    (Spaces, Must),
    (Close, May),
    (Comma, Must),
    (Word, Must),
    (Comma, Must),
    (Str, Must),
    (Comma, Must),
    (Str, Must),
    (Close, May),
];
/// `(\()?\s*(\w+)?\s*(\))?(,\s*)?("…")?(,\s*)?("…")?(,\s*)?("…")?(\))?`: a
/// boundary's arguments.
pub(super) const BOUNDARY_ARGS: &[(Part, Need)] = &[
    (Open, May),
    (Spaces, Must),
    (Word, May),
    (Spaces, Must),
    (Close, May),
    (Comma, May),
    (Str, May),
    (Comma, May),
    (Str, May),
    (Comma, May),
    (Str, May),
    (Close, May),
];
/// `(\()?(\w+)?(\))?(,\s*)("…")?(,\s*)?("…")?(\))?`: a component's arguments.
pub(super) const COMPONENT_ARGS: &[(Part, Need)] = &[
    (Open, May),
    (Word, May),
    (Close, May),
    (Comma, Must),
    (Str, May),
    (Comma, May),
    (Str, May),
    (Close, May),
];
/// `(\()?(\w+)?(\))?(,\s*)("…")?(,\s*)?("…")?(,\s*)?("…")?(\))?`: a
/// container's arguments.
pub(super) const CONTAINER_ARGS: &[(Part, Need)] = &[
    (Open, May),
    (Word, May),
    (Close, May),
    (Comma, Must),
    (Str, May),
    (Comma, May),
    (Str, May),
    (Comma, May),
    (Str, May),
    (Close, May),
];
/// `(\()?(\d+)(\))?(,\s*)(\w+)(,\s*)(\w+)(,\s*)("…")(,\s*)?("…")?(\))?`: an
/// indexed relation's arguments.
pub(super) const REL_INDEX_ARGS: &[(Part, Need)] = &[
    (Open, May),
    (Digits, Must),
    (Close, May),
    (Comma, Must),
    (Word, Must),
    (Comma, Must),
    (Word, Must),
    (Comma, Must),
    (Str, Must),
    (Comma, May),
    (Str, May),
    (Close, May),
];

impl Lexer<'_> {
    /// `(?i)^\s*(WORD)\b` for the alternation `words`: the word at the start of
    /// the line, the whole run of word characters, when it is one of them.
    /// `\b` lets only the alternative that ends with the run through.
    fn c4_keyword(&self, p: usize) -> Option<(usize, usize)> {
        if p != 0 {
            return None;
        }
        let start = self.ws(p);
        let end = self.word_end(start);
        (end > start).then_some((start, end))
    }

    /// `(?i)^\s*(WORD)\b`.
    pub(super) fn c4_word(&mut self, p: usize, words: &[&[u8]]) -> Option<usize> {
        let (start, end) = self.c4_keyword(p)?;
        if !words.iter().any(|word| self.ci(start, word) == Some(end)) {
            return None;
        }
        self.cap(1, start, end);
        Some(end)
    }

    /// `(?i)^\s*(WORD)\b` and the argument list `args`, whose groups count
    /// from 2.
    pub(super) fn c4_call(
        &mut self,
        p: usize,
        words: &[&[u8]],
        args: &[(Part, Need)],
    ) -> Option<usize> {
        let end = self.c4_word(p, words)?;
        let mut q = end;
        let mut group = 2;
        for &(part, need) in args {
            let found = match part {
                Open => self.lit(q, b"("),
                Close => self.lit(q, b")"),
                Word => self.class_run1(q, is_word),
                Digits => self.class_run1(q, is_digit),
                Comma => self.lit(q, b",").map(|comma| self.ws(comma)),
                Str => self.escaped_string(q),
                Spaces => {
                    q = self.ws(q);
                    continue;
                }
            };
            match (found, need) {
                (Some(after), _) => {
                    self.cap(group, q, after);
                    q = after;
                }
                (None, Must) => return None,
                (None, May) => {}
            }
            group += 1;
        }
        Some(q)
    }
}
