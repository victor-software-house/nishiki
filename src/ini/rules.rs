//! The grammar's rules as hand-written searches.
//!
//! Each rule is a regular expression, tried at every position of a line.
//! The line's next token is the match that starts first.
//! When two matches start together, the rule that comes first in the grammar
//! wins.
//!
//! A search below answers where its rule first matches at or after a
//! position, with the regular expression's backtracking taken into account.
//! The lexer can then keep each rule's next match until the cursor passes it.

use memchr::memchr;

use super::Kind;
use super::scope::S;
use crate::classes::{space_width, word_before, word_width};
use crate::limits::Steps;

/// A rule's first match at or after a position.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Hit {
    /// A comment that starts with `#` or `;`: where the match starts, and
    /// where the mark is.
    /// The match starts at byte 0 when blanks lead the line.
    /// Otherwise it starts at the mark.
    Comment {
        start: usize,
        mark: usize,
        kind: CommentMark,
    },
    /// A key: its bounds, and the `=` that follows it after any blanks.
    Key {
        start: usize,
        end: usize,
        equals: usize,
    },
    /// A section header `[...]`: the `]` that closes it.
    Section { close: usize },
    /// A quote that opens a string.
    Quote { at: usize, quote: Quote },
}

/// The two comment marks, each with its own scope.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum CommentMark {
    /// `#`.
    Hash,
    /// `;`.
    Semicolon,
}

impl CommentMark {
    fn byte(self) -> u8 {
        match self {
            CommentMark::Hash => b'#',
            CommentMark::Semicolon => b';',
        }
    }

    pub(super) fn scope(self) -> S {
        match self {
            CommentMark::Hash => S::CommentHash,
            CommentMark::Semicolon => S::CommentSemicolon,
        }
    }
}

/// The two quotes that open a string.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Quote {
    /// `'`, whose strings have backslash escapes.
    Single,
    /// `"`, whose strings have none.
    Double,
}

impl Quote {
    pub(super) fn byte(self) -> u8 {
        match self {
            Quote::Single => b'\'',
            Quote::Double => b'"',
        }
    }

    /// Whether a backslash in the string starts an escape.
    pub(super) fn escapes(self) -> bool {
        match self {
            Quote::Single => true,
            Quote::Double => false,
        }
    }

    /// The frame kind and the scope of the string it opens.
    pub(super) fn kind(self) -> (Kind, S) {
        match self {
            Quote::Single => (Kind::Single, S::Single),
            Quote::Double => (Kind::Double, S::Double),
        }
    }
}

impl Hit {
    /// Where the match starts.
    pub(super) fn start(self) -> usize {
        match self {
            Hit::Comment { start, .. } | Hit::Key { start, .. } => start,
            Hit::Section { .. } => 0,
            Hit::Quote { at, .. } => at,
        }
    }
}

/// A byte of the key class `[-.0-9A-Z_a-z]`.
fn is_key_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || matches!(b, b'-' | b'.' | b'_')
}

/// A line and the bytes its searches have read.
pub(super) struct Line<'a> {
    s: &'a [u8],
    /// The work of the line, which the scans spend.
    steps: &'a Steps,
}

impl<'a> Line<'a> {
    pub(super) fn new(s: &'a [u8], steps: &'a Steps) -> Self {
        Line { s, steps }
    }

    /// The width of the white space character at `i`, 0 when there is none.
    /// The line feed after the line counts.
    fn space_at(&self, i: usize) -> usize {
        if i == self.s.len() {
            1
        } else {
            space_width(self.s, i)
        }
    }

    /// The end of the run of white space from `i`.
    fn spaces(&mut self, mut i: usize) -> usize {
        let from = i;
        while let width @ 1.. = self.space_at(i) {
            i += width;
        }
        self.steps.spend(i - from);
        i
    }

    /// Whether the character at `i` is one `\w` matches.
    fn word_at(&self, i: usize) -> bool {
        word_width(self.s, i) > 0
    }

    /// The regex `\b` at `i`.
    fn boundary(&self, i: usize) -> bool {
        word_before(self.s, i) != self.word_at(i)
    }

    /// `(^[\t ]+)?(?=#)` and its twin for `;`.
    /// The first match from `from` is at the first `mark` byte.
    /// It is at the line's start instead when only tabs and spaces lie before
    /// the mark and the search starts there.
    pub(super) fn comment(&mut self, from: usize, kind: CommentMark) -> Option<Hit> {
        let rest = self.s.get(from..)?;
        let at = from + memchr(kind.byte(), rest)?;
        self.steps.spend(at - from);
        let leading =
            from == 0 && at > 0 && self.s[..at].iter().all(|&b| matches!(b, b'\t' | b' '));
        Some(Hit::Comment {
            start: if leading { 0 } else { at },
            mark: at,
            kind,
        })
    }

    /// `\b([-.0-9A-Z_a-z]+)\b\s*(=)`
    ///
    /// The key is a run of class bytes.
    /// Backtracking cannot shorten it.
    /// The byte after a shorter key is a class byte, which is neither a blank
    /// nor `=`.
    ///
    /// `\b` before the blanks needs a word byte last.
    /// A `-` or `.` is followed by a blank or `=`, and neither is a word
    /// character.
    /// So only whole runs are tried.
    /// The key starts at the first position of the run that has a `\b`.
    pub(super) fn key(&mut self, from: usize) -> Option<Hit> {
        let len = self.s.len();
        let mut i = from;
        loop {
            let gap = i;
            while i < len && !is_key_byte(self.s[i]) {
                i += 1;
            }
            self.steps.spend(i - gap);
            if i >= len {
                return None;
            }
            let run = i;
            while i < len && is_key_byte(self.s[i]) {
                i += 1;
            }
            self.steps.spend(i - run);
            let last = self.s[i - 1];
            if last.is_ascii_alphanumeric() || last == b'_' {
                let equals = self.spaces(i);
                if self.s.get(equals) == Some(&b'=')
                    && let Some(start) = (run..i).find(|&p| self.boundary(p))
                {
                    return Some(Hit::Key {
                        start,
                        end: i,
                        equals,
                    });
                }
            }
        }
    }

    /// `^(\[)(.*?)(])`: the first `]` of a line that starts with `[`.
    pub(super) fn section(&mut self) -> Option<Hit> {
        if self.s.first() != Some(&b'[') {
            return None;
        }
        let close = 1 + memchr(b']', &self.s[1..])?;
        self.steps.spend(close);
        Some(Hit::Section { close })
    }

    /// The first `quote` at or after `from`.
    pub(super) fn quote(&mut self, from: usize, quote: Quote) -> Option<Hit> {
        let rest = self.s.get(from..)?;
        let at = from + memchr(quote.byte(), rest)?;
        self.steps.spend(at - from);
        Some(Hit::Quote { at, quote })
    }
}
