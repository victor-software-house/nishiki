//! Strings, character constants, escapes and printf placeholders.

use super::engine::Lexer;
use super::scope::S;

impl Lexer<'_> {
    /// The quote `q`, with `punctuation.definition.string.begin` on it.
    pub(super) fn quote(&mut self, p: usize, q: u8) -> bool {
        if p >= self.len_all() || self.b(p) != q {
            return false;
        }
        self.span(p, p + 1, &[S::PunctStringBegin]);
        self.begin(p, p + 1)
    }

    fn hex_at(&self, i: usize) -> bool {
        i < self.len_all() && self.b(i).is_ascii_hexdigit()
    }

    /// `\h{0,max}` from `i`.
    fn hex_run(&self, mut i: usize, max: usize) -> usize {
        let mut k = 0;
        while k < max && self.hex_at(i) {
            i += 1;
            k += 1;
        }
        i
    }

    /// `\d{0,max}` from `i`.
    fn digit_run(&self, mut i: usize, max: usize) -> usize {
        for _ in 0..max {
            let width = self.digit_at(i);
            if width == 0 {
                break;
            }
            i += width;
        }
        i
    }

    /// The end of an escape sequence after the backslash at `p`, as
    /// `string_escaped_char` reads it; `octal` reads the digits of the
    /// three-digit form as octal digits, as `backslash_escapes` does.
    fn escape_end(&self, p: usize, octal: bool) -> Option<usize> {
        if self.b(p) != b'\\' || p >= self.len_all() {
            return None;
        }
        let after = p + 1;
        let lead = self.b(after);
        if after >= self.len_all() {
            return None;
        }
        if matches!(
            lead,
            b'"' | b'\''
                | b'?'
                | b'\\'
                | b'a'
                | b'b'
                | b'e'
                | b'f'
                | b'n'
                | b'p'
                | b'r'
                | b't'
                | b'v'
        ) {
            return Some(after + 1);
        }
        if (b'0'..=b'3').contains(&lead) {
            return Some(if octal {
                let mut at = after + 1;
                let mut count = 0;
                while count < 2 && (b'0'..=b'7').contains(&self.b(at)) && at < self.len_all() {
                    at += 1;
                    count += 1;
                }
                at
            } else {
                self.digit_run(after + 1, 2)
            });
        }
        if (b'4'..=b'7').contains(&lead) {
            return Some(self.digit_run(after + 1, 1));
        }
        match lead {
            b'x' => Some(self.hex_run(after + 1, 2)),
            b'u' => Some(self.hex_run(after + 1, 4)),
            b'U' => Some(self.hex_run(after + 1, 8)),
            _ => None,
        }
    }

    /// A valid escape sequence.
    pub(super) fn esc_char(&mut self, p: usize) -> bool {
        match self.escape_end(p, false) {
            Some(end) => self.plain(p, end),
            None => false,
        }
    }

    /// A valid escape sequence of an assembly string.
    pub(super) fn esc_backslash(&mut self, p: usize) -> bool {
        match self.escape_end(p, true) {
            Some(end) => self.plain(p, end),
            None => false,
        }
    }

    /// `\\.`: a backslash and any character but the line feed.
    pub(super) fn esc_invalid(&mut self, p: usize) -> bool {
        if self.b(p) != b'\\' || p >= self.len_all() {
            return false;
        }
        match self.char_at(p + 1) {
            Some((c, end)) if c != '\n' => self.plain(p, end),
            _ => false,
        }
    }

    /// `%(\d+\$)?[-
    /// #'+0]*[,:;_]?((-?\d+)|\*(-?\d+\$)?)?(\.((-?\d+)|\*(-?\d+\$)?)?)?(hh?
    /// |ll|[Ljlqtz]|vh|vl?|hv|hl)?[%AC-GOSUXac-ginopsux]`.
    pub(super) fn placeholder(&mut self, p: usize) -> bool {
        if self.b(p) != b'%' || p >= self.len_all() {
            return false;
        }
        match self.ph_arg(p + 1) {
            Some(end) => self.plain(p, end),
            None => false,
        }
    }

    /// `\d+` from `i`: every end it can have, the longest first.
    fn ph_digit_ends(
        &self,
        i: usize,
        mut then: impl FnMut(usize) -> Option<usize>,
    ) -> Option<usize> {
        let mut max = i;
        loop {
            let width = self.digit_at(max);
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
            let width = self.char_ending(end)?.len_utf8();
            end -= width;
        }
        None
    }

    /// `(\d+\$)?` and then the rest.
    fn ph_arg(&self, i: usize) -> Option<usize> {
        let with = self.ph_digit_ends(i, |end| {
            (self.b(end) == b'$' && end < self.len_all()).then(|| self.ph_flags(end + 1))?
        });
        with.or_else(|| self.ph_flags(i))
    }

    /// `[- #'+0]*` with the ends it can have, longest first.
    fn ph_flags(&self, i: usize) -> Option<usize> {
        let mut max = i;
        while max < self.len_all()
            && matches!(self.b(max), b'-' | b' ' | b'#' | b'\'' | b'+' | b'0')
        {
            max += 1;
        }
        self.spend(max - i);
        (i..=max).rev().find_map(|end| self.ph_flag(end))
    }

    /// `[,:;_]?`.
    fn ph_flag(&self, i: usize) -> Option<usize> {
        if i < self.len_all()
            && matches!(self.b(i), b',' | b':' | b';' | b'_')
            && let Some(end) = self.ph_width(i + 1, false)
        {
            return Some(end);
        }
        self.ph_width(i, false)
    }

    /// `((-?\d+)|\*(-?\d+\$)?)?` for the width, and the same after the dot
    /// for the precision, and then the rest.
    fn ph_width(&self, i: usize, precision: bool) -> Option<usize> {
        let rest = |end: usize| {
            if precision {
                self.ph_length(end)
            } else {
                self.ph_precision(end)
            }
        };
        let signed = if self.b(i) == b'-' && i < self.len_all() {
            i + 1
        } else {
            i
        };
        if let Some(end) = self.ph_digit_ends(signed, &rest) {
            return Some(end);
        }
        if self.b(i) == b'*' && i < self.len_all() {
            let star = i + 1;
            let dollar = self.ph_digit_ends(
                if self.b(star) == b'-' && star < self.len_all() {
                    star + 1
                } else {
                    star
                },
                |end| (self.b(end) == b'$' && end < self.len_all()).then(|| rest(end + 1))?,
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
        if self.b(i) == b'.'
            && i < self.len_all()
            && let Some(end) = self.ph_width(i + 1, true)
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
        let ok = i < self.len_all()
            && matches!(
                self.b(i),
                b'%' | b'A' | b'C'..=b'G' | b'O' | b'S' | b'U' | b'X' | b'a' | b'c'..=b'g' | b'i' | b'n' | b'o' | b'p' | b's' | b'u' | b'x'
            );
        ok.then_some(i + 1)
    }

    /// `(%)(?!"\s*(PRI|SCN))`: a stray percent sign.
    pub(super) fn placeholder_bad(&mut self, p: usize) -> bool {
        if self.b(p) != b'%' || p >= self.len_all() {
            return false;
        }
        if self.b(p + 1) == b'"' {
            let q = self.skip_space(p + 2);
            if self.starts(q, b"PRI") || self.starts(q, b"SCN") {
                return false;
            }
        }
        self.span(p, p + 1, &[S::InvalidPlaceholder]);
        self.plain(p, p + 1)
    }
}
