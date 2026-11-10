//! Operators, assignments and the small keyword expressions.

use super::super::engine::Lexer;
use crate::classes::is_space;
use crate::rules::Chars;

impl Lexer<'_> {
    /// The first of `texts` found at `p`: the end of it.
    fn one_of_texts(&self, p: usize, texts: &[&[u8]]) -> Option<usize> {
        texts.iter().find_map(|text| self.lit(p, text))
    }

    /// `(?<!\.\s*)` at `p`: whether a dot, and then only blanks, stands
    /// before it.
    pub(in crate::csharp) fn dot_before_blanks(&self, mut p: usize) -> bool {
        while let Some(c) = self.char_before(p) {
            if c == '.' {
                return true;
            }
            if !is_space(c) {
                return false;
            }
            p -= c.len_utf8();
        }
        false
    }

    /// `(?<!\.\s*)\b(await)\b`
    pub(in crate::csharp) fn await_keyword(&mut self, p: usize) -> Option<usize> {
        if self.dot_before_blanks(p) {
            return None;
        }
        let end = self.keyword(p, b"await")?;
        self.capture(1, p, end);
        Some(end)
    }

    /// `\b(word)\s*(\()`, for a keyword that opens a parenthesis.
    pub(in crate::csharp) fn keyword_paren(&mut self, p: usize, word: &[u8]) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        let end = self.lit(p, word)?;
        let paren = self.blanks(end);
        if self.at(paren) != b'(' || paren >= self.lim() {
            return None;
        }
        self.capture(1, p, end);
        self.capture(2, paren, paren + 1);
        Some(paren + 1)
    }

    /// `(?<![!=])(=)(?!=)`
    pub(in crate::csharp) fn assignment(&self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"=")?;
        (!matches!(self.prev(p), Some(b'!' | b'=')) && (self.at(end) != b'=' || end >= self.lim()))
            .then_some(end)
    }

    /// `\?(?!\?|\s*[.\[])`
    pub(in crate::csharp) fn conditional_question(&self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"?")?;
        if self.at(end) == b'?' && end < self.lim() {
            return None;
        }
        let next = self.blanks(end);
        (!self.byte_in(next, b".[")).then_some(end)
    }

    /// `(?:[-%*+/]|\?\?)=`
    pub(in crate::csharp) fn compound_assignment(&self, p: usize) -> Option<usize> {
        let op = self.one_of_texts(p, &[b"-", b"%", b"*", b"+", b"/", b"??"])?;
        self.lit(op, b"=")
    }

    /// `(?:[\&^]|<<|>>>?|\|)=`
    pub(in crate::csharp) fn bitwise_assignment(&self, p: usize) -> Option<usize> {
        let op = self.one_of_texts(p, &[b"&", b"^", b"<<", b">>>", b">>", b"|"])?;
        self.lit(op, b"=")
    }

    /// `(?:[-%*+/]|\?\?|[\&^]|<<|>>>?|\|)?=(?![=>])`
    pub(in crate::csharp) fn assignment_start(&self, p: usize) -> Option<usize> {
        let ops: &[&[u8]] = &[
            b"-", b"%", b"*", b"+", b"/", b"??", b"&", b"^", b"<<", b">>>", b">>", b"|",
        ];
        let not_arrow = |eq_end: usize| !self.byte_in(eq_end, b"=>");
        for op in ops {
            if let Some(eq_end) = self.lit(p, op).and_then(|after| self.lit(after, b"="))
                && not_arrow(eq_end)
            {
                return Some(eq_end);
            }
        }
        let eq_end = self.lit(p, b"=")?;
        not_arrow(eq_end).then_some(eq_end)
    }

    /// `<<|>>>?`
    pub(in crate::csharp) fn shift_operator(&self, p: usize) -> Option<usize> {
        self.lit(p, b"<<")
            .or_else(|| self.lit(p, b">>>"))
            .or_else(|| self.lit(p, b">>"))
    }

    /// `[!=]=`
    pub(in crate::csharp) fn equality_operator(&self, p: usize) -> Option<usize> {
        self.one_of_texts(p, &[b"!=", b"=="])
    }

    /// `<=|>=|[<>]`
    pub(in crate::csharp) fn relational_operator(&self, p: usize) -> Option<usize> {
        self.one_of_texts(p, &[b"<=", b">=", b"<", b">"])
    }

    /// `!|&&|\|\|`
    pub(in crate::csharp) fn logical_operator(&self, p: usize) -> Option<usize> {
        self.one_of_texts(p, &[b"!", b"&&", b"||"])
    }

    /// `[\&^|~]`
    pub(in crate::csharp) fn bitwise_operator(&self, p: usize) -> Option<usize> {
        self.one_of_texts(p, &[b"&", b"^", b"|", b"~"])
    }

    /// `\+|-(?!>)|[%*/]`
    pub(in crate::csharp) fn arithmetic_operator(&self, p: usize) -> Option<usize> {
        if let Some(end) = self.one_of_texts(p, &[b"+", b"%", b"*", b"/"]) {
            return Some(end);
        }
        let end = self.lit(p, b"-")?;
        (!(self.at(end) == b'>' && end < self.lim())).then_some(end)
    }

    /// `_(?![_[:alnum:]])`
    pub(in crate::csharp) fn discard(&self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"_")?;
        (self.alnum_run(end) == end && !self.byte_in(end, b"_")).then_some(end)
    }

    /// `(?=[]\&),:;=?^|}]|!=)`: where a pattern after `is` ends.
    pub(in crate::csharp) fn is_pattern_end(&self, p: usize) -> Option<usize> {
        (self.ahead_in(p, b"]&),:;=?^|}") || self.starts(p, b"!=")).then_some(p)
    }

    /// `\b(and|or|when)\b` at `p`.
    pub(in crate::csharp) fn pattern_word(&self, p: usize) -> bool {
        [&b"and"[..], b"or", b"when"]
            .iter()
            .any(|word| self.keyword(p, word).is_some())
    }

    /// `(?=[]\&),:;=?^|}]|!=|\b(and|or|when)\b)`: where a constant pattern
    /// ends.
    pub(in crate::csharp) fn pattern_end(&self, p: usize) -> Option<usize> {
        (self.ahead_in(p, b"]&),:;=?^|}") || self.starts(p, b"!=") || self.pattern_word(p))
            .then_some(p)
    }

    /// `<=?|>=?`
    pub(in crate::csharp) fn relational_pattern(&self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"<").or_else(|| self.lit(p, b">"))?;
        Some(self.lit(end, b"=").unwrap_or(end))
    }
}
