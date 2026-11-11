//! Sets of characters, as the writer reasons about them.

use std::cell::RefCell;
use std::collections::HashMap;

use nishiki::unstable::regex::Class;
use nishiki::unstable::{is_space, is_word};

/// The largest code point.
const MAX_CHAR: u32 = 0x10_ffff;

/// A set of characters: an ASCII mask, and sorted, disjoint ranges above it.
#[derive(Clone, Debug, Default)]
pub(super) struct Chars {
    /// Bit `n` holds when the character with code `n` is in the set.
    pub(super) ascii: u128,
    /// The characters from 0x80 up, as sorted, disjoint, inclusive ranges.
    pub(super) wide: Vec<(u32, u32)>,
}

thread_local! {
    /// Each class's characters, by the class as `Debug` prints it.
    /// Finding them tests every code point, so each class is tested once.
    static CLASSES: RefCell<HashMap<String, Chars>> = RefCell::new(HashMap::new());

    /// The characters of `\s`, as the lexer's `space_end` reads them.
    static SPACE: Chars = Chars::of_test(is_space);

    /// The characters of `\w`, as the lexer's `word_at` reads them.
    static WORD: Chars = Chars::of_test(is_word);
}

impl Chars {
    /// Every character.
    pub(super) fn all() -> Self {
        Chars {
            ascii: u128::MAX,
            wide: vec![(0x80, MAX_CHAR)],
        }
    }

    /// One character.
    pub(super) fn one(c: char) -> Self {
        let code = u32::from(c);
        if code < 0x80 {
            Chars {
                ascii: 1 << code,
                wide: Vec::new(),
            }
        } else {
            Chars {
                ascii: 0,
                wide: vec![(code, code)],
            }
        }
    }

    /// The characters of `class`.
    pub(super) fn of_class(class: &Class) -> Self {
        CLASSES.with_borrow_mut(|cache| {
            cache
                .entry(format!("{class:?}"))
                .or_insert_with(|| Chars::of_test_ascii(class.ascii(), |c| class.matches(c)))
                .clone()
        })
    }

    /// The characters of `\s`.
    pub(super) fn space() -> Self {
        SPACE.with(Chars::clone)
    }

    /// The characters of `\w`.
    pub(super) fn word() -> Self {
        WORD.with(Chars::clone)
    }

    /// The characters `test` takes.
    fn of_test(test: fn(char) -> bool) -> Self {
        let ascii = (0..0x80_u32)
            .filter(|&code| char::from_u32(code).is_some_and(test))
            .fold(0, |mask, code| mask | 1 << code);
        Chars::of_test_ascii(ascii, test)
    }

    /// The set with the ASCII mask `ascii`, and the characters from 0x80 up
    /// that `test` takes.
    fn of_test_ascii(ascii: u128, test: impl Fn(char) -> bool) -> Self {
        let mut wide: Vec<(u32, u32)> = Vec::new();
        for code in 0x80..=MAX_CHAR {
            if !char::from_u32(code).is_some_and(&test) {
                continue;
            }
            match wide.last_mut() {
                Some(last) if last.1 + 1 == code => last.1 = code,
                _ => wide.push((code, code)),
            }
        }
        Chars { ascii, wide }
    }

    /// Adds the characters of `other`.
    pub(super) fn add(&mut self, other: &Chars) {
        self.ascii |= other.ascii;
        let mut all = self.wide.clone();
        all.extend_from_slice(&other.wide);
        all.sort_unstable();
        let mut merged: Vec<(u32, u32)> = Vec::new();
        for (first, last) in all {
            match merged.last_mut() {
                Some(run) if first <= run.1.saturating_add(1) => run.1 = run.1.max(last),
                _ => merged.push((first, last)),
            }
        }
        self.wide = merged;
    }

    /// Whether the two sets hold the same characters.
    pub(super) fn same(&self, other: &Chars) -> bool {
        self.within(other) && other.within(self)
    }

    /// Whether every character of this set is in `other`.
    pub(super) fn within(&self, other: &Chars) -> bool {
        self.ascii & !other.ascii == 0
            && self.wide.iter().all(|&(first, last)| {
                other
                    .wide
                    .iter()
                    .any(|&(from, to)| from <= first && last <= to)
            })
    }

    /// Whether the two sets share a character.
    pub(super) fn meets(&self, other: &Chars) -> bool {
        if self.ascii & other.ascii != 0 {
            return true;
        }
        let (mut i, mut j) = (0, 0);
        while let (Some(a), Some(b)) = (self.wide.get(i), other.wide.get(j)) {
            if a.1 < b.0 {
                i += 1;
            } else if b.1 < a.0 {
                j += 1;
            } else {
                return true;
            }
        }
        false
    }

    /// The UTF-8 length of the widest character the two sets share.
    /// It is 0 when they share none.
    pub(super) fn widest_shared(&self, other: &Chars) -> usize {
        let mut widest = 0;
        let (mut i, mut j) = (0, 0);
        while let (Some(a), Some(b)) = (self.wide.get(i), other.wide.get(j)) {
            let top = a.1.min(b.1);
            if a.0.max(b.0) <= top {
                widest = widest.max(top);
            }
            if a.1 < b.1 {
                i += 1;
            } else {
                j += 1;
            }
        }
        if widest > 0 {
            char::from_u32(widest).map_or(4, char::len_utf8)
        } else {
            usize::from(self.ascii & other.ascii != 0)
        }
    }
}
