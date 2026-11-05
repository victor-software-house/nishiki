//! Keywords, constants and the grammar's word lists.

use super::{Lexer, is_ident_start, words};
use crate::rules::Chars;

impl Lexer<'_> {
    /// `\b(?<![$.])(break|…|await|(?<=for)\s+own)(?!\s*:)\b`.
    /// A word branch needs a word at `p`, and the `own` branch a blank, so
    /// at most one of them can match.
    pub(super) fn keyword_control(&self, p: usize) -> Option<usize> {
        if let Some(end) = self.unkeyed_word(p, words::KEYWORD_CONTROL) {
            return Some(end);
        }
        if !self.boundary(p)
            || matches!(self.prev(p), Some(b'$' | b'.'))
            || !self.starts_before(p, b"for")
        {
            return None;
        }
        let own = self.space_end(p);
        if own == p {
            return None;
        }
        let end = self.keyword(own, b"own")?;
        self.not_a_key(end).then_some(end)
    }

    /// `\b(?<![$.])(words)(?!\s*:)\b`, with `words` sorted.
    pub(super) fn unkeyed_word(&self, p: usize, words: &[&[u8]]) -> Option<usize> {
        if matches!(self.prev(p), Some(b'$' | b'.')) {
            return None;
        }
        let end = self.listed(p, words)?;
        self.not_a_key(end).then_some(end)
    }

    /// `(?!\s*:)` at `p`.
    fn not_a_key(&self, p: usize) -> bool {
        self.at(self.space_end(p)) != b':'
    }

    /// `(?![\s]*[:=][^=])` at `p`: no `:` or `=` follows that a character
    /// other than `=` comes after.
    /// The line feed is such a character.
    fn not_assigned(&self, p: usize) -> bool {
        let sign = self.space_end(p);
        !(matches!(self.at(sign), b':' | b'=') && self.avail(sign + 1) && self.at(sign + 1) != b'=')
    }

    /// Whether `p` follows neither `.` nor `::`: `(?<!\.|::)`.
    fn not_a_member(&self, p: usize) -> bool {
        self.prev(p) != Some(b'.') && !self.starts_before(p, b"::")
    }

    /// Whether `text` ends at `p`.
    fn starts_before(&self, p: usize, text: &[u8]) -> bool {
        p >= text.len() && self.text(p - text.len(), p) == text
    }

    /// `\b(?<!\.|::)(words)(?!\s*[:=][^=])\b`, with `words` sorted.
    pub(super) fn constant(&self, p: usize, words: &[&[u8]]) -> Option<usize> {
        if !self.not_a_member(p) {
            return None;
        }
        let end = self.listed(p, words)?;
        self.not_assigned(end).then_some(end)
    }

    /// `\b(?<!\.|::)extends(?!\s*[:=])\b`.
    pub(super) fn extends(&self, p: usize) -> Option<usize> {
        if !self.not_a_member(p) {
            return None;
        }
        let end = self.keyword(p, b"extends")?;
        (!matches!(self.at(self.space_end(end)), b':' | b'=')).then_some(end)
    }

    /// `(?<!\.)\b(?<!\$)(word)(?!\s*[:=][^=]|\$)\b`.
    pub(super) fn language_variable(&self, p: usize, word: &[u8]) -> Option<usize> {
        if matches!(self.prev(p), Some(b'$' | b'.')) {
            return None;
        }
        let end = self.keyword(p, word)?;
        (self.not_assigned(end) && self.at(end) != b'$').then_some(end)
    }

    /// `(?<!\.)\b(module|exports|__filename|__dirname|global|process)(?!\s*:)\
    /// b`.
    pub(super) fn support_variable(&self, p: usize) -> Option<usize> {
        if self.prev(p) == Some(b'.') {
            return None;
        }
        let end = self.listed(
            p,
            &[
                b"__dirname",
                b"__filename",
                b"exports",
                b"global",
                b"module",
                b"process",
            ],
        )?;
        self.not_a_key(end).then_some(end)
    }

    /// `\b(?<![$.])(?:(and|or|not)|(is(?:|nt)))(?!\s*:)\b`.
    /// `is` comes before `isnt`, whose `\b` it fails.
    pub(super) fn word_operator(&mut self, p: usize) -> Option<usize> {
        let end = self.unkeyed_word(p, &[b"and", b"is", b"isnt", b"not", b"or"])?;
        let group = if self.at(p) == b'i' { 2 } else { 1 };
        self.cap(group, p, end);
        Some(end)
    }

    /// `((?<=before)(words))\b`, with `words` sorted.
    /// `before` ends in `.`, so `\b` holds before a word at `p`.
    pub(super) fn word_after(&self, p: usize, before: &[u8], words: &[&[u8]]) -> Option<usize> {
        if !self.starts_before(p, before) {
            return None;
        }
        self.listed(p, words)
    }

    /// `\b(shift|…|plugins.refresh|…|back)\b`.
    /// The `.` of `plugins.refresh` takes any character but the line feed.
    /// No other word of the list starts with `plugins`.
    pub(super) fn support_method(&self, p: usize) -> Option<usize> {
        if let Some(end) = self.listed(p, words::SUPPORT_METHODS) {
            return Some(end);
        }
        if !self.boundary(p) {
            return None;
        }
        let dot = self.lit(p, b"plugins")?;
        if dot >= self.lim() {
            return None;
        }
        let end = self.lit(self.char_end(dot), b"refresh")?;
        self.word_at(end).is_none().then_some(end)
    }

    /// `\b(debugger|\\)\b`.
    /// A backslash is not a word character, so `\b` holds on both sides of
    /// it only between two words.
    pub(super) fn debugger(&self, p: usize) -> Option<usize> {
        if self.at(p) == b'\\' {
            return (self.boundary(p) && self.boundary(p + 1)).then_some(p + 1);
        }
        self.keyword(p, b"debugger")
    }

    /// `(new)\s+(?:(class)\s+(\w+(?:\.\w*)*)?|(\w+(?:\.\w*)*))`.
    /// `class` needs a blank after it; without one, it is the start of the
    /// second branch's name.
    /// Giving back the `\s+` leaves a blank where `class` or `\w` must stand.
    pub(super) fn new_instance(&mut self, p: usize) -> Option<usize> {
        let new = self.lit(p, b"new")?;
        let from = self.space_end(new);
        if from == new {
            return None;
        }
        if let Some(class) = self.lit(from, b"class") {
            let name = self.space_end(class);
            if name > class {
                self.cap(1, p, new);
                self.cap(2, from, class);
                let Some(end) = self.dotted_name(name) else {
                    return Some(name);
                };
                self.cap(3, name, end);
                return Some(end);
            }
        }
        let end = self.dotted_name(from)?;
        self.cap(1, p, new);
        self.cap(4, from, end);
        Some(end)
    }

    /// `\w+(?:\.\w*)*`: a `\w` character, then a run of `.` and `\w`.
    fn dotted_name(&self, p: usize) -> Option<usize> {
        let first = self.word_at(p)?;
        Some(self.dotted_rest(p + first))
    }

    /// `[.\w]*`.
    fn dotted_rest(&self, mut i: usize) -> usize {
        loop {
            if self.at(i) == b'.' {
                i += 1;
            } else if let Some(width) = self.word_at(i) {
                i += width;
            } else {
                return i;
            }
        }
    }

    /// `@?[$A-Z_a-z][.\w]*`, or `@?[$.A-Z_a-z][.\w]*` when `first` is
    /// [`ClassName::Dotted`].
    /// Without the `@`, its class would have to take the `@`, which it
    /// cannot.
    fn class_name(&self, p: usize, first: ClassName) -> Option<usize> {
        let from = self.lit(p, b"@").unwrap_or(p);
        let b = self.at(from);
        let starts = is_ident_start(b) || (first == ClassName::Dotted && b == b'.');
        starts.then(|| self.dotted_rest(from + 1))
    }

    /// `(?<=\s|^|[(\[])`.
    fn class_place(&self, p: usize) -> bool {
        self.after_space(p) || matches!(self.prev(p), Some(b'(' | b'['))
    }

    /// `(?<=\s|^|[(\[])(class)\s+(extends)\s+(@?[$.A-Z_a-z][.\w]*)`.
    pub(super) fn class_extends(&mut self, p: usize) -> Option<usize> {
        if !self.class_place(p) {
            return None;
        }
        let class = self.lit(p, b"class")?;
        let (extends, end) = self.extends_clause(class)?;
        self.cap(1, p, class);
        let after = extends + 7;
        self.cap(2, extends, after);
        self.cap(3, self.space_end(after), end);
        Some(end)
    }

    /// `\s+(extends)\s+(@?[$.A-Z_a-z][.\w]*)` at `p`: where `extends` starts,
    /// and where the match ends.
    fn extends_clause(&self, p: usize) -> Option<(usize, usize)> {
        let extends = self.space_end(p);
        if extends == p {
            return None;
        }
        let after = self.lit(extends, b"extends")?;
        let name = self.space_end(after);
        if name == after {
            return None;
        }
        Some((extends, self.class_name(name, ClassName::Dotted)?))
    }

    /// `(?<=\s|^|[(\[])(class)\b\s+(@?[$A-Z_a-z][.\w]*)?(?:\s+(extends)\s+(@?
    /// [$.A-Z_a-z][.\w]*))?`. Without a name, the blanks are all taken, and the
    /// `extends` clause cannot follow: its word would have been the name.
    pub(super) fn class(&mut self, p: usize) -> Option<usize> {
        if !self.class_place(p) {
            return None;
        }
        let class = self.keyword_end(p, b"class")?;
        let name = self.space_end(class);
        if name == class {
            return None;
        }
        self.cap(1, p, class);
        let Some(named) = self.class_name(name, ClassName::Plain) else {
            return Some(name);
        };
        self.cap(2, name, named);
        let Some((extends, end)) = self.extends_clause(named) else {
            return Some(named);
        };
        let after = extends + 7;
        self.cap(3, extends, after);
        self.cap(4, self.space_end(after), end);
        Some(end)
    }

    /// The end of `word` at `p` with `\b` after it, but none asked before.
    fn keyword_end(&self, p: usize, word: &[u8]) -> Option<usize> {
        let end = self.lit(p, word)?;
        self.boundary(end).then_some(end)
    }
}

/// The first character a class name in a `class` statement may start with.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ClassName {
    /// `[$A-Z_a-z]`.
    Plain,
    /// `[$.A-Z_a-z]`, after `extends`.
    Dotted,
}
