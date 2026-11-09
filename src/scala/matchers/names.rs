//! The name and operator classes that most of the grammar's expressions share.
//!
//! The grammar spells them out again in each expression:
//! - `idrest`: `[$0-9A-Z_a-z\p{Lt}\p{Lu}\p{Lo}\p{Nl}\p{Ll}]*`, then
//!   `(?:(?<=_)[op]+)?`;
//! - a plain name: `[$A-Z_a-z\p{Lt}\p{Lu}\p{Lo}\p{Nl}\p{Ll}]` and `idrest`;
//! - an operator name: `[op]+`;
//! - a quoted name: `` `[^`]+` ``.
//!
//! `op` is `[-!#%\&*+/:<-@^|~\p{Sm}\p{So}]`.

use super::super::engine::Lexer;
use crate::classes::{is_lower_or_other_letter, is_symbol, is_title_or_upper};
use crate::rules::Chars;

/// Whether `c` is an operator character, `[-!#%\&*+/:<-@^|~\p{Sm}\p{So}]`.
pub(in crate::scala) fn is_op(c: char) -> bool {
    matches!(
        c,
        '-' | '!'
            | '#'
            | '%'
            | '&'
            | '*'
            | '+'
            | '/'
            | ':'
            | '<'
            | '='
            | '>'
            | '?'
            | '@'
            | '^'
            | '|'
            | '~'
    ) || is_symbol(c)
}

/// Whether `c` is a letter the names take: `[\p{Lt}\p{Lu}\p{Lo}\p{Nl}\p{Ll}]`.
fn is_name_letter(c: char) -> bool {
    is_title_or_upper(c) || is_lower_or_other_letter(c)
}

/// Whether `c` can start a plain name:
/// `[$A-Z_a-z\p{Lt}\p{Lu}\p{Lo}\p{Nl}\p{Ll}]`.
pub(in crate::scala) fn is_name_start(c: char) -> bool {
    c == '$' || c == '_' || is_name_letter(c)
}

/// Whether `c` can follow in a plain name:
/// `[$0-9A-Z_a-z\p{Lt}\p{Lu}\p{Lo}\p{Nl}\p{Ll}]`.
pub(super) fn is_name_rest(c: char) -> bool {
    c.is_ascii_digit() || is_name_start(c)
}

/// Whether `c` can start an upper-case name: `[A-Z\p{Lt}\p{Lu}]`.
pub(in crate::scala) fn is_upper_start(c: char) -> bool {
    is_title_or_upper(c)
}

/// Whether `c` can start a lower-case name: `[$_a-z\p{Lo}\p{Nl}\p{Ll}]`.
pub(super) fn is_lower_start(c: char) -> bool {
    c == '$' || c == '_' || is_lower_or_other_letter(c)
}

impl Lexer<'_> {
    /// The end of the run of characters from `i` that `class` accepts.
    /// Each character counts against the line's work budget.
    pub(super) fn chars_while(&self, mut i: usize, class: impl Fn(char) -> bool) -> usize {
        let from = i;
        while let Some((c, next)) = self.char_at(i) {
            if !class(c) {
                break;
            }
            i = next;
        }
        self.spend(i - from);
        i
    }

    /// The end of `idrest` from `i`: name characters, then operator characters
    /// when the last name character is `_`.
    ///
    /// Both runs are greedy.
    /// The operator run is optional, so a caller that fails after it can
    /// retry at [`Lexer::idrest_letters`].
    pub(super) fn idrest(&self, i: usize) -> usize {
        let letters = self.idrest_letters(i);
        self.op_tail(letters)
    }

    /// The end of the name characters of `idrest` from `i`.
    pub(super) fn idrest_letters(&self, i: usize) -> usize {
        self.chars_while(i, is_name_rest)
    }

    /// `(?:(?<=_)[op]+)?` at `i`: the operator run after a `_`, or `i`.
    pub(super) fn op_tail(&self, i: usize) -> usize {
        if !self.prev_is_byte(i, b'_') {
            return i;
        }
        self.chars_while(i, is_op)
    }

    /// Whether the byte before `i` is `byte`.
    pub(super) fn prev_is_byte(&self, i: usize, byte: u8) -> bool {
        i > 0 && self.s.get(i - 1) == Some(&byte)
    }

    /// The end of a plain name at `i`, `start idrest`, where `start` is the
    /// class of the first character.
    pub(super) fn name_with(&self, i: usize, start: impl Fn(char) -> bool) -> Option<usize> {
        let (c, next) = self.char_at(i)?;
        start(c).then(|| self.idrest(next))
    }

    /// The end of a plain name at `i`.
    pub(in crate::scala) fn plain_name(&self, i: usize) -> Option<usize> {
        self.name_with(i, is_name_start)
    }

    /// The end of an operator name at `i`, `[op]+`.
    pub(in crate::scala) fn op_name(&self, i: usize) -> Option<usize> {
        let end = self.chars_while(i, is_op);
        (end > i).then_some(end)
    }

    /// The end of a quoted name at `i`, `` `[^`]+` ``.
    pub(in crate::scala) fn quoted_name(&self, i: usize) -> Option<usize> {
        if self.s.get(i) != Some(&b'`') {
            return None;
        }
        let rest = self.s.get(i + 1..)?;
        let close = memchr::memchr(b'`', rest)?;
        (close > 0).then_some(i + 1 + close + 1)
    }

    /// The end of a plain or operator name at `i`: `(?:plain|[op]+)`.
    ///
    /// The two alternatives start with different characters, so one decides.
    pub(super) fn id(&self, i: usize) -> Option<usize> {
        self.plain_name(i).or_else(|| self.op_name(i))
    }

    /// The end of a name at `i` with the quoted form after the others:
    /// `(?:plain|[op]+)|` followed by `` `[^`]+` ``.
    pub(super) fn any_name(&self, i: usize) -> Option<usize> {
        self.id(i).or_else(|| self.quoted_name(i))
    }

    /// The first end of a plain name at `i` that `accept` takes.
    /// The ends are tried in the order a backtracking matcher tries them:
    ///
    /// 1. the whole name;
    /// 2. one operator character less at a time;
    /// 3. one name character less at a time, down to the first.
    ///
    /// `start` is the class of the first character.
    pub(super) fn plain_end_where(
        &self,
        i: usize,
        start: impl Fn(char) -> bool,
        accept: impl Fn(usize) -> bool,
    ) -> Option<usize> {
        let (c, first) = self.char_at(i)?;
        if !start(c) {
            return None;
        }
        let letters = self.idrest_letters(first);
        let mut end = self.op_tail(letters);
        loop {
            if accept(end) {
                return Some(end);
            }
            if end == first {
                return None;
            }
            end = self.char_start_before(end);
        }
    }

    /// The first end of an operator name at `i` that `accept` takes, from the
    /// whole run down to its first character.
    pub(super) fn op_end_where(&self, i: usize, accept: impl Fn(usize) -> bool) -> Option<usize> {
        let first = self.char_at(i).filter(|&(c, _)| is_op(c))?.1;
        let mut end = self.chars_while(first, is_op);
        loop {
            if accept(end) {
                return Some(end);
            }
            if end == first {
                return None;
            }
            end = self.char_start_before(end);
        }
    }

    /// The first end of `(?:plain|[op]+)|` followed by `` `[^`]+` `` at `i`
    /// that `accept` takes.
    /// A quoted name has one end.
    pub(super) fn any_name_where(&self, i: usize, accept: impl Fn(usize) -> bool) -> Option<usize> {
        self.plain_end_where(i, is_name_start, &accept)
            .or_else(|| self.op_end_where(i, &accept))
            .or_else(|| self.quoted_name(i).filter(|&end| accept(end)))
    }

    /// The first end of `` `[^`]+` `` followed by `(?:plain|[op]+)` at `i`
    /// that `accept` takes: the order the package, import and `end` rules
    /// list them in.
    pub(super) fn quoted_first_where(
        &self,
        i: usize,
        accept: impl Fn(usize) -> bool,
    ) -> Option<usize> {
        self.quoted_name(i)
            .filter(|&end| accept(end))
            .or_else(|| self.plain_end_where(i, is_name_start, &accept))
            .or_else(|| self.op_end_where(i, &accept))
    }
}
