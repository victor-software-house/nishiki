//! Selectors: class, id and placeholder names, the `&` suffix, custom
//! elements, `:nth-*()` and attribute selectors.
//!
//! The names share one repeated group, `(?:[-0-9A-Z_a-z[^\x00-\x7F]]|
//! \\(?:\h{1,6}|.)|#\{|\.?\$|})+`. Each place in the line starts at most one
//! of its alternatives, and a backslash's hex digits are name characters
//! too, so the ends the group can stop at are the ends of the units read one
//! after another; a backslash with `\h{1,6}` stops at no end that this
//! misses.

use super::super::engine::Lexer;
use crate::rules::Chars;

/// The sigil before a selector name, which decides the lookahead after it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Sigil {
    /// `.`, a class.
    Class,
    /// `#`, an id.
    Id,
    /// `%`, a placeholder.
    Placeholder,
}

/// The value of an attribute selector.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Value {
    /// The name group, ending here.
    Bare(usize),
    /// A string in `quote`s, whose closing one is at `close`.
    Quoted { quote: u8, close: usize },
}

/// Whether a name's units include `.$`, a dot before a variable.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DotDollar {
    Allowed,
    Absent,
}

/// The pseudo-classes a custom element's lookahead accepts after `:`, as
/// prefixes: no `\b` follows them.
const CUSTOM_PSEUDO: &[&[u8]] = &[
    b"link",
    b"visited",
    b"hover",
    b"active",
    b"focus",
    b"target",
    b"lang",
    b"disabled",
    b"enabled",
    b"checked",
    b"indeterminate",
    b"root",
    b"nth-child",
    b"nth-of-type",
    b"nth-last-child",
    b"nth-last-of-type",
    b"first-child",
    b"last-child",
    b"first-of-type",
    b"last-of-type",
    b"only-child",
    b"only-of-type",
    b"empty",
    b"not",
    b"valid",
    b"invalid",
];

impl Lexer<'_> {
    /// The end of the name unit at `p`, if one starts there.
    fn name_unit(&self, p: usize, dots: DotDollar) -> Option<usize> {
        let b = *self.s.get(p)?;
        match b {
            b'-' | b'_' | b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' | b'$' | b'}' => Some(p + 1),
            0x80.. => Some(self.char_end(p)),
            b'\\' => (p + 1 < self.lim()).then(|| self.char_end(p + 1)),
            b'#' => (self.s.get(p + 1) == Some(&b'{')).then_some(p + 2),
            b'.' => {
                (dots == DotDollar::Allowed && self.s.get(p + 1) == Some(&b'$')).then_some(p + 2)
            }
            _ => None,
        }
    }

    /// The greedy repeated name group from `start` followed by `ahead`: the
    /// last unit end where `ahead` holds, as the group gives units back.
    /// `units` is left at the end of the units.
    fn name_greedy(
        &self,
        start: usize,
        dots: DotDollar,
        units: &mut usize,
        ahead: impl Fn(&Self, usize) -> bool,
    ) -> Option<usize> {
        let mut at = start;
        let mut found = None;
        while let Some(next) = self.name_unit(at, dots) {
            at = next;
            if ahead(self, at) {
                found = Some(at);
            }
        }
        self.spend(at - start);
        *units = at;
        found
    }

    /// `(?=$|[#)+,:>\[{|~\s]|…)`, the lookahead shared by the names, with
    /// `extra` for the bytes one of them adds.
    fn name_stop(&self, p: usize, extra: &[u8]) -> bool {
        self.at_eol(p)
            || matches!(
                self.at(p),
                b'#' | b')' | b'+' | b',' | b':' | b'>' | b'[' | b'{' | b'|' | b'~'
            )
            || extra.contains(&self.at(p))
            || self.space_at(p) > 0
            || self.starts(p, b"/*")
    }

    /// `\.[^$]` at `p`: a dot and a character after it, the line feed
    /// included, that is not `$`.
    fn dot_not_dollar(&self, p: usize) -> bool {
        self.at(p) == b'.'
            && (p + 1 < self.lim() || (p + 1 == self.lim() && self.nl))
            && self.at(p + 1) != b'$'
    }

    /// The class, id and placeholder names: the sigil, then the name group,
    /// then each one's lookahead.
    pub(super) fn selector_name(&mut self, p: usize, sigil: Sigil) -> Option<usize> {
        let byte = match sigil {
            Sigil::Class => b'.',
            Sigil::Id => b'#',
            Sigil::Placeholder => b'%',
        };
        if self.at(p) != byte || self.name_unit(p + 1, DotDollar::Allowed).is_none() {
            return None;
        }
        // A name read from an earlier sigil holds this one as `.$`: its
        // units go on from here as they did, and so does its match.
        let (cached, from, to, last) = self.x.name.get();
        let end = if cached == byte && from < p && p < to {
            last.filter(|&end| end > p + 1)
        } else {
            let mut units = p + 1;
            let found =
                self.name_greedy(
                    p + 1,
                    DotDollar::Allowed,
                    &mut units,
                    |lexer, k| match sigil {
                        Sigil::Class | Sigil::Placeholder => {
                            lexer.name_stop(k, b";") || lexer.dot_not_dollar(k)
                        }
                        Sigil::Id => lexer.name_stop(k, b"") || lexer.dot_not_dollar(k),
                    },
                );
            self.x.name.set((byte, p, units, found));
            found
        }?;
        self.cap(1, p, p + 1);
        self.cap(2, p + 1, end);
        Some(end)
    }

    /// `(?<=&)((?:[-0-9A-Z_a-z[^\x00-\x7F]]|\\(?:\h{1,6}|.)|#\{|[$}])+)
    /// (?=$|[#)+,.:>\[{|~\s]|/\*)`.
    pub(super) fn parent_selector_suffix(&mut self, p: usize) -> Option<usize> {
        if !self.prev_is(p, b'&') {
            return None;
        }
        let mut units = p;
        let end = self.name_greedy(p, DotDollar::Absent, &mut units, |lexer, k| {
            lexer.name_stop(k, b".")
        })?;
        self.cap(1, p, end);
        Some(end)
    }

    /// The lookahead of a custom element at `k`: `\.`, `\s++[^:]`,
    /// `\s*[,\[{]`, or `:` and one of [`CUSTOM_PSEUDO`].
    fn custom_ahead(&self, k: usize) -> bool {
        if self.at(k) == b'.' {
            return true;
        }
        let spaces = self.spaces(k);
        if spaces > k && spaces < self.lim() && self.at(spaces) != b':' {
            return true;
        }
        if matches!(self.at(spaces), b',' | b'[' | b'{') {
            return true;
        }
        self.at(k) == b':' && CUSTOM_PSEUDO.iter().any(|word| self.starts(k + 1, word))
    }

    /// `\b([0-9A-Za-z]+(-[0-9A-Za-z]+)+)(?=…)`: the last part gives
    /// characters back, and then whole parts, until the lookahead holds; the
    /// first part cannot, as a dash must follow it.
    ///
    /// A `\b` holds at the start of every part, so a chain read from its
    /// first part answers for the later ones: their match ends at the same
    /// place, when that lies past their own first part.
    pub(super) fn selector_custom(&mut self, p: usize) -> Option<usize> {
        if !self.at(p).is_ascii_alphanumeric() || self.word_before(p) {
            return None;
        }
        let first = self.skip(p, |b| b.is_ascii_alphanumeric());
        let (from, chain_end, best) = self.x.custom.get();
        let end = if from < p && p < chain_end {
            best.filter(|&k| k > first + 1)
        } else {
            let mut at = first;
            let mut found = None;
            while self.at(at) == b'-' && self.at(at + 1).is_ascii_alphanumeric() {
                let part = at + 1;
                at = self.skip(part, |b| b.is_ascii_alphanumeric());
                for k in part + 1..=at {
                    if self.custom_ahead(k) {
                        found = Some(k);
                    }
                }
            }
            self.x.custom.set((p, at, found));
            found
        }?;
        self.cap(1, p, end);
        Some(end)
    }

    /// `((:)\bnth-(?:|last-)(?:child|of-type))(\()`.
    pub(super) fn selector_pseudo_class_at0(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b':' {
            return None;
        }
        let nth = self.lit(p + 1, b"nth-")?;
        let kind = [Some(nth), self.lit(nth, b"last-")]
            .into_iter()
            .flatten()
            .find_map(|from| {
                self.lit(from, b"child")
                    .or_else(|| self.lit(from, b"of-type"))
            })?;
        if self.at(kind) != b'(' {
            return None;
        }
        self.cap(1, p, kind);
        self.cap(2, p, p + 1);
        self.cap(3, kind, kind + 1);
        Some(kind + 1)
    }

    /// `\d+`.
    pub(super) fn selector_pseudo_class_at0_at1(&mut self, p: usize) -> Option<usize> {
        let end = self.digit_end(p);
        (end > p).then_some(end)
    }

    /// `(?:(?<=\d)n|\b(n|even|odd))\b`.
    pub(super) fn selector_pseudo_class_at0_at2(&mut self, p: usize) -> Option<usize> {
        if self.at(p) == b'n'
            && self.char_before(p).is_some_and(crate::classes::is_digit)
            && self.word_at(p + 1).is_none()
        {
            return Some(p + 1);
        }
        self.word_in(p, &[b"n", b"even", b"odd"])
    }

    /// The value after an attribute selector's operator at `p`: the name
    /// group, or a quoted string up to a quote that `\s*]` follows, as the
    /// lazy `(.*?)` takes the first such quote.
    fn attribute_value(&self, p: usize) -> Option<Value> {
        let quote = self.at(p);
        if quote == b'"' || quote == b'\'' {
            return self
                .quote_bracket(quote, p + 1)
                .map(|close| Value::Quoted { quote, close });
        }
        let mut at = p;
        while let Some(next) = self.name_unit(at, DotDollar::Allowed) {
            at = next;
        }
        self.spend(at - p);
        (at > p).then_some(Value::Bare(at))
    }

    /// The first `quote` at or after `from` that `\s*]` follows, which a
    /// search from before it finds again.
    fn quote_bracket(&self, quote: u8, from: usize) -> Option<usize> {
        let (cached, searched, found) = self.x.quote.get();
        if cached == quote && searched <= from && found.is_none_or(|at| at >= from) {
            return found;
        }
        let mut at = from;
        let found = loop {
            if at >= self.lim() {
                break None;
            }
            if self.at(at) == quote && self.at(self.spaces(at + 1)) == b']' {
                break Some(at);
            }
            at += 1;
        };
        self.spend(at - from);
        self.x.quote.set((quote, from, found));
        found
    }

    /// `(?:\s*([$*^|~]?=)\s*(?:value))?\s*(])` after an attribute's name
    /// at `k`: with the operator and value, and else without. The groups
    /// are recorded once the whole of it matches.
    fn attribute_rest(&mut self, k: usize) -> Option<usize> {
        let op = self.spaces(k);
        let equals =
            if matches!(self.at(op), b'$' | b'*' | b'^' | b'|' | b'~') && self.at(op + 1) == b'=' {
                Some(op + 2)
            } else {
                (self.at(op) == b'=').then_some(op + 1)
            };
        if let Some(equals) = equals {
            let start = self.spaces(equals);
            if let Some(value) = self.attribute_value(start) {
                let end = match value {
                    Value::Bare(end) => end,
                    Value::Quoted { close, .. } => close + 1,
                };
                let bracket = self.spaces(end);
                if self.at(bracket) == b']' {
                    self.cap(3, op, equals);
                    match value {
                        Value::Bare(end) => self.cap(4, start, end),
                        Value::Quoted { quote, close } => {
                            let first = if quote == b'"' { 5 } else { 9 };
                            self.cap(first, start, close + 1);
                            self.cap(first + 1, start, start + 1);
                            self.cap(first + 2, start + 1, close);
                            self.cap(first + 3, close, close + 1);
                        }
                    }
                    self.cap(13, bracket, bracket + 1);
                    return Some(bracket + 1);
                }
            }
        }
        let bracket = self.spaces(k);
        if self.at(bracket) != b']' {
            return None;
        }
        self.cap(13, bracket, bracket + 1);
        Some(bracket + 1)
    }

    /// `(?i)(\[)\s*(name+?)(?:…)?\s*(])`: the name is lazy, so it takes one
    /// unit, then one more, until the rest matches after it.
    pub(super) fn selector_attribute(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'[' {
            return None;
        }
        let name = self.spaces(p + 1);
        let mut at = name;
        while let Some(next) = self.name_unit(at, DotDollar::Allowed) {
            at = next;
            if let Some(end) = self.attribute_rest(at) {
                self.spend(at - name);
                self.cap(1, p, p + 1);
                self.cap(2, name, at);
                return Some(end);
            }
        }
        self.spend(at - name);
        None
    }
}
