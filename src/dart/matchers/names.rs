//! Class names, the primitive types and function names.

use super::super::engine::Lexer;
use super::is_ident;
use crate::rules::Chars;

/// The deepest nesting of choices that the type arguments of a call explore
/// before the matcher gives up and reports the depth limit.
const GENERIC_CHOICES: usize = 32;

/// Whether a type name may take `\s*(factory\b)?` after it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Factory {
    /// It may: the blanks and `factory` join the match.
    Optional,
    /// It may not.
    Never,
}

/// Whether `b` is in `[$0-9<>?A-Z_a-z]`, which a call's type arguments take
/// one character at a time.
fn is_generic(b: u8) -> bool {
    is_ident(b) || matches!(b, b'<' | b'>' | b'?')
}

impl Lexer<'_> {
    /// `(?<!\$)\b(a|b|…)\b(?!\$)` for the type words `words`, with group 1
    /// around the word, and `\s*(factory\b)?` after it when `factory` is set.
    pub(super) fn type_word(
        &mut self,
        p: usize,
        words: &[&[u8]],
        factory: Factory,
    ) -> Option<usize> {
        let end = self.plain_word(p, words)?;
        self.capture(1, p, end);
        Some(self.factory(end, factory))
    }

    /// `(?<![$0-9A-Z_a-z])([$_]*[A-Z][$0-9A-Z_a-z]*)\b`, with group 1 around
    /// the name, and `\s*(factory\b)?` after it when `factory` is set.
    ///
    /// The name gives characters back until `\b` holds.
    /// Only a `$` can make it give any.
    pub(super) fn class_name(&mut self, p: usize, factory: Factory) -> Option<usize> {
        if self.prev(p).is_some_and(is_ident) {
            return None;
        }
        let upper = self.skip(p, |b| b == b'$' || b == b'_');
        if !self.at(upper).is_ascii_uppercase() {
            return None;
        }
        let run = self.skip(upper + 1, is_ident);
        let end = (upper + 1..=run).rev().find(|&q| self.boundary(q))?;
        self.capture(1, p, end);
        Some(self.factory(end, factory))
    }

    /// `\s*(factory\b)?` from `end`, with group 2 around `factory`.
    fn factory(&mut self, end: usize, factory: Factory) -> usize {
        match factory {
            Factory::Never => end,
            Factory::Optional => {
                let blanks = self.space_end(end);
                match self.lit(blanks, b"factory") {
                    Some(word) if self.word_at(word).is_none() => {
                        self.capture(2, blanks, word);
                        word
                    }
                    _ => blanks,
                }
            }
        }
    }

    /// `(?<=\.)new\b`.
    pub(super) fn constructor_new(&self, p: usize) -> Option<usize> {
        if !self.prev_is(p, b'.') {
            return None;
        }
        let end = self.lit(p, b"new")?;
        self.word_at(end).is_none().then_some(end)
    }

    /// A call or a declaration with its parameters:
    /// - the name, `([$_]*[a-z][$0-9A-Z_a-z]*)`, as group 1;
    /// - the optional type arguments, as group 2:
    ///   `(<(?:[$0-9<>?A-Z_a-z]|,\s*|\s+extends\s+)+>)?`;
    /// - then `[!?]?\(`.
    ///
    /// Giving back characters of the name cannot help.
    /// A character of the name is none of `<`, `!`, `?` and `(`.
    pub(super) fn function_name(&mut self, p: usize) -> Option<usize> {
        let lower = self.skip(p, |b| b == b'$' || b == b'_');
        if !self.at(lower).is_ascii_lowercase() {
            return None;
        }
        let name = self.skip(lower + 1, is_ident);
        let end = if self.at(name) == b'<' {
            let (close, end) = self.type_arguments(name + 1, true, 0)?;
            self.capture(2, name, close + 1);
            end
        } else {
            self.call(name)?
        };
        self.capture(1, p, name);
        Some(end)
    }

    /// `[!?]?\(` at `i`.
    fn call(&self, i: usize) -> Option<usize> {
        let paren = if matches!(self.at(i), b'!' | b'?') {
            i + 1
        } else {
            i
        };
        (self.at(paren) == b'(').then_some(paren + 1)
    }

    /// Where `\s+extends\s+` from the blank at `i` ends, as far as it can.
    fn extends_after(&self, i: usize) -> Option<usize> {
        let word = self.lit(self.space_end(i), b"extends")?;
        let end = self.space_end(word);
        (end > word).then_some(end)
    }

    /// The type arguments of a call from `from`, as the backtracking matcher
    /// explores `(?:[$0-9<>?A-Z_a-z]|,\s*|\s+extends\s+)+>[!?]?\(`.
    /// The answer is the `>` that closes them and the end of the call.
    ///
    /// The matcher takes as many tokens as it can.
    /// Then it tries the rest at each position back to `from`, latest first.
    /// A position is a choice only where blanks run into `extends`.
    /// The greedy token leads into `extends` as a word, and a shorter one
    /// leaves a blank that only `\s+extends\s+` takes.
    ///
    /// The greedy path is explored first, by a nested call.
    /// The other path goes on in this loop.
    /// `first` says that `from` follows the `<`, where one token is needed.
    fn type_arguments(&mut self, from: usize, first: bool, depth: usize) -> Option<(usize, usize)> {
        if depth > GENERIC_CHOICES {
            self.record_depth();
            return None;
        }
        let mut q = from;
        loop {
            let b = self.at(q);
            if q < self.lim() && is_generic(b) {
                q += 1;
                continue;
            }
            let (next, other) = match b {
                b',' => {
                    let next = self.space_end(q + 1);
                    let other =
                        (next > q + 1 && self.extends_after(q + 1).is_some()).then_some(q + 1);
                    (next, other)
                }
                _ if self.space_at(q) > 0 => {
                    let Some(next) = self.extends_after(q) else {
                        break;
                    };
                    let word = self.space_end(q) + b"extends".len();
                    let blank = word + self.space_at(word);
                    let other =
                        (next > blank && self.extends_after(blank).is_some()).then_some(blank);
                    (next, other)
                }
                _ => break,
            };
            let Some(other) = other else {
                q = next;
                continue;
            };
            if let Some(found) = self.type_arguments(next, false, depth + 1) {
                return Some(found);
            }
            q = other;
        }
        self.spend(q.saturating_sub(from));
        let last = if first { from + 1 } else { from };
        (last..=q.min(self.lim()))
            .rev()
            .filter(|&m| self.at(m) == b'>')
            .find_map(|m| self.call(m + 1).map(|end| (m, end)))
    }
}
