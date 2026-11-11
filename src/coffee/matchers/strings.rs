//! Strings, comments, heregexes and JSX tags.

use super::{Coffee, Dialect, Lexer};
use crate::rules::Chars;

/// The string an escape stands in, whose escapes differ in one branch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Quote {
    /// `"…"`, whose `3[0-6][0-7]` needs its third digit.
    Double,
    /// `'…'`, whose `3[0-6][0-7]?` does not.
    Single,
}

impl<const D: u8> Lexer<'_, D>
where
    Coffee<D>: Dialect,
{
    /// `(\\)(x\h{2}|[012][0-7]{0,2}|3[0-6][0-7]|37[0-7]?|[4-7][0-7]?|.)`, with
    /// `3[0-6][0-7]?` in a single-quoted string.
    /// A branch that fails leaves the next to try; `.` takes the character
    /// after the backslash.
    pub(super) fn string_escape(&mut self, p: usize, quote: Quote) -> Option<usize> {
        let from = self.lit(p, b"\\")?;
        let octal = |i: usize| matches!(self.at(i), b'0'..=b'7');
        let end = match self.at(from) {
            b'x' if self.at(from + 1).is_ascii_hexdigit()
                && self.at(from + 2).is_ascii_hexdigit() =>
            {
                from + 3
            }
            b'0'..=b'2' if octal(from + 1) => from + 2 + usize::from(octal(from + 2)),
            b'0'..=b'2' => from + 1,
            b'3' if matches!(self.at(from + 1), b'0'..=b'6') && octal(from + 2) => from + 3,
            b'3' if matches!(self.at(from + 1), b'0'..=b'6') && quote == Quote::Single => from + 2,
            b'3' if self.at(from + 1) == b'7' => from + 2 + usize::from(octal(from + 2)),
            b'4'..=b'7' => from + 1 + usize::from(octal(from + 1)),
            _ => self.any_char(from)?,
        };
        self.capture(1, p, from);
        Some(end)
    }

    /// `\\.`: a backslash and the character after it, which is not the line
    /// feed.
    pub(super) fn backslash_any(&self, p: usize) -> Option<usize> {
        let from = self.lit(p, b"\\")?;
        self.any_char(from)
    }

    /// `.`: the character at `i`, which is not the line feed.
    fn any_char(&self, i: usize) -> Option<usize> {
        (i < self.lim()).then(|| self.char_end(i))
    }

    /// `` (`)(.*)(`) ``: a backtick, and the line up to its last backtick.
    /// `.*` takes as much as it can, so the string ends at the last one.
    pub(super) fn script(&mut self, p: usize) -> Option<usize> {
        let open = self.lit(p, b"`")?;
        let close = open + memchr::memrchr(b'`', self.text(open, self.lim()))?;
        self.capture(1, p, open);
        self.capture(2, open, close);
        self.capture(3, close, close + 1);
        Some(close + 1)
    }

    /// `(?<!#)###(?!#)`.
    pub(super) fn block_comment(&self, p: usize) -> Option<usize> {
        if self.prev(p) == Some(b'#') {
            return None;
        }
        let end = self.lit(p, b"###")?;
        (self.at(end) != b'#').then_some(end)
    }

    /// `(?<=^|\s)@\w*(?=\s)`.
    /// Giving back the `\w*` leaves a `\w` character where `\s` must stand.
    pub(super) fn annotation(&self, p: usize) -> Option<usize> {
        if !self.after_space(p) {
            return None;
        }
        let end = self.word_end(self.lit(p, b"@")?);
        (self.space_at(end) > 0).then_some(end)
    }

    /// `(?<!\\)(#).*$\n?`: the rest of the line, with its line feed.
    pub(super) fn embedded_comment(&mut self, p: usize) -> Option<usize> {
        if self.prev(p) == Some(b'\\') {
            return None;
        }
        let end = self.lit(p, b"#")?;
        self.capture(1, p, end);
        let lim = self.lim();
        Some(if self.avail(lim) { lim + 1 } else { lim })
    }

    /// `(///)[gimuy]*`.
    pub(super) fn heregex_end(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"///")?;
        self.capture(1, p, end);
        Some(self.skip(end, |b| matches!(b, b'g' | b'i' | b'm' | b'u' | b'y')))
    }

    /// `\\[Bb]|[$^]`.
    pub(super) fn anchor(&self, p: usize) -> Option<usize> {
        match self.at(p) {
            b'$' | b'^' => Some(p + 1),
            b'\\' => matches!(self.at(p + 1), b'B' | b'b').then_some(p + 2),
            _ => None,
        }
    }

    /// `\\[1-9]\d*`.
    pub(super) fn back_reference(&self, p: usize) -> Option<usize> {
        let from = self.lit(p, b"\\")?;
        matches!(self.at(from), b'1'..=b'9').then(|| self.digit_end(from + 1))
    }

    /// `[*+?]|\{(\d+,\d+|\d+,|,\d+|\d+)}\??`.
    /// The branches take digits, a comma and digits.
    /// At least one side has a digit, and a comma follows a missing first
    /// number. Giving back digits leaves a digit where a comma or `}` must
    /// stand.
    pub(super) fn quantifier(&self, p: usize) -> Option<usize> {
        if matches!(self.at(p), b'*' | b'+' | b'?') {
            return Some(p + 1);
        }
        let first = self.lit(p, b"{")?;
        let comma = self.digit_end(first);
        let (second, has_comma) = match self.lit(comma, b",") {
            Some(after) => (after, true),
            None => (comma, false),
        };
        let close = self.digit_end(second);
        let numbered = comma > first || (has_comma && close > second);
        let end = self.lit(close, b"}").filter(|_| numbered)?;
        Some(self.lit(end, b"?").unwrap_or(end))
    }

    /// `(\()((\?=)|(\?!))`.
    pub(super) fn assertion(&mut self, p: usize) -> Option<usize> {
        let open = self.lit(p, b"(")?;
        let group = match self.text(open, (open + 2).min(self.lim())) {
            b"?=" => 3,
            b"?!" => 4,
            _ => return None,
        };
        self.capture(1, p, open);
        self.capture(group, open, open + 2);
        Some(open + 2)
    }

    /// `(\[)(\^)?`.
    pub(super) fn character_set(&mut self, p: usize) -> Option<usize> {
        let open = self.lit(p, b"[")?;
        self.capture(1, p, open);
        let Some(end) = self.lit(open, b"^") else {
            return Some(open);
        };
        self.capture(2, open, end);
        Some(end)
    }

    /// `(?:.|(\\(?:[0-7]{3}|x\h{2}|u\h{4}))|(\\c[A-Z])|(\\.))-(?:[^]\\]|…)`,
    /// the right side with the same three escapes in groups 4 to 6.
    /// `.` comes first, so a backslash on the left is tried as a plain
    /// character before each escape.
    /// `[^]\\]` takes the line feed.
    pub(super) fn character_range(&mut self, p: usize) -> Option<usize> {
        let lefts = [
            (self.any_char(p), None),
            (self.numeric_escape(p), Some(1)),
            (self.control_escape(p), Some(2)),
            (self.backslash_any(p), Some(3)),
        ];
        let (dash, left) = lefts.into_iter().find_map(|(end, group)| {
            end.filter(|&end| self.at(end) == b'-')
                .map(|end| (end, group))
        })?;
        let from = dash + 1;
        let plain = match self.at(from) {
            b']' | b'\\' | 0 => None,
            b'\n' => Some(from + 1),
            _ => Some(self.char_end(from)),
        };
        let (end, right) = [
            (plain, None),
            (self.numeric_escape(from), Some(4)),
            (self.control_escape(from), Some(5)),
            (self.backslash_any(from), Some(6)),
        ]
        .into_iter()
        .find_map(|(end, group)| end.map(|end| (end, group)))?;
        if let Some(group) = left {
            self.capture(group, p, dash);
        }
        if let Some(group) = right {
            self.capture(group, from, end);
        }
        Some(end)
    }

    /// `\\[DSWdsw]|\.`.
    pub(super) fn character_class(&self, p: usize) -> Option<usize> {
        match self.at(p) {
            b'.' => Some(p + 1),
            b'\\' => {
                matches!(self.at(p + 1), b'D' | b'S' | b'W' | b'd' | b's' | b'w').then_some(p + 2)
            }
            _ => None,
        }
    }

    /// `\\([0-7]{3}|x\h{2}|u\h{4})`.
    pub(super) fn numeric_escape(&self, p: usize) -> Option<usize> {
        let from = self.lit(p, b"\\")?;
        let run = |from: usize, len: usize, class: fn(&u8) -> bool| {
            (from..from + len)
                .all(|i| class(&self.at(i)))
                .then_some(from + len)
        };
        match self.at(from) {
            b'x' => run(from + 1, 2, u8::is_ascii_hexdigit),
            b'u' => run(from + 1, 4, u8::is_ascii_hexdigit),
            _ => run(from, 3, |b| (b'0'..=b'7').contains(b)),
        }
    }

    /// `\\c[A-Z]`.
    pub(super) fn control_escape(&self, p: usize) -> Option<usize> {
        let from = self.lit(p, b"\\c")?;
        self.at(from).is_ascii_uppercase().then_some(from + 1)
    }

    /// `(<)([-.\w]+)` or `(</)([-.\w]+)`, as `open` picks.
    pub(super) fn jsx_tag(&mut self, p: usize, open: &[u8]) -> Option<usize> {
        let from = self.lit(p, open)?;
        let end = self.tag_name_end(from);
        if end == from {
            return None;
        }
        self.capture(1, p, from);
        self.capture(2, from, end);
        Some(end)
    }

    /// The end of the run of `[-.\w]` characters from `i`.
    fn tag_name_end(&self, mut i: usize) -> usize {
        loop {
            if matches!(self.at(i), b'-' | b'.') {
                i += 1;
            } else if let Some(width) = self.word_at(i) {
                i += width;
            } else {
                return i;
            }
        }
    }

    /// `(/?>)`.
    pub(super) fn jsx_tag_end(&self, p: usize) -> Option<usize> {
        let from = self.lit(p, b"/").unwrap_or(p);
        self.lit(from, b">")
    }

    /// `(?:^|\s+)([-.\w]+)\s*(=)`.
    /// `^` is tried first, at the start of the line.
    /// Giving back the `\s+` or the name leaves a character the next part
    /// cannot take.
    pub(super) fn jsx_attribute(&mut self, p: usize) -> Option<usize> {
        if p == 0
            && let Some(end) = self.attribute_at(p)
        {
            return Some(end);
        }
        let name = self.space_end(p);
        if name == p {
            return None;
        }
        self.attribute_at(name)
    }

    /// `([-.\w]+)\s*(=)` at `p`.
    fn attribute_at(&mut self, p: usize) -> Option<usize> {
        let name = self.tag_name_end(p);
        if name == p {
            return None;
        }
        let equals = self.space_end(name);
        let end = self.lit(equals, b"=")?;
        self.capture(1, p, name);
        self.capture(2, equals, end);
        Some(end)
    }
}
