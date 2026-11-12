//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups of its match with [`Lexer::capture`].
//! The engine names those the grammar names.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would.
//! Where a quantifier could give characters back, the doc comment shows why it
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
use super::grammar::Pattern;
use super::words;
use crate::rules::Chars;
use crate::{bytes, case_fold};

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
    /// Whether `pattern` matches at `p`, and where its match ends.
    ///
    /// The arms come in the grammar's order.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm for each expression of the grammar"
    )]
    pub(super) fn matcher(&mut self, pattern: Pattern, p: usize) -> Option<usize> {
        match pattern {
            Pattern::Word => self.word(p),
            Pattern::ConstantsAt0 => self.prefixed_fraction(p, numbers::Base::Binary),
            Pattern::ConstantsAt1 => self.prefixed_exponent(p, numbers::Base::Binary),
            Pattern::ConstantsAt2 => self.prefixed_fraction(p, numbers::Base::Octal),
            Pattern::ConstantsAt3 => self.prefixed_exponent(p, numbers::Base::Octal),
            Pattern::ConstantsAt4 => self.decimal_fraction(p),
            Pattern::ConstantsAt5 => self.decimal_exponent(p),
            Pattern::ConstantsAt6 => self.packed_bcd(p),
            Pattern::ConstantsAt7 => self.prefixed_fraction(p, numbers::Base::Hex),
            Pattern::ConstantsAt8 => self.prefixed_exponent(p, numbers::Base::Hex),
            Pattern::ConstantsAt9 => self.dollar_fraction(p),
            Pattern::ConstantsAt10 => self.dollar_exponent(p),
            Pattern::ConstantsAt11 => self.integer(p, numbers::Base::Binary),
            Pattern::ConstantsAt12 => self.integer(p, numbers::Base::Octal),
            Pattern::ConstantsAt13 => self.decimal(p),
            Pattern::ConstantsAt14 => self.dollar_hex(p),
            Pattern::ConstantsAt15 => self.integer(p, numbers::Base::Hex),
            Pattern::EntitiesAt0 => self.section(p),
            Pattern::EntitiesAt1 => self.global(p),
            Pattern::EntitiesAt2 => self.dollar_name(p),
            Pattern::EntitiesAt3 => self.special_label(p),
            Pattern::EntitiesAt4 => self.label(p, labels::Head::Name),
            Pattern::EntitiesAt5 => self.local_number(p),
            Pattern::EntitiesAt6 => self.label(p, labels::Head::Invalid),
            Pattern::SupportAt0 => self.listed(
                p,
                words::SUPPORT_AT0,
                Case::Folded(words::SUPPORT_AT0_SPELLINGS),
            ),
            Pattern::SupportAt1 => self.listed(
                p,
                words::SUPPORT_AT1,
                Case::Folded(words::SUPPORT_AT1_SPELLINGS),
            ),
            Pattern::SupportAt2 => self.listed(
                p,
                words::SUPPORT_AT2,
                Case::Folded(words::SUPPORT_AT2_SPELLINGS),
            ),
            Pattern::SupportAt3 => self.listed(
                p,
                words::SUPPORT_AT3,
                Case::Folded(words::SUPPORT_AT3_SPELLINGS),
            ),
            Pattern::SupportAt4 => self.listed(
                p,
                words::SUPPORT_AT4,
                Case::Folded(words::SUPPORT_AT4_SPELLINGS),
            ),
            Pattern::SupportAt5 => self.braced(p, words::SUPPORT_AT5),
            Pattern::SupportAt6 => self.braced(p, words::SUPPORT_AT6),
            Pattern::SupportAt7 => self.braced(p, words::SUPPORT_AT7),
            Pattern::SupportAt8 => self.braced(p, words::SUPPORT_AT8),
            Pattern::SupportAt9 => self.dotted(p),
            Pattern::SupportAt10 => self.literal(p, words::SUPPORT_AT10),
            Pattern::SupportAt11 => self.literal(p, words::SUPPORT_AT11),
            Pattern::SupportAt12 => self.literal(p, words::SUPPORT_AT12),
            Pattern::SupportAt13 => self.literal(p, words::SUPPORT_AT13),
            Pattern::SupportAt14 => self.literal(p, words::SUPPORT_AT14),
            Pattern::SupportAt15 => self.literal(p, words::SUPPORT_AT15),
            Pattern::SupportAt16 => self.marked_name(p, b"__?USE_", b"?__"),
            Pattern::SupportAt17 => self.literal(p, words::SUPPORT_AT17),
            Pattern::SupportAt18 => self.literal(p, words::SUPPORT_AT18),
            Pattern::SupportAt19 => self.marked_name(p, b"__?ALIGN_", b"?__"),
            Pattern::SupportAt20 => self.literal(p, words::SUPPORT_AT20),
            Pattern::SupportAt21 => self.literal(p, words::SUPPORT_AT21),
            Pattern::SupportAt22 => self.literal(p, words::SUPPORT_AT22),
            Pattern::SupportAt23 => self.literal(p, words::SUPPORT_AT23),
            Pattern::SupportAt24 => self.underscored_name(p, b"__USE_"),
            Pattern::SupportAt25 => self.literal(p, words::SUPPORT_AT25),
            Pattern::SupportAt26 => self.literal(p, words::SUPPORT_AT26),
            Pattern::SupportAt27 => self.underscored_name(p, b"__ALIGN_"),
            Pattern::SupportAt28 => self.listed(p, words::SUPPORT_AT28, Case::Exact),
            Pattern::SupportAt29 => self.listed(p, words::SUPPORT_AT29, Case::Exact),
            Pattern::SupportAt30 => self.listed(
                p,
                words::SUPPORT_AT30,
                Case::Folded(words::SUPPORT_AT30_SPELLINGS),
            ),
            Pattern::CommentsAt0 => self.line_comment(p),
            Pattern::CommentsAt1 => self.lit(p, b"/*"),
            Pattern::CommentsAt1End => self.lit(p, b"*/"),
            Pattern::CommentsAt2 => self.if_zero(p),
            Pattern::CommentsAt2End => {
                self.directive_line(p, &[b"endif"], directives::After::Boundary)
            }
            Pattern::PreprocessorAt0 => {
                self.directive_line(p, words::PREPROCESSOR_AT0, directives::After::Boundary)
            }
            Pattern::PreprocessorAt1 => {
                self.directive_line(p, words::PREPROCESSOR_AT1, directives::After::Blanks)
            }
            Pattern::PreprocessorAt2 => {
                self.directive_line(p, words::PREPROCESSOR_AT2, directives::After::Boundary)
            }
            Pattern::PreprocessorAt3 => {
                self.directive_line(p, words::PREPROCESSOR_AT3, directives::After::Boundary)
            }
            Pattern::PreprocessorAt0End | Pattern::PreprocessorAt3End => {
                self.at_eol(p).then_some(p)
            }
            Pattern::PreprocessorAt1End | Pattern::PreprocessorAt2End => self.directive_end(p),
            Pattern::PreprocessorAt0At0
            | Pattern::PreprocessorAt1At0
            | Pattern::PreprocessorAt2At0
            | Pattern::PreprocessorAt3At0 => self.continuation(p),
            Pattern::PreprocessorFunctionsAt0 => self.function(p),
            Pattern::PreprocessorFunctionsAt0End => self.function_end(p),
            Pattern::PreprocessorAt1At1 | Pattern::PreprocessorAt1At1End => self.lit(p, b"\""),
            Pattern::PreprocessorAt1At2 => self.lit(p, b"<"),
            Pattern::PreprocessorAt1At2End => self.lit(p, b">"),
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
    ///
    /// It compares words in a byte loop.
    /// `Ord` on byte slices called libc's `memcmp`, whose count follows
    /// where the words lie, and moved asm's Callgrind row by 0.5%.
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
            match bytes::order(word, letters) {
                std::cmp::Ordering::Less => low = mid + 1,
                std::cmp::Ordering::Greater => high = mid,
                std::cmp::Ordering::Equal => return Some(mid),
            }
        }
        None
    }

    /// `Pattern::Word`: a register or a mnemonic, the whole run of `\w` at `p`,
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
                let letters = word.letters.get(..word.len?)?;
                words.binary_search_by(|listed| bytes::order(listed, letters))
            }
            (Case::Folded(spellings), Spelling::Ligature) => {
                let mut buf = [0; 2 * words::LONGEST];
                let kept = self.kept(p, word.end, &mut buf)?;
                spellings.binary_search_by(|listed| bytes::order(listed, kept))
            }
            (Case::Exact, Spelling::Letters | Spelling::Ligature) => {
                let text = self.text(p, word.end);
                words.binary_search_by(|listed| bytes::order(listed, text))
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
        self.capture(1, open, close);
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

    /// `\b(?:…)\b` for a word list made of `\w` and `?`, such as
    /// `__?FILE?__`, as written.
    ///
    /// A word can end only where a `\w` stands before and none after.
    /// The generator checks that no word of a list is another followed by
    /// such a place.
    /// So the first word that the list holds is the match.
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

#[cfg(all(test, feature = "json"))]
impl Lexer<'_> {
    /// Whether the merged pattern at `index` alone matches at `p`.
    /// The answer is the end of the run of `\w` there when that pattern lists
    /// it, as the first owner or as a later one.
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
