//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where. A matcher records the
//! capture groups the grammar names with [`Lexer::cap`]; the engine names them.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would. Where a quantifier could give characters back, the doc comment says
//! why it cannot matter.

mod annotations;
mod comments;
mod decls;
mod groups;
mod numbers;
mod operators;
mod strings;
mod words;

use super::engine::Lexer;
use super::grammar::Pat;
use crate::rules::Chars;

impl Lexer<'_> {
    /// Whether `pat` matches at `p`, and where its match ends.
    pub(super) fn matcher(&mut self, pat: Pat, p: usize) -> Option<usize> {
        match pat {
            // Package, import and their names.
            Pat::Import => self.directive(p, b"import"),
            Pat::Package => self.directive(p, b"package"),
            Pat::ImportEnd | Pat::PackageEnd => self.semicolon_or_end(p),
            Pat::ImportAt2 => self.lit(p, b"*"),

            // Comments.
            Pat::CommentLine => self.lit(p, b"//"),
            Pat::CommentLineEnd | Pat::AnnotationSiteEnd => self.at_eol(p).then_some(p),
            Pat::CommentBlock => self.block_comment_begin(p),
            Pat::CommentBlockEnd | Pat::CommentJavadocAt0End => self.lit(p, b"*/"),
            Pat::CommentJavadocAt0 => self.lit(p, b"/**"),
            Pat::CommentJavadocAt0At0 => self.javadoc_tag(p),
            Pat::CommentJavadocAt0At1 => self.javadoc_named(p, &[b"@param", b"@property"]),
            Pat::CommentJavadocAt0At2 => self.javadoc_bracketed(p),
            Pat::CommentJavadocAt0At3 => self.javadoc_named(p, &[b"@exception", b"@throws"]),
            Pat::CommentJavadocAt0At4 => self.javadoc_link(p),

            // Keywords.
            Pat::PrefixModifiers => self.word_in(p, words::PREFIX_MODIFIERS),
            Pat::PostfixModifiers => self.word_in(p, words::POSTFIX_MODIFIERS),
            Pat::SoftKeywords => self.word_in(p, words::SOFT_KEYWORDS),
            Pat::HardKeywords => self.word_in(p, words::HARD_KEYWORDS),
            Pat::ControlKeywords => self.word_in(p, words::CONTROL_KEYWORDS),
            Pat::SelfReference => self.self_reference(p),
            Pat::BooleanLiteral => self.word_in(p, &[b"true", b"false"]),
            Pat::NullLiteral => self.word_in(p, &[b"null"]),

            // Annotations.
            Pat::AnnotationSimple => self.annotation_simple(p),
            Pat::AnnotationSiteList => self.annotation_site(p, annotations::SiteForm::List),
            Pat::AnnotationSite => self.annotation_site(p, annotations::SiteForm::Single),
            Pat::AnnotationSiteListEnd => self.lit(p, b"]"),
            Pat::UnescapedAnnotation => self.unescaped_annotation(p),

            // Declarations and names.
            Pat::ClassDeclaration => self.class_declaration(p),
            Pat::Object => self.object(p),
            Pat::TypeAlias => self.type_alias(p),
            Pat::Function => self.function(p),
            Pat::VariableDeclaration => self.variable_declaration(p),
            Pat::TypeAnnotation => self.type_annotation(p),
            Pat::FunctionCall => self.function_call(p),
            Pat::MethodReference => self.method_reference(p),
            Pat::Key => self.key(p),

            // Strings.
            Pat::String => self.string_begin(p),
            Pat::StringEnd => self.lit(p, b"\""),
            Pat::StringEmpty => self.string_empty(p),
            Pat::StringAt0 | Pat::StringMultilineAt0 | Pat::CharacterAt0 => self.escape(p),
            Pat::StringEscapeSimple => self.template_variable(p),
            Pat::StringEscapeBracketed => self.template_begin(p),
            Pat::StringEscapeBracketedEnd => self.template_end(p),
            Pat::StringMultiline | Pat::StringMultilineEnd => self.lit(p, b"\"\"\""),
            Pat::Character | Pat::CharacterEnd => self.lit(p, b"'"),

            // Operators.
            Pat::LambdaArrow => self.lit(p, b"->"),
            Pat::OperatorsAt0 => self.comparison(p),
            Pat::OperatorsAt1 => self.arithmetic_assignment(p),
            Pat::OperatorsAt2 => self.lit(p, b"="),
            Pat::OperatorsAt3 => self.arithmetic(p),
            Pat::OperatorsAt4 => self.logical(p),
            Pat::OperatorsAt5 => self.increment(p),
            Pat::OperatorsAt6 => self.lit(p, b".."),

            // Numbers.
            Pat::DecimalLiteral => self.decimal(p),
            Pat::HexLiteral => self.hex(p),
            Pat::BinaryLiteral => self.binary(p),
        }
    }
}
