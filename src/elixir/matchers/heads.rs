//! Anonymous functions and their clauses, module and `def` heads, and calls.

use super::super::engine::Lexer;
use crate::rules::Chars;

impl Lexer<'_> {
    /// `\b(fn)\b(?!.*->)` when `arrow` is false, and `\b(fn)\b(?=.*->)` when
    /// it is true.
    /// `.` stops at the line feed, so the `->` lies in the line's text.
    pub(super) fn fn_keyword(&mut self, p: usize, arrow: bool) -> Option<usize> {
        if !(self.starts(p, b"fn") && self.boundary(p) && self.boundary(p + 2)) {
            return None;
        }
        if self.arrow_from(p + 2) != arrow {
            return None;
        }
        self.capture(1, p, p + 2);
        Some(p + 2)
    }

    /// Whether a `->` starts at or after `from` in the line's text.
    /// The line's last `->` answers it, found once per line.
    fn arrow_from(&self, from: usize) -> bool {
        let len = self.lim();
        let (seen, last) = self.x.last_arrow.get();
        let last = if seen == len {
            last
        } else {
            let found = memchr::memmem::rfind(self.text(0, len), b"->");
            self.x.last_arrow.set((len, found));
            found
        };
        last.is_some_and(|at| at >= from)
    }

    /// `(?>(->)|(when)|(\)))`: what ends a clause's head.
    pub(super) fn clause_end(&mut self, p: usize) -> Option<usize> {
        let (group, end) = if self.starts(p, b"->") {
            (1, p + 2)
        } else if self.starts(p, b"when") {
            (2, p + 4)
        } else if self.starts(p, b")") {
            (3, p + 1)
        } else {
            return None;
        };
        self.capture(group, p, end);
        Some(end)
    }

    /// The lookaheads of a clause's head at the start of the line, which the
    /// grammar reads as four conditions:
    ///
    /// ```text
    /// ^(?=.*->)
    /// ((?![^"']*(["'])[^"']*->)|(?=.*->[^"']*(["'])[^"']*->))
    /// ((?!.*\([^)]*->)|(?=[^()]*->)|(?=\s*\(.*\).*->))
    /// ((?!.*\b(fn)\b)|(?=.*->.*\bfn\b))
    /// ```
    ///
    /// The line has a `->` that the first quoted text does not hold, one
    /// outside the parentheses, and one before any `fn`.
    /// The groups match empty text, so the match records none.
    /// It reads the line a few times, at its start only.
    pub(super) fn clause_head(&self) -> bool {
        let s = self.text(0, self.lim());
        let Some(arrows) = Arrows::of(s) else {
            return false;
        };
        quotes_allow(s) && self.parens_allow(s, &arrows) && self.fn_allows(s, arrows.first)
    }

    /// `(?!.*\([^)]*->)|(?=[^()]*->)|(?=\s*\(.*\).*->)`.
    ///
    /// - The first holds when no `->` follows a `(` with no `)` between them.
    /// - The second holds when a `->` comes before the first parenthesis.
    /// - The third holds when the line opens with a `(`, after spaces, and a
    ///   `->` follows the first `)` after it.
    ///
    /// A negated class matches the line feed, but no `->` holds one.
    fn parens_allow(&self, s: &[u8], arrows: &Arrows) -> bool {
        if !arrows.after_open || arrows.first < arrows.first_paren {
            return true;
        }
        let lead = self.space_end(0);
        s.get(lead) == Some(&b'(')
            && memchr::memchr(b')', s.get(lead + 1..).unwrap_or_default())
                .is_some_and(|close| arrows.last > lead + 1 + close)
    }

    /// `(?!.*\b(fn)\b)|(?=.*->.*\bfn\b)`: the line has no word `fn`, or one
    /// after its first `->`.
    fn fn_allows(&self, s: &[u8], first_arrow: usize) -> bool {
        let mut last_fn = None;
        for at in memchr::memmem::find_iter(s, b"fn") {
            if self.boundary(at) && self.boundary(at + 2) {
                last_fn = Some(at);
            }
        }
        last_fn.is_none_or(|at| at >= first_arrow + 2)
    }

    /// `([A-Z]\w+)\s*(\.)\s*([_a-z]\w*[!?]?)`: a call on a module.
    /// `\w+` gives no characters back: a `\w` cannot stand where `\s*\.`
    /// starts.
    pub(super) fn module_call(&mut self, p: usize) -> Option<usize> {
        if !(p < self.lim() && self.at(p).is_ascii_uppercase()) {
            return None;
        }
        let name = self.word_end(p + 1);
        if name == p + 1 {
            return None;
        }
        let (dot, rest) = self.dot_after(name)?;
        let end = self.function_name(rest)?;
        self.capture(1, p, name);
        self.capture(2, dot, dot + 1);
        self.capture(3, rest, end);
        Some(end)
    }

    /// `(:\w+)\s*(\.)\s*(_?\w*[!?]?)`: a call on an atom.
    /// `_?\w*` is `\w*`, since `_` is a `\w`.
    pub(super) fn atom_call(&mut self, p: usize) -> Option<usize> {
        let start = self.lit(p, b":")?;
        let name = self.word_end(start);
        if name == start {
            return None;
        }
        let (dot, rest) = self.dot_after(name)?;
        let end = self.bang_end(self.word_end(rest));
        self.capture(1, p, name);
        self.capture(2, dot, dot + 1);
        self.capture(3, rest, end);
        Some(end)
    }

    /// `\s*\.\s*` at `from`: where the dot stands and where the spaces after
    /// it end.
    fn dot_after(&self, from: usize) -> Option<(usize, usize)> {
        let dot = self.space_end(from);
        (dot < self.lim() && self.at(dot) == b'.').then(|| (dot, self.space_end(dot + 1)))
    }

    /// `(\|>)\s*([_a-z]\w*[!?]?)`: a function after a pipe.
    pub(super) fn pipe_call(&mut self, p: usize) -> Option<usize> {
        let pipe = self.lit(p, b"|>")?;
        let start = self.space_end(pipe);
        let end = self.function_name(start)?;
        self.capture(1, p, pipe);
        self.capture(2, start, end);
        Some(end)
    }

    /// `\b[_a-z]\w*[!?]?(?=\s*\.?\s*\()`: a function name before its
    /// arguments.
    /// Without the `[!?]`, the lookahead would start at the `!` or `?`, where
    /// it fails, and a shorter run of `\w` leaves a `\w` there.
    pub(super) fn call_name(&self, p: usize) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        let end = self.function_name(p)?;
        self.arguments_ahead(end).then_some(end)
    }

    /// `\s*\.?\s*\(` at `from`.
    fn arguments_ahead(&self, from: usize) -> bool {
        let at = self.space_end(from);
        let at = if at < self.lim() && self.at(at) == b'.' {
            self.space_end(at + 1)
        } else {
            at
        };
        at < self.lim() && self.at(at) == b'('
    }

    /// `^\s*(word)\b`: the head of a module, protocol or implementation.
    pub(super) fn module_head(&mut self, p: usize, word: &[u8]) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let start = self.space_end(0);
        let end = start + word.len();
        if !(self.starts(start, word) && self.boundary(end)) {
            return None;
        }
        self.capture(1, start, end);
        Some(end)
    }

    /// `\b(do)\b`, which ends a module's head.
    pub(super) fn do_keyword(&mut self, p: usize) -> Option<usize> {
        let end = self.word_in(p, &[b"do"])?;
        self.capture(1, p, end);
        Some(end)
    }

    /// `^\s*(def(?:suffixes)p?)\s+(name)((\()|\s*)`: the head of a function,
    /// macro, delegate or guard, private when `private`.
    ///
    /// The `suffixes` come in the grammar's order, the empty one first.
    /// A word of `suffixes` whose `\s+` or name fails gives way to the next.
    /// The name is the grammar's `(?>prefix)?(?>…)`: the prefix is atomic,
    /// and dropped when no name follows it.
    pub(super) fn def_head(
        &mut self,
        p: usize,
        suffixes: &[&[u8]],
        private: bool,
    ) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let start = self.space_end(0);
        if !self.starts(start, b"def") {
            return None;
        }
        for suffix in suffixes {
            let mut keyword = start + 3;
            if !suffix.is_empty() {
                if !self.starts(keyword, suffix) {
                    continue;
                }
                keyword += suffix.len();
            }
            if private {
                if !self.starts(keyword, b"p") {
                    continue;
                }
                keyword += 1;
            }
            let name = self.space_end(keyword);
            if name == keyword {
                continue;
            }
            let Some(name_end) = self.def_name(name) else {
                continue;
            };
            self.capture(1, start, keyword);
            self.capture(2, name, name_end);
            if self.starts(name_end, b"(") {
                self.capture(3, name_end, name_end + 1);
                self.capture(4, name_end, name_end + 1);
                return Some(name_end + 1);
            }
            let end = self.space_end(name_end);
            self.capture(3, name_end, end);
            return Some(end);
        }
        None
    }

    /// `(?>[A-Z_a-z]\w*(?>\.|::))?(?>…)`: a function's name, with its module
    /// before it.
    fn def_name(&self, p: usize) -> Option<usize> {
        if let Some(module) = self.name(p) {
            let prefix = if self.starts(module, b".") {
                Some(module + 1)
            } else if self.starts(module, b"::") {
                Some(module + 2)
            } else {
                None
            };
            if let Some(end) = prefix.and_then(|at| self.def_main(at)) {
                return Some(end);
            }
        }
        self.def_main(p)
    }

    /// The atomic alternation of a function's name:
    /// `[A-Z_a-z]\w*(?>[!?]|=(?!>))?`, then the operators, in the grammar's
    /// order.
    /// The first alternative that matches is the name.
    fn def_main(&self, p: usize) -> Option<usize> {
        if let Some(end) = self.name(p) {
            let suffix = match self.at(end) {
                b'!' | b'?' if end < self.lim() => end + 1,
                b'=' if end < self.lim() && self.at(end + 1) != b'>' => end + 1,
                _ => end,
            };
            return Some(suffix);
        }
        self.first_lit(
            p,
            &[
                b"===", b"==", b">=", b">>", b">", b"<=>", b"<<", b"<=", b"<", b"%", b"&", b"/",
                b"`", b"|", b"**", b"*", b"=~", b"~", b"-@", b"-", b"+@", b"+", b"[]=", b"[]",
            ],
        )
    }

    /// `\b(do:)|\b(do)\b|(?=\s+(def(?:suffixes))\b)`: what ends a `def` head.
    /// The lookahead's group lies in the text after the match, and the
    /// grammar names no scope for it.
    pub(super) fn def_end(&mut self, p: usize, suffixes: &[&[u8]]) -> Option<usize> {
        if self.starts(p, b"do:") && self.boundary(p) {
            self.capture(1, p, p + 3);
            return Some(p + 3);
        }
        if let Some(end) = self.word_in(p, &[b"do"]) {
            self.capture(2, p, end);
            return Some(end);
        }
        let start = self.space_end(p);
        if start == p || !self.starts(start, b"def") {
            return None;
        }
        suffixes
            .iter()
            .any(|suffix| {
                let end = start + 3 + suffix.len();
                (suffix.is_empty() || self.starts(start + 3, suffix)) && self.boundary(end)
            })
            .then_some(p)
    }

    /// `\s(\\\\)`: a default argument.
    pub(super) fn default_argument(&mut self, p: usize) -> Option<usize> {
        let at = p + self.space_at(p);
        if at == p || !self.starts(at, b"\\\\") {
            return None;
        }
        self.capture(1, at, at + 2);
        Some(at + 2)
    }
}

/// `(?![^"']*(["'])[^"']*->)|(?=.*->[^"']*(["'])[^"']*->)`.
///
/// The first fails when the text between the first quote and the next
/// holds a `->`.
/// The second holds when, after some `->`, the text between the next
/// quote and the one after it holds a `->`.
/// A negated class matches the line feed, but no `->` holds one.
fn quotes_allow(s: &[u8]) -> bool {
    let quoted_arrow = |quote: usize| {
        let rest = s.get(quote + 1..).unwrap_or_default();
        let close = memchr::memchr2(b'"', b'\'', rest).unwrap_or(rest.len());
        memchr::memmem::find(rest.get(..close).unwrap_or_default(), b"->").is_some()
    };
    let next_quote = |from: usize| {
        s.get(from..)
            .and_then(|rest| memchr::memchr2(b'"', b'\'', rest))
            .map(|at| from + at)
    };
    let Some(first_quote) = next_quote(0) else {
        return true;
    };
    if !quoted_arrow(first_quote) {
        return true;
    }
    let mut tried = None;
    for arrow in memchr::memmem::find_iter(s, b"->") {
        let Some(quote) = next_quote(arrow + 2) else {
            break;
        };
        if tried == Some(quote) {
            continue;
        }
        if quoted_arrow(quote) {
            return true;
        }
        tried = Some(quote);
    }
    false
}

/// What a clause's head reads about the `->` of a line.
struct Arrows {
    /// Where the first `->` starts.
    first: usize,
    /// Where the last `->` starts.
    last: usize,
    /// Where the first `(` or `)` stands, or the text's end.
    first_paren: usize,
    /// Whether a `->` follows a `(` with no `)` between them.
    after_open: bool,
}

impl Arrows {
    /// The `->` of the text `s`, or `None` when it has none.
    /// A `->` holds no parenthesis, so one pass over the bytes answers all.
    fn of(s: &[u8]) -> Option<Arrows> {
        let mut first = None;
        let mut last = 0;
        let mut first_paren = s.len();
        let mut open = false;
        let mut after_open = false;
        for (at, &b) in s.iter().enumerate() {
            match b {
                b'(' | b')' => {
                    first_paren = first_paren.min(at);
                    open = b == b'(';
                }
                b'-' if s.get(at + 1) == Some(&b'>') => {
                    first.get_or_insert(at);
                    last = at;
                    after_open |= open;
                }
                _ => {}
            }
        }
        Some(Arrows {
            first: first?,
            last,
            first_paren,
            after_open,
        })
    }
}
