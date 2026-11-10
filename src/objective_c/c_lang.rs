//! The C part of the grammar, its `c_lang` entry: member access, calls,
//! parameters, brackets and comments.

use std::cell::Cell;
use std::cmp::Ordering;

use super::engine::ObjcMemory;
use super::words;
use crate::rules::{Chars, Grammar, Lexer};

/// The words before `(` that the Objective-C call pattern `CFunctionsAt1`
/// does not take:
/// `(while|for|do|if|else|switch|catch|enumerate|return|r?iterate)`.
const CALL_KEYWORDS: &[&[u8]] = &[
    b"while",
    b"for",
    b"do",
    b"if",
    b"else",
    b"switch",
    b"catch",
    b"enumerate",
    b"return",
    b"riterate",
    b"iterate",
];

/// The words of `(?:and|and_eq|…|alignas)` in `CBlockInnardsAt6`, in the
/// grammar's order.
const OPERATOR_WORDS: &[&[u8]] = &[
    b"and", b"and_eq", b"bitand", b"bitor", b"compl", b"not", b"not_eq", b"or", b"or_eq",
    b"typeid", b"xor", b"xor_eq", b"alignof", b"alignas",
];

/// The last name of a member access chain: a method before its `(`, or a
/// member.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Last {
    /// `([A-Z_a-z]\w*)(\()`.
    Method,
    /// `\b((?!excluded)[A-Z_a-z]\w*\b(?!\())`.
    Member,
}

impl<G: Grammar> Lexer<'_, G>
where
    G::Extra: ObjcMemory,
{
    /// The lists the word `start..end` belongs to, as bits of [`words`].
    ///
    /// About fifteen patterns test the same word at the same place, each
    /// against its own list.
    /// The last answer is kept, keyed by the word's span and the text's
    /// length, so the table is searched once per word.
    pub(super) fn lists(&self, start: usize, end: usize) -> u32 {
        let (last_start, last_end, len, mask) = self.x.memory().listed.get();
        if (last_start, last_end, len) == (start, end, self.lim()) {
            return mask;
        }
        let mask = self.search_words(self.text(start, end));
        self.x.memory().listed.set((start, end, self.lim(), mask));
        mask
    }

    /// The lists the word `text` belongs to, searched in the sorted table.
    fn search_words(&self, text: &[u8]) -> u32 {
        self.spend(12);
        let word = |at: usize| {
            let start = at
                .checked_sub(1)
                .map_or(0, |before| usize::from(words::WORD_ENDS[before]));
            words::WORD_BYTES
                .get(start..usize::from(words::WORD_ENDS[at]))
                .unwrap_or_default()
        };
        let (mut low, mut high) = (0, words::WORD_ENDS.len());
        while low < high {
            let mid = low + (high - low) / 2;
            match word(mid).cmp(text) {
                Ordering::Less => low = mid + 1,
                Ordering::Greater => high = mid,
                Ordering::Equal => return words::WORD_LISTS[mid],
            }
        }
        0
    }

    /// The whole word at `i`, with `\b` before it, when it is in one of the
    /// lists `mask`: where it ends.
    pub(super) fn listed_word(&self, i: usize, mask: u32) -> Option<usize> {
        if !self.boundary(i) || self.word_at(i).is_none() {
            return None;
        }
        let end = self.word_end(i);
        (self.lists(i, end) & mask != 0).then_some(end)
    }

    /// `(\s*)\b(list)\b`: blanks, recorded as group 1, and then a word of the
    /// lists `mask`, recorded as group 2.
    pub(super) fn spaced_word(&mut self, p: usize, mask: u32) -> Option<usize> {
        let start = self.space_end(p);
        let end = self.listed_word(start, mask)?;
        self.capture(1, p, start);
        self.capture(2, start, end);
        Some(end)
    }

    /// `[A-Z_a-z]\w*` at `i`: where it ends.
    pub(super) fn ident_end(&self, i: usize) -> Option<usize> {
        let b = self.at(i);
        ((b == b'_' || b.is_ascii_alphabetic()) && i < self.lim())
            .then(|| self.cached_run(i, &self.x.memory().word_run, Self::word_end))
    }

    /// `[A-Z_a-z][0-9A-Z_a-z]*` at `i`: where it ends.
    pub(crate) fn ascii_ident_end(&self, i: usize) -> Option<usize> {
        let b = self.at(i);
        ((b == b'_' || b.is_ascii_alphabetic()) && i < self.lim()).then(|| {
            self.cached_run(i, &self.x.memory().ascii_run, |lexer, from| {
                lexer.skip(from, |c| c == b'_' || c.is_ascii_alphanumeric())
            })
        })
    }

    /// The end of the run from `i` that `measure` finds, remembered in
    /// `cache`.
    /// A run measured from any of its characters ends at the same place, so a
    /// later question from inside the last run answers at once.
    fn cached_run(
        &self,
        i: usize,
        cache: &Cell<(usize, usize, usize)>,
        measure: fn(&Self, usize) -> usize,
    ) -> usize {
        let (start, end, len) = cache.get();
        if len == self.lim() && start <= i && i < end {
            return end;
        }
        let end = measure(self, i);
        cache.set((i, end, self.lim()));
        end
    }

    /// `\.\*?` or `->\*?` at `i`: where it ends, and whether it is the arrow.
    fn access_op(&self, i: usize) -> Option<(usize, bool)> {
        let (len, arrow) = match self.at(i) {
            b'.' if i < self.lim() => (1, false),
            b'-' if self.at(i + 1) == b'>' => (2, true),
            _ => return None,
        };
        let star = self.at(i + len) == b'*' && i + len < self.lim();
        Some((i + len + usize::from(star), arrow))
    }

    /// `((?:[A-Z_a-z]\w*|(?<=[])]))\s*)`: where the object and the blanks
    /// after it end.
    fn access_object(&self, p: usize) -> Option<usize> {
        if let Some(end) = self.ident_end(p) {
            return Some(self.space_end(end));
        }
        matches!(self.prev(p), Some(b']' | b')')).then(|| self.space_end(p))
    }

    /// The final identifier of a member access, which must not start with a
    /// word of [`words::MEMBER_EXCLUDED`]: where it ends.
    fn member_name(&self, q: usize) -> Option<usize> {
        if !self.boundary(q) {
            return None;
        }
        let end = self.ident_end(q)?;
        let mut k = q + 1;
        while k <= end {
            if self.lists(q, k) & words::MEMBER_EXCLUDED != 0 {
                return None;
            }
            k += 1;
        }
        (self.at(end) != b'(' || end >= self.lim()).then_some(end)
    }

    /// The final identifier of a method call, before its `(`: where it ends.
    fn method_name(&self, q: usize) -> Option<usize> {
        let end = self.ident_end(q)?;
        (self.at(end) == b'(' && end < self.lim()).then_some(end)
    }

    /// The chain `object.middle.name` at `p`, recording groups 1 to 5:
    /// where the name ends.
    ///
    /// The repeated objects after the first operator are greedy.
    /// They give one repetition back at a time until the last name matches.
    /// The match therefore ends at the last repetition that the name follows.
    /// One walk forward finds it.
    pub(super) fn access_chain(&mut self, p: usize, last: Last) -> Option<usize> {
        let object_end = self.access_object(p)?;
        let (op_end, arrow) = self.access_op(object_end)?;
        let mut found = None;
        let mut end = op_end;
        loop {
            let name_start = self.space_end(end);
            let name_end = match last {
                Last::Method => self.method_name(name_start),
                Last::Member => self.member_name(name_start),
            };
            if let Some(name_end) = name_end {
                found = Some((end, name_start, name_end));
            }
            let Some(next) = self.ident_end(end) else {
                break;
            };
            let after = self.space_end(next);
            let Some((rep_end, _)) = self.access_op(after) else {
                break;
            };
            end = self.space_end(rep_end);
        }
        let (middle_end, name_start, name_end) = found?;
        self.capture(1, p, object_end);
        self.capture(if arrow { 3 } else { 2 }, object_end, op_end);
        self.capture(4, op_end, middle_end);
        self.capture(5, name_start, name_end);
        Some(name_end)
    }

    /// `((?:[A-Z_a-z]\w*|(?<=[])]))\s*)(?:(\.\*?)|(->\*?))`.
    pub(super) fn access_head(&mut self, p: usize) -> Option<usize> {
        let object_end = self.access_object(p)?;
        let (op_end, arrow) = self.access_op(object_end)?;
        self.capture(1, p, object_end);
        self.capture(if arrow { 3 } else { 2 }, object_end, op_end);
        Some(op_end)
    }

    /// `(?<=operator)(?:[-!\&*+<=>]+|\(\)|\[])` at `i`: where it ends.
    fn operator_name(&self, i: usize) -> Option<usize> {
        if i < 8 || self.text(i - 8, i) != b"operator" {
            return None;
        }
        let j = self.skip(i, |b| {
            matches!(b, b'-' | b'!' | b'&' | b'*' | b'+' | b'<' | b'=' | b'>')
        });
        if j > i {
            return Some(j);
        }
        (self.starts(i, b"()") || self.starts(i, b"[]")).then_some(i + 2)
    }

    /// `(?:[A-Z_a-z][0-9A-Z_a-z]*+|::)++` at `i`: where the run ends.
    ///
    /// A walk can enter at a letter or `_` inside the last walk, in the same
    /// text.
    /// It takes the rest of that name and then the same steps.
    /// It ends where the last walk did.
    fn scoped_name(&self, i: usize) -> Option<usize> {
        let (start, end, len) = self.x.memory().scoped.get();
        let b = self.at(i);
        if len == self.lim() && start < i && i < end && (b == b'_' || b.is_ascii_alphabetic()) {
            return Some(end);
        }
        let mut j = i;
        loop {
            if let Some(end) = self.ascii_ident_end(j) {
                j = end;
            } else if self.starts(j, b"::") {
                j += 2;
            } else {
                break;
            }
        }
        if j == i {
            return None;
        }
        self.x.memory().scoped.set((i, j, self.lim()));
        Some(j)
    }

    /// The name of a call at `p` and where its `(` is:
    /// `(?!(?:excluded)\s*\()((?:name|::)++|operator name)\s*(\()`.
    pub(super) fn call_target(&self, p: usize, excluded: u32) -> Option<(usize, usize)> {
        let name_end = self.scoped_name(p).or_else(|| self.operator_name(p))?;
        let paren = self.space_end(name_end);
        if !(self.at(paren) == b'(' && paren < self.lim()) {
            return None;
        }
        if let Some(word_end) = self.ascii_ident_end(p) {
            let after = self.space_end(word_end);
            if self.at(after) == b'(' && self.lists(p, word_end) & excluded != 0 {
                return None;
            }
        }
        Some((name_end, paren))
    }

    /// `CFunctionCallInnardsAt5`: a call's name, group 1, and its `(`,
    /// group 2.
    pub(super) fn call_innards(&mut self, p: usize) -> Option<usize> {
        let (name_end, paren) = self.call_target(p, words::FUNCTION_EXCLUDED)?;
        self.capture(1, p, name_end);
        self.capture(2, paren, paren + 1);
        Some(paren + 1)
    }

    /// `CFunctionsAt1`: `(?:(?=\s)(?:(?<=else|new|return)|(?<!\w))(\s+))?\b
    /// ((?!(keywords)\s*\()(?:(?!NS)[A-Z_a-z][0-9A-Z_a-z]*+\b|::)++)\s*(\()`.
    pub(super) fn objc_call(&mut self, p: usize) -> Option<usize> {
        let start = if self.space_at(p) > 0 {
            let after_word = ["else", "new", "return"]
                .iter()
                .any(|w| p >= w.len() && self.text(p - w.len(), p) == w.as_bytes());
            if !after_word && self.word_before(p) {
                return None;
            }
            self.space_end(p)
        } else {
            p
        };
        if !self.boundary(start) {
            return None;
        }
        let mut j = start;
        loop {
            if self.starts(j, b"::") {
                j += 2;
                continue;
            }
            if self.starts(j, b"NS") {
                break;
            }
            let Some(end) = self.ascii_ident_end(j) else {
                break;
            };
            if self.word_at(end).is_some() {
                break;
            }
            j = end;
        }
        if j == start {
            return None;
        }
        if let Some(word_end) = self.ascii_ident_end(start) {
            let after = self.space_end(word_end);
            if self.at(after) == b'(' && CALL_KEYWORDS.contains(&self.text(start, word_end)) {
                return None;
            }
        }
        let paren = self.space_end(j);
        if !(self.at(paren) == b'(' && paren < self.lim()) {
            return None;
        }
        if start > p {
            self.capture(1, p, start);
        }
        self.capture(2, start, j);
        self.capture(4, paren, paren + 1);
        Some(paren + 1)
    }

    /// `CBlockInnardsAt6`:
    /// `(?=\s)(?<!else|new|return)(?<=\w)\s+(word)(name)\s*(\()`,
    /// with the words of [`OPERATOR_WORDS`].
    pub(super) fn operator_word_call(&mut self, p: usize) -> Option<usize> {
        if self.space_at(p) == 0 || !self.word_before(p) {
            return None;
        }
        if ["else", "new", "return"]
            .iter()
            .any(|w| p >= w.len() && self.text(p - w.len(), p) == w.as_bytes())
        {
            return None;
        }
        let start = self.space_end(p);
        for word in OPERATOR_WORDS {
            if !self.starts(start, word) {
                continue;
            }
            let name = start + word.len();
            let Some(name_end) = self.scoped_name(name) else {
                continue;
            };
            let paren = self.space_end(name_end);
            if self.at(paren) == b'(' && paren < self.lim() {
                self.capture(1, start, name);
                self.capture(2, name, name_end);
                return Some(paren + 1);
            }
        }
        None
    }

    /// `CLangAt33`: `(?<!\w)(?!\s*(?:excluded)\s*\()(?=[A-Z_a-z]\w*\s*\()`,
    /// which takes no text.
    pub(super) fn call_ahead(&self, p: usize) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        let end = self.ident_end(p)?;
        let paren = self.space_end(end);
        if !(self.at(paren) == b'(' && paren < self.lim()) {
            return None;
        }
        (self.lists(p, end) & words::CALL_EXCLUDED == 0).then_some(p)
    }

    /// `CProbablyAParameter`: `(?<=[0-9A-Z_a-z] |[]\&)*>])\s*([A-Z_a-z]\w*)\s*
    /// (?=(?:\[]\s*)?[),])`.
    pub(super) fn parameter(&mut self, p: usize) -> Option<usize> {
        let after_word = p >= 2 && self.at(p - 1) == b' ' && {
            let b = self.at(p - 2);
            b == b'_' || b.is_ascii_alphanumeric()
        };
        if !(after_word || matches!(self.prev(p), Some(b']' | b'&' | b')' | b'*' | b'>'))) {
            return None;
        }
        let start = self.space_end(p);
        let end = self.ident_end(start)?;
        let after = self.space_end(end);
        let ok = match self.at(after) {
            b')' | b',' => after < self.lim(),
            b'[' if self.at(after + 1) == b']' => {
                matches!(self.at(self.space_end(after + 2)), b')' | b',')
            }
            _ => false,
        };
        if !ok {
            return None;
        }
        self.capture(1, start, end);
        Some(after)
    }

    /// `CLangAt35`: `([A-Z_a-z][0-9A-Z_a-z]*|(?<=[])]))?(\[)(?!])`.
    pub(super) fn bracket_begin(&mut self, p: usize) -> Option<usize> {
        let open = self.ascii_ident_end(p).unwrap_or(p);
        if self.at(open) != b'[' || open >= self.lim() || self.at(open + 1) == b']' {
            return None;
        }
        if open > p || matches!(self.prev(p), Some(b']' | b')')) {
            self.capture(1, p, open);
        }
        self.capture(2, open, open + 1);
        Some(open + 1)
    }

    /// `\b[ksg][A-Z]\w*\b` with the first letter `first`.
    pub(crate) fn prefixed_name(&self, p: usize, first: u8) -> Option<usize> {
        (self.boundary(p) && self.at(p) == first && self.at(p + 1).is_ascii_uppercase())
            .then(|| self.word_end(p + 2))
    }

    /// `CLangAt30`: `\b([0-9A-Z_a-z]+_t)\b`.
    pub(super) fn underscore_t(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let end = self.skip(p, |b| b == b'_' || b.is_ascii_alphanumeric());
        (end >= p + 3 && self.text(end - 2, end) == b"_t" && self.word_at(end).is_none())
            .then_some(end)
    }

    /// `^/\* =(\s*.*?)\s*= \*/$\n?`, a banner comment, recording group 1.
    pub(super) fn banner_block(&mut self, p: usize) -> Option<usize> {
        if p != 0 || !self.starts(0, b"/* =") {
            return None;
        }
        let close = self.lim().checked_sub(4)?;
        if close < 4 || self.text(close, self.lim()) != b"= */" {
            return None;
        }
        let inner = self.blank_end(4).min(close);
        let trailing = self.blank_start_before(close).max(inner);
        self.capture(1, 4, trailing);
        Some(self.space_end(self.lim()).min(self.lim() + 1))
    }

    /// `^// =(\s*.*?)\s*=\s*$\n?`, a banner line, recording group 1.
    pub(super) fn banner_line(&mut self, p: usize) -> Option<usize> {
        if p != 0 || !self.starts(0, b"// =") {
            return None;
        }
        let last = self.blank_start_before(self.lim());
        if last < 5 || self.at(last - 1) != b'=' {
            return None;
        }
        let close = last - 1;
        let inner = self.blank_end(4).min(close);
        let trailing = self.blank_start_before(close).max(inner);
        self.capture(1, 4, trailing);
        Some(self.space_end(close + 1))
    }

    /// Where the blanks that end at `i` start.
    fn blank_start_before(&self, i: usize) -> usize {
        let mut start = i;
        while start > 0 {
            let before = self.char_start_before(start);
            match self.char_at(before) {
                Some((c, _)) if crate::classes::is_space(c) => start = before,
                _ => break,
            }
        }
        start
    }
}
