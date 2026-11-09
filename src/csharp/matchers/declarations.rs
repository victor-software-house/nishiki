//! Declarations: namespaces, types, members, accessors and the lookaheads
//! that choose among them.
//!
//! A member's expression begins with the type of the member, then tests what
//! follows the name.
//! Each matcher therefore offers the ways the type can end to the rest of the
//! expression.
//! The rest backtracks as Oniguruma does.

use super::super::engine::Lexer;
use super::types::{Shape, Ty};
use crate::rules::Chars;

/// The first characters of the six declaration keywords that
/// a property's negative lookahead looks for.
const DECLARATION_WORDS: &[&[u8]] = &[
    b"class",
    b"interface",
    b"struct",
    b"union",
    b"enum",
    b"event",
];

impl Lexer<'_> {
    /// `(?=\bword\b)`
    pub(in crate::csharp) fn ahead_of_keyword(&self, p: usize, word: &[u8]) -> Option<usize> {
        self.keyword(p, word).map(|_| p)
    }

    /// `(?=(\brecord\b\s+)?\bword\b)`: a declaration of kind `word`, which a
    /// `record` may precede.
    pub(in crate::csharp) fn ahead_of_record_or(&self, p: usize, word: &[u8]) -> Option<usize> {
        let after_record = self.keyword(p, b"record").and_then(|end| self.blanks1(end));
        if let Some(at) = after_record
            && self.keyword(at, word).is_some()
        {
            return Some(p);
        }
        self.keyword(p, word).map(|_| p)
    }

    /// `\b(namespace)\s+`
    pub(in crate::csharp) fn namespace_start(&mut self, p: usize) -> Option<usize> {
        let end = self.keyword_prefix(p, b"namespace")?;
        self.cap(1, p, end);
        self.blanks1(end)
    }

    /// `\bword` with no `\b` after it, whatever follows.
    fn keyword_prefix(&self, p: usize, word: &[u8]) -> Option<usize> {
        (!self.word_before(p)).then(|| self.lit(p, word)).flatten()
    }

    /// `(\b(record)\b\s+)?\b(class)\b\s+(@?ident)\s*`: the optional
    /// `record`, the keyword, and the name, as groups 2, 3 and 4.
    pub(in crate::csharp) fn class_head(&mut self, p: usize) -> Option<usize> {
        let name_end = self.type_head(p, |lexer, at| lexer.keyword(at, b"class"))?;
        Some(self.blanks(name_end))
    }

    /// `(\b(record)\b\s+)?(struct)\b\s+(@?ident)`: as for a class, with no
    /// `\b` before the keyword.
    pub(in crate::csharp) fn struct_head(&mut self, p: usize) -> Option<usize> {
        self.type_head(p, |lexer, at| {
            lexer
                .lit(at, b"struct")
                .filter(|&end| lexer.word_at(end).is_none())
        })
    }

    /// The optional `record`, the keyword that `keyword` finds, and the name
    /// of a type declaration: the end of the name.
    fn type_head(&mut self, p: usize, keyword: fn(&Self, usize) -> Option<usize>) -> Option<usize> {
        let with_record = self
            .keyword(p, b"record")
            .and_then(|end| self.blanks1(end).map(|after| (end, after)));
        let mut starts = [None, Some((None, p))];
        if let Some((end, after)) = with_record {
            starts[0] = Some((Some(end), after));
        }
        for (record_end, at) in starts.into_iter().flatten() {
            let Some(keyword_end) = keyword(self, at) else {
                continue;
            };
            let name_start = self.blanks1(keyword_end)?;
            let name_end = self.ident(name_start)?;
            if let Some(end) = record_end {
                self.cap(2, p, end);
            }
            self.cap(3, at, keyword_end);
            self.cap(4, name_start, name_end);
            return Some(name_end);
        }
        None
    }

    /// `(word)\b\s+(@?ident)`: a keyword and the name it declares, as
    /// groups 1 and 2.
    pub(in crate::csharp) fn keyword_and_name(&mut self, p: usize, word: &[u8]) -> Option<usize> {
        let end = self
            .lit(p, word)
            .filter(|&end| self.word_at(end).is_none())?;
        let name_start = self.blanks1(end)?;
        let name_end = self.ident(name_start)?;
        self.cap(1, p, end);
        self.cap(2, name_start, name_end);
        Some(name_end)
    }

    /// `(enum)\s+(@?ident)`
    pub(in crate::csharp) fn enum_head(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"enum")?;
        let name_start = self.blanks1(end)?;
        let name_end = self.ident(name_start)?;
        self.cap(1, p, end);
        self.cap(2, name_start, name_end);
        Some(name_end)
    }

    /// `(?=\{|where|;)`
    pub(in crate::csharp) fn base_types_end(&self, p: usize) -> Option<usize> {
        (self.ahead_in(p, b"{;") || self.starts(p, b"where")).then_some(p)
    }

    /// `(?=\{|=>)`
    pub(in crate::csharp) fn body_start_ahead(&self, p: usize) -> Option<usize> {
        (self.ahead_in(p, b"{") || self.starts(p, b"=>")).then_some(p)
    }

    /// `<[^<>]+>` at `p`: the end of the type parameters.
    fn angle_parameters(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'<' || p >= self.lim() {
            return None;
        }
        let end = self.skip(p + 1, |b| b != b'<' && b != b'>');
        (end > p + 1 && self.at(end) == b'>' && end < self.lim()).then_some(end + 1)
    }

    /// `\s*(<([^<>]+)>)?\s*(?=\()` after a name that ends at `p`: records
    /// the angle brackets as capture group `group` and returns the end of
    /// the blanks.
    pub(in crate::csharp) fn type_parameters_then_paren(
        &mut self,
        p: usize,
        group: usize,
    ) -> Option<usize> {
        let w = self.blanks(p);
        if let Some(close) = self.angle_parameters(w) {
            let end = self.blanks(close);
            if self.at(end) == b'(' && end < self.lim() {
                self.cap(group, w, close);
                return Some(end);
            }
        }
        (self.at(w) == b'(' && w < self.lim()).then_some(w)
    }

    /// `\b(delegate)\b\s+T\s+(\g<identifier>)\s*(<([^<>]+)>)?\s*(?=\()`
    pub(in crate::csharp) fn delegate_declaration(&mut self, p: usize) -> Option<usize> {
        let end = self.keyword(p, b"delegate")?;
        let start = self.blanks1(end)?;
        self.type_name(start, Shape::REF_READONLY, &mut |lx, ty| {
            let (name_start, ty_end) = lx.plus_blanks(ty)?;
            let name_end = lx.ident(name_start)?;
            let result = lx.type_parameters_then_paren(name_end, 8)?;
            lx.cap(1, p, end);
            lx.cap(2, ty.start, ty_end);
            lx.cap(7, name_start, name_end);
            Some(result)
        })
    }

    /// The second type of a member's expression,
    /// `(?<interface_name>\g<type_name>\s*\.\s*)?` then the member's name:
    /// tries an interface qualifier first and the bare name after it. `finish`
    /// receives the end of the name and answers for what follows it.
    fn interface_and_name(
        &mut self,
        at: usize,
        interface_group: usize,
        name_group: usize,
        finish: &mut dyn FnMut(&mut Self, usize, usize) -> Option<usize>,
    ) -> Option<usize> {
        let found = self.type_name(at, Shape::REF_READONLY, &mut |lx, ty| {
            let dot = lx.blanks(ty.core);
            if lx.at(dot) != b'.' || dot >= lx.lim() {
                return None;
            }
            let name_start = lx.blanks(dot + 1);
            let name_end = lx.ident(name_start)?;
            let result = finish(lx, name_start, name_end)?;
            lx.cap(interface_group, ty.start, name_start);
            lx.cap(name_group, name_start, name_end);
            Some(result)
        });
        if found.is_some() {
            return found;
        }
        let name_end = self.ident(at)?;
        let result = finish(self, at, name_end)?;
        self.cap(name_group, at, name_end);
        Some(result)
    }

    /// `(?<return_type>(?<type_name>T)\s+)(?<interface_name>\g<type_name>\s*\.\
    /// s*)?(\g<identifier>)\s*(<([^<>]+)>)?\s*(?=\()`
    pub(in crate::csharp) fn method_declaration(&mut self, p: usize) -> Option<usize> {
        self.type_name(p, Shape::REF_READONLY, &mut |lx, ty: Ty| {
            let (after, _) = lx.plus_blanks(ty)?;
            let result = lx.interface_and_name(after, 7, 8, &mut |lx, _, name_end| {
                lx.type_parameters_then_paren(name_end, 9)
            })?;
            lx.cap(1, ty.start, after);
            Some(result)
        })
    }

    /// `(?<indexer_name>this)\s*(?=\[)` after the qualifier: the indexer's
    /// `this`.
    pub(in crate::csharp) fn indexer_declaration(&mut self, p: usize) -> Option<usize> {
        self.type_name(p, Shape::REF_READONLY, &mut |lx, ty: Ty| {
            let (after, _) = lx.plus_blanks(ty)?;
            let found = lx.type_name(after, Shape::REF_READONLY, &mut |lx, qualifier| {
                let dot = lx.blanks(qualifier.core);
                if lx.at(dot) != b'.' || dot >= lx.lim() {
                    return None;
                }
                let this_start = lx.blanks(dot + 1);
                let this_end = lx.lit(this_start, b"this")?;
                let end = lx.blanks(this_end);
                if lx.at(end) != b'[' || end >= lx.lim() {
                    return None;
                }
                lx.cap(1, ty.start, after);
                lx.cap(7, qualifier.start, this_start);
                lx.cap(8, this_start, this_end);
                Some(end)
            });
            if found.is_some() {
                return found;
            }
            let this_end = lx.lit(after, b"this")?;
            let end = lx.blanks(this_end);
            if lx.at(end) != b'[' || end >= lx.lim() {
                return None;
            }
            lx.cap(1, ty.start, after);
            lx.cap(8, after, this_end);
            Some(end)
        })
    }

    /// `(?=\{|=>|//|/\*|$)` at `p`: what a property's name or an accessor's
    /// keyword is followed by.
    fn member_body_ahead(&self, p: usize) -> bool {
        self.ahead_in(p, b"{")
            || self.starts(p, b"=>")
            || self.starts(p, b"//")
            || self.starts(p, b"/*")
            || self.at_eol(p)
    }

    /// Whether a word of `[[:word:]\s]*` is one of the declaration keywords
    /// at a word boundary: the grammar's `(?![[:word:]\s]*\b(?:class|...)\b)`
    /// fails the property.
    fn declaration_word_ahead(&self, p: usize) -> bool {
        let mut i = p;
        loop {
            i = self.blanks(i);
            let start = i;
            while self.word_at(i).is_some() {
                i = self.cp(i).map_or(i + 1, |(_, end)| end);
            }
            if i == start {
                return false;
            }
            if !(start == p && self.word_before(p))
                && DECLARATION_WORDS.contains(&self.text(start, i))
            {
                return true;
            }
            self.spend(i - start);
        }
    }

    /// The property's return type, optional interface qualifier and name,
    /// then `\s*(?=\{|=>|//|/\*|$)`.
    pub(in crate::csharp) fn property_declaration(&mut self, p: usize) -> Option<usize> {
        if self.declaration_word_ahead(p) {
            return None;
        }
        self.type_name(p, Shape::REF_READONLY, &mut |lx, ty: Ty| {
            let (after, _) = lx.plus_blanks(ty)?;
            let result = lx.interface_and_name(after, 7, 8, &mut |lx, _, name_end| {
                let end = lx.blanks(name_end);
                lx.member_body_ahead(end).then_some(end)
            })?;
            lx.cap(1, ty.start, after);
            Some(result)
        })
    }

    /// `\b(event)\b\s*(?<return_type>T\s+)(?<interface_name>...)?(\
    /// g<identifier>)\s*(?=[,;={]|//|/\*|$)`
    pub(in crate::csharp) fn event_declaration(&mut self, p: usize) -> Option<usize> {
        let end = self.keyword(p, b"event")?;
        let start = self.blanks(end);
        self.type_name(start, Shape::STD, &mut |lx, ty: Ty| {
            let (after, _) = lx.plus_blanks(ty)?;
            let found = lx.type_name(after, Shape::STD, &mut |lx, qualifier| {
                let dot = lx.blanks(qualifier.core);
                if lx.at(dot) != b'.' || dot >= lx.lim() {
                    return None;
                }
                let name_start = lx.blanks(dot + 1);
                let name_end = lx.ident(name_start)?;
                let close = lx.blanks(name_end);
                lx.event_end_ahead(close).then_some(())?;
                lx.cap(1, p, end);
                lx.cap(2, ty.start, after);
                lx.cap(8, qualifier.start, name_start);
                lx.cap(9, name_start, name_end);
                Some(close)
            });
            if found.is_some() {
                return found;
            }
            let name_end = lx.ident(after)?;
            let close = lx.blanks(name_end);
            lx.event_end_ahead(close).then_some(())?;
            lx.cap(1, p, end);
            lx.cap(2, ty.start, after);
            lx.cap(9, after, name_end);
            Some(close)
        })
    }

    /// `(?=[,;={]|//|/\*|$)`
    fn event_end_ahead(&self, p: usize) -> bool {
        self.ahead_in(p, b",;={")
            || self.starts(p, b"//")
            || self.starts(p, b"/*")
            || self.at_eol(p)
    }

    /// `(@?ident)\s*(?=\(|$)`
    pub(in crate::csharp) fn constructor_declaration(&mut self, p: usize) -> Option<usize> {
        let name_end = self.ident(p)?;
        let end = self.blanks(name_end);
        if !(self.ahead_in(end, b"(") || self.at_eol(end)) {
            return None;
        }
        self.cap(1, p, name_end);
        Some(end)
    }

    /// `(~)(@?ident)\s*(?=\()`
    pub(in crate::csharp) fn destructor_declaration(&mut self, p: usize) -> Option<usize> {
        let tilde = self.lit(p, b"~")?;
        let name_end = self.ident(tilde)?;
        let end = self.blanks(name_end);
        if !self.ahead_in(end, b"(") {
            return None;
        }
        self.cap(1, p, tilde);
        self.cap(2, tilde, name_end);
        Some(end)
    }

    /// `\b(base|this)\b\s*(?=\()`, with the word fixed by the pattern.
    pub(in crate::csharp) fn constructor_initializer(
        &mut self,
        p: usize,
        word: &[u8],
    ) -> Option<usize> {
        let end = self.keyword(p, word)?;
        let after = self.blanks(end);
        if !self.ahead_in(after, b"(") {
            return None;
        }
        self.cap(1, p, end);
        Some(after)
    }

    /// `\b(fixed)\b\s+T\s+(\g<identifier>)\s*(?=\[)`
    pub(in crate::csharp) fn fixed_buffer_declaration(&mut self, p: usize) -> Option<usize> {
        let end = self.keyword(p, b"fixed")?;
        let start = self.blanks1(end)?;
        self.type_name(start, Shape::NAME_ONLY, &mut |lx, ty: Ty| {
            let (name_start, ty_end) = lx.plus_blanks(ty)?;
            let name_end = lx.ident(name_start)?;
            let close = lx.blanks(name_end);
            if !lx.ahead_in(close, b"[") {
                return None;
            }
            lx.cap(1, p, end);
            lx.cap(2, ty.start, ty_end);
            lx.cap(6, name_start, name_end);
            Some(close)
        })
    }

    /// `T\s+(\g<identifier>)\s*(?!=[=>])(?=[,;=]|$)`
    pub(in crate::csharp) fn field_declaration(&mut self, p: usize) -> Option<usize> {
        self.type_name(p, Shape::STD, &mut |lx, ty: Ty| {
            let (name_start, ty_end) = lx.plus_blanks(ty)?;
            let name_end = lx.ident(name_start)?;
            let end = lx.blanks(name_end);
            if lx.at(end) == b'=' && lx.byte_in(end + 1, b"=>") {
                return None;
            }
            if !(lx.ahead_in(end, b",;=") || lx.at_eol(end)) {
                return None;
            }
            lx.cap(1, ty.start, ty_end);
            lx.cap(6, name_start, name_end);
            Some(end)
        })
    }

    /// `\s*(?=[;{]|=>|//|/\*|$)` after an accessor's keyword: the end of the
    /// blanks.
    fn accessor_end(&self, p: usize) -> Option<usize> {
        let end = self.blanks(p);
        (self.ahead_in(end, b";{")
            || self.starts(end, b"=>")
            || self.starts(end, b"//")
            || self.starts(end, b"/*")
            || self.at_eol(end))
        .then_some(end)
    }

    /// `\b(add|remove)\b\s*(?=[;{]|=>|//|/\*|$)` and the `set` kin: an
    /// accessor's keyword, `word`, as capture group 1.
    pub(in crate::csharp) fn accessor(&mut self, p: usize, word: &[u8]) -> Option<usize> {
        let end = self.keyword(p, word)?;
        let result = self.accessor_end(end)?;
        self.cap(1, p, end);
        Some(result)
    }

    /// `(?:\b(readonly)\s+)?\b(get)\b\s*(?=[;{]|=>|//|/\*|$)`: a getter, with
    /// an optional leading `readonly` as group 1 and `get` as group 2.
    pub(in crate::csharp) fn getter(&mut self, p: usize) -> Option<usize> {
        let readonly = self
            .keyword(p, b"readonly")
            .and_then(|end| self.blanks1(end).map(|after| (end, after)));
        if let Some((readonly_end, after)) = readonly
            && let Some(end) = self.keyword(after, b"get")
            && let Some(result) = self.accessor_end(end)
        {
            self.cap(1, p, readonly_end);
            self.cap(2, after, end);
            return Some(result);
        }
        let end = self.keyword(p, b"get")?;
        let result = self.accessor_end(end)?;
        self.cap(2, p, end);
        Some(result)
    }

    /// `(where)\s+(@?ident)\s*(:)`
    pub(in crate::csharp) fn generic_constraint(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"where")?;
        let name_start = self.blanks1(end)?;
        let name_end = self.ident(name_start)?;
        let colon = self.blanks(name_end);
        if self.at(colon) != b':' || colon >= self.lim() {
            return None;
        }
        self.cap(1, p, end);
        self.cap(2, name_start, name_end);
        self.cap(3, colon, colon + 1);
        Some(colon + 1)
    }

    /// `(?=\{|where|;|=>)`
    pub(in crate::csharp) fn generic_constraints_end(&self, p: usize) -> Option<usize> {
        (self.ahead_in(p, b"{;") || self.starts(p, b"where") || self.starts(p, b"=>")).then_some(p)
    }

    /// `(new)\s*(\()\s*(\))`
    pub(in crate::csharp) fn new_constraint(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"new")?;
        let open = self.blanks(end);
        if self.at(open) != b'(' || open >= self.lim() {
            return None;
        }
        let close = self.blanks(open + 1);
        if self.at(close) != b')' || close >= self.lim() {
            return None;
        }
        self.cap(1, p, end);
        self.cap(2, open, open + 1);
        self.cap(3, close, close + 1);
        Some(close + 1)
    }

    /// `(?:(@?ident)\s*(\.))*(@?ident)\s*(<(?<type_args>...)*>\s*)?(?=\()`
    pub(in crate::csharp) fn base_class_constructor_call(&mut self, p: usize) -> Option<usize> {
        self.call_head(p)
    }

    /// `T\s*\b(operator)\b\s*([-!%\&*+/<=>^|~]+|true|false)\s*(?=\()`
    pub(in crate::csharp) fn operator_declaration(&mut self, p: usize) -> Option<usize> {
        self.type_name(p, Shape::REF_READONLY, &mut |lx, ty: Ty| {
            let keyword_start = lx.blanks(ty.end);
            let keyword_end = lx.keyword(keyword_start, b"operator")?;
            let op_start = lx.blanks(keyword_end);
            let class_end = lx.skip(op_start, |b| {
                matches!(
                    b,
                    b'-' | b'!'
                        | b'%'
                        | b'&'
                        | b'*'
                        | b'+'
                        | b'/'
                        | b'<'
                        | b'='
                        | b'>'
                        | b'^'
                        | b'|'
                        | b'~'
                )
            });
            let op_end = if class_end > op_start {
                class_end
            } else {
                lx.lit(op_start, b"true")
                    .or_else(|| lx.lit(op_start, b"false"))?
            };
            let end = lx.blanks(op_end);
            if !lx.ahead_in(end, b"(") {
                return None;
            }
            lx.cap(1, ty.start, ty.end);
            lx.cap(6, keyword_start, keyword_end);
            lx.cap(7, op_start, op_end);
            Some(end)
        })
    }

    /// `\b(explicit|implicit)\s*\b(operator)\s*T\s*(?=\()`
    pub(in crate::csharp) fn conversion_operator_declaration(&mut self, p: usize) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        let word_end = self
            .lit(p, b"explicit")
            .or_else(|| self.lit(p, b"implicit"))?;
        let keyword_start = self.blanks(word_end);
        if !self.boundary(keyword_start) {
            return None;
        }
        let keyword_end = self.lit(keyword_start, b"operator")?;
        let start = self.blanks(keyword_end);
        self.type_name(start, Shape::REF_READONLY, &mut |lx, ty: Ty| {
            let end = lx.blanks(ty.end);
            if !lx.ahead_in(end, b"(") {
                return None;
            }
            lx.cap(1, p, word_end);
            lx.cap(2, keyword_start, keyword_end);
            lx.cap(3, ty.start, ty.end);
            Some(end)
        })
    }
}
