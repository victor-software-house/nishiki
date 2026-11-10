//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups the grammar names with
//! [`Lexer::capture`]. The engine names them.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would.
//! Where a quantifier could give characters back, the doc comment shows why it
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
use super::grammar::Pattern;
use crate::rules::Chars;

impl Lexer<'_> {
    /// Whether `pattern` matches at `p`, and where its match ends.
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
    pub(super) fn matcher(&mut self, pattern: Pattern, p: usize) -> Option<usize> {
        match pattern {
            Pattern::RootAt0 => self.package_begin(p),
            Pattern::RootAt0End | Pattern::RootAt1End => self.terminator(p),
            Pattern::RootAt0At1 | Pattern::RootAt1At1 => self.misplaced_dot(p),
            Pattern::RootAt0At2 | Pattern::RootAt1At3 => self.misplaced_name_part(p),
            Pattern::RootAt0At3 => self.capitals(p),
            Pattern::RootAt0At4 | Pattern::RootAt1At4 => self.reserved_word(p),
            Pattern::RootAt0At5
            | Pattern::RootAt1At5
            | Pattern::CodeAt24
            | Pattern::GenericsAt8 => self.lit(p, b"."),
            Pattern::RootAt1 => self.import_begin(p),
            Pattern::RootAt1At2 => self.illegal_star(p),
            Pattern::RootAt1At6 => self.lit(p, b"*"),

            Pattern::CommentsAt0 => self.lit(p, b"/**/"),
            Pattern::CommentsInlineAt0 => self.lit(p, b"/*"),
            Pattern::CommentsInlineAt0End | Pattern::CommentsJavadocAt0End => self.lit(p, b"*/"),
            Pattern::CommentsInlineAt1 => self.line_comment_lead(p),
            Pattern::CommentsInlineAt1End => self.not_anchor(p),
            Pattern::CommentsInlineAt1At0 => self.lit(p, b"//"),
            Pattern::CommentsInlineAt1At0End => self.line_feed(p),
            Pattern::CommentsJavadocAt0 => self.javadoc_begin(p),
            Pattern::CommentsJavadocAt0At0 => self.javadoc_tag(p),
            Pattern::CommentsJavadocAt0At1 => self.javadoc_named(p, &[b"@param"]),
            Pattern::CommentsJavadocAt0At2 => self.javadoc_named(p, &[b"@exception", b"@throws"]),
            Pattern::CommentsJavadocAt0At3 => self.javadoc_link(p),

            Pattern::AnnotationsAt0 => self.annotation_begin(p),
            Pattern::AnnotationsAt0End
            | Pattern::ParensAt0End
            | Pattern::FunctionCallEnd
            | Pattern::MethodCallEnd
            | Pattern::MethodsAt1End
            | Pattern::RecordAt3End
            | Pattern::TryCatchFinallyAt0At0End
            | Pattern::TryCatchFinallyAt1At1End
            | Pattern::EnumsAt2At0At2At2End => self.lit(p, b")"),
            Pattern::AnnotationsAt0At0 => self.annotation_key(p),
            Pattern::AnnotationsAt1 => self.annotation_name(p),

            Pattern::Enums => self.enum_begin(p),
            Pattern::EnumsEnd
            | Pattern::AnonymousBlockAndInstanceInitializerEnd
            | Pattern::ClassEnd
            | Pattern::RecordEnd
            | Pattern::TryCatchFinallyAt0End
            | Pattern::TryCatchFinallyAt1End
            | Pattern::TryCatchFinallyAt2End
            | Pattern::ParensAt2End
            | Pattern::AnonymousClassesAndNewAt3At1End
            | Pattern::AnonymousClassesAndNewAt4At1End
            | Pattern::EnumsAt2At0At2At3End
            | Pattern::ModuleEnd => self.lit(p, b"}"),
            Pattern::EnumsAt0 => self.keyword_group(p, b"extends"),
            Pattern::EnumsAt1 => self.keyword_group(p, b"implements"),
            Pattern::EnumsAt0End => self.before_brace_or(p, b"implements"),
            Pattern::EnumsAt1End => self.before_brace_or(p, b"extends"),
            Pattern::EnumsAt2
            | Pattern::ClassAt7
            | Pattern::AnonymousBlockAndInstanceInitializer
            | Pattern::ParensAt2
            | Pattern::MethodsAt5
            | Pattern::TryCatchFinallyAt0At1
            | Pattern::TryCatchFinallyAt1At2
            | Pattern::TryCatchFinallyAt2At0
            | Pattern::RecordBody
            | Pattern::RecordConstructorAt3
            | Pattern::ModuleAt0
            | Pattern::AnonymousClassesAndNewAt3At1
            | Pattern::AnonymousClassesAndNewAt4At1
            | Pattern::EnumsAt2At0At2At3 => self.lit(p, b"{"),
            Pattern::EnumsAt2End
            | Pattern::ClassAt7End
            | Pattern::MethodsAt5End
            | Pattern::TryCatchFinallyAt0At1End
            | Pattern::TryCatchFinallyAt1At2End
            | Pattern::TryCatchFinallyAt2At0End
            | Pattern::RecordBodyEnd
            | Pattern::RecordConstructorAt3End
            | Pattern::ModuleAt0End => self.ahead_of(p, b"}"),
            Pattern::EnumsAt2At0 => self.prev_is(p, b'{').then_some(p),
            Pattern::EnumsAt2At0End => self.ahead_of(p, b";}"),
            Pattern::EnumsAt2At0At2 => self.enum_constant(p),
            Pattern::EnumsAt2At0At2End => self.enum_constant_end(p),
            Pattern::EnumsAt2At0At2At2
            | Pattern::ParensAt0
            | Pattern::TryCatchFinallyAt0At0
            | Pattern::TryCatchFinallyAt1At1 => self.lit(p, b"("),

            Pattern::Generics => self.lit(p, b"<"),
            Pattern::GenericsEnd => self.lit(p, b">"),
            Pattern::GenericsAt0 => self.keyword(p, b"extends"),
            Pattern::GenericsAt1 | Pattern::ConstantsAndSpecialVarsAt2 => self.keyword(p, b"super"),
            Pattern::GenericsAt2 => self.generic_name(p),
            Pattern::GenericsAt4 => self.ascii_ident_end(p),
            Pattern::GenericsAt5 => self.lit(p, b"?"),
            Pattern::GenericsAt6 => self.lit(p, b"&"),
            Pattern::GenericsAt7
            | Pattern::ObjectTypesInheritedAt2
            | Pattern::ParametersAt5
            | Pattern::ThrowsAt0
            | Pattern::CodeAt23 => self.lit(p, b","),
            Pattern::PrimitiveArraysAt0 => self.primitive_array(p),
            Pattern::PrimitiveArraysAt0End | Pattern::ObjectTypesAt1End => self.not_bracket(p),
            Pattern::PrimitiveTypes => self.primitive(p),
            Pattern::ObjectTypesAt1 => self.object_array(p),
            Pattern::ObjectTypesAt2 => self.object_generic(p),
            Pattern::ObjectTypesAt3 => self.object_type(p),
            Pattern::ObjectTypesInheritedAt1 => self.inherited_class(p),

            Pattern::ParensAt1 => self.lit(p, b"["),
            Pattern::ParensAt1End => self.lit(p, b"]"),

            Pattern::Class => self.class_ahead(p),
            Pattern::StorageModifiers => self.storage_modifier(p),
            Pattern::ClassAt3 => self.class_name(p),
            Pattern::ClassAt4 => self.lit(p, b"extends"),
            Pattern::ClassAt4End => self.ahead_of_any(p, &[b"{", b"implements", b"permits"]),
            Pattern::ClassAt5 | Pattern::RecordAt4 => self.keyword_blank(p, b"implements"),
            Pattern::ClassAt5End => self.class_clause_end(p, b"extends", b"permits"),
            Pattern::ClassAt6 => self.keyword_blank(p, b"permits"),
            Pattern::ClassAt6End => self.class_clause_end(p, b"extends", b"implements"),

            Pattern::Record => self.record_ahead(p),
            Pattern::RecordAt3 => self.record_head(p),
            Pattern::RecordAt4End | Pattern::RecordConstructorAt1End => {
                (self.at(self.space_end(p)) == b'{').then_some(p)
            }
            Pattern::RecordConstructor => self.method_ahead(p, Callable::Constructor),
            Pattern::RecordConstructorEnd | Pattern::MethodsEnd => self.method_end(p),
            Pattern::RecordConstructorAt1 => self.constructor_name(p),

            Pattern::TryCatchFinallyAt0 => self.keyword(p, b"try"),
            Pattern::TryCatchFinallyAt1 => self.keyword_group(p, b"catch"),
            Pattern::TryCatchFinallyAt1At1At2 => self.catch_type(p),
            Pattern::TryCatchFinallyAt1At1At2End => self.catch_separator(p),
            Pattern::TryCatchFinallyAt1At1At2At1 | Pattern::ParametersAt4 => {
                let end = self.word_end(p);
                (end > p).then_some(end)
            }
            Pattern::TryCatchFinallyAt2 => self.keyword(p, b"finally"),
            Pattern::AssertionsAt0 => self.assert_begin(p),
            Pattern::AssertionsAt0End => self.end_of_line(p),
            Pattern::AssertionsAt0At0 => self.lit(p, b":"),
            Pattern::KeywordsAt1 => self.one_of(p, b":?"),

            Pattern::ConstantsAndSpecialVarsAt0 => self.word_in(p, &[b"true", b"false", b"null"]),
            Pattern::ConstantsAndSpecialVarsAt1 => self.keyword(p, b"this"),
            Pattern::NumbersAt0 => self.hex_number(p),
            Pattern::NumbersAt1 => self.radix_number(p, Radix::Binary),
            Pattern::NumbersAt2 => self.radix_number(p, Radix::Octal),
            Pattern::NumbersAt3 => self.decimal_number(p),

            Pattern::AnonymousClassesAndNew => self.keyword(p, b"new"),
            Pattern::AnonymousClassesAndNewEnd
            | Pattern::AnonymousClassesAndNewAt3End
            | Pattern::AnonymousClassesAndNewAt4End => self.new_expression_end(p),
            Pattern::AnonymousClassesAndNewAt3 => self.prev_is(p, b')').then_some(p),
            Pattern::AnonymousClassesAndNewAt4 => self.prev_is(p, b']').then_some(p),
            Pattern::FunctionCall => self.call(p),
            Pattern::LambdaExpressionAt0 => self.lit(p, b"->"),
            Pattern::MethodCall => self.method_call(p),

            Pattern::KeywordsAt0 => self.keyword(p, b"throw"),
            Pattern::KeywordsAt2 => self.control_keyword(p),
            Pattern::KeywordsAt3 => self.keyword(p, b"instanceof"),
            Pattern::KeywordsAt4 => self.bitwise_operator(p),
            Pattern::KeywordsAt5 => self.bitwise_assignment(p),
            Pattern::KeywordsAt6 => self.comparison(p),
            Pattern::KeywordsAt7 => self.arithmetic_assignment(p),
            Pattern::KeywordsAt8 => self.lit(p, b"="),
            Pattern::KeywordsAt9 => self.increment(p),
            Pattern::KeywordsAt10 => self.one_of(p, b"-%*+/"),
            Pattern::KeywordsAt11 => self.logical(p),
            Pattern::KeywordsAt12 => self.one_of(p, b"&|"),
            Pattern::KeywordsAt13 => self.word_in(p, &[b"const", b"goto"]),

            Pattern::Variables => self.variable_ahead(p),
            Pattern::VariablesEnd | Pattern::VariablesLocalEnd => self.ahead_of(p, b":;="),
            Pattern::MemberVariablesEnd => self.ahead_of(p, b";="),
            Pattern::VariablesAt0 => self.definition(p, b",:;="),
            Pattern::VariablesLocal => self.local_variable_ahead(p),
            Pattern::VariablesLocalAt0 => self.keyword(p, b"var"),
            Pattern::VariablesLocalAt1 => self.definition(p, b":;="),
            Pattern::Objects => self.object(p),
            Pattern::PropertiesAt0 => self.property(p, Property::New),
            Pattern::PropertiesAt1 => self.property(p, Property::Chained),
            Pattern::PropertiesAt2 => self.property(p, Property::Plain),
            Pattern::PropertiesAt3 => self.property(p, Property::Digit),
            Pattern::MemberVariables => self.member_variable_ahead(p),

            Pattern::StringsAt0 | Pattern::StringsAt0End => self.lit(p, b"\"\"\""),
            Pattern::StringsAt0At0 => self.escape(p, Escapes::TextBlock),
            Pattern::StringsAt1 | Pattern::StringsAt1End => self.lit(p, b"\""),
            Pattern::StringsAt1At0 | Pattern::StringsAt2At0 => self.escape(p, Escapes::String),
            Pattern::StringsAt2 | Pattern::StringsAt2End => self.lit(p, b"'"),

            Pattern::CodeAt25 => self.lit(p, b";"),
            Pattern::StaticInitializerAt1 => self.lit(p, b"static"),
            Pattern::ClassFieldsAndMethodsAt0 => self.ahead_of(p, b"="),
            Pattern::ClassFieldsAndMethodsAt0End => self.ahead_of(p, b";"),
            Pattern::Methods => self.method_ahead(p, Callable::Method),
            Pattern::MethodsAt1 => self.method_head(p),
            Pattern::ParametersAt0 => self.keyword(p, b"final"),
            Pattern::ParametersAt6 => self.lit(p, b"..."),
            Pattern::MethodsAt3 => self.signature_ahead(p),
            Pattern::MethodsAt3End => self.return_type_end(p),
            Pattern::Throws => self.lit(p, b"throws"),
            Pattern::ThrowsEnd => self.ahead_of(p, b";{"),
            Pattern::ThrowsAt1 => self.throws_type(p),

            Pattern::Module => self.module_begin(p),
            Pattern::ModuleAt0At2 => self.word_in(
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
