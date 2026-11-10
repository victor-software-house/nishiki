//! Atoms, keyword keys, underscored names, comments and logical operators.

use super::super::engine::Lexer;
use crate::rules::Chars;

impl Lexer<'_> {
    /// `[@\w]*` from `p`: the rest of an atom's or a key's name.
    fn atom_name_end(&self, mut p: usize) -> usize {
        loop {
            if self.starts(p, b"@") {
                p += 1;
            } else if let Some(width) = self.word_at(p) {
                p += width;
            } else {
                return p;
            }
        }
    }

    /// `(?<!:)(:)(?>…)`: an atom.
    ///
    /// The atomic alternation takes its first alternative that matches: a
    /// name `[A-Z_a-z][@\w]*(?>[!?]|=(?![=>]))?`, then the operators in the
    /// grammar's order.
    /// `\.\.?\.?` comes before `\.\.//`, so the second never matches.
    pub(super) fn atom(&mut self, p: usize) -> Option<usize> {
        if self.prev(p) == Some(b':') {
            return None;
        }
        let start = self.lit(p, b":")?;
        let end = if let Some(name) = self.name_start(start) {
            let name = self.atom_name_end(name);
            match self.at(name) {
                b'!' | b'?' if name < self.lim() => name + 1,
                b'=' if name < self.lim() && !matches!(self.at(name + 1), b'=' | b'>') => name + 1,
                _ => name,
            }
        } else {
            self.first_lit(
                start,
                &[
                    b"<>", b"===", b"==", b"!==", b"!=", b"<<>>", b"<<<", b">>>", b"~~~", b"::",
                    b"<-", b"|>", b"=>", b"=~", b"/", b"=", b"\\\\", b"**", b"*", b"...", b"..",
                    b".", b">=", b">", b"<=", b"<", b"&&&", b"&&", b"&", b"++", b"+", b"--", b"-",
                    b"|||", b"||", b"|", b"!", b"@", b"%{}", b"{}", b"%", b"[]", b"^^^", b"^",
                ],
            )?
        };
        self.capture(1, p, start);
        Some(end)
    }

    /// The end of `[A-Z_a-z]` at `p`.
    fn name_start(&self, p: usize) -> Option<usize> {
        let b = self.at(p);
        (p < self.lim() && (b == b'_' || b.is_ascii_alphabetic())).then_some(p + 1)
    }

    /// `(?>[A-Z_a-z][@\w]*[!?]?)(:)(?!:)`: a key of a keyword list.
    /// The name is atomic, so a `!` or `?` it took stays taken.
    pub(super) fn keyword_key(&mut self, p: usize) -> Option<usize> {
        let name = self.bang_end(self.atom_name_end(self.name_start(p)?));
        if !(self.starts(name, b":") && self.at(name + 1) != b':') {
            return None;
        }
        self.capture(1, name, name + 1);
        Some(name + 1)
    }

    /// `\b_([^_]\w+[!?]?)`: a name that starts with `_`.
    /// `[^_]` takes any character, a space included.
    pub(super) fn underscored(&self, p: usize) -> Option<usize> {
        if self.word_before(p) || !self.starts(p, b"_") {
            return None;
        }
        let (c, rest) = self.char_at(p + 1)?;
        if c == '_' {
            return None;
        }
        let end = self.word_end(rest);
        (end > rest).then(|| self.bang_end(end))
    }

    /// `(^[\t ]+)?(?=mark)`: where a comment starts.
    /// At the start of the line, the blanks before the mark are group 1.
    pub(super) fn comment_start(&mut self, p: usize, mark: &[u8]) -> Option<usize> {
        if p == 0 {
            let blanks = self.skip(0, |b| matches!(b, b' ' | b'\t'));
            if blanks > 0 && self.starts(blanks, mark) {
                self.capture(1, 0, blanks);
                return Some(blanks);
            }
        }
        self.starts(p, mark).then_some(p)
    }

    /// `(?<=[\t ])!+|\bnot\b|&&|\band\b|\|\||\bor\b|\bxor\b`.
    pub(super) fn logical(&self, p: usize) -> Option<usize> {
        if self.starts(p, b"!") && matches!(self.prev(p), Some(b' ' | b'\t')) {
            return Some(self.skip(p, |b| b == b'!'));
        }
        self.word_in(p, &[b"not"])
            .or_else(|| self.lit(p, b"&&"))
            .or_else(|| self.word_in(p, &[b"and"]))
            .or_else(|| self.lit(p, b"||"))
            .or_else(|| self.word_in(p, &[b"or"]))
            .or_else(|| self.word_in(p, &[b"xor"]))
    }
}
