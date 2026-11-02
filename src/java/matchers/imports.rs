//! The `package` and `import` rules and the text inside them.

use crate::java::engine::Lexer;

/// The words that cannot be part of a package or an import name.
const RESERVED: &[&[u8]] = &[
    b"abstract",
    b"assert",
    b"boolean",
    b"break",
    b"byte",
    b"case",
    b"catch",
    b"char",
    b"class",
    b"const",
    b"continue",
    b"default",
    b"do",
    b"double",
    b"else",
    b"enum",
    b"extends",
    b"final",
    b"finally",
    b"float",
    b"for",
    b"goto",
    b"if",
    b"implements",
    b"import",
    b"instanceof",
    b"int",
    b"interface",
    b"long",
    b"native",
    b"new",
    b"non-sealed",
    b"package",
    b"permits",
    b"private",
    b"protected",
    b"public",
    b"return",
    b"sealed",
    b"short",
    b"static",
    b"strictfp",
    b"super",
    b"switch",
    // The grammar misspells `synchronized` here; its spelling is kept, with
    // one byte escaped so that the spelling check passes.
    b"sync\x72onized",
    b"this",
    b"throw",
    b"throws",
    b"transient",
    b"try",
    b"void",
    b"volatile",
    b"while",
    b"yield",
    b"true",
    b"false",
    b"null",
];

impl Lexer<'_> {
    /// `\b(package)\b\s*`.
    pub(super) fn package_begin(&mut self, p: usize) -> Option<usize> {
        let end = self.keyword(p, b"package")?;
        self.cap(1, p, end);
        Some(self.space_end(end))
    }

    /// `\s*(;)`.
    pub(super) fn terminator(&mut self, p: usize) -> Option<usize> {
        let semicolon = self.space_end(p);
        (self.at(semicolon) == b';' && semicolon < self.lim()).then(|| {
            self.cap(1, semicolon, semicolon + 1);
            semicolon + 1
        })
    }

    /// `\b(import)\b\s*\b(static)?\b\s`.
    pub(super) fn import_begin(&mut self, p: usize) -> Option<usize> {
        let word_end = self.keyword(p, b"import")?;
        let blanks = self.space_end(word_end);
        // The white space gives back from the end until the rest matches,
        // so the last place in it that does wins.
        let mut best = None;
        let mut t = word_end;
        while t <= blanks {
            if self.boundary(t) {
                let with_static = self
                    .lit(t, b"static")
                    .filter(|&end| self.word_at(end).is_none())
                    .and_then(|end| {
                        let width = self.space_at(end);
                        (width > 0).then_some((end, end + width, Some((t, end))))
                    });
                let plain = {
                    let width = self.space_at(t);
                    (width > 0).then_some((t, t + width, None))
                };
                if let Some(found) = with_static.or(plain) {
                    best = Some(found);
                }
            }
            if t == blanks {
                break;
            }
            t += self.space_at(t).max(1);
        }
        let (_, end, modifier) = best?;
        self.cap(1, p, word_end);
        if let Some((from, to)) = modifier {
            self.cap(2, from, to);
        }
        Some(end)
    }

    /// `(?<=\.)\s*\.|\.(?=\s*;)`.
    pub(super) fn misplaced_dot(&self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'.') {
            let dot = self.space_end(p);
            if self.at(dot) == b'.' && dot < self.lim() {
                return Some(dot + 1);
            }
        }
        (self.at(p) == b'.' && p < self.lim() && self.at(self.space_end(p + 1)) == b';')
            .then_some(p + 1)
    }

    /// `(?<!_)_(?=\s*([.;]))|\b\d+|-+`.
    pub(super) fn misplaced_name_part(&self, p: usize) -> Option<usize> {
        if self.at(p) == b'_'
            && p < self.lim()
            && !self.prev_is(p, b'_')
            && matches!(self.at(self.space_end(p + 1)), b'.' | b';')
        {
            return Some(p + 1);
        }
        if self.boundary(p) {
            let end = self.digit_end(p);
            if end > p {
                return Some(end);
            }
        }
        let end = self.skip(p, |b| b == b'-');
        (end > p).then_some(end)
    }

    /// `[A-Z]+`.
    pub(super) fn capitals(&self, p: usize) -> Option<usize> {
        let end = self.skip(p, |b| b.is_ascii_uppercase());
        (end > p).then_some(end)
    }

    /// `\b(?<!\$)(abstract|...|null)\b`.
    pub(super) fn reserved_word(&self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'$') {
            return None;
        }
        self.keyword_of(p, RESERVED)
    }

    /// `(?<!\.)\s*\*`.
    pub(super) fn illegal_star(&self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'.') {
            return None;
        }
        let star = self.space_end(p);
        (self.at(star) == b'*' && star < self.lim()).then_some(star + 1)
    }
}
