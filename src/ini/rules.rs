//! The grammar's rules as hand-written searches.
//!
//! Each rule is a regular expression, tried at every position of a line, and
//! the line's next token is the match that starts first, the rule that comes
//! first in the grammar winning a tie. A search below answers where its rule
//! first matches at or after a position, with the regular expression's
//! backtracking taken into account, so the lexer can keep each rule's next
//! match until the cursor passes it.

use memchr::memchr;

use crate::json::classes;

/// A rule's first match at or after a position.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Hit {
    /// A comment that starts with `#` or `;`: where the match starts, which
    /// is byte 0 when blanks lead the line and the comment mark's position
    /// otherwise, and where the mark is.
    Comment { start: usize, mark: usize },
    /// A key: its bounds, and the `=` that follows it after any blanks.
    Key {
        start: usize,
        end: usize,
        equals: usize,
    },
    /// A section header `[...]`: the `]` that closes it.
    Section { close: usize },
    /// A quote that opens a string.
    Quote { at: usize },
}

impl Hit {
    /// Where the match starts.
    pub(super) fn start(self) -> usize {
        match self {
            Hit::Comment { start, .. } | Hit::Key { start, .. } => start,
            Hit::Section { .. } => 0,
            Hit::Quote { at } => at,
        }
    }
}

/// How many bytes the character that starts with `lead` takes.
pub(super) fn char_len(lead: u8) -> usize {
    match lead {
        0xf0.. => 4,
        0xe0.. => 3,
        0xc0.. => 2,
        _ => 1,
    }
}

/// The regex `\s` for an ASCII byte.
fn is_ascii_space(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r')
}

/// A byte of the key class `[-.0-9A-Z_a-z]`.
fn is_key_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || matches!(b, b'-' | b'.' | b'_')
}

/// The regex `\w` for any character.
fn is_word(c: char) -> bool {
    if c.is_ascii() {
        c.is_ascii_alphanumeric() || c == '_'
    } else {
        classes::is_word(c)
    }
}

/// A line and the bytes its searches have read.
pub(super) struct Line<'a> {
    s: &'a [u8],
    /// Bytes read so far, for the work limit.
    pub(super) spent: u64,
}

impl<'a> Line<'a> {
    pub(super) fn new(s: &'a [u8]) -> Self {
        Line { s, spent: 0 }
    }

    fn spend(&mut self, bytes: usize) {
        let bytes = u64::try_from(bytes).unwrap_or(u64::MAX);
        self.spent = self.spent.saturating_add(bytes);
    }

    /// The character at `i` and its width; the line feed after the line is
    /// one character.
    fn char_at(&self, i: usize) -> Option<(char, usize)> {
        if i == self.s.len() {
            return Some(('\n', 1));
        }
        let width = char_len(*self.s.get(i)?);
        let text = std::str::from_utf8(self.s.get(i..i + width)?).ok()?;
        Some((text.chars().next()?, width))
    }

    /// The width of the white space character at `i`, 0 when there is none.
    fn space_at(&self, i: usize) -> usize {
        match self.s.get(i) {
            Some(&b) if b < 0x80 => usize::from(is_ascii_space(b)),
            Some(_) => match self.char_at(i) {
                Some((c, width)) if classes::is_space(c) => width,
                _ => 0,
            },
            None => usize::from(i == self.s.len()),
        }
    }

    /// The end of the run of white space from `i`.
    fn spaces(&mut self, mut i: usize) -> usize {
        let from = i;
        while let width @ 1.. = self.space_at(i) {
            i += width;
        }
        self.spend(i - from);
        i
    }

    /// Whether the character at `i` is one `\w` matches.
    fn word_at(&self, i: usize) -> bool {
        self.char_at(i).is_some_and(|(c, _)| is_word(c))
    }

    /// Whether the character that ends at `i` is one `\w` matches.
    fn word_before(&self, i: usize) -> bool {
        let Some(mut start) = i.checked_sub(1) else {
            return false;
        };
        while start > 0 && self.s.get(start).is_some_and(|&b| b & 0xc0 == 0x80) {
            start -= 1;
        }
        self.char_at(start).is_some_and(|(c, _)| is_word(c))
    }

    /// The regex `\b` at `i`.
    fn boundary(&self, i: usize) -> bool {
        self.word_before(i) != self.word_at(i)
    }

    /// `(^[\t ]+)?(?=#)` and its twin for `;`, whose first match from `from`
    /// is at the first `mark` byte, or at the line's start when only tabs
    /// and spaces lie before it and the search starts there.
    pub(super) fn comment(&mut self, from: usize, mark: u8) -> Option<Hit> {
        let rest = self.s.get(from..)?;
        let at = from + memchr(mark, rest)?;
        self.spend(at - from);
        let leading =
            from == 0 && at > 0 && self.s[..at].iter().all(|&b| matches!(b, b'\t' | b' '));
        Some(Hit::Comment {
            start: if leading { 0 } else { at },
            mark: at,
        })
    }

    /// `\b([-.0-9A-Z_a-z]+)\b\s*(=)`
    ///
    /// The key is a run of class bytes. Backtracking cannot shorten it,
    /// because the byte after a shorter key is a class byte, which is
    /// neither a blank nor `=`; and `\b` before the blanks needs a word byte
    /// last, since a `-` or `.` is followed by a blank or `=`, which are not
    /// word characters either. So only whole runs are tried, and the key
    /// starts at the first position of the run that has a `\b`.
    pub(super) fn key(&mut self, from: usize) -> Option<Hit> {
        let len = self.s.len();
        let mut i = from;
        loop {
            while i < len && !is_key_byte(self.s[i]) {
                i += 1;
            }
            if i >= len {
                self.spend(i.saturating_sub(from));
                return None;
            }
            let run = i;
            while i < len && is_key_byte(self.s[i]) {
                i += 1;
            }
            self.spend(i - run);
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
        self.spend(close);
        Some(Hit::Section { close })
    }

    /// The first `quote` at or after `from`.
    pub(super) fn quote(&mut self, from: usize, quote: u8) -> Option<Hit> {
        let rest = self.s.get(from..)?;
        let at = from + memchr(quote, rest)?;
        self.spend(at - from);
        Some(Hit::Quote { at })
    }
}
