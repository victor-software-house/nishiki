//! The rules of each context, in the grammar's priority order.

use super::declarations::{Binding, DeclarationKeyword, Pattern};
use super::lexer::Lexer;
use super::scope::S;
use super::state::Rule;

/// The entry of TypeScript's grammar that a Svelte expression starts at.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Entry {
    /// `source.ts#object-literal`.
    ObjectLiteral,
    /// `source.ts#object-binding-pattern` and the other binding patterns.
    Binding(Pattern, Binding),
    /// `source.ts#type-parameters`.
    TypeParameters,
}

impl<const D: u8> Lexer<'_, D> {
    /// Processes position `p` once: the innermost context's end rule first,
    /// then its other rules.
    /// `None` means nothing matched and the byte is plain text.
    pub(crate) fn step(&mut self, p: usize) -> Option<usize> {
        if self.halt_if(p, self.halts.map(|halts| halts.end)) {
            return Some(p);
        }
        #[cfg(any(feature = "lang-svelte", feature = "_textmate"))]
        if D & super::Dialect::SVELTE != 0
            && let Some(next) = self.svelte_variable(p)
        {
            return Some(next);
        }
        let rule = self.top();
        if let Some(next) = self.end(rule, p) {
            return Some(next);
        }
        self.rules(rule, p)
    }

    #[expect(
        clippy::too_many_lines,
        reason = "one arm per context, in the order of the grammar"
    )]
    fn rules(&mut self, rule: Rule, p: usize) -> Option<usize> {
        match rule {
            Rule::Top | Rule::Started => self.top_rules(p),
            Rule::Script => self.script_comment(p).or_else(|| self.top_rules(p)),
            Rule::JsxExpressionNoAttributes => self.jsx_tag_without_attributes(p),
            Rule::JsxNoAttributes | Rule::JsxTagBody => self.jsx_children(p),
            Rule::JsxExpressionTag => self.jsx_tag(p),
            Rule::JsxTag => self.jsx_tag_rules(p),
            Rule::JsxTagOpen => self.jsx_tag_open_rules(p),
            Rule::JsxAttributes => self.jsx_attribute_rules(p),
            Rule::JsxStringDouble | Rule::JsxStringSingle => self.jsx_string_rules(p),
            Rule::Block | Rule::CaseBlock => self.statements(p),
            Rule::ObjectLiteral => self.object_member(p),
            Rule::EntryObjectLiteral => self.entry(Entry::ObjectLiteral, p),
            Rule::EntryObjectBinding => {
                self.entry(Entry::Binding(Pattern::Object, Binding::Variable), p)
            }
            Rule::EntryObjectBindingConst => {
                self.entry(Entry::Binding(Pattern::Object, Binding::Constant), p)
            }
            Rule::EntryArrayBinding => {
                self.entry(Entry::Binding(Pattern::Array, Binding::Variable), p)
            }
            Rule::EntryArrayBindingConst => {
                self.entry(Entry::Binding(Pattern::Array, Binding::Constant), p)
            }
            Rule::EntryTypeParameters => self.entry(Entry::TypeParameters, p),
            Rule::Expression
            | Rule::JsxEmbeddedExpression
            | Rule::ArrayLiteral
            | Rule::ParenExpression
            | Rule::SwitchExpression
            | Rule::IfParen
            | Rule::Return
            | Rule::Decorator
            | Rule::NewExpression
            | Rule::Ternary
            | Rule::CaseClause
            | Rule::TemplateExpression
            | Rule::ClassColon
            | Rule::VariableInitializer1
            | Rule::VariableInitializer2
            | Rule::ObjectInitializer
            | Rule::ObjectSpread => self.expression(p),
            Rule::ArrayBinding => self
                .binding_element(p, Binding::Variable)
                .or_else(|| self.punctuation_comma(p)),
            Rule::ArrayBindingConst => self
                .binding_element(p, Binding::Constant)
                .or_else(|| self.punctuation_comma(p)),
            Rule::ObjectBinding => self.object_binding_element(p, Binding::Variable),
            Rule::ObjectBindingConst => self.object_binding_element(p, Binding::Constant),
            Rule::ObjectBindingElement => self
                .object_binding_element_property_name(p)
                .or_else(|| self.binding_element(p, Binding::Variable)),
            Rule::ObjectBindingElementConst => self
                .object_binding_element_property_name(p)
                .or_else(|| self.binding_element(p, Binding::Constant)),
            Rule::ObjectBindingProperty => self.object_binding_property_rules(p),
            Rule::ParameterObjectElement => self
                .object_binding_element_property_name(p)
                .or_else(|| self.parameter_binding_element(p))
                .or_else(|| self.paren_expression(p)),
            Rule::ArrowHead | Rule::PossibleArrow => self.arrow_head_rules(rule, p),
            Rule::ArrowBody => self
                .single_line_comment_consuming_line_ending(p)
                .or_else(|| self.declaration_block(p))
                .or_else(|| self.expression(p)),
            Rule::ArrowReturn | Rule::PossibleArrowReturn => self.arrow_return_type_body(p),
            Rule::ArrowReturnObject
            | Rule::TypeFunctionReturnObject
            | Rule::ReturnObject
            | Rule::TypeOperatorObject => self.type_object(p),
            Rule::Parameters => self.function_parameters_body(p),
            Rule::Cast
            | Rule::AsType
            | Rule::InstanceOf
            | Rule::TypeAnnotation1
            | Rule::TypeAnnotation2
            | Rule::TypeConditionalQuestionMark
            | Rule::TypeAliasEquals
            | Rule::TypeSpread
            | Rule::ParameterTypeAnnotation
            | Rule::ObjectAs
            | Rule::TemplateTypeExpression => self.type_(p),
            Rule::JsDocBraces | Rule::JsDocBrackets | Rule::JsDocType => self.doc_brackets(p),
            Rule::ClassDeclaration => self.class_rules(p),
            Rule::ClassBody => self.class_body_rules(p),
            Rule::Heritage => self.heritage_rules(p),
            Rule::CommentDoc => self.docblock(p),
            Rule::CommentBlock
            | Rule::CommentLine
            | Rule::CommentLineNewline
            | Rule::ScriptBlockComment
            | Rule::ScriptLineComment
            | Rule::ObjectMember
            | Rule::JsDocCaption
            | Rule::JsDocDefaultQuote
            | Rule::TypeOperator
            | Rule::VariableKeyword => None,
            Rule::ForLoop => self.for_loop_rules(p),
            Rule::ForParen => self.for_paren_rules(p),
            Rule::SwitchStatement => self.switch_statement_rules(p),
            Rule::SwitchBlock => self.switch_block_rules(p),
            Rule::IfOuter => self.if_outer_rules(p),
            Rule::ObjectBindingVariable => {
                self.binding_variable_rules(p, Pattern::Object, Binding::Variable)
            }
            Rule::ObjectBindingVariableConst => {
                self.binding_variable_rules(p, Pattern::Object, Binding::Constant)
            }
            Rule::ArrayBindingVariable => {
                self.binding_variable_rules(p, Pattern::Array, Binding::Variable)
            }
            Rule::ArrayBindingVariableConst => {
                self.binding_variable_rules(p, Pattern::Array, Binding::Constant)
            }
            Rule::ParameterObjectBinding | Rule::ParameterObjectPattern => {
                self.parameter_object_binding_element(p)
            }
            Rule::ParameterArrayBinding | Rule::ParameterArrayPattern => self
                .parameter_binding_element(p)
                .or_else(|| self.punctuation_comma(p)),
            Rule::Directive => self.directive_rules(p),
            Rule::DirectiveTag => self.directive_tag_rules(p),
            Rule::JsDocExample => self.doc_example_rules(p),
            Rule::JsDocTypeTag => self
                .doc_type_tag_rules(p)
                .or_else(|| self.doc_type_tag_names(p)),
            Rule::JsDocParamTag => self
                .doc_type_tag_rules(p)
                .or_else(|| self.doc_type_tag_names(p))
                .or_else(|| self.doc_optional_parameter(p)),
            Rule::JsDocTypedefTag => self
                .doc_type_tag_rules(p)
                .or_else(|| self.doc_typedef_name(p)),
            Rule::JsDocReturnTag => self.doc_type_tag_rules(p),
            Rule::JsDocInlineLink => self.doc_inline_link_rules(p),
            Rule::EnumDeclaration => self.enum_rules(p),
            Rule::EnumBody => self.enum_body_rules(p),
            Rule::EnumMember => self.comment(p).or_else(|| self.variable_initializer(p)),
            Rule::EnumKey => self
                .string(p)
                .or_else(|| self.array_literal(p))
                .or_else(|| self.comment(p))
                .or_else(|| self.variable_initializer(p)),
            Rule::ExportDefault => self.interface_declaration(p).or_else(|| self.expression(p)),
            Rule::Export | Rule::ImportFrom | Rule::ImportBlock => {
                if rule == Rule::ImportBlock {
                    self.import_export_clause(p)
                } else {
                    self.import_export_declaration(p)
                }
            }
            Rule::Field => self.field_rules(p),
            Rule::CallOuter => self.call_outer_rules(p),
            Rule::CallTarget | Rule::Call2Target => self.call_target_rules(p),
            Rule::Call2Outer => self.call2_outer_rules(p),
            Rule::FunctionDeclaration => self.function_name(p).or_else(|| self.function_body(p)),
            Rule::FunctionExpression => self
                .function_name(p)
                .or_else(|| self.single_line_comment_consuming_line_ending(p))
                .or_else(|| self.function_body(p)),
            Rule::Import => self.import_rules(p),
            Rule::ImportEqualsExternal => self.comment(p).or_else(|| self.string(p)),
            Rule::ImportEqualsInternal => self.import_equals_internal_rules(p),
            Rule::AssertClause => self.assert_clause_rules(p),
            Rule::Indexer => self.type_annotation(p),
            Rule::IndexerMapped => self.indexer_mapped_rules(p),
            Rule::InterfaceDeclaration => self.interface_rules(p),
            Rule::LabelBlock => self.declaration_block(p),
            Rule::Method => self.method_rules(p),
            Rule::ObjectMethod => self
                .method_rules(p)
                .or_else(|| self.object_literal_method_declaration(p, Rule::ObjectMethodNested)),
            Rule::ObjectMethodNested => self.method_declaration_name(p),
            Rule::MethodName => self.method_name_rules(p),
            Rule::NamespaceDeclaration => self.namespace_rules(p),
            Rule::ObjectKeyBracket => self.comment(p).or_else(|| self.array_literal(p)),
            Rule::ObjectKeyString => self.comment(p).or_else(|| self.string(p)),
            Rule::ObjectKeyNumber => self.comment(p).or_else(|| self.numeric_literal(p)),
            Rule::ObjectMethodAfterKey => self.function_body(p),
            Rule::ObjectPropertyValue => self.object_value_rules(p),
            Rule::ObjectArrowGeneric => self.object_arrow_generic_rules(p),
            Rule::ObjectArrowParen | Rule::ParenArrowParameters => {
                self.expression_inside_possibly_arrow_parens(p)
            }
            Rule::ObjectArrowOpen => self.type_parameters(p),
            Rule::YieldComment => self.comment(p),
            Rule::Regex | Rule::RegexGroup | Rule::RegexAssert => self.regexp(p),
            Rule::RegexSet => self.regex_set_rules(p),
            Rule::ReturnType1 | Rule::ReturnType2 => self.return_type_core(p),
            Rule::StringSingle | Rule::StringDouble => self.string_character_escape(p),
            Rule::Template | Rule::EntryTemplate => self
                .template_substitution(p, Rule::TemplateExpression)
                .or_else(|| self.string_character_escape(p)),
            Rule::TemplateType => self
                .template_substitution(p, Rule::TemplateTypeExpression)
                .or_else(|| self.string_character_escape(p)),
            Rule::TemplateCall => self.template_call_rules(p),
            Rule::TemplateCall2 => self.type_arguments(p),
            Rule::TemplateCallTarget => self.template_target_rules(p),
            Rule::TypeArguments | Rule::TypeNameArguments => self.type_arguments_body(p),
            Rule::TypeAlias => self.type_alias_rules(p),
            Rule::TypeConditional => self.type_conditional_rules(p),
            Rule::TypeConstructorTypeParameters => {
                self.comment(p).or_else(|| self.type_parameters(p))
            }
            Rule::TypeConstructor | Rule::TypeFunction => self.function_parameters(p),
            Rule::TypeFunctionReturn1 | Rule::TypeFunctionReturn2 => {
                self.type_function_return_type_core(p)
            }
            Rule::TypeObject => self.type_object_rules(p),
            Rule::TypeParameters => self.type_parameters_rules(p),
            Rule::TypeParen => self.type_paren_rules(p),
            Rule::TypeTuple => self.type_tuple_rules(p),
            Rule::Typeof => self.type_arguments(p).or_else(|| self.expression(p)),
            Rule::VariableLet => self.variable_rules(DeclarationKeyword::Let, p),
            Rule::VariableConst => self.variable_rules(DeclarationKeyword::Const, p),
            Rule::VariableUsing => self.variable_rules(DeclarationKeyword::Using, p),
            Rule::VariableComma => self.variable_comma_rules(DeclarationKeyword::Let, p),
            Rule::VariableCommaConst => self.variable_comma_rules(DeclarationKeyword::Const, p),
            Rule::VariableCommaUsing => self.variable_comma_rules(DeclarationKeyword::Using, p),
            Rule::VariableSingle => self
                .type_annotation(p)
                .or_else(|| self.string(p))
                .or_else(|| self.comment(p)),
        }
    }

    /// The rules of an expression that starts at `entry`, an entry of
    /// TypeScript's grammar, as `typescript::start_entry` opens it.
    /// Only Svelte's dialects and Astro's TSX open an entry rule.
    /// The test of `D` drops these calls from every other dialect's lexer.
    ///
    /// There, `object_literal` and the others keep one call site each, and
    /// stay inlined.
    /// In the dialects that open an entry, `object_literal` has a second call
    /// site here, and LLVM keeps it out of line.
    /// No attribute can force that: an inline attribute would hold for every
    /// dialect of `Lexer<D>`.
    fn entry(&mut self, entry: Entry, p: usize) -> Option<usize> {
        if D & super::Dialect::SVELTE == 0 && D != super::Dialect::TSX {
            return None;
        }
        match entry {
            Entry::ObjectLiteral => self.object_literal(p),
            Entry::Binding(pattern, binding) => self.binding_pattern(p, pattern, binding),
            Entry::TypeParameters => self.type_parameters_rules(p),
        }
    }

    fn top_rules(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.directive(p) {
            return Some(r);
        }
        self.statements(p).or_else(|| self.shebang(p))
    }

    /// `\A(#!).*(?=$)`.
    fn shebang(&mut self, p: usize) -> Option<usize> {
        if p != 0 || !self.first_line || !self.starts(0, "#!") {
            return None;
        }
        self.token(
            0,
            2,
            &[S::CommentLineShebang, S::PunctuationDefinitionComment],
        );
        self.capture(Rule::Top, 2, self.len, &[S::CommentLineShebang]);
        Some(self.len)
    }

    fn arrow_head_rules(&mut self, rule: Rule, p: usize) -> Option<usize> {
        if rule == Rule::PossibleArrow {
            return self.type_parameters(p).or_else(|| {
                (self.byte(p) == b'(').then(|| {
                    self.open_token(Rule::ParenArrowParameters, p, p + 1, &[S::MetaBraceRound])
                })?
            });
        }
        self.comment(p)
            .or_else(|| self.type_parameters(p))
            .or_else(|| self.function_parameters(p))
            .or_else(|| self.arrow_return_type(p))
            .or_else(|| self.possibly_arrow_return_type(p))
    }

    /// `(?<=\))\s*(:)` in an arrow function's head.
    fn arrow_return_type(&mut self, p: usize) -> Option<usize> {
        if self.prev_char(p) != Some(')') || self.full() {
            return None;
        }
        let j = self.ws(p);
        if self.byte(j) != b':' {
            return None;
        }
        self.flush(p);
        self.capture(
            Rule::ArrowReturn,
            j,
            j + 1,
            &[S::KeywordOperatorTypeAnnotation],
        );
        self.push(Rule::ArrowReturn);
        Some(j + 1)
    }

    fn object_binding_property_rules(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.string(p) {
            return Some(r);
        }
        if let Some(r) = self.array_literal(p) {
            return Some(r);
        }
        if let Some(r) = self.numeric_literal(p) {
            return Some(r);
        }
        let e = self.ident(p)?;
        self.token_rule(p, e, S::VariableObjectProperty)
    }

    fn indexer_mapped_rules(&mut self, p: usize) -> Option<usize> {
        if let Some(e) = self.keyword(p, "as") {
            return self.token_rule(p, e, S::KeywordControlAs);
        }
        self.type_(p)
    }

    fn type_conditional_rules(&mut self, p: usize) -> Option<usize> {
        if self.byte(p) == b'?' {
            return self.open_token(
                Rule::TypeConditionalQuestionMark,
                p,
                p + 1,
                &[S::KeywordOperatorTernary],
            );
        }
        self.type_(p)
    }

    /// Inside the binding pattern of a declaration.
    fn binding_variable_rules(
        &mut self,
        p: usize,
        pattern: Pattern,
        binding: Binding,
    ) -> Option<usize> {
        self.binding_pattern(p, pattern, binding)
            .or_else(|| self.type_annotation(p))
            .or_else(|| self.comment(p))
    }

    fn variable_rules(&mut self, which: DeclarationKeyword, p: usize) -> Option<usize> {
        if let Some(r) = self.variable_keyword_rule(p, which) {
            return Some(r);
        }
        let first = match which {
            DeclarationKeyword::Let => self
                .destructuring_variable(p, Binding::Variable)
                .or_else(|| self.variable_single_variable(p)),
            DeclarationKeyword::Const => self
                .destructuring_variable(p, Binding::Constant)
                .or_else(|| self.variable_single_const(p)),
            DeclarationKeyword::Using => self.variable_single_const(p),
        };
        if let Some(r) = first {
            return Some(r);
        }
        let comma = match which {
            DeclarationKeyword::Let => Rule::VariableComma,
            DeclarationKeyword::Const => Rule::VariableCommaConst,
            DeclarationKeyword::Using => Rule::VariableCommaUsing,
        };
        self.variable_initializer(p)
            .or_else(|| self.comment(p))
            .or_else(|| self.variable_comma(p, comma))
            .or_else(|| self.punctuation_comma(p))
    }

    fn variable_comma_rules(&mut self, which: DeclarationKeyword, p: usize) -> Option<usize> {
        if let Some(r) = self.single_line_comment_consuming_line_ending(p) {
            return Some(r);
        }
        if let Some(r) = self.comment(p) {
            return Some(r);
        }
        let first = match which {
            DeclarationKeyword::Let => self
                .destructuring_variable(p, Binding::Variable)
                .or_else(|| self.variable_single_variable(p)),
            DeclarationKeyword::Const => self
                .destructuring_variable(p, Binding::Constant)
                .or_else(|| self.variable_single_const(p)),
            DeclarationKeyword::Using => self.variable_single_const(p),
        };
        first.or_else(|| self.punctuation_comma(p))
    }

    fn template_call_rules(&mut self, p: usize) -> Option<usize> {
        if self.template_target_start(p) {
            return self.open_empty(Rule::TemplateCallTarget, p);
        }
        self.type_arguments(p)
    }

    fn template_target_rules(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.support_function_call_identifiers(p) {
            return Some(r);
        }
        let e = self.ident(p)?;
        self.token_rule(p, e, S::EntityNameFunctionTaggedTemplate)
    }
}
