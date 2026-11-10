//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups the grammar names with
//! [`Lexer::capture`].
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would.
//! Where a quantifier could give characters back, the doc comment shows why it
//! cannot matter.
//! `\w`, `\s` and `\b` take non-ASCII characters, as in Oniguruma, through
//! [`Chars`].
//! `[0-9]` and `[A-Za-z]` are ASCII.
//!
//! Most of the grammar is `\b(…)\b` and `\b(?i:…)\b` alternations of words.
//! Their words are in [`super::words`], each with the bits of the patterns
//! that list it.
//! `(?i)` compares words through [`crate::case_fold`].

use super::engine::{Lexer, Word};
use super::grammar::Pattern;
use super::words;
use crate::rules::Chars;
use crate::{bytes, case_fold};

/// Whether `b` is an ASCII `\w` character.
fn is_word(b: u8) -> bool {
    b == b'_' || b.is_ascii_alphanumeric()
}

/// Whether `b` may start a name, `[A-Z_a-z]`.
fn starts_name(b: u8) -> bool {
    b == b'_' || b.is_ascii_alphabetic()
}

/// The bits a sorted word table gives `word`, or 0 when it lacks it.
fn bits_of(table: &[(&[u8], u64)], word: &[u8]) -> u64 {
    table
        .binary_search_by(|&(listed, _)| bytes::order(listed, word))
        .ok()
        .and_then(|index| table.get(index))
        .map_or(0, |&(_, bits)| bits)
}

impl Lexer<'_> {
    /// Whether `pattern` matches at `p`, and where its match ends.
    pub(super) fn matcher(&mut self, pattern: Pattern, p: usize) -> Option<usize> {
        match pattern {
            Pattern::LineComment => self.lit(p, b"//"),
            Pattern::LineCommentEnd => self.at_eol(p).then_some(p),
            Pattern::Number => self.number(p),
            Pattern::Reference => self.reference(p),
            Pattern::Attribute => self.lit(p, b"["),
            Pattern::AttributeEnd => self.lit(p, b"]"),
            Pattern::AttributeAt0 => self.attribute_name(p),
            Pattern::Declaration => self.declaration(p),
            Pattern::CgBlock => self.block_keyword(p, b"CG"),
            Pattern::HlslBlock => self.block_keyword(p, b"HLSL"),
            Pattern::CgBlockEnd => self.end_keyword(p, b"ENDCG"),
            Pattern::HlslBlockEnd => self.end_keyword(p, b"ENDHLSL"),
            Pattern::String | Pattern::StringEnd => self.lit(p, b"\""),
            Pattern::BasicType
            | Pattern::Structure
            | Pattern::PropertyName
            | Pattern::PropertyValue
            | Pattern::ComparisonFunction
            | Pattern::StencilOperation
            | Pattern::TextureCombiners
            | Pattern::Fog
            | Pattern::BindChannels
            | Pattern::BlendOperations
            | Pattern::BlendFactors
            | Pattern::Fixed
            | Pattern::Transformations
            | Pattern::Camera
            | Pattern::Time
            | Pattern::Lighting
            | Pattern::FogVariable
            | Pattern::Various
            | Pattern::TargetPlatform
            | Pattern::TargetModel
            | Pattern::UnityVersion
            | Pattern::PlatformDifference
            | Pattern::Texture2D
            | Pattern::Structures
            | Pattern::Surface => self.listed(p, words::bit(pattern)),
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

    /// `\b([0-9]+\.?[0-9]*)\b`.
    ///
    /// When the `\b` fails after the fraction, giving back its digits leaves
    /// a digit before a digit, where no `\b` stands, until none is left.
    /// Then the `\b` holds between the `.` and the first digit, so `1.5x`
    /// matches `1.`.
    ///
    /// With no fraction digits, giving back the `.` leaves a `\b` between the
    /// last digit and the `.`, so `1. ` matches `1`.
    /// Giving back an integer digit leaves a digit before a digit.
    fn number(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let dot = self.skip(p, |b| b.is_ascii_digit());
        if dot == p {
            return None;
        }
        if self.at(dot) != b'.' {
            return self.boundary(dot).then_some(dot);
        }
        let digits = self.skip(dot + 1, |b| b.is_ascii_digit());
        if self.boundary(digits) {
            return Some(digits);
        }
        Some(if digits > dot + 1 { dot + 1 } else { dot })
    }

    /// `\[([A-Z_a-z][0-9A-Z_a-z]*)](?!\s*[A-Z_a-z][0-9A-Z_a-z]*\s*\(")`: a
    /// bracketed name, unless a name, a `(` and a `"` follow it.
    ///
    /// The name's run is fixed: a `]` must follow it.
    /// In the lookahead, giving back a blank or a name character would leave
    /// one where a name or a `(` must stand.
    fn reference(&self, p: usize) -> Option<usize> {
        let name = self.lit(p, b"[")?;
        if !starts_name(self.at(name)) {
            return None;
        }
        let close = self.skip(name + 1, is_word);
        let end = self.lit(close, b"]")?;
        let next = self.space_end(end);
        let follows = starts_name(self.at(next)) && {
            let paren = self.space_end(self.skip(next + 1, is_word));
            self.starts(paren, b"(\"")
        };
        (!follows).then_some(end)
    }

    /// `\G([A-Za-z]+)\b` right after an attribute's `[`.
    ///
    /// Giving back a letter would leave a letter, a `\w`, before the `\b`.
    fn attribute_name(&self, p: usize) -> Option<usize> {
        if !self.at_g(p) {
            return None;
        }
        let end = self.skip(p, |b| b.is_ascii_alphabetic());
        (end > p && self.boundary(end)).then_some(end)
    }

    /// `\b([A-Z_a-z][0-9A-Z_a-z]*)\s*\(`: a name and its `(`, blanks between.
    ///
    /// Giving back a name character would leave one where the blanks or the
    /// `(` must stand.
    fn declaration(&self, p: usize) -> Option<usize> {
        if !starts_name(self.at(p)) || !self.boundary(p) {
            return None;
        }
        let paren = self.space_end(self.skip(p + 1, is_word));
        self.lit(paren, b"(")
    }

    /// `\b(<prefix>(?:PROGRAM|INCLUDE))\b`, the whole word as group 1.
    fn block_keyword(&mut self, p: usize, prefix: &[u8]) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let rest = self.lit(p, prefix)?;
        let end = self
            .lit(rest, b"PROGRAM")
            .or_else(|| self.lit(rest, b"INCLUDE"))?;
        if !self.boundary(end) {
            return None;
        }
        self.capture(1, p, end);
        Some(end)
    }

    /// `\b(<word>)\b`, the word as group 1.
    fn end_keyword(&mut self, p: usize, word: &[u8]) -> Option<usize> {
        let end = self.keyword(p, word)?;
        self.capture(1, p, end);
        Some(end)
    }

    /// Where the scan goes on after no pattern matched at `p`.
    ///
    /// Inside a run of ASCII `\w`, no `\b` stands, so no word, number, name or
    /// block pattern can start there.
    /// A reference and an attribute start at a `[`, a comment and a string at
    /// a mark, and an attribute's name right after its `[`.
    /// The block ends and Unity's words need a `\b` too, and a comment's `$`
    /// cannot fall inside a run.
    pub(super) fn after_missed_word(&self, p: usize) -> usize {
        if !is_word(self.at(p)) {
            return self.char_end(p);
        }
        self.skip(p + 1, is_word)
    }
}
