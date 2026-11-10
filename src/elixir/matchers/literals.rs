//! Strings, sigils, documentation, escapes, numbers and character literals.

use super::super::engine::Lexer;
use crate::classes::{is_digit, is_space, is_word};
use crate::rules::Chars;

impl Lexer<'_> {
    /// `\s*~L"""` or `\s*~H"""`, `opening` after the spaces: a sigil that
    /// holds HTML.
    pub(super) fn html_sigil(&self, p: usize, opening: &[u8]) -> Option<usize> {
        self.lit(self.space_end(p), opening)
    }

    /// `\s*"""` or `\s*'''`, `close` after the spaces: the end of a sigil or
    /// a documentation heredoc, which may come mid-line.
    pub(super) fn spaced_close(&self, p: usize, close: &[u8]) -> Option<usize> {
        self.lit(self.space_end(p), close)
    }

    /// `^\s*"""` or `^\s*'''`: the line that closes a heredoc.
    pub(super) fn line_close(&self, p: usize, close: &[u8]) -> Option<usize> {
        if p != 0 {
            return None;
        }
        self.lit(self.space_end(0), close)
    }

    /// `@(module|type)?doc`: the end of a documentation attribute.
    /// A word that starts `module` or `type` but goes on with no `doc` falls
    /// back to the bare `@doc`, which then fails at its first letter.
    pub(super) fn doc_attribute(&self, p: usize) -> Option<usize> {
        let at = self.lit(p, b"@")?;
        self.lit(at, b"moduledoc")
            .or_else(|| self.lit(at, b"typedoc"))
            .or_else(|| self.lit(at, b"doc"))
    }

    /// `@(module|type)?doc (~[a-z])?quotes` or, when `upper`,
    /// `@(module|type)?doc ~[A-Z]quotes`: a documentation heredoc.
    /// Without the quotes after it, the optional sigil gives way, and the
    /// quotes must then start at the `~`.
    pub(super) fn doc_heredoc(&self, p: usize, quotes: &[u8], upper: bool) -> Option<usize> {
        let at = self.lit(self.doc_attribute(p)?, b" ")?;
        let letter = self.at(at + 1);
        let sigil = self.starts(at, b"~")
            && at + 1 < self.lim()
            && if upper {
                letter.is_ascii_uppercase()
            } else {
                letter.is_ascii_lowercase()
            };
        match (sigil, upper) {
            (true, _) => self
                .lit(at + 2, quotes)
                .or_else(|| (!upper).then(|| self.lit(at, quotes)).flatten()),
            (false, true) => None,
            (false, false) => self.lit(at, quotes),
        }
    }

    /// `\\(x[A-Fa-f\d]{1,2}|.)`: an escape.
    /// `{1,2}` takes two characters where it can, and `.` takes the `x` when
    /// no digit follows it.
    pub(super) fn escape(&mut self, p: usize) -> Option<usize> {
        let at = self.lit(p, b"\\")?;
        if self.starts(at, b"x") {
            let mut end = at + 1;
            for _ in 0..2 {
                match self.char_at(end) {
                    Some((c, next)) if c.is_ascii_hexdigit() || is_digit(c) => end = next,
                    Some(_) | None => break,
                }
            }
            if end > at + 1 {
                self.capture(1, at, end);
                return Some(end);
            }
        }
        let (_, end) = self.char_at(at)?;
        self.capture(1, at, end);
        Some(end)
    }

    /// `~[a-z]opening` or, when `upper`, `~[A-Z]opening`: a sigil with a
    /// fixed delimiter.
    pub(super) fn sigil_open(&self, p: usize, upper: bool, opening: &[u8]) -> Option<usize> {
        let letter = self.lit(p, b"~")?;
        self.sigil_letter(letter, upper)
            .then(|| self.lit(letter + 1, opening))
            .flatten()
    }

    /// `[a-z]` at `p`, or `[A-Z]` when `upper`.
    fn sigil_letter(&self, p: usize, upper: bool) -> bool {
        let b = self.at(p);
        p < self.lim()
            && if upper {
                b.is_ascii_uppercase()
            } else {
                b.is_ascii_lowercase()
            }
    }

    /// `~[a-z](\W)` or, when `upper`, `~[A-Z](\W)`: a sigil whose delimiter
    /// is any character that is not a `\w`.
    /// The line feed is one, so a sigil can open at the end of a line.
    pub(super) fn sigil_any(&mut self, p: usize, upper: bool) -> Option<usize> {
        let letter = self.lit(p, b"~")?;
        if !self.sigil_letter(letter, upper) {
            return None;
        }
        let delimiter = letter + 1;
        let end = match self.char_at(delimiter) {
            Some((c, end)) => (!is_word(c)).then_some(end)?,
            None => (self.at(delimiter) == b'\n').then_some(delimiter + 1)?,
        };
        self.capture(1, delimiter, end);
        Some(end)
    }

    /// `close[a-z]*`: the end of a sigil with a fixed delimiter, and its
    /// modifiers.
    pub(super) fn sigil_close(&self, p: usize, close: u8) -> Option<usize> {
        (p < self.lim() && self.at(p) == close)
            .then(|| self.skip(p + 1, |b| b.is_ascii_lowercase()))
    }

    /// `\1[a-z]*`: the end of a sigil that repeats its delimiter.
    /// A delimiter that was the line feed lies outside the line's text.
    /// Its begin text is therefore empty, and the next line feed closes the
    /// sigil.
    pub(super) fn sigil_any_close(&self, p: usize) -> Option<usize> {
        if self.begin_text().is_empty() {
            return (p == self.lim() && self.at(p) == b'\n').then_some(p + 1);
        }
        let width = self.begin_text_at(p)?;
        Some(self.skip(p + width, |b| b.is_ascii_lowercase()))
    }

    /// `\b0letter digit(?>_?digit)*\b`: a hexadecimal, binary or octal
    /// number.
    /// Each `(?>_?digit)` is atomic, and a shorter run leaves a digit or a
    /// `_`, both `\w`, where the last `\b` must hold.
    pub(super) fn radix(&self, p: usize, letter: u8, digit: fn(u8) -> bool) -> Option<usize> {
        if self.word_before(p) || !self.starts(p, b"0") || self.at(p + 1) != letter {
            return None;
        }
        let first = p + 2;
        if !(first < self.lim() && digit(self.at(first))) {
            return None;
        }
        let mut end = first + 1;
        loop {
            if self.at(end) == b'_' && end + 1 < self.lim() && digit(self.at(end + 1)) {
                end += 2;
            } else if end < self.lim() && digit(self.at(end)) {
                end += 1;
            } else {
                break;
            }
        }
        self.word_at(end).is_none().then_some(end)
    }

    /// The width of the `\d` character at `p`.
    fn digit_width(&self, p: usize) -> Option<usize> {
        let (c, end) = self.char_at(p)?;
        is_digit(c).then_some(end - p)
    }

    /// `\d(?>_?\d)*`: the end of a run of digits at `p`.
    fn digit_run(&self, p: usize) -> Option<usize> {
        let mut end = p + self.digit_width(p)?;
        loop {
            if self.at(end) == b'_'
                && let Some(width) = self.digit_width(end + 1)
            {
                end += 1 + width;
            } else if let Some(width) = self.digit_width(end) {
                end += width;
            } else {
                return Some(end);
            }
        }
    }

    /// `\b\d(?>_?\d)*\b`: an integer.
    /// A shorter run leaves a `\w` where the last `\b` must hold.
    pub(super) fn integer(&self, p: usize) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        let end = self.digit_run(p)?;
        self.word_at(end).is_none().then_some(end)
    }

    /// `\b\d(?>_?\d)*(\.(?![^\s\d])(?>_?\d)+)([Ee][-+]?\d(?>_?\d)*)?\b`: a
    /// float.
    ///
    /// The lookahead lets the fraction start only with a digit: a `_` fails
    /// it, and a space then fails `(?>_?\d)+`.
    /// Without the exponent, the last `\b` must hold after the fraction,
    /// where an `e` that starts no exponent is a `\w`.
    pub(super) fn float(&self, p: usize) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        let point = self.digit_run(p)?;
        if !self.starts(point, b".") {
            return None;
        }
        let fraction = self.digit_run(point + 1)?;
        if matches!(self.at(fraction), b'E' | b'e') && fraction < self.lim() {
            let sign = fraction + 1;
            let digits = if matches!(self.at(sign), b'+' | b'-') && sign < self.lim() {
                sign + 1
            } else {
                sign
            };
            if let Some(end) = self.digit_run(digits)
                && self.word_at(end).is_none()
            {
                return Some(end);
            }
        }
        self.word_at(fraction).is_none().then_some(fraction)
    }

    /// `(&)\d+`: a captured argument by position.
    pub(super) fn capture_number(&mut self, p: usize) -> Option<usize> {
        let start = self.lit(p, b"&")?;
        let end = self.digit_end(start);
        if end == start {
            return None;
        }
        self.capture(1, p, start);
        Some(end)
    }

    /// `(?<!\w)\?(\\(x\h{1,2}(?!\h)\b|[^CMx])|[^\\\s])`: a character
    /// literal.
    ///
    /// After `\x`, `\h{1,2}` holds only when one or two hexadecimal digits
    /// follow and no third: a third fails `(?!\h)` at either length.
    /// The negated class matches the line feed.
    /// When the escape fails, `[^\\\s]` fails at the backslash.
    pub(super) fn char_literal(&self, p: usize) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        let at = self.lit(p, b"?")?;
        if self.starts(at, b"\\") {
            let escaped = at + 1;
            if self.starts(escaped, b"x") {
                let digits = self.skip(escaped + 1, |b| b.is_ascii_hexdigit());
                let run = digits - escaped - 1;
                if (run == 1 || run == 2) && self.word_at(digits).is_none() {
                    return Some(digits);
                }
            }
            return match self.char_at(escaped) {
                Some((c, end)) => (!matches!(c, 'C' | 'M' | 'x')).then_some(end),
                None => (self.at(escaped) == b'\n').then_some(escaped + 1),
            };
        }
        let (c, end) = self.char_at(at)?;
        (!is_space(c)).then_some(end)
    }
}
