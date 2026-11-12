//! The text a rule is matched against, and the character classes of the
//! grammar's regular expressions.

use crate::classes::{self, is_space, is_word};
use crate::limits::Steps;
use crate::utf8;

/// A window on one line, as the `TextMate` engine presents it to a regular
/// expression.
///
/// The engine appends a newline to every line.
/// It shows a capture's own rules only the line up to the end of the capture.
/// `end` is that end.
/// In a line it is one past the last byte, which stands for the newline.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Cx<'a> {
    /// The line, without its line ending.
    pub(crate) line: &'a [u8],
    /// The offset where the text ends, at most `line.len() + 1`.
    pub(crate) end: usize,
    /// The position `\G` matches: where the last rule matched ended.
    pub(crate) anchor: Option<usize>,
    /// The line's work and the character before its start, behind one
    /// reference, so a window costs no more to make than one without it.
    pub(crate) facts: &'a LineFacts,
}

/// What the rules read about a line besides its bytes.
#[derive(Debug)]
pub(crate) struct LineFacts {
    /// The steps the line has spent and may spend.
    /// The lexer's loop checks them each turn.
    pub(crate) steps: Steps,
    /// The character before the line's start, for Markdown that another
    /// language embeds from the middle of its line.
    /// vscode-textmate matches on the whole line, so a lookbehind at the
    /// start reads it.
    /// None for a Markdown document.
    pub(crate) before_start: Option<char>,
}

impl Cx<'_> {
    /// The byte at `p`: the line's, the newline after it, or nothing at
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
    /// Before the start it is the last byte of [`LineFacts::before_start`].
    #[inline]
    pub(crate) fn before(&self, p: usize) -> Option<u8> {
        match p.checked_sub(1) {
            Some(q) => self.get(q),
            None => self.facts.before_start.map(last_byte),
        }
    }

    /// Whether the text at `p` starts with `prefix`.
    pub(crate) fn starts(&self, p: usize, prefix: &[u8]) -> bool {
        prefix
            .iter()
            .enumerate()
            .all(|(i, &b)| self.get(p + i) == Some(b))
    }

    /// A bare `^`: the start of the whole line.
    /// Markdown that another language embeds from the middle of its line
    /// starts after the line's start, so `^` fails at its start.
    /// vscode-textmate matches on the whole line.
    #[inline]
    pub(crate) fn at_line_start(&self, p: usize) -> bool {
        p == 0 && self.facts.before_start.is_none()
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

    /// The character at `p`: the line's, or the newline after it.
    #[inline]
    fn char_at(&self, p: usize) -> Option<char> {
        let first = self.get(p)?;
        if first < 0x80 {
            return Some(char::from(first));
        }
        utf8::char_at(self.line, p).map(|(c, _)| c)
    }

    /// The character that ends at `p`.
    #[inline]
    fn char_before(&self, p: usize) -> Option<char> {
        let last = self.before(p)?;
        if last < 0x80 {
            return Some(char::from(last));
        }
        self.wide_char_before(p)
    }

    /// The character of more than one byte that ends at `p`.
    /// Out of line, so [`Cx::char_before`] stays small enough to inline.
    #[cold]
    fn wide_char_before(&self, p: usize) -> Option<char> {
        if p == 0 {
            return self.facts.before_start;
        }
        let start = (p.saturating_sub(4)..p)
            .rev()
            .find(|&q| self.line.get(q).is_some_and(|&b| b & 0xc0 != 0x80))?;
        utf8::char_at(self.line, start)
            .filter(|&(_, width)| start + width == p)
            .map(|(c, _)| c)
    }

    /// The byte length of the whitespace character at `p`, or 0 when there is
    /// none.
    #[inline]
    pub(crate) fn space_at(&self, p: usize) -> usize {
        if let Some(b) = self.get(p).filter(u8::is_ascii) {
            return usize::from(classes::is_ascii_space(b));
        }
        self.char_at(p)
            .filter(|&c| is_space(c))
            .map_or(0, char::len_utf8)
    }

    /// The byte length of the whitespace character that ends at `p`, or 0.
    #[inline]
    pub(crate) fn space_before(&self, p: usize) -> usize {
        self.char_before(p)
            .filter(|&c| is_space(c))
            .map_or(0, char::len_utf8)
    }

    /// `\S` at `p`: a character that is not whitespace.
    #[inline]
    pub(crate) fn non_space_at(&self, p: usize) -> bool {
        self.char_at(p).is_some_and(|c| !is_space(c))
    }

    /// `(?<=\S)` at `p`.
    #[inline]
    pub(crate) fn non_space_before(&self, p: usize) -> bool {
        self.char_before(p).is_some_and(|c| !is_space(c))
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
        self.char_at(p).is_some_and(is_word)
    }

    /// `(?<=\w)` at `p`.
    #[inline]
    pub(crate) fn word_before(&self, p: usize) -> bool {
        self.char_before(p).is_some_and(is_word)
    }

    /// The byte length of the character at `p`.
    /// It is 1 for an invalid character.
    pub(crate) fn char_len(&self, p: usize) -> usize {
        self.char_at(p).map_or(1, char::len_utf8)
    }

    /// Whether the line has spent its work.
    /// A long scan then gives up.
    pub(crate) fn over(&self) -> bool {
        self.facts.steps.exhausted()
    }

    /// Counts `bytes` read ahead of the cursor against the line's work.
    pub(crate) fn spend(&self, bytes: usize) {
        self.facts.steps.spend(bytes);
    }
}

/// The last byte of `c` in UTF-8, as a test of one byte reads it: the
/// character when it is ASCII, else a continuation byte.
fn last_byte(c: char) -> u8 {
    u8::try_from(c).ok().filter(u8::is_ascii).unwrap_or(0x80)
}
