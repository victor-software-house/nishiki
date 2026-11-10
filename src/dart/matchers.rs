//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups the grammar names with
//! [`Lexer::capture`]. The engine names them.
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
use super::grammar::Pattern;
use crate::rules::Chars;

/// Whether `b` is in the grammar's `[$0-9A-Z_a-z]`.
fn is_ident(b: u8) -> bool {
    b == b'$' || b == b'_' || b.is_ascii_alphanumeric()
}

impl Lexer<'_> {
    /// Whether `pattern` matches at `p`, and where its match ends.
    ///
    /// The arms come in groups, in this order:
    /// - the script line and the import directives;
    /// - comments and their dartdoc;
    /// - keywords and constants;
    /// - class names, type arguments and function names;
    /// - strings and their interpolations;
    /// - operators, punctuation and annotations;
    /// - numbers.
    pub(super) fn matcher(&mut self, pattern: Pattern, p: usize) -> Option<usize> {
        match pattern {
            Pattern::Shebang => self.shebang(p),
            Pattern::Directive => self.directive(p),
            Pattern::DirectiveEnd | Pattern::PunctuationAt1 => self.lit(p, b";"),
            Pattern::DirectiveAt2 => self.word_in(p, &[b"as", b"show", b"hide"]),
            Pattern::DirectiveAt3 => self.keyword(p, b"if"),

            Pattern::CommentsAt0 => self.lit(p, b"/**/"),
            Pattern::CommentsDocOldschool => self.lit(p, b"/**"),
            Pattern::CommentsBlock => self.lit(p, b"/*"),
            Pattern::CommentsDocOldschoolEnd | Pattern::CommentsBlockEnd => self.lit(p, b"*/"),
            Pattern::CommentsDoc => self.lit(p, b"///"),
            Pattern::CommentsDocEnd => self.doc_comment_end(p),
            Pattern::CommentsInlineAt1 => self.line_comment(p),
            Pattern::DartdocAt0 => self.doc_reference(p),
            Pattern::DartdocAt1 => self.fence_open(p, b"///"),
            Pattern::DartdocAt1End => self.triple_fence_close(p),
            Pattern::DartdocCodeblockTriple => self.triple_code_line(p),
            Pattern::DartdocAt2 => self.fence_open(p, b"*"),
            Pattern::DartdocAt2End => self.block_fence_close(p),
            Pattern::DartdocCodeblockBlock => self.block_code_line(p),
            Pattern::DartdocCodeblockTripleEnd | Pattern::DartdocCodeblockBlockEnd => {
                self.at_eol(p).then_some(p)
            }
            Pattern::DartdocAt3 => self.doc_code_span(p),
            Pattern::DartdocAt4 => self.doc_indented_code(p),

            Pattern::KeywordsAt0 => self.plain_word(p, &[b"as"]),
            Pattern::KeywordsAt1 => self.plain_word(p, words::EXCEPTIONS),
            Pattern::KeywordsAt2 => self.plain_word(p, words::CONTROL),
            Pattern::KeywordsAt3 => self.asynchrony(p),
            Pattern::KeywordsAt4 => self.plain_word(p, &[b"assert"]),
            Pattern::KeywordsAt5 => self.new_keyword(p),
            Pattern::KeywordsAt6 => self.plain_word(p, &[b"return"]),
            Pattern::KeywordsAt7 => self.declaration(p),
            Pattern::KeywordsAt8 => self.plain_word(p, words::MODIFIERS),
            Pattern::KeywordsAt9 => self.plain_word(p, &[b"var"]),
            Pattern::ConstantsAndSpecialVarsAt0 => {
                self.plain_word(p, &[b"true", b"false", b"null"])
            }
            Pattern::ConstantsAndSpecialVarsAt1 => {
                self.plain_word(p, &[b"this", b"super", b"augmented"])
            }

            Pattern::ClassIdentifierWithOptionalFactoryMethodAt0 => {
                self.type_word(p, words::PRIMITIVES, names::Factory::Optional)
            }
            Pattern::ClassIdentifierWithOptionalFactoryMethodAt1 => {
                self.type_word(p, &[b"void"], names::Factory::Optional)
            }
            Pattern::ClassIdentifierWithOptionalFactoryMethodAt2 => {
                self.class_name(p, names::Factory::Optional)
            }
            Pattern::ClassIdentifierAt0 => self.plain_word(p, words::PRIMITIVES),
            Pattern::ClassIdentifierAt1 => self.plain_word(p, &[b"void"]),
            Pattern::ClassIdentifierAt2 => self.class_name(p, names::Factory::Never),
            Pattern::ClassIdentifierWithOptionalFactoryMethodAt2End
            | Pattern::ClassIdentifierAt2End => (self.at(p) != b'<').then_some(p),
            Pattern::TypeArgs => self.captured(p, b"<"),
            Pattern::TypeArgsEnd => self.captured(p, b">"),
            Pattern::TypeArgsAt1 | Pattern::PunctuationAt0 => self.lit(p, b","),
            Pattern::TypeArgsAt2 => self.lit(p, b"extends"),
            Pattern::FunctionIdentifierAt0 => self.function_name(p),
            Pattern::FunctionIdentifierAt1 => self.constructor_new(p),

            Pattern::StringsAt0 => self.triple_open(p, b"\"\"\""),
            Pattern::StringsAt1 => self.triple_open(p, b"'''"),
            Pattern::StringsAt2 => self.lit(p, b"r\"\"\""),
            Pattern::StringsAt3 => self.lit(p, b"r'''"),
            Pattern::StringsAt0End | Pattern::StringsAt2End => self.triple_close(p, b'"'),
            Pattern::StringsAt1End | Pattern::StringsAt3End => self.triple_close(p, b'\''),
            Pattern::StringsAt4 => self.quote_open(p, b'"'),
            Pattern::StringsAt6 => self.quote_open(p, b'\''),
            Pattern::StringsAt5 => self.lit(p, b"r\""),
            Pattern::StringsAt7 => self.lit(p, b"r'"),
            Pattern::StringsAt4End | Pattern::StringsAt5End => self.lit(p, b"\""),
            Pattern::StringsAt6End | Pattern::StringsAt7End => self.lit(p, b"'"),
            Pattern::StringsAt4At0
            | Pattern::StringsAt5At0
            | Pattern::StringsAt6At0
            | Pattern::StringsAt7At0 => self.line_feed(p),
            Pattern::StringInterpAt0 => self.interpolated_name(p),
            Pattern::StringInterpAt1 => self.lit(p, b"${"),
            Pattern::StringInterpAt1End | Pattern::ExpressionAt5End => self.lit(p, b"}"),
            Pattern::StringInterpAt2 => self.escape(p),
            Pattern::ExpressionAt4 => {
                let end = self.skip(p, |b| b == b'_' || b.is_ascii_alphanumeric());
                (end > p).then_some(end)
            }
            Pattern::ExpressionAt5 => self.lit(p, b"{"),

            Pattern::OperatorsAt0 => self.is_operator(p),
            Pattern::OperatorsAt1 => matches!(self.at(p), b':' | b'?').then_some(p + 1),
            Pattern::OperatorsAt2 => self.bitwise(p),
            Pattern::OperatorsAt3 => self.bitwise_assignment(p),
            Pattern::OperatorsAt4 => self.lit(p, b"=>"),
            Pattern::OperatorsAt5 => self.comparison(p),
            Pattern::OperatorsAt6 => self.arithmetic_assignment(p),
            Pattern::OperatorsAt7 => self.lit(p, b"="),
            Pattern::OperatorsAt8 => self.lit(p, b"--").or_else(|| self.lit(p, b"++")),
            Pattern::OperatorsAt9 => self.arithmetic(p),
            Pattern::OperatorsAt10 => self.logical(p),
            Pattern::PunctuationAt2 => self.lit(p, b"."),
            Pattern::Annotations => self.annotation(p),

            Pattern::ConstantsAndSpecialVarsAt2 => self.number(p),
        }
    }

    /// `(x)` for the literal `x`, with group 1 around it.
    fn captured(&mut self, p: usize, text: &[u8]) -> Option<usize> {
        let end = self.lit(p, text)?;
        self.capture(1, p, end);
        Some(end)
    }
}
