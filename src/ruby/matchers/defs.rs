//! Classes, modules, method definitions, their parameters, and calls.

use super::super::engine::Lexer;
use crate::rules::{CachedRuns, Chars};

/// What follows a method definition's name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum After {
    /// `\s*(\()`: parentheses around the parameters.
    Paren,
    /// `[\t ](?=[\t ]*[^#;\s])`: parameters without parentheses.
    Bare,
}

impl Lexer<'_> {
    /// The end of `([0-9A-Z_a-z]+)((::)[0-9A-Z_a-z]+)*` at `p`, and where its
    /// last `::` stands.
    fn class_path(&self, p: usize) -> Option<(usize, Option<usize>)> {
        let segment = |at: usize| self.skip(at, |b| b.is_ascii_alphanumeric() || b == b'_');
        let mut end = segment(p);
        if end == p {
            return None;
        }
        let mut last = None;
        while self.starts(end, b"::") {
            let next = segment(end + 2);
            if next == end + 2 {
                break;
            }
            last = Some(end);
            end = next;
        }
        Some((end, last))
    }

    /// Records a class path as `group`, and its last `::` as `separator`.
    fn record_path(
        &mut self,
        group: usize,
        separator: usize,
        start: usize,
        (end, last): (usize, Option<usize>),
    ) {
        self.cap(group, start, end);
        if let Some(at) = last {
            self.cap(separator, at, at + 2);
        }
    }

    /// `\b(word)\s+(path)` at `p`: the head of a class or module.
    fn definition_head(
        &mut self,
        p: usize,
        word: &[u8],
    ) -> Option<(usize, (usize, Option<usize>))> {
        let end = self.word_then(p, word)?;
        let name = self.space_end(end);
        if name == end {
            return None;
        }
        let path = self.class_path(name)?;
        self.cap(1, p, end);
        self.record_path(2, 5, name, path);
        Some((name, path))
    }

    /// `\b(class)\s+(path)\s*((<)\s*(path))?`
    pub(in crate::ruby) fn class_definition(&mut self, p: usize) -> Option<usize> {
        let (_, (end, _)) = self.definition_head(p, b"class")?;
        let blank = self.space_end(end);
        if self.byte_in(blank, |b| b == b'<') {
            let parent = self.space_end(blank + 1);
            if let Some(path) = self.class_path(parent) {
                self.cap(7, blank, blank + 1);
                self.record_path(8, 11, parent, path);
                return Some(path.0);
            }
        }
        Some(blank)
    }

    /// `\b(module)\s+(path)`
    pub(in crate::ruby) fn module_definition(&mut self, p: usize) -> Option<usize> {
        let (_, (end, _)) = self.definition_head(p, b"module")?;
        Some(end)
    }

    /// `\b(class)\s*(<<)\s*`
    pub(in crate::ruby) fn singleton_class(&mut self, p: usize) -> Option<usize> {
        let word = self.word_then(p, b"class")?;
        let open = self.space_end(word);
        let end = self.lit(open, b"<<")?;
        self.cap(1, p, word);
        self.cap(2, open, end);
        Some(self.space_end(end))
    }

    /// `(?=def\b)(?<=^|\s)(def)` at `p`: where `def` ends.
    fn def_word(&self, p: usize) -> Option<usize> {
        if !self.after_blank_or_start(p) {
            return None;
        }
        self.keyword(p, b"def")
    }

    /// The end of a method's name at `p`, the first of the atomic group's
    /// alternatives that matches:
    ///
    /// ```text
    /// [A-Z_a-z]\w*(?>[!?]|=(?!>))?|===?|!=|>[=>]?|<=>|<[<=]?|[%\&/`|]|\*\*?|=?~|[-+]@?|\[]=?
    /// ```
    fn method_name_end(&self, p: usize) -> Option<usize> {
        if let Some(end) = self.name_end(p) {
            let suffix = if self.byte_in(end, |b| b == b'!' || b == b'?') {
                1
            } else {
                usize::from(
                    self.byte_in(end, |b| b == b'=') && !self.byte_in(end + 1, |b| b == b'>'),
                )
            };
            return Some(end + suffix);
        }
        let optional =
            |at: usize, class: &dyn Fn(u8) -> bool| at + usize::from(self.byte_in(at, class));
        if self.starts(p, b"==") {
            return Some(optional(p + 2, &|b| b == b'='));
        }
        if self.starts(p, b"!=") {
            return Some(p + 2);
        }
        if self.starts(p, b"<=>") {
            return Some(p + 3);
        }
        match self.byte_in(p, |_| true).then(|| self.at(p)) {
            Some(b'>') => Some(optional(p + 1, &|b| b == b'=' || b == b'>')),
            Some(b'<') => Some(optional(p + 1, &|b| b == b'<' || b == b'=')),
            Some(b'%' | b'&' | b'/' | b'`' | b'|' | b'~') => Some(p + 1),
            Some(b'*') => Some(optional(p + 1, &|b| b == b'*')),
            Some(b'=') => self.byte_in(p + 1, |b| b == b'~').then_some(p + 2),
            Some(b'-' | b'+') => Some(optional(p + 1, &|b| b == b'@')),
            Some(b'[') => self
                .byte_in(p + 1, |b| b == b']')
                .then(|| optional(p + 2, &|b| b == b'=')),
            _ => None,
        }
    }

    /// Tries `rest` after a method's full name at `p`:
    /// `(?>[A-Z_a-z]\w*(?>\.|::))?` then the name.
    /// The receiver is atomic but optional.
    /// The name alone is therefore tried when `rest` fails after both.
    fn after_method_name<T>(
        &self,
        p: usize,
        rest: impl Fn(&Self, usize) -> Option<T>,
    ) -> Option<T> {
        let receiver = self.name_end(p).and_then(|end| {
            if self.byte_in(end, |b| b == b'.') {
                Some(end + 1)
            } else {
                self.lit(end, b"::")
            }
        });
        if let Some(found) = receiver
            .and_then(|start| self.method_name_end(start))
            .and_then(|end| rest(self, end))
        {
            return Some(found);
        }
        self.method_name_end(p).and_then(|end| rest(self, end))
    }

    /// `(?=def\b)(?<=^|\s)(def)\s+(name)` and then `after`.
    fn method_definition(&mut self, p: usize, after: After) -> Option<usize> {
        let word = self.def_word(p)?;
        let name = self.space_end(word);
        if name == word {
            return None;
        }
        let (name_end, end) = self.after_method_name(name, |lx, end| match after {
            After::Paren => {
                let open = lx.space_end(end);
                lx.byte_in(open, |b| b == b'(').then_some((end, open + 1))
            }
            After::Bare => {
                let next = end + 1;
                let first = lx.skip(next, |b| b == b'\t' || b == b' ');
                let params = lx.byte_in(end, |b| b == b'\t' || b == b' ')
                    && lx.avail(first)
                    && !lx.byte_in(first, |b| b == b'#' || b == b';')
                    && !lx.space_char(first);
                params.then_some((end, next))
            }
        })?;
        self.cap(1, p, word);
        self.cap(2, name, name_end);
        if after == After::Paren {
            self.cap(3, end - 1, end);
        }
        Some(end)
    }

    /// `(?=def\b)(?<=^|\s)(def)\s+(name)\s*(\()`
    pub(in crate::ruby) fn def_with_parens(&mut self, p: usize) -> Option<usize> {
        self.method_definition(p, After::Paren)
    }

    /// `(?=def\b)(?<=^|\s)(def)\s+(name)[\t ](?=[\t ]*[^#;\s])`
    pub(in crate::ruby) fn def_with_bare_params(&mut self, p: usize) -> Option<usize> {
        self.method_definition(p, After::Bare)
    }

    /// `(?=def\b)(?<=^|\s)(def)\b(\s+(name))?`
    pub(in crate::ruby) fn def_without_params(&mut self, p: usize) -> Option<usize> {
        let word = self.def_word(p)?;
        self.cap(1, p, word);
        let name = self.space_end(word);
        if name > word
            && let Some(end) = self.after_method_name(name, |_, end| Some(end))
        {
            self.cap(3, name, end);
            return Some(end);
        }
        Some(word)
    }

    /// Where bare parameters end:
    ///
    /// ```text
    /// (?=;)|(?<=[]!"')?`}\w])(?=\s*#|\s*$)
    /// ```
    pub(in crate::ruby) fn bare_params_end(&mut self, p: usize) -> Option<usize> {
        if self.byte_in(p, |b| b == b';') {
            return Some(p);
        }
        let closes = self.word_before(p)
            || self.prev(p).is_some_and(|b| {
                matches!(b, b']' | b'!' | b'"' | b'\'' | b')' | b'?' | b'`' | b'}')
            });
        (closes && self.rest_is_comment_or_blank(p)).then_some(p)
    }

    /// `(?=\s*#|\s*$)` at `p`.
    fn rest_is_comment_or_blank(&self, p: usize) -> bool {
        let next = self.space_end(p);
        self.byte_in(next, |b| b == b'#') || self.at_eol(next)
    }

    /// `(?=[,;]|\s*#|\s*$)`: where one bare parameter ends.
    pub(in crate::ruby) fn bare_param_end(&mut self, p: usize) -> Option<usize> {
        (self.byte_in(p, |b| b == b',' || b == b';') || self.rest_is_comment_or_blank(p))
            .then_some(p)
    }

    /// `(?=[\&*A-Z_a-z])`: where a parameter starts.
    pub(in crate::ruby) fn param_start(&mut self, p: usize) -> Option<usize> {
        (self.name_start(p) || self.byte_in(p, |b| b == b'&' || b == b'*')).then_some(p)
    }

    /// `(?=[),])`: where a parameter in parentheses ends.
    pub(in crate::ruby) fn param_end(&mut self, p: usize) -> Option<usize> {
        self.byte_in(p, |b| b == b')' || b == b',').then_some(p)
    }

    /// `\G(&|\*\*?)?(?:([A-Z_a-z]\w*[!?]?(:))|([A-Z_a-z]\w*))`: a parameter's
    /// name, as a keyword with its `:` or as a plain name.
    pub(in crate::ruby) fn param_name(&mut self, p: usize) -> Option<usize> {
        if !self.at_g(p) {
            return None;
        }
        let name = self.splat_end(p);
        let end = self.name_end(name)?;
        if name > p {
            self.cap(1, p, name);
        }
        let mark = end + usize::from(self.byte_in(end, |b| b == b'!' || b == b'?'));
        if self.byte_in(mark, |b| b == b':') {
            self.cap(2, name, mark + 1);
            self.cap(3, mark, mark + 1);
            return Some(mark + 1);
        }
        self.cap(4, name, end);
        Some(end)
    }

    /// The end of `(?:&|\*\*?)?` at `p`.
    fn splat_end(&self, p: usize) -> usize {
        if self.byte_in(p, |b| b == b'&') {
            p + 1
        } else if self.byte_in(p, |b| b == b'*') {
            p + 1 + usize::from(self.byte_in(p + 1, |b| b == b'*'))
        } else {
            p
        }
    }

    /// `(->)\(`
    pub(in crate::ruby) fn lambda_parens(&mut self, p: usize) -> Option<usize> {
        let arrow = self.lit(p, b"->")?;
        let end = self.lit(arrow, b"(")?;
        self.cap(1, p, arrow);
        Some(end)
    }

    /// `(?<=\{|\{\s+|[^$0-:@-Z_a-z]do|^do|[^$0-:@-Z_a-z]do\s+|^do\s+)(\|)`:
    /// the bar that opens a block's parameters.
    pub(in crate::ruby) fn block_params_open(&mut self, p: usize) -> Option<usize> {
        if !self.byte_in(p, |b| b == b'|') {
            return None;
        }
        let mut before = p;
        while before > 0
            && self
                .char_before(before)
                .is_some_and(crate::classes::is_space)
        {
            before = self.char_start_before(before);
        }
        let brace = self.prev_is(before, b'{');
        let word = before >= 2
            && self.text(before - 2, before) == b"do"
            && (before == 2
                || !self.prev(before - 2).is_some_and(
                    |b| matches!(b, b'$' | b'0'..=b':' | b'@'..=b'Z' | b'_' | b'a'..=b'z'),
                ));
        (brace || word).then(|| {
            self.cap(1, p, p + 1);
            p + 1
        })
    }

    /// `(?<!\|)(\|)(?!\|)`
    pub(in crate::ruby) fn block_params_close(&mut self, p: usize) -> Option<usize> {
        (self.byte_in(p, |b| b == b'|')
            && !self.prev_is(p, b'|')
            && !self.byte_in(p + 1, |b| b == b'|'))
        .then(|| {
            self.cap(1, p, p + 1);
            p + 1
        })
    }

    /// `(?![(,|\s])`: where a block parameter starts.
    pub(in crate::ruby) fn block_param_start(&mut self, p: usize) -> Option<usize> {
        let refused = self.byte_in(p, |b| matches!(b, b'(' | b',' | b'|'))
            || (self.avail(p) && self.space_char(p));
        (!refused).then_some(p)
    }

    /// `(?=,|\|\s*)`: where a block parameter ends.
    pub(in crate::ruby) fn block_param_end(&mut self, p: usize) -> Option<usize> {
        self.byte_in(p, |b| b == b',' || b == b'|').then_some(p)
    }

    /// `\G((?:&|\*\*?)?)([A-Z_a-z][_\w]*)`
    pub(in crate::ruby) fn block_param_name(&mut self, p: usize) -> Option<usize> {
        if !self.at_g(p) {
            return None;
        }
        let name = self.splat_end(p);
        let end = self.name_end(name)?;
        self.cap(1, p, name);
        self.cap(2, name, end);
        Some(end)
    }

    /// Whether `[!0-9?A-Z_a-z]` holds for the byte `b`.
    fn method_char(b: u8) -> bool {
        b.is_ascii_alphanumeric() || matches!(b, b'!' | b'?' | b'_')
    }

    /// Whether a character outside `[!0-9?A-Z_a-z]` stands at `p`.
    fn ends_method_name(&self, p: usize) -> bool {
        self.avail(p) && !(p < self.lim() && Self::method_char(self.at(p)))
    }

    /// The end of `[A-Za-z][!0-9?A-Z_a-z]*` at `p`, when a character outside
    /// the class follows it.
    fn called_name_end(&self, p: usize) -> Option<usize> {
        if !self.byte_in(p, |b| b.is_ascii_alphabetic()) {
            return None;
        }
        let end = self.skip(p + 1, Self::method_char);
        self.ends_method_name(end).then_some(end)
    }

    /// `(?<=[^.]\.|::)(?=[A-Za-z][!0-9?A-Z_a-z]*[^!0-9?A-Z_a-z])`: a method
    /// called after a `.` or `::`.
    pub(in crate::ruby) fn method_call_start(&mut self, p: usize) -> Option<usize> {
        let after_dot = p >= 2 && self.prev_is(p, b'.') && !self.prev_is(p - 1, b'.');
        let after_colons = p >= 2 && self.text(p - 2, p) == b"::";
        ((after_dot || after_colons) && self.called_name_end(p).is_some()).then_some(p)
    }

    /// `(?<=[!0-9?A-Z_a-z])(?=[^!0-9?A-Z_a-z])`
    pub(in crate::ruby) fn method_call_end(&mut self, p: usize) -> Option<usize> {
        (self.prev(p).is_some_and(Self::method_char) && self.ends_method_name(p)).then_some(p)
    }

    /// `([A-Za-z][!0-9?A-Z_a-z]*)(?=[^!0-9?A-Z_a-z])`
    pub(in crate::ruby) fn method_call_name(&mut self, p: usize) -> Option<usize> {
        let end = self.called_name_end(p)?;
        self.cap(1, p, end);
        Some(end)
    }

    /// `([A-Za-z]\w*[!?]?)(\()`: a call with parentheses.
    pub(in crate::ruby) fn call_with_parens(&mut self, p: usize) -> Option<usize> {
        if !self.byte_in(p, |b| b.is_ascii_alphabetic()) {
            return None;
        }
        let end = self.word_run(p + 1);
        let name = end + usize::from(self.byte_in(end, |b| b == b'!' || b == b'?'));
        self.byte_in(name, |b| b == b'(').then(|| {
            self.cap(1, p, name);
            self.cap(2, name, name + 1);
            name + 1
        })
    }
}
