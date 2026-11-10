//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups the grammar names with
//! [`Lexer::capture`]. The engine names them.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would.
//! Where a quantifier could give characters back, the doc comment shows why it
//! cannot matter, or what giving them back leaves to test.
//! `\w`, `\s` and `\d` take non-ASCII characters, as in Oniguruma, through
//! [`Chars`].

mod declarations;
mod values;

use super::engine::Lexer;
use super::grammar::Pattern;
use crate::rules::Chars;

/// Whether `b` is in the grammar's `[0-9A-Z_a-z]`.
fn is_name(b: u8) -> bool {
    b == b'_' || b.is_ascii_alphanumeric()
}

/// Whether `b` is in the grammar's `[.0-9A-Z_a-z]`.
fn is_dotted_name(b: u8) -> bool {
    b == b'.' || is_name(b)
}

impl Lexer<'_> {
    /// Whether `pattern` matches at `p`, and where its match ends.
    ///
    /// The arms follow the grammar's order: comments, the file's statements,
    /// values, declarations, then fields.
    pub(super) fn matcher(&mut self, pattern: Pattern, p: usize) -> Option<usize> {
        match pattern {
            Pattern::CommentsAt0 => self.lit(p, b"/*"),
            Pattern::CommentsAt0End => self.lit(p, b"*/"),
            Pattern::CommentsAt1 => self.lit(p, b"//"),
            Pattern::CommentsAt1End => self.line_end(p),
            Pattern::Syntax => self.syntax(p),
            Pattern::Package => self.package(p),
            Pattern::Import => self.import(p),
            Pattern::OptionStmt => self.option_statement(p),
            Pattern::OptionStmtEnd
            | Pattern::ReservedEnd
            | Pattern::EnumAt3End
            | Pattern::FieldEnd
            | Pattern::MapfieldEnd => self.semicolon(p),
            Pattern::Constants => self.constant(p),
            Pattern::Number => self.number(p),
            Pattern::String => self.string(p),
            Pattern::SubMsgOption => self.lit(p, b"{"),
            Pattern::SubMsgOptionEnd
            | Pattern::MessageEnd
            | Pattern::EnumEnd
            | Pattern::ServiceEnd
            | Pattern::OneofEnd => self.lit(p, b"}"),
            Pattern::Kv => {
                let end = self.word_end(p);
                let colon = self.space_end(end);
                if end == p || self.at(colon) != b':' {
                    return None;
                }
                self.capture(1, p, end);
                self.capture(2, colon, colon + 1);
                Some(colon + 1)
            }
            Pattern::KvEnd => match self.at(p) {
                b';' => self.semicolon(p),
                b',' => Some(p + 1),
                b'/' | b'}' | b'_' => Some(p),
                b if b.is_ascii_alphabetic() => Some(p),
                _ => None,
            },
            Pattern::Message => self.block_head(p, &[b"message", b"extend"], NameStart::Any),
            Pattern::Reserved => {
                let end = self.lit(p, b"reserved")?;
                let blanks = self.space_end(end);
                if blanks == end {
                    return None;
                }
                self.capture(1, p, end);
                Some(blanks)
            }
            Pattern::ReservedAt0 => self.reserved_range(p),
            Pattern::Enum => self.block_head(p, &[b"enum"], NameStart::Letter),
            Pattern::EnumAt3 => self.enum_value(p),
            Pattern::FieldOptions => self.lit(p, b"["),
            Pattern::FieldOptionsEnd => self.lit(p, b"]"),
            Pattern::OptionName => {
                let name = self.option_name(p)?;
                self.capture(1, p, name.first);
                if let Some((start, end)) = name.inner {
                    self.capture(2, start, end);
                }
                if let Some((start, end)) = name.outer {
                    self.capture(3, start, end);
                }
                Some(name.end)
            }
            Pattern::Service => self.named_head(p, b"service", is_dotted_name),
            Pattern::Method => {
                let end = self.lit(p, b"rpc")?;
                let name = self.space_end(end);
                if name == end || !self.at(name).is_ascii_alphabetic() {
                    return None;
                }
                let name_end = self.skip(name + 1, is_name);
                self.capture(1, p, end);
                self.capture(2, name, name_end);
                Some(name_end)
            }
            Pattern::MethodEnd => match self.at(p) {
                b'}' => Some(p + 1),
                b';' => self.semicolon(p),
                _ => None,
            },
            Pattern::RpcKeywords => self.word_in(p, &[b"stream", b"returns"]),
            Pattern::Ident => {
                let start = if self.at(p) == b'.' { p + 1 } else { p };
                self.at(start)
                    .is_ascii_alphabetic()
                    .then(|| self.skip(start + 1, is_dotted_name))
            }
            Pattern::Oneof => self.named_head(p, b"oneof", is_name),
            Pattern::Field => self.field(p),
            Pattern::Mapfield => self.map_field(p),
        }
    }

    /// `$\n?`: the end of the line, with its line feed when it has one.
    fn line_end(&self, p: usize) -> Option<usize> {
        if !self.at_eol(p) {
            return None;
        }
        Some(if self.at(p) == b'\n' { p + 1 } else { p })
    }

    /// `(;)`, with its group.
    fn semicolon(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, b";")?;
        self.capture(1, p, end);
        Some(end)
    }

    /// `\b(true|false|max|[A-Z_]+)\b`.
    ///
    /// Giving back the run of `[A-Z_]` leaves a word character on both sides
    /// of the `\b`.
    fn constant(&self, p: usize) -> Option<usize> {
        if let Some(end) = self.word_in(p, &[b"true", b"false", b"max"]) {
            return Some(end);
        }
        if !self.boundary(p) {
            return None;
        }
        let end = self.skip(p, |b| b == b'_' || b.is_ascii_uppercase());
        (end > p && self.boundary(end)).then_some(end)
    }

    /// The end of a run of `[.\w]` from `i`.
    fn dotted_word_end(&self, mut i: usize) -> usize {
        let from = i;
        loop {
            if self.at(i) == b'.' {
                i += 1;
            } else if let Some(width) = self.word_at(i) {
                i += width;
            } else {
                break;
            }
        }
        self.spend(i - from);
        i
    }
}

/// How a declared name must start.
#[derive(Clone, Copy, Debug)]
enum NameStart {
    /// `[A-Z_a-z]`.
    Any,
    /// `[A-Za-z]`.
    Letter,
}
