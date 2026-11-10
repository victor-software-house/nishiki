//! The grammar's regular expressions as hand-written matchers.
//!
//! Each matcher answers what its expression would at a position.
//! That includes where its backtracking lands.
//! `Onig`'s `\s`, `\d` and `\w` read non-ASCII characters.
//! Those classes go through the table the JSON lexer measured on Shiki.
//! The keyword rules are case-insensitive for ASCII words.

use memchr::memchr2;

use super::engine::{Closer, Hit, Lexer};
use super::rules::Pattern;
use super::scope::S;
use super::state::Rule;
use super::words::{FUNCTIONS, GLOBALS, KEYWORDS};
use crate::case_fold;
use crate::classes::{digit_width, is_ascii_space, space_width, word_before, word_width};
use crate::language::{Halts, NO_HALTS};
use crate::utf8::{char_len, char_start};

/// The length of the longest word of the keyword, function and global
/// tables.
/// A word longer than it is in none of them.
pub(super) const LONGEST: usize = longest();

/// [`LONGEST`], computed from the tables.
/// A regenerated table then cannot outgrow it.
const fn longest() -> usize {
    let mut max = 0;
    let mut i = 0;
    while i < KEYWORDS.len() {
        max = max_len(max, KEYWORDS[i]);
        i += 1;
    }
    i = 0;
    while i < FUNCTIONS.len() {
        max = max_len(max, FUNCTIONS[i].0);
        i += 1;
    }
    i = 0;
    while i < GLOBALS.len() {
        max = max_len(max, GLOBALS[i]);
        i += 1;
    }
    max
}

/// The greater of `max` and the length of `word`.
const fn max_len(max: usize, word: &str) -> usize {
    if word.len() > max { word.len() } else { max }
}

impl Lexer<'_> {
    // Characters.

    /// The width of the white space character at `i`, 0 when there is none.
    /// The line feed at the end of a line is white space.
    pub(super) fn space_w(&self, i: usize) -> usize {
        if i == self.lim() {
            usize::from(self.nl)
        } else {
            space_width(self.s, i)
        }
    }

    /// Whether a word starts at `i`: a word character after a non-word one.
    ///
    /// It is `#[inline]` because the keyword matchers, in another module, ask
    /// it at most positions.
    /// At the default sixteen codegen units, a generic method that is not
    /// `#[inline]` is called out of line across modules (`bench/AGENTS.md`
    /// rule 13).
    #[inline]
    pub(super) fn word_start(&self, i: usize) -> bool {
        word_width(self.s, i) > 0 && !word_before(self.s, i)
    }

    /// The end of the run of word characters from `i`.
    pub(super) fn word_run(&self, mut i: usize) -> usize {
        let from = i;
        let (start, end) = self.x.run.get();
        if start == from {
            return end;
        }
        while let width @ 1.. = word_width(self.s, i) {
            i += width;
        }
        self.spend(i - from);
        self.x.run.set((from, i));
        i
    }

    /// The end of the run of digits from `i`.
    pub(super) fn digits(&self, mut i: usize) -> usize {
        while let width @ 1.. = digit_width(self.s, i) {
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
    /// case, as `(?i)` reads it.
    /// A character above ASCII stands for the letters it folds to, such as
    /// `ſ` for `s` and `ß` for `ss`.
    pub(super) fn lit(&self, i: usize, word: &str) -> Option<usize> {
        case_fold::literal_end(self.s, i, word.as_bytes())
    }

    /// The regex `\b` after a word that ends at `i`.
    pub(super) fn bnd(&self, i: usize) -> bool {
        word_width(self.s, i) == 0
    }

    /// The first of `words` at `i` (case-insensitive) that a boundary
    /// follows.
    fn word_of(&self, i: usize, words: &[&str]) -> Option<usize> {
        words
            .iter()
            .find_map(|word| self.lit(i, word).filter(|&end| self.bnd(end)))
    }

    /// The word `i..j` lowered in `buf`, each character above ASCII as the
    /// letters it folds to.
    /// It is `None` when the result is longer than any word of the tables.
    /// It is also `None` when a character above ASCII folds to no ASCII
    /// letters.
    pub(super) fn lower<'b>(
        &self,
        i: usize,
        j: usize,
        buf: &'b mut [u8; LONGEST],
    ) -> Option<&'b [u8]> {
        let len = case_fold::lower_into(self.s.get(i..j)?, buf)?;
        buf.get(..len)
    }

    // Skipping.

    /// The first position from `p` where `rule` can match.
    /// The bytes in between start none of its patterns and end it nowhere.
    ///
    /// It is `#[inline]` because the scan in the engine module calls it after
    /// every miss.
    /// At the default sixteen codegen units, a generic method that is not
    /// `#[inline]` is called out of line across modules (`bench/AGENTS.md`
    /// rule 13).
    #[inline]
    pub(super) fn skip_to(&self, rule: Rule, p: usize) -> usize {
        let text = &self.s[p.min(self.lim())..self.lim()];
        let found = match rule {
            Rule::LineComment => None,
            Rule::StrSingle => memchr2(b'\'', b'\\', text),
            Rule::StrBacktick => memchr2(b'`', b'\\', text),
            Rule::StrDouble => memchr2(b'"', b'#', text),
            Rule::StrBrace | Rule::StrModr => memchr2(b'}', b'#', text),
            Rule::Bracket => memchr2(b'[', b']', text),
            Rule::BlockComment => memchr2(b'/', b'*', text),
            Rule::Root => return self.root_skip::<false>(p, &NO_HALTS),
            Rule::CommentDash | Rule::CommentHash => return p,
        };
        let skipped = found.unwrap_or(text.len());
        self.spend(skipped);
        p + skipped
    }

    /// The first position from `p` where a pattern of the root can start.
    ///
    /// When `HALTING`, for SQL that another language embeds, it also stops
    /// where `halt` or `after` holds.
    /// The search then asks there.
    /// Without `HALTING` the tests compile out.
    pub(super) fn root_skip<const HALTING: bool>(&self, mut p: usize, halts: &Halts<'_>) -> usize {
        let from = p;
        while p < self.lim()
            && !self.root_start(p)
            && !(HALTING
                && (halts.0.holds(self.s, p)
                    || halts.1.is_some_and(|after| after.holds(self.s, p))))
        {
            p += char_len(self.s[p]);
        }
        self.spend(p - from);
        p.min(self.lim() + usize::from(self.nl)).max(from)
    }

    /// Whether a pattern of the root can start at `p`.
    ///
    /// `(\w+?)\.(\w+)` needs no boundary.
    /// A search that starts inside a word can therefore match there.
    #[expect(
        clippy::inline_always,
        reason = "measured at one codegen unit: called from both instances of the root skip, the halting one and the plain one, it stopped being inlined, at a call per place"
    )]
    #[inline(always)]
    fn root_start(&self, p: usize) -> bool {
        let b = self.s[p];
        if b >= 0x80 || b.is_ascii_alphanumeric() || b == b'_' {
            return match b {
                b'd' | b'D' | b'N' => true,
                _ => {
                    if self.space_w(p) > 0 {
                        return p == 0 || p == self.pos || self.space_w(p - 1) == 0;
                    }
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
        if start >= self.lim() {
            return start == self.lim() && self.nl;
        }
        start = char_start(self.s, start);
        self.space_w(start) > 0
    }

    // Ends.

    /// The end pattern of `rule` at `p`.
    ///
    /// The arms match, in order:
    /// - `Bracket`: `\]`;
    /// - `CommentDash` and `CommentHash`: `(?!\G)`, anywhere `\G` does not;
    /// - `LineComment`: `\n`;
    /// - `BlockComment`: `\*/`;
    /// - the strings: their closing quote or brace.
    #[inline]
    pub(super) fn end_at(&self, rule: Rule, p: usize) -> Option<Hit> {
        let one = |byte: u8, scope: S| {
            (p < self.lim() && self.s[p] == byte)
                .then(|| Hit::new(p, p + 1).capture(0, p, p + 1, scope))
        };
        match rule {
            Rule::Root => None,
            Rule::Bracket => one(b']', S::PunctBracketEnd),
            Rule::CommentDash | Rule::CommentHash => {
                (!self.g || p != self.pos).then(|| Hit::new(p, p))
            }
            Rule::LineComment => (p == self.lim() && self.nl).then(|| Hit::new(p, p + 1)),
            Rule::BlockComment => self
                .s
                .get(p..self.lim())
                .is_some_and(|rest| rest.starts_with(b"*/"))
                .then(|| Hit::new(p, p + 2).capture(0, p, p + 2, S::PunctComment)),
            Rule::StrSingle => one(b'\'', S::PunctStrEnd),
            Rule::StrBacktick => one(b'`', S::PunctStrEnd),
            Rule::StrDouble => one(b'"', S::PunctStrEnd),
            Rule::StrBrace | Rule::StrModr => one(b'}', S::PunctStrEnd),
        }
    }

    // Patterns.

    /// The match of `pattern` at `p`.
    ///
    /// It is `#[inline]` because `hit`, in the engine module, calls it for
    /// every candidate pattern.
    /// At the default sixteen codegen units, a generic method that is not
    /// `#[inline]` is called out of line across modules (`bench/AGENTS.md`
    /// rule 13).
    /// The matchers it calls that inline at one unit are `#[inline]` too.
    /// Without them, sixteen units ran 11% more instructions than one.
    #[inline]
    pub(super) fn pattern_at(&self, pattern: Pattern, p: usize) -> Option<Hit> {
        match pattern {
            Pattern::Variable => self.variable(p),
            Pattern::BracketChar => self.bracket_char(p),
            Pattern::Create => self.create(p),
            Pattern::Drop => self.drop_object(p),
            Pattern::DropTable => self.drop_table(p),
            Pattern::Alter => self.alter(p),
            Pattern::Types => self.types(p),
            Pattern::Modifier => self.modifier(p),
            Pattern::Number => self.number(p),
            Pattern::Dml => self.dml(p),
            Pattern::Ddl => self.ddl(p),
            Pattern::Values => self.plain_word(p, &["values"]),
            Pattern::Luw => self.luw(p),
            Pattern::Authorization => self.authorization(p),
            Pattern::In => self.plain_word(p, &["in"]),
            Pattern::CommentOn => self.comment_on(p),
            Pattern::As => self.plain_word(p, &["as"]),
            Pattern::Order => self.plain_word(p, &["desc", "asc"]),
            Pattern::Star => self.single(p, b'*'),
            Pattern::Comparison => self.comparison(p),
            Pattern::Math => self
                .single(p, b'-')
                .or_else(|| self.single(p, b'+'))
                .or_else(|| self.single(p, b'/')),
            Pattern::Concat => self.text_hit(p, b"||"),
            Pattern::Function => self.function(p),
            Pattern::DbTable => self.db_table(p),
            Pattern::StrSingleFull => self.single_quoted(p),
            Pattern::StrBacktickFull => self.closed(p, b'`', |b| b == b'`' || b == b'\\'),
            Pattern::StrDoubleFull => self.closed(p, b'"', |b| b == b'"' || b == b'#'),
            Pattern::Keyword => self.keyword(p),
            Pattern::Parens => self.parens(p),
            Pattern::Escape => self.escape(p),
            Pattern::Interpolation => self.interpolation(p),
            Pattern::BracketBegin => self.begin(p, b"[", S::PunctBracketBegin),
            Pattern::CommentDashBegin => self.comment_begin(p, b"--"),
            Pattern::CommentHashBegin => self.comment_begin(p, b"#"),
            Pattern::LineCommentBegin => self.begin(p, b"--", S::PunctComment),
            Pattern::BlockBegin => self.begin(p, b"/*", S::PunctComment),
            Pattern::StrSingleBegin => self.begin(p, b"'", S::PunctStrBegin),
            Pattern::StrBacktickBegin => self.begin(p, b"`", S::PunctStrBegin),
            Pattern::StrDoubleBegin => self.begin(p, b"\"", S::PunctStrBegin),
            Pattern::StrBraceBegin => self.begin(p, b"%{", S::PunctStrBegin),
            Pattern::StrModrBegin => self.begin(p, b"%r{", S::PunctStrBegin),
            Pattern::End => self.end_at(self.top_rule(), p),
        }
    }

    /// A match of the one byte `b`.
    fn single(&self, p: usize, b: u8) -> Option<Hit> {
        (p < self.lim() && self.s[p] == b).then(|| Hit::new(p, p + 1))
    }

    /// A match of the text `text`.
    fn text_hit(&self, p: usize, text: &[u8]) -> Option<Hit> {
        self.s
            .get(p..self.lim())
            .is_some_and(|rest| rest.starts_with(text))
            .then(|| Hit::new(p, p + text.len()))
    }

    /// A rule's begin: `text`, scoped as its punctuation.
    fn begin(&self, p: usize, text: &[u8], scope: S) -> Option<Hit> {
        self.text_hit(p, text)
            .map(|hit| hit.capture(0, p, p + text.len(), scope))
    }

    /// `[!<>]?=|<>|<|>`
    fn comparison(&self, p: usize) -> Option<Hit> {
        let b = *self.s.get(p).filter(|_| p < self.lim())?;
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
        if self.at(p) != b'@' || p >= self.lim() || (p > 0 && self.s[p - 1] == b'@') {
            return None;
        }
        let end = self.word_run(p + 1);
        (end > p + 1).then(|| Hit::new(p, end))
    }

    /// `\[|\]\]` inside brackets.
    fn bracket_char(&self, p: usize) -> Option<Hit> {
        self.single(p, b'[').or_else(|| self.text_hit(p, b"]]"))
    }

    /// `(\()(\))`
    fn parens(&self, p: usize) -> Option<Hit> {
        self.text_hit(p, b"()").map(|hit| {
            hit.capture(0, p, p + 1, S::PunctScopeBegin)
                .capture(1, p + 1, p + 2, S::PunctScopeEnd)
        })
    }

    /// `\\.`
    fn escape(&self, p: usize) -> Option<Hit> {
        (p + 1 < self.lim() && self.s[p] == b'\\')
            .then(|| Hit::new(p, p + 1 + char_len(self.s[p + 1])))
    }

    /// `(#\{)([^\}]*)(\})`
    fn interpolation(&self, p: usize) -> Option<Hit> {
        if !self.s.get(p..self.lim())?.starts_with(b"#{") {
            return None;
        }
        let close = self.find_closer(Closer::Brace, p + 2)?;
        Some(
            Hit::new(p, close + 1)
                .capture(0, p, p + 2, S::PunctStrBegin)
                .capture(1, close, close + 1, S::PunctStrEnd),
        )
    }

    /// `(?:(?<![a-zA-Z0-9_])(N))?(')[^']*(')`
    fn single_quoted(&self, p: usize) -> Option<Hit> {
        let quote = match self.at(p) {
            b'\'' if p < self.lim() => p,
            b'N' if p < self.lim()
                && self.at(p + 1) == b'\''
                && p + 1 < self.lim()
                && !self
                    .prev(p)
                    .is_some_and(|b| b.is_ascii_alphanumeric() || b == b'_') =>
            {
                p + 1
            }
            _ => return None,
        };
        let close = self.find_closer(Closer::Quote, quote + 1)?;
        Some(
            Hit::new(p, close + 1)
                .capture(0, quote, quote + 1, S::PunctStrBegin)
                .capture(1, close, close + 1, S::PunctStrEnd),
        )
    }

    /// `(q)[^q...]*(q)`: a string closed on the line.
    /// Its body has none of the bytes `stop` takes.
    fn closed(&self, p: usize, quote: u8, stop: impl Fn(u8) -> bool) -> Option<Hit> {
        if p >= self.lim() || self.s[p] != quote {
            return None;
        }
        let body = self.s[p + 1..self.lim()].iter().position(|&b| stop(b))?;
        self.spend(body);
        let close = p + 1 + body;
        (self.s[close] == quote).then(|| {
            Hit::new(p, close + 1)
                .capture(0, p, p + 1, S::PunctStrBegin)
                .capture(1, close, close + 1, S::PunctStrEnd)
        })
    }

    /// `(^[ \t]+)?(?=--)` and `(^[ \t]+)?(?=#)`
    fn comment_begin(&self, p: usize, text: &[u8]) -> Option<Hit> {
        if p == 0 && matches!(self.at(0), b' ' | b'\t') {
            let mut end = 0;
            while end < self.lim() && matches!(self.s[end], b' ' | b'\t') {
                end += 1;
            }
            self.spend(end);
            return self
                .s
                .get(end..self.lim())
                .is_some_and(|rest| rest.starts_with(text))
                .then(|| Hit::new(0, end).capture(0, 0, end, S::LeadingSpace));
        }
        self.s
            .get(p..self.lim())
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
    #[inline]
    fn number(&self, p: usize) -> Option<Hit> {
        if !self.word_start(p) || digit_width(self.s, p) == 0 {
            return None;
        }
        let end = self.digits(p);
        self.bnd(end).then(|| Hit::new(p, end))
    }

    /// The alternatives of `alts` that fit at `p`, in the order a regular
    /// expression tries them.
    /// Each gives where the type ends and the end of the white space after it.
    /// When `space` is false, that second end is the match's end.
    /// An optional prefix is tried before the alternative without it.
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
    #[inline]
    fn object_type(&self, p: usize, alts: &[Alt], space: bool) -> Option<(usize, usize)> {
        self.object_types(p, alts, space).next()
    }

    /// The create statement: `create`, an optional `or replace`, an object
    /// type, and a name that a quote may enclose.
    ///
    /// The optional `or replace` is taken when it fits.
    /// It is given back when nothing after it fits.
    /// Each type is given back the same way when no name follows it.
    #[inline]
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
        let (phrase, type_start, type_end, name, name_end, end) = [replaced, Some(created)]
            .into_iter()
            .flatten()
            .find_map(|phrase| {
                let start = self.spaces1(phrase)?;
                self.object_types(start, CREATE_TYPES, true)
                    .find_map(|(type_end, after)| {
                        let (quote, name) = match self.at(after) {
                            q @ (b'\'' | b'"' | b'`') if after < self.lim() => (Some(q), after + 1),
                            _ => (None, after),
                        };
                        let name_end = self.word_run(name);
                        if name_end == name {
                            return None;
                        }
                        let end = match quote {
                            Some(q) if name_end < self.lim() && self.s[name_end] == q => {
                                name_end + 1
                            }
                            Some(_) => return None,
                            None => name_end,
                        };
                        Some((phrase, start, type_end, name, name_end, end))
                    })
            })?;
        Some(
            Hit::new(0, end)
                .capture(0, word, phrase, S::KwCreate)
                .capture(1, type_start, type_end, S::KwOther)
                .capture(2, name, name_end, S::EntityFunction),
        )
    }

    /// `^\s*(drop)\s+(aggregate|...)`
    #[inline]
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
                .capture(0, word, dropped, S::KwCreate)
                .capture(1, start, end, S::KwOther),
        )
    }

    /// `\s*(drop)\s+(table)\s+(\w+)(\s+cascade)?\b`.
    /// It has no `^`.
    /// It starts where the white space before `drop` does.
    #[inline]
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
            .capture(0, word, dropped, S::KwCreate)
            .capture(1, table, table_end, S::KwTable)
            .capture(2, name, name_end, S::EntityFunction);
        if let Some(cascade_end) = cascade {
            hit = hit.capture(3, name_end, cascade_end, S::KwCascade);
        }
        Some(hit)
    }

    /// `^\s*(alter)\s+(aggregate|...)\s+)`
    #[inline]
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
                .capture(0, word, altered, S::KwCreate)
                .capture(1, start, end, S::KwTable),
        )
    }

    /// `^\s*(comment\s+on\s+(table|...))\s+`
    #[inline]
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

    /// `\b(name)\b\s*\(` for a function, or `(?<!@)@@\b(name)\b\s*\(`.
    #[inline]
    fn function(&self, p: usize) -> Option<Hit> {
        let global = self.at(p) == b'@' && p < self.lim();
        let start = if global {
            if (p > 0 && self.s[p - 1] == b'@') || self.at(p + 1) != b'@' || p + 1 >= self.lim() {
                return None;
            }
            p + 2
        } else if self.word_start(p) {
            p
        } else {
            return None;
        };
        if word_width(self.s, start) == 0 {
            return None;
        }
        let end = self.word_run(start);
        let mut buf = [0; LONGEST];
        let word = self.lower(start, end, &mut buf)?;
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
        (self.at(paren) == b'(' && paren < self.lim())
            .then(|| Hit::new(p, paren + 1).capture(0, start, end, scope))
    }

    /// `(\w+?)\.(\w+)`
    #[inline]
    fn db_table(&self, p: usize) -> Option<Hit> {
        if word_width(self.s, p) == 0 || !(p == self.pos || self.word_start(p)) {
            return None;
        }
        let database = self.word_run(p);
        if self.at(database) != b'.'
            || database >= self.lim()
            || word_width(self.s, database + 1) == 0
        {
            return None;
        }
        let table = self.word_run(database + 1);
        Some(
            Hit::new(p, table)
                .capture(0, p, database, S::DatabaseName)
                .capture(1, database + 1, table, S::TableName),
        )
    }

    /// The keywords of the rule that closes the grammar's word rules.
    ///
    /// `priority` matches with the space after it, when a word follows that
    /// space.
    #[inline]
    fn keyword(&self, p: usize) -> Option<Hit> {
        if !self.word_start(p) {
            return None;
        }
        let end = self.word_run(p);
        let mut buf = [0; LONGEST];
        if let Some(word) = self.lower(p, end, &mut buf) {
            if KEYWORDS
                .binary_search_by(|name| name.as_bytes().cmp(word))
                .is_ok()
            {
                return Some(Hit::new(p, self.create_group(word, end)));
            }
            if word == b"priority"
                && self.at(end) == b' '
                && end < self.lim()
                && word_width(self.s, end + 1) > 0
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
            (at < self.lim() && self.s[at] == b'\\').then_some(())?;
            let mut i = at + 1;
            let from = i;
            while i < self.lim() && self.s[i].eq_ignore_ascii_case(&b's') {
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
