//! The capture groups that the grammar tokenizes with patterns of their own.

use super::engine::CssLike;
use super::grammar::Nested;
use super::scope::S;
use crate::rules::Lexer;
use crate::scope::scopes;

impl<G: CssLike> Lexer<'_, G> {
    /// Emits the tokens of capture group text `from..to`, and leaves the
    /// cursor at `to`.
    /// The text sits under the stack `base`.
    pub(crate) fn css_capture(&mut self, kind: Nested, from: usize, to: usize, base: u32) {
        match kind {
            Nested::Escapes => self.escapes(from, to, base),
            Nested::CommentBlock => self.comments(from, to, base),
        }
        self.emit(to, base);
    }

    /// The escapes of the grammar's `escapes` rule in `from..to`.
    /// They are a backslash and up to six hex digits, a backslash that ends
    /// the text, and a backslash and one character.
    fn escapes(&mut self, from: usize, to: usize, base: u32) {
        let mut i = from;
        while i < to {
            if self.s[i] != b'\\' {
                i = self.char_end(i);
                continue;
            }
            let digits = self.hex_escape_end(i).map_or(i + 1, |end| end.min(to));
            let (end, scopes) = if digits > i + 1 {
                (digits, scopes![S::ConstCharacterEscapeCodepoint])
            } else if i + 1 >= to {
                (to, scopes![S::ConstCharacterEscapeNewline])
            } else {
                (
                    self.char_end(i + 1).min(to),
                    scopes![S::ConstCharacterEscape],
                )
            };
            self.emit(i, base);
            let node = G::css_path(self, base, scopes);
            self.emit(end, node);
            i = end;
        }
    }

    /// The block comments in `from..to`.
    /// One that does not close ends with the text.
    fn comments(&mut self, from: usize, to: usize, base: u32) {
        let mut i = from;
        while i + 1 < to {
            if !(self.s[i] == b'/' && self.s[i + 1] == b'*') {
                i += 1;
                continue;
            }
            let comment = G::css_path(self, base, scopes![S::CmtBlock]);
            let begin = G::css_path(self, comment, scopes![S::PDefinitionCommentBegin]);
            self.emit(i, base);
            self.emit(i + 2, begin);
            let mut j = i + 2;
            while j + 1 < to && !(self.s[j] == b'*' && self.s[j + 1] == b'/') {
                j += 1;
            }
            if j + 1 < to {
                self.emit(j, comment);
                let end = G::css_path(self, comment, scopes![S::PDefinitionCommentEnd]);
                self.emit(j + 2, end);
                i = j + 2;
            } else {
                self.emit(to, comment);
                return;
            }
        }
    }
}
