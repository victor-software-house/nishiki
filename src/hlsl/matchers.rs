//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would.
//! Where a quantifier could give characters back, the doc comment says why it
//! cannot matter.
//! `\w`, `\s` and `\b` take non-ASCII characters, as in Oniguruma, through
//! [`Chars`].
//! `[0-9]` and `\h` are ASCII.
//!
//! Most of the grammar is `\b(…)\b` and `\b(?i:…)\b` alternations of words.
//! Their words are in [`super::words`], each with the bits of the patterns
//! that list it.
//! `(?i)` compares words through [`crate::case_fold`].

use super::engine::{Lexer, Word};
use super::grammar::Pat;
use super::words;
use crate::rules::Chars;
use crate::{bytes, case_fold};

/// Whether `b` is an ASCII `\w` character.
fn is_word(b: u8) -> bool {
    b == b'_' || b.is_ascii_alphanumeric()
}

/// The bits a sorted word table gives `word`, or 0 when it lacks it.
fn bits_of(table: &[(&[u8], u64)], word: &[u8]) -> u64 {
    table
        .binary_search_by(|&(listed, _)| bytes::order(listed, word))
        .ok()
        .and_then(|index| table.get(index))
        .map_or(0, |&(_, bits)| bits)
}

/// Whether the sorted list `words` holds `word`.
fn holds(words: &[&[u8]], word: &[u8]) -> bool {
    words
        .binary_search_by(|listed| bytes::order(listed, word))
        .is_ok()
}

impl Lexer<'_> {
    /// Whether `pat` matches at `p`, and where its match ends.
    pub(super) fn matcher(&mut self, pat: Pat, p: usize) -> Option<usize> {
        match pat {
            Pat::BlockComment => self.lit(p, b"/*"),
            Pat::BlockCommentEnd => self.lit(p, b"*/"),
            Pat::LineComment => self.lit(p, b"//"),
            Pat::LineCommentEnd => self.at_eol(p).then_some(p),
            Pat::Float => self.float(p),
            Pat::Fraction => self.fraction(p),
            Pat::Integer => self.integer(p),
            Pat::Hex => self.hex(p),
            Pat::Preprocessor => self.directive(p),
            Pat::Function => self.function(p),
            Pat::Semantic => self.semantic(p, words::SEMANTIC_WORDS, words::SEMANTIC_DIGITS),
            Pat::SemanticSm4 => {
                self.semantic(p, words::SEMANTIC_SM4_WORDS, words::SEMANTIC_SM4_DIGITS)
            }
            Pat::SemanticSm5 => {
                self.semantic(p, words::SEMANTIC_SM5_WORDS, words::SEMANTIC_SM5_DIGITS)
            }
            Pat::SemanticSm51 => {
                self.semantic(p, words::SEMANTIC_SM51_WORDS, words::SEMANTIC_SM51_DIGITS)
            }
            Pat::Bool
            | Pat::Control
            | Pat::Compile
            | Pat::Typedef
            | Pat::BasicType
            | Pat::Modifier
            | Pat::FloatModifier
            | Pat::PostfixModifier
            | Pat::Interpolation
            | Pat::GeometryModifier
            | Pat::StringType
            | Pat::ObjectType
            | Pat::RasterizerOrderedType
            | Pat::RwType
            | Pat::StreamType
            | Pat::LegacySampler
            | Pat::Sampler
            | Pat::LegacyTexture
            | Pat::Texture
            | Pat::Structured
            | Pat::FxConstant
            | Pat::FxStateType
            | Pat::Technique
            | Pat::BlendStateKey
            | Pat::DepthStencilStateKey
            | Pat::RasterizerStateKey
            | Pat::SamplerStateKey
            | Pat::Blend
            | Pat::BlendOp
            | Pat::DepthWriteMask
            | Pat::ComparisonFunc
            | Pat::StencilOp
            | Pat::FillMode
            | Pat::CullMode
            | Pat::Filter
            | Pat::AddressMode => self.listed(p, words::bit(pat)),
            Pat::String | Pat::StringEnd => self.lit(p, b"\""),
            Pat::StringAt0 => self.escape(p),
        }
    }

    /// `\b(…)\b` or `\b(?i:…)\b` for the word pattern whose bit is `bit`: the
    /// end of the word at `p` when the pattern lists it.
    ///
    /// Every word is made of `\w`, so an alternative can only match the whole
    /// word.
    fn listed(&self, p: usize, bit: u64) -> Option<usize> {
        let word = self.word(p);
        if word.bits & bit == 0 {
            return None;
        }
        word.end
    }

    /// The word at `p` and the patterns that list it, looked up once for
    /// every word pattern that asks about it.
    fn word(&self, p: usize) -> Word {
        let kept = self.x.word.get();
        if kept.start == p {
            return kept;
        }
        let mut word = Word {
            start: p,
            end: None,
            bits: 0,
        };
        if self.boundary(p) {
            let end = self.word_end(p);
            let text = self.text(p, end);
            let mut letters = [0; words::LONGEST_FOLDED];
            let folded = case_fold::lower_into(text, &mut letters)
                .and_then(|len| letters.get(..len))
                .map_or(0, |lower| bits_of(words::FOLDED, lower));
            word.end = Some(end);
            word.bits = bits_of(words::WORDS, text) | folded;
        }
        self.x.word.set(word);
        word
    }

    /// `([Ff])?\b` at `i`.
    fn suffix(&self, i: usize) -> Option<usize> {
        if matches!(self.at(i), b'F' | b'f') && self.boundary(i + 1) {
            return Some(i + 1);
        }
        self.boundary(i).then_some(i)
    }

    /// `\b[0-9]+\.[0-9]*([Ff])?\b`.
    ///
    /// When the suffix fails, giving back digits of the fraction leaves a
    /// digit before a digit, where no `\b` stands, until none is left.
    /// Then the `\b` holds between the `.` and the first digit, so `1.5x`
    /// matches `1.`.
    /// Giving back digits before the `.` would put a digit where the `.` must
    /// stand.
    fn float(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let dot = self.skip(p, |b| b.is_ascii_digit());
        if dot == p || self.at(dot) != b'.' {
            return None;
        }
        let digits = self.skip(dot + 1, |b| b.is_ascii_digit());
        self.suffix(digits)
            .or_else(|| (digits > dot + 1).then_some(dot + 1))
    }

    /// `(\.([0-9]+)([Ff])?)\b`, with no `\b` before the `.`.
    ///
    /// Giving back a digit leaves a digit before a digit, where no `\b`
    /// stands, and `[0-9]+` keeps at least one.
    fn fraction(&self, p: usize) -> Option<usize> {
        let start = self.lit(p, b".")?;
        let digits = self.skip(start, |b| b.is_ascii_digit());
        if digits == start {
            return None;
        }
        self.suffix(digits)
    }

    /// `\b([0-9]+([Ff])?)\b`.
    ///
    /// Giving back a digit leaves a digit before a digit, where no `\b`
    /// stands.
    fn integer(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let digits = self.skip(p, |b| b.is_ascii_digit());
        if digits == p {
            return None;
        }
        self.suffix(digits)
    }

    /// `\b(0([Xx])\h+)\b`.
    ///
    /// Giving back a hex digit leaves a hex digit, a `\w`, before the `\b`.
    fn hex(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) || !matches!(self.lit(p, b"0").map(|x| self.at(x)), Some(b'X' | b'x'))
        {
            return None;
        }
        let digits = self.skip(p + 2, |b| b.is_ascii_hexdigit());
        (digits > p + 2 && self.boundary(digits)).then_some(digits)
    }

    /// `^\s*#\s*(define|elif|…)`: a directive at the start of the line, the
    /// first of the grammar's order that the text starts with.
    ///
    /// No `\b` follows the directive, so `#ifx` matches `#if`.
    /// Giving back a blank of either `\s*` would put a blank where the `#` or
    /// the directive must stand.
    fn directive(&self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let hash = self.space_end(p);
        let name = self.space_end(self.lit(hash, b"#")?);
        words::DIRECTIVES
            .iter()
            .find_map(|directive| self.lit(name, directive))
    }

    /// `\b([A-Z_a-z][0-9A-Z_a-z]*)(?=\s*\()`: a name before a `(`.
    ///
    /// Giving back a character of the name would leave a name character where
    /// the blanks or the `(` must stand.
    fn function(&self, p: usize) -> Option<usize> {
        let first = self.at(p);
        if !(first == b'_' || first.is_ascii_alphabetic()) || !self.boundary(p) {
            return None;
        }
        let end = self.skip(p + 1, is_word);
        let after = self.space_end(end);
        (after < self.lim() && self.at(after) == b'(').then_some(end)
    }

    /// `(?<=:\s?)(?i:…)\b` for a semantic list: the end of the word at `p`
    /// when it is one of `alone`, or one of `digits` followed by ASCII digits.
    ///
    /// No `\b` stands before the word, but the character before it is a `:`
    /// or a blank.
    /// Every word of `digits` ends in a letter or `_`, so the word's trailing
    /// digits are all the `[0-9]*`.
    /// Giving back one of them would leave a digit before the `\b`.
    ///
    /// The stem is folded without its digits, so a digit run of any length
    /// still matches, as `[0-9]*` does.
    /// A Kody review of
    /// [pull request 612](https://github.com/victor-software-house/nishiki-archive/pull/612)
    /// found this.
    fn semantic(&self, p: usize, alone: &[&[u8]], digits: &[&[u8]]) -> Option<usize> {
        if !self.after_colon(p) {
            return None;
        }
        let end = self.word_end(p);
        let text = self.text(p, end);
        let mut letters = [0; words::LONGEST_FOLDED];
        if let Some(len) = case_fold::lower_into(text, &mut letters)
            && holds(alone, letters.get(..len)?)
        {
            return Some(end);
        }
        let stem = text
            .iter()
            .rposition(|b| !b.is_ascii_digit())
            .map_or(0, |last| last + 1);
        let len = case_fold::lower_into(text.get(..stem)?, &mut letters)?;
        holds(digits, letters.get(..len)?).then_some(end)
    }

    /// The lookbehind `(?<=:\s?)` at `p`: a `:` right before it, or a `:` and
    /// one `\s` character.
    fn after_colon(&self, p: usize) -> bool {
        if p == 0 {
            return false;
        }
        if self.prev_is(p, b':') {
            return true;
        }
        let blank = self.char_start_before(p);
        self.space_at(blank) == p - blank && self.prev_is(blank, b':')
    }

    /// `\\.` in a string: a backslash and the character after it, which is not
    /// the line feed.
    fn escape(&self, p: usize) -> Option<usize> {
        let next = self.lit(p, b"\\")?;
        (next < self.lim()).then(|| self.char_end(next))
    }

    /// Where the scan goes on after no pattern matched at `p`.
    ///
    /// Inside a run of ASCII `\w`, no `\b` stands, so no word, number or name
    /// pattern can start there.
    /// A fraction starts at a `.`, a semantic after a `:` or a blank, and a
    /// directive at the start of the line.
    ///
    /// The rules hold only their ends and the string's escape, which start
    /// with no `\w`, and the line comment's `$`, which a run cannot pass.
    pub(super) fn after_missed_word(&self, p: usize) -> usize {
        if !is_word(self.at(p)) {
            return self.char_end(p);
        }
        self.skip(p + 1, is_word)
    }
}
