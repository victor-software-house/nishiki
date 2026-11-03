//! The patterns that are a word, an operator or a punctuation mark.

use super::engine::{Lexer, WordLists};
use super::scope::S;
use super::wordset;

impl Lexer<'_> {
    /// The end of the word at `p` when it is in the lists `bits`: `\b(…)\b`.
    pub(super) fn word_in(&self, p: usize, bits: u32) -> Option<usize> {
        let known = self.lists.get();
        if known.start == p {
            return (known.lists & bits != 0).then_some(known.end);
        }
        if self.word_at(p) == 0 || self.word_before(p) {
            self.lists.set(WordLists {
                start: p,
                end: p,
                lists: 0,
            });
            return None;
        }
        let end = self.skip_word(p);
        let lists = wordset::of(self.text(p, end));
        self.lists.set(WordLists {
            start: p,
            end,
            lists,
        });
        (lists & bits != 0).then_some(end)
    }

    /// A word of the lists `bits` with `scope` on it.
    pub(super) fn word_rule(&mut self, p: usize, bits: u32, scope: &'static [S]) -> bool {
        match self.word_in(p, bits) {
            Some(end) => self.plain(p, end, scope),
            None => false,
        }
    }

    /// `\bk[A-Z]\w*\b` and its siblings: a word of lead letter `lead` and
    /// an upper-case letter.
    pub(super) fn mac_classic(&mut self, p: usize, lead: u8, scope: &'static [S]) -> bool {
        if self.b(p) != lead || self.word_before(p) || !self.b(p + 1).is_ascii_uppercase() {
            return false;
        }
        let end = self.skip_word(p + 1);
        self.plain(p, end, scope)
    }

    /// `\b([0-9A-Z_a-z]+_t)\b`.
    pub(super) fn posix_reserved(&mut self, p: usize) -> bool {
        if self.word_at(p) == 0 || self.word_before(p) {
            return false;
        }
        let mut end = p;
        while self.b(end).is_ascii_alphanumeric() || self.b(end) == b'_' {
            end += 1;
        }
        self.spend(end - p);
        let ok = end - p >= 3 && self.text(p, end).ends_with(b"_t") && self.word_at(end) == 0;
        ok && self.plain(p, end, &[S::SupportPosix])
    }

    /// `typedef`: no boundary before or after, as the grammar has none.
    pub(super) fn typedef(&mut self, p: usize) -> bool {
        self.starts(p, b"typedef") && self.plain(p, p + 7, &[S::KeywordTypedef])
    }

    /// `\b(enum|struct|union)\b`, named `storage.type.$1.c`.
    pub(super) fn enum_struct_union(&mut self, p: usize) -> bool {
        let Some(end) = self.word_in(p, wordset::TAG_TYPE) else {
            return false;
        };
        self.plain_dyn(p, end, "storage.type.", (p, end))
    }

    /// `\b(NAME)\b` of the predefined macros, named by the macro.
    pub(super) fn predefined(&mut self, p: usize) -> bool {
        let Some(end) = self.word_in(p, wordset::PREDEF) else {
            return false;
        };
        self.plain_dyn(
            p,
            end,
            "entity.name.other.preprocessor.macro.predefined.",
            (p, end),
        )
    }

    /// `\b__([A-Z_]+)__\b`.
    pub(super) fn predefined_probably(&mut self, p: usize) -> bool {
        if self.b(p) != b'_' || self.word_before(p) {
            return false;
        }
        let end = self.skip_word(p);
        let text = self.text(p, end);
        let ok = text.len() >= 5
            && text.starts_with(b"__")
            && text.ends_with(b"__")
            && text.iter().all(|&c| c.is_ascii_uppercase() || c == b'_');
        ok && self.plain_dyn(
            p,
            end,
            "entity.name.other.preprocessor.macro.predefined.probably.",
            (p + 2, end - 2),
        )
    }

    /// `(?<![$\w])(sizeof)(?![$\w])`.
    pub(super) fn sizeof(&mut self, p: usize) -> bool {
        if !self.starts(p, b"sizeof") {
            return false;
        }
        let end = p + 6;
        let before = self.prev(p) == Some(b'$') || self.word_before(p);
        let after = self.b(end) == b'$' && end < self.len_all() || self.word_at(end) > 0;
        !before && !after && self.plain(p, end, &[S::OpSizeof])
    }

    /// An operator at `p`: the scope of the first alternative that matches.
    pub(super) fn op_dec(&mut self, p: usize) -> bool {
        self.starts(p, b"--") && self.plain(p, p + 2, &[S::OpDecrement])
    }

    pub(super) fn op_inc(&mut self, p: usize) -> bool {
        self.starts(p, b"++") && self.plain(p, p + 2, &[S::OpIncrement])
    }

    /// `(?:[-%*+]|(?<!\()/)=`.
    pub(super) fn op_compound(&mut self, p: usize) -> bool {
        let lead = self.b(p);
        let ok = matches!(lead, b'-' | b'%' | b'*' | b'+')
            || (lead == b'/' && self.prev(p) != Some(b'('));
        ok && self.b(p + 1) == b'=' && self.plain(p, p + 2, &[S::OpAssignCompound])
    }

    /// `(?:[&^]|<<|>>|\|)=`.
    pub(super) fn op_bitwise_assign(&mut self, p: usize) -> bool {
        let len = if self.starts(p, b"<<") || self.starts(p, b">>") {
            2
        } else if matches!(self.b(p), b'&' | b'^' | b'|') {
            1
        } else {
            return false;
        };
        self.b(p + len) == b'=' && self.plain(p, p + len + 1, &[S::OpAssignBitwise])
    }

    pub(super) fn op_shift(&mut self, p: usize) -> bool {
        (self.starts(p, b"<<") || self.starts(p, b">>")) && self.plain(p, p + 2, &[S::OpShift])
    }

    /// `!=|<=|>=|==|[<>]`.
    pub(super) fn op_compare(&mut self, p: usize) -> bool {
        let lead = self.b(p);
        if matches!(lead, b'!' | b'<' | b'>' | b'=') && self.b(p + 1) == b'=' {
            return self.plain(p, p + 2, &[S::OpCompare]);
        }
        matches!(lead, b'<' | b'>') && self.plain(p, p + 1, &[S::OpCompare])
    }

    /// `&&|!|\|\|`.
    pub(super) fn op_logical(&mut self, p: usize) -> bool {
        if self.starts(p, b"&&") || self.starts(p, b"||") {
            return self.plain(p, p + 2, &[S::OpLogical]);
        }
        self.b(p) == b'!' && self.plain(p, p + 1, &[S::OpLogical])
    }

    pub(super) fn op_bit(&mut self, p: usize) -> bool {
        matches!(self.b(p), b'&' | b'^' | b'|' | b'~') && self.plain(p, p + 1, &[S::Operator])
    }

    pub(super) fn op_assign(&mut self, p: usize) -> bool {
        self.b(p) == b'=' && self.plain(p, p + 1, &[S::OpAssign])
    }

    pub(super) fn op_arith(&mut self, p: usize) -> bool {
        matches!(self.b(p), b'-' | b'%' | b'*' | b'+' | b'/')
            && self.plain(p, p + 1, &[S::Operator])
    }

    /// One byte `c` with `scope`.
    pub(super) fn byte_rule(&mut self, p: usize, c: u8, scope: &'static [S]) -> bool {
        p < self.len_all() && self.b(p) == c && self.plain(p, p + 1, scope)
    }

    /// `(?<!\.)\.\.\.(?!\.)`.
    pub(super) fn vararg(&mut self, p: usize) -> bool {
        self.starts(p, b"...")
            && self.prev(p) != Some(b'.')
            && self.b(p + 3) != b'.'
            && self.plain(p, p + 3, &[S::PunctVararg])
    }

    /// `(?-im:(?<!:):(?!:))`.
    pub(super) fn range_colon(&mut self, p: usize) -> bool {
        self.b(p) == b':'
            && p < self.len_all()
            && self.prev(p) != Some(b':')
            && self.b(p + 1) != b':'
            && self.plain(p, p + 1, &[S::PunctRangeBased])
    }

    /// `(\\)\n`: a backslash at the end of the line.
    pub(super) fn line_cont(&mut self, p: usize) -> bool {
        if self.b(p) != b'\\' || p >= self.len_all() || self.b(p + 1) != b'\n' || p + 1 != self.lim
        {
            return false;
        }
        if !self.nl {
            return false;
        }
        self.cap(p, p + 1, &[S::EscapeContinuation]);
        self.plain(p, p + 2, &[])
    }
}
