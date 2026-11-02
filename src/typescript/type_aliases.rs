//! Type alias declarations.

use super::kind::Kind;
use super::lexer::Lexer;
use super::scope::S;

impl Lexer<'_> {
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
}
