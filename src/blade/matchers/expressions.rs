//! Blade's own expressions: variables, members, calls, casts, operators and
//! keywords.
//! Each differs from PHP's in Blade's name class, `[_a-z\x7F-ÿ]`, or in an
//! older form of the expression.

use super::super::engine::Lexer;
use crate::rules::Chars;

impl Lexer<'_> {
    /// `(?i)(\$+)NAME\b`.
    pub(super) fn variable(&mut self, p: usize) -> Option<usize> {
        let at = self.skip(p, |b| b == b'$');
        if at == p {
            return None;
        }
        let end = self.bname_bounded(at)?;
        self.capture(1, p, at);
        Some(end)
    }

    /// `(?i)(\$+)(NAME)(?=\s*\()`.
    pub(super) fn invoke(&mut self, p: usize) -> Option<usize> {
        let at = self.skip(p, |b| b == b'$');
        if at == p {
            return None;
        }
        let end = self.bname(at)?;
        if self.at(self.space_end(end)) != b'(' {
            return None;
        }
        self.capture(1, p, at);
        self.capture(2, at, end);
        Some(end)
    }

    /// A variable in a string, with a property or an index:
    ///
    /// ```text
    /// (?i)((\$)(?<name>NAME))(?:(->)(\g<name>)|(\[)(?:(\d+)|((\$)\g<name>)|(NAME))(]))?
    /// ```
    pub(super) fn interpolated_variable(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'$' {
            return None;
        }
        let name = self.bname(p + 1)?;
        self.capture(1, p, name);
        self.capture(2, p, p + 1);
        if let Some(arrow) = self.lit(name, b"->")
            && let Some(end) = self.bname(arrow)
        {
            self.capture(4, name, arrow);
            self.capture(5, arrow, end);
            return Some(end);
        }
        if self.at(name) != b'[' {
            return Some(name);
        }
        let index = name + 1;
        let digits = self.digit_end(index);
        if digits > index && self.at(digits) == b']' {
            self.capture(6, name, index);
            self.capture(7, index, digits);
            self.capture(11, digits, digits + 1);
            return Some(digits + 1);
        }
        if self.at(index) == b'$'
            && let Some(end) = self.bname(index + 1)
            && self.at(end) == b']'
        {
            self.capture(6, name, index);
            self.capture(8, index, end);
            self.capture(9, index, index + 1);
            self.capture(11, end, end + 1);
            return Some(end + 1);
        }
        if let Some(end) = self.bname(index)
            && self.at(end) == b']'
        {
            self.capture(6, name, index);
            self.capture(10, index, end);
            self.capture(11, end, end + 1);
            return Some(end + 1);
        }
        Some(name)
    }

    /// `(?i)((\$\{)(?<name>NAME)(}))`.
    pub(super) fn braced_variable(&mut self, p: usize) -> Option<usize> {
        let open = self.lit(p, b"${")?;
        let name = self.bname(open)?;
        if self.at(name) != b'}' {
            return None;
        }
        self.capture(1, p, name + 1);
        self.capture(2, p, open);
        self.capture(4, name, name + 1);
        Some(name + 1)
    }

    /// `(?i)\b(NAME)(?=\s*::)`.
    pub(super) fn class_before_scope(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let end = self.bname(p)?;
        if !self.starts(self.space_end(end), b"::") {
            return None;
        }
        self.capture(1, p, end);
        Some(end)
    }

    /// `(?i)(::)\s*(NAME)\s*(\()`.
    pub(super) fn static_call(&mut self, p: usize) -> Option<usize> {
        let colons = self.lit(p, b"::")?;
        let at = self.space_end(colons);
        let name = self.bname(at)?;
        let open = self.space_end(name);
        if self.at(open) != b'(' {
            return None;
        }
        self.capture(1, p, colons);
        self.capture(2, at, name);
        self.capture(3, open, open + 1);
        Some(open + 1)
    }

    /// `(?i)(::)\s*(?:((\$+)NAME)|(NAME))?`.
    pub(super) fn static_access(&mut self, p: usize) -> Option<usize> {
        let colons = self.lit(p, b"::")?;
        let at = self.space_end(colons);
        self.capture(1, p, colons);
        let dollars = self.skip(at, |b| b == b'$');
        if dollars > at {
            if let Some(end) = self.bname(dollars) {
                self.capture(2, at, end);
                self.capture(3, at, dollars);
                return Some(end);
            }
            return Some(at);
        }
        match self.bname(at) {
            Some(end) => {
                self.capture(4, at, end);
                Some(end)
            }
            None => Some(at),
        }
    }

    /// `(->)(\$?\{)`.
    pub(super) fn object_brace(&mut self, p: usize) -> Option<usize> {
        let arrow = self.lit(p, b"->")?;
        let end = self.lit(arrow, b"${").or_else(|| self.lit(arrow, b"{"))?;
        self.capture(1, p, arrow);
        self.capture(2, arrow, end);
        Some(end)
    }

    /// `(?i)(->)(NAME)\s*(\()`.
    pub(super) fn method_call(&mut self, p: usize) -> Option<usize> {
        let arrow = self.lit(p, b"->")?;
        let name = self.bname(arrow)?;
        let open = self.space_end(name);
        if self.at(open) != b'(' {
            return None;
        }
        self.capture(1, p, arrow);
        self.capture(2, arrow, name);
        self.capture(3, open, open + 1);
        Some(open + 1)
    }

    /// `(?i)(->)((\$+)?NAME)?`.
    /// Without a name after the `$`, no name follows the arrow either, so the
    /// match is the arrow alone.
    pub(super) fn property_access(&mut self, p: usize) -> Option<usize> {
        let arrow = self.lit(p, b"->")?;
        self.capture(1, p, arrow);
        let dollars = self.skip(arrow, |b| b == b'$');
        let Some(end) = self.bname(dollars) else {
            return Some(arrow);
        };
        if dollars > arrow {
            self.capture(3, arrow, dollars);
        }
        self.capture(2, arrow, end);
        Some(end)
    }

    /// `(?i)(\\?\bNAME(?:\\NAME)+)\s*(\()`: a call of a namespaced function.
    pub(super) fn qualified_call(&mut self, p: usize) -> Option<usize> {
        let at = p + usize::from(self.at(p) == b'\\');
        if !self.boundary(at) {
            return None;
        }
        let mut end = self.bname(at)?;
        let mut parts = 0;
        while self.at(end) == b'\\'
            && let Some(next) = self.bname(end + 1)
        {
            end = next;
            parts += 1;
        }
        let open = self.space_end(end);
        if parts == 0 || self.at(open) != b'(' {
            return None;
        }
        self.capture(1, p, end);
        self.capture(2, open, open + 1);
        Some(open + 1)
    }

    /// `(?i)(\\)?\b(NAME)\s*(\()`.
    pub(super) fn call(&mut self, p: usize) -> Option<usize> {
        let slash = self.at(p) == b'\\';
        let at = p + usize::from(slash);
        if !self.boundary(at) {
            return None;
        }
        let end = self.bname(at)?;
        let open = self.space_end(end);
        if self.at(open) != b'(' {
            return None;
        }
        if slash {
            self.capture(1, p, at);
        }
        self.capture(2, at, end);
        self.capture(3, open, open + 1);
        Some(open + 1)
    }

    /// `(?i)(new)\s+`.
    pub(super) fn new_keyword(&mut self, p: usize) -> Option<usize> {
        let end = self.ilit(p, b"new")?;
        let at = self.space_end(end);
        if at == end {
            return None;
        }
        self.capture(1, p, end);
        Some(at)
    }

    /// `(?i)(parent|static|self)(?![0-9_a-z\x7F-ÿ])`.
    pub(super) fn class_keyword(&mut self, p: usize) -> Option<usize> {
        let end = [b"parent".as_slice(), b"static", b"self"]
            .iter()
            .find_map(|word| self.ilit(p, word))?;
        if self.name_continues(end) {
            return None;
        }
        self.capture(1, p, end);
        Some(end)
    }

    /// `(?=[^$0-9\\_a-z\x7F-ÿ])`, without case folding: an upper-case letter
    /// ends it too.
    pub(super) fn not_lower_name_at(&self, p: usize) -> Option<usize> {
        if p >= self.lim() {
            return self.avail(p).then_some(p);
        }
        let (c, _) = self.char_at(p)?;
        let inside = c == '$'
            || c == '\\'
            || c == '_'
            || c.is_ascii_digit()
            || c.is_ascii_lowercase()
            || ('\u{7f}'..='\u{ff}').contains(&c);
        (!inside).then_some(p)
    }

    /// `(array)(\()`.
    pub(super) fn array_call(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"array(")?;
        self.capture(1, p, end - 1);
        self.capture(2, end - 1, end);
        Some(end)
    }

    /// `(?i)(\()\s*(array|real|double|float|int(?:eger)?|bool(?:ean)?
    /// |string|object|binary|unset)\s*(\))`. `int` and `bool` take their longer
    /// forms first, and give way to the short ones where `)` does not follow
    /// the long ones.
    pub(super) fn cast(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'(' {
            return None;
        }
        let at = self.space_end(p + 1);
        let words: [&[u8]; 12] = [
            b"array", b"real", b"double", b"float", b"integer", b"int", b"boolean", b"bool",
            b"string", b"object", b"binary", b"unset",
        ];
        for word in words {
            if let Some(end) = self.ilit(at, word) {
                let close = self.space_end(end);
                if self.at(close) == b')' {
                    self.capture(1, p, p + 1);
                    self.capture(2, at, end);
                    self.capture(3, close, close + 1);
                    return Some(close + 1);
                }
            }
        }
        None
    }

    /// `(?:|[-%\&*+/^|]|<<|>>)=`: the empty alternative comes first, so `=`
    /// alone matches before any compound form.
    pub(super) fn assignment(&self, p: usize) -> Option<usize> {
        if self.at(p) == b'=' && p < self.lim() {
            return Some(p + 1);
        }
        let op = match self.at(p) {
            b'-' | b'%' | b'&' | b'*' | b'+' | b'/' | b'^' | b'|' => p + 1,
            b'<' | b'>' if self.at(p + 1) == self.at(p) => p + 2,
            _ => return None,
        };
        (self.at(op) == b'=').then_some(op + 1)
    }

    /// `[-%*+/]`.
    pub(super) fn arithmetic(&self, p: usize) -> Option<usize> {
        matches!(self.at(p), b'-' | b'%' | b'*' | b'+' | b'/').then_some(p + 1)
    }

    /// `(?i)(!|&&|\|\|)|\b(and|or|xor|as)\b`.
    pub(super) fn logical(&self, p: usize) -> Option<usize> {
        if self.at(p) == b'!' {
            return Some(p + 1);
        }
        self.lit(p, b"&&")
            .or_else(|| self.lit(p, b"||"))
            .or_else(|| self.iword_in(p, &[b"and", b"or", b"xor", b"as"]))
    }

    /// `\s*\b(break|case|…|yield)\b`, without case folding, with the word as
    /// group 1.
    pub(super) fn control_keyword(&mut self, p: usize) -> Option<usize> {
        let at = self.space_end(p);
        let end = self.word_in(
            at,
            &[
                b"break",
                b"case",
                b"continue",
                b"declare",
                b"default",
                b"die",
                b"do",
                b"elseif",
                b"else",
                b"enddeclare",
                b"endforeach",
                b"endfor",
                b"endif",
                b"endswitch",
                b"endwhile",
                b"exit",
                b"foreach",
                b"for",
                b"if",
                b"return",
                b"switch",
                b"use",
                b"while",
                b"yield",
            ],
        )?;
        self.capture(1, at, end);
        Some(end)
    }

    /// `(?i)\b((?:require|include)(?:_once)?)\s+`.
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
            let at = self.space_end(end);
            if at > end {
                self.capture(1, p, end);
                return Some(at);
            }
        }
        None
    }

    /// `(?i)(goto)\s+(NAME)`.
    pub(super) fn goto(&mut self, p: usize) -> Option<usize> {
        let word = self.ilit(p, b"goto")?;
        let at = self.space_end(word);
        if at == word {
            return None;
        }
        let end = self.bname(at)?;
        self.capture(1, p, word);
        self.capture(2, at, end);
        Some(end)
    }

    /// `(?i)^\s*(NAME)\s*:(?!:)`: a label.
    pub(super) fn label(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let at = self.space_end(0);
        let end = self.bname(at)?;
        let colon = self.space_end(end);
        if self.at(colon) != b':' || colon >= self.lim() || self.at(colon + 1) == b':' {
            return None;
        }
        self.capture(1, at, end);
        Some(colon + 1)
    }

    /// `(?i)(?=[0-9\\_a-z\x7F-ÿ]+(::)(NAME)?)`.
    pub(super) fn ahead_scope(&self, p: usize) -> Option<usize> {
        let end = self.qualified_end(p);
        (end > p && self.starts(end, b"::")).then_some(p)
    }

    /// `(?i)(::)(NAME)?`.
    pub(super) fn scope_member(&mut self, p: usize) -> Option<usize> {
        let colons = self.lit(p, b"::")?;
        self.capture(1, p, colons);
        match self.bname(colons) {
            Some(end) => {
                self.capture(2, colons, end);
                Some(end)
            }
            None => Some(colons),
        }
    }
}
