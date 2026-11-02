//! The text a rule is matched against, and the character classes of the
//! grammar's regular expressions.

use std::cell::Cell;

/// A window on one line, as the `TextMate` engine presents it to a regular
/// expression.
///
/// The engine appends a newline to every line, and shows a capture's own
/// rules only the line up to the end of the capture. `end` is that end: in a
/// line it is one past the last byte, which stands for the newline.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Cx<'a> {
    /// The line, without its line ending.
    pub(crate) line: &'a [u8],
    /// The offset where the text ends, at most `line.len() + 1`.
    pub(crate) end: usize,
    /// The position `\G` matches: where the last rule matched ended.
    pub(crate) anchor: Option<usize>,
    /// The steps the line has spent, which the lexer's loop checks against
    /// the line's work limit.
    pub(crate) steps: &'a Cell<u64>,
    /// The steps the line may spend, `u64::MAX` for no limit.
    pub(crate) budget: u64,
}

/// `\s` for a character: ASCII whitespace and Unicode's `White_Space`, which
/// Oniguruma's `\s` takes in a UTF-8 pattern.
fn is_space_char(c: char) -> bool {
    c.is_whitespace()
}

/// `\w` for a character: letters and digits (as [`char::is_alphanumeric`]
/// counts them), the combining marks of the common blocks, and `_` with the
/// other connector punctuation.
fn is_word_char(c: char) -> bool {
    c.is_alphanumeric()
        || matches!(
            c,
            '_' | '\u{203f}'
                | '\u{2040}'
                | '\u{2054}'
                | '\u{fe33}'
                | '\u{fe34}'
                | '\u{fe4d}'..='\u{fe4f}'
                | '\u{ff3f}'
                | '\u{0300}'..='\u{036f}'
                | '\u{1ab0}'..='\u{1aff}'
                | '\u{1dc0}'..='\u{1dff}'
                | '\u{20d0}'..='\u{20ff}'
                | '\u{fe20}'..='\u{fe2f}'
        )
}

impl Cx<'_> {
    /// The byte at `p`: the line's, the newline just past it, or nothing at
    /// or after `end`.
    #[inline]
    pub(crate) fn get(&self, p: usize) -> Option<u8> {
        if p >= self.end {
            None
        } else {
            Some(self.line.get(p).copied().unwrap_or(b'\n'))
        }
    }

    /// The byte at `p`, or 0 where there is none.
    pub(crate) fn at(&self, p: usize) -> u8 {
        self.get(p).unwrap_or(0)
    }

    /// The byte before `p`.
    #[inline]
    pub(crate) fn before(&self, p: usize) -> Option<u8> {
        p.checked_sub(1).and_then(|q| self.get(q))
    }

    /// Whether the text at `p` starts with `prefix`.
    pub(crate) fn starts(&self, p: usize, prefix: &[u8]) -> bool {
        prefix
            .iter()
            .enumerate()
            .all(|(i, &b)| self.get(p + i) == Some(b))
    }

    /// `^` or `\G`: the start of the line, or where the last rule ended.
    #[inline]
    pub(crate) fn at_start(&self, p: usize) -> bool {
        p == 0 || self.anchor == Some(p)
    }

    /// `$`: the end of the text, or the newline before it.
    #[inline]
    pub(crate) fn at_eol(&self, p: usize) -> bool {
        matches!(self.get(p), None | Some(b'\n'))
    }

    /// The offset after the run of `class` bytes that starts at `p`.
    pub(crate) fn skip(&self, mut p: usize, class: impl Fn(u8) -> bool) -> usize {
        while let Some(b) = self.get(p) {
            if !class(b) {
                break;
            }
            p += 1;
        }
        p
    }

    /// `\s*$` at `p`.
    #[inline]
    pub(crate) fn only_space_after(&self, p: usize) -> bool {
        self.get(self.skip_space(p)).is_none()
    }

    /// The character at `p`: the line's, or the newline just past it.
    #[inline]
    fn char_at(&self, p: usize) -> Option<char> {
        let first = self.get(p)?;
        if first < 0x80 {
            return Some(char::from(first));
        }
        let len = match first {
            0xc0..=0xdf => 2,
            0xe0..=0xef => 3,
            _ => 4,
        };
        let bytes = self.line.get(p..p + len)?;
        std::str::from_utf8(bytes).ok()?.chars().next()
    }

    /// The character that ends at `p`.
    #[inline]
    fn char_before(&self, p: usize) -> Option<char> {
        let last = self.before(p)?;
        if last < 0x80 {
            return Some(char::from(last));
        }
        let start = (p.saturating_sub(4)..p)
            .rev()
            .find(|&q| self.line.get(q).is_some_and(|&b| b & 0xc0 != 0x80))?;
        std::str::from_utf8(self.line.get(start..p)?)
            .ok()?
            .chars()
            .next()
    }

    /// The byte length of the whitespace character at `p`, or 0 when there is
    /// none.
    #[inline]
    pub(crate) fn space_at(&self, p: usize) -> usize {
        if let Some(b) = self.get(p).filter(u8::is_ascii) {
            return usize::from(matches!(b, b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r'));
        }
        self.char_at(p)
            .filter(|&c| is_space_char(c))
            .map_or(0, char::len_utf8)
    }

    /// The byte length of the whitespace character that ends at `p`, or 0.
    #[inline]
    pub(crate) fn space_before(&self, p: usize) -> usize {
        self.char_before(p)
            .filter(|&c| is_space_char(c))
            .map_or(0, char::len_utf8)
    }

    /// `\S` at `p`: a character that is not whitespace.
    #[inline]
    pub(crate) fn non_space_at(&self, p: usize) -> bool {
        self.char_at(p).is_some_and(|c| !is_space_char(c))
    }

    /// `(?<=\S)` at `p`.
    #[inline]
    pub(crate) fn non_space_before(&self, p: usize) -> bool {
        self.char_before(p).is_some_and(|c| !is_space_char(c))
    }

    /// The offset after the whitespace that starts at `p`.
    #[inline]
    pub(crate) fn skip_space(&self, mut p: usize) -> usize {
        loop {
            let len = self.space_at(p);
            if len == 0 {
                return p;
            }
            p += len;
        }
    }

    /// `\w` at `p`.
    #[inline]
    pub(crate) fn word_at(&self, p: usize) -> bool {
        self.char_at(p).is_some_and(is_word_char)
    }

    /// `(?<=\w)` at `p`.
    #[inline]
    pub(crate) fn word_before(&self, p: usize) -> bool {
        self.char_before(p).is_some_and(is_word_char)
    }

    /// The byte length of the character at `p`, which is 1 for an invalid one.
    pub(crate) fn char_len(&self, p: usize) -> usize {
        self.char_at(p).map_or(1, char::len_utf8)
    }

    /// Whether the line has spent its work, so that a long scan gives up.
    pub(crate) fn over(&self) -> bool {
        self.steps.get() > self.budget
    }

    /// Counts `bytes` read ahead of the cursor against the line's work.
    pub(crate) fn spend(&self, bytes: usize) {
        let bytes = u64::try_from(bytes).unwrap_or(u64::MAX);
        self.steps.set(self.steps.get().saturating_add(bytes));
    }
}
