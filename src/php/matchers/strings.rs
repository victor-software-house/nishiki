//! Strings, regular expressions in strings, SQL strings, interpolation,
//! heredocs and nowdocs.

use super::super::engine::{Lexer, is_name};
use crate::rules::Chars;

/// `[ADSUXeimsux]`: a flag after a regular expression.
fn is_flag(b: u8) -> bool {
    matches!(
        b,
        b'A' | b'D' | b'S' | b'U' | b'X' | b'e' | b'i' | b'm' | b's' | b'u' | b'x'
    )
}

impl Lexer<'_> {
    /// `"/(?=(\\.|[^"/])++/[ADSUXeimsux]*")` for `"`, and for `'`:
    ///
    /// ```text
    /// '/(?=(\\(?:\\(?:\\['\\]?|[^'])|.)|[^'/])++/[ADSUXeimsux]*')
    /// ```
    ///
    /// The `++` is possessive: each step takes its first alternative that
    /// matches, and none gives back.
    pub(super) fn regex_open(&self, p: usize, quote: u8) -> Option<usize> {
        if self.at(p) != quote || self.at(p + 1) != b'/' {
            return None;
        }
        let mut at = p + 2;
        let mut steps = 0;
        loop {
            let next = if quote == b'\'' && self.at(at) == b'\\' && self.avail(at) {
                self.single_quoted_regex_escape(at).or_else(|| Some(at + 1))
            } else if self.at(at) == b'\\'
                && at < self.lim()
                && self.avail(at + 1)
                && at + 1 < self.lim()
            {
                Some(self.char_end(at + 1))
            } else if self.avail(at) && !matches!(self.at(at), b'/') && self.at(at) != quote {
                Some(if at == self.lim() {
                    at + 1
                } else {
                    self.char_end(at)
                })
            } else {
                None
            };
            match next {
                Some(next) => {
                    at = next;
                    steps += 1;
                }
                None => break,
            }
        }
        self.spend(at - p);
        if steps == 0 || self.at(at) != b'/' {
            return None;
        }
        let flags = self.skip(at + 1, is_flag);
        (self.at(flags) == quote).then_some(p + 2)
    }

    /// `\\(?:\\(?:\\['\\]?|[^'])|.)`.
    pub(super) fn single_quoted_regex_escape(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'\\' || p >= self.lim() {
            return None;
        }
        if self.at(p + 1) == b'\\' && p + 1 < self.lim() {
            if self.at(p + 2) == b'\\' && p + 2 < self.lim() {
                let tail = matches!(self.at(p + 3), b'\'' | b'\\') && p + 3 < self.lim();
                return Some(p + 3 + usize::from(tail));
            }
            if self.avail(p + 2) && self.at(p + 2) != b'\'' {
                return Some(if p + 2 == self.lim() {
                    p + 3
                } else {
                    self.char_end(p + 2)
                });
            }
            return Some(p + 2);
        }
        (p + 1 < self.lim()).then(|| self.char_end(p + 1))
    }

    /// `(/)([ADSUXeimsux]*)(")` for `"`, and with `'` for `'`.
    pub(super) fn regex_close(&self, p: usize, quote: u8) -> Option<usize> {
        if self.at(p) != b'/' {
            return None;
        }
        let flags = self.skip(p + 1, is_flag);
        (self.at(flags) == quote && flags < self.lim()).then_some(flags + 1)
    }

    /// `(\\){1,2}[]$.\[^{}]`.
    pub(super) fn regex_escape(&self, p: usize) -> Option<usize> {
        let special = |b: u8| matches!(b, b']' | b'$' | b'.' | b'[' | b'^' | b'{' | b'}');
        if self.at(p) != b'\\' {
            return None;
        }
        if self.at(p + 1) == b'\\' && special(self.at(p + 2)) {
            return Some(p + 3);
        }
        special(self.at(p + 1)).then_some(p + 2)
    }

    /// `\[(?:\^?])?`.
    pub(super) fn regex_class_open(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'[' {
            return None;
        }
        if self.at(p + 1) == b'^' && self.at(p + 2) == b']' {
            return Some(p + 3);
        }
        Some(p + 1 + usize::from(self.at(p + 1) == b']'))
    }

    /// `(\{)\d+(,\d+)?(})`.
    pub(super) fn regex_repeat(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'{' {
            return None;
        }
        let digits = self.digit_end(p + 1);
        if digits == p + 1 {
            return None;
        }
        let mut close = digits;
        if self.at(digits) == b',' {
            let more = self.digit_end(digits + 1);
            if more > digits + 1 {
                close = more;
            }
        }
        if self.at(close) != b'}' {
            return None;
        }
        self.cap(1, p, p + 1);
        self.cap(3, close, close + 1);
        Some(close + 1)
    }

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
        self.cap(1, p, p + 1);
        Some(text)
    }

    /// `\\[0-7]{1,3}`.
    pub(super) fn octal_escape(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'\\' {
            return None;
        }
        let digits = (p + 1..p + 4)
            .take_while(|&at| matches!(self.at(at), b'0'..=b'7'))
            .count();
        (digits > 0).then_some(p + 1 + digits)
    }

    /// `\\x\h{1,2}`.
    pub(super) fn hex_escape(&self, p: usize) -> Option<usize> {
        let at = self.lit(p, b"\\x")?;
        let digits = (at..at + 2)
            .take_while(|&k| self.at(k).is_ascii_hexdigit())
            .count();
        (digits > 0).then_some(at + digits)
    }

    /// `\\u\{\h+}`.
    pub(super) fn unicode_escape(&self, p: usize) -> Option<usize> {
        let at = self.lit(p, b"\\u{")?;
        let end = self.skip(at, |b| b.is_ascii_hexdigit());
        (end > at && self.at(end) == b'}').then_some(end + 1)
    }

    /// `\{(?=\$.*?})`.
    pub(super) fn brace_interpolation(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'{' || self.at(p + 1) != b'$' {
            return None;
        }
        (p + 2..self.lim())
            .any(|at| self.at(at) == b'}')
            .then_some(p + 1)
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

    /// `(#)(\\"|[^"])*(?="|$)` and `(--)…`, with `quote` the string's quote:
    /// a SQL comment runs to the string's end or the line's, with its line
    /// feed.
    pub(super) fn sql_comment(&mut self, p: usize, mark: &[u8], quote: u8) -> Option<usize> {
        let mut at = self.lit(p, mark)?;
        loop {
            if self.at(at) == b'\\' && self.at(at + 1) == quote && at + 1 < self.lim() {
                at += 2;
            } else if self.avail(at) && !(self.at(at) == quote && at < self.lim()) {
                at = if at == self.lim() {
                    at + 1
                } else {
                    self.char_end(at)
                };
            } else {
                break;
            }
        }
        self.spend(at - p);
        self.cap(1, p, p + mark.len());
        Some(at)
    }

    /// `'(?=((\\')|[^"'])*("|$))` and its kin: a quote `quote` inside a SQL
    /// string whose own quote is `own`, when the rest of the string holds no
    /// unescaped `quote` before its end or the line's.
    pub(super) fn sql_quote(&self, p: usize, quote: u8, own: u8) -> Option<usize> {
        if self.at(p) != quote || p >= self.lim() {
            return None;
        }
        let mut at = p + 1;
        loop {
            let b = self.at(at);
            if at >= self.lim() {
                return Some(p + 1);
            }
            if b == b'\\' && self.at(at + 1) == quote && at + 1 < self.lim() {
                at += 2;
            } else if b == own {
                return Some(p + 1);
            } else if b == quote {
                return None;
            } else {
                at = self.char_end(at);
            }
        }
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

    /// `(<<<)\s*("?)(LABEL)(\2)(\s*)$`, for the first of `labels` that ends
    /// the line, with `<<<`, the label and the trailing spaces as groups 1, 3
    /// and 5.
    pub(super) fn heredoc_open(&mut self, p: usize, labels: &[&[u8]]) -> Option<usize> {
        let arrows = self.lit(p, b"<<<")?;
        let at = self.space_end(arrows);
        let quoted = self.at(at) == b'"';
        let from = at + usize::from(quoted);
        for label in labels {
            let Some(end) = self.lit(from, label) else {
                continue;
            };
            if quoted && self.at(end) != b'"' {
                continue;
            }
            let spaces = end + usize::from(quoted);
            let line_end = self.space_end(spaces);
            if self.at_eol(line_end) {
                self.cap(1, p, arrows);
                self.cap(3, from, end);
                self.cap(5, spaces, line_end);
                return Some(line_end);
            }
        }
        None
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
            self.cap(1, p, arrows);
            self.cap(3, from, end);
            self.cap(5, spaces, line_end);
            return Some(line_end);
        }
        None
    }

    /// `(<<<)\s*'(LABEL)'(\s*)$`, with `<<<`, the label and the trailing
    /// spaces as groups 1, 2 and 3.
    pub(super) fn nowdoc_open(&mut self, p: usize, labels: &[&[u8]]) -> Option<usize> {
        let arrows = self.lit(p, b"<<<")?;
        let at = self.space_end(arrows);
        if self.at(at) != b'\'' {
            return None;
        }
        for label in labels {
            let Some(end) = self.lit(at + 1, label) else {
                continue;
            };
            if self.at(end) != b'\'' {
                continue;
            }
            let line_end = self.space_end(end + 1);
            if self.at_eol(line_end) {
                self.cap(1, p, arrows);
                self.cap(2, at + 1, end);
                self.cap(3, end + 1, line_end);
                return Some(line_end);
            }
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
        self.cap(1, p, arrows);
        self.cap(2, at + 1, end);
        self.cap(3, end + 1, line_end);
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
        let label = label.map_or_else(|| self.payload_text().as_bytes(), |label| label);
        if label.is_empty() {
            return None;
        }
        let end = self.lit(at, label)?;
        if end < self.lim() && is_name(self.at(end)) {
            return None;
        }
        self.cap(1, at, end);
        Some(end)
    }
}
