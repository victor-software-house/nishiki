//! Declarations, parameters, calls, names, numbers, comments and line
//! continuation.

use super::super::engine::Lexer;
use super::super::grammar::{ANNOTATIONS, BUILTIN_CLASSES};
use crate::rules::Chars;

impl Lexer<'_> {
    /// `(extends)\s+([A-Z_a-z]\w*\.[A-Z_a-z]\w*)?`.
    /// Without the dotted name, `\s+` keeps every blank it took.
    pub(super) fn extends(&mut self, p: usize) -> Option<usize> {
        let keyword = self.lit(p, b"extends")?;
        let spaces = self.spaces1(keyword)?;
        self.cap(1, p, keyword);
        if let Some(first) = self.name(spaces)
            && self.starts(first, b".")
            && let Some(second) = self.name(first + 1)
        {
            self.cap(2, spaces, second);
            return Some(second);
        }
        Some(spaces)
    }

    /// `(get)\s*(:)`.
    pub(super) fn getter(&mut self, p: usize) -> Option<usize> {
        let keyword = self.lit(p, b"get")?;
        let colon = self.space_end(keyword);
        let end = self.lit(colon, b":")?;
        self.cap(1, p, keyword);
        self.cap(2, colon, end);
        Some(end)
    }

    /// `(set)\s*(\()\s*([A-Z_a-z]\w*)\s*(\))\s*(:)`.
    pub(super) fn setter(&mut self, p: usize) -> Option<usize> {
        let keyword = self.lit(p, b"set")?;
        let open = self.space_end(keyword);
        let name = self.space_end(self.lit(open, b"(")?);
        let name_end = self.name(name)?;
        let close = self.space_end(name_end);
        let colon = self.space_end(self.lit(close, b")")?);
        let end = self.lit(colon, b":")?;
        self.cap(1, p, keyword);
        self.cap(2, open, open + 1);
        self.cap(3, name, name_end);
        self.cap(4, close, close + 1);
        self.cap(5, colon, end);
        Some(end)
    }

    /// `(func)\s?(?=\()`.
    /// Without the `(` after the blank, `\s?` gives the blank back, and the
    /// lookahead then fails at it.
    pub(super) fn lambda(&mut self, p: usize) -> Option<usize> {
        let keyword = self.lit(p, b"func")?;
        let end = keyword + self.space_at(keyword);
        if !self.starts(end, b"(") {
            return None;
        }
        self.cap(1, p, keyword);
        Some(end)
    }

    /// `\s*([A-Z_a-z]\w*)\s*(:)\s*([A-Z_a-z]\w*)?`: a parameter with a type.
    pub(super) fn annotated_parameter(&mut self, p: usize) -> Option<usize> {
        let name = self.space_end(p);
        let name_end = self.name(name)?;
        let colon = self.space_end(name_end);
        let after = self.lit(colon, b":")?;
        let kind = self.space_end(after);
        self.cap(1, name, name_end);
        self.cap(2, colon, after);
        match self.name(kind) {
            Some(end) => {
                self.cap(3, kind, end);
                Some(end)
            }
            None => Some(kind),
        }
    }

    /// `([A-Z_a-z]\w*)\s*(?:(,)|(?=[\n#)=]))`: a parameter without a type.
    /// `\s*` can take the line feed, but then neither branch holds, so it
    /// gives the line feed back to the lookahead.
    pub(super) fn parameter(&mut self, p: usize) -> Option<usize> {
        let name_end = self.name(p)?;
        let at = self.blanks_end(name_end);
        self.cap(1, p, name_end);
        if self.starts(at, b",") {
            self.cap(2, at, at + 1);
            return Some(at + 1);
        }
        (self.avail(at) && matches!(self.at(at), b'\n' | b'#' | b')' | b'=')).then_some(at)
    }

    /// `(?<=^class)\s+([A-Z_a-z]\w*)\s*(?=:)`.
    pub(super) fn class_declaration(&mut self, p: usize) -> Option<usize> {
        if p != 5 || self.text(0, 5) != b"class" {
            return None;
        }
        let name = self.spaces1(p)?;
        let name_end = self.name(name)?;
        let end = self.space_end(name_end);
        if !self.starts(end, b":") {
            return None;
        }
        self.cap(1, name, name_end);
        Some(end)
    }

    /// `\b(?:(var)|(const))\b`.
    pub(super) fn variable_keyword(&mut self, p: usize) -> Option<usize> {
        if let Some(end) = self.word_in(p, &[b"var"]) {
            self.cap(1, p, end);
            return Some(end);
        }
        let end = self.word_in(p, &[b"const"])?;
        self.cap(2, p, end);
        Some(end)
    }

    /// `(:)?\s*([gs]et)\s+=\s+([A-Z_a-z]\w*)`: Godot 3's accessor.
    /// Without the rest, the optional `:` gives way, and `[gs]et` then fails
    /// at the `:`.
    pub(super) fn legacy_accessor(&mut self, p: usize) -> Option<usize> {
        let (colon, start) = if self.starts(p, b":") {
            (Some(p), p + 1)
        } else {
            (None, p)
        };
        let keyword = self.space_end(start);
        if !(self.starts(keyword, b"get") || self.starts(keyword, b"set")) {
            return None;
        }
        let equals = self.spaces1(keyword + 3)?;
        let name = self.spaces1(self.lit(equals, b"=")?)?;
        let end = self.name(name)?;
        if let Some(colon) = colon {
            self.cap(1, colon, colon + 1);
        }
        self.cap(2, keyword, keyword + 3);
        self.cap(3, name, end);
        Some(end)
    }

    /// `(:)\s*([A-Z_a-z]\w*)?`: a variable's type.
    pub(super) fn type_annotation(&mut self, p: usize) -> Option<usize> {
        let after = self.lit(p, b":")?;
        let kind = self.space_end(after);
        self.cap(1, p, after);
        match self.name(kind) {
            Some(end) => {
                self.cap(2, kind, end);
                Some(end)
            }
            None => Some(kind),
        }
    }

    /// `(setget)\s+([A-Z_a-z]\w*)(?:,\s*([A-Z_a-z]\w*))?`.
    pub(super) fn setget(&mut self, p: usize) -> Option<usize> {
        let keyword = self.lit(p, b"setget")?;
        let name = self.spaces1(keyword)?;
        let name_end = self.name(name)?;
        self.cap(1, p, keyword);
        self.cap(2, name, name_end);
        if let Some(comma) = self.lit(name_end, b",") {
            let getter = self.space_end(comma);
            if let Some(end) = self.name(getter) {
                self.cap(3, getter, end);
                return Some(end);
            }
        }
        Some(name_end)
    }

    /// `\s*(signal)\s+([A-Z_a-z]\w*)(?=[\n\s])`: a signal without
    /// parameters.
    pub(super) fn signal_bare(&mut self, p: usize) -> Option<usize> {
        let keyword = self.space_end(p);
        let keyword_end = self.lit(keyword, b"signal")?;
        let name = self.spaces1(keyword_end)?;
        let end = self.name(name)?;
        if self.space_at(end) == 0 {
            return None;
        }
        self.cap(1, keyword, keyword_end);
        self.cap(2, name, end);
        Some(end)
    }

    /// `\s*(keyword)\s+([A-Z_a-z]\w*)\s*(?=\()`: the head of a function or a
    /// signal with parameters.
    pub(super) fn named_head(&mut self, p: usize, keyword: &[u8]) -> Option<usize> {
        let start = self.space_end(p);
        let keyword_end = self.lit(start, keyword)?;
        let name = self.spaces1(keyword_end)?;
        let name_end = self.name(name)?;
        let end = self.space_end(name_end);
        if !self.starts(end, b"(") {
            return None;
        }
        self.cap(1, start, keyword_end);
        self.cap(2, name, name_end);
        Some(end)
    }

    /// `(?=\b[A-Z_a-z]\w*\b\s*\()`: a call ahead.
    pub(super) fn call_ahead(&self, p: usize) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        let end = self.name(p)?;
        self.starts(self.space_end(end), b"(").then_some(p)
    }

    /// `(?=\))(?!\)\s*\()`: the `)` that ends a call, unless another call's
    /// `(` follows it.
    pub(super) fn arguments_end(&self, p: usize) -> Option<usize> {
        let next = self.lit(p, b")")?;
        (!self.starts(self.space_end(next), b"(")).then_some(p)
    }

    /// `\b([A-Z_a-z]\w*)\s*(=)(?!=)`: a named argument.
    pub(super) fn named_argument(&mut self, p: usize) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        let name_end = self.name(p)?;
        let equals = self.space_end(name_end);
        let end = self.lone_equals(equals)?;
        self.cap(1, p, name_end);
        self.cap(2, equals, end);
        Some(end)
    }

    /// `\s*(\))\s*(\()`: a call on what a call returned.
    pub(super) fn chained_call(&mut self, p: usize) -> Option<usize> {
        let close = self.space_end(p);
        let open = self.space_end(self.lit(close, b")")?);
        let end = self.lit(open, b"(")?;
        self.cap(1, close, close + 1);
        self.cap(2, open, end);
        Some(end)
    }

    /// `(?<=class_name)\s+([A-Z_a-z]\w*(\.([A-Z_a-z]\w*))?)`.
    pub(super) fn class_name(&mut self, p: usize) -> Option<usize> {
        if p < 10 || self.text(p - 10, p) != b"class_name" {
            return None;
        }
        let name = self.spaces1(p)?;
        let first = self.name(name)?;
        if self.starts(first, b".")
            && let Some(end) = self.name(first + 1)
        {
            self.cap(1, name, end);
            self.cap(2, first, end);
            self.cap(3, first + 1, end);
            return Some(end);
        }
        self.cap(1, name, first);
        Some(first)
    }

    /// `\b([A-Z_a-z]\w*).(new)\(`.
    /// `.` takes any character.
    /// So the name gives characters back until `new(` follows the next one.
    /// In `xynew(`, the name is `x`, the `.` takes `y`, and `new` follows.
    pub(super) fn class_new(&mut self, p: usize) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        let longest = self.name(p)?;
        let mut name_end = longest;
        loop {
            if let Some((_, after)) = self.char_at(name_end)
                && self.starts(after, b"new(")
            {
                self.cap(1, p, name_end);
                self.cap(2, after, after + 3);
                return Some(after + 4);
            }
            if name_end == p + 1 {
                return None;
            }
            name_end = self.char_start_before(name_end);
        }
    }

    /// `\s+(is)\s+(not?)\s+\s+([A-Z_a-z]\w*)`.
    /// `not?` gives its `t` back only to a `\s+` that cannot start at it.
    pub(super) fn class_is(&mut self, p: usize) -> Option<usize> {
        let keyword = self.spaces1(p)?;
        let keyword_end = self.lit(keyword, b"is")?;
        let negation = self.spaces1(keyword_end)?;
        let negation_end = self
            .lit(negation, b"not")
            .or_else(|| self.lit(negation, b"no"))?;
        let first = self.space_at(negation_end);
        let class = self.space_end(negation_end);
        if first == 0 || class == negation_end + first {
            return None;
        }
        let end = self.name(class)?;
        self.cap(1, keyword, keyword_end);
        self.cap(2, negation, negation_end);
        self.cap(3, class, end);
        Some(end)
    }

    /// `\b([A-Z][0-9A-Z_a-z]*)\.([0-9A-Z_]+)`: an enum member.
    pub(super) fn class_enum(&mut self, p: usize) -> Option<usize> {
        if self.word_before(p) || !(p < self.lim() && self.at(p).is_ascii_uppercase()) {
            return None;
        }
        let class = self.skip(p + 1, |b| b == b'_' || b.is_ascii_alphanumeric());
        let member = self.lit(class, b".")?;
        let end = self.skip(member, |b| {
            b == b'_' || b.is_ascii_uppercase() || b.is_ascii_digit()
        });
        if end == member {
            return None;
        }
        self.cap(1, p, class);
        self.cap(2, member, end);
        Some(end)
    }

    /// `\b([A-Z_a-z]\w*)\b(?=\s*\()`: a method name before its arguments.
    pub(super) fn any_method(&mut self, p: usize) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        let end = self.name(p)?;
        if !self.starts(self.space_end(end), b"(") {
            return None;
        }
        self.cap(1, p, end);
        Some(end)
    }

    /// `\b(?<![#$%@])([A-Z_a-z]\w*)\b(?!\()`: a variable.
    pub(super) fn any_variable(&mut self, p: usize) -> Option<usize> {
        if self.word_before(p) || matches!(self.prev(p), Some(b'#' | b'$' | b'%' | b'@')) {
            return None;
        }
        let end = self.name(p)?;
        if self.at(end) == b'(' && end < self.lim() {
            return None;
        }
        self.cap(1, p, end);
        Some(end)
    }

    /// `\b(\.)\s*(?<![#$%@])(?:([A-Z_][0-9A-Z_]*)|([A-Z_a-z]\w*))\b(?!\()`:
    /// a property.
    /// The constant form gives way where a `\w` or a `(` follows it.
    pub(super) fn any_property(&mut self, p: usize) -> Option<usize> {
        if !(self.starts(p, b".") && self.word_before(p)) {
            return None;
        }
        let name = self.space_end(p + 1);
        if matches!(self.prev(name), Some(b'#' | b'$' | b'%' | b'@')) {
            return None;
        }
        let first = self.at(name);
        if name < self.lim() && (first == b'_' || first.is_ascii_uppercase()) {
            let end = self.skip(name + 1, |b| {
                b == b'_' || b.is_ascii_uppercase() || b.is_ascii_digit()
            });
            if self.word_at(end).is_none() && !(end < self.lim() && self.at(end) == b'(') {
                self.cap(1, p, p + 1);
                self.cap(2, name, end);
                return Some(end);
            }
        }
        let end = self.name(name)?;
        if end < self.lim() && self.at(end) == b'(' {
            return None;
        }
        self.cap(1, p, p + 1);
        self.cap(3, name, end);
        Some(end)
    }

    /// `(?<![^.]\.|:)\b(classes)\b`: a builtin class.
    /// `[^.]\.` needs a character before the `.`, so a `.` at the start of
    /// the line does not keep a class from matching.
    pub(super) fn builtin_class(&mut self, p: usize) -> Option<usize> {
        match self.prev(p) {
            Some(b':') => return None,
            Some(b'.') if p >= 2 && self.prev(p - 1) != Some(b'.') => return None,
            Some(_) | None => {}
        }
        let end = self.word_in(p, BUILTIN_CLASSES)?;
        self.cap(1, p, end);
        Some(end)
    }

    /// `\b([A-Z_][0-9A-Z_]*)\b`: a constant.
    /// A shorter run leaves a `\w` where the last `\b` must hold.
    pub(super) fn const_var(&mut self, p: usize) -> Option<usize> {
        let first = self.at(p);
        if self.word_before(p) || !(p < self.lim() && (first == b'_' || first.is_ascii_uppercase()))
        {
            return None;
        }
        let end = self.skip(p + 1, |b| {
            b == b'_' || b.is_ascii_uppercase() || b.is_ascii_digit()
        });
        if self.word_at(end).is_some() {
            return None;
        }
        self.cap(1, p, end);
        Some(end)
    }

    /// `\b(?<!\.)(words)\b`: a statement keyword not right after a `.`.
    pub(super) fn statement_keyword(&mut self, p: usize, words: &[&[u8]]) -> Option<usize> {
        if self.prev(p) == Some(b'.') {
            return None;
        }
        self.word_group(p, words)
    }

    /// `(?:^|:)\s*(words)\b`, and when `ahead`, the lookahead
    /// `(?=\s*([-"#'(+:\[{\w\d]|$))` before the `\b`.
    /// The `^` comes first: at a `:` that starts the line, the keyword must
    /// then follow the `:`.
    pub(super) fn case_or_match(
        &mut self,
        p: usize,
        words: &[&[u8]],
        ahead: bool,
    ) -> Option<usize> {
        let starts = [(p == 0).then_some(p), self.lit(p, b":")];
        for start in starts.into_iter().flatten() {
            let keyword = self.space_end(start);
            let Some(end) = words.iter().find_map(|&word| self.lit(keyword, word)) else {
                continue;
            };
            if self.word_at(end).is_some() || (ahead && !self.case_ahead(end)) {
                continue;
            }
            self.cap(1, keyword, end);
            return Some(end);
        }
        None
    }

    /// `\s*([-"#'(+:\[{\w\d]|$)` at `p`.
    fn case_ahead(&self, p: usize) -> bool {
        let at = self.blanks_end(p);
        self.at_eol(at)
            || matches!(
                self.at(at),
                b'-' | b'"' | b'#' | b'\'' | b'(' | b'+' | b':' | b'[' | b'{'
            )
            || self.word_at(at).is_some()
    }

    /// `(@)(annotations)\b`: an annotation.
    pub(super) fn annotation(&mut self, p: usize) -> Option<usize> {
        let name = self.lit(p, b"@")?;
        let end = self.word_in(name, ANNOTATIONS)?;
        self.cap(1, p, name);
        self.cap(2, name, end);
        Some(end)
    }

    /// `\b[A-Z]+(?:[a-z]+[0-9A-Z_a-z]*)+\b`: a name in `PascalCase`.
    /// `[A-Z]+` gives capitals back only to a `[a-z]+` that cannot start at
    /// one, so the first run of capitals must meet a small letter.
    /// The rest takes the run of ASCII word characters, where the `\b` must
    /// then hold.
    pub(super) fn pascal_case(&self, p: usize) -> Option<usize> {
        if self.word_before(p) || !(p < self.lim() && self.at(p).is_ascii_uppercase()) {
            return None;
        }
        let capitals = self.skip(p, |b| b.is_ascii_uppercase());
        if !(capitals < self.lim() && self.at(capitals).is_ascii_lowercase()) {
            return None;
        }
        let end = self.skip(capitals, |b| b == b'_' || b.is_ascii_alphanumeric());
        self.word_at(end).is_none().then_some(end)
    }

    /// `0b[01_]+` or `0x[_\h]+`, with `prefix` and `digit`.
    pub(super) fn radix(&self, p: usize, prefix: &[u8], digit: fn(u8) -> bool) -> Option<usize> {
        let start = self.lit(p, prefix)?;
        let end = self.skip(start, digit);
        (end > start).then_some(end)
    }

    /// `[Ee][-+]?[0-9_]+` at `p`: an exponent.
    fn exponent(&self, p: usize) -> Option<usize> {
        if !(p < self.lim() && matches!(self.at(p), b'E' | b'e')) {
            return None;
        }
        let sign = p + 1;
        let digits = if sign < self.lim() && matches!(self.at(sign), b'+' | b'-') {
            sign + 1
        } else {
            sign
        };
        let end = self.skip(digits, |b| b == b'_' || b.is_ascii_digit());
        (end > digits).then_some(end)
    }

    /// `[0-9][0-9_]*` at `p`.
    fn digits(&self, p: usize) -> Option<usize> {
        (p < self.lim() && self.at(p).is_ascii_digit())
            .then(|| self.skip(p + 1, |b| b == b'_' || b.is_ascii_digit()))
    }

    /// `\.[0-9][0-9_]*([Ee][-+]?[0-9_]+)?`.
    pub(super) fn point_number(&mut self, p: usize) -> Option<usize> {
        let fraction = self.digits(self.lit(p, b".")?)?;
        Some(self.optional_exponent(fraction))
    }

    /// `([Ee][-+]?[0-9_]+)?` at `p`, as group 1 or 2 of the number.
    fn optional_exponent(&mut self, p: usize) -> usize {
        match self.exponent(p) {
            Some(end) => {
                self.cap(1, p, end);
                end
            }
            None => p,
        }
    }

    /// `([0-9][0-9_]*)\.[0-9_]*([Ee][-+]?[0-9_]+)?` or, when `exponent`,
    /// `([0-9][0-9_]*)?\.[0-9_]*([Ee][-+]?[0-9_]+)`.
    pub(super) fn decimal(&mut self, p: usize, exponent: bool) -> Option<usize> {
        let int = self.digits(p);
        if int.is_none() && !exponent {
            return None;
        }
        let point = int.unwrap_or(p);
        let fraction = self.skip(self.lit(point, b".")?, |b| b == b'_' || b.is_ascii_digit());
        if let Some(int) = int {
            self.cap(1, p, int);
        }
        match self.exponent(fraction) {
            Some(end) => {
                self.cap(2, fraction, end);
                Some(end)
            }
            None => (!exponent).then_some(fraction),
        }
    }

    /// `[0-9][0-9_]*[Ee][-+]?[0-9_]+`.
    pub(super) fn exponent_integer(&self, p: usize) -> Option<usize> {
        self.exponent(self.digits(p)?)
    }

    /// `-?[0-9][0-9_]*`.
    pub(super) fn integer(&self, p: usize) -> Option<usize> {
        let start = if self.starts(p, b"-") { p + 1 } else { p };
        self.digits(start)
    }

    /// The end of the line, with its line feed: `.*$\n?` from `p`.
    fn rest_of_line(&self, p: usize) -> usize {
        let end = self.lim().max(p);
        if self.at(end) == b'\n' { end + 1 } else { end }
    }

    /// `(##?).*$\n?`: a comment.
    pub(super) fn comment(&mut self, p: usize) -> Option<usize> {
        let mark = self.lit(p, b"##").or_else(|| self.lit(p, b"#"))?;
        self.cap(1, p, mark);
        Some(self.rest_of_line(mark))
    }

    /// `#(end)?region.*$\n?`.
    pub(super) fn region(&mut self, p: usize) -> Option<usize> {
        let after = self.lit(p, b"#")?;
        let word = match self.lit(after, b"end") {
            Some(end) if self.starts(end, b"region") => {
                self.cap(1, after, end);
                end
            }
            Some(_) | None => after,
        };
        Some(self.rest_of_line(self.lit(word, b"region")?))
    }

    /// `(\\)\s*(\S.*$\n?)`: text after a line continuation, which the grammar
    /// marks invalid.
    pub(super) fn continued_text(&mut self, p: usize) -> Option<usize> {
        let after = self.lit(p, b"\\")?;
        let text = self.space_end(after);
        if text >= self.lim() {
            return None;
        }
        let end = self.rest_of_line(text);
        self.cap(1, p, after);
        self.cap(2, text, end);
        Some(end)
    }

    /// `(\\)\s*$\n?`: a line continuation.
    /// `\s*` takes the line feed, and `$` holds after it.
    pub(super) fn continuation(&mut self, p: usize) -> Option<usize> {
        let after = self.lit(p, b"\\")?;
        let end = self.space_end(after);
        if !self.at_eol(end) {
            return None;
        }
        self.cap(1, p, after);
        Some(self.rest_of_line(end))
    }

    /// `(?=^\s*$)|(?!(\s*[Rr]?('''|"""|["']))|\G()$)`: what ends a line
    /// continuation.
    ///
    /// The first branch holds on a line of blanks.
    /// The second holds where no string opens after blanks and `\G()$` fails.
    /// So the rule goes on only while the next line opens a string.
    pub(super) fn continuation_end(&self, p: usize) -> Option<usize> {
        if p == 0 && self.space_end(0) >= self.lim() {
            return Some(p);
        }
        let start = self.space_end(p);
        let quote = if matches!(self.at(start), b'R' | b'r') && start < self.lim() {
            start + 1
        } else {
            start
        };
        let opens = self.starts(quote, b"'''")
            || self.starts(quote, b"\"\"\"")
            || matches!(self.at(quote), b'"' | b'\'') && quote < self.lim();
        let empty = self.at_g(p) && self.at_eol(p);
        (!opens && !empty).then_some(p)
    }
}
