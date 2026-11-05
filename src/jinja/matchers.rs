//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups of its match with [`Lexer::cap`].
//! The engine names those the grammar names.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would.
//! Where a quantifier could give characters back, the doc comment says why it
//! cannot matter, or how the matcher gives them back.
//!
//! The grammar's names, `[A-Z_a-z][0-9A-Z_a-z]*`, are ASCII.
//! `\b`, `\s` and `\h` read as Oniguruma reads them, `\b` and `\s` through
//! [`crate::classes`].

use super::engine::Lexer;
use super::grammar::Pat;
use crate::rules::Chars;

/// The keywords of `Keywords` that are one word, in the grammar's order.
/// Each must be the whole word, so the order does not decide a match.
const KEYWORDS: &[&[u8]] = &[
    b"and",
    b"else",
    b"if",
    b"in",
    b"import",
    b"not",
    b"or",
    b"recursive",
];

impl Lexer<'_> {
    /// Whether `pat` matches at `p`, and where its match ends.
    pub(super) fn matcher(&mut self, pat: Pat, p: usize) -> Option<usize> {
        match pat {
            Pat::Raw => self.raw_tag(p, b"raw"),
            Pat::RawEnd => self.raw_tag(p, b"endraw"),
            Pat::Comments => self.opener(p, b"{#"),
            Pat::CommentsEnd => self.closer(p, b"#}"),
            Pat::Output => self.opener(p, b"{{"),
            Pat::OutputEnd => self.closer(p, b"}}"),
            Pat::Statement => self.opener(p, b"{%"),
            Pat::StatementEnd => self.closer(p, b"%}"),
            Pat::Block => self.keyword_and_name(p, b"block"),
            Pat::Filter => self.keyword_and_name(p, b"filter"),
            Pat::Is => self.keyword_and_name(p, b"is"),
            Pat::TagName => self.tag_name(p),
            Pat::Keywords => self.keyword(p),
            Pat::Constants => self.word_in(p, &[b"true", b"false", b"none"]),
            Pat::Builtins => self.word_in(p, &[b"loop", b"super", b"self", b"varargs", b"kwargs"]),
            Pat::Name => self.name(p),
            Pat::Arithmetic => self.arithmetic(p),
            Pat::FilterCall => self.mark_and_name(p, b"|"),
            Pat::Member => self.mark_and_name(p, b"."),
            Pat::Brackets => self.lit(p, b"["),
            Pat::BracketsEnd => self.lit(p, b"]"),
            Pat::Parens => self.lit(p, b"("),
            Pat::ParensEnd => self.lit(p, b")"),
            Pat::Braces => self.lit(p, b"{"),
            Pat::BracesEnd => self.lit(p, b"}"),
            Pat::Punctuation => self.punctuation(p),
            Pat::Comparison => self.comparison(p),
            Pat::Assignment => self.lit(p, b"="),
            Pat::DoubleString | Pat::DoubleStringEnd => self.lit(p, b"\""),
            Pat::SingleString | Pat::SingleStringEnd => self.lit(p, b"'"),
            Pat::Regex => self.lit(p, b"@/"),
            Pat::RegexEnd => self.lit(p, b"/"),
            Pat::SimpleEscapes => self.simple_escape(p),
            Pat::EscapedChar => self.hex_escape(p),
            Pat::EscapedUnicodeChar => self.unicode_escape(p),
        }
    }

    /// `\{#-?`, `\{\{-?` or `\{%-?`: an opening delimiter, with the `-` that
    /// trims the blanks before it.
    fn opener(&self, p: usize, text: &[u8]) -> Option<usize> {
        let end = self.lit(p, text)?;
        Some(if self.starts(end, b"-") { end + 1 } else { end })
    }

    /// `-?#}`, `-?}}` or `-?%}`: a closing delimiter.
    /// A `-` that the delimiter does not follow gives way, and the delimiter
    /// then fails at the `-`.
    fn closer(&self, p: usize, text: &[u8]) -> Option<usize> {
        let start = if self.starts(p, b"-") { p + 1 } else { p };
        self.lit(start, text)
    }

    /// `(\{%)\s*(word)\s*(%})`: the tag that opens or closes a raw block.
    /// The blanks can take the line feed, and the word then fails after it.
    fn raw_tag(&mut self, p: usize, word: &[u8]) -> Option<usize> {
        let open = self.lit(p, b"{%")?;
        let word_start = self.space_end(open);
        let word_end = self.lit(word_start, word)?;
        let close = self.space_end(word_end);
        let end = self.lit(close, b"%}")?;
        self.cap(1, p, open);
        self.cap(2, word_start, word_end);
        self.cap(3, close, end);
        Some(end)
    }

    /// The end of `[A-Z_a-z][0-9A-Z_a-z]*` at `p`.
    fn name(&self, p: usize) -> Option<usize> {
        let b = self.at(p);
        (p < self.lim() && (b == b'_' || b.is_ascii_alphabetic()))
            .then(|| self.skip(p + 1, |b| b == b'_' || b.is_ascii_alphanumeric()))
    }

    /// The end of `[A-Z_a-z][0-9A-Z_a-z]*\b` at `p`.
    /// The run gives back only to a place inside it, where `\b` fails, so a
    /// `\w` past ASCII after it fails the name.
    fn bounded_name(&self, p: usize) -> Option<usize> {
        let end = self.name(p)?;
        self.word_at(end).is_none().then_some(end)
    }

    /// `\s*\b(keyword)\s+([A-Z_a-z][0-9A-Z_a-z]*)\b`: `block`, `filter` or
    /// `is` and the name after it.
    /// The blanks before give nothing back, since the keyword starts with a
    /// letter; those after give nothing back, since the name does too.
    fn keyword_and_name(&mut self, p: usize, keyword: &[u8]) -> Option<usize> {
        let start = self.space_end(p);
        if self.word_before(start) {
            return None;
        }
        let keyword_end = self.lit(start, keyword)?;
        let name = self.space_end(keyword_end);
        if name == keyword_end {
            return None;
        }
        let end = self.bounded_name(name)?;
        self.cap(1, start, keyword_end);
        self.cap(2, name, end);
        Some(end)
    }

    /// `(?<=\{%-?)\s*\b([A-Z_a-z][0-9A-Z_a-z]*)\b(?!\s*[,=])`: the name that
    /// starts a statement tag.
    ///
    /// The lookbehind has two lengths: `{%` or `{%-` must end at `p`.
    /// Oniguruma reads such a lookbehind as its two branches.
    pub(super) fn tag_name(&mut self, p: usize) -> Option<usize> {
        let before = |width: usize, text: &[u8]| {
            p.checked_sub(width)
                .is_some_and(|start| self.text(start, p) == text)
        };
        if !(before(2, b"{%") || before(3, b"{%-")) {
            return None;
        }
        let name = self.space_end(p);
        let end = self.bounded_name(name)?;
        let next = self.space_end(end);
        if self.starts(next, b",") || self.starts(next, b"=") {
            return None;
        }
        self.cap(1, name, end);
        Some(end)
    }

    /// `\b(and|else|…|recursive|with(out)?\s+context)\b`: a keyword.
    /// `(out)?` without `\s+context` after it gives way, and `\s+` then
    /// fails at the `o`.
    fn keyword(&self, p: usize) -> Option<usize> {
        if let Some(end) = self.word_in(p, KEYWORDS) {
            return Some(end);
        }
        if self.word_before(p) {
            return None;
        }
        let with = self.lit(p, b"with")?;
        let blanks = self.lit(with, b"out").unwrap_or(with);
        let context = self.space_end(blanks);
        if context == blanks {
            return None;
        }
        let end = self.lit(context, b"context")?;
        self.word_at(end).is_none().then_some(end)
    }

    /// `([-+]|\*\*?|//|[%/])`: the first alternative that matches.
    fn arithmetic(&mut self, p: usize) -> Option<usize> {
        let end = match self.at(p) {
            b'-' | b'+' | b'%' if p < self.lim() => p + 1,
            b'*' if p < self.lim() => {
                if self.starts(p + 1, b"*") {
                    p + 2
                } else {
                    p + 1
                }
            }
            b'/' if p < self.lim() => {
                if self.starts(p + 1, b"/") {
                    p + 2
                } else {
                    p + 1
                }
            }
            _ => return None,
        };
        self.cap(1, p, end);
        Some(end)
    }

    /// `(mark)([A-Z_a-z][0-9A-Z_a-z]*)`: a filter after `|` or a member after
    /// `.`.
    fn mark_and_name(&mut self, p: usize, mark: &[u8]) -> Option<usize> {
        let name = self.lit(p, mark)?;
        let end = self.name(name)?;
        self.cap(1, p, name);
        self.cap(2, name, end);
        Some(end)
    }

    /// `([,.:|])`.
    fn punctuation(&mut self, p: usize) -> Option<usize> {
        if !(p < self.lim() && matches!(self.at(p), b',' | b'.' | b':' | b'|')) {
            return None;
        }
        self.cap(1, p, p + 1);
        Some(p + 1)
    }

    /// `(==|<=|=>|[<>]|!=)`: the first alternative that matches.
    fn comparison(&mut self, p: usize) -> Option<usize> {
        let end = self
            .lit(p, b"==")
            .or_else(|| self.lit(p, b"<="))
            .or_else(|| self.lit(p, b"=>"))
            .or_else(|| (p < self.lim() && matches!(self.at(p), b'<' | b'>')).then_some(p + 1))
            .or_else(|| self.lit(p, b"!="))?;
        self.cap(1, p, end);
        Some(end)
    }

    /// `(\\\n)|(\\\\)|(\\")|(\\')|(\\a)|(\\b)|(\\f)|(\\n)|(\\r)|(\\t)|(\\v)`:
    /// an escape, each in a group of its own.
    /// The first takes the line feed that ends the line.
    fn simple_escape(&mut self, p: usize) -> Option<usize> {
        let after = self.lit(p, b"\\")?;
        if !self.avail(after) {
            return None;
        }
        let group = match self.at(after) {
            b'\n' if after == self.lim() => 1,
            b'\\' => 2,
            b'"' => 3,
            b'\'' => 4,
            b'a' => 5,
            b'b' => 6,
            b'f' => 7,
            b'n' => 8,
            b'r' => 9,
            b't' => 10,
            b'v' => 11,
            _ => return None,
        };
        self.cap(group, p, after + 1);
        Some(after + 1)
    }

    /// `\\x[0-9A-F]{2}`: two upper-case hexadecimal digits.
    fn hex_escape(&self, p: usize) -> Option<usize> {
        let digits = self.lit(p, b"\\x")?;
        let end = digits + 2;
        (end <= self.lim()
            && self
                .text(digits, end)
                .iter()
                .all(|&b| b.is_ascii_digit() || (b'A'..=b'F').contains(&b)))
        .then_some(end)
    }

    /// `(\\U\h{8})|(\\u\h{4})|(\\N\{[ A-Za-z]+})`: a Unicode escape.
    /// `[ A-Za-z]+` stops before the `}`, which it does not take.
    fn unicode_escape(&mut self, p: usize) -> Option<usize> {
        let hex = |start: usize, count: usize| {
            let end = start + count;
            (end <= self.lim() && self.text(start, end).iter().all(u8::is_ascii_hexdigit))
                .then_some(end)
        };
        if let Some(end) = self.lit(p, b"\\U").and_then(|start| hex(start, 8)) {
            self.cap(1, p, end);
            return Some(end);
        }
        if let Some(end) = self.lit(p, b"\\u").and_then(|start| hex(start, 4)) {
            self.cap(2, p, end);
            return Some(end);
        }
        let name = self.lit(p, b"\\N{")?;
        let close = self.skip(name, |b| b == b' ' || b.is_ascii_alphabetic());
        if close == name {
            return None;
        }
        let end = self.lit(close, b"}")?;
        self.cap(3, p, end);
        Some(end)
    }
}
