//! Members of classes, interfaces, type literals and object literals.

use super::bytes::{ANY, BLANK, Bytes, MEMBER_START, STRING_START};
use super::kind::Kind;
use super::lexer::Lexer;
use super::scope::S;

/// The captures of a method's modifiers, kept inline: a prefix holds at most
/// the four modifiers the grammar lists, and the lookahead builds one at every
/// word of a class body.
struct CapList {
    len: usize,
    items: [(usize, usize, S); Self::MAX],
}

impl Default for CapList {
    fn default() -> Self {
        CapList {
            len: 0,
            items: [(0, 0, S::Source); Self::MAX],
        }
    }
}

impl CapList {
    const MAX: usize = 4;

    fn push(&mut self, cap: (usize, usize, S)) {
        if let Some(slot) = self.items.get_mut(self.len) {
            *slot = cap;
            self.len += 1;
        }
    }

    fn items(&self) -> &[(usize, usize, S)] {
        &self.items[..self.len]
    }
}

/// The modifiers before a method, as the grammar's prefix spells them.
#[derive(Default)]
struct MethodPrefix {
    caps: CapList,
    end: usize,
}

impl<const D: u8> Lexer<'_, D> {
    // Class members.

    pub(crate) fn class_body_rules(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.comment(p) {
            return Some(r);
        }
        if let Some(r) = self.decorator(p) {
            return Some(r);
        }
        if self.prev_char(p) == Some(':') && !self.full() {
            let j = self.ws(p);
            self.flush(p);
            self.fin(Kind::ClassColon, j);
            self.push(Kind::ClassColon);
            return Some(j);
        }
        self.only(
            "c.method_declaration",
            MEMBER_START,
            p,
            Self::method_declaration,
        )
        .or_else(|| {
            self.only(
                "c.indexer_declaration",
                BLANK.with(b'r').with(b'['),
                p,
                Self::indexer_declaration,
            )
        })
        .or_else(|| {
            self.only(
                "c.field_declaration",
                MEMBER_START,
                p,
                Self::field_declaration,
            )
        })
        .or_else(|| self.only("c.string", STRING_START, p, Self::string))
        .or_else(|| {
            self.only(
                "c.type_annotation",
                Bytes::of(b":"),
                p,
                Self::type_annotation,
            )
        })
        .or_else(|| {
            self.only(
                "c.variable_initializer",
                Bytes::of(b"="),
                p,
                Self::variable_initializer,
            )
        })
        .or_else(|| {
            self.only(
                "c.access_modifier",
                Bytes::of(b"adoprs"),
                p,
                Self::access_modifier,
            )
        })
        .or_else(|| {
            self.only(
                "c.property_accessor",
                Bytes::of(b"a\x67s"),
                p,
                Self::property_accessor,
            )
        })
        .or_else(|| self.only("c.async_modifier", Bytes::of(b"a"), p, Self::async_modifier))
        .or_else(|| {
            self.only(
                "c.after_operator_block_as_object_literal",
                BLANK.with(b'{'),
                p,
                Self::after_operator_block_as_object_literal,
            )
        })
        .or_else(|| self.only("c.decl_block", Bytes::of(b"{"), p, Self::decl_block))
        .or_else(|| self.only("c.expression", ANY, p, Self::expression))
        .or_else(|| {
            self.only(
                "c.punctuation_comma",
                Bytes::of(b","),
                p,
                Self::punctuation_comma,
            )
        })
        .or_else(|| {
            self.only(
                "c.punctuation_semicolon",
                Bytes::of(b";"),
                p,
                Self::punctuation_semicolon,
            )
        })
    }

    fn access_modifier(&mut self, p: usize) -> Option<usize> {
        let e = self.kw_of(
            p,
            &[
                "abstract",
                "declare",
                "override",
                "public",
                "protected",
                "private",
                "readonly",
                "static",
            ],
        )?;
        self.tok(p, e, S::StorageModifier)
    }

    fn property_accessor(&mut self, p: usize) -> Option<usize> {
        let e = self.kw_of(p, &["accessor", "get", "set"])?;
        self.tok(p, e, S::StorageTypeProperty)
    }

    fn async_modifier(&mut self, p: usize) -> Option<usize> {
        let e = self.kw(p, "async")?;
        self.tok(p, e, S::StorageModifierAsync)
    }

    /// The modifiers before a method, in the order the grammar lists them:
    /// `override`, a visibility, `abstract` and `async`, each followed by
    /// blanks, after the keyword guard at `p`.
    fn method_prefix(&self, p: usize) -> Option<MethodPrefix> {
        if !self.kw_before(p) {
            return None;
        }
        let mut prefix = MethodPrefix {
            caps: CapList::default(),
            end: p,
        };
        let mut at = p;
        let steps: [&[&str]; 4] = [
            &["override"],
            &["public", "private", "protected"],
            &["abstract"],
            &["async"],
        ];
        for (i, words) in steps.iter().enumerate() {
            for word in *words {
                if self.starts(at, word) && self.boundary(at) {
                    let e = at + word.len();
                    let k = self.ws(e);
                    if k > e {
                        let scope = if i == 3 {
                            S::StorageModifierAsync
                        } else {
                            S::StorageModifier
                        };
                        prefix.caps.push((at, e, scope));
                        at = k;
                        break;
                    }
                }
            }
        }
        prefix.end = at;
        Some(prefix)
    }

    pub(crate) fn method_declaration(&mut self, p: usize) -> Option<usize> {
        if self.full() {
            return None;
        }
        let first = self.method_prefix(p)?;
        // The grammar's optional groups back off, so the empty prefix is
        // tried when the full one does not lead to a method.
        let empty = MethodPrefix {
            caps: CapList::default(),
            end: p,
        };
        for prefix in [&first, &empty] {
            if let Some(r) = self.method_with_prefix(p, prefix) {
                return Some(r);
            }
        }
        None
    }

    fn method_with_prefix(&mut self, p: usize, prefix: &MethodPrefix) -> Option<usize> {
        let at = prefix.end;
        // `constructor`.
        let k = self.ws(at);
        if self.starts(k, "constructor")
            && self.boundary(k)
            && self.boundary(k + 11)
            && self.b(k + 11) != b':'
            && self.kw_after(k + 11)
        {
            self.flush(p);
            for &(a, b, s) in prefix.caps.items() {
                self.cap(Kind::Method, a, b, &[s]);
            }
            self.cap(Kind::Method, k, k + 11, &[S::StorageType]);
            self.push(Kind::Method);
            return Some(k + 11);
        }
        // `new (` or a bare call signature.
        let mut sig = at;
        let mut new_cap = None;
        let mut star_cap = None;
        if self.starts(k, "new")
            && self.boundary(k)
            && self.boundary(k + 3)
            && self.b(k + 3) != b':'
            && self.kw_after(k + 3)
        {
            new_cap = Some((k, k + 3));
            sig = k + 3;
        } else if self.b(at) == b'*' {
            star_cap = Some((at, at + 1));
            sig = self.ws(at + 1);
        }
        let la = self.ws(sig);
        if self.tp_paren(la).is_some() {
            self.flush(p);
            for &(a, b, s) in prefix.caps.items() {
                self.cap(Kind::Method, a, b, &[s]);
            }
            if let Some((a, b)) = new_cap {
                self.cap(Kind::Method, a, b, &[S::KeywordOperatorNew]);
            }
            if let Some((a, b)) = star_cap {
                self.cap(Kind::Method, a, b, &[S::KeywordGeneratorAsterisk]);
            }
            self.fin(Kind::Method, sig);
            self.push(Kind::Method);
            return Some(sig);
        }
        // A named method, with an optional `get`, `set` and `*`.
        let mut j = at;
        let mut accessor = None;
        if (self.starts(j, "get") || self.starts(j, "set")) && self.boundary(j) {
            let e = j + 3;
            let n = self.ws(e);
            if n > e {
                accessor = Some((j, e));
                j = n;
            }
        }
        let mut star = None;
        if self.b(j) == b'*' {
            star = Some((j, j + 1));
            j = self.ws(j + 1);
        }
        if self.named_method_ahead(j) {
            self.flush(p);
            for &(a, b, s) in prefix.caps.items() {
                self.cap(Kind::Method, a, b, &[s]);
            }
            if let Some((a, b)) = accessor {
                self.cap(Kind::Method, a, b, &[S::StorageTypeProperty]);
            }
            if let Some((a, b)) = star {
                self.cap(Kind::Method, a, b, &[S::KeywordGeneratorAsterisk]);
            }
            self.fin(Kind::Method, j);
            self.push(Kind::Method);
            return Some(j);
        }
        // Without `get` or `set` the word is the method's name.
        if accessor.is_some() {
            let mut j = at;
            let mut star = None;
            if self.b(j) == b'*' {
                star = Some((j, j + 1));
                j = self.ws(j + 1);
            }
            if self.named_method_ahead(j) {
                self.flush(p);
                for &(a, b, s) in prefix.caps.items() {
                    self.cap(Kind::Method, a, b, &[s]);
                }
                if let Some((a, b)) = star {
                    self.cap(Kind::Method, a, b, &[S::KeywordGeneratorAsterisk]);
                }
                self.fin(Kind::Method, j);
                self.push(Kind::Method);
                return Some(j);
            }
        }
        None
    }

    /// `\s*(KEY\s*(\??))\s*(<…>\s*)?\(` at `j`.
    pub(crate) fn named_method_ahead(&self, j: usize) -> bool {
        let k = self.ws(j);
        let Some(e) = self.key_end(k) else {
            return false;
        };
        let mut m = self.ws(e);
        if self.b(m) == b'?' {
            m += 1;
        }
        let m = self.ws(m);
        self.tp_paren(m).is_some()
    }

    /// `(?=(KEY)\s*(\??)\s*[(<])` begins the name of a method.
    pub(crate) fn method_declaration_name(&mut self, p: usize) -> Option<usize> {
        let e = self.key_end(p)?;
        let mut m = self.ws(e);
        if self.b(m) == b'?' {
            m = self.ws(m + 1);
        }
        if !matches!(self.b(m), b'(' | b'<') {
            return None;
        }
        self.open0(Kind::MethodName, p)
    }

    pub(crate) fn method_name_rules(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.string(p) {
            return Some(r);
        }
        if let Some(r) = self.array_literal(p) {
            return Some(r);
        }
        if let Some(r) = self.numeric_literal(p) {
            return Some(r);
        }
        if let Some(e) = self.ident(p) {
            self.one(p, e, &[S::MetaDefinitionMethod, S::EntityNameFunction]);
            return Some(e);
        }
        (self.b(p) == b'?').then(|| self.tok(p, p + 1, S::KeywordOperatorOptional))?
    }

    pub(crate) fn method_rules(&mut self, p: usize) -> Option<usize> {
        self.method_declaration_name(p)
            .or_else(|| self.function_body(p))
    }

    /// `(?:B(readonly)\s*)?\s*(\[)\s*(ID)\s*(?=:)`.
    pub(crate) fn indexer_declaration(&mut self, p: usize) -> Option<usize> {
        if self.full() {
            return None;
        }
        let mut j = p;
        let mut readonly = None;
        if self.kw(p, "readonly").is_some() {
            let e = p + 8;
            readonly = Some((p, e));
            j = e;
        }
        let o = self.ws(j);
        if self.b(o) != b'[' {
            return None;
        }
        let n0 = self.ws(o + 1);
        let n = self.ident(n0)?;
        let c = self.ws(n);
        if self.b(c) != b':' {
            return None;
        }
        self.flush(p);
        if let Some((a, b)) = readonly {
            self.cap(Kind::Indexer, a, b, &[S::StorageModifier]);
        }
        self.cap(Kind::Indexer, o, o + 1, &[S::MetaBraceSquare]);
        self.cap(Kind::Indexer, n0, n, &[S::VariableParameter]);
        self.fin(Kind::Indexer, c);
        self.push(Kind::Indexer);
        Some(c)
    }

    /// `(?:B([-+])?(readonly)\s*)?\s*(\[)\s*(ID)\s+(in)\s+`.
    pub(crate) fn indexer_mapped_type_declaration(&mut self, p: usize) -> Option<usize> {
        if self.full() {
            return None;
        }
        let mut j = p;
        let mut sign = None;
        let mut readonly = None;
        if self.kw_before(p) {
            let mut k = p;
            if matches!(self.b(k), b'-' | b'+') {
                sign = Some((k, k + 1));
                k += 1;
            }
            if self.starts(k, "readonly") {
                let e = k + 8;
                let w = self.ws(e);
                readonly = Some((k, e));
                j = w;
            } else {
                sign = None;
            }
        }
        let attempt = |this: &mut Self,
                       start: usize,
                       sign: Option<(usize, usize)>,
                       readonly: Option<(usize, usize)>| {
            let o = this.ws(start);
            if this.b(o) != b'[' {
                return None;
            }
            let n0 = this.ws(o + 1);
            let n = this.ident(n0)?;
            let w = this.ws(n);
            if w == n || !this.starts(w, "in") {
                return None;
            }
            let after = this.ws(w + 2);
            if after == w + 2 {
                return None;
            }
            this.flush(p);
            if let Some((a, b)) = sign {
                this.cap(Kind::IndexerMapped, a, b, &[S::KeywordOperatorTypeModifier]);
            }
            if let Some((a, b)) = readonly {
                this.cap(Kind::IndexerMapped, a, b, &[S::StorageModifier]);
            }
            this.cap(Kind::IndexerMapped, o, o + 1, &[S::MetaBraceSquare]);
            this.cap(Kind::IndexerMapped, n0, n, &[S::EntityNameType]);
            this.cap(
                Kind::IndexerMapped,
                w,
                w + 2,
                &[S::KeywordOperatorExpressionIn],
            );
            this.fin(Kind::IndexerMapped, after);
            this.push(Kind::IndexerMapped);
            Some(after)
        };
        if readonly.is_some()
            && let Some(r) = attempt(self, j, sign, readonly)
        {
            return Some(r);
        }
        attempt(self, p, None, None)
    }

    /// `(?<!\()(?:B(readonly)\s+)?(?=\s*KEY\s*[?!]?\s*([,:;=}]|$))`.
    pub(crate) fn field_declaration(&mut self, p: usize) -> Option<usize> {
        if self.prev_char(p) == Some('(') || self.full() {
            return None;
        }
        let mut readonly = None;
        let mut j = p;
        if self.kw_before(p) && self.starts(p, "readonly") {
            let e = p + 8;
            let k = self.ws(e);
            if k > e && self.field_ahead(k) {
                readonly = Some((p, e));
                j = k;
            }
        }
        if readonly.is_none() && !self.field_ahead(p) {
            return None;
        }
        self.flush(p);
        if let Some((a, b)) = readonly {
            self.cap(Kind::Field, a, b, &[S::StorageModifier]);
        }
        self.fin(Kind::Field, j);
        self.push(Kind::Field);
        Some(j)
    }

    /// `\s*(KEY)\s*(?:(?:(\?)|(!))\s*)?([,:;=}]|$)`.
    pub(crate) fn field_ahead(&self, i: usize) -> bool {
        let k = self.ws(i);
        let Some(e) = self.field_key_end(k) else {
            return false;
        };
        let mut m = self.ws(e);
        if matches!(self.b(m), b'?' | b'!') {
            m = self.ws(m + 1);
        }
        matches!(self.b(m), b',' | b':' | b';' | b'=' | b'}') || self.eol(m)
    }

    /// A field key: a number, `#?` name, string or computed name.
    fn field_key_end(&self, k: usize) -> Option<usize> {
        self.number_end(k)
            .or_else(|| self.hash_ident(k))
            .or_else(|| self.quoted(k))
            .or_else(|| self.computed_key(k))
    }

    pub(crate) fn field_rules(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.variable_initializer(p) {
            return Some(r);
        }
        if let Some(r) = self.type_annotation(p) {
            return Some(r);
        }
        if let Some(r) = self.string(p) {
            return Some(r);
        }
        if let Some(r) = self.array_literal(p) {
            return Some(r);
        }
        if let Some(r) = self.numeric_literal(p) {
            return Some(r);
        }
        if let Some(r) = self.comment(p) {
            return Some(r);
        }
        if let Some(e) = self.hash_ident(p) {
            let mut k = e;
            let mut optional = None;
            let mut definite = None;
            match self.b(k) {
                b'?' => {
                    optional = Some(k);
                    k += 1;
                }
                b'!' => {
                    definite = Some(k);
                    k += 1;
                }
                _ => {}
            }
            if self.named_fn_ahead(k) {
                self.flush(p);
                self.cap(
                    Kind::Top,
                    p,
                    e,
                    &[S::MetaDefinitionProperty, S::EntityNameFunction],
                );
                if let Some(at) = optional {
                    self.cap(Kind::Top, at, at + 1, &[S::KeywordOperatorOptional]);
                }
                if let Some(at) = definite {
                    self.cap(
                        Kind::Top,
                        at,
                        at + 1,
                        &[S::KeywordOperatorDefiniteassignment],
                    );
                }
                return Some(k);
            }
            self.one(
                p,
                e,
                &[S::MetaDefinitionProperty, S::VariableObjectProperty],
            );
            return Some(e);
        }
        match self.b(p) {
            b'?' => self.tok(p, p + 1, S::KeywordOperatorOptional),
            b'!' => self.tok(p, p + 1, S::KeywordOperatorDefiniteassignment),
            _ => None,
        }
    }

    // Object literal members.

    pub(crate) fn object_member(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.comment(p) {
            return Some(r);
        }
        if let Some(r) = self.object_literal_method_declaration(p, Kind::ObjMethod) {
            return Some(r);
        }
        if self.b(p) == b'[' {
            return self.open0(Kind::ObjKeyBracket, p);
        }
        if matches!(self.b(p), b'"' | b'\'' | b'`') {
            return self.open0(Kind::ObjKeyStr, p);
        }
        if self.number_end(p).is_some() {
            return self.open0(Kind::ObjKeyNum, p);
        }
        if matches!(self.prev_char(p), Some(']' | '"' | '\'' | '`')) {
            let j = self.ws(p);
            if matches!(self.b(j), b'(' | b'<') && !self.full() {
                return self.open0(Kind::ObjMethodAfterKey, p);
            }
        }
        if let Some(r) = self.object_member_key(p) {
            return Some(r);
        }
        if self.starts(p, "...") {
            return self.open1(Kind::ObjSpread, p, p + 3, &[S::KeywordOperatorSpread]);
        }
        if let Some(e) = self.ident(p) {
            let j = self.ws(e);
            if matches!(self.b(j), b',' | b'}')
                || self.eol(j)
                || self.starts(j, "//")
                || self.starts(j, "/*")
            {
                self.flush(p);
                self.cap(Kind::ObjMember, p, e, &[S::VariableOtherReadwrite]);
                self.fin(Kind::ObjMember, j);
                return Some(j);
            }
        }
        if let Some(r) = self.object_as(p) {
            return Some(r);
        }
        if let Some(e) = self.ident(p) {
            let j = self.ws(e);
            if self.b(j) == b'=' && !self.full() {
                return self.open0(Kind::ObjInit, p);
            }
        }
        if self.b(p) == b':' {
            return self.object_value_begin(p);
        }
        self.punctuation_comma(p).or_else(|| self.decl_block(p))
    }

    /// `B(?:\b(async)\s+)?(?:\b([gs]et)\s+)?(?:(\*)\s*)?(?=…\()`.
    pub(crate) fn object_literal_method_declaration(
        &mut self,
        p: usize,
        kind: Kind,
    ) -> Option<usize> {
        if !self.kw_before(p) || self.full() {
            return None;
        }
        let mut at = p;
        let mut caps = CapList::default();
        if self.starts(at, "async") && self.boundary(at) {
            let e = at + 5;
            let k = self.ws(e);
            if k > e {
                caps.push((at, e, S::StorageModifierAsync));
                at = k;
            }
        }
        let mut with_accessor = at;
        let mut accessor_cap = None;
        if (self.starts(at, "get") || self.starts(at, "set")) && self.boundary(at) {
            let e = at + 3;
            let k = self.ws(e);
            if k > e {
                accessor_cap = Some((at, e));
                with_accessor = k;
            }
        }
        for (start, acc) in [(with_accessor, accessor_cap), (at, None)] {
            let mut j = start;
            let mut star = None;
            if self.b(j) == b'*' {
                star = Some((j, j + 1));
                j = self.ws(j + 1);
            }
            if self.named_method_ahead(j) {
                self.flush(p);
                for &(a, b, s) in caps.items() {
                    self.cap(kind, a, b, &[s]);
                }
                if let Some((a, b)) = acc {
                    self.cap(kind, a, b, &[S::StorageTypeProperty]);
                }
                if let Some((a, b)) = star {
                    self.cap(kind, a, b, &[S::KeywordGeneratorAsterisk]);
                }
                self.fin(kind, j);
                self.push(kind);
                return Some(j);
            }
            if acc.is_none() {
                break;
            }
        }
        // Without the `async` prefix.
        if !caps.items().is_empty() {
            let j = p;
            if self.named_method_ahead(j) {
                self.flush(p);
                self.fin(kind, j);
                self.push(kind);
                return Some(j);
            }
        }
        None
    }

    /// The key forms before a `:`.
    fn object_member_key(&mut self, p: usize) -> Option<usize> {
        // `(?![$_[:alpha:]])(\d+)\s*(?=comments*:)`.
        if self.b(p).is_ascii_digit() {
            let mut e = p;
            while self.b(e).is_ascii_digit() {
                e += 1;
            }
            let j = self.ws(e);
            if self.b(self.ws_comments(j)) == b':' {
                self.flush(p);
                self.cap(Kind::ObjMember, p, j, &[S::MetaObjectLiteralKey]);
                self.cap(
                    Kind::ObjMember,
                    p,
                    e,
                    &[S::MetaObjectLiteralKey, S::ConstantNumericDecimal],
                );
                return Some(j);
            }
        }
        let e = self.ident(p)?;
        let j = self.ws(e);
        let after = self.ws_comments(j);
        if self.b(after) != b':' {
            return None;
        }
        // The value is a function.
        let v = self.ws_comments_after_colon(after + 1);
        if self.fn_value_ahead(v) {
            self.flush(p);
            self.cap(
                Kind::ObjMember,
                p,
                e,
                &[S::MetaObjectLiteralKey, S::EntityNameFunction],
            );
            self.cap(Kind::ObjMember, e, j, &[S::MetaObjectLiteralKey]);
            return Some(j);
        }
        self.flush(p);
        self.cap(Kind::ObjMember, p, j, &[S::MetaObjectLiteralKey]);
        Some(j)
    }

    /// `(\s*/\*…\*/)*\s*` after a colon.
    fn ws_comments_after_colon(&self, i: usize) -> usize {
        let mut j = i;
        loop {
            let k = self.ws(j);
            if let Some(e) = self.block_comment(k) {
                j = e;
            } else {
                return k;
            }
        }
    }

    /// `B(as)\s+(const)(?=\s*([,}]|$))` and `as`/`satisfies` types.
    fn object_as(&mut self, p: usize) -> Option<usize> {
        if let Some(e) = self.kw(p, "as") {
            let j = self.ws(e);
            if j > e {
                if let Some(c) = self.kw(j, "const") {
                    let k = self.ws(c);
                    if matches!(self.b(k), b',' | b'}') || self.eol(k) {
                        self.flush(p);
                        self.cap(Kind::ObjMember, p, e, &[S::KeywordControlAs]);
                        self.cap(Kind::ObjMember, j, c, &[S::StorageModifier]);
                        return Some(c);
                    }
                }
                if !self.full() {
                    self.flush(p);
                    self.cap(Kind::ObjAs, p, e, &[S::KeywordControlAs]);
                    self.fin(Kind::ObjAs, j);
                    self.push(Kind::ObjAs);
                    return Some(j);
                }
            }
        }
        if let Some(e) = self.kw(p, "satisfies") {
            let j = self.ws(e);
            if j > e && !self.full() {
                self.flush(p);
                self.cap(Kind::ObjAs, p, e, &[S::KeywordControlSatisfies]);
                self.fin(Kind::ObjAs, j);
                self.push(Kind::ObjAs);
                return Some(j);
            }
        }
        None
    }

    fn object_value_begin(&mut self, p: usize) -> Option<usize> {
        self.open1(
            Kind::ObjValue,
            p,
            p + 1,
            &[S::MetaObjectLiteralKey, S::PunctuationSeparatorKeyValue],
        )
    }

    /// Inside the value of an object member.
    pub(crate) fn object_value_rules(&mut self, p: usize) -> Option<usize> {
        if self.full() {
            return self.possibly_arrow_then_expression(p);
        }
        if let Some(r) = self.object_value_arrow(p) {
            return Some(r);
        }
        self.possibly_arrow_then_expression(p)
    }

    fn possibly_arrow_then_expression(&mut self, p: usize) -> Option<usize> {
        self.possibly_arrow_return_type(p)
            .or_else(|| self.expression(p))
    }

    /// The four forms of arrow parameters at the start of an object value.
    fn object_value_arrow(&mut self, p: usize) -> Option<usize> {
        let prev = self.prev_char(p);
        // `(?<=:)\s*(async)?(?=\s*(<…>)\(\s*(…open…))`.
        if prev == Some(':') {
            let j = self.ws(p);
            let mut ends = Vec::with_capacity(2);
            if self.starts(j, "async") {
                ends.push(j + 5);
            }
            ends.push(j);
            for &e in &ends {
                let k = self.ws(e);
                if self.b(k) == b'<'
                    && self.tpa(k).is_some_and(|t| self.b(t) == b'(')
                    && self.paren_open_ahead(k)
                {
                    self.flush(p);
                    if e > j {
                        self.cap(Kind::Top, j, e, &[S::StorageModifierAsync]);
                    }
                    self.push(Kind::ObjArrowGeneric);
                    return Some(e);
                }
            }
            // `(?<=:)\s*(async)?\s*(\()(?=…open…)`.
            for &e in &ends {
                let k = self.ws(e);
                if self.b(k) == b'(' && self.paren_open_after(k + 1) {
                    self.flush(p);
                    if e > j {
                        self.cap(Kind::Top, j, e, &[S::StorageModifierAsync]);
                    }
                    self.cap(Kind::ObjArrowParen, k, k + 1, &[S::MetaBraceRound]);
                    self.push(Kind::ObjArrowParen);
                    return Some(k + 1);
                }
            }
            // `(?<=:)\s*(async)?(?=\s*<\s*$)`.
            for &e in &ends {
                let k = self.ws(e);
                if self.b(k) == b'<' && self.eol(self.ws(k + 1)) {
                    self.flush(p);
                    if e > j {
                        self.cap(Kind::Top, j, e, &[S::StorageModifierAsync]);
                    }
                    self.push(Kind::ObjArrowOpen);
                    return Some(e);
                }
            }
        }
        // `(?<=>)\s*(\()(?=…open…)`.
        if prev == Some('>') {
            let k = self.ws(p);
            if self.b(k) == b'(' && self.paren_open_after(k + 1) && !self.full() {
                self.flush(p);
                self.cap(Kind::ObjArrowParen, k, k + 1, &[S::MetaBraceRound]);
                self.push(Kind::ObjArrowParen);
                return Some(k + 1);
            }
        }
        None
    }

    /// The text after a `(`: `\s*(([\[{]\s*)?$|…destructured…)`.
    fn paren_open_after(&self, i: usize) -> bool {
        let m = self.ws(i);
        if self.eol(m) {
            return true;
        }
        if matches!(self.b(m), b'[' | b'{') && self.eol(self.ws(m + 1)) {
            return true;
        }
        match self.b(m) {
            b'{' => self
                .balanced(m, b'{', b'}')
                .is_some_and(|e| self.param_default_tail(e, b'{')),
            b'[' => self
                .balanced(m, b'[', b']')
                .is_some_and(|e| self.param_default_tail(e, b'[')),
            _ => false,
        }
    }

    pub(crate) fn object_arrow_generic_rules(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.type_parameters(p) {
            return Some(r);
        }
        if self.b(p) == b'(' {
            return self.open1(Kind::ObjArrowParen, p, p + 1, &[S::MetaBraceRound]);
        }
        None
    }

    /// Inside parentheses that may hold arrow parameters.
    pub(crate) fn expression_inside_possibly_arrow_parens(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.expression_without_identifiers(p) {
            return Some(r);
        }
        if let Some(r) = self.comment(p) {
            return Some(r);
        }
        if let Some(r) = self.string(p) {
            return Some(r);
        }
        if let Some(r) = self.decorator(p) {
            return Some(r);
        }
        if let Some(r) = self.destructuring_parameter(p) {
            return Some(r);
        }
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
                self.tok(p, e, S::StorageModifier);
                return Some(j);
            }
        }
        if let Some(r) = self.parameter_like(p, super::functions::ParamStyle::ExpressionParameter) {
            return Some(r);
        }
        self.type_annotation(p)
            .or_else(|| self.variable_initializer(p))
            .or_else(|| {
                (self.b(p) == b',').then(|| self.tok(p, p + 1, S::PunctuationSeparatorParameter))?
            })
            .or_else(|| self.identifiers(p))
            .or_else(|| self.expression_punctuation(p))
    }

    // Parameter binding patterns.

    pub(crate) fn parameter_binding_element(&mut self, p: usize) -> Option<usize> {
        self.comment(p)
            .or_else(|| self.string(p))
            .or_else(|| self.numeric_literal(p))
            .or_else(|| self.regex(p))
            .or_else(|| self.parameter_pattern(p, true))
            .or_else(|| self.parameter_pattern(p, false))
            .or_else(|| self.destructuring_parameter_rest(p))
            .or_else(|| self.variable_initializer(p))
    }

    /// `(?:(\.\.\.)\s*)?(\{)` or `(\[)` inside a parameter pattern.
    pub(crate) fn parameter_pattern(&mut self, p: usize, curly: bool) -> Option<usize> {
        let mut j = p;
        let mut rest = None;
        if self.starts(j, "...") {
            rest = Some((j, j + 3));
            j = self.ws(j + 3);
        }
        let want = if curly { b'{' } else { b'[' };
        if self.b(j) != want || self.full() {
            return None;
        }
        self.flush(p);
        if let Some((a, b)) = rest {
            self.cap(Kind::Top, a, b, &[S::KeywordOperatorRest]);
        }
        let (kind, scope) = if curly {
            (
                Kind::ParamObjPattern,
                S::PunctuationDefinitionBindingPatternObject,
            )
        } else {
            (
                Kind::ParamArrPattern,
                S::PunctuationDefinitionBindingPatternArray,
            )
        };
        self.cap(kind, j, j + 1, &[scope]);
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
            self.cap(Kind::Top, a, b, &[S::KeywordOperatorRest]);
        }
        self.cap(Kind::Top, j, e, &[S::VariableParameter]);
        Some(e)
    }

    pub(crate) fn parameter_object_binding_element(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.comment(p) {
            return Some(r);
        }
        if self.key_colon_ahead(p) {
            return self.open0(Kind::ParamObjElem, p);
        }
        self.parameter_pattern(p, true)
            .or_else(|| self.destructuring_parameter_rest(p))
            .or_else(|| self.variable_initializer(p))
            .or_else(|| self.punctuation_comma(p))
    }
}
