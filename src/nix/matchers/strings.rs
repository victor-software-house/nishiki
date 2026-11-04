//! Strings, paths, URIs and comments.

use super::Lexer;
use crate::rules::Chars;

/// Whether `b` is in the grammar's path class, `[-+.0-9A-Z_a-z]`.
fn is_path(b: u8) -> bool {
    matches!(b, b'-' | b'+' | b'.' | b'_') || b.is_ascii_alphanumeric()
}

/// Whether `b` is in the class a URI's scheme goes on with, `[-+.0-9A-Za-z]`.
fn is_scheme(b: u8) -> bool {
    matches!(b, b'-' | b'+' | b'.') || b.is_ascii_alphanumeric()
}

/// Whether `b` is in the class of a URI's rest, `[!$-'*-:=?-Z_a-z~]`.
fn is_uri(b: u8) -> bool {
    matches!(b, b'!' | b'$'..=b'\'' | b'*'..=b':' | b'=' | b'?'..=b'Z' | b'_' | b'a'..=b'z' | b'~')
}

impl Lexer<'_> {
    /// `''(?![$']|\\.)`: the end of an indented string.
    /// A `''` before `$`, `'` or an escaped character is an escape instead.
    pub(super) fn indented_close(&self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"''")?;
        let escapes = match self.at(end) {
            b'$' | b'\'' => true,
            b'\\' => end + 1 < self.lim(),
            _ => false,
        };
        (!escapes).then_some(end)
    }

    /// `''([$']|\\.)`: an escape in an indented string.
    pub(super) fn indented_escape(&self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"''")?;
        match self.at(end) {
            b'$' | b'\'' => Some(end + 1),
            b'\\' if end + 1 < self.lim() => Some(self.char_end(end + 1)),
            _ => None,
        }
    }

    /// `\\.`: an escape in a quoted string.
    /// The `.` takes any character but the line feed.
    pub(super) fn escape(&self, p: usize) -> Option<usize> {
        (self.at(p) == b'\\' && p + 1 < self.lim()).then(|| self.char_end(p + 1))
    }

    /// `(~?[-+.0-9A-Z_a-z]*(/[-+.0-9A-Z_a-z]+)+)`: a path.
    ///
    /// No run takes a `/`, so the runs give back nothing the next part could
    /// use.
    /// Without the `~`, the path would start at the `~`, which no class holds.
    pub(super) fn path_text(&self, p: usize) -> Option<usize> {
        let start = if self.at(p) == b'~' { p + 1 } else { p };
        let mut end = self.skip(start, is_path);
        let mut parts = 0_usize;
        while self.at(end) == b'/' && is_path(self.at(end + 1)) {
            end = self.skip(end + 1, is_path);
            parts += 1;
        }
        (parts > 0).then_some(end)
    }

    /// `(<[-+.0-9A-Z_a-z]+(/[-+.0-9A-Z_a-z]+)*>)`: a search path.
    /// No run takes a `/` or a `>`, so none gives back anything.
    pub(super) fn search_path(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'<' || !is_path(self.at(p + 1)) {
            return None;
        }
        let mut end = self.skip(p + 1, is_path);
        while self.at(end) == b'/' && is_path(self.at(end + 1)) {
            end = self.skip(end + 1, is_path);
        }
        (self.at(end) == b'>').then_some(end + 1)
    }

    /// `([A-Za-z][-+.0-9A-Za-z]*:[!$-'*-:=?-Z_a-z~]+)`: a URI.
    /// The scheme's run takes no `:`, so it gives back nothing.
    pub(super) fn uri(&self, p: usize) -> Option<usize> {
        if !self.at(p).is_ascii_alphabetic() {
            return None;
        }
        let colon = self.skip(p + 1, is_scheme);
        if self.at(colon) != b':' || !is_uri(self.at(colon + 1)) {
            return None;
        }
        Some(self.skip(colon + 1, is_uri))
    }

    /// `/\*([^*]|\*[^/])*`: the start of a block comment, with as much of
    /// its text as holds no `*/`.
    ///
    /// The run can take a `*` before another `*`, so `/* a **/` runs on past
    /// its `*/`, as in Shiki.
    /// The run takes the line feed too.
    pub(super) fn block_comment_open(&self, p: usize) -> Option<usize> {
        let mut i = self.lit(p, b"/*")?;
        let from = i;
        while self.avail(i) {
            if self.at(i) != b'*' {
                i += 1;
            } else if self.avail(i + 1) && self.at(i + 1) != b'/' {
                i += 2;
            } else {
                break;
            }
        }
        self.spend(i - from);
        Some(i)
    }
}
