//! Expressions: literal values, operators, brackets, braces, attribute access,
//! function calls, parentheses and names.

use super::lexer::Lexer;
use super::scope::S;
use super::state::Rule;
use crate::classes::word_before;

impl Lexer<'_> {
    /// The grammar's `expressions` at `p`, in its order: literal values,
    /// operators, `[for`, `{for`, brackets, objects, attribute access and
    /// splats, function calls, then parentheses.
    pub(super) fn expression(&mut self, p: usize) -> Option<usize> {
        match self.s[p] {
            b'"' => Some(self.string(p)),
            b'<' => self.heredoc(p).or_else(|| self.operator(p)),
            b'[' => Some(self.bracket(p)),
            b'{' => Some(self.brace(p)),
            b'(' => Some(self.begin(p, p + 1, Rule::Parens, S::ParensBegin)),
            b'.' => self.operator(p).or_else(|| Some(self.access(p))),
            _ if self.word_at(p) => self.word_value(p).or_else(|| self.function(p)),
            _ => self.operator(p),
        }
    }

    /// The grammar's `object_key_values` at `p`, after the rules an object
    /// reads first: literal values, operators, `[for`, heredocs and function
    /// calls.
    /// An object reads no bracket, parenthesis, attribute access or name.
    pub(super) fn object_value(&mut self, p: usize) -> Option<usize> {
        match self.s[p] {
            b'"' => Some(self.string(p)),
            b'<' => self.heredoc(p).or_else(|| self.operator(p)),
            b'[' => {
                let f = self.for_after(p + 1)?;
                Some(self.begin_for(p, f, Rule::TupleFor, S::BracketsBegin))
            }
            _ if self.word_at(p) => self.word_value(p).or_else(|| self.function(p)),
            _ => self.operator(p),
        }
    }

    /// A number, `true`, `false`, `null` or a type keyword at `p`.
    fn word_value(&mut self, p: usize) -> Option<usize> {
        if let Some(end) = self.number(p) {
            return Some(end);
        }
        let word = self.whole_word(p)?;
        let leaf = match word {
            b"true" | b"false" | b"null" => S::Constant,
            b"any" | b"string" | b"number" | b"bool" | b"list" | b"set" | b"map" | b"tuple"
            | b"object" => S::StorageType,
            _ => return None,
        };
        let end = p + word.len();
        self.tok_at(p, end, &[leaf]);
        Some(end)
    }

    /// The end of the exponent `[Ee][-+]?\d+` at `i` and where its digits
    /// start, when one is there.
    fn exponent(&self, i: usize) -> Option<(usize, usize)> {
        if !matches!(self.at(i), b'e' | b'E') {
            return None;
        }
        let digits = i + 1 + usize::from(matches!(self.at(i + 1), b'-' | b'+'));
        let end = self.digits_end(digits);
        (end > digits).then_some((digits, end))
    }

    /// A number at `p`, as the grammar's three rules read one in turn:
    /// `\b\d+([Ee][-+]?)\d+\b`, `\b\d+(\.)\d+(?:([Ee][-+]?)\d+)?\b` and
    /// `\b\d+\b`.
    fn number(&mut self, p: usize) -> Option<usize> {
        if word_before(self.s, p) {
            return None;
        }
        let digits = self.digits_end(p);
        if digits == p {
            return None;
        }
        if let Some((sign_end, end)) = self.exponent(digits)
            && !self.word_at(end)
        {
            self.tok_at(p, digits, &[S::Float]);
            self.tok(sign_end, &[S::Float, S::Exponent]);
            self.tok(end, &[S::Float]);
            return Some(end);
        }
        if self.at(digits) == b'.' {
            let fraction = self.digits_end(digits + 1);
            if fraction > digits + 1 {
                let exponent = self
                    .exponent(fraction)
                    .filter(|&(_, end)| !self.word_at(end));
                if exponent.is_some() || !self.word_at(fraction) {
                    self.tok_at(p, digits, &[S::Float]);
                    self.tok(digits + 1, &[S::Float, S::Decimal]);
                    self.tok(fraction, &[S::Float]);
                    let Some((sign_end, end)) = exponent else {
                        return Some(fraction);
                    };
                    self.tok(sign_end, &[S::Float, S::Exponent]);
                    self.tok(end, &[S::Float]);
                    return Some(end);
                }
            }
        }
        if self.word_at(digits) {
            return None;
        }
        self.tok_at(p, digits, &[S::Integer]);
        Some(digits)
    }

    /// An operator at `p`, in the order of the grammar's `operators`.
    /// A lone `=`, `&` or `|` is none.
    pub(super) fn operator(&mut self, p: usize) -> Option<usize> {
        let next = self.at(p + 1);
        let (len, leaf) = match self.s[p] {
            b'>' | b'<' | b'=' | b'!' if next == b'=' => (2, S::Operator),
            b'+' | b'-' | b'*' | b'/' | b'%' => (1, S::Arithmetic),
            b'&' if next == b'&' => (2, S::Logical),
            b'|' if next == b'|' => (2, S::Logical),
            b'!' => (1, S::Logical),
            b'>' | b'<' | b'?' | b':' => (1, S::Operator),
            b'.' if next == b'.' && self.at(p + 2) == b'.' => (3, S::Operator),
            b'=' if next == b'>' => (2, S::Operator),
            _ => return None,
        };
        self.tok_at(p, p + len, &[leaf]);
        Some(p + len)
    }

    /// Where `for` starts after `\s?` at `i`, when `\b` follows it.
    pub(super) fn for_after(&self, i: usize) -> Option<usize> {
        let space = crate::classes::space_width(self.s, i);
        if space > 0 && self.word_is(i + space, b"for") {
            return Some(i + space);
        }
        self.word_is(i, b"for").then_some(i)
    }

    /// Opens a `for` context at `p`, whose `for` starts at `f`.
    pub(super) fn begin_for(&mut self, p: usize, f: usize, rule: Rule, punct: S) -> usize {
        self.gap(p);
        if self.push(rule, 0) {
            self.tok(p + 1, &[punct]);
            self.tok_at(f, f + 3, &[S::Control]);
        }
        f + 3
    }

    /// `[for` or `[` at `p`.
    fn bracket(&mut self, p: usize) -> usize {
        match self.for_after(p + 1) {
            Some(f) => self.begin_for(p, f, Rule::TupleFor, S::BracketsBegin),
            None => self.begin(p, p + 1, Rule::Brackets, S::BracketsBegin),
        }
    }

    /// `{for` or `{` at `p`.
    fn brace(&mut self, p: usize) -> usize {
        match self.for_after(p + 1) {
            Some(f) => self.begin_for(p, f, Rule::ObjectFor, S::BracesBegin),
            None => self.begin(p, p + 1, Rule::Objects, S::BracesBegin),
        }
    }

    /// `.` at `p` with the member name or index after it, or `.*`.
    ///
    /// The name is `\p{alpha}[-\w]*`, and `(?!null|false|true)` guards its
    /// scope at each of its bytes.
    /// A name that starts with one of those words loses the scope on its
    /// first character only.
    pub(super) fn access(&mut self, p: usize) -> usize {
        self.tok_at(p, p + 1, &[S::Accessor]);
        let name = p + 1;
        if self.at(name) == b'*' {
            self.tok(name + 1, &[S::Splat]);
            return name + 1;
        }
        if self.alpha_width(name) > 0 {
            let end = self.dash_word_end(name);
            let from = if self.keyword_prefix(name) {
                name + 1
            } else {
                name
            };
            self.tok_at(from, end, &[S::Member]);
            return end;
        }
        let end = self.digits_end(name);
        self.tok_at(name, end, &[S::Integer]);
        end
    }

    /// `([-:\w]+)(\()` at `p`, a word character: a function call.
    ///
    /// Each name in it is namespaced or built in.
    fn function(&mut self, p: usize) -> Option<usize> {
        if p < self.no_call_before {
            return None;
        }
        let name_end = self.run_end_with_colons(p);
        if self.at(name_end) != b'(' {
            self.no_call_before = name_end;
            return None;
        }
        self.gap(p);
        if !self.push(Rule::Functions, 0) {
            return Some(name_end + 1);
        }
        let mut q = p;
        while q < name_end {
            if self.alpha_width(q) > 0 && !word_before(self.s, q) {
                if let Some(end) = self.namespaced(q) {
                    self.tok_at(q, end, &[S::Namespaced]);
                    q = end;
                    continue;
                }
                if let Some(end) = self.trim(q, self.dash_word_end(q)) {
                    self.tok_at(q, end, &[S::Builtin]);
                    q = end;
                    continue;
                }
            }
            q += self.width_at(q);
        }
        self.tok_at(name_end, name_end + 1, &[S::ParensBegin]);
        Some(name_end + 1)
    }

    /// The end of the `[-:\w]*` run at `i`.
    fn run_end_with_colons(&self, i: usize) -> usize {
        let mut end = i;
        loop {
            let next = match self.at(end) {
                b'-' | b':' => end + 1,
                _ if self.word_at(end) => self.word_end(end),
                _ => {
                    self.steps.spend(end - i);
                    return end;
                }
            };
            end = next;
        }
    }

    /// `\p{alpha}[-_\w]*::(\p{alpha}[-_\w]*::)?\p{alpha}[-_\w]*\b` at `q`.
    fn namespaced(&self, q: usize) -> Option<usize> {
        let first = self.dash_word_end(q);
        if !self.s[first..].starts_with(b"::") || self.alpha_width(first + 2) == 0 {
            return None;
        }
        let second = self.dash_word_end(first + 2);
        if self.s[second..].starts_with(b"::")
            && self.alpha_width(second + 2) > 0
            && let Some(end) = self.trim(second + 2, self.dash_word_end(second + 2))
        {
            return Some(end);
        }
        self.trim(first + 2, second)
    }

    /// `\b(?!null|false|true)\p{alpha}[-_[:alnum:]]*\b` at `p`: the end of a
    /// local name.
    pub(super) fn identifier(&self, p: usize) -> Option<usize> {
        if word_before(self.s, p) || self.alpha_width(p) == 0 || self.keyword_prefix(p) {
            return None;
        }
        self.trim(p, self.name_end(p))
    }

    /// A local name at `p`.
    pub(super) fn local(&mut self, p: usize) -> Option<usize> {
        let end = self.identifier(p)?;
        self.tok_at(p, end, &[S::Readwrite]);
        Some(end)
    }
}
