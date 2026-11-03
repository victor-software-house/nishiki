//! The character classes of the grammar's expressions.

use std::sync::OnceLock;

use crate::classes::is_alpha;
use crate::fnv::FnvMap;
use crate::python::engine::Lexer;
use crate::python::words;
use crate::rules::Chars;

impl Lexer<'_> {
    /// The end of the run of `\w` characters from `i`. Patterns ask about
    /// the same word one after another, so the last run is kept.
    pub(in crate::python) fn word_end(&self, i: usize) -> usize {
        let (start, end) = self.x.word_run.get();
        if start <= i && i < end {
            return end;
        }
        let mut e = i;
        while let Some(width) = self.word_at(e) {
            e += width;
        }
        self.spend(e - i);
        if e > i {
            self.x.word_run.set((i, e));
        }
        e
    }

    /// The end of the identifier `[_[:alpha:]]\w*` at `i`, when one starts.
    pub(in crate::python) fn ident_end(&self, i: usize) -> Option<usize> {
        let (c, first) = self.char_at(i)?;
        if c != '_' && !is_alpha(c) {
            return None;
        }
        Some(self.word_end(first))
    }

    /// The end of the word at `i` when `\b` holds before it and the word is
    /// one of `list`, a sorted list of words.
    pub(in crate::python) fn word_in(&self, i: usize, list: &[&[u8]]) -> Option<usize> {
        debug_assert!(list.is_sorted(), "word list not sorted: {list:?}");
        if !self.boundary(i) {
            return None;
        }
        let end = self.word_end(i);
        list.binary_search(&self.text(i, end)).ok().map(|_| end)
    }

    /// The end of the word at `i` when `\b` holds before it and one of the
    /// word lists in `bits` has the word.
    pub(in crate::python) fn word_listed(&self, i: usize, bits: u8) -> Option<usize> {
        if !self.boundary(i) {
            return None;
        }
        let end = self.word_end(i);
        let (at, mask) = self.x.word_mask.get();
        let mask = if at == i {
            mask
        } else {
            let mask = word_masks().get(self.text(i, end)).copied().unwrap_or(0);
            self.x.word_mask.set((i, mask));
            mask
        };
        (mask & bits != 0).then_some(end)
    }
}

/// The word lists a word can be in.
pub(in crate::python) const FUNCTION: u8 = 1;
pub(in crate::python) const LEGACY: u8 = 2;
pub(in crate::python) const TYPE: u8 = 4;
pub(in crate::python) const EXCEPTION: u8 = 8;
pub(in crate::python) const MAGIC_FUNCTION: u8 = 16;
pub(in crate::python) const MAGIC_VARIABLE: u8 = 32;

/// Which of the big word lists have each word, built on first use.
fn word_masks() -> &'static FnvMap<&'static [u8], u8> {
    static MASKS: OnceLock<FnvMap<&'static [u8], u8>> = OnceLock::new();
    MASKS.get_or_init(|| {
        let mut masks: FnvMap<&'static [u8], u8> = FnvMap::default();
        for (list, bit) in [
            (words::BUILTIN_FUNCTIONS, FUNCTION),
            (words::LEGACY_BUILTINS, LEGACY),
            (words::BUILTIN_TYPES, TYPE),
            (words::BUILTIN_EXCEPTIONS, EXCEPTION),
            (words::MAGIC_FUNCTIONS, MAGIC_FUNCTION),
            (words::MAGIC_VARIABLES, MAGIC_VARIABLE),
        ] {
            for word in list {
                *masks.entry(word).or_insert(0) |= bit;
            }
        }
        masks
    })
}
