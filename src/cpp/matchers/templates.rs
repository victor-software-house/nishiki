//! Template arguments between angle brackets, which nest.

use super::super::engine::Lexer;
use super::super::limits::MAX_CALL_DEPTH;

/// The groups of the last comment seen inside template arguments: its
/// marker, its text with the end, and the end.
pub(crate) type LastComment = Option<[(usize, usize); 3]>;

impl Lexer<'_> {
    /// The template arguments `<…>` at `p`: where the closing `>` ends, and
    /// the last comment inside.
    /// They are `(?<!<)<(?!<)`, then any run of a comment, a quoted string, a
    /// nested arguments group that calls itself, or other characters but
    /// quotes, `/` and angle brackets, then `>`.
    /// A `<` that another `<` precedes or follows does not open arguments.
    /// That holds in the group that calls itself too.
    pub(crate) fn template_args(&self, p: usize) -> Option<(usize, LastComment)> {
        let mut last = None;
        let end = self.template_args_in(p, &mut last, 0)?;
        Some((end, last))
    }

    /// One level of [`Lexer::template_args`], `depth` levels in.
    /// Oniguruma lets a group call itself 19 times inside the group that
    /// defines it.
    /// That allows 20 levels of `<`, and a match with a 21st fails.
    /// The body is a possessive run of `[^"'/<>]` and `/[^*]`.
    /// The line feed is a character the class takes.
    fn template_args_in(&self, p: usize, last: &mut LastComment, depth: usize) -> Option<usize> {
        if depth >= MAX_CALL_DEPTH || self.at(p) != b'<' || p >= self.lim() {
            return None;
        }
        if self.prev(p) == Some(b'<') || self.at(p + 1) == b'<' {
            return None;
        }
        let mut pos = p + 1;
        loop {
            self.spend(1);
            let b = self.at(pos);
            if pos >= self.lim() && !(self.nl && pos == self.lim()) {
                return None;
            }
            if self.starts(pos, b"/*") {
                let at = memchr::memmem::find(&self.s[pos + 2..], b"*/")?;
                let close = pos + 2 + at;
                *last = Some([(pos, pos + 2), (pos + 2, close + 2), (close, close + 2)]);
                pos = close + 2;
                continue;
            }
            match b {
                b'"' | b'\'' if pos < self.lim() => {
                    let at = self.s[pos + 1..].iter().position(|&c| c == b)?;
                    pos = pos + 1 + at + 1;
                }
                b'<' if pos < self.lim() => {
                    pos = self.template_args_in(pos, last, depth + 1)?;
                }
                b'>' if pos < self.lim() => return Some(pos + 1),
                _ => {
                    let start = pos;
                    loop {
                        let c = self.at(pos);
                        let real = pos < self.lim();
                        let taken = if real {
                            !matches!(c, b'"' | b'\'' | b'/' | b'<' | b'>')
                        } else {
                            self.nl && pos == self.lim()
                        };
                        if taken {
                            pos += 1;
                        } else if real
                            && c == b'/'
                            && self.at(pos + 1) != b'*'
                            && (pos + 1 < self.lim() || self.nl)
                        {
                            pos += 2;
                        } else {
                            break;
                        }
                    }
                    if pos == start {
                        return None;
                    }
                    if pos > self.lim() {
                        return None;
                    }
                }
            }
        }
    }

    /// `(TEMPLATE)\s*+` as group 1, with the last comment as groups 2 to 4:
    /// the arguments of a template call.
    pub(super) fn template_call_innards(&mut self, p: usize) -> Option<usize> {
        let (end, comment) = self.template_args(p)?;
        if let Some(parts) = comment {
            for (i, (a, b)) in parts.iter().enumerate() {
                self.cap(2 + i, *a, *b);
            }
        }
        Some(self.skip_space(end))
    }
}
