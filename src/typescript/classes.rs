//! Classes, interfaces and their heritage clauses.

use super::kind::Kind;
use super::lexer::Lexer;
use super::scope::S;

impl<const D: u8> Lexer<'_, D> {
    pub(crate) fn class_declaration(&mut self, p: usize) -> Option<usize> {
        let pre = self.declaration_start(p)?;
        let mut k = pre.at;
        let mut abstract_cap = None;
        if self.starts(k, "abstract") && self.boundary(k) {
            let e = k + "abstract".len();
            let j = self.ws(e);
            abstract_cap = Some((k, e));
            k = j;
        }
        if !self.starts(k, "class") || !self.boundary(k + "class".len()) || self.full() {
            return None;
        }
        let e = k + "class".len();
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
            let e = k + "abstract".len();
            let j = self.ws(e);
            if j > e {
                abstract_cap = Some((k, e));
                k = j;
            }
        }
        if !self.starts(k, "class") || !self.boundary(k + "class".len()) || self.full() {
            return None;
        }
        let e = k + "class".len();
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
            let e = k + "abstract".len();
            abstract_cap = Some((k, e));
            k = self.ws(e);
        }
        if !self.starts(k, "interface") || !self.boundary(k + "interface".len()) || self.full() {
            return None;
        }
        let e = k + "interface".len();
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
}
