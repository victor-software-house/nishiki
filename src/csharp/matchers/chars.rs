//! The character classes and runs the grammar's expressions are built from.
//!
//! `\s`, `\w` and `\d` read the Unicode classes as Shiki's regex engine does;
//! `[:alpha:]` is the Unicode `Alphabetic` property and `[:alnum:]` adds the
//! decimal digits to it.

use memchr::{memchr, memrchr2};

use super::super::engine::Lexer;
use super::super::limits::MAX_NESTING;
use crate::classes::{is_digit, is_space};
use crate::rules::Chars;

/// `[-_[:alnum:]]` for a character.
pub(super) fn is_name_char(c: char) -> bool {
    c == '-' || is_alnum(c)
}

/// `[_[:alpha:]]` for a character.
fn is_alpha(c: char) -> bool {
    c == '_' || c.is_alphabetic()
}

/// `[_[:alnum:]]` for a character.
pub(super) fn is_alnum(c: char) -> bool {
    c == '_' || c.is_alphabetic() || is_digit(c)
}

/// Whether a bracketed group of the grammar needs an item in its body: `+`
/// or `*` in `(?:[^<>]|\g<group>)+`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::csharp) enum Body {
    NonEmpty,
    MayBeEmpty,
}

impl Lexer<'_> {
    /// The character at `p`: never one at or after the end of the text.
    pub(in crate::csharp) fn cp(&self, p: usize) -> Option<(char, usize)> {
        if p >= self.lim() {
            return None;
        }
        self.char_at(p)
    }

    /// The width of the `\s` character at `p`, 0 when there is none. The line
    /// feed after the line is one.
    pub(in crate::csharp) fn space_width(&self, p: usize) -> usize {
        match self.at(p) {
            0 => 0,
            b if b < 0x80 => usize::from(matches!(b, b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r')),
            _ => match self.cp(p) {
                Some((c, end)) if is_space(c) => end - p,
                _ => 0,
            },
        }
    }

    /// `\s*` from `p`: the end of the run. A search from inside the last
    /// run found ends where it does, so a long run is read once.
    pub(in crate::csharp) fn blanks(&self, mut p: usize) -> usize {
        let lim = self.lim();
        let (start, end, run_lim) = self.x.blank_run.get();
        if run_lim == lim && start <= p && p < end {
            return end;
        }
        let from = p;
        loop {
            let width = self.space_width(p);
            if width == 0 {
                self.spend(p - from);
                if p > from {
                    self.x.blank_run.set((from, p, lim));
                }
                return p;
            }
            p += width;
        }
    }

    /// `\s+` from `p`: the end of the run, when it has a character.
    pub(in crate::csharp) fn blanks1(&self, p: usize) -> Option<usize> {
        let end = self.blanks(p);
        (end > p).then_some(end)
    }

    /// `[_[:alpha:]]` at `p`: the end of the character.
    pub(in crate::csharp) fn alpha_at(&self, p: usize) -> Option<usize> {
        let b = self.at(p);
        if b < 0x80 {
            return (p < self.lim() && (b.is_ascii_alphabetic() || b == b'_')).then_some(p + 1);
        }
        self.cp(p).filter(|&(c, _)| is_alpha(c)).map(|(_, end)| end)
    }

    /// `[_[:alnum:]]*` from `p`: the end of the run.
    pub(in crate::csharp) fn alnum_run(&self, mut p: usize) -> usize {
        let from = p;
        while p < self.lim() {
            let b = self.s[p];
            if b < 0x80 {
                if b.is_ascii_alphanumeric() || b == b'_' {
                    p += 1;
                    continue;
                }
                break;
            }
            match self.cp(p) {
                Some((c, end)) if is_alnum(c) => p = end,
                _ => break,
            }
        }
        self.spend(p - from);
        p
    }

    /// `[_[:alpha:]][_[:alnum:]]*` at `p`: the end of the name.
    pub(in crate::csharp) fn plain_ident(&self, p: usize) -> Option<usize> {
        let first = self.alpha_at(p)?;
        Some(self.alnum_run(first))
    }

    /// `@?[_[:alpha:]][_[:alnum:]]*` at `p`, the grammar's `identifier`: the
    /// end of it.
    /// Giving back the end of a longer name would leave a name character next.
    /// No expression that follows a name accepts one, so the longest name is
    /// the only one tried.
    pub(in crate::csharp) fn ident(&self, p: usize) -> Option<usize> {
        let start = if self.at(p) == b'@' { p + 1 } else { p };
        self.plain_ident(start)
    }

    /// `\b` word `\b` with no `.` before it: a keyword the grammar does not
    /// take for a member name. The end of the word.
    ///
    /// Always inlined, as `Chars::keyword` is.
    /// It holds that body, and without the mark it went out of line.
    #[expect(
        clippy::inline_always,
        reason = "measured at one and sixteen codegen units: holding keyword's body, it stopped being inlined into try_match, at a call per word pattern"
    )]
    #[inline(always)]
    pub(in crate::csharp) fn non_member_keyword(&self, p: usize, word: &[u8]) -> Option<usize> {
        self.keyword(p, word).filter(|_| !self.prev_is(p, b'.'))
    }

    /// Whether the byte at `p` is one of `set`.
    pub(in crate::csharp) fn byte_in(&self, p: usize, set: &[u8]) -> bool {
        p < self.lim() && set.contains(&self.s[p])
    }

    /// The regex `(?=[set])` at `p`; the line feed after the line is a byte
    /// of the text, so a set that lists it matches there.
    pub(in crate::csharp) fn ahead_in(&self, p: usize, set: &[u8]) -> bool {
        let b = self.at(p);
        self.avail(p) && set.contains(&b)
    }

    /// The end of the group that opens at `open_at`.
    /// The group is the grammar's `(?:[^<>]|\g<group>)+` closed by `close`, or
    /// its `*` kin as `body` selects.
    /// Its body holds anything but the two brackets, and nested groups.
    /// A body may hold the line feed after the line, which no group closes.
    ///
    /// Bit `d` of a word records whether the group open at depth `d` has an
    /// item yet.
    pub(in crate::csharp) fn balanced(
        &self,
        open_at: usize,
        open: u8,
        close: u8,
        body: Body,
    ) -> Option<usize> {
        if self.at(open_at) != open || open_at >= self.lim() {
            return None;
        }
        let mut items: u64 = u64::from(body == Body::MayBeEmpty);
        let mut depth: u32 = 0;
        let mut i = open_at + 1;
        let lim = self.lim();
        loop {
            let Some(found) = self.s[i.min(lim)..lim]
                .iter()
                .position(|&b| b == open || b == close)
            else {
                self.spend(lim - open_at);
                return None;
            };
            let at = i + found;
            if at > i {
                items |= 1 << depth;
            }
            if self.s[at] == open {
                items |= 1 << depth;
                depth += 1;
                if depth >= MAX_NESTING {
                    return None;
                }
                match body {
                    Body::MayBeEmpty => items |= 1 << depth,
                    Body::NonEmpty => items &= !(1 << depth),
                }
            } else if items >> depth & 1 == 0 {
                self.spend(at - open_at);
                return None;
            } else if depth == 0 {
                self.spend(at - open_at);
                return Some(at + 1);
            } else {
                depth -= 1;
            }
            i = at + 1;
        }
    }

    /// The first `byte` at or after `p`, before the end of the text.
    pub(in crate::csharp) fn find_byte(&self, p: usize, byte: u8) -> Option<usize> {
        let lim = self.lim();
        let rest = self.s.get(p..lim)?;
        let found = memchr(byte, rest).map(|i| p + i);
        self.spend(found.map_or(lim - p, |at| at - p));
        found
    }

    /// The last `,` or `:` before the end of the text, found once per end so
    /// that `(?=.*[,:])` costs one search a line.
    pub(in crate::csharp) fn last_comma_or_colon(&self) -> Option<usize> {
        let lim = self.lim();
        if let Some((end, found)) = self.x.last_comma_or_colon.get()
            && end == lim
        {
            return found;
        }
        let found = memrchr2(b',', b':', self.s.get(..lim).unwrap_or_default());
        self.spend(lim);
        self.x.last_comma_or_colon.set(Some((lim, found)));
        found
    }
}
