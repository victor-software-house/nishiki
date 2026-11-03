//! Export declarations and the statement keywords that end a declaration.

use super::kind::Kind;
use super::lexer::Lexer;
use super::scope::S;

impl<const D: u8> Lexer<'_, D> {
    #[expect(
        clippy::many_single_char_names,
        reason = "positions are named by role: `p` where the rule starts, `e` where a part ends, `j` and `k` for the cursor"
    )]
    pub(crate) fn export_declaration(&mut self, p: usize) -> Option<usize> {
        if !self.starts(p, "export") || !self.kw_before(p) || !self.boundary(p) {
            return None;
        }
        let e = p + "export".len();
        // `export as namespace Name`.
        let j = self.ws(e);
        if j > e && self.starts(j, "as") && self.boundary(j + 2) {
            let k = self.ws(j + 2);
            if k > j + 2 && self.starts(k, "namespace") && self.boundary(k + "namespace".len()) {
                let m = self.ws(k + "namespace".len());
                if m > k + "namespace".len()
                    && let Some(n) = self.ident(m)
                {
                    self.one(p, e, &[S::KeywordControlExport]);
                    self.cap(Kind::Top, j, j + 2, &[S::KeywordControlAs]);
                    self.cap(
                        Kind::Top,
                        k,
                        k + "namespace".len(),
                        &[S::StorageTypeNamespace],
                    );
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
        if t > e && self.starts(t, "type") && self.boundary(t + "type".len()) {
            type_cap = Some((t, t + "type".len()));
            after = t + "type".len();
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
        if d > after && self.starts(d, "default") && self.boundary(d + "default".len()) {
            let tail = self.ws(d + "default".len());
            if tail > d + "default".len() {
                default_start = Some(d);
            }
        }
        if let Some(d) = default_start {
            self.flush(p);
            self.cap(Kind::ExportDefault, p, e, &[S::KeywordControlExport]);
            if let Some((a, b)) = type_cap {
                self.cap(Kind::ExportDefault, a, b, &[S::KeywordControlType]);
            }
            self.cap(
                Kind::ExportDefault,
                d,
                d + "default".len(),
                &[S::KeywordControlDefault],
            );
            self.push(Kind::ExportDefault);
            return Some(d + 7);
        }
        // `export type`? followed by `*`, `{` or a name list.
        let mut after = e;
        let mut type_cap = None;
        let t = self.ws(e);
        if t > e && self.starts(t, "type") && self.boundary(t + "type".len()) {
            // The optional `type` is tried first, then skipped.
            let try_type = self.export_clause_ahead(t + "type".len());
            if try_type {
                type_cap = Some((t, t + "type".len()));
                after = t + "type".len();
            }
        }
        if !self.boundary(after) {
            return None;
        }
        // `(?!(\$)|(\s*:))`.
        if self.b(after) == b'$' || self.b(self.ws(after)) == b':' {
            return None;
        }
        if !self.export_clause_ahead(after) {
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
    fn export_clause_ahead(&self, i: usize) -> bool {
        let j = self.ws(i);
        if matches!(self.b(j), b'*' | b'{') {
            return true;
        }
        let Some(e) = self.ident(j) else {
            return false;
        };
        if !(self.b(e) == b',' || self.space_at(e)) {
            return false;
        }
        !self.statement_keyword_at(j)
    }

    /// `\s*(?:abstract|async|…)\b` at `j`.
    pub(crate) fn statement_keyword_at(&self, j: usize) -> bool {
        self.statement_keyword(j).is_some()
    }
}
