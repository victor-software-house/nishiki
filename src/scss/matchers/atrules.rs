//! At-rules: their keywords, the names after `@include`, `@mixin`,
//! `@namespace` and `@keyframes`, and the operators of the flow-control
//! rules.

use super::super::engine::Lexer;
use crate::rules::Chars;

/// What follows an at-rule's keyword in its expression.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Tail {
    /// `\s*`.
    Spaces,
    /// Nothing.
    None,
}

/// The words of `@warn`, `@debug` and `@error`.
const WARN: [&[u8]; 3] = [b"warn", b"debug", b"error"];

impl Lexer<'_> {
    /// `((@)word)\b` at `p`, with group 1 the keyword and group 2 the `@`;
    /// the end of the keyword.
    fn keyword_at(&mut self, p: usize, word: &[u8]) -> Option<usize> {
        if self.at(p) != b'@' {
            return None;
        }
        let end = self.lit(p + 1, word)?;
        if self.word_at(end).is_some() {
            return None;
        }
        self.cap(1, p, end);
        self.cap(2, p, p + 1);
        Some(end)
    }

    /// `\s*((@)word)\b`, then `\s*` for [`Tail::Spaces`].
    pub(super) fn at_keyword(&mut self, p: usize, word: &[u8], tail: Tail) -> Option<usize> {
        let at = self.spaces(p);
        let end = self.keyword_at(at, word)?;
        Some(match tail {
            Tail::Spaces => self.spaces(end),
            Tail::None => end,
        })
    }

    /// `^\s*((@)word)\b`, then `\s*` for [`Tail::Spaces`].
    pub(super) fn line_at_keyword(&mut self, p: usize, word: &[u8], tail: Tail) -> Option<usize> {
        if p != 0 {
            return None;
        }
        self.at_keyword(p, word, tail)
    }

    /// `((@)word)\b`.
    pub(super) fn at_bare_keyword(&mut self, p: usize, word: &[u8]) -> Option<usize> {
        self.keyword_at(p, word)
    }

    /// `\s*((@)(warn|debug|error))\b\s*`.
    pub(super) fn at_rule_warn(&mut self, p: usize) -> Option<usize> {
        let at = self.spaces(p);
        let end = WARN.iter().find_map(|word| self.keyword_at(at, word))?;
        Some(self.spaces(end))
    }

    /// `\s*((@)else(\s*(if)?))\s*`: nothing in it is required after
    /// `@else`, and no `\b` follows the `if`.
    pub(super) fn at_rule_else(&mut self, p: usize) -> Option<usize> {
        let at = self.spaces(p);
        if self.at(at) != b'@' {
            return None;
        }
        let word = self.lit(at + 1, b"else")?;
        let spaces = self.spaces(word);
        let keyword = self.lit(spaces, b"if").unwrap_or(spaces);
        self.cap(1, at, keyword);
        self.cap(2, at, at + 1);
        Some(self.spaces(keyword))
    }

    /// `\s*((@)(at-root))(\s+|$)`.
    pub(super) fn at_rule_at_root(&mut self, p: usize) -> Option<usize> {
        let at = self.spaces(p);
        if self.at(at) != b'@' {
            return None;
        }
        let end = self.lit(at + 1, b"at-root")?;
        let after = self.spaces(end);
        if after == end && !self.at_eol(end) {
            return None;
        }
        self.cap(1, at, end);
        self.cap(2, at, at + 1);
        Some(after)
    }

    /// `^\s*((@)page)(?=[:\s])\s*([-:\w]*)`.
    pub(super) fn at_rule_page_at0(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let at = self.spaces(p);
        if self.at(at) != b'@' {
            return None;
        }
        let end = self.lit(at + 1, b"page")?;
        if self.at(end) != b':' && self.space_at(end) == 0 {
            return None;
        }
        let name = self.spaces(end);
        let mut stop = name;
        loop {
            if matches!(self.s.get(stop), Some(b'-' | b':')) {
                stop += 1;
            } else if let Some(width) = self.word_at(stop) {
                stop += width;
            } else {
                break;
            }
        }
        self.spend(stop - at);
        self.cap(1, at, end);
        self.cap(2, at, at + 1);
        self.cap(3, name, stop);
        Some(stop)
    }

    /// `(?<=@include)\s+(?:([-\w]+)\s*(\.))?([-\w]+)`, then `\s*(\()` for
    /// the rule with parameters.
    /// The optional module name is tried first.
    /// Without the dot after it, the name is read again as the mixin's.
    fn include_name(&mut self, p: usize, open: bool) -> Option<usize> {
        if !self.after(p, b"@include") {
            return None;
        }
        let start = self.spaces_run(p)?;
        if let Some(end) = self.include_tail(start, true, open) {
            return Some(end);
        }
        self.include_tail(start, false, open)
    }

    /// The part of [`Lexer::include_name`] after the white space, with or
    /// without the module name.
    fn include_tail(&mut self, start: usize, module: bool, open: bool) -> Option<usize> {
        let mut name = start;
        let mut module_name = None;
        if module {
            let module_end = self.dash_word_run(start)?;
            let dot = self.spaces(module_end);
            if self.at(dot) != b'.' {
                return None;
            }
            module_name = Some((module_end, dot));
            name = dot + 1;
        }
        let name_end = self.dash_word_run(name)?;
        let paren = if open {
            let paren = self.spaces(name_end);
            if self.at(paren) != b'(' {
                return None;
            }
            Some(paren)
        } else {
            None
        };
        if let Some((module_end, dot)) = module_name {
            self.cap(1, start, module_end);
            self.cap(2, dot, dot + 1);
        }
        self.cap(3, name, name_end);
        Some(match paren {
            Some(paren) => {
                self.cap(4, paren, paren + 1);
                paren + 1
            }
            None => name_end,
        })
    }

    /// `(?<=@include)\s+(?:([-\w]+)\s*(\.))?([-\w]+)\s*(\()`.
    pub(super) fn at_rule_include_at0(&mut self, p: usize) -> Option<usize> {
        self.include_name(p, true)
    }

    /// `(?<=@include)\s+(?:([-\w]+)\s*(\.))?([-\w]+)`.
    pub(super) fn at_rule_include_at1(&mut self, p: usize) -> Option<usize> {
        self.include_name(p, false)
    }

    /// `(?<=@mixin)\s+([-\w]+)\s*(\()`.
    pub(super) fn at_rule_mixin_at0(&mut self, p: usize) -> Option<usize> {
        if !self.after(p, b"@mixin") {
            return None;
        }
        let name = self.spaces_run(p)?;
        let name_end = self.dash_word_run(name)?;
        let paren = self.spaces(name_end);
        if self.at(paren) != b'(' {
            return None;
        }
        self.cap(1, name, name_end);
        self.cap(2, paren, paren + 1);
        Some(paren + 1)
    }

    /// `(?<=@mixin)\s+([-\w]+)`.
    pub(super) fn at_rule_mixin_at1(&mut self, p: usize) -> Option<usize> {
        if !self.after(p, b"@mixin") {
            return None;
        }
        let name = self.spaces_run(p)?;
        let name_end = self.dash_word_run(name)?;
        self.cap(1, name, name_end);
        Some(name_end)
    }

    /// `(?<=@namespace)\s+(?=url)`.
    pub(super) fn at_rule_namespace_at0(&mut self, p: usize) -> Option<usize> {
        if !self.after(p, b"@namespace") {
            return None;
        }
        let end = self.spaces_run(p)?;
        self.starts(end, b"url").then_some(end)
    }

    /// `(?<=@namespace)\s+([-\w]*)`.
    pub(super) fn at_rule_namespace_at1(&mut self, p: usize) -> Option<usize> {
        if !self.after(p, b"@namespace") {
            return None;
        }
        let name = self.spaces_run(p)?;
        let end = self.dash_word_end(name);
        self.cap(1, name, end);
        Some(end)
    }

    /// The lookbehind `(?<=^|\s)` at `p`.
    fn after_line_start_or_space(&self, p: usize) -> bool {
        p == 0
            || self
                .char_before(p)
                .is_some_and(|c| self.space_at(p - c.len_utf8()) > 0)
    }

    /// `(?<=^|\s)(@)(?:-(?:webkit|moz)-)?keyframes\b`.
    pub(super) fn at_rule_keyframes(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'@' || !self.after_line_start_or_space(p) {
            return None;
        }
        let prefixed = self
            .lit(p + 1, b"-webkit-")
            .or_else(|| self.lit(p + 1, b"-moz-"));
        let end = [prefixed, Some(p + 1)]
            .into_iter()
            .flatten()
            .find_map(|from| self.lit(from, b"keyframes"))
            .filter(|&end| self.word_at(end).is_none())?;
        self.cap(1, p, p + 1);
        Some(end)
    }

    /// `(?<=@keyframes)\s+((?:[A-Z_a-z][-\w]|-[A-Z_a-z])[-\w]*)`.
    pub(super) fn at_rule_keyframes_at0(&mut self, p: usize) -> Option<usize> {
        if !self.after(p, b"@keyframes") {
            return None;
        }
        let name = self.spaces_run(p)?;
        let letter = |b: u8| b.is_ascii_alphabetic() || b == b'_';
        let first = self.at(name);
        let second = name + 1;
        let starts = if letter(first) {
            self.at(second) == b'-' || self.word_at(second).is_some()
        } else {
            first == b'-' && letter(self.at(second))
        };
        if !starts {
            return None;
        }
        let rest = if self.at(second) == b'-' || first == b'-' {
            second + 1
        } else {
            second + self.word_at(second).unwrap_or(1)
        };
        let end = self.dash_word_end(rest);
        self.cap(1, name, end);
        Some(end)
    }

    /// `(?<=@keyframes)\s+(")`, or with `'`.
    pub(super) fn keyframes_quote(&mut self, p: usize, quote: u8) -> Option<usize> {
        if !self.after(p, b"@keyframes") {
            return None;
        }
        let at = self.spaces_run(p)?;
        if self.at(at) != quote {
            return None;
        }
        self.cap(1, at, at + 1);
        Some(at + 1)
    }

    /// `\b(?:(?:100|[1-9]\d|\d)%|from|to)(?=\s*\{)`.
    /// Each alternative is tried in order, because the first that the
    /// lookahead accepts wins.
    pub(super) fn at_rule_keyframes_at3_at0(&mut self, p: usize) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        let digit = |q: usize| {
            self.char_at(q)
                .filter(|&(c, _)| crate::classes::is_digit(c))
        };
        let mut candidates = [None; 5];
        candidates[0] = self.lit(p, b"100").and_then(|end| self.lit(end, b"%"));
        if matches!(self.at(p), b'1'..=b'9') {
            candidates[1] = digit(p + 1).and_then(|(_, end)| self.lit(end, b"%"));
        }
        candidates[2] = digit(p).and_then(|(_, end)| self.lit(end, b"%"));
        candidates[3] = self.lit(p, b"from");
        candidates[4] = self.lit(p, b"to");
        candidates
            .into_iter()
            .flatten()
            .find(|&end| self.spaces_before(end, b"{").is_some())
    }

    /// `(?<=^|\s)(@)supports\b`.
    pub(super) fn at_rule_supports(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'@' || !self.after_line_start_or_space(p) {
            return None;
        }
        let end = self.lit(p + 1, b"supports")?;
        if self.word_at(end).is_some() {
            return None;
        }
        self.cap(1, p, p + 1);
        Some(end)
    }

    /// `==|!=|<=|>=|[<>]`.
    pub(super) fn comparison_operators(&mut self, p: usize) -> Option<usize> {
        [b"==", b"!=", b"<=", b">="]
            .iter()
            .find_map(|op| self.lit(p, *op))
            .or_else(|| matches!(self.at(p), b'<' | b'>').then_some(p + 1))
    }

    /// `(==|!=|<=|>=|[<>]|from|to|through)`, with no `\b` around the words.
    pub(super) fn at_rule_for_at0(&mut self, p: usize) -> Option<usize> {
        self.comparison_operators(p)
            .or_else(|| self.lit(p, b"from"))
            .or_else(|| self.lit(p, b"to"))
            .or_else(|| self.lit(p, b"through"))
    }

    /// `\b(in|,)\b`.
    pub(super) fn at_rule_each_at0(&mut self, p: usize) -> Option<usize> {
        if let Some(end) = self.keyword(p, b"in") {
            return Some(end);
        }
        (self.at(p) == b',' && self.word_before(p) && self.word_at(p + 1).is_some())
            .then_some(p + 1)
    }

    /// `\b([-\w]+)(\*)`: the run cannot give back a character that would
    /// be the `*`.
    pub(super) fn at_rule_forward_at1(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let end = self.dash_word_run(p)?;
        if self.at(end) != b'*' {
            return None;
        }
        self.cap(1, p, end);
        self.cap(2, end, end + 1);
        Some(end + 1)
    }

    /// `\b[-\w]+\b`: the run gives characters back until a `\b` follows.
    pub(super) fn at_rule_forward_at2(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let end = self.dash_word_run(p)?;
        self.boundary_back(p, end)
    }
}
