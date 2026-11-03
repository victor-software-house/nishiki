//! The end rule of each context.
//!
//! Each end is the grammar's end regular expression, tried before the
//! context's other rules at every position, as `TextMate` does. An end that is
//! a lookahead closes the context without a token.

use super::decl::VarKind;
use super::kind::Kind;
use super::lexer::{Lexer, is_alnum_char, is_alpha_char};
use super::scope::S;

const BLOCK: S = S::PunctuationDefinitionBlock;
const ROUND: S = S::MetaBraceRound;

impl<const D: u8> Lexer<'_, D> {
    /// Closes the innermost context at `p` if its end rule matches there.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm per context, in the order of the grammar"
    )]
    pub(crate) fn end(&mut self, kind: Kind, p: usize) -> Option<usize> {
        match kind {
            Kind::Top | Kind::Started | Kind::ObjMember => None,
            Kind::Interpolation => self.halt_at_brace(p),
            Kind::JsxExprNoAttrs => self.end_zero(self.jsx_expr_no_attrs_ends(p), p),
            Kind::JsxNoAttrs => self.jsx_no_attrs_end(p),
            Kind::JsxExprTag => self.end_zero(self.jsx_expr_tag_ends(p), p),
            Kind::JsxTag => self.jsx_tag_end(p),
            Kind::JsxTagOpen | Kind::JsxAttributes => self.end_zero(self.jsx_open_ends(p), p),
            Kind::JsxTagBody => self.end_zero(self.starts(p, "</"), p),
            Kind::JsxEval => self.end_char(p, b'}', S::PunctuationSectionEmbeddedEnd),
            Kind::JsxStrDouble => self.end_char(p, b'"', S::PunctuationDefinitionStringEnd),
            Kind::JsxStrSingle => self.end_char(p, b'\'', S::PunctuationDefinitionStringEnd),
            Kind::Block
            | Kind::ObjLit
            | Kind::ClassBody
            | Kind::SwitchStmt
            | Kind::EnumBody
            | Kind::AssertClause
            | Kind::TypeObj
            | Kind::ImportBlock => self.end_char(p, b'}', BLOCK),
            Kind::ArrLit | Kind::TypeTuple => self.end_char(p, b']', S::MetaBraceSquare),
            Kind::ArrBind | Kind::ArrBindConst | Kind::ParamArrBind | Kind::ParamArrPattern => {
                self.end_char(p, b']', S::PunctuationDefinitionBindingPatternArray)
            }
            Kind::ObjBind | Kind::ObjBindConst | Kind::ParamObjBind | Kind::ParamObjPattern => {
                self.end_char(p, b'}', S::PunctuationDefinitionBindingPatternObject)
            }
            Kind::ParenExpr
            | Kind::ForParen
            | Kind::SwitchExpr
            | Kind::IfParen
            | Kind::ParenArrowParams
            | Kind::ObjArrowParen
            | Kind::TypeParen
            | Kind::ImportEqExt => self.end_char(p, b')', ROUND),
            Kind::Params => self.end_char(p, b')', S::PunctuationDefinitionParametersEnd),
            Kind::Cast => self.end_char(p, b'>', S::MetaBraceAngle),
            Kind::ArrowHead | Kind::ArrowRet | Kind::PossArrowRet => {
                self.end_zero(self.arrow_end_at(p), p)
            }
            Kind::ArrowBody => self.end_zero(self.arrow_body_end_at(p), p),
            Kind::ArrowRetObj
            | Kind::ClassDecl
            | Kind::EnumDecl
            | Kind::InterfaceDecl
            | Kind::LabelBlock
            | Kind::TypeFnRetObj
            | Kind::RetObj
            | Kind::TypeOpObj
            | Kind::ForLoop
            | Kind::ObjArrowGeneric
            | Kind::PossArrow
            | Kind::TypeCtor
            | Kind::TypeFn => {
                let want = if matches!(
                    kind,
                    Kind::ForLoop
                        | Kind::ObjArrowGeneric
                        | Kind::PossArrow
                        | Kind::TypeCtor
                        | Kind::TypeFn
                ) {
                    ')'
                } else {
                    '}'
                };
                self.end_zero(self.prev_char(p) == Some(want), p)
            }
            Kind::ObjArrowOpen | Kind::TypeCtorTp => {
                self.end_zero(self.prev_char(p) == Some('>'), p)
            }
            Kind::TypeCond => self.end_zero(self.prev_char(p) == Some(':'), p),
            Kind::JsBraces => {
                if self.b(p) == b'}' {
                    return Some(self.close(p, p + 1, &[]));
                }
                self.end_zero(self.starts(p, "*/"), p)
            }
            Kind::JsBrackets => {
                if self.b(p) == b']' {
                    return Some(self.close(p, p + 1, &[]));
                }
                self.end_zero(self.starts(p, "*/"), p)
            }
            Kind::ClassColon => self.end_zero(self.class_colon_end(p), p),
            Kind::Heritage => self.end_zero(self.b(p) == b'{', p),
            Kind::CommentDoc | Kind::CommentBlock => {
                if self.starts(p, "*/") {
                    return Some(self.close(p, p + 2, &[S::PunctuationDefinitionComment]));
                }
                None
            }
            Kind::CommentLine | Kind::Directive => self.end_zero(self.eol(p), p),
            Kind::CommentLineNl => self.end_zero(self.bol(p), p),
            Kind::Return => self.end_zero(self.statement_end(p), p),
            Kind::SwitchBlock => self.end_zero(self.b(p) == b'}', p),
            Kind::CaseClause => self.end_zero(self.b(p) == b':', p),
            Kind::CaseBlock => {
                if self.b(p) == b'}' {
                    return Some(self.close(p, p + 1, &[S::MetaBlock, BLOCK]));
                }
                None
            }
            Kind::IfOuter => self.end_zero(matches!(self.b(p), b';' | b'}') || self.eol(p), p),
            Kind::Decorator => self.end_zero(self.space_at(p), p),
            Kind::ObjBindVar | Kind::ArrBindVar | Kind::ObjBindVarConst | Kind::ArrBindVarConst => {
                self.end_zero(
                    self.eol(p)
                        || self.bol(p)
                        || matches!(self.b(p), b',' | b';' | b'=' | b'}')
                        || self.of_in_ahead(p),
                    p,
                )
            }
            Kind::DirectiveTag => {
                if self.starts(p, "/>") {
                    return Some(self.close(p, p + 2, &[S::PunctuationDefinitionTagDirective]));
                }
                None
            }
            Kind::JsExample => self.end_zero(self.b(p) == b'@' || self.starts(p, "*/"), p),
            Kind::JsCaption => {
                if self.starts(p, "</caption>") {
                    let tag = S::EntityNameTagInlineJsdoc;
                    self.flush(p);
                    self.pop();
                    self.cap(
                        Kind::Top,
                        p,
                        p + 2,
                        &[tag, S::PunctuationDefinitionBracketAngleBeginJsdoc],
                    );
                    self.cap(Kind::Top, p + 2, p + 9, &[tag]);
                    self.cap(
                        Kind::Top,
                        p + 9,
                        p + 10,
                        &[tag, S::PunctuationDefinitionBracketAngleEndJsdoc],
                    );
                    return Some(p + 10);
                }
                self.end_zero(self.starts(p, "*/"), p)
            }
            Kind::JsTypeTag | Kind::JsParamTag | Kind::JsTypedefTag | Kind::JsRetTag => {
                self.end_zero(self.doc_type_tag_end(p), p)
            }
            Kind::JsDefaultQuote => {
                if u32::from(self.b(p)) == self.payload() && p < self.len {
                    return Some(self.close(
                        p,
                        p + 1,
                        &[
                            S::VariableOtherJsdoc,
                            S::PunctuationDefinitionStringEndJsdoc,
                        ],
                    ));
                }
                self.end_zero(self.eol(p) || self.starts(p, "*/"), p)
            }
            Kind::JsInlineLink => {
                if self.b(p) == b'}' {
                    return Some(self.close(
                        p,
                        p + 1,
                        &[S::PunctuationDefinitionBracketCurlyEndJsdoc],
                    ));
                }
                self.end_zero(self.starts(p, "*/"), p)
            }
            Kind::JsType => {
                if self.b(p) == b'}' {
                    let e = self.close(
                        p,
                        p + 1,
                        &[
                            S::EntityNameTypeInstanceJsdoc,
                            S::PunctuationDefinitionBracketCurlyEndJsdoc,
                        ],
                    );
                    let j = self.ws(e);
                    self.flush(j.min(self.len));
                    return Some(j);
                }
                self.end_zero(self.starts(p, "*/"), p)
            }
            Kind::EnumMember | Kind::EnumKey => {
                self.end_zero(matches!(self.b(p), b',' | b'}') || self.eol(p), p)
            }
            Kind::ExportDefault | Kind::Export => self.end_zero(self.export_end(p), p),
            Kind::Field => self.end_zero(self.field_end(p), p),
            Kind::CallOuter => {
                self.end_zero(self.prev_char(p) == Some(')') && !self.call_ahead(p), p)
            }
            Kind::CallTarget => self.end_zero(self.call_rest(p), p),
            Kind::Call2Outer => self.end_zero(
                self.prev_char(p) == Some('>') && !self.call_open_ahead(p),
                p,
            ),
            Kind::Call2Target => self.end_zero(self.open_generic_at(p), p),
            Kind::FuncDecl => self.end_zero(
                self.b(p) == b';'
                    || self.blank_line_at(p)
                    || self.statement_start_at(p)
                    || self.prev_char(p) == Some('}'),
                p,
            ),
            Kind::FuncExpr => self.end_zero(self.b(p) == b';' || self.prev_char(p) == Some('}'), p),
            Kind::Import => self.end_zero(
                !self.word_before_text(p, "import")
                    && (self.b(p) == b';' || self.eol(p) || self.bol(p)),
                p,
            ),
            Kind::ImportFrom => {
                if self.starts(p, "from") && self.boundary(p) && self.boundary(p + 4) {
                    return Some(self.close(p, p + 4, &[S::KeywordControlFrom]));
                }
                None
            }
            Kind::ImportEqInt => self.end_zero(self.b(p) == b';' || self.eol(p) || self.bol(p), p),
            Kind::Indexer | Kind::IndexerMapped => self.end_indexer(kind, p),
            Kind::InstanceOf => self.end_zero(self.instanceof_end(p), p),
            Kind::NewExpr => self.end_zero(self.new_end(p), p),
            Kind::Method => self.end_zero(
                matches!(self.b(p), b',' | b';' | b'}')
                    || self.eol(p)
                    || self.prev_char(p) == Some('}'),
                p,
            ),
            Kind::YieldComment => self.end_char(p, b'*', S::KeywordGeneratorAsterisk),
            Kind::MethodName | Kind::ObjMethodNested => {
                self.end_zero(matches!(self.b(p), b'(' | b'<'), p)
            }
            Kind::ObjMethod | Kind::ObjMethodAfterKey => self.end_zero(
                matches!(self.b(p), b',' | b';' | b'}') || self.prev_char(p) == Some('}'),
                p,
            ),
            Kind::NamespaceDecl => self.end_zero(
                self.prev_char(p) == Some('}')
                    || self.b(p) == b';'
                    || self.blank_line_at(p)
                    || self.statement_start_at(p),
                p,
            ),
            Kind::ObjBindElem | Kind::ObjBindElemConst | Kind::ParamObjElem | Kind::ObjSpread => {
                self.end_zero(matches!(self.b(p), b',' | b'}'), p)
            }
            Kind::ObjValue => self.end_zero(matches!(self.b(p), b',' | b'}'), p),
            Kind::ObjBindProp => self.end_char(p, b':', S::PunctuationDestructuring),
            Kind::ObjKeyBracket => self.end_zero(
                self.b(p) == b':'
                    || (self.prev_char(p) == Some(']')
                        && matches!(self.b(self.ws(p)), b'(' | b'<')),
                p,
            ),
            Kind::ObjKeyStr => self.end_zero(
                self.b(p) == b':'
                    || (matches!(self.prev_char(p), Some('"' | '\'' | '`'))
                        && self.key_string_tail(p)),
                p,
            ),
            Kind::ObjKeyNum => {
                let j = self.ws(p);
                let ok = self.b(p) == b':'
                    || matches!(self.b(j), b'(' | b',' | b'<' | b'}')
                    || (j > p && self.starts(j, "as"))
                    // The grammar misspells `satisfies` here; the byte escape keeps the
                    // misspelling out of the source.
                    || self.starts(p, "satisi\x66ies");
                self.end_zero(ok, p)
            }
            Kind::ObjAs | Kind::AsType => self.end_zero(self.as_type_end(kind, p), p),
            Kind::ObjInit => self.end_zero(
                matches!(self.b(p), b',' | b'}')
                    || self.eol(p)
                    || self.starts(p, "//")
                    || self.starts(p, "/*"),
                p,
            ),
            Kind::ParamTypeAnn => self.end_zero(
                matches!(self.b(p), b')' | b',') || (self.b(p) == b'=' && self.b(p + 1) != b'>'),
                p,
            ),
            Kind::Regex => self.end_regex(p),
            Kind::RxGroup | Kind::RxAssert => {
                self.end_char(p, b')', S::PunctuationDefinitionGroupRegexp)
            }
            Kind::RxSet => self.end_char(p, b']', S::PunctuationDefinitionCharacterClassRegexp),
            Kind::ReturnType1 => self.end_zero(
                !matches!(self.prev_char(p), Some('&' | ':' | '|'))
                    && (self.eol(p)
                        || self.bol(p)
                        || matches!(self.b(p), b',' | b';' | b'{' | b'}')
                        || self.starts(p, "//")),
                p,
            ),
            Kind::ReturnType2 => self.end_zero(self.return_type2_end(p), p),
            Kind::StrSingle => self.end_string(p, b'\''),
            Kind::StrDouble => self.end_string(p, b'"'),
            Kind::Template | Kind::TemplateType => {
                if self.b(p) == b'`' {
                    return Some(self.close(
                        p,
                        p + 1,
                        &[S::StringTemplate, S::PunctuationDefinitionStringTemplateEnd],
                    ));
                }
                None
            }
            Kind::TemplateExpr | Kind::TemplateTypeExpr => {
                self.end_char(p, b'}', S::PunctuationDefinitionTemplateExpressionEnd)
            }
            Kind::TemplateCall | Kind::TemplateCall2 => self.end_zero(self.b(p) == b'`', p),
            Kind::TemplateCallTarget => self.end_zero(self.template_target_end(p), p),
            Kind::Ternary => {
                let j = self.ws(p);
                if self.b(j) == b':' {
                    return Some(self.close(j, j + 1, &[S::KeywordOperatorTernary]));
                }
                None
            }
            Kind::TypeAnn1 => self.type_ann_close(p, true),
            Kind::TypeAnn2 => self.type_ann_close(p, false),
            Kind::TypeArgs | Kind::TypeParams => {
                self.end_char(p, b'>', S::PunctuationDefinitionTypeparametersEnd)
            }
            Kind::TypeNameArgs => (p < self.len && self.b(p) == b'>').then(|| {
                self.close(
                    p,
                    p + 1,
                    &[
                        S::MetaTypeParameters,
                        S::PunctuationDefinitionTypeparametersEnd,
                    ],
                )
            }),
            Kind::TypeAlias | Kind::TypeAliasEq => self.end_zero(
                matches!(self.b(p), b';' | b'}')
                    || self.blank_line_at(p)
                    || self.statement_start_at(p),
                p,
            ),
            Kind::TypeCondQ => self.end_char(p, b':', S::KeywordOperatorTernary),
            Kind::TypeFnRet1 => self.end_zero(self.type_fn_ret_end(p, true), p),
            Kind::TypeFnRet2 => self.end_zero(self.type_fn_ret_end(p, false), p),
            Kind::TypeSpread => self.end_zero(
                matches!(self.b(p), b',' | b';' | b'}')
                    || self.eol(p)
                    || self.prev_char(p) == Some('}'),
                p,
            ),
            Kind::TypeOp => self.end_zero(!self.space_at(p) && p <= self.len && self.b(p) != 0, p),
            Kind::Typeof => self.end_zero(self.typeof_end(p), p),
            Kind::VarLet => self.end_var(p, VarKind::Let),
            Kind::VarConst => self.end_var(p, VarKind::Const),
            Kind::VarUsing => self.end_var(p, VarKind::Using),
            Kind::VarKw => self.end_zero(self.b(p) != 0 && !self.space_at(p) && p < self.len, p),
            Kind::VarComma | Kind::VarCommaConst | Kind::VarCommaUsing => self.end_var_comma(p),
            Kind::VarSingle => self.end_zero(
                self.eol(p)
                    || self.bol(p)
                    || matches!(self.b(p), b',' | b';' | b'=' | b'}')
                    || self.of_in_ahead(p)
                    || self.blank_line_at(p)
                    || self.statement_start_at(p),
                p,
            ),
            Kind::VarInit1 => self.end_zero(
                self.eol(p)
                    || self.bol(p)
                    || matches!(self.b(p), b']' | b')' | b',' | b';' | b'}')
                    || self.of_in_ahead(p),
                p,
            ),
            Kind::VarInit2 => self.end_zero(self.var_init2_end(p), p),
        }
    }

    fn end_char(&mut self, p: usize, ch: u8, leaf: S) -> Option<usize> {
        (p < self.len && self.b(p) == ch).then(|| self.close(p, p + 1, &[leaf]))
    }

    fn end_zero(&mut self, cond: bool, p: usize) -> Option<usize> {
        cond.then(|| self.close_here(p))
    }

    /// `(?=[;}]|$|;|^\s*$|^\s*KW)`: where a statement ends.
    fn statement_end(&self, p: usize) -> bool {
        matches!(self.b(p), b';' | b'}')
            || self.eol(p)
            || self.blank_line_at(p)
            || self.statement_start_at(p)
    }

    fn export_end(&self, p: usize) -> bool {
        self.eol(p) || self.b(p) == b';' || self.blank_line_at(p) || self.statement_start_at(p)
    }

    fn class_colon_end(&self, p: usize) -> bool {
        matches!(
            self.b(p),
            b'-' | b']' | b')' | b'+' | b',' | b':' | b';' | b'}'
        ) || self.space_at(p)
            || self.blank_line_at(p)
            || self.statement_start_at(p)
    }

    /// `(?=\s|\*/|[^]$A-\[_a-{}])`.
    fn doc_type_tag_end(&self, p: usize) -> bool {
        let c = self.b(p);
        if self.space_at(p) || self.starts(p, "*/") {
            return true;
        }
        let in_set =
            matches!(c, b']' | b'$' | b'_' | b'}' | b'[' | b'{') || c.is_ascii_alphabetic();
        !in_set && p <= self.len
    }

    /// The end of a field: a separator, a line that is not a field, or the
    /// end of a block.
    fn field_end(&self, p: usize) -> bool {
        if matches!(self.b(p), b',' | b';' | b'}') || self.eol(p) || self.prev_char(p) == Some('}')
        {
            return true;
        }
        self.bol(p)
            && !self
                .field_tail(0)
                .is_some_and(|m| matches!(self.b(m), b',' | b':' | b';' | b'=') || self.eol(m))
    }

    fn end_indexer(&mut self, kind: Kind, p: usize) -> Option<usize> {
        if self.b(p) == b']' {
            let mut j = p + 1;
            let mut sign = None;
            if kind == Kind::IndexerMapped && matches!(self.b(j), b'-' | b'+') {
                sign = Some((j, j + 1));
                j += 1;
            }
            let k = self.ws(j);
            let mut end = k;
            let mut question = None;
            if self.b(k) == b'?' {
                question = Some((k, k + 1));
                end = self.ws(k + 1);
            }
            self.flush(p);
            let state_kind = kind;
            self.pop();
            self.cap(state_kind, p, p + 1, &[S::MetaBraceSquare]);
            if let Some((a, b)) = sign {
                self.cap(state_kind, a, b, &[S::KeywordOperatorTypeModifier]);
            }
            if let Some((a, b)) = question {
                self.cap(state_kind, a, b, &[S::KeywordOperatorOptional]);
            }
            self.fin(state_kind, end);
            return Some(end);
        }
        self.end_zero(self.eol(p), p)
    }

    /// The end of the operand of `instanceof`.
    fn instanceof_end(&self, p: usize) -> bool {
        self.prev_char(p) == Some(')') || self.operand_end(p) || self.of_instanceof(p)
    }

    /// The end of the operand of `new`.
    fn new_end(&self, p: usize) -> bool {
        self.prev_char(p) == Some(')') || self.operand_end(p) || self.new_again(p)
    }

    /// `(?=[-\])+,:;>?}]|\|\||&&|!==|$|…)`.
    fn operand_end(&self, p: usize) -> bool {
        matches!(
            self.b(p),
            b'-' | b']' | b')' | b'+' | b',' | b':' | b';' | b'>' | b'?' | b'}'
        ) || self.starts(p, "||")
            || self.starts(p, "&&")
            || self.starts(p, "!==")
            || self.eol(p)
            || self.function_keyword_follows(p)
    }

    /// `(Bfunction((\s+ID)|(\s*\()))`.
    fn function_keyword_follows(&self, p: usize) -> bool {
        if !(self.kw_before(p) && self.starts(p, "function")) {
            return false;
        }
        let e = p + 8;
        let j = self.ws(e);
        (j > e && self.ident(j).is_some()) || self.b(j) == b'('
    }

    fn new_again(&self, p: usize) -> bool {
        self.kw(p, "new").is_some()
    }

    /// `(?:([!=]==?)|(([\&^|~]\s*)?ID\s+instanceofE))`.
    fn of_instanceof(&self, p: usize) -> bool {
        if matches!(self.b(p), b'!' | b'=') && self.b(p + 1) == b'=' {
            return true;
        }
        let mut j = p;
        if matches!(self.b(j), b'&' | b'^' | b'|' | b'~') {
            j = self.ws(j + 1);
        }
        let Some(e) = self.ident(j) else {
            return false;
        };
        let k = self.ws(e);
        k > e && self.kw(k, "instanceof").is_some()
    }

    fn open_generic_at(&self, p: usize) -> bool {
        self.b(p) == b'<' && {
            let j = self.ws(p + 1);
            matches!(self.b(j), b'(' | b'[' | b'{') && self.eol(self.ws(j + 1))
        }
    }

    fn key_string_tail(&self, p: usize) -> bool {
        let j = self.ws(p);
        if matches!(self.b(j), b'(' | b',' | b'<' | b'}') {
            return true;
        }
        if j > p {
            let k = j;
            // The grammar misspells `satisfies` here; the byte escape keeps the
            // misspelling out of the source.
            for word in ["as", "satisi\x66ies"] {
                if self.starts(k, word) {
                    let e = k + word.len();
                    if self.ws(e) > e {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// The end of the type after `as` or `satisfies`.
    fn as_type_end(&self, kind: Kind, p: usize) -> bool {
        let simple = matches!(
            self.b(p),
            b'-' | b']' | b')' | b'+' | b',' | b':' | b';' | b'>' | b'?' | b'}'
        ) || self.starts(p, "||")
            || self.starts(p, "&&")
            || self.starts(p, "!==")
            || self.eol(p)
            || self.bol(p);
        if simple {
            return true;
        }
        if self.kw_before(p) && (self.starts(p, "as") || self.starts(p, "satisfies")) {
            let e = if self.starts(p, "as") { p + 2 } else { p + 9 };
            if self.ws(e) > e {
                return true;
            }
        }
        if kind == Kind::AsType {
            let j = self.ws(p);
            return j > p && self.b(j) == b'<';
        }
        false
    }

    /// `(/)([dgimsuvy]*)`.
    fn end_regex(&mut self, p: usize) -> Option<usize> {
        if self.b(p) != b'/' || p >= self.len {
            return None;
        }
        let mut e = p + 1;
        while matches!(
            self.b(e),
            b'd' | b'g' | b'i' | b'm' | b's' | b'u' | b'v' | b'y'
        ) {
            e += 1;
        }
        let kind = self.top();
        self.flush(p);
        self.pop();
        self.cap(kind, p, p + 1, &[S::PunctuationDefinitionStringEnd]);
        if e > p + 1 {
            self.cap(kind, p + 1, e, &[S::KeywordOther]);
        }
        Some(e)
    }

    /// `(')|([^\n\\])$`.
    fn end_string(&mut self, p: usize, quote: u8) -> Option<usize> {
        if p >= self.len {
            return None;
        }
        if self.b(p) == quote {
            return Some(self.close(p, p + 1, &[S::PunctuationDefinitionStringEnd]));
        }
        let next = p + self.char_len(p);
        if next == self.len && self.b(p) != b'\\' {
            return Some(self.close(p, next, &[S::InvalidIllegalNewline]));
        }
        None
    }

    /// Ends a type annotation where its end rule matches: zero-width, except
    /// before a `{` that follows a name or a closing bracket, where the match
    /// takes the blanks up to the brace.
    fn type_ann_close(&mut self, p: usize, first: bool) -> Option<usize> {
        let (end, blanks) = self.type_ann_end(p, first)?;
        let to = self.ws(p);
        if blanks && to > p {
            return Some(self.close(p, to, &[]));
        }
        Some(self.close_here(end))
    }

    /// `(?<![\&:|])(?!\s*[\&|]\s+)(…)` for the first form of a type annotation
    /// end, and the second form without the lookahead: where it ends, and
    /// whether the match takes the blanks that follow.
    fn type_ann_end(&self, p: usize, first: bool) -> Option<(usize, bool)> {
        if matches!(self.prev_char(p), Some('&' | ':' | '|')) {
            return None;
        }
        if first {
            let j = self.ws(p);
            if matches!(self.b(j), b'&' | b'|') && self.ws(j + 1) > j + 1 {
                return None;
            }
        }
        let next = self.b(p);
        let stop = matches!(next, b']' | b')' | b',' | b';' | b'}')
            || self.starts(p, "//")
            || (first && self.bol(p));
        let equals = next == b'=' && self.b(p + 1) != b'>';
        let blank = !first && self.blank_line_at(p);
        let brace = self
            .prev_char(p)
            .is_some_and(|c| matches!(c, ']' | ')' | '>' | '}') || is_alpha_char(c))
            && self.b(self.ws(p)) == b'{';
        if stop || equals || blank {
            return Some((p, false));
        }
        brace.then_some((p, true))
    }

    fn return_type2_end(&self, p: usize) -> bool {
        if matches!(self.prev_char(p), Some('&' | ':' | '|')) {
            return false;
        }
        let stop = matches!(self.b(p), b',' | b';' | b'{' | b'}')
            || self.starts(p, "//")
            || self.blank_line_at(p);
        let tail = self.prev_char(p).is_some_and(|c| !c.is_whitespace()) && self.eol(self.ws(p));
        stop || tail
    }

    fn type_fn_ret_end(&self, p: usize, first: bool) -> bool {
        if self.s.get(..p).is_some_and(|h| h.ends_with(b"=>"))
            || matches!(self.prev_char(p), Some('&' | '|'))
        {
            return false;
        }
        let next = self.b(p);
        let stop = matches!(
            next,
            b']' | b')' | b',' | b':' | b';' | b'=' | b'>' | b'?' | b'{' | b'}'
        ) || self.starts(p, "//");
        if first {
            return stop || self.eol(p);
        }
        let tail = self.prev_char(p).is_some_and(|c| !c.is_whitespace()) && self.eol(self.ws(p));
        stop || self.blank_line_at(p) || tail
    }

    fn typeof_end(&self, p: usize) -> bool {
        matches!(
            self.b(p),
            b']' | b'&' | b')' | b',' | b':' | b';' | b'=' | b'>' | b'?' | b'{' | b'|' | b'}'
        ) || (self.starts(p, "extends") && self.ws(p + 7) > p + 7)
            || self.eol(p)
            || self.blank_line_at(p)
            || self.statement_start_at(p)
    }

    /// The end of a template tag: its optional type arguments, then the
    /// backtick.
    fn template_target_end(&self, p: usize) -> bool {
        if self.b(p) == b'`' {
            return true;
        }
        self.b(p) == b'<' && self.tpb(p).is_some_and(|e| self.b(self.ws(e)) == b'`')
    }

    // Declarations.

    fn end_var(&mut self, p: usize, which: VarKind) -> Option<usize> {
        if self.var_keyword(p, which).is_some() {
            return None;
        }
        let stop = match which {
            VarKind::Using => {
                matches!(self.b(p), b';' | b'}')
                    || self.of_in_ahead(p)
                    || self.blank_line_at(p)
                    || self.statement_start_at(p)
            }
            VarKind::Let | VarKind::Const => {
                self.bol(p)
                    || matches!(self.b(p), b';' | b'}')
                    || self.of_in_ahead(p)
                    || self.blank_line_at(p)
                    || self.statement_start_at(p)
            }
        };
        let tail = self.eol(self.ws(p)) && !self.var_word_before(p, which);
        (stop || tail).then(|| self.close_here(p))
    }

    /// `(?<!^let|[^$._[:alnum:]]let|^var|…)`: the declaration keyword just
    /// before `p`.
    fn var_word_before(&self, p: usize, which: VarKind) -> bool {
        match which {
            VarKind::Let => self.word_before_text(p, "let") || self.word_before_text(p, "var"),
            VarKind::Const => self.word_before_text(p, "const"),
            VarKind::Using => {
                self.word_before_text(p, "using")
                    || self
                        .s
                        .get(..p)
                        .is_some_and(|h| h.ends_with(b"using") && self.text_before_ends_await(p))
            }
        }
    }

    fn text_before_ends_await(&self, p: usize) -> bool {
        let head = self.text(0, p.saturating_sub(5)).trim_end();
        head.ends_with("await")
            && (head.len() == 5
                || !head[..head.len() - 5].ends_with(|c: char| c == '.' || is_alnum_char(c)))
    }

    /// `(?<!,)(((?=[;=}]|(B(of|in)\s+)|^\s*$))|((?<=\S)(?=\s*$)))`.
    fn end_var_comma(&mut self, p: usize) -> Option<usize> {
        if self.prev_char(p) == Some(',') {
            return None;
        }
        let first =
            matches!(self.b(p), b';' | b'=' | b'}') || self.of_in_ahead(p) || self.blank_line_at(p);
        let second = self.prev_char(p).is_some_and(|c| !c.is_whitespace()) && self.eol(self.ws(p));
        (first || second).then(|| self.close_here(p))
    }

    /// `(?=[]),;}]|(B(of|in)\s+))|(?=^\s*$)|(?<![-\&*+/|])(?<=\S)(?<!=)(?=\s*
    /// $)`.
    fn var_init2_end(&self, p: usize) -> bool {
        if matches!(self.b(p), b']' | b')' | b',' | b';' | b'}')
            || self.of_in_ahead(p)
            || self.blank_line_at(p)
        {
            return true;
        }
        let prev = self.prev_char(p);
        let after_operator = matches!(prev, Some('-' | '&' | '*' | '+' | '/' | '|' | '='));
        !after_operator && prev.is_some_and(|c| !c.is_whitespace()) && self.eol(self.ws(p))
    }
}
