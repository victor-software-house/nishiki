//! Types: generics, primitive types and the dotted names of classes.

use crate::java::engine::Lexer;

impl Lexer<'_> {
    /// The end of the ASCII identifier `[$A-Z_a-z][$0-9A-Z_a-z]*` at `i`.
    pub(super) fn ascii_ident_end(&self, i: usize) -> Option<usize> {
        let b = *self.s.get(i)?;
        if !(b.is_ascii_alphabetic() || b == b'_' || b == b'$') {
            return None;
        }
        let (start, end) = self.ascii_run.get();
        if start <= i && i < end {
            return Some(end);
        }
        let mut e = i + 1;
        while self
            .s
            .get(e)
            .is_some_and(|&b| b.is_ascii_alphanumeric() || b == b'_' || b == b'$')
        {
            e += 1;
        }
        self.ascii_run.set((i, e));
        Some(e)
    }

    /// `(?<!\.)([$A-Z_a-z][$0-9A-Z_a-z]*)(?=\s*<)`: a name before `<`.
    pub(super) fn generic_name(&mut self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'.') {
            return None;
        }
        let end = self.ascii_ident_end(p)?;
        let angle = self.space_end(end);
        if self.at(angle) != b'<' {
            return None;
        }
        self.cap(1, p, end);
        Some(end)
    }

    /// `\b(void|boolean|byte|char|short|int|float|long|double)\b`, and with
    /// `\s*(?=\[)` after it the begin of a primitive array.
    pub(super) fn primitive(&mut self, p: usize, array: bool) -> Option<usize> {
        let end = self.keyword_of(
            p,
            &[
                b"void", b"boolean", b"byte", b"char", b"short", b"int", b"float", b"long",
                b"double",
            ],
        )?;
        if !array {
            return Some(end);
        }
        let bracket = self.space_end(end);
        if self.at(bracket) != b'[' {
            return None;
        }
        self.cap(1, p, end);
        Some(bracket)
    }

    /// `(?!\s*\[)`.
    pub(super) fn not_bracket(&self, p: usize) -> Option<usize> {
        (self.at(self.space_end(p)) != b'[').then_some(p)
    }

    /// The dotted names `(?:[A-Z_a-z]\w*\s*\.\s*)*[A-Z_]\w*` from `p`:
    /// `check` is asked about each place the final name can start, with where
    /// it ends, and the last one it accepts wins, as the longest run of
    /// names does in the grammar. It returns the match's start of the final
    /// name, its end and what `check` made of it.
    fn dotted_type(
        &self,
        p: usize,
        check: impl Fn(usize, usize) -> Option<usize>,
    ) -> Option<(usize, usize, usize)> {
        if !self.boundary(p) {
            return None;
        }
        let mut best = None;
        let mut i = p;
        loop {
            let b = self.at(i);
            if i >= self.lim() {
                break;
            }
            if b.is_ascii_uppercase() || b == b'_' {
                let end = self.word_end(i);
                if let Some(result) = check(i, end) {
                    best = Some((i, end, result));
                }
            }
            if !(b.is_ascii_alphabetic() || b == b'_') {
                break;
            }
            let word = self.word_end(i);
            let dot = self.space_end(word);
            if self.at(dot) != b'.' {
                break;
            }
            i = self.space_end(dot + 1);
        }
        best
    }

    /// `\b((?:[A-Z_a-z]\w*\s*\.\s*)*)([A-Z_]\w*)\s*(?=\[)`: the begin of an
    /// array of objects.
    pub(super) fn object_array(&mut self, p: usize) -> Option<usize> {
        let (last, end, after) = self.dotted_type(p, |_, end| {
            let bracket = self.space_end(end);
            (self.at(bracket) == b'[').then_some(bracket)
        })?;
        self.cap(1, p, last);
        self.cap(2, last, end);
        Some(after)
    }

    /// `\b((?:[A-Z_a-z]\w*\s*\.\s*)*[A-Z_]\w*)\s*(?=<)`.
    pub(super) fn object_generic(&mut self, p: usize) -> Option<usize> {
        let (_, end, after) = self.dotted_type(p, |_, end| {
            let angle = self.space_end(end);
            (self.at(angle) == b'<').then_some(angle)
        })?;
        self.cap(1, p, end);
        Some(after)
    }

    /// `\b(...[A-Z_]\w*)\b((?=\s*[\n$A-Z_a-z])|(?=\s*\.\.\.))`: an object
    /// type that a name or a `...` follows.
    pub(super) fn object_type(&mut self, p: usize) -> Option<usize> {
        let (_, end, _) = self.dotted_type(p, |_, end| {
            let next = self.space_end(end);
            let name_follows = next > self.lim()
                || (next < self.lim()
                    && (self.s[next].is_ascii_alphabetic() || matches!(self.s[next], b'$' | b'_')));
            (name_follows || self.starts(next, b"...")).then_some(end)
        })?;
        self.cap(1, p, end);
        Some(end)
    }

    /// `\b(?:[A-Z]\w*\s*(\.)\s*)*[A-Z]\w*\b`: an inherited class, with the
    /// last dot as group 1.
    pub(super) fn inherited_class(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let mut best = None;
        let mut dot = None;
        let mut i = p;
        while self.at(i).is_ascii_uppercase() && i < self.lim() {
            let word = self.word_end(i);
            best = Some((word, dot));
            let next = self.space_end(word);
            if self.at(next) != b'.' {
                break;
            }
            dot = Some(next);
            i = self.space_end(next + 1);
        }
        let (end, dot) = best?;
        if let Some(dot) = dot {
            self.cap(1, dot, dot + 1);
        }
        Some(end)
    }
}
