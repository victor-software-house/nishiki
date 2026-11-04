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
//! `(?i)` compares words through [`crate::case_fold`], so `ſ`, the Kelvin sign,
//! `ß` and the Latin ligatures match the letters they fold to.

mod commands;
mod comments;
mod numbers;
mod operators;
mod strings;
mod variables;

use variables::Members::{Kept, Never};
use variables::Sigils::{Both, Dollar};

use super::engine::Lexer;
use super::grammar::Pat;
use crate::case_fold;
use crate::classes::{is_digit, is_letter};
use crate::rules::Chars;

/// The longest word a case-insensitive word list holds, with room for the
/// letters a ligature folds to.
const LONGEST_WORD: usize = 48;

/// Whether `c` is in `[_\p{L}\d]`, the characters of a variable's name.
fn is_name(c: char) -> bool {
    c == '_' || is_letter(c) || is_digit(c)
}

impl Lexer<'_> {
    /// Whether `pat` matches at `p`, and where its match ends.
    ///
    /// The arms come in groups, in this order:
    /// - comments and the `#requires` directive;
    /// - commands, functions, attributes and declarations;
    /// - variables;
    /// - strings and their escapes;
    /// - groups, blocks and brackets;
    /// - operators and keywords;
    /// - numbers.
    pub(super) fn matcher(&mut self, pat: Pat, p: usize) -> Option<usize> {
        match pat {
            Pat::BlockComment => self.lit(p, b"<#"),
            Pat::BlockCommentEnd => self.lit(p, b"#>"),
            Pat::CommentLine => self.line_comment(p),
            Pat::CommentLineEnd => self.line_comment_end(p),
            Pat::CommentEmbeddedDocsAt0 => self.help_keyword(p),
            Pat::CommentEmbeddedDocsAt1 => self.help_keyword_with_argument(p),
            Pat::RequiresDirective => self.requires(p),
            Pat::RequiresDirectiveEnd | Pat::StopParsingEnd => self.at_eol(p).then_some(p),
            Pat::RequiresDirectiveAt0 => self.requires_parameter(p),
            Pat::RequiresDirectiveAt1 => self.requires_value(p),

            Pat::CommandsAt0 => self.command(p),
            Pat::CommandsAt1 => self.standalone(p, b"foreach-object"),
            Pat::CommandsAt2 => self.standalone(p, b"where-object"),
            Pat::CommandsAt3 => self.standalone(p, b"sort-object"),
            Pat::CommandsAt4 => self.standalone(p, b"tee-object"),
            Pat::RootAt20 => self.executable(p),
            Pat::UsingDirective => self.using(p),
            Pat::Function => self.function(p),
            Pat::FunctionEnd => matches!(self.at(p), b'(' | b'{').then_some(p),
            Pat::Attribute => self.attribute(p),
            Pat::AttributeAt0At1 => self.attribute_argument(p),
            Pat::RootAt25 => self.class(p),
            Pat::StopParsing => self.stop_parsing(p),
            Pat::StopParsingAt0 => (p < self.lim()).then(|| self.lim()),

            Pat::VariableAt0 | Pat::VariableNoPropertyAt0 => self.language_constant(p),
            Pat::VariableAt1 => self.listed_variable(p, variables::CONSTANTS, Kept),
            Pat::VariableNoPropertyAt1 => self.listed_variable(p, variables::CONSTANTS, Never),
            Pat::VariableAt2 => self.automatic(p, Kept),
            Pat::VariableNoPropertyAt2 => self.automatic(p, Never),
            Pat::VariableAt3 => self.listed_variable(p, variables::PREFERENCES, Kept),
            Pat::VariableNoPropertyAt3 => self.listed_variable(p, variables::PREFERENCES, Never),
            Pat::VariableAt4 => self.scoped(p, Both, Kept),
            Pat::VariableNoPropertyAt4 => self.scoped(p, Dollar, Never),
            Pat::VariableAt5 => self.braced_scoped(p, Kept),
            Pat::VariableNoPropertyAt5 => self.braced_scoped(p, Never),
            Pat::VariableAt6 => self.variable(p, Both, Kept),
            Pat::VariableNoPropertyAt6 => self.variable(p, Dollar, Never),
            Pat::VariableAt7 => self.braced(p, Kept),
            Pat::VariableNoPropertyAt7 => self.braced(p, Never),

            Pat::DoubleQuotedString | Pat::DoubleQuotedStringEnd => self.double_quote(p),
            Pat::DoubleQuotedStringAt3 => self.double_quote(p).and_then(|q| self.double_quote(q)),
            Pat::DoubleQuotedStringAt0 => self.address(p),
            Pat::DoubleQuotedStringAt5 => self.line_continuation(p),
            Pat::DoubleQuotedStringEscapesAt0 => self.escape(p),
            Pat::UnicodeEscapeAt0 => self.unicode_escape(p),
            Pat::UnicodeEscapeAt1 => self.bad_unicode_escape(p),
            Pat::SingleQuoted | Pat::SingleQuotedEnd => self.single_quote(p),
            Pat::SingleQuotedAt0 => self.single_quote(p).and_then(|q| self.single_quote(q)),
            Pat::HereDouble => self.here_open(p, Lexer::double_quote),
            Pat::HereSingle => self.here_open(p, Lexer::single_quote),
            Pat::HereDoubleEnd => self.here_close(p, Lexer::double_quote),
            Pat::HereSingleEnd => self.here_close(p, Lexer::single_quote),

            Pat::Hashtable => self.sigil_open(p, b'@', b'{'),
            Pat::ArrayExpression => self.sigil_open(p, b'@', b'('),
            Pat::ComplexSubexpression => self.substatement(p, 3),
            Pat::Interpolation => self.substatement(p, 5),
            Pat::HashtableEnd => self.closer(p, b'}'),
            Pat::AttributeEnd => self.closer(p, b']'),
            Pat::InterpolationEnd => self.closer(p, b')'),
            Pat::HashtableAt0 => self.key_assignment(p),
            Pat::Scriptblock => self.lit(p, b"{"),
            Pat::ScriptblockEnd => self.lit(p, b"}"),
            Pat::Subexpression | Pat::AttributeAt0 => self.lit(p, b"("),
            Pat::SubexpressionEnd
            | Pat::AttributeAt0End
            | Pat::ArrayExpressionEnd
            | Pat::ComplexSubexpressionEnd => self.lit(p, b")"),
            Pat::Type => self.lit(p, b"["),
            Pat::TypeEnd => self.lit(p, b"]"),
            Pat::TypeAt0 => self.type_name(p),

            Pat::RootAt1 => self.redirection(p),
            Pat::RootAt21 => self.control(p),
            Pat::RootAt22 => self.foreach_where(p),
            Pat::RootAt24 => self.modifier(p),
            Pat::RootAt26 => self.type_operator(p),
            Pat::RootAt27 => self.comparison(p),
            Pat::RootAt28 => self.dash_word_or_bang(p, operators::UNARY),
            Pat::RootAt29 => self.dash_word_or_bang(p, operators::LOGICAL),
            Pat::RootAt30 => self.dash_word(p, operators::BITWISE),
            Pat::RootAt31 => self.dash_word(p, &[b"f"]),
            Pat::RootAt32 => self.assignment(p),
            Pat::RootAt33 => self.terminator(p),
            Pat::RootAt34 => self.other_operator(p),
            Pat::RootAt35 => self.range(p),

            Pat::NumericConstantAt0 => self.hex(p),
            Pat::NumericConstantAt1 => self.real(p),
            Pat::NumericConstantAt2 => self.binary(p),
            Pat::NumericConstantAt3 => self.exponent(p, numbers::Dot::Never),
            Pat::NumericConstantAt4 => self.exponent(p, numbers::Dot::Needed),
            Pat::NumericConstantAt5 => self.typed_integer(p),
            Pat::NumericConstantAt6 => self.integer(p),
        }
    }

    /// `(?i:word)` at `i`, and its end.
    fn folded(&self, i: usize, word: &[u8]) -> Option<usize> {
        case_fold::literal_end(self.s, i, word)
    }

    /// The end of the first of `words`, tried in order under `(?i)` at `i`,
    /// after which `then` holds.
    fn first_folded(
        &self,
        i: usize,
        words: &[&[u8]],
        then: impl Fn(&Self, usize) -> bool,
    ) -> Option<usize> {
        words
            .iter()
            .find_map(|word| self.folded(i, word).filter(|&end| then(self, end)))
    }

    /// `(?i:a|b|…)(?!\w)` from `i`, for the lowercase words `words`: the run of
    /// `\w` from `i`, when it folds to one of them.
    ///
    /// No word is cut short.
    /// The `(?!\w)` after it holds only where the run ends.
    fn folded_word(&self, i: usize, words: &[&[u8]]) -> Option<usize> {
        let end = self.word_end(i);
        if end == i {
            return None;
        }
        let mut lower = [0; LONGEST_WORD];
        let len = case_fold::lower_into(self.text(i, end), &mut lower)?;
        words.contains(&&lower[..len]).then_some(end)
    }

    /// The end of the run of characters from `i` that `class` accepts.
    fn class_end(&self, i: usize, class: fn(char) -> bool) -> usize {
        let mut at = i;
        while let Some((c, next)) = self.char_at(at)
            && class(c)
        {
            at = next;
        }
        self.spend(at - i);
        at
    }

    /// Whether a `\p{L}` character stands at `i`.
    fn letter_at(&self, i: usize) -> bool {
        self.char_at(i).is_some_and(|(c, _)| is_letter(c))
    }

    /// `(x)` for the byte `x`, `byte`, with group 1 around it.
    fn closer(&mut self, p: usize, byte: u8) -> Option<usize> {
        if self.at(p) != byte {
            return None;
        }
        self.cap(1, p, p + 1);
        Some(p + 1)
    }
}
