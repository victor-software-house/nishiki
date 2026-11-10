//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups of its match with [`Lexer::capture`].
//! The engine names those the grammar names.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would.
//! Where a quantifier could give characters back, the doc comment shows why it
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
    MODIFIERS, OPERATORS, Pattern, VB_CONSTANTS,
};
use crate::rules::Chars;

impl Lexer<'_> {
    /// Whether `pattern` matches at `p`, and where its match ends.
    pub(super) fn matcher(&mut self, pattern: Pattern, p: usize) -> Option<usize> {
        match pattern {
            Pattern::LineFeed | Pattern::CommentAt0End => self.line_feed(p),
            Pattern::RoundBrackets => self.lit(p, b"("),
            Pattern::RoundBracketsEnd => self.lit(p, b")"),
            Pattern::Tabs => (p == 0 && self.starts(p, b"\t")).then_some(p),
            Pattern::TabsEnd => (self.avail(p) && self.at(p) != b'\t').then_some(p),
            Pattern::TabsAt0 => self.pair(p, b"\t"),
            Pattern::Spaces => (p == 0 && self.starts(p, b" ")).then_some(p),
            Pattern::SpacesEnd => (self.avail(p) && self.at(p) != b' ').then_some(p),
            Pattern::SpacesAt0 => self.pair(p, b"  "),
            Pattern::FunctionHead => self.function_head(p),
            Pattern::Comment => self.comment(p),
            Pattern::CommentEnd => (!self.at_g(p)).then_some(p),
            Pattern::CommentAt0 => self.lit(p, b"'"),
            Pattern::Control => self.ordered(p, &CONTROL),
            Pattern::Operators => self.ordered(p, &OPERATORS),
            Pattern::DimName => self.dim_name(p),
            Pattern::Storage => self.storage(p),
            Pattern::Modifiers => self.ordered(p, &MODIFIERS),
            Pattern::Constants => self.constant(p),
            Pattern::String | Pattern::StringEnd => self.lit(p, b"\""),
            Pattern::StringAt0 => self.lit(p, b"\"\""),
            Pattern::DollarVariable => self.dollar_variable(p),
            Pattern::AspClasses => self.ordered(p, &ASP_CLASSES),
            Pattern::AspCollections => self.ordered(p, &ASP_COLLECTIONS),
            Pattern::AspConstants => self.ordered(p, &ASP_CONSTANTS),
            Pattern::AspFunctions => self.ordered(p, &ASP_FUNCTIONS),
            Pattern::AspEvents => self.ordered(p, &ASP_EVENTS),
            Pattern::AsType => self.as_type(p),
            Pattern::Functions => self.ordered(p, &FUNCTIONS),
            Pattern::Numbers => self.number(p),
            Pattern::VbConstants => self.ordered(p, &VB_CONSTANTS),
            Pattern::FunctionCall => self.function_call(p),
            Pattern::Variable => self.variable(p),
            Pattern::JsOperators => self.js_operator(p),
        }
    }

    /// `\n`: the line feed that ends the line.
    fn line_feed(&self, p: usize) -> Option<usize> {
        (p == self.lim() && self.at(p) == b'\n').then_some(p + 1)
    }

    /// `(unit)(unit)?`: an odd and an even step of indentation.
    fn pair(&mut self, p: usize, unit: &[u8]) -> Option<usize> {
        let odd = self.lit(p, unit)?;
        self.capture(1, p, odd);
        match self.lit(odd, unit) {
            Some(even) => {
                self.capture(2, odd, even);
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
            self.capture(1, p, blanks);
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
