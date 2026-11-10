//! Heredocs: the opener, `((((<<(~)?) *')(LABEL)(')))(.*)\n?` and its other
//! quotes, and the line that closes it, `^((?!\5)\s+)?((\6))$`.
//!
//! The generator splits each opener on the `~` of `<<~`, so a pattern takes
//! the `~` or leaves it out ([`Tilde`]).
//! The groups of an opener:
//! - 1 and 2: `<<`, the `~`, the blanks and the quoted label;
//! - 3: `<<`, the `~`, the blanks and the opening quote;
//! - 6: the label, which the closing line repeats;
//! - 7: the closing quote;
//! - 8: the rest of the line, which the grammar lexes again as Perl.

use super::super::engine::Lexer;
use super::{Label, Quote, Tilde};
use crate::rules::Chars;

/// ``["'),;`]``: a byte a bare label stops at, besides `\s`.
fn ends_bare(b: u8) -> bool {
    matches!(b, b'"' | b'\'' | b')' | b',' | b';' | b'`')
}

impl Quote {
    /// The quote before the label, and the one after it.
    fn marks(self) -> (Option<u8>, Option<u8>) {
        match self {
            Quote::Single => (Some(b'\''), Some(b'\'')),
            Quote::Double => (Some(b'"'), Some(b'"')),
            Quote::Backtick => (Some(b'`'), Some(b'`')),
            Quote::Backslash => (Some(b'\\'), None),
            Quote::None => (None, None),
        }
    }
}

impl Lexer<'_> {
    /// A heredoc's opener at `p`.
    ///
    /// The blanks, ` *`, give nothing back: a quote follows them, or a bare
    /// label, which cannot start with a blank.
    /// A quoted label, `[^']*`, runs to the first closing quote.
    pub(super) fn heredoc(
        &mut self,
        p: usize,
        tilde: Tilde,
        quote: Quote,
        label: Label,
    ) -> Option<usize> {
        let mut at = self.lit(p, b"<<")?;
        if tilde == Tilde::Yes {
            at = self.lit(at, b"~")?;
        }
        at = self.skip(at, |b| b == b' ');
        let (open, close) = quote.marks();
        if let Some(open) = open {
            if self.at(at) != open || at >= self.lim() {
                return None;
            }
            at += 1;
        }
        let quoted = at;
        let label_end = match label {
            Label::Is(text) => self.lit(at, text)?,
            Label::Quoted => {
                let close = close?;
                self.skip(at, |b| b != close)
            }
            Label::Bare => self.bare_label(at)?,
        };
        let opener = match close {
            Some(close) if self.at(label_end) == close && label_end < self.lim() => label_end + 1,
            Some(_) => return None,
            None => label_end,
        };
        let lim = self.lim();
        self.capture(1, p, opener);
        self.capture(2, p, opener);
        self.capture(3, p, quoted);
        self.capture(6, quoted, label_end);
        self.capture(7, label_end, opener);
        self.capture(8, opener, lim);
        Some(if self.avail(lim) { lim + 1 } else { lim })
    }

    /// ``(?![ $(=\d])[^"'),;`\s]*``: a bare label, which may be empty.
    fn bare_label(&self, p: usize) -> Option<usize> {
        if p < self.lim()
            && (matches!(self.at(p), b' ' | b'$' | b'(' | b'=') || self.digit_end(p) > p)
        {
            return None;
        }
        let mut at = p;
        while at < self.lim() && !ends_bare(self.at(at)) && !self.is_space_at(at) {
            at = self.char_end(at);
        }
        Some(at)
    }

    /// `^((?!~)\s+)?((LABEL))$` or `^((?!)\s+)?((LABEL))$`: the line that
    /// closes a heredoc, the label alone.
    /// With the `~`, blanks may come first; `(?!)` allows none without it.
    /// A label that the pattern does not name is the rule's begin text.
    ///
    /// The blanks give characters back from the longest run, since a quoted
    /// label can start with a blank.
    pub(super) fn heredoc_end(
        &mut self,
        p: usize,
        tilde: Tilde,
        label: Option<&'static [u8]>,
    ) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let label: &[u8] = match label {
            Some(text) => text,
            None => self.begin_text().as_bytes(),
        };
        let mut blanks = match tilde {
            Tilde::Yes => self.space_end(0),
            Tilde::No => 0,
        };
        loop {
            if let Some(end) = self.lit(blanks, label)
                && self.at_eol(end)
            {
                if blanks > 0 {
                    self.capture(1, 0, blanks);
                }
                self.capture(2, blanks, end);
                self.capture(3, blanks, end);
                return Some(end);
            }
            if blanks == 0 {
                return None;
            }
            blanks = self.char_start_before(blanks);
        }
    }
}
