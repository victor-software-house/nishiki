//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups the grammar names with [`Lexer::cap`].
//! The engine names them.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would.
//! Where a quantifier could give characters back, the doc comment says why it
//! cannot matter.
//! `\w`, `\s` and `\d` take non-ASCII characters, as in Oniguruma, through
//! [`Chars`].
//!
//! Most of the grammar is `\b(?i:(…))\b` alternations of words.
//! Their words are in [`super::words`].
//! `(?i)` compares words through [`crate::case_fold`].

use super::engine::{LONGEST_WORD, Lexer, Lowered};
use super::grammar::{Pattern, Rule};
use super::words;
use crate::rules::Chars;
use crate::{bytes, case_fold};

/// Whether `b` is an ASCII `\w` character.
fn is_word(b: u8) -> bool {
    b == b'_' || b.is_ascii_alphanumeric()
}

/// Where a routine's head ends: the routine word, then the name after it.
#[derive(Clone, Copy, Debug)]
struct Head {
    /// The end of `function`, `procedure` or another routine word.
    word_end: usize,
    /// Where the name starts.
    name: usize,
    /// Where the name ends, with its `.` part.
    name_end: usize,
}

impl Lexer<'_> {
    /// Whether `pattern` matches at `p`, and where its match ends.
    pub(super) fn matcher(&mut self, pattern: Pattern, p: usize) -> Option<usize> {
        match pattern {
            Pattern::Keywords => self.listed(p, words::KEYWORDS),
            Pattern::Prototype => self.prototype(p),
            Pattern::Function => {
                let head = self.head(p, words::FUNCTION)?;
                self.cap(1, p, head.word_end);
                self.cap(2, head.name, head.name_end);
                Some(head.name_end)
            }
            Pattern::Variable => self.listed(p, words::VARIABLE),
            Pattern::Operator => self.listed(p, words::OPERATOR),
            Pattern::Control => self.listed(p, words::CONTROL),
            Pattern::Types => self.listed(p, words::TYPES),
            Pattern::Constant => self.listed(p, words::CONSTANT),
            Pattern::Assert => self.listed(p, words::ASSERT),
            Pattern::Number => self.number(p),
            Pattern::Hex => self.hex(p),
            Pattern::Directive => self.lit(p, b"{$"),
            Pattern::BraceComment => {
                (self.at(p) == b'{' && self.at(p + 1) != b'$').then_some(p + 1)
            }
            Pattern::DirectiveEnd | Pattern::BraceCommentEnd => self.lit(p, b"}"),
            Pattern::LineComment => self.line_comment(p),
            Pattern::LineCommentEnd => (!self.at_g(p)).then_some(p),
            Pattern::LineCommentAt0 => self.lit(p, b"//"),
            Pattern::LineCommentAt0End => self.line_feed(p),
            Pattern::ParenComment => self.lit(p, b"(*"),
            Pattern::ParenCommentEnd => self.lit(p, b"*)"),
            Pattern::String | Pattern::StringEnd => self.lit(p, b"'"),
            Pattern::StringAt0 => self.lit(p, b"''"),
            Pattern::CharCode => {
                let digits = self.lit(p, b"#")?;
                let end = self.digit_end(digits);
                (end > digits).then_some(end)
            }
        }
    }

    /// `\b(?i:(…))\b` for the alternation `words`: the end of the word at `p`
    /// when it is one of them.
    ///
    /// Every word is made of `\w`, so an alternative can only match the whole
    /// word.
    fn listed(&self, p: usize, words: &[&[u8]]) -> Option<usize> {
        debug_assert!(
            words.is_sorted(),
            "a word list is sorted for its binary search"
        );
        if !self.boundary(p) {
            return None;
        }
        let word = self.lowered(p);
        let letters = word.letters.get(..word.len?)?;
        let listed = words
            .binary_search_by(|word| bytes::order(word, letters))
            .is_ok();
        (word.end > p && listed).then_some(word.end)
    }

    /// The word at `p` and its letters, lowered once for every word pattern
    /// that asks about it.
    fn lowered(&self, p: usize) -> Lowered {
        let kept = self.x.lowered.get();
        if kept.start == p {
            return kept;
        }
        let end = self.word_end(p);
        let mut word = Lowered {
            start: p,
            end,
            len: None,
            letters: [0; LONGEST_WORD],
        };
        word.len = case_fold::lower_into(self.text(p, end), &mut word.letters);
        self.x.lowered.set(word);
        word
    }

    /// `\b(?i:(…))\b\s+(\w+(\.\w+)?)` for the routine words `words`.
    ///
    /// Giving back a blank of `\s+` would start the name at a blank.
    /// Giving back a character of a `\w+` would leave a `\w` where a `.` or
    /// the end must stand.
    fn head(&self, p: usize, words: &[&[u8]]) -> Option<Head> {
        let word_end = self.listed(p, words)?;
        let name = self.space_end(word_end);
        if name == word_end {
            return None;
        }
        let mut name_end = self.word_end(name);
        if name_end == name {
            return None;
        }
        if self.at(name_end) == b'.' {
            let dotted = self.word_end(name_end + 1);
            if dotted > name_end + 1 {
                name_end = dotted;
            }
        }
        Some(Head {
            word_end,
            name,
            name_end,
        })
    }

    /// `\b(?i:(function|procedure|constructor|destructor))\b\s+(\w+(\.\w+)?)(\
    /// (.*?\))?;\s*(?=(?i:attribute|forward|external))`: a routine declared
    /// ahead of its body.
    ///
    /// The lazy `.*?` tries each `)` in turn, the nearest first.
    /// Without the parentheses, the `;` must follow the name.
    fn prototype(&mut self, p: usize) -> Option<usize> {
        let head = self.head(p, words::PROTOTYPE)?;
        let end = match self.at(head.name_end) {
            b';' => self.ahead(head.name_end + 1),
            b'(' => (head.name_end + 1..self.lim())
                .filter(|&close| self.at(close) == b')' && self.at(close + 1) == b';')
                .find_map(|close| self.ahead(close + 2)),
            _ => None,
        }?;
        self.cap(1, p, head.word_end);
        self.cap(2, head.name, head.name_end);
        Some(end)
    }

    /// `\s*(?=(?i:attribute|forward|external))` at `i`.
    ///
    /// Giving back a blank of `\s*` would put a blank where the lookahead
    /// wants a letter.
    fn ahead(&self, i: usize) -> Option<usize> {
        let end = self.space_end(i);
        words::PROTOTYPE_AHEAD
            .iter()
            .any(|word| case_fold::literal_end(self.s, end, word).is_some())
            .then_some(end)
    }

    /// `\b(\d+)|(\d*\.\d+([Ee][-+]?\d+)?)\b`.
    ///
    /// The first branch has no `\b` after it, and the second none before it.
    /// So `12ab` matches `12`, and `x1.5` matches `1.5` after the `x`.
    /// Giving back a digit of the second branch would leave a digit before the
    /// `.` or before the `\b`.
    fn number(&self, p: usize) -> Option<usize> {
        let digits = self.digit_end(p);
        if digits > p && self.boundary(p) {
            return Some(digits);
        }
        let fraction = self.lit(digits, b".")?;
        let fraction_end = self.digit_end(fraction);
        if fraction_end == fraction {
            return None;
        }
        if let Some(exponent) = self.exponent(fraction_end)
            && self.boundary(exponent)
        {
            return Some(exponent);
        }
        self.boundary(fraction_end).then_some(fraction_end)
    }

    /// `[Ee][-+]?\d+` at `i`.
    fn exponent(&self, i: usize) -> Option<usize> {
        if !matches!(self.at(i), b'E' | b'e') {
            return None;
        }
        let sign = i + 1 + usize::from(matches!(self.at(i + 1), b'+' | b'-'));
        let end = self.digit_end(sign);
        (end > sign).then_some(end)
    }

    /// `\$\h{1,16}\b`.
    ///
    /// Giving back a hex digit would leave a hex digit, a `\w`, before the
    /// `\b`.
    fn hex(&self, p: usize) -> Option<usize> {
        let start = self.lit(p, b"$")?;
        let end = self.skip(start, |b| b.is_ascii_hexdigit()).min(start + 16);
        (end > start && self.boundary(end)).then_some(end)
    }

    /// `(^[\t ]+)?(?=//)`: the blanks that start a line before `//`, as group
    /// 1, or nothing before a `//` anywhere.
    fn line_comment(&mut self, p: usize) -> Option<usize> {
        if p == 0 {
            let blanks = self.skip(0, |b| b == b' ' || b == b'\t');
            if blanks > 0 && self.lit(blanks, b"//").is_some() {
                self.cap(1, 0, blanks);
                return Some(blanks);
            }
        }
        self.lit(p, b"//").map(|_| p)
    }

    /// `\n`: the line feed that ends the line.
    fn line_feed(&self, p: usize) -> Option<usize> {
        (p == self.lim() && self.nl).then_some(p + 1)
    }

    /// Where the scan goes on after no pattern matched at `p`.
    ///
    /// Inside a run of ASCII `\w`, no `\b` stands, so no word pattern can
    /// start there.
    /// Only the number's second branch, `\d*\.\d+`, needs no `\b` before it.
    /// It can start at the run's trailing digits when a `.` or a character
    /// above ASCII follows the run.
    ///
    /// Every digit of those digits reads the same rest, so the first one
    /// decides for all.
    /// The other rules hold only their ends, and `''` in a string, which
    /// start with no `\w`.
    pub(super) fn after_missed_word(&self, p: usize) -> usize {
        if !is_word(self.at(p)) {
            return self.char_end(p);
        }
        let end = self.skip(p + 1, is_word);
        let after = self.at(end);
        if self.top_rule() != Rule::Root || (after != b'.' && after.is_ascii()) {
            return end;
        }
        let mut digits = end;
        while digits > p && self.at(digits - 1).is_ascii_digit() {
            digits -= 1;
        }
        if p < digits && digits < end {
            digits
        } else {
            end
        }
    }
}
