//! Comments, and the Doxygen commands inside documentation comments.

use super::engine::Lexer;
use super::scope::S;
use super::state::Rule;
use crate::scope::{Scopes, scopes};

/// The words a Doxygen command may be, in the order the grammar lists them.
/// Each entry of a list stands for one alternative.
/// `em??` is `e` then `em`.
pub(super) const DOXY_BARE: &[&str] = &[
    "callergraph",
    "callgraph",
    "else",
    "endif",
    "f$",
    "f[",
    "f]",
    "hidecallergraph",
    "hidecallgraph",
    "hiderefby",
    "hiderefs",
    "hideinitializer",
    "htmlinclude",
    "n",
    "nosubgrouping",
    "private",
    "privatesection",
    "protected",
    "protectedsection",
    "public",
    "publicsection",
    "pure",
    "showinitializer",
    "showrefby",
    "showrefs",
    "tableofcontents",
    "\"",
    "#",
    "$",
    "%",
    ".",
    "<",
    "=",
    ">",
    "::",
    "|",
    "--",
    "---",
];

pub(super) const DOXY_A: &[&str] = &[
    "a",
    "anchor",
    "b",
    "c",
    "cite",
    "copybrief",
    "copydetail",
    "copydoc",
    "def",
    "dir",
    "dontinclude",
    "e",
    "em",
    "emoji",
    "enum",
    "example",
    "extends",
    "file",
    "idlexcept",
    "implements",
    "include",
    "includedoc",
    "includelineno",
    "latexinclude",
    "link",
    "memberof",
    "namespace",
    "p",
    "package",
    "ref",
    "refitem",
    "related",
    "relates",
    "relatedalso",
    "relatesalso",
    "verbinclude",
];

pub(super) const DOXY_B: &[&str] = &[
    "addindex",
    "addtogroup",
    "category",
    "class",
    "defgroup",
    "diafile",
    "dotfile",
    "elseif",
    "fn",
    "headerfile",
    "if",
    "ifnot",
    "image",
    "ingroup",
    "interface",
    "line",
    "mainpage",
    "mscfile",
    "name",
    "overload",
    "page",
    "property",
    "protocol",
    "section",
    "skip",
    "skipline",
    "snippet",
    "snippetdoc",
    "snippetlineno",
    "struct",
    "subpage",
    "subsection",
    "subsubsection",
    "typedef",
    "union",
    "until",
    "vhdlflow",
    "weakgroup",
];

pub(super) const DOXY_C: &[&str] = &[
    "arg",
    "attention",
    "author",
    "authors",
    "brief",
    "bug",
    "copyright",
    "date",
    "deprecated",
    "details",
    "exception",
    "invariant",
    "li",
    "note",
    "par",
    "paragraph",
    "param",
    "post",
    "pre",
    "remark",
    "remarks",
    "result",
    "return",
    "returns",
    "retval",
    "sa",
    "see",
    "short",
    "since",
    "test",
    "throw",
    "todo",
    "tparam",
    "version",
    "warning",
    "xrefitem",
];

pub(super) const DOXY_D: &[&str] = &[
    "code",
    "cond",
    "docbookonly",
    "dot",
    "htmlonly",
    "internal",
    "latexonly",
    "link",
    "manonly",
    "msc",
    "parblock",
    "rtfonly",
    "secreflist",
    "uml",
    "verbatim",
    "xmlonly",
    "endcode",
    "endcond",
    "enddocbookonly",
    "enddot",
    "endhtmlonly",
    "endinternal",
    "endlatexonly",
    "endlink",
    "endmanonly",
    "endmsc",
    "endparblock",
    "endrtfonly",
    "endsecreflist",
    "enduml",
    "endverbatim",
    "endxmlonly",
];

impl Lexer<'_> {
    /// `^(?>\s*)(//[!/]+)`: a documentation line comment.
    pub(super) fn doc_line(&mut self, p: usize) -> bool {
        if p != 0 {
            return false;
        }
        let q = self.skip_space(0);
        if !self.starts(q, b"//") {
            return false;
        }
        let mut e = q + 2;
        while matches!(self.b(e), b'!' | b'/') && e < self.len_all() {
            e += 1;
        }
        if e == q + 2 {
            return false;
        }
        self.span(q, e, scopes![S::PunctCommentDoc]);
        self.begin(p, e)
    }

    /// `(/\*[!*]+(?=\s))(.+)([!*]*\*/)`: a documentation comment that ends
    /// on its line.
    pub(super) fn doc_block_one(&mut self, p: usize) -> bool {
        if !self.starts(p, b"/*") {
            return false;
        }
        let mut k = p + 2;
        while matches!(self.b(k), b'!' | b'*') && k < self.len_all() {
            k += 1;
        }
        if k == p + 2 || self.space_at(k) == 0 {
            return false;
        }
        let Some(body) = self.text(k, self.lim()).get(1..) else {
            return false;
        };
        self.spend(body.len());
        let Some(at) = memchr::memmem::rfind(body, b"*/") else {
            return false;
        };
        let m = k + 1 + at;
        self.span(p, k, scopes![S::PunctCommentBeginDoc]);
        self.span_retok(k, m, S::NO_SCOPES, Rule::Doxygen);
        self.span(m, m + 2, scopes![S::PunctCommentEndDoc]);
        self.plain(p, m + 2)
    }

    /// `((?>\s*)/\*[!*]+(?:(?:\n|$)|(?=\s)))`: the start of a documentation
    /// block comment.
    pub(super) fn doc_block_begin(&mut self, p: usize) -> bool {
        let q = self.skip_space(p);
        if !self.starts(q, b"/*") {
            return false;
        }
        let mut e = q + 2;
        while matches!(self.b(e), b'!' | b'*') && e < self.len_all() {
            e += 1;
        }
        if e == q + 2 || !(e == self.lim() || self.space_at(e) > 0) {
            return false;
        }
        let end = if e == self.lim() && self.nl { e + 1 } else { e };
        self.span(p, end, scopes![S::PunctCommentBeginDoc]);
        self.begin(p, end)
    }

    /// The start of the whitespace run that ends at `end`, not before `floor`.
    fn space_run_start(&self, end: usize, floor: usize) -> usize {
        let mut g = end;
        while g > floor {
            match self.char_ending(g) {
                Some(c) if self.space_at(g - c.len_utf8()) > 0 => g -= c.len_utf8(),
                _ => break,
            }
        }
        g
    }

    /// `^/\* =(\s*.*?)\s*= \*/$\n?`.
    pub(super) fn banner_block(&mut self, p: usize) -> bool {
        if p != 0 || !self.starts(0, b"/* =") || !self.nl {
            return false;
        }
        let n = self.lim();
        if n < 8 || !self.text(n - 4, n).eq(b"= */") {
            return false;
        }
        let suffix = n - 4;
        let lead = self.skip_space(4).min(suffix);
        let end = lead.max(self.space_run_start(suffix, 4));
        self.span(4, end, scopes![S::MetaTocBlock]);
        self.plain(0, n + 1)
    }

    /// `^// =(\s*.*?)\s*=$\n?`.
    pub(super) fn banner_line(&mut self, p: usize) -> bool {
        if p != 0 || !self.starts(0, b"// =") || !self.nl {
            return false;
        }
        let n = self.lim();
        if n < 5 || self.b(n - 1) != b'=' {
            return false;
        }
        let suffix = n - 1;
        let lead = self.skip_space(4).min(suffix);
        let end = lead.max(self.space_run_start(suffix, 4));
        self.span(4, end, scopes![S::MetaTocLine]);
        self.plain(0, n + 1)
    }

    /// `(/\*)` and `\s*+(/\*)`: a block comment.
    pub(super) fn block_comment(&mut self, p: usize) -> bool {
        let q = self.skip_space(p);
        if !self.starts(q, b"/*") {
            return false;
        }
        self.span(q, q + 2, scopes![S::PunctCommentBegin]);
        self.begin(p, q + 2)
    }

    /// `((?:^[\t ]+)?)(?=//)`: the start of a line comment, with the
    /// indentation of the line it starts.
    pub(super) fn line_leading(&mut self, p: usize) -> bool {
        if self.starts(p, b"//") {
            return self.begin(p, p);
        }
        if p != 0 || !matches!(self.b(0), b' ' | b'\t') {
            return false;
        }
        let mut k = 0;
        while matches!(self.b(k), b' ' | b'\t') && k < self.len_all() {
            k += 1;
        }
        if !self.starts(k, b"//") {
            return false;
        }
        self.span(0, k, scopes![S::PunctWhitespaceLeading]);
        self.begin(0, k)
    }

    /// `(//)` inside the start of a line comment.
    pub(super) fn line_slash(&mut self, p: usize) -> bool {
        if !self.starts(p, b"//") {
            return false;
        }
        self.span(p, p + 2, scopes![S::PunctComment]);
        self.begin(p, p + 2)
    }

    /// `\s*+(//)`: a line comment.
    pub(super) fn line_comment(&mut self, p: usize) -> bool {
        let q = self.skip_space(p);
        if !self.starts(q, b"//") {
            return false;
        }
        self.span(q, q + 2, scopes![S::PunctComment]);
        self.begin(p, q + 2)
    }

    /// The end of a complete `/* … */` comment at `p`:
    /// `(/\*)((?>…)((?>\*+)/))`.
    /// It returns where the body ends, where the closing starts, and where the
    /// closing ends.
    pub(super) fn inline_comment_at(&self, p: usize) -> Option<(usize, usize)> {
        if !self.starts(p, b"/*") {
            return None;
        }
        let end = self.len_all();
        let mut i = p + 2;
        while i < end {
            if self.b(i) != b'*' {
                i += 1;
                continue;
            }
            let mut j = i;
            while j < end && self.b(j) == b'*' {
                j += 1;
            }
            if j < end && self.b(j) == b'/' {
                self.spend(j - p);
                return Some((i, j + 1));
            }
            if j >= end {
                self.spend(j - p);
                return None;
            }
            i = j + 1;
        }
        self.spend(i - p);
        None
    }

    /// `(/\*)((?>…)((?>\*+)/))`: a whole comment, in a capture lexed
    /// again.
    pub(super) fn inline_comment(&mut self, p: usize) -> bool {
        let Some((close, end)) = self.inline_comment_at(p) else {
            return false;
        };
        self.span(p, p + 2, scopes![S::CommentBlock, S::PunctCommentBegin]);
        self.span(p + 2, end, scopes![S::CommentBlock]);
        self.span_retok(close, end, S::NO_SCOPES, Rule::InlineClose);
        self.plain(p, end)
    }

    /// `(?<=[!*/\s])[@\\]`: where a Doxygen command starts.
    fn doxy_lead(&self, p: usize) -> Option<usize> {
        if !matches!(self.b(p), b'@' | b'\\') || p >= self.len_all() {
            return None;
        }
        let ok = match self.prev(p) {
            None => false,
            Some(b'!' | b'*' | b'/') => true,
            Some(_) => {
                let width = self.char_ending(p).map_or(1, char::len_utf8);
                self.space_at(p - width) > 0
            }
        };
        ok.then_some(p + 1)
    }

    /// The first of `list` that is at `q` and followed by `\b`.
    fn doxy_word(&self, q: usize, list: &[&str]) -> Option<usize> {
        list.iter().find_map(|word| {
            let end = q + word.len();
            (self.starts(q, word.as_bytes()) && self.boundary(end)).then_some(end)
        })
    }

    /// `(?:\{[^}]*})?` after a command.
    fn doxy_braces(&self, q: usize) -> usize {
        if self.b(q) != b'{' || q >= self.len_all() {
            return q;
        }
        let mut i = q + 1;
        while i < self.len_all() && self.b(i) != b'}' {
            i += 1;
        }
        self.spend(i - q);
        if i < self.len_all() { i + 1 } else { q }
    }

    /// A command that is one of `list`, as a plain match.
    pub(super) fn doxy_plain(&mut self, p: usize, list: &[&str]) -> bool {
        let Some(q) = self.doxy_lead(p) else {
            return false;
        };
        let Some(end) = self.doxy_word(q, list) else {
            return false;
        };
        let end = self.doxy_braces(end);
        self.plain(p, end)
    }

    /// A command of `words`, whitespace and the argument that follows.
    pub(super) fn doxy_arg(&mut self, p: usize, words: &[&str], scope: Scopes<S>) -> bool {
        let Some(cmd) = self.doxy_lead(p) else {
            return false;
        };
        for word in words {
            let word_end = cmd + word.len();
            if !self.starts(cmd, word.as_bytes()) {
                continue;
            }
            let arg_start = self.skip_space(word_end);
            if arg_start == word_end {
                continue;
            }
            let mut arg_end = arg_start;
            while arg_end < self.len_all() && self.space_at(arg_end) == 0 {
                arg_end += self.width_at(arg_end);
            }
            self.spend(arg_end - arg_start);
            if arg_end == arg_start {
                continue;
            }
            self.span(p, word_end, scopes![S::StorageDoxygen]);
            self.span(arg_start, arg_end, scope);
            return self.plain(p, arg_end);
        }
        false
    }

    /// `((?<=[!*/\s])[@\\]param)(?:\s*\[((?:,?\s*(?:in|out)\s*)+)])?\s+\b(\w+)\
    /// b`.
    pub(super) fn doxy_param(&mut self, p: usize) -> bool {
        let Some(q) = self.doxy_lead(p) else {
            return false;
        };
        if !self.starts(q, b"param") {
            return false;
        }
        let e = q + 5;
        if let Some((open, close, after)) = self.param_directions(e)
            && let Some((ws_end, word_end)) = self.param_name(after)
        {
            self.span(p, e, scopes![S::StorageDoxygen]);
            self.span_retok(open, close, S::NO_SCOPES, Rule::ParameterDirections);
            self.span(ws_end, word_end, scopes![S::VariableParameter]);
            return self.plain(p, word_end);
        }
        let Some((ws_end, word_end)) = self.param_name(e) else {
            return false;
        };
        self.span(p, e, scopes![S::StorageDoxygen]);
        self.span(ws_end, word_end, scopes![S::VariableParameter]);
        self.plain(p, word_end)
    }

    /// `\s*\[((?:,?\s*(?:in|out)\s*)+)]`: where the directions start and
    /// end, and where the bracket ends.
    fn param_directions(&self, e: usize) -> Option<(usize, usize, usize)> {
        let s = self.skip_space(e);
        if self.b(s) != b'[' || s >= self.len_all() {
            return None;
        }
        let open = s + 1;
        let mut i = open;
        let mut count = 0;
        loop {
            let mut j = i;
            if self.b(j) == b',' && j < self.len_all() {
                j += 1;
            }
            j = self.skip_space(j);
            let word = if self.starts(j, b"in") {
                2
            } else if self.starts(j, b"out") {
                3
            } else {
                break;
            };
            i = self.skip_space(j + word);
            count += 1;
        }
        (count > 0 && self.b(i) == b']' && i < self.len_all()).then_some((open, i, i + 1))
    }

    /// `\s+\b(\w+)\b`: where the word starts and ends.
    fn param_name(&self, e: usize) -> Option<(usize, usize)> {
        let s = self.skip_space(e);
        if s == e || self.word_at(s) == 0 {
            return None;
        }
        Some((s, self.skip_word(s)))
    }

    /// One of the words `in` and `out` in a direction list, named by it.
    pub(super) fn param_dir(&mut self, p: usize) -> bool {
        for word in ["in", "out"] {
            if self.starts(p, word.as_bytes()) {
                self.capture_dyn(
                    p,
                    p + word.len(),
                    "keyword.other.parameter.direction.",
                    (p, p + word.len()),
                );
                return self.plain(p, p + word.len());
            }
        }
        false
    }

    /// `\b[A-Z]+:|@[_a-z]+:`.
    pub(super) fn gtkdoc(&mut self, p: usize) -> bool {
        if self.b(p) == b'@' {
            let mut e = p + 1;
            while matches!(self.b(e), b'_' | b'a'..=b'z') && e < self.len_all() {
                e += 1;
            }
            self.spend(e - p);
            return e > p + 1 && self.b(e) == b':' && self.plain(p, e + 1);
        }
        if !self.b(p).is_ascii_uppercase() || self.word_before(p) {
            return false;
        }
        let mut e = p;
        while self.b(e).is_ascii_uppercase() && e < self.len_all() {
            e += 1;
        }
        self.spend(e - p);
        self.b(e) == b':' && self.plain(p, e + 1)
    }

    /// `\*/` in a closing of an inline comment.
    pub(super) fn inline_end(&mut self, p: usize) -> bool {
        self.starts(p, b"*/") && self.plain(p, p + 2)
    }
}
