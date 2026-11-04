//! At-rules, media queries, keyframes and mixin guards.

use super::super::engine::Lexer;
use super::chars::{Class, Escape};
use crate::rules::Chars;

/// What follows an at-rule's name in a lookahead for it.
#[derive(Clone, Copy, Debug)]
pub(super) enum Tail {
    /// Anything.
    Any,
    /// `\b`.
    Boundary,
}

impl Lexer<'_> {
    /// Whether `(?=\{|$)` holds at `p`: a `{` follows, or the line ends,
    /// its line feed taken or not.
    fn block_or_eol(&self, p: usize) -> bool {
        p >= self.lim() || self.at(p) == b'{'
    }

    /// `(?=\s*@word)`, or `(?=\s*@word\b)`.
    pub(super) fn at_rule_ahead(&self, p: usize, word: &[u8], tail: Tail) -> Option<usize> {
        let at = self.spaces(p);
        let end = self.lit(at, b"@").and_then(|name| self.lit(name, word))?;
        match tail {
            Tail::Any => Some(p),
            Tail::Boundary => self.boundary(end).then_some(p),
        }
    }

    /// `((@)word)` at `at`: the whole in group 1 and the `@` in group 2.
    fn at_name(&mut self, at: usize, word: &[u8]) -> Option<usize> {
        let end = self.lit(at, b"@").and_then(|name| self.lit(name, word))?;
        self.cap(1, at, end);
        self.cap(2, at, at + 1);
        Some(end)
    }

    /// `((@)word)`.
    pub(super) fn at_keyword(&mut self, p: usize, word: &[u8]) -> Option<usize> {
        self.at_name(p, word)
    }

    /// `\s*((@)word)`.
    /// It is out of line because `less_matcher` calls it directly.
    #[inline(never)]
    pub(super) fn spaced_at_keyword(&mut self, p: usize, word: &[u8]) -> Option<usize> {
        let at = self.spaces(p);
        self.at_name(at, word)
    }

    /// `\s*((@)word)\b\s*`.
    pub(super) fn at_rule_word(&mut self, p: usize, word: &[u8]) -> Option<usize> {
        let at = self.spaces(p);
        let end = self.lit(at, b"@").and_then(|name| self.lit(name, word))?;
        if !self.boundary(end) {
            return None;
        }
        self.at_name(at, word)?;
        Some(self.spaces(end))
    }

    /// `\s*((@)namespace)\s+`.
    pub(super) fn at_namespace(&mut self, p: usize) -> Option<usize> {
        let at = self.spaces(p);
        let end = self.lit(at, b"@namespace")?;
        let after = self.spaces_run(end)?;
        self.at_name(at, b"namespace")?;
        Some(after)
    }

    /// `\s*((@)font-face)\s*(?=\{|$)`.
    pub(super) fn at_font_face(&mut self, p: usize) -> Option<usize> {
        let at = self.spaces(p);
        let end = self.lit(at, b"@font-face")?;
        let after = self.spaces(end);
        if !self.block_or_eol(after) {
            return None;
        }
        self.at_name(at, b"font-face")?;
        Some(after)
    }

    /// `\s*((@)keyframes)(?=.*?\{)`.
    pub(super) fn at_keyframes(&mut self, p: usize) -> Option<usize> {
        let at = self.spaces(p);
        let end = self.lit(at, b"@keyframes")?;
        let lexer = &*self;
        let brace = *lexer.x.brace.get_or_init(move || {
            let brace = lexer.text(0, lexer.lim()).iter().rposition(|&b| b == b'{');
            lexer.spend(lexer.lim() - brace.unwrap_or(0));
            brace
        });
        if brace.is_none_or(|brace| brace < end) {
            return None;
        }
        self.at_name(at, b"keyframes")
    }

    /// `\s*((@)page)\s*(?:(:)(first|left|right))?\s*(?=\{|$)`.
    pub(super) fn at_page(&mut self, p: usize) -> Option<usize> {
        let at = self.spaces(p);
        let end = self.lit(at, b"@page")?;
        let colon = self.spaces(end);
        if self.at(colon) == b':'
            && let Some(name) = self.first_of(colon + 1, &[b"first", b"left", b"right"])
        {
            let after = self.spaces(name);
            if self.block_or_eol(after) {
                self.at_name(at, b"page")?;
                self.cap(3, colon, colon + 1);
                self.cap(4, colon + 1, name);
                return Some(after);
            }
        }
        if !self.block_or_eol(colon) {
            return None;
        }
        self.at_name(at, b"page")?;
        Some(colon)
    }

    /// `\s*((@)counter-style)\b\s+(?:(?i:\b(decimal|none)\b)|(NAME))\s*(?=\
    /// {|$)`.
    pub(super) fn at_counter_style(&mut self, p: usize) -> Option<usize> {
        let at = self.spaces(p);
        let end = self.lit(at, b"@counter-style")?;
        if !self.boundary(end) {
            return None;
        }
        let name = self.spaces_run(end)?;
        let keyword = [b"decimal".as_slice(), b"none"]
            .into_iter()
            .find_map(|word| {
                self.boundary(name)
                    .then(|| self.folded_end(name, word))
                    .flatten()
                    .filter(|&end| self.boundary(end))
            })
            .filter(|&stop| self.block_or_eol(self.spaces(stop)));
        let (group, stop) = if let Some(stop) = keyword {
            (3, stop)
        } else {
            let stop = self
                .name(name, Class::Alpha, Class::DashWord, Escape::Hex)
                .filter(|&stop| self.block_or_eol(self.spaces(stop)))?;
            (4, stop)
        };
        self.at_name(at, b"counter-style")?;
        self.cap(group, name, stop);
        Some(self.spaces(stop))
    }

    /// `(?<=(["'])|(["']\)))\s*`: white space after a quoted or `url("…")`
    /// import.
    pub(super) fn after_import_url(&self, p: usize) -> Option<usize> {
        let quote = |b: Option<u8>| matches!(b, Some(b'"' | b'\''));
        let after =
            quote(self.prev(p)) || self.prev(p) == Some(b')') && p >= 2 && quote(self.prev(p - 1));
        after.then(|| self.spaces(p))
    }

    /// `\s*(?:(from|to)|((?:\.[0-9]+|[0-9]+(?:\.[0-9]*)?)(%)))\s*,?\s*`: group
    /// 2 holds the `%` as well as the number.
    pub(super) fn keyframe_selector(&mut self, p: usize) -> Option<usize> {
        let at = self.spaces(p);
        let end = if let Some(end) = self.first_of(at, &[b"from", b"to"]) {
            self.cap(1, at, end);
            end
        } else {
            let digits = self.ascii_digits(at);
            let number = if digits > at {
                if self.at(digits) == b'.' {
                    self.ascii_digits(digits + 1)
                } else {
                    digits
                }
            } else if self.at(at) == b'.' && self.ascii_digits(at + 1) > at + 1 {
                self.ascii_digits(at + 1)
            } else {
                return None;
            };
            if self.at(number) != b'%' || number >= self.lim() {
                return None;
            }
            self.cap(2, at, number + 1);
            self.cap(3, number, number + 1);
            number + 1
        };
        let comma = self.spaces(end);
        let comma = if self.at(comma) == b',' && comma < self.lim() {
            comma + 1
        } else {
            comma
        };
        Some(self.spaces(comma))
    }

    /// `\s*(only|not)?\s*
    /// (all|aural|braille|embossed|handheld|print|projection|screen|tty|tv)?`,
    /// It always matches.
    #[expect(
        clippy::unnecessary_wraps,
        reason = "less_matcher tail-calls it and needs the same return type"
    )]
    pub(super) fn media_query(&mut self, p: usize) -> Option<usize> {
        const TYPES: [&[u8]; 10] = [
            b"all",
            b"aural",
            b"braille",
            b"embossed",
            b"handheld",
            b"print",
            b"projection",
            b"screen",
            b"tty",
            b"tv",
        ];
        let at = self.spaces(p);
        let mut end = at;
        if let Some(stop) = self.first_of(at, &[b"only", b"not"]) {
            self.cap(1, at, stop);
            end = stop;
        }
        let kind = self.spaces(end);
        if let Some(stop) = self.first_of(kind, &TYPES) {
            self.cap(2, kind, stop);
            return Some(stop);
        }
        Some(kind)
    }

    /// `\s*(and)?\s*(\()\s*`.
    pub(super) fn media_feature_open(&mut self, p: usize) -> Option<usize> {
        let at = self.spaces(p);
        if let Some(and) = self.lit(at, b"and") {
            let open = self.spaces(and);
            if self.at(open) == b'(' && open < self.lim() {
                self.cap(1, at, and);
                self.cap(2, open, open + 1);
                return Some(self.spaces(open + 1));
            }
        }
        if self.at(at) != b'(' || at >= self.lim() {
            return None;
        }
        self.cap(2, at, at + 1);
        Some(self.spaces(at + 1))
    }

    /// `\s*(\d+)(/)(\d+)`.
    pub(super) fn ratio(&mut self, p: usize) -> Option<usize> {
        let at = self.spaces(p);
        let first = self.digits(at);
        if first == at || self.at(first) != b'/' {
            return None;
        }
        let second = self.digits(first + 1);
        if second == first + 1 {
            return None;
        }
        self.cap(1, at, first);
        self.cap(2, first, first + 1);
        self.cap(3, first + 1, second);
        Some(second)
    }

    /// `\s*(when)(?=.*?)`.
    pub(super) fn when(&mut self, p: usize) -> Option<usize> {
        let at = self.spaces(p);
        let end = self.lit(at, b"when")?;
        self.cap(1, at, end);
        Some(end)
    }

    /// `(?=\s*when\b)`.
    pub(super) fn when_ahead(&self, p: usize) -> Option<usize> {
        let end = self.lit(self.spaces(p), b"when")?;
        self.boundary(end).then_some(p)
    }

    /// `\s*(and|not|or)?\s*(?=\()`.
    pub(super) fn guard_operator(&mut self, p: usize) -> Option<usize> {
        let at = self.spaces(p);
        if let Some(end) = self.first_of(at, &[b"and", b"not", b"or"]) {
            let open = self.spaces(end);
            if self.at(open) == b'(' && open < self.lim() {
                self.cap(1, at, end);
                return Some(open);
            }
        }
        (self.at(at) == b'(' && at < self.lim()).then_some(at)
    }

    /// `default((\()(\)))`.
    pub(super) fn default_call(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"default()")?;
        self.cap(1, p + 7, end);
        self.cap(2, p + 7, p + 8);
        self.cap(3, p + 8, end);
        Some(end)
    }

    /// `\s*(?=[^;{])`: the white space gives back its last character when no
    /// other follows it.
    pub(super) fn spaces_before_other(&self, p: usize) -> Option<usize> {
        let end = self.spaces(p);
        if end < self.lim() && !matches!(self.at(end), b';' | b'{') {
            return Some(end);
        }
        (end > p).then(|| self.char_start_before(end))
    }

    /// `\s*(?:(,)|(?=[;{]))`.
    pub(super) fn comma_or_block(&mut self, p: usize) -> Option<usize> {
        let at = self.spaces(p);
        match self.at(at) {
            b',' if at < self.lim() => {
                self.cap(1, at, at + 1);
                Some(at + 1)
            }
            b';' | b'{' if at < self.lim() => Some(at),
            _ => None,
        }
    }

    /// `\s*((?=;|$))`.
    pub(super) fn semicolon_or_eol_ahead(&self, p: usize) -> Option<usize> {
        let end = self.spaces(p);
        (end >= self.lim() || self.at(end) == b';').then_some(end)
    }
}
