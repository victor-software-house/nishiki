//! Preprocessor directives: the conditional family, `#define` and the
//! directives that take the rest of their line.

use super::engine::{Head, Lexer};
use super::scope::S;
use super::state::R;
use super::wordset;
use crate::scope::{Scopes, scopes};

/// A directive line, `^\s*(#)\s*keyword\b`.
#[derive(Clone, Copy, Debug)]
pub(super) struct Dir {
    pub(super) hash: usize,
    /// Where the keyword starts and ends.
    pub(super) kw: (usize, usize),
}

/// What follows a conditional directive for it to be live or dead code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Look {
    No,
    /// `(?=\s*\(*\b0*1\b\)*\s*(?:$|//|/\*))`
    Enabled,
    /// `(?=\s*\(*\b0+\b\)*\s*(?:$|//|/\*))`
    Disabled,
}

/// Whether a conditional directive's begin scopes the directive as a
/// whole with `meta.preprocessor`, besides its keyword and `#`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Meta {
    Whole,
    Omitted,
}

/// The keywords of `#if`, `#ifdef` and `#ifndef`, the longest first.
const IF_FAMILY: &[&str] = &["ifndef", "ifdef", "if"];

impl Lexer<'_> {
    /// `^\s*(#)\s*KEYWORD\b` at the start of the line, for the first of
    /// `kws` that matches.
    pub(super) fn directive(&self, p: usize, kws: &[&str]) -> Option<Dir> {
        if p != 0 {
            return None;
        }
        let (hash, k) = match self.x.head.get() {
            Head::Plain => return None,
            Head::Directive { hash, word } => (hash, word),
            Head::Unknown => {
                let hash = self.skip_space(0);
                if hash >= self.len_all() || self.b(hash) != b'#' {
                    self.x.head.set(Head::Plain);
                    return None;
                }
                let word = self.skip_space(hash + 1);
                self.x.head.set(Head::Directive { hash, word });
                (hash, word)
            }
        };
        kws.iter().find_map(|kw| {
            let end = k + kw.len();
            (self.starts(k, kw.as_bytes()) && self.word_at(end) == 0)
                .then_some(Dir { hash, kw: (k, end) })
        })
    }

    /// Whether the lookahead after a conditional directive holds at `e`.
    fn look_ahead(&self, e: usize, look: Look) -> bool {
        if look == Look::No {
            return true;
        }
        let mut q = self.skip_space(e);
        let parens = q;
        while self.b(q) == b'(' && q < self.len_all() {
            q += 1;
        }
        self.spend(q - parens);
        if !self.boundary(q) {
            return false;
        }
        let mut r = q;
        while self.b(r) == b'0' && r < self.len_all() {
            r += 1;
        }
        self.spend(r - q);
        match look {
            Look::Enabled => {
                if self.b(r) != b'1' || r >= self.len_all() {
                    return false;
                }
                r += 1;
            }
            Look::Disabled => {
                if r == q {
                    return false;
                }
            }
            Look::No => {}
        }
        if !self.boundary(r) {
            return false;
        }
        let closes = r;
        while self.b(r) == b')' && r < self.len_all() {
            r += 1;
        }
        self.spend(r - closes);
        let s = self.skip_space(r);
        s >= self.len_all() || self.starts(s, b"//") || self.starts(s, b"/*")
    }

    /// The captures of a conditional directive: the whole directive, the
    /// word and the `#`.
    fn directive_caps(&mut self, dir: Dir, meta: Meta) {
        if meta == Meta::Whole {
            self.span(0, dir.kw.1, scopes![S::MetaPreprocessor]);
        }
        self.span(dir.hash, dir.kw.1, scopes![S::DirectiveConditional]);
        self.span(dir.hash, dir.hash + 1, scopes![S::PunctDirective]);
    }

    /// The begin of a conditional rule.
    pub(super) fn cond_begin(&mut self, p: usize, kws: &[&str], look: Look, meta: Meta) -> bool {
        let Some(dir) = self.directive(p, kws) else {
            return false;
        };
        if !self.look_ahead(dir.kw.1, look) {
            return false;
        }
        self.directive_caps(dir, meta);
        self.begin(0, dir.kw.1)
    }

    /// `#if`, `#ifdef` and `#ifndef`.
    pub(super) fn cond_if(&mut self, p: usize) -> bool {
        self.cond_begin(p, IF_FAMILY, Look::No, Meta::Whole)
    }

    /// The `#if` of a branch the grammar reads as live or dead.
    pub(super) fn cond_if_look(&mut self, p: usize, look: Look) -> bool {
        self.cond_begin(p, &["if"], look, Meta::Whole)
    }

    /// `^\s*#\s*(e(?:lse|lif|ndif))\b`.
    pub(super) fn stray(&mut self, p: usize) -> bool {
        let Some(dir) = self.directive(p, &["else", "elif", "endif"]) else {
            return false;
        };
        self.cap_dyn(0, dir.kw.1, "invalid.illegal.stray-", dir.kw);
        self.plain(0, dir.kw.1)
    }

    /// `^\s*#\s*if(n?def)?\b.*$`: a conditional inside a dead branch.
    pub(super) fn nest_begin(&mut self, p: usize) -> bool {
        self.directive(p, IF_FAMILY).is_some() && self.begin(0, self.lim())
    }

    /// A line feed that starts the body of a branch.
    pub(super) fn newline_begin(&mut self, p: usize) -> bool {
        self.nl && p == self.lim() && self.begin(p, p + 1)
    }

    /// Whether a comment starts at `p` that the head of a conditional line
    /// stops at: `//`, or `/*` that no line continuation follows.
    pub(super) fn comment_starts(&self, p: usize) -> bool {
        self.starts(p, b"//") || (self.starts(p, b"/*") && !self.continues_after(p + 2))
    }

    /// `.*\\\s*\n` from `q`: the line ends with a backslash.
    fn continues_after(&self, q: usize) -> bool {
        if !self.nl {
            return false;
        }
        let mut e = self.lim();
        while e > q {
            match self.char_ending(e) {
                Some(c) if self.space_at(e - c.len_utf8()) > 0 => e -= c.len_utf8(),
                _ => break,
            }
        }
        e > q && self.b(e - 1) == b'\\'
    }

    /// `\G(?=.)(?!/(?:/|\*(?!.*\\\s*\n)))`: the head of a conditional line.
    pub(super) fn head_begin(&mut self, p: usize) -> bool {
        self.at_g(p) && p < self.lim() && !self.comment_starts(p) && self.begin(p, p)
    }

    /// The end of the head of a conditional line: a comment, or the end of
    /// the line that no backslash continues.
    pub(super) fn head_end(&mut self, p: usize, continued: bool) -> bool {
        let line_end = self.nl && p == self.lim() && !(continued && self.prev(p) == Some(b'\\'));
        (self.comment_starts(p) || line_end) && self.end_at(p, p, None)
    }

    /// `\bdefined\b` with the lookahead that makes it a directive keyword.
    pub(super) fn cond_defined(&mut self, p: usize) -> bool {
        if !self.starts(p, b"defined") || self.word_before(p) || self.word_at(p + 7) > 0 {
            return false;
        }
        let e = p + 7;
        let s = self.skip_space(e);
        if s >= self.len_all() {
            return self.plain(p, s);
        }
        if self.defined_look(e) {
            return self.plain(p, e);
        }
        false
    }

    /// The lookahead after `defined`.
    fn defined_look(&self, e: usize) -> bool {
        let mut q = self.skip_space(e);
        let parens = q;
        while self.b(q) == b'(' && q < self.len_all() {
            q += 1;
        }
        self.spend(q - parens);
        q = self.skip_space(q);
        if self.starts(q, b"defined") && self.word_at(q + 7) == 0 {
            return false;
        }
        let c = self.b(q);
        if !(c.is_ascii_alphabetic() || c == b'_' || c == b'$') || q >= self.len_all() {
            return false;
        }
        let mut run = q + 1;
        while run < self.len_all() && (self.b(run) == b'$' || self.word_at(run) > 0) {
            run += self.width_at(run);
        }
        self.spend(run - q);
        let mut end = run;
        while end > q {
            if self.boundary(end) && self.defined_tail(end) {
                return true;
            }
            let Some(c) = self.char_ending(end) else {
                break;
            };
            end -= c.len_utf8();
        }
        false
    }

    /// `\s*\)*\s*(?:\n|//|/\*|[:?]|&&|\|\||\\\s*\n)` from `i`.
    fn defined_tail(&self, i: usize) -> bool {
        let mut j = self.skip_space(i);
        let closes = j;
        while self.b(j) == b')' && j < self.len_all() {
            j += 1;
        }
        self.spend(j - closes);
        let s = self.skip_space(j);
        if s > self.lim() && self.nl {
            return true;
        }
        if self.starts(s, b"//")
            || self.starts(s, b"/*")
            || matches!(self.b(s), b':' | b'?') && s < self.len_all()
            || self.starts(s, b"&&")
            || self.starts(s, b"||")
        {
            return true;
        }
        self.b(s) == b'\\' && s < self.len_all() && self.skip_space(s + 1) > self.lim()
    }

    /// `\bdefined\b` that no directive keyword lookahead takes.
    pub(super) fn cond_defined_invalid(&mut self, p: usize) -> bool {
        self.starts(p, b"defined")
            && !self.word_before(p)
            && self.word_at(p + 7) == 0
            && self.plain(p, p + 7)
    }

    /// `[$A-Z_a-z][$\w]*`, with `extra` the other characters the name has.
    fn name_end(&self, p: usize, extra: u8) -> Option<usize> {
        let c = self.b(p);
        if p >= self.len_all() || !(c.is_ascii_alphabetic() || c == b'_' || c == b'$') {
            return None;
        }
        let mut e = p + 1;
        while e < self.len_all() && (self.b(e) == b'$' || self.b(e) == extra || self.word_at(e) > 0)
        {
            e += self.width_at(e);
        }
        self.spend(e - p);
        Some(e)
    }

    /// A macro name, with `scope`.
    pub(super) fn macro_name(&mut self, p: usize, extra: u8) -> bool {
        match self.name_end(p, extra) {
            Some(end) => self.plain(p, end),
            None => false,
        }
    }

    /// A one-word directive with `scope` on its word:
    /// `^\s*((#)\s*word)\b`.
    pub(super) fn line_directive(&mut self, p: usize, word: &str, scope: Scopes<S>) -> bool {
        let Some(dir) = self.directive(p, &[word]) else {
            return false;
        };
        self.span(dir.hash, dir.kw.1, scope);
        self.span(dir.hash, dir.hash + 1, scopes![S::PunctDirective]);
        self.begin(0, dir.kw.1)
    }

    /// `^\s*((#)\s*(error|warning))\b\s*` and the include directives.
    /// Each directive's word names the scope.
    pub(super) fn named_directive(&mut self, p: usize, kws: &[&str], prefix: &'static str) -> bool {
        let Some(dir) = self.directive(p, kws) else {
            return false;
        };
        self.cap_dyn(dir.hash, dir.kw.1, prefix, dir.kw);
        self.span(dir.hash, dir.hash + 1, scopes![S::PunctDirective]);
        let end = self.skip_space(dir.kw.1);
        self.begin(0, end)
    }

    /// `^\s*(((#)\s*pragma\s+mark)\s+(.*))`.
    pub(super) fn pragma_mark(&mut self, p: usize) -> bool {
        let Some(dir) = self.directive(p, &["pragma"]) else {
            return false;
        };
        let after = dir.kw.1;
        let m = self.skip_space(after);
        if m == after || !self.starts(m, b"mark") {
            return false;
        }
        let mark_end = m + 4;
        let t = self.skip_space(mark_end);
        if t == mark_end {
            return false;
        }
        let mut end = t;
        while end < self.lim() {
            end += self.width_at(end);
        }
        self.spend(end - t);
        let text_start = t.min(end);
        self.span(dir.hash, end, scopes![S::MetaPreprocessorPragma]);
        self.span(dir.hash, mark_end, scopes![S::DirectivePragmaMark]);
        self.span(dir.hash, dir.hash + 1, scopes![S::PunctDirective]);
        self.span(text_start, end, scopes![S::EntityTagPragmaMark]);
        self.plain(0, end.max(t))
    }

    /// The prefix of a `#define`: white space and comments, as the
    /// pattern's lazy loop takes them, and the rest of the directive.
    pub(super) fn define_begin(&mut self, p: usize) -> bool {
        let mut q = p;
        loop {
            let next = if self.space_at(q) > 0 {
                self.skip_space(q)
            } else if let Some((_, end)) = self.inline_comment_at(q) {
                end
            } else {
                break;
            };
            q = next;
            if self.define_rest(p, q) {
                return true;
            }
        }
        self.b(p) == b'#' && self.define_rest(p, p)
    }

    /// `((#)\s*define)\b\s+(name)(?:(\()([^()\\]+)(\)))?` at `q`, after a
    /// prefix that starts at `p`.
    fn define_rest(&mut self, p: usize, q: usize) -> bool {
        if self.b(q) != b'#' || q >= self.len_all() {
            return false;
        }
        let keyword = self.skip_space(q + 1);
        if !self.starts(keyword, b"define") || self.word_at(keyword + 6) > 0 {
            return false;
        }
        let keyword_end = keyword + 6;
        let ws = self.skip_space(keyword_end);
        if ws == keyword_end {
            return false;
        }
        let Some(name_end) = self.ident_end(ws) else {
            return false;
        };
        let mut end = name_end;
        let mut params = None;
        if self.b(name_end) == b'(' && name_end < self.len_all() {
            let mut close = name_end + 1;
            while close < self.len_all() && !matches!(self.b(close), b'(' | b')' | b'\\') {
                close += 1;
            }
            self.spend(close - name_end);
            if close > name_end + 1 && self.b(close) == b')' && close < self.len_all() {
                params = Some((name_end + 1, close));
                end = close + 1;
            }
        }
        self.span_retok(p, q, S::NO_SCOPES, R::Inline);
        self.span(q, keyword_end, scopes![S::DirectiveDefine]);
        self.span(q, q + 1, scopes![S::PunctDirective]);
        self.span(ws, name_end, scopes![S::EntityFunctionPre]);
        if let Some((open, close)) = params {
            self.span(open - 1, open, scopes![S::PunctParamsBegin]);
            self.span_retok(open, close, S::NO_SCOPES, R::DefineParams);
            self.span(close, close + 1, scopes![S::PunctParamsEnd]);
        }
        self.begin(p, end)
    }

    /// `(?<=[(,])\s*(name)\s*` in the parameters of a macro.
    pub(super) fn define_param(&mut self, p: usize) -> bool {
        if !matches!(self.prev(p), Some(b'(' | b',')) {
            return false;
        }
        let q = self.skip_space(p);
        let Some(end) = self.ident_end(q) else {
            return false;
        };
        let r = self.skip_space(end);
        self.span(q, end, scopes![S::VariableParamPre]);
        self.plain(p, r)
    }

    /// The lookahead that starts a call inside a macro:
    /// `(?!(?:…)\s*\()(?=name\s*\()`.
    pub(super) fn define_func(&mut self, p: usize) -> bool {
        self.call_target(p, wordset::DEFINE_EXCL).is_some() && self.begin(p, p)
    }

    /// A `{`, with `S::PunctBlockBegin` on it.
    pub(super) fn brace(&mut self, p: usize) -> bool {
        if self.b(p) != b'{' || p >= self.len_all() {
            return false;
        }
        self.span(p, p + 1, scopes![S::PunctBlockBegin]);
        self.begin(p, p + 1)
    }

    /// A `(`, with `S::PunctParensBegin` on it.
    pub(super) fn paren(&mut self, p: usize) -> bool {
        if self.b(p) != b'(' || p >= self.len_all() {
            return false;
        }
        self.span(p, p + 1, scopes![S::PunctParensBegin]);
        self.begin(p, p + 1)
    }

    /// A single `?` that opens a ternary of a conditional line.
    pub(super) fn ternary_begin(&mut self, p: usize) -> bool {
        if self.b(p) != b'?' || p >= self.len_all() {
            return false;
        }
        self.span(p, p + 1, scopes![S::OpTernary]);
        self.begin(p, p + 1)
    }

    /// `[^"']`: any one character but a quote.
    pub(super) fn any_but_quote(&mut self, p: usize) -> bool {
        if p >= self.len_all() || matches!(self.b(p), b'"' | b'\'') {
            return false;
        }
        self.begin(p, p + self.width_at(p))
    }

    /// `<`: the start of a header name.
    pub(super) fn angle_begin(&mut self, p: usize) -> bool {
        if self.b(p) != b'<' || p >= self.len_all() {
            return false;
        }
        self.span(p, p + 1, scopes![S::PunctStringBegin]);
        self.begin(p, p + 1)
    }
}
