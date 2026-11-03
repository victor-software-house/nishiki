//! Types: annotations, generics, object and function types.

use super::bytes::{Bytes, COMMENT_START, DIGIT, NAME_START, STRING_START};
use super::kind::Kind;
use super::lexer::Lexer;
use super::look::Annotation;
use super::scope::S;

impl<const D: u8> Lexer<'_, D> {
    /// The `type` rule group.
    pub(crate) fn type_(&mut self, p: usize) -> Option<usize> {
        self.only("t.comment", COMMENT_START, p, Self::comment)
            .or_else(|| self.only("t.type_string", STRING_START, p, Self::type_string))
            .or_else(|| {
                self.only(
                    "t.numeric_literal",
                    DIGIT.with(b'.'),
                    p,
                    Self::numeric_literal,
                )
            })
            .or_else(|| {
                self.only(
                    "t.type_primitive",
                    Bytes::of(b"snbavu"),
                    p,
                    Self::type_primitive,
                )
            })
            .or_else(|| {
                self.only(
                    "t.type_builtin_literals",
                    Bytes::of(b"tfuno"),
                    p,
                    Self::type_builtin_literals,
                )
            })
            .or_else(|| {
                self.only(
                    "t.type_parameters",
                    Bytes::of(b"<"),
                    p,
                    Self::type_parameters,
                )
            })
            .or_else(|| self.only("t.type_tuple", Bytes::of(b"["), p, Self::type_tuple))
            .or_else(|| self.only("t.type_object", Bytes::of(b"{"), p, Self::type_object))
            .or_else(|| {
                self.only(
                    "t.type_operators",
                    Bytes::of(b"tik&|:?"),
                    p,
                    Self::type_operators,
                )
            })
            .or_else(|| {
                self.only(
                    "t.type_conditional",
                    Bytes::of(b"e"),
                    p,
                    Self::type_conditional,
                )
            })
            .or_else(|| {
                self.only(
                    "t.type_fn_type_parameters",
                    Bytes::of(b"an("),
                    p,
                    Self::type_fn_type_parameters,
                )
            })
            .or_else(|| {
                self.only(
                    "t.type_paren_or_function_parameters",
                    Bytes::of(b"("),
                    p,
                    Self::type_paren_or_function_parameters,
                )
            })
            .or_else(|| {
                self.only(
                    "t.type_function_return_type",
                    Bytes::of(b"="),
                    p,
                    Self::type_function_return_type,
                )
            })
            .or_else(|| self.only("t.type_readonly", Bytes::of(b"r"), p, Self::type_readonly))
            .or_else(|| self.only("t.type_name", NAME_START, p, Self::type_name))
    }

    fn type_string(&mut self, p: usize) -> Option<usize> {
        match self.b(p) {
            b'\'' => self.open1(
                Kind::StrSingle,
                p,
                p + 1,
                &[S::PunctuationDefinitionStringBegin],
            ),
            b'"' => self.open1(
                Kind::StrDouble,
                p,
                p + 1,
                &[S::PunctuationDefinitionStringBegin],
            ),
            _ => self.template_type(p),
        }
    }

    fn type_primitive(&mut self, p: usize) -> Option<usize> {
        let e = self.kw_of(
            p,
            &[
                "string", "number", "bigint", "boolean", "symbol", "any", "void", "never",
                "unknown",
            ],
        )?;
        self.tok(p, e, S::SupportTypePrimitive)
    }

    fn type_builtin_literals(&mut self, p: usize) -> Option<usize> {
        let e = self.kw_of(p, &["this", "true", "false", "undefined", "null", "object"])?;
        self.tok(p, e, S::SupportTypeBuiltin)
    }

    /// `<` of a type-parameter list.
    pub(crate) fn type_parameters(&mut self, p: usize) -> Option<usize> {
        if self.b(p) != b'<' {
            return None;
        }
        self.open1(
            Kind::TypeParams,
            p,
            p + 1,
            &[S::PunctuationDefinitionTypeparametersBegin],
        )
    }

    fn type_tuple(&mut self, p: usize) -> Option<usize> {
        if self.b(p) != b'[' {
            return None;
        }
        self.open1(Kind::TypeTuple, p, p + 1, &[S::MetaBraceSquare])
    }

    pub(crate) fn type_object(&mut self, p: usize) -> Option<usize> {
        if self.b(p) != b'{' {
            return None;
        }
        self.open1(Kind::TypeObj, p, p + 1, &[S::PunctuationDefinitionBlock])
    }

    fn type_operators(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.typeof_operator(p) {
            return Some(r);
        }
        if let Some(r) = self.type_infer(p) {
            return Some(r);
        }
        let b = self.b(p);
        if b == b'&' || b == b'|' {
            let j = self.ws(p + 1);
            let kind = if self.b(j) == b'{' {
                Kind::TypeOpObj
            } else {
                Kind::TypeOp
            };
            return self.open1(kind, p, p + 1, &[S::KeywordOperatorType]);
        }
        if let Some(e) = self.kw(p, "keyof") {
            return self.tok(p, e, S::KeywordOperatorExpressionKeyof);
        }
        if b == b':' || b == b'?' {
            return self.tok(p, p + 1, S::KeywordOperatorTernary);
        }
        if let Some(e) = self.kw(p, "import")
            && self.b(self.ws(e)) == b'('
        {
            return self.tok(p, e, S::KeywordOperatorExpressionImport);
        }
        None
    }

    /// `infer T` and `infer T extends`.
    #[expect(
        clippy::many_single_char_names,
        reason = "positions are named by role: `p` where the rule starts, `e` where a part ends, `j` and `k` for the cursor"
    )]
    fn type_infer(&mut self, p: usize) -> Option<usize> {
        let e = self.kw_of(p, &["infer"])?;
        let j = self.ws(e);
        if j == e {
            return None;
        }
        let n = self.ident(j)?;
        if !self.kw_after(n) {
            return None;
        }
        self.flush(p);
        let meta = S::MetaTypeInfer;
        self.cap(Kind::Top, p, e, &[meta, S::KeywordOperatorExpressionInfer]);
        self.cap(Kind::Top, e, j, &[meta]);
        self.cap(Kind::Top, j, n, &[meta, S::EntityNameType]);
        let m = self.ws(n);
        if m > n && self.starts(m, "extends") && self.kw_after(m + "extends".len()) {
            self.cap(Kind::Top, n, m, &[meta]);
            self.cap(
                Kind::Top,
                m,
                m + "extends".len(),
                &[meta, S::KeywordOperatorExpressionExtends],
            );
            return Some(m + "extends".len());
        }
        Some(n)
    }

    fn type_conditional(&mut self, p: usize) -> Option<usize> {
        let e = self.kw(p, "extends")?;
        let j = self.ws(e);
        if j == e || self.full() {
            return None;
        }
        self.flush(p);
        self.cap(Kind::TypeCond, p, e, &[S::StorageModifier]);
        self.fin(Kind::TypeCond, j);
        self.push(Kind::TypeCond);
        Some(j)
    }

    fn type_fn_type_parameters(&mut self, p: usize) -> Option<usize> {
        // `abstract? new` then `<` or `(`.
        let mut start = p;
        let mut abstract_end = None;
        if self.kw(p, "abstract").is_some_and(|e| self.ws(e) > e) {
            let e = p + 8;
            abstract_end = Some(e);
            start = self.ws(e);
        }
        if self.starts(start, "new") && self.kw_before(p) && self.boundary(start + "new".len()) {
            let e = start + "new".len();
            let after = self.ws(e);
            if self.b(after) == b'<' && !self.full() {
                self.flush(p);
                let meta = S::MetaTypeConstructor;
                if let Some(a) = abstract_end {
                    self.cap(Kind::Top, p, a, &[meta, S::StorageModifier]);
                }
                self.cap(Kind::Top, start, e, &[meta, S::KeywordControlNew]);
                self.push(Kind::TypeCtorTp);
                return Some(e);
            }
            if self.b(after) == b'(' && !self.full() {
                self.flush(p);
                if let Some(a) = abstract_end {
                    self.cap(Kind::TypeCtor, p, a, &[S::StorageModifier]);
                }
                self.cap(Kind::TypeCtor, start, e, &[S::KeywordControlNew]);
                self.fin(Kind::TypeCtor, after);
                self.push(Kind::TypeCtor);
                return Some(after);
            }
        }
        if self.b(p) == b'(' && self.type_function_ahead(p) {
            return self.open0(Kind::TypeFn, p);
        }
        None
    }

    /// `\(\s*((\))|(\.\.\.)|([$_[:alnum:]]+\s*(([,:=?])|(\)\s*=>))))`.
    fn type_function_ahead(&self, p: usize) -> bool {
        let k = self.ws(p + 1);
        if self.b(k) == b')' || self.starts(k, "...") {
            return true;
        }
        let mut e = k;
        while self.alnum_at(e) {
            e += self.char_len(e);
        }
        if e == k {
            return false;
        }
        let m = self.ws(e);
        matches!(self.b(m), b',' | b':' | b'=' | b'?')
            || (self.b(m) == b')' && self.starts(self.ws(m + 1), "=>"))
    }

    fn type_paren_or_function_parameters(&mut self, p: usize) -> Option<usize> {
        if self.b(p) != b'(' {
            return None;
        }
        self.open1(Kind::TypeParen, p, p + 1, &[S::MetaBraceRound])
    }

    pub(crate) fn type_function_return_type(&mut self, p: usize) -> Option<usize> {
        if !self.starts(p, "=>") || self.full() {
            return None;
        }
        let j = self.ws(p + 2);
        let kind = if self.eol(j) {
            Kind::TypeFnRet2
        } else {
            Kind::TypeFnRet1
        };
        self.open1(kind, p, p + 2, &[S::StorageTypeFunctionArrow])
    }

    fn type_readonly(&mut self, p: usize) -> Option<usize> {
        let e = self.kw(p, "readonly")?;
        let j = self.ws(e);
        self.one(p, e, &[S::StorageModifier]);
        self.fin(Kind::Top, j);
        Some(j)
    }

    fn type_name(&mut self, p: usize) -> Option<usize> {
        let e = self.ident(p)?;
        let j = self.ws(e);
        // `Module.<` and `Name<`.
        if let Some(a) = self.accessor_at(j) {
            let k = self.ws(a);
            if self.b(k) == b'<' && !self.full() {
                self.flush(p);
                self.cap(Kind::Top, p, e, &[S::EntityNameTypeModule]);
                self.cap(Kind::Top, j, a, &[Self::accessor_scope(j, a)]);
                self.cap(
                    Kind::Top,
                    k,
                    k + 1,
                    &[
                        S::MetaTypeParameters,
                        S::PunctuationDefinitionTypeparametersBegin,
                    ],
                );
                self.push(Kind::TypeNameArgs);
                return Some(k + 1);
            }
            self.flush(p);
            self.cap(Kind::Top, p, e, &[S::EntityNameTypeModule]);
            self.cap(Kind::Top, j, a, &[Self::accessor_scope(j, a)]);
            return Some(a);
        }
        if self.b(j) == b'<' && !self.full() {
            self.flush(p);
            self.cap(Kind::Top, p, e, &[S::EntityNameType]);
            self.cap(
                Kind::Top,
                j,
                j + 1,
                &[
                    S::MetaTypeParameters,
                    S::PunctuationDefinitionTypeparametersBegin,
                ],
            );
            self.push(Kind::TypeNameArgs);
            return Some(j + 1);
        }
        self.tok(p, e, S::EntityNameType)
    }

    /// `(:)` of a type annotation.
    pub(crate) fn type_annotation(&mut self, p: usize) -> Option<usize> {
        if self.b(p) != b':' || self.full() {
            return None;
        }
        let j = self.ws(p + 1);
        let kind = if self.eol(j) {
            Kind::TypeAnn2
        } else {
            Kind::TypeAnn1
        };
        self.open1(kind, p, p + 1, &[S::KeywordOperatorTypeAnnotation])
    }

    /// `(:)` of a parameter's type annotation.
    pub(crate) fn parameter_type_annotation(&mut self, p: usize) -> Option<usize> {
        if self.b(p) != b':' {
            return None;
        }
        self.open1(
            Kind::ParamTypeAnn,
            p,
            p + 1,
            &[S::KeywordOperatorTypeAnnotation],
        )
    }

    // Rules inside types.

    pub(crate) fn type_arguments_body(&mut self, p: usize) -> Option<usize> {
        if let Some(e) = self.kw(p, "_") {
            return self.tok(p, e, S::KeywordOperatorType);
        }
        self.type_(p).or_else(|| self.punctuation_comma(p))
    }

    pub(crate) fn type_params_rules(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.comment(p) {
            return Some(r);
        }
        if let Some(e) = self.kw_of(p, &["extends", "in", "out", "const"]) {
            return self.tok(p, e, S::StorageModifier);
        }
        self.type_(p)
            .or_else(|| self.punctuation_comma(p))
            .or_else(|| {
                (self.b(p) == b'=' && self.b(p + 1) != b'>')
                    .then(|| self.tok(p, p + 1, S::KeywordOperatorAssignment))?
            })
    }

    pub(crate) fn type_tuple_rules(&mut self, p: usize) -> Option<usize> {
        if self.starts(p, "...") {
            return self.tok(p, p + 3, S::KeywordOperatorRest);
        }
        if let Some(e) = self.ident(p)
            && self.kw_before(p)
        {
            let mut j = self.ws(e);
            let opt = if self.b(j) == b'?' {
                let at = j;
                j = self.ws(j + 1);
                Some(at)
            } else {
                None
            };
            if self.b(j) == b':' {
                self.one(p, e, &[S::EntityNameLabel]);
                if let Some(at) = opt {
                    self.cap(Kind::Top, at, at + 1, &[S::KeywordOperatorOptional]);
                }
                self.cap(Kind::Top, j, j + 1, &[S::PunctuationSeparatorLabel]);
                return Some(j + 1);
            }
        }
        self.type_(p).or_else(|| self.punctuation_comma(p))
    }

    pub(crate) fn type_paren_rules(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.type_parameter_name(p) {
            return Some(r);
        }
        self.type_annotation(p)
            .or_else(|| {
                (self.b(p) == b',').then(|| self.tok(p, p + 1, S::PunctuationSeparatorParameter))?
            })
            .or_else(|| self.type_(p))
    }

    /// The parameter name rules of a parenthesised type: a name that starts a
    /// function type, or any name before a colon.
    fn type_parameter_name(&mut self, p: usize) -> Option<usize> {
        let mut j = p;
        let mut modifier = None;
        for word in ["public", "private", "protected", "readonly"] {
            if let Some(e) = self.kw(p, word) {
                let k = self.ws(e);
                if k > e {
                    modifier = Some((p, e));
                    j = k;
                }
                break;
            }
        }
        let mut rest = None;
        if self.starts(j, "...") {
            rest = Some((j, j + 3));
            j = self.ws(j + 3);
        }
        if matches!(self.prev_char(j), Some(':' | '=')) {
            return None;
        }
        if !self.kw_before(j) {
            return None;
        }
        let (name_end, this) = if let Some(e) = self.kw(j, "this") {
            (e, true)
        } else {
            (self.ident(j)?, false)
        };
        if !self.kw_after(name_end) {
            return None;
        }
        let mut k = self.ws(name_end);
        let opt = if self.b(k) == b'?' {
            let at = k;
            k += 1;
            Some(at)
        } else {
            None
        };
        // The first rule: a name whose type is a function.
        let is_fn = self.colon_fn_ahead(k, Annotation::FunctionType);
        let is_param = self.b(k) == b':';
        if !is_fn && !is_param {
            return None;
        }
        self.flush(p);
        if let Some((a, b)) = modifier {
            self.cap(Kind::Top, a, b, &[S::StorageModifier]);
        }
        if let Some((a, b)) = rest {
            self.cap(Kind::Top, a, b, &[S::KeywordOperatorRest]);
        }
        let name_scopes: &[S] = match (is_fn, this) {
            (true, true) => &[S::EntityNameFunction, S::VariableLanguageThis],
            (true, false) => &[S::EntityNameFunction],
            (false, true) => &[S::VariableParameter, S::VariableLanguageThis],
            (false, false) => &[S::VariableParameter],
        };
        self.cap(Kind::Top, j, name_end, name_scopes);
        if let Some(at) = opt {
            self.cap(Kind::Top, at, at + 1, &[S::KeywordOperatorOptional]);
        }
        Some(opt.map_or(name_end, |at| at + 1))
    }

    pub(crate) fn type_object_rules(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.comment(p) {
            return Some(r);
        }
        self.method_declaration(p)
            .or_else(|| self.indexer_declaration(p))
            .or_else(|| self.indexer_mapped_type_declaration(p))
            .or_else(|| self.field_declaration(p))
            .or_else(|| self.type_annotation(p))
            .or_else(|| {
                self.starts(p, "...")
                    .then(|| self.open1(Kind::TypeSpread, p, p + 3, &[S::KeywordOperatorSpread]))?
            })
            .or_else(|| self.punctuation_comma(p))
            .or_else(|| self.punctuation_semicolon(p))
            .or_else(|| self.type_(p))
    }

    /// Rules shared by function return types and arrow return types.
    pub(crate) fn return_type_core(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.comment(p) {
            return Some(r);
        }
        if matches!(self.prev_char(p), Some('&' | ':' | '|')) && self.b(self.ws(p)) == b'{' {
            return self.open0(Kind::RetObj, p);
        }
        self.type_predicate_operator(p).or_else(|| self.type_(p))
    }

    pub(crate) fn type_function_return_type_core(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.comment(p) {
            return Some(r);
        }
        if self.s.get(..p).is_some_and(|h| h.ends_with(b"=>")) && self.b(self.ws(p)) == b'{' {
            return self.open0(Kind::TypeFnRetObj, p);
        }
        self.type_predicate_operator(p).or_else(|| self.type_(p))
    }

    pub(crate) fn arrow_return_type_body(&mut self, p: usize) -> Option<usize> {
        if self.prev_char(p) == Some(':') && self.b(self.ws(p)) == b'{' {
            return self.open0(Kind::ArrowRetObj, p);
        }
        self.type_predicate_operator(p).or_else(|| self.type_(p))
    }

    /// `x is T`, `asserts x is T`, `asserts x`, `asserts` and `is`.
    fn type_predicate_operator(&mut self, p: usize) -> Option<usize> {
        if !self.kw_before(p) {
            return None;
        }
        // (?:(asserts)\s+)?(?!asserts)(?:(this)|(ID))\s(is)E
        let mut j = p;
        let mut asserts = None;
        if let Some(e) = self.kw(p, "asserts") {
            let k = self.ws(e);
            if k > e {
                asserts = Some((p, e));
                j = k;
            }
        }
        // `(?!asserts)`: a name that starts with `asserts` is not a predicate.
        if !self.starts(j, "asserts")
            && let Some((name_end, this)) = self.predicate_name(j)
            && self.space_at(name_end)
        {
            let m = self.ws(name_end);
            if self.starts(m, "is") && self.kw_after(m + 2) {
                self.flush(p);
                if let Some((a, b)) = asserts {
                    self.cap(Kind::Top, a, b, &[S::KeywordOperatorTypeAsserts]);
                }
                self.cap(
                    Kind::Top,
                    j,
                    name_end,
                    if this {
                        &[S::VariableParameter, S::VariableLanguageThis]
                    } else {
                        &[S::VariableParameter]
                    },
                );
                self.cap(Kind::Top, m, m + 2, &[S::KeywordOperatorExpressionIs]);
                return Some(m + 2);
            }
        }
        // asserts x
        if let Some(e) = self.kw(p, "asserts") {
            let k = self.ws(e);
            if k > e
                && !self.starts(k, "is")
                && let Some((name_end, this)) = self.predicate_name(k)
                && self.kw_after(name_end)
            {
                self.flush(p);
                self.cap(Kind::Top, p, e, &[S::KeywordOperatorTypeAsserts]);
                self.cap(
                    Kind::Top,
                    k,
                    name_end,
                    if this {
                        &[S::VariableParameter, S::VariableLanguageThis]
                    } else {
                        &[S::VariableParameter]
                    },
                );
                return Some(name_end);
            }
            return self.tok(p, e, S::KeywordOperatorTypeAsserts);
        }
        if let Some(e) = self.kw(p, "is") {
            return self.tok(p, e, S::KeywordOperatorExpressionIs);
        }
        None
    }

    /// `this` or a name, with the keyword guard after it.
    fn predicate_name(&self, j: usize) -> Option<(usize, bool)> {
        if self.starts(j, "this")
            && !self.alnum_at(j + "this".len())
            && self.kw_after(j + "this".len())
        {
            return Some((j + "this".len(), true));
        }
        let e = self.ident(j)?;
        self.kw_after(e).then_some((e, false))
    }
}
