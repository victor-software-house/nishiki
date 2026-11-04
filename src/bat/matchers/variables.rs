//! Escapes, parameters and variable expansions.

use super::super::engine::Lexer;
use crate::classes::is_digit;
use crate::rules::Chars;

impl Lexer<'_> {
    /// `%%|\^\^!|\^(?=.)|\^\n`.
    pub(super) fn escape(&self, p: usize) -> Option<usize> {
        if self.starts(p, b"%%") {
            return Some(p + 2);
        }
        self.lit_byte(p, b'^')?;
        if self.starts(p, b"^^!") {
            return Some(p + 3);
        }
        if p + 1 < self.lim() {
            return Some(p + 1);
        }
        (self.at(p + 1) == b'\n').then_some(p + 2)
    }

    /// The end of `[adfnpstxz]*` under `(?i)` from `i`: the modifiers of a
    /// parameter, `ſ` among them, since it folds to `s`.
    fn modifier_end(&self, i: usize) -> usize {
        let mut at = i;
        loop {
            match self.s.get(at..) {
                Some(
                    [
                        b'a' | b'd' | b'f' | b'n' | b'p' | b's' | b't' | b'x' | b'z' | b'A' | b'D'
                        | b'F' | b'N' | b'P' | b'S' | b'T' | b'X' | b'Z',
                        ..,
                    ],
                ) => at += 1,
                Some([0xc5, 0xbf, ..]) => at += 2,
                _ => break,
            }
        }
        self.spend(at - i);
        at
    }

    /// The end of the `\d` at `i`, when one stands there.
    fn digit_at(&self, i: usize) -> Option<usize> {
        if i >= self.lim() {
            return None;
        }
        self.char_at(i)
            .filter(|&(c, _)| is_digit(c))
            .map(|(_, end)| end)
    }

    /// `(%)(?:(?i:~[adfnpstxz]*(?:\$PATH:)?)?\d|\*)`: a parameter such as
    /// `%1` or `%~dp0`, with group 1 around the `%`.
    ///
    /// No modifier given back helps: a modifier is a letter, never `\d`.
    pub(super) fn parameter(&mut self, p: usize) -> Option<usize> {
        self.lit_byte(p, b'%')?;
        let after = p + 1;
        let mut end = None;
        if self.at(after) == b'~' && after < self.lim() {
            let mods = self.modifier_end(after + 1);
            let path = self.folded(mods, b"$path:").unwrap_or(mods);
            end = self.digit_at(path);
        }
        let end = end
            .or_else(|| self.digit_at(after))
            .or_else(|| self.lit_byte(after, b'*'))?;
        self.cap(1, p, p + 1);
        Some(end)
    }

    /// `(%%)(?i:~[adfnpstxz]*(?:\$PATH:)?)?[A-Za-z]`: a `for` loop's
    /// parameter, with group 1 around `%%`.
    pub(super) fn repeat_parameter(&mut self, p: usize) -> Option<usize> {
        let end = self.repeat_parameter_end(p)?;
        self.cap(1, p, p + 2);
        Some(end)
    }

    /// Where `#repeatParameter` ends at `p`, without recording groups.
    ///
    /// The modifiers are letters, so `[A-Za-z]` after them can take back
    /// the last ASCII one: `%%~nx` is the modifier `n` and the name `x`.
    /// A `ſ` given back is no ASCII letter, so the one before it is tried.
    pub(super) fn repeat_parameter_end(&self, p: usize) -> Option<usize> {
        if !self.starts(p, b"%%") {
            return None;
        }
        let tilde = p + 2;
        let letter =
            |at: usize| (at < self.lim() && self.at(at).is_ascii_alphabetic()).then_some(at + 1);
        if self.at(tilde) == b'~' && tilde < self.lim() {
            let first = tilde + 1;
            let mods = self.modifier_end(first);
            if let Some(path) = self.folded(mods, b"$path:")
                && let Some(end) = letter(path)
            {
                return Some(end);
            }
            if let Some(end) = letter(mods) {
                return Some(end);
            }
            let given_back = self
                .text(first, mods)
                .iter()
                .rposition(u8::is_ascii_alphabetic);
            if let Some(at) = given_back {
                return Some(first + at + 1);
            }
        }
        letter(tilde)
    }

    /// `%(?=[^%]+%)` or `!(?=[^!]+!)` for the byte `mark`: an expansion
    /// that a later `mark` on the line closes.
    pub(super) fn expansion(&mut self, p: usize, mark: u8) -> Option<usize> {
        if self.lit_byte(p, mark).is_none() || p + 1 >= self.lim() || self.at(p + 1) == mark {
            return None;
        }
        let marks = self.marks();
        let last = if mark == b'%' {
            marks.percent
        } else {
            marks.bang
        };
        last.is_some_and(|last| last > p + 1).then_some(p + 1)
    }

    /// `([-+]?\d+)(?:(,)([-+]?\d+))?`: a substring's offset and length, with
    /// groups around each number and the comma.
    pub(super) fn substring(&mut self, p: usize) -> Option<usize> {
        let first = self.signed_digits(p)?;
        self.cap(1, p, first);
        if self.lit_byte(first, b',').is_some()
            && let Some(second) = self.signed_digits(first + 1)
        {
            self.cap(2, first, first + 1);
            self.cap(3, first + 1, second);
            return Some(second);
        }
        Some(first)
    }

    /// The end of `[-+]?\d+` at `i`.
    fn signed_digits(&self, i: usize) -> Option<usize> {
        let digits = if matches!(self.at(i), b'-' | b'+') && i < self.lim() {
            i + 1
        } else {
            i
        };
        let end = self.digit_end(digits);
        (end > digits).then_some(end)
    }

    /// `[^\n!%=]+`.
    pub(super) fn replace_text(&self, p: usize) -> Option<usize> {
        let end = self.skip(p, |b| !matches!(b, b'!' | b'%' | b'='));
        (end > p).then_some(end)
    }

    /// `[^x]+` for the byte `x`, which takes the line feed too.
    pub(super) fn run_without(&self, p: usize, byte: u8) -> Option<usize> {
        let mut end = self.skip(p, |b| b != byte);
        if end == self.lim() && self.at(end) == b'\n' {
            end += 1;
        }
        (end > p).then_some(end)
    }
}
