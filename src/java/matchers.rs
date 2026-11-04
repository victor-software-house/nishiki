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
mod chars;
mod code;
mod comments;
mod decls;
mod imports;
mod keywords;
mod lookaheads;
mod names;
mod numbers;
mod types;
mod variables;

use code::{Escapes, Property};
use decls::Callable;
use numbers::Radix;

use super::engine::Lexer;
use super::grammar::Pat;
use crate::rules::Chars;

impl Lexer<'_> {
    /// Whether `pat` matches at `p`, and where its match ends.
    ///
    /// The arms come in groups, in this order:
    /// - package, import and their names;
    /// - comments;
    /// - annotations;
    /// - enums;
    /// - generics and types;
    /// - brackets;
    /// - classes;
    /// - records;
    /// - statements;
    /// - constants and numbers;
    /// - expressions;
    /// - keywords and operators;
    /// - variables and names;
    /// - strings;
    /// - members;
    /// - modules.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm for each expression of the grammar"
    )]
    pub(super) fn matcher(&mut self, pat: Pat, p: usize) -> Option<usize> {
        match pat {
            Pat::RootAt0 => self.package_begin(p),
            Pat::RootAt0End | Pat::RootAt1End => self.terminator(p),
            Pat::RootAt0At1 | Pat::RootAt1At1 => self.misplaced_dot(p),
            Pat::RootAt0At2 | Pat::RootAt1At3 => self.misplaced_name_part(p),
            Pat::RootAt0At3 => self.capitals(p),
            Pat::RootAt0At4 | Pat::RootAt1At4 => self.reserved_word(p),
            Pat::RootAt0At5 | Pat::RootAt1At5 | Pat::CodeAt24 | Pat::GenericsAt8 => {
                self.lit(p, b".")
            }
            Pat::RootAt1 => self.import_begin(p),
            Pat::RootAt1At2 => self.illegal_star(p),
            Pat::RootAt1At6 => self.lit(p, b"*"),

            Pat::CommentsAt0 => self.lit(p, b"/**/"),
            Pat::CommentsInlineAt0 => self.lit(p, b"/*"),
            Pat::CommentsInlineAt0End | Pat::CommentsJavadocAt0End => self.lit(p, b"*/"),
            Pat::CommentsInlineAt1 => self.line_comment_lead(p),
            Pat::CommentsInlineAt1End => self.not_anchor(p),
            Pat::CommentsInlineAt1At0 => self.lit(p, b"//"),
            Pat::CommentsInlineAt1At0End => self.line_feed(p),
            Pat::CommentsJavadocAt0 => self.javadoc_begin(p),
            Pat::CommentsJavadocAt0At0 => self.javadoc_tag(p),
            Pat::CommentsJavadocAt0At1 => self.javadoc_named(p, &[b"@param"]),
            Pat::CommentsJavadocAt0At2 => self.javadoc_named(p, &[b"@exception", b"@throws"]),
            Pat::CommentsJavadocAt0At3 => self.javadoc_link(p),

            Pat::AnnotationsAt0 => self.annotation_begin(p),
            Pat::AnnotationsAt0End
            | Pat::ParensAt0End
            | Pat::FunctionCallEnd
            | Pat::MethodCallEnd
            | Pat::MethodsAt1End
            | Pat::RecordAt3End
            | Pat::TryCatchFinallyAt0At0End
            | Pat::TryCatchFinallyAt1At1End
            | Pat::EnumsAt2At0At2At2End => self.lit(p, b")"),
            Pat::AnnotationsAt0At0 => self.annotation_key(p),
            Pat::AnnotationsAt1 => self.annotation_name(p),

            Pat::Enums => self.enum_begin(p),
            Pat::EnumsEnd
            | Pat::AnonymousBlockAndInstanceInitializerEnd
            | Pat::ClassEnd
            | Pat::RecordEnd
            | Pat::TryCatchFinallyAt0End
            | Pat::TryCatchFinallyAt1End
            | Pat::TryCatchFinallyAt2End
            | Pat::ParensAt2End
            | Pat::AnonymousClassesAndNewAt3At1End
            | Pat::AnonymousClassesAndNewAt4At1End
            | Pat::EnumsAt2At0At2At3End
            | Pat::ModuleEnd => self.lit(p, b"}"),
            Pat::EnumsAt0 => self.keyword_group(p, b"extends"),
            Pat::EnumsAt1 => self.keyword_group(p, b"implements"),
            Pat::EnumsAt0End => self.before_brace_or(p, b"implements"),
            Pat::EnumsAt1End => self.before_brace_or(p, b"extends"),
            Pat::EnumsAt2
            | Pat::ClassAt7
            | Pat::AnonymousBlockAndInstanceInitializer
            | Pat::ParensAt2
            | Pat::MethodsAt5
            | Pat::TryCatchFinallyAt0At1
            | Pat::TryCatchFinallyAt1At2
            | Pat::TryCatchFinallyAt2At0
            | Pat::RecordBody
            | Pat::RecordConstructorAt3
            | Pat::ModuleAt0
            | Pat::AnonymousClassesAndNewAt3At1
            | Pat::AnonymousClassesAndNewAt4At1
            | Pat::EnumsAt2At0At2At3 => self.lit(p, b"{"),
            Pat::EnumsAt2End
            | Pat::ClassAt7End
            | Pat::MethodsAt5End
            | Pat::TryCatchFinallyAt0At1End
            | Pat::TryCatchFinallyAt1At2End
            | Pat::TryCatchFinallyAt2At0End
            | Pat::RecordBodyEnd
            | Pat::RecordConstructorAt3End
            | Pat::ModuleAt0End => self.ahead_of(p, b"}"),
            Pat::EnumsAt2At0 => self.prev_is(p, b'{').then_some(p),
            Pat::EnumsAt2At0End => self.ahead_of(p, b";}"),
            Pat::EnumsAt2At0At2 => self.enum_constant(p),
            Pat::EnumsAt2At0At2End => self.enum_constant_end(p),
            Pat::EnumsAt2At0At2At2
            | Pat::ParensAt0
            | Pat::TryCatchFinallyAt0At0
            | Pat::TryCatchFinallyAt1At1 => self.lit(p, b"("),

            Pat::Generics => self.lit(p, b"<"),
            Pat::GenericsEnd => self.lit(p, b">"),
            Pat::GenericsAt0 => self.keyword(p, b"extends"),
            Pat::GenericsAt1 | Pat::ConstantsAndSpecialVarsAt2 => self.keyword(p, b"super"),
            Pat::GenericsAt2 => self.generic_name(p),
            Pat::GenericsAt4 => self.ascii_ident_end(p),
            Pat::GenericsAt5 => self.lit(p, b"?"),
            Pat::GenericsAt6 => self.lit(p, b"&"),
            Pat::GenericsAt7
            | Pat::ObjectTypesInheritedAt2
            | Pat::ParametersAt5
            | Pat::ThrowsAt0
            | Pat::CodeAt23 => self.lit(p, b","),
            Pat::PrimitiveArraysAt0 => self.primitive_array(p),
            Pat::PrimitiveArraysAt0End | Pat::ObjectTypesAt1End => self.not_bracket(p),
            Pat::PrimitiveTypes => self.primitive(p),
            Pat::ObjectTypesAt1 => self.object_array(p),
            Pat::ObjectTypesAt2 => self.object_generic(p),
            Pat::ObjectTypesAt3 => self.object_type(p),
            Pat::ObjectTypesInheritedAt1 => self.inherited_class(p),

            Pat::ParensAt1 => self.lit(p, b"["),
            Pat::ParensAt1End => self.lit(p, b"]"),

            Pat::Class => self.class_ahead(p),
            Pat::StorageModifiers => self.storage_modifier(p),
            Pat::ClassAt3 => self.class_name(p),
            Pat::ClassAt4 => self.lit(p, b"extends"),
            Pat::ClassAt4End => self.ahead_of_any(p, &[b"{", b"implements", b"permits"]),
            Pat::ClassAt5 | Pat::RecordAt4 => self.keyword_blank(p, b"implements"),
            Pat::ClassAt5End => self.class_clause_end(p, b"extends", b"permits"),
            Pat::ClassAt6 => self.keyword_blank(p, b"permits"),
            Pat::ClassAt6End => self.class_clause_end(p, b"extends", b"implements"),

            Pat::Record => self.record_ahead(p),
            Pat::RecordAt3 => self.record_head(p),
            Pat::RecordAt4End | Pat::RecordConstructorAt1End => {
                (self.at(self.space_end(p)) == b'{').then_some(p)
            }
            Pat::RecordConstructor => self.method_ahead(p, Callable::Constructor),
            Pat::RecordConstructorEnd | Pat::MethodsEnd => self.method_end(p),
            Pat::RecordConstructorAt1 => self.constructor_name(p),

            Pat::TryCatchFinallyAt0 => self.keyword(p, b"try"),
            Pat::TryCatchFinallyAt1 => self.keyword_group(p, b"catch"),
            Pat::TryCatchFinallyAt1At1At2 => self.catch_type(p),
            Pat::TryCatchFinallyAt1At1At2End => self.catch_separator(p),
            Pat::TryCatchFinallyAt1At1At2At1 | Pat::ParametersAt4 => {
                let end = self.word_end(p);
                (end > p).then_some(end)
            }
            Pat::TryCatchFinallyAt2 => self.keyword(p, b"finally"),
            Pat::AssertionsAt0 => self.assert_begin(p),
            Pat::AssertionsAt0End => self.end_of_line(p),
            Pat::AssertionsAt0At0 => self.lit(p, b":"),
            Pat::KeywordsAt1 => self.one_of(p, b":?"),

            Pat::ConstantsAndSpecialVarsAt0 => self.word_in(p, &[b"true", b"false", b"null"]),
            Pat::ConstantsAndSpecialVarsAt1 => self.keyword(p, b"this"),
            Pat::NumbersAt0 => self.hex_number(p),
            Pat::NumbersAt1 => self.radix_number(p, Radix::Binary),
            Pat::NumbersAt2 => self.radix_number(p, Radix::Octal),
            Pat::NumbersAt3 => self.decimal_number(p),

            Pat::AnonymousClassesAndNew => self.keyword(p, b"new"),
            Pat::AnonymousClassesAndNewEnd
            | Pat::AnonymousClassesAndNewAt3End
            | Pat::AnonymousClassesAndNewAt4End => self.new_expression_end(p),
            Pat::AnonymousClassesAndNewAt3 => self.prev_is(p, b')').then_some(p),
            Pat::AnonymousClassesAndNewAt4 => self.prev_is(p, b']').then_some(p),
            Pat::FunctionCall => self.call(p),
            Pat::LambdaExpressionAt0 => self.lit(p, b"->"),
            Pat::MethodCall => self.method_call(p),

            Pat::KeywordsAt0 => self.keyword(p, b"throw"),
            Pat::KeywordsAt2 => self.control_keyword(p),
            Pat::KeywordsAt3 => self.keyword(p, b"instanceof"),
            Pat::KeywordsAt4 => self.bitwise_operator(p),
            Pat::KeywordsAt5 => self.bitwise_assignment(p),
            Pat::KeywordsAt6 => self.comparison(p),
            Pat::KeywordsAt7 => self.arithmetic_assignment(p),
            Pat::KeywordsAt8 => self.lit(p, b"="),
            Pat::KeywordsAt9 => self.increment(p),
            Pat::KeywordsAt10 => self.one_of(p, b"-%*+/"),
            Pat::KeywordsAt11 => self.logical(p),
            Pat::KeywordsAt12 => self.one_of(p, b"&|"),
            Pat::KeywordsAt13 => self.word_in(p, &[b"const", b"goto"]),

            Pat::Variables => self.variable_ahead(p),
            Pat::VariablesEnd | Pat::VariablesLocalEnd => self.ahead_of(p, b":;="),
            Pat::MemberVariablesEnd => self.ahead_of(p, b";="),
            Pat::VariablesAt0 => self.definition(p, b",:;="),
            Pat::VariablesLocal => self.local_variable_ahead(p),
            Pat::VariablesLocalAt0 => self.keyword(p, b"var"),
            Pat::VariablesLocalAt1 => self.definition(p, b":;="),
            Pat::Objects => self.object(p),
            Pat::PropertiesAt0 => self.property(p, Property::New),
            Pat::PropertiesAt1 => self.property(p, Property::Chained),
            Pat::PropertiesAt2 => self.property(p, Property::Plain),
            Pat::PropertiesAt3 => self.property(p, Property::Digit),
            Pat::MemberVariables => self.member_variable_ahead(p),

            Pat::StringsAt0 | Pat::StringsAt0End => self.lit(p, b"\"\"\""),
            Pat::StringsAt0At0 => self.escape(p, Escapes::TextBlock),
            Pat::StringsAt1 | Pat::StringsAt1End => self.lit(p, b"\""),
            Pat::StringsAt1At0 | Pat::StringsAt2At0 => self.escape(p, Escapes::String),
            Pat::StringsAt2 | Pat::StringsAt2End => self.lit(p, b"'"),

            Pat::CodeAt25 => self.lit(p, b";"),
            Pat::StaticInitializerAt1 => self.lit(p, b"static"),
            Pat::ClassFieldsAndMethodsAt0 => self.ahead_of(p, b"="),
            Pat::ClassFieldsAndMethodsAt0End => self.ahead_of(p, b";"),
            Pat::Methods => self.method_ahead(p, Callable::Method),
            Pat::MethodsAt1 => self.method_head(p),
            Pat::ParametersAt0 => self.keyword(p, b"final"),
            Pat::ParametersAt6 => self.lit(p, b"..."),
            Pat::MethodsAt3 => self.signature_ahead(p),
            Pat::MethodsAt3End => self.return_type_end(p),
            Pat::Throws => self.lit(p, b"throws"),
            Pat::ThrowsEnd => self.ahead_of(p, b";{"),
            Pat::ThrowsAt1 => self.throws_type(p),

            Pat::Module => self.module_begin(p),
            Pat::ModuleAt0At2 => self.word_in(
                p,
                &[
                    b"requires",
                    b"transitive",
                    b"exports",
                    b"opens",
                    b"to",
                    b"uses",
                    b"provides",
                    b"with",
                ],
            ),
        }
    }
}
