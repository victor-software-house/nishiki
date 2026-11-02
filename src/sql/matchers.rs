//! The grammar's regular expressions as hand-written matchers.
//!
//! Each answers what its expression would at a position, including where its
//! backtracking lands. `Onig`'s `\s`, `\d` and `\w` read non-ASCII
//! characters, so they go through the table the JSON lexer measured on Shiki.
//! The keyword rules are case-insensitive for ASCII words.

use memchr::memchr2;

use super::engine::{Hit, Lexer, char_len};
use super::rules::Pat;
use super::scope::S;
use super::state::Rule;
use super::words::{FUNCTIONS, GLOBALS, KEYWORDS};
use crate::classes;

/// The ASCII lowercase letter that `c` matches without regard to case: itself,
/// or `s` for `ſ` and `k` for the Kelvin sign, which `Onig` folds to them.
fn fold(c: char) -> Option<u8> {
    match c {
        'ſ' => Some(b's'),
        '\u{212a}' => Some(b'k'),
        _ => u8::try_from(c).ok().map(|b| b.to_ascii_lowercase()),
    }
}

/// The regex `\s` for an ASCII byte.
fn is_ascii_space(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r')
}

/// The longest word of the keyword and function tables.
pub(super) const LONGEST: usize = 48;

impl Lexer<'_> {
    // Characters.

    /// The character at `i` and its width, where one starts in the text.
    pub(super) fn char_at(&self, i: usize) -> Option<(char, usize)> {
        if i >= self.lim {
            return None;
        }
        let width = char_len(self.s[i]);
        let text = std::str::from_utf8(self.s.get(i..i + width)?).ok()?;
        Some((text.chars().next()?, width))
    }

    /// The width of the white space character at `i`, 0 when there is none.
    /// The line feed at the end of a line is white space.
    pub(super) fn space_w(&self, i: usize) -> usize {
        if i < self.lim {
            let b = self.s[i];
            if b < 0x80 {
                return usize::from(is_ascii_space(b));
            }
            return match self.char_at(i) {
                Some((c, width)) if classes::is_space(c) => width,
                _ => 0,
            };
        }
        usize::from(i == self.lim && self.nl)
    }

    /// The width of the word character at `i`, 0 when there is none.
    pub(super) fn word_w(&self, i: usize) -> usize {
        if i >= self.lim {
            return 0;
        }
        let b = self.s[i];
        if b < 0x80 {
            return usize::from(b.is_ascii_alphanumeric() || b == b'_');
        }
        match self.char_at(i) {
            Some((c, width)) if classes::is_word(c) => width,
            _ => 0,
        }
    }

    /// The width of the digit character at `i`, as `\d` reads it.
    pub(super) fn digit_w(&self, i: usize) -> usize {
        if i >= self.lim {
            return 0;
        }
        let b = self.s[i];
        if b < 0x80 {
            return usize::from(b.is_ascii_digit());
        }
        match self.char_at(i) {
            Some((c, width)) if classes::is_digit(c) => width,
            _ => 0,
        }
    }

    /// Whether the character before `i` is a word character.
    pub(super) fn word_before(&self, i: usize) -> bool {
        let Some(mut start) = i.checked_sub(1) else {
            return false;
        };
        while start > 0 && self.s[start] & 0xc0 == 0x80 {
            start -= 1;
        }
        self.word_w(start) > 0
    }

    /// Whether a word starts at `i`: a word character after a non-word one.
    pub(super) fn word_start(&self, i: usize) -> bool {
        self.word_w(i) > 0 && !self.word_before(i)
    }

    /// The end of the run of word characters from `i`.
    pub(super) fn word_run(&self, mut i: usize) -> usize {
        let from = i;
        let (start, end) = self.run.get();
        if start == from {
            return end;
        }
        while let width @ 1.. = self.word_w(i) {
            i += width;
        }
        self.spend(i - from);
        self.run.set((from, i));
        i
    }

    /// The end of the run of digits from `i`.
    pub(super) fn digits(&self, mut i: usize) -> usize {
        while let width @ 1.. = self.digit_w(i) {
            i += width;
        }
        i
    }

    /// The end of the white space from `i`, the regex `\s*`.
    pub(super) fn spaces(&self, mut i: usize) -> usize {
        let from = i;
        while let width @ 1.. = self.space_w(i) {
            i += width;
        }
        self.spend(i - from);
        i
    }

    /// The end of the white space from `i`, the regex `\s+`.
    pub(super) fn spaces1(&self, i: usize) -> Option<usize> {
        let end = self.spaces(i);
        (end > i).then_some(end)
    }

    /// The end of the ASCII word `word` at `i`, matched without regard to
    /// case, as `(?i)` reads it: `ſ` is an `s` and the Kelvin sign a `k`.
    pub(super) fn lit(&self, i: usize, word: &str) -> Option<usize> {
        let mut at = i;
        for &expected in word.as_bytes() {
            let (c, width) = self.char_at(at)?;
            if fold(c)? != expected.to_ascii_lowercase() {
                return None;
            }
            at += width;
        }
        Some(at)
    }

    /// The regex `\b` after a word that ends at `i`.
    pub(super) fn bnd(&self, i: usize) -> bool {
        self.word_w(i) == 0
    }

    /// The first of `words` at `i` (case-insensitive) that a boundary
    /// follows.
    fn word_of(&self, i: usize, words: &[&str]) -> Option<usize> {
        words
            .iter()
            .find_map(|word| self.lit(i, word).filter(|&end| self.bnd(end)))
    }

    /// The lowercase of the word `i..j` in `buf`, or `None` when it is longer
    /// than any word of the tables or has a character that no ASCII letter
    /// matches without regard to case.
    pub(super) fn lower<'b>(
        &self,
        i: usize,
        j: usize,
        buf: &'b mut [u8; LONGEST],
    ) -> Option<&'b [u8]> {
        let text = std::str::from_utf8(self.s.get(i..j)?).ok()?;
        let mut len = 0;
        for c in text.chars() {
            *buf.get_mut(len)? = fold(c)?;
            len += 1;
        }
        buf.get(..len)
    }

    // Skipping.

    /// The first position from `p` where `rule` can match: the bytes in
    /// between start none of its patterns and end it nowhere.
    pub(super) fn skip(&self, rule: Rule, p: usize) -> usize {
        let text = &self.s[p.min(self.lim)..self.lim];
        let found = match rule {
            Rule::LineComment => None,
            Rule::StrSingle => memchr2(b'\'', b'\\', text),
            Rule::StrBacktick => memchr2(b'`', b'\\', text),
            Rule::StrDouble => memchr2(b'"', b'#', text),
            Rule::StrBrace | Rule::StrModr => memchr2(b'}', b'#', text),
            Rule::Bracket => memchr2(b'[', b']', text),
            Rule::BlockComment => memchr2(b'/', b'*', text),
            Rule::Root => return self.root_skip(p),
            Rule::CommentDash | Rule::CommentHash => return p,
        };
        let skipped = found.unwrap_or(text.len());
        self.spend(skipped);
        p + skipped
    }

    /// The first position from `p` where a pattern of the root can start.
    fn root_skip(&self, mut p: usize) -> usize {
        let from = p;
        while p < self.lim && !self.root_start(p) {
            p += char_len(self.s[p]);
        }
        self.spend(p - from);
        p.min(self.lim + usize::from(self.nl)).max(from)
    }

    /// Whether a pattern of the root can start at `p`.
    fn root_start(&self, p: usize) -> bool {
        let b = self.s[p];
        if b >= 0x80 || b.is_ascii_alphanumeric() || b == b'_' {
            return match b {
                b'd' | b'D' | b'N' => true,
                _ => {
                    if self.space_w(p) > 0 {
                        return p == 0 || p == self.pos || self.space_w(p - 1) == 0;
                    }
                    // `(\w+?)\.(\w+)` needs no boundary, so a search that
                    // starts inside a word can match there.
                    p == self.pos || self.word_start(p)
                }
            };
        }
        if is_ascii_space(b) {
            return p == 0 || p == self.pos || !self.after_space(p);
        }
        matches!(
            b,
            b'@' | b'['
                | b'-'
                | b'#'
                | b'/'
                | b'*'
                | b'='
                | b'<'
                | b'>'
                | b'!'
                | b'+'
                | b'|'
                | b'\''
                | b'`'
                | b'"'
                | b'%'
                | b'('
        )
    }

    /// Whether the character before `p` is white space.
    fn after_space(&self, p: usize) -> bool {
        let Some(mut start) = p.checked_sub(1) else {
            return false;
        };
        if start >= self.lim {
            return start == self.lim && self.nl;
        }
        while start > 0 && self.s[start] & 0xc0 == 0x80 {
            start -= 1;
        }
        self.space_w(start) > 0
    }

    // Ends.

    /// The end pattern of `rule` at `p`.
    pub(super) fn end_at(&self, rule: Rule, p: usize) -> Option<Hit> {
        let one = |byte: u8, scope: S| {
            (p < self.lim && self.s[p] == byte).then(|| Hit::new(p, p + 1).cap(0, p, p + 1, scope))
        };
        match rule {
            Rule::Root => None,
            // `\]`
            Rule::Bracket => one(b']', S::PunctBracketEnd),
            // `(?!\G)`: it matches anywhere `\G` does not.
            Rule::CommentDash | Rule::CommentHash => {
                (!self.g || p != self.pos).then(|| Hit::new(p, p))
            }
            // `\n`
            Rule::LineComment => (p == self.lim && self.nl).then(|| Hit::new(p, p + 1)),
            // `\*/`
            Rule::BlockComment => self
                .s
                .get(p..self.lim)
                .is_some_and(|rest| rest.starts_with(b"*/"))
                .then(|| Hit::new(p, p + 2).cap(0, p, p + 2, S::PunctComment)),
            Rule::StrSingle => one(b'\'', S::PunctStrEnd),
            Rule::StrBacktick => one(b'`', S::PunctStrEnd),
            Rule::StrDouble => one(b'"', S::PunctStrEnd),
            Rule::StrBrace | Rule::StrModr => one(b'}', S::PunctStrEnd),
        }
    }

    // Patterns.

    /// The match of `pat` at `p`.
    pub(super) fn pat_at(&self, pat: Pat, p: usize) -> Option<Hit> {
        match pat {
            Pat::Variable => self.variable(p),
            Pat::BracketChar => self.bracket_char(p),
            Pat::Create => self.create(p),
            Pat::Drop => self.drop_object(p),
            Pat::DropTable => self.drop_table(p),
            Pat::Alter => self.alter(p),
            Pat::Types => self.types(p),
            Pat::Modifier => self.modifier(p),
            Pat::Number => self.number(p),
            Pat::Dml => self.dml(p),
            Pat::Ddl => self.ddl(p),
            Pat::Values => self.plain_word(p, &["values"]),
            Pat::Luw => self.luw(p),
            Pat::Authorization => self.authorization(p),
            Pat::In => self.plain_word(p, &["in"]),
            Pat::CommentOn => self.comment_on(p),
            Pat::As => self.plain_word(p, &["as"]),
            Pat::Order => self.plain_word(p, &["desc", "asc"]),
            Pat::Star => self.single(p, b'*'),
            Pat::Comparison => self.comparison(p),
            Pat::Math => self
                .single(p, b'-')
                .or_else(|| self.single(p, b'+'))
                .or_else(|| self.single(p, b'/')),
            Pat::Concat => self.text(p, b"||"),
            Pat::Function => self.function(p),
            Pat::DbTable => self.db_table(p),
            Pat::StrSingleFull => self.single_quoted(p),
            Pat::StrBacktickFull => self.closed(p, b'`', |b| b == b'`' || b == b'\\'),
            Pat::StrDoubleFull => self.closed(p, b'"', |b| b == b'"' || b == b'#'),
            Pat::Keyword => self.keyword(p),
            Pat::Parens => self.parens(p),
            Pat::Escape => self.escape(p),
            Pat::Interpolation => self.interpolation(p),
            Pat::BracketBegin => self.begin(p, b"[", S::PunctBracketBegin),
            Pat::CommentDashBegin => self.comment_begin(p, b"--"),
            Pat::CommentHashBegin => self.comment_begin(p, b"#"),
            Pat::LineCommentBegin => self.begin(p, b"--", S::PunctComment),
            Pat::BlockBegin => self.begin(p, b"/*", S::PunctComment),
            Pat::StrSingleBegin => self.begin(p, b"'", S::PunctStrBegin),
            Pat::StrBacktickBegin => self.begin(p, b"`", S::PunctStrBegin),
            Pat::StrDoubleBegin => self.begin(p, b"\"", S::PunctStrBegin),
            Pat::StrBraceBegin => self.begin(p, b"%{", S::PunctStrBegin),
            Pat::StrModrBegin => self.begin(p, b"%r{", S::PunctStrBegin),
        }
    }

    /// A match of the one byte `b`.
    fn single(&self, p: usize, b: u8) -> Option<Hit> {
        (p < self.lim && self.s[p] == b).then(|| Hit::new(p, p + 1))
    }

    /// A match of the text `text`.
    fn text(&self, p: usize, text: &[u8]) -> Option<Hit> {
        self.s
            .get(p..self.lim)
            .is_some_and(|rest| rest.starts_with(text))
            .then(|| Hit::new(p, p + text.len()))
    }

    /// A rule's begin: `text`, scoped as its punctuation.
    fn begin(&self, p: usize, text: &[u8], scope: S) -> Option<Hit> {
        self.text(p, text)
            .map(|hit| hit.cap(0, p, p + text.len(), scope))
    }

    /// `[!<>]?=|<>|<|>`
    fn comparison(&self, p: usize) -> Option<Hit> {
        let b = *self.s.get(p).filter(|_| p < self.lim)?;
        let next = self.at(p + 1);
        let width = if next == b'=' && matches!(b, b'!' | b'<' | b'>') {
            2
        } else if b == b'=' {
            1
        } else if b == b'<' && next == b'>' {
            2
        } else if matches!(b, b'<' | b'>') {
            1
        } else {
            return None;
        };
        Some(Hit::new(p, p + width))
    }

    /// `((?<!@)@)\b(\w+)\b`
    fn variable(&self, p: usize) -> Option<Hit> {
        if self.at(p) != b'@' || p >= self.lim || (p > 0 && self.s[p - 1] == b'@') {
            return None;
        }
        let end = self.word_run(p + 1);
        (end > p + 1).then(|| Hit::new(p, end))
    }

    /// `\[|\]\]` inside brackets.
    fn bracket_char(&self, p: usize) -> Option<Hit> {
        self.single(p, b'[').or_else(|| self.text(p, b"]]"))
    }

    /// `(\()(\))`
    fn parens(&self, p: usize) -> Option<Hit> {
        self.text(p, b"()").map(|hit| {
            hit.cap(0, p, p + 1, S::PunctScopeBegin)
                .cap(1, p + 1, p + 2, S::PunctScopeEnd)
        })
    }

    /// `\\.`
    fn escape(&self, p: usize) -> Option<Hit> {
        (p + 1 < self.lim && self.s[p] == b'\\')
            .then(|| Hit::new(p, p + 1 + char_len(self.s[p + 1])))
    }

    /// `(#\{)([^\}]*)(\})`
    fn interpolation(&self, p: usize) -> Option<Hit> {
        if !self.s.get(p..self.lim)?.starts_with(b"#{") {
            return None;
        }
        let close = self.find_byte(1, b'}', p + 2)?;
        Some(
            Hit::new(p, close + 1)
                .cap(0, p, p + 2, S::PunctStrBegin)
                .cap(1, close, close + 1, S::PunctStrEnd),
        )
    }

    /// `(?:(?<![a-zA-Z0-9_])(N))?(')[^']*(')`
    fn single_quoted(&self, p: usize) -> Option<Hit> {
        let quote = match self.at(p) {
            b'\'' if p < self.lim => p,
            b'N' if p < self.lim
                && self.at(p + 1) == b'\''
                && p + 1 < self.lim
                && !self
                    .prev(p)
                    .is_some_and(|b| b.is_ascii_alphanumeric() || b == b'_') =>
            {
                p + 1
            }
            _ => return None,
        };
        let close = self.find_byte(0, b'\'', quote + 1)?;
        Some(
            Hit::new(p, close + 1)
                .cap(0, quote, quote + 1, S::PunctStrBegin)
                .cap(1, close, close + 1, S::PunctStrEnd),
        )
    }

    /// `(q)[^q...]*(q)`: a string closed on the line, whose body has none of
    /// the bytes `stop` takes.
    fn closed(&self, p: usize, quote: u8, stop: impl Fn(u8) -> bool) -> Option<Hit> {
        if p >= self.lim || self.s[p] != quote {
            return None;
        }
        let body = self.s[p + 1..self.lim].iter().position(|&b| stop(b))?;
        self.spend(body);
        let close = p + 1 + body;
        (self.s[close] == quote).then(|| {
            Hit::new(p, close + 1)
                .cap(0, p, p + 1, S::PunctStrBegin)
                .cap(1, close, close + 1, S::PunctStrEnd)
        })
    }

    /// `(^[ \t]+)?(?=--)` and `(^[ \t]+)?(?=#)`
    fn comment_begin(&self, p: usize, text: &[u8]) -> Option<Hit> {
        if p == 0 && matches!(self.at(0), b' ' | b'\t') {
            let mut end = 0;
            while end < self.lim && matches!(self.s[end], b' ' | b'\t') {
                end += 1;
            }
            self.spend(end);
            return self
                .s
                .get(end..self.lim)
                .is_some_and(|rest| rest.starts_with(text))
                .then(|| Hit::new(0, end).cap(0, 0, end, S::LeadingSpace));
        }
        self.s
            .get(p..self.lim)
            .is_some_and(|rest| rest.starts_with(text))
            .then(|| Hit::new(p, p))
    }

    // Words.

    /// A whole word of `words`, case-insensitive: `\b(word|...)\b`.
    fn plain_word(&self, p: usize, words: &[&str]) -> Option<Hit> {
        if !self.word_start(p) {
            return None;
        }
        self.word_of(p, words).map(|end| Hit::new(p, end))
    }

    /// `\b\d+\b`
    fn number(&self, p: usize) -> Option<Hit> {
        if !self.word_start(p) || self.digit_w(p) == 0 {
            return None;
        }
        let end = self.digits(p);
        self.bnd(end).then(|| Hit::new(p, end))
    }

    /// The alternatives of `alts` that fit at `p`, in the order a regular
    /// expression tries them: where the type ends, and the end of the white
    /// space after it, which is the end when `space` is false. An optional
    /// prefix is tried before the alternative without it.
    fn object_types<'a>(
        &'a self,
        p: usize,
        alts: &'a [Alt],
        space: bool,
    ) -> impl Iterator<Item = (usize, usize)> + 'a {
        alts.iter().flat_map(move |alt| {
            let with = alt
                .prefix
                .and_then(|prefix| self.lit(p, prefix))
                .and_then(|end| self.spaces1(end))
                .and_then(|at| self.lit(at, alt.words));
            [with, self.lit(p, alt.words)]
                .into_iter()
                .flatten()
                .filter_map(move |end| {
                    if space {
                        self.spaces1(end).map(|after| (end, after))
                    } else {
                        Some((end, end))
                    }
                })
        })
    }

    /// The first alternative of `alts` at `p` that fits.
    fn object_type(&self, p: usize, alts: &[Alt], space: bool) -> Option<(usize, usize)> {
        self.object_types(p, alts, space).next()
    }

    /// The create statement: `create`, an optional `or replace`, an object
    /// type, and a name that a quote may enclose.
    fn create(&self, p: usize) -> Option<Hit> {
        if p != 0 {
            return None;
        }
        let word = self.spaces(0);
        let created = self.lit(word, "create")?;
        let replaced = self
            .spaces1(created)
            .and_then(|at| self.lit(at, "or"))
            .and_then(|at| self.spaces1(at))
            .and_then(|at| self.lit(at, "replace"));
        // The optional `or replace` is taken when it fits, and given back when
        // nothing after it does; so is each type, when no name follows it.
        let (phrase, kind_start, kind_end, name, name_end, end) = [replaced, Some(created)]
            .into_iter()
            .flatten()
            .find_map(|phrase| {
                let start = self.spaces1(phrase)?;
                self.object_types(start, CREATE_TYPES, true)
                    .find_map(|(kind_end, after)| {
                        let (quote, name) = match self.at(after) {
                            q @ (b'\'' | b'"' | b'`') if after < self.lim => (Some(q), after + 1),
                            _ => (None, after),
                        };
                        let name_end = self.word_run(name);
                        if name_end == name {
                            return None;
                        }
                        let end = match quote {
                            Some(q) if name_end < self.lim && self.s[name_end] == q => name_end + 1,
                            Some(_) => return None,
                            None => name_end,
                        };
                        Some((phrase, start, kind_end, name, name_end, end))
                    })
            })?;
        Some(
            Hit::new(0, end)
                .cap(0, word, phrase, S::KwCreate)
                .cap(1, kind_start, kind_end, S::KwOther)
                .cap(2, name, name_end, S::EntityFunction),
        )
    }

    /// `^\s*(drop)\s+(aggregate|...)`
    fn drop_object(&self, p: usize) -> Option<Hit> {
        if p != 0 {
            return None;
        }
        let word = self.spaces(0);
        let dropped = self.lit(word, "drop")?;
        let start = self.spaces1(dropped)?;
        let (end, _) = self.object_type(start, DROP_TYPES, false)?;
        Some(
            Hit::new(0, end)
                .cap(0, word, dropped, S::KwCreate)
                .cap(1, start, end, S::KwOther),
        )
    }

    /// `\s*(drop)\s+(table)\s+(\w+)(\s+cascade)?\b`: it has no `^`, so it
    /// starts where the white space before `drop` does.
    fn drop_table(&self, p: usize) -> Option<Hit> {
        let starts_run = p == 0 || p == self.pos || !self.after_space(p);
        let space = self.space_w(p) > 0;
        if (space && !starts_run) || (!space && !matches!(self.at(p), b'd' | b'D')) {
            return None;
        }
        let word = self.spaces(p);
        let dropped = self.lit(word, "drop")?;
        let table = self.spaces1(dropped)?;
        let table_end = self.lit(table, "table")?;
        let name = self.spaces1(table_end)?;
        let name_end = self.word_run(name);
        if name_end == name {
            return None;
        }
        let cascade = self
            .spaces1(name_end)
            .and_then(|at| self.lit(at, "cascade"))
            .filter(|&end| self.bnd(end));
        let end = cascade.unwrap_or(name_end);
        let mut hit = Hit::new(p, end)
            .cap(0, word, dropped, S::KwCreate)
            .cap(1, table, table_end, S::KwTable)
            .cap(2, name, name_end, S::EntityFunction);
        if let Some(cascade_end) = cascade {
            hit = hit.cap(3, name_end, cascade_end, S::KwCascade);
        }
        Some(hit)
    }

    /// `^\s*(alter)\s+(aggregate|...)\s+)`
    fn alter(&self, p: usize) -> Option<Hit> {
        if p != 0 {
            return None;
        }
        let word = self.spaces(0);
        let altered = self.lit(word, "alter")?;
        let start = self.spaces1(altered)?;
        let (end, after) = self.object_type(start, ALTER_TYPES, true)?;
        Some(
            Hit::new(0, after)
                .cap(0, word, altered, S::KwCreate)
                .cap(1, start, end, S::KwTable),
        )
    }

    /// `^\s*(comment\s+on\s+(table|...))\s+`
    fn comment_on(&self, p: usize) -> Option<Hit> {
        if p != 0 {
            return None;
        }
        let word = self.spaces(0);
        let start = self
            .lit(word, "comment")
            .and_then(|at| self.spaces1(at))
            .and_then(|at| self.lit(at, "on"))
            .and_then(|at| self.spaces1(at))?;
        let (_, after) = self.object_type(start, COMMENT_TYPES, true)?;
        Some(Hit::new(0, after))
    }

    // The words of the other statements.

    /// The text of the word run at `p` in lowercase, for the tables.
    fn lowered<'b>(&self, p: usize, end: usize, buf: &'b mut [u8; LONGEST]) -> Option<&'b [u8]> {
        self.lower(p, end, buf)
    }

    /// `\b(name)\b\s*\(` for a function, or `(?<!@)@@\b(name)\b\s*\(`.
    fn function(&self, p: usize) -> Option<Hit> {
        let global = self.at(p) == b'@' && p < self.lim;
        let start = if global {
            if (p > 0 && self.s[p - 1] == b'@') || self.at(p + 1) != b'@' || p + 1 >= self.lim {
                return None;
            }
            p + 2
        } else if self.word_start(p) {
            p
        } else {
            return None;
        };
        if self.word_w(start) == 0 {
            return None;
        }
        let end = self.word_run(start);
        let mut buf = [0; LONGEST];
        let word = self.lowered(start, end, &mut buf)?;
        let scope = if global {
            GLOBALS
                .binary_search_by(|name| name.as_bytes().cmp(word))
                .ok()
                .map(|_| S::FnGlobalvar)
        } else {
            FUNCTIONS
                .binary_search_by(|(name, _)| name.as_bytes().cmp(word))
                .ok()
                .and_then(|index| FUNCTIONS.get(index))
                .map(|&(_, scope)| scope)
        }?;
        let paren = self.spaces(end);
        (self.at(paren) == b'(' && paren < self.lim)
            .then(|| Hit::new(p, paren + 1).cap(0, start, end, scope))
    }

    /// `(\w+?)\.(\w+)`
    fn db_table(&self, p: usize) -> Option<Hit> {
        if self.word_w(p) == 0 || !(p == self.pos || self.word_start(p)) {
            return None;
        }
        let database = self.word_run(p);
        if self.at(database) != b'.' || database >= self.lim || self.word_w(database + 1) == 0 {
            return None;
        }
        let table = self.word_run(database + 1);
        Some(Hit::new(p, table).cap(0, p, database, S::DatabaseName).cap(
            1,
            database + 1,
            table,
            S::TableName,
        ))
    }

    /// The keywords of the rule that closes the grammar's word rules.
    fn keyword(&self, p: usize) -> Option<Hit> {
        if !self.word_start(p) {
            return None;
        }
        let end = self.word_run(p);
        let mut buf = [0; LONGEST];
        if let Some(word) = self.lowered(p, end, &mut buf) {
            if KEYWORDS
                .binary_search_by(|name| name.as_bytes().cmp(word))
                .is_ok()
            {
                return Some(Hit::new(p, self.create_group(word, end)));
            }
            // `priority ` with its space, which a word follows.
            if word == b"priority"
                && self.at(end) == b' '
                && end < self.lim
                && self.word_w(end + 1) > 0
            {
                return Some(Hit::new(p, end + 1));
            }
        }
        None
    }

    /// The end of `create` with the group the grammar writes with a literal
    /// backslash: `\s+or\s+alter`, a backslash and `s`s, `or`, a backslash
    /// and `s`s, `alter`, and a boundary.
    fn create_group(&self, word: &[u8], end: usize) -> usize {
        if word != b"create" {
            return end;
        }
        let skip = |at: usize| -> Option<usize> {
            (at < self.lim && self.s[at] == b'\\').then_some(())?;
            let mut i = at + 1;
            let from = i;
            while i < self.lim && self.s[i].eq_ignore_ascii_case(&b's') {
                i += 1;
            }
            (i > from).then_some(i)
        };
        skip(end)
            .and_then(|at| self.lit(at, "or"))
            .and_then(skip)
            .and_then(|at| self.lit(at, "alter"))
            .filter(|&at| self.bnd(at))
            .unwrap_or(end)
    }
}

/// An alternative of an object type: its words, with the single space the
/// grammar writes between them, and the prefix the grammar makes optional.
struct Alt {
    prefix: Option<&'static str>,
    words: &'static str,
}

const fn alt(words: &'static str) -> Alt {
    Alt {
        prefix: None,
        words,
    }
}

const CREATE_TYPES: &[Alt] = &[
    alt("aggregate"),
    alt("conversion"),
    alt("database"),
    alt("domain"),
    alt("function"),
    alt("group"),
    Alt {
        prefix: Some("unique"),
        words: "index",
    },
    alt("language"),
    alt("operator class"),
    alt("operator"),
    alt("rule"),
    alt("schema"),
    alt("sequence"),
    alt("table"),
    alt("tablespace"),
    alt("trigger"),
    alt("type"),
    alt("user"),
    alt("view"),
];

const DROP_TYPES: &[Alt] = &[
    alt("aggregate"),
    alt("conversion"),
    alt("database"),
    alt("domain"),
    alt("function"),
    alt("group"),
    alt("index"),
    alt("language"),
    alt("operator class"),
    alt("operator"),
    alt("rule"),
    alt("schema"),
    alt("sequence"),
    alt("table"),
    alt("tablespace"),
    alt("trigger"),
    alt("type"),
    alt("user"),
    alt("view"),
];

const ALTER_TYPES: &[Alt] = &[
    alt("aggregate"),
    alt("conversion"),
    alt("database"),
    alt("domain"),
    alt("function"),
    alt("group"),
    alt("index"),
    alt("language"),
    alt("operator class"),
    alt("operator"),
    alt("procedure"),
    alt("proc"),
    alt("rule"),
    alt("schema"),
    alt("sequence"),
    alt("table"),
    alt("tablespace"),
    alt("trigger"),
    alt("type"),
    alt("user"),
    alt("view"),
];

const COMMENT_TYPES: &[Alt] = &[
    alt("table"),
    alt("column"),
    alt("aggregate"),
    alt("constraint"),
    alt("database"),
    alt("domain"),
    alt("function"),
    alt("index"),
    alt("operator"),
    alt("rule"),
    alt("schema"),
    alt("sequence"),
    alt("trigger"),
    alt("type"),
    alt("view"),
];
