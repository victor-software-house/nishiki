//! Members of classes, interfaces, type literals and object literals.

use super::bytes::{ANY, BLANK, Bytes, MEMBER_START, STRING_START};
use super::lexer::Lexer;
use super::scope::S;
use super::state::Rule;
use crate::classes;

/// The captures of a method's modifiers, kept inline.
/// A prefix holds at most the four modifiers the grammar lists.
/// The lookahead builds one at every word of a class body.
struct CaptureList {
    len: usize,
    items: [(usize, usize, S); Self::MAX],
}

impl Default for CaptureList {
    fn default() -> Self {
        CaptureList {
            len: 0,
            items: [(0, 0, S::Source); Self::MAX],
        }
    }
}

impl CaptureList {
    const MAX: usize = 4;

    fn push(&mut self, capture: (usize, usize, S)) {
        if let Some(slot) = self.items.get_mut(self.len) {
            *slot = capture;
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
    captures: CaptureList,
    end: usize,
}

impl<const D: u8> Lexer<'_, D> {
    pub(crate) fn class_body_rules(&mut self, p: usize) -> Option<usize> {
        self.comment(p)
            .or_else(|| self.decorator(p))
            .or_else(|| self.class_colon(p))
            .or_else(|| {
                self.only(
                    "c.method_declaration",
                    MEMBER_START,
                    p,
                    Self::method_declaration,
                )
            })
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
            .or_else(|| {
                self.only(
                    "c.declaration_block",
                    Bytes::of(b"{"),
                    p,
                    Self::declaration_block,
                )
            })
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

    /// The white space after a colon in a class body, in a `ClassColon` frame.
    fn class_colon(&mut self, p: usize) -> Option<usize> {
        if self.prev_char(p) != Some(':') || self.full() {
            return None;
        }
        let j = self.ws(p);
        self.flush(p);
        self.finish(Rule::ClassColon, j);
        self.push(Rule::ClassColon);
        Some(j)
    }

    fn access_modifier(&mut self, p: usize) -> Option<usize> {
        let e = self.keyword_of(
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
        self.token_rule(p, e, S::StorageModifier)
    }

    fn property_accessor(&mut self, p: usize) -> Option<usize> {
        let e = self.keyword_of(p, &["accessor", "get", "set"])?;
        self.token_rule(p, e, S::StorageTypeProperty)
    }

    fn async_modifier(&mut self, p: usize) -> Option<usize> {
        let e = self.keyword(p, "async")?;
        self.token_rule(p, e, S::StorageModifierAsync)
    }

    /// The modifiers before a method, in the order the grammar lists them:
    /// `override`, a visibility, `abstract` and `async`, each followed by
    /// blanks, after the keyword guard at `p`.
    fn method_prefix(&self, p: usize) -> Option<MethodPrefix> {
        if !self.keyword_before(p) {
            return None;
        }
        let mut prefix = MethodPrefix {
            captures: CaptureList::default(),
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
                        prefix.captures.push((at, e, scope));
                        at = k;
                        break;
                    }
                }
            }
        }
        prefix.end = at;
        Some(prefix)
    }

    /// A method declaration in a class or an interface.
    ///
    /// The grammar's optional groups back off.
    /// The empty prefix is therefore tried when the full one does not lead to
    /// a method.
    pub(crate) fn method_declaration(&mut self, p: usize) -> Option<usize> {
        if self.full() {
            return None;
        }
        let first = self.method_prefix(p)?;
        let empty = MethodPrefix {
            captures: CaptureList::default(),
            end: p,
        };
        for prefix in [&first, &empty] {
            if let Some(r) = self.method_with_prefix(p, prefix) {
                return Some(r);
            }
        }
        None
    }

    /// A method after `prefix`: `constructor`, `new (` or a bare call
    /// signature, or a named method with an optional `get`, `set` and `*`.
    ///
    /// Without `get` or `set` the word is the method's name.
    #[expect(
        clippy::many_single_char_names,
        reason = "positions are named by role: `p` where the rule starts, `e` where a part ends, `j` and `k` for the cursor"
    )]
    fn method_with_prefix(&mut self, p: usize, prefix: &MethodPrefix) -> Option<usize> {
        let at = prefix.end;
        let k = self.ws(at);
        if self.starts(k, "constructor")
            && self.boundary(k)
            && self.boundary(k + "constructor".len())
            && self.byte(k + "constructor".len()) != b':'
            && self.keyword_after(k + "constructor".len())
        {
            self.flush(p);
            for &(a, b, s) in prefix.captures.items() {
                self.capture(Rule::Method, a, b, &[s]);
            }
            self.capture(Rule::Method, k, k + "constructor".len(), &[S::StorageType]);
            self.push(Rule::Method);
            return Some(k + "constructor".len());
        }
        let mut sig = at;
        let mut new_capture = None;
        let mut star_capture = None;
        if self.starts(k, "new")
            && self.boundary(k)
            && self.boundary(k + "new".len())
            && self.byte(k + "new".len()) != b':'
            && self.keyword_after(k + "new".len())
        {
            new_capture = Some((k, k + "new".len()));
            sig = k + "new".len();
        } else if self.byte(at) == b'*' {
            star_capture = Some((at, at + 1));
            sig = self.ws(at + 1);
        }
        let la = self.ws(sig);
        if self.type_parameters_paren(la).is_some() {
            self.flush(p);
            for &(a, b, s) in prefix.captures.items() {
                self.capture(Rule::Method, a, b, &[s]);
            }
            if let Some((a, b)) = new_capture {
                self.capture(Rule::Method, a, b, &[S::KeywordOperatorNew]);
            }
            if let Some((a, b)) = star_capture {
                self.capture(Rule::Method, a, b, &[S::KeywordGeneratorAsterisk]);
            }
            self.finish(Rule::Method, sig);
            self.push(Rule::Method);
            return Some(sig);
        }
        let mut j = at;
        let mut accessor = None;
        if (self.starts(j, "get") || self.starts(j, "set")) && self.boundary(j) {
            let e = j + "get".len();
            let n = self.ws(e);
            if n > e {
                accessor = Some((j, e));
                j = n;
            }
        }
        let mut star = None;
        if self.byte(j) == b'*' {
            star = Some((j, j + 1));
            j = self.ws(j + 1);
        }
        if self.named_method_ahead(j) {
            self.flush(p);
            for &(a, b, s) in prefix.captures.items() {
                self.capture(Rule::Method, a, b, &[s]);
            }
            if let Some((a, b)) = accessor {
                self.capture(Rule::Method, a, b, &[S::StorageTypeProperty]);
            }
            if let Some((a, b)) = star {
                self.capture(Rule::Method, a, b, &[S::KeywordGeneratorAsterisk]);
            }
            self.finish(Rule::Method, j);
            self.push(Rule::Method);
            return Some(j);
        }
        if accessor.is_some() {
            let mut j = at;
            let mut star = None;
            if self.byte(j) == b'*' {
                star = Some((j, j + 1));
                j = self.ws(j + 1);
            }
            if self.named_method_ahead(j) {
                self.flush(p);
                for &(a, b, s) in prefix.captures.items() {
                    self.capture(Rule::Method, a, b, &[s]);
                }
                if let Some((a, b)) = star {
                    self.capture(Rule::Method, a, b, &[S::KeywordGeneratorAsterisk]);
                }
                self.finish(Rule::Method, j);
                self.push(Rule::Method);
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
        if self.byte(m) == b'?' {
            m += 1;
        }
        let m = self.ws(m);
        self.type_parameters_paren(m).is_some()
    }

    /// `(?=(KEY)\s*(\??)\s*[(<])` begins the name of a method.
    pub(crate) fn method_declaration_name(&mut self, p: usize) -> Option<usize> {
        let e = self.key_end(p)?;
        let mut m = self.ws(e);
        if self.byte(m) == b'?' {
            m = self.ws(m + 1);
        }
        if !matches!(self.byte(m), b'(' | b'<') {
            return None;
        }
        self.open_empty(Rule::MethodName, p)
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
            self.token(p, e, &[S::MetaDefinitionMethod, S::EntityNameFunction]);
            return Some(e);
        }
        (self.byte(p) == b'?').then(|| self.token_rule(p, p + 1, S::KeywordOperatorOptional))?
    }

    pub(crate) fn method_rules(&mut self, p: usize) -> Option<usize> {
        self.method_declaration_name(p)
            .or_else(|| self.function_body(p))
    }

    /// `(?:B(readonly)\s*)?\s*(\[)\s*(ID)\s*(?=:)`.
    #[expect(
        clippy::many_single_char_names,
        reason = "positions are named by role: `p` where the rule starts, `e` where a part ends, `j` and `k` for the cursor"
    )]
    pub(crate) fn indexer_declaration(&mut self, p: usize) -> Option<usize> {
        if self.full() {
            return None;
        }
        let mut j = p;
        let mut readonly = None;
        if self.keyword(p, "readonly").is_some() {
            let e = p + 8;
            readonly = Some((p, e));
            j = e;
        }
        let o = self.ws(j);
        if self.byte(o) != b'[' {
            return None;
        }
        let n0 = self.ws(o + 1);
        let n = self.ident(n0)?;
        let c = self.ws(n);
        if self.byte(c) != b':' {
            return None;
        }
        self.flush(p);
        if let Some((a, b)) = readonly {
            self.capture(Rule::Indexer, a, b, &[S::StorageModifier]);
        }
        self.capture(Rule::Indexer, o, o + 1, &[S::MetaBraceSquare]);
        self.capture(Rule::Indexer, n0, n, &[S::VariableParameter]);
        self.finish(Rule::Indexer, c);
        self.push(Rule::Indexer);
        Some(c)
    }

    /// `(?:B([-+])?(readonly)\s*)?\s*(\[)\s*(ID)\s+(in)\s+`.
    #[expect(
        clippy::many_single_char_names,
        reason = "positions are named by role: `p` where the rule starts, `e` where a part ends, `j` and `k` for the cursor"
    )]
    pub(crate) fn indexer_mapped_type_declaration(&mut self, p: usize) -> Option<usize> {
        if self.full() {
            return None;
        }
        let mut j = p;
        let mut sign = None;
        let mut readonly = None;
        if self.keyword_before(p) {
            let mut k = p;
            if matches!(self.byte(k), b'-' | b'+') {
                sign = Some((k, k + 1));
                k += 1;
            }
            if self.starts(k, "readonly") {
                let e = k + "readonly".len();
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
            if this.byte(o) != b'[' {
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
                this.capture(Rule::IndexerMapped, a, b, &[S::KeywordOperatorTypeModifier]);
            }
            if let Some((a, b)) = readonly {
                this.capture(Rule::IndexerMapped, a, b, &[S::StorageModifier]);
            }
            this.capture(Rule::IndexerMapped, o, o + 1, &[S::MetaBraceSquare]);
            this.capture(Rule::IndexerMapped, n0, n, &[S::EntityNameType]);
            this.capture(
                Rule::IndexerMapped,
                w,
                w + 2,
                &[S::KeywordOperatorExpressionIn],
            );
            this.finish(Rule::IndexerMapped, after);
            this.push(Rule::IndexerMapped);
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
        if self.keyword_before(p) && self.starts(p, "readonly") {
            let e = p + "readonly".len();
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
            self.capture(Rule::Field, a, b, &[S::StorageModifier]);
        }
        self.finish(Rule::Field, j);
        self.push(Rule::Field);
        Some(j)
    }

    /// `\s*(KEY)\s*(?:(?:(\?)|(!))\s*)?([,:;=}]|$)`, the lookahead of a
    /// field's begin.
    pub(crate) fn field_ahead(&self, i: usize) -> bool {
        self.field_tail(i).is_some_and(|m| {
            matches!(self.byte(m), b',' | b':' | b';' | b'=' | b'}') || self.eol(m)
        })
    }

    /// `\s*(KEY)\s*(?:(?:(\?)|(!))\s*)?`: where the character after a field
    /// key sits.
    /// The begin accepts `}` there, and the end's `^` branch does not.
    /// A line such as `  c }` therefore opens a field at column 0 and closes
    /// it there at once.
    /// `TextMate` then gives the rest of the line to the field and keeps it
    /// open.
    pub(crate) fn field_tail(&self, i: usize) -> Option<usize> {
        let e = self.field_key_end(self.ws(i))?;
        let mut m = self.ws(e);
        if matches!(self.byte(m), b'?' | b'!') {
            m = self.ws(m + 1);
        }
        Some(m)
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
            match self.byte(k) {
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
            if self.named_function_ahead(k) {
                self.flush(p);
                self.capture(
                    Rule::Top,
                    p,
                    e,
                    &[S::MetaDefinitionProperty, S::EntityNameFunction],
                );
                if let Some(at) = optional {
                    self.capture(Rule::Top, at, at + 1, &[S::KeywordOperatorOptional]);
                }
                if let Some(at) = definite {
                    self.capture(
                        Rule::Top,
                        at,
                        at + 1,
                        &[S::KeywordOperatorDefiniteassignment],
                    );
                }
                return Some(k);
            }
            self.token(
                p,
                e,
                &[S::MetaDefinitionProperty, S::VariableObjectProperty],
            );
            return Some(e);
        }
        match self.byte(p) {
            b'?' => self.token_rule(p, p + 1, S::KeywordOperatorOptional),
            b'!' => self.token_rule(p, p + 1, S::KeywordOperatorDefiniteassignment),
            _ => None,
        }
    }

    pub(crate) fn object_member(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.comment(p) {
            return Some(r);
        }
        if let Some(r) = self.object_literal_method_declaration(p, Rule::ObjectMethod) {
            return Some(r);
        }
        if self.byte(p) == b'[' {
            return self.open_empty(Rule::ObjectKeyBracket, p);
        }
        if matches!(self.byte(p), b'"' | b'\'' | b'`') {
            return self.open_empty(Rule::ObjectKeyString, p);
        }
        if self.number_end(p).is_some() {
            return self.open_empty(Rule::ObjectKeyNumber, p);
        }
        if matches!(self.prev_char(p), Some(']' | '"' | '\'' | '`')) {
            let j = self.ws(p);
            if matches!(self.byte(j), b'(' | b'<') && !self.full() {
                return self.open_empty(Rule::ObjectMethodAfterKey, p);
            }
        }
        if let Some(r) = self.object_member_key(p) {
            return Some(r);
        }
        if self.starts(p, "...") {
            return self.open_token(Rule::ObjectSpread, p, p + 3, &[S::KeywordOperatorSpread]);
        }
        if let Some(e) = self.ident(p) {
            let j = self.ws(e);
            if matches!(self.byte(j), b',' | b'}')
                || self.eol(j)
                || self.starts(j, "//")
                || self.starts(j, "/*")
            {
                self.flush(p);
                self.capture(Rule::ObjectMember, p, e, &[S::VariableOtherReadwrite]);
                self.finish(Rule::ObjectMember, j);
                return Some(j);
            }
        }
        if let Some(r) = self.object_as(p) {
            return Some(r);
        }
        if let Some(e) = self.ident(p) {
            let j = self.ws(e);
            if self.byte(j) == b'=' && !self.full() {
                return self.open_empty(Rule::ObjectInitializer, p);
            }
        }
        if self.byte(p) == b':' {
            return self.object_value_begin(p);
        }
        self.punctuation_comma(p)
            .or_else(|| self.declaration_block(p))
    }

    /// `B(?:\b(async)\s+)?(?:\b([gs]et)\s+)?(?:(\*)\s*)?(?=…\()`.
    ///
    /// Without the `async` prefix.
    pub(crate) fn object_literal_method_declaration(
        &mut self,
        p: usize,
        rule: Rule,
    ) -> Option<usize> {
        if !self.keyword_before(p) || self.full() {
            return None;
        }
        let mut at = p;
        let mut captures = CaptureList::default();
        if self.starts(at, "async") && self.boundary(at) {
            let e = at + "async".len();
            let k = self.ws(e);
            if k > e {
                captures.push((at, e, S::StorageModifierAsync));
                at = k;
            }
        }
        let mut with_accessor = at;
        let mut accessor_capture = None;
        if (self.starts(at, "get") || self.starts(at, "set")) && self.boundary(at) {
            let e = at + "get".len();
            let k = self.ws(e);
            if k > e {
                accessor_capture = Some((at, e));
                with_accessor = k;
            }
        }
        for (start, acc) in [(with_accessor, accessor_capture), (at, None)] {
            let mut j = start;
            let mut star = None;
            if self.byte(j) == b'*' {
                star = Some((j, j + 1));
                j = self.ws(j + 1);
            }
            if self.named_method_ahead(j) {
                self.flush(p);
                for &(a, b, s) in captures.items() {
                    self.capture(rule, a, b, &[s]);
                }
                if let Some((a, b)) = acc {
                    self.capture(rule, a, b, &[S::StorageTypeProperty]);
                }
                if let Some((a, b)) = star {
                    self.capture(rule, a, b, &[S::KeywordGeneratorAsterisk]);
                }
                self.finish(rule, j);
                self.push(rule);
                return Some(j);
            }
            if acc.is_none() {
                break;
            }
        }
        if !captures.items().is_empty() {
            let j = p;
            if self.named_method_ahead(j) {
                self.flush(p);
                self.finish(rule, j);
                self.push(rule);
                return Some(j);
            }
        }
        None
    }

    /// The key forms before a `:`.
    ///
    /// `(?![$_[:alpha:]])(\d+)\s*(?=comments*:)`, where `\d` takes every
    /// digit Shiki's engine does, such as `٤٢`.
    ///
    /// The value is a function.
    fn object_member_key(&mut self, p: usize) -> Option<usize> {
        let mut e = p;
        while let width @ 1.. = classes::digit_width(self.bytes, e) {
            e += width;
        }
        if e > p {
            let j = self.ws(e);
            if self.byte(self.ws_comments(j)) == b':' {
                self.flush(p);
                self.capture(
                    Rule::ObjectMember,
                    p,
                    e,
                    &[S::MetaObjectLiteralKey, S::ConstantNumericDecimal],
                );
                self.capture(Rule::ObjectMember, e, j, &[S::MetaObjectLiteralKey]);
                return Some(j);
            }
        }
        let e = self.ident(p)?;
        let j = self.ws(e);
        let after = self.ws_comments(j);
        if self.byte(after) != b':' {
            return None;
        }
        let v = self.ws_comments(after + 1);
        if self.function_value_ahead(v) {
            self.flush(p);
            self.capture(
                Rule::ObjectMember,
                p,
                e,
                &[S::MetaObjectLiteralKey, S::EntityNameFunction],
            );
            self.capture(Rule::ObjectMember, e, j, &[S::MetaObjectLiteralKey]);
            return Some(j);
        }
        self.flush(p);
        self.capture(Rule::ObjectMember, p, j, &[S::MetaObjectLiteralKey]);
        Some(j)
    }

    /// `B(as)\s+(const)(?=\s*([,}]|$))` and `as`/`satisfies` types.
    fn object_as(&mut self, p: usize) -> Option<usize> {
        if let Some(e) = self.keyword(p, "as") {
            let j = self.ws(e);
            if j > e {
                if let Some(c) = self.keyword(j, "const") {
                    let k = self.ws(c);
                    if matches!(self.byte(k), b',' | b'}') || self.eol(k) {
                        self.flush(p);
                        self.capture(Rule::ObjectMember, p, e, &[S::KeywordControlAs]);
                        self.capture(Rule::ObjectMember, j, c, &[S::StorageModifier]);
                        return Some(c);
                    }
                }
                if !self.full() {
                    self.flush(p);
                    self.capture(Rule::ObjectAs, p, e, &[S::KeywordControlAs]);
                    self.finish(Rule::ObjectAs, j);
                    self.push(Rule::ObjectAs);
                    return Some(j);
                }
            }
        }
        if let Some(e) = self.keyword(p, "satisfies") {
            let j = self.ws(e);
            if j > e && !self.full() {
                self.flush(p);
                self.capture(Rule::ObjectAs, p, e, &[S::KeywordControlSatisfies]);
                self.finish(Rule::ObjectAs, j);
                self.push(Rule::ObjectAs);
                return Some(j);
            }
        }
        None
    }

    fn object_value_begin(&mut self, p: usize) -> Option<usize> {
        self.open_token(
            Rule::ObjectPropertyValue,
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
    ///
    /// `(?<=:)\s*(async)?(?=\s*(<…>)\(\s*(…open…))`.
    ///
    /// `(?<=:)\s*(async)?\s*(\()(?=…open…)`.
    ///
    /// `(?<=:)\s*(async)?(?=\s*<\s*$)`.
    ///
    /// `(?<=>)\s*(\()(?=…open…)`.
    fn object_value_arrow(&mut self, p: usize) -> Option<usize> {
        let prev = self.prev_char(p);
        if prev == Some(':') {
            let j = self.ws(p);
            let both = [j + "async".len(), j];
            let ends = if self.starts(j, "async") {
                &both[..]
            } else {
                &both[1..]
            };
            for &e in ends {
                let k = self.ws(e);
                if self.byte(k) == b'<'
                    && self
                        .arrow_type_parameters(k)
                        .is_some_and(|t| self.byte(t) == b'(')
                    && self.paren_open_ahead(k)
                {
                    self.flush(p);
                    if e > j {
                        self.capture(Rule::Top, j, e, &[S::StorageModifierAsync]);
                    }
                    self.push(Rule::ObjectArrowGeneric);
                    return Some(e);
                }
            }
            for &e in ends {
                let k = self.ws(e);
                if self.byte(k) == b'(' && self.paren_open_after(k + 1) {
                    self.flush(p);
                    if e > j {
                        self.capture(Rule::Top, j, e, &[S::StorageModifierAsync]);
                    }
                    self.capture(Rule::ObjectArrowParen, k, k + 1, &[S::MetaBraceRound]);
                    self.push(Rule::ObjectArrowParen);
                    return Some(k + 1);
                }
            }
            for &e in ends {
                let k = self.ws(e);
                if self.byte(k) == b'<' && self.eol(self.ws(k + 1)) {
                    self.flush(p);
                    if e > j {
                        self.capture(Rule::Top, j, e, &[S::StorageModifierAsync]);
                    }
                    self.push(Rule::ObjectArrowOpen);
                    return Some(e);
                }
            }
        }
        if prev == Some('>') {
            let k = self.ws(p);
            if self.byte(k) == b'(' && self.paren_open_after(k + 1) && !self.full() {
                self.flush(p);
                self.capture(Rule::ObjectArrowParen, k, k + 1, &[S::MetaBraceRound]);
                self.push(Rule::ObjectArrowParen);
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
        if matches!(self.byte(m), b'[' | b'{') && self.eol(self.ws(m + 1)) {
            return true;
        }
        match self.byte(m) {
            b'{' => self
                .balanced(m, b'{', b'}')
                .is_some_and(|e| self.parameter_default_tail(e, b'{')),
            b'[' => self
                .balanced(m, b'[', b']')
                .is_some_and(|e| self.parameter_default_tail(e, b'[')),
            _ => false,
        }
    }

    pub(crate) fn object_arrow_generic_rules(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.type_parameters(p) {
            return Some(r);
        }
        if self.byte(p) == b'(' {
            return self.open_token(Rule::ObjectArrowParen, p, p + 1, &[S::MetaBraceRound]);
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
        if let Some(e) = self.keyword_of(
            p,
            &["override", "public", "protected", "private", "readonly"],
        ) {
            let j = self.ws(e);
            if j > e
                && ["override", "public", "protected", "private", "readonly"]
                    .iter()
                    .any(|w| self.starts(j, w))
            {
                self.token_rule(p, e, S::StorageModifier);
                return Some(j);
            }
        }
        if let Some(r) =
            self.parameter_like(p, super::functions::ParameterStyle::ExpressionParameter)
        {
            return Some(r);
        }
        self.type_annotation(p)
            .or_else(|| self.variable_initializer(p))
            .or_else(|| {
                (self.byte(p) == b',')
                    .then(|| self.token_rule(p, p + 1, S::PunctuationSeparatorParameter))?
            })
            .or_else(|| self.identifiers(p))
            .or_else(|| self.expression_punctuation(p))
    }
}
