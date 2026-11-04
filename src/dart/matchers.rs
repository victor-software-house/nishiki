//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups the grammar names with [`Lexer::cap`].
//! The engine names them.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would.
//! Where a quantifier could give characters back, the doc comment says why it
//! cannot matter, or how the matcher gives them back.

mod comments;
mod names;
mod numbers;
mod operators;
mod strings;
mod words;

use super::engine::Lexer;
use super::grammar::Pat;
use crate::rules::Chars;

/// Whether `b` is in the grammar's `[$0-9A-Z_a-z]`.
fn is_ident(b: u8) -> bool {
    b == b'$' || b == b'_' || b.is_ascii_alphanumeric()
}

impl Lexer<'_> {
    /// Whether `pat` matches at `p`, and where its match ends.
    ///
    /// The arms come in groups, in this order:
    /// - the script line and the import directives;
    /// - comments and their dartdoc;
    /// - keywords and constants;
    /// - class names, type arguments and function names;
    /// - strings and their interpolations;
    /// - operators, punctuation and annotations;
    /// - numbers.
    pub(super) fn matcher(&mut self, pat: Pat, p: usize) -> Option<usize> {
        match pat {
            Pat::Shebang => self.shebang(p),
            Pat::Directive => self.directive(p),
            Pat::DirectiveEnd | Pat::PunctuationAt1 => self.lit(p, b";"),
            Pat::DirectiveAt2 => self.word_in(p, &[b"as", b"show", b"hide"]),
            Pat::DirectiveAt3 => self.keyword(p, b"if"),

            Pat::CommentsAt0 => self.lit(p, b"/**/"),
            Pat::CommentsDocOldschool => self.lit(p, b"/**"),
            Pat::CommentsBlock => self.lit(p, b"/*"),
            Pat::CommentsDocOldschoolEnd | Pat::CommentsBlockEnd => self.lit(p, b"*/"),
            Pat::CommentsDoc => self.lit(p, b"///"),
            Pat::CommentsDocEnd => self.doc_comment_end(p),
            Pat::CommentsInlineAt1 => self.line_comment(p),
            Pat::DartdocAt0 => self.doc_reference(p),
            Pat::DartdocAt1 => self.fence_open(p, b"///"),
            Pat::DartdocAt1End => self.triple_fence_close(p),
            Pat::DartdocCodeblockTriple => self.triple_code_line(p),
            Pat::DartdocAt2 => self.fence_open(p, b"*"),
            Pat::DartdocAt2End => self.block_fence_close(p),
            Pat::DartdocCodeblockBlock => self.block_code_line(p),
            Pat::DartdocCodeblockTripleEnd | Pat::DartdocCodeblockBlockEnd => {
                self.at_eol(p).then_some(p)
            }
            Pat::DartdocAt3 => self.doc_code_span(p),
            Pat::DartdocAt4 => self.doc_indented_code(p),

            Pat::KeywordsAt0 => self.plain_word(p, &[b"as"]),
            Pat::KeywordsAt1 => self.plain_word(p, words::EXCEPTIONS),
            Pat::KeywordsAt2 => self.plain_word(p, words::CONTROL),
            Pat::KeywordsAt3 => self.asynchrony(p),
            Pat::KeywordsAt4 => self.plain_word(p, &[b"assert"]),
            Pat::KeywordsAt5 => self.new_keyword(p),
            Pat::KeywordsAt6 => self.plain_word(p, &[b"return"]),
            Pat::KeywordsAt7 => self.declaration(p),
            Pat::KeywordsAt8 => self.plain_word(p, words::MODIFIERS),
            Pat::KeywordsAt9 => self.plain_word(p, &[b"var"]),
            Pat::ConstantsAndSpecialVarsAt0 => self.plain_word(p, &[b"true", b"false", b"null"]),
            Pat::ConstantsAndSpecialVarsAt1 => {
                self.plain_word(p, &[b"this", b"super", b"augmented"])
            }

            Pat::ClassIdentifierWithOptionalFactoryMethodAt0 => {
                self.type_word(p, words::PRIMITIVES, names::Factory::Optional)
            }
            Pat::ClassIdentifierWithOptionalFactoryMethodAt1 => {
                self.type_word(p, &[b"void"], names::Factory::Optional)
            }
            Pat::ClassIdentifierWithOptionalFactoryMethodAt2 => {
                self.class_name(p, names::Factory::Optional)
            }
            Pat::ClassIdentifierAt0 => self.plain_word(p, words::PRIMITIVES),
            Pat::ClassIdentifierAt1 => self.plain_word(p, &[b"void"]),
            Pat::ClassIdentifierAt2 => self.class_name(p, names::Factory::Never),
            Pat::ClassIdentifierWithOptionalFactoryMethodAt2End | Pat::ClassIdentifierAt2End => {
                (self.at(p) != b'<').then_some(p)
            }
            Pat::TypeArgs => self.captured(p, b"<"),
            Pat::TypeArgsEnd => self.captured(p, b">"),
            Pat::TypeArgsAt1 | Pat::PunctuationAt0 => self.lit(p, b","),
            Pat::TypeArgsAt2 => self.lit(p, b"extends"),
            Pat::FunctionIdentifierAt0 => self.function_name(p),
            Pat::FunctionIdentifierAt1 => self.constructor_new(p),

            Pat::StringsAt0 => self.triple_open(p, b"\"\"\""),
            Pat::StringsAt1 => self.triple_open(p, b"'''"),
            Pat::StringsAt2 => self.lit(p, b"r\"\"\""),
            Pat::StringsAt3 => self.lit(p, b"r'''"),
            Pat::StringsAt0End | Pat::StringsAt2End => self.triple_close(p, b'"'),
            Pat::StringsAt1End | Pat::StringsAt3End => self.triple_close(p, b'\''),
            Pat::StringsAt4 => self.quote_open(p, b'"'),
            Pat::StringsAt6 => self.quote_open(p, b'\''),
            Pat::StringsAt5 => self.lit(p, b"r\""),
            Pat::StringsAt7 => self.lit(p, b"r'"),
            Pat::StringsAt4End | Pat::StringsAt5End => self.lit(p, b"\""),
            Pat::StringsAt6End | Pat::StringsAt7End => self.lit(p, b"'"),
            Pat::StringsAt4At0 | Pat::StringsAt5At0 | Pat::StringsAt6At0 | Pat::StringsAt7At0 => {
                self.line_feed(p)
            }
            Pat::StringInterpAt0 => self.interpolated_name(p),
            Pat::StringInterpAt1 => self.lit(p, b"${"),
            Pat::StringInterpAt1End | Pat::ExpressionAt5End => self.lit(p, b"}"),
            Pat::StringInterpAt2 => self.escape(p),
            Pat::ExpressionAt4 => {
                let end = self.skip(p, |b| b == b'_' || b.is_ascii_alphanumeric());
                (end > p).then_some(end)
            }
            Pat::ExpressionAt5 => self.lit(p, b"{"),

            Pat::OperatorsAt0 => self.is_operator(p),
            Pat::OperatorsAt1 => matches!(self.at(p), b':' | b'?').then_some(p + 1),
            Pat::OperatorsAt2 => self.bitwise(p),
            Pat::OperatorsAt3 => self.bitwise_assignment(p),
            Pat::OperatorsAt4 => self.lit(p, b"=>"),
            Pat::OperatorsAt5 => self.comparison(p),
            Pat::OperatorsAt6 => self.arithmetic_assignment(p),
            Pat::OperatorsAt7 => self.lit(p, b"="),
            Pat::OperatorsAt8 => self.lit(p, b"--").or_else(|| self.lit(p, b"++")),
            Pat::OperatorsAt9 => self.arithmetic(p),
            Pat::OperatorsAt10 => self.logical(p),
            Pat::PunctuationAt2 => self.lit(p, b"."),
            Pat::Annotations => self.annotation(p),

            Pat::ConstantsAndSpecialVarsAt2 => self.number(p),
        }
    }

    /// `(x)` for the literal `x`, with group 1 around it.
    fn captured(&mut self, p: usize, text: &[u8]) -> Option<usize> {
        let end = self.lit(p, text)?;
        self.cap(1, p, end);
        Some(end)
    }
}
