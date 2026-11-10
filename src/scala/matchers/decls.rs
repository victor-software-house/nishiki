//! Declarations, modifiers and the names that follow keywords.

use super::super::engine::Lexer;
use super::names::{is_lower_start, is_name_start, is_upper_start};
use crate::rules::Chars;

/// The words `\b(synchronized|@volatile|…)\b` lists, in its order.
const OTHER_MODIFIERS: &[&[u8]] = &[
    b"synchronized",
    b"@volatile",
    b"abstract",
    b"final",
    b"lazy",
    b"sealed",
    b"implicit",
    b"override",
    b"@transient",
    b"@native",
];

/// The soft modifiers `\b(transparent|opaque|…)\b` lists.
const SOFT_MODIFIERS: &[&[u8]] = &[
    b"transparent",
    b"opaque",
    b"infix",
    b"open",
    b"inline",
    b"into",
];

/// The declaration keywords a soft modifier must come before.
const DECLARATION_WORDS: &[&[u8]] = &[
    b"def", b"val", b"var", b"given", b"type", b"class", b"trait", b"object", b"enum",
];

/// The declaration keywords that end the search for `if` or `match` after an
/// `inline`.
const INLINE_STOPS: &[&[u8]] = &[b"val", b"def", b"given"];

impl Lexer<'_> {
    /// The end of the word `word` at `i` with `\b` on both sides.
    pub(super) fn word(&self, i: usize, word: &[u8]) -> Option<usize> {
        self.keyword(i, word)
    }

    /// Whether `/*` or `//` starts at `i`.
    fn comment_at(&self, i: usize) -> bool {
        self.at(i) == b'/' && matches!(self.at(i + 1), b'*' | b'/')
    }

    /// `\s*(?!/[*/])` after a keyword that ends at `keyword`.
    ///
    /// It returns where the blanks end and whether a name may follow.
    /// When a comment follows the blanks, `\s*` gives one character back.
    /// No name starts with a blank, so the match ends there without one.
    fn blanks_before_name(&self, keyword: usize) -> Option<(usize, bool)> {
        let blanks = self.space_end(keyword);
        if !self.comment_at(blanks) {
            return Some((blanks, true));
        }
        (blanks > keyword).then(|| (self.char_start_before(blanks), false))
    }

    /// `\b(keyword)\b\s*(?!/[*/])((?:plain|[op]+)|quoted)?`, the form of
    /// `def`, `trait` and `type`: the keyword is group 1 and the name group 2.
    pub(super) fn named_declaration(&mut self, p: usize, keyword: &[u8]) -> Option<usize> {
        let word = self.word(p, keyword)?;
        self.declaration_name(p, word, 2)
    }

    /// The `\s*(?!/[*/])(name)?` after a declaration keyword that runs from
    /// `p` to `word`: the keyword is group 1 and the name group `group`.
    fn declaration_name(&mut self, p: usize, word: usize, group: usize) -> Option<usize> {
        let (blanks, named) = self.blanks_before_name(word)?;
        self.capture(1, p, word);
        if named && let Some(end) = self.any_name(blanks) {
            self.capture(group, blanks, end);
            return Some(end);
        }
        Some(blanks)
    }

    /// `(?<!\.)\b(type)\b\s*(?!/[*/])(name)?`.
    pub(super) fn type_declaration(&mut self, p: usize) -> Option<usize> {
        if self.prev_is_byte(p, b'.') {
            return None;
        }
        self.named_declaration(p, b"type")
    }

    /// `\b(?:(case)\s+)?(class|object|enum)\b\s*(?!/[*/])(name)?`.
    ///
    /// `case` and its blanks come first.
    /// `\s+` gives blanks back only to a keyword that starts with a blank,
    /// which none does.
    pub(super) fn class_declaration(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let mut start = p;
        let mut case = None;
        if let Some(word) = self.lit(p, b"case") {
            let blanks = self.space_end(word);
            if blanks > word && self.class_word(blanks).is_some() {
                case = Some(word);
                start = blanks;
            }
        }
        let word = self.class_word(start)?;
        let (blanks, named) = self.blanks_before_name(word)?;
        if let Some(case) = case {
            self.capture(1, p, case);
        }
        self.capture(2, start, word);
        if named && let Some(end) = self.any_name(blanks) {
            self.capture(3, blanks, end);
            return Some(end);
        }
        Some(blanks)
    }

    /// The end of `(class|object|enum)\b` at `i`.
    fn class_word(&self, i: usize) -> Option<usize> {
        [b"class".as_slice(), b"object", b"enum"]
            .iter()
            .find_map(|word| self.lit(i, word).filter(|&end| self.boundary(end)))
    }

    /// `\b(package)\s+(object)\b\s*(?!/[*/])(name)?`.
    pub(super) fn package_object(&mut self, p: usize) -> Option<usize> {
        let package = self.lit(p, b"package").filter(|_| self.boundary(p))?;
        let blanks = self.space_end(package);
        if blanks == package {
            return None;
        }
        let word = self.word(blanks, b"object")?;
        let (name, named) = self.blanks_before_name(word)?;
        self.capture(1, p, package);
        self.capture(2, blanks, word);
        if named && let Some(end) = self.any_name(name) {
            self.capture(3, name, end);
            return Some(end);
        }
        Some(name)
    }

    /// `\b(?:(val)|(var))\b\s*(?!/[*/])(?=(?:(?:plain|[op]+)|quoted)?\()`.
    ///
    /// The lookahead needs a `(` after the name or in its place.
    /// When a comment follows the blanks, `\s*` gives a blank back, and
    /// neither a name nor `(` starts there.
    pub(super) fn val_or_var_call(&mut self, p: usize) -> Option<usize> {
        let (word, group) = self
            .word(p, b"val")
            .map(|end| (end, 1))
            .or_else(|| self.word(p, b"var").map(|end| (end, 2)))?;
        let (blanks, named) = self.blanks_before_name(word)?;
        if !named {
            return None;
        }
        let paren = |end: usize| self.at(end) == b'(' && end < self.lim();
        if self.any_name_where(blanks, paren).is_none() && !paren(blanks) {
            return None;
        }
        self.capture(group, p, word);
        Some(blanks)
    }

    /// `\b(keyword)\b\s*(?!/[*/])((?:name)(?:\s*,\s*(?:name))*)?(?!")`, the
    /// form of `val` and `var`: the keyword is group 1 and the names group 2.
    ///
    /// `(?!")` refuses a `"` after the last name.
    /// A backtracking matcher then gives one character of that name back.
    /// A name of one character is dropped with the comma before it, or, as
    /// the first name, leaves no group.
    pub(super) fn val_or_var(&mut self, p: usize, keyword: &[u8]) -> Option<usize> {
        let word = self.word(p, keyword)?;
        let (blanks, named) = self.blanks_before_name(word)?;
        let mut list_end = None;
        if named && let Some(first) = self.any_name(blanks) {
            let mut last = (blanks, first);
            let mut before_last = None;
            loop {
                let comma = self.space_end(last.1);
                if self.at(comma) != b',' || comma >= self.lim() {
                    break;
                }
                let next = self.space_end(comma + 1);
                let Some(end) = self.any_name(next) else {
                    break;
                };
                before_last = Some(last.1);
                last = (next, end);
            }
            list_end = Some(self.drop_before_quote(last, before_last));
        }
        let end = match list_end {
            Some(Some(end)) => {
                self.capture(2, blanks, end);
                end
            }
            Some(None) | None => {
                if self.at(blanks) == b'"' && blanks < self.lim() {
                    if blanks == word {
                        return None;
                    }
                    self.char_start_before(blanks)
                } else {
                    blanks
                }
            }
        };
        self.capture(1, p, word);
        Some(end)
    }

    /// The end of the names of a `val` or `var` list after `(?!")`, from the
    /// last name `(from, end)` and the end of the name before it.
    /// `None` when no name is left.
    fn drop_before_quote(
        &self,
        (from, end): (usize, usize),
        before_last: Option<usize>,
    ) -> Option<usize> {
        if self.at(end) != b'"' || end >= self.lim() {
            return Some(end);
        }
        let quoted = self.at(from) == b'`';
        let shorter = self.char_start_before(end);
        if !quoted && shorter > from {
            return Some(shorter);
        }
        before_last
    }

    /// `\b(given)\b\s*([$_a-z\p{Lo}\p{Nl}\p{Ll}]idrest|quoted)?`.
    pub(super) fn given_declaration(&mut self, p: usize) -> Option<usize> {
        let word = self.word(p, b"given")?;
        let blanks = self.space_end(word);
        self.capture(1, p, word);
        let name = self
            .name_with(blanks, is_lower_start)
            .or_else(|| self.quoted_name(blanks));
        if let Some(end) = name {
            self.capture(2, blanks, end);
            return Some(end);
        }
        Some(blanks)
    }

    /// `\b(keyword)\s+`: the begin of the package, import and export rules.
    pub(super) fn keyword_and_blanks(&mut self, p: usize, keyword: &[u8]) -> Option<usize> {
        let word = self.lit(p, keyword).filter(|_| self.boundary(p))?;
        let end = self.space_end(word);
        if end == word {
            return None;
        }
        self.capture(1, p, word);
        Some(end)
    }

    /// `\b(extends|with|derives)\b\s*(upper|quoted|(?=…)|(?=…)|(?="))?`.
    ///
    /// The three last alternatives are lookaheads.
    /// They match the empty text, so only an upper-case or quoted name
    /// moves the end.
    pub(super) fn inheritance(&mut self, p: usize) -> Option<usize> {
        let word = [b"extends".as_slice(), b"with", b"derives"]
            .iter()
            .find_map(|word| self.word(p, word))?;
        let blanks = self.space_end(word);
        self.capture(1, p, word);
        let name = self
            .name_with(blanks, is_upper_start)
            .or_else(|| self.quoted_name(blanks));
        if let Some(end) = name {
            self.capture(2, blanks, end);
            return Some(end);
        }
        Some(blanks)
    }

    /// `^\s*(extension)\s+(?=[(\[])`.
    pub(super) fn extension(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let start = self.space_end(0);
        let word = self.lit(start, b"extension")?;
        let end = self.space_end(word);
        if end == word || !matches!(self.at(end), b'(' | b'[') {
            return None;
        }
        self.capture(1, start, word);
        Some(end)
    }

    /// `\b(new)\b`.
    pub(super) fn new_keyword(&mut self, p: usize) -> Option<usize> {
        let end = self.word(p, b"new")?;
        self.capture(1, p, end);
        Some(end)
    }

    /// `\b(pr(?:ivate\[\S+]|otected\[\S+]|ivate|otected))\b`.
    ///
    /// `\S+` backtracks to each `]` it ran over, the last first.
    /// The `\b` after a `]` needs a `\w` after it.
    pub(super) fn access_modifier(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        for word in [b"private".as_slice(), b"protected"] {
            let Some(open) = self.lit(p, word).and_then(|end| self.lit(end, b"[")) else {
                continue;
            };
            let run = self.non_space_end(open);
            let mut end = run;
            while end > open + 1 {
                if self.at(end - 1) == b']' && self.boundary(end) {
                    return Some(end);
                }
                end = self.char_start_before(end);
            }
        }
        [b"private".as_slice(), b"protected"]
            .iter()
            .find_map(|word| self.lit(p, word).filter(|&end| self.boundary(end)))
    }

    /// `\b(synchronized|@volatile|…|@native)\b`.
    ///
    /// The `\b` before an `@` needs a `\w` before it.
    pub(super) fn other_modifier(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        OTHER_MODIFIERS
            .iter()
            .find_map(|word| self.lit(p, word).filter(|&end| self.boundary(end)))
    }

    /// `(?<=^|\s)\b(transparent|opaque|infix|open|inline|into)\b`, then
    /// `(?=[a-z\s]*\b(def|val|…|enum)\b)`.
    ///
    /// The lookahead's run backtracks to each place a declaration word starts.
    pub(super) fn soft_modifier(&self, p: usize) -> Option<usize> {
        if p != 0 && self.space_at(self.char_start_before(p)) == 0 {
            return None;
        }
        let end = self.word_in(p, SOFT_MODIFIERS)?;
        let mut run = end;
        while run < self.lim() {
            let width = self.space_at(run);
            if width > 0 {
                run += width;
            } else if self.at(run).is_ascii_lowercase() {
                run += 1;
            } else {
                break;
            }
        }
        self.spend(run - end);
        let mut k = run;
        loop {
            if self.word_in(k, DECLARATION_WORDS).is_some() {
                return Some(end);
            }
            if k == end {
                return None;
            }
            k = self.char_start_before(k);
        }
    }

    /// `\b(inline)(?=\s+((?:plain|[op]+)|quoted)\s*:)`.
    ///
    /// The name backtracks: an operator name or tail can take the `:`.
    pub(super) fn inline_modifier(&self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"inline").filter(|_| self.boundary(p))?;
        let name = self.space_end(end);
        if name == end {
            return None;
        }
        let colon = |i: usize| {
            let at = self.space_end(i);
            self.at(at) == b':' && at < self.lim()
        };
        self.any_name_where(name, colon).map(|_| end)
    }

    /// `\b(inline)\b(?=(?:.(?!\b(?:val|def|given)\b))*\b(if|match)\b)`.
    ///
    /// The lookahead runs over each character that no `val`, `def` or
    /// `given` follows, then backtracks to an `if` or `match`.
    pub(super) fn inline_control(&self, p: usize) -> Option<usize> {
        let end = self.word(p, b"inline")?;
        let mut stop = end;
        while stop < self.lim() {
            let next = self.char_end(stop);
            if self.word_in(next, INLINE_STOPS).is_some() {
                break;
            }
            stop = next;
        }
        self.spend(stop - end);
        let mut k = stop;
        loop {
            if self.word_in(k, &[b"if", b"match"]).is_some() {
                return Some(end);
            }
            if k == end {
                return None;
            }
            k = self.char_start_before(k);
        }
    }

    /// `\s+(as)\s+([$_a-z\p{Lo}\p{Nl}\p{Ll}]idrest)\b`.
    ///
    /// `\b` can fall inside the name: `$` and letter numbers are not `\w`.
    /// The name gives characters back until one holds.
    pub(super) fn named_bound(&mut self, p: usize) -> Option<usize> {
        if self.space_at(p) == 0 {
            return None;
        }
        let word = self.space_end(p);
        let r#as = self.lit(word, b"as")?;
        let name = self.space_end(r#as);
        if name == r#as {
            return None;
        }
        let end = self.plain_end_where(name, is_lower_start, |end| self.boundary(end))?;
        self.capture(1, word, r#as);
        self.capture(2, name, end);
        Some(end)
    }

    /// `(?<=\()\s*(using)\s`.
    pub(super) fn using_clause(&mut self, p: usize) -> Option<usize> {
        if !self.prev_is_byte(p, b'(') {
            return None;
        }
        let start = self.space_end(p);
        let word = self.lit(start, b"using")?;
        let width = self.space_at(word);
        if width == 0 {
            return None;
        }
        self.capture(1, start, word);
        Some(word + width)
    }

    /// `(?<=[^$.0-9A-Z_a-z])(quoted|[$_a-z\p{Lo}\p{Nl}\p{Ll}]idrest)\s*(:)\s+`.
    ///
    /// An operator tail can take the `:`, so the name backtracks.
    pub(super) fn parameter(&mut self, p: usize) -> Option<usize> {
        let before = self.char_before(p)?;
        if before == '$' || before == '.' || before == '_' || before.is_ascii_alphanumeric() {
            return None;
        }
        let colon_then_blanks = |end: usize| {
            let colon = self.space_end(end);
            self.at(colon) == b':' && colon < self.lim() && self.space_at(colon + 1) > 0
        };
        let name = self
            .quoted_name(p)
            .filter(|&end| colon_then_blanks(end))
            .or_else(|| self.plain_end_where(p, is_lower_start, colon_then_blanks))?;
        let colon = self.space_end(name);
        let end = self.space_end(colon + 1);
        self.capture(1, p, name);
        self.capture(2, colon, colon + 1);
        Some(end)
    }

    /// `\b(([A-Z]\w*)(?:(?<=_)[op]+)?)`.
    pub(super) fn qualified_class_name(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) || !self.at(p).is_ascii_uppercase() {
            return None;
        }
        let end = self.op_tail(self.word_end(p + 1));
        self.capture(1, p, end);
        Some(end)
    }

    /// `\.(type)(?!plain|[0-9])`: no name character or digit follows.
    pub(super) fn singleton_type(&mut self, p: usize) -> Option<usize> {
        let word = self.lit(p, b".type")?;
        if let Some((c, _)) = self.char_at(word)
            && (is_name_start(c) || c.is_ascii_digit())
        {
            return None;
        }
        self.capture(1, p + 1, word);
        Some(word)
    }
}
