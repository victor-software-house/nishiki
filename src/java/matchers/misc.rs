//! The small rules: modifiers, annotations, enum constants, `catch` and
//! `throws` names, and the lookaheads that end a clause.

use crate::java::engine::Lexer;
use crate::rules::Chars;

/// The storage modifiers, in the grammar's order.
const MODIFIERS: &[&[u8]] = &[
    b"public",
    b"private",
    b"protected",
    b"static",
    b"final",
    b"native",
    b"synchronized",
    b"abstract",
    b"threadsafe",
    b"transient",
    b"volatile",
    b"default",
    b"strictfp",
    b"sealed",
    b"non-sealed",
];

/// What a member variable's lookahead starts with.
const MEMBER_STARTS: &[&[u8]] = &[
    b"private",
    b"protected",
    b"public",
    b"native",
    b"synchronized",
    b"abstract",
    b"threadsafe",
    b"transient",
    b"static",
    b"final",
];

impl Lexer<'_> {
    /// `\b(public|...|non-sealed)\b`.
    pub(super) fn storage_modifier(&self, p: usize) -> Option<usize> {
        self.keyword_of(p, MODIFIERS)
    }

    /// The lookahead `(?=X)` where `X` is one character of `set`.
    pub(super) fn ahead_of(&self, p: usize, set: &[u8]) -> Option<usize> {
        self.one_of(p, set).map(|_| p)
    }

    /// The lookahead `(?=a|b|...)` of literal alternatives.
    pub(super) fn ahead_of_any(&self, p: usize, texts: &[&[u8]]) -> Option<usize> {
        texts.iter().any(|text| self.starts(p, text)).then_some(p)
    }

    /// `\b(word)\b` with the word as group 1.
    pub(super) fn keyword_group(&mut self, p: usize, word: &[u8]) -> Option<usize> {
        let end = self.keyword(p, word)?;
        self.cap(1, p, end);
        Some(end)
    }

    /// `(word)\s`: the word and one white space character.
    pub(super) fn keyword_blank(&mut self, p: usize, word: &[u8]) -> Option<usize> {
        let end = self.lit(p, word)?;
        let width = self.space_at(end);
        if width == 0 {
            return None;
        }
        self.cap(1, p, end);
        Some(end + width)
    }

    /// `(?={|\bword\b)`.
    pub(super) fn before_brace_or(&self, p: usize, word: &[u8]) -> Option<usize> {
        (self.at(p) == b'{' && p < self.lim() || self.keyword(p, word).is_some()).then_some(p)
    }

    /// `(?=\s*extends|permits|\{)` and its kin: `first` after blanks, or
    /// `second`, or a `{`.
    pub(super) fn class_clause_end(&self, p: usize, first: &[u8], second: &[u8]) -> Option<usize> {
        (self.starts(self.space_end(p), first)
            || self.starts(p, second)
            || (self.at(p) == b'{' && p < self.lim()))
        .then_some(p)
    }

    /// `((@)\s*([^(\s]+))(\()`: an annotation with arguments.
    pub(super) fn annotation_begin(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'@' || p >= self.lim() {
            return None;
        }
        let name = self.space_end(p + 1);
        let end = self.annotation_name_end(name);
        if end == name || self.at(end) != b'(' || end >= self.lim() {
            return None;
        }
        self.cap(1, p, end);
        self.cap(2, p, p + 1);
        self.cap(3, name, end);
        self.cap(4, end, end + 1);
        Some(end + 1)
    }

    /// The end of the run of `[^(\s]` characters from `i`, kept for the
    /// searches that start inside it.
    fn annotation_name_end(&self, i: usize) -> usize {
        let (start, end) = self.x.memo.annotation.get();
        if start <= i && i < end {
            return end;
        }
        let mut e = i;
        while e < self.lim() && self.s[e] != b'(' && self.space_at(e) == 0 {
            e = self.char_end(e);
        }
        self.spend(e - i);
        if e > i {
            self.x.memo.annotation.set((i, e));
        }
        e
    }

    /// `(\w*)\s*(=)`: a key of an annotation's arguments.
    pub(super) fn annotation_key(&mut self, p: usize) -> Option<usize> {
        let word = self.word_end(p);
        let equals = self.space_end(word);
        if self.at(equals) != b'=' || equals >= self.lim() {
            return None;
        }
        self.cap(1, p, word);
        self.cap(2, equals, equals + 1);
        Some(equals + 1)
    }

    /// `(@)(interface)\s+(\w*)|((@)\s*(\w+))`: an annotation without
    /// arguments, or an annotation type's declaration.
    pub(super) fn annotation_name(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'@' || p >= self.lim() {
            return None;
        }
        if let Some(end) = self.lit(p + 1, b"interface") {
            let name = self.space_end(end);
            if name > end {
                let name_end = self.word_end(name);
                self.cap(1, p, p + 1);
                self.cap(2, p + 1, end);
                self.cap(3, name, name_end);
                return Some(name_end);
            }
        }
        let name = self.space_end(p + 1);
        let end = self.word_end(name);
        if end == name {
            return None;
        }
        self.cap(4, p, end);
        self.cap(5, p, p + 1);
        self.cap(6, name, end);
        Some(end)
    }

    /// `\b(\w+)\b`: the name of an enum constant.
    pub(super) fn enum_constant(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) || self.word_at(p).is_none() {
            return None;
        }
        let end = self.word_end(p);
        self.cap(1, p, end);
        Some(end)
    }

    /// `(,)|(?=[;}])`.
    pub(super) fn enum_constant_end(&mut self, p: usize) -> Option<usize> {
        if p >= self.lim() {
            return None;
        }
        match self.s[p] {
            b',' => {
                self.cap(1, p, p + 1);
                Some(p + 1)
            }
            b';' | b'}' => Some(p),
            _ => None,
        }
    }

    /// `(})|(?=;)`.
    pub(super) fn method_end(&mut self, p: usize) -> Option<usize> {
        if p >= self.lim() {
            return None;
        }
        match self.s[p] {
            b'}' => {
                self.cap(1, p, p + 1);
                Some(p + 1)
            }
            b';' => Some(p),
            _ => None,
        }
    }

    /// `(\w+)`: the name of a record's constructor.
    pub(super) fn constructor_name(&mut self, p: usize) -> Option<usize> {
        let end = self.word_end(p);
        (end > p).then(|| {
            self.cap(1, p, end);
            end
        })
    }

    /// `[$A-Z_a-z][$.0-9A-Z_a-z]*`: a type of `catch` or `throws`.
    pub(super) fn dotted_name(&self, p: usize) -> Option<usize> {
        let first = *self.s.get(p)?;
        if !(first.is_ascii_alphabetic() || first == b'_' || first == b'$') {
            return None;
        }
        Some(self.skip(p + 1, |b| {
            b.is_ascii_alphanumeric() || matches!(b, b'_' | b'$' | b'.')
        }))
    }

    /// The begin `[$A-Z_a-z][$.0-9A-Z_a-z]*` of a `catch` parameter's type,
    /// which keeps its text as group 0.
    pub(super) fn catch_type(&self, p: usize) -> Option<usize> {
        self.dotted_name(p)
    }

    /// `(\|)|(?=\))`.
    pub(super) fn catch_separator(&mut self, p: usize) -> Option<usize> {
        if p >= self.lim() {
            return None;
        }
        match self.s[p] {
            b'|' => {
                self.cap(1, p, p + 1);
                Some(p + 1)
            }
            b')' => Some(p),
            _ => None,
        }
    }

    /// `[$A-Z_a-z][$.0-9A-Z_a-z]*`: a type in a `throws` clause.
    pub(super) fn throws_type(&self, p: usize) -> Option<usize> {
        self.dotted_name(p)
    }

    /// `(?=\b(var)\b\s+[$A-Z_a-z][$\w]*\s*([:;=]))`.
    pub(super) fn local_variable_ahead(&self, p: usize) -> Option<usize> {
        let end = self.keyword(p, b"var")?;
        let name = self.space_end(end);
        if name == end {
            return None;
        }
        let name_end = self.ident_end(name)?;
        self.one_of(self.space_end(name_end), b":;=").map(|_| p)
    }

    /// `(?=private|protected|...|final)`: one of the modifiers' text.
    pub(super) fn member_variable_ahead(&self, p: usize) -> Option<usize> {
        self.ahead_of_any(p, MEMBER_STARTS)
    }
}
