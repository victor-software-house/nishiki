//! Hand-written matchers for the expressions that would run most often
//! through the backtracking matcher: they start at every word or every
//! position of a line.

use super::super::engine::Lexer;
use crate::rules::{CachedRuns, Chars};

/// The characters of `[-\]!%*+./:<=>\[_[:alnum:]]`.
fn control_char(c: char) -> bool {
    matches!(
        c,
        '-' | ']' | '!' | '%' | '*' | '+' | '.' | '/' | ':' | '<' | '=' | '>' | '[' | '_'
    ) || c.is_alphanumeric()
}

impl Lexer<'_> {
    /// The text `word` ends at `p`, with a `\b` on both sides.
    pub(in crate::go) fn word_ends_at(&self, p: usize, word: &[u8]) -> bool {
        let Some(start) = p.checked_sub(word.len()) else {
            return false;
        };
        self.text(start, p) == word && !self.word_before(start) && self.word_at(p).is_none()
    }

    /// `(?<=\brange\b|;|\bif\b|\bfor\b|[<>]|<=|>=|==|!=|\w[-%*+/]|
    /// \w[-%*+/]=|\|\||&&)` at `p`.
    fn after_control_behind(&self, p: usize) -> bool {
        let before = |back: usize| p.checked_sub(back).map(|i| self.at(i));
        let sign = |b: Option<u8>| matches!(b, Some(b'-' | b'%' | b'*' | b'+' | b'/'));
        let word_at_back = |back: usize| {
            p.checked_sub(back)
                .is_some_and(|i| self.word_at(i).is_some() && i < self.lim())
        };
        self.word_ends_at(p, b"range")
            || matches!(before(1), Some(b';' | b'<' | b'>'))
            || self.word_ends_at(p, b"if")
            || self.word_ends_at(p, b"for")
            || (before(1) == Some(b'=') && matches!(before(2), Some(b'<' | b'>' | b'=' | b'!')))
            || (sign(before(1)) && word_at_back(2))
            || (before(1) == Some(b'=') && sign(before(2)) && word_at_back(3))
            || (before(1) == Some(b'|') && before(2) == Some(b'|'))
            || (before(1) == Some(b'&') && before(2) == Some(b'&'))
    }

    /// `(?<=\brange\b|;|...)\s*((?![]\[]+)[-\]!%*+./:<=>\[_[:alnum:]]+)\s*
    /// (?=\{)`
    ///
    /// The run and the blanks around it share no character, so each
    /// quantifier has one end that lets the rest match.
    pub(in crate::go) fn after_control_variables(&mut self, p: usize) -> Option<usize> {
        if !self.after_control_behind(p) {
            return None;
        }
        let start = self.spaces(p).min(self.lim());
        if matches!(self.at(start), b']' | b'[') {
            return None;
        }
        let mut end = start;
        while let Some((c, next)) = self.char_at(end).filter(|_| end < self.lim()) {
            if !control_char(c) {
                break;
            }
            end = next;
        }
        if end == start {
            return None;
        }
        let after = self.spaces(end).min(self.lim());
        (self.at(after) == b'{' && after < self.lim()).then(|| {
            self.cap(1, start, end);
            after
        })
    }

    /// The end of the balanced `\[(?:[^]\[]|\g<brackets>)*]` at `p`.
    fn balanced_brackets(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'[' || p >= self.lim() {
            return None;
        }
        let mut depth = 0_u32;
        let mut i = p;
        while i < self.lim() {
            match self.at(i) {
                b'[' => depth += 1,
                b']' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(i + 1);
                    }
                }
                _ => {}
            }
            i += 1;
        }
        None
    }

    /// `\b(?!(?:struct|interface)\b)([.\w]+)(?<brackets>\[(?:[^]\[]|\
    /// g<brackets>)*])?(?=\{)`
    pub(in crate::go) fn other_struct_interface_expressions_at1(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        if self.word_before(p) == self.word_at(p).is_some() {
            return None;
        }
        let end = self.run_of(4, p, |lx, i| lx.at(i) == b'.' || lx.word_at(i).is_some());
        if end == p {
            return None;
        }
        if self.keyword(p, b"struct").is_some() || self.keyword(p, b"interface").is_some() {
            return None;
        }
        let brackets = self.balanced_brackets(end);
        let after = brackets.unwrap_or(end);
        (self.at(after) == b'{' && after < self.lim()).then(|| {
            self.cap(1, p, end);
            if let Some(close) = brackets {
                self.cap(2, end, close);
            }
            after
        })
    }

    /// `(?:((?<=\.)\b\w+)|\b(\w+))(?<brackets>\[(?:[^]\[]|\g<brackets>)*])?(?=\
    /// ()`
    pub(in crate::go) fn support_functions(&mut self, p: usize) -> Option<usize> {
        if self.word_before(p) || self.word_at(p).is_none() || p >= self.lim() {
            return None;
        }
        let end = self.word_run(p);
        let brackets = self.balanced_brackets(end);
        let after = brackets.unwrap_or(end);
        if self.at(after) != b'(' || after >= self.lim() {
            return None;
        }
        if p > 0 && self.at(p - 1) == b'.' {
            self.cap(1, p, end);
        } else {
            self.cap(2, p, end);
        }
        if let Some(close) = brackets {
            self.cap(3, end, close);
        }
        Some(after)
    }

    /// The end of the run of characters `class` accepts from `p`, which the
    /// cache `slot` remembers: a search from inside a run ends where it does.
    fn run_of(&self, slot: usize, p: usize, class: impl Fn(&Self, usize) -> bool) -> usize {
        self.sync_runs();
        let (from, to) = self.x.runs[slot].get();
        if from <= p && p < to {
            self.spend(1);
            return to;
        }
        let mut i = p;
        while i < self.lim() && class(self, i) {
            i = self.char_end(i);
        }
        self.spend(i - p + 1);
        self.x.runs[slot].set((p, i));
        i
    }

    /// The end of the comma list `item(?:,\s*item)*` from `p`, where `item`
    /// is a run of characters `class` accepts. `slot` caches the list and
    /// `item_slot` its items.
    fn comma_list(
        &self,
        slot: usize,
        item_slot: usize,
        p: usize,
        class: impl Fn(&Self, usize) -> bool + Copy,
    ) -> usize {
        self.sync_runs();
        let (from, to) = self.x.runs[slot].get();
        if from <= p && p < to {
            self.spend(1);
            return to;
        }
        let mut end = self.run_of(item_slot, p, class);
        loop {
            if self.at(end) != b',' || end >= self.lim() {
                break;
            }
            let next = self.spaces(end + 1).min(self.lim());
            let item_end = self.run_of(item_slot, next, class);
            if item_end == next {
                break;
            }
            end = item_end;
        }
        self.x.runs[slot].set((p, end));
        end
    }

    /// `\b\w+(?:,\s*\w+)*(?=\s*:=)`
    pub(in crate::go) fn variable_assignment_at0(&mut self, p: usize) -> Option<usize> {
        if self.word_before(p) || self.word_at(p).is_none() || p >= self.lim() {
            return None;
        }
        let end = self.comma_list(2, 0, p, |lx: &Self, i: usize| lx.word_at(i).is_some());
        let after = self.spaces(end).min(self.lim());
        self.starts(after, b":=").then_some(end)
    }

    /// `\b[*.\w]+(?:,\s*[*.\w]+)*(?=\s*=(?!=))`
    pub(in crate::go) fn variable_assignment_at1(&mut self, p: usize) -> Option<usize> {
        let in_class =
            |lx: &Self, i: usize| matches!(lx.at(i), b'*' | b'.') || lx.word_at(i).is_some();
        if self.word_before(p) == self.word_at(p).is_some() || p >= self.lim() || !in_class(self, p)
        {
            return None;
        }
        let end = self.comma_list(3, 1, p, in_class);
        let after = self.spaces(end).min(self.lim());
        (self.at(after) == b'=' && after < self.lim() && self.at(after + 1) != b'=').then_some(end)
    }

    /// `\b([.\w]+:(?!=))`
    pub(in crate::go) fn property_variables(&mut self, p: usize) -> Option<usize> {
        let in_class = |lx: &Self, i: usize| lx.at(i) == b'.' || lx.word_at(i).is_some();
        if self.word_before(p) == self.word_at(p).is_some() || p >= self.lim() || !in_class(self, p)
        {
            return None;
        }
        let end = self.run_of(4, p, in_class);
        (self.at(end) == b':' && end < self.lim() && self.at(end + 1) != b'=').then(|| {
            self.cap(1, p, end + 1);
            end + 1
        })
    }
}
