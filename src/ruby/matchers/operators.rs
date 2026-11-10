//! Operators and punctuation.

use super::super::engine::Lexer;
use crate::rules::Chars;

/// The assignment operators of
/// `<<=|%=|&{1,2}=|\*=|\*\*=|\+=|-=|\^=|\|{1,2}=|<<`, in the order the
/// alternation tries them.
const ASSIGNMENTS: &[&[u8]] = &[
    b"<<=", b"%=", b"&&=", b"&=", b"*=", b"**=", b"+=", b"-=", b"^=", b"||=", b"|=", b"<<",
];

impl Lexer<'_> {
    /// The end of the first of `options` the line has at `p`.
    fn first_of(&self, p: usize, options: &[&[u8]]) -> Option<usize> {
        options.iter().find_map(|option| self.lit(p, option))
    }

    /// `<<=|%=|&{1,2}=|\*=|\*\*=|\+=|-=|\^=|\|{1,2}=|<<`
    pub(in crate::ruby) fn assignment_operator(&mut self, p: usize) -> Option<usize> {
        self.first_of(p, ASSIGNMENTS)
    }

    /// `<=>|<(?![<=])|>(?![<=>])|<=|>=|===?|=~|!=|!~|(?<=[\t ])\?`
    pub(in crate::ruby) fn comparison(&mut self, p: usize) -> Option<usize> {
        if self.starts(p, b"<=>") {
            return Some(p + 3);
        }
        let next = p + 1;
        match self.byte_in(p, |_| true).then(|| self.at(p)) {
            Some(b'<') if !self.byte_in(next, |b| b == b'<' || b == b'=') => Some(next),
            Some(b'>') if !self.byte_in(next, |b| matches!(b, b'<' | b'=' | b'>')) => Some(next),
            Some(b'?') => self
                .prev(p)
                .is_some_and(|b| b == b'\t' || b == b' ')
                .then_some(next),
            _ => self.first_of(p, &[b"<=", b">=", b"===", b"==", b"=~", b"!=", b"!~"]),
        }
    }

    /// `(?<=^|[\t !])!|&&|\|\||\^`
    pub(in crate::ruby) fn logical_operator(&mut self, p: usize) -> Option<usize> {
        let bang = self.byte_in(p, |b| b == b'!')
            && (p == 0
                || self
                    .prev(p)
                    .is_some_and(|b| matches!(b, b'\t' | b' ' | b'!')));
        if bang {
            return Some(p + 1);
        }
        self.first_of(p, &[b"&&", b"||", b"^"])
    }

    /// `(op)\s*(?![A-Z])`: an operator before a method's name.
    /// When a capital follows the blanks, they give back their last character.
    /// That character then follows instead.
    fn before_method(&mut self, p: usize, op_end: usize) -> Option<usize> {
        let mut end = self.space_end(op_end);
        if self.byte_in(end, |b| b.is_ascii_uppercase()) {
            if end == op_end {
                return None;
            }
            end = self.char_start_before(end);
        }
        self.capture(1, p, op_end);
        Some(end)
    }

    /// `(&\.)\s*(?![A-Z])`
    pub(in crate::ruby) fn safe_navigation(&mut self, p: usize) -> Option<usize> {
        let op_end = self.lit(p, b"&.")?;
        self.before_method(p, op_end)
    }

    /// `(\.|::)\s*(?![A-Z])`
    pub(in crate::ruby) fn method_separator(&mut self, p: usize) -> Option<usize> {
        let op_end = self.first_of(p, &[b".", b"::"])?;
        self.before_method(p, op_end)
    }

    /// `(::)\s*(?=[A-Z])`
    pub(in crate::ruby) fn namespace_separator(&mut self, p: usize) -> Option<usize> {
        let op_end = self.lit(p, b"::")?;
        let end = self.space_end(op_end);
        self.byte_in(end, |b| b.is_ascii_uppercase()).then(|| {
            self.capture(1, p, op_end);
            end
        })
    }

    /// `([%\&]|\*\*|[-*+/])`
    pub(in crate::ruby) fn arithmetic(&mut self, p: usize) -> Option<usize> {
        self.first_of(p, &[b"%", b"&", b"**", b"-", b"*", b"+", b"/"])
    }

    /// `[|~]|>>`
    pub(in crate::ruby) fn bitwise(&mut self, p: usize) -> Option<usize> {
        self.first_of(p, &[b"|", b"~", b">>"])
    }

    /// `[()]`
    pub(in crate::ruby) fn paren(&mut self, p: usize) -> Option<usize> {
        self.first_of(p, &[b"(", b")"])
    }

    /// A backtick that no `.` precedes: `` (?<!\.)` ``.
    pub(in crate::ruby) fn command_start(&mut self, p: usize) -> Option<usize> {
        (!self.prev_is(p, b'.'))
            .then(|| self.lit(p, b"`"))
            .flatten()
    }
}
