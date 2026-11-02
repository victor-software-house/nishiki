//! The text a pattern is matched against, and the character classes of the
//! grammar's regular expressions: `\w`, `\s`, `\d` and `\b`.

use super::engine::{Act, Dyn, Hit, Lexer, Retok, Span};
use super::scope::S;
use super::state::R;
use crate::classes;
use crate::classes::is_ascii_space;
use crate::utf8::char_len;

impl Lexer<'_> {
    /// The character at `i` and its width; the line feed after the text
    /// counts as a character.
    pub(super) fn char_at(&self, i: usize) -> Option<(char, usize)> {
        if i >= self.len_all() {
            return None;
        }
        let lead = self.b(i);
        if lead < 0x80 {
            return Some((char::from(lead), 1));
        }
        let width = char_len(lead);
        let text = std::str::from_utf8(self.s.get(i..i + width)?).ok()?;
        Some((text.chars().next()?, width))
    }

    /// The character that ends at `i`.
    pub(super) fn char_before(&self, i: usize) -> Option<char> {
        let mut start = i.checked_sub(1)?;
        if start >= self.s.len() {
            return Some('\n');
        }
        while start > 0 && self.s.get(start).is_some_and(|b| b & 0xc0 == 0x80) {
            start -= 1;
        }
        let lead = *self.s.get(start)?;
        let text = std::str::from_utf8(self.s.get(start..start + char_len(lead))?).ok()?;
        text.chars().next()
    }

    /// The width of the `\w` character at `i`, or 0.
    pub(super) fn word_at(&self, i: usize) -> usize {
        let b = self.b(i);
        if b < 0x80 {
            return usize::from(i < self.len_all() && (b.is_ascii_alphanumeric() || b == b'_'));
        }
        match self.char_at(i) {
            Some((c, width)) if classes::is_word(c) => width,
            _ => 0,
        }
    }

    /// Whether the character before `i` is a `\w` character.
    pub(super) fn word_before(&self, i: usize) -> bool {
        match self.prev(i) {
            None => false,
            Some(b) if b < 0x80 => b.is_ascii_alphanumeric() || b == b'_',
            Some(_) => self.char_before(i).is_some_and(classes::is_word),
        }
    }

    /// The width of the `\s` character at `i`, or 0.
    pub(super) fn space_at(&self, i: usize) -> usize {
        let b = self.b(i);
        if b < 0x80 {
            return usize::from(i < self.len_all() && is_ascii_space(b));
        }
        match self.char_at(i) {
            Some((c, width)) if classes::is_space(c) => width,
            _ => 0,
        }
    }

    /// The width of the `\d` character at `i`, or 0.
    pub(super) fn digit_at(&self, i: usize) -> usize {
        let b = self.b(i);
        if b < 0x80 {
            return usize::from(i < self.len_all() && b.is_ascii_digit());
        }
        match self.char_at(i) {
            Some((c, width)) if classes::is_digit(c) => width,
            _ => 0,
        }
    }

    /// The end of the run that starts at `i` and holds the characters
    /// `width_of` accepts, found once per run: the cache slot `slot` keeps
    /// the last run, and a search from inside it ends where it does.
    fn run_end(&self, i: usize, slot: usize, width_of: impl Fn(usize) -> usize) -> usize {
        let cache = &self.runs[slot];
        let (from, to) = cache.get();
        if from <= i && i <= to {
            self.spend(1);
            return to;
        }
        let mut j = i;
        loop {
            let width = width_of(j);
            if width == 0 {
                break;
            }
            j += width;
        }
        self.spend(j - i + 1);
        cache.set((i, j));
        j
    }

    /// The end of `\s*` from `i`.
    pub(super) fn skip_space(&self, i: usize) -> usize {
        self.run_end(i, 0, |j| self.space_at(j))
    }

    /// The end of `\w*` from `i`.
    pub(super) fn skip_word(&self, i: usize) -> usize {
        self.run_end(i, 1, |j| self.word_at(j))
    }

    /// The end of `[A-Za-z_]\w*` at `i`.
    pub(super) fn ident_end(&self, i: usize) -> Option<usize> {
        let b = self.b(i);
        (i < self.len_all() && (b.is_ascii_alphabetic() || b == b'_'))
            .then(|| self.skip_word(i + 1))
    }

    /// The end of `[A-Za-z_][0-9A-Za-z_]*` at `i`, which is ASCII only.
    pub(super) fn ascii_ident_end(&self, i: usize) -> Option<usize> {
        let b = self.b(i);
        if i >= self.len_all() || !(b.is_ascii_alphabetic() || b == b'_') {
            return None;
        }
        Some(self.run_end(i + 1, 2, |j| {
            let c = self.b(j);
            usize::from(j < self.len_all() && (c.is_ascii_alphanumeric() || c == b'_'))
        }))
    }

    /// The regex `\b` at `i`.
    pub(super) fn boundary(&self, i: usize) -> bool {
        self.word_before(i) != (self.word_at(i) > 0)
    }

    /// The text from `start` to `end`.
    pub(super) fn text(&self, start: usize, end: usize) -> &[u8] {
        self.s.get(start..end.min(self.s.len())).unwrap_or(&[])
    }

    // Hits.

    /// A `match` rule with `scopes` on the whole match.
    pub(super) fn plain(&mut self, start: usize, end: usize, scopes: &'static [S]) -> bool {
        self.hit = Hit {
            start,
            end,
            act: Act::Plain(scopes, None),
        };
        true
    }

    /// A `match` rule whose name is `prefix` and the text `from..to` and `.c`.
    pub(super) fn plain_dyn(
        &mut self,
        start: usize,
        end: usize,
        prefix: &'static str,
        text: (usize, usize),
    ) -> bool {
        self.hit = Hit {
            start,
            end,
            act: Act::Plain(
                &[],
                Some(Dyn {
                    prefix,
                    start: text.0,
                    end: text.1,
                }),
            ),
        };
        true
    }

    /// The begin of `rule`.
    pub(super) fn begin(&mut self, start: usize, end: usize, rule: R) -> bool {
        self.hit = Hit {
            start,
            end,
            act: Act::Begin(rule),
        };
        true
    }

    /// A capture with `scopes`.
    pub(super) fn cap(&mut self, start: usize, end: usize, scopes: &'static [S]) {
        self.sp.push(Span {
            start,
            end,
            scopes,
            dynamic: None,
            retok: Retok::No,
        });
    }

    /// A capture whose name is `prefix`, the text `text` and `.c`.
    pub(super) fn cap_dyn(
        &mut self,
        start: usize,
        end: usize,
        prefix: &'static str,
        text: (usize, usize),
    ) {
        self.sp.push(Span {
            start,
            end,
            scopes: &[],
            dynamic: Some(Dyn {
                prefix,
                start: text.0,
                end: text.1,
            }),
            retok: Retok::No,
        });
    }

    /// A capture lexed again with `retok`, under `scopes`.
    pub(super) fn cap_retok(
        &mut self,
        start: usize,
        end: usize,
        scopes: &'static [S],
        retok: Retok,
    ) {
        self.sp.push(Span {
            start,
            end,
            scopes,
            dynamic: None,
            retok,
        });
    }
}
