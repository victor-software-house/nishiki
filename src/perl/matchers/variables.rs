//! Variables, such as `$x`, `@{x}`, `$#x` and `$&`, and the escapes inside
//! interpolated strings and regular expressions.

use super::super::engine::{Lexer, is_special};
use crate::classes::is_ascii_word;
use crate::rules::Chars;

impl Lexer<'_> {
    /// Whether no `[0-9A-Z_a-z]` stands at `p`, as `(?![0-9A-Z_a-z])` asks.
    fn no_ascii_word(&self, p: usize) -> bool {
        !(p < self.lim() && is_ascii_word(self.at(p)))
    }

    /// `(\$)&(?![0-9A-Z_a-z])` and the like: `$` and one mark.
    pub(super) fn punct_var(&mut self, p: usize, mark: u8) -> Option<usize> {
        (self.at(p) == b'$'
            && self.at(p + 1) == mark
            && p + 1 < self.lim()
            && self.no_ascii_word(p + 2))
        .then(|| {
            self.cap(1, p, p + 1);
            p + 2
        })
    }

    /// `(\$)[!#$%()*,-/:-@\[-_ab|~](?![0-9A-Z_a-z])`: a special variable.
    pub(super) fn special_var(&mut self, p: usize) -> Option<usize> {
        (self.at(p) == b'$'
            && is_special(self.at(p + 1))
            && p + 1 < self.lim()
            && self.no_ascii_word(p + 2))
        .then(|| {
            self.cap(1, p, p + 1);
            p + 2
        })
    }

    /// `(\$)[0-9]+(?![0-9A-Z_a-z])`: a numbered match variable.
    /// The digits give nothing back: a shorter run leaves a digit next.
    pub(super) fn digit_var(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'$' || p >= self.lim() {
            return None;
        }
        let end = self.skip(p + 1, |b| b.is_ascii_digit());
        (end > p + 1 && self.no_ascii_word(end)).then(|| {
            self.cap(1, p, p + 1);
            end
        })
    }

    /// The sigil `[$%@](#)?` at `p`, and where it ends with and without its
    /// `#`.
    fn sigil(&self, p: usize) -> Option<(usize, Option<usize>)> {
        if !matches!(self.at(p), b'$' | b'%' | b'@') || p >= self.lim() {
            return None;
        }
        let hash = (self.at(p + 1) == b'#' && p + 1 < self.lim()).then_some(p + 2);
        Some((p + 1, hash))
    }

    /// One unit of `([$0-9A-Z_a-z]|::)` at `p`, or of `([$7A-Za-z]|::)` when
    /// `first`.
    fn name_unit(&self, p: usize, first: bool) -> Option<usize> {
        if p >= self.lim() {
            return None;
        }
        let b = self.at(p);
        let one = if first {
            b == b'$' || b == b'7' || b.is_ascii_alphabetic()
        } else {
            b == b'$' || is_ascii_word(b)
        };
        if one {
            Some(p + 1)
        } else {
            self.starts(p, b"::").then_some(p + 2)
        }
    }

    /// `([$7A-Za-z]|::)([$0-9A-Z_a-z]|::)*\b` from `p`: the end of the
    /// longest name that a word boundary follows.
    /// Units go back one at a time from the longest name, so the last end
    /// with a boundary wins.
    fn name_to_boundary(&self, p: usize) -> Option<usize> {
        let mut at = self.name_unit(p, true)?;
        let mut found = self.boundary(at).then_some(at);
        while let Some(next) = self.name_unit(at, false) {
            at = next;
            if self.boundary(at) {
                found = Some(at);
            }
        }
        self.spend(at - p);
        found
    }

    /// `([$%@](#)?)([$7A-Za-z]|::)([$0-9A-Z_a-z]|::)*\b`: a named variable.
    /// Without its `#`, the name would start with `#`, which it cannot.
    pub(super) fn named_var(&mut self, p: usize) -> Option<usize> {
        let (sigil, hash) = self.sigil(p)?;
        let sigil = hash.unwrap_or(sigil);
        let end = self.name_to_boundary(sigil)?;
        self.cap(1, p, sigil);
        Some(end)
    }

    /// `(\$\{)(?:[$7A-Za-z]|::)(?:[$0-9A-Z_a-z]|::)*(})`: a braced name.
    /// The name stops at the `}`, which it cannot hold.
    pub(super) fn braced_var(&mut self, p: usize) -> Option<usize> {
        if !self.starts(p, b"${") {
            return None;
        }
        let mut at = self.name_unit(p + 2, true)?;
        while let Some(next) = self.name_unit(at, false) {
            at = next;
        }
        self.spend(at - p);
        (self.at(at) == b'}' && at < self.lim()).then(|| {
            self.cap(1, p, p + 2);
            self.cap(2, at, at + 1);
            at + 1
        })
    }

    /// `([$%@](#)?)[0-9_]\b`: a sigil and one digit or `_`.
    pub(super) fn short_var(&mut self, p: usize) -> Option<usize> {
        let (sigil, hash) = self.sigil(p)?;
        let sigil = hash.unwrap_or(sigil);
        if !(self.at(sigil).is_ascii_digit() || self.at(sigil) == b'_') || sigil >= self.lim() {
            return None;
        }
        self.boundary(sigil + 1).then(|| {
            self.cap(1, p, sigil);
            sigil + 1
        })
    }

    /// `\\\d+`: a backslash and digits.
    pub(super) fn escape_digits(&self, p: usize) -> Option<usize> {
        let end = self.digit_end(self.lit(p, b"\\")?);
        (end > p + 1).then_some(end)
    }

    /// `\\c[^\\\s]`: a control character.
    pub(super) fn escape_control(&self, p: usize) -> Option<usize> {
        let at = self.lit(p, b"\\c")?;
        self.char_in(at, |b| b != b'\\')
            .filter(|_| !self.is_space_at(at))
    }

    /// `\\g(?:\{(?:\w*|-\d+)}|\d+)`: a back-reference by number or name.
    pub(super) fn escape_group(&self, p: usize) -> Option<usize> {
        let at = self.lit(p, b"\\g")?;
        if self.at(at) == b'{' && at < self.lim() {
            let word = self.word_end(at + 1);
            if self.at(word) == b'}' && word < self.lim() {
                return Some(word + 1);
            }
            if self.at(at + 1) == b'-' {
                let digits = self.digit_end(at + 2);
                if digits > at + 2 && self.at(digits) == b'}' && digits < self.lim() {
                    return Some(digits + 1);
                }
            }
            return None;
        }
        let digits = self.digit_end(at);
        (digits > at).then_some(digits)
    }

    /// `\\k(?:\{\w*}|<\w*>|'\w*')`: a back-reference by name.
    pub(super) fn escape_named(&self, p: usize) -> Option<usize> {
        let at = self.lit(p, b"\\k")?;
        let close = match self.at(at) {
            b'{' => b'}',
            b'<' => b'>',
            b'\'' => b'\'',
            _ => return None,
        };
        if at >= self.lim() {
            return None;
        }
        let word = self.word_end(at + 1);
        (self.at(word) == close && word < self.lim()).then_some(word + 1)
    }

    /// `\\N\{[^}]*}`: a character by name.
    pub(super) fn escape_char_name(&self, p: usize) -> Option<usize> {
        let at = self.lit(p, b"\\N{")?;
        let end = self.skip(at, |b| b != b'}');
        (self.at(end) == b'}' && end < self.lim()).then_some(end + 1)
    }

    /// `\\o\{\d*}`: an octal escape.
    pub(super) fn escape_octal(&self, p: usize) -> Option<usize> {
        let end = self.digit_end(self.lit(p, b"\\o{")?);
        (self.at(end) == b'}' && end < self.lim()).then_some(end + 1)
    }

    /// `\\[Pp](?:\{\w*}|P)`: a Unicode property.
    pub(super) fn escape_property(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'\\' || !matches!(self.at(p + 1), b'P' | b'p') || p + 1 >= self.lim() {
            return None;
        }
        let at = p + 2;
        if self.at(at) == b'{' && at < self.lim() {
            let word = self.word_end(at + 1);
            if self.at(word) == b'}' && word < self.lim() {
                return Some(word + 1);
            }
        }
        (self.at(at) == b'P' && at < self.lim()).then_some(at + 1)
    }

    /// `\\x(?:[0-9A-Za-z]{2}|\{\w*})?`: a hex escape, whose digits may be
    /// missing.
    pub(super) fn escape_hex(&self, p: usize) -> Option<usize> {
        let at = self.lit(p, b"\\x")?;
        if self.at(at).is_ascii_alphanumeric()
            && self.at(at + 1).is_ascii_alphanumeric()
            && at + 1 < self.lim()
        {
            return Some(at + 2);
        }
        if self.at(at) == b'{' && at < self.lim() {
            let word = self.word_end(at + 1);
            if self.at(word) == b'}' && word < self.lim() {
                return Some(word + 1);
            }
        }
        Some(at)
    }

    /// `\\.`: a backslash and the character after it.
    pub(super) fn escape_any(&self, p: usize) -> Option<usize> {
        let at = self.lit(p, b"\\")?;
        (at < self.lim()).then(|| self.char_end(at))
    }
}
