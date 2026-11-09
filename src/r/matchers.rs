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
//! A regular expression sees the line feed at the end of a line.
//! So `\s` and a negated class such as `[^>]` match it there, and a match can
//! end one past the text.
//! `.` never matches it.

mod code;
mod roxygen;
mod strings;

use super::engine::Lexer;
use super::grammar::Pat;
use crate::rules::Chars;

/// The brackets that `[]()\[{}]` names in an `@examples` block.
const BRACKETS: [u8; 6] = *b"]()[{}";

/// The operators of the six raw-string forms, in the grammar's order.
/// Each holds the quote, the bracket after the dashes and the bracket before
/// them at the end.
const RAW: [(u8, u8, u8); 6] = [
    (b'"', b'{', b'}'),
    (b'\'', b'{', b'}'),
    (b'"', b'[', b']'),
    (b'\'', b'[', b']'),
    (b'"', b'(', b')'),
    (b'\'', b'(', b')'),
];

impl Lexer<'_> {
    /// Whether `pat` matches at `p`, and where its match ends.
    ///
    /// The arms come in groups, in this order:
    /// - roxygen comments, their tokens, their Markdown and `@examples`;
    /// - comments, brackets and function definitions;
    /// - strings, raw strings and escapes;
    /// - keywords, names, numbers and operators.
    pub(super) fn matcher(&mut self, pat: Pat, p: usize) -> Option<usize> {
        match pat {
            Pat::Roxygen => self.roxygen_lead(p, 1),
            Pat::RoxygenEnd => self.at_eol(p).then_some(p),
            Pat::MarkdownAt0At0 => self.roxygen_lead(p, 0),
            Pat::MarkdownAt0 => self.fence(p),
            Pat::MarkdownAt0End => self.fence_end(p),
            Pat::MarkdownAt1 => self.markdown_link(p),
            Pat::MarkdownAt2 => self.markdown_bold_words(p),
            Pat::MarkdownAt3 => self.markdown_italic_words(p),
            Pat::MarkdownAt4 => self.markdown_bold(p),
            Pat::MarkdownAt5 => self.markdown_italic(p),
            Pat::MarkdownAt6 => self.markdown_code(p),
            Pat::MarkdownAt7 => self.markdown_angle(p),
            Pat::RoxygenTokensAt0 => self.lit(p, b"@@"),
            Pat::RoxygenTokensAt1 => self.param_tag(p),
            Pat::RoxygenTokensAt1End => self.param_end(p),
            Pat::RoxygenTokensAt1At0 | Pat::FunctionDefinitionAt0 => self.dotted_word(p, 1),
            Pat::RoxygenTokensAt1At1 => self.lit(p, b","),
            Pat::RoxygenTokensAt2 => self.tag(p),
            Pat::Latex => self.latex(p),
            Pat::RoxygenAt3 => (p < self.lim()).then(|| self.char_end(p)),
            Pat::RoxygenExample => self.examples(p),
            Pat::RoxygenExampleEnd => self.examples_end(p),
            Pat::RoxygenExampleAt1 => BRACKETS.contains(&self.at(p)).then_some(p + 1),
            Pat::Comment => (self.at(p) == b'#').then(|| self.lim()),
            Pat::BracketsAt0 => self.lit(p, b"{"),
            Pat::BracketsAt0End => self.lit(p, b"}"),
            Pat::BracketsAt1 => self.lit(p, b"["),
            Pat::BracketsAt1End => self.lit(p, b"]"),
            Pat::BracketsAt2 => self.lit(p, b"("),
            Pat::BracketsAt2End => self.lit(p, b")"),
            Pat::BracketsAt1At0 => self.argument_name(p),
            Pat::FunctionDefinition => self.function_definition(p),
            Pat::FunctionDefinitionEnd => {
                let end = self.lit(p, b")")?;
                self.cap(1, p, end);
                Some(end)
            }
            Pat::FunctionDefinitionAt0End => matches!(self.at(p), b')' | b',').then_some(p),
            Pat::RawStringsAt0 => self.raw_open(p, RAW[0]),
            Pat::RawStringsAt1 => self.raw_open(p, RAW[1]),
            Pat::RawStringsAt2 => self.raw_open(p, RAW[2]),
            Pat::RawStringsAt3 => self.raw_open(p, RAW[3]),
            Pat::RawStringsAt4 => self.raw_open(p, RAW[4]),
            Pat::RawStringsAt5 => self.raw_open(p, RAW[5]),
            Pat::RawStringsAt0End => self.raw_close(p, RAW[0]),
            Pat::RawStringsAt1End => self.raw_close(p, RAW[1]),
            Pat::RawStringsAt2End => self.raw_close(p, RAW[2]),
            Pat::RawStringsAt3End => self.raw_close(p, RAW[3]),
            Pat::RawStringsAt4End => self.raw_close(p, RAW[4]),
            Pat::RawStringsAt5End => self.raw_close(p, RAW[5]),
            Pat::Qstring | Pat::QstringEnd => self.lit(p, b"'"),
            Pat::Qqstring | Pat::QqstringEnd => self.lit(p, b"\""),
            Pat::IdentifierQuoted | Pat::IdentifierQuotedEnd => self.lit(p, b"`"),
            Pat::IdentifierQuotedAt0 => self.lit(p, b"\\`"),
            Pat::EscapeCode => self.escape_code(p),
            Pat::EscapeHex => self.escape_hex(p),
            Pat::EscapeOctal => self.escape_octal(p),
            Pat::EscapeUnicode => self.escape_unicode(p),
            Pat::EscapeInvalid => self.escape_any(p),
            Pat::KeywordsControl => self.control_keyword(p),
            Pat::KeywordsBuiltin => self.builtin_keyword(p),
            Pat::KeywordsConstant => self.constant(p),
            Pat::NamespaceCall => self.namespace(p),
            Pat::FunctionCall => self.function_call(p),
            Pat::IdentifierSyntactic => self.identifier(p),
            Pat::NumbersAt0 => self.hex_number(p),
            Pat::NumbersAt1 => self.number(p),
            Pat::Operators => self.operator(p),
        }
    }

    /// The end of the run of `[.\w]` characters from `i`.
    fn dotted_end(&self, mut i: usize) -> usize {
        let from = i;
        loop {
            if self.at(i) == b'.' && i < self.lim() {
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

    /// `([.\w]+)`: a name with dots, recorded as `group`.
    fn dotted_word(&mut self, p: usize, group: usize) -> Option<usize> {
        let end = self.dotted_end(p);
        (end > p).then(|| {
            self.cap(group, p, end);
            end
        })
    }
}
