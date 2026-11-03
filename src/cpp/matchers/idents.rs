//! Identifiers, which may spell characters as universal character names, and
//! the expressions built on them.

use super::super::engine::Lexer;
use super::chars::{is_digit, is_space};
use super::gaps::GapForm;
use crate::rules::Chars;

/// Where each prefix of an identifier ends, the longest first, as
/// [`Lexer::ident_ends`] finds them. It steps back over the identifier one
/// character or universal character name at a time: inside an identifier a
/// backslash only starts a name, so the name that ends at a place is the one
/// whose backslash stands 6 or 10 bytes before it.
#[derive(Clone)]
pub(crate) struct IdentEnds<'a> {
    s: &'a [u8],
    /// Where the identifier starts.
    start: usize,
    /// Where its first character ends: the shortest prefix.
    first: usize,
    /// The end to give next.
    next: Option<usize>,
}

impl IdentEnds<'_> {
    /// Where the whole identifier ends.
    pub(crate) fn longest(self) -> Option<usize> {
        self.next
    }

    /// The end of the prefix one character shorter than the one at `end`.
    fn back(&self, end: usize) -> usize {
        let name = |len: usize, lead: u8| {
            end >= self.start + len
                && self.s.get(end - len) == Some(&b'\\')
                && self.s.get(end - len + 1) == Some(&lead)
        };
        if name(10, b'U') {
            end - 10
        } else if name(6, b'u') {
            end - 6
        } else {
            end - 1
        }
    }
}

impl Iterator for IdentEnds<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let end = self.next?;
        self.next = (end > self.first).then(|| self.back(end));
        Some(end)
    }
}

impl<'a> Lexer<'a> {
    /// Where each prefix of the identifier at `p` ends, the longest first:
    /// `(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?:u\h{4}|U\h{8}))*`
    /// can give back one character or name at a time.
    pub(super) fn ident_ends(&self, p: usize) -> IdentEnds<'a> {
        let first = self.at(p);
        let first_end = if p < self.lim() && (first.is_ascii_alphabetic() || first == b'_') {
            Some(p + 1)
        } else {
            self.ucn(p)
        };
        let Some(first_end) = first_end else {
            return IdentEnds {
                s: self.s,
                start: p,
                first: p,
                next: None,
            };
        };
        let mut end = first_end;
        loop {
            let b = self.at(end);
            if end < self.lim() && (b.is_ascii_alphanumeric() || b == b'_') {
                end += 1;
            } else if let Some(next) = self.ucn(end) {
                end = next;
            } else {
                break;
            }
        }
        IdentEnds {
            s: self.s,
            start: p,
            first: first_end,
            next: Some(end),
        }
    }
}

impl Lexer<'_> {
    /// `ID(?!\w)` at `p`: where the longest prefix of the identifier that no
    /// word character follows ends.
    pub(super) fn ident_bounded(&self, p: usize) -> Option<usize> {
        self.ident_ends(p).find(|&end| self.word_at(end).is_none())
    }

    /// `(?<!\w)ID(?!\w)`.
    pub(super) fn word_ident(&self, p: usize) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        self.ident_bounded(p)
    }

    /// `ID(?=:)`.
    pub(super) fn ident_ahead_colon(&self, p: usize) -> Option<usize> {
        self.ident_ends(p)
            .find(|&end| self.at(end) == b':' && end < self.lim())
    }

    /// `(?<=:)ID`.
    pub(super) fn after_colon_ident(&self, p: usize) -> Option<usize> {
        if self.prev(p) != Some(b':') {
            return None;
        }
        self.ident_ends(p).longest()
    }

    /// `(?<=:)~ID`.
    pub(super) fn after_colon_destructor(&self, p: usize) -> Option<usize> {
        if self.prev(p) != Some(b':') || self.at(p) != b'~' || p >= self.lim() {
            return None;
        }
        self.ident_ends(p + 1).longest()
    }

    /// `(\.)(ID)`.
    pub(super) fn dot_ident(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'.' || p >= self.lim() {
            return None;
        }
        let end = self.ident_ends(p + 1).longest()?;
        self.cap(1, p, p + 1);
        self.cap(2, p + 1, end);
        Some(end)
    }

    /// `##?ID(?!\w)`.
    ///
    /// The second `#` is optional: try with it first.
    pub(super) fn macro_argument(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'#' || p >= self.lim() {
            return None;
        }
        let at = if self.at(p + 1) == b'#' && p + 1 < self.lim() {
            if let Some(end) = self.ident_bounded(p + 2) {
                return Some(end);
            }
            p + 1
        } else {
            p + 1
        };
        self.ident_bounded(at)
    }

    /// `(?<!\w)ID(?!\w)(?=::)`.
    pub(super) fn ident_ahead_scope(&self, p: usize) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        self.ident_ends(p)
            .find(|&end| self.word_at(end).is_none() && self.starts(end, b"::"))
    }

    /// `((?<!\w)ID(?!\w))(\{)`.
    pub(super) fn ident_brace(&mut self, p: usize) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        let end = self
            .ident_ends(p)
            .find(|&end| self.word_at(end).is_none() && self.at(end) == b'{' && end < self.lim())?;
        self.cap(1, p, end);
        self.cap(2, end, end + 1);
        Some(end + 1)
    }

    /// `(using)\s+((?<!\w)ID(?!\w))`.
    pub(super) fn using_alias(&mut self, p: usize) -> Option<usize> {
        let word = self.lit(p, b"using")?;
        let name = self.skip_space(word);
        if name == word {
            return None;
        }
        let end = self.word_ident(name)?;
        self.cap(1, p, word);
        self.cap(2, name, end);
        Some(end)
    }

    /// `(?<=[(,])\s+{0,1}(ID)\s+{0,1}`.
    pub(super) fn macro_parameter(&mut self, p: usize) -> Option<usize> {
        if !matches!(self.prev(p), Some(b'(' | b',')) {
            return None;
        }
        let name = self.skip_space(p);
        let end = self.ident_ends(name).longest()?;
        self.cap(1, name, end);
        Some(self.skip_space(end))
    }

    /// `ID\b(?<!(?:\W|^)(?:struct|class|union|enum))`.
    ///
    /// The lookbehind fails when a keyword ends here and a non-word character
    /// or the start of the line precedes it.
    pub(super) fn parameter_name(&self, p: usize) -> Option<usize> {
        self.ident_ends(p).find(|&end| {
            if !self.boundary(end) {
                return false;
            }
            ![&b"struct"[..], b"class", b"union", b"enum"]
                .iter()
                .any(|word| {
                    self.ends_with_at(end, word)
                        && (end == word.len() || !self.word_before(end - word.len()))
                })
        })
    }

    /// `(")(?:((?:[A-Za-z]|UCN)(?:[0-9A-Z_a-z]|UCN)*)|(_(?:[0-9A-Z_a-z]|UCN)*
    /// ))?` and the same for `'`: the end of a literal and its suffix.
    pub(super) fn literal_end(&mut self, p: usize, mark: u8) -> Option<usize> {
        if self.at(p) != mark || p >= self.lim() {
            return None;
        }
        self.cap(1, p, p + 1);
        let start = p + 1;
        let first = self.at(start);
        if start < self.lim() && first.is_ascii_alphabetic() || self.ucn(start).is_some() {
            let end = self.ident_ends(start).longest()?;
            self.cap(2, start, end);
            return Some(end);
        }
        if start < self.lim() && first == b'_' {
            let end = self.ident_ends(start).longest().unwrap_or(start + 1);
            self.cap(3, start, end);
            return Some(end);
        }
        Some(start)
    }

    /// `(ID)GAP(?:(?:(?=]|\z|$)|(,))|(=))`.
    pub(super) fn lambda_capture(&mut self, p: usize) -> Option<usize> {
        let name = self.ident_ends(p).longest()?;
        for gap in self.gaps(name) {
            let at = gap.end;
            let found = if self.at(at) == b']' || self.at_eol(at) {
                Some((at, None))
            } else if self.at(at) == b',' && at < self.lim() {
                Some((at + 1, Some(6)))
            } else if self.at(at) == b'=' && at < self.lim() {
                Some((at + 1, Some(7)))
            } else {
                None
            };
            if let Some((end, group)) = found {
                self.cap(1, p, name);
                self.put_gap(2, GapForm::Parts, name, &gap);
                if let Some(group) = group {
                    self.cap(group, at, end);
                }
                return Some(end);
            }
        }
        None
    }

    /// `(?<=[,<])\s+{0,1}(ID)\s+((?:ID)?)\s+{0,1}(=)`.
    pub(super) fn template_default(&mut self, p: usize) -> Option<usize> {
        if !matches!(self.prev(p), Some(b',' | b'<')) {
            return None;
        }
        let first = self.skip_space(p);
        let name = self.ident_ends(first).longest()?;
        let spaces = self.skip_space(name);
        if spaces == name {
            return None;
        }
        if let Some(second) = self.ident_ends(spaces).longest() {
            let eq = self.skip_space(second);
            if self.at(eq) == b'=' && eq < self.lim() {
                self.cap(1, first, name);
                self.cap(2, spaces, second);
                self.cap(3, eq, eq + 1);
                return Some(eq + 1);
            }
        }
        if self.at(spaces) == b'=' && spaces < self.lim() {
            self.cap(1, first, name);
            self.cap(3, spaces, spaces + 1);
            return Some(spaces + 1);
        }
        None
    }

    /// `\[GAP(ID)GAP]`.
    pub(super) fn bracketed_name(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'[' || p >= self.lim() {
            return None;
        }
        for first in self.gaps(p + 1) {
            let Some(name) = self.ident_ends(first.end).longest() else {
                continue;
            };
            for second in self.gaps(name) {
                if self.at(second.end) == b']' && second.end < self.lim() {
                    self.put_gap(1, GapForm::Parts, p + 1, &first);
                    self.cap(5, first.end, name);
                    self.put_gap(6, GapForm::Parts, name, &second);
                    return Some(second.end + 1);
                }
            }
        }
        None
    }

    /// `(?<![(,:\s])GAP(ID)GAP(?=…)`: a parameter name, with `lookahead`
    /// deciding whether what follows is right.
    pub(super) fn parameter_name_gap(
        &mut self,
        p: usize,
        lookahead: impl Fn(&Self, usize) -> bool,
    ) -> Option<usize> {
        if let Some(c) = self.ch_before(p)
            && (matches!(c, '(' | ',' | ':') || is_space(c))
        {
            return None;
        }
        for first in self.gaps(p) {
            let Some(name) = self.ident_ends(first.end).longest() else {
                continue;
            };
            for second in self.gaps(name) {
                if lookahead(self, second.end) {
                    self.put_gap(1, GapForm::Parts, p, &first);
                    self.cap(5, first.end, name);
                    self.put_gap(6, GapForm::Parts, name, &second);
                    return Some(second.end);
                }
            }
        }
        None
    }

    /// `[\n),=\[]`-style lookahead for parameters: `(?=[\n),=\[])`.
    pub(super) fn ahead_parameter_end(&self, at: usize) -> bool {
        matches!(self.at(at), b'\n' | b')' | b',' | b'=' | b'[') && (at < self.lim() || self.nl)
    }

    /// `(?=[),=\[]|//|(?:\n|$))`.
    pub(super) fn ahead_parameter_value_end(&self, at: usize) -> bool {
        (matches!(self.at(at), b')' | b',' | b'=' | b'[') && at < self.lim())
            || self.starts(at, b"//")
            || (self.at(at) == b'\n' && self.nl)
            || self.at_eol(at)
    }

    /// `^(GAP(#)\s+{0,1}undef)\bGAP((?<!\w)ID(?!\w))`.
    pub(super) fn undef(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        for first in self.gaps(0) {
            if self.at(first.end) != b'#' || first.end >= self.lim() {
                continue;
            }
            let name = self.skip_space(first.end + 1);
            let Some(word) = self.lit(name, b"undef") else {
                continue;
            };
            if !self.boundary(word) {
                continue;
            }
            for second in self.gaps(word) {
                let Some(end) = self.word_ident(second.end) else {
                    continue;
                };
                self.cap(1, 0, word);
                self.put_gap(2, GapForm::Plain, 0, &first);
                self.cap(4, first.end, first.end + 1);
                self.put_gap(5, GapForm::Plain, word, &second);
                self.cap(7, second.end, end);
                return Some(end);
            }
        }
        None
    }

    /// `^(GAP(#)\s+{0,1}define)\b\s+{0,1}((?<!\w)ID(?!\w))` for `#define`,
    /// with the gap that captures a comment's parts.
    pub(super) fn macro_define(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        for first in self.gaps(0) {
            if self.at(first.end) != b'#' || first.end >= self.lim() {
                continue;
            }
            let name = self.skip_space(first.end + 1);
            let Some(word) = self.lit(name, b"define") else {
                continue;
            };
            if !self.boundary(word) {
                continue;
            }
            let ident = self.skip_space(word);
            let Some(end) = self.word_ident(ident) else {
                continue;
            };
            self.cap(1, 0, word);
            self.put_gap(2, GapForm::Parts, 0, &first);
            self.cap(6, first.end, first.end + 1);
            self.cap(7, ident, end);
            return Some(end);
        }
        None
    }

    /// `GAP((?<!\w)goto(?!\w))GAP(ID)`.
    pub(super) fn goto_statement(&mut self, p: usize) -> Option<usize> {
        for first in self.gaps(p) {
            let Some(word) = self.word_lit(first.end, b"goto") else {
                continue;
            };
            for second in self.gaps(word) {
                if let Some(end) = self.ident_ends(second.end).longest() {
                    self.put_gap(1, GapForm::Plain, p, &first);
                    self.cap(3, first.end, word);
                    self.put_gap(4, GapForm::Plain, word, &second);
                    self.cap(6, second.end, end);
                    return Some(end);
                }
            }
        }
        None
    }

    /// `GAP((?<!\w)ID(?!\w))\b(?<!case|default)GAP(:)(?!:)`.
    pub(super) fn label(&mut self, p: usize) -> Option<usize> {
        for first in self.gaps(p) {
            if self.word_before(first.end) {
                continue;
            }
            for name in self.ident_ends(first.end) {
                if self.word_at(name).is_some() || !self.boundary(name) {
                    continue;
                }
                if self.ends_with_at(name, b"case") || self.ends_with_at(name, b"default") {
                    continue;
                }
                for second in self.gaps(name) {
                    let at = second.end;
                    if self.at(at) == b':' && at < self.lim() && self.at(at + 1) != b':' {
                        self.put_gap(1, GapForm::Plain, p, &first);
                        self.cap(3, first.end, name);
                        self.put_gap(4, GapForm::Plain, name, &second);
                        self.cap(6, at, at + 1);
                        return Some(at + 1);
                    }
                }
            }
        }
        None
    }

    /// `((?<!\w)ID(?!\w))GAP(?:((?<!\w)final(?!\w))GAP)?(?=[:{]|$)`.
    ///
    /// The form with `final` is tried first.
    pub(super) fn class_name_final(&mut self, p: usize) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        for name in self.ident_ends(p) {
            if self.word_at(name).is_some() {
                continue;
            }
            for first in self.gaps(name) {
                if let Some(word) = self.word_lit(first.end, b"final") {
                    for second in self.gaps(word) {
                        if self.ahead_class_body(second.end) {
                            self.cap(1, p, name);
                            self.put_gap(2, GapForm::Parts, name, &first);
                            self.cap(6, first.end, word);
                            self.put_gap(7, GapForm::Parts, word, &second);
                            return Some(second.end);
                        }
                    }
                }
                if self.ahead_class_body(first.end) {
                    self.cap(1, p, name);
                    self.put_gap(2, GapForm::Parts, name, &first);
                    return Some(first.end);
                }
            }
        }
        None
    }

    /// `(?=[:{]|$)`.
    fn ahead_class_body(&self, at: usize) -> bool {
        (matches!(self.at(at), b':' | b'{') && at < self.lim()) || self.at_eol(at)
    }

    /// `(?<=\w)`-style digit helper kept for the number expressions.
    #[expect(dead_code, reason = "for the number expressions")]
    pub(super) fn digit_run(&self, p: usize) -> usize {
        self.span_of(p, is_digit)
    }
}
