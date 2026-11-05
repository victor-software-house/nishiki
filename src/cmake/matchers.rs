//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups of its match with [`Lexer::cap`].
//! The engine names those the grammar names.
//!
//! Most of the grammar is `\b(?i:…)\b` alternations of words, some with `\w+`
//! in them.
//! Their words are in [`super::words`].
//! `(?i)` compares words through [`crate::case_fold`].

use super::engine::{LONGEST_WORD, Lexer, Lowered};
use super::grammar::Pat;
use super::words::{self, Words};
use crate::rules::Chars;
use crate::{bytes, case_fold};

impl Lexer<'_> {
    /// Whether `pat` matches at `p`, and where its match ends.
    pub(super) fn matcher(&mut self, pat: Pat, p: usize) -> Option<usize> {
        match pat {
            Pat::Constants => self.listed(p, &words::CONSTANTS),
            Pat::Keywords => self.listed(p, &words::KEYWORDS),
            Pat::Commands => self.command(p),
            Pat::Variables => self.listed(p, &words::VARIABLES),
            Pat::TargetVariables => self.listed(p, &words::TARGET_VARIABLES),
            Pat::CmakeVariables => self.listed(p, &words::CMAKE_VARIABLES),
            Pat::CacheProperties => self.listed(p, &words::CACHE_PROPERTIES),
            Pat::SourceProperties => self.listed(p, &words::SOURCE_PROPERTIES),
            Pat::TestProperties => self.listed(p, &words::TEST_PROPERTIES),
            Pat::DirectoryProperties => self.listed(p, &words::DIRECTORY_PROPERTIES),
            Pat::GlobalProperties => self.listed(p, &words::GLOBAL_PROPERTIES),
            Pat::TargetProperties => self.listed(p, &words::TARGET_PROPERTIES),
            Pat::CxxFlags => self.listed(p, &words::CXX_FLAGS),
            Pat::BuildName => self.keyword(p, b"BUILD_NAME"),
            Pat::Expansion => self.expansion(p, b"${"),
            Pat::EnvExpansion => self.expansion(p, b"$ENV{"),
            Pat::BracketComment => self.bracket_open(p, b"#["),
            Pat::BracketArgument => self.bracket_open(p, b"["),
            Pat::BracketCommentEnd | Pat::BracketArgumentEnd => self.bracket_close(p),
            Pat::BracketCommentAt0
            | Pat::BracketArgumentAt0
            | Pat::EscapedStringAt0
            | Pat::StringAt0 => self.escape(p),
            Pat::LineComment => (self.at(p) == b'#').then(|| self.lim()),
            Pat::EscapedString | Pat::EscapedStringEnd => self.lit(p, b"\\\""),
            Pat::String | Pat::StringEnd => self.lit(p, b"\""),
        }
    }

    /// `\b(?i:…)\b` for the alternation `words`: the end of the word at `p`
    /// when it is one of the plain words or fits one with `\w+`.
    /// A word followed by characters that are not `\w`, such as
    /// `compile_definitions_<config>`, matches with `\b` after them.
    ///
    /// Every plain word and every part of a word with `\w+` is made of `\w`.
    /// Such an alternative can only match the whole word.
    fn listed(&self, p: usize, words: &Words) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let word = self.lowered(p);
        if word.end > p {
            let found = match word.len.and_then(|len| word.letters.get(..len)) {
                Some(letters) => {
                    words
                        .exact
                        .binary_search_by(|word| bytes::order(word, letters))
                        .is_ok()
                        || words.globs.iter().any(|parts| fits(letters, parts))
                }
                None => words
                    .globs
                    .iter()
                    .any(|parts| self.glob(p, word.end, parts)),
            };
            if found {
                return Some(word.end);
            }
        }
        words.literals.iter().find_map(|literal| {
            self.folded(p, literal)
                .filter(|&after| self.boundary(after))
        })
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

    /// `word` under `(?i)` at `i`.
    fn folded(&self, i: usize, word: &[u8]) -> Option<usize> {
        case_fold::literal_end(self.s, i, word)
    }

    /// Whether the word `from..end` is the literal parts `parts` with one or
    /// more word characters between each two, under `(?i)`.
    /// It reads the text, for a word whose letters do not lower.
    fn glob(&self, from: usize, end: usize, parts: &[&[u8]]) -> bool {
        let Some((first, rest)) = parts.split_first() else {
            return from == end;
        };
        let Some(after) = self
            .s
            .get(..end)
            .and_then(|text| case_fold::literal_end(text, from, first))
        else {
            return false;
        };
        if rest.is_empty() {
            return after == end;
        }
        let mut gap = after;
        while gap < end {
            gap = self.char_end(gap);
            if self.glob(gap, end, rest) {
                return true;
            }
        }
        false
    }

    /// `^\s*\b(?i:add_compile_options|…|write_file)\b`: a command at the
    /// start of a line, with the blanks before it.
    fn command(&self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        self.listed(self.space_end(p), &words::COMMANDS)
    }

    /// `\$\{\w+}` when `open` is `${`, and `\$ENV\{\w+}` when it is `$ENV{`.
    fn expansion(&self, p: usize, open: &[u8]) -> Option<usize> {
        let name = self.lit(p, open)?;
        let close = self.word_end(name);
        (close > name && self.at(close) == b'}').then_some(close + 1)
    }

    /// `#\[(=*)\[` when `open` is `#[`, and `\[(=*)\[` when it is `[`, group 1
    /// around the `=` run, which the end repeats.
    fn bracket_open(&mut self, p: usize, open: &[u8]) -> Option<usize> {
        let equals = self.lit(p, open)?;
        let run = self.skip(equals, |b| b == b'=');
        if self.at(run) != b'[' {
            return None;
        }
        self.cap(1, equals, run);
        Some(run + 1)
    }

    /// `]\1]`: the `]`, the `=` run of the opening bracket and the `]`.
    fn bracket_close(&self, p: usize) -> Option<usize> {
        let run = self.lit(p, b"]")?;
        let close = self.lit(run, self.payload_text().as_bytes())?;
        self.lit(close, b"]")
    }

    /// `\\(.|$)`: a backslash and the character after it, or a backslash at
    /// the end of the line.
    fn escape(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'\\' {
            return None;
        }
        if p + 1 < self.lim() {
            return Some(self.char_end(p + 1));
        }
        self.at_eol(p + 1).then_some(p + 1)
    }
}

/// Whether the lowercase word `letters` is the literal parts `parts` with one
/// or more letters between each two.
///
/// The generator checks that no part starts or ends next to a `\w+` with a
/// letter that a fold such as `ß` or `ﬁ` spells.
/// The lowered letters of one character therefore never fall on both sides of
/// a part's edge.
fn fits(letters: &[u8], parts: &[&[u8]]) -> bool {
    match parts {
        [] => letters.is_empty(),
        [only] => bytes::same(letters, only),
        [first, rest @ ..] => {
            let Some(last) = rest.last() else {
                return false;
            };
            let ends = letters
                .len()
                .checked_sub(last.len())
                .and_then(|start| letters.get(start..))
                .is_some_and(|tail| bytes::same(tail, last));
            if !ends {
                return false;
            }
            let tail = letters
                .get(..first.len())
                .filter(|head| bytes::same(head, first))
                .and_then(|_| letters.get(first.len()..));
            tail.is_some_and(|tail| {
                (1..=tail.len()).any(|gap| tail.get(gap..).is_some_and(|after| fits(after, rest)))
            })
        }
    }
}
