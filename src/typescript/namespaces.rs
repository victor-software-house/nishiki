//! Namespace and module declarations.

use super::kind::Kind;
use super::lexer::Lexer;
use super::scope::S;

impl<const D: u8> Lexer<'_, D> {
    #[expect(
        clippy::many_single_char_names,
        reason = "positions are named by role: `p` where the rule starts, `e` where a part ends, `j` and `k` for the cursor"
    )]
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
        let c = self.byte(j);
        let ok = matches!(c, b'"' | b'$' | b'\'' | b'_' | b'`' | b'[') || self.id_start(j);
        if !ok {
            return None;
        }
        self.flush(p);
        self.prefix_tokens(Kind::NamespaceDecl, &pre);
        self.capture(Kind::NamespaceDecl, k, e, &[S::StorageTypeNamespace]);
        self.finish(Kind::NamespaceDecl, j);
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
            return self.token_rule(p, e, S::EntityNameTypeModule);
        }
        self.punctuation_accessor(p).or_else(|| self.decl_block(p))
    }
}
