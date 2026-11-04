//! The printf placeholder of a string, with the backtracking of its
//! expression: `%`, an argument number, flags, a width, a precision, a length
//! and a conversion.

use super::super::engine::Lexer;
use super::chars::is_digit;

/// The number fields of a placeholder.
/// Both take the same digits.
#[derive(Clone, Copy)]
enum Field {
    /// The width.
    /// A precision may follow it.
    Width,
    /// The precision, after the dot.
    Precision,
}

impl Lexer<'_> {
    /// The width of the `\d` character at `i`, or 0.
    fn digit_width(&self, i: usize) -> usize {
        match self.ch(i) {
            Some((c, end)) if is_digit(c) => end - i,
            _ => 0,
        }
    }

    /// `%(\d+\$)?[-
    /// #'+0]*[,:;_]?((-?\d+)|\*(-?\d+\$)?)?(\.((-?\d+)|\*(-?\d+\$)?)?)?(hh?
    /// |ll|[Ljlqtz]|vh|vl?|hv|hl)?[%AC-GOSUXac-ginopsux]`.
    pub(super) fn placeholder(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'%' || p >= self.lim() {
            return None;
        }
        if self.at(p + 1) == b'\'' || self.at(p + 1) == b'"' {
            return None;
        }
        self.ph_arg(p + 1)
    }

    /// `\d+` from `i`: every end it can have, the longest first.
    fn ph_digit_ends(
        &self,
        i: usize,
        mut then: impl FnMut(usize) -> Option<usize>,
    ) -> Option<usize> {
        let mut max = i;
        loop {
            let width = self.digit_width(max);
            if width == 0 {
                break;
            }
            max += width;
        }
        self.spend(max - i);
        let mut end = max;
        while end > i {
            if let Some(found) = then(end) {
                return Some(found);
            }
            let width = self.ch_before(end)?.len_utf8();
            end -= width;
        }
        None
    }

    /// `(\d+\$)?` and then the rest.
    fn ph_arg(&self, i: usize) -> Option<usize> {
        let with = self.ph_digit_ends(i, |end| {
            (self.at(end) == b'$' && end < (self.lim() + usize::from(self.nl)))
                .then(|| self.ph_flags(end + 1))?
        });
        with.or_else(|| self.ph_flags(i))
    }

    /// `[- #'+0]*` with the ends it can have, longest first.
    fn ph_flags(&self, i: usize) -> Option<usize> {
        let mut max = i;
        while max < (self.lim() + usize::from(self.nl))
            && matches!(self.at(max), b'-' | b' ' | b'#' | b'\'' | b'+' | b'0')
        {
            max += 1;
        }
        self.spend(max - i);
        (i..=max).rev().find_map(|end| self.ph_flag(end))
    }

    /// `[,:;_]?`.
    fn ph_flag(&self, i: usize) -> Option<usize> {
        if i < (self.lim() + usize::from(self.nl))
            && matches!(self.at(i), b',' | b':' | b';' | b'_')
            && let Some(end) = self.ph_width(i + 1, Field::Width)
        {
            return Some(end);
        }
        self.ph_width(i, Field::Width)
    }

    /// `((-?\d+)|\*(-?\d+\$)?)?` for the width, and the same after the dot
    /// for the precision, and then the rest.
    fn ph_width(&self, i: usize, field: Field) -> Option<usize> {
        let rest = |end: usize| match field {
            Field::Width => self.ph_precision(end),
            Field::Precision => self.ph_length(end),
        };
        let signed = if self.at(i) == b'-' && i < (self.lim() + usize::from(self.nl)) {
            i + 1
        } else {
            i
        };
        if let Some(end) = self.ph_digit_ends(signed, &rest) {
            return Some(end);
        }
        if self.at(i) == b'*' && i < (self.lim() + usize::from(self.nl)) {
            let star = i + 1;
            let dollar = self.ph_digit_ends(
                if self.at(star) == b'-' && star < (self.lim() + usize::from(self.nl)) {
                    star + 1
                } else {
                    star
                },
                |end| {
                    (self.at(end) == b'$' && end < (self.lim() + usize::from(self.nl)))
                        .then(|| rest(end + 1))?
                },
            );
            if dollar.is_some() {
                return dollar;
            }
            if let Some(end) = rest(star) {
                return Some(end);
            }
        }
        rest(i)
    }

    /// `(\.((-?\d+)|\*(-?\d+\$)?)?)?`.
    fn ph_precision(&self, i: usize) -> Option<usize> {
        if self.at(i) == b'.'
            && i < (self.lim() + usize::from(self.nl))
            && let Some(end) = self.ph_width(i + 1, Field::Precision)
        {
            return Some(end);
        }
        self.ph_length(i)
    }

    /// `(hh?|ll|[Ljlqtz]|vh|vl?|hv|hl)?` and the conversion.
    fn ph_length(&self, i: usize) -> Option<usize> {
        const LENGTHS: &[&[u8]] = &[
            b"hh", b"h", b"ll", b"L", b"j", b"l", b"q", b"t", b"z", b"vh", b"vl", b"v", b"hv",
            b"hl",
        ];
        for length in LENGTHS {
            if self.starts(i, length)
                && let Some(end) = self.ph_conversion(i + length.len())
            {
                return Some(end);
            }
        }
        self.ph_conversion(i)
    }

    /// `[%AC-GOSUXac-ginopsux]`.
    fn ph_conversion(&self, i: usize) -> Option<usize> {
        let ok = i < (self.lim() + usize::from(self.nl))
            && matches!(
                self.at(i),
                b'%' | b'A' | b'C'..=b'G' | b'O' | b'S' | b'U' | b'X' | b'a' | b'c'..=b'g' | b'i' | b'n' | b'o' | b'p' | b's' | b'u' | b'x'
            );
        ok.then_some(i + 1)
    }
}
