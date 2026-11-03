//! Preprocessor directives, access labels, memory operators and the
//! qualifier chains that follow parameters.

use super::super::engine::Lexer;
use super::gaps::{Gap, GapForm};
use crate::rules::Chars;

/// The groups of `delete[]`, `delete` and `new`: the keyword and the brackets,
/// the lone `delete`, and the `new`.
type MemoryGroups = [Option<(usize, usize)>; 4];

/// A reference chain: where it ends, its first gap, and the last
/// repetition's gap with where it starts.
pub(super) type Reference = (usize, Option<Gap>, Option<(usize, Gap)>);

/// The qualifiers that may end a function's declarator.
const QUALIFIERS: &[&[u8]] = &[b"override", b"volatile", b"noexcept", b"final", b"const"];

impl Lexer<'_> {
    /// `^(GAP(#)\s+{0,1}pragma\s+mark)\s+(.*)`.
    pub(super) fn pragma_mark(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        for gap in self.gaps(0) {
            if self.at(gap.end) != b'#' || gap.end >= self.lim() {
                continue;
            }
            let name = self.skip_space(gap.end + 1);
            let Some(pragma) = self.lit(name, b"pragma") else {
                continue;
            };
            let word = self.skip_space(pragma);
            if word == pragma {
                continue;
            }
            let Some(mark) = self.lit(word, b"mark") else {
                continue;
            };
            let rest = self.skip_space(mark);
            if rest == mark {
                continue;
            }
            let end = if rest <= self.lim() { self.lim() } else { rest };
            self.cap(1, 0, mark);
            self.put_gap(2, GapForm::Plain, 0, &gap);
            self.cap(4, gap.end, gap.end + 1);
            if end > rest {
                self.cap(5, rest, end);
            }
            return Some(end);
        }
        None
    }

    /// A gap, then `protected`, `private` or `public`, white space and a
    /// `:`: an access specifier.
    pub(super) fn access_label(&mut self, p: usize) -> Option<usize> {
        for gap in self.gaps(p) {
            let Some(word) = self.lit_any(gap.end, &[b"protected", b"private", b"public"]) else {
                continue;
            };
            let colon = self.skip_space(word);
            if self.at(colon) != b':' || colon >= self.lim() {
                continue;
            }
            self.put_gap(1, GapForm::Plain, p, &gap);
            self.cap(3, gap.end, colon + 1);
            self.cap(4, gap.end, word);
            self.cap(5, colon, colon + 1);
            return Some(colon + 1);
        }
        None
    }

    /// `^(GAP(#)\s+{0,1}(error|warning))\b\s+{0,1}`.
    pub(super) fn diagnostic(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        for gap in self.gaps(0) {
            if self.at(gap.end) != b'#' || gap.end >= self.lim() {
                continue;
            }
            let name = self.skip_space(gap.end + 1);
            for word in [&b"error"[..], b"warning"] {
                let Some(end) = self.lit(name, word) else {
                    continue;
                };
                if !self.boundary(end) {
                    continue;
                }
                self.cap(1, 0, end);
                self.put_gap(2, GapForm::Parts, 0, &gap);
                self.cap(6, gap.end, gap.end + 1);
                self.cap(7, name, end);
                return Some(self.skip_space(end));
            }
        }
        None
    }

    /// `GAP((?:(?:(delete)\s+{0,1}(\[])|(delete))|(new))(?!\w))`.
    ///
    /// The alternatives, in order: `delete[]`, `delete`, `new`.
    pub(super) fn memory_operator(&mut self, p: usize) -> Option<usize> {
        for gap in self.gaps(p) {
            let at = gap.end;
            let mut found: Option<(usize, MemoryGroups)> = None;
            if let Some(word) = self.lit(at, b"delete") {
                let open = self.skip_space(word);
                if self.starts(open, b"[]") && self.word_at(open + 2).is_none() {
                    found = Some((
                        open + 2,
                        [Some((at, word)), Some((open, open + 2)), None, None],
                    ));
                } else if self.word_at(word).is_none() {
                    found = Some((word, [None, None, Some((at, word)), None]));
                }
            }
            if found.is_none()
                && let Some(word) = self.lit(at, b"new")
                && self.word_at(word).is_none()
            {
                found = Some((word, [None, None, None, Some((at, word))]));
            }
            let Some((end, groups)) = found else { continue };
            self.put_gap(1, GapForm::Plain, p, &gap);
            self.cap(3, at, end);
            for (i, group) in groups.iter().enumerate() {
                if let Some((a, b)) = group {
                    self.cap(4 + i, *a, *b);
                }
            }
            return Some(end);
        }
        None
    }

    /// `^GAP(#)\s+{0,1}((?<!\w)e(?:ndif|lse|lif|lifdef|lifndef)(?!\w))`.
    pub(super) fn conditional_standalone(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        for gap in self.gaps(0) {
            if self.at(gap.end) != b'#' || gap.end >= self.lim() {
                continue;
            }
            let name = self.skip_space(gap.end + 1);
            let words: &[&[u8]] = &[b"endif", b"else", b"elif", b"elifdef", b"elifndef"];
            let Some(end) = self.word_in(name, words) else {
                continue;
            };
            self.put_gap(1, GapForm::Plain, 0, &gap);
            self.cap(3, gap.end, gap.end + 1);
            self.cap(4, name, end);
            return Some(end);
        }
        None
    }

    /// `\b(NUMBER)GAP(<)` and `(<<)`: a number and a `<` or `<<` after it.
    ///
    /// The tail takes as much as it can and gives back one character at a time.
    pub(super) fn number_angle(&mut self, p: usize, angle: &[u8]) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        if self.word_before(p) {
            return None;
        }
        let start = if self.at(p) == b'.' && p < self.lim() {
            self.digit_char(p + 1).or_else(|| self.digit_char(p))?
        } else {
            self.digit_char(p)?
        };
        let longest = self.number_tail(start).unwrap_or(start);
        for end in (start..=longest).rev() {
            for gap in self.gaps(end) {
                if let Some(close) = self.lit(gap.end, angle) {
                    self.cap(1, p, end);
                    self.put_gap(2, GapForm::Plain, end, &gap);
                    self.cap(4, gap.end, close);
                    return Some(close);
                }
            }
        }
        None
    }

    /// `((?:(?:GAP(?<!\w)(?:override|…)(?!\w))+)(?=\s*[\n\r;={])`, group 1.
    pub(super) fn trailing_qualifiers(&mut self, p: usize) -> Option<usize> {
        let end = self.qualifier_chain(p)?;
        self.cap(1, p, end);
        Some(end)
    }

    /// One or more gaps and qualifiers from `from`, followed by
    /// `\s*[\n\r;={]`: where the chain ends.
    pub(super) fn qualifier_chain(&self, from: usize) -> Option<usize> {
        let _depth = self.deeper()?;
        self.gaps(from).find_map(|gap| {
            let end = self.word_in(gap.end, QUALIFIERS)?;
            self.qualifier_chain_after(end)
        })
    }

    /// The rest of a qualifier chain after a qualifier that ends at `pos`:
    /// more gaps and qualifiers, the longest first, or the end of the chain
    /// here.
    fn qualifier_chain_after(&self, pos: usize) -> Option<usize> {
        let _depth = self.deeper()?;
        self.gaps(pos)
            .find_map(|gap| {
                let end = self.word_in(gap.end, QUALIFIERS)?;
                self.qualifier_chain_after(end)
            })
            .or_else(|| self.ahead_qualifier_end(pos).then_some(pos))
    }

    /// `(?=\s*[\n\r;={])`: white space that holds a line break, or `;`, `=` or
    /// `{` after it.
    pub(super) fn ahead_qualifier_end_pub(&self, pos: usize) -> bool {
        self.ahead_qualifier_end(pos)
    }

    fn ahead_qualifier_end(&self, pos: usize) -> bool {
        let end = self.skip_space(pos);
        let breaks = self.s[pos.min(self.lim())..end.min(self.lim())]
            .iter()
            .any(|&b| b == b'\n' || b == b'\r');
        let virtual_break = self.nl && end > self.lim() && pos <= self.lim();
        breaks || virtual_break || (end < self.lim() && matches!(self.s[end], b';' | b'=' | b'{'))
    }

    /// `(?:[\&*]GAP)*[\&*]` from `pos`, with at least `min` repetitions before
    /// the last symbol, `count` of them already taken.
    ///
    /// It gives `rest` each way it matches, in the order the expression tries
    /// them, as where it ends and the gap of the last repetition, until
    /// `rest` accepts one. `last` is the gap of the repetition before `pos`.
    pub(super) fn find_symbol_chain<R>(
        &self,
        pos: usize,
        min: usize,
        count: usize,
        only: Option<u8>,
        last: Option<(usize, Gap)>,
        rest: &mut impl FnMut(&Self, usize, Option<(usize, Gap)>) -> Option<R>,
    ) -> Option<R> {
        let b = self.at(pos);
        if pos >= self.lim() || !only.map_or(b == b'&' || b == b'*', |o| b == o) {
            return None;
        }
        let _depth = self.deeper()?;
        for gap in self.gaps(pos + 1) {
            let deeper = Some((pos + 1, gap));
            if let Some(found) = self.find_symbol_chain(gap.end, min, count + 1, only, deeper, rest)
            {
                return Some(found);
            }
        }
        if count >= min {
            return rest(self, pos + 1, last);
        }
        None
    }

    /// `GAP?(?:[\&*]GAP)*[\&*]` at `p`: it gives `rest` each way it matches,
    /// as its end, the first gap and the last repetition's gap, until `rest`
    /// accepts one.
    pub(super) fn find_reference_chain<R>(
        &self,
        p: usize,
        mut rest: impl FnMut(&Self, Reference) -> Option<R>,
    ) -> Option<R> {
        for first in self.gaps(p) {
            let found =
                self.find_symbol_chain(first.end, 0, 0, None, None, &mut |lexer, end, last| {
                    rest(lexer, (end, Some(first), last))
                });
            if found.is_some() {
                return found;
            }
        }
        self.find_symbol_chain(p, 0, 0, None, None, &mut |lexer, end, last| {
            rest(lexer, (end, None, last))
        })
    }

    /// `GAP?(?:[\&*]GAP)*[\&*]`: group 0 only; the gaps are groups 1 to 4 and
    /// 5 to 8.
    pub(super) fn reference_chain(&mut self, p: usize) -> Option<usize> {
        let (end, first, last) = self.find_reference_chain(p, |_, chain| Some(chain))?;
        if let Some(gap) = first {
            self.put_gap(1, GapForm::Parts, p, &gap);
        }
        if let Some((from, gap)) = last {
            self.put_gap(5, GapForm::Parts, from, &gap);
        }
        Some(end)
    }

    /// `(?:&GAP){2,}&`: the gap of the last repetition is groups 1 to 4.
    pub(super) fn ampersand_run(&mut self, p: usize) -> Option<usize> {
        let (end, last) =
            self.find_symbol_chain(p, 2, 0, Some(b'&'), None, &mut |_, end, last| {
                Some((end, last))
            })?;
        if let Some((from, gap)) = last {
            self.put_gap(1, GapForm::Parts, from, &gap);
        }
        Some(end)
    }
}
