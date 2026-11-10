//! Declarations: modules, imports, classes, instances, data and type
//! declarations, foreign functions, deriving clauses and records.

use super::super::engine::Lexer;
use super::{is_name, is_op, is_var_start};
use crate::classes::{is_digit, is_lower, is_title, is_title_or_upper};
use crate::rules::Chars;

/// The quote that a type's `where` must not touch on either side:
/// `(?<!'')where\b(?!'')`.
const TWO_QUOTES: &[u8] = b"''";

/// `['._\p{Ll}\p{Lu}\p{Lt}\d]`: a character of a constructor's word.
fn is_word_part(c: char) -> bool {
    matches!(c, '\'' | '.' | '_') || is_lower(c) || is_title_or_upper(c) || is_digit(c)
}

/// `[_\p{Ll}\p{Lu}\p{Lt}\d]`: a character of a GADT constructor after its
/// first.
fn is_gadt_part(c: char) -> bool {
    c == '_' || is_lower(c) || is_title_or_upper(c) || is_digit(c)
}

/// `[_\p{Ll}\p{Lu}]`: the first character of a record field.
fn is_field_start(c: char) -> bool {
    c == '_' || is_lower(c) || (is_title_or_upper(c) && !is_title(c))
}

/// What stops the run of a declaration's head.
#[derive(Clone, Copy)]
struct Stop {
    /// The words that stop it, as `\b(?<!')word\b(?!')`.
    words: &'static [&'static [u8]],
    /// Whether `::` and `∷` stop it as `=` does.
    colons: bool,
}

impl Lexer<'_> {
    /// `(\b(?<!')(module)|^(signature))\b((?!'))`, with groups around the
    /// word.
    pub(super) fn module_head(&mut self, p: usize) -> Option<usize> {
        if let Some(end) = self.keyword_at(p, b"module") {
            self.capture(2, p, end);
            return Some(end);
        }
        if p != 0 {
            return None;
        }
        let end = self.lit(0, b"signature")?;
        if self.word_at(end).is_some() || self.at(end) == b'\'' {
            return None;
        }
        self.capture(3, 0, end);
        Some(end)
    }

    /// `(?<conid>[\p{Lu}\p{Lt}]['_\p{Ll}\p{Lu}\p{Lt}\d]*(\.\g<conid>)?)`: a
    /// module name, one or more constructors joined by dots.
    ///
    /// A dot that no constructor follows is given back.
    pub(super) fn module_name(&self, p: usize) -> Option<usize> {
        let mut end = self.name_from(p, is_title_or_upper)?;
        while self.at(end) == b'.'
            && let Some(next) = self.name_from(end + 1, is_title_or_upper)
        {
            end = next;
        }
        Some(end)
    }

    /// `([\p{Lu}\p{Lt}]['_\p{Ll}\p{Lu}\p{Lt}\d]*)|(\()\s*(:[OP]+)\s*(\))`,
    /// or with `[OP]+` alone when not `colon`: the name or operator that ends
    /// an exported pattern or type.
    pub(super) fn constructor_or_operator(&mut self, p: usize, colon: bool) -> Option<usize> {
        if let Some(end) = self.name_from(p, is_title_or_upper) {
            self.capture(1, p, end);
            return Some(end);
        }
        let found = self.paren_op(p, |l, i| {
            if colon {
                l.colon_ops(i, false)
            } else {
                l.ops(i, false)
            }
        })?;
        Some(self.capture_paren([2, 3, 4], p, found))
    }

    /// `(?<!')\b[_\p{Ll}]['_\p{Ll}\p{Lu}\p{Lt}\d]*`: a variable.
    pub(super) fn variable(&self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'\'') || self.word_before(p) {
            return None;
        }
        self.name_from(p, is_var_start)
    }

    /// `(?<!')\b[\p{Lu}\p{Lt}]['_\p{Ll}\p{Lu}\p{Lt}\d]*`: a constructor.
    pub(super) fn constructor(&self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'\'') || self.word_before(p) {
            return None;
        }
        self.name_from(p, is_title_or_upper)
    }

    /// `^(\s*)(word)\b((?!'))`: a declaration that starts a line, with groups
    /// around its indentation and its word.
    pub(super) fn declaration_head(&mut self, p: usize, word: &[u8]) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let indent = self.space_end(0);
        let end = self.lit(indent, word)?;
        if self.word_at(end).is_some() || self.at(end) == b'\'' {
            return None;
        }
        self.capture(1, 0, indent);
        self.capture(2, indent, end);
        Some(end)
    }

    /// `(qualified|as|hiding)`, with group 1 around the first word that
    /// matches.
    pub(super) fn import_word(&mut self, p: usize) -> Option<usize> {
        let end = [&b"qualified"[..], b"as", b"hiding"]
            .iter()
            .find_map(|word| self.lit(p, word))?;
        self.capture(1, p, end);
        Some(end)
    }

    /// Whether the run of a declaration's head stops at `i`:
    /// `(?<![OP])(?:=|--+)(?![OP])|\b(?<!')(?:where|deriving)\b(?!')|\{-`,
    /// with the words and the colons of `stop`.
    ///
    /// Giving a dash back leaves a dash after the rest, an operator character.
    fn head_stops(&self, i: usize, stop: Stop) -> bool {
        if self.starts(i, b"{-") {
            return true;
        }
        if stop
            .words
            .iter()
            .any(|word| self.keyword_at(i, word).is_some())
        {
            return true;
        }
        if self.before_is(i, is_op) {
            return false;
        }
        let dashes = self.skip(i, |b| b == b'-');
        if dashes - i >= 2 {
            return !self.here_is(dashes, is_op);
        }
        let symbols: &[&[u8]] = if stop.colons {
            &[b"=", b"::", "∷".as_bytes()]
        } else {
            &[b"="]
        };
        self.alone_after(i, symbols).is_some()
    }

    /// The end of `(?:(?!STOP).)*` from `i`: the first place the head stops,
    /// or the line's end.
    fn head_end(&self, i: usize, stop: Stop) -> usize {
        let mut at = i;
        while at < self.lim() && !self.head_stops(at, stop) {
            at = self.char_end(at);
        }
        self.spend(at - i);
        at
    }

    /// `(?=\b(?<!'')where\b(?!''))` at `i`.
    fn type_where_ahead(&self, i: usize) -> bool {
        if self.word_before(i) || (i >= 2 && self.text(i - 2, i) == TWO_QUOTES) {
            return false;
        }
        self.lit(i, b"where")
            .is_some_and(|end| self.word_at(end).is_none() && !self.starts(end, TWO_QUOTES))
    }

    /// The end of a data declaration's head from `i`, and whether it holds.
    ///
    /// For a GADT the head must end before `where`.
    /// The run then gives characters back from the place it stops until
    /// `where` stands there.
    fn data_head_end(&self, i: usize, gadt: bool) -> Option<usize> {
        let stop = Stop {
            words: &[b"where", b"deriving"],
            colons: false,
        };
        let mut end = self.head_end(i, stop);
        if !gadt {
            return Some(end);
        }
        loop {
            if self.type_where_ahead(end) {
                return Some(end);
            }
            if end <= i {
                return None;
            }
            end = self.char_start_before(end);
        }
    }

    /// `^(\s*)(data|newtype)(?:\s+(family|instance))?\s+(((?!STOP).)*)` for
    /// the word `word`, with groups around the indentation, the word, the
    /// optional `family` or `instance`, and the head.
    /// For a GADT, `gadt`, only `instance` may follow, and the head ends where
    /// `(?=\b(?<!'')where\b(?!''))` holds.
    ///
    /// The path through `instance` is tried first.
    /// When it fails, the head starts at the word after the blanks.
    pub(super) fn data_head(&mut self, p: usize, word: &[u8], gadt: bool) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let indent = self.space_end(0);
        let after = self.lit(indent, word)?;
        let blanks = self.space_end(after);
        if blanks == after {
            return None;
        }
        let keywords: &[&[u8]] = if gadt {
            &[b"instance"]
        } else {
            &[b"family", b"instance"]
        };
        if let Some(keyword) = keywords
            .iter()
            .find_map(|keyword| self.lit(blanks, keyword))
        {
            let head = self.space_end(keyword);
            if head > keyword
                && let Some(end) = self.data_head_end(head, gadt)
            {
                self.capture(1, 0, indent);
                self.capture(2, indent, after);
                self.capture(3, blanks, keyword);
                self.capture(4, head, end);
                return Some(end);
            }
        }
        let end = self.data_head_end(blanks, gadt)?;
        self.capture(1, 0, indent);
        self.capture(2, indent, after);
        self.capture(4, blanks, end);
        Some(end)
    }

    /// `^(\s*)(type)\s+(family)\b(?!')(((?!STOP).)*)`, with groups around the
    /// indentation, `type`, `family` and the head.
    pub(super) fn type_family_head(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let indent = self.space_end(0);
        let after = self.lit(indent, b"type")?;
        let blanks = self.space_end(after);
        if blanks == after {
            return None;
        }
        let family = self.lit(blanks, b"family")?;
        if self.word_at(family).is_some() || self.at(family) == b'\'' {
            return None;
        }
        let end = self.head_end(
            family,
            Stop {
                words: &[b"where"],
                colons: false,
            },
        );
        self.capture(1, 0, indent);
        self.capture(2, indent, after);
        self.capture(3, blanks, family);
        self.capture(4, family, end);
        Some(end)
    }

    /// `^(\s*)(type)(?:\s+(instance))?\s+(((?!STOP).)*)`, where `::` and `∷`
    /// stop the head as `=` does and no word does, with groups around the
    /// indentation, `type`, the optional `instance` and the head.
    pub(super) fn type_head(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let stop = Stop {
            words: &[],
            colons: true,
        };
        let indent = self.space_end(0);
        let after = self.lit(indent, b"type")?;
        let blanks = self.space_end(after);
        if blanks == after {
            return None;
        }
        self.capture(1, 0, indent);
        self.capture(2, indent, after);
        if let Some(instance) = self.lit(blanks, b"instance") {
            let head = self.space_end(instance);
            if head > instance {
                let end = self.head_end(head, stop);
                self.capture(3, blanks, instance);
                self.capture(4, head, end);
                return Some(end);
            }
        }
        let end = self.head_end(blanks, stop);
        self.capture(4, blanks, end);
        Some(end)
    }

    /// `\s+(::|∷)(?![OP])` at `i`: the blanks and the colons after them.
    fn spaced_colons(&self, i: usize) -> Option<(usize, usize)> {
        let blanks = self.space_end(i);
        if blanks == i {
            return None;
        }
        let end = self.alone_after(blanks, &[b"::", "∷".as_bytes()])?;
        Some((blanks, end))
    }

    /// `^(\s*)(pattern)\s+(.*?)\s+(::|∷)(?![OP])`: a pattern synonym's
    /// signature, with groups around the indentation, `pattern`, the names
    /// and the colons.
    ///
    /// The names grow one character at a time until blanks and colons follow.
    /// When none do, the first blanks give one back, which leaves the names
    /// empty when colons follow them.
    pub(super) fn pattern_signature(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let indent = self.space_end(0);
        let after = self.lit(indent, b"pattern")?;
        let names = self.space_end(after);
        if names == after {
            return None;
        }
        let mut at = names;
        let found = loop {
            if let Some((colons, end)) = self.spaced_colons(at) {
                break Some((at, colons, end));
            }
            if at >= self.lim() {
                break None;
            }
            at = self.char_end(at);
        };
        self.spend(at - names);
        let (names_end, colons, end) = match found {
            Some(found) => found,
            None if names - after >= 2 => {
                let end = self.alone_after(names, &[b"::", "∷".as_bytes()])?;
                (names - 1, names, end)
            }
            None => return None,
        };
        let names_start = names_end.min(names);
        self.capture(1, 0, indent);
        self.capture(2, indent, after);
        self.capture(3, names_start, names_end);
        self.capture(4, colons, end);
        Some(end)
    }

    /// `^\s*(pattern)\b(?!')`: a pattern synonym, with group 1 around
    /// `pattern`.
    pub(super) fn pattern_head(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let indent = self.space_end(0);
        let end = self.lit(indent, b"pattern")?;
        if self.word_at(end).is_some() || self.at(end) == b'\'' {
            return None;
        }
        self.capture(1, indent, end);
        Some(end)
    }

    /// `^(\s*)(type)\s+(role)\b(?!')`: a role annotation, with groups around
    /// the indentation, `type` and `role`.
    pub(super) fn role_head(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let indent = self.space_end(0);
        let after = self.lit(indent, b"type")?;
        let blanks = self.space_end(after);
        if blanks == after {
            return None;
        }
        let end = self.lit(blanks, b"role")?;
        if self.word_at(end).is_some() || self.at(end) == b'\'' {
            return None;
        }
        self.capture(1, 0, indent);
        self.capture(2, indent, after);
        self.capture(3, blanks, end);
        Some(end)
    }

    /// `^(\s*)(foreign)\s+(word)\s+` for `import` or `export`: a foreign
    /// declaration, with groups around the indentation, `foreign` and the
    /// word.
    pub(super) fn foreign_head(&mut self, p: usize, word: &[u8]) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let indent = self.space_end(0);
        let after = self.lit(indent, b"foreign")?;
        let blanks = self.space_end(after);
        if blanks == after {
            return None;
        }
        let word_end = self.lit(blanks, word)?;
        let end = self.space_end(word_end);
        if end == word_end {
            return None;
        }
        self.capture(1, 0, indent);
        self.capture(2, indent, after);
        self.capture(3, blanks, word_end);
        Some(end)
    }

    /// `\b(?<!')(ccall|cplusplus|dotnet|jvm|stdcall|prim|capi)\s+`, with
    /// group 1 around the first convention that blanks follow.
    pub(super) fn calling_convention(&mut self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'\'') || self.word_before(p) {
            return None;
        }
        let words: [&[u8]; 7] = [
            b"ccall",
            b"cplusplus",
            b"dotnet",
            b"jvm",
            b"stdcall",
            b"prim",
            b"capi",
        ];
        let (word, end) = words.iter().find_map(|word| {
            let word = self.lit(p, word)?;
            let end = self.space_end(word);
            (end > word).then_some((word, end))
        })?;
        self.capture(1, p, word);
        Some(end)
    }

    /// The end of `\b(?<!x)([_\p{Ll}]['_\p{Ll}\p{Lu}\p{Lt}\d]*)\b(?!')` at
    /// `i`, where `x` is `quotes`, the text that must not stand before it.
    fn variable_word(&self, i: usize, quotes: &[u8]) -> Option<usize> {
        if self.word_before(i) || (i >= quotes.len() && self.text(i - quotes.len(), i) == quotes) {
            return None;
        }
        let first = self.char_here(i).filter(|&(c, _)| is_var_start(c))?.1;
        let end = self.class_end(first, is_name);
        self.tail_end(first, end)
    }

    /// `(?=")|(?=\b(?<!')([_\p{Ll}]['_\p{Ll}\p{Lu}\p{Lt}\d]*)\b(?!'))`: a
    /// foreign function's string or name ahead.
    pub(super) fn foreign_name_ahead(&self, p: usize) -> Option<usize> {
        (self.at(p) == b'"' && p < self.lim() || self.variable_word(p, b"'").is_some()).then_some(p)
    }

    /// `(?=(::|∷)(?![OP]))`.
    pub(super) fn double_colon_ahead(&self, p: usize) -> Option<usize> {
        self.alone_after(p, &[b"::", "∷".as_bytes()]).map(|_| p)
    }

    /// `\b(?<!')(safe|unsafe|interruptible)\b(?!')` at `p`, and its end.
    fn safety(&self, p: usize) -> Option<usize> {
        [&b"safe"[..], b"unsafe", b"interruptible"]
            .iter()
            .find_map(|word| self.keyword_at(p, word))
    }

    /// `\b(?<!'')([_\p{Ll}]…)\b(?!')|(\()\s*(?!--+\))([OP]+)\s*(\))` at `i`:
    /// a foreign function's name or operator.
    fn foreign_name_at(&self, i: usize) -> Option<ForeignName> {
        if let Some(end) = self.variable_word(i, b"''") {
            return Some(ForeignName::Name(end));
        }
        let found = self.paren_op(i, |l, at| {
            if l.dashes_close(at) {
                None
            } else {
                l.ops(at, false)
            }
        })?;
        Some(ForeignName::Operator(found))
    }

    /// `\b(?<!')(safe|unsafe|interruptible)\b(?!')\s*("(?:\\"|[^"])*")?\s*(?:
    /// NAME|\(\s*(?!--+\))([OP]+)\s*\))`, with groups around the safety, the
    /// string, the name and the operator.
    ///
    /// Each way the string can close is tried, then no string.
    pub(super) fn safety_and_name(&mut self, p: usize) -> Option<usize> {
        let safety = self.safety(p)?;
        let string = self.space_end(safety);
        let found = self.quoted_try(string, |l, close| {
            let at = l.space_end(close);
            l.foreign_name_at(at).map(|name| (Some(close), at, name))
        });
        let (close, at, name) = match found {
            Some(found) => found,
            None => (None, string, self.foreign_name_at(string)?),
        };
        self.capture(1, p, safety);
        if let Some(close) = close {
            self.capture(2, string, close);
        }
        match name {
            ForeignName::Name(end) => {
                self.capture(3, at, end);
                Some(end)
            }
            ForeignName::Operator((start, end, close)) => {
                self.capture(4, start, end);
                Some(close + 1)
            }
        }
    }

    /// `\b(?<!')(safe|unsafe|interruptible)\b(?!')\s*("(?:\\"|[^"])*")?\s*$`,
    /// with groups around the safety and the string.
    pub(super) fn safety_to_end(&mut self, p: usize) -> Option<usize> {
        let safety = self.safety(p)?;
        let string = self.space_end(safety);
        let to_end = |l: &Self, at: usize| {
            let end = l.space_end(at);
            l.at_eol(end).then_some(end)
        };
        let found = self.quoted_try(string, |l, close| {
            to_end(l, close).map(|end| (Some(close), end))
        });
        let (close, end) = match found {
            Some(found) => found,
            None => (None, to_end(self, string)?),
        };
        self.capture(1, p, safety);
        if let Some(close) = close {
            self.capture(2, string, close);
        }
        Some(end)
    }

    /// `\b(?<!'')([_\p{Ll}]…)\b(?!')|(\()\s*(?!--+\))([OP]+)\s*(\))`: a
    /// foreign function's name or operator, with groups around it.
    pub(super) fn foreign_name(&mut self, p: usize) -> Option<usize> {
        match self.foreign_name_at(p)? {
            ForeignName::Name(end) => {
                self.capture(1, p, end);
                Some(end)
            }
            ForeignName::Operator(found) => Some(self.capture_paren([2, 3, 4], p, found)),
        }
    }

    /// `(?<!')\b(where)\s*(\{)(?!-)`, with groups around `where` and the brace.
    pub(super) fn where_brace(&mut self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'\'') || self.word_before(p) {
            return None;
        }
        let word = self.lit(p, b"where")?;
        let brace = self.space_end(word);
        if !(brace < self.lim() && self.at(brace) == b'{' && self.at(brace + 1) != b'-') {
            return None;
        }
        self.capture(1, p, word);
        self.capture(2, brace, brace + 1);
        Some(brace + 1)
    }

    /// `^(\s*)(?:\b((?<!')[\p{Lu}\p{Lt}]…)|(\()\s*(:[OP]*)\s*(\)))`: a GADT
    /// constructor that starts a line, with groups around the indentation and
    /// the name or the operator.
    pub(super) fn gadt_head(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let indent = self.space_end(0);
        if !self.word_before(indent)
            && !self.prev_is(indent, b'\'')
            && let Some(end) = self.name_from(indent, is_title_or_upper)
        {
            self.capture(1, 0, indent);
            self.capture(2, indent, end);
            return Some(end);
        }
        let found = self.paren_op(indent, |l, i| l.colon_ops(i, true))?;
        self.capture(1, 0, indent);
        Some(self.capture_paren([3, 4, 5], indent, found))
    }

    /// `\b((?<!')[\p{Lu}\p{Lt}][_\p{Ll}\p{Lu}\p{Lt}\d]*)|(\()\s*(:[OP]*)\s*(\
    /// ))`: a GADT constructor after the first, with groups around it.
    pub(super) fn gadt_constructor(&mut self, p: usize) -> Option<usize> {
        if !self.word_before(p)
            && !self.prev_is(p, b'\'')
            && let Some((c, next)) = self.char_here(p)
            && is_title_or_upper(c)
        {
            let end = self.class_end(next, is_gadt_part);
            self.capture(1, p, end);
            return Some(end);
        }
        let found = self.paren_op(p, |l, i| l.colon_ops(i, true))?;
        Some(self.capture_paren([2, 3, 4], p, found))
    }

    /// The end of `\s+(stock|newtype|anyclass)` at `i`, and where the word
    /// starts.
    fn strategy(&self, i: usize) -> Option<(usize, usize)> {
        let blanks = self.space_end(i);
        if blanks == i {
            return None;
        }
        let end = [&b"stock"[..], b"newtype", b"anyclass"]
            .iter()
            .find_map(|word| self.lit(blanks, word))?;
        Some((blanks, end))
    }

    /// `^(\s*)(deriving)\s+(?:(via|stock|newtype|anyclass)\s+)?`: a standalone
    /// deriving clause, with groups around the indentation, `deriving` and
    /// the strategy.
    pub(super) fn deriving_head(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let indent = self.space_end(0);
        let after = self.lit(indent, b"deriving")?;
        let blanks = self.space_end(after);
        if blanks == after {
            return None;
        }
        self.capture(1, 0, indent);
        self.capture(2, indent, after);
        let strategy = [&b"via"[..], b"stock", b"newtype", b"anyclass"]
            .iter()
            .find_map(|word| self.lit(blanks, word))
            .filter(|&word| self.space_end(word) > word);
        let Some(word) = strategy else {
            return Some(blanks);
        };
        self.capture(3, blanks, word);
        Some(self.space_end(word))
    }

    /// `(deriving)(?:\s+(stock|newtype|anyclass))?\s*(\()`, with groups around
    /// `deriving`, the strategy and the parenthesis.
    pub(super) fn deriving_paren(&mut self, p: usize) -> Option<usize> {
        let after = self.lit(p, b"deriving")?;
        let paren = |l: &Self, at: usize| {
            let open = l.space_end(at);
            (open < l.lim() && l.at(open) == b'(').then_some(open)
        };
        if let Some((start, word)) = self.strategy(after)
            && let Some(open) = paren(self, word)
        {
            self.capture(1, p, after);
            self.capture(2, start, word);
            self.capture(3, open, open + 1);
            return Some(open + 1);
        }
        let open = paren(self, after)?;
        self.capture(1, p, after);
        self.capture(3, open, open + 1);
        Some(open + 1)
    }

    /// `(deriving)(?:\s+(stock|newtype|anyclass))?\s+([\p{Lu}\p{Lt}]…)(\
    /// s+(via)\s+(.*)$)?`, with groups around `deriving`, the strategy, the
    /// class, and the `via` clause and its type.
    pub(super) fn deriving_class(&mut self, p: usize) -> Option<usize> {
        let after = self.lit(p, b"deriving")?;
        let class = |l: &Self, at: usize| {
            let start = l.space_end(at);
            if start == at {
                return None;
            }
            l.name_from(start, is_title_or_upper)
                .map(|end| (start, end))
        };
        let (strategy, (start, end)) = match self
            .strategy(after)
            .and_then(|(s, w)| Some(((s, w), class(self, w)?)))
        {
            Some((strategy, class)) => (Some(strategy), class),
            None => (None, class(self, after)?),
        };
        self.capture(1, p, after);
        if let Some((from, to)) = strategy {
            self.capture(2, from, to);
        }
        self.capture(3, start, end);
        let via = self.space_end(end);
        if via > end
            && let Some(via_end) = self.lit(via, b"via")
        {
            let rest = self.space_end(via_end);
            if rest > via_end {
                let rest_end = rest.max(self.lim());
                self.capture(4, end, rest_end);
                self.capture(5, via, via_end);
                self.capture(6, rest, rest_end);
                return Some(rest_end);
            }
        }
        Some(end)
    }

    /// `(\{)(?!-)`, with group 1 around the brace.
    pub(super) fn record_open(&mut self, p: usize) -> Option<usize> {
        if !(p < self.lim() && self.at(p) == b'{' && self.at(p + 1) != b'-') {
            return None;
        }
        self.capture(1, p, p + 1);
        Some(p + 1)
    }

    /// `(?<!-)(})`, with group 1 around the brace.
    pub(super) fn record_close(&mut self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'-') {
            return None;
        }
        self.group_byte(p, b'}')
    }

    /// `([_\p{Ll}]…)|(\()\s*([OP]+)\s*(\))`: a field in a record declaration,
    /// with groups around its name or operator.
    pub(super) fn record_decl_field(&mut self, p: usize) -> Option<usize> {
        if let Some(end) = self.name_from(p, is_var_start) {
            self.capture(1, p, end);
            return Some(end);
        }
        let found = self.paren_op(p, |l, i| l.ops(i, false))?;
        Some(self.capture_paren([2, 3, 4], p, found))
    }

    /// `(,)|(?=})`, with group 1 around the comma.
    pub(super) fn field_end(&mut self, p: usize) -> Option<usize> {
        if p < self.lim() && self.at(p) == b'}' {
            return Some(p);
        }
        self.group_byte(p, b',')
    }

    /// `([_\p{Ll}\p{Lu}]['._\p{Ll}\p{Lu}\p{Lt}\d]*)|(\()\s*([OP]+)\s*(\))`: a
    /// field in a record value, with groups around its name or operator.
    pub(super) fn record_field(&mut self, p: usize) -> Option<usize> {
        if let Some((c, next)) = self.char_here(p)
            && is_field_start(c)
        {
            let end = self.class_end(next, is_word_part);
            self.capture(1, p, end);
            return Some(end);
        }
        let found = self.paren_op(p, |l, i| l.ops(i, false))?;
        Some(self.capture_paren([2, 3, 4], p, found))
    }

    /// `(?<![OP])(?:(=)|(\|))(?![OP])`, with a group around `=` or `|`.
    pub(super) fn adt_bar(&mut self, p: usize) -> Option<usize> {
        if self.before_is(p, is_op) || self.here_is(p + 1, is_op) {
            return None;
        }
        let group = match self.at(p) {
            b'=' => 1,
            b'|' => 2,
            _ => return None,
        };
        self.capture(group, p, p + 1);
        Some(p + 1)
    }

    /// The end of `(?<paren>\((?:[^()]?|\g<paren>)*\))` at `i`: balanced
    /// parentheses.
    ///
    /// Only one parse exists: a parenthesis in the text opens or closes a
    /// level.
    fn balanced_parens(&self, i: usize) -> Option<usize> {
        if self.at(i) != b'(' || i >= self.lim() {
            return None;
        }
        let mut depth = 0_usize;
        let mut at = i;
        while at < self.lim() {
            match self.at(at) {
                b'(' => depth += 1,
                b')' => {
                    depth -= 1;
                    if depth == 0 {
                        self.spend(at - i);
                        return Some(at + 1);
                    }
                }
                _ => {}
            }
            at += 1;
        }
        self.spend(at - i);
        None
    }

    /// Each end of `(?<brac>\((?:[^]\[]?|\g<brac>)*])` at `i`, in the order
    /// backtracking tries them, until `then` accepts one.
    ///
    /// A `(` inside may open a nested level, which a `]` closes.
    /// The first `]` closes the outer level first.
    /// Then each later `]` does, while enough `(` before it opened levels for
    /// the ones before it.
    /// No `[` may stand inside.
    fn bracket_ends<T>(&self, i: usize, mut then: impl FnMut(usize) -> Option<T>) -> Option<T> {
        if self.at(i) != b'(' || i >= self.lim() {
            return None;
        }
        let mut opens = 0_usize;
        let mut closes = 0_usize;
        let mut at = i + 1;
        while at < self.lim() {
            match self.at(at) {
                b'[' => break,
                b'(' => opens += 1,
                b']' => {
                    if closes > opens {
                        break;
                    }
                    if let Some(found) = then(at + 1) {
                        self.spend(at - i);
                        return Some(found);
                    }
                    closes += 1;
                }
                _ => {}
            }
            at += 1;
        }
        self.spend(at - i);
        None
    }

    /// `\s*(?:(?<![OP])(:[OP]*)|(`)([\p{Lu}\p{Lt}]…)(`))` at `i`: the
    /// operator or backticked constructor after a constructor's first
    /// argument, and its groups.
    fn infix_constructor(&self, i: usize) -> Option<InfixConstructor> {
        let at = self.space_end(i);
        if !self.before_is(at, is_op)
            && let Some(end) = self.colon_ops(at, true)
        {
            return Some(InfixConstructor::Operator(at, end));
        }
        let name = self.lit(at, b"`")?;
        let end = self.name_from(name, is_title_or_upper)?;
        let close = self.lit(end, b"`")?;
        Some(InfixConstructor::Backticks(at, end, close))
    }

    /// The end of a data declaration's constructor, its first match after
    /// `=` or `|`:
    /// `(?:\G|^)\s*(?:WORD|'?PARENS|'?BRACKETS)\s*(?:OPERATOR|BACKTICKED)`,
    /// an infix constructor after its first argument, or
    /// `(?<!')\b([\p{Lu}\p{Lt}]…)` anywhere, or `(\()\s*(:[OP]*)\s*(\))`.
    ///
    /// The groups are numbered as in the grammar:
    /// - 1, the word;
    /// - 2, the parentheses;
    /// - 4, the brackets;
    /// - 6, the operator;
    /// - 7 to 9, the backticked constructor;
    /// - 10, the constructor;
    /// - 11 to 13, the parenthesised operator.
    ///
    /// The word gives back no character that helps: the operator or backtick
    /// after it is not a word character.
    pub(super) fn adt_constructor(&mut self, p: usize) -> Option<usize> {
        if (p == 0 || self.at_g(p))
            && let Some(end) = self.infix_after_argument(p)
        {
            return Some(end);
        }
        if !self.word_before(p)
            && !self.prev_is(p, b'\'')
            && let Some(end) = self.name_from(p, is_title_or_upper)
        {
            self.capture(10, p, end);
            return Some(end);
        }
        let found = self.paren_op(p, |l, i| l.colon_ops(i, true))?;
        Some(self.capture_paren([11, 12, 13], p, found))
    }

    /// The anchored alternative of [`Lexer::adt_constructor`]: an argument and
    /// the infix constructor after it.
    fn infix_after_argument(&mut self, p: usize) -> Option<usize> {
        let start = self.space_end(p);
        if self.boundary(start) && !self.prev_is(start, b'\'') {
            let end = self.class_end(start, is_word_part);
            if end > start
                && let Some(infix) = self.infix_constructor(end)
            {
                self.capture(1, start, end);
                return Some(self.capture_infix(infix));
            }
        }
        let quoted = usize::from(self.at(start) == b'\'' && start < self.lim());
        for open in [start + quoted, start] {
            if let Some(end) = self.balanced_parens(open)
                && let Some(infix) = self.infix_constructor(end)
            {
                self.capture(2, start, end);
                return Some(self.capture_infix(infix));
            }
            if let Some((end, infix)) =
                self.bracket_ends(open, |end| Some((end, self.infix_constructor(end)?)))
            {
                self.capture(4, start, end);
                return Some(self.capture_infix(infix));
            }
            if quoted == 0 {
                break;
            }
        }
        None
    }

    /// The groups of an infix constructor, and the end of the match.
    fn capture_infix(&mut self, infix: InfixConstructor) -> usize {
        match infix {
            InfixConstructor::Operator(start, end) => {
                self.capture(6, start, end);
                end
            }
            InfixConstructor::Backticks(open, end, close) => {
                self.capture(7, open, open + 1);
                self.capture(8, open + 1, end);
                self.capture(9, end, close);
                close
            }
        }
    }

    /// `(.*)(?<![OP])(=>|⇒)(?![OP])`: a context, with groups around the
    /// constraints and the arrow.
    ///
    /// The constraints give characters back from the line's end until an
    /// arrow stands alone, so the last such arrow wins.
    pub(super) fn context(&mut self, p: usize) -> Option<usize> {
        if p > self.lim() {
            return None;
        }
        let mut at = self.lim();
        loop {
            if !self.before_is(at, is_op)
                && let Some(end) = self.alone_after(at, &[b"=>", "⇒".as_bytes()])
            {
                self.spend(self.lim() - at);
                self.capture(1, p, at);
                self.capture(2, at, end);
                return Some(end);
            }
            if at <= p {
                self.spend(self.lim() - p);
                return None;
            }
            at = self.char_start_before(at);
        }
    }

    /// `(?<!')\b(?:(where|let|m?do)|(of))\s*(\{)(?!-)`: a layout herald with
    /// an explicit brace, with groups around the word and the brace.
    pub(super) fn layout_herald(&mut self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'\'') || self.word_before(p) {
            return None;
        }
        let words: [(&[u8], usize); 5] = [
            (b"where", 1),
            (b"let", 1),
            (b"mdo", 1),
            (b"do", 1),
            (b"of", 2),
        ];
        let (end, group, brace) = words.iter().find_map(|&(word, group)| {
            let end = self.lit(p, word)?;
            let brace = self.space_end(end);
            (brace < self.lim() && self.at(brace) == b'{' && self.at(brace + 1) != b'-')
                .then_some((end, group, brace))
        })?;
        self.capture(group, p, end);
        self.capture(3, brace, brace + 1);
        Some(brace + 1)
    }

    /// `\b(?<!')(?:(where|let|in|default)|(m?
    /// do|if|then|else|case|of|proc|rec))\b(?!')`, with a group around the
    /// word.
    pub(super) fn keyword(&mut self, p: usize) -> Option<usize> {
        let first: [&[u8]; 4] = [b"where", b"let", b"in", b"default"];
        let second: [&[u8]; 9] = [
            b"mdo", b"do", b"if", b"then", b"else", b"case", b"of", b"proc", b"rec",
        ];
        if let Some(end) = first.iter().find_map(|word| self.keyword_at(p, word)) {
            self.capture(1, p, end);
            return Some(end);
        }
        let end = second.iter().find_map(|word| self.keyword_at(p, word))?;
        self.capture(2, p, end);
        Some(end)
    }

    /// `^\s*(infix[lr]?)\s+(.*)`: a fixity declaration, with groups around
    /// the keyword and the operators.
    ///
    /// Giving back `l` or `r` leaves it where blanks must stand.
    pub(super) fn fixity(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let indent = self.space_end(0);
        let infix = self.lit(indent, b"infix")?;
        let word = if matches!(self.at(infix), b'l' | b'r') && infix < self.lim() {
            infix + 1
        } else {
            infix
        };
        let rest = self.space_end(word);
        if rest == word {
            return None;
        }
        let end = rest.max(self.lim());
        self.capture(1, indent, word);
        self.capture(2, rest, end);
        Some(end)
    }
}

/// What follows a constructor's first argument.
#[derive(Clone, Copy)]
enum InfixConstructor {
    /// `:[OP]*`, from and to.
    Operator(usize, usize),
    /// `` `Name` ``: the opening backtick, the name's end and the match's end.
    Backticks(usize, usize, usize),
}

/// A foreign function's name or operator.
#[derive(Clone, Copy)]
enum ForeignName {
    /// A name, and its end.
    Name(usize),
    /// An operator in parentheses: where it starts and ends, and where the
    /// `)` stands.
    Operator((usize, usize, usize)),
}
