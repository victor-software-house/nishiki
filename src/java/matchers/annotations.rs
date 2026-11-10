//! The matchers of annotations: their names, arguments and keys.

use crate::java::engine::Lexer;
use crate::rules::Chars;

impl Lexer<'_> {
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
        self.capture(1, p, end);
        self.capture(2, p, p + 1);
        self.capture(3, name, end);
        self.capture(4, end, end + 1);
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
        self.capture(1, p, word);
        self.capture(2, equals, equals + 1);
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
                self.capture(1, p, p + 1);
                self.capture(2, p + 1, end);
                self.capture(3, name, name_end);
                return Some(name_end);
            }
        }
        let name = self.space_end(p + 1);
        let end = self.word_end(name);
        if end == name {
            return None;
        }
        self.capture(4, p, end);
        self.capture(5, p, p + 1);
        self.capture(6, name, end);
        Some(end)
    }
}
