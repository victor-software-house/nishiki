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
//! `^` holds only at the start of the line: Shiki's engine does not match it
//! after the line feed that ends the line.
//! `.` matches any character but the line feed, and a negated class matches
//! the line feed too.
//! `\w`, `\s` and `\d` read Unicode, through [`crate::classes`].
//! Under `(?i)`, a literal also matches the characters above ASCII that fold
//! to its letters, through [`crate::case_fold`].

mod names;
mod numbers;
mod words;

use super::engine::Lexer;
use super::grammar::{
    ASP_CLASSES, ASP_COLLECTIONS, ASP_CONSTANTS, ASP_EVENTS, ASP_FUNCTIONS, CONTROL, FUNCTIONS,
    MODIFIERS, OPERATORS, Pat, VB_CONSTANTS,
};
use crate::rules::Chars;

impl Lexer<'_> {
    /// Whether `pat` matches at `p`, and where its match ends.
    pub(super) fn matcher(&mut self, pat: Pat, p: usize) -> Option<usize> {
        match pat {
            Pat::LineFeed | Pat::CommentAt0End => self.line_feed(p),
            Pat::RoundBrackets => self.lit(p, b"("),
            Pat::RoundBracketsEnd => self.lit(p, b")"),
            Pat::Tabs => (p == 0 && self.starts(p, b"\t")).then_some(p),
            Pat::TabsEnd => (self.avail(p) && self.at(p) != b'\t').then_some(p),
            Pat::TabsAt0 => self.pair(p, b"\t"),
            Pat::Spaces => (p == 0 && self.starts(p, b" ")).then_some(p),
            Pat::SpacesEnd => (self.avail(p) && self.at(p) != b' ').then_some(p),
            Pat::SpacesAt0 => self.pair(p, b"  "),
            Pat::FunctionHead => self.function_head(p),
            Pat::Comment => self.comment(p),
            Pat::CommentEnd => (!self.at_g(p)).then_some(p),
            Pat::CommentAt0 => self.lit(p, b"'"),
            Pat::Control => self.ordered(p, &CONTROL),
            Pat::Operators => self.ordered(p, &OPERATORS),
            Pat::DimName => self.dim_name(p),
            Pat::Storage => self.storage(p),
            Pat::Modifiers => self.ordered(p, &MODIFIERS),
            Pat::Constants => self.constant(p),
            Pat::String | Pat::StringEnd => self.lit(p, b"\""),
            Pat::StringAt0 => self.lit(p, b"\"\""),
            Pat::DollarVariable => self.dollar_variable(p),
            Pat::AspClasses => self.ordered(p, &ASP_CLASSES),
            Pat::AspCollections => self.ordered(p, &ASP_COLLECTIONS),
            Pat::AspConstants => self.ordered(p, &ASP_CONSTANTS),
            Pat::AspFunctions => self.ordered(p, &ASP_FUNCTIONS),
            Pat::AspEvents => self.ordered(p, &ASP_EVENTS),
            Pat::AsType => self.as_type(p),
            Pat::Functions => self.ordered(p, &FUNCTIONS),
            Pat::Numbers => self.number(p),
            Pat::VbConstants => self.ordered(p, &VB_CONSTANTS),
            Pat::FunctionCall => self.function_call(p),
            Pat::Variable => self.variable(p),
            Pat::JsOperators => self.js_operator(p),
        }
    }

    /// `\n`: the line feed that ends the line.
    fn line_feed(&self, p: usize) -> Option<usize> {
        (p == self.lim() && self.at(p) == b'\n').then_some(p + 1)
    }

    /// `(unit)(unit)?`: an odd and an even step of indentation.
    fn pair(&mut self, p: usize, unit: &[u8]) -> Option<usize> {
        let odd = self.lit(p, unit)?;
        self.cap(1, p, odd);
        match self.lit(odd, unit) {
            Some(even) => {
                self.cap(2, odd, even);
                Some(even)
            }
            None => Some(odd),
        }
    }

    /// `(^[\t ]+)?(?=')`: the start of a comment, with the blanks that open
    /// the line before it.
    /// Without a `'` after the blanks, the group gives way, and the lookahead
    /// then fails at the first blank.
    fn comment(&mut self, p: usize) -> Option<usize> {
        let blanks = if p == 0 {
            self.skip(p, |b| matches!(b, b'\t' | b' '))
        } else {
            p
        };
        if !self.starts(blanks, b"'") {
            return None;
        }
        if blanks > p {
            self.cap(1, p, blanks);
        }
        Some(blanks)
    }

    /// The JavaScript operators of `JsOperators`, as its alternation takes
    /// them: the first alternative in order that matches.
    ///
    /// An earlier alternative hides several later ones.
    /// `[!$%\&*]` takes the `!` of `!==`, the `&` of `&&` and `&=`, the `*` of
    /// `*=` and the `%` of `%=`.
    /// `--?` and `[+~]` take the `-` of `-=` and the `+` of `+=`.
    fn js_operator(&self, p: usize) -> Option<usize> {
        if p >= self.lim() {
            return None;
        }
        let two = |text: &[u8]| self.lit(p, text);
        match self.at(p) {
            b'!' | b'$' | b'%' | b'&' | b'*' | b'~' => Some(p + 1),
            b'-' => two(b"--").or(Some(p + 1)),
            b'+' => two(b"++").or(Some(p + 1)),
            b'=' => two(b"===").or_else(|| two(b"==")).or(Some(p + 1)),
            b'<' => two(b"<=")
                .or_else(|| two(b"<<="))
                .or_else(|| two(b"<>"))
                .or(Some(p + 1)),
            b'>' => two(b">=")
                .or_else(|| two(b">>="))
                .or_else(|| two(b">>>="))
                .or(Some(p + 1)),
            b'|' => two(b"||"),
            b'?' => two(b"?:"),
            b'/' => two(b"/="),
            b'^' => two(b"^="),
            _ => self.word_in(
                p,
                &[b"in", b"instanceof", b"new", b"delete", b"typeof", b"void"],
            ),
        }
    }
}
