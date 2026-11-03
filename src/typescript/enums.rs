//! Enum declarations.

use super::kind::Kind;
use super::lexer::Lexer;
use super::scope::S;

impl<const D: u8> Lexer<'_, D> {
    #[expect(
        clippy::many_single_char_names,
        reason = "positions are named by role: `p` where the rule starts, `e` where a part ends, `j` and `k` for the cursor"
    )]
    pub(crate) fn enum_declaration(&mut self, p: usize) -> Option<usize> {
        let pre = self.declaration_start(p)?;
        let mut k = pre.at;
        let mut const_cap = None;
        if self.starts(k, "const") && self.boundary(k) {
            let e = k + "const".len();
            let j = self.ws(e);
            if j > e {
                const_cap = Some((k, e));
                k = j;
            }
        }
        if !self.starts(k, "enum") || !self.boundary(k) || self.full() {
            return None;
        }
        let e = k + "enum".len();
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
}
