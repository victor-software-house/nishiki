//! Functions, keywords, macros, types, symbols and the end of a `for` head.

use super::super::chars::is_macro_operator;
use super::super::engine::Lexer;
use super::operators::ends_operand;
use crate::classes::is_space;
use crate::rules::Chars;

/// The words of `keyword.control`.
/// `else` comes before `elseif`, which the `\b` after it lets through.
pub(super) const CONTROL: &[&[u8]] = &[
    b"if",
    b"else",
    b"elseif",
    b"for",
    b"while",
    b"begin",
    b"let",
    b"do",
    b"try",
    b"catch",
    b"finally",
    b"return",
    b"break",
    b"continue",
];

/// The words of `keyword.other`, in the grammar's order.
/// The second word of a pair follows the first after white space.
const OTHER: &[(&[u8], Option<&[u8]>)] = &[
    (b"function", None),
    (b"mutable", Some(b"struct")),
    (b"struct", None),
    (b"macro", None),
    (b"quote", None),
    (b"abstract", Some(b"type")),
    (b"primitive", Some(b"type")),
    (b"typegroup", None),
    (b"module", None),
    (b"baremodule", None),
    (b"where", None),
];

/// The keywords of a type declaration, in the grammar's order.
const DECLARED: &[(&[u8], Option<&[u8]>)] = &[
    (b"struct", None),
    (b"mutable", Some(b"struct")),
    (b"abstract", Some(b"type")),
    (b"primitive", Some(b"type")),
];

/// A function's name after `function` or `macro`.
/// It holds the dot after its module, the name, and the end of the type
/// parameters in braces after it.
/// That end is the name's end when the name has no type parameters.
struct Name {
    module_dot: Option<usize>,
    ident: (usize, usize),
    end: usize,
}

impl Lexer<'_> {
    /// The end of the first of `words` at `p` that `\b` ends.
    /// A pair's words have white space between them.
    fn first_word(&self, p: usize, words: &[(&[u8], Option<&[u8]>)]) -> Option<usize> {
        words.iter().find_map(|&(first, second)| {
            let mut end = self.lit(p, first)?;
            if let Some(second) = second {
                let space = self.space_end(end);
                end = self.lit(space, second).filter(|_| space > end)?;
            }
            (self.word_at(end).is_none()).then_some(end)
        })
    }

    /// `\b(?<![.:_])(?:function|mutable\s+struct|…|where)\b`.
    pub(super) fn keyword_other(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) || (p > 0 && matches!(self.at(p - 1), b'.' | b':' | b'_')) {
            return None;
        }
        self.first_word(p, OTHER)
    }

    /// `\b(?<![:_])(?:a|b|…)\b` for the one-word list `words`.
    pub(super) fn keyword_in(&self, p: usize, words: &[&[u8]]) -> Option<usize> {
        if p > 0 && matches!(self.at(p - 1), b':' | b'_') {
            return None;
        }
        self.word_in(p, words)
    }

    /// `^public\b`.
    pub(super) fn public(&self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let end = self.lit(p, b"public")?;
        self.word_at(end).is_none().then_some(end)
    }

    /// `(?<=\S\s+)\b(as)\b(?=\s+\S)`: `as` with white space on both sides,
    /// and a character other than white space beyond each.
    pub(super) fn as_keyword(&self, p: usize) -> Option<usize> {
        let end = self.keyword(p, b"as")?;
        let mut back = p;
        while let Some(c) = self.char_before(back)
            && is_space(c)
        {
            back = self.char_start_before(back);
        }
        let ahead = self.space_end(end);
        (back < p && back > 0 && ahead > end && ahead < self.lim()).then_some(end)
    }

    /// `@(\.|IDENT|[[\p{S}\p{P}]&&[^@\s]]+)`: a macro's name.
    pub(super) fn macro_name(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'@' {
            return None;
        }
        if self.at(p + 1) == b'.' {
            return Some(p + 2);
        }
        if let Some(end) = self.ident(p + 1) {
            return Some(end);
        }
        let mut q = p + 1;
        while let Some(end) = self.class_char(q, is_macro_operator) {
            q = end;
        }
        (q > p + 1).then_some(q)
    }

    /// A function defined by assignment, such as `f(x) = x`.
    /// The expression is `(IDENT)(BRACES)?(?=SIGNATURE)`.
    /// The lookahead is the one `assigned_signature` reads.
    /// Group 1 is the name, group 2 the type parameters.
    /// Braces that the lookahead does not follow are given back, and `(`
    /// cannot stand at a `{`.
    pub(super) fn function_assignment(&mut self, p: usize) -> Option<usize> {
        let name = self.ident(p)?;
        let end = self.braces(name).unwrap_or(name);
        if !self.assigned_signature(end) {
            return None;
        }
        self.capture(1, p, name);
        self.capture(2, name, end);
        Some(end)
    }

    /// The lookahead of a function defined by assignment:
    /// - `\([^#]*\)`, parentheses;
    /// - `(::\S+)?`, a return type;
    /// - `(\s*\bwhere\b\s+.+?)?`, a `where` clause;
    /// - `\s*?=(?![=>])`, an `=`.
    ///
    /// `[^#]*` can end at any `)` before the first `#`, so each is tried.
    fn assigned_signature(&self, p: usize) -> bool {
        if self.at(p) != b'(' {
            return false;
        }
        let Some(rest) = self.s.get(p + 1..) else {
            return false;
        };
        let stop = p + 1 + memchr::memchr(b'#', rest).unwrap_or(rest.len());
        let mut from = p + 1;
        while let Some(k) = self.s.get(from..stop).and_then(|s| memchr::memchr(b')', s)) {
            let close = from + k;
            self.spend(close + 1 - from);
            if self.signature_tail(close + 1) {
                return true;
            }
            from = close + 1;
        }
        false
    }

    /// `(::\S+)?(\s*\bwhere\b\s+.+?)?\s*?=(?![=>])` at `p`.
    /// `\S+` can end anywhere in its run, so each end is tried.
    fn signature_tail(&self, p: usize) -> bool {
        if self.where_clause(p) || self.assignment(p) {
            return true;
        }
        if !self.starts(p, b"::") {
            return false;
        }
        let mut end = p + 2;
        while end < self.lim() && self.space_at(end) == 0 {
            end = self.char_end(end);
            self.spend(1);
            if self.where_clause(end) || self.assignment(end) {
                return true;
            }
        }
        false
    }

    /// `\s*\bwhere\b\s+.+?\s*?=(?![=>])` at `p`.
    /// `.+?` can take any characters of the line.
    /// So the clause holds when an `=` stands after the first space and at
    /// least one more character.
    /// Neither `=` nor `>` may follow that `=`.
    fn where_clause(&self, p: usize) -> bool {
        let word = self.space_end(p);
        let Some(end) = self.keyword(word, b"where") else {
            return false;
        };
        let space = self.space_at(end);
        if space == 0 || end >= self.lim() {
            return false;
        }
        let from = end + space + 1;
        let mut q = from;
        while let Some(k) = self.s.get(q..).and_then(|s| memchr::memchr(b'=', s)) {
            let at = q + k;
            if !matches!(self.at(at + 1), b'=' | b'>') {
                self.spend(at - from);
                return true;
            }
            q = at + 1;
        }
        false
    }

    /// `\s*?=(?![=>])` at `p`: white space, then an `=` that `=` and `>` do
    /// not follow.
    fn assignment(&self, p: usize) -> bool {
        let q = self.space_end(p);
        self.at(q) == b'=' && !matches!(self.at(q + 1), b'=' | b'>')
    }

    /// `\b(function|macro)(?:\s+(?:IDENT(\.))?(IDENT)(BRACES)?|\s*)(?=\()`:
    /// the head of a function or macro.
    /// Group 1 is the keyword, group 2 the dot after a module, group 3 the
    /// name and group 4 its type parameters.
    pub(super) fn function_keyword(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let keyword = self.lit(p, b"function").or_else(|| self.lit(p, b"macro"))?;
        let space = self.space_end(keyword);
        let named = if space > keyword {
            self.function_name(space)
        } else {
            None
        };
        self.capture(1, p, keyword);
        if let Some(Name {
            module_dot,
            ident,
            end,
        }) = named
        {
            if let Some(dot) = module_dot {
                self.capture(2, dot, dot + 1);
            }
            self.capture(3, ident.0, ident.1);
            self.capture(4, ident.1, end);
            return Some(end);
        }
        (self.at(space) == b'(').then_some(space)
    }

    /// `(?:IDENT(\.))?(IDENT)(BRACES)?(?=\()` at `p`.
    /// The module and its dot are given back when the rest fails after them.
    fn function_name(&self, p: usize) -> Option<Name> {
        let first = self.ident(p)?;
        if self.at(first) == b'.'
            && let Some(name) = self.ident(first + 1)
            && let Some(end) = self.called(name)
        {
            return Some(Name {
                module_dot: Some(first),
                ident: (first + 1, name),
                end,
            });
        }
        let end = self.called(first)?;
        Some(Name {
            module_dot: None,
            ident: (p, first),
            end,
        })
    }

    /// `(BRACES)?(?=\()` at `p`: the end of the braces, or `p` without them,
    /// when a `(` follows.
    fn called(&self, p: usize) -> Option<usize> {
        let end = self.braces(p).unwrap_or(p);
        (self.at(end) == b'(').then_some(end)
    }

    /// `(IDENT)(BRACES)?\.?(\()`: a call.
    /// Group 1 is the name, group 2 the type parameters, group 3 the
    /// parenthesis.
    pub(super) fn function_call(&mut self, p: usize) -> Option<usize> {
        let name = self.ident(p)?;
        let braces = self.braces(name).unwrap_or(name);
        let open = braces + usize::from(self.at(braces) == b'.');
        if self.at(open) != b'(' {
            return None;
        }
        self.capture(1, p, name);
        self.capture(2, name, braces);
        self.capture(3, open, open + 1);
        Some(open + 1)
    }

    /// `\)(('|(\.'))*\.?')?`: the end of a call, whose transposes are group 1.
    pub(super) fn function_call_end(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, b")")?;
        match self.primes(end) {
            Some(primes) => {
                self.capture(1, end, primes);
                Some(primes)
            }
            None => Some(end),
        }
    }

    /// A type declaration:
    /// - `!:_(?:struct|mutable\s+struct|abstract\s+type|primitive\s+type)`;
    /// - `\s+(IDENT)`, the name;
    /// - `(\s*(<:)\s*IDENT(?:\{.*})?)?`, the supertype.
    ///
    /// Its leading `!:_` is literal, so the grammar's rule matches only text
    /// that holds it.
    /// Group 1 is the name, group 2 the supertype with its operator, which is
    /// group 3.
    pub(super) fn type_decl(&mut self, p: usize) -> Option<usize> {
        let keyword = self.lit(p, b"!:_")?;
        let keyword_end = self.first_word_unbounded(keyword, DECLARED)?;
        let start = self.space_end(keyword_end);
        if start == keyword_end {
            return None;
        }
        let name = self.ident(start)?;
        self.capture(1, start, name);
        let op = self.space_end(name);
        let Some(after_op) = self.lit(op, b"<:") else {
            return Some(name);
        };
        let Some(supertype) = self.ident(self.space_end(after_op)) else {
            return Some(name);
        };
        let end = match self.at(supertype) {
            b'{' => self
                .s
                .get(supertype + 1..)
                .and_then(|rest| memchr::memrchr(b'}', rest))
                .map_or(supertype, |k| supertype + 2 + k),
            _ => supertype,
        };
        self.capture(2, name, end);
        self.capture(3, op, after_op);
        Some(end)
    }

    /// The end of the first of `words` at `p`, with no `\b` after it.
    fn first_word_unbounded(&self, p: usize, words: &[(&[u8], Option<&[u8]>)]) -> Option<usize> {
        words.iter().find_map(|&(first, second)| {
            let end = self.lit(p, first)?;
            match second {
                Some(second) => {
                    let space = self.space_end(end);
                    self.lit(space, second).filter(|_| space > end)
                }
                None => Some(end),
            }
        })
    }

    /// ``(?<![]!)}′⁺-ₜ∇[:word:]]):IDENT(?!REST)(?!["`])``: a symbol.
    /// `REST*` takes the whole run, so `(?!REST)` holds.
    /// A quote after the run fails the match, since giving a character back
    /// leaves one of `REST` next.
    pub(super) fn symbol(&self, p: usize) -> Option<usize> {
        if self.at(p) != b':' || self.char_before(p).is_some_and(ends_operand) {
            return None;
        }
        let end = self.ident(p + 1)?;
        (!matches!(self.at(end), b'"' | b'`')).then_some(end)
    }

    /// `(?<![,\s])(\s*\n)`: the end of a `for` head, at a line that does not
    /// end in a comma.
    /// Past the line feed, where a comment's end leaves the cursor, the line
    /// feed stands before the position and no line feed after it.
    pub(super) fn for_end(&self, p: usize) -> Option<usize> {
        if p > self.lim() || self.char_before(p).is_some_and(|c| c == ',' || is_space(c)) {
            return None;
        }
        let end = self.space_end(p);
        (self.nl && end == self.lim() + 1).then_some(end)
    }
}
