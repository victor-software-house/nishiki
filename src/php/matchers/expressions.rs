//! Variables, objects, calls, operators, keywords and numbers.

use super::super::engine::{Lexer, is_name, is_name_start};
use super::is_qualified;
use crate::rules::Chars;

/// `(?i)[$_a-z]`, by character: `ſ` and the Kelvin sign fold to letters.
fn starts_word(c: char) -> bool {
    c == '$' || c == '_' || c.is_ascii_alphabetic() || c == 'ſ' || c == '\u{212a}'
}

impl Lexer<'_> {
    /// `(\$)((_(COOKIE|FILES|GET|POST|REQUEST))|arg([cv]))\b`.
    pub(super) fn global_variable(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'$' {
            return None;
        }
        let names: [&[u8]; 7] = [
            b"_COOKIE",
            b"_FILES",
            b"_GET",
            b"_POST",
            b"_REQUEST",
            b"argc",
            b"argv",
        ];
        let end = names.iter().find_map(|name| {
            self.lit(p + 1, name)
                .filter(|&end| self.word_at(end).is_none())
        })?;
        self.cap(1, p, p + 1);
        Some(end)
    }

    /// `(\$)((GLOBALS|_(ENV|SERVER|SESSION)))`.
    pub(super) fn superglobal(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'$' {
            return None;
        }
        let names: [&[u8]; 4] = [b"GLOBALS", b"_ENV", b"_SERVER", b"_SESSION"];
        let end = names.iter().find_map(|name| self.lit(p + 1, name))?;
        self.cap(1, p, p + 1);
        Some(end)
    }

    /// A variable in a string, with a property or an index:
    ///
    /// ```text
    /// (?i)((\$)(?<name>NAME))\s*(?:(\??->)\s*(\g<name>)|(\[)(?:(\d+)|((\$)\g<name>)|(NAME))(]))?
    /// ```
    ///
    /// `\g<name>` calls the name's pattern again.
    /// The spaces after the variable are part of the match, with or without
    /// what follows them.
    pub(super) fn interpolated_variable(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'$' {
            return None;
        }
        let name = self.name(p + 1)?;
        self.cap(1, p, name);
        self.cap(2, p, p + 1);
        let at = self.space_end(name);
        let arrow = at + usize::from(self.at(at) == b'?');
        if let Some(arrow_end) = self.lit(arrow, b"->") {
            let member = self.space_end(arrow_end);
            if let Some(end) = self.name(member) {
                self.cap(4, at, arrow_end);
                self.cap(5, member, end);
                return Some(end);
            }
        }
        if self.at(at) == b'[' {
            let index = at + 1;
            let digits = self.digit_end(index);
            if digits > index && self.at(digits) == b']' {
                self.cap(6, at, index);
                self.cap(7, index, digits);
                self.cap(11, digits, digits + 1);
                return Some(digits + 1);
            }
            if self.at(index) == b'$'
                && let Some(end) = self.name(index + 1)
                && self.at(end) == b']'
            {
                self.cap(6, at, index);
                self.cap(8, index, end);
                self.cap(9, index, index + 1);
                self.cap(11, end, end + 1);
                return Some(end + 1);
            }
            if let Some(end) = self.name(index)
                && self.at(end) == b']'
            {
                self.cap(6, at, index);
                self.cap(10, index, end);
                self.cap(11, end, end + 1);
                return Some(end + 1);
            }
        }
        Some(at)
    }

    /// `(?i)((\$\{)(?<name>NAME)(}))`.
    pub(super) fn braced_variable(&mut self, p: usize) -> Option<usize> {
        let open = self.lit(p, b"${")?;
        let name = self.name(open)?;
        if self.at(name) != b'}' {
            return None;
        }
        self.cap(1, p, name + 1);
        self.cap(2, p, open);
        self.cap(4, name, name + 1);
        Some(name + 1)
    }

    /// `(?i)(\$+)NAME`.
    pub(super) fn variable(&mut self, p: usize) -> Option<usize> {
        let at = self.skip(p, |b| b == b'$');
        if at == p {
            return None;
        }
        let end = self.name(at)?;
        self.cap(1, p, at);
        Some(end)
    }

    /// `\$\{(?=.*?})`.
    pub(super) fn dollar_brace(&self, p: usize) -> Option<usize> {
        let open = self.lit(p, b"${")?;
        (open..self.lim())
            .any(|at| self.at(at) == b'}')
            .then_some(open)
    }

    /// `([A-Z\\_a-z\x7F-\x{10FFFF}][0-9A-Z\\_a-z\x7F-\x{10FFFF}]*)(?=\s*::)`.
    pub(super) fn class_before_scope(&mut self, p: usize) -> Option<usize> {
        let b = self.at(p);
        if p >= self.lim() || !(b == b'\\' || is_name_start(b)) {
            return None;
        }
        let end = self.skip(p + 1, is_qualified);
        if !self.starts(self.space_end(end), b"::") {
            return None;
        }
        self.cap(1, p, end);
        Some(end)
    }

    /// `(?i)(::)\s*(NAME)\s*(\()`.
    pub(super) fn static_call(&mut self, p: usize) -> Option<usize> {
        let colons = self.lit(p, b"::")?;
        let at = self.space_end(colons);
        let name = self.name(at)?;
        let open = self.space_end(name);
        if self.at(open) != b'(' {
            return None;
        }
        self.cap(1, p, colons);
        self.cap(2, at, name);
        self.cap(3, open, open + 1);
        Some(open + 1)
    }

    /// `(?i)(::)\s*(class)\b`.
    pub(super) fn class_constant_name(&mut self, p: usize) -> Option<usize> {
        let colons = self.lit(p, b"::")?;
        let at = self.space_end(colons);
        let end = self.ilit(at, b"class")?;
        if self.word_at(end).is_some() {
            return None;
        }
        self.cap(1, p, colons);
        self.cap(2, at, end);
        Some(end)
    }

    /// `(?i)(::)\s*(?:((\$+)NAME)|(NAME))?`.
    pub(super) fn static_access(&mut self, p: usize) -> Option<usize> {
        let colons = self.lit(p, b"::")?;
        let at = self.space_end(colons);
        self.cap(1, p, colons);
        let dollars = self.skip(at, |b| b == b'$');
        if dollars > at {
            if let Some(end) = self.name(dollars) {
                self.cap(2, at, end);
                self.cap(3, at, dollars);
                return Some(end);
            }
            return Some(at);
        }
        match self.name(at) {
            Some(end) => {
                self.cap(4, at, end);
                Some(end)
            }
            None => Some(at),
        }
    }

    /// `(?i)((\$+)NAME)\s*(\()`.
    pub(super) fn invoke(&mut self, p: usize) -> Option<usize> {
        let at = self.skip(p, |b| b == b'$');
        if at == p {
            return None;
        }
        let end = self.name(at)?;
        let open = self.space_end(end);
        if self.at(open) != b'(' {
            return None;
        }
        self.cap(1, p, end);
        self.cap(2, p, at);
        self.cap(3, open, open + 1);
        Some(open + 1)
    }

    /// `(?i)(\()\s*
    /// ((bool|int|float|string|array|object)|(binary|boolean|integer|double|real|unset))\
    /// s*(\))`. The words are not bounded, so `int` gives way to `integer` only
    /// where `)` does not follow it.
    pub(super) fn cast(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'(' {
            return None;
        }
        let at = self.space_end(p + 1);
        let words: [&[u8]; 12] = [
            b"bool", b"int", b"float", b"string", b"array", b"object", b"binary", b"boolean",
            b"integer", b"double", b"real", b"unset",
        ];
        for (k, word) in words.iter().enumerate() {
            if let Some(end) = self.ilit(at, word) {
                let close = self.space_end(end);
                if self.at(close) == b')' {
                    self.cap(1, p, p + 1);
                    self.cap(2, at, end);
                    if k < 6 {
                        self.cap(3, at, end);
                    }
                    self.cap(5, close, close + 1);
                    return Some(close + 1);
                }
            }
        }
        None
    }

    /// `(?i)\b(?:(a|b)|(c)|…)\b`: the first of `groups` that is the word at
    /// `p`, as its group.
    pub(super) fn storage_modifier_of(
        &mut self,
        p: usize,
        groups: &[(&[u8], usize)],
    ) -> Option<usize> {
        let (end, group) = groups
            .iter()
            .find_map(|&(word, group)| self.ikeyword(p, word).map(|end| (end, group)))?;
        self.cap(group, p, end);
        Some(end)
    }

    /// `\??->` at `p`, and its end.
    fn arrow(&self, p: usize) -> Option<usize> {
        self.lit(p + usize::from(self.at(p) == b'?'), b"->")
    }

    /// `(\??->)\s*(\$?\{)`.
    pub(super) fn object_brace(&mut self, p: usize) -> Option<usize> {
        let arrow = self.arrow(p)?;
        let at = self.space_end(arrow);
        let end = self.lit(at, b"${").or_else(|| self.lit(at, b"{"))?;
        self.cap(1, p, arrow);
        self.cap(2, at, end);
        Some(end)
    }

    /// `(?i)(\??->)\s*(NAME)\s*(\()`.
    pub(super) fn method_call(&mut self, p: usize) -> Option<usize> {
        let arrow = self.arrow(p)?;
        let at = self.space_end(arrow);
        let name = self.name(at)?;
        let open = self.space_end(name);
        if self.at(open) != b'(' {
            return None;
        }
        self.cap(1, p, arrow);
        self.cap(2, at, name);
        self.cap(3, open, open + 1);
        Some(open + 1)
    }

    /// `(?i)(\??->)\s*((\$+)?NAME)?`.
    pub(super) fn property_access(&mut self, p: usize) -> Option<usize> {
        let arrow = self.arrow(p)?;
        let at = self.space_end(arrow);
        self.cap(1, p, arrow);
        let dollars = self.skip(at, |b| b == b'$');
        let Some(end) = self.name(dollars) else {
            return Some(at);
        };
        if dollars > at {
            self.cap(3, at, dollars);
        }
        self.cap(2, at, end);
        Some(end)
    }

    /// `(?i)(=)(&)|(&)(?=[$_a-z])`.
    pub(super) fn reference(&mut self, p: usize) -> Option<usize> {
        if self.starts(p, b"=&") {
            self.cap(1, p, p + 1);
            self.cap(2, p + 1, p + 2);
            return Some(p + 2);
        }
        if self.at(p) != b'&' || !self.char_at(p + 1).is_some_and(|(c, _)| starts_word(c)) {
            return None;
        }
        self.cap(3, p, p + 1);
        Some(p + 1)
    }

    /// `===?|!==?|<>`.
    pub(super) fn comparison(&self, p: usize) -> Option<usize> {
        if let Some(end) = self.lit(p, b"==").or_else(|| self.lit(p, b"!=")) {
            return Some(end + usize::from(self.at(end) == b'='));
        }
        self.lit(p, b"<>")
    }

    /// `(?:|[-+]|\*\*?|[%\&/^|]|<<|>>|\?\?)=`: the empty alternative comes
    /// first, so `=` alone matches before any compound form.
    pub(super) fn assignment(&self, p: usize) -> Option<usize> {
        if self.at(p) == b'=' && p < self.lim() {
            return Some(p + 1);
        }
        let op = match self.at(p) {
            b'-' | b'+' | b'%' | b'&' | b'/' | b'^' | b'|' => p + 1,
            b'*' => {
                if self.at(p + 1) == b'*' && self.at(p + 2) == b'=' {
                    return Some(p + 3);
                }
                p + 1
            }
            b'<' | b'>' | b'?' if self.at(p + 1) == self.at(p) => p + 2,
            _ => return None,
        };
        (self.at(op) == b'=').then_some(op + 1)
    }

    /// `<=>?|>=|[<>]`.
    pub(super) fn relation(&self, p: usize) -> Option<usize> {
        if let Some(end) = self.lit(p, b"<=") {
            return Some(end + usize::from(self.at(end) == b'>'));
        }
        self.lit(p, b">=")
            .or_else(|| matches!(self.at(p), b'<' | b'>').then_some(p + 1))
    }

    /// `[-+]|\*\*?|[%/]`.
    pub(super) fn arithmetic(&self, p: usize) -> Option<usize> {
        match self.at(p) {
            b'-' | b'+' | b'%' | b'/' => Some(p + 1),
            b'*' => Some(p + 1 + usize::from(self.at(p + 1) == b'*')),
            _ => None,
        }
    }

    /// `(?i)(!|&&|\|\|)|\b(and|or|xor)\b`.
    pub(super) fn logical(&self, p: usize) -> Option<usize> {
        if self.at(p) == b'!' {
            return Some(p + 1);
        }
        self.lit(p, b"&&")
            .or_else(|| self.lit(p, b"||"))
            .or_else(|| self.iword_in(p, &[b"and", b"or", b"xor"]))
    }

    /// `<<|>>|[\&^|~]`.
    pub(super) fn bitwise(&self, p: usize) -> Option<usize> {
        self.lit(p, b"<<")
            .or_else(|| self.lit(p, b">>"))
            .or_else(|| matches!(self.at(p), b'&' | b'^' | b'|' | b'~').then_some(p + 1))
    }

    /// `(?<![0-9A-Z_a-z\x7F-\x{10FFFF}])`, by byte.
    fn after_non_name(&self, p: usize) -> bool {
        self.prev(p).is_none_or(|b| !is_name(b))
    }

    /// `(\\?(?<![…])NAME(?:\\NAME)+)\s*(\()`: a call of a namespaced function.
    pub(super) fn qualified_call(&mut self, p: usize) -> Option<usize> {
        let at = p + usize::from(self.at(p) == b'\\');
        if !self.after_non_name(at) {
            return None;
        }
        let mut end = self.name(at)?;
        let mut parts = 0;
        while self.at(end) == b'\\'
            && let Some(next) = self.name(end + 1)
        {
            end = next;
            parts += 1;
        }
        let open = self.space_end(end);
        if parts == 0 || self.at(open) != b'(' {
            return None;
        }
        self.cap(1, p, end);
        self.cap(2, open, open + 1);
        Some(open + 1)
    }

    /// `(\\)?(?<![0-9A-Z_a-z\x7F-\x{10FFFF}])(NAME)\s*(\()`.
    pub(super) fn call(&mut self, p: usize) -> Option<usize> {
        let slash = self.at(p) == b'\\';
        let at = p + usize::from(slash);
        if !self.after_non_name(at) {
            return None;
        }
        let end = self.name(at)?;
        let open = self.space_end(end);
        if self.at(open) != b'(' {
            return None;
        }
        if slash {
            self.cap(1, p, at);
        }
        self.cap(2, at, end);
        self.cap(3, open, open + 1);
        Some(open + 1)
    }

    /// `(?i)\b(instanceof)\s+(?=[$\\_a-z])`.
    pub(super) fn instanceof(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let end = self.ilit(p, b"instanceof")?;
        let at = self.space_end(end);
        let next = self
            .char_at(at)
            .is_some_and(|(c, _)| c == '\\' || starts_word(c));
        if at == end || !next {
            return None;
        }
        self.cap(1, p, end);
        Some(at)
    }

    /// `(?i)(new)\s+(?!class\b)([$0-9\\_a-z\x7F-\x{10FFFF}]+)` followed by
    /// `(?![(0-9\\_a-z\x7F-\x{10FFFF}])` without `call`, else by `\s*(\()`.
    /// Without `call` the class name gives back to its last `$` when a name
    /// character or `(` follows it: `$` is in the name and not in the
    /// lookahead.
    pub(super) fn instantiation(&mut self, p: usize, call: bool) -> Option<usize> {
        let new_end = self.ilit(p, b"new")?;
        let at = self.space_end(new_end);
        if at == new_end {
            return None;
        }
        if self
            .ilit(at, b"class")
            .is_some_and(|end| self.word_at(end).is_none())
        {
            return None;
        }
        let end = self.skip(at, |b| b == b'$' || is_qualified(b));
        if end == at {
            return None;
        }
        if call {
            let open = self.space_end(end);
            if self.at(open) != b'(' {
                return None;
            }
            self.cap(1, p, new_end);
            self.cap(2, at, end);
            self.cap(3, open, open + 1);
            return Some(open + 1);
        }
        let free = |b: u8, at_end: bool| at_end || !(b == b'(' || is_qualified(b));
        let end = if free(self.at(end), end >= self.lim()) {
            end
        } else {
            (at + 1..end).rev().find(|&k| self.at(k) == b'$')?
        };
        self.cap(1, p, new_end);
        self.cap(2, at, end);
        Some(end)
    }

    /// `(?i)^\s*(NAME(?<!default|else))\s*:(?!:)`.
    pub(super) fn label(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let at = self.space_end(0);
        let end = self.name(at)?;
        let ends_with = |word: &[u8]| {
            end >= word.len() && self.text(end - word.len(), end).eq_ignore_ascii_case(word)
        };
        if ends_with(b"default") || ends_with(b"else") {
            return None;
        }
        let colon = self.space_end(end);
        if self.at(colon) != b':' || self.at(colon + 1) == b':' {
            return None;
        }
        self.cap(1, at, end);
        Some(colon + 1)
    }

    /// `(?i)^\s*(NAME)\s*(?=:(?!:))`.
    pub(super) fn ternary_label(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let at = self.space_end(0);
        let end = self.name(at)?;
        let colon = self.space_end(end);
        if self.at(colon) != b':' || self.at(colon + 1) == b':' {
            return None;
        }
        self.cap(1, at, end);
        Some(colon)
    }

    /// `\bswitch\b(?!\s*\(.*\)\s*:)`: a `switch` whose condition is not
    /// followed by `:` on the line.
    pub(super) fn switch(&self, p: usize) -> Option<usize> {
        let end = self.keyword(p, b"switch")?;
        let open = self.space_end(end);
        if self.at(open) == b'(' && open < self.lim() {
            let colon_after = (open + 1..self.lim())
                .filter(|&at| self.at(at) == b')')
                .any(|close| self.at(self.space_end(close + 1)) == b':');
            if colon_after {
                return None;
            }
        }
        Some(end)
    }

    /// `\s*\b(yield\s+from)\b`.
    pub(super) fn yield_from(&mut self, p: usize) -> Option<usize> {
        let at = self.space_end(p);
        if !self.boundary(at) {
            return None;
        }
        let word = self.lit(at, b"yield")?;
        let from = self.space_end(word);
        if from == word {
            return None;
        }
        let end = self.lit(from, b"from")?;
        if self.word_at(end).is_some() {
            return None;
        }
        self.cap(1, at, end);
        Some(end)
    }

    /// `\b(break|case|…|yield)\b`: a control keyword.
    pub(super) fn control_keyword(&mut self, p: usize) -> Option<usize> {
        let end = self.word_in(
            p,
            &[
                b"break",
                b"case",
                b"continue",
                b"declare",
                b"default",
                b"die",
                b"do",
                b"else",
                b"elseif",
                b"enddeclare",
                b"endfor",
                b"endforeach",
                b"endif",
                b"endswitch",
                b"endwhile",
                b"exit",
                b"for",
                b"foreach",
                b"if",
                b"return",
                b"switch",
                b"use",
                b"while",
                b"yield",
            ],
        )?;
        self.cap(1, p, end);
        Some(end)
    }

    /// `(?i)\b((?:require|include)(?:_once)?)(\s+|(?=\())`.
    pub(super) fn include(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let word = self
            .ilit(p, b"require")
            .or_else(|| self.ilit(p, b"include"))?;
        for end in [self.ilit(word, b"_once"), Some(word)]
            .into_iter()
            .flatten()
        {
            let spaces = self.space_end(end);
            if spaces > end {
                self.cap(1, p, end);
                return Some(spaces);
            }
            if self.at(end) == b'(' {
                self.cap(1, p, end);
                return Some(end);
            }
        }
        None
    }

    /// `\b(catch)\s*(\()`.
    pub(super) fn catch(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let end = self.lit(p, b"catch")?;
        let open = self.space_end(end);
        if self.at(open) != b'(' {
            return None;
        }
        self.cap(1, p, end);
        self.cap(2, open, open + 1);
        Some(open + 1)
    }

    /// `(?i)([0-9\\_a-z\x7F-\x{10FFFF}]+(?:\s*\|\s*[0-9\\_a-z\x7F-\
    /// x{10FFFF}]+)*)\s*((\$+)NAME)?`.
    pub(super) fn caught(&mut self, p: usize) -> Option<usize> {
        let mut end = self.qualified(p)?;
        loop {
            let bar = self.space_end(end);
            if self.at(bar) != b'|' {
                break;
            }
            match self.qualified(self.space_end(bar + 1)) {
                Some(next) => end = next,
                None => break,
            }
        }
        self.cap(1, p, end);
        let at = self.space_end(end);
        let dollars = self.skip(at, |b| b == b'$');
        if dollars > at
            && let Some(name) = self.name(dollars)
        {
            self.cap(2, at, name);
            self.cap(3, at, dollars);
            return Some(name);
        }
        Some(at)
    }

    /// `0[Xx]\h+(?:_\h+)*` and its binary and octal kin: `0`, a letter of
    /// `marks`, and digits that `digit` accepts in groups joined by `_`.
    pub(super) fn radix(&self, p: usize, marks: &[u8], digit: fn(u8) -> bool) -> Option<usize> {
        if self.at(p) != b'0' || !marks.contains(&self.at(p + 1)) {
            return None;
        }
        self.digit_groups(p + 2, digit)
    }

    /// `D+(?:_D+)*` from `p`, for the digits `digit` accepts.
    fn digit_groups(&self, p: usize, digit: fn(u8) -> bool) -> Option<usize> {
        let mut end = self.skip(p, digit);
        if end == p {
            return None;
        }
        while self.at(end) == b'_' {
            let next = self.skip(end + 1, digit);
            if next == end + 1 {
                break;
            }
            end = next;
        }
        Some(end)
    }

    /// `0(?:_?[0-7]+)+`.
    pub(super) fn octal(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'0' {
            return None;
        }
        let octal = |b: u8| matches!(b, b'0'..=b'7');
        let mut end = p + 1;
        loop {
            let from = end + usize::from(self.at(end) == b'_');
            let next = self.skip(from, octal);
            if next == from {
                break;
            }
            end = next;
        }
        (end > p + 1).then_some(end)
    }

    /// `[Ee][-+]?D` from `p`, where `D` is `[0-9]+(?:_[0-9]+)*`.
    fn exponent(&self, p: usize) -> Option<usize> {
        if !matches!(self.at(p), b'e' | b'E') {
            return None;
        }
        let at = p + 1 + usize::from(matches!(self.at(p + 1), b'-' | b'+'));
        self.digit_groups(at, |b| b.is_ascii_digit())
    }

    /// A decimal number with a point or an exponent, where `D` is
    /// `[0-9]+(?:_[0-9]+)*` and `E` is `[Ee][-+]?D`:
    ///
    /// ```text
    /// D?(\.)D E? | D(\.)D? E? | D E
    /// ```
    pub(super) fn decimal(&mut self, p: usize) -> Option<usize> {
        let digits = |b: u8| b.is_ascii_digit();
        let whole = self.digit_groups(p, digits);
        let point = whole.unwrap_or(p);
        if self.at(point) == b'.'
            && let Some(fraction) = self.digit_groups(point + 1, digits)
        {
            self.cap(1, point, point + 1);
            return Some(self.exponent(fraction).unwrap_or(fraction));
        }
        let whole = whole?;
        if self.at(whole) == b'.' {
            let fraction = self.digit_groups(whole + 1, digits).unwrap_or(whole + 1);
            self.cap(2, whole, whole + 1);
            return Some(self.exponent(fraction).unwrap_or(fraction));
        }
        self.exponent(whole)
    }

    /// `0|[1-9](?:_?[0-9]+)*`.
    pub(super) fn integer(&self, p: usize) -> Option<usize> {
        match self.at(p) {
            b'0' => Some(p + 1),
            b'1'..=b'9' => {
                let mut end = p + 1;
                loop {
                    let from = end + usize::from(self.at(end) == b'_');
                    let next = self.skip(from, |b| b.is_ascii_digit());
                    if next == from {
                        break;
                    }
                    end = next;
                }
                Some(end)
            }
            _ => None,
        }
    }
}
