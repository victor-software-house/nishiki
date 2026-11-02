//! Annotations and their use-site targets.

use super::super::engine::Lexer;
use crate::rules::Chars;

impl Lexer<'_> {
    /// The end of the run of `[.\w]` characters from `i`.
    fn dotted_end(&self, mut i: usize) -> usize {
        let from = i;
        loop {
            if self.s.get(i) == Some(&b'.') {
                i += 1;
            } else if let Some(width) = self.word_at(i) {
                i += width;
            } else {
                break;
            }
        }
        self.spend(i - from);
        i
    }

    /// The start of the character that ends at `i`.
    fn char_start_before(&self, i: usize) -> usize {
        let mut start = i - 1;
        while start > 0 && self.s[start] & 0xc0 == 0x80 {
            start -= 1;
        }
        start
    }

    /// The end of `[.\w]+\b` from `from`, where the run took its first
    /// character: the run gives back characters until `\b` holds, and `after`
    /// rejects the end it is not allowed to have.
    fn dotted_name(&self, from: usize, mut after: impl FnMut(usize) -> bool) -> Option<usize> {
        let mut end = self.dotted_end(from);
        while end > from {
            if self.boundary(end) && after(end) {
                return Some(end);
            }
            end = self.char_start_before(end);
        }
        None
    }

    /// `(?<!\w)@[.\w]+\b(?!:)`: the run gives back characters while a `:`
    /// follows its end, so `@file:` fails and `@a.b:` keeps `@a.`.
    pub(super) fn annotation_simple(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'@' || self.word_before(p) {
            return None;
        }
        self.dotted_name(p + 1, |end| self.at(end) != b':')
    }

    /// `\b[.\w]+\b`.
    pub(super) fn unescaped_annotation(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        self.dotted_name(p, |_| true)
    }

    /// `(?<!\w)(@\w+):\s*\[` for the list form and `(?<!\w)(@\w+):\s*(?!\[)`
    /// for the other: the target, the colon and the blanks. The second gives
    /// back one blank when a `[` follows them, and fails when there is none.
    pub(super) fn annotation_site(&mut self, p: usize, form: SiteForm) -> Option<usize> {
        if self.at(p) != b'@' || self.word_before(p) {
            return None;
        }
        let name = self.word_end(p + 1);
        if name == p + 1 || self.at(name) != b':' {
            return None;
        }
        let blanks = self.space_end(name + 1);
        let bracket = self.at(blanks) == b'[';
        let end = match form {
            SiteForm::List => {
                if !bracket {
                    return None;
                }
                blanks + 1
            }
            SiteForm::Single if bracket => {
                if blanks == name + 1 {
                    return None;
                }
                self.char_start_before(blanks)
            }
            SiteForm::Single => blanks,
        };
        self.cap(1, p, name);
        Some(end)
    }
}

/// Which of the two use-site target patterns `annotation_site` matches.
#[derive(Clone, Copy, Debug)]
pub(super) enum SiteForm {
    /// `@target: [`, a list of annotations.
    List,
    /// `@target:` followed by one annotation.
    Single,
}
