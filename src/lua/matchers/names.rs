//! Names: function heads, keywords, the standard library, variables and
//! operators.

use super::super::engine::Lexer;
use crate::rules::{CachedRuns, Chars};

/// The control keywords.
pub(super) const KEYWORDS: &[&[u8]] = &[
    b"break", b"do", b"else", b"for", b"if", b"elseif", b"goto", b"return", b"then", b"repeat",
    b"while", b"until", b"end", b"in",
];

/// The standard library's global functions.
const BUILTINS: &[&[u8]] = &[
    b"assert",
    b"collectgarbage",
    b"dofile",
    b"error",
    b"getfenv",
    b"getmetatable",
    b"ipairs",
    b"load",
    b"loadfile",
    b"loadstring",
    b"module",
    b"next",
    b"pairs",
    b"pcall",
    b"print",
    b"rawequal",
    b"rawget",
    b"rawlen",
    b"rawset",
    b"require",
    b"select",
    b"setfenv",
    b"setmetatable",
    b"tonumber",
    b"tostring",
    b"type",
    b"unpack",
    b"xpcall",
];

/// The constants of the standard library's tables, by table.
const LIBRARY_CONSTANTS: &[(&[u8], &[&[u8]])] = &[
    (b"math", &[b"pi", b"huge", b"maxinteger", b"mininteger"]),
    (b"utf8", &[b"charpattern"]),
    (b"io", &[b"stdin", b"stdout", b"stderr"]),
    (
        b"package",
        &[
            b"config",
            b"cpath",
            b"loaded",
            b"loaders",
            b"path",
            b"preload",
            b"searchers",
        ],
    ),
];

/// The functions of the standard library's tables, by table, with the
/// grammar's `[gs]`, `[Cc]` and `?` spelled out.
const LIBRARY_FUNCTIONS: &[(&[u8], &[&[u8]])] = &[
    (
        b"coroutine",
        &[
            b"create",
            b"isyieldable",
            b"close",
            b"resume",
            b"running",
            b"status",
            b"wrap",
            b"yield",
        ],
    ),
    (
        b"string",
        &[
            b"byte",
            b"char",
            b"dump",
            b"find",
            b"format",
            b"gmatch",
            b"gsub",
            b"len",
            b"lower",
            b"match",
            b"pack",
            b"packsize",
            b"rep",
            b"reverse",
            b"sub",
            b"unpack",
            b"upper",
        ],
    ),
    (
        b"table",
        &[
            b"concat", b"create", b"insert", b"maxn", b"move", b"pack", b"remove", b"sort",
            b"unpack",
        ],
    ),
    (
        b"math",
        &[
            b"abs",
            b"acos",
            b"asin",
            b"atan",
            b"atan2",
            b"ceil",
            b"cos",
            b"cosh",
            b"deg",
            b"exp",
            b"floor",
            b"fmod",
            b"frexp",
            b"ldexp",
            b"log",
            b"log10",
            b"max",
            b"min",
            b"modf",
            b"pow",
            b"rad",
            b"random",
            b"randomseed",
            b"sin",
            b"sinh",
            b"sqrt",
            b"tan",
            b"tanh",
            b"tointeger",
            b"type",
        ],
    ),
    (
        b"io",
        &[
            b"close", b"flush", b"input", b"lines", b"open", b"output", b"popen", b"read",
            b"tmpfile", b"type", b"write",
        ],
    ),
    (
        b"os",
        &[
            b"clock",
            b"date",
            b"difftime",
            b"execute",
            b"exit",
            b"getenv",
            b"remove",
            b"rename",
            b"setlocale",
            b"time",
            b"tmpname",
        ],
    ),
    (b"package", &[b"loadlib", b"seeall", b"searchpath"]),
    (
        b"debug",
        &[
            b"debug",
            b"getfenv",
            b"setfenv",
            b"gethook",
            b"sethook",
            b"getinfo",
            b"getlocal",
            b"setlocal",
            b"getmetatable",
            b"setmetatable",
            b"getregistry",
            b"getupvalue",
            b"setupvalue",
            b"getuservalue",
            b"setuservalue",
            b"setCstacklimit",
            b"setcstacklimit",
            b"traceback",
            b"upvalueid",
            b"upvaluejoin",
        ],
    ),
    (
        b"bit32",
        &[
            b"arshift", b"band", b"bnot", b"bor", b"btest", b"bxor", b"extract", b"replace",
            b"lrotate", b"lshift", b"rrotate", b"rshift",
        ],
    ),
    (
        b"utf8",
        &[b"char", b"codes", b"codepoint", b"len", b"offset"],
    ),
];

impl Lexer<'_> {
    /// `function\b(?![,:])` at `p`: the end of the keyword.
    fn function_word(&self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"function")?;
        (self.word_at(end).is_none() && !self.ahead_in(end, b",:")).then_some(end)
    }

    /// `\b(?:(local)\s+)?(function)\b(?![,:])`: a function's head, with
    /// `local` as capture group 1 and `function` as 2.
    pub(super) fn function_head(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        if let Some(local) = self.lit(p, b"local") {
            let at = self.space_end(local);
            if at > local
                && let Some(end) = self.function_word(at)
            {
                self.capture(1, p, local);
                self.capture(2, at, end);
                return Some(end);
            }
        }
        let end = self.function_word(p)?;
        self.capture(2, p, end);
        Some(end)
    }

    /// `\b(function)\b(?![,:])`
    pub(super) fn function_keyword(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        self.function_word(p)
    }

    /// `(?<=[-\]"')\[{}])`: where a function's head ends, after its
    /// parameters or a string or table argument.
    pub(super) fn after_function_head(&self, p: usize) -> Option<usize> {
        self.prev(p)
            .is_some_and(|b| b"-]\"')[{}".contains(&b))
            .then_some(p)
    }

    /// `(\))|(?=[-\]"'\[{}])|(?<!\.)\.(?!\.)`: the closing parenthesis as
    /// capture group 1, or where the parameters end without one.
    pub(super) fn parameters_end(&mut self, p: usize) -> Option<usize> {
        if let Some(end) = self.lit(p, b")") {
            self.capture(1, p, end);
            return Some(end);
        }
        if self.ahead_in(p, b"-]\"'[{}") {
            return Some(p);
        }
        let end = self.lit(p, b".")?;
        self.whole_dots(p, end).then_some(end)
    }

    /// `(\.{3})\s*([A-Z_a-z][0-9A-Z_a-z]*)`: a named vararg.
    /// `...` is capture group 1, and the name is 2.
    pub(super) fn vararg_parameter(&mut self, p: usize) -> Option<usize> {
        let dots = self.lit(p, b"...")?;
        let name = self.space_end(dots);
        let end = self.ascii_name(name)?;
        self.capture(1, p, dots);
        self.capture(2, name, end);
        Some(end)
    }

    /// `\b([A-Z_a-z][0-9A-Z_a-z]*)\b\s*(?=:)`: a function name before the
    /// colon of a method. The blanks cannot give back a colon.
    pub(super) fn function_name_before_colon(&self, p: usize) -> Option<usize> {
        let end = self.space_end(self.bounded_name(p)?);
        (self.at(end) == b':' && end < self.lim()).then_some(end)
    }

    /// `\b(goto)\s+([A-Z_a-z][0-9A-Z_a-z]*)`: `goto` as capture group 1 and its
    /// label as 2.
    pub(super) fn goto_label(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let word = self.lit(p, b"goto")?;
        let name = self.space_end(word);
        if name == word {
            return None;
        }
        let end = self.ascii_name(name)?;
        self.capture(1, p, word);
        self.capture(2, name, end);
        Some(end)
    }

    /// `(::)\s*[A-Z_a-z][0-9A-Z_a-z]*\s*(::)`: a label.
    /// Its two `::` are capture groups 1 and 2.
    /// Neither the name nor the blanks can give back a colon.
    pub(super) fn label(&mut self, p: usize) -> Option<usize> {
        let open = self.lit(p, b"::")?;
        let close = self.space_end(self.ascii_name(self.space_end(open))?);
        let end = self.lit(close, b"::")?;
        self.capture(1, p, open);
        self.capture(2, close, end);
        Some(end)
    }

    /// `<\s*(c(?:onst|lose))\s*>`: a local variable's attribute.
    pub(super) fn attribute(&self, p: usize) -> Option<usize> {
        let name = self.space_end(self.lit(p, b"<")?);
        let word = self
            .lit(name, b"const")
            .or_else(|| self.lit(name, b"close"))?;
        self.lit(self.space_end(word), b">")
    }

    /// `^\s*(global)\b(?!\s*=)`: `global` that starts a line and declares, as
    /// capture group 1.
    pub(super) fn global_declaration(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let at = self.space_end(p);
        let end = self.lit(at, b"global")?;
        if self.word_at(end).is_some() {
            return None;
        }
        let after = self.space_end(end);
        if self.at(after) == b'=' && after < self.lim() {
            return None;
        }
        self.capture(1, at, end);
        Some(end)
    }

    /// `(?!\s*=(?!=))`: no assignment follows `p`.
    fn not_assignment(&self, p: usize) -> bool {
        let at = self.space_end(p);
        !(self.at(at) == b'='
            && at < self.lim()
            && !(self.at(at + 1) == b'=' && at + 1 < self.lim()))
    }

    /// `\s*=\s*\b(function)\b` at `p`: a function is assigned.
    fn function_assigned(&self, p: usize) -> bool {
        let at = self.space_end(p);
        self.at(at) == b'='
            && at < self.lim()
            && self
                .keyword_out_of_line(self.space_end(at + 1), b"function")
                .is_some()
    }

    /// `\b(table\.(member|...))\b` at `p` for one of `tables`: the end of the
    /// member, whose whole word must be in the table's list.
    fn library_member(&self, p: usize, tables: &[(&[u8], &[&[u8]])]) -> Option<usize> {
        let table = self.word_run(p);
        let members = tables
            .iter()
            .find(|(name, _)| self.text(p, table) == *name)?
            .1;
        let start = self.lit(table, b".")?;
        let end = self.word_run(start);
        members.contains(&self.text(start, end)).then_some(end)
    }

    /// A constant, or the vararg `...`:
    ///
    /// ```text
    /// (?<![^.]\.|:)\b(false|nil(?!:)|true|_ENV|_G|_VERSION|math\.(...)|...)\b|(?<!\.)\.{3}(?!\.)
    /// ```
    pub(super) fn constant(&self, p: usize) -> Option<usize> {
        if self.not_member(p) && self.boundary(p) {
            let word = self.word_run(p);
            let found = match self.text(p, word) {
                b"false" | b"true" | b"_ENV" | b"_G" | b"_VERSION" => Some(word),
                b"nil" if !(self.at(word) == b':' && word < self.lim()) => Some(word),
                _ => self.library_member(p, LIBRARY_CONSTANTS),
            };
            if found.is_some() {
                return found;
            }
        }
        let end = self.lit(p, b"...")?;
        self.whole_dots(p, end).then_some(end)
    }

    /// `(?<![^.]\.|:)\b(words)\b`: one of `words`, unless it is a member.
    pub(super) fn not_member_word(&self, p: usize, words: &[&[u8]]) -> Option<usize> {
        if !self.not_member(p) {
            return None;
        }
        self.word_in(p, words)
    }

    /// `(?<![^.]\.|:)\b(assert|...|xpcall)\b(?!\s*=(?!=))`: a global function
    /// of the standard library that is not assigned to.
    pub(super) fn builtin_function(&self, p: usize) -> Option<usize> {
        self.not_member_word(p, BUILTINS)
            .filter(|&end| self.not_assignment(end))
    }

    /// `(?<![^.]\.|:)\b(async)\b(?!\s*=(?!=))`
    pub(super) fn async_word(&self, p: usize) -> Option<usize> {
        self.not_member_word(p, &[b"async"])
            .filter(|&end| self.not_assignment(end))
    }

    /// `(?<![^.]\.|:)\b(coroutine\.(...)|...|utf8\.(...))\b(?!\s*=(?!=))`: a
    /// function of a standard library table that is not assigned to.
    pub(super) fn library_function(&self, p: usize) -> Option<usize> {
        if !self.not_member(p) || !self.boundary(p) {
            return None;
        }
        self.library_member(p, LIBRARY_FUNCTIONS)
            .filter(|&end| self.not_assignment(end))
    }

    /// `\b(\|\||&&|!)\b`: an operator of another language between two words.
    pub(super) fn symbol_operator(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let end = self
            .lit(p, b"||")
            .or_else(|| self.lit(p, b"&&"))
            .or_else(|| self.lit(p, b"!"))?;
        self.boundary(end).then_some(end)
    }

    /// `\b([A-Z_a-z][0-9A-Z_a-z]*)\b(?=\s*(?:["'({]|\[\[))`: a name called with
    /// arguments, a string or a table.
    pub(super) fn call_name(&self, p: usize) -> Option<usize> {
        let end = self.bounded_name(p)?;
        let at = self.space_end(end);
        (self.ahead_in(at, b"\"'({") || self.starts(at, b"[[")).then_some(end)
    }

    /// `\b([A-Z_a-z][0-9A-Z_a-z]*)\b(?=\s*\??:)`: a name before a colon.
    pub(super) fn name_before_colon(&self, p: usize) -> Option<usize> {
        let end = self.bounded_name(p)?;
        let at = self.space_end(end);
        let colon = self.byte_of(at, b"?").unwrap_or(at);
        (self.at(colon) == b':' && colon < self.lim()).then_some(end)
    }

    /// `(?<=[^.]\.|:)\b([A-Z_a-z][0-9A-Z_a-z]*)\b(?!\s*=\s*\b(function)\b)`: a
    /// member that is not assigned a function.
    pub(super) fn member_name(&self, p: usize) -> Option<usize> {
        if self.not_member(p) {
            return None;
        }
        self.variable_name(p)
    }

    /// `\b([A-Z_a-z][0-9A-Z_a-z]*)\b(?!\s*=\s*\b(function)\b)`
    pub(super) fn variable_name(&self, p: usize) -> Option<usize> {
        self.bounded_name(p)
            .filter(|&end| !self.function_assigned(end))
    }

    /// `\b([A-Z_a-z][0-9A-Z_a-z]*)\b(?=\s*=\s*\b(function)\b)`
    pub(super) fn function_variable(&self, p: usize) -> Option<usize> {
        self.bounded_name(p)
            .filter(|&end| self.function_assigned(end))
    }

    /// `[-#%*+/^]|==?|~=|!=|<=?|>=?|(?<!\.)\.{2}(?!\.)`
    pub(super) fn operator(&self, p: usize) -> Option<usize> {
        if p >= self.lim() {
            return None;
        }
        let next_is_equals = self.at(p + 1) == b'=' && p + 1 < self.lim();
        match self.at(p) {
            b'-' | b'#' | b'%' | b'*' | b'+' | b'/' | b'^' => Some(p + 1),
            b'=' | b'<' | b'>' => Some(if next_is_equals { p + 2 } else { p + 1 }),
            b'~' | b'!' => next_is_equals.then_some(p + 2),
            b'.' => {
                let end = self.lit(p, b"..")?;
                self.whole_dots(p, end).then_some(end)
            }
            _ => None,
        }
    }
}
