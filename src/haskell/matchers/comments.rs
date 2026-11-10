//! Comments, Haddock comments, pragmas and the C preprocessor.

use super::super::engine::Lexer;
use super::super::grammar::{DEPRECATED_EXTENSIONS, EXTENSIONS, PRAGMAS};
use super::is_op;
use crate::rules::Chars;

impl Lexer<'_> {
    /// `^(#).*$`: a C preprocessor line, with group 1 around the `#`.
    pub(super) fn cpp(&mut self, p: usize) -> Option<usize> {
        if p != 0 || self.lim() == 0 || self.at(0) != b'#' {
            return None;
        }
        self.capture(1, 0, 1);
        Some(self.lim())
    }

    /// `(?i)\b(?<!')word\b(?!')` at `i`: a pragma's word without regard to
    /// case, and its end.
    fn folded_keyword(&self, i: usize, word: &[u8]) -> Option<usize> {
        if self.prev_is(i, b'\'') || self.word_before(i) || self.word_at(i).is_none() {
            return None;
        }
        let end = self.folded(i, word)?;
        (self.word_at(end).is_none() && self.at(end) != b'\'').then_some(end)
    }

    /// `(?i)\b(?<!')(LANGUAGE)\b(?!')`, with group 1 around the word.
    pub(super) fn pragma_word(&mut self, p: usize, word: &[u8]) -> Option<usize> {
        let end = self.folded_keyword(p, word)?;
        self.capture(1, p, end);
        Some(end)
    }

    /// `(?:No)?(?:a|b|…)` for the extensions `list`: the end of the first
    /// extension in order after `No`, or else at `p`.
    /// No `\b` follows, so an extension that starts a longer one wins.
    fn extension_end(&self, p: usize, list: &[&[u8]]) -> Option<usize> {
        let first = |at: usize| list.iter().find_map(|word| self.lit(at, word));
        self.lit(p, b"No").and_then(first).or_else(|| first(p))
    }

    /// `(?:No)?(?:AutoDeriveTypeable|…)`: a deprecated extension.
    pub(super) fn deprecated_extension(&self, p: usize) -> Option<usize> {
        self.extension_end(p, DEPRECATED_EXTENSIONS)
    }

    /// `((?:No)?(?:AllowAmbiguousTypes|…))`: an extension, with group 1
    /// around it.
    pub(super) fn extension(&mut self, p: usize) -> Option<usize> {
        let end = self.extension_end(p, EXTENSIONS)?;
        self.capture(1, p, end);
        Some(end)
    }

    /// `(?i)SPECIALI[SZ]E` at `i`, and its end.
    fn specialise_word(&self, i: usize) -> Option<usize> {
        let stem = self.folded(i, b"speciali")?;
        let letter = self
            .folded(stem, b"s")
            .or_else(|| self.folded(stem, b"z"))?;
        self.folded(letter, b"e")
    }

    /// `\[[^]\[]*]` at `i`, and its end.
    fn phase(&self, i: usize) -> Option<usize> {
        if !(i < self.lim() && self.at(i) == b'[') {
            return None;
        }
        let close = self.skip(i + 1, |b| b != b'[' && b != b']');
        (close < self.lim() && self.at(close) == b']').then_some(close + 1)
    }

    /// `(?i)\b(?<!')(SPECIALI[SZ]E)(?:\s*(\[[^]\[]*])?\s*|\s+)(instance)\b(?!'
    /// )`, with groups around the word, the phase and `instance`.
    ///
    /// The phase is tried first, then `instance` right after the blanks.
    /// The `\s+` alternative matches nothing the first one does not.
    pub(super) fn specialise_instance(&mut self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'\'') || self.word_before(p) || self.word_at(p).is_none() {
            return None;
        }
        let word = self.specialise_word(p)?;
        let instance = |l: &Self, at: usize| {
            l.folded(at, b"instance")
                .filter(|&end| l.word_at(end).is_none() && l.at(end) != b'\'')
        };
        let blanks = self.space_end(word);
        if let Some(phase) = self.phase(blanks) {
            let at = self.space_end(phase);
            if let Some(end) = instance(self, at) {
                self.capture(1, p, word);
                self.capture(2, blanks, phase);
                self.capture(3, at, end);
                return Some(end);
            }
        }
        let end = instance(self, blanks)?;
        self.capture(1, p, word);
        self.capture(3, blanks, end);
        Some(end)
    }

    /// `(?i)\b(?<!')(SPECIALI[SZ]E)\b(?!')(?:\s+(INLINE)\b(?!'))?\s*(\[[^]\[]*
    /// ])?\s*`, with groups around the word, `INLINE` and the phase.
    pub(super) fn specialise(&mut self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'\'') || self.word_before(p) || self.word_at(p).is_none() {
            return None;
        }
        let word = self.specialise_word(p)?;
        if self.word_at(word).is_some() || self.at(word) == b'\'' {
            return None;
        }
        self.capture(1, p, word);
        let mut at = word;
        let blanks = self.space_end(word);
        if blanks > word
            && let Some(inline) = self.folded_keyword(blanks, b"inline")
        {
            self.capture(2, blanks, inline);
            at = inline;
        }
        let blanks = self.space_end(at);
        if let Some(phase) = self.phase(blanks) {
            self.capture(3, blanks, phase);
            return Some(self.space_end(phase));
        }
        Some(blanks)
    }

    /// `(?i)\b(?<!')(LANGUAGE|OPTIONS_GHC|…)\b(?!')`, with group 1 around the
    /// first name in order that matches as a whole word.
    pub(super) fn pragma_name(&mut self, p: usize) -> Option<usize> {
        let end = PRAGMAS
            .iter()
            .find_map(|word| self.folded_keyword(p, word))?;
        self.capture(1, p, end);
        Some(end)
    }

    /// `(?i)\b(DEPRECATED|WARNING)\b`, with group 1 around the word.
    pub(super) fn deprecated(&mut self, p: usize) -> Option<usize> {
        if self.word_before(p) || self.word_at(p).is_none() {
            return None;
        }
        let end = [&b"deprecated"[..], b"warning"].iter().find_map(|word| {
            self.folded(p, word)
                .filter(|&end| self.word_at(end).is_none())
        })?;
        self.capture(1, p, end);
        Some(end)
    }

    /// The end of `--\s[x]` at `i`, for the marks `marks` that may follow.
    fn haddock_dashes(&self, i: usize, marks: &[u8]) -> Option<usize> {
        let blank = self.lit(i, b"--")?;
        let width = self.space_at(blank);
        let mark = blank + width;
        (width > 0 && mark < self.lim() && marks.contains(&self.at(mark))).then_some(mark + 1)
    }

    /// `^(\s*)(--\s[$|])`: a Haddock comment that the lines after it continue,
    /// with groups around the indentation and the marker.
    pub(super) fn haddock_line(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let dashes = self.space_end(0);
        let end = self.haddock_dashes(dashes, b"$|")?;
        self.capture(1, 0, dashes);
        self.capture(2, dashes, end);
        Some(end)
    }

    /// `(?=^(?!\1--+(?![OP])))`: the start of a line that does not continue
    /// the Haddock comment, a line with no `--` right after the comment's
    /// indentation.
    pub(super) fn haddock_line_end(&self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let indent = self.begin_text().as_bytes();
        let continues = self.s.starts_with(indent) && {
            let dashes = self.skip(indent.len(), |b| b == b'-');
            dashes - indent.len() >= 2 && !self.here_is(dashes, is_op)
        };
        (!continues).then_some(p)
    }

    /// `(^[\t ]+)?(--\s[*^])`: a Haddock comment for what comes before, with
    /// groups around the indentation and the marker.
    pub(super) fn haddock_mark(&mut self, p: usize) -> Option<usize> {
        if p == 0 {
            let indent = self.skip(0, |b| b == b' ' || b == b'\t');
            if indent > 0
                && let Some(end) = self.haddock_dashes(indent, b"*^")
            {
                self.capture(1, 0, indent);
                self.capture(2, indent, end);
                return Some(end);
            }
        }
        let end = self.haddock_dashes(p, b"*^")?;
        self.capture(2, p, end);
        Some(end)
    }

    /// `\{-\s?[$*^|]`: a Haddock block comment.
    ///
    /// A blank given back leaves the mark at the blank, where it fails.
    pub(super) fn haddock_block(&self, p: usize) -> Option<usize> {
        let open = self.lit(p, b"{-")?;
        let mark = open + self.space_at(open);
        (mark < self.lim() && matches!(self.at(mark), b'$' | b'*' | b'^' | b'|'))
            .then_some(mark + 1)
    }

    /// Whether `--+(?![OP])` matches at `i`: two or more dashes that no
    /// operator character follows.
    ///
    /// Giving a dash back leaves a dash after the rest, an operator character.
    fn comment_dashes(&self, i: usize) -> bool {
        let end = self.skip(i, |b| b == b'-');
        end - i >= 2 && !self.here_is(end, is_op)
    }

    /// `(^[\t ]+)?(?=--+(?![OP]))`: the start of a line comment, with group 1
    /// around the indentation before it.
    pub(super) fn line_comment(&mut self, p: usize) -> Option<usize> {
        if p == 0 {
            let indent = self.skip(0, |b| b == b' ' || b == b'\t');
            if indent > 0 && self.comment_dashes(indent) {
                self.capture(1, 0, indent);
                return Some(indent);
            }
        }
        self.comment_dashes(p).then_some(p)
    }
}
