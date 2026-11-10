//! Comments, strings, heredocs and the interpolations inside them.

use super::lexer::{Feed, Lexer, Tail};
use super::scope::S;
use super::state::Rule;
use crate::classes::space_width;

impl Lexer<'_> {
    /// A comment at `p`: `#` or `//` to the end of the line, or `/*`.
    ///
    /// A line comment's end, `$\n?`, takes the line feed.
    /// An inline `if` that the comment sits in then stays open.
    pub(super) fn comment(&mut self, p: usize) -> Option<usize> {
        let (marker, leaf) = match (self.s[p], self.at(p + 1)) {
            (b'#', _) => (1, S::CommentHash),
            (b'/', b'/') => (2, S::CommentSlash),
            (b'/', b'*') => return Some(self.begin(p, p + 2, Rule::BlockComment, S::PunctComment)),
            _ => return None,
        };
        let len = self.s.len();
        self.tok_at(p, p + marker, &[leaf, S::PunctComment]);
        self.tok(len, &[leaf]);
        if self.feed == Feed::Ahead {
            self.feed = Feed::Taken;
        }
        Some(len)
    }

    /// Inside `/* … */`.
    pub(super) fn in_block_comment(&mut self, p: usize) -> usize {
        if self.s[p..].starts_with(b"*/") {
            return self.finish(p, p + 2, S::PunctComment);
        }
        let next = memchr::memchr(b'*', &self.s[p + 1..]).map_or(self.s.len(), |k| p + 1 + k);
        self.steps.spend(next - p);
        next
    }

    /// `"` at `p`.
    pub(super) fn string(&mut self, p: usize) -> usize {
        self.begin(p, p + 1, Rule::String, S::StrBegin)
    }

    /// Inside a quoted string: its `"`, interpolations and escapes.
    pub(super) fn in_string(&mut self, p: usize) -> usize {
        match self.s[p] {
            b'"' => return self.finish(p, p + 1, S::StrEnd),
            b'\\' => return self.escape(p).unwrap_or(p + 1),
            b'$' | b'%' => return self.interpolation(p).unwrap_or(p + 1),
            _ => {}
        }
        let next = self.s[p..]
            .iter()
            .position(|&b| matches!(b, b'"' | b'\\' | b'$' | b'%'))
            .map_or(self.s.len(), |k| p + k);
        self.steps.spend(next - p);
        next
    }

    /// `\\(?:["\\nrt]|u(\h{8}|\h{4}))` at `p`.
    fn escape(&mut self, p: usize) -> Option<usize> {
        let hex = |from: usize, n: usize| {
            self.s
                .get(from..from + n)
                .is_some_and(|digits| digits.iter().all(u8::is_ascii_hexdigit))
        };
        let end = match self.at(p + 1) {
            b'"' | b'\\' | b'n' | b'r' | b't' => p + 2,
            b'u' if hex(p + 2, 8) => p + 10,
            b'u' if hex(p + 2, 4) => p + 6,
            _ => return None,
        };
        self.tok_at(p, end, &[S::Escape]);
        Some(end)
    }

    /// `(?<![$%])([$%]\{)` at `p`: an interpolation or a template directive.
    fn interpolation(&mut self, p: usize) -> Option<usize> {
        let after_marker = p > 0 && matches!(self.s[p - 1], b'$' | b'%');
        if self.at(p + 1) != b'{' || after_marker {
            return None;
        }
        Some(self.begin(p, p + 2, Rule::Interpolation, S::InterpolationBegin))
    }

    /// `(<<-?)\s*(\w+)\s*$` at `p`: a heredoc, whose label ends the line.
    pub(super) fn heredoc(&mut self, p: usize) -> Option<usize> {
        if self.at(p + 1) != b'<' {
            return None;
        }
        let operator_end = p + 2 + usize::from(self.at(p + 2) == b'-');
        let label = self.skip_space(operator_end);
        let label_end = self.word_end(label);
        if label_end == label || self.skip_space(label_end) != self.s.len() {
            return None;
        }
        let text = std::str::from_utf8(&self.s[label..label_end]).ok()?;
        let Some(id) = self.em.arena.intern(text) else {
            self.em.report.record(crate::limits::Limit::Nodes);
            return None;
        };
        self.gap(p);
        if self.push(Rule::Heredoc, id) {
            self.tok(operator_end, &[S::HeredocOperator]);
            self.tok_at(label, label_end, &[S::HeredocLabel]);
        }
        Some(label_end)
    }

    /// Inside a heredoc: the line `^\s*LABEL\s*$` that ends it, then
    /// interpolations.
    pub(super) fn in_heredoc(&mut self, p: usize) -> usize {
        let len = self.s.len();
        if p == 0 {
            let label = self.skip_space(0);
            let ends = {
                let text = self.em.arena.string(self.em.frame.carried).as_bytes();
                !text.is_empty()
                    && self.s[label..].starts_with(text)
                    && self.skip_space(label + text.len()) == len
            };
            if ends {
                self.tok(len, &[S::HeredocLabel]);
                self.em.pop();
                return len;
            }
        }
        if matches!(self.s[p], b'$' | b'%') {
            return self.interpolation(p).unwrap_or(p + 1);
        }
        let next = memchr::memchr2(b'$', b'%', &self.s[p..]).map_or(len, |k| p + k);
        self.steps.spend(next - p);
        next
    }

    /// Inside an interpolation: its `}`, the trim markers `~\s` and `\s~`,
    /// template keywords, expressions and names.
    pub(super) fn in_interpolation(&mut self, p: usize) -> usize {
        let len = self.s.len();
        match self.s[p] {
            b'}' => return self.finish(p, p + 1, S::InterpolationEnd),
            b'~' => {
                let space = space_width(self.s, p + 1);
                let feed = p + 1 == len && self.feed == Feed::Ahead;
                if space > 0 || feed {
                    let end = p + 1 + space;
                    self.tok_at(p, end, &[S::TrimLeft]);
                    return end;
                }
            }
            _ => {
                let space = space_width(self.s, p);
                if space > 0 && self.at(p + space) == b'~' {
                    self.tok_at(p, p + space + 1, &[S::TrimRight]);
                    return p + space + 1;
                }
            }
        }
        if let Some(word) = self.whole_word(p)
            && matches!(
                word,
                b"if" | b"else" | b"endif" | b"for" | b"in" | b"endfor"
            )
        {
            self.tok_at(p, p + word.len(), &[S::Control]);
            return p + word.len();
        }
        if let Some(end) = self.expression(p) {
            return end;
        }
        if let Some(end) = self.local(p) {
            return end;
        }
        self.skip(p, Tail::Plain)
    }
}
