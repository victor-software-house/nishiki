//! Function declarations and expressions, their parameters and return types.

use super::bytes::{BLANK, Bytes, NAME_START, STRING_START};
use super::kind::Kind;
use super::lexer::Lexer;
use super::scope::S;

impl<const D: u8> Lexer<'_, D> {
    /// `(async\s+)?(function)\b(?:\s*(\*))?(?:(?:\s+|(?<=\*))(ID))?\s*` at
    /// `at`: the pieces of the head.
    fn function_head(&self, at: usize) -> Option<FunctionHead> {
        let mut j = at;
        let mut async_cap = None;
        if self.starts(j, "async") {
            let e = j + 5;
            let k = self.ws(e);
            if k > e && self.starts(k, "function") {
                async_cap = Some((j, e));
                j = k;
            }
        }
        if !self.starts(j, "function") || !self.boundary(j + 8) {
            return None;
        }
        let kw = (j, j + 8);
        let mut e = j + 8;
        let mut star = None;
        let k = self.ws(e);
        if self.b(k) == b'*' {
            star = Some((k, k + 1));
            e = k + 1;
        }
        let mut name = None;
        let n = self.ws(e);
        if (n > e || star.is_some())
            && let Some(ne) = self.ident(n)
        {
            name = Some((n, ne));
            e = ne;
        }
        let end = self.ws(e);
        Some(FunctionHead {
            async_cap,
            kw,
            star,
            name,
            end,
        })
    }

    fn emit_function_head(&mut self, kind: Kind, head: &FunctionHead) {
        if let Some((a, b)) = head.async_cap {
            self.cap(kind, a, b, &[S::StorageModifierAsync]);
        }
        self.cap(kind, head.kw.0, head.kw.1, &[S::StorageTypeFunction]);
        if let Some((a, b)) = head.star {
            self.cap(kind, a, b, &[S::KeywordGeneratorAsterisk]);
        }
        if let Some((a, b)) = head.name {
            self.cap(
                kind,
                a,
                b,
                &[S::MetaDefinitionFunction, S::EntityNameFunction],
            );
        }
        self.fin(kind, head.end);
    }

    pub(crate) fn function_declaration(&mut self, p: usize) -> Option<usize> {
        let pre = self.declaration_start(p)?;
        let head = self.function_head(pre.at)?;
        if self.full() {
            return None;
        }
        self.flush(p);
        self.prefix_tokens(Kind::FuncDecl, &pre);
        self.emit_function_head(Kind::FuncDecl, &head);
        self.push(Kind::FuncDecl);
        Some(head.end)
    }

    pub(crate) fn function_expression(&mut self, p: usize) -> Option<usize> {
        if !self.b(p).is_ascii_lowercase() || !self.kw_before(p) {
            return None;
        }
        let head = self.function_head(p)?;
        if self.full() {
            return None;
        }
        self.flush(p);
        self.emit_function_head(Kind::FuncExpr, &head);
        self.push(Kind::FuncExpr);
        Some(head.end)
    }

    pub(crate) fn function_name(&mut self, p: usize) -> Option<usize> {
        let e = self.ident(p)?;
        self.one(p, e, &[S::MetaDefinitionFunction, S::EntityNameFunction]);
        Some(e)
    }

    pub(crate) fn function_body(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.comment(p) {
            return Some(r);
        }
        self.type_parameters(p)
            .or_else(|| self.function_parameters(p))
            .or_else(|| self.return_type(p))
            .or_else(|| self.type_function_return_type(p))
            .or_else(|| self.decl_block(p))
            .or_else(|| {
                (self.b(p) == b'*').then(|| self.tok(p, p + 1, S::KeywordGeneratorAsterisk))?
            })
    }

    pub(crate) fn function_parameters(&mut self, p: usize) -> Option<usize> {
        if self.b(p) != b'(' {
            return None;
        }
        self.open1(
            Kind::Params,
            p,
            p + 1,
            &[S::PunctuationDefinitionParametersBegin],
        )
    }

    /// `(?<=\))\s*(:)`.
    pub(crate) fn return_type(&mut self, p: usize) -> Option<usize> {
        if self.prev_char(p) != Some(')') {
            return None;
        }
        let j = self.ws(p);
        if self.b(j) != b':' || self.full() {
            return None;
        }
        let k = self.ws(j + 1);
        let kind = if self.eol(k) {
            Kind::ReturnType2
        } else {
            Kind::ReturnType1
        };
        self.flush(p);
        self.cap(kind, j, j + 1, &[S::KeywordOperatorTypeAnnotation]);
        self.push(kind);
        Some(j + 1)
    }

    pub(crate) fn function_parameters_body(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.comment(p) {
            return Some(r);
        }
        self.only("p.string", STRING_START, p, Self::string)
            .or_else(|| self.only("p.decorator", Bytes::of(b"@"), p, Self::decorator))
            .or_else(|| {
                self.only(
                    "p.destructuring",
                    BLANK.or(Bytes::of(b".[{")),
                    p,
                    Self::destructuring_parameter,
                )
            })
            .or_else(|| self.only("p.name", NAME_START.with(b'.'), p, Self::parameter_name))
            .or_else(|| {
                self.only(
                    "p.type_annotation",
                    Bytes::of(b":"),
                    p,
                    Self::parameter_type_annotation,
                )
            })
            .or_else(|| {
                self.only(
                    "p.initializer",
                    Bytes::of(b"="),
                    p,
                    Self::variable_initializer,
                )
            })
            .or_else(|| {
                (self.b(p) == b',').then(|| self.tok(p, p + 1, S::PunctuationSeparatorParameter))?
            })
    }

    /// `(?<![:=])\s*(?:(\.\.\.)\s*)?(\{)` and its array form.
    pub(crate) fn destructuring_parameter(&mut self, p: usize) -> Option<usize> {
        if matches!(self.prev_char(p), Some(':' | '=')) || self.full() {
            return None;
        }
        let mut j = self.ws(p);
        let mut rest = None;
        if self.starts(j, "...") {
            rest = Some((j, j + 3));
            j = self.ws(j + 3);
        }
        let (kind, scope) = match self.b(j) {
            b'{' => (
                Kind::ParamObjBind,
                S::PunctuationDefinitionBindingPatternObject,
            ),
            b'[' => (
                Kind::ParamArrBind,
                S::PunctuationDefinitionBindingPatternArray,
            ),
            _ => return None,
        };
        self.flush(p);
        if let Some((a, b)) = rest {
            self.cap(kind, a, b, &[S::KeywordOperatorRest]);
        }
        self.cap(kind, j, j + 1, &[scope]);
        self.push(kind);
        Some(j + 1)
    }

    /// The name of a parameter, with the grammar's two function-aware forms.
    pub(crate) fn parameter_name(&mut self, p: usize) -> Option<usize> {
        // `public readonly x`: a modifier followed by another modifier.
        if let Some(e) = self.kw_of(
            p,
            &["override", "public", "protected", "private", "readonly"],
        ) {
            let j = self.ws(e);
            if j > e
                && ["override", "public", "protected", "private", "readonly"]
                    .iter()
                    .any(|w| self.starts(j, w))
            {
                return self.tok(p, e, S::StorageModifier).map(|r| self.fin_ws(r));
            }
        }
        self.parameter_like(p, ParamStyle::Parameter)
    }

    fn fin_ws(&mut self, e: usize) -> usize {
        self.ws(e)
    }

    /// A parameter, or a field-like name before a function-aware lookahead.
    pub(crate) fn parameter_like(&mut self, p: usize, style: ParamStyle) -> Option<usize> {
        let mut j = p;
        let mut modifier = None;
        if let Some(e) = self.kw_of(
            p,
            &["override", "public", "private", "protected", "readonly"],
        ) {
            let k = self.ws(e);
            if k > e {
                modifier = Some((p, e));
                j = k;
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
        let (name_end, this) = if let Some(e) = self.kw_this(j) {
            (e, true)
        } else {
            let e = self.ident(j)?;
            if !self.kw_after(e) {
                return None;
            }
            (e, false)
        };
        let mut k = self.ws(name_end);
        let opt = if self.b(k) == b'?' {
            let at = k;
            k += 1;
            Some(at)
        } else {
            None
        };
        let function = self.named_fn_ahead(k);
        let lookahead_ok = match style {
            ParamStyle::Parameter => true,
            ParamStyle::ExpressionParameter => {
                function
                    || self.b(self.ws(k)) == b','
                    || self.b(self.ws(k)) == b':'
                    || self.eol(self.ws(k))
            }
        };
        if !function && !lookahead_ok {
            return None;
        }
        self.flush(p);
        if let Some((a, b)) = modifier {
            self.cap(Kind::Top, a, b, &[S::StorageModifier]);
        }
        if let Some((a, b)) = rest {
            self.cap(Kind::Top, a, b, &[S::KeywordOperatorRest]);
        }
        let scopes: &[S] = match (function, this) {
            (true, true) => &[S::EntityNameFunction, S::VariableLanguageThis],
            (true, false) => &[S::EntityNameFunction],
            (false, true) => &[S::VariableParameter, S::VariableLanguageThis],
            (false, false) => &[S::VariableParameter],
        };
        self.cap(Kind::Top, j, name_end, scopes);
        if let Some(at) = opt {
            self.cap(Kind::Top, at, at + 1, &[S::KeywordOperatorOptional]);
            return Some(at + 1);
        }
        Some(name_end)
    }

    fn kw_this(&self, j: usize) -> Option<usize> {
        (self.starts(j, "this") && self.kw_after(j + 4)).then_some(j + 4)
    }
}

/// The pieces of `function name` that begin a function.
pub(crate) struct FunctionHead {
    async_cap: Option<(usize, usize)>,
    kw: (usize, usize),
    star: Option<(usize, usize)>,
    name: Option<(usize, usize)>,
    end: usize,
}

/// How a parameter-like name decides whether it names a function.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ParamStyle {
    /// A parameter of a function.
    Parameter,
    /// A parameter inside parentheses that may turn out to be arrow
    /// parameters.
    ExpressionParameter,
}
