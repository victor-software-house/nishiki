//! The type expressions: the `chan` prefixes, type arguments and bracketed
//! lists that the grammar's long expressions repeat.
//! The module also holds the matchers of the type declarations built from
//! them.
//!
//! A `chan` prefix is `(?:\s*CH)+{0,1}` or `(?:CH)+{0,1}`, where CH is
//! `[]*\[]+{0,1}(?:<-\s*)?\bchan\b(?:\s*<-)?\s*`.
//! What follows it in an expression may fail after the longest prefix and
//! match after a shorter one.
//! [`Lexer::after_chans`] therefore tries the ends a backtracking matcher
//! would, in its order.

use super::super::engine::Lexer;
use crate::rules::Chars;

/// Whether each `chan` unit of a prefix may start with white space.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::go) enum Lead {
    /// `(?:\s*CH)`
    Spaces,
    /// `(?:CH)`
    Tight,
}

/// Whether the position a prefix leaves follows at least one unit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::go) enum Prefix {
    Empty,
    Taken,
}

/// How many positions a backward walk keeps on the stack at once.
const CHECKPOINTS: usize = 16;

impl Lexer<'_> {
    /// Visits the `count` positions of a chain from `first`, the last first,
    /// with each one's index.
    /// It returns the first visit that matches.
    /// `step` gives the position after one.
    ///
    /// The positions are found again from checkpoints, not kept.
    /// A chain longer than [`CHECKPOINTS`] keeps that many evenly spaced
    /// positions.
    /// It walks each stretch between them the same way, from the last.
    /// A hostile line therefore costs no allocation.
    /// Its walk stays near linear, because each level of checkpoints walks
    /// the chain once.
    fn backwards<T>(
        &self,
        first: usize,
        count: usize,
        index: usize,
        step: &impl Fn(&Self, usize) -> usize,
        visit: &mut impl FnMut(&Self, usize, usize) -> Option<T>,
    ) -> Option<T> {
        let stride = count.div_ceil(CHECKPOINTS).max(1);
        let mut marks = [0; CHECKPOINTS];
        let mut q = first;
        let stretches = count.div_ceil(stride);
        for (n, mark) in marks.iter_mut().enumerate().take(stretches) {
            *mark = q;
            if n + 1 < stretches {
                for _ in 0..stride {
                    q = step(self, q);
                }
            }
        }
        for n in (0..stretches).rev() {
            let len = if n + 1 == stretches {
                count - n * stride
            } else {
                stride
            };
            let at = index + n * stride;
            let found = if stride == 1 {
                visit(self, at, marks[n])
            } else {
                self.backwards(marks[n], len, at, step, visit)
            };
            if found.is_some() {
                return found;
            }
        }
        None
    }

    /// One `chan` unit at `p`: where it ends with the arrow after `chan`
    /// taken, and where it ends without it when there is one.
    ///
    /// The marks before `<-` or `chan` are a run of characters neither can
    /// start with.
    /// They are therefore taken whole.
    fn chan_unit(&self, p: usize, lead: Lead) -> Option<(usize, Option<usize>)> {
        let start = match lead {
            Lead::Spaces => self.spaces(p),
            Lead::Tight => p,
        };
        let mut i = self.marks(start);
        if self.starts(i, b"<-") {
            i = self.spaces(i + 2);
        }
        let word = self.keyword(i, b"chan")?;
        let blank = self.spaces(word);
        if self.starts(blank, b"<-") {
            Some((self.spaces(blank + 2), Some(blank)))
        } else {
            Some((blank, None))
        }
    }

    /// The end of the longest `chan` prefix at `p`: the one an expression
    /// takes when nothing after the prefix can fail.
    pub(in crate::go) fn chans(&self, p: usize, lead: Lead) -> usize {
        let mut q = p;
        while let Some((end, _)) = self.chan_unit(q, lead) {
            q = end;
        }
        q
    }

    /// Tries `rest` after a `chan` prefix at `p` at each end the prefix can
    /// have, in a backtracking matcher's order.
    /// It returns the first match.
    ///
    /// The matcher takes each unit with its arrow, and as many units as it
    /// can.
    /// It gives them back from the last.
    /// A unit's ends without its arrow, and with less of its white space,
    /// lead `rest` to the places the longer ends did.
    /// The exception is the end before the arrow.
    /// It is tried after the units that follow it.
    ///
    /// White space given back leaves `rest` before white space.
    /// There `rest` fails, or skips to where it was.
    pub(in crate::go) fn after_chans<T>(
        &self,
        p: usize,
        lead: Lead,
        rest: impl Fn(&Self, usize, Prefix) -> Option<T>,
    ) -> Option<T> {
        let mut count = 0;
        let mut q = p;
        while let Some((end, _)) = self.chan_unit(q, lead) {
            count += 1;
            q = end;
        }
        let units = |count: usize| {
            if count > 0 {
                Prefix::Taken
            } else {
                Prefix::Empty
            }
        };
        if let Some(found) = rest(self, q, units(count)) {
            return Some(found);
        }
        let step =
            |lx: &Self, start: usize| lx.chan_unit(start, lead).map_or(start, |(end, _)| end);
        self.backwards(p, count, 0, &step, &mut |lx, index, start| {
            let before_arrow = lx.chan_unit(start, lead).and_then(|(_, before)| before);
            before_arrow
                .and_then(|at| rest(lx, at, Prefix::Taken))
                .or_else(|| rest(lx, start, units(index)))
        })
    }

    /// Whether `(?!B\b(?:words)\b)` holds at `p`, with B the marks.
    pub(in crate::go) fn not_marked_word(&self, p: usize, words: &[&[u8]]) -> bool {
        self.word_in(self.marks(p), words).is_none()
    }

    /// Tries `rest` at each end of the type arguments
    /// `(?:\[(?:K+(?:,\s*K+)*)?])?` at `p`, with K `[]*.\[{}\w]`, in a
    /// backtracking matcher's order.
    /// It returns the first match.
    ///
    /// Each item takes the whole run of K, `]` included.
    /// The list therefore closes at a `]` given back from the last item,
    /// tried from the right.
    /// The earlier items reach a comma only whole.
    pub(in crate::go) fn after_type_args<T>(
        &self,
        p: usize,
        rest: impl Fn(&Self, usize) -> Option<T>,
    ) -> Option<T> {
        if p < self.lim() && self.at(p) == b'[' {
            let open = p + 1;
            let item_end = |lx: &Self, start: usize| {
                lx.words_and(start, |b| {
                    matches!(b, b']' | b'*' | b'.' | b'[' | b'{' | b'}')
                })
            };
            let mut count = 0;
            let mut start = open;
            loop {
                let end = item_end(self, start);
                if end == start {
                    break;
                }
                count += 1;
                if !(end < self.lim() && self.at(end) == b',') {
                    break;
                }
                start = self.spaces(end + 1);
            }
            let step = |lx: &Self, start: usize| lx.spaces(item_end(lx, start) + 1);
            let found = self.backwards(open, count, 0, &step, &mut |lx, _, first| {
                let end = item_end(lx, first);
                (first + 1..end)
                    .rev()
                    .filter(|&close| lx.at(close) == b']')
                    .find_map(|close| rest(lx, close + 1))
            });
            if found.is_some() {
                return found;
            }
            if open < self.lim()
                && self.at(open) == b']'
                && let Some(found) = rest(self, open + 1)
            {
                return Some(found);
            }
        }
        rest(self, p)
    }

    /// The end of the bracketed list
    /// `\[(?:[*.\w]+{0,1}(?:\[[^]]+{0,1}])?(?:,\s+)?)+]` at `p`.
    ///
    /// No part of an item can take a `]` except the inner brackets.
    /// Those close at the first `]`.
    /// The items are therefore taken whole.
    /// The list closes where they stop, or not at all.
    pub(in crate::go) fn type_list(&self, p: usize) -> Option<usize> {
        if p >= self.lim() || self.at(p) != b'[' {
            return None;
        }
        let mut i = p + 1;
        loop {
            let start = i;
            i = self.star_dot_words(i);
            if i < self.lim()
                && self.at(i) == b'['
                && let Some(close) = self.find_byte(i + 1, b']')
            {
                i = close + 1;
            }
            if i < self.lim() && self.at(i) == b',' {
                let after = self.spaces(i + 1);
                if after > i + 1 {
                    i = after;
                }
            }
            if i == start {
                break;
            }
        }
        (i < self.lim() && self.at(i) == b']').then_some(i + 1)
    }

    /// The end of `\([^)]+{0,1}\)` at `p`.
    pub(in crate::go) fn parens(&self, p: usize) -> Option<usize> {
        if p >= self.lim() || self.at(p) != b'(' {
            return None;
        }
        self.find_byte(p + 1, b')').map(|close| close + 1)
    }

    /// The end of `\([^)]+\)` at `p`: the same with something inside.
    pub(in crate::go) fn filled_parens(&self, p: usize) -> Option<usize> {
        self.parens(p).filter(|&end| end > p + 2)
    }

    /// `(])((?:CH)+{0,1}(?!B\b(?:func|struct|map)\b)B[.\w]+TA)?`, with TA the
    /// type arguments.
    pub(in crate::go) fn map_types_end(&mut self, p: usize) -> Option<usize> {
        if p >= self.lim() || self.at(p) != b']' {
            return None;
        }
        self.cap(1, p, p + 1);
        let value = self.after_chans(p + 1, Lead::Tight, |lx, q, _| {
            if !lx.not_marked_word(q, &[b"func", b"map", b"struct"]) {
                return None;
            }
            let name = lx.marks(q);
            let end = lx.dot_words(name);
            (end > name).then(|| lx.after_type_args(end, |_, at| Some(at)).unwrap_or(end))
        });
        Some(value.map_or(p + 1, |end| {
            self.cap(2, p + 1, end);
            end
        }))
    }

    /// `\b(struct)\b\s*(\{)`
    pub(in crate::go) fn struct_variables_types(&mut self, p: usize) -> Option<usize> {
        self.keyword_and_open(p, b"struct", b'{')
    }

    /// `\b(interface)\b\s*(\{)`
    pub(in crate::go) fn interface_variables_types(&mut self, p: usize) -> Option<usize> {
        self.keyword_and_open(p, b"interface", b'{')
    }

    /// `\b(type)\b\s*(\()`
    pub(in crate::go) fn multi_types(&mut self, p: usize) -> Option<usize> {
        self.keyword_and_open(p, b"type", b'(')
    }

    /// `\b(word)\b\s*(open)`
    fn keyword_and_open(&mut self, p: usize, word: &[u8], open: u8) -> Option<usize> {
        let end = self.keyword(p, word)?;
        let at = self.spaces(end);
        if at >= self.lim() || self.at(at) != open {
            return None;
        }
        self.cap(1, p, end);
        self.cap(2, at, at + 1);
        Some(at + 1)
    }

    /// `([*.\w]+)?(\[)`
    pub(in crate::go) fn parameter_variable_types_at2(&mut self, p: usize) -> Option<usize> {
        let end = self.star_dot_words(p);
        if end >= self.lim() || self.at(end) != b'[' {
            return None;
        }
        if end > p {
            self.cap(1, p, end);
        }
        self.cap(2, end, end + 1);
        Some(end + 1)
    }

    /// `(?<=\.\()(?:\b(type)\b|((?:\s*CH)+{0,1}B[.\w]+TA))(?=\))`
    pub(in crate::go) fn type_assertion_inline(&mut self, p: usize) -> Option<usize> {
        if p < 2 || self.at(p - 2) != b'.' || self.at(p - 1) != b'(' || p > self.lim() {
            return None;
        }
        if let Some(end) = self.keyword(p, b"type")
            && end < self.lim()
            && self.at(end) == b')'
        {
            self.cap(1, p, end);
            return Some(end);
        }
        let end = self.after_chans(p, Lead::Spaces, |lx, q, _| {
            let name = lx.marks(q);
            let end = lx.dot_words(name);
            if end == name {
                return None;
            }
            lx.after_type_args(end, |lx, at| {
                (at < lx.lim() && lx.at(at) == b')').then_some(at)
            })
        })?;
        self.cap(2, p, end);
        Some(end)
    }

    /// ```text
    /// ^\s*\b(type)\b\s*([*.\w]+)\s+(?!(?:=\s*)?B\b(?:struct|interface)\b)([\s\S]+)
    /// ```
    ///
    /// When `struct` or `interface` follows the blanks, the matcher gives
    /// one blank back.
    /// It starts the last group on that blank, where the lookahead cannot
    /// match.
    pub(in crate::go) fn single_type_at0(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let word = self.spaces(p);
        let end = self.keyword(word, b"type")?;
        let name = self.spaces(end);
        let name_end = self.star_dot_words(name);
        if name_end == name {
            return None;
        }
        let blank = self.spaces(name_end);
        if blank == name_end {
            return None;
        }
        let declares = |lx: &Self, at: usize| {
            let after = if at < lx.lim() && lx.at(at) == b'=' {
                lx.spaces(at + 1)
            } else {
                at
            };
            !lx.not_marked_word(after, &[b"interface", b"struct"])
        };
        let mut start = blank;
        while start > name_end && (!self.avail(start) || declares(self, start)) {
            start = self.char_start_before(start);
        }
        if start == name_end {
            return None;
        }
        self.cap(1, word, end);
        self.cap(2, name, name_end);
        self.cap(3, start, self.text_end());
        Some(self.text_end())
    }

    /// `(?:^|\s+)\b(type)\b\s*([*.\w]+)(?=\[)`
    pub(in crate::go) fn single_type_at1(&mut self, p: usize) -> Option<usize> {
        let at_start = (p == 0).then(|| self.keyword(p, b"type")).flatten();
        let (word, end) = if let Some(end) = at_start {
            (p, end)
        } else {
            let word = self.spaces(p);
            if word == p {
                return None;
            }
            (word, self.keyword(word, b"type")?)
        };
        let name = self.spaces(end);
        let name_end = self.star_dot_words(name);
        if name_end == name || name_end >= self.lim() || self.at(name_end) != b'[' {
            return None;
        }
        self.cap(1, word, end);
        self.cap(2, name, name_end);
        Some(name_end)
    }

    /// ```text
    /// (?<=])(\s+(?:=\s*)?(?:CH)+{0,1}(?!B\b(?:struct|interface|func)\b)[-\]*.\[\w]+(?:,\s*[]*.\[\w]+)*)?
    /// ```
    pub(in crate::go) fn single_type_at1_end(&mut self, p: usize) -> Option<usize> {
        if p == 0 || p > self.lim() || self.at(p - 1) != b']' {
            return None;
        }
        let blank = self.spaces(p);
        if blank == p {
            return Some(p);
        }
        let start = if blank < self.lim() && self.at(blank) == b'=' {
            self.spaces(blank + 1)
        } else {
            blank
        };
        let end = self.after_chans(start, Lead::Tight, |lx, q, _| {
            if !lx.not_marked_word(q, &[b"func", b"interface", b"struct"]) {
                return None;
            }
            let mut end = lx.words_and(q, |b| matches!(b, b'-' | b']' | b'*' | b'.' | b'['));
            if end == q {
                return None;
            }
            while end < lx.lim() && lx.at(end) == b',' {
                let item = lx.spaces(end + 1);
                let item_end = lx.type_run(item);
                if item_end == item {
                    break;
                }
                end = item_end;
            }
            Some(end)
        });
        Some(end.map_or(p, |end| {
            self.cap(1, p, end);
            end
        }))
    }

    /// `^\s*\b(type)\b\s*([.\w]+)`
    pub(in crate::go) fn struct_interface_declaration(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let word = self.spaces(p);
        let end = self.keyword(word, b"type")?;
        let name = self.spaces(end);
        let name_end = self.dot_words(name);
        if name_end == name {
            return None;
        }
        self.cap(1, word, end);
        self.cap(2, name, name_end);
        Some(name_end)
    }

    /// `(?<!\w)(\(B[.\w]+TA\))(?=\()`
    pub(in crate::go) fn double_parentheses_types(&mut self, p: usize) -> Option<usize> {
        if self.word_before(p) || p >= self.lim() || self.at(p) != b'(' {
            return None;
        }
        let name = self.marks(p + 1);
        let name_end = self.dot_words(name);
        if name_end == name {
            return None;
        }
        let end = self.after_type_args(name_end, |lx, at| {
            let closed = at + 1 < lx.lim() && lx.at(at) == b')' && lx.at(at + 1) == b'(';
            closed.then_some(at + 1)
        })?;
        self.cap(1, p, end);
        Some(end)
    }

    /// `([*.\w]+)(\[[^]]+{0,1}])`
    pub(in crate::go) fn generic_types(&mut self, p: usize) -> Option<usize> {
        let name_end = self.star_dot_words(p);
        if name_end == p || name_end >= self.lim() || self.at(name_end) != b'[' {
            return None;
        }
        let close = self.find_byte(name_end + 1, b']')?;
        self.cap(1, p, name_end);
        self.cap(2, name_end, close + 1);
        Some(close + 1)
    }
}
