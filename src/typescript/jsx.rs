//! JSX: tags, their attributes and children, and the code embedded in them.
//!
//! The `javascript`, `jsx` and `tsx` grammars read `<` after an operator or
//! a keyword that takes a value as the start of a tag. A tag is lexed as the
//! grammar's contexts: an expression that holds tags, a tag with or without
//! attributes, its attributes, its children and the `{…}` code inside them.

use super::bytes::Bytes;
use super::chars::{Before, is_alnum_char};
use super::kind::Kind;
use super::lexer::Lexer;
use super::scope::S;

/// A tag name, `(?:(ns)(:))?(tag)?`, with the pieces that carry scopes.
#[derive(Clone, Copy)]
struct TagName {
    /// The namespace, its end and the colon after it.
    namespace: Option<(usize, usize)>,
    /// The name, its end and whether it is a component rather than an HTML
    /// tag.
    name: Option<(usize, usize, bool)>,
    /// Where the pieces end.
    end: usize,
}

/// The tag lookahead's pieces: the `<` and the name that follows.
#[derive(Clone, Copy)]
struct TagStart {
    lt: usize,
    name: TagName,
}

/// Whether a tag name must follow its namespace, or may be left out as in
/// `<>` and `<ns:>`.
#[derive(Clone, Copy, Debug)]
enum TagPart {
    /// The name may be absent.
    Optional,
    /// The name must be there.
    Required,
}

impl<const D: u8> Lexer<'_, D> {
    /// The run of `[$_[:alpha:]][-$._[:alnum:]]*` at `i`, which a tag name
    /// does not end in `-` or `.`: its end.
    fn tag_run(&self, i: usize) -> Option<usize> {
        if !self.id_start(i) {
            return None;
        }
        let mut e = i + self.char_len(i);
        while let Some(c) = self.char_at(e)
            && (matches!(c, '-' | '.') || is_alnum_char(c))
        {
            e += c.len_utf8();
        }
        self.spend(e - i);
        (!matches!(self.bytes.get(e - 1), Some(b'-' | b'.'))).then_some(e)
    }

    /// `(?:(ns)(:))?(tag)` at `i`, with the tag optional or required as
    /// `tag` says: the pieces, and where the name ends.
    fn tag_name(&self, i: usize, tag: TagPart) -> Option<TagName> {
        let optional = matches!(tag, TagPart::Optional);
        if let Some(ns_end) = self.tag_run(i)
            && self.byte(ns_end) == b':'
        {
            let with_ns = self.tag_at(ns_end + 1);
            if with_ns.is_some() || optional {
                return Some(TagName {
                    namespace: Some((i, ns_end)),
                    name: with_ns,
                    end: with_ns.map_or(ns_end + 1, |(_, e, _)| e),
                });
            }
        }
        let name = self.tag_at(i);
        if name.is_some() || optional {
            return Some(TagName {
                namespace: None,
                name,
                end: name.map_or(i, |(_, e, _)| e),
            });
        }
        None
    }

    /// `(?:[a-z][0-9a-z]*|([$_[:alpha:]][-$._[:alnum:]]*))(?<![-.])` at `i`:
    /// its start, its end and whether it is a component.
    fn tag_at(&self, i: usize) -> Option<(usize, usize, bool)> {
        let e = self.tag_run(i)?;
        let html = self.bytes[i].is_ascii_lowercase()
            && self.bytes[i..e]
                .iter()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit());
        Some((i, e, !html))
    }

    /// `(?<!\+\+|--)(?<=[(*,:=>?\[{]|&&|\|\||\?|\*/|^await|…|^)` at `p`: where
    /// a tag may start after what precedes it.
    fn jsx_before(&self, p: usize) -> bool {
        const TAG: Before = Before {
            bytes: Bytes::of(b"(*,:=>?[{"),
            operators: &["&&", "||", "*/"],
            words: &["await", "return", "default", "yield"],
            never: &["++", "--"],
        };
        p == 0 || self.operator_before(p.min(self.len), &TAG)
    }

    /// `(<)\s*(?:(ns)(:))?(tag)?\s*(>)` at `i`: a tag without attributes.
    fn tag_without_attributes_at(&self, i: usize) -> Option<(TagStart, usize)> {
        if self.byte(i) != b'<' {
            return None;
        }
        let name = self.tag_name(self.ws(i + 1), TagPart::Optional)?;
        let gt = self.ws(name.end);
        (self.byte(gt) == b'>').then_some((TagStart { lt: i, name }, gt))
    }

    /// `(<)\s*(?:(ns)(:))?(tag)(?=((<\s*)|(\s+))(?!\?)|/?>)` at `i`: the start
    /// of a tag that has attributes, or none.
    fn tag_start_at(&self, i: usize) -> Option<TagStart> {
        if self.byte(i) != b'<' {
            return None;
        }
        let name = self.tag_name(self.ws(i + 1), TagPart::Required)?;
        let e = name.end;
        let angle = self.byte(e) == b'<' && {
            let j = self.ws(e + 1);
            self.byte(j) != b'?' || j > e + 1
        };
        let blank = {
            let j = self.ws(e);
            j > e && (self.byte(j) != b'?' || j > e + 1)
        };
        let close = self.byte(e) == b'>' || (self.byte(e) == b'/' && self.byte(e + 1) == b'>');
        (angle || blank || close).then_some(TagStart { lt: i, name })
    }

    /// Emits the captures of a tag start: the `<`, the namespace and its
    /// colon, and the name.
    fn tag_start_tokens(&mut self, kind: Kind, tag: &TagStart) {
        self.capture(
            kind,
            tag.lt,
            tag.lt + 1,
            &[S::PunctuationDefinitionTagBegin],
        );
        self.tag_name_tokens(kind, &tag.name);
    }

    /// Emits the captures of a tag name.
    fn tag_name_tokens(&mut self, kind: Kind, name: &TagName) {
        if let Some((a, e)) = name.namespace {
            self.capture(kind, a, e, &[S::EntityNameTagNamespace]);
            self.capture(kind, e, e + 1, &[S::PunctuationSeparatorNamespace]);
        }
        if let Some((a, e, component)) = name.name {
            if component {
                self.capture(kind, a, e, &[S::EntityNameTag, S::SupportClassComponent]);
            } else {
                self.capture(kind, a, e, &[S::EntityNameTag]);
            }
        }
    }

    /// The `jsx` rule: a tag in an expression, after what may precede one.
    pub(crate) fn jsx(&mut self, p: usize) -> Option<usize> {
        if !Self::DIALECT.jsx() || self.full() || !self.jsx_before(p) {
            return None;
        }
        let q = self.ws(p);
        if self.tag_without_attributes_at(q).is_some() {
            return self.open_empty(Kind::JsxExprNoAttrs, q);
        }
        if !self.generic_head_at(q) && self.tag_start_at(q).is_some() {
            return self.open_empty(Kind::JsxExprTag, q);
        }
        None
    }

    /// `<\s*[$_[:alpha:]][$_[:alnum:]]*((\s+extends\s+[^=>])|,)`: a type
    /// parameter list, which an arrow function takes, not a tag.
    fn generic_head_at(&self, q: usize) -> bool {
        if self.byte(q) != b'<' {
            return false;
        }
        let Some(e) = self.ident(self.ws(q + 1)) else {
            return false;
        };
        if self.byte(e) == b',' {
            return true;
        }
        let j = self.ws(e);
        if j == e || !self.starts(j, "extends") {
            return false;
        }
        let k = self.ws(j + "extends".len());
        k > j + "extends".len() && (k > j + 8 || !matches!(self.byte(k), b'=' | b'>'))
    }

    /// The `jsx-tag-without-attributes` rule at `p`.
    pub(crate) fn jsx_tag_without_attributes(&mut self, p: usize) -> Option<usize> {
        let (tag, gt) = self.tag_without_attributes_at(p)?;
        if self.full() {
            return None;
        }
        self.flush(p);
        self.tag_start_tokens(Kind::JsxNoAttrs, &tag);
        self.capture(
            Kind::JsxNoAttrs,
            gt,
            gt + 1,
            &[S::PunctuationDefinitionTagEnd],
        );
        self.push(Kind::JsxNoAttrs);
        Some(gt + 1)
    }

    /// The `jsx-tag` rule at `p`: a zero-width start before a tag that has
    /// attributes.
    pub(crate) fn jsx_tag(&mut self, p: usize) -> Option<usize> {
        self.tag_start_at(p)?;
        self.open_empty(Kind::JsxTag, p)
    }

    /// The rules of `jsx-children`.
    pub(crate) fn jsx_children(&mut self, p: usize) -> Option<usize> {
        self.jsx_tag_without_attributes(p)
            .or_else(|| self.jsx_tag(p))
            .or_else(|| self.jsx_evaluated_code(p))
            .or_else(|| self.jsx_entities(p))
    }

    /// `{`: the code of an attribute or a child.
    pub(crate) fn jsx_evaluated_code(&mut self, p: usize) -> Option<usize> {
        if self.byte(p) != b'{' {
            return None;
        }
        self.open_token(
            Kind::JsxEval,
            p,
            p + 1,
            &[S::PunctuationSectionEmbeddedBegin],
        )
    }

    /// `(&)([0-9A-Za-z]+|#[0-9]+|#x\h+)(;)`: a character entity.
    pub(crate) fn jsx_entities(&mut self, p: usize) -> Option<usize> {
        if self.byte(p) != b'&' {
            return None;
        }
        let mut e = p + 1;
        if self.byte(e) == b'#' {
            e += 1;
            let digits = e;
            while self.byte(e).is_ascii_digit() {
                e += 1;
            }
            if e == digits || self.byte(e) != b';' {
                e = digits;
                if self.byte(e) != b'x' {
                    return None;
                }
                e += 1;
                let hex = e;
                while self.byte(e).is_ascii_hexdigit() {
                    e += 1;
                }
                if e == hex || self.byte(e) != b';' {
                    return None;
                }
            }
        } else {
            while self.byte(e).is_ascii_alphanumeric() {
                e += 1;
            }
            if e == p + 1 || self.byte(e) != b';' {
                return None;
            }
        }
        self.flush(p);
        self.capture(
            Kind::Top,
            p,
            p + 1,
            &[S::ConstantCharacterEntity, S::PunctuationDefinitionEntity],
        );
        self.capture(Kind::Top, p + 1, e, &[S::ConstantCharacterEntity]);
        self.capture(
            Kind::Top,
            e,
            e + 1,
            &[S::ConstantCharacterEntity, S::PunctuationDefinitionEntity],
        );
        Some(e + 1)
    }

    /// The rules inside `jsx-tag`: the opening tag and the children.
    pub(crate) fn jsx_tag_rules(&mut self, p: usize) -> Option<usize> {
        if let Some(tag) = self.tag_start_at(p)
            && !self.full()
        {
            self.flush(p);
            self.tag_start_tokens(Kind::JsxTagOpen, &tag);
            self.push(Kind::JsxTagOpen);
            return Some(tag.name.end);
        }
        if self.byte(p) == b'>' && !self.full() {
            return self.open_token(
                Kind::JsxTagBody,
                p,
                p + 1,
                &[S::PunctuationDefinitionTagEnd],
            );
        }
        None
    }

    /// The rules inside the opening tag: comments, type arguments and the
    /// attributes.
    pub(crate) fn jsx_tag_open_rules(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.comment(p) {
            return Some(r);
        }
        if let Some(r) = self.type_arguments(p) {
            return Some(r);
        }
        let j = self.ws(p);
        if j > p && !self.full() {
            self.flush(p);
            self.capture(Kind::JsxAttributes, p, j, &[]);
            self.push(Kind::JsxAttributes);
            return Some(j);
        }
        None
    }

    /// The rules inside the attributes.
    pub(crate) fn jsx_attribute_rules(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.comment(p) {
            return Some(r);
        }
        if let Some(r) = self.jsx_attribute_name(p) {
            return Some(r);
        }
        if self.byte(p) == b'=' && self.assignment_ahead(p + 1) {
            return self.token_rule(p, p + 1, S::KeywordOperatorAssignment);
        }
        if self.byte(p) == b'"' && !self.full() {
            return self.open_token(
                Kind::JsxStrDouble,
                p,
                p + 1,
                &[S::PunctuationDefinitionStringBegin],
            );
        }
        if self.byte(p) == b'\'' && !self.full() {
            return self.open_token(
                Kind::JsxStrSingle,
                p,
                p + 1,
                &[S::PunctuationDefinitionStringBegin],
            );
        }
        if let Some(r) = self.jsx_evaluated_code(p) {
            return Some(r);
        }
        let mut e = p;
        while e < self.len && !self.space_at(e) {
            e += self.char_len(e);
        }
        self.spend(e - p);
        (e > p).then(|| self.token_rule(p, e, S::InvalidIllegalAttribute))?
    }

    /// `=(?=\s*(?:["'{]|/\*|//|\n))`: what may follow an attribute's `=`, a
    /// quote, a brace, a comment or the line's end.
    fn assignment_ahead(&self, i: usize) -> bool {
        let j = self.ws(i);
        j >= self.len
            || matches!(self.byte(j), b'"' | b'\'' | b'{')
            || self.starts(j, "/*")
            || self.starts(j, "//")
    }

    /// `\s*(?:(ns)(:))?([$_[:alpha:]][-$_[:alnum:]]*)(?=[=\s]|/?>|/\*|//)`.
    fn jsx_attribute_name(&mut self, p: usize) -> Option<usize> {
        let start = self.ws(p);
        let name = |lexer: &Self, i: usize, dots: bool| -> Option<usize> {
            if !lexer.id_start(i) {
                return None;
            }
            let mut e = i + lexer.char_len(i);
            while let Some(c) = lexer.char_at(e)
                && (matches!(c, '-' | '$' | '_') || (dots && c == '.') || is_alnum_char(c))
            {
                e += c.len_utf8();
            }
            lexer.spend(e - i);
            Some(e)
        };
        let mut found = None;
        if let Some(ns_end) = name(self, start, true)
            && self.byte(ns_end) == b':'
            && let Some(e) = name(self, ns_end + 1, false)
            && self.attribute_follows(e)
        {
            found = Some((Some((start, ns_end)), ns_end + 1, e));
        }
        if found.is_none()
            && let Some(e) = name(self, start, false)
            && self.attribute_follows(e)
        {
            found = Some((None, start, e));
        }
        let (namespace, name_start, e) = found?;
        self.flush(p);
        if let Some((a, ns_end)) = namespace {
            self.capture(
                Kind::Top,
                a,
                ns_end,
                &[S::EntityOtherAttributeNameNamespace],
            );
            self.capture(
                Kind::Top,
                ns_end,
                ns_end + 1,
                &[S::PunctuationSeparatorNamespace],
            );
        }
        self.capture(Kind::Top, name_start, e, &[S::EntityOtherAttributeName]);
        Some(e)
    }

    /// `(?=[=\s]|/?>|/\*|//)` after an attribute's name.
    fn attribute_follows(&self, e: usize) -> bool {
        let c = self.byte(e);
        c == b'='
            || self.space_at(e)
            || c == b'>'
            || (c == b'/' && matches!(self.byte(e + 1), b'>' | b'*' | b'/'))
    }

    /// The rules inside a JSX string: character entities.
    pub(crate) fn jsx_string_rules(&mut self, p: usize) -> Option<usize> {
        self.jsx_entities(p)
    }

    /// The end of `jsx-tag-without-attributes-in-expression`: the position
    /// where the lookahead for such a tag no longer holds.
    pub(crate) fn jsx_expr_no_attrs_ends(&self, p: usize) -> bool {
        self.tag_without_attributes_at(p).is_none()
    }

    /// The end of `jsx-tag-in-expression`.
    pub(crate) fn jsx_expr_tag_ends(&self, p: usize) -> bool {
        self.tag_start_at(p).is_none()
    }

    /// `(</)\s*(?:(ns)(:))?(tag)?\s*(>)` at `p`: a closing tag, with the end
    /// of its pieces.
    fn closing_tag_at(&self, p: usize) -> Option<(TagName, usize)> {
        if !self.starts(p, "</") {
            return None;
        }
        let name = self.tag_name(self.ws(p + 2), TagPart::Optional)?;
        let gt = self.ws(name.end);
        (self.byte(gt) == b'>').then_some((name, gt))
    }

    /// Ends `jsx-tag-without-attributes` at a closing tag.
    pub(crate) fn jsx_no_attrs_end(&mut self, p: usize) -> Option<usize> {
        let (name, gt) = self.closing_tag_at(p)?;
        Some(self.close_tag(p, &name, gt, &[]))
    }

    /// Ends `jsx-tag` at `/>` or a closing tag.
    pub(crate) fn jsx_tag_end(&mut self, p: usize) -> Option<usize> {
        if self.starts(p, "/>") {
            return Some(self.close(p, p + 2, &[S::PunctuationDefinitionTagEnd]));
        }
        let (name, gt) = self.closing_tag_at(p)?;
        Some(self.close_tag(p, &name, gt, &[]))
    }

    /// Ends the innermost tag context with the captures of a closing tag.
    fn close_tag(&mut self, p: usize, name: &TagName, gt: usize, _leaf: &[S]) -> usize {
        let kind = self.top();
        self.flush(p);
        self.pop();
        self.capture(kind, p, p + 2, &[S::PunctuationDefinitionTagBegin]);
        self.tag_name_tokens(kind, name);
        self.capture(kind, gt, gt + 1, &[S::PunctuationDefinitionTagEnd]);
        gt + 1
    }

    /// `(?=/?>)`.
    pub(crate) fn jsx_open_ends(&self, p: usize) -> bool {
        self.byte(p) == b'>' || (self.byte(p) == b'/' && self.byte(p + 1) == b'>')
    }
}
