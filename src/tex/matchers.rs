//! Where each of TeX's regular expressions matches.
//!
//! The matchers are generic over the grammar, so LaTeX runs them for the TeX
//! rules its grammar includes.

use super::grammar::Rx;
use super::words::{CONTROL, FUNCTION, OPERATOR, SYMBOL, WORD_BYTES, WORD_ENDS, WORD_LISTS};
use crate::classes::is_alpha;
use crate::rules::{Chars, Grammar, Lexer};

/// The argument types of an expl3 function name, after its `:`.
const ARGUMENT_TYPES: &[u8] = b"DFNTVcefnopvwx";

/// The delimiters a `\left`, `\right` or `\big` takes as one character.
const DELIMITERS: &[u8] = b"]().<>[|";

impl<G: Grammar> Lexer<'_, G> {
    /// Whether `rx` matches at `p`, and where its match ends.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm per expression: the table of what each matches"
    )]
    pub(crate) fn tex_matcher(&mut self, rx: Rx, p: usize) -> Option<usize> {
        match rx {
            Rx::IffalseBlock => {
                let end = self.tex_line_start_command(p, b"\\iffalse")?;
                (!self.tex_fi_follows(end)).then(|| {
                    self.capture(1, p, end);
                    self.capture(2, p, p + 1);
                    end
                })
            }
            Rx::IffalseBlockEnd => {
                let end = self.lit(p, b"\\else").or_else(|| self.lit(p, b"\\fi"))?;
                self.boundary(end).then(|| {
                    self.capture(1, p, end);
                    self.capture(2, p, p + 1);
                    end
                })
            }
            Rx::Comment => {
                if p == 0 {
                    let blanks = self.skip(0, |b| b == b' ' || b == b'\t');
                    if blanks > 0 && self.at(blanks) == b'%' && blanks < self.lim() {
                        self.capture(1, 0, blanks);
                        return Some(blanks);
                    }
                }
                (self.at(p) == b'%' && p < self.lim()).then_some(p)
            }
            Rx::CommentEnd => (!self.at_g(p)).then_some(p),
            Rx::CommentAt0 => {
                let end = self.lit(p, b"%")?;
                Some(self.lit(end, b":").unwrap_or(end))
            }
            Rx::CommentAt0End => self.tex_line_end(p),
            Rx::CommentAt1 => {
                if p != 0 {
                    return None;
                }
                let name = self.lit(0, b"%!TEX ")?;
                let value = self.tex_non_space_run(name);
                let end = self.lit(value, b" =")?;
                self.capture(1, 0, name - 1);
                Some(end)
            }
            Rx::Braces => self.tex_unescaped(p, b'{'),
            Rx::BracesEnd => self.tex_unescaped(p, b'}'),
            Rx::Conditionals => {
                let end = self.tex_line_start_command(p, b"\\if")?;
                if self.at(end) == b'f' && end < self.lim() && self.boundary(end + 1) {
                    return None;
                }
                Some(self.skip(end, |b| b.is_ascii_lowercase()))
            }
            Rx::ConditionalsEnd => self.tex_line_start_command(p, b"\\fi"),
            Rx::MacroControl => self.tex_listed_command(p, CONTROL),
            Rx::Catcode => {
                let name = self.lit(p, b"\\catcode`")?;
                let code = [self.lit(name, b"\\"), Some(name)]
                    .into_iter()
                    .flatten()
                    .find_map(|after| {
                        let char_end = self.tex_any_char(after)?;
                        let equals = self.lit(char_end, b"=")?;
                        let digits = self.digit_end(equals);
                        (digits > equals).then_some((char_end, equals, digits))
                    });
                let (char_end, equals, digits) = code?;
                self.capture(1, p, name - 1);
                self.capture(2, p, p + 1);
                self.capture(3, char_end, equals);
                self.capture(4, equals, digits);
                Some(digits)
            }
            Rx::RootAt4 => (matches!(self.at(p), b'[' | b']') && p < self.lim()).then_some(p + 1),
            Rx::DollarMath => {
                let end = self.lit(p, b"$")?;
                let end = self.lit(end, b"$").unwrap_or(end);
                self.capture(1, p, end);
                Some(end)
            }
            Rx::DollarMathEnd => {
                let end = p + self.payload_at(p)?;
                self.capture(1, p, end);
                Some(end)
            }
            Rx::DollarMathAt0 => self.lit(p, b"\\$"),
            Rx::MathContentAt0 => {
                let name = self.lit(p, b"\\text").or_else(|| self.lit(p, b"\\mbox"))?;
                let end = self.lit(name, b"{")?;
                self.capture(1, p, name);
                self.capture(2, p, p + 1);
                self.capture(3, name, end);
                Some(end)
            }
            Rx::MathContentAt0End => self.lit(p, b"}"),
            Rx::RootAt6 => self.lit(p, b"\\\\"),
            Rx::Ifnextchar => {
                let end = self.lit(p, b"\\@ifnextchar")?;
                (matches!(self.at(end), b'(' | b'[' | b'{') && end < self.lim()).then_some(end + 1)
            }
            Rx::MacroGeneralAt0 => self.tex_expl3_name(p, b'\\'),
            Rx::MacroGeneralAt1 => self.tex_expl3_name(p, b'.'),
            Rx::MacroGeneralAt2 => {
                let name = self.lit(p, b"\\")?;
                let end = if matches!(self.at(name), b',' | b';') && name < self.lim() {
                    name + 1
                } else {
                    self.tex_letter_run(name)
                };
                (end > name).then(|| {
                    self.capture(1, p, name);
                    end
                })
            }
            Rx::MacroGeneralAt3 => {
                let name = self.lit(p, b"\\")?;
                let end = if name == self.lim() {
                    self.avail(name).then_some(name + 1)?
                } else {
                    let (c, end) = self.char_at(name)?;
                    (!c.is_ascii() || !Self::tex_command_letter(self.at(name))).then_some(end)?
                };
                self.capture(1, p, name);
                Some(end)
            }
            Rx::MathContentAt1 => {
                let end = self.lit(p, b"\\")?;
                (matches!(self.at(end), b'{' | b'}') && end < self.lim()).then_some(end + 1)
            }
            Rx::MathContentAt2 => self.tex_sized_delimiter(p),
            Rx::MathContentAt3 => self.tex_listed_command(p, SYMBOL),
            Rx::MathContentAt4 => self.tex_listed_word(p, OPERATOR),
            Rx::MathContentAt5 => self.tex_listed_word(p, FUNCTION),
            Rx::MathContentAt6 => {
                let end = self.lit(p, b"\\Sexpr{")?;
                self.capture(1, p, end);
                self.capture(2, p, p + 1);
                self.capture(3, end - 1, end);
                Some(end)
            }
            Rx::MathContentAt6End => {
                let end = self.lit(p, b"}")?;
                self.capture(1, p, end);
                self.capture(2, p, end);
                self.capture(3, p, end);
                Some(end)
            }
            Rx::MathContentAt6At0 => {
                let closes = self.at(p) == b'}' && p < self.lim();
                (self.at_g(p) && !closes).then_some(p)
            }
            Rx::MathContentAt6At0End => (self.at(p) == b'}' && p < self.lim()).then_some(p),
            Rx::MathContentAt7 => {
                let name = self.lit(p, b"\\")?;
                if self.lit(name, b"begin{").is_some() || self.lit(name, b"verb").is_some() {
                    return None;
                }
                let end = self.skip(name, |b| b.is_ascii_alphabetic());
                (end > name).then(|| {
                    self.capture(1, p, name);
                    end
                })
            }
            Rx::MathContentAt10 => self.tex_unescaped(p, b'('),
            Rx::MathContentAt11 => self.tex_unescaped(p, b')'),
            Rx::MathContentAt12 => {
                let digits = self.tex_digit_run(p);
                if self.at(digits) == b'.' && digits < self.lim() {
                    let fraction = self.tex_digit_run(digits + 1);
                    if fraction > digits + 1 {
                        return Some(fraction);
                    }
                }
                (digits > p).then_some(digits)
            }
            Rx::MathContentAt13 => match self.at(p) {
                b'-' | b'*' | b'+' | b'/' if p < self.lim() => Some(p + 1),
                b @ (b'^' | b'_') if p < self.lim() => {
                    (!self.prev_is(p, b) && self.at(p + 1) != b).then_some(p + 1)
                }
                _ => None,
            },
        }
    }

    /// `(?<=^\s*)` and then `command` at `p`: where it ends.
    fn tex_line_start_command(&self, p: usize, command: &[u8]) -> Option<usize> {
        let mut i = 0;
        while i < p {
            let width = self.space_at(i);
            if width == 0 {
                return None;
            }
            i += width;
        }
        self.lit(p, command)
    }

    /// The end of the run of `\S` characters from `i`.
    fn tex_non_space_run(&self, mut i: usize) -> usize {
        while i < self.lim() && self.space_at(i) == 0 {
            i = self.char_end(i);
        }
        i
    }

    /// `\s*[{}]\s*\\fi\b` at `i`, what keeps an `\iffalse` from opening a
    /// block.
    fn tex_fi_follows(&self, i: usize) -> bool {
        let brace = self.space_end(i);
        if !matches!(self.at(brace), b'{' | b'}') || brace >= self.lim() {
            return false;
        }
        let fi = self.space_end(brace + 1);
        self.lit(fi, b"\\fi").is_some_and(|end| self.boundary(end))
    }

    /// `$\n?` at `p`: the end of the line, with its line feed.
    fn tex_line_end(&self, p: usize) -> Option<usize> {
        if p == self.lim() {
            return Some(if self.avail(p) { p + 1 } else { p });
        }
        self.at_eol(p).then_some(p)
    }

    /// `(?<!\\)` and then `byte` at `p`.
    fn tex_unescaped(&self, p: usize, byte: u8) -> Option<usize> {
        (self.at(p) == byte && p < self.lim() && !self.prev_is(p, b'\\')).then_some(p + 1)
    }

    /// The regex `.` at `i`: any character but the line feed.
    fn tex_any_char(&self, i: usize) -> Option<usize> {
        self.char_at(i).map(|(_, end)| end)
    }

    /// Whether `b` is in `[@-Za-z]`, the bytes that continue a command name.
    fn tex_command_letter(b: u8) -> bool {
        matches!(b, b'@'..=b'Z' | b'a'..=b'z')
    }

    /// The end of the run of `[@\p{Alphabetic}]` characters from `i`.
    fn tex_letter_run(&self, mut i: usize) -> usize {
        while let Some((c, end)) = self.char_at(i)
            && (c == '@' || is_alpha(c))
        {
            i = end;
        }
        i
    }

    /// The end of the run of `[0-9]` from `i`.
    fn tex_digit_run(&self, i: usize) -> usize {
        self.skip(i, |b| b.is_ascii_digit())
    }

    /// `(\\)(words)(?![@-Za-z])` at `p`, with the words of `list`: a `\` and
    /// then a run of `[@-Za-z]` that is a whole word of the list.
    fn tex_listed_command(&mut self, p: usize, list: u32) -> Option<usize> {
        let name = self.lit(p, b"\\")?;
        let end = self.skip(name, Self::tex_command_letter);
        (listed(self.text(name, end), WORD_BYTES, &WORD_ENDS, &WORD_LISTS) & list != 0).then(|| {
            self.capture(1, p, name);
            end
        })
    }

    /// `(\\)(words)\b` at `p`, with the words of `list`: a `\` and then a run
    /// of `\w` that is a whole word of the list.
    fn tex_listed_word(&mut self, p: usize, list: u32) -> Option<usize> {
        let name = self.lit(p, b"\\")?;
        let end = self.word_end(name);
        (listed(self.text(name, end), WORD_BYTES, &WORD_ENDS, &WORD_LISTS) & list != 0).then(|| {
            self.capture(1, p, name);
            end
        })
    }

    /// `(lead)_*[@\p{Alphabetic}]+(?:_[@\p{Alphabetic}]+)*:[DFNTVcefnopvwx]*`
    /// at `p`: an expl3 function or variable name.
    /// No class of the name overlaps the next, so the name needs no
    /// backtracking.
    fn tex_expl3_name(&mut self, p: usize, lead: u8) -> Option<usize> {
        let start = self.lit(p, &[lead])?;
        let mut i = self.skip(start, |b| b == b'_');
        let mut run = self.tex_letter_run(i);
        if run == i {
            return None;
        }
        while self.at(run) == b'_' && run < self.lim() {
            let next = self.tex_letter_run(run + 1);
            if next == run + 1 {
                break;
            }
            run = next;
        }
        i = self.lit(run, b":")?;
        let end = self.skip(i, |b| ARGUMENT_TYPES.contains(&b));
        self.capture(1, p, start);
        Some(end)
    }

    /// `\\(left|right|((bigg??|Bigg??)[lr]?))` and a delimiter at `p`, tried
    /// in the alternation's order.
    fn tex_sized_delimiter(&self, p: usize) -> Option<usize> {
        let name = self.lit(p, b"\\")?;
        for size in [&b"left"[..], b"right"] {
            if let Some(end) = self.lit(name, size).and_then(|end| self.tex_delimiter(end)) {
                return Some(end);
            }
        }
        for size in [&b"big"[..], b"bigg", b"Big", b"Bigg"] {
            let Some(end) = self.lit(name, size) else {
                continue;
            };
            if matches!(self.at(end), b'l' | b'r')
                && end < self.lim()
                && let Some(after) = self.tex_delimiter(end + 1)
            {
                return Some(after);
            }
            if let Some(after) = self.tex_delimiter(end) {
                return Some(after);
            }
        }
        None
    }

    /// `([]().<>\[|]|\\[{|}]|\\[lr]?[Vv]ert|\\[lr]angle)` at `i`.
    fn tex_delimiter(&self, i: usize) -> Option<usize> {
        if DELIMITERS.contains(&self.at(i)) && i < self.lim() {
            return Some(i + 1);
        }
        let name = self.lit(i, b"\\")?;
        if matches!(self.at(name), b'{' | b'|' | b'}') && name < self.lim() {
            return Some(name + 1);
        }
        let side = if matches!(self.at(name), b'l' | b'r') && name < self.lim() {
            name + 1
        } else {
            name
        };
        let vert = |at: usize| {
            matches!(self.at(at), b'V' | b'v')
                .then(|| self.lit(at + 1, b"ert"))
                .flatten()
        };
        vert(side).or_else(|| (side > name).then(|| self.lit(side, b"angle")).flatten())
    }
}

/// The lists `word` belongs to, from a sorted table of a grammar's long
/// alternations; 0 for a word in none.
///
/// The table is a generated `words` module.
/// `bytes` holds its words end to end, `ends` where each ends, and `lists`
/// the lists of each.
/// LaTeX reads its own table with it.
/// The arrays' length `N` is a constant: taking the tables as slices cost
/// TeX 1.2% at one codegen unit.
pub(crate) fn listed<const N: usize>(
    word: &[u8],
    bytes: &[u8],
    ends: &[u16; N],
    lists: &[u32; N],
) -> u32 {
    let mut low = 0;
    let mut high = N;
    while low < high {
        let mid = low + (high - low) / 2;
        let start = mid
            .checked_sub(1)
            .map_or(0, |before| usize::from(ends[before]));
        let entry = &bytes[start..usize::from(ends[mid])];
        match crate::bytes::order(entry, word) {
            std::cmp::Ordering::Less => low = mid + 1,
            std::cmp::Ordering::Greater => high = mid,
            std::cmp::Ordering::Equal => return lists[mid],
        }
    }
    0
}
