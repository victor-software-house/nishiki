//! Expressions that put the gap block before or after a word.

use super::super::engine::Lexer;
use super::gaps::GapForm;
use crate::rules::Chars;

/// Whether a parenthesis is a capture group, for [`Lexer::gap_paren`].
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Paren {
    /// Not a group.
    Plain,
    /// Group 5.
    Group,
}

impl Lexer<'_> {
    /// `GAP((?<!\w)(?:a|b|…)(?!\w))`, with the gap of the plain form as
    /// groups 1 and 2 and the word as group `word_group`.
    pub(super) fn gap_word(
        &mut self,
        p: usize,
        words: &[&[u8]],
        word_group: usize,
    ) -> Option<usize> {
        for gap in self.gaps(p) {
            if let Some(end) = self.word_in(gap.end, words) {
                self.put_gap(1, GapForm::Plain, p, &gap);
                self.capture(word_group, gap.end, end);
                return Some(end);
            }
        }
        None
    }

    /// `GAP((?<!\w)word(?!\w))` with the gap of the form that captures a
    /// comment's parts as groups 1 to 4 and the word as group 5.
    pub(super) fn gap4_word(&mut self, p: usize, words: &[&[u8]]) -> Option<usize> {
        for gap in self.gaps(p) {
            if let Some(end) = self.word_in(gap.end, words) {
                self.put_gap(1, GapForm::Parts, p, &gap);
                self.capture(5, gap.end, end);
                return Some(end);
            }
        }
        None
    }

    /// `((?<!\w)word(?!\w))GAP(\()`, the word as group 1, the gap as groups
    /// 2 to 5 and the parenthesis as group 6.
    /// `bounded` stands for `\bword` instead of `(?<!\w)word`.
    pub(super) fn word_gap_paren(&mut self, p: usize, word: &[u8]) -> Option<usize> {
        let end = self.word_lit(p, word)?;
        self.gap_then_paren(p, end, 1, 2, 6)
    }

    /// The gap after a word that ends at `end`, and a `(`.
    fn gap_then_paren(
        &mut self,
        p: usize,
        end: usize,
        word_group: usize,
        gap_base: usize,
        paren_group: usize,
    ) -> Option<usize> {
        for gap in self.gaps(end) {
            if self.at(gap.end) == b'(' && gap.end < self.lim() {
                self.capture(word_group, p, end);
                self.put_gap(gap_base, GapForm::Parts, end, &gap);
                self.capture(paren_group, gap.end, gap.end + 1);
                return Some(gap.end + 1);
            }
        }
        None
    }

    /// `\b(sizeof\.\.\.)GAP(\()`.
    pub(super) fn sizeof_pack(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let end = self.lit(p, b"sizeof...")?;
        self.gap_then_paren(p, end, 1, 2, 6)
    }

    /// `((?<!\w)final(?!\w))GAP` with the gap as groups 2 to 5.
    pub(super) fn final_gap(&mut self, p: usize) -> Option<usize> {
        let end = self.word_lit(p, b"final")?;
        let gap = self.gaps(end).next()?;
        self.capture(1, p, end);
        self.put_gap(2, GapForm::Parts, end, &gap);
        Some(gap.end)
    }

    /// `(=)GAP(?:(default)|(delete))`.
    pub(super) fn equals_default(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'=' || p >= self.lim() {
            return None;
        }
        let from = p + 1;
        for gap in self.gaps(from) {
            for (group, word) in [(6, &b"default"[..]), (7, b"delete")] {
                if let Some(end) = self.lit(gap.end, word) {
                    self.capture(1, p, from);
                    self.put_gap(2, GapForm::Parts, from, &gap);
                    self.capture(group, gap.end, end);
                    return Some(end);
                }
            }
        }
        None
    }

    /// `GAP(extern)(?=\s*")`.
    pub(super) fn extern_block(&mut self, p: usize) -> Option<usize> {
        for gap in self.gaps(p) {
            if let Some(end) = self.lit(gap.end, b"extern") {
                let q = self.skip_space(end);
                if self.at(q) == b'"' && q < self.lim() {
                    self.put_gap(1, GapForm::Parts, p, &gap);
                    self.capture(5, gap.end, end);
                    return Some(end);
                }
            }
        }
        None
    }

    /// `GAP(?=\w)`.
    pub(super) fn gap_ahead_word(&mut self, p: usize) -> Option<usize> {
        for gap in self.gaps(p) {
            if self.ahead_word(gap.end) {
                self.put_gap(1, GapForm::Parts, p, &gap);
                return Some(gap.end);
            }
        }
        None
    }

    /// `GAP\(` and `GAP(\()`: the gap as groups 1 to 4, and the parenthesis,
    /// which is group 5 in the second form.
    pub(super) fn gap_paren(&mut self, p: usize, paren: Paren) -> Option<usize> {
        for gap in self.gaps(p) {
            if self.at(gap.end) == b'(' && gap.end < self.lim() {
                self.put_gap(1, GapForm::Parts, p, &gap);
                if paren == Paren::Group {
                    self.capture(5, gap.end, gap.end + 1);
                }
                return Some(gap.end + 1);
            }
        }
        None
    }

    /// `^GAP(?:\n|$)`: a line of white space and comments.
    pub(super) fn gap_line(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        for gap in self.gaps(0) {
            let end = gap.end;
            let tail = if self.nl && end == self.lim() {
                Some(end + 1)
            } else if self.at_eol(end) {
                Some(end)
            } else {
                None
            };
            if let Some(tail) = tail {
                self.put_gap(1, GapForm::Parts, 0, &gap);
                return Some(tail);
            }
        }
        None
    }

    /// `GAP((?<!\w)static_assert|_Static_assert(?!\w))GAP(\()`.
    pub(super) fn static_assert(&mut self, p: usize) -> Option<usize> {
        for first in self.gaps(p) {
            let at = first.end;
            let word = self.word_prefix(at, b"static_assert").or_else(|| {
                self.lit(at, b"_Static_assert")
                    .filter(|&e| self.word_at(e).is_none())
            });
            let Some(word) = word else { continue };
            for second in self.gaps(word) {
                if self.at(second.end) == b'(' && second.end < self.lim() {
                    self.put_gap(1, GapForm::Parts, p, &first);
                    self.capture(5, at, word);
                    self.put_gap(6, GapForm::Parts, word, &second);
                    self.capture(10, second.end, second.end + 1);
                    return Some(second.end + 1);
                }
            }
        }
        None
    }

    /// `^GAP(#)\s+{0,1}WORD\b`, the hash as group 5.
    /// The match ends after the directive's name.
    pub(super) fn directive_head(&mut self, p: usize, word: &[u8]) -> Option<usize> {
        if p != 0 {
            return None;
        }
        for gap in self.gaps(0) {
            if self.at(gap.end) != b'#' || gap.end >= self.lim() {
                continue;
            }
            let name = self.skip_space(gap.end + 1);
            let Some(end) = self.lit(name, word) else {
                continue;
            };
            if !self.boundary(end) {
                continue;
            }
            self.put_gap(1, GapForm::Parts, 0, &gap);
            self.capture(5, gap.end, gap.end + 1);
            return Some(end);
        }
        None
    }

    /// `^GAP(#)\s+{0,1}(if(?:n?def|))`: group 6 is the directive.
    pub(super) fn conditional_head(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        for gap in self.gaps(0) {
            if self.at(gap.end) != b'#' || gap.end >= self.lim() {
                continue;
            }
            let name = self.skip_space(gap.end + 1);
            let end = self
                .lit(name, b"ifndef")
                .or_else(|| self.lit(name, b"ifdef"))
                .or_else(|| self.lit(name, b"if"))?;
            self.put_gap(1, GapForm::Parts, 0, &gap);
            self.capture(5, gap.end, gap.end + 1);
            self.capture(6, name, end);
            return Some(end);
        }
        None
    }

    /// `((?<!\w)(?:(?:constexpr|…|friend)|(?:thread_local|…|const))(?!\w))GAP`
    /// with the gap as groups 2 to 5.
    pub(super) fn specifier_gap(&mut self, p: usize) -> Option<usize> {
        let end = self.word_in(
            p,
            &[
                b"constexpr",
                b"consteval",
                b"explicit",
                b"mutable",
                b"virtual",
                b"inline",
                b"friend",
                b"thread_local",
                b"volatile",
                b"register",
                b"restrict",
                b"static",
                b"extern",
                b"const",
            ],
        )?;
        let gap = self.gaps(end).next()?;
        self.capture(1, p, end);
        self.put_gap(2, GapForm::Parts, end, &gap);
        Some(gap.end)
    }
}
