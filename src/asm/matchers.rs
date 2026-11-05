//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups of its match with [`Lexer::cap`].
//! The engine names those the grammar names.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would.
//! Where a quantifier could give characters back, the doc comment says why it
//! cannot matter, or how the matcher gives them back.
//!
//! Most patterns are `\b(…)\b` over words made of `\w`.
//! Such a pattern matches only the whole run of `\w` from a run's start, so a
//! matcher looks the run up in a sorted list.
//! `(?i)` compares words through [`crate::case_fold`].
//! `\w`, `\s` and `[[:alpha:]]` read Unicode, through [`crate::classes`].

mod directives;
mod labels;
mod numbers;

use super::engine::{Lexer, Lowered, Spelling};
use super::grammar::Pat;
use super::words;
use crate::case_fold;
use crate::rules::Chars;

/// How a support list's words are compared.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Case {
    /// Without regard to case, as `(?i)` reads them, with the list's
    /// spellings with `ß` or a Latin ligature that Oniguruma accepts.
    Folded(&'static [&'static [u8]]),
    /// As written.
    Exact,
}

impl Lexer<'_> {
    /// Whether `pat` matches at `p`, and where its match ends.
    ///
    /// The arms come in the grammar's order.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm for each expression of the grammar"
    )]
    pub(super) fn matcher(&mut self, pat: Pat, p: usize) -> Option<usize> {
        match pat {
            Pat::Word => self.word(p),
            Pat::ConstantsAt0 => self.prefixed_fraction(p, numbers::Base::Binary),
            Pat::ConstantsAt1 => self.prefixed_exponent(p, numbers::Base::Binary),
            Pat::ConstantsAt2 => self.prefixed_fraction(p, numbers::Base::Octal),
            Pat::ConstantsAt3 => self.prefixed_exponent(p, numbers::Base::Octal),
            Pat::ConstantsAt4 => self.decimal_fraction(p),
            Pat::ConstantsAt5 => self.decimal_exponent(p),
            Pat::ConstantsAt6 => self.packed_bcd(p),
            Pat::ConstantsAt7 => self.prefixed_fraction(p, numbers::Base::Hex),
            Pat::ConstantsAt8 => self.prefixed_exponent(p, numbers::Base::Hex),
            Pat::ConstantsAt9 => self.dollar_fraction(p),
            Pat::ConstantsAt10 => self.dollar_exponent(p),
            Pat::ConstantsAt11 => self.integer(p, numbers::Base::Binary),
            Pat::ConstantsAt12 => self.integer(p, numbers::Base::Octal),
            Pat::ConstantsAt13 => self.decimal(p),
            Pat::ConstantsAt14 => self.dollar_hex(p),
            Pat::ConstantsAt15 => self.integer(p, numbers::Base::Hex),
            Pat::EntitiesAt0 => self.section(p),
            Pat::EntitiesAt1 => self.global(p),
            Pat::EntitiesAt2 => self.dollar_name(p),
            Pat::EntitiesAt3 => self.special_label(p),
            Pat::EntitiesAt4 => self.label(p, labels::Head::Name),
            Pat::EntitiesAt5 => self.local_number(p),
            Pat::EntitiesAt6 => self.label(p, labels::Head::Invalid),
            Pat::SupportAt0 => self.listed(
                p,
                words::SUPPORT_AT0,
                Case::Folded(words::SUPPORT_AT0_SPELLINGS),
            ),
            Pat::SupportAt1 => self.listed(
                p,
                words::SUPPORT_AT1,
                Case::Folded(words::SUPPORT_AT1_SPELLINGS),
            ),
            Pat::SupportAt2 => self.listed(
                p,
                words::SUPPORT_AT2,
                Case::Folded(words::SUPPORT_AT2_SPELLINGS),
            ),
            Pat::SupportAt3 => self.listed(
                p,
                words::SUPPORT_AT3,
                Case::Folded(words::SUPPORT_AT3_SPELLINGS),
            ),
            Pat::SupportAt4 => self.listed(
                p,
                words::SUPPORT_AT4,
                Case::Folded(words::SUPPORT_AT4_SPELLINGS),
            ),
            Pat::SupportAt5 => self.braced(p, words::SUPPORT_AT5),
            Pat::SupportAt6 => self.braced(p, words::SUPPORT_AT6),
            Pat::SupportAt7 => self.braced(p, words::SUPPORT_AT7),
            Pat::SupportAt8 => self.braced(p, words::SUPPORT_AT8),
            Pat::SupportAt9 => self.dotted(p),
            Pat::SupportAt10 => self.literal(p, words::SUPPORT_AT10),
            Pat::SupportAt11 => self.literal(p, words::SUPPORT_AT11),
            Pat::SupportAt12 => self.literal(p, words::SUPPORT_AT12),
            Pat::SupportAt13 => self.literal(p, words::SUPPORT_AT13),
            Pat::SupportAt14 => self.literal(p, words::SUPPORT_AT14),
            Pat::SupportAt15 => self.literal(p, words::SUPPORT_AT15),
            Pat::SupportAt16 => self.marked_name(p, b"__?USE_", b"?__"),
            Pat::SupportAt17 => self.literal(p, words::SUPPORT_AT17),
            Pat::SupportAt18 => self.literal(p, words::SUPPORT_AT18),
            Pat::SupportAt19 => self.marked_name(p, b"__?ALIGN_", b"?__"),
            Pat::SupportAt20 => self.literal(p, words::SUPPORT_AT20),
            Pat::SupportAt21 => self.literal(p, words::SUPPORT_AT21),
            Pat::SupportAt22 => self.literal(p, words::SUPPORT_AT22),
            Pat::SupportAt23 => self.literal(p, words::SUPPORT_AT23),
            Pat::SupportAt24 => self.underscored_name(p, b"__USE_"),
            Pat::SupportAt25 => self.literal(p, words::SUPPORT_AT25),
            Pat::SupportAt26 => self.literal(p, words::SUPPORT_AT26),
            Pat::SupportAt27 => self.underscored_name(p, b"__ALIGN_"),
            Pat::SupportAt28 => self.listed(p, words::SUPPORT_AT28, Case::Exact),
            Pat::SupportAt29 => self.listed(p, words::SUPPORT_AT29, Case::Exact),
            Pat::SupportAt30 => self.listed(
                p,
                words::SUPPORT_AT30,
                Case::Folded(words::SUPPORT_AT30_SPELLINGS),
            ),
            Pat::CommentsAt0 => self.line_comment(p),
            Pat::CommentsAt1 => self.lit(p, b"/*"),
            Pat::CommentsAt1End => self.lit(p, b"*/"),
            Pat::CommentsAt2 => self.if_zero(p),
            Pat::CommentsAt2End => self.directive_line(p, &[b"endif"], directives::After::Boundary),
            Pat::PreprocessorAt0 => {
                self.directive_line(p, words::PREPROCESSOR_AT0, directives::After::Boundary)
            }
            Pat::PreprocessorAt1 => {
                self.directive_line(p, words::PREPROCESSOR_AT1, directives::After::Blanks)
            }
            Pat::PreprocessorAt2 => {
                self.directive_line(p, words::PREPROCESSOR_AT2, directives::After::Boundary)
            }
            Pat::PreprocessorAt3 => {
                self.directive_line(p, words::PREPROCESSOR_AT3, directives::After::Boundary)
            }
            Pat::PreprocessorAt0End | Pat::PreprocessorAt3End => self.at_eol(p).then_some(p),
            Pat::PreprocessorAt1End | Pat::PreprocessorAt2End => self.directive_end(p),
            Pat::PreprocessorAt0At0
            | Pat::PreprocessorAt1At0
            | Pat::PreprocessorAt2At0
            | Pat::PreprocessorAt3At0 => self.continuation(p),
            Pat::PreprocessorFunctionsAt0 => self.function(p),
            Pat::PreprocessorFunctionsAt0End => self.function_end(p),
            Pat::PreprocessorAt1At1 | Pat::PreprocessorAt1At1End => self.lit(p, b"\""),
            Pat::PreprocessorAt1At2 => self.lit(p, b"<"),
            Pat::PreprocessorAt1At2End => self.lit(p, b">"),
        }
    }

    // Words.

    /// The run of `\w` at `p` and its letters, lowered once for every word
    /// pattern that asks about it.
    fn lowered(&self, p: usize) -> Lowered {
        let kept = self.x.lowered.get();
        if kept.start == p && kept.text == self.lim() {
            return kept;
        }
        let end = self.word_end(p);
        let mut word = Lowered {
            start: p,
            end,
            text: self.lim(),
            len: None,
            letters: [0; words::LONGEST],
            spelling: Spelling::Letters,
        };
        word.len = case_fold::lower_into(self.text(p, end), &mut word.letters);
        if !self.text(p, end).is_ascii() {
            word.spelling = Self::spelling(self.text(p, end));
        }
        self.x.lowered.set(word);
        word
    }

    /// Whether `text` holds `ß`, `ẞ` or a Latin ligature, the characters
    /// that fold to more than one letter.
    #[cold]
    fn spelling(text: &[u8]) -> Spelling {
        let ligature = std::str::from_utf8(text).is_ok_and(|text| {
            text.chars()
                .any(|c| case_fold::letters(c).is_some_and(|letters| letters.len() > 1))
        });
        if ligature {
            Spelling::Ligature
        } else {
            Spelling::Letters
        }
    }

    /// The run of `\w` from `p` to `end` as the generated spellings keep it,
    /// written into `buf`: ASCII letters lowercase, `ſ` as `s`, the Kelvin
    /// sign as `k`, `ẞ` as `ß` and `ﬅ` as `ﬆ`.
    /// `None` when the run holds another character above ASCII that `(?i)`
    /// folds to no ASCII letters, or does not fit.
    #[cold]
    fn kept<'b>(
        &self,
        p: usize,
        end: usize,
        buf: &'b mut [u8; 2 * words::LONGEST],
    ) -> Option<&'b [u8]> {
        let text = std::str::from_utf8(self.text(p, end)).ok()?;
        let mut len = 0;
        for c in text.chars() {
            let c = match c {
                '\u{17f}' => 's',
                '\u{212a}' => 'k',
                '\u{1e9e}' => '\u{df}',
                '\u{fb05}' => '\u{fb06}',
                c if c.is_ascii() => c.to_ascii_lowercase(),
                c if case_fold::letters(c).is_some() => c,
                _ => return None,
            };
            let width = c.len_utf8();
            c.encode_utf8(buf.get_mut(len..len + width)?);
            len += width;
        }
        buf.get(..len)
    }

    /// The index in [`words::WORDS`] of the lowered `letters`.
    fn word_index(letters: &[u8]) -> Option<usize> {
        let mut low = 0;
        let mut high = words::ENDS.len();
        while low < high {
            let mid = low + (high - low) / 2;
            let start = mid
                .checked_sub(1)
                .and_then(|before| words::ENDS.get(before))
                .map_or(0, |&end| usize::from(end));
            let end = usize::from(*words::ENDS.get(mid)?);
            let word = words::WORDS.get(start..end)?;
            match word.cmp(letters) {
                std::cmp::Ordering::Less => low = mid + 1,
                std::cmp::Ordering::Greater => high = mid,
                std::cmp::Ordering::Equal => return Some(mid),
            }
        }
        None
    }

    /// `Pat::Word`: a register or a mnemonic, the whole run of `\w` at `p`,
    /// scoped with the name of the first merged pattern that lists it.
    fn word(&mut self, p: usize) -> Option<usize> {
        let (end, owner) = self.word_owner(p)?;
        self.span(p, end, *words::NAMES.get(usize::from(owner))?);
        Some(end)
    }

    /// The end of the run of `\w` at `p` and its owner, when the merged word
    /// list holds it.
    pub(super) fn word_owner(&self, p: usize) -> Option<(usize, u8)> {
        if self.word_before(p) {
            return None;
        }
        let word = self.lowered(p);
        let letters = word.letters.get(..word.len?)?;
        if word.end == p {
            return None;
        }
        match word.spelling {
            Spelling::Letters => {
                let index = Self::word_index(letters)?;
                Some((word.end, *words::OWNERS.get(index)?))
            }
            Spelling::Ligature => {
                let mut buf = [0; 2 * words::LONGEST];
                let kept = self.kept(p, word.end, &mut buf)?;
                let index = words::SPELLINGS.binary_search(&kept).ok()?;
                Some((word.end, *words::SPELLING_OWNERS.get(index)?))
            }
        }
    }

    /// `\b(?:…)\b` for the alternation `words`, compared as `case` says:
    /// the end of the run of `\w` at `p` when it is one of them.
    fn listed(&self, p: usize, words: &[&[u8]], case: Case) -> Option<usize> {
        debug_assert!(
            words.is_sorted(),
            "a word list is sorted for its binary search"
        );
        if self.word_before(p) {
            return None;
        }
        let word = self.lowered(p);
        if word.end == p {
            return None;
        }
        let found = match (case, word.spelling) {
            (Case::Folded(_), Spelling::Letters) => {
                words.binary_search(&word.letters.get(..word.len?)?)
            }
            (Case::Folded(spellings), Spelling::Ligature) => {
                let mut buf = [0; 2 * words::LONGEST];
                spellings.binary_search(&self.kept(p, word.end, &mut buf)?)
            }
            (Case::Exact, Spelling::Letters | Spelling::Ligature) => {
                words.binary_search(&self.text(p, word.end))
            }
        };
        found.is_ok().then_some(word.end)
    }

    /// `\{(…)}` for the alternation `words`: a word between braces, as
    /// group 1.
    fn braced(&mut self, p: usize, words: &[&[u8]]) -> Option<usize> {
        let open = self.lit(p, b"{")?;
        let word = words
            .iter()
            .find(|word| self.starts(open, word) && self.at(open + word.len()) == b'}')?;
        let close = open + word.len();
        self.cap(1, open, close);
        Some(close + 1)
    }

    /// `\.\.(?:start|imagebase|…)\b`: `..` and a word of the list.
    fn dotted(&self, p: usize) -> Option<usize> {
        let word = self.lit(p, b"..")?;
        let end = self.word_end(word);
        (end > word
            && words::SUPPORT_AT9
                .binary_search(&self.text(word, end))
                .is_ok())
        .then_some(end)
    }

    /// `\b(?:…)\b` for a list of words made of `\w` and `?`, such as
    /// `__?FILE?__`, as written.
    ///
    /// A word can end only where a `\w` stands before and none after.
    /// The generator checks that no word of a list is another followed by
    /// such a place, so the first that the list holds is the match.
    /// The search stops past the list's longest word, so a long run of
    /// `__?` costs no more than that at each place.
    fn literal(&self, p: usize, words: &[&[u8]]) -> Option<usize> {
        if self.word_before(p) || self.word_at(p).is_none() {
            return None;
        }
        let longest = words.iter().map(|word| word.len()).max().unwrap_or(0);
        let mut i = p;
        while i <= p + longest {
            let run = self.word_end(i);
            if run > i && words.binary_search(&self.text(p, run)).is_ok() {
                return Some(run);
            }
            if self.at(run) != b'?' {
                return None;
            }
            i = run + 1;
        }
        None
    }

    /// `\b` and `head`, a run of `\w`, then `tail` and `\b`:
    /// `\b__\?USE_\w+\?__\b` and `\b__\?ALIGN_(\w+)\?__\b`.
    ///
    /// Giving back a `\w` would leave a `\w` where the `?` of `tail` must
    /// stand.
    fn marked_name(&self, p: usize, head: &[u8], tail: &[u8]) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        let name = self.lit(p, head)?;
        let end = self.word_end(name);
        if end == name {
            return None;
        }
        let after = self.lit(end, tail)?;
        self.word_at(after).is_none().then_some(after)
    }

    /// `\b` and `head`, then `\w+__` and `\b`: `\b__USE_\w+__\b` and
    /// `\b__ALIGN_(\w+)__\b`.
    ///
    /// The greedy `\w+` takes the run's closing `__` too, then gives back two
    /// characters for it.
    /// A `__` that ends before the run's end has a `\w` after it, where no
    /// `\b` stands, so the match ends at the run's end.
    fn underscored_name(&self, p: usize, head: &[u8]) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        let name = self.lit(p, head)?;
        let end = self.word_end(name);
        (end >= name + 3 && self.text(end - 2, end) == b"__").then_some(end)
    }
}

#[cfg(test)]
impl Lexer<'_> {
    /// Whether the merged pattern at `index` alone matches at `p`: the end of
    /// the run of `\w` there when that pattern lists it, as the first owner or
    /// as a later one.
    pub(super) fn listed_in(&self, p: usize, index: u8) -> Option<usize> {
        let (end, owner) = self.word_owner(p)?;
        if owner == index {
            return Some(end);
        }
        let word = self.lowered(p);
        let mut buf = [0; 2 * words::LONGEST];
        let (spelled, also_in) = match word.spelling {
            Spelling::Letters => (word.letters.get(..word.len?)?, words::ALSO_IN),
            Spelling::Ligature => (self.kept(p, end, &mut buf)?, words::SPELLINGS_ALSO_IN),
        };
        also_in
            .iter()
            .any(|&(also, list)| also == spelled && list == index)
            .then_some(end)
    }
}
