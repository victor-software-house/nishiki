//! Strings, regular expressions, SQL strings, heredocs and nowdocs.

use super::super::PhpFamily;
use super::is_flag;
use crate::rules::{Chars, Lexer};

impl<G: PhpFamily> Lexer<'_, G> {
    /// `"/(?=(\\.|[^"/])++/[ADSUXeimsux]*")` for `"`, and for `'`:
    ///
    /// ```text
    /// '/(?=(\\(?:\\(?:\\['\\]?|[^'])|.)|[^'/])++/[ADSUXeimsux]*')
    /// ```
    ///
    /// The `++` is possessive: each step takes its first alternative that
    /// matches, and none gives back.
    pub(crate) fn quoted_regex_open(&self, p: usize, quote: u8) -> Option<usize> {
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
    pub(crate) fn single_quoted_regex_escape(&self, p: usize) -> Option<usize> {
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

    /// `\\u\{\h+}`.
    pub(crate) fn unicode_escape(&self, p: usize) -> Option<usize> {
        let at = self.lit(p, b"\\u{")?;
        let end = self.skip(at, |b| b.is_ascii_hexdigit());
        (end > at && self.at(end) == b'}').then_some(end + 1)
    }

    /// `\{(?=\$.*?})`.
    pub(crate) fn brace_interpolation(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'{' || self.at(p + 1) != b'$' {
            return None;
        }
        (p + 2..self.lim())
            .any(|at| self.at(at) == b'}')
            .then_some(p + 1)
    }

    /// `(#)(\\"|[^"])*(?="|$)` and `(--)…`, with `quote` the string's quote:
    /// a SQL comment runs to the string's end or the line's, with its line
    /// feed.
    pub(crate) fn sql_string_comment(&mut self, p: usize, mark: &[u8], quote: u8) -> Option<usize> {
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
        self.capture(1, p, p + mark.len());
        Some(at)
    }

    /// `(<<<)\s*("?)(LABEL)(\2)(\s*)$`, for the first of `labels` that ends
    /// the line, with `<<<`, the label and the trailing spaces as groups 1, 3
    /// and 5.
    pub(crate) fn labelled_heredoc_open(&mut self, p: usize, labels: &[&[u8]]) -> Option<usize> {
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
                self.capture(1, p, arrows);
                self.capture(3, from, end);
                self.capture(5, spaces, line_end);
                return Some(line_end);
            }
        }
        None
    }

    /// `(<<<)\s*'(LABEL)'(\s*)$`, with `<<<`, the label and the trailing
    /// spaces as groups 1, 2 and 3.
    pub(crate) fn nowdoc_open(&mut self, p: usize, labels: &[&[u8]]) -> Option<usize> {
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
                self.capture(1, p, arrows);
                self.capture(2, at + 1, end);
                self.capture(3, end + 1, line_end);
                return Some(line_end);
            }
        }
        None
    }
}
