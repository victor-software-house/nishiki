//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups the grammar names with
//! [`Lexer::capture`]. The engine names them.
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
use super::grammar::Pattern;
use crate::rules::Chars;

impl Lexer<'_> {
    /// Whether `pattern` matches at `p`, and where its match ends.
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
    pub(super) fn matcher(&mut self, pattern: Pattern, p: usize) -> Option<usize> {
        match pattern {
            Pattern::UsingDirective => self.using_directive(p),
            Pattern::UsingDirectiveEnd | Pattern::CommentsAt1At0End => self.line_feed(p),
            Pattern::UsingDirectiveAt2 => self.directive_value(p),
            Pattern::ScriptHeader => self.script_header(p),

            Pattern::BlockCommentsAt0 => self.lit(p, b"/**/"),
            Pattern::BlockCommentsAt1 => self.scaladoc_begin(p),
            Pattern::BlockCommentsAt1End | Pattern::BlockCommentsAt2End => self.lit(p, b"*/"),
            Pattern::BlockCommentsAt1At0 => self.doc_tag_name(p, &[b"@param"]),
            Pattern::BlockCommentsAt1At1 => self.doc_tag_name(p, &[b"@tparam", b"@throws"]),
            Pattern::BlockCommentsAt1At2 => self.doc_tag(p),
            Pattern::BlockCommentsAt1At3 => self.doc_link(p),
            Pattern::BlockCommentsAt2 => self.lit(p, b"/*"),
            Pattern::CommentsAt1 => self.line_comment_lead(p),
            Pattern::CommentsAt1End => self.not_anchor(p),
            Pattern::CommentsAt1At0 => self.lit(p, b"//"),

            Pattern::ConstantsAt0 => self.word_in(p, &[b"false", b"null", b"true"]),
            Pattern::ConstantsAt1 => self.hex(p),
            Pattern::ConstantsAt2 => self.decimal(p),
            Pattern::ConstantsAt3 => self.point_decimal(p),
            Pattern::ConstantsAt4 => self.binary(p),
            Pattern::ConstantsAt5 => self.word_in(p, &[b"this", b"super"]),

            Pattern::StringsAt0 => self.lit(p, b"\"\"\""),
            Pattern::StringsAt0End => self.triple_quote_end(p),
            Pattern::StringsAt0At0 | Pattern::StringsAt2At1 => self.triple_escape(p),
            Pattern::StringsAt1 => self.raw_string(p, b"\"\"\""),
            Pattern::StringsAt1End | Pattern::StringsAt2End => {
                self.interpolated_string_end(p, b"\"\"\"")
            }
            Pattern::StringsAt1At0 | Pattern::StringsAt4At0 | Pattern::StringsAt5At0 => {
                self.dollar_escape(p)
            }
            Pattern::StringsAt1At2
            | Pattern::StringsAt2At2
            | Pattern::StringsAt4At2
            | Pattern::StringsAt5At4 => self.any_char(p),
            Pattern::StringsAt2 => self.interpolated_string(p, b"\"\"\""),
            Pattern::StringsAt3
            | Pattern::StringsAt3End
            | Pattern::XmlDoublequotedString
            | Pattern::XmlDoublequotedStringEnd => self.lit_byte(p, b'"'),
            Pattern::StringsAt3At0 | Pattern::StringsAt5At2 | Pattern::CharLiteralAt1At0 => {
                self.escape(p)
            }
            Pattern::StringsAt3At1 | Pattern::StringsAt5At3 | Pattern::CharLiteralAt1At1 => {
                self.any_escape(p)
            }
            Pattern::StringsAt4 => self.raw_string(p, b"\""),
            Pattern::StringsAt4End | Pattern::StringsAt5End => {
                self.interpolated_string_end(p, b"\"")
            }
            Pattern::StringsAt5 => self.interpolated_string(p, b"\""),
            Pattern::StringInterpolationAt0 => self.lit(p, b"$$"),
            Pattern::StringInterpolationAt1 => self.interpolated_name(p),
            Pattern::StringInterpolationAt2 => self.lit(p, b"${"),

            Pattern::StorageModifiersAt0 => self.access_modifier(p),
            Pattern::StorageModifiersAt1 => self.other_modifier(p),
            Pattern::StorageModifiersAt2 => self.soft_modifier(p),
            Pattern::DeclarationsAt0 => self.named_declaration(p, b"def"),
            Pattern::DeclarationsAt1 => self.named_declaration(p, b"trait"),
            Pattern::DeclarationsAt2 => self.class_declaration(p),
            Pattern::DeclarationsAt3 => self.type_declaration(p),
            Pattern::DeclarationsAt4 => self.val_or_var_call(p),
            Pattern::DeclarationsAt5 => self.val_or_var(p, b"val"),
            Pattern::DeclarationsAt6 => self.val_or_var(p, b"var"),
            Pattern::DeclarationsAt7 => self.package_object(p),
            Pattern::DeclarationsAt9 => self.given_declaration(p),
            Pattern::InheritanceAt0 => self.inheritance(p),
            Pattern::ExtensionAt0 => self.extension(p),
            Pattern::Initialization => self.new_keyword(p),
            Pattern::InlineAt0 => self.inline_modifier(p),
            Pattern::InlineAt1 => self.inline_control(p),
            Pattern::NamedBoundsAt0 => self.named_bound(p),
            Pattern::UsingAt0 => self.using_clause(p),
            Pattern::ParameterListAt0 => self.parameter(p),
            Pattern::QualifiedClassName => self.qualified_class_name(p),
            Pattern::SingletonType => self.singleton_type(p),

            Pattern::DeclarationsAt8 => self.keyword_and_blanks(p, b"package"),
            Pattern::Imports => self.keyword_and_blanks(p, b"import"),
            Pattern::Exports => self.keyword_and_blanks(p, b"export"),
            Pattern::DeclarationsAt8End | Pattern::ImportsEnd | Pattern::ExportsEnd => {
                self.after_semicolon_or_line_feed(p)
            }
            Pattern::DeclarationsAt8At1 | Pattern::ImportsAt4 | Pattern::ExportsAt3 => {
                self.path_name(p)
            }
            Pattern::DeclarationsAt8At2 | Pattern::ImportsAt5 | Pattern::ExportsAt4 => {
                self.lit_byte(p, b'.')
            }
            Pattern::ImportsAt1
            | Pattern::ImportsAt6At1
            | Pattern::ExportsAt1
            | Pattern::ExportsAt5At1 => self.word(p, b"given"),
            Pattern::ImportsAt2 => self.import_as(p),
            Pattern::ImportsAt3 | Pattern::ExportsAt2 => self.upper_name(p),
            Pattern::ImportsAt6
            | Pattern::ExportsAt5
            | Pattern::XmlEmbeddedContentAt0
            | Pattern::CurlyBraces
            | Pattern::MetaBracketsAt0 => self.lit_byte(p, b'{'),
            Pattern::ImportsAt6End
            | Pattern::ExportsAt5End
            | Pattern::StringInterpolationAt2End
            | Pattern::XmlEmbeddedContentAt0End
            | Pattern::CurlyBracesEnd
            | Pattern::MetaBracketsAt1 => self.lit_byte(p, b'}'),
            Pattern::ImportsAt6At0 | Pattern::ExportsAt5At0 => self.renaming(p),
            Pattern::ImportsAt6At2 | Pattern::ExportsAt5At2 => self.selector(p),

            Pattern::XmlLiteralAt0 => self.empty_element(p),
            Pattern::XmlLiteralAt0End => self.empty_element_end(p),
            Pattern::XmlLiteralAt1 => self.tag_open(p),
            Pattern::XmlLiteralAt1End => self.tag_close(p),
            Pattern::XmlEmbeddedContentAt1 => self.attribute(p),
            Pattern::XmlSinglequotedString
            | Pattern::XmlSinglequotedStringEnd
            | Pattern::CharLiteralAt1 => self.lit_byte(p, b'\''),
            Pattern::XmlEntity => self.entity(p),

            Pattern::KeywordsAt0 => self.word_in(p, &[b"return", b"throw"]),
            Pattern::KeywordsAt1 => {
                self.word_in(p, &[b"classOf", b"isInstanceOf", b"asInstanceOf"])
            }
            Pattern::KeywordsAt2 => self.word_in(
                p,
                &[
                    b"else", b"if", b"then", b"do", b"while", b"for", b"yield", b"match", b"case",
                ],
            ),
            Pattern::KeywordsAt3 => self.end_of(p, &[b"if", b"while", b"for", b"match"]),
            Pattern::KeywordsAt4 => self.end_of(p, &[b"val"]),
            Pattern::KeywordsAt5 => self.end_of(p, &[b"var"]),
            Pattern::KeywordsAt6 => self.end_of_type(p),
            Pattern::KeywordsAt7 => self.word_in(p, &[b"catch", b"finally", b"try"]),
            Pattern::KeywordsAt8 => self.end_of(p, &[b"try"]),
            Pattern::KeywordsAt9 => self.end_of_name(p),
            Pattern::KeywordsAt10 => self.long_operator(p),
            Pattern::KeywordsAt11 => self.pair_operator(p),
            Pattern::KeywordsAt12 => self.single_operator(p),

            Pattern::ScalaQuotedOrSymbolAt0 => self.quoted_symbol(p),
            Pattern::ScalaQuotedOrSymbolAt1 => self.staging_quote(p, b'{'),
            Pattern::ScalaQuotedOrSymbolAt2 => self.staging_quote(p, b'['),
            Pattern::ScalaQuotedOrSymbolAt3 => self.staging_splice(p),
            Pattern::CharLiteralAt0 => self.quote_char(p),
            Pattern::CharLiteralAt1End => self.char_literal_end(p),
            Pattern::CharLiteralAt1At2 => self.not_quotes(p),
            Pattern::CharLiteralAt1At3 => self.not_quote(p),
            Pattern::EmptyParentheses => self.empty_parentheses(p),
            Pattern::BackQuotedVariable => self.quoted_name(p),
            Pattern::MetaBracketsAt2 => self.bracket(p),
            Pattern::MetaBounds => self.bounds(p),
            Pattern::MetaColonsAt0 => self.lone_colon(p),
        }
    }
}
