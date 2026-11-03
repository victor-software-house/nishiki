//! The rules of each context, in the grammar's priority order.

use super::decl::{Binding, Pattern, VarKind};
use super::kind::Kind;
use super::lexer::Lexer;
use super::scope::S;

impl<const D: u8> Lexer<'_, D> {
    /// Processes position `p` once: the innermost context's end rule first,
    /// then its other rules. `None` means nothing matched and the byte is
    /// plain text.
    pub(crate) fn step(&mut self, p: usize) -> Option<usize> {
        if self.halt_if(p, self.halts.map(|halts| halts.end)) {
            return Some(p);
        }
        let kind = self.top();
        if let Some(next) = self.end(kind, p) {
            return Some(next);
        }
        self.rules(kind, p)
    }

    #[expect(
        clippy::too_many_lines,
        reason = "one arm per context, in the order of the grammar"
    )]
    fn rules(&mut self, kind: Kind, p: usize) -> Option<usize> {
        match kind {
            Kind::Top | Kind::Started => self.top_rules(p),
            Kind::Script => self.script_comment(p).or_else(|| self.top_rules(p)),
            Kind::JsxExprNoAttrs => self.jsx_tag_without_attributes(p),
            Kind::JsxNoAttrs | Kind::JsxTagBody => self.jsx_children(p),
            Kind::JsxExprTag => self.jsx_tag(p),
            Kind::JsxTag => self.jsx_tag_rules(p),
            Kind::JsxTagOpen => self.jsx_tag_open_rules(p),
            Kind::JsxAttributes => self.jsx_attribute_rules(p),
            Kind::JsxStrDouble | Kind::JsxStrSingle => self.jsx_string_rules(p),
            Kind::Block | Kind::CaseBlock => self.statements(p),
            Kind::ObjLit => self.object_member(p),
            Kind::JsxEval
            | Kind::ArrLit
            | Kind::ParenExpr
            | Kind::SwitchExpr
            | Kind::IfParen
            | Kind::Return
            | Kind::Decorator
            | Kind::NewExpr
            | Kind::Ternary
            | Kind::CaseClause
            | Kind::TemplateExpr
            | Kind::ClassColon
            | Kind::VarInit1
            | Kind::VarInit2
            | Kind::ObjInit
            | Kind::ObjSpread => self.expression(p),
            Kind::ArrBind => self
                .binding_element(p, Binding::Variable)
                .or_else(|| self.punctuation_comma(p)),
            Kind::ArrBindConst => self
                .binding_element(p, Binding::Constant)
                .or_else(|| self.punctuation_comma(p)),
            Kind::ObjBind => self.object_binding_element(p, Binding::Variable),
            Kind::ObjBindConst => self.object_binding_element(p, Binding::Constant),
            Kind::ObjBindElem => self
                .object_binding_element_property_name(p)
                .or_else(|| self.binding_element(p, Binding::Variable)),
            Kind::ObjBindElemConst => self
                .object_binding_element_property_name(p)
                .or_else(|| self.binding_element(p, Binding::Constant)),
            Kind::ObjBindProp => self.object_binding_prop_rules(p),
            Kind::ParamObjElem => self
                .object_binding_element_property_name(p)
                .or_else(|| self.parameter_binding_element(p))
                .or_else(|| self.paren_expression(p)),
            Kind::ArrowHead | Kind::PossArrow => self.arrow_head_rules(kind, p),
            Kind::ArrowBody => self
                .single_line_comment_consuming_line_ending(p)
                .or_else(|| self.decl_block(p))
                .or_else(|| self.expression(p)),
            Kind::ArrowRet | Kind::PossArrowRet => self.arrow_return_type_body(p),
            Kind::ArrowRetObj | Kind::TypeFnRetObj | Kind::RetObj | Kind::TypeOpObj => {
                self.type_object(p)
            }
            Kind::Params => self.function_parameters_body(p),
            Kind::Cast
            | Kind::AsType
            | Kind::InstanceOf
            | Kind::TypeAnn1
            | Kind::TypeAnn2
            | Kind::TypeCondQ
            | Kind::TypeAliasEq
            | Kind::TypeSpread
            | Kind::ParamTypeAnn
            | Kind::ObjAs
            | Kind::TemplateTypeExpr => self.type_(p),
            Kind::JsBraces | Kind::JsBrackets | Kind::JsType => self.doc_brackets(p),
            Kind::ClassDecl => self.class_rules(p),
            Kind::ClassBody => self.class_body_rules(p),
            Kind::Heritage => self.heritage_rules(p),
            Kind::CommentDoc => self.docblock(p),
            Kind::CommentBlock
            | Kind::CommentLine
            | Kind::CommentLineNl
            | Kind::ScriptBlockComment
            | Kind::ScriptLineComment
            | Kind::ObjMember
            | Kind::JsCaption
            | Kind::JsDefaultQuote
            | Kind::TypeOp
            | Kind::VarKw => None,
            Kind::ForLoop => self.for_loop_rules(p),
            Kind::ForParen => self.for_paren_rules(p),
            Kind::SwitchStmt => self.switch_statement_rules(p),
            Kind::SwitchBlock => self.switch_block_rules(p),
            Kind::IfOuter => self.if_outer_rules(p),
            Kind::ObjBindVar => self.binding_variable_rules(p, Pattern::Object, Binding::Variable),
            Kind::ObjBindVarConst => {
                self.binding_variable_rules(p, Pattern::Object, Binding::Constant)
            }
            Kind::ArrBindVar => self.binding_variable_rules(p, Pattern::Array, Binding::Variable),
            Kind::ArrBindVarConst => {
                self.binding_variable_rules(p, Pattern::Array, Binding::Constant)
            }
            Kind::ParamObjBind | Kind::ParamObjPattern => self.parameter_object_binding_element(p),
            Kind::ParamArrBind | Kind::ParamArrPattern => self
                .parameter_binding_element(p)
                .or_else(|| self.punctuation_comma(p)),
            Kind::Directive => self.directive_rules(p),
            Kind::DirectiveTag => self.directive_tag_rules(p),
            Kind::JsExample => self.doc_example_rules(p),
            Kind::JsTypeTag => self
                .doc_type_tag_rules(p)
                .or_else(|| self.doc_type_tag_names(p)),
            Kind::JsParamTag => self
                .doc_type_tag_rules(p)
                .or_else(|| self.doc_type_tag_names(p))
                .or_else(|| self.doc_optional_param(p)),
            Kind::JsTypedefTag => self
                .doc_type_tag_rules(p)
                .or_else(|| self.doc_typedef_name(p)),
            Kind::JsRetTag => self.doc_type_tag_rules(p),
            Kind::JsInlineLink => self.doc_inline_link_rules(p),
            Kind::EnumDecl => self.enum_rules(p),
            Kind::EnumBody => self.enum_body_rules(p),
            Kind::EnumMember => self.comment(p).or_else(|| self.variable_initializer(p)),
            Kind::EnumKey => self
                .string(p)
                .or_else(|| self.array_literal(p))
                .or_else(|| self.comment(p))
                .or_else(|| self.variable_initializer(p)),
            Kind::ExportDefault => self.interface_declaration(p).or_else(|| self.expression(p)),
            Kind::Export | Kind::ImportFrom | Kind::ImportBlock => {
                if kind == Kind::ImportBlock {
                    self.import_export_clause(p)
                } else {
                    self.import_export_declaration(p)
                }
            }
            Kind::Field => self.field_rules(p),
            Kind::CallOuter => self.call_outer_rules(p),
            Kind::CallTarget | Kind::Call2Target => self.call_target_rules(p),
            Kind::Call2Outer => self.call2_outer_rules(p),
            Kind::FuncDecl => self.function_name(p).or_else(|| self.function_body(p)),
            Kind::FuncExpr => self
                .function_name(p)
                .or_else(|| self.single_line_comment_consuming_line_ending(p))
                .or_else(|| self.function_body(p)),
            Kind::Import => self.import_rules(p),
            Kind::ImportEqExt => self.comment(p).or_else(|| self.string(p)),
            Kind::ImportEqInt => self.import_eq_int_rules(p),
            Kind::AssertClause => self.assert_clause_rules(p),
            Kind::Indexer => self.type_annotation(p),
            Kind::IndexerMapped => self.indexer_mapped_rules(p),
            Kind::InterfaceDecl => self.interface_rules(p),
            Kind::LabelBlock => self.decl_block(p),
            Kind::Method => self.method_rules(p),
            Kind::ObjMethod => self
                .method_rules(p)
                .or_else(|| self.object_literal_method_declaration(p, Kind::ObjMethodNested)),
            Kind::ObjMethodNested => self.method_declaration_name(p),
            Kind::MethodName => self.method_name_rules(p),
            Kind::NamespaceDecl => self.namespace_rules(p),
            Kind::ObjKeyBracket => self.comment(p).or_else(|| self.array_literal(p)),
            Kind::ObjKeyStr => self.comment(p).or_else(|| self.string(p)),
            Kind::ObjKeyNum => self.comment(p).or_else(|| self.numeric_literal(p)),
            Kind::ObjMethodAfterKey => self.function_body(p),
            Kind::ObjValue => self.object_value_rules(p),
            Kind::ObjArrowGeneric => self.object_arrow_generic_rules(p),
            Kind::ObjArrowParen | Kind::ParenArrowParams => {
                self.expression_inside_possibly_arrow_parens(p)
            }
            Kind::ObjArrowOpen => self.type_parameters(p),
            Kind::YieldComment => self.comment(p),
            Kind::Regex | Kind::RxGroup | Kind::RxAssert => self.regexp(p),
            Kind::RxSet => self.regex_set_rules(p),
            Kind::ReturnType1 | Kind::ReturnType2 => self.return_type_core(p),
            Kind::StrSingle | Kind::StrDouble => self.string_character_escape(p),
            Kind::Template => self
                .template_substitution(p, Kind::TemplateExpr)
                .or_else(|| self.string_character_escape(p)),
            Kind::TemplateType => self
                .template_substitution(p, Kind::TemplateTypeExpr)
                .or_else(|| self.string_character_escape(p)),
            Kind::TemplateCall => self.template_call_rules(p),
            Kind::TemplateCall2 => self.type_arguments(p),
            Kind::TemplateCallTarget => self.template_target_rules(p),
            Kind::TypeArgs | Kind::TypeNameArgs => self.type_arguments_body(p),
            Kind::TypeAlias => self.type_alias_rules(p),
            Kind::TypeCond => self.type_conditional_rules(p),
            Kind::TypeCtorTp => self.comment(p).or_else(|| self.type_parameters(p)),
            Kind::TypeCtor | Kind::TypeFn => self.function_parameters(p),
            Kind::TypeFnRet1 | Kind::TypeFnRet2 => self.type_function_return_type_core(p),
            Kind::TypeObj => self.type_object_rules(p),
            Kind::TypeParams => self.type_params_rules(p),
            Kind::TypeParen => self.type_paren_rules(p),
            Kind::TypeTuple => self.type_tuple_rules(p),
            Kind::Typeof => self.type_arguments(p).or_else(|| self.expression(p)),
            Kind::VarLet => self.var_rules(VarKind::Let, p),
            Kind::VarConst => self.var_rules(VarKind::Const, p),
            Kind::VarUsing => self.var_rules(VarKind::Using, p),
            Kind::VarComma => self.var_comma_rules(VarKind::Let, p),
            Kind::VarCommaConst => self.var_comma_rules(VarKind::Const, p),
            Kind::VarCommaUsing => self.var_comma_rules(VarKind::Using, p),
            Kind::VarSingle => self
                .type_annotation(p)
                .or_else(|| self.string(p))
                .or_else(|| self.comment(p)),
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
        self.capture(Kind::Top, 2, self.len, &[S::CommentLineShebang]);
        Some(self.len)
    }

    fn arrow_head_rules(&mut self, kind: Kind, p: usize) -> Option<usize> {
        if kind == Kind::PossArrow {
            return self.type_parameters(p).or_else(|| {
                (self.byte(p) == b'(').then(|| {
                    self.open_token(Kind::ParenArrowParams, p, p + 1, &[S::MetaBraceRound])
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
            Kind::ArrowRet,
            j,
            j + 1,
            &[S::KeywordOperatorTypeAnnotation],
        );
        self.push(Kind::ArrowRet);
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
            return self.open_token(Kind::TypeCondQ, p, p + 1, &[S::KeywordOperatorTernary]);
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

    fn var_rules(&mut self, which: VarKind, p: usize) -> Option<usize> {
        if let Some(r) = self.var_kw_rule(p, which) {
            return Some(r);
        }
        let first = match which {
            VarKind::Let => self
                .destructuring_variable(p, Binding::Variable)
                .or_else(|| self.var_single_variable(p)),
            VarKind::Const => self
                .destructuring_variable(p, Binding::Constant)
                .or_else(|| self.var_single_const(p)),
            VarKind::Using => self.var_single_const(p),
        };
        if let Some(r) = first {
            return Some(r);
        }
        let comma = match which {
            VarKind::Let => Kind::VarComma,
            VarKind::Const => Kind::VarCommaConst,
            VarKind::Using => Kind::VarCommaUsing,
        };
        self.variable_initializer(p)
            .or_else(|| self.comment(p))
            .or_else(|| self.var_comma(p, comma))
            .or_else(|| self.punctuation_comma(p))
    }

    fn var_comma_rules(&mut self, which: VarKind, p: usize) -> Option<usize> {
        if let Some(r) = self.single_line_comment_consuming_line_ending(p) {
            return Some(r);
        }
        if let Some(r) = self.comment(p) {
            return Some(r);
        }
        let first = match which {
            VarKind::Let => self
                .destructuring_variable(p, Binding::Variable)
                .or_else(|| self.var_single_variable(p)),
            VarKind::Const => self
                .destructuring_variable(p, Binding::Constant)
                .or_else(|| self.var_single_const(p)),
            VarKind::Using => self.var_single_const(p),
        };
        first.or_else(|| self.punctuation_comma(p))
    }

    fn template_call_rules(&mut self, p: usize) -> Option<usize> {
        if self.template_target_start(p) {
            return self.open_empty(Kind::TemplateCallTarget, p);
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
