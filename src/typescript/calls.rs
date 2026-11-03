//! Calls, arrow functions, casts and the expression forms that open a context.

use super::bytes::{Bytes, LITERAL_START, NAME_START};
use super::chars::Before;
use super::kind::Kind;
use super::lexer::Lexer;
use super::look::CallKind;
use super::scope::S;

/// The declaration keywords that end the head of an arrow function at the start
/// of a line.
const ARROW_END_WORDS: [&str; 14] = [
    "export",
    "function",
    "class",
    "interface",
    "let",
    "var",
    "const",
    "import",
    "enum",
    "namespace",
    "module",
    "type",
    "abstract",
    "declare",
];

impl<const D: u8> Lexer<'_, D> {
    pub(crate) fn function_call(&mut self, p: usize) -> Option<usize> {
        match self.call_kind(p)? {
            CallKind::Call => self.open_empty(Kind::CallOuter, p),
            CallKind::Open => self.open_empty(Kind::Call2Outer, p),
        }
    }

    /// The rules inside a call: its target, comments, optional-chaining marks,
    /// type arguments and the argument list.
    pub(crate) fn call_outer_rules(&mut self, p: usize) -> Option<usize> {
        if self.call_target_start(p) {
            return self.open_empty(Kind::CallTarget, p);
        }
        self.comment(p)
            .or_else(|| self.call_optionals(p))
            .or_else(|| self.type_arguments(p))
            .or_else(|| self.paren_expression(p))
    }

    pub(crate) fn call2_outer_rules(&mut self, p: usize) -> Option<usize> {
        if self.call_target_start(p) {
            return self.open_empty(Kind::Call2Target, p);
        }
        self.comment(p)
            .or_else(|| self.call_optionals(p))
            .or_else(|| self.type_arguments(p))
    }

    fn call_optionals(&mut self, p: usize) -> Option<usize> {
        if self.starts(p, "?.") {
            self.token(
                p,
                p + 2,
                &[S::MetaFunctionCall, S::PunctuationAccessorOptional],
            );
            return Some(p + 2);
        }
        if self.byte(p) == b'!' {
            self.token(
                p,
                p + 1,
                &[S::MetaFunctionCall, S::KeywordOperatorDefiniteassignment],
            );
            return Some(p + 1);
        }
        None
    }

    /// The rules inside a call target: names, accessors and literals, and the
    /// called name itself.
    pub(crate) fn call_target_rules(&mut self, p: usize) -> Option<usize> {
        self.support_function_call_identifiers(p).or_else(|| {
            let e = self.hash_ident(p)?;
            self.token_rule(p, e, S::EntityNameFunction)
        })
    }

    pub(crate) fn support_function_call_identifiers(&mut self, p: usize) -> Option<usize> {
        self.only("f.literal", LITERAL_START, p, Self::literal)
            .or_else(|| {
                self.only(
                    "f.support_objects",
                    Bytes::of(b"aPinem.?"),
                    p,
                    Self::support_objects,
                )
            })
            .or_else(|| {
                self.only(
                    "f.object_identifiers",
                    NAME_START.or(Bytes::of(b".?")),
                    p,
                    Self::object_identifiers,
                )
            })
            .or_else(|| {
                self.only(
                    "f.accessor",
                    Bytes::of(b".?"),
                    p,
                    Self::punctuation_accessor,
                )
            })
            .or_else(|| {
                self.only("f.import", Bytes::of(b"i"), p, |s, p| {
                    let e = s.kw(p, "import")?;
                    let j = s.ws(e);
                    let quoted = matches!(s.byte(s.ws(j + 1)), b'"' | b'\'' | b'`');
                    (s.byte(j) == b'(' && quoted)
                        .then(|| s.token_rule(p, e, S::KeywordOperatorExpressionImport))?
                })
            })
    }

    /// `<` of type arguments.
    pub(crate) fn type_arguments(&mut self, p: usize) -> Option<usize> {
        if self.byte(p) != b'<' {
            return None;
        }
        self.open_token(
            Kind::TypeArgs,
            p,
            p + 1,
            &[S::PunctuationDefinitionTypeparametersBegin],
        )
    }

    pub(crate) fn arrow_function(&mut self, p: usize) -> Option<usize> {
        if self.byte(p) == b'=' && self.byte(p + 1) == b'>' {
            return self.open_token(Kind::ArrowBody, p, p + 2, &[S::StorageTypeFunctionArrow]);
        }
        self.arrow_single_parameter(p)
            .or_else(|| self.arrow_head(p))
    }

    /// `async? name =>`.
    fn arrow_single_parameter(&mut self, p: usize) -> Option<usize> {
        if self.starts(p, "async") && self.kw_before(p) && self.boundary(p) {
            let e = p + "async".len();
            let j = self.ws(e);
            if j > e
                && let Some(ie) = self.ident(j)
            {
                let k = self.ws(ie);
                if self.starts(k, "=>") {
                    self.flush(p);
                    self.capture(Kind::ArrowHead, p, e, &[S::StorageModifierAsync]);
                    self.capture(Kind::ArrowHead, j, ie, &[S::VariableParameter]);
                    self.finish(Kind::ArrowHead, k);
                    return Some(k);
                }
            }
        }
        let ie = self.ident(p)?;
        let k = self.ws(ie);
        if !self.starts(k, "=>") {
            return None;
        }
        self.flush(p);
        self.capture(Kind::ArrowHead, p, ie, &[S::VariableParameter]);
        self.finish(Kind::ArrowHead, k);
        Some(k)
    }

    /// The parameters of an arrow function, once the lookahead sees the arrow.
    fn arrow_head(&mut self, p: usize) -> Option<usize> {
        let mut j = p;
        let mut async_end = None;
        if self.starts(p, "async") && self.kw_before(p) && self.boundary(p) {
            j = p + "async".len();
            async_end = Some(j);
        }
        let blocked = matches!(self.prev_char(j), Some(']' | '!' | ')' | '}'));
        if blocked {
            return None;
        }
        let k = self.ws(j);
        if !self.arrow_ahead(k) || self.full() {
            return None;
        }
        self.flush(p);
        if let Some(e) = async_end {
            self.capture(Kind::ArrowHead, p, e, &[S::StorageModifierAsync]);
        }
        self.finish(Kind::ArrowHead, k);
        self.push(Kind::ArrowHead);
        Some(k)
    }

    /// The end of an arrow function's head: the arrow, a brace, or a line that
    /// starts a declaration.
    pub(crate) fn arrow_end_at(&self, p: usize) -> bool {
        if self.starts(p, "=>") || self.byte(p) == b'{' {
            return true;
        }
        if !Self::bol(p) {
            return false;
        }
        let j = self.ws(0);
        let after = ARROW_END_WORDS
            .iter()
            .find(|word| self.starts(j, word))
            .map(|word| j + word.len())
            .or_else(|| self.using_end(j))
            .or_else(|| self.await_using_end(j));
        after.is_some_and(|e| self.ws(e) > e)
    }

    /// The end of an arrow function's body: after the first token that is not
    /// a brace, once the arrow and its blanks are behind.
    pub(crate) fn arrow_body_end_at(&self, p: usize) -> bool {
        let prev = self.prev_char(p);
        let after_token = prev.is_some_and(|c| c == '}' || !c.is_whitespace())
            && !(p >= 2 && self.bytes.get(p - 2..p) == Some(b"=>"));
        let before_token =
            self.byte(p) != b'{' && !self.space_at(p) && self.byte(p) != 0 && p < self.len;
        (after_token || before_token)
            && !(self.byte(p) == b'/' && matches!(self.byte(p + 1), b'*' | b'/'))
    }

    /// `(?<=[(,=])\s*(async)?(?=…)`: a parenthesis that the line leaves open,
    /// which may be arrow parameters.
    ///
    /// The byte before `p` tells which of the rule's lookbehinds can hold: `(`,
    /// `,` and `=`, the `>` of `=>`, the `n` of `return`, and the `)` of a
    /// return type. Any other byte, anywhere but the line's start, rules them
    /// all out.
    pub(crate) fn paren_expression_possibly_arrow(&mut self, p: usize) -> Option<usize> {
        if self.full() {
            return None;
        }
        let before = p.checked_sub(1).and_then(|i| self.bytes.get(i)).copied();
        if p != 0 && !matches!(before, Some(b'(' | b',' | b'=' | b'>' | b'n' | b')')) {
            return None;
        }
        let prev = self.prev_char(p);
        let first = prev.is_some_and(|c| matches!(c, '(' | ',' | '='));
        let second = first
            || self.bytes.get(..p).is_some_and(|h| h.ends_with(b"=>"))
            || self.return_before(p);
        if !second {
            return self.possibly_arrow_return_type(p);
        }
        let j = self.ws(p);
        let mut ends = [None, Some(j)];
        if self.starts(j, "async") {
            ends[0] = Some(j + "async".len());
        }
        if first {
            for e in ends.into_iter().flatten() {
                if self.paren_open_ahead(self.ws(e)) {
                    return Some(self.possibly_arrow_open(p, j, e));
                }
            }
        }
        for e in ends.into_iter().flatten() {
            if self.line_open_ahead(self.ws(e)) {
                return Some(self.possibly_arrow_open(p, j, e));
            }
        }
        self.possibly_arrow_return_type(p)
    }

    fn possibly_arrow_open(&mut self, p: usize, j: usize, e: usize) -> usize {
        self.flush(p);
        if e > j {
            self.capture(Kind::Top, j, e, &[S::StorageModifierAsync]);
        }
        self.push(Kind::PossArrow);
        e
    }

    /// `(<…>)?\(` then the line ends, or a destructured parameter leaves it
    /// open.
    fn line_open_ahead(&self, i: usize) -> bool {
        let at_paren = self.tp_paren(i).is_some_and(|j| self.eol(self.ws(j)));
        let lone = self.byte(i) == b'<' && self.eol(self.ws(i + 1));
        let generic =
            self.byte(i) == b'<' && self.arrow_type_params(i).is_some_and(|e| self.eol(e));
        at_paren || lone || generic
    }

    /// `^return` or `[^$._[:alnum:]]return` ending at `p`.
    pub(crate) fn return_before(&self, p: usize) -> bool {
        self.word_before_text(p, "return")
    }

    /// `(?<=\)|^)\s*(:)(?=\s*TYPE+\s*=>)`: the return type of arrow parameters
    /// that began on an earlier line.
    pub(crate) fn possibly_arrow_return_type(&mut self, p: usize) -> Option<usize> {
        if !(Self::bol(p) || self.prev_char(p) == Some(')')) || self.full() {
            return None;
        }
        let j = self.ws(p);
        if self.byte(j) != b':' {
            return None;
        }
        if !self.arrow_return_ahead(self.ws(j + 1)) {
            return None;
        }
        self.flush(p);
        self.capture(
            Kind::PossArrowRet,
            j,
            j + 1,
            &[
                S::MetaArrow,
                S::MetaReturnTypeArrow,
                S::KeywordOperatorTypeAnnotation,
            ],
        );
        self.push(Kind::PossArrowRet);
        Some(j + 1)
    }

    /// A cast, `<T>` or `<const>` before an expression.
    ///
    /// The JSX grammars replace the rule with the JSX one, which has already
    /// been tried at this position.
    pub(crate) fn cast(&mut self, p: usize) -> Option<usize> {
        if Self::DIALECT.jsx() {
            return None;
        }
        let j = self.ws(p);
        if self.byte(j) != b'<' {
            return None;
        }
        let k = self.ws(j + 1);
        if self.starts(k, "const") {
            let m = self.ws(k + "const".len());
            if self.byte(m) == b'>' {
                self.flush(p);
                self.capture(Kind::Cast, j, j + 1, &[S::MetaBraceAngle]);
                self.capture(Kind::Cast, k, k + "const".len(), &[S::StorageModifier]);
                self.capture(Kind::Cast, m, m + 1, &[S::MetaBraceAngle]);
                return Some(m + 1);
            }
        }
        if self.full() {
            return None;
        }
        let open =
            self.byte(j + 1) != b'=' && !self.starts(j + 1, "<=") && !self.eol(self.ws(j + 1));
        if open && self.cast_before(p) {
            self.flush(p);
            self.capture(Kind::Cast, j, j + 1, &[S::MetaBraceAngle]);
            self.push(Kind::Cast);
            return Some(j + 1);
        }
        if Self::bol(p) {
            let e = self.ident(j + 1);
            if e.is_some_and(|e| self.byte(self.ws(e)) == b'>') {
                self.flush(p);
                self.capture(Kind::Cast, j, j + 1, &[S::MetaBraceAngle]);
                self.push(Kind::Cast);
                return Some(j + 1);
            }
        }
        None
    }

    /// The lookbehind of a cast: an operator, or a keyword that takes an
    /// operand, not a `++` or `--` suffix.
    ///
    /// `[^+]\+` and `[^-]-`: a lone plus or minus, with a character before it.
    fn cast_before(&self, p: usize) -> bool {
        const CAST: Before = Before {
            bytes: Bytes::of(b"&(*,:=>?^|"),
            operators: &[],
            words: &["return", "throw", "yield", "await", "default"],
            never: &["++", "--"],
        };
        let lone_sign = p >= 2
            && matches!(self.bytes.get(p - 1), Some(b'+' | b'-'))
            && self.bytes.get(p - 2) != self.bytes.get(p - 1);
        self.operator_before(p, &CAST) || lone_sign
    }

    pub(crate) fn ternary_expression(&mut self, p: usize) -> Option<usize> {
        if self.byte(p) != b'?' || self.byte(p + 1) == b'?' {
            return None;
        }
        if self.byte(p + 1) == b'.' && !self.byte(p + 2).is_ascii_digit() {
            return None;
        }
        self.open_token(Kind::Ternary, p, p + 1, &[S::KeywordOperatorTernary])
    }

    pub(crate) fn new_expr(&mut self, p: usize) -> Option<usize> {
        let e = self.kw(p, "new")?;
        self.open_token(Kind::NewExpr, p, e, &[S::KeywordOperatorNew])
    }

    pub(crate) fn instanceof_expr(&mut self, p: usize) -> Option<usize> {
        let e = self.kw(p, "instanceof")?;
        self.open_token(
            Kind::InstanceOf,
            p,
            e,
            &[S::KeywordOperatorExpressionInstanceof],
        )
    }
}
