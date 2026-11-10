//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups the grammar names with
//! [`Lexer::capture`]. The engine names them.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would.
//! Where a quantifier could give characters back, the doc comment says why it
//! cannot matter, or what giving them back leaves to test.

mod numbers;
mod strings;

use super::engine::Lexer;
use super::grammar::Pattern;
use crate::rules::Chars;

/// Whether `b` is in the grammar's `[0-9A-Z_a-z]`.
fn is_word(b: u8) -> bool {
    b == b'_' || b.is_ascii_alphanumeric()
}

/// Whether `b` can start a name: the grammar's `[A-Z_a-z]`.
fn is_name_start(b: u8) -> bool {
    b == b'_' || b.is_ascii_alphabetic()
}

/// The words of `\b(TODO|FIXME|XXX|NOTE)\b`.
const TODO: &[&[u8]] = &[b"TODO", b"FIXME", b"XXX", b"NOTE"];

/// The storage keywords.
const STORAGE: &[&[u8]] = &[
    b"extern",
    b"packed",
    b"export",
    b"pub",
    b"noalias",
    b"inline",
    b"comptime",
    b"volatile",
    b"align",
    b"linksection",
    b"threadlocal",
    b"allowzero",
    b"noinline",
    b"callconv",
];

/// The type keywords.
const TYPES: &[&[u8]] = &[
    b"bool",
    b"void",
    b"noreturn",
    b"type",
    b"error",
    b"anyerror",
    b"anyframe",
    b"anytype",
    b"anyopaque",
];

/// The number types spelled out in full.
/// `u\d+` and `i\d+` are matched apart.
const NUMBER_TYPES: &[&[u8]] = &[
    b"f16",
    b"f32",
    b"f64",
    b"f80",
    b"f128",
    b"isize",
    b"usize",
    b"comptime_int",
    b"comptime_float",
];

/// The C types, `c_` and one of these.
const C_TYPES: &[&[u8]] = &[
    b"c_char",
    b"c_short",
    b"c_ushort",
    b"c_int",
    b"c_uint",
    b"c_long",
    b"c_ulong",
    b"c_longlong",
    b"c_ulonglong",
    b"c_longdouble",
];

impl Lexer<'_> {
    /// Whether `pattern` matches at `p`, and where its match ends.
    ///
    /// The arms follow the grammar's order: comments, strings, keywords,
    /// operators, punctuation, numbers, builtins and names.
    pub(super) fn matcher(&mut self, pattern: Pattern, p: usize) -> Option<usize> {
        match pattern {
            Pattern::CommentsAt0 => {
                let end = self.lit(p, b"//")?;
                (matches!(self.at(end), b'!' | b'/')
                    && self.avail(end + 1)
                    && self.at(end + 1) != b'/')
                    .then_some(end + 1)
            }
            Pattern::CommentsAt0End | Pattern::CommentsAt1End | Pattern::StringsAt1End => {
                self.at_eol(p).then_some(p)
            }
            Pattern::CommentContentsAt0 => {
                let end = self.word_in(p, TODO)?;
                Some(if self.at(end) == b':' { end + 1 } else { end })
            }
            Pattern::CommentsAt1 => self.lit(p, b"//"),
            Pattern::StringsAt0
            | Pattern::StringsAt0End
            | Pattern::VariablesAt0At2End
            | Pattern::VariablesAt2At1End => self.lit(p, b"\""),
            Pattern::StringcontentAt0 => self.escape(p),
            Pattern::StringcontentAt1 => {
                (self.at(p) == b'\\' && p + 1 < self.lim()).then(|| self.char_end(p + 1))
            }
            Pattern::StringsAt1 => self.lit(p, b"\\\\"),
            Pattern::StringsAt2 => self.char_literal(p),
            Pattern::KeywordsAt0 => {
                let end = self.keyword(p, b"inline")?;
                self.keyword(self.space_end(end), b"fn")
                    .is_none()
                    .then_some(end)
            }
            Pattern::KeywordsAt1 => self.word_in(p, &[b"while", b"for"]),
            Pattern::KeywordsAt2 => self.word_in(p, STORAGE),
            Pattern::KeywordsAt3 => self.word_in(p, &[b"struct", b"enum", b"union", b"opaque"]),
            Pattern::KeywordsAt4 => self.word_in(p, &[b"asm", b"unreachable"]),
            Pattern::KeywordsAt5 => self.word_in(
                p,
                &[b"break", b"return", b"continue", b"defer", b"errdefer"],
            ),
            Pattern::KeywordsAt6 => self.word_in(p, &[b"resume", b"suspend", b"nosuspend"]),
            Pattern::KeywordsAt7 => self.word_in(p, &[b"try", b"catch"]),
            Pattern::KeywordsAt8 => self.word_in(p, &[b"if", b"else", b"switch", b"orelse"]),
            Pattern::KeywordsAt9 => self.word_in(p, &[b"null", b"undefined"]),
            Pattern::KeywordsAt10 => self.word_in(p, &[b"true", b"false"]),
            Pattern::KeywordsAt11 => self.word_in(p, &[b"test", b"and", b"or"]),
            Pattern::KeywordsAt12 => self.word_in(p, TYPES),
            Pattern::KeywordsAt13 => self.number_type(p),
            Pattern::KeywordsAt14 => self.word_in(p, C_TYPES),
            Pattern::OperatorsAt0 => {
                (self.prev(p) == Some(b'[') && self.starts(p, b"*c") && self.at(p + 2) == b']')
                    .then_some(p + 2)
            }
            Pattern::OperatorsAt1 => self.comparison(p),
            Pattern::OperatorsAt2 => self.arithmetic(p),
            Pattern::OperatorsAt3 => self.bitwise(p),
            Pattern::OperatorsAt4 => [b"==".as_slice(), b"++", b"**", b"->"]
                .iter()
                .any(|pair| self.starts(p, pair))
                .then_some(p + 2),
            Pattern::OperatorsAt5 => self.lit(p, b"="),
            Pattern::OperatorsAt6 => self.lit(p, b"?"),
            Pattern::PunctuationAt0 => self.lit(p, b"."),
            Pattern::PunctuationAt1 => self.lit(p, b","),
            Pattern::PunctuationAt2 => self.lit(p, b":"),
            Pattern::PunctuationAt3 => self.lit(p, b";"),
            Pattern::NumbersAt0 => self.hex_float(p),
            Pattern::NumbersAt1 => self.float(p),
            Pattern::NumbersAt2 => self.decimal(p),
            Pattern::NumbersAt3 => self.prefixed(p, b"0x", |b| b.is_ascii_hexdigit()),
            Pattern::NumbersAt4 => self.prefixed(p, b"0o", |b| matches!(b, b'0'..=b'7')),
            Pattern::NumbersAt5 => self.prefixed(p, b"0b", |b| matches!(b, b'0' | b'1')),
            Pattern::NumbersAt6 => self.invalid_number(p),
            Pattern::SupportAt0 => (self.at(p) == b'@' && is_name_start(self.at(p + 1)))
                .then(|| self.skip(p + 2, is_word)),
            Pattern::VariablesAt0At0 => self.function_name(p, NameStart::Upper),
            Pattern::VariablesAt0At1 => self.function_name(p, NameStart::Any),
            Pattern::VariablesAt0At2 => {
                let blanks = self.fn_blanks(p)?;
                let end = self.lit(blanks, b"@\"")?;
                self.capture(1, p, p + 2);
                Some(end)
            }
            Pattern::VariablesAt0At3 => self.word_in(p, &[b"const", b"var", b"fn"]),
            Pattern::VariablesAt1At0 => self.call_name(p, NameStart::Upper),
            Pattern::VariablesAt1At1 => self.call_name(p, NameStart::Any),
            Pattern::VariablesAt2At0 => {
                if !self.boundary(p) || !is_name_start(self.at(p)) {
                    return None;
                }
                let end = self.skip(p + 1, is_word);
                self.boundary(end).then_some(end)
            }
            Pattern::VariablesAt2At1 => self.lit(p, b"@\""),
        }
    }

    /// A number type:
    ///
    /// ```text
    /// \b(f16|f32|f64|f80|f128|u\d+|i\d+|isize|usize|comptime_int|comptime_float)\b
    /// ```
    ///
    /// `\d` is a word character, so `u\d+` before `\b` takes the whole run
    /// of digits, and the word must end there.
    fn number_type(&self, p: usize) -> Option<usize> {
        if let Some(end) = self.word_in(p, NUMBER_TYPES) {
            return Some(end);
        }
        if !matches!(self.at(p), b'u' | b'i') || !self.boundary(p) {
            return None;
        }
        let end = self.digit_end(p + 1);
        (end > p + 1 && self.word_at(end).is_none()).then_some(end)
    }

    /// `\b((and|or))\b|(==|!=|<=|>=|[<>])`.
    fn comparison(&self, p: usize) -> Option<usize> {
        if let Some(end) = self.word_in(p, &[b"and", b"or"]) {
            return Some(end);
        }
        match self.at(p) {
            b'=' | b'!' => (self.at(p + 1) == b'=').then_some(p + 2),
            b'<' | b'>' => Some(if self.at(p + 1) == b'=' { p + 2 } else { p + 1 }),
            _ => None,
        }
    }

    /// `(-%?|\+%?|\*%?|[%/])=?`.
    fn arithmetic(&self, p: usize) -> Option<usize> {
        let end = match self.at(p) {
            b'-' | b'+' | b'*' if self.at(p + 1) == b'%' => p + 2,
            b'-' | b'+' | b'*' | b'%' | b'/' => p + 1,
            _ => return None,
        };
        Some(if self.at(end) == b'=' { end + 1 } else { end })
    }

    /// `(<<%?|>>|[!\&^|~])=?`.
    fn bitwise(&self, p: usize) -> Option<usize> {
        let end = if self.starts(p, b"<<") {
            if self.at(p + 2) == b'%' { p + 3 } else { p + 2 }
        } else if self.starts(p, b">>") {
            p + 2
        } else if matches!(self.at(p), b'!' | b'&' | b'^' | b'|' | b'~') {
            p + 1
        } else {
            return None;
        };
        Some(if self.at(end) == b'=' { end + 1 } else { end })
    }

    /// `\b(fn)\s+`: the end of the blanks after `fn`.
    fn fn_blanks(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) || !self.starts(p, b"fn") {
            return None;
        }
        let end = self.space_end(p + 2);
        (end > p + 2).then_some(end)
    }

    /// `\b(fn)\s+([A-Z][0-9A-Za-z]*)\b` and
    /// `\b(fn)\s+([A-Z_a-z][0-9A-Z_a-z]*)\b`.
    ///
    /// Giving back blanks leaves a blank where the name must start.
    /// Giving back the name's run leaves a word character on both sides of
    /// the `\b`.
    fn function_name(&mut self, p: usize, start: NameStart) -> Option<usize> {
        let name = self.fn_blanks(p)?;
        let end = self.name_end(name, start)?;
        if !self.boundary(end) {
            return None;
        }
        self.capture(1, p, p + 2);
        self.capture(2, name, end);
        Some(end)
    }

    /// `([A-Z][0-9A-Za-z]*)(?=\s*\()` and `([A-Z_a-z][0-9A-Z_a-z]*)(?=\s*\()`.
    /// Giving back the name's run leaves a name character before the `(`.
    fn call_name(&self, p: usize, start: NameStart) -> Option<usize> {
        let end = self.name_end(p, start)?;
        (self.at(self.space_end(end)) == b'(').then_some(end)
    }

    /// The end of a name from `p` that starts as `start` says.
    /// A type name goes on with `[0-9A-Za-z]`, without `_`.
    fn name_end(&self, p: usize, start: NameStart) -> Option<usize> {
        match start {
            NameStart::Upper => self
                .at(p)
                .is_ascii_uppercase()
                .then(|| self.skip(p + 1, |b| b.is_ascii_alphanumeric())),
            NameStart::Any => is_name_start(self.at(p)).then(|| self.skip(p + 1, is_word)),
        }
    }
}

/// How a name must start.
#[derive(Clone, Copy, Debug)]
enum NameStart {
    /// `[A-Z]`, a type's name.
    Upper,
    /// `[A-Z_a-z]`.
    Any,
}
