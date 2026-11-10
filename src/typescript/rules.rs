//! The rules of each context, in the grammar's priority order.

use super::decl::{Binding, Pattern, VarKeyword};
use super::lexer::Lexer;
use super::scope::S;
use super::state::Rule;

/// The entry of TypeScript's grammar that a Svelte expression starts at.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Entry {
    /// `source.ts#object-literal`.
    ObjLit,
    /// `source.ts#object-binding-pattern` and the other binding patterns.
    Binding(Pattern, Binding),
    /// `source.ts#type-parameters`.
    TypeParams,
}

impl<const D: u8> Lexer<'_, D> {
    /// Processes position `p` once: the innermost context's end rule first,
    /// then its other rules.
    /// `None` means nothing matched and the byte is plain text.
    pub(crate) fn step(&mut self, p: usize) -> Option<usize> {
        if self.halt_if(p, self.halts.map(|halts| halts.end)) {
            return Some(p);
        }
        #[cfg(feature = "lang-svelte")]
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
            Rule::JsxExprNoAttrs => self.jsx_tag_without_attributes(p),
            Rule::JsxNoAttrs | Rule::JsxTagBody => self.jsx_children(p),
            Rule::JsxExprTag => self.jsx_tag(p),
            Rule::JsxTag => self.jsx_tag_rules(p),
            Rule::JsxTagOpen => self.jsx_tag_open_rules(p),
            Rule::JsxAttributes => self.jsx_attribute_rules(p),
            Rule::JsxStrDouble | Rule::JsxStrSingle => self.jsx_string_rules(p),
            Rule::Block | Rule::CaseBlock => self.statements(p),
            Rule::ObjLit => self.object_member(p),
            Rule::EntryObjLit => self.entry(Entry::ObjLit, p),
            Rule::EntryObjBind => self.entry(Entry::Binding(Pattern::Object, Binding::Variable), p),
            Rule::EntryObjBindConst => {
                self.entry(Entry::Binding(Pattern::Object, Binding::Constant), p)
            }
            Rule::EntryArrBind => self.entry(Entry::Binding(Pattern::Array, Binding::Variable), p),
            Rule::EntryArrBindConst => {
                self.entry(Entry::Binding(Pattern::Array, Binding::Constant), p)
            }
            Rule::EntryTypeParams => self.entry(Entry::TypeParams, p),
            Rule::Expression
            | Rule::JsxEval
            | Rule::ArrLit
            | Rule::ParenExpr
            | Rule::SwitchExpr
            | Rule::IfParen
            | Rule::Return
            | Rule::Decorator
            | Rule::NewExpr
            | Rule::Ternary
            | Rule::CaseClause
            | Rule::TemplateExpr
            | Rule::ClassColon
            | Rule::VarInit1
            | Rule::VarInit2
            | Rule::ObjInit
            | Rule::ObjSpread => self.expression(p),
            Rule::ArrBind => self
                .binding_element(p, Binding::Variable)
                .or_else(|| self.punctuation_comma(p)),
            Rule::ArrBindConst => self
                .binding_element(p, Binding::Constant)
                .or_else(|| self.punctuation_comma(p)),
            Rule::ObjBind => self.object_binding_element(p, Binding::Variable),
            Rule::ObjBindConst => self.object_binding_element(p, Binding::Constant),
            Rule::ObjBindElem => self
                .object_binding_element_property_name(p)
                .or_else(|| self.binding_element(p, Binding::Variable)),
            Rule::ObjBindElemConst => self
                .object_binding_element_property_name(p)
                .or_else(|| self.binding_element(p, Binding::Constant)),
            Rule::ObjBindProp => self.object_binding_prop_rules(p),
            Rule::ParamObjElem => self
                .object_binding_element_property_name(p)
                .or_else(|| self.parameter_binding_element(p))
                .or_else(|| self.paren_expression(p)),
            Rule::ArrowHead | Rule::PossArrow => self.arrow_head_rules(rule, p),
            Rule::ArrowBody => self
                .single_line_comment_consuming_line_ending(p)
                .or_else(|| self.decl_block(p))
                .or_else(|| self.expression(p)),
            Rule::ArrowRet | Rule::PossArrowRet => self.arrow_return_type_body(p),
            Rule::ArrowRetObj | Rule::TypeFnRetObj | Rule::RetObj | Rule::TypeOpObj => {
                self.type_object(p)
            }
            Rule::Params => self.function_parameters_body(p),
            Rule::Cast
            | Rule::AsType
            | Rule::InstanceOf
            | Rule::TypeAnn1
            | Rule::TypeAnn2
            | Rule::TypeCondQ
            | Rule::TypeAliasEq
            | Rule::TypeSpread
            | Rule::ParamTypeAnn
            | Rule::ObjAs
            | Rule::TemplateTypeExpr => self.type_(p),
            Rule::JsBraces | Rule::JsBrackets | Rule::JsType => self.doc_brackets(p),
            Rule::ClassDecl => self.class_rules(p),
            Rule::ClassBody => self.class_body_rules(p),
            Rule::Heritage => self.heritage_rules(p),
            Rule::CommentDoc => self.docblock(p),
            Rule::CommentBlock
            | Rule::CommentLine
            | Rule::CommentLineNl
            | Rule::ScriptBlockComment
            | Rule::ScriptLineComment
            | Rule::ObjMember
            | Rule::JsCaption
            | Rule::JsDefaultQuote
            | Rule::TypeOp
            | Rule::VarKw => None,
            Rule::ForLoop => self.for_loop_rules(p),
            Rule::ForParen => self.for_paren_rules(p),
            Rule::SwitchStmt => self.switch_statement_rules(p),
            Rule::SwitchBlock => self.switch_block_rules(p),
            Rule::IfOuter => self.if_outer_rules(p),
            Rule::ObjBindVar => self.binding_variable_rules(p, Pattern::Object, Binding::Variable),
            Rule::ObjBindVarConst => {
                self.binding_variable_rules(p, Pattern::Object, Binding::Constant)
            }
            Rule::ArrBindVar => self.binding_variable_rules(p, Pattern::Array, Binding::Variable),
            Rule::ArrBindVarConst => {
                self.binding_variable_rules(p, Pattern::Array, Binding::Constant)
            }
            Rule::ParamObjBind | Rule::ParamObjPattern => self.parameter_object_binding_element(p),
            Rule::ParamArrBind | Rule::ParamArrPattern => self
                .parameter_binding_element(p)
                .or_else(|| self.punctuation_comma(p)),
            Rule::Directive => self.directive_rules(p),
            Rule::DirectiveTag => self.directive_tag_rules(p),
            Rule::JsExample => self.doc_example_rules(p),
            Rule::JsTypeTag => self
                .doc_type_tag_rules(p)
                .or_else(|| self.doc_type_tag_names(p)),
            Rule::JsParamTag => self
                .doc_type_tag_rules(p)
                .or_else(|| self.doc_type_tag_names(p))
                .or_else(|| self.doc_optional_param(p)),
            Rule::JsTypedefTag => self
                .doc_type_tag_rules(p)
                .or_else(|| self.doc_typedef_name(p)),
            Rule::JsRetTag => self.doc_type_tag_rules(p),
            Rule::JsInlineLink => self.doc_inline_link_rules(p),
            Rule::EnumDecl => self.enum_rules(p),
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
            Rule::FuncDecl => self.function_name(p).or_else(|| self.function_body(p)),
            Rule::FuncExpr => self
                .function_name(p)
                .or_else(|| self.single_line_comment_consuming_line_ending(p))
                .or_else(|| self.function_body(p)),
            Rule::Import => self.import_rules(p),
            Rule::ImportEqExt => self.comment(p).or_else(|| self.string(p)),
            Rule::ImportEqInt => self.import_eq_int_rules(p),
            Rule::AssertClause => self.assert_clause_rules(p),
            Rule::Indexer => self.type_annotation(p),
            Rule::IndexerMapped => self.indexer_mapped_rules(p),
            Rule::InterfaceDecl => self.interface_rules(p),
            Rule::LabelBlock => self.decl_block(p),
            Rule::Method => self.method_rules(p),
            Rule::ObjMethod => self
                .method_rules(p)
                .or_else(|| self.object_literal_method_declaration(p, Rule::ObjMethodNested)),
            Rule::ObjMethodNested => self.method_declaration_name(p),
            Rule::MethodName => self.method_name_rules(p),
            Rule::NamespaceDecl => self.namespace_rules(p),
            Rule::ObjKeyBracket => self.comment(p).or_else(|| self.array_literal(p)),
            Rule::ObjKeyStr => self.comment(p).or_else(|| self.string(p)),
            Rule::ObjKeyNum => self.comment(p).or_else(|| self.numeric_literal(p)),
            Rule::ObjMethodAfterKey => self.function_body(p),
            Rule::ObjValue => self.object_value_rules(p),
            Rule::ObjArrowGeneric => self.object_arrow_generic_rules(p),
            Rule::ObjArrowParen | Rule::ParenArrowParams => {
                self.expression_inside_possibly_arrow_parens(p)
            }
            Rule::ObjArrowOpen => self.type_parameters(p),
            Rule::YieldComment => self.comment(p),
            Rule::Regex | Rule::RxGroup | Rule::RxAssert => self.regexp(p),
            Rule::RxSet => self.regex_set_rules(p),
            Rule::ReturnType1 | Rule::ReturnType2 => self.return_type_core(p),
            Rule::StrSingle | Rule::StrDouble => self.string_character_escape(p),
            Rule::Template | Rule::EntryTemplate => self
                .template_substitution(p, Rule::TemplateExpr)
                .or_else(|| self.string_character_escape(p)),
            Rule::TemplateType => self
                .template_substitution(p, Rule::TemplateTypeExpr)
                .or_else(|| self.string_character_escape(p)),
            Rule::TemplateCall => self.template_call_rules(p),
            Rule::TemplateCall2 => self.type_arguments(p),
            Rule::TemplateCallTarget => self.template_target_rules(p),
            Rule::TypeArgs | Rule::TypeNameArgs => self.type_arguments_body(p),
            Rule::TypeAlias => self.type_alias_rules(p),
            Rule::TypeCond => self.type_conditional_rules(p),
            Rule::TypeCtorTp => self.comment(p).or_else(|| self.type_parameters(p)),
            Rule::TypeCtor | Rule::TypeFn => self.function_parameters(p),
            Rule::TypeFnRet1 | Rule::TypeFnRet2 => self.type_function_return_type_core(p),
            Rule::TypeObj => self.type_object_rules(p),
            Rule::TypeParams => self.type_params_rules(p),
            Rule::TypeParen => self.type_paren_rules(p),
            Rule::TypeTuple => self.type_tuple_rules(p),
            Rule::Typeof => self.type_arguments(p).or_else(|| self.expression(p)),
            Rule::VarLet => self.var_rules(VarKeyword::Let, p),
            Rule::VarConst => self.var_rules(VarKeyword::Const, p),
            Rule::VarUsing => self.var_rules(VarKeyword::Using, p),
            Rule::VarComma => self.var_comma_rules(VarKeyword::Let, p),
            Rule::VarCommaConst => self.var_comma_rules(VarKeyword::Const, p),
            Rule::VarCommaUsing => self.var_comma_rules(VarKeyword::Using, p),
            Rule::VarSingle => self
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
    /// No attribute asks for that: an inline attribute would hold for every
    /// dialect of `Lexer<D>`.
    fn entry(&mut self, entry: Entry, p: usize) -> Option<usize> {
        if D & super::Dialect::SVELTE == 0 && D != super::Dialect::TSX {
            return None;
        }
        match entry {
            Entry::ObjLit => self.object_literal(p),
            Entry::Binding(pattern, binding) => self.binding_pattern(p, pattern, binding),
            Entry::TypeParams => self.type_params_rules(p),
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
        if rule == Rule::PossArrow {
            return self.type_parameters(p).or_else(|| {
                (self.byte(p) == b'(').then(|| {
                    self.open_token(Rule::ParenArrowParams, p, p + 1, &[S::MetaBraceRound])
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
            Rule::ArrowRet,
            j,
            j + 1,
            &[S::KeywordOperatorTypeAnnotation],
        );
        self.push(Rule::ArrowRet);
        Some(j + 1)
    }

    fn object_binding_prop_rules(&mut self, p: usize) -> Option<usize> {
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
        if let Some(e) = self.kw(p, "as") {
            return self.token_rule(p, e, S::KeywordControlAs);
        }
        self.type_(p)
    }

    fn type_conditional_rules(&mut self, p: usize) -> Option<usize> {
        if self.byte(p) == b'?' {
            return self.open_token(Rule::TypeCondQ, p, p + 1, &[S::KeywordOperatorTernary]);
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

    fn var_rules(&mut self, which: VarKeyword, p: usize) -> Option<usize> {
        if let Some(r) = self.var_kw_rule(p, which) {
            return Some(r);
        }
        let first = match which {
            VarKeyword::Let => self
                .destructuring_variable(p, Binding::Variable)
                .or_else(|| self.var_single_variable(p)),
            VarKeyword::Const => self
                .destructuring_variable(p, Binding::Constant)
                .or_else(|| self.var_single_const(p)),
            VarKeyword::Using => self.var_single_const(p),
        };
        if let Some(r) = first {
            return Some(r);
        }
        let comma = match which {
            VarKeyword::Let => Rule::VarComma,
            VarKeyword::Const => Rule::VarCommaConst,
            VarKeyword::Using => Rule::VarCommaUsing,
        };
        self.variable_initializer(p)
            .or_else(|| self.comment(p))
            .or_else(|| self.var_comma(p, comma))
            .or_else(|| self.punctuation_comma(p))
    }

    fn var_comma_rules(&mut self, which: VarKeyword, p: usize) -> Option<usize> {
        if let Some(r) = self.single_line_comment_consuming_line_ending(p) {
            return Some(r);
        }
        if let Some(r) = self.comment(p) {
            return Some(r);
        }
        let first = match which {
            VarKeyword::Let => self
                .destructuring_variable(p, Binding::Variable)
                .or_else(|| self.var_single_variable(p)),
            VarKeyword::Const => self
                .destructuring_variable(p, Binding::Constant)
                .or_else(|| self.var_single_const(p)),
            VarKeyword::Using => self.var_single_const(p),
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
