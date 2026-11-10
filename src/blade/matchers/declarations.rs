//! Blade's own declarations: namespaces, classes, interfaces, traits,
//! functions and their parameters.
//! Blade's copy of PHP declares them with older expressions than Shiki's
//! `php`, most anchored at the line's start.

use super::super::engine::Lexer;
use crate::rules::Chars;

/// The modifiers that may stand before `function`, and after `as` in a `use`
/// block.
const MODIFIERS: [&[u8]; 6] = [
    b"final",
    b"abstract",
    b"public",
    b"private",
    b"protected",
    b"static",
];

/// A parameter's variable, `(&)?\s*(\.\.\.)?(\$+)NAME`.
#[derive(Clone, Copy)]
struct Variable {
    /// Where `&` stands, if it does.
    amp: Option<usize>,
    /// Where `...` stands, if it does.
    dots: Option<usize>,
    /// Where the `$` signs start.
    dollars: usize,
    /// Where the `$` signs end and the name starts.
    name: usize,
    /// Where the name ends.
    end: usize,
}

impl Lexer<'_> {
    /// `(?i)(?=\\?[0-9_a-z]+\\)`: an ASCII name with a namespace after it.
    /// The Kelvin sign and `ſ` fold into the class.
    pub(super) fn ahead_ascii_namespaced(&self, p: usize) -> Option<usize> {
        let from = p + usize::from(self.at(p) == b'\\');
        let end = self.ascii_name_end(from);
        (end > from && self.at(end) == b'\\').then_some(p)
    }

    /// `(?i)([_a-z][0-9_a-z]*)?(?=[^0-9\\_a-z])`: the last segment of an ASCII
    /// qualified name, as group 1, and a character that ends it.
    pub(super) fn ascii_last_segment(&mut self, p: usize) -> Option<usize> {
        let ends = |lexer: &Self, at: usize| {
            lexer.avail(at) && (at == lexer.lim() || !lexer.ascii_qualified_at(at))
        };
        let first = !self.at(p).is_ascii_digit() && self.ascii_name_end(p) > p;
        if first {
            let end = self.ascii_name_end(p);
            if !ends(self, end) {
                return None;
            }
            self.capture(1, p, end);
            return Some(end);
        }
        ends(self, p).then_some(p)
    }

    /// `(?=[A-Z\\_a-z])`, without case folding.
    pub(super) fn ahead_class_start(&self, p: usize) -> Option<usize> {
        let b = self.at(p);
        (p < self.lim() && (b == b'\\' || b == b'_' || b.is_ascii_alphabetic())).then_some(p)
    }

    /// `(?i)(?:(namespace)|NAME)?(\\)(?=.*?[^0-9\\_a-z\x7F-ÿ])`.
    /// The lookahead holds where any character after the backslash is not a
    /// name character, the line feed included.
    pub(super) fn namespace_part(&mut self, p: usize) -> Option<usize> {
        let slash = if let Some(end) = self.ilit(p, b"namespace")
            && self.at(end) == b'\\'
        {
            self.capture(1, p, end);
            end
        } else {
            let at = self.bname(p).unwrap_or(p);
            if self.at(at) != b'\\' || at >= self.lim() {
                return None;
            }
            at
        };
        let after = slash + 1;
        let mut at = after;
        while !self.unqualified_at(at) {
            if at >= self.lim() {
                return None;
            }
            at = self.char_end(at);
        }
        self.capture(2, slash, after);
        Some(after)
    }

    /// `(?i)(?=[0-9_a-z\x7F-ÿ]*[^0-9\\_a-z\x7F-ÿ])`.
    pub(super) fn namespace_end(&self, p: usize) -> Option<usize> {
        let end = self.name_run_end(p);
        self.unqualified_at(end).then_some(p)
    }

    /// `(?i)^\s*(interface)\s+(NAME)\s*(extends)?\s*`.
    pub(super) fn interface(&mut self, p: usize) -> Option<usize> {
        let (word, at, name) = self.declared(p, b"interface")?;
        let spaces = self.space_end(name);
        let end = match self.ilit(spaces, b"extends") {
            Some(extends) => {
                self.capture(3, spaces, extends);
                self.space_end(extends)
            }
            None => spaces,
        };
        self.capture(1, word.0, word.1);
        self.capture(2, at, name);
        Some(end)
    }

    /// `(?i)((?:NAME\s*,\s*)*)(NAME)?\s*(?:(?=\{)|$)`: the interfaces that an
    /// interface extends, up to its body or the line's end.
    pub(super) fn extended_interfaces(&mut self, p: usize) -> Option<usize> {
        let mut at = p;
        while let Some(name) = self.bname(at) {
            let comma = self.space_end(name);
            if self.at(comma) != b',' || comma >= self.lim() {
                break;
            }
            at = self.space_end(comma + 1);
        }
        let last = self.bname(at);
        let end = self.space_end(last.unwrap_or(at));
        if !((self.at(end) == b'{' && end < self.lim()) || self.at_eol(end)) {
            return None;
        }
        self.capture(1, p, at);
        if let Some(last) = last {
            self.capture(2, at, last);
        }
        Some(end)
    }

    /// `(?i)^\s*(trait)\s+(NAME)`.
    pub(super) fn trait_declaration(&mut self, p: usize) -> Option<usize> {
        let (word, at, name) = self.declared(p, b"trait")?;
        self.capture(1, word.0, word.1);
        self.capture(2, at, name);
        Some(name)
    }

    /// `(?=\{)`.
    pub(super) fn ahead_brace(&self, p: usize) -> Option<usize> {
        (self.at(p) == b'{' && p < self.lim()).then_some(p)
    }

    /// `(?i)(?:^|(?<=<\?php))\s*(namespace)\s+([0-9\\_a-z\x7F-ÿ]+)(?=\s*;)`.
    pub(super) fn namespace_line(&mut self, p: usize) -> Option<usize> {
        let after_tag = p >= 5 && self.text(p - 5, p).eq_ignore_ascii_case(b"<?php");
        if p != 0 && !after_tag {
            return None;
        }
        let at = self.space_end(p);
        let word = self.ilit(at, b"namespace")?;
        let name = self.space_end(word);
        if name == word {
            return None;
        }
        let end = self.qualified_end(name);
        if end == name || self.at(self.space_end(end)) != b';' {
            return None;
        }
        self.capture(1, at, word);
        self.capture(2, name, end);
        Some(end)
    }

    /// `(?i)[0-9\\_a-z\x7F-ÿ]+`.
    pub(super) fn qualified_name(&self, p: usize) -> Option<usize> {
        let end = self.qualified_end(p);
        (end > p).then_some(end)
    }

    /// `(?<=})|(?=;)`.
    pub(super) fn after_brace_or_semicolon(&self, p: usize) -> Option<usize> {
        (self.prev_is(p, b'}') || (self.at(p) == b';' && p < self.lim())).then_some(p)
    }

    /// `(?i)^\s*(?:(abstract|final)\s+)?(class)\s+(NAME)`.
    pub(super) fn class_declaration(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let start = self.space_end(0);
        let modifier = [b"abstract".as_slice(), b"final"]
            .iter()
            .find_map(|word| self.ilit(start, word))
            .filter(|&end| self.space_end(end) > end);
        for (at, modifier) in [
            modifier.map(|end| (self.space_end(end), Some(end))),
            Some((start, None)),
        ]
        .into_iter()
        .flatten()
        {
            let Some(word) = self.ilit(at, b"class") else {
                continue;
            };
            let name_at = self.space_end(word);
            if name_at == word {
                continue;
            }
            let Some(name) = self.bname(name_at) else {
                continue;
            };
            if let Some(end) = modifier {
                self.capture(1, start, end);
            }
            self.capture(2, at, word);
            self.capture(3, name_at, name);
            return Some(name);
        }
        None
    }

    /// `(?i)(?=[^0-9\\_a-z\x7F-ÿ])`.
    pub(super) fn ahead_unqualified(&self, p: usize) -> Option<usize> {
        self.unqualified_at(p).then_some(p)
    }

    /// `(?i)(?=\\?NAME\\)`.
    pub(super) fn ahead_namespaced(&self, p: usize) -> Option<usize> {
        let from = p + usize::from(self.at(p) == b'\\');
        let end = self.bname(from)?;
        (self.at(end) == b'\\').then_some(p)
    }

    /// `(?i)(NAME)?(?=[^0-9\\_a-z\x7F-ÿ])`.
    pub(super) fn last_segment(&mut self, p: usize) -> Option<usize> {
        match self.bname(p) {
            Some(end) if self.unqualified_at(end) => {
                self.capture(1, p, end);
                Some(end)
            }
            Some(_) => None,
            None => self.unqualified_at(p).then_some(p),
        }
    }

    /// `(?i)(?=[;{])`.
    pub(super) fn ahead_body(&self, p: usize) -> Option<usize> {
        (matches!(self.at(p), b';' | b'{') && p < self.lim()).then_some(p)
    }

    /// `(?i)(?=[0-9\\_a-z\x7F-ÿ]+)`.
    pub(super) fn ahead_qualified(&self, p: usize) -> Option<usize> {
        self.qualified_at(p).then_some(p)
    }

    /// `(?i)\s*(?:,|(?=[^0-9\\_a-z\x7F-ÿ\s]))\s*`: the end of a name in an
    /// `implements` list.
    pub(super) fn implemented_end(&self, p: usize) -> Option<usize> {
        let at = self.space_end(p);
        if self.at(at) == b',' && at < self.lim() {
            return Some(self.space_end(at + 1));
        }
        (at < self.lim() && self.unqualified_at(at) && self.space_at(at) == 0).then_some(at)
    }

    /// `(?i)(NAME)((?:\s*\|\s*NAME)*)\s*((\$+)NAME)`: the types and the
    /// variable of a `catch`.
    pub(super) fn caught(&mut self, p: usize) -> Option<usize> {
        let first = self.bname(p)?;
        let mut types = first;
        loop {
            let bar = self.space_end(types);
            if self.at(bar) != b'|' || bar >= self.lim() {
                break;
            }
            let Some(next) = self.bname(self.space_end(bar + 1)) else {
                break;
            };
            types = next;
        }
        let dollars = self.space_end(types);
        let name = self.skip(dollars, |b| b == b'$');
        if name == dollars {
            return None;
        }
        let end = self.bname(name)?;
        self.capture(1, p, first);
        self.capture(2, first, types);
        self.capture(3, dollars, end);
        self.capture(4, dollars, name);
        Some(end)
    }

    /// `(?i)\b(function)\s*(?=\()`.
    pub(super) fn closure(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let word = self.ilit(p, b"function")?;
        let open = self.space_end(word);
        if self.at(open) != b'(' {
            return None;
        }
        self.capture(1, p, word);
        Some(open)
    }

    /// `(array)\s+(VARIABLE)\s*(=)\s*(array)\s*(\()`, under `(?i)`: a
    /// parameter whose default is an empty `array(`.
    pub(super) fn array_default_parameter(&mut self, p: usize) -> Option<usize> {
        let word = self.ilit(p, b"array")?;
        let at = self.space_end(word);
        if at == word {
            return None;
        }
        let variable = self.parameter_variable(at, false)?;
        let equals = self.space_end(variable.end);
        if self.at(equals) != b'=' {
            return None;
        }
        let array = self.space_end(equals + 1);
        let array_end = self.ilit(array, b"array")?;
        let open = self.space_end(array_end);
        if self.at(open) != b'(' {
            return None;
        }
        self.capture(1, p, word);
        self.record(variable, at, 2);
        self.capture(5, equals, equals + 1);
        self.capture(6, array, array_end);
        self.capture(7, open, open + 1);
        Some(open + 1)
    }

    /// `(?i)(array|callable)\s+(VARIABLE)(?:\s*(=)\s*(?:(null)|(\[)(…)(])|(\S*?
    /// \(\)|\S*?)))?\s*(?=[),]|/[*/]|#|$)`.
    ///
    /// The default tries these in order, each until what follows holds:
    /// - `null`;
    /// - a bracketed array;
    /// - the shortest run of non-spaces ending in `()`;
    /// - the shortest run of non-spaces.
    pub(super) fn array_parameter(&mut self, p: usize) -> Option<usize> {
        let word = self
            .ilit(p, b"array")
            .or_else(|| self.ilit(p, b"callable"))?;
        let at = self.space_end(word);
        if at == word {
            return None;
        }
        let variable = self.parameter_variable(at, false)?;
        let default = self.array_parameter_default(variable.end);
        let end = match default {
            Some((_, end)) => end,
            None => self.parameter_tail(variable.end)?,
        };
        self.capture(1, p, word);
        self.record(variable, at, 2);
        if let Some((default, _)) = default {
            self.capture(5, default.equals, default.equals + 1);
            match default.value {
                DefaultValue::Null(to) => self.capture(6, default.from, to),
                DefaultValue::Brackets(close) => {
                    self.capture(7, default.from, default.from + 1);
                    self.capture(8, default.from + 1, close);
                    self.capture(9, close, close + 1);
                }
                DefaultValue::Text(to) => self.capture(10, default.from, to),
            }
        }
        Some(end)
    }

    /// The default of [`Lexer::array_parameter`] after the variable at `p`,
    /// with where the whole match ends.
    fn array_parameter_default(&self, p: usize) -> Option<(Default, usize)> {
        let equals = self.space_end(p);
        if self.at(equals) != b'=' || equals >= self.lim() {
            return None;
        }
        let from = self.space_end(equals + 1);
        let default = |value| Default {
            equals,
            from,
            value,
        };
        if let Some(to) = self.ilit(from, b"null")
            && let Some(end) = self.parameter_tail(to)
        {
            return Some((default(DefaultValue::Null(to)), end));
        }
        if self.at(from) == b'['
            && let Some(close) = self.balanced(from + 1, true)
            && let Some(end) = self.parameter_tail(close + 1)
        {
            return Some((default(DefaultValue::Brackets(close)), end));
        }
        let mut to = from;
        while to < self.lim() && self.space_at(to) == 0 {
            if self.starts(to, b"()")
                && let Some(end) = self.parameter_tail(to + 2)
            {
                return Some((default(DefaultValue::Text(to + 2)), end));
            }
            to = self.char_end(to);
        }
        let mut to = from;
        loop {
            if let Some(end) = self.parameter_tail(to) {
                return Some((default(DefaultValue::Text(to)), end));
            }
            if to >= self.lim() || self.space_at(to) > 0 {
                return None;
            }
            to = self.char_end(to);
        }
    }

    /// `(?>[^]\[]+|\[\g<N>])*` from `p` up to its `]`: the inside of brackets.
    /// When `nests`, an inner pair nests, as `\g<N>` calls the inside again.
    /// Otherwise an inner pair is `[[]` only, as `\g<N>` calls the `(\[)`
    /// group.
    ///
    /// - It counts the open pairs instead of recursing, so a long run of `[`
    ///   cannot overflow the stack. Only brackets are special inside, so a
    ///   count is the whole state.
    /// - It spends a step for each byte it reads, as [`Lexer::run_end`] does.
    fn balanced(&self, mut p: usize, nests: bool) -> Option<usize> {
        let lim = self.lim();
        let from = p;
        let mut open = 0_usize;
        let close = loop {
            match self.at(p) {
                _ if p >= lim => break None,
                b']' if open == 0 => break Some(p),
                b']' => {
                    open -= 1;
                    p += 1;
                }
                b'[' if nests => {
                    open += 1;
                    p += 1;
                }
                b'[' if self.starts(p, b"[[]") => p += 3,
                b'[' => break None,
                _ => p += 1,
            }
        };
        self.spend(p - from);
        close
    }

    /// `\s*(?=[),]|/[*/]|#|$)` at `p`, and where its spaces end.
    fn parameter_tail(&self, p: usize) -> Option<usize> {
        let at = self.space_end(p);
        let follows = (matches!(self.at(at), b')' | b',' | b'#') && at < self.lim())
            || self.starts(at, b"/*")
            || self.starts(at, b"//")
            || self.at_eol(at);
        follows.then_some(at)
    }

    /// `(?=[),]|/[*/]|#)`.
    pub(super) fn parameter_ends(&self, p: usize) -> Option<usize> {
        let follows = (matches!(self.at(p), b')' | b',' | b'#') && p < self.lim())
            || self.starts(p, b"/*")
            || self.starts(p, b"//");
        follows.then_some(p)
    }

    /// `(&)?\s*(\.\.\.)?(\$+)NAME` at `p`, or without the dots when not
    /// `dots`.
    fn parameter_variable(&self, p: usize, dots: bool) -> Option<Variable> {
        let amp = (self.at(p) == b'&').then_some(p);
        let at = self.space_end(p + usize::from(amp.is_some()));
        let dots = (dots && self.starts(at, b"...")).then_some(at);
        let dollars = at + dots.map_or(0, |_| 3);
        let name = self.skip(dollars, |b| b == b'$');
        if name == dollars {
            return None;
        }
        let end = self.bname(name)?;
        Some(Variable {
            amp,
            dots,
            dollars,
            name,
            end,
        })
    }

    /// Records a parameter's variable from `from` for a pattern without a
    /// `...` group: the whole as group `whole`, then its `&` and its `$`
    /// signs.
    fn record(&mut self, variable: Variable, from: usize, whole: usize) {
        self.capture(whole, from, variable.end);
        if let Some(amp) = variable.amp {
            self.capture(whole + 1, amp, amp + 1);
        }
        self.capture(whole + 2, variable.dollars, variable.name);
    }

    /// `(?i)(\\?(?:NAME\\)*)(NAME)\s+((&)?\s*(\.\.\.)?(\$+)NAME)`.
    pub(super) fn typed_parameter(&mut self, p: usize) -> Option<usize> {
        let mut at = p + usize::from(self.at(p) == b'\\');
        while let Some(end) = self.bname(at)
            && self.at(end) == b'\\'
        {
            at = end + 1;
        }
        let ty = self.bname(at)?;
        let from = self.space_end(ty);
        if from == ty {
            return None;
        }
        let variable = self.parameter_variable(from, true)?;
        self.capture(1, p, at);
        self.capture(2, at, ty);
        self.record_dotted(variable, from, 3);
        Some(variable.end)
    }

    /// `(?i)((&)?\s*(\.\.\.)?(\$+)NAME)\s*(?=[),]|/[*/]|#|$)`.
    pub(super) fn untyped_parameter(&mut self, p: usize) -> Option<usize> {
        let variable = self.parameter_variable(p, true)?;
        let end = self.parameter_tail(variable.end)?;
        self.record_dotted(variable, p, 1);
        Some(end)
    }

    /// `(?i)((&)?\s*(\.\.\.)?(\$+)NAME)\s*(=)\s*(?:(\[)((?>[^]\[]+|\[\g<6>])*
    /// )(]))?`. `\g<6>` calls the `(\[)` group, so an inner pair is `[[]` only.
    pub(super) fn defaulted_parameter(&mut self, p: usize) -> Option<usize> {
        let variable = self.parameter_variable(p, true)?;
        let equals = self.space_end(variable.end);
        if self.at(equals) != b'=' || equals >= self.lim() {
            return None;
        }
        let after = self.space_end(equals + 1);
        self.record_dotted(variable, p, 1);
        self.capture(5, equals, equals + 1);
        if self.at(after) == b'['
            && after < self.lim()
            && let Some(close) = self.balanced(after + 1, false)
        {
            self.capture(6, after, after + 1);
            self.capture(7, after + 1, close);
            self.capture(8, close, close + 1);
            return Some(close + 1);
        }
        Some(after)
    }

    /// `(?i)((&)?\s*(\$+)NAME)\s*(?=[),])`.
    pub(super) fn closure_parameter(&mut self, p: usize) -> Option<usize> {
        let variable = self.parameter_variable(p, false)?;
        let end = self.space_end(variable.end);
        if !(matches!(self.at(end), b')' | b',') && end < self.lim()) {
            return None;
        }
        self.record(variable, p, 1);
        Some(end)
    }

    /// Records a parameter's variable from `from` for a pattern with a `...`
    /// group: the whole as group `whole`, then its `&`, its `...` and its `$`
    /// signs.
    fn record_dotted(&mut self, variable: Variable, from: usize, whole: usize) {
        self.capture(whole, from, variable.end);
        if let Some(amp) = variable.amp {
            self.capture(whole + 1, amp, amp + 1);
        }
        if let Some(dots) = variable.dots {
            self.capture(whole + 2, dots, dots + 3);
        }
        self.capture(whole + 3, variable.dollars, variable.name);
    }

    /// `((?:(?:MODIFIER)\s+)*)(function)\s+(?i:(__(?:call|…
    /// |callStatic))|(NAME))\s*(\()`. The modifiers and `function` keep their
    /// case. A magic name gives way to the next one, and then to a plain name,
    /// where `(` does not follow it: `__callStatic` past `__call`.
    pub(super) fn method(&mut self, p: usize) -> Option<usize> {
        let mut at = p;
        while let Some(end) = MODIFIERS.iter().find_map(|word| self.lit(at, word)) {
            let next = self.space_end(end);
            if next == end {
                break;
            }
            at = next;
        }
        let function = self.lit(at, b"function")?;
        let name_at = self.space_end(function);
        if name_at == function {
            return None;
        }
        let magic: [&[u8]; 16] = [
            b"__call",
            b"__construct",
            b"__debuginfo",
            b"__destruct",
            b"__get",
            b"__set",
            b"__isset",
            b"__unset",
            b"__tostring",
            b"__clone",
            b"__set_state",
            b"__sleep",
            b"__wakeup",
            b"__autoload",
            b"__invoke",
            b"__callstatic",
        ];
        let open_after = |lexer: &Self, end: usize| {
            let open = lexer.space_end(end);
            (lexer.at(open) == b'(').then_some(open)
        };
        let named = magic
            .iter()
            .find_map(|word| {
                let end = self.ilit(name_at, word)?;
                open_after(self, end).map(|open| (3, end, open))
            })
            .or_else(|| {
                let end = self.bname(name_at)?;
                open_after(self, end).map(|open| (4, end, open))
            });
        let (group, name, open) = named?;
        self.capture(1, p, at);
        self.capture(2, at, function);
        self.capture(group, name_at, name);
        self.capture(5, open, open + 1);
        Some(open + 1)
    }

    /// `(\))(?:\s*(:)\s*([A-Z_a-z\x7F-ÿ][0-9A-Z_a-z\x7F-ÿ]*))?`: the end of a
    /// method's parameters, and its return type.
    pub(super) fn parameters_close(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b')' || p >= self.lim() {
            return None;
        }
        self.capture(1, p, p + 1);
        let colon = self.space_end(p + 1);
        if self.at(colon) == b':' && colon < self.lim() {
            let ty = self.space_end(colon + 1);
            if let Some(end) = self.bname_exact(ty) {
                self.capture(2, colon, colon + 1);
                self.capture(3, ty, end);
                return Some(end);
            }
        }
        Some(p + 1)
    }

    /// `(?i)\b(as)\s+(MODIFIER)\s+(NAME)\b`.
    pub(super) fn use_as_modifier(&mut self, p: usize) -> Option<usize> {
        let (word, at) = self.as_word(p, b"as")?;
        let modifier = MODIFIERS.iter().find_map(|word| self.ilit(at, word))?;
        let name_at = self.space_end(modifier);
        if name_at == modifier {
            return None;
        }
        let end = self.bname_bounded(name_at)?;
        self.capture(1, p, word);
        self.capture(2, at, modifier);
        self.capture(3, name_at, end);
        Some(end)
    }

    /// `(?i)\b(as)\s+(NAME)\b`.
    pub(super) fn use_as_name(&mut self, p: usize) -> Option<usize> {
        let (word, at) = self.as_word(p, b"as")?;
        let end = self.bname_bounded(at)?;
        self.capture(1, p, word);
        self.capture(2, at, end);
        Some(end)
    }

    /// `(?i)\b(insteadof)\s+(NAME)`.
    pub(super) fn insteadof(&mut self, p: usize) -> Option<usize> {
        let (word, at) = self.as_word(p, b"insteadof")?;
        let end = self.bname(at)?;
        self.capture(1, p, word);
        self.capture(2, at, end);
        Some(end)
    }

    /// `(?i)\b(word)\s+` at `p`: where the word ends and its spaces end.
    fn as_word(&self, p: usize, word: &[u8]) -> Option<(usize, usize)> {
        if !self.boundary(p) {
            return None;
        }
        let end = self.ilit(p, word)?;
        let at = self.space_end(end);
        (at > end).then_some((end, at))
    }

    /// `^(?:final|abstract|public|private|protected|static)$`, on a capture's
    /// text.
    pub(super) fn modifier_alone(&self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        MODIFIERS
            .iter()
            .find_map(|word| self.lit(p, word).filter(|&end| self.at_eol(end)))
    }

    /// `(?i)^\s*(word)\s+(NAME)`: a declaration at the line's start, with the
    /// word's span, where the name starts and where it ends.
    fn declared(&self, p: usize, word: &[u8]) -> Option<((usize, usize), usize, usize)> {
        if p != 0 {
            return None;
        }
        let start = self.space_end(0);
        let end = self.ilit(start, word)?;
        let at = self.space_end(end);
        if at == end {
            return None;
        }
        let name = self.bname(at)?;
        Some(((start, end), at, name))
    }
}

/// The default of an array parameter.
#[derive(Clone, Copy)]
struct Default {
    /// Where `=` stands.
    equals: usize,
    /// Where the value starts.
    from: usize,
    /// The value.
    value: DefaultValue,
}

/// The value of an array parameter's default.
#[derive(Clone, Copy)]
enum DefaultValue {
    /// `null`, ending at the value.
    Null(usize),
    /// `[…]`, with where its `]` stands.
    Brackets(usize),
    /// A run of non-spaces, ending at the value.
    Text(usize),
}
