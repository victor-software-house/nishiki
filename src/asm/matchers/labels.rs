//! The matchers of the grammar's entities: sections, directives, `$` names
//! and labels.
//!
//! A label's characters are `[#$.?@_~[:alnum:]]`, and `[:alpha:]` and
//! `[:alnum:]` read Unicode, as Oniguruma does.

use super::super::engine::{LabelRun, Lexer};
use crate::classes::{is_alnum, is_alpha};
use crate::rules::Chars;

/// What a label's first character may be.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::asm) enum Head {
    /// `[?_[:alpha:]]`, a label.
    Name,
    /// `[$0-9@~]`, an invalid label.
    Invalid,
}

impl Lexer<'_> {
    /// The end of the character at `i` when `ascii` accepts its byte, or,
    /// above ASCII, when `class` accepts it.
    fn class_char(
        &self,
        i: usize,
        ascii: fn(u8) -> bool,
        class: fn(char) -> bool,
    ) -> Option<usize> {
        let b = *self.s.get(i)?;
        if b < 0x80 {
            return ascii(b).then_some(i + 1);
        }
        let (c, end) = self.char_at(i)?;
        class(c).then_some(end)
    }

    /// The end of the run of `[#$.?@_~[:alnum:]]` from `i`.
    /// A place inside the run last measured shares its end.
    fn label_end(&self, mut i: usize) -> usize {
        let kept = self.x.label.get();
        if kept.text == self.lim() && kept.start <= i && i <= kept.end {
            return kept.end;
        }
        let from = i;
        while let Some(end) = self.class_char(
            i,
            |b| {
                b.is_ascii_alphanumeric()
                    || matches!(b, b'#' | b'$' | b'.' | b'?' | b'@' | b'_' | b'~')
            },
            is_alnum,
        ) {
            i = end;
        }
        self.spend(i - from);
        self.x.label.set(LabelRun {
            start: from,
            end: i,
            text: self.lim(),
        });
        i
    }

    /// `((se(?:ction|gment))\s+)?\.((ro)?data|bss|text)`: a section name.
    ///
    /// No `\b` follows the name, so `.textual` starts with `.text`.
    /// Giving back a blank of `\s+` would leave a blank where the `.` must
    /// stand.
    pub(super) fn section(&self, p: usize) -> Option<usize> {
        let keyword = self.lit(p, b"section").or_else(|| self.lit(p, b"segment"));
        let dot = match keyword {
            Some(end) if self.space_at(end) > 0 && self.at(self.space_end(end)) == b'.' => {
                self.space_end(end)
            }
            Some(_) | None => p,
        };
        let name = self.lit(dot, b".")?;
        self.lit(name, b"rodata")
            .or_else(|| self.lit(name, b"data"))
            .or_else(|| self.lit(name, b"bss"))
            .or_else(|| self.lit(name, b"text"))
    }

    /// `^\.?(globa?l|extern|required)\b`: a directive at the start of a line.
    pub(super) fn global(&self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let word = p + usize::from(self.at(p) == b'.');
        let end = self.word_end(word);
        matches!(
            self.text(word, end),
            b"global" | b"globl" | b"extern" | b"required"
        )
        .then_some(end)
    }

    /// `(\$\w+)\b`: a name after `$`.
    pub(super) fn dollar_name(&self, p: usize) -> Option<usize> {
        let name = self.lit(p, b"$")?;
        let end = self.word_end(name);
        (end > name).then_some(end)
    }

    /// `(\.\.@)([?_[:alpha:]][#$.?@_~[:alnum:]]*)(?:(:)?|\b)`: a special
    /// label after `..@`, with an optional colon.
    ///
    /// The optional `(:)?` matches, empty if it must, so the `\b` branch is
    /// never tried.
    pub(super) fn special_label(&mut self, p: usize) -> Option<usize> {
        let name = self.lit(p, b"..@")?;
        let first = self.class_char(
            name,
            |b| b.is_ascii_alphabetic() || matches!(b, b'?' | b'_'),
            is_alpha,
        )?;
        let end = self.label_end(first);
        self.cap(1, p, name);
        self.cap(2, name, end);
        if self.at(end) == b':' {
            self.cap(3, end, end + 1);
            return Some(end + 1);
        }
        Some(end)
    }

    /// `(?:(\.)?|\b)(head[#$.?@_~[:alnum:]]*)(:)`: a label, or an invalid
    /// one, before a colon.
    ///
    /// The optional `(\.)?` matches, empty if it must, so the `\b` branch
    /// adds nothing: the label may start inside a word.
    /// Left out, the `.` would have to start the name, which neither head
    /// takes.
    /// The name's class holds no `:`, so the colon follows the whole run.
    pub(super) fn label(&mut self, p: usize, head: Head) -> Option<usize> {
        let name = p + usize::from(self.at(p) == b'.');
        let first = match head {
            Head::Name => self.class_char(
                name,
                |b| b.is_ascii_alphabetic() || matches!(b, b'?' | b'_'),
                is_alpha,
            ),
            Head::Invalid => self.class_char(
                name,
                |b| b.is_ascii_digit() || matches!(b, b'$' | b'@' | b'~'),
                |_| false,
            ),
        }?;
        let end = self.label_end(first);
        if self.at(end) != b':' {
            return None;
        }
        if name > p {
            self.cap(1, p, name);
        }
        self.cap(2, name, end);
        self.cap(3, end, end + 1);
        Some(end + 1)
    }

    /// `(\.)([0-9]+[#$.?@_~[:alnum:]]*)(?:(:)?|\b)`: a local label of digits
    /// after a dot, with an optional colon.
    ///
    /// The name's class holds the digits, so the name is the whole run.
    pub(super) fn local_number(&mut self, p: usize) -> Option<usize> {
        let name = self.lit(p, b".")?;
        if !self.at(name).is_ascii_digit() {
            return None;
        }
        let end = self.label_end(name);
        self.cap(1, p, name);
        self.cap(2, name, end);
        if self.at(end) == b':' {
            self.cap(3, end, end + 1);
            return Some(end + 1);
        }
        Some(end)
    }
}
