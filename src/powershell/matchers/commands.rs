//! Commands, functions, attributes, `using` and `class`.

use super::super::engine::Lexer;
use crate::classes::{is_digit, is_letter};
use crate::rules::Chars;

/// The verbs of a command's name, lowercase.
const VERBS: &[&[u8]] = &[
    b"add",
    b"approve",
    b"assert",
    b"backup",
    b"block",
    b"build",
    b"checkpoint",
    b"clear",
    b"close",
    b"compare",
    b"complete",
    b"compress",
    b"confirm",
    b"connect",
    b"convert",
    b"convertfrom",
    b"convertto",
    b"copy",
    b"debug",
    b"deny",
    b"deploy",
    b"disable",
    b"disconnect",
    b"dismount",
    b"edit",
    b"enable",
    b"enter",
    b"exit",
    b"expand",
    b"export",
    b"find",
    b"format",
    b"get",
    b"grant",
    b"group",
    b"hide",
    b"import",
    b"initialize",
    b"install",
    b"invoke",
    b"join",
    b"limit",
    b"lock",
    b"measure",
    b"merge",
    b"mount",
    b"move",
    b"new",
    b"open",
    b"optimize",
    b"out",
    b"ping",
    b"pop",
    b"protect",
    b"publish",
    b"push",
    b"read",
    b"receive",
    b"redo",
    b"register",
    b"remove",
    b"rename",
    b"repair",
    b"request",
    b"reset",
    b"resize",
    b"resolve",
    b"restart",
    b"restore",
    b"resume",
    b"revoke",
    b"save",
    b"search",
    b"select",
    b"send",
    b"set",
    b"show",
    b"skip",
    b"split",
    b"start",
    b"step",
    b"stop",
    b"submit",
    b"suspend",
    b"switch",
    b"sync",
    b"test",
    b"trace",
    b"unblock",
    b"undo",
    b"uninstall",
    b"unlock",
    b"unprotect",
    b"unpublish",
    b"unregister",
    b"update",
    b"use",
    b"wait",
    b"watch",
    b"write",
];

/// The extensions a command's name may end with, in the grammar's order.
const COMMAND_EXTENSIONS: &[&[u8]] = &[b"exe", b"cmd", b"bat", b"ps1"];

/// The extensions of an executable, in the grammar's order.
const EXECUTABLE_EXTENSIONS: &[&[u8]] = &[b"exe", b"com", b"cmd", b"bat"];

/// The words that open a function, in the grammar's order.
const FUNCTIONS: &[&[u8]] = &[b"function", b"filter", b"configuration", b"workflow"];

/// The scopes a function's name may take, in the grammar's order.
const FUNCTION_SCOPES: &[&[u8]] = &[b"global", b"local", b"script", b"private"];

/// The attributes the grammar names, lowercase.
const ATTRIBUTES: &[&[u8]] = &[
    b"cmdletbinding",
    b"alias",
    b"outputtype",
    b"parameter",
    b"validatenotnull",
    b"validatenotnullorempty",
    b"validatecount",
    b"validateset",
    b"allownull",
    b"allowemptycollection",
    b"allowemptystring",
    b"validatescript",
    b"validaterange",
    b"validatepattern",
    b"validatelength",
    b"supportswildcards",
];

/// The named arguments of an attribute, lowercase.
const ATTRIBUTE_ARGUMENTS: &[&[u8]] = &[
    b"mandatory",
    b"valuefrompipeline",
    b"valuefrompipelinebypropertyname",
    b"valuefromremainingarguments",
    b"position",
    b"parametersetname",
    b"defaultparametersetname",
    b"supportsshouldprocess",
    b"supportspaging",
    b"positionalbinding",
    b"helpuri",
    b"confirmimpact",
    b"helpmessage",
];

/// The longest verb, in bytes.
const LONGEST_VERB: usize = 11;

/// Whether `c` is in `[-:\\_\p{L}\d]`, the characters of a command's module
/// path.
fn is_path(c: char) -> bool {
    matches!(c, '-' | ':' | '\\' | '_') || is_letter(c) || is_digit(c)
}

/// Whether `c` is in `[-._\p{L}\d]`, the characters of a function's name.
fn is_function_name(c: char) -> bool {
    matches!(c, '-' | '.' | '_') || is_letter(c) || is_digit(c)
}

/// Whether `c` is in `[-_\p{L}\d]`, the characters of a class's name.
fn is_class_name(c: char) -> bool {
    matches!(c, '-' | '_') || is_letter(c) || is_digit(c)
}

impl Lexer<'_> {
    /// `(?:([-:\\_\p{L}\d])*\\)?\b(?i:Add|…|Write)-.+?(?:\.(?i:
    /// exe|cmd|bat|ps1))?\b`: a command, with an optional module path before
    /// it.
    ///
    /// The path's characters include `\`.
    /// The path therefore gives characters back to each `\` in it, the last
    /// first, and the command is tried after each.
    /// Then it is tried with no path.
    pub(super) fn command(&self, p: usize) -> Option<usize> {
        let run = self.class_end(p, is_path);
        let mut end = run;
        while let Some(slash) = self
            .s
            .get(p..end)
            .and_then(|path| memchr::memrchr(b'\\', path))
        {
            let slash = p + slash;
            if let Some(found) = self.command_name(slash + 1) {
                return Some(found);
            }
            end = slash;
        }
        self.command_name(p)
    }

    /// `\b(?i:Add|…|Write)-.+?(?:\.(?i:exe|cmd|bat|ps1))?\b` at `q`.
    ///
    /// Each verb must be followed by `-`, so the verb is the word before the
    /// `-`.
    /// The `.+?` takes as few characters as it can, one at least.
    /// After each it tries an extension, then `\b`.
    fn command_name(&self, q: usize) -> Option<usize> {
        if !self.boundary(q) {
            return None;
        }
        let dash = self.verb(q)?;
        let mut end = dash + 1;
        while end < self.lim() {
            end = self.char_end(end);
            if self.at(end) == b'.'
                && let Some(found) = self.first_folded(end + 1, COMMAND_EXTENSIONS, Chars::boundary)
            {
                self.spend(end - dash);
                return Some(found);
            }
            if self.boundary(end) {
                self.spend(end - dash);
                return Some(end);
            }
        }
        self.spend(end - dash);
        None
    }

    /// Where the `-` after a verb at `q` stands.
    ///
    /// An ASCII word is looked up lowercase.
    /// A word with a character above ASCII is compared verb by verb under
    /// `(?i)`.
    fn verb(&self, q: usize) -> Option<usize> {
        let letters = self.skip(q, |b| b.is_ascii_alphabetic());
        if self.at(letters) >= 0x80 {
            return VERBS
                .iter()
                .find_map(|verb| self.folded(q, verb).filter(|&end| self.at(end) == b'-'));
        }
        if self.at(letters) != b'-' || letters == q || letters - q > LONGEST_VERB {
            return None;
        }
        let mut lower = [0; LONGEST_VERB];
        let word = lower.get_mut(..letters - q)?;
        word.copy_from_slice(self.text(q, letters));
        word.make_ascii_lowercase();
        VERBS.contains(&&*word).then_some(letters)
    }

    /// `(?<!\w)(?i:word)(?!\w)` for the command `word`, such as
    /// `foreach-object`.
    pub(super) fn standalone(&self, p: usize, word: &[u8]) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        self.folded(p, word)
            .filter(|&end| self.word_at(end).is_none())
    }

    /// `\b((([-.0-9A-Z_a-z]+)\.(?i:exe|com|cmd|bat)))\b`: an executable.
    ///
    /// The name's characters include `.`.
    /// The name therefore gives characters back to each `.` in it, the last
    /// first.
    pub(super) fn executable(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let run = self.skip(p, |b| {
            matches!(b, b'-' | b'.' | b'_') || b.is_ascii_alphanumeric()
        });
        let mut end = run;
        while let Some(dot) = self
            .s
            .get(p + 1..end)
            .and_then(|name| memchr::memrchr(b'.', name))
        {
            let dot = p + 1 + dot;
            if let Some(found) = self.first_folded(dot + 1, EXECUTABLE_EXTENSIONS, Chars::boundary)
            {
                return Some(found);
            }
            end = dot;
        }
        None
    }

    /// `(?<!\w)(?i:(using))\s+(?i:(namespace|module))\s+(?i:((?:\w+\.?)+))`:
    /// a `using` directive, groups 1 to 3 around its words.
    pub(super) fn using(&mut self, p: usize) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        let using = self.folded(p, b"using")?;
        let keyword_at = self.space_end(using);
        if keyword_at == using {
            return None;
        }
        let keyword = self.first_folded(keyword_at, &[b"namespace", b"module"], |_, _| true)?;
        let name_at = self.space_end(keyword);
        if name_at == keyword {
            return None;
        }
        let mut name = name_at;
        loop {
            let word = self.word_end(name);
            if word == name {
                break;
            }
            name = if self.at(word) == b'.' {
                word + 1
            } else {
                word
            };
        }
        if name == name_at {
            return None;
        }
        self.cap(1, p, using);
        self.cap(2, keyword_at, keyword);
        self.cap(3, name_at, name);
        Some(name)
    }

    /// `^\s*+(?i)(function|filter|configuration|workflow)\s+(?:
    /// (global|local|script|private):)?([-._\p{L}\d]+)`: a function's
    /// declaration, group 1 around its keyword, group 2 around its scope and
    /// group 3 around its name.
    ///
    /// With a scope but no name after it, the scope is read as the name.
    pub(super) fn function(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let keyword_at = self.space_end(p);
        let keyword =
            self.first_folded(keyword_at, FUNCTIONS, |lexer, at| lexer.space_at(at) > 0)?;
        let scope_at = self.space_end(keyword);
        let scoped = self
            .first_folded(scope_at, FUNCTION_SCOPES, |lexer, at| lexer.at(at) == b':')
            .and_then(|scope| {
                let name = self.class_end(scope + 1, is_function_name);
                (name > scope + 1).then_some((scope, name))
            });
        let (name_at, name) = if let Some((scope, name)) = scoped {
            self.cap(2, scope_at, scope);
            (scope + 1, name)
        } else {
            let name = self.class_end(scope_at, is_function_name);
            if name == scope_at {
                return None;
            }
            (scope_at, name)
        };
        self.cap(1, keyword_at, keyword);
        self.cap(3, name_at, name);
        Some(name)
    }

    /// `(\[)\s*\b(?i)(cmdletbinding|…|supportswildcards)\b`: an attribute,
    /// group 1 around its bracket and group 2 around its name.
    pub(super) fn attribute(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'[' {
            return None;
        }
        let name_at = self.space_end(p + 1);
        if !self.boundary(name_at) {
            return None;
        }
        let name = self.folded_word(name_at, ATTRIBUTES)?;
        self.cap(1, p, p + 1);
        self.cap(2, name_at, name);
        Some(name)
    }

    /// `(?i)\b(mandatory|…|helpmessage)\b\s+{0,1}(=)?`: an attribute's named
    /// argument, group 1 around its name and group 2 around its `=`.
    pub(super) fn attribute_argument(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let name = self.folded_word(p, ATTRIBUTE_ARGUMENTS)?;
        self.cap(1, p, name);
        let equals = self.space_end(name);
        if self.at(equals) == b'=' {
            self.cap(2, equals, equals + 1);
            return Some(equals + 1);
        }
        Some(equals)
    }

    /// `(?<![-\w])((?i:class)|[%?])\s+([-_\p{L}\d]?{1,})\b`: a class's
    /// declaration, group 1 around its keyword and group 2 around its name.
    ///
    /// The name gives characters back until `\b` holds.
    /// It may give all of them back, when a word character that is not in
    /// the class follows the blanks.
    pub(super) fn class(&mut self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'-') || self.word_before(p) {
            return None;
        }
        let keyword = match self.at(p) {
            b'%' | b'?' => p + 1,
            _ => self.folded(p, b"class")?,
        };
        let name_at = self.space_end(keyword);
        if name_at == keyword {
            return None;
        }
        let mut name = self.class_end(name_at, is_class_name);
        while !self.boundary(name) {
            if name == name_at {
                return None;
            }
            name = self.char_start_before(name);
        }
        self.cap(1, p, keyword);
        self.cap(2, name_at, name);
        Some(name)
    }

    /// `(?<!\w)(--%)(?!\w)`: the stop-parsing token, group 1 around it.
    pub(super) fn stop_parsing(&mut self, p: usize) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        let end = self.lit(p, b"--%")?;
        if self.word_at(end).is_some() {
            return None;
        }
        self.cap(1, p, end);
        Some(end)
    }
}
