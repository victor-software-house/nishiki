//! The matchers of f-strings: their prefixes, braces, conversions and format
//! specs, and the text between.

use super::Quotes;
use crate::python::engine::{Ahead, Lexer};
use crate::python::grammar::FstringPat;
use crate::rules::Chars;

impl Lexer<'_> {
    pub(super) fn fstring_matcher(&mut self, pat: FstringPat, p: usize) -> Option<usize> {
        match pat {
            FstringPat::FstringFnormQuotedMultiLine => self.fnorm_begin(p, Quotes::Three),
            FstringPat::FstringFnormQuotedSingleLine => self.fnorm_begin(p, Quotes::One),
            FstringPat::FstringNormfQuotedMultiLine => self.normf_begin(p, Quotes::Three),
            FstringPat::FstringNormfQuotedSingleLine => self.normf_begin(p, Quotes::One),
            FstringPat::FstringRawQuotedMultiLine => self.fraw_begin(p, Quotes::Three),
            FstringPat::FstringRawQuotedSingleLine => self.fraw_begin(p, Quotes::One),
            FstringPat::FstringFnormQuotedMultiLineEnd
            | FstringPat::FstringNormfQuotedMultiLineEnd
            | FstringPat::FstringRawQuotedMultiLineEnd => self.string_end_multi(p),
            FstringPat::FstringFnormQuotedSingleLineEnd
            | FstringPat::FstringNormfQuotedSingleLineEnd
            | FstringPat::FstringRawQuotedSingleLineEnd => self.string_end_single(p),
            FstringPat::FstringFormattingBracesAt0 => self.empty_placeholder(p),
            FstringPat::FstringFormattingBracesAt1 => {
                (self.starts(p, b"{{") || self.starts(p, b"}}")).then_some(p + 2)
            }
            FstringPat::FstringFormattingSingeBrace => (self.at(p) == b'}'
                && p < self.lim()
                && !(self.at(p + 1) == b'}' && p + 1 < self.lim()))
            .then_some(p + 1),
            FstringPat::FstringMultiBrace | FstringPat::FstringSingleBrace => self.brace_cap(p),
            FstringPat::FstringMultiBraceEnd => self.brace_close(p),
            FstringPat::FstringSingleBraceEnd | FstringPat::FstringIllegalSingleBraceEnd => {
                self.brace_close_or_feed(p)
            }
            FstringPat::FstringIllegalSingleBrace => self.illegal_single_brace(p),
            FstringPat::FstringTerminatorMultiAt0 | FstringPat::FstringTerminatorSingleAt0 => {
                self.terminator_equals(p)
            }
            FstringPat::FstringTerminatorMultiAt1 | FstringPat::FstringTerminatorSingleAt1 => {
                self.terminator_conversion(p)
            }
            FstringPat::FstringTerminatorMultiAt2 | FstringPat::FstringTerminatorSingleAt2 => {
                self.terminator_spec(p)
            }
            FstringPat::FstringTerminatorMultiTail | FstringPat::FstringTerminatorSingleTail => {
                self.tail_begin(p)
            }
            FstringPat::FstringTerminatorMultiTailEnd => {
                (self.at(p) == b'}' && p < self.lim()).then_some(p)
            }
            FstringPat::FstringTerminatorSingleTailEnd => {
                (self.at(p) == b'}' && p < self.lim() || p == self.lim()).then_some(p)
            }
            FstringPat::FstringTerminatorMultiTailAt2
            | FstringPat::FstringTerminatorSingleTailAt2 => (p < self.lim()
                && b"%EFGXbcdefgnosx".contains(&self.at(p))
                && self.at(p + 1) == b'}'
                && p + 1 < self.lim())
            .then_some(p + 1),
            FstringPat::FstringTerminatorMultiTailAt3
            | FstringPat::FstringTerminatorSingleTailAt3 => {
                if self.at(p) != b'.' || p >= self.lim() {
                    return None;
                }
                let e = self.digit_end(p + 1);
                (e > p + 1).then_some(e)
            }
            FstringPat::FstringTerminatorMultiTailAt4
            | FstringPat::FstringTerminatorSingleTailAt4 => {
                (self.at(p) == b',' && p < self.lim()).then_some(p + 1)
            }
            FstringPat::FstringTerminatorMultiTailAt5
            | FstringPat::FstringTerminatorSingleTailAt5 => {
                let e = self.digit_end(p);
                (e > p).then_some(e)
            }
            FstringPat::FstringTerminatorMultiTailAt6
            | FstringPat::FstringTerminatorSingleTailAt6 => {
                (self.at(p) == b'#' && p < self.lim()).then_some(p + 1)
            }
            FstringPat::FstringTerminatorMultiTailAt7
            | FstringPat::FstringTerminatorSingleTailAt7 => {
                (matches!(self.at(p), b'-' | b' ' | b'+') && p < self.lim()).then_some(p + 1)
            }
            FstringPat::FstringTerminatorMultiTailAt8
            | FstringPat::FstringTerminatorSingleTailAt8 => {
                (matches!(self.at(p), b'<' | b'=' | b'>' | b'^') && p < self.lim()).then_some(p + 1)
            }
            FstringPat::FstringTerminatorMultiTailAt9
            | FstringPat::FstringTerminatorSingleTailAt9 => self.word_at(p).map(|w| p + w),
            FstringPat::FstringMultiCore | FstringPat::FstringRawMultiCore => {
                self.core(p, Quotes::Three)
            }
            FstringPat::FstringSingleCore | FstringPat::FstringRawSingleCore => {
                self.core(p, Quotes::One)
            }
            FstringPat::Impossible => None,
        }
    }

    /// The end `(\3)` of a triple-quoted f-string.
    fn string_end_multi(&mut self, p: usize) -> Option<usize> {
        let width = self.payload_at(p)?;
        self.cap(1, p, p + width);
        Some(p + width)
    }

    /// The end `(\3)|((?<!\\)\n)` of a single-line f-string.
    fn string_end_single(&mut self, p: usize) -> Option<usize> {
        if let Some(width) = self.payload_at(p) {
            self.cap(1, p, p + width);
            return Some(p + width);
        }
        if p == self.lim() && !self.prev_is(p, b'\\') {
            self.cap(2, p, p + 1);
            return Some(p + 1);
        }
        None
    }

    /// The width of the quote at `p`, as [`Lexer::quote_width`].
    fn fquote(&self, p: usize, quotes: Quotes) -> Option<usize> {
        if quotes == Quotes::Three {
            (self.starts(p, b"'''") || self.starts(p, b"\"\"\"")).then_some(3)
        } else {
            (matches!(self.at(p), b'"' | b'\'') && p < self.lim()).then_some(1)
        }
    }

    /// The begin `\b([Ff])([BUbu])?('''|""")` or its single-line form.
    fn fnorm_begin(&mut self, p: usize, quotes: Quotes) -> Option<usize> {
        if !self.boundary(p) || !matches!(self.at(p), b'F' | b'f') || p >= self.lim() {
            return None;
        }
        let mut q = p + 1;
        let mut second = None;
        if matches!(self.at(q), b'B' | b'U' | b'b' | b'u')
            && q < self.lim()
            && self.fquote(q + 1, quotes).is_some()
        {
            second = Some(q);
            q += 1;
        }
        let width = self.fquote(q, quotes)?;
        self.cap(1, p, p + 1);
        if let Some(s) = second {
            self.cap(2, s, s + 1);
        }
        self.cap(3, q, q + width);
        if quotes == Quotes::One {
            self.cap(4, q, q + width);
        }
        Some(q + width)
    }

    /// The begin `\b([BUbu])([Ff])('''|""")` or its single-line form.
    fn normf_begin(&mut self, p: usize, quotes: Quotes) -> Option<usize> {
        if !self.boundary(p)
            || !matches!(self.at(p), b'B' | b'U' | b'b' | b'u')
            || !matches!(self.at(p + 1), b'F' | b'f')
            || p + 1 >= self.lim()
        {
            return None;
        }
        let width = self.fquote(p + 2, quotes)?;
        self.cap(1, p, p + 1);
        self.cap(2, p + 1, p + 2);
        self.cap(3, p + 2, p + 2 + width);
        if quotes == Quotes::One {
            self.cap(4, p + 2, p + 2 + width);
        }
        Some(p + 2 + width)
    }

    /// The begin `\b([Rr][Ff]|[Ff][Rr])('''|""")` or its single-line form.
    fn fraw_begin(&mut self, p: usize, quotes: Quotes) -> Option<usize> {
        let (a, b) = (self.at(p), self.at(p + 1));
        let ok = (matches!(a, b'R' | b'r') && matches!(b, b'F' | b'f'))
            || (matches!(a, b'F' | b'f') && matches!(b, b'R' | b'r'));
        if !self.boundary(p) || !ok || p + 1 >= self.lim() {
            return None;
        }
        let width = self.fquote(p + 2, quotes)?;
        self.cap(1, p, p + 2);
        self.cap(2, p + 2, p + 2 + width);
        if quotes == Quotes::One {
            self.cap(3, p + 2, p + 2 + width);
        }
        Some(p + 2 + width)
    }

    /// The match `(\{)(\s*?)(})`.
    fn empty_placeholder(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'{' || p >= self.lim() {
            return None;
        }
        let mut q = p + 1;
        loop {
            if self.at(q) == b'}' && q < self.lim() {
                self.cap(1, p, p + 1);
                self.cap(2, p + 1, q);
                self.cap(3, q, q + 1);
                return Some(q + 1);
            }
            let width = self.space_at(q);
            if width == 0 {
                return None;
            }
            q += width;
        }
    }

    /// The begin `(\{)`.
    fn brace_cap(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'{' || p >= self.lim() {
            return None;
        }
        self.cap(1, p, p + 1);
        Some(p + 1)
    }

    /// The end `(})`.
    fn brace_close(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'}' || p >= self.lim() {
            return None;
        }
        self.cap(1, p, p + 1);
        Some(p + 1)
    }

    /// The end `(})|(?=\n)`.
    fn brace_close_or_feed(&mut self, p: usize) -> Option<usize> {
        if let Some(end) = self.brace_close(p) {
            return Some(end);
        }
        (p == self.lim()).then_some(p)
    }

    /// The begin `(\{)(?=[^\n}]*$\n?)`: a brace with no `}` after it on the
    /// line.
    fn illegal_single_brace(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'{' || p >= self.lim() {
            return None;
        }
        if self.last(Ahead::CloseBrace).is_some_and(|last| last > p) {
            return None;
        }
        self.cap(1, p, p + 1);
        Some(p + 1)
    }

    /// `![ars]` at `i`: where it ends.
    fn conversion_end(&self, i: usize) -> Option<usize> {
        (self.at(i) == b'!' && matches!(self.at(i + 1), b'a' | b'r' | b's') && i + 1 < self.lim())
            .then_some(i + 2)
    }

    /// The match `(=(![ars])?)(?=})`.
    fn terminator_equals(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'=' || p >= self.lim() {
            return None;
        }
        let close = |at: usize| self.at(at) == b'}' && at < self.lim();
        if let Some(e) = self.conversion_end(p + 1)
            && close(e)
        {
            return Some(e);
        }
        close(p + 1).then_some(p + 1)
    }

    /// The match `(=?![ars])(?=})`.
    fn terminator_conversion(&self, p: usize) -> Option<usize> {
        let start = if self.at(p) == b'=' && p < self.lim() {
            p + 1
        } else {
            p
        };
        let e = self.conversion_end(start)?;
        (self.at(e) == b'}' && e < self.lim()).then_some(e)
    }

    /// `=?(?:![ars])?` at `p`: where it ends.
    fn terminator_head(&self, p: usize) -> usize {
        let mut i = p;
        if self.at(i) == b'=' && i < self.lim() {
            i += 1;
        }
        self.conversion_end(i).unwrap_or(i)
    }

    /// The match `(=?(?:![ars])?)(:spec)(?=})`.
    fn terminator_spec(&mut self, p: usize) -> Option<usize> {
        let head = self.terminator_head(p);
        if self.at(head) != b':' || head >= self.lim() {
            return None;
        }
        let close = self.format_spec(head + 1)?;
        self.cap(1, p, head);
        self.cap(2, head, close);
        Some(close)
    }

    /// The begin `(=?(?:![ars])?)(:)(?=.*?\{)`.
    fn tail_begin(&mut self, p: usize) -> Option<usize> {
        let head = self.terminator_head(p);
        if self.at(head) != b':' || head >= self.lim() {
            return None;
        }
        if self.last(Ahead::OpenBrace)? < head + 1 {
            return None;
        }
        self.cap(1, p, head);
        self.cap(2, head, head + 1);
        Some(head + 1)
    }

    /// The match `(.+?)($(\n?)|(?=[\\{}]|'''|"""))|\n`, or for a single-line
    /// f-string `(.+?)($(\n?)|(?=[\\{}]|(["'])|((?<!\\)\n)))|\n`.
    fn core(&self, p: usize, quotes: Quotes) -> Option<usize> {
        if p == self.lim() {
            // `.+?` needs a character: the line feed alone is the match.
            return Some(p + 1);
        }
        if p > self.lim() {
            return None;
        }
        // At least one character, then the first end that fits.
        let mut e = self.char_end(p);
        loop {
            self.spend(1);
            if e >= self.lim() {
                // `$(\n?)`.
                return Some(self.lim() + 1);
            }
            let b = self.at(e);
            let special = matches!(b, b'\\' | b'{' | b'}')
                || if quotes == Quotes::Three {
                    self.starts(e, b"'''") || self.starts(e, b"\"\"\"")
                } else {
                    matches!(b, b'"' | b'\'')
                };
            if special {
                return Some(e);
            }
            e = self.char_end(e);
        }
    }
}
