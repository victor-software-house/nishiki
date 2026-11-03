//! Statements and declarations.

use super::bytes::{ANY, BLANK, Bytes, COMMENT_START, LOWER, NAME_START, STRING_START};
use super::chars::Before;
use super::kind::Kind;
use super::lexer::Lexer;
use super::scope::S;

/// `(?:\b(export)\s+)?(?:\b(declare)\s+)?` at a position.
pub(crate) struct Prefix {
    export: Option<(usize, usize)>,
    declare: Option<(usize, usize)>,
    /// Where the declaration keyword starts.
    pub(crate) at: usize,
}

/// Whether the names a binding pattern declares are constants, which the
/// grammar scopes apart. A `using` declaration has no patterns, so a
/// [`VarKind`] would carry a case no pattern meets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Binding {
    /// `let`, `var` and parameters.
    Variable,
    /// `const`.
    Constant,
}

/// Which bracket a binding pattern opens with.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Pattern {
    /// `{`.
    Object,
    /// `[`.
    Array,
}

impl Pattern {
    /// The bracket that opens the pattern.
    pub(crate) fn open(self) -> u8 {
        match self {
            Pattern::Object => b'{',
            Pattern::Array => b'[',
        }
    }
}

/// The three declaration keyword forms the grammar keeps apart.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum VarKind {
    Let,
    Const,
    Using,
}

impl<const D: u8> Lexer<'_, D> {
    pub(crate) fn statements(&mut self, p: usize) -> Option<usize> {
        self.only("s.declaration", LOWER.with(b'@'), p, Self::declaration)
            .or_else(|| {
                self.only(
                    "s.control",
                    Bytes::of(b"bcdefgiprstw"),
                    p,
                    Self::control_statement,
                )
            })
            .or_else(|| {
                self.only(
                    "s.opblock",
                    BLANK.with(b'{'),
                    p,
                    Self::after_operator_block_as_object_literal,
                )
            })
            .or_else(|| self.only("s.block", Bytes::of(b"{"), p, Self::decl_block))
            .or_else(|| self.only("s.label", NAME_START, p, Self::label))
            .or_else(|| self.only("s.expression", ANY, p, Self::expression))
            .or_else(|| {
                self.only(
                    "s.semicolon",
                    Bytes::of(b";"),
                    p,
                    Self::punctuation_semicolon,
                )
            })
            .or_else(|| self.only("s.string", STRING_START, p, Self::string))
            .or_else(|| self.only("s.comment", COMMENT_START, p, Self::comment))
    }

    pub(crate) fn declaration(&mut self, p: usize) -> Option<usize> {
        self.only("d.decorator", Bytes::of(b"@"), p, Self::decorator)
            .or_else(|| self.only("d.var", Bytes::of(b"vlcuaed"), p, Self::var_expr))
            .or_else(|| {
                self.only(
                    "d.function",
                    Bytes::of(b"afed"),
                    p,
                    Self::function_declaration,
                )
            })
            .or_else(|| self.only("d.class", Bytes::of(b"aced"), p, Self::class_declaration))
            .or_else(|| {
                self.only(
                    "d.interface",
                    Bytes::of(b"aied"),
                    p,
                    Self::interface_declaration,
                )
            })
            .or_else(|| self.only("d.enum", Bytes::of(b"ced"), p, Self::enum_declaration))
            .or_else(|| {
                self.only(
                    "d.namespace",
                    Bytes::of(b"nmed"),
                    p,
                    Self::namespace_declaration,
                )
            })
            .or_else(|| self.only("d.type", Bytes::of(b"ted"), p, Self::type_alias_declaration))
            .or_else(|| {
                self.only(
                    "d.importeq",
                    Bytes::of(b"ied"),
                    p,
                    Self::import_equals_declaration,
                )
            })
            .or_else(|| self.only("d.import", Bytes::of(b"ied"), p, Self::import_declaration))
            .or_else(|| self.only("d.export", Bytes::of(b"e"), p, Self::export_declaration))
            .or_else(|| {
                self.only("d.modifier", Bytes::of(b"de"), p, |s, p| {
                    let e = s.kw_of(p, &["declare", "export"])?;
                    s.token_rule(p, e, S::StorageModifier)
                })
            })
    }

    pub(crate) fn decorator(&mut self, p: usize) -> Option<usize> {
        if self.byte(p) != b'@' || !self.kw_before(p) {
            return None;
        }
        self.open_token(Kind::Decorator, p, p + 1, &[S::PunctuationDecorator])
    }

    pub(crate) fn decl_block(&mut self, p: usize) -> Option<usize> {
        if self.byte(p) != b'{' {
            return None;
        }
        self.open_token(Kind::Block, p, p + 1, &[S::PunctuationDefinitionBlock])
    }

    /// `(?<!\+\+|--)(?<=[!(+,:=>?\[]|^await|…|&&|\|\||\*)\s*(\{)`.
    pub(crate) fn after_operator_block_as_object_literal(&mut self, p: usize) -> Option<usize> {
        let j = self.ws(p);
        if self.byte(j) != b'{' || self.full() || !self.object_literal_before(p) {
            return None;
        }
        self.flush(p);
        self.capture(Kind::ObjLit, j, j + 1, &[S::PunctuationDefinitionBlock]);
        self.push(Kind::ObjLit);
        Some(j + 1)
    }

    fn object_literal_before(&self, p: usize) -> bool {
        const OBJECT_LITERAL: Before = Before {
            bytes: Bytes::of(b"!(+,:=>?[*"),
            operators: &["&&", "||"],
            words: &["await", "return", "yield", "throw", "in", "of", "typeof"],
            never: &["++", "--"],
        };
        self.operator_before(p, &OBJECT_LITERAL)
    }

    pub(crate) fn label(&mut self, p: usize) -> Option<usize> {
        let e = self.ident(p)?;
        let j = self.ws(e);
        if self.byte(j) != b':' {
            return None;
        }
        let k = self.ws(j + 1);
        if self.byte(k) == b'{' && !self.full() {
            self.flush(p);
            self.capture(Kind::LabelBlock, p, e, &[S::EntityNameLabel]);
            self.capture(Kind::LabelBlock, j, j + 1, &[S::PunctuationSeparatorLabel]);
            self.push(Kind::LabelBlock);
            return Some(j + 1);
        }
        self.token(p, e, &[S::EntityNameLabel]);
        self.capture(Kind::Top, j, j + 1, &[S::PunctuationSeparatorLabel]);
        Some(j + 1)
    }

    /// `(?:\b(export)\s+)?(?:\b(declare)\s+)?` at `p`.
    pub(crate) fn decl_prefix(&self, p: usize) -> Prefix {
        let mut at = p;
        let mut export = None;
        let mut declare = None;
        if self.starts(at, "export") && self.boundary(at) {
            let e = at + "export".len();
            let k = self.ws(e);
            if k > e {
                export = Some((at, e));
                at = k;
            }
        }
        if self.starts(at, "declare") && self.boundary(at) {
            let e = at + "declare".len();
            let k = self.ws(e);
            if k > e {
                declare = Some((at, e));
                at = k;
            }
        }
        Prefix {
            export,
            declare,
            at,
        }
    }

    /// Emits the `export` and `declare` captures of a begin match.
    pub(crate) fn prefix_tokens(&mut self, kind: Kind, pre: &Prefix) {
        if let Some((a, b)) = pre.export {
            self.capture(kind, a, b, &[S::KeywordControlExport]);
        }
        if let Some((a, b)) = pre.declare {
            self.capture(kind, a, b, &[S::StorageModifier]);
        }
    }

    /// The prefix of a declaration begin, once the guard at `p` holds.
    pub(crate) fn declaration_start(&self, p: usize) -> Option<Prefix> {
        if !self.byte(p).is_ascii_lowercase() || !self.kw_before(p) {
            return None;
        }
        Some(self.decl_prefix(p))
    }

    pub(crate) fn var_expr(&mut self, p: usize) -> Option<usize> {
        if !self.byte(p).is_ascii_lowercase() {
            return None;
        }
        if self.var_keyword(p, VarKind::Let).is_some() {
            return self.open_empty(Kind::VarLet, p);
        }
        if self.var_keyword(p, VarKind::Const).is_some() {
            return self.open_empty(Kind::VarConst, p);
        }
        if self.var_keyword(p, VarKind::Using).is_some() {
            return self.open_empty(Kind::VarUsing, p);
        }
        None
    }

    /// `B(?:\b(export)\s+)?(?:\b(declare)\s+)?\b(var|let)E` and its `const`
    /// and `using` forms: the prefix and the end of the keyword.
    pub(crate) fn var_keyword(&self, p: usize, which: VarKind) -> Option<(Prefix, usize)> {
        let pre = self.declaration_start(p)?;
        let k = pre.at;
        let e = match which {
            VarKind::Let => {
                if !self.boundary(k) {
                    return None;
                }
                if self.starts(k, "var") || self.starts(k, "let") {
                    k + "var".len()
                } else {
                    return None;
                }
            }
            VarKind::Const => {
                if !self.starts(k, "const") || !self.boundary(k) {
                    return None;
                }
                let e = k + "const".len();
                let j = self.ws(e);
                if j > e && self.starts(j, "enum") && self.boundary(j + "enum".len()) {
                    return None;
                }
                e
            }
            VarKind::Using => self.await_using_end(k).or_else(|| self.using_end(k))?,
        };
        self.kw_after(e).then_some((pre, e))
    }

    /// `B (of|in)\s+`.
    pub(crate) fn of_in_ahead(&self, p: usize) -> bool {
        if !self.kw_before(p) {
            return false;
        }
        ["of", "in"]
            .iter()
            .any(|w| self.starts(p, w) && self.ws(p + 2) > p + 2)
    }

    /// The `var`, `let` or `const` keyword that begins a declaration, with its
    /// trailing blanks.
    pub(crate) fn var_kw_rule(&mut self, p: usize, which: VarKind) -> Option<usize> {
        let (pre, e) = self.var_keyword(p, which)?;
        if self.full() {
            return None;
        }
        self.flush(p);
        self.prefix_tokens(Kind::VarKw, &pre);
        self.capture(Kind::VarKw, pre.at, e, &[S::StorageType]);
        let j = self.ws(e);
        self.finish(Kind::VarKw, j);
        self.push(Kind::VarKw);
        Some(j)
    }

    /// `(,)\s*(?=$|//)` after a declaration.
    pub(crate) fn var_comma(&mut self, p: usize, kind: Kind) -> Option<usize> {
        if self.byte(p) != b',' || self.full() {
            return None;
        }
        let j = self.ws(p + 1);
        if !(self.eol(j) || self.starts(j, "//")) {
            return None;
        }
        let end = if self.eol(j) { p + 1 } else { j };
        self.flush(p);
        self.capture(kind, p, p + 1, &[S::PunctuationSeparatorComma]);
        self.finish(kind, end);
        self.push(kind);
        Some(end)
    }

    /// The name rules of a `let` or `var` declaration.
    pub(crate) fn var_single_variable(&mut self, p: usize) -> Option<usize> {
        if self.full() {
            return None;
        }
        let e = self.ident(p)?;
        let mut k = e;
        let mut bang = None;
        if self.byte(k) == b'!' {
            bang = Some(k);
            k += 1;
        }
        if self.named_fn_ahead(k) {
            return Some(self.var_single_begin(
                p,
                e,
                bang,
                k,
                &[S::MetaDefinitionVariable, S::EntityNameFunction],
            ));
        }
        if let Some(c) = self.constant_name(p) {
            let mut k = c;
            let mut bang = None;
            if self.byte(k) == b'!' {
                bang = Some(k);
                k += 1;
            }
            return Some(self.var_single_begin(
                p,
                c,
                bang,
                k,
                &[S::MetaDefinitionVariable, S::VariableOtherConstant],
            ));
        }
        Some(self.var_single_begin(
            p,
            e,
            bang,
            k,
            &[S::MetaDefinitionVariable, S::VariableOtherReadwrite],
        ))
    }

    /// The name rule of a `const` declaration.
    pub(crate) fn var_single_const(&mut self, p: usize) -> Option<usize> {
        if self.full() {
            return None;
        }
        let e = self.ident(p)?;
        if self.named_fn_ahead(e) {
            return Some(self.var_single_begin(
                p,
                e,
                None,
                e,
                &[
                    S::MetaDefinitionVariable,
                    S::VariableOtherConstant,
                    S::EntityNameFunction,
                ],
            ));
        }
        Some(self.var_single_begin(
            p,
            e,
            None,
            e,
            &[S::MetaDefinitionVariable, S::VariableOtherConstant],
        ))
    }

    fn var_single_begin(
        &mut self,
        p: usize,
        e: usize,
        bang: Option<usize>,
        end: usize,
        scopes: &[S],
    ) -> usize {
        self.flush(p);
        self.capture(Kind::VarSingle, p, e, scopes);
        if let Some(b) = bang {
            self.capture(
                Kind::VarSingle,
                b,
                b + 1,
                &[S::KeywordOperatorDefiniteassignment],
            );
        }
        self.push(Kind::VarSingle);
        end
    }

    /// `(?<![:=]|^of|[^$._[:alnum:]]of|^in|[^$._[:alnum:]]in)\s*(?=\{)` and
    /// its array form.
    pub(crate) fn destructuring_variable(&mut self, p: usize, binding: Binding) -> Option<usize> {
        let j = self.ws(p);
        let open = self.byte(j);
        if !matches!(open, b'{' | b'[') || self.full() {
            return None;
        }
        if matches!(self.prev_char(p), Some(':' | '='))
            || self.word_before_text(p, "of")
            || self.word_before_text(p, "in")
        {
            return None;
        }
        let kind = match (open, binding) {
            (b'{', Binding::Variable) => Kind::ObjBindVar,
            (b'{', Binding::Constant) => Kind::ObjBindVarConst,
            (_, Binding::Variable) => Kind::ArrBindVar,
            (_, Binding::Constant) => Kind::ArrBindVarConst,
        };
        self.flush(p);
        self.finish(kind, j);
        self.push(kind);
        Some(j)
    }

    /// `(?:(\.\.\.)\s*)?(\{)` or `(\[)`: a binding pattern.
    pub(crate) fn binding_pattern(
        &mut self,
        p: usize,
        pattern: Pattern,
        binding: Binding,
    ) -> Option<usize> {
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
        let (kind, scope) = match (pattern, binding) {
            (Pattern::Object, Binding::Variable) => {
                (Kind::ObjBind, S::PunctuationDefinitionBindingPatternObject)
            }
            (Pattern::Object, Binding::Constant) => (
                Kind::ObjBindConst,
                S::PunctuationDefinitionBindingPatternObject,
            ),
            (Pattern::Array, Binding::Variable) => {
                (Kind::ArrBind, S::PunctuationDefinitionBindingPatternArray)
            }
            (Pattern::Array, Binding::Constant) => (
                Kind::ArrBindConst,
                S::PunctuationDefinitionBindingPatternArray,
            ),
        };
        self.capture(kind, j, j + 1, &[scope]);
        self.push(kind);
        Some(j + 1)
    }

    /// `(?:(\.\.\.)\s*)?(ID)`: the rest element of a binding pattern.
    pub(crate) fn destructuring_variable_rest(
        &mut self,
        p: usize,
        binding: Binding,
    ) -> Option<usize> {
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
        let name = match binding {
            Binding::Constant => S::VariableOtherConstant,
            Binding::Variable => S::VariableOtherReadwrite,
        };
        self.capture(Kind::Top, j, e, &[S::MetaDefinitionVariable, name]);
        Some(e)
    }

    /// `binding-element` and its `const` form.
    pub(crate) fn binding_element(&mut self, p: usize, binding: Binding) -> Option<usize> {
        self.comment(p)
            .or_else(|| self.string(p))
            .or_else(|| self.numeric_literal(p))
            .or_else(|| self.regex(p))
            .or_else(|| self.binding_pattern(p, Pattern::Object, binding))
            .or_else(|| self.binding_pattern(p, Pattern::Array, binding))
            .or_else(|| self.destructuring_variable_rest(p, binding))
            .or_else(|| self.variable_initializer(p))
    }

    pub(crate) fn object_binding_element(&mut self, p: usize, binding: Binding) -> Option<usize> {
        if let Some(r) = self.comment(p) {
            return Some(r);
        }
        if self.key_colon_ahead(p) {
            let kind = match binding {
                Binding::Constant => Kind::ObjBindElemConst,
                Binding::Variable => Kind::ObjBindElem,
            };
            return self.open_empty(kind, p);
        }
        self.binding_pattern(p, Pattern::Object, binding)
            .or_else(|| self.destructuring_variable_rest(p, binding))
            .or_else(|| self.variable_initializer(p))
            .or_else(|| self.punctuation_comma(p))
    }

    /// `(?=(NUM|(ID)|STR|(\[…])+)\s*(:))`.
    pub(crate) fn key_colon_ahead(&self, p: usize) -> bool {
        self.key_end(p)
            .is_some_and(|e| self.byte(self.ws(e)) == b':')
    }

    /// A property key at `i`: a number, a name, a string or `[…]`.
    pub(crate) fn key_end(&self, i: usize) -> Option<usize> {
        self.number_end(i)
            .or_else(|| self.ident(i))
            .or_else(|| self.quoted(i))
            .or_else(|| self.computed_key(i))
    }

    /// `\[([^]\[]|\[[^]\[]*])+]`.
    pub(crate) fn computed_key(&self, i: usize) -> Option<usize> {
        if self.byte(i) != b'[' {
            return None;
        }
        let end = self.window();
        let mut j = i + 1;
        let mut items = 0;
        while j < end {
            self.spend(1);
            match self.byte(j) {
                b']' => return (items > 0).then_some(j + 1),
                b'[' => {
                    let mut k = j + 1;
                    while k < end && !matches!(self.byte(k), b']' | b'[') {
                        self.spend(1);
                        k += 1;
                    }
                    if self.byte(k) != b']' {
                        return None;
                    }
                    j = k + 1;
                }
                _ => j += self.char_len(j),
            }
            items += 1;
        }
        None
    }

    pub(crate) fn object_binding_element_property_name(&mut self, p: usize) -> Option<usize> {
        if !self.key_colon_ahead(p) {
            return None;
        }
        self.open_empty(Kind::ObjBindProp, p)
    }

    /// `variable-initializer`: `=` and what it assigns.
    pub(crate) fn variable_initializer(&mut self, p: usize) -> Option<usize> {
        if self.byte(p) != b'='
            || self.byte(p + 1) == b'='
            || matches!(self.prev_char(p), Some('!' | '='))
        {
            return None;
        }
        let j = self.ws(p + 1);
        let nonblank = !self.eol(j);
        let ends_with_arrow = self.arrow_ends_line_after(p + 1);
        let kind = if nonblank && !ends_with_arrow {
            Kind::VarInit1
        } else {
            Kind::VarInit2
        };
        self.open_token(kind, p, p + 1, &[S::KeywordOperatorAssignment])
    }

    /// `^\s*` followed by one of the statement keywords the grammar treats as
    /// the start of a new statement, as the end rules spell it.
    pub(crate) fn statement_start_at(&self, p: usize) -> bool {
        if !Self::bol(p) {
            return false;
        }
        let j = self.ws(0);
        self.statement_keyword(j).is_some()
    }

    /// The end of a statement keyword at `j`, with its trailing `\b`.
    pub(crate) fn statement_keyword(&self, j: usize) -> Option<usize> {
        const WORDS: [&str; 30] = [
            "abstract",
            "async",
            "break",
            "case",
            "catch",
            "class",
            "const",
            "continue",
            "declare",
            "do",
            "else",
            "enum",
            "export",
            "finally",
            "function",
            "for",
            "goto",
            "if",
            "import",
            "interface",
            "let",
            "module",
            "namespace",
            "switch",
            "return",
            "throw",
            "try",
            "type",
            "var",
            "while",
        ];
        if let Some(e) = self.await_using_end(j) {
            return Some(e);
        }
        if let Some(e) = self.using_end(j) {
            return Some(e);
        }
        for word in WORDS {
            if self.starts(j, word) && self.boundary(j + word.len()) {
                return Some(j + word.len());
            }
        }
        None
    }

    /// `\busing(?=\s+(?!in\b|of\b(?!\s*(?:of\b|=)))[$_[:alpha:]])\b`: its end.
    pub(crate) fn using_end(&self, j: usize) -> Option<usize> {
        if !self.starts(j, "using") || !self.boundary(j) {
            return None;
        }
        let e = j + "using".len();
        let k = self.ws(e);
        if k == e || !self.id_start(k) {
            return None;
        }
        let word_end = self.alnum_end(k);
        let word = self.text(k, word_end);
        if word == "in" {
            return None;
        }
        if word == "of" {
            let m = self.ws(word_end);
            if !(self.starts(m, "of") && self.boundary(m + 2) || self.byte(m) == b'=') {
                return None;
            }
        }
        self.boundary(e).then_some(e)
    }

    /// `\bawait\s+\busing(…)\b`: its end.
    pub(crate) fn await_using_end(&self, j: usize) -> Option<usize> {
        if !self.starts(j, "await") || !self.boundary(j) {
            return None;
        }
        let k = self.ws(j + "await".len());
        if k == j + "await".len() {
            return None;
        }
        self.using_end(k)
    }
}
