//! The grammar's regular expressions as hand-written matchers.
//!
//! Each matcher answers what its expression would at a position.
//! That includes where its backtracking lands.
//! `Onig`'s `\s`, `\S` and `\w` read non-ASCII characters.
//! Those classes go through the table the JSON lexer measured on Shiki.

use memchr::memchr;

use super::engine::{CaptureName, Hit, Lexer};
use super::rules::Pattern;
use super::scope::{BUILTINS, FUNCTIONS, S, SPECIAL_TARGETS};
use super::state::Rule;
use crate::classes;
use crate::classes::is_ascii_space;
use crate::utf8::char_len;

/// The text a `(` or `{` opens: the byte that closes it.
fn closer(open: u8) -> u8 {
    if open == b'(' { b')' } else { b'}' }
}

impl Lexer<'_> {
    // Characters.

    /// The character at `i` and its width, where one starts in the text.
    fn char_width(&self, i: usize) -> Option<(char, usize)> {
        if i >= self.lim() {
            return None;
        }
        let width = char_len(self.s[i]);
        let text = std::str::from_utf8(self.s.get(i..i + width)?).ok()?;
        Some((text.chars().next()?, width))
    }

    /// The width of the white space character at `i`, 0 when there is none.
    /// The line feed at the end of a line is white space.
    fn space_w(&self, i: usize) -> usize {
        if i < self.lim() {
            let b = self.s[i];
            if b < 0x80 {
                return usize::from(is_ascii_space(b));
            }
            return match self.char_width(i) {
                Some((c, width)) if classes::is_space(c) => width,
                _ => 0,
            };
        }
        usize::from(i == self.lim() && self.nl)
    }

    /// The width of the character at `i` that `\S` matches, 0 when none.
    fn nonspace_w(&self, i: usize) -> usize {
        if i >= self.lim() {
            return 0;
        }
        match self.space_w(i) {
            0 => char_len(self.s[i]),
            _ => 0,
        }
    }

    /// Whether the character at `i` is a word character, as `\w` reads it.
    fn word_at(&self, i: usize) -> bool {
        if i >= self.lim() {
            return false;
        }
        let b = self.s[i];
        if b < 0x80 {
            return b.is_ascii_alphanumeric() || b == b'_';
        }
        self.char_width(i).is_some_and(|(c, _)| classes::is_word(c))
    }

    /// The end of the white space from `i`, the regex `\s*`.
    fn spaces(&self, mut i: usize) -> usize {
        let from = i;
        while let width @ 1.. = self.space_w(i) {
            i += width;
        }
        self.spend(i - from);
        i
    }

    /// The end of the run of `[ ]` from `i`.
    fn blanks(&self, mut i: usize) -> usize {
        let from = i;
        while i < self.lim() && self.s[i] == b' ' {
            i += 1;
        }
        self.spend(i - from);
        i
    }

    /// The end of the run of characters that `\S` matches from `i`.
    fn word_end(&self, mut i: usize) -> usize {
        let from = i;
        while let width @ 1.. = self.nonspace_w(i) {
            i += width;
        }
        self.spend(i - from);
        i
    }

    /// `\b` after a word that ends at `i`.
    fn boundary_after(&self, i: usize) -> bool {
        !self.word_at(i)
    }

    /// `(?<!\\)\n`: the line feed at `i`, not after a backslash.
    fn plain_newline(&self, i: usize) -> bool {
        i == self.lim() && self.nl && self.prev(i) != Some(b'\\')
    }

    // Ends.

    /// The end pattern of `rule` at `p`.
    ///
    /// The arms match, in order:
    /// - `Comment`: `(?!\G)`, anywhere `\G` does not;
    /// - `CommentHash` and `Prereq`: `(?=[^\\])$`;
    /// - the one-line heads, from `Include` to `Else`: `^`;
    /// - `Define` and `Conditional`: their closing keyword;
    /// - `Target` and `Recipe`: `[^\\]$`;
    /// - `TargetName`: `(?=\s|$)`;
    /// - `Assign`: `\n`;
    /// - `VarParen` and `VarBrace`: `\)|((?<!\\)\n)`, with `}` for a brace;
    /// - the function calls and their inner groups: `(?=\)|((?<!\\)\n))`, with
    ///   `}` for a brace;
    /// - `FlavorParen` and `FlavorBrace`: `(?=\))`, with `}` for a brace;
    /// - `InterpParen` and `InterpBrace`: `\)`, with `}` for a brace.
    #[expect(
        clippy::inline_always,
        reason = "measured: the matchers are the hot loop"
    )]
    #[inline(always)]
    pub(super) fn end_at(&self, rule: Rule, p: usize) -> Option<Hit> {
        let zero = || Hit::new(p, p);
        match rule {
            Rule::Root | Rule::RetokTarget | Rule::RetokVars => None,
            Rule::Comment => (!self.g || p != self.pos).then(zero),
            Rule::CommentHash | Rule::Prereq => (p == self.lim() && self.nl).then(zero),
            Rule::Include
            | Rule::Vpath
            | Rule::Export
            | Rule::OverridePrivate
            | Rule::Unexport
            | Rule::DefineHead
            | Rule::Condition
            | Rule::Else => (p == 0).then(zero),
            Rule::Define => self.end_keyword(p, b"endef", S::KwOverride),
            Rule::Conditional => self.end_keyword(p, b"endif", S::KwEndif),
            Rule::Target | Rule::Recipe => {
                if !self.avail(p) || self.at(p) == b'\\' {
                    return None;
                }
                let width = if p < self.lim() {
                    char_len(self.s[p])
                } else {
                    1
                };
                self.at_eol(p + width).then(|| Hit::new(p, p + width))
            }
            Rule::TargetName => (self.space_w(p) > 0 || self.at_eol(p)).then(zero),
            Rule::Assign => (p == self.lim() && self.nl).then(|| Hit::new(p, p + 1)),
            Rule::VarParen | Rule::VarBrace => {
                let close = if rule == Rule::VarParen { b')' } else { b'}' };
                ((p < self.lim() && self.s[p] == close) || self.plain_newline(p))
                    .then(|| Hit::new(p, p + 1).capture(0, p, p + 1, S::PunctVar))
            }
            Rule::FuncParen | Rule::AnotherParen => {
                ((p < self.lim() && self.s[p] == b')') || self.plain_newline(p)).then(zero)
            }
            Rule::FuncBrace | Rule::AnotherBrace => {
                ((p < self.lim() && self.s[p] == b'}') || self.plain_newline(p)).then(zero)
            }
            Rule::FlavorParen => (p < self.lim() && self.s[p] == b')').then(zero),
            Rule::FlavorBrace => (p < self.lim() && self.s[p] == b'}').then(zero),
            Rule::InterpParen => (p < self.lim() && self.s[p] == b')').then(|| Hit::new(p, p + 1)),
            Rule::InterpBrace => (p < self.lim() && self.s[p] == b'}').then(|| Hit::new(p, p + 1)),
        }
    }

    /// `^\s*(word)\b`, the end of `define` and of a conditional.
    fn end_keyword(&self, p: usize, word: &[u8], scope: S) -> Option<Hit> {
        if p != 0 {
            return None;
        }
        let from = self.spaces(0);
        let end = from + word.len();
        (self.starts(from, word) && self.boundary_after(end))
            .then(|| Hit::new(0, end).capture(0, from, end, scope))
    }

    // Patterns.

    /// The match of `pattern` at `p`.
    /// The end pattern has no match here, because it depends on the rule.
    /// `Grammar::matcher` answers it.
    #[expect(
        clippy::inline_always,
        reason = "measured: the matchers are the hot loop"
    )]
    #[inline(always)]
    pub(super) fn pattern_at(&self, pattern: Pattern, p: usize) -> Option<Hit> {
        match pattern {
            Pattern::End => None,
            Pattern::SimpleVar => self.simple_var(p),
            Pattern::Comma => self.single(p, b','),
            Pattern::Percent => self.single(p, b'%'),
            Pattern::PercentStar => self.single(p, b'%').or_else(|| self.single(p, b'*')),
            Pattern::Continuation => {
                (p + 1 == self.lim() && self.nl && self.s[p] == b'\\').then(|| Hit::new(p, p + 2))
            }
            Pattern::AnyWord => {
                let end = self.word_end(p);
                (end > p).then(|| Hit::new(p, end))
            }
            Pattern::BuiltinParen => self.builtin(p, b'('),
            Pattern::BuiltinBrace => self.builtin(p, b'{'),
            Pattern::SpecialTarget => self.special_target(p),
            Pattern::Comment => self.comment(p),
            Pattern::CommentHash => (p < self.lim() && self.s[p] == b'#')
                .then(|| Hit::new(p, p + 1).capture(0, p, p + 1, S::PunctComment)),
            Pattern::VarParen => self.dollar(p, b'('),
            Pattern::VarBrace => self.dollar(p, b'{'),
            Pattern::FuncParen => self.function(p, b'('),
            Pattern::FuncBrace => self.function(p, b'{'),
            Pattern::FlavorParen => self.flavor(p, b'('),
            Pattern::FlavorBrace => self.flavor(p, b'{'),
            Pattern::AnotherParen => self.another(p, b'('),
            Pattern::AnotherBrace => self.another(p, b'{'),
            Pattern::InterpParen => self.single(p, b'('),
            Pattern::InterpBrace => self.single(p, b'{'),
            Pattern::Include => self.keyword_line(p, true),
            Pattern::Vpath => self.keyword_line(p, false),
            Pattern::Define => self.define(p),
            Pattern::DefineHead => {
                (self.at_g(p) && !(p == self.lim() && self.nl)).then(|| Hit::new(p, p))
            }
            Pattern::Export => self.leading_word(p, &[(b"export", S::KwExport)]),
            Pattern::OverridePrivate => self.leading_word(
                p,
                &[(b"override", S::KwOverride), (b"private", S::KwPrivate)],
            ),
            Pattern::Unexport => self.leading_word(
                p,
                &[(b"unexport", S::KwUnexport), (b"undefine", S::KwUndefine)],
            ),
            Pattern::Conditional => self.conditional(p),
            Pattern::Condition | Pattern::Prereq => self.at_g(p).then(|| Hit::new(p, p)),
            Pattern::Else => self.else_line(p),
            Pattern::Target => self.target(p),
            Pattern::TargetName => (self.nonspace_w(p) > 0).then(|| Hit::new(p, p)),
            Pattern::Recipe => self.recipe(p),
            Pattern::Assign => self.assign(p),
        }
    }

    /// A match of the one byte `b`.
    fn single(&self, p: usize, b: u8) -> Option<Hit> {
        (p < self.lim() && self.s[p] == b).then(|| Hit::new(p, p + 1))
    }

    /// `\$[^(){}]`
    fn simple_var(&self, p: usize) -> Option<Hit> {
        if p >= self.lim() || self.s[p] != b'$' || !self.avail(p + 1) {
            return None;
        }
        let width = if p + 1 < self.lim() {
            if matches!(self.s[p + 1], b'(' | b')' | b'{' | b'}') {
                return None;
            }
            char_len(self.s[p + 1])
        } else {
            1
        };
        Some(Hit::new(p, p + 1 + width))
    }

    /// `\$\(` and `\${`: the rule's begin is a group that adds the scope of
    /// the variable's punctuation.
    fn dollar(&self, p: usize, open: u8) -> Option<Hit> {
        (p + 1 < self.lim() && self.s[p] == b'$' && self.s[p + 1] == open)
            .then(|| Hit::new(p, p + 2).capture(0, p, p + 2, S::PunctVar))
    }

    /// `(?<=\()(MAKEFILES|...)(?=\s*\))`
    fn builtin(&self, p: usize, open: u8) -> Option<Hit> {
        if self.prev(p) != Some(open) {
            return None;
        }
        BUILTINS.iter().find_map(|word| {
            let end = p + word.len();
            (self.starts(p, word.as_bytes()) && self.at(self.spaces(end)) == closer(open))
                .then(|| Hit::new(p, end))
        })
    }

    /// `(?<=\()(subst|...)\s`: the word has the scope of its function.
    fn function(&self, p: usize, open: u8) -> Option<Hit> {
        if self.prev(p) != Some(open) {
            return None;
        }
        let run = self.s[p.min(self.lim())..self.lim()]
            .iter()
            .take_while(|b| b.is_ascii_lowercase() || **b == b'-')
            .count();
        self.spend(run);
        let word = &self.s[p..p + run];
        let &(_, scope) = FUNCTIONS.iter().find(|(name, _)| name.as_bytes() == word)?;
        let end = p + run;
        let blank = self.space_w(end);
        (blank > 0).then(|| Hit::new(p, end + blank).capture(0, p, end, scope))
    }

    /// `(?<=\()(origin|flavor)\s(?=[^\s)]+\s*\))`
    fn flavor(&self, p: usize, open: u8) -> Option<Hit> {
        if self.prev(p) != Some(open) {
            return None;
        }
        let (len, scope) = if self.starts(p, b"origin") {
            (6, S::FnOrigin)
        } else if self.starts(p, b"flavor") {
            (6, S::FnFlavor)
        } else {
            return None;
        };
        let end = p + len;
        let blank = self.space_w(end);
        if blank == 0 {
            return None;
        }
        let close = closer(open);
        let mut i = end + blank;
        let from = i;
        while i < self.lim() && self.nonspace_w(i) > 0 && self.s[i] != close {
            i += char_len(self.s[i]);
        }
        self.spend(i - from);
        (i > from && self.at(self.spaces(i)) == close)
            .then(|| Hit::new(p, end + blank).capture(0, p, end, scope))
    }

    /// `(?<=\()(?!\))`
    fn another(&self, p: usize, open: u8) -> Option<Hit> {
        (self.prev(p) == Some(open) && self.at(p) != closer(open)).then(|| Hit::new(p, p))
    }

    /// `(^|\G)`... comment: `(^[ ]+)?((?<!\\)(\\\\)*)(?=#)`
    fn comment(&self, p: usize) -> Option<Hit> {
        if p == 0 && self.at(0) == b' ' {
            let from = self.blanks(0);
            return self
                .even_backslashes(from)
                .map(|end| Hit::new(0, end).capture(0, 0, from, S::LeadingSpace));
        }
        if p > 0 && self.prev(p) == Some(b'\\') {
            return None;
        }
        self.even_backslashes(p).map(|end| Hit::new(p, end))
    }

    /// The end of the backslashes at `from` when they are an even number and a
    /// `#` follows.
    fn even_backslashes(&self, from: usize) -> Option<usize> {
        let mut i = from;
        while i < self.lim() && self.s[i] == b'\\' {
            i += 1;
        }
        self.spend(i - from);
        ((i - from).is_multiple_of(2) && i < self.lim() && self.s[i] == b'#').then_some(i)
    }

    /// `^[ ]*([s\-]?include)\b` and `^[ ]*(vpath)\b`.
    fn keyword_line(&self, p: usize, include: bool) -> Option<Hit> {
        if p != 0 {
            return None;
        }
        let from = self.blanks(0);
        let (word, scope): (&[u8], S) = if include {
            (b"include", S::KwInclude)
        } else {
            (b"vpath", S::KwVpath)
        };
        let word_start = if include && matches!(self.at(from), b's' | b'-') {
            from + 1
        } else {
            from
        };
        let end = word_start + word.len();
        (self.starts(word_start, word) && self.boundary_after(end))
            .then(|| Hit::new(0, end).capture(0, from, end, scope))
    }

    /// `^[ ]*(export)\b` and the like: the first of `words` that fits.
    fn leading_word(&self, p: usize, words: &[(&[u8], S)]) -> Option<Hit> {
        if p != 0 {
            return None;
        }
        let from = self.blanks(0);
        words.iter().find_map(|&(word, scope)| {
            let end = from + word.len();
            (self.starts(from, word) && self.boundary_after(end))
                .then(|| Hit::new(0, end).capture(0, from, end, scope))
        })
    }

    /// `^\s*(?:(override)\s*)?(define)\s*([^\s]+)\s*(=|\?=|:=|\+=)?(?=\s)`
    ///
    /// The `\s*` after the name gives back one character at a time.
    /// It stops when the operator, or nothing, is followed by white space.
    /// The last place that works therefore wins.
    /// One pass keeps it, with nothing collected.
    fn define(&self, p: usize) -> Option<Hit> {
        if p != 0 {
            return None;
        }
        let lead = self.spaces(0);
        let mut over = None;
        let mut at = lead;
        if self.starts(lead, b"override") {
            over = Some((lead, lead + 8));
            at = self.spaces(lead + 8);
        }
        if !self.starts(at, b"define") {
            return None;
        }
        let define_end = at + 6;
        let name = self.spaces(define_end);
        let name_end = self.word_end(name);
        if name_end == name {
            return None;
        }
        let at_stop = |stop: usize| {
            let width = self.operator(stop, false);
            if width > 0 && self.space_w(stop + width) > 0 {
                return Some((stop + width, Some((stop, stop + width))));
            }
            (self.space_w(stop) > 0).then_some((stop, None))
        };
        let mut found = at_stop(name_end);
        let mut i = name_end;
        while let width @ 1.. = self.space_w(i) {
            i += width;
            found = at_stop(i).or(found);
        }
        let (end, op) = found?;
        let mut hit = Hit::new(0, end);
        if let Some((from, to)) = over {
            hit = hit.capture(0, from, to, S::KwOverride);
        }
        hit = hit.capture(1, at, define_end, S::KwDefine);
        hit = hit.capture(2, name, name_end, S::VarOther);
        if let Some((from, to)) = op {
            hit = hit.capture(3, from, to, S::KeyValue);
        }
        Some(hit)
    }

    /// The width of the assignment operator at `i`: `=`, `?=`, `:=` or `+=`.
    /// With `bang`, it also takes `!=`, and `=` not after one of `?:+!`.
    fn operator(&self, i: usize, assign: bool) -> usize {
        if assign {
            if self.at(i) == b'=' && !matches!(self.prev(i), Some(b'?' | b':' | b'+' | b'!')) {
                return 1;
            }
            if i + 1 < self.lim()
                && self.s[i + 1] == b'='
                && matches!(self.s[i], b'?' | b':' | b'+' | b'!')
            {
                return 2;
            }
            return 0;
        }
        if self.at(i) == b'=' {
            return 1;
        }
        if i + 1 < self.lim() && self.s[i + 1] == b'=' && matches!(self.s[i], b'?' | b':' | b'+') {
            return 2;
        }
        0
    }

    /// `^\s*(ifeq|ifneq|ifdef|ifndef)(?=\s)`
    fn conditional(&self, p: usize) -> Option<Hit> {
        if p != 0 {
            return None;
        }
        let from = self.spaces(0);
        let words: [(&[u8], S); 4] = [
            (b"ifeq", S::KwIfeq),
            (b"ifneq", S::KwIfneq),
            (b"ifdef", S::KwIfdef),
            (b"ifndef", S::KwIfndef),
        ];
        words.iter().find_map(|&(word, scope)| {
            let end = from + word.len();
            (self.starts(from, word) && self.space_w(end) > 0)
                .then(|| Hit::new(0, end).capture(0, from, end, scope))
        })
    }

    /// `^\s*else(?=\s)\s*(ifeq|ifneq|ifdef|ifndef)*(?=\s)`
    ///
    /// The repetition takes as many conditional words as it can.
    /// It gives them back one at a time.
    /// `\s*` gives back its characters the same way.
    /// The last end that white space follows therefore wins.
    /// One pass keeps it, with nothing collected.
    fn else_line(&self, p: usize) -> Option<Hit> {
        if p != 0 {
            return None;
        }
        let from = self.spaces(0);
        let word_end = from + 4;
        if !self.starts(from, b"else") || self.space_w(word_end) == 0 {
            return None;
        }
        let last_end = |stop: usize| {
            let mut last = (self.space_w(stop) > 0).then_some(stop);
            let mut at = stop;
            while let Some(len) = [&b"ifeq"[..], b"ifneq", b"ifdef", b"ifndef"]
                .iter()
                .find(|word| self.starts(at, word))
                .map(|word| word.len())
            {
                at += len;
                if self.space_w(at) > 0 {
                    last = Some(at);
                }
            }
            last
        };
        let mut found = last_end(word_end);
        let mut i = word_end;
        while let width @ 1.. = self.space_w(i) {
            i += width;
            found = last_end(i).or(found);
        }
        let end = found?;
        Some(Hit::new(0, end).capture(0, 0, end, S::KwElse))
    }

    /// `^(?!\t)([^:]*)(:)(?!\=)`: the targets are lexed again.
    fn target(&self, p: usize) -> Option<Hit> {
        if p != 0 || self.at(0) == b'\t' {
            return None;
        }
        self.spend(self.lim());
        let colon = memchr(b':', &self.s[..self.lim()])?;
        if self.at(colon + 1) == b'=' {
            return None;
        }
        let hit = Hit::new(0, colon + 1);
        let hit = hit.group(0, 0, colon, CaptureName::None, Some(Rule::RetokTarget));
        Some(hit.capture(1, colon, colon + 1, S::KeyValue))
    }

    /// `^\s*(\.(PHONY|...))\s*$`
    fn special_target(&self, p: usize) -> Option<Hit> {
        if p != 0 {
            return None;
        }
        let from = self.spaces(0);
        SPECIAL_TARGETS.iter().find_map(|&(name, scope)| {
            let end = from + name.len();
            (self.starts(from, name.as_bytes()) && self.at_eol(self.spaces(end)))
                .then(|| Hit::new(0, self.spaces(end)).capture(0, from, end, scope))
        })
    }

    /// `^\t([+\-@]*)`
    fn recipe(&self, p: usize) -> Option<Hit> {
        if p != 0 || self.at(0) != b'\t' {
            return None;
        }
        let mut end = 1;
        while end < self.lim() && matches!(self.s[end], b'+' | b'-' | b'@') {
            end += 1;
        }
        self.spend(end);
        Some(Hit::new(0, end).group(0, 1, end, CaptureName::Flags, None))
    }

    /// `(^[ ]*|\G\s*)([^\s:#=]+)\s*((?<![?:+!])=|\?=|:=|\+=|!=)`
    fn assign(&self, p: usize) -> Option<Hit> {
        if p == 0
            && let Some(hit) = self.assign_from(p, self.blanks(0))
        {
            return Some(hit);
        }
        if self.at_g(p) {
            return self.assign_from(p, self.spaces(p));
        }
        None
    }

    /// The assignment whose name starts at `from`.
    ///
    /// The name gives back characters, from the longest, until white space and
    /// an operator follow it.
    fn assign_from(&self, p: usize, from: usize) -> Option<Hit> {
        let mut run = from;
        while run < self.lim() {
            let width = self.nonspace_w(run);
            if width == 0 || matches!(self.s[run], b':' | b'#' | b'=') {
                break;
            }
            run += width;
        }
        self.spend(run - from);
        if run == from {
            return None;
        }
        let mut end = run;
        loop {
            let after = self.spaces(end);
            let width = self.operator(after, true);
            if width > 0 {
                return Some(
                    Hit::new(p, after + width)
                        .group(
                            0,
                            from,
                            end,
                            CaptureName::One(S::VarOther),
                            Some(Rule::RetokVars),
                        )
                        .capture(1, after, after + width, S::KeyValue),
                );
            }
            end -= 1;
            while end > from && self.s[end] & 0xc0 == 0x80 {
                end -= 1;
            }
            if end == from {
                return None;
            }
        }
    }
}
