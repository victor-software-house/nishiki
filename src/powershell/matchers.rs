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
use super::grammar::Pattern;
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
    /// Whether `pattern` matches at `p`, and where its match ends.
    ///
    /// The arms come in groups, in this order:
    /// - comments and the `#requires` directive;
    /// - commands, functions, attributes and declarations;
    /// - variables;
    /// - strings and their escapes;
    /// - groups, blocks and brackets;
    /// - operators and keywords;
    /// - numbers.
    pub(super) fn matcher(&mut self, pattern: Pattern, p: usize) -> Option<usize> {
        match pattern {
            Pattern::BlockComment => self.lit(p, b"<#"),
            Pattern::BlockCommentEnd => self.lit(p, b"#>"),
            Pattern::CommentLine => self.line_comment(p),
            Pattern::CommentLineEnd => self.line_comment_end(p),
            Pattern::CommentEmbeddedDocsAt0 => self.help_keyword(p),
            Pattern::CommentEmbeddedDocsAt1 => self.help_keyword_with_argument(p),
            Pattern::RequiresDirective => self.requires(p),
            Pattern::RequiresDirectiveEnd | Pattern::StopParsingEnd => self.at_eol(p).then_some(p),
            Pattern::RequiresDirectiveAt0 => self.requires_parameter(p),
            Pattern::RequiresDirectiveAt1 => self.requires_value(p),

            Pattern::CommandsAt0 => self.command(p),
            Pattern::CommandsAt1 => self.standalone(p, b"foreach-object"),
            Pattern::CommandsAt2 => self.standalone(p, b"where-object"),
            Pattern::CommandsAt3 => self.standalone(p, b"sort-object"),
            Pattern::CommandsAt4 => self.standalone(p, b"tee-object"),
            Pattern::RootAt20 => self.executable(p),
            Pattern::UsingDirective => self.using(p),
            Pattern::Function => self.function(p),
            Pattern::FunctionEnd => matches!(self.at(p), b'(' | b'{').then_some(p),
            Pattern::Attribute => self.attribute(p),
            Pattern::AttributeAt0At1 => self.attribute_argument(p),
            Pattern::RootAt25 => self.class(p),
            Pattern::StopParsing => self.stop_parsing(p),
            Pattern::StopParsingAt0 => (p < self.lim()).then(|| self.lim()),

            Pattern::VariableAt0 | Pattern::VariableNoPropertyAt0 => self.language_constant(p),
            Pattern::VariableAt1 => self.listed_variable(p, variables::CONSTANTS, Kept),
            Pattern::VariableNoPropertyAt1 => self.listed_variable(p, variables::CONSTANTS, Never),
            Pattern::VariableAt2 => self.automatic(p, Kept),
            Pattern::VariableNoPropertyAt2 => self.automatic(p, Never),
            Pattern::VariableAt3 => self.listed_variable(p, variables::PREFERENCES, Kept),
            Pattern::VariableNoPropertyAt3 => {
                self.listed_variable(p, variables::PREFERENCES, Never)
            }
            Pattern::VariableAt4 => self.scoped(p, Both, Kept),
            Pattern::VariableNoPropertyAt4 => self.scoped(p, Dollar, Never),
            Pattern::VariableAt5 => self.braced_scoped(p, Kept),
            Pattern::VariableNoPropertyAt5 => self.braced_scoped(p, Never),
            Pattern::VariableAt6 => self.variable(p, Both, Kept),
            Pattern::VariableNoPropertyAt6 => self.variable(p, Dollar, Never),
            Pattern::VariableAt7 => self.braced(p, Kept),
            Pattern::VariableNoPropertyAt7 => self.braced(p, Never),

            Pattern::DoubleQuotedString | Pattern::DoubleQuotedStringEnd => self.double_quote(p),
            Pattern::DoubleQuotedStringAt3 => {
                self.double_quote(p).and_then(|q| self.double_quote(q))
            }
            Pattern::DoubleQuotedStringAt0 => self.address(p),
            Pattern::DoubleQuotedStringAt5 => self.line_continuation(p),
            Pattern::DoubleQuotedStringEscapesAt0 => self.escape(p),
            Pattern::UnicodeEscapeAt0 => self.unicode_escape(p),
            Pattern::UnicodeEscapeAt1 => self.bad_unicode_escape(p),
            Pattern::SingleQuoted | Pattern::SingleQuotedEnd => self.single_quote(p),
            Pattern::SingleQuotedAt0 => self.single_quote(p).and_then(|q| self.single_quote(q)),
            Pattern::HereDouble => self.here_open(p, Lexer::double_quote),
            Pattern::HereSingle => self.here_open(p, Lexer::single_quote),
            Pattern::HereDoubleEnd => self.here_close(p, Lexer::double_quote),
            Pattern::HereSingleEnd => self.here_close(p, Lexer::single_quote),

            Pattern::Hashtable => self.sigil_open(p, b'@', b'{'),
            Pattern::ArrayExpression => self.sigil_open(p, b'@', b'('),
            Pattern::ComplexSubexpression => self.substatement(p, 3),
            Pattern::Interpolation => self.substatement(p, 5),
            Pattern::HashtableEnd => self.closer(p, b'}'),
            Pattern::AttributeEnd => self.closer(p, b']'),
            Pattern::InterpolationEnd => self.closer(p, b')'),
            Pattern::HashtableAt0 => self.key_assignment(p),
            Pattern::Scriptblock => self.lit(p, b"{"),
            Pattern::ScriptblockEnd => self.lit(p, b"}"),
            Pattern::Subexpression | Pattern::AttributeAt0 => self.lit(p, b"("),
            Pattern::SubexpressionEnd
            | Pattern::AttributeAt0End
            | Pattern::ArrayExpressionEnd
            | Pattern::ComplexSubexpressionEnd => self.lit(p, b")"),
            Pattern::Type => self.lit(p, b"["),
            Pattern::TypeEnd => self.lit(p, b"]"),
            Pattern::TypeAt0 => self.type_name(p),

            Pattern::RootAt1 => self.redirection(p),
            Pattern::RootAt21 => self.control(p),
            Pattern::RootAt22 => self.foreach_where(p),
            Pattern::RootAt24 => self.modifier(p),
            Pattern::RootAt26 => self.type_operator(p),
            Pattern::RootAt27 => self.comparison(p),
            Pattern::RootAt28 => self.dash_word_or_bang(p, operators::UNARY),
            Pattern::RootAt29 => self.dash_word_or_bang(p, operators::LOGICAL),
            Pattern::RootAt30 => self.dash_word(p, operators::BITWISE),
            Pattern::RootAt31 => self.dash_word(p, &[b"f"]),
            Pattern::RootAt32 => self.assignment(p),
            Pattern::RootAt33 => self.terminator(p),
            Pattern::RootAt34 => self.other_operator(p),
            Pattern::RootAt35 => self.range(p),

            Pattern::NumericConstantAt0 => self.hex(p),
            Pattern::NumericConstantAt1 => self.real(p),
            Pattern::NumericConstantAt2 => self.binary(p),
            Pattern::NumericConstantAt3 => self.exponent(p, numbers::Dot::Never),
            Pattern::NumericConstantAt4 => self.exponent(p, numbers::Dot::Needed),
            Pattern::NumericConstantAt5 => self.typed_integer(p),
            Pattern::NumericConstantAt6 => self.integer(p),
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
        self.capture(1, p, p + 1);
        Some(p + 1)
    }
}
