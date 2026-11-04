//! Regular expression literals written `/…/` and the parts the grammar names
//! inside them.

use super::super::engine::Lexer;
use crate::rules::Chars;

/// The words that may follow a regular expression's closing `/`.
/// The grammar tests them as prefixes, with no `\b` after them.
const FOLLOWING_WORDS: &[&[u8]] = &[
    b"if", b"else", b"elsif", b"then", b"do", b"end", b"unless", b"while", b"until", b"or", b"and",
];

/// The operators that may follow a regular expression's closing `/`.
const FOLLOWING_OPERATORS: &[&[u8]] = &[b"||", b"&&", b"<=>", b"=>", b"==", b"=~", b"!~", b"!="];

impl Lexer<'_> {
    /// The `/` that opens a regular expression, when the line has its close:
    ///
    /// ```text
    /// (?<![)\w])((/))(?![*+?])(?!\s*$)(?=(?:\\/|[^/])*+/[eimnosux]*\s*(?:…))
    /// ```
    ///
    /// The possessive body takes `\/` whole and any other character alone.
    /// It therefore stops at the first `/` that no backslash right before it
    /// escapes, counting a backslash that a backslash escapes.
    /// Of the flags, each end from the longest is tried.
    /// The blanks after them cannot end early.
    /// No alternative after them starts with a blank, and `$` holds at the end
    /// of the blanks whenever it holds within them.
    pub(in crate::ruby) fn regexp_start(&mut self, p: usize) -> Option<usize> {
        let refused_before = self.word_before(p) || self.prev_is(p, b')');
        if refused_before || !self.byte_in(p, |b| b == b'/') {
            return None;
        }
        let body = p + 1;
        if self.byte_in(body, |b| matches!(b, b'*' | b'+' | b'?'))
            || self.space_end(body) >= self.lim()
        {
            return None;
        }
        let close = self.regexp_body_end(body)?;
        let flags = close + 1;
        let longest = self.skip(flags, |b| {
            matches!(b, b'e' | b'i' | b'm' | b'n' | b'o' | b's' | b'u' | b'x')
        });
        let follows = (flags..=longest)
            .rev()
            .any(|end| self.follows_regexp(self.space_end(end)));
        follows.then(|| {
            self.cap(1, p, body);
            self.cap(2, p, body);
            body
        })
    }

    /// Where `(?:\\/|[^/])*+` stops from `p`, at a `/`.
    ///
    /// Every `/` the scan passes is the second character of a `\/` it took
    /// whole.
    /// The scan from just after it therefore starts where this one has a
    /// boundary, and stops at the same `/` or at none.
    /// The last scan is kept, and a line of escaped slashes is scanned once.
    fn regexp_body_end(&self, p: usize) -> Option<usize> {
        let text_end = self.lim() * 2 + usize::from(self.nl);
        let (end, from, close) = self.x.regexp_close.get();
        if end == text_end && from <= p && close.is_none_or(|close| p <= close) {
            self.spend(1);
            return close;
        }
        let mut at = p;
        let close = loop {
            if at >= self.lim() {
                break None;
            }
            match self.at(at) {
                b'/' => break Some(at),
                b'\\' if self.byte_in(at + 1, |b| b == b'/') => at += 2,
                _ => at = self.char_end(at),
            }
        };
        self.spend(at - p + 1);
        self.x.regexp_close.set((text_end, p, close));
        close
    }

    /// `(?:[]#),.:?}]|\|\||&&|<=>|=>|==|=~|!~|!=|;|$|if|…|and)` at `p`.
    fn follows_regexp(&self, p: usize) -> bool {
        self.byte_in(p, |b| {
            matches!(
                b,
                b']' | b'#' | b')' | b',' | b'.' | b':' | b'?' | b'}' | b';'
            )
        }) || self.at_eol(p)
            || FOLLOWING_OPERATORS.iter().any(|op| self.starts(p, op))
            || FOLLOWING_WORDS.iter().any(|word| self.starts(p, word))
    }

    /// `((/[eimnosux]*))`
    pub(in crate::ruby) fn regexp_end(&mut self, p: usize) -> Option<usize> {
        let end = self.close_with_flags(p, b'/')?;
        self.cap(1, p, end);
        self.cap(2, p, end);
        Some(end)
    }

    /// `(\{)\d+(,\d+)?(})`: a repetition count.
    pub(in crate::ruby) fn regexp_repetition(&mut self, p: usize) -> Option<usize> {
        if !self.byte_in(p, |b| b == b'{') {
            return None;
        }
        let digits = |at: usize| {
            let end = self.digit_end(at);
            (end > at).then_some(end)
        };
        let low = digits(p + 1)?;
        let high = if self.byte_in(low, |b| b == b',') {
            digits(low + 1).unwrap_or(low)
        } else {
            low
        };
        self.byte_in(high, |b| b == b'}').then(|| {
            self.cap(1, p, p + 1);
            self.cap(3, high, high + 1);
            high + 1
        })
    }

    /// `\[(?:\^?])?`: the start of a character class.
    pub(in crate::ruby) fn regexp_class_start(&mut self, p: usize) -> Option<usize> {
        let open = self.lit(p, b"[")?;
        let caret = open + usize::from(self.byte_in(open, |b| b == b'^'));
        if self.byte_in(caret, |b| b == b']') {
            Some(caret + 1)
        } else {
            Some(open)
        }
    }

    /// `(?<=^|\s)(#)\s(?=[-\t !,.0-9?A-Za-z[^\x00-\x7F]]*$)`: a comment in an
    /// extended regular expression.
    pub(in crate::ruby) fn regexp_comment(&mut self, p: usize) -> Option<usize> {
        if !self.after_blank_or_start(p) || !self.byte_in(p, |b| b == b'#') {
            return None;
        }
        let blank = self.space_at(p + 1);
        if blank == 0 {
            return None;
        }
        let rest = p + 1 + blank;
        let plain = |b: u8| {
            b >= 0x80
                || b.is_ascii_alphanumeric()
                || matches!(b, b'-' | b'\t' | b' ' | b'!' | b',' | b'.' | b'?')
        };
        (rest > self.lim() || self.skip(rest, plain) == self.lim()).then(|| {
            self.cap(1, p, p + 1);
            rest
        })
    }

    /// `$\n?`
    pub(in crate::ruby) fn line_end_and_feed(&mut self, p: usize) -> Option<usize> {
        self.at_eol(p).then(|| self.newline_end(p))
    }

    /// `\n`
    pub(in crate::ruby) fn line_feed(&mut self, p: usize) -> Option<usize> {
        (p == self.lim() && self.nl).then_some(p + 1)
    }

    /// The end of `\n?` at `p`.
    fn newline_end(&self, p: usize) -> usize {
        p + usize::from(p == self.lim() && self.nl)
    }
}
