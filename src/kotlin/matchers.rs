//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups the grammar names with [`Lexer::cap`].
//! The engine names them.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would.
//! Where a quantifier could give characters back, the doc comment says why it
//! cannot matter.

mod annotations;
mod comments;
mod decls;
mod groups;
mod numbers;
mod operators;
mod strings;
mod words;

use super::engine::Lexer;
use super::grammar::Pattern;
use crate::rules::Chars;

impl Lexer<'_> {
    /// Whether `pattern` matches at `p`, and where its match ends.
    ///
    /// The arms come in groups, in this order:
    /// - package, import and their names;
    /// - comments;
    /// - keywords;
    /// - annotations;
    /// - declarations and names;
    /// - strings;
    /// - operators;
    /// - numbers.
    pub(super) fn matcher(&mut self, pattern: Pattern, p: usize) -> Option<usize> {
        match pattern {
            Pattern::Import => self.directive(p, b"import"),
            Pattern::Package => self.directive(p, b"package"),
            Pattern::ImportEnd | Pattern::PackageEnd => self.semicolon_or_end(p),
            Pattern::ImportAt2 => self.lit(p, b"*"),

            Pattern::CommentLine => self.lit(p, b"//"),
            Pattern::CommentLineEnd | Pattern::AnnotationSiteEnd => self.at_eol(p).then_some(p),
            Pattern::CommentBlock => self.block_comment_begin(p),
            Pattern::CommentBlockEnd | Pattern::CommentJavadocAt0End => self.lit(p, b"*/"),
            Pattern::CommentJavadocAt0 => self.lit(p, b"/**"),
            Pattern::CommentJavadocAt0At0 => self.javadoc_tag(p),
            Pattern::CommentJavadocAt0At1 => self.javadoc_named(p, &[b"@param", b"@property"]),
            Pattern::CommentJavadocAt0At2 => self.javadoc_bracketed(p),
            Pattern::CommentJavadocAt0At3 => self.javadoc_named(p, &[b"@exception", b"@throws"]),
            Pattern::CommentJavadocAt0At4 => self.javadoc_link(p),

            Pattern::PrefixModifiers => self.word_in(p, words::PREFIX_MODIFIERS),
            Pattern::PostfixModifiers => self.word_in(p, words::POSTFIX_MODIFIERS),
            Pattern::SoftKeywords => self.word_in(p, words::SOFT_KEYWORDS),
            Pattern::HardKeywords => self.word_in(p, words::HARD_KEYWORDS),
            Pattern::ControlKeywords => self.word_in(p, words::CONTROL_KEYWORDS),
            Pattern::SelfReference => self.self_reference(p),
            Pattern::BooleanLiteral => self.word_in(p, &[b"true", b"false"]),
            Pattern::NullLiteral => self.word_in(p, &[b"null"]),

            Pattern::AnnotationSimple => self.annotation_simple(p),
            Pattern::AnnotationSiteList => self.annotation_site(p, annotations::SiteForm::List),
            Pattern::AnnotationSite => self.annotation_site(p, annotations::SiteForm::Single),
            Pattern::AnnotationSiteListEnd => self.lit(p, b"]"),
            Pattern::UnescapedAnnotation => self.unescaped_annotation(p),

            Pattern::ClassDeclaration => self.class_declaration(p),
            Pattern::Object => self.object(p),
            Pattern::TypeAlias => self.type_alias(p),
            Pattern::Function => self.function(p),
            Pattern::VariableDeclaration => self.variable_declaration(p),
            Pattern::TypeAnnotation => self.type_annotation(p),
            Pattern::FunctionCall => self.function_call(p),
            Pattern::MethodReference => self.method_reference(p),
            Pattern::Key => self.key(p),

            Pattern::String => self.string_begin(p),
            Pattern::StringEnd => self.lit(p, b"\""),
            Pattern::StringEmpty => self.string_empty(p),
            Pattern::StringAt0 | Pattern::StringMultilineAt0 | Pattern::CharacterAt0 => {
                self.escape(p)
            }
            Pattern::StringEscapeSimple => self.template_variable(p),
            Pattern::StringEscapeBracketed => self.template_begin(p),
            Pattern::StringEscapeBracketedEnd => self.template_end(p),
            Pattern::StringMultiline | Pattern::StringMultilineEnd => self.lit(p, b"\"\"\""),
            Pattern::Character | Pattern::CharacterEnd => self.lit(p, b"'"),

            Pattern::LambdaArrow => self.lit(p, b"->"),
            Pattern::OperatorsAt0 => self.comparison(p),
            Pattern::OperatorsAt1 => self.arithmetic_assignment(p),
            Pattern::OperatorsAt2 => self.lit(p, b"="),
            Pattern::OperatorsAt3 => self.arithmetic(p),
            Pattern::OperatorsAt4 => self.logical(p),
            Pattern::OperatorsAt5 => self.increment(p),
            Pattern::OperatorsAt6 => self.lit(p, b".."),

            Pattern::DecimalLiteral => self.decimal(p),
            Pattern::HexLiteral => self.hex(p),
            Pattern::BinaryLiteral => self.binary(p),
        }
    }
}
