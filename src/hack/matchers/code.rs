//! Declarations, statements, class names, namespaces, variables, calls,
//! operators and numbers.

use super::super::engine::Lexer;
use super::{is_iletter, is_iname, is_iname_start, is_ipath, is_iword, is_name, is_name_start};
use crate::rules::Chars;

/// Which declaration of a module a line opens.
#[derive(Clone, Copy, Debug)]
pub(super) enum Module {
    /// `new module name`: the module's definition.
    New,
    /// `module name`: a file's membership in the module.
    Plain,
}

/// `public|internal`: the visibility before a declaration.
const VISIBILITY: [&[u8]; 2] = [b"public", b"internal"];

/// The modifiers before a function's name.
const FUNCTION_MODIFIERS: [&[u8]; 8] = [
    b"final",
    b"abstract",
    b"public",
    b"private",
    b"protected",
    b"internal",
    b"static",
    b"async",
];

/// The magic methods, in the grammar's order.
const MAGIC: [&[u8]; 17] = [
    b"call",
    b"construct",
    b"destruct",
    b"get",
    b"set",
    b"isset",
    b"unset",
    b"tostring",
    b"clone",
    b"set_state",
    b"sleep",
    b"wakeup",
    b"autoload",
    b"invoke",
    b"callStatic",
    b"dispose",
    b"disposeAsync",
];

/// The types of a cast, longer spellings first where one starts another.
const CASTS: [&[u8]; 17] = [
    b"array",
    b"real",
    b"double",
    b"float",
    b"integer",
    b"int",
    b"boolean",
    b"bool",
    b"string",
    b"object",
    b"binary",
    b"unset",
    b"arraykey",
    b"nonnull",
    b"dict",
    b"vec",
    b"keyset",
];

impl Lexer<'_> {
    /// `(?i)word\b`, with no `\b` before it.
    fn iword_end(&self, p: usize, word: &[u8]) -> Option<usize> {
        let end = self.ilit(p, word)?;
        self.word_at(end).is_none().then_some(end)
    }

    /// `(?i)(?:(public|internal)\s+)?` at `p`, then `then` from where it
    /// leaves off: with the visibility first, and without it when that fails.
    fn after_visibility(
        &mut self,
        p: usize,
        then: impl Fn(&mut Self, usize) -> Option<usize>,
    ) -> Option<usize> {
        for word in VISIBILITY {
            if let Some(end) = self.ilit(p, word) {
                let next = self.space_end(end);
                if next > end
                    && let Some(found) = then(self, next)
                {
                    self.cap(1, p, end);
                    return Some(found);
                }
            }
        }
        then(self, p)
    }

    /// `(?i)\s+([0-9_a-z]+)` after a keyword that ends at `p`, with the name as
    /// group `group`: the end of the name.
    fn spaces_name(&mut self, p: usize, group: usize) -> Option<usize> {
        let at = self.space_end(p);
        if at == p {
            return None;
        }
        let end = self.chars_while(at, is_iword);
        if end == at {
            return None;
        }
        self.cap(group, at, end);
        Some(end)
    }

    /// `^(?i)\s*(?:(public|internal)\s+)?(interface)\b`.
    pub(super) fn interface(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let at = self.space_end(0);
        self.after_visibility(at, |lexer, q| {
            let end = lexer.iword_end(q, b"interface")?;
            lexer.cap(2, q, end);
            Some(end)
        })
    }

    /// `([-+])?([A-Z_a-z][0-9A-Z_a-z]*)(?:\s+(as|super)\
    /// s+([A-Z_a-z][0-9A-Z_a-z]*))?`.
    pub(super) fn generic_parameter(&self, p: usize) -> Option<usize> {
        let at = p + usize::from(matches!(self.at(p), b'-' | b'+'));
        let name = self.ident(at)?;
        let space = self.space_end(name);
        if space > name {
            for word in [&b"as"[..], b"super"] {
                if let Some(end) = self.lit(space, word) {
                    let bound = self.space_end(end);
                    if bound > end
                        && let Some(bound_end) = self.ident(bound)
                    {
                        return Some(bound_end);
                    }
                }
            }
        }
        Some(name)
    }

    /// `([A-Z_a-z][0-9A-Z_a-z]*)<`.
    pub(super) fn generic_type(&mut self, p: usize) -> Option<usize> {
        let name = self.ident(p)?;
        if self.at(name) != b'<' {
            return None;
        }
        self.cap(1, p, name);
        Some(name + 1)
    }

    /// `((,|\.\.\.)?\s*\))`.
    pub(super) fn shape_close(&mut self, p: usize) -> Option<usize> {
        let mark = if self.at(p) == b',' {
            p + 1
        } else {
            self.lit(p, b"...").unwrap_or(p)
        };
        let close = self.space_end(mark);
        if self.at(close) != b')' || close >= self.lim() {
            return None;
        }
        self.cap(1, p, close + 1);
        Some(close + 1)
    }

    /// `(?i)(?=\\?[0-9_a-z]+\\)`: a qualified name ahead.
    pub(super) fn ahead_qualified(&self, p: usize) -> Option<usize> {
        let at = p + usize::from(self.at(p) == b'\\');
        let end = self.chars_while(at, is_iword);
        (end > at && self.at(end) == b'\\').then_some(p)
    }

    /// `(?i)([_a-z][0-9_a-z]*)?(?=[^0-9\\_a-z])`: the last segment of a
    /// qualified name, or nothing before a character that ends one.
    pub(super) fn last_segment(&mut self, p: usize) -> Option<usize> {
        let ends = |lexer: &Self, at: usize| lexer.avail(at) && !lexer.char_is(at, is_ipath);
        if self.char_is(p, is_iletter) {
            let end = self.chars_while(p, is_iword);
            if !ends(self, end) {
                return None;
            }
            self.cap(1, p, end);
            return Some(end);
        }
        ends(self, p).then_some(p)
    }

    /// `(?i)((namespace)|[0-9_a-z]+)?(\\)(?=.*?[^0-9\\_a-z])`.
    pub(super) fn namespace_part(&mut self, p: usize) -> Option<usize> {
        let keyword = self
            .ilit(p, b"namespace")
            .filter(|&end| self.at(end) == b'\\');
        let (named, slash) = if let Some(end) = keyword {
            (Some(end), end)
        } else {
            let end = self.chars_while(p, is_iword);
            if end > p && self.at(end) == b'\\' {
                (Some(end), end)
            } else if self.at(p) == b'\\' {
                (None, p)
            } else {
                return None;
            }
        };
        let mut at = slash + 1;
        let ahead = loop {
            if at >= self.lim() {
                break self.avail(at);
            }
            if !self.char_is(at, is_ipath) {
                break true;
            }
            at = self.char_end(at);
        };
        if !ahead {
            return None;
        }
        if let Some(end) = named {
            self.cap(1, p, end);
        }
        self.cap(3, slash, slash + 1);
        Some(slash + 1)
    }

    /// `(?i)(?=[0-9_a-z]*[^0-9\\_a-z])`.
    pub(super) fn namespace_end(&self, p: usize) -> Option<usize> {
        let end = self.chars_while(p, is_iword);
        (self.avail(end) && !self.char_is(end, is_ipath)).then_some(p)
    }

    /// `(?i)\s*(?:,|(?=[^0-9\\_a-z\s]))\s*`.
    pub(super) fn implements_separator(&self, p: usize) -> Option<usize> {
        let at = self.space_end(p);
        if self.at(at) == b',' && at < self.lim() {
            return Some(self.space_end(at + 1));
        }
        (at < self.lim() && !self.char_is(at, is_ipath) && self.space_at(at) == 0).then_some(at)
    }

    /// A namespaced constant ahead:
    ///
    /// ```text
    /// (?i)(?=((\\NAME\\[_a-z][0-9\\_a-z]*)|(NAME\\[_a-z][0-9\\_a-z]*))[^0-9\\_a-z])
    /// ```
    ///
    /// `[0-9\\_a-z]*` takes every such character, so the character after it
    /// ends the constant.
    pub(super) fn ahead_namespaced_constant(&self, p: usize) -> Option<usize> {
        let at = p + usize::from(self.at(p) == b'\\');
        let name = self.iname(at)?;
        if self.at(name) != b'\\' || !self.char_is(name + 1, is_iletter) {
            return None;
        }
        let end = self.chars_while(name + 1, is_ipath);
        self.avail(end).then_some(p)
    }

    /// `(?i)\b(TRUE|FALSE|NULL|__(FILE|DIR|FUNCTION|CLASS|METHOD|LINE|NAMESPACE)__)\b`.
    pub(super) fn language_constant(&self, p: usize) -> Option<usize> {
        self.iword_in(
            p,
            &[
                b"true",
                b"false",
                b"null",
                b"__file__",
                b"__dir__",
                b"__function__",
                b"__class__",
                b"__method__",
                b"__line__",
                b"__namespace__",
            ],
        )
    }

    /// `(?i)(?:|new)type\s+([0-9_a-z]+)` at `p`, with the keyword as group 2
    /// and the name as group 3.
    fn type_and_name(&mut self, p: usize) -> Option<usize> {
        for keyword in [&b"type"[..], b"newtype"] {
            if let Some(end) = self.ilit(p, keyword)
                && let Some(name) = self.spaces_name(end, 3)
            {
                self.cap(2, p, end);
                return Some(name);
            }
        }
        None
    }

    /// `(?i)^\s*(?:(module)\s*)?((?:|new)type)\s+([0-9_a-z]+)`.
    pub(super) fn typedef(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let at = self.space_end(0);
        if let Some(module) = self.ilit(at, b"module") {
            let next = self.space_end(module);
            if let Some(end) = self.type_and_name(next) {
                self.cap(1, at, module);
                return Some(end);
            }
        }
        self.type_and_name(at)
    }

    /// `(?i)^\s*(?:(public|internal)\s+)?(enum)\s+(class)\s+([0-9_a-z]+)\s*:?`.
    pub(super) fn enum_class(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let at = self.space_end(0);
        self.after_visibility(at, |lexer, q| {
            let keyword = lexer.ilit(q, b"enum")?;
            let class = lexer.space_end(keyword);
            if class == keyword {
                return None;
            }
            let class_end = lexer.ilit(class, b"class")?;
            let name = lexer.spaces_name(class_end, 4)?;
            lexer.cap(2, q, keyword);
            lexer.cap(3, class, class_end);
            let end = lexer.space_end(name);
            Some(end + usize::from(lexer.at(end) == b':' && end < lexer.lim()))
        })
    }

    /// `(?i)^\s*(?:(public|internal)\s+)?(enum)\s+([0-9_a-z]+)\s*:?`.
    pub(super) fn enum_declaration(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let at = self.space_end(0);
        self.after_visibility(at, |lexer, q| {
            let keyword = lexer.ilit(q, b"enum")?;
            let name = lexer.spaces_name(keyword, 3)?;
            lexer.cap(2, q, keyword);
            let end = lexer.space_end(name);
            Some(end + usize::from(lexer.at(end) == b':' && end < lexer.lim()))
        })
    }

    /// `(?i)^\s*(?:(public|internal)\s+)?(trait)\s+([0-9_a-z]+)\s*`.
    pub(super) fn trait_declaration(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let at = self.space_end(0);
        self.after_visibility(at, |lexer, q| {
            let keyword = lexer.ilit(q, b"trait")?;
            let name = lexer.spaces_name(keyword, 3)?;
            lexer.cap(2, q, keyword);
            Some(lexer.space_end(name))
        })
    }

    /// `^\s*(new)\s+(module)\s+([.0-9A-Z_a-z]+)\b` for [`Module::New`], and
    /// `^\s*(module)\s+([.0-9A-Z_a-z]+)\b` for [`Module::Plain`].
    /// The name gives back trailing dots until `\b` holds.
    pub(super) fn module_declaration(&mut self, p: usize, module: Module) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let mut at = self.space_end(0);
        let group = match module {
            Module::New => {
                let end = self.lit(at, b"new")?;
                let next = self.space_end(end);
                if next == end {
                    return None;
                }
                self.cap(1, at, end);
                at = next;
                2
            }
            Module::Plain => 1,
        };
        let keyword = self.lit(at, b"module")?;
        let name = self.space_end(keyword);
        if name == keyword {
            return None;
        }
        let mut end = self.skip(name, |b| b == b'.' || super::is_ident(b));
        while end > name && !self.boundary(end) {
            end -= 1;
        }
        if end == name {
            return None;
        }
        self.cap(group, at, keyword);
        self.cap(group + 1, name, end);
        Some(end)
    }

    /// `(?i)(?:^\s*|\s*)(namespace)\b\s+(?=([0-9\\_a-z]*\s*($|[;{]|(/[*/
    /// ])))|$)`.
    pub(super) fn namespace_declaration(&mut self, p: usize) -> Option<usize> {
        let at = self.space_end(p);
        let keyword = self.iword_end(at, b"namespace")?;
        let name = self.space_end(keyword);
        if name == keyword {
            return None;
        }
        let rest = self.space_end(self.chars_while(name, is_ipath));
        let ahead = self.at_eol(rest)
            || matches!(self.at(rest), b';' | b'{') && rest < self.lim()
            || self.starts(rest, b"/*")
            || self.starts(rest, b"//")
            || self.at_eol(name);
        if !ahead {
            return None;
        }
        self.cap(1, at, keyword);
        Some(name)
    }

    /// `(?i)(?=\s*$|[^0-9\\_a-z])`.
    pub(super) fn namespace_declaration_end(&self, p: usize) -> Option<usize> {
        (self.at_eol(self.space_end(p)) || self.avail(p) && !self.char_is(p, is_ipath)).then_some(p)
    }

    /// `(?i)\s*\b(use)\s+`.
    pub(super) fn use_statement(&mut self, p: usize) -> Option<usize> {
        let at = self.space_end(p);
        if !self.boundary(at) {
            return None;
        }
        let end = self.ilit(at, b"use")?;
        let next = self.space_end(end);
        if next == end {
            return None;
        }
        self.cap(1, at, end);
        Some(next)
    }

    /// `(?=;|^\s*$)`.
    pub(super) fn use_end(&self, p: usize) -> Option<usize> {
        let semicolon = self.at(p) == b';' && p < self.lim();
        (semicolon || p == 0 && self.at_eol(self.space_end(0))).then_some(p)
    }

    /// `(?=[,;]|$)` at `p`.
    fn list_end(&self, p: usize) -> bool {
        matches!(self.at(p), b',' | b';') && p < self.lim() || self.at_eol(p)
    }

    /// `(?i)(?:\s*(as)\b\s*([0-9_a-z]*)\s*(?=[,;]|$)|(?=[,;]|$))`.
    pub(super) fn use_alias(&mut self, p: usize) -> Option<usize> {
        let at = self.space_end(p);
        if let Some(keyword) = self.iword_end(at, b"as") {
            let name = self.space_end(keyword);
            let name_end = self.chars_while(name, is_iword);
            let end = self.space_end(name_end);
            if self.list_end(end) {
                self.cap(1, at, keyword);
                self.cap(2, name, name_end);
                return Some(end);
            }
        }
        self.list_end(p).then_some(p)
    }

    /// `(?i)^\s*((?:(?:final|abstract|public|internal)\s+)*)(class)\
    /// s+([0-9_a-z]+)\s*`.
    pub(super) fn class_declaration(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let start = self.space_end(0);
        let mut at = start;
        loop {
            let next = [&b"final"[..], b"abstract", b"public", b"internal"]
                .iter()
                .find_map(|word| {
                    let end = self.ilit(at, word)?;
                    let next = self.space_end(end);
                    (next > end).then_some(next)
                });
            match next {
                Some(next) => at = next,
                None => break,
            }
        }
        let keyword = self.ilit(at, b"class")?;
        let name = self.spaces_name(keyword, 3)?;
        self.cap(1, start, at);
        self.cap(2, at, keyword);
        Some(self.space_end(name))
    }

    /// `\s*\b(await|break|…|while)\b`: a control keyword.
    pub(super) fn control_keyword(&mut self, p: usize) -> Option<usize> {
        let at = self.space_end(p);
        let end = self.word_in(
            at,
            &[
                b"await",
                b"break",
                b"case",
                b"continue",
                b"concurrent",
                b"default",
                b"delay",
                b"do",
                b"else",
                b"for",
                b"foreach",
                b"if",
                b"nameof",
                b"return",
                b"switch",
                b"use",
                b"while",
            ],
        )?;
        self.cap(1, at, end);
        Some(end)
    }

    /// `(?i)\b((?:require|include)(?:_once)?)\b\s*`.
    pub(super) fn include(&mut self, p: usize) -> Option<usize> {
        let end = self.iword_in(
            p,
            &[b"require", b"include", b"require_once", b"include_once"],
        )?;
        self.cap(1, p, end);
        Some(self.space_end(end))
    }

    /// `(?i)(NAME)((?:\s*\|\s*NAME)*)\s*((\$+)NAME)`, with PHP's names: the
    /// types a `catch` names and its variable.
    pub(super) fn caught(&mut self, p: usize) -> Option<usize> {
        let first = self.name(p)?;
        let mut end = first;
        loop {
            let bar = self.space_end(end);
            if self.at(bar) != b'|' {
                break;
            }
            match self.name(self.space_end(bar + 1)) {
                Some(next) => end = next,
                None => break,
            }
        }
        let dollars = self.space_end(end);
        let name = self.skip(dollars, |b| b == b'$');
        if name == dollars {
            return None;
        }
        let name_end = self.name(name)?;
        self.cap(1, p, first);
        self.cap(2, first, end);
        self.cap(3, dollars, name_end);
        self.cap(4, dollars, name);
        Some(name_end)
    }

    /// `(?i)\b(const)\s+(?=[\\_a-z])`.
    pub(super) fn const_declaration(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let end = self.ilit(p, b"const")?;
        let next = self.space_end(end);
        if next == end || !(self.at(next) == b'\\' || self.char_is(next, is_iletter)) {
            return None;
        }
        self.cap(1, p, end);
        Some(next)
    }

    /// `(?i)([_a-z][0-9_a-z]*)\s*(?==(?!=))`.
    pub(super) fn const_name(&mut self, p: usize) -> Option<usize> {
        let name = self.iname(p)?;
        let at = self.space_end(name);
        if self.at(at) != b'=' || at >= self.lim() || self.at(at + 1) == b'=' {
            return None;
        }
        self.cap(1, p, name);
        Some(at)
    }

    /// `(?i)\s*(?:(public|internal)\s+)?(function)\s*(?=\()`.
    pub(super) fn closure(&mut self, p: usize) -> Option<usize> {
        let at = self.space_end(p);
        self.after_visibility(at, |lexer, q| {
            let end = lexer.ilit(q, b"function")?;
            let open = lexer.space_end(end);
            if lexer.at(open) != b'(' {
                return None;
            }
            lexer.cap(2, q, end);
            Some(open)
        })
    }

    /// `(?:\s*(&))?\s*((\$+)NAME)\s*(?=[),])`.
    pub(super) fn closure_use_variable(&mut self, p: usize) -> Option<usize> {
        let at = self.space_end(p);
        let amp = self.at(at) == b'&';
        let dollars = if amp { self.space_end(at + 1) } else { at };
        let name = self.skip(dollars, |b| b == b'$');
        if name == dollars {
            return None;
        }
        let name_end = self.hname(name)?;
        let after = self.space_end(name_end);
        if !matches!(self.at(after), b')' | b',') || after >= self.lim() {
            return None;
        }
        if amp {
            self.cap(1, at, at + 1);
        }
        self.cap(2, dollars, name_end);
        self.cap(3, dollars, name);
        Some(after)
    }

    /// `(?i)((\$+)[_a-z\x7F-ÿ][0-9_a-z\x7F-ÿ]*)`.
    pub(super) fn parameter(&mut self, p: usize) -> Option<usize> {
        let name = self.skip(p, |b| b == b'$');
        if name == p || !self.char_is(name, is_iname_start) {
            return None;
        }
        let end = self.chars_while(name, is_iname);
        self.cap(1, p, end);
        self.cap(2, p, name);
        Some(end)
    }

    /// `(?i)\s*(?=[),]|$)`.
    pub(super) fn parameter_end(&self, p: usize) -> Option<usize> {
        let at = self.space_end(p);
        (matches!(self.at(at), b')' | b',') && at < self.lim() || self.at_eol(at)).then_some(at)
    }

    /// A function's declaration up to its name:
    ///
    /// ```text
    /// \s*((?:(?:final|…|async)\s+)*)(function)\s+(?:(__(?:call|…|disposeAsync)(?=[^0-9A-Z_a-z\x7F-ÿ]))|([0-9A-Z_a-z]+))
    /// ```
    pub(super) fn function_declaration(&mut self, p: usize) -> Option<usize> {
        let start = self.space_end(p);
        let mut at = start;
        loop {
            let next = FUNCTION_MODIFIERS.iter().find_map(|word| {
                let end = self.lit(at, word)?;
                let next = self.space_end(end);
                (next > end).then_some(next)
            });
            match next {
                Some(next) => at = next,
                None => break,
            }
        }
        let keyword = self.lit(at, b"function")?;
        let name = self.space_end(keyword);
        if name == keyword {
            return None;
        }
        let magic = self.lit(name, b"__").and_then(|word| {
            MAGIC.iter().find_map(|magic| {
                let end = self.lit(word, magic)?;
                (self.avail(end) && !self.char_is(end, is_name)).then_some(end)
            })
        });
        let end = if let Some(end) = magic {
            self.cap(3, name, end);
            end
        } else {
            let end = self.skip(name, super::is_ident);
            if end == name {
                return None;
            }
            self.cap(4, name, end);
            end
        };
        self.cap(1, start, at);
        self.cap(2, at, keyword);
        Some(end)
    }

    /// `(?i)(\$+)([_a-z][0-9_a-z]*)(?=\s*\()`.
    pub(super) fn invoke_call(&mut self, p: usize) -> Option<usize> {
        let name = self.skip(p, |b| b == b'$');
        if name == p {
            return None;
        }
        let end = self.iname(name)?;
        if !self.ahead_call(end) {
            return None;
        }
        self.cap(1, p, name);
        self.cap(2, name, end);
        Some(end)
    }

    /// `(?i)\s*(?=[$0-9\\_a-z]+(::)…)`: the rest of the lookahead is
    /// optional, so it holds wherever `::` follows the run.
    pub(super) fn ahead_static_access(&self, p: usize) -> Option<usize> {
        let at = self.space_end(p);
        let end = self.chars_while(at, |c| c == '$' || is_ipath(c));
        (end > at && self.starts(end, b"::")).then_some(at)
    }

    /// `(::)(?:([A-Z_a-z][0-9A-Z_a-z]*)\s*\(|((\$+)NAME)|(NAME))?`.
    pub(super) fn static_access(&mut self, p: usize) -> Option<usize> {
        let colons = self.lit(p, b"::")?;
        self.cap(1, p, colons);
        if let Some(name) = self.ident(colons) {
            let open = self.space_end(name);
            if self.at(open) == b'(' && open < self.lim() {
                self.cap(2, colons, name);
                return Some(open + 1);
            }
        }
        let dollars = self.skip(colons, |b| b == b'$');
        if dollars > colons
            && let Some(name) = self.hname(dollars)
        {
            self.cap(3, colons, name);
            self.cap(4, colons, dollars);
            return Some(name);
        }
        if let Some(name) = self.hname(colons) {
            self.cap(5, colons, name);
            return Some(name);
        }
        Some(colons)
    }

    /// A variable with a property or an index:
    ///
    /// ```text
    /// ((\$)(?<name>NAME))(?:(->)(\g<name>)|(\[)(?:(\d+)|((\$)\g<name>)|(\w+))(]))?
    /// ```
    pub(super) fn variable(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'$' {
            return None;
        }
        let name = self.hname(p + 1)?;
        self.cap(1, p, name);
        self.cap(2, p, p + 1);
        if let Some(arrow) = self.lit(name, b"->")
            && let Some(member) = self.hname(arrow)
        {
            self.cap(4, name, arrow);
            self.cap(5, arrow, member);
            return Some(member);
        }
        if self.at(name) != b'[' {
            return Some(name);
        }
        let index = name + 1;
        let close =
            |lexer: &Self, end: usize| (lexer.at(end) == b']' && end < lexer.lim()).then_some(end);
        let digits = self.digit_end(index);
        if digits > index
            && let Some(end) = close(self, digits)
        {
            self.cap(6, name, index);
            self.cap(7, index, end);
            self.cap(11, end, end + 1);
            return Some(end + 1);
        }
        if self.at(index) == b'$'
            && let Some(inner) = self.hname(index + 1)
            && let Some(end) = close(self, inner)
        {
            self.cap(6, name, index);
            self.cap(8, index, end);
            self.cap(9, index, index + 1);
            self.cap(11, end, end + 1);
            return Some(end + 1);
        }
        let word = self.word_end(index);
        if word > index
            && let Some(end) = close(self, word)
        {
            self.cap(6, name, index);
            self.cap(10, index, end);
            self.cap(11, end, end + 1);
            return Some(end + 1);
        }
        Some(name)
    }

    /// `((\$\{)(?<name>NAME)(}))`.
    pub(super) fn braced_variable(&mut self, p: usize) -> Option<usize> {
        let at = self.lit(p, b"${")?;
        let name = self.hname(at)?;
        if self.at(name) != b'}' {
            return None;
        }
        self.cap(1, p, name + 1);
        self.cap(2, p, at);
        self.cap(4, name, name + 1);
        Some(name + 1)
    }

    /// `(\$+)[A-Z_a-z\x7F-ÿ][0-9A-Z_a-z\x7F-ÿ]*?\b`: the lazy name ends at the
    /// first place where `\b` holds.
    pub(super) fn basic_variable(&mut self, p: usize) -> Option<usize> {
        let name = self.skip(p, |b| b == b'$');
        if name == p || !self.char_is(name, is_name_start) {
            return None;
        }
        let mut at = self.char_end(name);
        loop {
            if self.boundary(at) {
                self.cap(1, p, name);
                return Some(at);
            }
            if !self.char_is(at, is_name) {
                return None;
            }
            at = self.char_end(at);
        }
    }

    /// `(\$\{)(?=.*?})`.
    pub(super) fn variable_brace_open(&mut self, p: usize) -> Option<usize> {
        let at = self.lit(p, b"${")?;
        memchr::memchr(b'}', self.text(at, self.lim()))?;
        self.cap(1, p, at);
        Some(at)
    }

    /// `(array)(\()`.
    pub(super) fn array_open(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"array(")?;
        self.cap(1, p, p + 5);
        self.cap(2, p + 5, end);
        Some(end)
    }

    /// `(?i)\s*\(\s*(array|…|keyset)\s*\)`: a cast.
    pub(super) fn cast(&mut self, p: usize) -> Option<usize> {
        let open = self.space_end(p);
        if self.at(open) != b'(' {
            return None;
        }
        let at = self.space_end(open + 1);
        for word in CASTS {
            if let Some(end) = self.ilit(at, word) {
                let close = self.space_end(end);
                if self.at(close) == b')' && close < self.lim() {
                    self.cap(1, at, end);
                    return Some(close + 1);
                }
            }
        }
        None
    }

    /// `(?i)\b(array|…|keyset)\b`: a type keyword.
    pub(super) fn type_keyword(&self, p: usize) -> Option<usize> {
        self.iword_in(
            p,
            &[
                b"array",
                b"real",
                b"double",
                b"float",
                b"int",
                b"integer",
                b"bool",
                b"boolean",
                b"string",
                b"class",
                b"clone",
                b"var",
                b"function",
                b"interface",
                b"trait",
                b"parent",
                b"self",
                b"object",
                b"arraykey",
                b"nonnull",
                b"dict",
                b"vec",
                b"keyset",
            ],
        )
    }

    /// `(?i)\b(global|abstract|const|extends|implements|final|p(r(ivate|otected)|ublic)|internal|static)\b`.
    pub(super) fn modifier_keyword(&self, p: usize) -> Option<usize> {
        self.iword_in(
            p,
            &[
                b"global",
                b"abstract",
                b"const",
                b"extends",
                b"implements",
                b"final",
                b"private",
                b"protected",
                b"public",
                b"internal",
                b"static",
            ],
        )
    }

    /// `(->)(\$?\{)`.
    pub(super) fn object_brace(&mut self, p: usize) -> Option<usize> {
        let arrow = self.lit(p, b"->")?;
        let open = arrow + usize::from(self.at(arrow) == b'$');
        if self.at(open) != b'{' {
            return None;
        }
        self.cap(1, p, arrow);
        self.cap(2, arrow, open + 1);
        Some(open + 1)
    }

    /// `(->)(?:([A-Z_a-z][0-9A-Z_a-z]*)\s*\(|((\$+)?NAME))?`.
    pub(super) fn object_member(&mut self, p: usize) -> Option<usize> {
        let arrow = self.lit(p, b"->")?;
        self.cap(1, p, arrow);
        if let Some(name) = self.ident(arrow) {
            let open = self.space_end(name);
            if self.at(open) == b'(' && open < self.lim() {
                self.cap(2, arrow, name);
                return Some(open + 1);
            }
        }
        let dollars = self.skip(arrow, |b| b == b'$');
        if let Some(name) = self.hname(dollars) {
            self.cap(3, arrow, name);
            if dollars > arrow {
                self.cap(4, arrow, dollars);
            }
            return Some(name);
        }
        Some(arrow)
    }

    /// `(!==?|===?)`.
    pub(super) fn equality(&self, p: usize) -> Option<usize> {
        self.first_lit(p, &[b"!==", b"!=", b"===", b"=="])
    }

    /// `(?:|[-%\&*+/^|]|<<|>>)=`.
    pub(super) fn assignment(&self, p: usize) -> Option<usize> {
        if self.at(p) == b'=' && p < self.lim() {
            return Some(p + 1);
        }
        if matches!(
            self.at(p),
            b'-' | b'%' | b'&' | b'*' | b'+' | b'/' | b'^' | b'|'
        ) && self.at(p + 1) == b'='
        {
            return Some(p + 2);
        }
        self.first_lit(p, &[b"<<=", b">>="])
    }

    /// `(<=|>=|[<>])`.
    pub(super) fn comparison(&self, p: usize) -> Option<usize> {
        self.first_lit(p, &[b"<=", b">=", b"<", b">"])
    }

    /// `(?i)\b(as|is|upcast)\b\s+(?=[$?\\_a-z~])`.
    pub(super) fn type_operator(&mut self, p: usize) -> Option<usize> {
        let end = self.iword_in(p, &[b"as", b"is", b"upcast"])?;
        let next = self.space_end(end);
        let ahead =
            matches!(self.at(next), b'$' | b'?' | b'\\' | b'~') || self.char_is(next, is_iletter);
        if next == end || !ahead {
            return None;
        }
        self.cap(1, p, end);
        Some(next)
    }

    /// `(?i)(?=\\?[0-9\\_a-z]+\\[_a-z][0-9_a-z]*\s*\()`: a call of a
    /// namespaced function ahead.
    /// The run `[0-9\\_a-z]+` gives back to its last `\`, after which the
    /// function's name runs to the run's end.
    pub(super) fn ahead_qualified_call(&self, p: usize) -> Option<usize> {
        let tries = [p + usize::from(self.at(p) == b'\\'), p];
        tries.into_iter().find_map(|at| {
            let end = self.chars_while(at, is_ipath);
            let slash = (at..end).rev().find(|&i| self.at(i) == b'\\')?;
            (slash > at && self.char_is(slash + 1, is_iletter) && self.ahead_call(end)).then_some(p)
        })
    }

    /// `(?i)(?=[0-9\\_a-z]*[_a-z][0-9_a-z]*\s*\()`: a call ahead.
    /// The name runs to the end of the run, after its last `\`, and starts at
    /// any letter there.
    pub(super) fn ahead_user_call(&self, p: usize) -> Option<usize> {
        let end = self.chars_while(p, is_ipath);
        let segment = (p..end)
            .rev()
            .find(|&i| self.at(i) == b'\\')
            .map_or(p, |slash| slash + 1);
        let mut at = segment;
        while at < end {
            if self.char_is(at, is_iletter) {
                return self.ahead_call(end).then_some(p);
            }
            at = self.char_end(at);
        }
        None
    }

    /// `(?i)(\\)?(?=\b[_a-z][0-9_a-z]*\s*\()`.
    pub(super) fn call_name(&mut self, p: usize) -> Option<usize> {
        let at = p + usize::from(self.at(p) == b'\\');
        if !self.boundary(at) || !self.char_is(at, is_iletter) {
            return None;
        }
        if !self.ahead_call(self.chars_while(at, is_iword)) {
            return None;
        }
        if at > p {
            self.cap(1, p, at);
        }
        Some(at)
    }

    /// `(?i)\b(isset|unset|e(val|mpty)|list)(?=\s*\()`.
    pub(super) fn builtin_call(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        [&b"isset"[..], b"unset", b"eval", b"empty", b"list"]
            .iter()
            .find_map(|word| self.ilit(p, word).filter(|&end| self.ahead_call(end)))
    }

    /// `[Ee][-+]?[0-9]+` at `at`, then `\b`: the exponent's digits give back
    /// until `\b` holds.
    fn exponent_end(&self, at: usize) -> Option<usize> {
        if !matches!(self.at(at), b'E' | b'e') {
            return None;
        }
        let digits = at + 1 + usize::from(matches!(self.at(at + 1), b'-' | b'+'));
        let mut end = self.skip(digits, |b| b.is_ascii_digit());
        while end > digits {
            if self.boundary(end) {
                return Some(end);
            }
            end -= 1;
        }
        None
    }

    /// The end `e` of a number's digits, then an exponent and `\b`, or `\b`
    /// alone.
    fn number_tail(&self, end: usize) -> Option<usize> {
        self.exponent_end(end)
            .or_else(|| self.boundary(end).then_some(end))
    }

    /// A number, as Oniguruma backtracks through
    /// `\b((0([Xx])\h*)|(([0-9]+\.?[0-9]*)|(\.[0-9]+))(([Ee])([-+])?[0-9]+)?)\
    /// b`.
    pub(super) fn number(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        if self.at(p) == b'0' && matches!(self.at(p + 1), b'x' | b'X') {
            let mut end = self.skip(p + 2, |b| b.is_ascii_hexdigit());
            loop {
                if self.boundary(end) {
                    return Some(end);
                }
                if end == p + 2 {
                    break;
                }
                end -= 1;
            }
        }
        if self.at(p).is_ascii_digit() {
            let digits = self.skip(p, |b| b.is_ascii_digit());
            if self.at(digits) == b'.' {
                let mut end = self.skip(digits + 1, |b| b.is_ascii_digit());
                loop {
                    if let Some(found) = self.number_tail(end) {
                        return Some(found);
                    }
                    if end == digits + 1 {
                        break;
                    }
                    end -= 1;
                }
            }
            return self.number_tail(digits);
        }
        if self.at(p) == b'.' {
            let mut end = self.skip(p + 1, |b| b.is_ascii_digit());
            while end > p + 1 {
                if let Some(found) = self.number_tail(end) {
                    return Some(found);
                }
                end -= 1;
            }
        }
        None
    }

    /// `(parent|static|self)(?=[^0-9_a-z])`, case as written.
    pub(super) fn instantiated_keyword(&self, p: usize) -> Option<usize> {
        [&b"parent"[..], b"static", b"self"]
            .iter()
            .find_map(|word| {
                let end = self.lit(p, word)?;
                let next = self.at(end);
                let blocked = next.is_ascii_digit() || next == b'_' || next.is_ascii_lowercase();
                (self.avail(end) && !(blocked && end < self.lim())).then_some(end)
            })
    }

    /// `(Vector|ImmVector|Set|ImmSet|Map|ImmMap|Pair)\s*(\{)`.
    pub(super) fn literal_collection(&mut self, p: usize) -> Option<usize> {
        let words: [&[u8]; 7] = [
            b"Vector",
            b"ImmVector",
            b"Set",
            b"ImmSet",
            b"Map",
            b"ImmMap",
            b"Pair",
        ];
        let end = words.iter().find_map(|word| self.lit(p, word))?;
        let open = self.space_end(end);
        if self.at(open) != b'{' || open >= self.lim() {
            return None;
        }
        self.cap(1, p, end);
        self.cap(2, open, open + 1);
        Some(open + 1)
    }
}
