//! Parameters of a function: the expressions that take storage words and a
//! type name before a `,`, `)` or `=`.

use super::super::engine::Lexer;
use super::gaps::{Gap, GapForm};
use super::heads::STORAGE;
use super::qualified::{Layout, TypeMatch};
use crate::rules::Chars;

/// The last iteration of the storage loop: the word and the gap after it.
type Last = (usize, usize, Gap);

/// The type name of a parameter: its group, start and end, and the gap
/// after it.
type Named = (usize, usize, usize, Gap);

/// What follows the pointer marks of a function pointer parameter: the
/// `(`, the name, the last `[…]`, the `)` and the `(` of the parameters.
#[derive(Clone, Copy)]
struct PointerTail {
    open: usize,
    name_at: usize,
    name_end: usize,
    brackets: Option<(usize, usize)>,
    close: usize,
    call: usize,
}

impl Lexer<'_> {
    /// `(?:(WORD)GAP)+` at `p`, every way, in the order the expression tries
    /// them: the first for which `rest` matches what follows, given where the
    /// loop ends and its last iteration.
    ///
    /// Each iteration tries a later one first, then itself alone.
    fn find_storage_loop<R>(
        &self,
        p: usize,
        rest: &mut impl FnMut(&Self, usize, Last) -> Option<R>,
    ) -> Option<R> {
        let _depth = self.deeper()?;
        for word in STORAGE {
            let Some(end) = self.lit(p, word) else {
                continue;
            };
            for gap in self.gaps(end) {
                if gap.end > p
                    && let Some(found) = self.find_storage_loop(gap.end, rest)
                {
                    return Some(found);
                }
                if let Some(found) = rest(self, gap.end, (p, end, gap)) {
                    return Some(found);
                }
            }
        }
        None
    }

    /// `(?<!(?:\W|^)(?:STORAGE))` at `end`: no storage word ends there.
    fn behind_storage(&self, end: usize) -> bool {
        STORAGE.iter().any(|word| {
            self.ends_with_at(end, word)
                && (end == word.len() || !self.word_before(end - word.len()))
        })
    }

    /// A storage loop, a gap, then a builtin type or another name that is no
    /// storage word, a gap and a `,`, `)` or `=` ahead.
    pub(super) fn parameter_storage(&mut self, p: usize) -> Option<usize> {
        let (loop_end, last, second, (group, a, e, third)) =
            self.find_storage_loop(p, &mut |lexer, loop_end, last| {
                lexer.gaps(loop_end).find_map(|second| {
                    Some((loop_end, last, second, lexer.parameter_type(second.end)?))
                })
            })?;
        self.cap(1, p, loop_end);
        self.cap(2, last.0, last.1);
        self.put_gap(3, GapForm::Parts, last.1, &last.2);
        self.put_gap(7, GapForm::Parts, loop_end, &second);
        self.cap(group, a, e);
        self.put_gap(16, GapForm::Parts, e, &third);
        Some(third.end)
    }

    /// The type name of [`Lexer::parameter_storage`] at `at`: each builtin
    /// type, then the longest name that is no storage word, with a gap and a
    /// `,`, `)` or `=` after it.
    fn parameter_type(&self, at: usize) -> Option<Named> {
        let builtin = self
            .builtin_types(at)
            .into_iter()
            .map(|(group, a, e)| (10 + group, a, e));
        let other = (!self.word_before(at))
            .then(|| {
                self.ident_ends(at).find(|&name| {
                    self.word_at(name).is_none()
                        && self.boundary(name)
                        && !self.behind_storage(name)
                })
            })
            .flatten()
            .map(|name| (15, at, name));
        builtin.chain(other).find_map(|(group, a, e)| {
            let third = self.gaps(e).find(|gap| {
                gap.end < self.lim() && matches!(self.at(gap.end), b')' | b',' | b'=')
            })?;
            Some((group, a, e, third))
        })
    }

    /// A qualified type, pointer marks and `(*name[]…)(` of a function
    /// pointer parameter.
    pub(super) fn function_pointer_parameter(&mut self, p: usize) -> Option<usize> {
        let layout = Layout {
            attribute: Some(2),
            gap_a: 3,
            gap_b: 7,
            scope: 11,
            scope_template: (12, 13),
            gap_c: 14,
            template: 18,
            gap_form: GapForm::Parts,
        };
        let start = self.skip_space(p);
        let (found, run, at, gap, tail) = self.find_type(start, &layout, |lexer, found| {
            let found: TypeMatch = *found;
            lexer.find_pointer_run(found.end, |lexer, run| {
                let at = run.map_or(found.end, |r| r.0);
                lexer.gaps(at).find_map(|gap| {
                    Some((found, run, at, gap, lexer.function_pointer_tail(gap.end)?))
                })
            })
        })?;
        let after = found.end;
        self.put_type(&layout, &found);
        self.cap(1, p, after);
        if let Some((end, first, last)) = run {
            self.cap(19, after, end);
            if let Some(g) = first {
                self.put_gap(20, GapForm::Parts, after, &g);
            }
            if let Some((from, g)) = last {
                self.put_gap(24, GapForm::Parts, from, &g);
            }
        }
        self.put_gap(28, GapForm::Parts, at, &gap);
        let open = tail.open;
        self.cap(32, open, open + 1);
        self.cap(33, open + 1, open + 2);
        self.cap(34, tail.name_at, tail.name_end);
        if let Some((a, z)) = tail.brackets {
            self.cap(35, a, a + 1);
            self.cap(36, a + 1, z);
            self.cap(37, z, z + 1);
        }
        self.cap(38, tail.close, tail.close + 1);
        self.cap(39, tail.call, tail.call + 1);
        Some(tail.call + 1)
    }

    /// `\(\*\s*(ID)?\s*(?:(\[)(\w*)(\])\s*)*(\))\s*(\()` at `open`: the name is
    /// its longest, and the brackets are taken while they close.
    fn function_pointer_tail(&self, open: usize) -> Option<PointerTail> {
        if open >= self.lim() || self.at(open) != b'(' {
            return None;
        }
        if self.at(open + 1) != b'*' || open + 1 >= self.lim() {
            return None;
        }
        let name_at = self.skip_space(open + 2);
        let name_end = self.ident_ends(name_at).longest().unwrap_or(name_at);
        let mut close = self.skip_space(name_end);
        let mut brackets = None;
        while close < self.lim() && self.at(close) == b'[' {
            let word = self.word_end(close + 1);
            if word >= self.lim() || self.at(word) != b']' {
                break;
            }
            brackets = Some((close, word));
            close = self.skip_space(word + 1);
        }
        if close >= self.lim() || self.at(close) != b')' {
            return None;
        }
        let call = self.skip_space(close + 1);
        if call >= self.lim() || self.at(call) != b'(' {
            return None;
        }
        Some(PointerTail {
            open,
            name_at,
            name_end,
            brackets,
            close,
            call,
        })
    }
}
