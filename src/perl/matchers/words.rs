//! Declarations, keywords, the table of built-in functions and the other
//! words and marks of the root.

use std::cmp::Ordering;

use super::super::engine::Lexer;
use super::super::words::{FUNCTION_ENDS, FUNCTIONS};
use crate::rules::Chars;

/// The keywords of `(?<!->)\b(continue|…|eval)\b`.
const CONTROL: [&[u8]; 27] = [
    b"continue",
    b"default",
    b"die",
    b"do",
    b"else",
    b"elsif",
    b"exit",
    b"for",
    b"foreach",
    b"given",
    b"goto",
    b"if",
    b"last",
    b"next",
    b"redo",
    b"return",
    b"select",
    b"unless",
    b"until",
    b"wait",
    b"when",
    b"while",
    b"switch",
    b"case",
    b"require",
    b"use",
    b"eval",
];

/// `[-0-9A-Z_a-z]`: a character of a sub's name.
fn is_sub_name(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b'-'
}

/// Word `at` of [`FUNCTIONS`].
fn function_word(at: usize) -> &'static [u8] {
    let start = at
        .checked_sub(1)
        .and_then(|before| FUNCTION_ENDS.get(before))
        .map_or(0, |&end| usize::from(end));
    let end = FUNCTION_ENDS.get(at).map_or(start, |&end| usize::from(end));
    FUNCTIONS.as_bytes().get(start..end).unwrap_or(&[])
}

/// Whether `word` is one of [`FUNCTIONS`].
fn is_function(word: &[u8]) -> bool {
    let (mut low, mut high) = (0, FUNCTION_ENDS.len());
    while low < high {
        let mid = low + (high - low) / 2;
        match function_word(mid).cmp(word) {
            Ordering::Less => low = mid + 1,
            Ordering::Greater => high = mid,
            Ordering::Equal => return true,
        }
    }
    false
}

impl Lexer<'_> {
    /// `\b(ARGV|DATA|…|write|y)\b`: a built-in function or handle.
    /// Every word of the list is a whole run of `\w`, so the run at `p` is
    /// looked up.
    pub(super) fn function(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let end = self.word_end(p);
        (end > p && is_function(self.text(p, end))).then_some(end)
    }

    /// `(?<!->)\b(continue|…|eval)\b`: a control keyword, not after `->`.
    pub(super) fn control_keyword(&self, p: usize) -> Option<usize> {
        if p >= 2 && self.starts(p - 2, b"->") {
            return None;
        }
        self.word_in(p, &CONTROL)
    }

    /// `\b(\w+)\s*(?==>)`: a word before `=>`.
    /// The word gives nothing back: a shorter one leaves a word character
    /// where `=>` must be.
    pub(super) fn hash_key(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) || self.word_at(p).is_none() {
            return None;
        }
        let end = self.word_end(p);
        let at = self.space_end(end);
        self.starts(at, b"=>").then(|| {
            self.capture(1, p, end);
            at
        })
    }

    /// `(?<=\{)\s*\w+\s*(?=})`: a word alone in braces.
    pub(super) fn braced_word(&self, p: usize) -> Option<usize> {
        if self.prev(p) != Some(b'{') {
            return None;
        }
        let word = self.space_end(p);
        let end = self.word_end(word);
        let at = self.space_end(end);
        (end > word && self.at(at) == b'}' && at < self.lim()).then_some(at)
    }

    /// `^\s*(package)\s+([^;\s]+)`: a package declaration.
    pub(super) fn package(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let at = self.space_end(0);
        let keyword = self.lit(at, b"package")?;
        let name = self.space_end(keyword);
        let mut end = name;
        while end < self.lim() && self.at(end) != b';' && !self.is_space_at(end) {
            end = self.char_end(end);
        }
        (name > keyword && end > name).then(|| {
            self.capture(1, at, keyword);
            self.capture(2, name, end);
            end
        })
    }

    /// `\b(sub)(?:\s+([-0-9A-Z_a-z]+))?\s*(?:\([$*;@]*\))?[^{\w]`: `sub`, its
    /// name and prototype, and one more character that is neither `{` nor a
    /// word character.
    ///
    /// Each part gives characters back in the order a backtracking matcher
    /// tries them.
    /// The name gives back characters from its end; the last character is
    /// then the next character of the name, which only `-` can be.
    pub(super) fn sub(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let keyword = self.lit(p, b"sub")?;
        let blanks = self.space_end(keyword);
        if blanks > keyword {
            let name = self.skip(blanks, is_sub_name);
            let mut end = name;
            while end > blanks {
                if let Some(tail) = self.sub_tail(end) {
                    self.capture(1, p, keyword);
                    self.capture(2, blanks, end);
                    return Some(tail);
                }
                end -= 1;
            }
        }
        let tail = self.sub_tail(keyword)?;
        self.capture(1, p, keyword);
        Some(tail)
    }

    /// `\s*(?:\([$*;@]*\))?[^{\w]` from `p`.
    /// The blanks give characters back one at a time, and the last of them
    /// then matches `[^{\w]`.
    fn sub_tail(&self, p: usize) -> Option<usize> {
        let blanks = self.space_end(p);
        let prototype = (self.at(blanks) == b'(' && blanks < self.lim())
            .then(|| self.skip(blanks + 1, |b| matches!(b, b'$' | b'*' | b';' | b'@')))
            .filter(|&close| self.at(close) == b')' && close < self.lim())
            .map(|close| close + 1);
        if let Some(after) = prototype
            && let Some(end) = self.not_brace_or_word(after)
        {
            return Some(end);
        }
        if let Some(end) = self.not_brace_or_word(blanks) {
            return Some(end);
        }
        (blanks > p).then_some(blanks)
    }

    /// `[^{\w]` at `p`: the end of one character that is neither `{` nor a
    /// word character, the line feed included.
    fn not_brace_or_word(&self, p: usize) -> Option<usize> {
        if !self.avail(p) || self.at(p) == b'{' || self.word_at(p).is_some() {
            return None;
        }
        Some(if p < self.lim() {
            self.char_end(p)
        } else {
            p + 1
        })
    }

    /// `^(?=(\t| {4}))`: a line indented by a tab or four spaces.
    pub(super) fn indent_start(&self, p: usize) -> Option<usize> {
        (p == 0 && ((self.at(0) == b'\t' && self.lim() > 0) || self.starts(0, b"    ")))
            .then_some(0)
    }

    /// `(?=[^\t\s])`: before a character that is not a blank.
    pub(super) fn indent_end(&self, p: usize) -> Option<usize> {
        (self.avail(p) && self.at(p) != b'\t' && !self.is_space_at(p)).then_some(p)
    }

    /// `(\t| {4})(\t| {4})?`: one or two steps of indentation.
    pub(super) fn indent(&mut self, p: usize) -> Option<usize> {
        let first = self.indent_step(p)?;
        self.capture(1, p, first);
        if let Some(second) = self.indent_step(first) {
            self.capture(2, first, second);
            return Some(second);
        }
        Some(first)
    }

    /// `\t| {4}` at `p`.
    fn indent_step(&self, p: usize) -> Option<usize> {
        if self.at(p) == b'\t' && p < self.lim() {
            Some(p + 1)
        } else {
            self.lit(p, b"    ")
        }
    }

    /// `\b(__(?:FILE|LINE|PACKAGE|SUB)__)\b`: a magic constant.
    pub(super) fn magic_constant(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let at = self.lit(p, b"__")?;
        let name = [b"FILE".as_slice(), b"LINE", b"PACKAGE", b"SUB"]
            .into_iter()
            .find_map(|word| self.lit(at, word))?;
        let end = self.lit(name, b"__")?;
        self.word_at(end).is_none().then(|| {
            self.capture(1, p, end);
            end
        })
    }

    /// `\b(__(?:DATA__|END__))\n?`: the end of the code.
    pub(super) fn data_section(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let end = self
            .lit(p, b"__DATA__")
            .or_else(|| self.lit(p, b"__END__"))?;
        self.capture(1, p, end);
        Some(if end == self.lim() && self.avail(end) {
            end + 1
        } else {
            end
        })
    }

    /// `(?<!\w)-[ABCMORSTWXb-gklopr-uwxz]\b`: a file test, such as `-e`.
    pub(super) fn file_test(&self, p: usize) -> Option<usize> {
        if self.word_before(p) || self.at(p) != b'-' || p + 1 >= self.lim() {
            return None;
        }
        let test = matches!(
            self.at(p + 1),
            b'A' | b'B' | b'C' | b'M' | b'O' | b'R' | b'S' | b'T' | b'W' | b'X' | b'b'..=b'g' | b'k'
                | b'l' | b'o' | b'p' | b'r'..=b'u' | b'w' | b'x' | b'z'
        );
        (test && self.word_at(p + 2).is_none()).then_some(p + 2)
    }

    /// `((?:<=|[-=])>)`: `<=>`, `->` or `=>`.
    pub(super) fn arrow(&mut self, p: usize) -> Option<usize> {
        let end = if self.starts(p, b"<=>") {
            p + 3
        } else if matches!(self.at(p), b'-' | b'=') && self.at(p + 1) == b'>' && p + 1 < self.lim()
        {
            p + 2
        } else {
            return None;
        };
        self.capture(1, p, end);
        Some(end)
    }

    /// `^__\w+__`: a line that starts with a name in double underscores.
    /// `\w+` takes the closing underscores too, and gives them back.
    pub(super) fn dunder_line(&self, p: usize) -> Option<usize> {
        if p != 0 || !self.starts(0, b"__") {
            return None;
        }
        let run = self.word_end(2);
        let first = self.char_end(2).min(run);
        let mut at = run.checked_sub(2)?;
        while at >= first && at > 2 {
            if self.starts(at, b"__") {
                return Some(at + 2);
            }
            at -= 1;
        }
        None
    }

    /// `\b(format)\s+(\w+)\s*=`: a format declaration.
    pub(super) fn format(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let keyword = self.lit(p, b"format")?;
        let name = self.space_end(keyword);
        let end = self.word_end(name);
        let at = self.space_end(end);
        (name > keyword && end > name && self.at(at) == b'=' && at < self.lim()).then(|| {
            self.capture(1, p, keyword);
            self.capture(2, name, end);
            at + 1
        })
    }

    /// `^\.\s*$`: the line that ends a format.
    pub(super) fn format_end(&self, p: usize) -> Option<usize> {
        if p != 0 || self.at(0) != b'.' || self.lim() == 0 {
            return None;
        }
        let end = self.space_end(1);
        self.at_eol(end).then_some(end)
    }

    /// `\b(x)\s*(\d+)\b`: the repetition operator and its count.
    pub(super) fn repetition(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) || self.at(p) != b'x' || p >= self.lim() {
            return None;
        }
        let digits = self.space_end(p + 1);
        let end = self.digit_end(digits);
        (end > digits && self.boundary(end)).then(|| {
            self.capture(1, p, p + 1);
            self.capture(2, digits, end);
            end
        })
    }
}
