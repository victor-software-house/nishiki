//! Code: argument names, function definitions and calls, keywords, names,
//! numbers and operators.

use super::super::engine::Lexer;
use crate::classes::{is_connector, is_digit, is_letter, is_letter_number, is_nonenclosing_mark};
use crate::rules::Chars;

/// The control keywords after `\\`, in the grammar's order:
/// `(?:\\|function|if|else|in|break|next|repeat|for|while)\b`.
const CONTROL: [&[u8]; 9] = [
    b"function",
    b"if",
    b"else",
    b"in",
    b"break",
    b"next",
    b"repeat",
    b"for",
    b"while",
];

/// The functions that `keywords-builtin` names before a `(`, in the grammar's
/// order, so `tryCatch` comes before `try`.
const BUILTINS: [&[u8]; 19] = [
    b"setGroupGeneric",
    b"setRefClass",
    b"setGeneric",
    b"NextMethod",
    b"setMethod",
    b"UseMethod",
    b"tryCatch",
    b"setClass",
    b"warning",
    b"require",
    b"library",
    b"R6Class",
    b"return",
    b"switch",
    b"attach",
    b"detach",
    b"source",
    b"stop",
    b"try",
];

/// The constants of `keywords-constant`, in the grammar's order, so each
/// `NA_` form comes before `NA`.
const CONSTANTS: [&[u8]; 10] = [
    b"NA_character_",
    b"NA_integer_",
    b"NA_complex_",
    b"NA_real_",
    b"TRUE",
    b"FALSE",
    b"NULL",
    b"Inf",
    b"NaN",
    b"NA",
];

/// The operators of two or three bytes, in the grammar's order after
/// `%.*?%`: the first that the text starts with wins.
const LONG_OPERATORS: [&[u8]; 16] = [
    b":::", b"::", b":=", b"|>", b"=>", b"%%", b">=", b"<=", b"==", b"!=", b"<<-", b"->>", b"->",
    b"<-", b"||", b"&&",
];

impl Lexer<'_> {
    /// `([.\w]+)\s*(?==[^=])`: an argument's name before a single `=`.
    /// The name gives nothing back, since a blank or `=` must follow it.
    /// Neither do the blanks: a shorter run leaves a blank where `=` must
    /// stand.
    pub(super) fn argument_name(&mut self, p: usize) -> Option<usize> {
        let end = self.dotted_end(p);
        if end == p {
            return None;
        }
        let blanks = self.space_end(end);
        if self.at(blanks) != b'=' || !self.avail(blanks + 1) || self.at(blanks + 1) == b'=' {
            return None;
        }
        self.cap(1, p, end);
        Some(blanks)
    }

    /// `(function)\s*(\()`: the start of a function definition.
    /// No boundary comes before `function`.
    pub(super) fn function_definition(&mut self, p: usize) -> Option<usize> {
        let word = self.lit(p, b"function")?;
        let open = self.space_end(word);
        if self.at(open) != b'(' {
            return None;
        }
        self.cap(1, p, word);
        self.cap(2, open, open + 1);
        Some(open + 1)
    }

    /// `(?:\\|function|if|else|in|break|next|repeat|for|while)\b`.
    /// Only a boundary after the word is asked, so `xif` holds `if`.
    pub(super) fn control_keyword(&self, p: usize) -> Option<usize> {
        if self.at(p) == b'\\' {
            return self.boundary(p + 1).then_some(p + 1);
        }
        CONTROL
            .iter()
            .filter_map(|word| self.lit(p, word))
            .find(|&end| self.boundary(end))
    }

    /// `(?:setGroupGeneric|…|stop|try)(?=\()`: a built-in function before its
    /// arguments.
    pub(super) fn builtin_keyword(&self, p: usize) -> Option<usize> {
        if !matches!(
            self.at(p),
            b'N' | b'R' | b'U' | b'a' | b'd' | b'l' | b'r' | b's' | b't' | b'w'
        ) {
            return None;
        }
        BUILTINS
            .iter()
            .filter_map(|word| self.lit(p, word))
            .find(|&end| self.at(end) == b'(')
    }

    /// `(?:NA_character_|…|NaN|NA)\b`: a constant.
    pub(super) fn constant(&self, p: usize) -> Option<usize> {
        if !matches!(self.at(p), b'F' | b'I' | b'N' | b'T') {
            return None;
        }
        CONSTANTS
            .iter()
            .filter_map(|word| self.lit(p, word))
            .find(|&end| self.boundary(end))
    }

    /// `(?<![.0-9A-Z_a-z])([.A-Z_a-z][.0-9A-Z_a-z]*)(:::?)`: a package before
    /// `::` or `:::`.
    /// The name gives nothing back, since a colon must follow it.
    pub(super) fn namespace(&mut self, p: usize) -> Option<usize> {
        let dotted = |b: u8| b == b'.' || b.is_ascii_alphanumeric() || b == b'_';
        if self.prev(p).is_some_and(dotted) || self.at(p).is_ascii_digit() || !dotted(self.at(p)) {
            return None;
        }
        let name = self.skip(p, dotted);
        let colons = self.lit(name, b"::")?;
        let end = self.lit(colons, b":").unwrap_or(colons);
        self.cap(1, p, name);
        self.cap(2, name, end);
        Some(end)
    }

    /// `([.\w]+)(?=\()`: a function's name before its arguments.
    /// The name gives nothing back, since `(` must follow it.
    pub(super) fn function_call(&mut self, p: usize) -> Option<usize> {
        let end = self.dotted_end(p);
        if end == p || self.at(end) != b'(' {
            return None;
        }
        self.cap(1, p, end);
        Some(end)
    }

    /// `[.\p{L}\p{Nl}][.\p{L}\p{Nl}\p{Mn}\p{Mc}\d\p{Pc}]*`: a name.
    /// In ASCII, a letter or `.` starts it, and letters, digits, `.` and `_`
    /// go on with it.
    pub(super) fn identifier(&self, p: usize) -> Option<usize> {
        let (c, mut end) = self.char_at(p)?;
        if !(c == '.' || is_letter(c) || is_letter_number(c)) {
            return None;
        }
        while let Some((c, next)) = self.char_at(end) {
            let part = if c.is_ascii() {
                c == '.' || c == '_' || c.is_ascii_alphanumeric()
            } else {
                is_letter(c)
                    || is_letter_number(c)
                    || is_nonenclosing_mark(c)
                    || is_digit(c)
                    || is_connector(c)
            };
            if !part {
                break;
            }
            end = next;
        }
        self.spend(end - p);
        Some(end)
    }

    /// `0[Xx]\h+(?:p[-+]?\d+)?[Li]?`: a hexadecimal number.
    pub(super) fn hex_number(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'0' || !matches!(self.at(p + 1), b'X' | b'x') {
            return None;
        }
        let digits = self.skip(p + 2, |b| b.is_ascii_hexdigit());
        if digits == p + 2 {
            return None;
        }
        let mut end = digits;
        if self.at(end) == b'p' {
            let sign = end + 1 + usize::from(matches!(self.at(end + 1), b'-' | b'+'));
            let power = self.digit_end(sign);
            if power > sign {
                end = power;
            }
        }
        Some(end + usize::from(matches!(self.at(end), b'L' | b'i')))
    }

    /// `(?:\d+(?:\.\d*)?|\.\d+)(?:[Ee][-+]?\d*)?[Li]?`: a number.
    /// The exponent's digits may be missing, so `1e` is a number.
    pub(super) fn number(&self, p: usize) -> Option<usize> {
        let digits = self.digit_end(p);
        let mut end = if digits > p {
            if self.at(digits) == b'.' {
                self.digit_end(digits + 1)
            } else {
                digits
            }
        } else if self.at(p) == b'.' {
            let fraction = self.digit_end(p + 1);
            if fraction == p + 1 {
                return None;
            }
            fraction
        } else {
            return None;
        };
        if matches!(self.at(end), b'E' | b'e') {
            let sign = end + 1 + usize::from(matches!(self.at(end + 1), b'-' | b'+'));
            end = self.digit_end(sign);
        }
        Some(end + usize::from(matches!(self.at(end), b'L' | b'i')))
    }

    /// `%.*?%|:::?|:=|…|\*\*?|[!$\&,/:<>?@^|~]`: an operator.
    /// `%.*?%` ends at the next `%`, so `%%` is one of its matches.
    pub(super) fn operator(&self, p: usize) -> Option<usize> {
        let b = self.at(p);
        if b == b'%' {
            let close = memchr::memchr(b'%', self.text(p + 1, self.lim()))?;
            self.spend(close);
            return Some(p + close + 2);
        }
        if let Some(end) = LONG_OPERATORS.iter().find_map(|op| self.lit(p, op)) {
            return Some(end);
        }
        match b {
            b'-' | b'+' | b'=' | b'!' | b'$' | b'&' | b',' | b'/' | b':' | b'<' | b'>' | b'?'
            | b'@' | b'^' | b'|' | b'~' => Some(p + 1),
            b'*' => Some(p + 1 + usize::from(self.at(p + 1) == b'*')),
            _ => None,
        }
    }
}
