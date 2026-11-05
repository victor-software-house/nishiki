//! Names, attributes and declarations: namespaces, `use`, classes, traits,
//! interfaces, enums, functions, parameters, properties and constants.

use super::super::engine::Lexer;
use super::{ParameterVariable, ReturnEnd, is_qualified};
use crate::php_family::matchers::is_name;
use crate::rules::Chars;

impl Lexer<'_> {
    /// `(?i)(?=\\?NAME\\)`: a name with a namespace after it.
    pub(super) fn ahead_namespaced(&self, p: usize) -> Option<usize> {
        let from = p + usize::from(self.at(p) == b'\\');
        let end = self.name(from)?;
        (self.at(end) == b'\\').then_some(p)
    }

    /// `(?i)(NAME)?(?![0-9\\_a-z\x7F-\x{10FFFF}])`: the last segment of a
    /// qualified name, as group 1.
    /// A shorter name would leave a name character after it, and no name
    /// leaves the name at `p`.
    pub(super) fn last_segment(&mut self, p: usize) -> Option<usize> {
        match self.name(p) {
            Some(end) if !is_qualified(self.at(end)) || end > self.lim() => {
                self.cap(1, p, end);
                Some(end)
            }
            Some(_) => None,
            None => (!is_qualified(self.at(p)) || p >= self.lim()).then_some(p),
        }
    }

    /// `(?i)(NAME)?(?=[^0-9\\_a-z\x7F-\x{10FFFF}])`: like
    /// [`Lexer::last_segment`], but a character must follow.
    pub(super) fn last_segment_before_other(&mut self, p: usize) -> Option<usize> {
        let followed = |lexer: &Self, at: usize| {
            lexer.avail(at) && (at == lexer.lim() || !is_qualified(lexer.at(at)))
        };
        match self.name(p) {
            Some(end) if followed(self, end) => {
                self.cap(1, p, end);
                Some(end)
            }
            Some(_) => None,
            None => followed(self, p).then_some(p),
        }
    }

    /// `(?i)(NAME)(?![0-9\\_a-z\x7F-\x{10FFFF}])`.
    pub(super) fn name_alone(&mut self, p: usize) -> Option<usize> {
        let end = self.name(p)?;
        if end < self.lim() && is_qualified(self.at(end)) {
            return None;
        }
        self.cap(1, p, end);
        Some(end)
    }

    /// `(?i)(?:(namespace)|NAME)?(\\)`.
    pub(super) fn namespace_part(&mut self, p: usize) -> Option<usize> {
        if let Some(end) = self.ilit(p, b"namespace")
            && self.at(end) == b'\\'
        {
            self.cap(1, p, end);
            self.cap(2, end, end + 1);
            return Some(end + 1);
        }
        let at = self.name(p).unwrap_or(p);
        if self.at(at) != b'\\' {
            return None;
        }
        self.cap(2, at, at + 1);
        Some(at + 1)
    }

    /// `(?i)(?![0-9_a-z\x7F-\x{10FFFF}]*\\)`.
    pub(super) fn namespace_end(&self, p: usize) -> Option<usize> {
        (self.at(self.skip(p, is_name)) != b'\\').then_some(p)
    }

    /// `(?i)(\\)?\
    /// b(Attribute|SensitiveParameter|AllowDynamicProperties|ReturnTypeWillChange|Override|Deprecated)\
    /// b`.
    pub(super) fn attribute_name(&mut self, p: usize) -> Option<usize> {
        let slash = self.at(p) == b'\\';
        let at = p + usize::from(slash);
        let end = self.iword_in(
            at,
            &[
                b"attribute",
                b"sensitiveparameter",
                b"allowdynamicproperties",
                b"returntypewillchange",
                b"override",
                b"deprecated",
            ],
        )?;
        if slash {
            self.cap(1, p, at);
        }
        Some(end)
    }

    /// `([0-9A-Z\\_a-z\x7F-\x{10FFFF}]+)\s*(\()`.
    pub(super) fn attribute_call(&mut self, p: usize) -> Option<usize> {
        let end = self.qualified(p)?;
        let open = self.space_end(end);
        if self.at(open) != b'(' {
            return None;
        }
        self.cap(1, p, end);
        self.cap(2, open, open + 1);
        Some(open + 1)
    }

    /// `(?i)(?<=^|[(,])\s*(NAME)\s*(:)(?!:)`.
    pub(super) fn named_argument(&mut self, p: usize) -> Option<usize> {
        if p != 0 && !matches!(self.prev(p), Some(b'(' | b',')) {
            return None;
        }
        let at = self.space_end(p);
        let end = self.name(at)?;
        let colon = self.space_end(end);
        if self.at(colon) != b':' || self.at(colon + 1) == b':' {
            return None;
        }
        self.cap(1, at, end);
        self.cap(2, colon, colon + 1);
        Some(colon + 1)
    }

    /// `(?i)(?:^|(?<=<\?php))\s*(namespace)\s+([0-9\\_a-z\x7F-\x{10FFFF}]+)(?=\
    /// s*;)` with `named`, else `(?i)(?:^|(?<=<\?php))\s*(namespace)\s+`.
    pub(super) fn namespace_statement(&mut self, p: usize, named: bool) -> Option<usize> {
        let after_tag = p >= 5 && self.text(p - 5, p).eq_ignore_ascii_case(b"<?php");
        if p != 0 && !after_tag {
            return None;
        }
        let word = self.space_end(p);
        let word_end = self.ilit(word, b"namespace")?;
        let at = self.space_end(word_end);
        if at == word_end {
            return None;
        }
        if !named {
            self.cap(1, word, word_end);
            return Some(at);
        }
        let end = self.qualified(at)?;
        if self.at(self.space_end(end)) != b';' {
            return None;
        }
        self.cap(1, word, word_end);
        self.cap(2, at, end);
        Some(end)
    }

    /// `(?i)\b(as)\s+((final|abstract)|(p(?:ublic|rivate|rotected))|(static))\
    /// s+(NAME)`.
    pub(super) fn trait_alias_modifier(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let as_end = self.ilit(p, b"as")?;
        let at = self.space_end(as_end);
        if at == as_end {
            return None;
        }
        let words: [(&[u8], usize); 6] = [
            (b"final", 3),
            (b"abstract", 3),
            (b"public", 4),
            (b"private", 4),
            (b"protected", 4),
            (b"static", 5),
        ];
        let (end, group) = words
            .iter()
            .find_map(|&(word, group)| self.ilit(at, word).map(|end| (end, group)))?;
        let name_at = self.space_end(end);
        if name_at == end {
            return None;
        }
        let name = self.name(name_at)?;
        self.cap(1, p, as_end);
        self.cap(2, at, end);
        self.cap(group, at, end);
        self.cap(6, name_at, name);
        Some(name)
    }

    /// `(?i)\b(as)\s+`.
    pub(super) fn use_as(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        self.word_then_spaces(p, b"as")
    }

    /// `(?i)(?=[^0-9A-Z\\_a-z\x7F-\x{10FFFF}])`, and with `dollar` also not
    /// `$`.
    pub(super) fn ahead_other(&self, p: usize, dollar: bool) -> Option<usize> {
        let b = self.at(p);
        let other = p == self.lim() || !(is_qualified(b) || (dollar && b == b'$'));
        (self.avail(p) && other).then_some(p)
    }

    /// `(?i)\b(const)\s+(?:(T)\s+)?(NAME)`.
    pub(super) fn class_constant(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let word_end = self.ilit(p, b"const")?;
        let at = self.space_end(word_end);
        if at == word_end {
            return None;
        }
        let typed = self.each_type(at, &mut |lexer, end| {
            let name_at = lexer.space_end(end);
            if name_at == end {
                return None;
            }
            lexer.name(name_at).map(|name| (end, name_at, name))
        });
        self.cap(1, p, word_end);
        if let Some((end, name_at, name)) = typed {
            self.cap(2, at, end);
            self.cap(3, name_at, name);
            return Some(name);
        }
        let name = self.name(at)?;
        self.cap(3, at, name);
        Some(name)
    }

    /// `(?i)\b(enum)\s+(NAME)(?:\s*(:)\s*(int|string)\b)?`.
    pub(super) fn enum_head(&mut self, p: usize) -> Option<usize> {
        let name = self.keyword_name(p, b"enum")?;
        let colon = self.space_end(name);
        if self.at(colon) != b':' {
            return Some(name);
        }
        let at = self.space_end(colon + 1);
        let Some(end) = [b"int".as_slice(), b"string"].iter().find_map(|word| {
            self.ilit(at, word)
                .filter(|&end| self.word_at(end).is_none())
        }) else {
            return Some(name);
        };
        self.cap(3, colon, colon + 1);
        self.cap(4, at, end);
        Some(end)
    }

    /// `(?i)\b((case))\s*(NAME)`.
    pub(super) fn enum_case(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let word_end = self.ilit(p, b"case")?;
        let at = self.space_end(word_end);
        let name = self.name(at)?;
        self.cap(1, p, word_end);
        self.cap(2, p, word_end);
        self.cap(3, at, name);
        Some(name)
    }

    /// `(?i)(a|b)|(c)|…`: the first of `words` at `p`, as its group.
    pub(super) fn modifier(&mut self, p: usize, words: &[(&[u8], usize)]) -> Option<usize> {
        let (end, group) = words
            .iter()
            .find_map(|&(word, group)| self.ilit(p, word).map(|end| (end, group)))?;
        self.cap(group, p, end);
        Some(end)
    }

    /// `(?i)\b((p(?:ublic|rivate|rotected))\(set\))|\b(p(?:
    /// ublic|rivate|rotected))\b|\b(readonly)\b`, and with `statics` the last
    /// also takes `static`.
    pub(super) fn visibility(&mut self, p: usize, statics: bool) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let visibility = [b"public".as_slice(), b"private", b"protected"]
            .iter()
            .find_map(|word| self.ilit(p, word));
        if let Some(end) = visibility {
            if let Some(set) = self.ilit(end, b"(set)") {
                self.cap(1, p, set);
                self.cap(2, p, end);
                return Some(set);
            }
            if self.word_at(end).is_none() {
                self.cap(3, p, end);
                return Some(end);
            }
        }
        let others: &[&[u8]] = if statics {
            &[b"readonly", b"static"]
        } else {
            &[b"readonly"]
        };
        let end = self.iword_in(p, others)?;
        self.cap(4, p, end);
        Some(end)
    }

    /// The class head:
    ///
    /// ```text
    /// (?i)\b(?:((?:(?:final|abstract|readonly)\s+)*)(class)\s+(NAME)
    /// |(new)\b\s*(#\[.*])?\s*(?:((readonly))\s+)?\b(class)\b)
    /// ```
    pub(super) fn class_head(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let mut at = p;
        while let Some(end) = [b"final".as_slice(), b"abstract", b"readonly"]
            .iter()
            .find_map(|word| self.ilit(at, word))
        {
            let next = self.space_end(end);
            if next == end {
                break;
            }
            at = next;
        }
        if let Some(class_end) = self.ilit(at, b"class") {
            let name_at = self.space_end(class_end);
            if name_at > class_end
                && let Some(name) = self.name(name_at)
            {
                self.cap(1, p, at);
                self.cap(2, at, class_end);
                self.cap(3, name_at, name);
                return Some(name);
            }
        }
        let new_end = self.ilit(p, b"new")?;
        if self.word_at(new_end).is_some() {
            return None;
        }
        let after_new = self.space_end(new_end);
        let mut below = self.lim();
        loop {
            let attribute = if self.starts(after_new, b"#[") {
                (after_new + 2..below).rev().find(|&at| self.at(at) == b']')
            } else {
                None
            };
            let from = attribute.map_or(after_new, |close| close + 1);
            if let Some(rest) = self.new_class_rest(self.space_end(from)) {
                self.cap(4, p, new_end);
                if let Some(close) = attribute {
                    self.cap(5, after_new, close + 1);
                }
                if let Some((start, end)) = rest.readonly {
                    self.cap(6, start, end);
                    self.cap(7, start, end);
                }
                self.cap(8, rest.class.0, rest.class.1);
                return Some(rest.class.1);
            }
            below = attribute?;
        }
    }

    /// `(?:((readonly))\s+)?\b(class)\b` at `at`.
    fn new_class_rest(&self, at: usize) -> Option<NewClass> {
        let class = |lexer: &Self, from: usize| {
            if !lexer.boundary(from) {
                return None;
            }
            let end = lexer.ilit(from, b"class")?;
            lexer.word_at(end).is_none().then_some(end)
        };
        if let Some(readonly) = self.ilit(at, b"readonly") {
            let next = self.space_end(readonly);
            if next > readonly
                && let Some(end) = class(self, next)
            {
                return Some(NewClass {
                    readonly: Some((at, readonly)),
                    class: (next, end),
                });
            }
        }
        class(self, at).map(|end| NewClass {
            readonly: None,
            class: (at, end),
        })
    }

    /// `(?<=class)\s*(\()`.
    pub(super) fn anonymous_class_arguments(&mut self, p: usize) -> Option<usize> {
        if p < 5 || self.text(p - 5, p) != b"class" {
            return None;
        }
        let open = self.space_end(p);
        if self.at(open) != b'(' {
            return None;
        }
        self.cap(1, open, open + 1);
        Some(open + 1)
    }

    /// `(?i)\b(word)\s*(?=&?\s*\()`, for `function` and `fn`.
    pub(super) fn function_keyword(&mut self, p: usize, word: &[u8]) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let end = self.ilit(p, word)?;
        let at = self.space_end(end);
        let amp = at + usize::from(self.at(at) == b'&');
        if self.at(self.space_end(amp)) != b'(' {
            return None;
        }
        self.cap(1, p, end);
        Some(at)
    }

    /// `(&)?\s*(\()`.
    pub(super) fn parameters_open(&mut self, p: usize) -> Option<usize> {
        let amp = self.at(p) == b'&';
        let open = self.space_end(p + usize::from(amp));
        if self.at(open) != b'(' {
            return None;
        }
        if amp {
            self.cap(1, p, p + 1);
        }
        self.cap(2, open, open + 1);
        Some(open + 1)
    }

    /// `(?:(&)\s*)?(\()`.
    pub(super) fn arrow_parameters_open(&mut self, p: usize) -> Option<usize> {
        if self.at(p) == b'&' {
            let open = self.space_end(p + 1);
            if self.at(open) != b'(' {
                return None;
            }
            self.cap(1, p, p + 1);
            self.cap(2, open, open + 1);
            return Some(open + 1);
        }
        let end = self.lit(p, b"(")?;
        self.cap(2, p, end);
        Some(end)
    }

    /// `(?i)((?:(&)\s*)?(\$+)NAME)\s*(?=[),])`.
    pub(super) fn closure_use_variable(&mut self, p: usize) -> Option<usize> {
        let amp = self.at(p) == b'&';
        let dollars = if amp { self.space_end(p + 1) } else { p };
        let name_at = self.skip(dollars, |b| b == b'$');
        if name_at == dollars {
            return None;
        }
        let end = self.name(name_at)?;
        let after = self.space_end(end);
        if !matches!(self.at(after), b')' | b',') || after >= self.lim() {
            return None;
        }
        self.cap(1, p, end);
        if amp {
            self.cap(2, p, p + 1);
        }
        self.cap(3, dollars, name_at);
        Some(after)
    }

    /// `(?i)(:)\s*(T)(?=\s*(?:\{|/[*/]|#|$))` for a function, and with `=>`
    /// in place of `\{` for an arrow function.
    pub(super) fn return_type(&mut self, p: usize, follows: ReturnEnd) -> Option<usize> {
        if self.at(p) != b':' {
            return None;
        }
        let at = self.space_end(p + 1);
        let end = self.each_type(at, &mut |lexer, end| {
            let next = lexer.space_end(end);
            let body = match follows {
                ReturnEnd::Brace => lexer.at(next) == b'{' && next < lexer.lim(),
                ReturnEnd::Arrow => lexer.starts(next, b"=>"),
            };
            let comment =
                lexer.starts(next, b"/*") || lexer.starts(next, b"//") || lexer.starts(next, b"#");
            (body || comment || lexer.at_eol(next)).then_some(end)
        })?;
        self.cap(1, p, p + 1);
        self.cap(2, at, end);
        Some(end)
    }

    /// Records a parameter's variable: the whole as group `whole`, its `&` as
    /// `whole + 1` and its `$` as `whole + 2`.
    fn record_variable(&mut self, from: usize, variable: ParameterVariable, whole: usize) {
        self.cap(whole, from, variable.end);
        if let Some(amp) = variable.amp {
            self.cap(whole + 1, amp, amp + 1);
        }
        self.cap(whole + 2, variable.dollar, variable.dollar + 1);
    }

    /// `(?=\s*(?:[),]|/[*/]|#|$))`.
    fn parameter_end_or_eol(&self, p: usize) -> bool {
        self.parameter_end(p) || self.at_eol(self.space_end(p))
    }

    /// `(?i)(?:(T)\s+)?((?:(&)\s*)?(\.\.\.)(\$)NAME)(?=\s*(?:[),]|/[*/]|#|$))`.
    pub(super) fn variadic_parameter(&mut self, p: usize) -> Option<usize> {
        let variadic = |lexer: &Self, at: usize| {
            let (amp, dots) = if lexer.at(at) == b'&' {
                (Some(at), lexer.space_end(at + 1))
            } else {
                (None, at)
            };
            let dollar = lexer.lit(dots, b"...")?;
            if lexer.at(dollar) != b'$' {
                return None;
            }
            let end = lexer.name(dollar + 1)?;
            lexer
                .parameter_end_or_eol(end)
                .then_some((amp, dots, dollar, end))
        };
        let typed = self.each_type(p, &mut |lexer, end| {
            let at = lexer.space_end(end);
            if at == end {
                return None;
            }
            variadic(lexer, at).map(|found| (end, at, found))
        });
        let (ty, at, (amp, dots, dollar, end)) = match typed {
            Some((ty_end, at, found)) => (Some(ty_end), at, found),
            None => (None, p, variadic(self, p)?),
        };
        if let Some(ty_end) = ty {
            self.cap(1, p, ty_end);
        }
        self.cap(2, at, end);
        if let Some(amp) = amp {
            self.cap(3, amp, amp + 1);
        }
        self.cap(4, dots, dollar);
        self.cap(5, dollar, dollar + 1);
        Some(end)
    }

    /// `(?i)(T)\s+((?:(&)\s*)?(\$)NAME)`.
    pub(super) fn typed_parameter(&mut self, p: usize) -> Option<usize> {
        let (ty_end, at, variable) = self.each_type(p, &mut |lexer, end| {
            let at = lexer.space_end(end);
            if at == end {
                return None;
            }
            lexer
                .parameter_variable(at)
                .map(|variable| (end, at, variable))
        })?;
        self.cap(1, p, ty_end);
        self.record_variable(at, variable, 2);
        Some(variable.end)
    }

    /// `(?i)((?:(&)\s*)?(\$)NAME)(?=\s*(?:[),]|/[*/]|#|$))`.
    pub(super) fn untyped_parameter(&mut self, p: usize) -> Option<usize> {
        let variable = self.parameter_variable(p)?;
        if !self.parameter_end_or_eol(variable.end) {
            return None;
        }
        self.record_variable(p, variable, 1);
        Some(variable.end)
    }

    /// `(?i)((?:(&)\s*)?(\$)NAME)\s*(=)\s*`.
    pub(super) fn defaulted_parameter(&mut self, p: usize) -> Option<usize> {
        let variable = self.parameter_variable(p)?;
        let equals = self.space_end(variable.end);
        if self.at(equals) != b'=' {
            return None;
        }
        self.record_variable(p, variable, 1);
        self.cap(4, equals, equals + 1);
        Some(self.space_end(equals + 1))
    }

    /// `((?:(?:a|b|…)\s+)*)`: case-sensitive modifiers before `function`, and
    /// where they end.
    fn modifiers(&self, p: usize, words: &[&[u8]]) -> usize {
        let mut at = p;
        while let Some(end) = words.iter().find_map(|word| self.lit(at, word)) {
            let next = self.space_end(end);
            if next == end {
                break;
            }
            at = next;
        }
        at
    }

    /// `((?:(?:final|abstract|public|private|protected)\s+)*)(function)\
    /// s+(__construct)\s*(\()`.
    pub(super) fn constructor(&mut self, p: usize) -> Option<usize> {
        let at = self.modifiers(
            p,
            &[b"final", b"abstract", b"public", b"private", b"protected"],
        );
        let function_end = self.lit(at, b"function")?;
        let name_at = self.space_end(function_end);
        if name_at == function_end {
            return None;
        }
        let name = self.lit(name_at, b"__construct")?;
        let open = self.space_end(name);
        if self.at(open) != b'(' {
            return None;
        }
        self.cap(1, p, at);
        self.cap(2, at, function_end);
        self.cap(3, name_at, name);
        self.cap(4, open, open + 1);
        Some(open + 1)
    }

    /// `(?=\s*(?:\{|/[*/]|#|$|;))`, after a signature.
    fn body_follows(&self, p: usize) -> bool {
        let at = self.space_end(p);
        (matches!(self.at(at), b'{' | b'#' | b';') && at < self.lim())
            || self.starts(at, b"/*")
            || self.starts(at, b"//")
            || self.at_eol(at)
    }

    /// The end of a constructor's parameters:
    ///
    /// ```text
    /// (?i)(\))\s*(:\s*(?:\?\s*)?(?!\s)[\&()0-9\\_a-z|\x7F-\x{10FFFF}\s]+(?<!\s))?(?=\s*(?:\{|/[*/]|#|$|;))
    /// ```
    ///
    /// The class run gives back one character at a time until it ends on a
    /// character that is not `\s` and a body follows.
    pub(super) fn constructor_close(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b')' || p >= self.lim() {
            return None;
        }
        let at = self.space_end(p + 1);
        if self.at(at) == b':' {
            let mut from = self.space_end(at + 1);
            if self.at(from) == b'?' {
                from = self.space_end(from + 1);
            }
            if self.space_at(from) == 0 {
                let in_class = |lexer: &Self, i: usize| {
                    let b = lexer.at(i);
                    lexer.space_at(i) > 0
                        || matches!(b, b'&' | b'(' | b')' | b'\\' | b'_' | b'|')
                        || b.is_ascii_alphanumeric()
                        || b >= 0x7f
                };
                let mut end = from;
                while self.avail(end) && in_class(self, end) {
                    end = if end == self.lim() {
                        end + 1
                    } else {
                        self.char_end(end)
                    };
                }
                self.spend(end - from);
                while end > from {
                    let last_space = self.space_at(self.char_start_before(end)) > 0
                        && end <= self.lim()
                        || end == self.lim() + 1;
                    if !last_space && self.body_follows(end) {
                        self.cap(1, p, p + 1);
                        self.cap(2, at, end);
                        return Some(end);
                    }
                    end = self.char_start_before(end);
                }
            }
        }
        if !self.body_follows(at) {
            return None;
        }
        self.cap(1, p, p + 1);
        Some(at)
    }

    /// `(?:(?:a|b|…)(?:\(set\))?|…)(?:\s+|(?=\?)))++`: possessive property
    /// modifiers, case-insensitive, and where they end, or `None` without one.
    fn property_modifiers(&self, p: usize, statics: bool) -> Option<usize> {
        let separated = |lexer: &Self, at: usize| {
            let next = lexer.space_end(at);
            if next > at {
                Some(next)
            } else {
                (lexer.at(at) == b'?').then_some(at)
            }
        };
        let mut at = p;
        loop {
            let visibility = [b"public".as_slice(), b"private", b"protected"]
                .iter()
                .find_map(|word| self.ilit(at, word));
            let next = if let Some(end) = visibility {
                self.ilit(end, b"(set)")
                    .and_then(|set| separated(self, set))
                    .or_else(|| separated(self, end))
            } else {
                let others: &[&[u8]] = if statics {
                    &[b"static", b"readonly"]
                } else {
                    &[b"readonly"]
                };
                others
                    .iter()
                    .find_map(|word| self.ilit(at, word))
                    .and_then(|end| separated(self, end))
            };
            match next {
                Some(next) if next > at => at = next,
                Some(_) | None => break,
            }
        }
        (at > p).then_some(at)
    }

    /// A promoted constructor parameter:
    ///
    /// ```text
    /// (?i)((?:(?:p(?:ublic|rivate|rotected)(?:\(set\))?|readonly)(?:\s+|(?=\?)))++)(?:(T)\s+)?((?:(&)\s*)?(\$)NAME)
    /// ```
    pub(super) fn promoted_parameter(&mut self, p: usize) -> Option<usize> {
        let at = self.property_modifiers(p, false)?;
        let typed = self.each_type(at, &mut |lexer, end| {
            let var_at = lexer.space_end(end);
            if var_at == end {
                return None;
            }
            lexer
                .parameter_variable(var_at)
                .map(|variable| (end, var_at, variable))
        });
        let (var_at, variable) = match typed {
            Some((ty_end, var_at, variable)) => {
                self.cap(2, at, ty_end);
                (var_at, variable)
            }
            None => (at, self.parameter_variable(at)?),
        };
        self.cap(1, p, at);
        self.record_variable(var_at, variable, 3);
        Some(variable.end)
    }

    /// A method head:
    ///
    /// ```text
    /// ((?:(?:final|abstract|public|private|protected|static)\s+)*)(function)\s+
    /// (?i:(__(?:call|construct|…|unserialize))|(&)?\s*([A-Z_a-z\x7F-\x{10FFFF}][0-9A-Z_a-z\x7F-\x{10FFFF}]*))\s*(\()
    /// ```
    pub(super) fn method_head(&mut self, p: usize) -> Option<usize> {
        let at = self.modifiers(
            p,
            &[
                b"final",
                b"abstract",
                b"public",
                b"private",
                b"protected",
                b"static",
            ],
        );
        let function_end = self.lit(at, b"function")?;
        let name_at = self.space_end(function_end);
        if name_at == function_end {
            return None;
        }
        let magic: [&[u8]; 18] = [
            b"__call",
            b"__construct",
            b"__debugInfo",
            b"__destruct",
            b"__get",
            b"__set",
            b"__isset",
            b"__unset",
            b"__toString",
            b"__clone",
            b"__set_state",
            b"__sleep",
            b"__wakeup",
            b"__autoload",
            b"__invoke",
            b"__callStatic",
            b"__serialize",
            b"__unserialize",
        ];
        let opens = |lexer: &Self, end: usize| {
            let open = lexer.space_end(end);
            (lexer.at(open) == b'(').then_some(open)
        };
        for word in magic {
            if let Some(end) = self.ilit(name_at, word)
                && let Some(open) = opens(self, end)
            {
                self.cap(1, p, at);
                self.cap(2, at, function_end);
                self.cap(3, name_at, end);
                self.cap(6, open, open + 1);
                return Some(open + 1);
            }
        }
        let amp = self.at(name_at) == b'&';
        let name_from = self.space_end(name_at + usize::from(amp));
        let name = self.name(name_from)?;
        let open = opens(self, name)?;
        self.cap(1, p, at);
        self.cap(2, at, function_end);
        if amp {
            self.cap(4, name_at, name_at + 1);
        }
        self.cap(5, name_from, name);
        self.cap(6, open, open + 1);
        Some(open + 1)
    }

    /// `(?i)(\))(?:\s*(:)\s*(T))?(?=\s*(?:\{|/[*/]|#|$|;))`.
    pub(super) fn method_close(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b')' || p >= self.lim() {
            return None;
        }
        let colon = self.space_end(p + 1);
        if self.at(colon) == b':' {
            let at = self.space_end(colon + 1);
            if let Some(end) =
                self.each_type(at, &mut |lexer, end| lexer.body_follows(end).then_some(end))
            {
                self.cap(1, p, p + 1);
                self.cap(2, colon, colon + 1);
                self.cap(3, at, end);
                return Some(end);
            }
        }
        if !self.body_follows(p + 1) {
            return None;
        }
        self.cap(1, p, p + 1);
        Some(p + 1)
    }

    /// A property:
    ///
    /// ```text
    /// (?i)((?:(?:p(?:ublic|rivate|rotected)(?:\(set\))?|static|readonly)(?:\s+|(?=\?)))++)(T)?\s+((\$)NAME)
    /// ```
    pub(super) fn property(&mut self, p: usize) -> Option<usize> {
        let at = self.property_modifiers(p, true)?;
        let variable = |lexer: &Self, from: usize| {
            let spaced = lexer.space_end(from);
            if spaced == from || lexer.at(spaced) != b'$' {
                return None;
            }
            lexer.name(spaced + 1).map(|end| (spaced, end))
        };
        let typed = self.each_type(at, &mut |lexer, end| {
            variable(lexer, end).map(|found| (end, found))
        });
        let (dollar, end) = match typed {
            Some((ty_end, found)) => {
                self.cap(2, at, ty_end);
                found
            }
            None => variable(self, at)?,
        };
        self.cap(1, p, at);
        self.cap(3, dollar, end);
        self.cap(4, dollar, dollar + 1);
        Some(end)
    }

    /// `(array)\s*(\()`.
    pub(super) fn array_open(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"array")?;
        let open = self.space_end(end);
        if self.at(open) != b'(' {
            return None;
        }
        self.cap(1, p, end);
        self.cap(2, open, open + 1);
        Some(open + 1)
    }

    /// `(?i)(::)\s*(NAME)?`.
    pub(super) fn static_member(&mut self, p: usize) -> Option<usize> {
        let colons = self.lit(p, b"::")?;
        let at = self.space_end(colons);
        self.cap(1, p, colons);
        match self.name(at) {
            Some(end) => {
                self.cap(2, at, end);
                Some(end)
            }
            None => Some(at),
        }
    }
}

/// What follows `new` in an anonymous class: `readonly`, if taken, and
/// `class`, as spans.
struct NewClass {
    /// The span of `readonly`.
    readonly: Option<(usize, usize)>,
    /// The span of `class`.
    class: (usize, usize),
}
