//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups of its match with [`Lexer::capture`].
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
use super::grammar::Pattern;
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
    /// Whether `pattern` matches at `p`, and where its match ends.
    ///
    /// The arms come in groups, in this order:
    /// - roxygen comments, their tokens, their Markdown and `@examples`;
    /// - comments, brackets and function definitions;
    /// - strings, raw strings and escapes;
    /// - keywords, names, numbers and operators.
    pub(super) fn matcher(&mut self, pattern: Pattern, p: usize) -> Option<usize> {
        match pattern {
            Pattern::Roxygen => self.roxygen_lead(p, 1),
            Pattern::RoxygenEnd => self.at_eol(p).then_some(p),
            Pattern::MarkdownAt0At0 => self.roxygen_lead(p, 0),
            Pattern::MarkdownAt0 => self.fence(p),
            Pattern::MarkdownAt0End => self.fence_end(p),
            Pattern::MarkdownAt1 => self.markdown_link(p),
            Pattern::MarkdownAt2 => self.markdown_bold_words(p),
            Pattern::MarkdownAt3 => self.markdown_italic_words(p),
            Pattern::MarkdownAt4 => self.markdown_bold(p),
            Pattern::MarkdownAt5 => self.markdown_italic(p),
            Pattern::MarkdownAt6 => self.markdown_code(p),
            Pattern::MarkdownAt7 => self.markdown_angle(p),
            Pattern::RoxygenTokensAt0 => self.lit(p, b"@@"),
            Pattern::RoxygenTokensAt1 => self.param_tag(p),
            Pattern::RoxygenTokensAt1End => self.param_end(p),
            Pattern::RoxygenTokensAt1At0 | Pattern::FunctionDefinitionAt0 => self.dotted_word(p, 1),
            Pattern::RoxygenTokensAt1At1 => self.lit(p, b","),
            Pattern::RoxygenTokensAt2 => self.tag(p),
            Pattern::Latex => self.latex(p),
            Pattern::RoxygenAt3 => (p < self.lim()).then(|| self.char_end(p)),
            Pattern::RoxygenExample => self.examples(p),
            Pattern::RoxygenExampleEnd => self.examples_end(p),
            Pattern::RoxygenExampleAt1 => BRACKETS.contains(&self.at(p)).then_some(p + 1),
            Pattern::Comment => (self.at(p) == b'#').then(|| self.lim()),
            Pattern::BracketsAt0 => self.lit(p, b"{"),
            Pattern::BracketsAt0End => self.lit(p, b"}"),
            Pattern::BracketsAt1 => self.lit(p, b"["),
            Pattern::BracketsAt1End => self.lit(p, b"]"),
            Pattern::BracketsAt2 => self.lit(p, b"("),
            Pattern::BracketsAt2End => self.lit(p, b")"),
            Pattern::BracketsAt1At0 => self.argument_name(p),
            Pattern::FunctionDefinition => self.function_definition(p),
            Pattern::FunctionDefinitionEnd => {
                let end = self.lit(p, b")")?;
                self.capture(1, p, end);
                Some(end)
            }
            Pattern::FunctionDefinitionAt0End => matches!(self.at(p), b')' | b',').then_some(p),
            Pattern::RawStringsAt0 => self.raw_open(p, RAW[0]),
            Pattern::RawStringsAt1 => self.raw_open(p, RAW[1]),
            Pattern::RawStringsAt2 => self.raw_open(p, RAW[2]),
            Pattern::RawStringsAt3 => self.raw_open(p, RAW[3]),
            Pattern::RawStringsAt4 => self.raw_open(p, RAW[4]),
            Pattern::RawStringsAt5 => self.raw_open(p, RAW[5]),
            Pattern::RawStringsAt0End => self.raw_close(p, RAW[0]),
            Pattern::RawStringsAt1End => self.raw_close(p, RAW[1]),
            Pattern::RawStringsAt2End => self.raw_close(p, RAW[2]),
            Pattern::RawStringsAt3End => self.raw_close(p, RAW[3]),
            Pattern::RawStringsAt4End => self.raw_close(p, RAW[4]),
            Pattern::RawStringsAt5End => self.raw_close(p, RAW[5]),
            Pattern::Qstring | Pattern::QstringEnd => self.lit(p, b"'"),
            Pattern::Qqstring | Pattern::QqstringEnd => self.lit(p, b"\""),
            Pattern::IdentifierQuoted | Pattern::IdentifierQuotedEnd => self.lit(p, b"`"),
            Pattern::IdentifierQuotedAt0 => self.lit(p, b"\\`"),
            Pattern::EscapeCode => self.escape_code(p),
            Pattern::EscapeHex => self.escape_hex(p),
            Pattern::EscapeOctal => self.escape_octal(p),
            Pattern::EscapeUnicode => self.escape_unicode(p),
            Pattern::EscapeInvalid => self.escape_any(p),
            Pattern::KeywordsControl => self.control_keyword(p),
            Pattern::KeywordsBuiltin => self.builtin_keyword(p),
            Pattern::KeywordsConstant => self.constant(p),
            Pattern::NamespaceCall => self.namespace(p),
            Pattern::FunctionCall => self.function_call(p),
            Pattern::IdentifierSyntactic => self.identifier(p),
            Pattern::NumbersAt0 => self.hex_number(p),
            Pattern::NumbersAt1 => self.number(p),
            Pattern::Operators => self.operator(p),
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
            self.capture(group, p, end);
            end
        })
    }
}
