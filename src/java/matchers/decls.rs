//! The declarations: classes, records, enums, modules, methods and
//! variables, whose rules open on a lookahead over the rest of the line.

use crate::java::engine::Lexer;
use crate::rules::Chars;

/// The declaration whose keyword [`Lexer::keyword_ahead`] looks for.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Decl {
    /// `class`, `interface` or `enum`, in a run that may hold hyphens.
    Class,
    /// `record`.
    Record,
}

/// What [`Lexer::method_ahead`] looks for.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Callable {
    /// A method: a `(` ahead.
    Method,
    /// A record's constructor: a `{` ahead, with no `(` before it.
    Constructor,
}

impl Lexer<'_> {
    /// Whether the word at `j` is a declaration keyword that a name follows:
    /// `\b(?:class|(?<!@)interface|enum)\s+[$\w]+`.
    fn class_keyword(&self, j: usize) -> bool {
        if !self.boundary(j) {
            return false;
        }
        let end = if let Some(end) = self.lit(j, b"class") {
            end
        } else if let Some(end) = self.lit(j, b"interface") {
            if self.prev_is(j, b'@') {
                return false;
            }
            end
        } else if let Some(end) = self.lit(j, b"enum") {
            end
        } else {
            return false;
        };
        self.name_follows(end)
    }

    /// `\brecord\s+[$\w]+` at `j`.
    fn record_keyword(&self, j: usize) -> bool {
        self.boundary(j)
            && self
                .lit(j, b"record")
                .is_some_and(|end| self.name_follows(end))
    }

    /// `\s+[$\w]+` at `i`.
    fn name_follows(&self, i: usize) -> bool {
        let start = self.space_end(i);
        start > i && self.name_end(start) > start
    }

    /// `(?=\w?[-\w\s]*\bKEYWORD...)`: a keyword in the run of word
    /// characters, blanks and (for a class) hyphens that starts at `p`.
    fn keyword_ahead(&self, p: usize, decl: Decl) -> bool {
        let memo = match decl {
            Decl::Class => &self.x.memo.keyword,
            Decl::Record => &self.x.memo.record,
        };
        let (start, end, last) = memo.get();
        if start <= p && p <= end {
            return last.is_some_and(|j| j >= p);
        }
        let mut run_end = p;
        while run_end <= self.lim() {
            let width = self.space_at(run_end);
            if width > 0 {
                run_end += width;
            } else if let Some(width) = self.word_at(run_end) {
                run_end += width;
            } else if decl == Decl::Class && self.at(run_end) == b'-' && run_end < self.lim() {
                run_end += 1;
            } else {
                break;
            }
        }
        self.spend(run_end - p + 1);
        let mut last = None;
        let mut j = p;
        while j <= run_end {
            let qualifies = match decl {
                Decl::Class => self.class_keyword(j),
                Decl::Record => self.record_keyword(j),
            };
            if qualifies {
                last = Some(j);
            }
            j = if j < self.lim() {
                self.char_end(j)
            } else {
                j + 1
            };
        }
        memo.set((p, run_end, last));
        last.is_some_and(|j| j >= p)
    }

    /// `(?=\w?[-\w\s]*\b(?:class|(?<!@)interface|enum)\s+[$\w]+)`.
    pub(super) fn class_ahead(&self, p: usize) -> Option<usize> {
        self.keyword_ahead(p, Decl::Class).then_some(p)
    }

    /// `(?=\w?[\w\s]*\brecord\s+[$\w]+)`.
    pub(super) fn record_ahead(&self, p: usize) -> Option<usize> {
        self.keyword_ahead(p, Decl::Record).then_some(p)
    }

    /// `class|(?<!@)interface|enum` then a name, with the keyword and name
    /// as groups 1 and 2: `(class|(?<!@)interface|enum)\s+([$\w]+)`.
    pub(super) fn class_name(&mut self, p: usize) -> Option<usize> {
        let end = self
            .lit(p, b"class")
            .or_else(|| self.lit(p, b"interface").filter(|_| !self.prev_is(p, b'@')))
            .or_else(|| self.lit(p, b"enum"))?;
        let start = self.space_end(end);
        if start == end {
            return None;
        }
        let name = self.name_end(start);
        if name == start {
            return None;
        }
        self.cap(1, p, end);
        self.cap(2, start, name);
        Some(name)
    }

    /// `^\s*([\w\s]*)(enum)\s+(\w+)`.
    pub(super) fn enum_begin(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let first = self.space_end(0);
        if first > self.lim() {
            return None;
        }
        // `[\w\s]*` takes the longest run, then gives back until `enum`
        // and a name follow.
        let mut run_end = first;
        while run_end <= self.lim() {
            let width = self.word_at(run_end).or_else(|| {
                let w = self.space_at(run_end);
                (w > 0).then_some(w)
            });
            match width {
                Some(w) => run_end += w,
                None => break,
            }
        }
        self.spend(run_end - first);
        let mut j = run_end.min(self.lim());
        loop {
            if self.starts(j, b"enum") {
                let after = j + 4;
                let name = self.space_end(after);
                if name > after {
                    let end = self.word_end(name);
                    if end > name {
                        self.cap(1, first, j);
                        self.cap(2, j, after);
                        self.cap(3, name, end);
                        return Some(end);
                    }
                }
            }
            if j == first {
                return None;
            }
            j -= 1;
            while j > first && self.s[j] & 0xc0 == 0x80 {
                j -= 1;
            }
        }
    }

    /// `((open)\s)?(module)\s+(\w+)`.
    pub(super) fn module_begin(&mut self, p: usize) -> Option<usize> {
        let with_open = self.lit(p, b"open").and_then(|end| {
            let width = self.space_at(end);
            (width > 0).then_some(end + width)
        });
        for module in [with_open, Some(p)].into_iter().flatten() {
            let Some(end) = self.lit(module, b"module") else {
                continue;
            };
            let name = self.space_end(end);
            if name == end {
                continue;
            }
            let name_end = self.word_end(name);
            if name_end == name {
                continue;
            }
            if let Some(after_open) = with_open.filter(|_| module != p) {
                self.cap(1, p, after_open);
                self.cap(2, p, p + 4);
            }
            self.cap(3, module, end);
            self.cap(4, name, name_end);
            return Some(name_end);
        }
        None
    }

    /// `(record)\s+([$\w]+)(<[$\w]+>)?(\()`.
    pub(super) fn record_head(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"record")?;
        let name = self.space_end(end);
        if name == end {
            return None;
        }
        let name_end = self.name_end(name);
        if name_end == name {
            return None;
        }
        let mut open = name_end;
        let generics = (self.at(open) == b'<')
            .then(|| {
                let inner = self.name_end(open + 1);
                (inner > open + 1 && self.at(inner) == b'>').then_some(inner + 1)
            })
            .flatten();
        if let Some(close) = generics {
            self.cap(3, name_end, close);
            open = close;
        }
        if self.at(open) != b'(' {
            return None;
        }
        self.cap(1, p, end);
        self.cap(2, name, name_end);
        self.cap(4, open, open + 1);
        Some(open + 1)
    }

    /// `(?!new)(?=[<\w].*\s+)(?=([^/=]|/(?!/))+\()` for a method and
    /// `(?!new)(?=[<\w].*\s+)(?=([^(/=]|/(?!/))+(?=\{))` for a record's
    /// constructor: the run from `p` that has no `=` or `//` (and, for a
    /// constructor, no `(`) holds a `(` (or a `{`) after its first character.
    pub(super) fn method_ahead(&self, p: usize, callable: Callable) -> Option<usize> {
        if self.starts(p, b"new") || p >= self.lim() {
            return None;
        }
        if !(self.s[p] == b'<' || self.word_at(p).is_some()) {
            return None;
        }
        let (target, memo) = match callable {
            Callable::Constructor => (b'{', &self.x.memo.constructor),
            Callable::Method => (b'(', &self.x.memo.method),
        };
        let (start, end, last) = memo.get();
        if start <= p && p < end {
            return last.filter(|&at| at > p).map(|_| p);
        }
        let mut i = p;
        let mut last = None;
        while i < self.lim() {
            let b = self.s[i];
            let stops = b == b'='
                || (callable == Callable::Constructor && b == b'(')
                || (b == b'/' && self.at(i + 1) == b'/');
            if stops {
                break;
            }
            if b == target {
                last = Some(i);
            }
            i += 1;
        }
        self.spend(i - p);
        memo.set((p, i.max(p + 1), last));
        last.filter(|&at| at > p).map(|_| p)
    }

    /// `(?=\w.*\s+\w+\s*\()`: a word, then later a blank, a word and a `(`.
    pub(super) fn signature_ahead(&self, p: usize) -> Option<usize> {
        self.word_at(p)?;
        let (known, last) = self.x.memo.signature.get();
        let last = if known {
            last
        } else {
            let last = self.last_signature();
            self.x.memo.signature.set((true, last));
            last
        };
        last.filter(|&e| e >= p + 2).map(|_| p)
    }

    /// The last start of a word that follows a white space character and
    /// precedes `\s*\(`, over the whole line.
    fn last_signature(&self) -> Option<usize> {
        let mut best = None;
        for open in memchr::memchr_iter(b'(', self.s) {
            let mut k = open;
            while k > 0 && self.space_at(self.char_start(k - 1)) > 0 {
                k = self.char_start(k - 1);
            }
            let mut e = k;
            while e > 0 && self.word_before(e) {
                e = self.char_start(e - 1);
            }
            if e < k && e > 0 && self.space_at(self.char_start(e - 1)) > 0 {
                best = Some(best.map_or(e, |b: usize| b.max(e)));
            }
        }
        self.spend(self.s.len());
        best
    }

    /// The start of the character that contains byte `i`.
    fn char_start(&self, mut i: usize) -> usize {
        while i > 0 && self.s.get(i).is_some_and(|b| b & 0xc0 == 0x80) {
            i -= 1;
        }
        i
    }

    /// `(?=\s+\w+\s*\()`: blanks, a word and a `(` from `p`.
    pub(super) fn return_type_end(&self, p: usize) -> Option<usize> {
        let word = self.space_end(p);
        if word == p {
            return None;
        }
        let end = self.word_end(word);
        if end == word {
            return None;
        }
        (self.at(self.space_end(end)) == b'(').then_some(p)
    }

    /// `(\w+)\s*(\()`.
    pub(super) fn method_head(&mut self, p: usize) -> Option<usize> {
        let end = self.word_end(p);
        if end == p {
            return None;
        }
        let open = self.space_end(end);
        if self.at(open) != b'(' {
            return None;
        }
        self.cap(1, p, end);
        self.cap(2, open, open + 1);
        Some(open + 1)
    }
}
