//! Import declarations and the clauses they hold.

use super::kind::Kind;
use super::lexer::Lexer;
use super::scope::S;

impl<const D: u8> Lexer<'_, D> {
    #[expect(
        clippy::many_single_char_names,
        reason = "positions are named by role: `p` where the rule starts, `e` where a part ends, `j` and `k` for the cursor"
    )]
    pub(crate) fn import_equals_declaration(&mut self, p: usize) -> Option<usize> {
        let pre = self.declaration_start(p)?;
        let k = pre.at;
        if !self.starts(k, "import") || !self.boundary(k) || self.full() {
            return None;
        }
        let mut j = k + "import".len();
        let mut type_cap = None;
        let t = self.ws(j);
        if t > j && self.starts(t, "type") && self.boundary(t + "type".len()) {
            let after = self.ws(t + "type".len());
            if after > t + "type".len() {
                type_cap = Some((t, t + "type".len()));
                j = t + "type".len();
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
            let q = self.ws(r0 + "require".len());
            self.b(q) == b'('
        };
        self.flush(p);
        let kind = if external {
            Kind::ImportEqExt
        } else {
            Kind::ImportEqInt
        };
        if !external && self.starts(r0, "require") && self.boundary(r0 + "require".len()) {
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
            let q = self.ws(r0 + "require".len());
            self.cap(kind, r0, r0 + "require".len(), &[S::KeywordControlRequire]);
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
        let mut e = k + "import".len();
        let mut type_cap = None;
        let t = self.ws(e);
        if t > e && self.starts(t, "type") && self.boundary(t + "type".len()) {
            let after = self.ws(t + "type".len());
            // `(?!\s+from)`.
            if !(after > t + "type".len()
                && self.starts(after, "from")
                && self.boundary(after + "from".len()))
            {
                type_cap = Some((t, t + "type".len()));
                e = t + "type".len();
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
        if self.starts(p, "from") && self.boundary(p) && self.boundary(p + "from".len()) {
            return self.tok(p, p + "from".len(), S::KeywordControlFrom);
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
            (p + "with".len(), S::KeywordControlWith)
        } else if self.starts(p, "assert") {
            (p + "assert".len(), S::KeywordControlAssert)
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
        if self.starts(p, "default") && self.boundary(p) && self.boundary(p + "default".len()) {
            return self.tok(p, p + "default".len(), S::KeywordControlDefault);
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
            let e = j + "type".len();
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

    #[expect(
        clippy::many_single_char_names,
        reason = "positions are named by role: `p` where the rule starts, `e` where a part ends, `j` and `k` for the cursor"
    )]
    fn import_alias_from(
        &mut self,
        p: usize,
        j: usize,
        type_cap: Option<(usize, usize)>,
    ) -> Option<usize> {
        let (first_end, first_scope): (usize, S) = if self.starts(j, "default")
            && self.boundary(j)
            && self.boundary(j + "default".len())
        {
            (j + "default".len(), S::KeywordControlDefault)
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
            if self.starts(m, "default") && self.kw_after(m + "default".len()) {
                (m + "default".len(), S::KeywordControlDefault)
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
            let e = j + "type".len();
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
}
