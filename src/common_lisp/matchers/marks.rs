//! The matchers of the grammar's marks: numbers, characters, quotes, the
//! sharpsign forms and the earmuffs of a special name.

use super::super::engine::{Lexer, Muffs};
use crate::case_fold;
use crate::rules::Chars;

/// The two number patterns.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::common_lisp) enum Number {
    /// `ConstantAt1`: integers, ratios, decimals with an `e` exponent, and
    /// numbers in a radix.
    Rational,
    /// `ConstantAt3`: floats with a `.` or an exponent marker.
    Float,
}

/// The sharpsign patterns that read `#`, digits and one mark.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::common_lisp) enum Sharp {
    /// `(#)([0-9]*)(?=\()`: a vector.
    Paren,
    /// `(#)([0-9]*)(\*)(?=[01])`: a bit vector.
    Bits,
    /// `(#)([0-9]+)([Aa])(?=.)`: an array.
    Array,
    /// `(#)([0-9]+)(=)(?=.)`: a label.
    Label,
    /// `(#)([0-9]+)(#)(?=.)`: a reference to a label.
    Reference,
}

/// How many ASCII digits `t` starts with.
fn digits(t: &[u8]) -> usize {
    t.iter().take_while(|b| b.is_ascii_digit()).count()
}

/// `t` without its first byte when that byte is `+` or `-`.
fn unsigned(t: &[u8]) -> &[u8] {
    match t {
        [b'+' | b'-', rest @ ..] => rest,
        _ => t,
    }
}

/// Whether `t` is an exponent: one of `markers`, an optional sign and digits.
fn exponent(t: &[u8], markers: &[u8]) -> bool {
    match t {
        [marker, rest @ ..] if markers.contains(marker) => {
            let rest = unsigned(rest);
            !rest.is_empty() && digits(rest) == rest.len()
        }
        _ => false,
    }
}

/// Whether `t` is, whole, one branch of `ConstantAt1`:
/// `[-+]?[0-9]+(?:/[0-9]+)*`, `[-+]?[0-9]*\.?[0-9]+([Ee][-+]?[0-9]+)?`,
/// `#[Bb][-+/01]+`, `#[Oo][-+/0-7]+`, `#[Xx][-+/\h]+` or
/// `#[0-9]+[Rr]?[-+/0-9A-Za-z]+`.
///
/// The last branch's class takes digits and `R`, so it holds exactly when a
/// digit follows the `#` and at least one class character follows that.
fn rational(t: &[u8]) -> bool {
    if let [b'#', radix, rest @ ..] = t {
        let class: fn(&u8) -> bool = match radix {
            b'B' | b'b' => |b| matches!(b, b'-' | b'+' | b'/' | b'0' | b'1'),
            b'O' | b'o' => |b| matches!(b, b'-' | b'+' | b'/' | b'0'..=b'7'),
            b'X' | b'x' => |b| matches!(b, b'-' | b'+' | b'/') || b.is_ascii_hexdigit(),
            b'0'..=b'9' => |b| matches!(b, b'-' | b'+' | b'/') || b.is_ascii_alphanumeric(),
            _ => return false,
        };
        return !rest.is_empty() && rest.iter().all(class);
    }
    let body = unsigned(t);
    let whole = digits(body);
    if whole > 0 && ratio_tail(&body[whole..]) {
        return true;
    }
    let (fraction, rest) = match body.get(whole) {
        Some(b'.') => {
            let after = &body[whole + 1..];
            let n = digits(after);
            (n, &after[n..])
        }
        _ => (whole, &body[whole..]),
    };
    fraction > 0 && (rest.is_empty() || exponent(rest, b"Ee"))
}

/// Whether `t` is `(?:/[0-9]+)*`, whole.
fn ratio_tail(t: &[u8]) -> bool {
    let mut rest = t;
    while let [b'/', after @ ..] = rest {
        let n = digits(after);
        if n == 0 {
            return false;
        }
        rest = &after[n..];
    }
    rest.is_empty()
}

/// Whether `t` is, whole, one branch of `ConstantAt3`:
/// `[-+]?[0-9]*\.[0-9]*(([DEFLSdefls])[-+]?[0-9]+)?` or
/// `[-+]?[0-9]+(\.[0-9]*)?([DEFLSdefls])[-+]?[0-9]+`.
fn float(t: &[u8]) -> bool {
    const MARKERS: &[u8] = b"DEFLSdefls";
    let body = unsigned(t);
    let whole = digits(body);
    let rest = &body[whole..];
    match rest {
        [b'.', after @ ..] => {
            let n = digits(after);
            let tail = &after[n..];
            tail.is_empty() || exponent(tail, MARKERS)
        }
        _ => whole > 0 && exponent(rest, MARKERS),
    }
}

impl Lexer<'_> {
    /// `ConstantAt1` or `ConstantAt3`, after `(?<=^|[(\s]|,@|,\.?)` and
    /// before `(?=([)\s]))`.
    ///
    /// No branch holds a delimiter, so a match is the run up to the next one,
    /// and that delimiter must not be `(`.
    /// Every branch then reads the same text, so their order cannot matter.
    pub(super) fn number(&self, p: usize, number: Number) -> Option<usize> {
        if !self.after_form(p) {
            return None;
        }
        let end = self.run_end(p);
        if end == p || !self.delimiter(end) || self.at(end) == b'(' {
            return None;
        }
        let text = self.text(p, end);
        let found = match number {
            Number::Rational => rational(text),
            Number::Float => float(text),
        };
        found.then_some(end)
    }

    /// `(?i)(?<=\s)(\.)(?=\s)`: a dot between blanks.
    pub(super) fn dot(&self, p: usize) -> Option<usize> {
        (self.space_before(p) && self.at(p) == b'.' && self.space_at(p + 1) > 0).then_some(p + 1)
    }

    /// `(?i)(?<=^|[(\s])#\\\S+?(?=([()\s]))`: a character such as `#\a` or
    /// `#\Space`.
    ///
    /// The lazy `\S+?` takes one character, then every character up to the
    /// next delimiter: a character that is not a delimiter is a `\S`.
    pub(super) fn character(&self, p: usize) -> Option<usize> {
        if !self.after_open(p) || self.lit(p, b"#\\").is_none() || !self.non_space(p + 2) {
            return None;
        }
        let end = self.run_end(self.char_end(p + 2));
        self.delimiter(end).then_some(end)
    }

    /// `PunctuationAt0`: a quote or a backquote after
    /// `(?<=^|[(\s]|,@|,\.?)` and before a `\S`.
    pub(super) fn quote(&self, p: usize) -> Option<usize> {
        (self.after_form(p) && matches!(self.at(p), b'\'' | b'`') && self.non_space(p + 1))
            .then_some(p + 1)
    }

    /// The sharpsign patterns of [`Sharp`], after `(?<=^|[(\s]|,@|,\.?)`:
    /// `#` as group 1, the digits as group 2 and the mark as group 3.
    ///
    /// The digits are greedy, and giving one back leaves a digit where the
    /// mark must stand.
    pub(super) fn sharp_digits(&mut self, p: usize, sharp: Sharp) -> Option<usize> {
        if !self.after_form(p) || self.at(p) != b'#' {
            return None;
        }
        let digits = self.skip(p + 1, |b| b.is_ascii_digit());
        let some = digits > p + 1;
        let mark = self.at(digits);
        let end = match sharp {
            Sharp::Paren => (mark == b'(').then_some(digits),
            Sharp::Bits => {
                (mark == b'*' && matches!(self.at(digits + 1), b'0' | b'1')).then_some(digits + 1)
            }
            Sharp::Array => (some && matches!(mark, b'A' | b'a') && self.any_char(digits + 1))
                .then_some(digits + 1),
            Sharp::Label => {
                (some && mark == b'=' && self.any_char(digits + 1)).then_some(digits + 1)
            }
            Sharp::Reference => {
                (some && mark == b'#' && self.any_char(digits + 1)).then_some(digits + 1)
            }
        }?;
        self.capture(1, p, p + 1);
        if some {
            self.capture(2, p + 1, digits);
        }
        if end > digits {
            self.capture(3, digits, end);
        }
        Some(end)
    }

    /// `(?=.)` at `i`: a character that is not the line feed.
    fn any_char(&self, i: usize) -> bool {
        i < self.lim()
    }

    /// `(?i)(?<=^|[(\s]|,@|,\.?)(#0??\*)(?=([()\s]))`: `#*` or `#0*`.
    ///
    /// The lazy `0??` tries `#*` first.
    pub(super) fn sharp_star(&self, p: usize) -> Option<usize> {
        if !self.after_form(p) || self.at(p) != b'#' {
            return None;
        }
        if self.at(p + 1) == b'*' && self.delimiter(p + 2) {
            return Some(p + 2);
        }
        (self.lit(p + 1, b"0*").is_some() && self.delimiter(p + 3)).then_some(p + 3)
    }

    /// `(?i)(?<=^|[(\s]|,@|,\.?)(#([-+]))(?=\S)`: a feature expression.
    pub(super) fn sharp_sign(&self, p: usize) -> Option<usize> {
        (self.after_form(p)
            && self.at(p) == b'#'
            && matches!(self.at(p + 1), b'+' | b'-')
            && self.non_space(p + 2))
        .then_some(p + 2)
    }

    /// `(?i)(?<=^|[(\s]|,@|,\.?)(#([',.CPScps]))(?=\S)`: `#'`, `#,`, `#.`,
    /// `#C`, `#P` or `#S`.
    /// Under `(?i)` the class takes `ſ`, which folds to `s`.
    pub(super) fn sharp_reader(&self, p: usize) -> Option<usize> {
        if !self.after_form(p) || self.at(p) != b'#' {
            return None;
        }
        let mark = p + 1;
        let end = match self.at(mark) {
            b'\'' | b',' | b'.' | b'C' | b'P' | b'S' | b'c' | b'p' | b's' => mark + 1,
            b if b >= 0x80 => match self.char_at(mark) {
                Some((c, end)) if case_fold::letters(c) == Some(b"s") => end,
                _ => return None,
            },
            _ => return None,
        };
        self.non_space(end).then_some(end)
    }

    /// `(?i)(?<=^|[(\s]|,@|,\.?)(#)(:)(?=\S)`: an uninterned symbol's
    /// prefix, with `#` as group 1.
    pub(super) fn sharp_keyword(&mut self, p: usize) -> Option<usize> {
        if !self.after_form(p) || self.lit(p, b"#:").is_none() || !self.non_space(p + 2) {
            return None;
        }
        self.capture(1, p, p + 1);
        Some(p + 2)
    }

    /// `PunctuationAt11`, after `(?<=^|[(\s])` and before a `\S`.
    /// It is one of:
    /// - a backquote with `#` as group 2;
    /// - a backquote as group 3 with its comma as group 4;
    /// - a comma alone as group 5.
    ///
    /// Each comma is `,(?:@|\.?)`.
    ///
    /// The comma's `(?:@|\.?)` gives back its `@` or `.` when no `\S`
    /// follows, and a comma is itself a `\S`.
    pub(super) fn backquote(&mut self, p: usize) -> Option<usize> {
        if !self.after_open(p) {
            return None;
        }
        match self.at(p) {
            b'`' => {
                if self.at(p + 1) == b'#' && self.non_space(p + 2) {
                    self.capture(2, p, p + 2);
                    return Some(p + 2);
                }
                if !self.non_space(p + 1) {
                    return None;
                }
                self.capture(3, p, p + 1);
                if self.at(p + 1) == b',' {
                    let end = self.comma(p + 1);
                    if self.non_space(end) {
                        self.capture(4, p + 1, end);
                        return Some(end);
                    }
                }
                Some(p + 1)
            }
            b',' => {
                let end = self.comma(p);
                self.non_space(end).then(|| {
                    self.capture(5, p, end);
                    end
                })
            }
            _ => None,
        }
    }

    /// `,(?:@|\.?)` at `i`, a comma, followed by `(?=\S)`: the end of `,@`
    /// or `,.` when a `\S` follows it, or else of the comma.
    pub(super) fn comma(&self, i: usize) -> usize {
        if matches!(self.at(i + 1), b'@' | b'.') && self.non_space(i + 2) {
            i + 2
        } else {
            i + 1
        }
    }

    /// `\+[^+\s]+\+` or `\*[^*\s]+\*` at `i`, followed by
    /// `(?=([()\s]))`: earmuffs around a name.
    ///
    /// Giving back a character of the greedy class would leave a character
    /// other than the closing mark there.
    fn earmuffs(&self, i: usize) -> Option<usize> {
        let mark = self.at(i);
        if !matches!(mark, b'+' | b'*') {
            return None;
        }
        let mut close = i + 1;
        while self.non_space(close) && self.at(close) != mark {
            close = self.char_end(close);
        }
        self.spend(close - i);
        (close > i + 1 && self.at(close) == mark && self.delimiter(close + 1)).then_some(close + 1)
    }

    /// `StyleGuideAt1` and `StyleGuideAt2`: earmuffs of `mark` after
    /// `(?<=\S:|^|[(\s]|,@|,\.?)`.
    pub(super) fn earmuffs_after(&self, p: usize, mark: u8) -> Option<usize> {
        if self.at(p) != mark {
            return None;
        }
        let after_package =
            self.prev(p) == Some(b':') && p >= 2 && self.non_space(self.char_start_before(p - 1));
        if !after_package && !self.after_form(p) {
            return None;
        }
        self.earmuffs(p)
    }

    /// `(?i)(?<=(?:^|[(\s]|,@|,\.?)')(\S+?)(::?)((\+[^+\s]+\+)|(\*[^*\s]+\*))(?
    /// =([()\s]))`: a quoted symbol whose package ends in `:` or `::` before
    /// earmuffs. Group 3, the earmuffs, takes `source.commonlisp`.
    ///
    /// The lazy `\S+?` tries each end in turn, the nearest first, and at
    /// each `::?` tries `::` before `:`.
    /// The first end that works does not depend on where the run of `\S`
    /// started.
    /// A place later in the run of the last search therefore reuses it.
    pub(super) fn quoted_earmuffs(&mut self, p: usize) -> Option<usize> {
        if self.prev(p) != Some(b'\'') || !self.after_form(p - 1) || !self.non_space(p) {
            return None;
        }
        let first = self.char_end(p);
        let kept = self.x.muffs.get();
        let reuse = kept.text == self.lim()
            && kept.from <= first
            && first <= kept.to
            && kept.found.is_none_or(|(colons, _, _)| colons >= first);
        let found = if reuse {
            kept.found
        } else {
            let (found, to) = self.search_muffs(first);
            self.x.muffs.set(Muffs {
                from: first,
                to,
                text: self.lim(),
                found,
            });
            found
        };
        let (colons, muffs, end) = found?;
        self.capture(1, p, colons);
        self.capture(2, colons, muffs);
        self.capture(3, muffs, end);
        Some(end)
    }

    /// The first place from `from` in the run of `\S` where `::?` and
    /// earmuffs match, and the end of the run searched.
    fn search_muffs(&self, from: usize) -> (Option<(usize, usize, usize)>, usize) {
        let mut q = from;
        loop {
            if self.at(q) == b':' {
                if self.at(q + 1) == b':'
                    && let Some(end) = self.earmuffs(q + 2)
                {
                    return (Some((q, q + 2, end)), q);
                }
                if let Some(end) = self.earmuffs(q + 1) {
                    return (Some((q, q + 1, end)), q);
                }
            }
            if !self.non_space(q) {
                return (None, q);
            }
            let next = self.char_end(q);
            self.spend(next - q);
            q = next;
        }
    }
}
