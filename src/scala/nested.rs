//! The capture groups that the grammar tokenizes with patterns of their own.

use super::engine::Lexer;
use super::grammar::Nested;
use super::matchers::{is_name_start, is_op, is_upper_start};
use super::scope::S;
use crate::scope::{Scopes, scopes};

impl Lexer<'_> {
    /// Emits the tokens of capture group text `from..to`, and leaves the
    /// cursor at `to`.
    /// The text sits under the stack `base`.
    pub(super) fn tokenize_capture(&mut self, kind: Nested, from: usize, to: usize, base: u32) {
        let to = to.min(self.s.len());
        match kind {
            Nested::Operators => self.operator_pairs(from, to, base),
            Nested::Operator => self.operator_one(from, to, base),
            Nested::UsingName => self.using_name(from, to, base),
        }
        self.emit(to, base);
    }

    /// Emits `scopes` over `from..to` on top of `base`, after the text before
    /// `from` under `base`.
    fn emit_under(&mut self, from: usize, to: usize, base: u32, scopes: Scopes<S>) {
        self.emit(from, base);
        let node = self.path(base, scopes);
        self.emit(to, node);
    }

    /// The patterns of a run of two or more operator characters, tried from
    /// the left: `(\|\||&&)` is logical, `([!<=>]=)` a comparison, and `..`
    /// takes any other two characters.
    /// A last single character takes none.
    fn operator_pairs(&mut self, from: usize, to: usize, base: u32) {
        let mut i = from;
        while i < to {
            let pair = &self.s[i..to.min(i + 2)];
            let scopes = match pair {
                b"||" | b"&&" => scopes![S::KwOperatorLogical],
                [b'!' | b'<' | b'=' | b'>', b'='] => scopes![S::KwOperatorComparison],
                _ => scopes![S::KwOperator],
            };
            let second = self.char_end(i);
            if second >= to {
                break;
            }
            let end = self.char_end(second);
            self.emit_under(i, end, base, scopes);
            i = end;
        }
    }

    /// The patterns of one operator character: `(!)` is logical,
    /// `([-%*+/~])` arithmetic, `([<=>])` a comparison, and `.` any other.
    fn operator_one(&mut self, from: usize, to: usize, base: u32) {
        if from >= to {
            return;
        }
        let scopes = match self.s[from] {
            b'!' => scopes![S::KwOperatorLogical],
            b'-' | b'%' | b'*' | b'+' | b'/' | b'~' => scopes![S::KwOperatorArithmetic],
            b'<' | b'=' | b'>' => scopes![S::KwOperatorComparison],
            _ => scopes![S::KwOperator],
        };
        let end = self.char_end(from).min(to);
        self.emit_under(from, end, base, scopes);
    }

    /// The patterns of a `//> using` directive's name: names, quoted names and
    /// operator names are imports, and `.` is punctuation.
    /// Other characters take neither.
    ///
    /// The patterns see only the name, which ends at the first blank.
    /// A quoted name that closes after it does not match, and its backtick
    /// takes neither.
    /// The other names hold no blank, so they end inside the name.
    fn using_name(&mut self, from: usize, to: usize, base: u32) {
        let mut i = from;
        while i < to {
            if self.s[i] == b'.' {
                self.emit_under(i, i + 1, base, scopes![S::PDefinitionImport]);
                i += 1;
                continue;
            }
            match self.using_name_at(i).filter(|&end| end <= to) {
                Some(end) if end > i => {
                    self.emit_under(i, end, base, scopes![S::EntNameImport]);
                    i = end;
                }
                _ => i = self.char_end(i),
            }
        }
    }

    /// The end of `upper|quoted|(?:plain|[op]+)` at `i`.
    fn using_name_at(&self, i: usize) -> Option<usize> {
        let (c, _) = self.char_at(i)?;
        if is_upper_start(c) || is_name_start(c) {
            return self.plain_name(i);
        }
        if c == '`' {
            return self.quoted_name(i);
        }
        is_op(c).then(|| self.op_name(i)).flatten()
    }
}
