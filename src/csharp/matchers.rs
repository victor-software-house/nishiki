//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups the grammar names with
//! [`Lexer::capture`]. The engine names them.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would.
//! Where a quantifier could give characters back, the doc comment shows why it
//! cannot matter.
//! Otherwise the matcher offers each way to the rest of the expression.
//!
//! Some patterns are a literal, a keyword or an alternation of lookarounds at
//! one byte.
//! They are generated with the grammar and matched here without a function of
//! their own.

mod chars;
mod comments;
mod declarations;
mod expressions;
mod numbers;
mod operators;
mod preprocessor;
mod statements;
mod strings;
mod types;

use strings::Literal;

use super::engine::Lexer;
use super::grammar::{Fn, How, Pattern};
use crate::rules::Chars;

/// The capture groups of a probe's answer, by group number.
#[cfg(test)]
pub(super) type ProbeGroups =
    [Option<(usize, usize)>; <super::engine::CSharp as crate::rules::Grammar>::GROUPS];

impl Lexer<'_> {
    /// Whether `pattern` matches at `p`, and where its match ends.
    ///
    /// It has no `#[inline]`, as Java's matcher has none.
    /// The rule engine's scan then calls it, and `try_match` and `hand` inline
    /// into it here, in the same codegen unit.
    pub(super) fn matcher(&mut self, pattern: Pattern, p: usize) -> Option<usize> {
        self.try_match(pattern, p)
    }

    fn try_match(&mut self, pattern: Pattern, p: usize) -> Option<usize> {
        match pattern.how() {
            How::Lit(text) => self.lit(p, text),
            How::Group(text) => {
                let end = self.lit(p, text)?;
                self.capture(1, p, end);
                Some(end)
            }
            How::Word { word, dot, group } => {
                let end = if dot {
                    self.non_member_keyword(p, word)?
                } else {
                    self.keyword(p, word)?
                };
                if group {
                    self.capture(1, p, end);
                }
                Some(end)
            }
            How::Assert(atoms) => atoms
                .iter()
                .any(|&(behind, bytes)| {
                    if behind {
                        self.prev(p).is_some_and(|b| bytes.contains(&b))
                    } else {
                        self.ahead_in(p, bytes)
                    }
                })
                .then_some(p),
            How::Hand(f) => self.hand(f, pattern, p),
        }
    }

    /// The answer of `pattern` at `p` with its capture groups, for the probe
    /// test that compares the matchers with Oniguruma.
    #[cfg(test)]
    pub(super) fn probe(&mut self, pattern: Pattern, p: usize) -> Option<(usize, ProbeGroups)> {
        let end = self.try_match(pattern, p)?;
        let mut captures: ProbeGroups =
            [None; <super::engine::CSharp as crate::rules::Grammar>::GROUPS];
        for (group, capture) in captures.iter_mut().enumerate().skip(1) {
            *capture = self.capture_span(group);
        }
        Some((end, captures))
    }

    /// The matcher of one expression that is not generated: one arm for each.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm for each expression of the grammar"
    )]
    fn hand(&mut self, f: Fn, pattern: Pattern, p: usize) -> Option<usize> {
        let word = pattern.word();
        match f {
            // Preprocessor.
            Fn::Preprocessor => self.preprocessor(p),
            Fn::PreprocessorEnd => self.after_line_end(p),
            Fn::PreprocessorCommentAt0 => self.preprocessor_comment(p),
            Fn::PreprocessorDefineOrUndef => self.preprocessor_define(p),
            Fn::PreprocessorIfOrElif => self.preprocessor_if(p),
            Fn::PreprocessorIfOrElifEnd | Fn::NestedBeginAt0End => self.before_line_end(p),
            Fn::PreprocessorExpressionAt1 => self.preprocessor_symbol(p),
            Fn::PreprocessorExpressionAt2 => self.preprocessor_operator(p),
            Fn::PreprocessorElseOrEndif => self.preprocessor_else(p),
            Fn::PreprocessorWarningOrError => self.preprocessor_warning(p),
            Fn::PreprocessorRegion => self.preprocessor_region(p),
            Fn::PreprocessorLoadAt0 => self.quoted_text(p),
            Fn::PreprocessorLineAt0 => self.preprocessor_line_word(p),
            Fn::PreprocessorLineAt1 => self.ascii_digits(p),
            Fn::PreprocessorPragmaWarning => self.pragma_warning(p),
            Fn::PreprocessorPragmaChecksum => self.pragma_checksum(p),
            Fn::PreprocessorAppDirective => self.app_directive(p),
            Fn::PreprocessorAppDirectivePackageNameAt0 => self.package_name_part(p),
            Fn::PreprocessorAppDirectivePackageNameAt1 => self.plain_ident(p),
            Fn::PreprocessorAppDirectivePackage | Fn::PreprocessorAppDirectiveSdk => {
                let word: &[u8] = if f == Fn::PreprocessorAppDirectivePackage {
                    b"package"
                } else {
                    b"sdk"
                };
                self.app_directive_package(p, word)
            }
            Fn::PreprocessorAppDirectiveProperty => self.app_directive_property(p),
            Fn::PreprocessorAppDirectiveExclude => self.app_directive_text(p, b"exclude"),
            Fn::PreprocessorAppDirectiveInclude => self.app_directive_text(p, b"include"),
            Fn::PreprocessorAppDirectiveProject => self.app_directive_text(p, b"project"),
            Fn::PreprocessorAppDirectiveRef => self.app_directive_text(p, b"ref"),
            Fn::PreprocessorAppDirectiveGeneric => self.app_directive_generic(p),

            // Comments and the XML of documentation comments.
            Fn::CommentAt0 => self.doc_line_open(p),
            Fn::CommentAt0While => self.doc_line_continues(p),
            Fn::XmlCharacterEntityAt0 => self.xml_character_entity(p),
            Fn::XmlTag => self.xml_tag_open(p),
            Fn::XmlTagEnd => self.xml_tag_close(p),
            Fn::XmlAttributeAt0 => self.xml_attribute(p),
            Fn::CommentAt1 => self.doc_block_open(p),
            Fn::CommentAt1End => self.doc_block_close(p),
            Fn::CommentAt1At0 => self.doc_block_body_open(p),
            Fn::CommentAt1At0While => self.doc_block_line_continues(p),
            Fn::CommentAt2 => self.line_comment_open(p),
            Fn::CommentAt2While => self.line_comment_continues(p),

            // Directives, names and attributes.
            Fn::ExternAliasDirective => self.extern_alias(p),
            Fn::ExternAliasDirectiveAt0 => self.ident(p),
            Fn::UsingDirectiveAt0 => self.using_static(p),
            Fn::UsingDirectiveAt1 => self.using_alias(p),
            Fn::UsingDirectiveAt2 => self.using_namespace(p),
            Fn::TypeNameAt0 => self.name_then(p, b"::"),
            Fn::TypeNameAt1 => self.name_then(p, b"."),
            Fn::TypeNameAt2 => self.dot_then_name(p),
            Fn::TupleElement => self.tuple_element(p),
            Fn::OperatorAssignment => self.assignment(p),
            Fn::AttributeSection => self.attribute_section(p),
            Fn::AttributeNamedArgument => self.attribute_named_argument(p),
            Fn::AttributeNamedArgumentEnd => self.ahead_of_class(p, b"),"),
            Fn::ExpressionOperatorExpressionChecked | Fn::TypeOperatorExpressionDefault => {
                self.keyword_paren(p, word)
            }

            // Strings.
            Fn::RawInterpolatedStringFiveOrMoreQuoteOneOrMoreInterpolation => {
                self.raw_interpolated_start(p, 1, 5)
            }
            Fn::RawInterpolatedStringFiveOrMoreQuoteOneOrMoreInterpolationEnd => {
                self.quotes_at_least(p, 5)
            }
            Fn::RawInterpolatedStringThreeOrMoreQuoteThreeOrMoreInterpolation => {
                self.raw_interpolated_start(p, 3, 3)
            }
            Fn::RawInterpolatedStringThreeOrMoreQuoteThreeOrMoreInterpolationEnd => {
                self.quotes_at_least(p, 3)
            }
            Fn::DoubleRawInterpolation => self.double_raw_interpolation(p),
            Fn::RawInterpolation => self.raw_interpolation(p),
            Fn::InterpolatedStringEnd => self.string_end(p, b'"', b"\\{"),
            Fn::StringCharacterEscape => self.character_escape(p, Literal::String),
            Fn::Interpolation => self.interpolation(p),
            Fn::VerbatimInterpolatedString => self.verbatim_interpolated_start(p),
            Fn::VerbatimInterpolatedStringEnd => self.verbatim_end(p),
            Fn::CharLiteralEnd => self.string_end(p, b'\'', b"\\"),
            Fn::CharCharacterEscape => self.character_escape(p, Literal::Char),
            Fn::StringLiteral => self.string_start(p),
            Fn::StringLiteralEnd => self.string_end(p, b'"', b"\\"),

            // Operators.
            Fn::WithExpression => self.with_expression(p),
            Fn::ConditionalOperatorAt0 => self.conditional_question(p),
            Fn::AssignmentOperatorsAt0 => self.compound_assignment(p),
            Fn::AssignmentOperatorsAt1 => self.bitwise_assignment(p),
            Fn::AssignmentExpression => self.assignment_start(p),
            Fn::ExpressionOperatorsAt0 => self.shift_operator(p),
            Fn::ExpressionOperatorsAt1 => self.equality_operator(p),
            Fn::ExpressionOperatorsAt2 => self.relational_operator(p),
            Fn::ExpressionOperatorsAt3 => self.logical_operator(p),
            Fn::ExpressionOperatorsAt4 => self.bitwise_operator(p),
            Fn::ExpressionOperatorsAt7 => self.arithmetic_operator(p),
            Fn::AwaitExpression => self.await_keyword(p),

            // Queries.
            Fn::QueryExpression => self.range_clause(p, b"from"),
            Fn::JoinClause => self.range_clause(p, b"join"),
            Fn::LetClause => self.let_clause(p),
            Fn::WhereClause => self.keyword_and_blanks(p, b"where"),
            Fn::JoinOn => self.keyword_and_blanks(p, b"on"),
            Fn::JoinEquals => self.keyword_and_blanks(p, b"equals"),
            Fn::JoinInto => self.join_into(p),
            Fn::OrderbyClause => self.keyword_and_blanks(p, b"orderby"),
            Fn::SelectClause => self.keyword_and_blanks(p, b"select"),
            Fn::GroupClause => self.keyword_and_blanks(p, b"group"),
            Fn::GroupBy => self.keyword_and_blanks(p, b"by"),
            Fn::AsExpression => self.as_expression(p),

            // Patterns.
            Fn::IsExpressionEnd => self.is_pattern_end(p),
            Fn::DiscardPattern => self.discard(p),
            Fn::CastedConstantPattern => self.casted_constant(p),
            Fn::CastedConstantPatternEnd => self.pattern_end(p),
            Fn::RelationalPattern => self.relational_pattern(p),
            Fn::TypePattern => self.ahead_of_name(p),
            Fn::TypePatternAt0 => self.at_anchor(p),
            Fn::TypePatternAt0End => self.type_pattern_end(p),
            Fn::TypeSubpatternAt1End => self.type_subpattern_end(p, b"]&(),.:;<=?[^{|}"),
            Fn::TypeSubpatternAt3End => self.type_subpattern_end(p, b"]&(),:;<=?[^{|}"),
            Fn::TypeSubpatternAt6 => self.tight_question(p),
            Fn::TypePatternAt1 => self.type_pattern_body_ahead(p),
            Fn::SubpatternAt0 => self.subpattern(p),
            Fn::SwitchLabelEnd => {
                if let Some(end) = self.lit(p, b":") {
                    self.capture(1, p, end);
                    return Some(end);
                }
                self.ahead_of_class(p, b"}")
            }
            Fn::SwitchLabelAt1 => self.not_blank_ahead(p),
            Fn::SwitchLabelAt1End => self.word_or_class_ahead(p, b":}", &[], &[b"when"]),
            Fn::SwitchExpressionAt2End => self.word_or_class_ahead(p, b",}", &[b"=>"], &[]),
            Fn::SwitchExpressionAt3End => self.word_or_class_ahead(p, b",}", &[b"=>"], &[b"when"]),

            // Numbers.
            Fn::NestedBeginAt0 => (p < self.lim()).then_some(p),
            Fn::NestedPatternAt0 => self.thousands_separator(p),
            Fn::NestedBeginAt0At0 => self.real_number(p),
            Fn::NestedBeginAt0At1 => self.binary_number(p),
            Fn::NestedBeginAt0At2 => self.hex_number(p),
            Fn::NestedBeginAt0At3 => self.integer_number(p),
            Fn::NestedBeginAt0At4 => self.invalid_number(p),
            Fn::NumericLiteral => self.numeric_literal(p),

            // Expressions.
            Fn::ExplicitAnonymousFunctionParameterRef => self.explicit_function_parameter(p, word),
            Fn::ImplicitAnonymousFunctionParameter => self.name_then_boundary(p),
            Fn::TypeParameterListAt1 => self.type_parameter_name(p),
            Fn::AnonymousMethodExpressionAt0 => self.lambda_start(p),
            Fn::AnonymousMethodExpressionAt1 => self.delegate_start(p),
            Fn::AnonymousMethodExpressionAt0At2 => self.ref_or_non_blank(p),
            Fn::ObjectCreationExpressionWithParameters => self.object_creation_with_arguments(p),
            Fn::ObjectCreationExpressionWithNoParameters => {
                self.object_creation_without_arguments(p)
            }
            Fn::ArrayCreationExpressionNew => self.array_creation(p, word),
            Fn::AnonymousObjectCreationExpression => self.anonymous_object_creation(p),
            Fn::NamedArgument => self.named_argument(p),
            Fn::NamedArgumentEnd => self.ahead_of_class(p, b"]),"),
            Fn::DeclarationExpressionLocal => self.declaration_expression(p, b"]),", 7),
            Fn::DeclarationExpressionTuple => self.declaration_expression(p, b"),", 7),
            Fn::InvocationExpression => self.invocation(p),
            Fn::MemberAccessExpressionAt0 => self.member_access(p),
            Fn::MemberAccessExpressionAt1 => self.member_with_type_arguments(p),
            Fn::MemberAccessExpressionAt2 => self.member_object(p),
            Fn::ElementAccessExpression => self.element_access(p),
            Fn::ElementAccessExpressionEnd => self.element_access_end(p),
            Fn::CastExpression => self.cast_expression(p),
            Fn::TupleLiteral => self.tuple_literal(p),
            Fn::TupleLiteralElement => self.tuple_element_name(p),
            Fn::TupleDeconstructionElementListAt4 => self.tuple_element_variable(p),
            Fn::TupleDeconstructionAssignment => self.tuple_assignment(p),
            Fn::LocalTupleDeclarationDeconstruction => self.local_tuple_assignment(p),

            // Statements.
            Fn::WhileStatement => self.keyword_then_paren(p, b"while"),
            Fn::IfStatement => self.keyword_then_paren(p, b"if"),
            Fn::WhenClause => self.when_clause(p),
            Fn::YieldReturnStatement => self.yield_statement(p, b"return"),
            Fn::YieldBreakStatement => self.yield_statement(p, b"break"),
            Fn::ContextControlStatementChecked => self.context_control(p, word),
            Fn::ForStatementAt0At0 => (self.avail(p) && !self.ahead_in(p, b");")).then_some(p),
            Fn::LocalVariableDeclaration => self.local_variable(p),
            Fn::VariableInitializer => self.variable_initializer(p),
            Fn::LocalTupleVarDeconstruction => self.local_tuple_var(p),
            Fn::ForeachStatementAt1At1 => self.foreach_variable(p),
            Fn::ForeachStatementAt1At2 => self.foreach_tuple(p),
            Fn::CatchClauseAt0At0 => self.catch_declaration(p),
            Fn::LocalConstantDeclaration => self.local_constant(p),
            Fn::LocalFunctionDeclaration => self.local_function(p),
            Fn::ParameterRef => self.parameter(p, word),
            Fn::GenericConstraints => self.generic_constraint(p),
            Fn::GenericConstraintsEnd => self.generic_constraints_end(p),
            Fn::GenericConstraintsAt5 => self.new_constraint(p),

            // Declarations.
            Fn::NamespaceDeclaration => self.namespace_start(p),
            Fn::ClassDeclaration => self.ahead_of_record_or(p, b"class"),
            Fn::StructDeclaration => self.ahead_of_record_or(p, b"struct"),
            Fn::ClassDeclarationAt0 => self.class_head(p),
            Fn::StructDeclarationAt0 => self.struct_head(p),
            Fn::BaseTypesEnd => self.base_types_end(p),
            Fn::BaseClassConstructorCall => self.base_class_constructor_call(p),
            Fn::DelegateDeclaration => self.delegate_declaration(p),
            Fn::EnumDeclaration => self.ahead_of_keyword(p, b"enum"),
            Fn::EnumDeclarationAt0 => self.starts(p, b"enum").then_some(p),
            Fn::EnumDeclarationAt0At1 => self.enum_head(p),
            Fn::EnumDeclarationAt1At4End => self.ahead_of_class(p, b",}"),
            Fn::InterfaceDeclaration => self.ahead_of_keyword(p, b"interface"),
            Fn::InterfaceDeclarationAt0 => self.keyword_and_name(p, b"interface"),
            Fn::UnionDeclaration => self.ahead_of_keyword(p, b"union"),
            Fn::UnionDeclarationAt0 => self.keyword_and_name(p, b"union"),
            Fn::RecordDeclaration => self.ahead_of_keyword(p, b"record"),
            Fn::RecordDeclarationAt0 => self.keyword_and_name(p, b"record"),
            Fn::PropertyDeclaration => self.property_declaration(p),
            Fn::PropertyAccessorsAt3 => self.getter(p),
            Fn::PropertyAccessorsAt4Set | Fn::EventAccessorsAt2Add => self.accessor(p, word),
            Fn::ConstructorDeclaration => self.constructor_declaration(p),
            Fn::ConstructorDeclarationAt0End => self.body_start_ahead(p),
            Fn::ConstructorInitializerBase => self.constructor_initializer(p, word),
            Fn::FixedSizeBufferDeclaration => self.fixed_buffer_declaration(p),
            Fn::FieldDeclaration => self.field_declaration(p),
            Fn::EventDeclaration => self.event_declaration(p),
            Fn::IndexerDeclaration => self.indexer_declaration(p),
            Fn::BracketedParameterList => self.ahead_of_bracket(p, b'['),
            Fn::BracketedParameterListEnd => self.ahead_of_bracket(p, b']'),
            Fn::DestructorDeclaration => self.destructor_declaration(p),
            Fn::OperatorDeclaration => self.operator_declaration(p),
            Fn::ConversionOperatorDeclaration => self.conversion_operator_declaration(p),
            Fn::MethodDeclaration => self.method_declaration(p),
        }
    }
}
