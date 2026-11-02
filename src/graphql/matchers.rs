//! The grammar's regular expressions as hand-written matchers.
//!
//! Each matcher answers whether its expression matches starting at a given
//! position, and with which capture groups, including where backtracking
//! lands. The text ends with a line feed that is not in the line: `\s`
//! matches it and `.` does not.

use super::engine::{Hit, Lexer};
use super::rules::Pat;
use super::scope::S;
use super::state::Rule;
use crate::json::classes;

/// The regex `\s` for an ASCII byte.
fn is_ascii_space(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r')
}

/// A byte of `[0-9A-Z_a-z]`.
fn is_name_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

#[expect(
    clippy::many_single_char_names,
    reason = "a matcher names the positions of its regular expression"
)]
impl Lexer<'_> {
    // Character classes.

    /// The character at `i` and its width.
    fn char_at(&self, i: usize) -> Option<(char, usize)> {
        if i == self.lim {
            return Some(('\n', 1));
        }
        let width = super::engine::char_len(*self.s.get(i)?);
        let text = std::str::from_utf8(self.s.get(i..i + width)?).ok()?;
        Some((text.chars().next()?, width))
    }

    /// The width of the white space character at `i`, 0 when there is none.
    fn space_at(&self, i: usize) -> usize {
        match self.at(i) {
            0 => 0,
            b if b < 0x80 => usize::from(is_ascii_space(b)),
            _ => match self.char_at(i) {
                Some((c, width)) if classes::is_space(c) => width,
                _ => 0,
            },
        }
    }

    /// The end of the run of white space from `i`: the regex `\s*`.
    pub(super) fn ws(&self, i: usize) -> usize {
        let (from, to) = self.space_run.get();
        if from <= i && i < to {
            self.spend(1);
            return to;
        }
        let mut j = i;
        while let width @ 1.. = self.space_at(j) {
            j += width;
        }
        self.spend(j - i + 1);
        self.space_run.set((i, j));
        j
    }

    /// Whether the character at `i` is one `\w` matches.
    fn word_at(&self, i: usize) -> bool {
        match self.at(i) {
            0 => false,
            b if b < 0x80 => is_name_byte(b),
            _ => self.char_at(i).is_some_and(|(c, _)| classes::is_word(c)),
        }
    }

    /// Whether the character that ends at `i` is one `\w` matches.
    fn word_before(&self, i: usize) -> bool {
        let Some(mut start) = i.checked_sub(1) else {
            return false;
        };
        while start > 0 && self.s.get(start).is_some_and(|&b| b & 0xc0 == 0x80) {
            start -= 1;
        }
        self.word_at(start)
    }

    /// The regex `\b` at `i`.
    fn boundary(&self, i: usize) -> bool {
        self.word_before(i) != self.word_at(i)
    }

    /// The end of the name `[A-Z_a-z][0-9A-Z_a-z]*` at `i`.
    fn name_end(&self, i: usize) -> Option<usize> {
        let first = self.at(i);
        if !(first.is_ascii_alphabetic() || first == b'_') {
            return None;
        }
        let (from, to) = self.word_run.get();
        if from <= i && i < to {
            self.spend(1);
            return Some(to);
        }
        let mut j = i + 1;
        while j < self.lim && is_name_byte(self.s[j]) {
            j += 1;
        }
        self.spend(j - i);
        self.word_run.set((i, j));
        Some(j)
    }

    /// The end of the run of ASCII letters and `_` from `i`: `[A-Z_a-z]*`.
    fn letters_end(&self, mut i: usize) -> usize {
        let from = i;
        while i < self.lim && (self.s[i].is_ascii_alphabetic() || self.s[i] == b'_') {
            i += 1;
        }
        self.spend(i - from + 1);
        i
    }

    /// Whether the text at `i` is `word`.
    fn lit(&self, i: usize, word: &[u8]) -> bool {
        word.iter().enumerate().all(|(k, &b)| self.at(i + k) == b)
    }

    /// The end of the keyword `word` at `i`, with a boundary on each side:
    /// `\bword\b`.
    fn keyword(&self, i: usize, word: &[u8]) -> Option<usize> {
        let end = i + word.len();
        (self.boundary(i) && self.lit(i, word) && self.boundary(end)).then_some(end)
    }

    // Shapes that several patterns share.

    /// `\s*(c)` with the group scoped.
    fn single(&self, p: usize, c: u8, scope: S) -> Option<Hit> {
        let i = self.ws(p);
        (self.at(i) == c).then(|| Hit::new(p, i + 1).cap(0, i, i + 1, scope))
    }

    /// `\s*(name)` with the group scoped.
    fn named(&self, p: usize, scope: S) -> Option<Hit> {
        let i = self.ws(p);
        let e = self.name_end(i)?;
        Some(Hit::new(p, e).cap(0, i, e, scope))
    }

    /// `\s*(name)(?:\s*(!))?`, the group and the `!` scoped.
    fn typed(&self, p: usize, scope: S) -> Option<Hit> {
        let i = self.ws(p);
        let e = self.name_end(i)?;
        let k = self.ws(e);
        let hit = Hit::new(p, e).cap(0, i, e, scope);
        if self.at(k) == b'!' {
            Some(Hit { end: k + 1, ..hit }.cap(1, k, k + 1, S::NullType))
        } else {
            Some(hit)
        }
    }

    /// The lookahead `(?=\s*\(|:)` after a name that ends at `e`.
    fn field_ahead(&self, e: usize) -> bool {
        self.at(self.ws(e)) == b'(' || self.at(e) == b':'
    }

    // Begin and match patterns.

    /// The match of `pat` that starts at `p`.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm for each pattern of the grammar"
    )]
    pub(super) fn pat_at(&self, pat: Pat, p: usize) -> Option<Hit> {
        match pat {
            Pat::Hash => {
                let i = self.ws(p);
                (self.at(i) == b'#' && i < self.lim)
                    .then(|| Hit::new(p, self.lim).cap(0, p, i, S::LeadingSpace))
            }
            Pat::Ampersand => self.single(p, b'&', S::Ampersand),
            Pat::Colon => self.single(p, b':', S::Colon),
            Pat::Comma => self.single(p, b',', S::Comma),
            Pat::Boolean => {
                let i = self.ws(p);
                let e = self
                    .keyword(i, b"true")
                    .or_else(|| self.keyword(i, b"false"))?;
                Some(Hit::new(p, e).cap(0, i, e, S::Boolean))
            }
            Pat::Null => {
                let i = self.ws(p);
                let e = self.keyword(i, b"null")?;
                Some(Hit::new(p, e).cap(0, i, e, S::Null))
            }
            Pat::EnumValue => {
                let e = self.name_end(self.ws(p))?;
                Some(Hit::new(p, e))
            }
            Pat::Alias => {
                let i = self.ws(p);
                let e = self.name_end(i)?;
                let k = self.ws(e);
                (self.at(k) == b':').then(|| {
                    Hit::new(p, k + 1)
                        .cap(0, i, e, S::Alias)
                        .cap(1, k, k + 1, S::Colon)
                })
            }
            Pat::Field => self.named(p, S::Variable),
            Pat::Name => self.named(p, S::FunctionName),
            Pat::Implemented | Pat::SchemaType => self.named(p, S::SupportType),
            Pat::Float => self.float(p),
            Pat::ObjectField => {
                let i = self.ws(p);
                let e = self.name_end(i)?;
                let k = self.ws(e);
                (self.at(k) == b':').then(|| {
                    Hit::new(p, k + 1)
                        .nested(0, i, e, S::ObjectKey, S::ObjectKeyName)
                        .cap(1, k, k + 1, S::Punctuation)
                })
            }
            Pat::Operation => {
                let i = self.ws(p);
                let e = self
                    .keyword(i, b"query")
                    .or_else(|| self.keyword(i, b"mutation"))?;
                Some(Hit::new(p, e).cap(0, i, e, S::Operation))
            }
            Pat::Scalar => {
                let i = self.ws(p);
                let e = self.keyword(i, b"scalar")?;
                let j = self.ws(e);
                let n = self.name_end(j)?;
                Some(
                    Hit::new(p, n)
                        .cap(0, i, e, S::KeywordScalar)
                        .cap(1, j, n, S::ScalarName),
                )
            }
            Pat::BuiltinType => {
                let i = self.ws(p);
                let e = [&b"Int"[..], b"Float", b"String", b"Boolean", b"ID"]
                    .iter()
                    .find_map(|word| self.keyword(i, word))?;
                let k = self.ws(e);
                let hit = Hit::new(p, e).cap(0, i, e, S::BuiltinType);
                if self.at(k) == b'!' {
                    Some(Hit { end: k + 1, ..hit }.cap(1, k, k + 1, S::NullType))
                } else {
                    Some(hit)
                }
            }
            Pat::TypeName => self.typed(p, S::SupportType),
            Pat::SkipNewlines => {
                (p <= self.lim && self.ws(p) > self.lim).then(|| Hit::new(p, self.lim + 1))
            }
            Pat::VariableName => {
                let i = self.ws(p);
                if self.at(i) != b'$' {
                    return None;
                }
                let e = self.name_end(i + 1)?;
                Some(Hit::new(p, e).cap(0, i, e, S::Variable))
            }
            Pat::UnionMember => {
                let i = self.ws(p);
                if self.at(i) != b'|' {
                    return None;
                }
                let j = self.ws(i + 1);
                let e = self.name_end(j)?;
                Some(
                    Hit::new(p, e)
                        .cap(0, i, i + 1, S::UnionOr)
                        .cap(1, j, e, S::SupportType),
                )
            }
            Pat::Location => {
                let i = self.ws(p);
                if self.at(i) != b'|' {
                    return None;
                }
                let j = self.ws(i + 1);
                let e = self.letters_end(j);
                Some(Hit::new(p, e).cap(0, j, e, S::LocationType))
            }
            Pat::EscapeSimple => {
                let ok = self.at(p) == b'\\'
                    && matches!(
                        self.at(p + 1),
                        b'"' | b'\'' | b'/' | b'\\' | b'b' | b'f' | b'n' | b'r' | b't'
                    );
                ok.then(|| Hit::new(p, p + 2))
            }
            Pat::EscapeUnicode => {
                let ok = self.at(p) == b'\\'
                    && self.at(p + 1) == b'u'
                    && (2..6).all(|k| self.at(p + k).is_ascii_hexdigit());
                ok.then(|| Hit::new(p, p + 6))
            }
            Pat::Spaces => Some(Hit::new(p, self.ws(p))),
            Pat::Open(rule) => self.begin_at(rule, p),
        }
    }

    /// `\s*(-?(0|[1-9][0-9]*)(\.[0-9]+)?(([Ee])([-+])?[0-9]+)?)`
    fn float(&self, p: usize) -> Option<Hit> {
        let i = self.ws(p);
        let mut e = i + usize::from(self.at(i) == b'-');
        if self.at(e) == b'0' {
            e += 1;
        } else if (b'1'..=b'9').contains(&self.at(e)) {
            while self.at(e).is_ascii_digit() {
                e += 1;
            }
            self.spend(e - i);
        } else {
            return None;
        }
        if self.at(e) == b'.' && self.at(e + 1).is_ascii_digit() {
            e += 1;
            while self.at(e).is_ascii_digit() {
                e += 1;
            }
        }
        if matches!(self.at(e), b'E' | b'e') {
            let sign = usize::from(matches!(self.at(e + 1), b'-' | b'+'));
            if self.at(e + 1 + sign).is_ascii_digit() {
                e += 1 + sign;
                while self.at(e).is_ascii_digit() {
                    e += 1;
                }
            }
        }
        Some(Hit::new(p, e).cap(0, i, e, S::Float))
    }

    /// The begin pattern of `rule` at `p`.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm for each rule of the grammar"
    )]
    fn begin_at(&self, rule: Rule, p: usize) -> Option<Hit> {
        match rule {
            Rule::CommentTriple => self
                .lit(p, b"\"\"\"")
                .then(|| Hit::new(p, p + 3).cap(0, p, p + 3, S::LeadingSpace)),
            Rule::CommentSingle => (self.at(p) == b'"' && p < self.lim)
                .then(|| Hit::new(p, p + 1).cap(0, p, p + 1, S::LeadingSpace)),
            Rule::Fragment => self.begin_fragment(p),
            Rule::DirectiveDefinition => {
                let i = self.ws(p);
                let e = self.keyword(i, b"directive")?;
                let j = self.ws(e);
                if self.at(j) != b'@' {
                    return None;
                }
                let n = self.name_end(j + 1)?;
                Some(Hit::new(p, n).cap(0, i, e, S::KeywordDirective).cap(
                    1,
                    j,
                    n,
                    S::DirectiveName,
                ))
            }
            Rule::DirectiveLocations => {
                let i = self.ws(p);
                let e = self.keyword(i, b"on")?;
                let j = self.ws(e);
                let k = self.letters_end(j);
                Some(
                    Hit::new(p, k)
                        .cap(0, i, e, S::KeywordOn)
                        .cap(1, j, k, S::LocationType),
                )
            }
            Rule::TypeInterface => self.begin_type(p),
            Rule::Implements => {
                let i = self.ws(p);
                let e = self.keyword(i, b"implements")?;
                Some(Hit::new(p, self.ws(e)).cap(0, i, e, S::KeywordImplements))
            }
            Rule::Enum => {
                let i = self.ws(p);
                let e = self.keyword(i, b"enum")?;
                let j = self.ws(e);
                let n = self.name_end(j)?;
                Some(
                    Hit::new(p, n)
                        .cap(0, i, e, S::KeywordEnum)
                        .cap(1, j, n, S::EnumType),
                )
            }
            Rule::EnumBody | Rule::SchemaBody | Rule::TypeObject | Rule::Selection => {
                self.single(p, b'{', S::Operator)
            }
            Rule::Schema => {
                let i = self.ws(p);
                let e = self.keyword(i, b"schema")?;
                Some(Hit::new(p, e).cap(0, i, e, S::KeywordSchema))
            }
            Rule::SchemaField | Rule::TypeField => {
                let i = self.ws(p);
                let e = self.name_end(i)?;
                let scope = if rule == Rule::SchemaField {
                    S::ArgumentsVariable
                } else {
                    S::Variable
                };
                self.field_ahead(e)
                    .then(|| Hit::new(p, e).cap(0, i, e, scope))
            }
            Rule::Union => {
                let i = self.ws(p);
                let e = self.keyword(i, b"union")?;
                let j = self.ws(e);
                let n = self.name_end(j)?;
                Some(
                    Hit::new(p, n)
                        .cap(0, i, e, S::KeywordUnion)
                        .cap(1, j, n, S::SupportType),
                )
            }
            Rule::UnionMembers => {
                let i = self.ws(p);
                if self.at(i) != b'=' {
                    return None;
                }
                let j = self.ws(i + 1);
                let n = self.name_end(j)?;
                Some(
                    Hit::new(p, n)
                        .cap(0, i, i + 1, S::Assignment)
                        .cap(1, j, n, S::SupportType),
                )
            }
            Rule::Directive => {
                let i = self.ws(p);
                if self.at(i) != b'@' {
                    return None;
                }
                let j = self.ws(i + 1);
                let n = self.name_end(j)?;
                Some(Hit::new(p, n).cap(0, i, n, S::DirectiveName))
            }
            Rule::Arguments => self.single(p, b'(', S::BraceDirective),
            Rule::Argument => {
                let i = self.ws(p);
                let e = self.name_end(i)?;
                let k = self.ws(e);
                (self.at(k) == b':').then(|| {
                    Hit::new(p, k + 1)
                        .cap(0, i, e, S::Parameter)
                        .cap(1, k, k + 1, S::Colon)
                })
            }
            Rule::String => {
                let i = self.ws(p);
                (self.at(i) == b'"' && i < self.lim).then(|| {
                    Hit::new(p, i + 1).nested(0, i, i + 1, S::StringDouble, S::StringBegin)
                })
            }
            Rule::List | Rule::TypeList => self.single(p, b'[', S::BraceSquare),
            Rule::Object => self.single(p, b'{', S::BraceCurly),
            Rule::Variables => self.single(p, b'(', S::BraceRound),
            Rule::Variable => {
                let i = self.ws(p);
                let from = i + usize::from(self.at(i) == b'$');
                let e = self.name_end(from)?;
                self.field_ahead(e)
                    .then(|| Hit::new(p, e).cap(0, i, e, S::Parameter))
            }
            Rule::Default => {
                let width = self.space_at(p);
                (width > 0 && self.at(p + width) == b'=').then(|| {
                    let at = p + width;
                    Hit::new(p, at + 1).cap(0, at, at + 1, S::Assignment)
                })
            }
            Rule::Spread => {
                let i = self.ws(p);
                if !self.lit(i, b"...") {
                    return None;
                }
                let j = self.ws(i + 3);
                if self.keyword(j, b"on").is_some() {
                    return None;
                }
                let n = self.name_end(j)?;
                Some(
                    Hit::new(p, n)
                        .cap(0, i, i + 3, S::Spread)
                        .cap(1, j, n, S::FragmentVariable),
                )
            }
            Rule::InlineFragment => {
                let i = self.ws(p);
                if !self.lit(i, b"...") {
                    return None;
                }
                let j = self.ws(i + 3);
                let hit = Hit::new(p, j).cap(0, i, i + 3, S::Spread);
                let on = self.keyword(j, b"on").and_then(|e| {
                    let k = self.ws(e);
                    Some((e, k, self.name_end(k)?))
                });
                Some(match on {
                    Some((e, k, n)) => Hit { end: n, ..hit }.cap(1, j, e, S::KeywordOn).cap(
                        2,
                        k,
                        n,
                        S::SupportType,
                    ),
                    None => hit,
                })
            }
            Rule::Interpolation => {
                let i = self.ws(p);
                (self.at(i) == b'$' && self.at(i + 1) == b'{')
                    .then(|| Hit::new(p, i + 2).cap(0, i, i + 2, S::SubstitutionBegin))
            }
            Rule::Root => None,
        }
    }

    /// `\s*\b(fragment)\b\s*(name)?\s*\b(on)\b\s*(name)`
    fn begin_fragment(&self, p: usize) -> Option<Hit> {
        let i = self.ws(p);
        let e = self.keyword(i, b"fragment")?;
        let j = self.ws(e);
        let finish = |name: Option<(usize, usize)>, on_at: usize| {
            let on_end = self.keyword(on_at, b"on")?;
            let m = self.ws(on_end);
            let n = self.name_end(m)?;
            let mut hit = Hit::new(p, n).cap(0, i, e, S::KeywordFragment);
            if let Some((from, to)) = name {
                hit = hit.cap(1, from, to, S::FragmentName);
            }
            Some(
                hit.cap(2, on_at, on_end, S::KeywordOn)
                    .cap(3, m, n, S::SupportType),
            )
        };
        if let Some(n1) = self.name_end(j)
            && let hit @ Some(_) = finish(Some((j, n1)), self.ws(n1))
        {
            return hit;
        }
        finish(None, j)
    }

    /// `\s*\b(?:(extends?)?\b\s*\b(type)|(interface)|(input))\b\s*(name)?`
    fn begin_type(&self, p: usize) -> Option<Hit> {
        let i = self.ws(p);
        let after = |keyword_end: usize, hit: Hit| {
            if !self.boundary(keyword_end) {
                return None;
            }
            let j = self.ws(keyword_end);
            Some(match self.name_end(j) {
                Some(n) => Hit { end: n, ..hit }.cap(4, j, n, S::SupportType),
                None => Hit { end: j, ..hit },
            })
        };
        if self.boundary(i) {
            // `(extends?)?` takes `extends` or `extend` when it can.
            let extend = if self.lit(i, b"extends") {
                Some(i + 7)
            } else if self.lit(i, b"extend") {
                Some(i + 6)
            } else {
                None
            };
            if let Some(x) = extend.filter(|&x| self.boundary(x)) {
                let t = self.ws(x);
                if self.boundary(t) && self.lit(t, b"type") {
                    let hit = Hit::new(p, t + 4).cap(0, i, x, S::KeywordType).cap(
                        1,
                        t,
                        t + 4,
                        S::KeywordType,
                    );
                    if let found @ Some(_) = after(t + 4, hit) {
                        return found;
                    }
                }
            }
            if self.lit(i, b"type") {
                let hit = Hit::new(p, i + 4).cap(1, i, i + 4, S::KeywordType);
                if let found @ Some(_) = after(i + 4, hit) {
                    return found;
                }
            }
            if self.lit(i, b"interface") {
                let hit = Hit::new(p, i + 9).cap(2, i, i + 9, S::KeywordInterface);
                if let found @ Some(_) = after(i + 9, hit) {
                    return found;
                }
            }
            if self.lit(i, b"input") {
                let hit = Hit::new(p, i + 5).cap(3, i, i + 5, S::KeywordInput);
                return after(i + 5, hit);
            }
        }
        None
    }

    // End patterns.

    /// Whether the rule's end pattern is tried after its patterns.
    pub(super) fn end_last(rule: Rule) -> bool {
        matches!(
            rule,
            Rule::DirectiveDefinition
                | Rule::DirectiveLocations
                | Rule::TypeInterface
                | Rule::Union
                | Rule::UnionMembers
                | Rule::Directive
                | Rule::Default
                | Rule::Spread
                | Rule::InlineFragment
        )
    }

    /// The end pattern of `rule` at `p`.
    pub(super) fn end_at(&self, rule: Rule, p: usize) -> Option<Hit> {
        match rule {
            Rule::Root => None,
            Rule::CommentTriple => self.lit(p, b"\"\"\"").then(|| Hit::new(p, p + 3)),
            Rule::CommentSingle => (self.at(p) == b'"' && p < self.lim).then(|| Hit::new(p, p + 1)),
            Rule::Fragment | Rule::Enum | Rule::Schema => {
                (p > 0 && self.at(p - 1) == b'}').then(|| Hit::new(p, p))
            }
            Rule::DirectiveDefinition
            | Rule::DirectiveLocations
            | Rule::TypeInterface
            | Rule::Union
            | Rule::UnionMembers
            | Rule::Directive
            | Rule::Spread
            | Rule::InlineFragment => (p < self.lim).then(|| Hit::new(p, p)),
            Rule::Implements => {
                let i = self.ws(p);
                (self.at(i) == b'{').then(|| Hit::new(p, i))
            }
            Rule::EnumBody | Rule::SchemaBody | Rule::Selection | Rule::TypeObject => {
                self.single(p, b'}', S::Operator)
            }
            Rule::Arguments => self.single(p, b')', S::BraceDirective),
            Rule::Object => self.single(p, b'}', S::BraceCurly),
            Rule::List => self.single(p, b']', S::BraceSquare),
            Rule::Variables => self.single(p, b')', S::BraceRound),
            Rule::SchemaField | Rule::TypeField => {
                self.separator_end(p, |i| self.at(i) == b'}', false)
            }
            Rule::Variable => self.separator_end(p, |i| matches!(self.at(i), b')' | b'}'), true),
            Rule::Argument => self.argument_end(p),
            Rule::String => {
                let i = self.ws(p);
                (self.at(i) == b'"' && i < self.lim)
                    .then(|| Hit::new(p, i + 1).nested(0, i, i + 1, S::StringDouble, S::StringEnd))
            }
            Rule::TypeList => {
                let i = self.ws(p);
                if self.at(i) != b']' {
                    return None;
                }
                let k = self.ws(i + 1);
                let hit = Hit::new(p, i + 1).cap(0, i, i + 1, S::BraceSquare);
                if self.at(k) == b'!' {
                    Some(Hit { end: k + 1, ..hit }.cap(1, k, k + 1, S::NullType))
                } else {
                    Some(hit)
                }
            }
            Rule::Default => matches!(self.at(p), b'\n' | b')' | b',').then(|| Hit::new(p, p)),
            Rule::Interpolation => (self.at(p) == b'}' && p < self.lim)
                .then(|| Hit::new(p, p + 1).cap(0, p, p + 1, S::SubstitutionEnd)),
        }
    }

    /// `(?=\s*((name)\s*([(:])|(stop)))|\s*(,)`, where a variable's name may
    /// start with `$`: a lookahead at a field or the closing token, or a comma
    /// that ends the rule.
    fn separator_end(&self, p: usize, stop: impl Fn(usize) -> bool, dollar: bool) -> Option<Hit> {
        let i = self.ws(p);
        let from = i + usize::from(dollar && self.at(i) == b'$');
        let field = self
            .name_end(from)
            .is_some_and(|e| matches!(self.at(self.ws(e)), b'(' | b':'));
        if field || stop(i) {
            return Some(Hit::new(p, p));
        }
        (self.at(i) == b',').then(|| Hit::new(p, i + 1).cap(4, i, i + 1, S::Comma))
    }

    /// `(?=\s*(?:(name)\s*(:)|\)))|\s*(,)`
    fn argument_end(&self, p: usize) -> Option<Hit> {
        let i = self.ws(p);
        let next_argument = self
            .name_end(i)
            .is_some_and(|e| self.at(self.ws(e)) == b':');
        if next_argument || self.at(i) == b')' {
            return Some(Hit::new(p, p));
        }
        (self.at(i) == b',').then(|| Hit::new(p, i + 1).cap(2, i, i + 1, S::Comma))
    }
}
