//! The identifier and operator classes of the grammar that several files of
//! matchers share.

use std::cell::Cell;

use super::super::engine::Lexer;
use crate::classes::{is_letter, is_mark, is_number};

/// Whether a run of operator characters takes `.` as well as head and
/// continuation characters.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::swift) enum Dots {
    /// The run takes dots.
    Taken,
    /// A dot ends the run.
    Refused,
}

/// What the matchers remember about identifiers on a line: the last run of
/// identifier characters, as `(from, to)`.
/// A scan that starts inside a run ends where the run ends.
/// A line of thousands of identifier characters is then walked once, not
/// from every start the engine tries.
#[derive(Debug, Default)]
pub(in crate::swift) struct Memo {
    tail: Cell<(usize, usize)>,
}

/// The grammar's identifier at a position: its optional backticks, a letter or
/// `_`, then letters, numbers and marks.
///
/// The text from the start up to `body` is the opening backtick, when there
/// is one.
/// The closing backtick is `tail..end`.
#[derive(Clone, Copy, Debug)]
pub(in crate::swift) struct Ident {
    /// Where the name itself starts.
    pub(in crate::swift) body: usize,
    /// Where the name itself ends.
    pub(in crate::swift) tail: usize,
    /// The end, after the closing backtick when there is one.
    pub(in crate::swift) end: usize,
}

impl Ident {
    /// The width of the backtick that opened the identifier and of the one
    /// that closed it: 0 or 1.
    pub(in crate::swift) fn quote(self) -> usize {
        self.end - self.tail
    }
}

/// `[_\p{L}\p{N}\p{M}]`, which follows the first character of an identifier.
fn is_ident_tail(c: char) -> bool {
    c == '_' || is_letter(c) || is_number(c) || is_mark(c)
}

/// The grammar's operator head class `oph`.
pub(in crate::swift) fn is_op_head(c: char) -> bool {
    matches!(c,
        '-' | '!' | '%' | '&' | '*' | '+' | '/' | '<'..='?' | '^' | '|' | '~'
        | '\u{a1}'..='\u{a7}' | '\u{a9}' | '\u{ab}' | '\u{ac}' | '\u{ae}'
        | '\u{b0}' | '\u{b1}' | '\u{b6}' | '\u{bb}' | '\u{bf}' | '\u{d7}' | '\u{f7}'
        | '\u{2016}' | '\u{2017}' | '\u{2020}'..='\u{2027}' | '\u{2030}'..='\u{203e}'
        | '\u{2041}'..='\u{2053}' | '\u{2055}'..='\u{205e}' | '\u{2190}'..='\u{23ff}'
        | '\u{2500}'..='\u{2775}' | '\u{2794}'..='\u{2bff}' | '\u{2e00}'..='\u{2e7f}'
        | '\u{3001}'..='\u{3003}' | '\u{3008}'..='\u{3030}')
}

/// The grammar's operator continuation class `opc`.
pub(in crate::swift) fn is_op_continue(c: char) -> bool {
    matches!(c,
        '\u{300}'..='\u{36f}' | '\u{1dc0}'..='\u{1dff}' | '\u{20d0}'..='\u{20ff}'
        | '\u{fe00}'..='\u{fe0f}' | '\u{fe20}'..='\u{fe2f}' | '\u{e0100}'..='\u{e01ef}')
}

/// `[-!%\&*+./<=>^|~]`, the ASCII operator characters that the arrow and
/// ampersand patterns look around.
pub(in crate::swift) fn is_op_ascii(b: u8) -> bool {
    matches!(
        b,
        b'-' | b'!'
            | b'%'
            | b'&'
            | b'*'
            | b'+'
            | b'.'
            | b'/'
            | b'<'
            | b'='
            | b'>'
            | b'^'
            | b'|'
            | b'~'
    )
}

impl Lexer<'_> {
    /// The end of the run of identifier characters from `from`.
    /// A start inside the run last scanned reuses its end.
    pub(in crate::swift) fn ident_tail_end(&self, from: usize) -> usize {
        let (lo, hi) = self.x.names.tail.get();
        if lo <= from && from < hi {
            return hi;
        }
        let mut i = from;
        while let Some((c, end)) = self.char_at(i)
            && is_ident_tail(c)
        {
            i = end;
        }
        self.spend(i - from);
        self.x.names.tail.set((from, i));
        i
    }

    /// The identifier at `i`: an optional backtick, a head character, the
    /// tail, and the same backtick again.
    /// When the backtick is there but does not close, giving it back leaves a
    /// backtick where the head belongs.
    /// There is no other way to match.
    pub(in crate::swift) fn ident_at(&self, i: usize) -> Option<Ident> {
        let quote = usize::from(self.at(i) == b'`');
        let body = i + quote;
        let (c, head_end) = self.char_at(body)?;
        if c != '_' && !is_letter(c) {
            return None;
        }
        let tail = self.ident_tail_end(head_end);
        if quote == 1 && self.at(tail) != b'`' {
            return None;
        }
        Some(Ident {
            body,
            tail,
            end: tail + quote,
        })
    }

    /// Records the two backtick groups `q` and `q + 1` of the identifier that
    /// starts at `start`.
    pub(in crate::swift) fn capture_quotes(&mut self, q: usize, start: usize, id: Ident) {
        self.capture(q, start, start + id.quote());
        self.capture(q + 1, id.end - id.quote(), id.end);
    }
}
