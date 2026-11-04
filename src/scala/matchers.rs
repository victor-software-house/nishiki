//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups the grammar names with [`Lexer::cap`].
//! The engine names them.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would.
//! Where a quantifier could give characters back, the doc comment says why it
//! cannot matter, or the matcher tries each end in order.

mod comments;
mod decls;
mod imports;
mod keywords;
mod names;
mod numbers;
mod strings;
mod xml;

pub(in crate::scala) use names::{is_name_start, is_op, is_upper_start};

use super::engine::Lexer;
use super::grammar::Pat;
use crate::rules::Chars;

impl Lexer<'_> {
    /// Whether `pat` matches at `p`, and where its match ends.
    ///
    /// The arms come in groups, in this order:
    /// - the `//> using` directive and the script header;
    /// - comments and Scaladoc;
    /// - constants and numbers;
    /// - strings and interpolation;
    /// - modifiers and declarations;
    /// - package, import and export rules;
    /// - XML literals;
    /// - keywords, `end` markers and operators;
    /// - quotes, character literals and punctuation.
    pub(super) fn matcher(&mut self, pat: Pat, p: usize) -> Option<usize> {
        match pat {
            Pat::UsingDirective => self.using_directive(p),
            Pat::UsingDirectiveEnd | Pat::CommentsAt1At0End => self.line_feed(p),
            Pat::UsingDirectiveAt2 => self.directive_value(p),
            Pat::ScriptHeader => self.script_header(p),

            Pat::BlockCommentsAt0 => self.lit(p, b"/**/"),
            Pat::BlockCommentsAt1 => self.scaladoc_begin(p),
            Pat::BlockCommentsAt1End | Pat::BlockCommentsAt2End => self.lit(p, b"*/"),
            Pat::BlockCommentsAt1At0 => self.doc_tag_name(p, &[b"@param"]),
            Pat::BlockCommentsAt1At1 => self.doc_tag_name(p, &[b"@tparam", b"@throws"]),
            Pat::BlockCommentsAt1At2 => self.doc_tag(p),
            Pat::BlockCommentsAt1At3 => self.doc_link(p),
            Pat::BlockCommentsAt2 => self.lit(p, b"/*"),
            Pat::CommentsAt1 => self.line_comment_lead(p),
            Pat::CommentsAt1End => self.not_anchor(p),
            Pat::CommentsAt1At0 => self.lit(p, b"//"),

            Pat::ConstantsAt0 => self.word_in(p, &[b"false", b"null", b"true"]),
            Pat::ConstantsAt1 => self.hex(p),
            Pat::ConstantsAt2 => self.decimal(p),
            Pat::ConstantsAt3 => self.point_decimal(p),
            Pat::ConstantsAt4 => self.binary(p),
            Pat::ConstantsAt5 => self.word_in(p, &[b"this", b"super"]),

            Pat::StringsAt0 => self.lit(p, b"\"\"\""),
            Pat::StringsAt0End => self.triple_quote_end(p),
            Pat::StringsAt0At0 | Pat::StringsAt2At1 => self.triple_escape(p),
            Pat::StringsAt1 => self.raw_string(p, b"\"\"\""),
            Pat::StringsAt1End | Pat::StringsAt2End => self.interpolated_string_end(p, b"\"\"\""),
            Pat::StringsAt1At0 | Pat::StringsAt4At0 | Pat::StringsAt5At0 => self.dollar_escape(p),
            Pat::StringsAt1At2 | Pat::StringsAt2At2 | Pat::StringsAt4At2 | Pat::StringsAt5At4 => {
                self.any_char(p)
            }
            Pat::StringsAt2 => self.interpolated_string(p, b"\"\"\""),
            Pat::StringsAt3
            | Pat::StringsAt3End
            | Pat::XmlDoublequotedString
            | Pat::XmlDoublequotedStringEnd => self.lit_byte(p, b'"'),
            Pat::StringsAt3At0 | Pat::StringsAt5At2 | Pat::CharLiteralAt1At0 => self.escape(p),
            Pat::StringsAt3At1 | Pat::StringsAt5At3 | Pat::CharLiteralAt1At1 => self.any_escape(p),
            Pat::StringsAt4 => self.raw_string(p, b"\""),
            Pat::StringsAt4End | Pat::StringsAt5End => self.interpolated_string_end(p, b"\""),
            Pat::StringsAt5 => self.interpolated_string(p, b"\""),
            Pat::StringInterpolationAt0 => self.lit(p, b"$$"),
            Pat::StringInterpolationAt1 => self.interpolated_name(p),
            Pat::StringInterpolationAt2 => self.lit(p, b"${"),

            Pat::StorageModifiersAt0 => self.access_modifier(p),
            Pat::StorageModifiersAt1 => self.other_modifier(p),
            Pat::StorageModifiersAt2 => self.soft_modifier(p),
            Pat::DeclarationsAt0 => self.named_declaration(p, b"def"),
            Pat::DeclarationsAt1 => self.named_declaration(p, b"trait"),
            Pat::DeclarationsAt2 => self.class_declaration(p),
            Pat::DeclarationsAt3 => self.type_declaration(p),
            Pat::DeclarationsAt4 => self.val_or_var_call(p),
            Pat::DeclarationsAt5 => self.val_or_var(p, b"val"),
            Pat::DeclarationsAt6 => self.val_or_var(p, b"var"),
            Pat::DeclarationsAt7 => self.package_object(p),
            Pat::DeclarationsAt9 => self.given_declaration(p),
            Pat::InheritanceAt0 => self.inheritance(p),
            Pat::ExtensionAt0 => self.extension(p),
            Pat::Initialization => self.new_keyword(p),
            Pat::InlineAt0 => self.inline_modifier(p),
            Pat::InlineAt1 => self.inline_control(p),
            Pat::NamedBoundsAt0 => self.named_bound(p),
            Pat::UsingAt0 => self.using_clause(p),
            Pat::ParameterListAt0 => self.parameter(p),
            Pat::QualifiedClassName => self.qualified_class_name(p),
            Pat::SingletonType => self.singleton_type(p),

            Pat::DeclarationsAt8 => self.keyword_and_blanks(p, b"package"),
            Pat::Imports => self.keyword_and_blanks(p, b"import"),
            Pat::Exports => self.keyword_and_blanks(p, b"export"),
            Pat::DeclarationsAt8End | Pat::ImportsEnd | Pat::ExportsEnd => {
                self.after_semicolon_or_line_feed(p)
            }
            Pat::DeclarationsAt8At1 | Pat::ImportsAt4 | Pat::ExportsAt3 => self.path_name(p),
            Pat::DeclarationsAt8At2 | Pat::ImportsAt5 | Pat::ExportsAt4 => self.lit_byte(p, b'.'),
            Pat::ImportsAt1 | Pat::ImportsAt6At1 | Pat::ExportsAt1 | Pat::ExportsAt5At1 => {
                self.word(p, b"given")
            }
            Pat::ImportsAt2 => self.import_as(p),
            Pat::ImportsAt3 | Pat::ExportsAt2 => self.upper_name(p),
            Pat::ImportsAt6
            | Pat::ExportsAt5
            | Pat::XmlEmbeddedContentAt0
            | Pat::CurlyBraces
            | Pat::MetaBracketsAt0 => self.lit_byte(p, b'{'),
            Pat::ImportsAt6End
            | Pat::ExportsAt5End
            | Pat::StringInterpolationAt2End
            | Pat::XmlEmbeddedContentAt0End
            | Pat::CurlyBracesEnd
            | Pat::MetaBracketsAt1 => self.lit_byte(p, b'}'),
            Pat::ImportsAt6At0 | Pat::ExportsAt5At0 => self.renaming(p),
            Pat::ImportsAt6At2 | Pat::ExportsAt5At2 => self.selector(p),

            Pat::XmlLiteralAt0 => self.empty_element(p),
            Pat::XmlLiteralAt0End => self.empty_element_end(p),
            Pat::XmlLiteralAt1 => self.tag_open(p),
            Pat::XmlLiteralAt1End => self.tag_close(p),
            Pat::XmlEmbeddedContentAt1 => self.attribute(p),
            Pat::XmlSinglequotedString | Pat::XmlSinglequotedStringEnd | Pat::CharLiteralAt1 => {
                self.lit_byte(p, b'\'')
            }
            Pat::XmlEntity => self.entity(p),

            Pat::KeywordsAt0 => self.word_in(p, &[b"return", b"throw"]),
            Pat::KeywordsAt1 => self.word_in(p, &[b"classOf", b"isInstanceOf", b"asInstanceOf"]),
            Pat::KeywordsAt2 => self.word_in(
                p,
                &[
                    b"else", b"if", b"then", b"do", b"while", b"for", b"yield", b"match", b"case",
                ],
            ),
            Pat::KeywordsAt3 => self.end_of(p, &[b"if", b"while", b"for", b"match"]),
            Pat::KeywordsAt4 => self.end_of(p, &[b"val"]),
            Pat::KeywordsAt5 => self.end_of(p, &[b"var"]),
            Pat::KeywordsAt6 => self.end_of_type(p),
            Pat::KeywordsAt7 => self.word_in(p, &[b"catch", b"finally", b"try"]),
            Pat::KeywordsAt8 => self.end_of(p, &[b"try"]),
            Pat::KeywordsAt9 => self.end_of_name(p),
            Pat::KeywordsAt10 => self.long_operator(p),
            Pat::KeywordsAt11 => self.pair_operator(p),
            Pat::KeywordsAt12 => self.single_operator(p),

            Pat::ScalaQuotedOrSymbolAt0 => self.quoted_symbol(p),
            Pat::ScalaQuotedOrSymbolAt1 => self.staging_quote(p, b'{'),
            Pat::ScalaQuotedOrSymbolAt2 => self.staging_quote(p, b'['),
            Pat::ScalaQuotedOrSymbolAt3 => self.staging_splice(p),
            Pat::CharLiteralAt0 => self.quote_char(p),
            Pat::CharLiteralAt1End => self.char_literal_end(p),
            Pat::CharLiteralAt1At2 => self.not_quotes(p),
            Pat::CharLiteralAt1At3 => self.not_quote(p),
            Pat::EmptyParentheses => self.empty_parentheses(p),
            Pat::BackQuotedVariable => self.quoted_name(p),
            Pat::MetaBracketsAt2 => self.bracket(p),
            Pat::MetaBounds => self.bounds(p),
            Pat::MetaColonsAt0 => self.lone_colon(p),
        }
    }
}
