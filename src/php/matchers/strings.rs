//! Strings, regular expressions in strings, SQL strings, interpolation,
//! heredocs and nowdocs.

use super::super::engine::Lexer;
use crate::php_family::matchers::is_name;
use crate::rules::Chars;

impl Lexer<'_> {
    /// `(?i)(?<=^|\s)(#)\s(?=[-\t !,.0-9?_a-z\x7F-\x{10FFFF}[^\x00-\x7F]]*$)`:
    /// a comment in a heredoc's regular expression, whose text holds only
    /// words, digits, blanks and light punctuation.
    pub(super) fn regex_comment(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'#' || p >= self.lim() {
            return None;
        }
        if p != 0 && !self.char_before(p).is_some_and(crate::classes::is_space) {
            return None;
        }
        let space = self.space_at(p + 1);
        if space == 0 {
            return None;
        }
        let text = p + 1 + space;
        let plain = |b: u8| {
            matches!(b, b'-' | b'\t' | b' ' | b'!' | b',' | b'.' | b'?' | b'_')
                || b.is_ascii_alphanumeric()
                || b >= 0x7f
        };
        if text < self.lim() && self.skip(text, plain) != self.lim() {
            return None;
        }
        self.capture(1, p, p + 1);
        Some(text)
    }

    /// `"\s*(?=(SELECT|INSERT|UPDATE|DELETE|CREATE|REPLACE|ALTER|AND|WITH)\b)`
    /// for `"`, and with `'` for `'`.
    pub(super) fn sql_open(&self, p: usize, quote: u8) -> Option<usize> {
        if self.at(p) != quote || p >= self.lim() {
            return None;
        }
        let at = self.space_end(p + 1);
        let words: [&[u8]; 9] = [
            b"SELECT", b"INSERT", b"UPDATE", b"DELETE", b"CREATE", b"REPLACE", b"ALTER", b"AND",
            b"WITH",
        ];
        words
            .iter()
            .any(|word| {
                self.lit(at, word)
                    .is_some_and(|end| self.word_at(end).is_none())
            })
            .then_some(at)
    }

    /// `(?i)(?=<<<\s*("?)(NAME)(\1)\s*$)` for `"`, and
    /// `(?i)(?=<<<\s*'(NAME)'\s*$)` for `'`.
    pub(super) fn heredoc_ahead(&self, p: usize, quote: u8) -> Option<usize> {
        let at = self.space_end(self.lit(p, b"<<<")?);
        let quoted = self.at(at) == quote;
        if quote == b'\'' && !quoted {
            return None;
        }
        let name = self.name(at + usize::from(quoted))?;
        let after = name + usize::from(quoted);
        if quoted && self.at(name) != quote {
            return None;
        }
        self.at_eol(self.space_end(after)).then_some(p)
    }

    /// `(?i)(<<<)\s*("?)([_a-z\x7F-\x{10FFFF}]+[0-9_a-z\x7F-\x{10FFFF}]*)(\2)(\
    /// s*)`.
    pub(super) fn heredoc_open_any(&mut self, p: usize) -> Option<usize> {
        let arrows = self.lit(p, b"<<<")?;
        let at = self.space_end(arrows);
        let quoted = self.at(at) == b'"';
        for (quoted, from) in [(quoted, at + usize::from(quoted)), (false, at)] {
            let Some(end) = self.name(from) else {
                continue;
            };
            if quoted && self.at(end) != b'"' {
                continue;
            }
            let spaces = end + usize::from(quoted);
            let line_end = self.space_end(spaces);
            self.capture(1, p, arrows);
            self.capture(3, from, end);
            self.capture(5, spaces, line_end);
            return Some(line_end);
        }
        None
    }

    /// `(?i)(<<<)\s*'([_a-z\x7F-\x{10FFFF}]+[0-9_a-z\x7F-\x{10FFFF}]*)'(\s*)`.
    pub(super) fn nowdoc_open_any(&mut self, p: usize) -> Option<usize> {
        let arrows = self.lit(p, b"<<<")?;
        let at = self.space_end(arrows);
        if self.at(at) != b'\'' {
            return None;
        }
        let end = self.name(at + 1)?;
        if self.at(end) != b'\'' {
            return None;
        }
        let line_end = self.space_end(end + 1);
        self.capture(1, p, arrows);
        self.capture(2, at + 1, end);
        self.capture(3, end + 1, line_end);
        Some(line_end)
    }

    /// `^\s*(LABEL)(?![0-9A-Z_a-z\x7F-\x{10FFFF}])`, the line that closes a
    /// heredoc or a nowdoc, with the label as group 1.
    /// Without `label`, the label is the one the begin took, `\3` or `\2`.
    pub(super) fn label_line(&mut self, p: usize, label: Option<&'static [u8]>) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let at = self.space_end(0);
        let label = label.map_or_else(|| self.begin_text().as_bytes(), |label| label);
        if label.is_empty() {
            return None;
        }
        let end = self.lit(at, label)?;
        if end < self.lim() && is_name(self.at(end)) {
            return None;
        }
        self.capture(1, at, end);
        Some(end)
    }
}
