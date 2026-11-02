//! Functions, classes, interfaces, enums, modules, imports and exports, and
//! the control statements.

use super::kind::Kind;
use super::lexer::Lexer;
use super::scope::S;

impl Lexer<'_> {
    // Control statements.

    pub(crate) fn control_statement(&mut self, p: usize) -> Option<usize> {
        if !self.b(p).is_ascii_lowercase() {
            return None;
        }
        if let Some(r) = self.switch_statement(p).or_else(|| self.for_loop(p)) {
            return Some(r);
        }
        if let Some(e) = self.kw_of(p, &["catch", "finally", "throw", "try"]) {
            return self.tok(p, e, S::KeywordControlTrycatch);
        }
        if let Some(e) = self.kw_of(p, &["break", "continue", "goto"]) {
            let j = self.ws(e);
            if j > e
                && let Some(n) = self.ident(j)
                && self.kw_after(n)
            {
                self.one(p, e, &[S::KeywordControlLoop]);
                self.cap(Kind::Top, j, n, &[S::EntityNameLabel]);
                return Some(n);
            }
        }
        if let Some(e) = self.kw_of(p, &["break", "continue", "do", "goto", "while"]) {
            return self.tok(p, e, S::KeywordControlLoop);
        }
        if let Some(e) = self.kw(p, "return") {
            return self.open1(Kind::Return, p, e, &[S::KeywordControlFlow]);
        }
        if let Some(e) = self.kw_of(p, &["case", "default", "switch"]) {
            return self.tok(p, e, S::KeywordControlSwitch);
        }
        if let Some(r) = self.if_statement(p) {
            return Some(r);
        }
        if let Some(e) = self.kw_of(p, &["else", "if"]) {
            return self.tok(p, e, S::KeywordControlConditional);
        }
        if let Some(e) = self.kw(p, "with") {
            return self.tok(p, e, S::KeywordControlWith);
        }
        if let Some(e) = self.kw(p, "package") {
            return self.tok(p, e, S::KeywordControl);
        }
        if let Some(e) = self.kw(p, "debugger") {
            return self.tok(p, e, S::KeywordOtherDebugger);
        }
        None
    }

    fn switch_statement(&mut self, p: usize) -> Option<usize> {
        let e = self
            .kw_before(p)
            .then(|| self.starts(p, "switch").then_some(p + 6))
            .flatten()?;
        if !self.boundary(p) || self.b(self.ws(e)) != b'(' {
            return None;
        }
        self.open0(Kind::SwitchStmt, p)
    }

    /// The rules of a `switch` statement before its braces.
    pub(crate) fn switch_statement_rules(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.comment(p) {
            return Some(r);
        }
        if self.kw_before(p) && self.starts(p, "switch") {
            let e = p + 6;
            let j = self.ws(e);
            if self.b(j) == b'(' && !self.full() {
                self.flush(p);
                self.cap(Kind::SwitchExpr, p, e, &[S::KeywordControlSwitch]);
                self.cap(Kind::SwitchExpr, j, j + 1, &[S::MetaBraceRound]);
                self.push(Kind::SwitchExpr);
                return Some(j + 1);
            }
        }
        if self.b(p) == b'{' {
            return self.open1(
                Kind::SwitchBlock,
                p,
                p + 1,
                &[S::PunctuationDefinitionBlock],
            );
        }
        None
    }

    pub(crate) fn switch_block_rules(&mut self, p: usize) -> Option<usize> {
        if let Some(e) = self.kw(p, "case") {
            return self.open1(Kind::CaseClause, p, e, &[S::KeywordControlSwitch]);
        }
        if self.starts(p, "default") && self.kw_before(p) && self.b(p + 7) == b':' {
            let e = p + 7;
            if self.kw_after(e) {
                return self.open1(Kind::CaseClause, p, e, &[S::KeywordControlSwitch]);
            }
        }
        if self.b(p) == b':' {
            let j = self.ws(p + 1);
            if self.b(j) == b'{' && !self.full() {
                self.flush(p);
                self.cap(
                    Kind::CaseBlock,
                    p,
                    p + 1,
                    &[
                        S::CaseClauseExpr,
                        S::PunctuationDefinitionSectionCaseStatement,
                    ],
                );
                self.cap(
                    Kind::CaseBlock,
                    j,
                    j + 1,
                    &[S::MetaBlock, S::PunctuationDefinitionBlock],
                );
                self.push(Kind::CaseBlock);
                return Some(j + 1);
            }
            return Some(self.tok2(
                p,
                p + 1,
                S::CaseClauseExpr,
                S::PunctuationDefinitionSectionCaseStatement,
            ));
        }
        self.statements(p)
    }

    fn tok2(&mut self, a: usize, b: usize, s1: S, s2: S) -> usize {
        self.one(a, b, &[s1, s2]);
        b
    }

    fn for_loop(&mut self, p: usize) -> Option<usize> {
        let e = self.kw(p, "for")?;
        let mut j = self.ws_comments(e);
        // `(\s+|(\s*/\*…\*/\s*))await`
        if j > e && self.starts(j, "await") {
            j = self.ws_comments(j + 5);
        }
        if self.b(j) != b'(' || self.full() {
            return None;
        }
        self.open1(Kind::ForLoop, p, e, &[S::KeywordControlLoop])
    }

    pub(crate) fn for_loop_rules(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.comment(p) {
            return Some(r);
        }
        if let Some(e) = self.starts(p, "await").then_some(p + 5) {
            return self.tok(p, e, S::KeywordControlLoop);
        }
        if self.b(p) == b'(' {
            return self.open1(Kind::ForParen, p, p + 1, &[S::MetaBraceRound]);
        }
        None
    }

    pub(crate) fn for_paren_rules(&mut self, p: usize) -> Option<usize> {
        self.var_expr(p)
            .or_else(|| self.expression(p))
            .or_else(|| self.punctuation_semicolon(p))
    }

    /// `if (…)` not followed directly by a brace.
    fn if_statement(&mut self, p: usize) -> Option<usize> {
        if !(self.starts(p, "if") && self.kw_before(p) && self.boundary(p)) {
            return None;
        }
        let j = self.ws(p + 2);
        if self.b(j) != b'(' {
            return None;
        }
        let close = self.balanced(j, b'(', b')')?;
        // `\s*(?!\{)` backs off a blank, so only `){` fails.
        let ok = self.ws(close) > close || self.b(close) != b'{';
        if !ok {
            return None;
        }
        self.open0(Kind::IfOuter, p)
    }

    pub(crate) fn if_outer_rules(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.comment(p) {
            return Some(r);
        }
        if self.kw_before(p) && self.starts(p, "if") {
            let j = self.ws(p + 2);
            if self.b(j) == b'(' && !self.full() {
                self.flush(p);
                self.cap(Kind::IfParen, p, p + 2, &[S::KeywordControlConditional]);
                self.cap(Kind::IfParen, j, j + 1, &[S::MetaBraceRound]);
                self.push(Kind::IfParen);
                return Some(j + 1);
            }
        }
        if let Some(r) = self.regex_after_paren(p) {
            return Some(r);
        }
        self.statements(p)
    }

    /// `(?<=\))\s*/(?![*/])(?=REGEX)`.
    fn regex_after_paren(&mut self, p: usize) -> Option<usize> {
        if self.prev_char(p) != Some(')') || self.full() {
            return None;
        }
        let q = self.ws(p);
        if self.b(q) != b'/' || matches!(self.b(q + 1), b'*' | b'/') || !self.regex_body(q, false) {
            return None;
        }
        self.flush(p);
        self.cap(
            Kind::Regex,
            q,
            q + 1,
            &[S::PunctuationDefinitionStringBegin],
        );
        self.push(Kind::Regex);
        Some(q + 1)
    }

    // Functions.

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
        self.string(p)
            .or_else(|| self.decorator(p))
            .or_else(|| self.destructuring_parameter(p))
            .or_else(|| self.parameter_name(p))
            .or_else(|| self.parameter_type_annotation(p))
            .or_else(|| self.variable_initializer(p))
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

    // Classes, interfaces, enums, namespaces and type aliases.

    pub(crate) fn class_declaration(&mut self, p: usize) -> Option<usize> {
        let pre = self.declaration_start(p)?;
        let mut k = pre.at;
        let mut abstract_cap = None;
        if self.starts(k, "abstract") && self.boundary(k) {
            let e = k + 8;
            let j = self.ws(e);
            abstract_cap = Some((k, e));
            k = j;
        }
        if !self.starts(k, "class") || !self.boundary(k + 5) || self.full() {
            return None;
        }
        let e = k + 5;
        let j = self.ws(e);
        if !(j > e || self.starts(e, "/*") || self.starts(e, "//")) {
            return None;
        }
        self.flush(p);
        self.prefix_tokens(Kind::ClassDecl, &pre);
        if let Some((a, b)) = abstract_cap {
            self.cap(Kind::ClassDecl, a, b, &[S::StorageModifier]);
        }
        self.cap(Kind::ClassDecl, k, e, &[S::StorageTypeClass]);
        self.push(Kind::ClassDecl);
        Some(e)
    }

    pub(crate) fn class_expression(&mut self, p: usize) -> Option<usize> {
        if !self.b(p).is_ascii_lowercase() || !self.kw_before(p) {
            return None;
        }
        let mut k = p;
        let mut abstract_cap = None;
        if self.starts(k, "abstract") {
            let e = k + 8;
            let j = self.ws(e);
            if j > e {
                abstract_cap = Some((k, e));
                k = j;
            }
        }
        if !self.starts(k, "class") || !self.boundary(k + 5) || self.full() {
            return None;
        }
        let e = k + 5;
        let after = self.b(e);
        let ok = self.ws(e) > e
            || matches!(after, b'<' | b'{')
            || self.starts(e, "/*")
            || self.starts(e, "//");
        if !ok {
            return None;
        }
        self.flush(p);
        if let Some((a, b)) = abstract_cap {
            self.cap(Kind::ClassDecl, a, b, &[S::StorageModifier]);
        }
        self.cap(Kind::ClassDecl, k, e, &[S::StorageTypeClass]);
        self.push(Kind::ClassDecl);
        Some(e)
    }

    pub(crate) fn class_rules(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.comment(p) {
            return Some(r);
        }
        if let Some(r) = self.class_or_interface_heritage(p) {
            return Some(r);
        }
        if let Some(e) = self.ident(p) {
            return self.tok(p, e, S::EntityNameTypeClass);
        }
        self.type_parameters(p)
            .or_else(|| self.class_or_interface_body(p))
    }

    pub(crate) fn interface_declaration(&mut self, p: usize) -> Option<usize> {
        let pre = self.declaration_start(p)?;
        let mut k = pre.at;
        let mut abstract_cap = None;
        if self.starts(k, "abstract") && self.boundary(k) {
            let e = k + 8;
            abstract_cap = Some((k, e));
            k = self.ws(e);
        }
        if !self.starts(k, "interface") || !self.boundary(k + 9) || self.full() {
            return None;
        }
        let e = k + 9;
        if !(self.ws(e) > e || self.starts(e, "/*") || self.starts(e, "//")) {
            return None;
        }
        self.flush(p);
        self.prefix_tokens(Kind::InterfaceDecl, &pre);
        if let Some((a, b)) = abstract_cap {
            self.cap(Kind::InterfaceDecl, a, b, &[S::StorageModifier]);
        }
        self.cap(Kind::InterfaceDecl, k, e, &[S::StorageTypeInterface]);
        self.push(Kind::InterfaceDecl);
        Some(e)
    }

    pub(crate) fn interface_rules(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.comment(p) {
            return Some(r);
        }
        if let Some(r) = self.class_or_interface_heritage(p) {
            return Some(r);
        }
        if let Some(e) = self.ident(p) {
            return self.tok(p, e, S::EntityNameTypeInterface);
        }
        self.type_parameters(p)
            .or_else(|| self.class_or_interface_body(p))
    }

    pub(crate) fn class_or_interface_body(&mut self, p: usize) -> Option<usize> {
        if self.b(p) != b'{' {
            return None;
        }
        self.open1(Kind::ClassBody, p, p + 1, &[S::PunctuationDefinitionBlock])
    }

    pub(crate) fn class_or_interface_heritage(&mut self, p: usize) -> Option<usize> {
        let e = self
            .kw_before(p)
            .then(|| self.kw_of(p, &["extends", "implements"]))
            .flatten()?;
        if !self.boundary(p) {
            return None;
        }
        self.open1(Kind::Heritage, p, e, &[S::StorageModifier])
    }

    pub(crate) fn heritage_rules(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.comment(p) {
            return Some(r);
        }
        if let Some(r) = self.class_or_interface_heritage(p) {
            return Some(r);
        }
        if let Some(r) = self.type_parameters(p) {
            return Some(r);
        }
        if let Some(r) = self.expression_without_identifiers(p) {
            return Some(r);
        }
        if let Some(e) = self.ident(p) {
            let j = self.ws(e);
            if let Some(a) = self.accessor_at(j) {
                let k = self.ws(a);
                if self.hash_ident_member_chain(k) {
                    self.one(p, e, &[S::EntityNameTypeModule]);
                    self.cap(Kind::Top, j, a, &[Self::accessor_scope(j, a)]);
                    return Some(a);
                }
            }
            return self.tok(p, e, S::EntityOtherInheritedClass);
        }
        self.expression_punctuation(p)
    }

    /// `[$_[:alpha:]][$_[:alnum:]]*(\s*\??\.\s*[$_[:alpha:]][$_[:alnum:]]*)*\s*
    /// ` at `k`: a name, which the module-qualified rule looks ahead for.
    fn hash_ident_member_chain(&self, k: usize) -> bool {
        self.ident(k).is_some()
    }

    pub(crate) fn enum_declaration(&mut self, p: usize) -> Option<usize> {
        let pre = self.declaration_start(p)?;
        let mut k = pre.at;
        let mut const_cap = None;
        if self.starts(k, "const") && self.boundary(k) {
            let e = k + 5;
            let j = self.ws(e);
            if j > e {
                const_cap = Some((k, e));
                k = j;
            }
        }
        if !self.starts(k, "enum") || !self.boundary(k) || self.full() {
            return None;
        }
        let e = k + 4;
        let j = self.ws(e);
        if j == e {
            return None;
        }
        let n = self.ident(j)?;
        self.flush(p);
        self.prefix_tokens(Kind::EnumDecl, &pre);
        if let Some((a, b)) = const_cap {
            self.cap(Kind::EnumDecl, a, b, &[S::StorageModifier]);
        }
        self.cap(Kind::EnumDecl, k, e, &[S::StorageTypeEnum]);
        self.cap(Kind::EnumDecl, j, n, &[S::EntityNameTypeEnum]);
        self.push(Kind::EnumDecl);
        Some(n)
    }

    pub(crate) fn enum_rules(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.comment(p) {
            return Some(r);
        }
        if self.b(p) == b'{' {
            return self.open1(Kind::EnumBody, p, p + 1, &[S::PunctuationDefinitionBlock]);
        }
        None
    }

    pub(crate) fn enum_body_rules(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.comment(p) {
            return Some(r);
        }
        if self.id_start(p) {
            let e = self.ident(p)?;
            return self.open1(Kind::EnumMember, p, e, &[S::VariableOtherEnummember]);
        }
        if self.enum_key_ahead(p) {
            return self.open0(Kind::EnumKey, p);
        }
        self.punctuation_comma(p)
    }

    /// `(?=(STR|`…`|\[…]+))`.
    fn enum_key_ahead(&self, p: usize) -> bool {
        self.quoted(p).is_some() || self.computed_key(p).is_some()
    }

    pub(crate) fn namespace_declaration(&mut self, p: usize) -> Option<usize> {
        let pre = self.declaration_start(p)?;
        let k = pre.at;
        let e = self.kw_of_boundary(k, &["namespace", "module"])?;
        if self.full() {
            return None;
        }
        let j = self.ws(e);
        if j == e {
            return None;
        }
        let c = self.b(j);
        let ok = matches!(c, b'"' | b'$' | b'\'' | b'_' | b'`' | b'[') || self.id_start(j);
        if !ok {
            return None;
        }
        self.flush(p);
        self.prefix_tokens(Kind::NamespaceDecl, &pre);
        self.cap(Kind::NamespaceDecl, k, e, &[S::StorageTypeNamespace]);
        self.fin(Kind::NamespaceDecl, j);
        self.push(Kind::NamespaceDecl);
        Some(j)
    }

    /// A word with `\b` on both sides.
    fn kw_of_boundary(&self, k: usize, words: &[&str]) -> Option<usize> {
        words
            .iter()
            .find(|w| self.starts(k, w) && self.boundary(k) && self.boundary(k + w.len()))
            .map(|w| k + w.len())
    }

    pub(crate) fn namespace_rules(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.comment(p) {
            return Some(r);
        }
        if let Some(r) = self.string(p) {
            return Some(r);
        }
        if let Some(e) = self.ident(p) {
            return self.tok(p, e, S::EntityNameTypeModule);
        }
        self.punctuation_accessor(p).or_else(|| self.decl_block(p))
    }

    pub(crate) fn type_alias_declaration(&mut self, p: usize) -> Option<usize> {
        let pre = self.declaration_start(p)?;
        let k = pre.at;
        if !self.starts(k, "type") || !self.boundary(k) || !self.boundary(k + 4) || self.full() {
            return None;
        }
        let e = k + 4;
        let j = self.ws(e);
        if j == e {
            return None;
        }
        let n = self.ident(j)?;
        let end = self.ws(n);
        self.flush(p);
        self.prefix_tokens(Kind::TypeAlias, &pre);
        self.cap(Kind::TypeAlias, k, e, &[S::StorageTypeType]);
        self.cap(Kind::TypeAlias, j, n, &[S::EntityNameTypeAlias]);
        self.fin(Kind::TypeAlias, end);
        self.push(Kind::TypeAlias);
        Some(end)
    }

    pub(crate) fn type_alias_rules(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.comment(p) {
            return Some(r);
        }
        if let Some(r) = self.type_parameters(p) {
            return Some(r);
        }
        if self.b(p) == b'=' && !self.full() {
            let j = self.ws(p + 1);
            let mut intrinsic = None;
            if self.starts(j, "intrinsic") && self.kw_after(j + 9) {
                intrinsic = Some((j, j + 9));
            }
            self.flush(p);
            self.cap(Kind::TypeAliasEq, p, p + 1, &[S::KeywordOperatorAssignment]);
            if let Some((a, b)) = intrinsic {
                self.cap(Kind::TypeAliasEq, a, b, &[S::KeywordControlIntrinsic]);
            } else {
                self.fin(Kind::TypeAliasEq, j);
            }
            self.push(Kind::TypeAliasEq);
            return Some(intrinsic.map_or(j, |(_, e)| e));
        }
        None
    }

    // Imports and exports.

    pub(crate) fn import_equals_declaration(&mut self, p: usize) -> Option<usize> {
        let pre = self.declaration_start(p)?;
        let k = pre.at;
        if !self.starts(k, "import") || !self.boundary(k) || self.full() {
            return None;
        }
        let mut j = k + 6;
        let mut type_cap = None;
        let t = self.ws(j);
        if t > j && self.starts(t, "type") && self.boundary(t + 4) {
            let after = self.ws(t + 4);
            if after > t + 4 {
                type_cap = Some((t, t + 4));
                j = t + 4;
            }
        }
        let n0 = self.ws(j);
        if n0 == j {
            return None;
        }
        let n = self.ident(n0)?;
        let eq = self.ws(n);
        if self.b(eq) != b'=' {
            return None;
        }
        let r0 = self.ws(eq + 1);
        let external = self.starts(r0, "require") && {
            let q = self.ws(r0 + 7);
            self.b(q) == b'('
        };
        self.flush(p);
        let kind = if external {
            Kind::ImportEqExt
        } else {
            Kind::ImportEqInt
        };
        if !external && self.starts(r0, "require") && self.boundary(r0 + 7) {
            return None;
        }
        self.prefix_tokens(kind, &pre);
        self.cap(kind, k, k + 6, &[S::KeywordControlImport]);
        if let Some((a, b)) = type_cap {
            self.cap(kind, a, b, &[S::KeywordControlType]);
        }
        self.cap(kind, n0, n, &[S::VariableOtherReadwriteAlias]);
        self.cap(kind, eq, eq + 1, &[S::KeywordOperatorAssignment]);
        if external {
            let q = self.ws(r0 + 7);
            self.cap(kind, r0, r0 + 7, &[S::KeywordControlRequire]);
            self.cap(kind, q, q + 1, &[S::MetaBraceRound]);
            self.push(kind);
            return Some(q + 1);
        }
        self.fin(kind, r0);
        self.push(kind);
        Some(r0)
    }

    pub(crate) fn import_eq_int_rules(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.single_line_comment_consuming_line_ending(p) {
            return Some(r);
        }
        if let Some(r) = self.comment(p) {
            return Some(r);
        }
        if let Some(e) = self.ident(p) {
            let j = self.ws(e);
            if let Some(a) = self.accessor_at(j) {
                self.one(p, e, &[S::EntityNameTypeModule]);
                self.cap(Kind::Top, j, a, &[Self::accessor_scope(j, a)]);
                return Some(a);
            }
            return self.tok(p, e, S::VariableOtherReadwrite);
        }
        None
    }

    pub(crate) fn import_declaration(&mut self, p: usize) -> Option<usize> {
        let pre = self.declaration_start(p)?;
        let k = pre.at;
        if !self.starts(k, "import") || !self.boundary(k) || self.full() {
            return None;
        }
        let mut e = k + 6;
        let mut type_cap = None;
        let t = self.ws(e);
        if t > e && self.starts(t, "type") && self.boundary(t + 4) {
            let after = self.ws(t + 4);
            // `(?!\s+from)`.
            if !(after > t + 4 && self.starts(after, "from") && self.boundary(after + 4)) {
                type_cap = Some((t, t + 4));
                e = t + 4;
            }
        }
        // `(?!\s*[(:])E`.
        let nxt = self.ws(e);
        if matches!(self.b(nxt), b'(' | b':') || !self.kw_after(e) {
            return None;
        }
        self.flush(p);
        self.prefix_tokens(Kind::Import, &pre);
        self.cap(Kind::Import, k, k + 6, &[S::KeywordControlImport]);
        if let Some((a, b)) = type_cap {
            self.cap(Kind::Import, a, b, &[S::KeywordControlType]);
        }
        self.fin(Kind::Import, e);
        self.push(Kind::Import);
        Some(e)
    }

    pub(crate) fn import_rules(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.single_line_comment_consuming_line_ending(p) {
            return Some(r);
        }
        if let Some(r) = self.comment(p) {
            return Some(r);
        }
        if let Some(r) = self.string(p) {
            return Some(r);
        }
        if self.prev_import(p) && !matches!(self.b(self.ws(p)), b'"' | b'\'') {
            return self.open0(Kind::ImportFrom, p);
        }
        self.import_export_declaration(p)
    }

    /// `(?<=(?:^|[^$._[:alnum:]])import)`.
    fn prev_import(&self, p: usize) -> bool {
        self.word_before_text(p, "import")
    }

    pub(crate) fn import_export_declaration(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.comment(p) {
            return Some(r);
        }
        if let Some(r) = self.string(p) {
            return Some(r);
        }
        if self.b(p) == b'{' {
            return self.open1(
                Kind::ImportBlock,
                p,
                p + 1,
                &[S::PunctuationDefinitionBlock],
            );
        }
        if self.starts(p, "from") && self.boundary(p) && self.boundary(p + 4) {
            return self.tok(p, p + 4, S::KeywordControlFrom);
        }
        if let Some(r) = self.import_export_assert_clause(p) {
            return Some(r);
        }
        self.import_export_clause(p)
    }

    fn import_export_assert_clause(&mut self, p: usize) -> Option<usize> {
        if !self.kw_before(p) || self.full() {
            return None;
        }
        let (e, scope) = if self.starts(p, "with") {
            (p + 4, S::KeywordControlWith)
        } else if self.starts(p, "assert") {
            (p + 6, S::KeywordControlAssert)
        } else {
            return None;
        };
        let j = self.ws(e);
        if self.b(j) != b'{' {
            return None;
        }
        self.flush(p);
        self.cap(Kind::AssertClause, p, e, &[scope]);
        self.cap(
            Kind::AssertClause,
            j,
            j + 1,
            &[S::PunctuationDefinitionBlock],
        );
        self.push(Kind::AssertClause);
        Some(j + 1)
    }

    pub(crate) fn assert_clause_rules(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.comment(p) {
            return Some(r);
        }
        if let Some(r) = self.string(p) {
            return Some(r);
        }
        if let Some(e) = self.ident(p) {
            let j = self.ws_comments(e);
            if self.b(j) == b':' {
                let end = self.ws(e);
                self.one(p, end, &[S::MetaObjectLiteralKey]);
                return Some(end);
            }
        }
        (self.b(p) == b':').then(|| self.tok(p, p + 1, S::PunctuationSeparatorKeyValue))?
    }

    pub(crate) fn import_export_clause(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.comment(p) {
            return Some(r);
        }
        if let Some(r) = self.import_alias(p) {
            return Some(r);
        }
        if let Some(r) = self.punctuation_comma(p) {
            return Some(r);
        }
        if self.b(p) == b'*' {
            return self.tok(p, p + 1, S::ConstantLanguageImportExportAll);
        }
        if self.starts(p, "default") && self.boundary(p) && self.boundary(p + 7) {
            return self.tok(p, p + 7, S::KeywordControlDefault);
        }
        self.import_name(p)
    }

    /// `(type\s+)?(default|*|ID|STR)\s+as\s+(default|ID|STR)`.
    fn import_alias(&mut self, p: usize) -> Option<usize> {
        if !self.kw_before(p) {
            return None;
        }
        let mut j = p;
        let mut type_cap = None;
        if self.starts(j, "type") && self.boundary(j) {
            let e = j + 4;
            let k = self.ws(e);
            if k > e {
                type_cap = Some((j, e));
                j = k;
            }
        }
        // The grammar tries `type` as the optional modifier first, then as a
        // name.
        for attempt in 0..2 {
            let (start, tcap) = if attempt == 0 {
                (j, type_cap)
            } else {
                (p, None)
            };
            if attempt == 1 && type_cap.is_none() {
                break;
            }
            if let Some(r) = self.import_alias_from(p, start, tcap) {
                return Some(r);
            }
        }
        None
    }

    fn import_alias_from(
        &mut self,
        p: usize,
        j: usize,
        type_cap: Option<(usize, usize)>,
    ) -> Option<usize> {
        let (first_end, first_scope): (usize, S) =
            if self.starts(j, "default") && self.boundary(j) && self.boundary(j + 7) {
                (j + 7, S::KeywordControlDefault)
            } else if self.b(j) == b'*' {
                (j + 1, S::ConstantLanguageImportExportAll)
            } else if let Some(e) = self.ident(j) {
                if !self.boundary(j) {
                    return None;
                }
                (e, S::VariableOtherReadwrite)
            } else {
                let e = self.quoted(j)?;
                (e, S::StringQuotedAlias)
            };
        let k = self.ws(first_end);
        if k == first_end || !self.starts(k, "as") || !self.boundary(k + 2) {
            return None;
        }
        let m = self.ws(k + 2);
        if m == k + 2 {
            return None;
        }
        let (second_end, second_scope): (usize, S) =
            if self.starts(m, "default") && self.kw_after(m + 7) {
                (m + 7, S::KeywordControlDefault)
            } else if let Some(e) = self.ident(m) {
                (e, S::VariableOtherReadwriteAlias)
            } else {
                let e = self.quoted(m)?;
                (e, S::StringQuotedAlias)
            };
        self.flush(p);
        if let Some((a, b)) = type_cap {
            self.cap(Kind::Top, a, b, &[S::KeywordControlType]);
        }
        if first_scope == S::StringQuotedAlias {
            self.quoted_alias_tokens(j, first_end);
        } else {
            self.cap(Kind::Top, j, first_end, &[first_scope]);
        }
        self.cap(Kind::Top, k, k + 2, &[S::KeywordControlAs]);
        if second_scope == S::StringQuotedAlias {
            self.quoted_alias_tokens(m, second_end);
        } else {
            self.cap(Kind::Top, m, second_end, &[second_scope]);
        }
        Some(second_end)
    }

    fn quoted_alias_tokens(&mut self, a: usize, b: usize) {
        self.cap(Kind::Top, a, b, &[S::StringQuotedAlias]);
    }

    /// `(type\s+)?(ID|STR)`.
    fn import_name(&mut self, p: usize) -> Option<usize> {
        let mut j = p;
        let mut type_cap = None;
        if self.starts(j, "type") && self.boundary(j) {
            let e = j + 4;
            let k = self.ws(e);
            if k > e && (self.ident(k).is_some() || self.quoted(k).is_some()) {
                type_cap = Some((j, e));
                j = k;
            }
        }
        let (e, scope) = if let Some(e) = self.ident(j) {
            (e, S::VariableOtherReadwriteAlias)
        } else {
            let e = self.quoted(j)?;
            (e, S::StringQuotedAlias)
        };
        self.flush(p);
        if let Some((a, b)) = type_cap {
            self.cap(Kind::Top, a, b, &[S::KeywordControlType]);
        }
        self.cap(Kind::Top, j, e, &[scope]);
        Some(e)
    }

    pub(crate) fn export_declaration(&mut self, p: usize) -> Option<usize> {
        if !self.starts(p, "export") || !self.kw_before(p) || !self.boundary(p) {
            return None;
        }
        let e = p + 6;
        // `export as namespace Name`.
        let j = self.ws(e);
        if j > e && self.starts(j, "as") && self.boundary(j + 2) {
            let k = self.ws(j + 2);
            if k > j + 2 && self.starts(k, "namespace") && self.boundary(k + 9) {
                let m = self.ws(k + 9);
                if m > k + 9
                    && let Some(n) = self.ident(m)
                {
                    self.one(p, e, &[S::KeywordControlExport]);
                    self.cap(Kind::Top, j, j + 2, &[S::KeywordControlAs]);
                    self.cap(Kind::Top, k, k + 9, &[S::StorageTypeNamespace]);
                    self.cap(Kind::Top, m, n, &[S::EntityNameTypeModule]);
                    return Some(n);
                }
            }
        }
        if self.full() {
            return None;
        }
        // `export (type)? (=|default)`.
        let mut after = e;
        let mut type_cap = None;
        let t = self.ws(e);
        let mut default_start = None;
        if t > e && self.starts(t, "type") && self.boundary(t + 4) {
            type_cap = Some((t, t + 4));
            after = t + 4;
        }
        let eq = self.ws(after);
        if self.b(eq) == b'=' && self.b(eq + 1) != b'=' || self.b(eq) == b'=' {
            self.flush(p);
            self.cap(Kind::ExportDefault, p, e, &[S::KeywordControlExport]);
            if let Some((a, b)) = type_cap {
                self.cap(Kind::ExportDefault, a, b, &[S::KeywordControlType]);
            }
            self.cap(
                Kind::ExportDefault,
                eq,
                eq + 1,
                &[S::KeywordOperatorAssignment],
            );
            self.push(Kind::ExportDefault);
            return Some(eq + 1);
        }
        let d = self.ws(after);
        if d > after && self.starts(d, "default") && self.boundary(d + 7) {
            let tail = self.ws(d + 7);
            if tail > d + 7 {
                default_start = Some(d);
            }
        }
        if let Some(d) = default_start {
            self.flush(p);
            self.cap(Kind::ExportDefault, p, e, &[S::KeywordControlExport]);
            if let Some((a, b)) = type_cap {
                self.cap(Kind::ExportDefault, a, b, &[S::KeywordControlType]);
            }
            self.cap(Kind::ExportDefault, d, d + 7, &[S::KeywordControlDefault]);
            self.push(Kind::ExportDefault);
            return Some(d + 7);
        }
        // `export type`? followed by `*`, `{` or a name list.
        let mut after = e;
        let mut type_cap = None;
        let t = self.ws(e);
        if t > e && self.starts(t, "type") && self.boundary(t + 4) {
            // The optional `type` is tried first, then skipped.
            let try_type = self.export_clause_ahead(t + 4, true);
            if try_type {
                type_cap = Some((t, t + 4));
                after = t + 4;
            }
        }
        if !self.boundary(after) {
            return None;
        }
        // `(?!(\$)|(\s*:))`.
        if self.b(after) == b'$' || self.b(self.ws(after)) == b':' {
            return None;
        }
        if !self.export_clause_ahead(after, false) {
            return None;
        }
        self.flush(p);
        self.cap(Kind::Export, p, e, &[S::KeywordControlExport]);
        if let Some((a, b)) = type_cap {
            self.cap(Kind::Export, a, b, &[S::KeywordControlType]);
        }
        self.push(Kind::Export);
        Some(after)
    }

    /// `(?=\s*[*{])` or a name followed by `,` or blanks that is not a
    /// statement keyword.
    fn export_clause_ahead(&self, i: usize, _after_type: bool) -> bool {
        let j = self.ws(i);
        if matches!(self.b(j), b'*' | b'{') {
            return true;
        }
        let Some(e) = self.ident(j) else {
            return false;
        };
        if !(self.b(e) == b',' || super::lexer::is_ws(self.b(e))) {
            return false;
        }
        !self.statement_keyword_at(j)
    }

    /// `\s*(?:abstract|async|…)\b` at `j`.
    pub(crate) fn statement_keyword_at(&self, j: usize) -> bool {
        self.statement_keyword(j).is_some()
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
