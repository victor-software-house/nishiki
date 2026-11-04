//! Function declarations and expressions, their parameters and return types.

use super::bytes::{BLANK, Bytes, NAME_START, STRING_START};
use super::decl::Pattern;
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
            let e = j + "async".len();
            let k = self.ws(e);
            if k > e && self.starts(k, "function") {
                async_cap = Some((j, e));
                j = k;
            }
        }
        if !self.starts(j, "function") || !self.boundary(j + "function".len()) {
            return None;
        }
        let kw = (j, j + "function".len());
        let mut e = j + "function".len();
        let mut star = None;
        let k = self.ws(e);
        if self.byte(k) == b'*' {
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
            self.capture(kind, a, b, &[S::StorageModifierAsync]);
        }
        self.capture(kind, head.kw.0, head.kw.1, &[S::StorageTypeFunction]);
        if let Some((a, b)) = head.star {
            self.capture(kind, a, b, &[S::KeywordGeneratorAsterisk]);
        }
        if let Some((a, b)) = head.name {
            self.capture(
                kind,
                a,
                b,
                &[S::MetaDefinitionFunction, S::EntityNameFunction],
            );
        }
        self.finish(kind, head.end);
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
        if !self.byte(p).is_ascii_lowercase() || !self.kw_before(p) {
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
        self.token(p, e, &[S::MetaDefinitionFunction, S::EntityNameFunction]);
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
                (self.byte(p) == b'*')
                    .then(|| self.token_rule(p, p + 1, S::KeywordGeneratorAsterisk))?
            })
    }

    pub(crate) fn function_parameters(&mut self, p: usize) -> Option<usize> {
        if self.byte(p) != b'(' {
            return None;
        }
        self.open_token(
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
        if self.byte(j) != b':' || self.full() {
            return None;
        }
        let k = self.ws(j + 1);
        let kind = if self.eol(k) {
            Kind::ReturnType2
        } else {
            Kind::ReturnType1
        };
        self.flush(p);
        self.capture(kind, j, j + 1, &[S::KeywordOperatorTypeAnnotation]);
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
                (self.byte(p) == b',')
                    .then(|| self.token_rule(p, p + 1, S::PunctuationSeparatorParameter))?
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
        let (kind, scope) = match self.byte(j) {
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
            self.capture(kind, a, b, &[S::KeywordOperatorRest]);
        }
        self.capture(kind, j, j + 1, &[scope]);
        self.push(kind);
        Some(j + 1)
    }

    /// The name of a parameter, with the grammar's two function-aware forms.
    ///
    /// `public readonly x`: a modifier followed by another modifier.
    pub(crate) fn parameter_name(&mut self, p: usize) -> Option<usize> {
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
                return self
                    .token_rule(p, e, S::StorageModifier)
                    .map(|r| self.fin_ws(r));
            }
        }
        self.parameter_like(p, ParamStyle::Parameter)
    }

    fn fin_ws(&mut self, e: usize) -> usize {
        self.ws(e)
    }

    /// A parameter, or a field-like name before a function-aware lookahead.
    /// The grammar's modifier group is optional.
    /// When the rest fails after a modifier, the modifier word is tried as the
    /// name, as in `(readonly 42`.
    pub(crate) fn parameter_like(&mut self, p: usize, style: ParamStyle) -> Option<usize> {
        if let Some(e) = self.kw_of(
            p,
            &["override", "public", "private", "protected", "readonly"],
        ) {
            let k = self.ws(e);
            if k > e
                && let Some(r) = self.parameter_after(p, k, Some((p, e)), style)
            {
                return Some(r);
            }
        }
        self.parameter_after(p, p, None, style)
    }

    /// [`Self::parameter_like`] with the name, or its `...`, at `j`, after
    /// the `modifier` if there is one.
    fn parameter_after(
        &mut self,
        p: usize,
        mut j: usize,
        modifier: Option<(usize, usize)>,
        style: ParamStyle,
    ) -> Option<usize> {
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
        let opt = if self.byte(k) == b'?' {
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
                    || self.byte(self.ws(k)) == b','
                    || self.byte(self.ws(k)) == b':'
                    || self.eol(self.ws(k))
            }
        };
        if !function && !lookahead_ok {
            return None;
        }
        self.flush(p);
        if let Some((a, b)) = modifier {
            self.capture(Kind::Top, a, b, &[S::StorageModifier]);
        }
        if let Some((a, b)) = rest {
            self.capture(Kind::Top, a, b, &[S::KeywordOperatorRest]);
        }
        let scopes: &[S] = match (function, this) {
            (true, true) => &[S::EntityNameFunction, S::VariableLanguageThis],
            (true, false) => &[S::EntityNameFunction],
            (false, true) => &[S::VariableParameter, S::VariableLanguageThis],
            (false, false) => &[S::VariableParameter],
        };
        self.capture(Kind::Top, j, name_end, scopes);
        if let Some(at) = opt {
            self.capture(Kind::Top, at, at + 1, &[S::KeywordOperatorOptional]);
            return Some(at + 1);
        }
        Some(name_end)
    }

    fn kw_this(&self, j: usize) -> Option<usize> {
        (self.starts(j, "this") && self.kw_after(j + "this".len())).then_some(j + "this".len())
    }

    pub(crate) fn parameter_binding_element(&mut self, p: usize) -> Option<usize> {
        self.comment(p)
            .or_else(|| self.string(p))
            .or_else(|| self.numeric_literal(p))
            .or_else(|| self.regex(p))
            .or_else(|| self.parameter_pattern(p, Pattern::Object))
            .or_else(|| self.parameter_pattern(p, Pattern::Array))
            .or_else(|| self.destructuring_parameter_rest(p))
            .or_else(|| self.variable_initializer(p))
    }

    /// `(?:(\.\.\.)\s*)?(\{)` or `(\[)` inside a parameter pattern.
    pub(crate) fn parameter_pattern(&mut self, p: usize, pattern: Pattern) -> Option<usize> {
        let mut j = p;
        let mut rest = None;
        if self.starts(j, "...") {
            rest = Some((j, j + 3));
            j = self.ws(j + 3);
        }
        if self.byte(j) != pattern.open() || self.full() {
            return None;
        }
        self.flush(p);
        if let Some((a, b)) = rest {
            self.capture(Kind::Top, a, b, &[S::KeywordOperatorRest]);
        }
        let (kind, scope) = match pattern {
            Pattern::Object => (
                Kind::ParamObjPattern,
                S::PunctuationDefinitionBindingPatternObject,
            ),
            Pattern::Array => (
                Kind::ParamArrPattern,
                S::PunctuationDefinitionBindingPatternArray,
            ),
        };
        self.capture(kind, j, j + 1, &[scope]);
        self.push(kind);
        Some(j + 1)
    }

    fn destructuring_parameter_rest(&mut self, p: usize) -> Option<usize> {
        let mut j = p;
        let mut rest = None;
        if self.starts(j, "...") {
            rest = Some((j, j + 3));
            j = self.ws(j + 3);
        }
        let e = self.ident(j)?;
        self.flush(p);
        if let Some((a, b)) = rest {
            self.capture(Kind::Top, a, b, &[S::KeywordOperatorRest]);
        }
        self.capture(Kind::Top, j, e, &[S::VariableParameter]);
        Some(e)
    }

    pub(crate) fn parameter_object_binding_element(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.comment(p) {
            return Some(r);
        }
        if self.key_colon_ahead(p) {
            return self.open_empty(Kind::ParamObjElem, p);
        }
        self.parameter_pattern(p, Pattern::Object)
            .or_else(|| self.destructuring_parameter_rest(p))
            .or_else(|| self.variable_initializer(p))
            .or_else(|| self.punctuation_comma(p))
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
