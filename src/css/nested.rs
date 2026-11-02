//! The capture groups that the grammar tokenizes with patterns of their own.

use super::engine::Lexer;
use super::grammar::Nested;
use super::scope::S;

impl Lexer<'_> {
    /// Emits the tokens of capture group text `from..to`, which sits under
    /// the stack `base`, and leaves the cursor at `to`.
    pub(super) fn tokenize_capture(&mut self, kind: Nested, from: usize, to: usize, base: u32) {
        match kind {
            Nested::Escapes => self.escapes(from, to, base),
            Nested::CommentBlock => self.comments(from, to, base),
        }
        self.emit(to, base);
    }

    /// The escapes of the grammar's `escapes` rule in `from..to`: a backslash
    /// and up to six hex digits, a backslash that ends the text, and a
    /// backslash and one character.
    fn escapes(&mut self, from: usize, to: usize, base: u32) {
        let mut i = from;
        while i < to {
            if self.s[i] != b'\\' {
                i = self.char_end(i);
                continue;
            }
            let digits = self
                .skip(i + 1, |b| b.is_ascii_hexdigit())
                .min(i + 7)
                .min(to);
            let (end, scope) = if digits > i + 1 {
                (digits, S::ConstCharacterEscapeCodepoint)
            } else if i + 1 >= to {
                (to, S::ConstCharacterEscapeNewline)
            } else {
                (self.char_end(i + 1).min(to), S::ConstCharacterEscape)
            };
            self.emit(i, base);
            let node = self.path(base, &[scope]);
            self.emit(end, node);
            i = end;
        }
    }

    /// The block comments in `from..to`; one that does not close ends with
    /// the text.
    fn comments(&mut self, from: usize, to: usize, base: u32) {
        let mut i = from;
        while i + 1 < to {
            if !(self.s[i] == b'/' && self.s[i + 1] == b'*') {
                i += 1;
                continue;
            }
            let comment = self.path(base, &[S::CmtBlock]);
            let begin = self.path(comment, &[S::PDefinitionCommentBegin]);
            self.emit(i, base);
            self.emit(i + 2, begin);
            let mut j = i + 2;
            while j + 1 < to && !(self.s[j] == b'*' && self.s[j + 1] == b'/') {
                j += 1;
            }
            if j + 1 < to {
                self.emit(j, comment);
                let end = self.path(comment, &[S::PDefinitionCommentEnd]);
                self.emit(j + 2, end);
                i = j + 2;
            } else {
                self.emit(to, comment);
                return;
            }
        }
    }
}
