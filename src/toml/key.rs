//! Keys, and the `=` that follows one.
//!
//! A key is bare names, quoted names, or both, joined by dots.
//! Some keys begin with a `"` and end with the first later `"` that an `=`
//! follows.
//! Such a key is read as a single quoted name, whatever its dots and quotes.
//! Its inner quotes are then marked invalid.
//! Shiki's output does this, so the lexer does it too.

use memchr::{memchr, memchr2};

use super::lexer::Lexer;
use super::scope::S;
use crate::classes::space_width;
use crate::utf8::char_start;

/// Where a key that was found ends.
#[derive(Clone, Copy)]
struct Key {
    /// The end of the key's text.
    end: usize,
    /// The `=` after it.
    equals: usize,
    /// Whether the key is one quoted name from `"` to `"`, which `end`
    /// follows.
    block: bool,
}

/// Whether a key's names are all bare or one of them opens with a quote.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Quoting {
    Bare,
    Quoted,
}

/// Whether `b` is a character of a bare name, `[A-Za-z0-9_-]`.
fn is_bare(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b'-'
}

impl Lexer<'_> {
    /// A key, its `=` and the value after it, at `p`.
    pub(super) fn key_value(&mut self, p: usize) -> Option<usize> {
        let b = self.s[p];
        if !(is_bare(b) || b == b'"' || b == b'\'') || p < self.no_key_before {
            return None;
        }
        let key = self.find_key(p)?;
        if key.block {
            self.block(p, key.end);
        } else {
            self.dotted(p, key.end);
        }
        self.tok_at(key.equals, key.equals + 1, &[S::PunctKeyValue]);
        Some(self.value_after_equals(key.equals + 1))
    }

    /// The key at `p`, if one starts there and an `=` follows it.
    fn find_key(&mut self, p: usize) -> Option<Key> {
        if self.last_eq.is_none_or(|last| last < p) {
            return None;
        }
        if self.s[p] == b'"'
            && let Some(close) = self.next_quote_equals(p)
        {
            let equals = self.skip_space(close + 1);
            return Some(Key {
                end: close + 1,
                equals,
                block: true,
            });
        }
        if p == self.failed_chain.get().0 {
            self.skip_failed_name(p);
            return None;
        }
        let (mut quoting, mut end) = self.name(p);
        let mut names = usize::from(end.is_some());
        let mut key_end = end?;
        let mut second = usize::MAX;
        loop {
            let dot = self.skip_space(key_end);
            if self.at(dot) != b'.' {
                break;
            }
            let next = self.skip_space(dot + 1);
            let (next_quoting, next_end) = self.name(next);
            if next_quoting == Quoting::Quoted {
                quoting = Quoting::Quoted;
            }
            end = next_end;
            match end {
                Some(name_end) => {
                    names += 1;
                    key_end = name_end;
                    if names == 2 {
                        second = next;
                    }
                }
                None if names >= 2 => {
                    key_end = next;
                    break;
                }
                None => break,
            }
        }
        let equals = self.skip_space(key_end);
        if self.at(equals) == b'=' {
            return Some(Key {
                end: key_end,
                equals,
                block: false,
            });
        }
        match quoting {
            Quoting::Bare => self.no_key_before = key_end,
            Quoting::Quoted if second != usize::MAX => self.failed_chain.set((second, key_end)),
            Quoting::Quoted => {}
        }
        None
    }

    /// Notes that the key that starts at the name at `p` failed, and finds
    /// the name after it in the failed chain.
    fn skip_failed_name(&self, p: usize) {
        let end = self.failed_chain.get().1;
        let mut next = usize::MAX;
        if let Some(name_end) = self.name_end(p) {
            let dot = self.skip_space(name_end);
            if self.at(dot) == b'.' {
                let after = self.skip_space(dot + 1);
                if after < end && self.name_end(after).is_some() {
                    next = after;
                }
            }
        }
        self.failed_chain.set((next, end));
    }

    /// Whether the name at `i` opens with a quote, and its end if it is a
    /// whole name.
    /// A quote that never closes still makes the name quoted.
    /// That decides what a failed key remembers.
    fn name(&self, i: usize) -> (Quoting, Option<usize>) {
        let quoting = if matches!(self.at(i), b'"' | b'\'') {
            Quoting::Quoted
        } else {
            Quoting::Bare
        };
        (quoting, self.name_end(i))
    }

    /// The end of the bare or quoted name at `i`, if one starts there.
    fn name_end(&self, i: usize) -> Option<usize> {
        match self.at(i) {
            b'"' => {
                let mut j = i + 1;
                loop {
                    let rest = self.s.get(j..)?;
                    let offset = memchr2(b'"', b'\\', rest)?;
                    self.steps.spend(offset);
                    j += offset;
                    if self.s[j] == b'"' {
                        return Some(j + 1);
                    }
                    if j + 1 >= self.s.len() {
                        return None;
                    }
                    j += 1 + self.width_at(j + 1);
                }
            }
            b'\'' => {
                let rest = self.s.get(i + 1..)?;
                let offset = memchr(b'\'', rest)?;
                self.steps.spend(offset);
                Some(i + 1 + offset + 1)
            }
            b if is_bare(b) => {
                let mut j = i + 1;
                while is_bare(self.at(j)) {
                    j += 1;
                }
                self.steps.spend(j - i);
                Some(j)
            }
            _ => None,
        }
    }

    /// The first `"` after `p` that `\s*=` follows: where a key that begins
    /// with the `"` at `p` ends.
    fn next_quote_equals(&self, p: usize) -> Option<usize> {
        let (start, answer) = self.quote_eq.get();
        if start <= p && p < answer {
            return (answer != usize::MAX).then_some(answer);
        }
        let mut from = p + 1;
        loop {
            let Some(offset) = memchr(b'=', self.s.get(from..)?) else {
                self.quote_eq.set((p, usize::MAX));
                return None;
            };
            let equals = from + offset;
            self.steps.spend(offset);
            let mut j = equals;
            while j > 0 && self.space_before(j) {
                j = char_start(self.s, j - 1);
            }
            self.steps.spend(equals - j);
            if j > p + 1 && self.s[j - 1] == b'"' {
                self.quote_eq.set((p, j - 1));
                return Some(j - 1);
            }
            from = equals + 1;
        }
    }

    /// Whether the character that ends at `i` is white space.
    fn space_before(&self, i: usize) -> bool {
        let start = char_start(self.s, i - 1);
        space_width(self.s, start) > 0
    }

    /// Emits a key that is one quoted name, from `"` at `p` to `"` at
    /// `end - 1`.
    fn block(&mut self, p: usize, end: usize) {
        self.tok_at(p, p + 1, &[S::Key, S::PunctKeyBegin]);
        let close = end - 1;
        let mut i = p + 1;
        while i < close {
            match self.s[i] {
                b'"' => {
                    self.tok_at(i, i + 1, &[S::Invalid]);
                    i += 1;
                }
                b'\\' => match self.escape(i) {
                    Some((len, scope)) if i + len <= close => {
                        self.tok_at(i, i + len, &[scope]);
                        i += len;
                    }
                    _ => i += 1,
                },
                _ => i += self.width_at(i),
            }
        }
        self.tok_at(close, end, &[S::Key, S::PunctKeyEnd]);
    }

    /// Emits a key of names joined by dots, from `p` to `end`.
    /// Its whole text is scoped as a key, white space between the names and
    /// dots included.
    fn dotted(&mut self, p: usize, end: usize) {
        let mut i = p;
        while i < end {
            match self.s[i] {
                b'.' => {
                    self.tok_at(i, i + 1, &[S::Key, S::PunctKeySep]);
                    i += 1;
                }
                quote @ (b'"' | b'\'') => i = self.quoted_name(i, quote),
                _ => {
                    let from = i;
                    while i < end && !matches!(self.s[i], b'.' | b'"' | b'\'') {
                        i += self.width_at(i);
                    }
                    self.tok_at(from, i, &[S::Key]);
                }
            }
        }
    }

    /// Emits the quoted name that starts at `i`, and returns where it ends.
    fn quoted_name(&mut self, i: usize, quote: u8) -> usize {
        let Some(close) = self.name_end(i).map(|end| end - 1) else {
            return i + 1;
        };
        self.tok_at(i, i + 1, &[S::Key, S::PunctKeyBegin]);
        let mut j = i + 1;
        while j < close {
            if quote == b'"'
                && self.s[j] == b'\\'
                && let Some((len, scope)) = self.escape(j)
                && j + len <= close
            {
                self.tok(j, &[S::Key]);
                self.tok(j + len, &[S::Key, scope]);
                j += len;
            } else {
                j += self.width_at(j);
            }
        }
        self.tok(close, &[S::Key]);
        self.tok(close + 1, &[S::Key, S::PunctKeyEnd]);
        close + 1
    }
}
