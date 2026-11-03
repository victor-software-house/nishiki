//! The capture groups that the grammar tokenizes with patterns of their own.

use super::engine::Lexer;
use super::grammar::Nested;
use super::scope::S;

/// The storage modifiers of the grammar's `storage-modifiers` pattern that a
/// word of `[\w\s]*` can be.
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
];

impl Lexer<'_> {
    /// Emits the tokens of capture group text `from..to`, which sits under
    /// the stack `base`, and leaves the cursor at `to`.
    pub(super) fn tokenize_capture(&mut self, kind: Nested, from: usize, to: usize, base: u32) {
        match kind {
            Nested::TypeChain => self.type_chain(from, to, base),
            Nested::Modifiers => self.modifier_words(from, to, base),
            Nested::Generics => self.generic_names(from, to, base),
        }
        self.emit(to, base);
    }

    /// `[A-Za-z_]\w*` is a type and `\.` a separator, in a chain such as
    /// `java.util.List`.
    fn type_chain(&mut self, from: usize, to: usize, base: u32) {
        let mut i = from;
        while i < to {
            let b = self.s[i];
            if b.is_ascii_alphabetic() || b == b'_' {
                let end = self.word_run_end(i, to);
                self.emit(i, base);
                let node = self.path(base, &[S::StorType]);
                self.emit(end, node);
                i = end;
            } else if b == b'.' {
                self.emit(i, base);
                let node = self.path(base, &[S::PSeparatorPeriod]);
                self.emit(i + 1, node);
                i += 1;
            } else {
                i = self.char_end(i);
            }
        }
    }

    /// The words of `[\w\s]*` that are storage modifiers.
    fn modifier_words(&mut self, from: usize, to: usize, base: u32) {
        let mut i = from;
        while i < to {
            let end = self.word_run_end(i, to);
            if end == i {
                i = self.char_end(i);
                continue;
            }
            if MODIFIERS.contains(&self.text(i, end)) {
                self.emit(i, base);
                let node = self.path(base, &[S::StorModifier]);
                self.emit(end, node);
            }
            i = end;
        }
    }

    /// `<` name `>`: the brackets, and the name's identifier.
    fn generic_names(&mut self, from: usize, to: usize, base: u32) {
        let node = self.path(base, &[S::PBracketAngle]);
        self.emit(from, base);
        self.emit(from + 1, node);
        let body = from + 1;
        let close = to.saturating_sub(1).max(body);
        let word = self.text(body, close);
        let scope = match word {
            b"extends" => Some(S::StorModifierExtends),
            b"super" => Some(S::StorModifierSuper),
            _ => None,
        };
        if let Some(scope) = scope {
            let node = self.path(base, &[scope]);
            self.emit(close, node);
        } else if let Some(offset) = word
            .iter()
            .position(|b| b.is_ascii_alphabetic() || matches!(b, b'$' | b'_'))
        {
            let start = body + offset;
            let end = self.skip(start, |b| {
                b.is_ascii_alphanumeric() || matches!(b, b'$' | b'_')
            });
            self.emit(start, base);
            let node = self.path(base, &[S::StorTypeGeneric]);
            self.emit(end, node);
        }
        self.emit(close, base);
        let node = self.path(base, &[S::PBracketAngle]);
        self.emit(to, node);
    }

    /// The end of the run of `\w` characters from `i`, before `limit`.
    fn word_run_end(&self, mut i: usize, limit: usize) -> usize {
        while i < limit {
            match self.char_at(i) {
                Some((c, end)) if crate::classes::is_word(c) => i = end,
                _ => break,
            }
        }
        i
    }
}
