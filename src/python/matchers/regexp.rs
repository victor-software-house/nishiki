//! The matchers of regular-expression strings: `r"..."` and its kin.
//!
//! The grammar writes each rule of the expression four times, once for each
//! quote style, so the ends and the nested rules take a [`Flavour`].

use crate::python::engine::{Ahead, Lexer};
use crate::python::grammar::{Flavour, Pat, Re};

impl Flavour {
    /// The quote text that closes a string of this style.
    fn quote(self) -> &'static [u8] {
        match self {
            Flavour::SingleOne => b"'",
            Flavour::DoubleOne => b"\"",
            Flavour::SingleThree => b"'''",
            Flavour::DoubleThree => b"\"\"\"",
        }
    }

    /// Whether the string spans lines.
    fn three(self) -> bool {
        matches!(self, Flavour::SingleThree | Flavour::DoubleThree)
    }
}

impl Lexer<'_> {
    #[expect(
        clippy::wildcard_enum_match_arm,
        reason = "the dispatch sends this function its own patterns only"
    )]
    pub(super) fn regexp_matcher(&mut self, pat: Pat, p: usize) -> Option<usize> {
        if let Some((flavour, kind)) = pat.regexp() {
            return self.flavoured(flavour, kind, p);
        }
        match pat {
            Pat::RegexpSingleOneLine => self.regexp_begin(p, Flavour::SingleOne),
            Pat::RegexpDoubleOneLine => self.regexp_begin(p, Flavour::DoubleOne),
            Pat::RegexpSingleThreeLine => self.regexp_begin(p, Flavour::SingleThree),
            Pat::RegexpDoubleThreeLine => self.regexp_begin(p, Flavour::DoubleThree),
            Pat::RegexpSingleOneLineEnd => self.regexp_line_end(p, Flavour::SingleOne),
            Pat::RegexpDoubleOneLineEnd => self.regexp_line_end(p, Flavour::DoubleOne),
            Pat::RegexpSingleThreeLineEnd => self.regexp_line_end(p, Flavour::SingleThree),
            Pat::RegexpDoubleThreeLineEnd => self.regexp_line_end(p, Flavour::DoubleThree),
            Pat::CommentsStringSingleThree | Pat::CommentsStringDoubleThree => self.cap_hash(p),
            Pat::CommentsStringSingleThreeEnd => {
                (self.at_eol(p) || self.starts(p, b"'''")).then_some(p)
            }
            Pat::CommentsStringDoubleThreeEnd => {
                (self.at_eol(p) || self.starts(p, b"\"\"\"")).then_some(p)
            }
            Pat::RegexpQuantifier => self.quantifier(p),
            Pat::RegexpBaseCommonAt0 => (self.at(p) == b'.' && p < self.lim()).then_some(p + 1),
            Pat::RegexpBaseCommonAt1 => (self.at(p) == b'^' && p < self.lim()).then_some(p + 1),
            Pat::RegexpBaseCommonAt2 => (self.at(p) == b'$' && p < self.lim()).then_some(p + 1),
            Pat::RegexpBaseCommonAt3 => {
                if !matches!(self.at(p), b'*' | b'+' | b'?') || p >= self.lim() {
                    return None;
                }
                Some(p + 1 + usize::from(self.at(p + 1) == b'?' && p + 1 < self.lim()))
            }
            Pat::RegexpBaseCommonAt4 => (self.at(p) == b'|' && p < self.lim()).then_some(p + 1),
            Pat::RegexpEscapeSpecial => (self.at(p) == b'\\'
                && b"ABDSWZbdsw".contains(&self.at(p + 1))
                && p + 1 < self.lim())
            .then_some(p + 2),
            Pat::RegexpEscapeCharacter => self.escape_character(p),
            Pat::RegexpEscapeUnicode => self.escape_unicode(p),
            Pat::RegexpBackreferenceNumber => self.backreference_number(p),
            Pat::RegexpEscapeCatchall => self.escape_catchall(p),
            Pat::RegexpFlags => self.flags(p),
            Pat::RegexpBackreference => self.named_backreference(p),
            Pat::RegexpCharacterSetEscapesAt0 => (self.at(p) == b'\\'
                && b"\\abfnrtv".contains(&self.at(p + 1))
                && p + 1 < self.lim())
            .then_some(p + 2),
            Pat::RegexpCharacterSetEscapesAt2 => self.octal_escape(p),
            _ => None,
        }
    }

    /// The regex `(#)`.
    fn cap_hash(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'#' || p >= self.lim() {
            return None;
        }
        self.cap(1, p, p + 1);
        Some(p + 1)
    }

    /// The begin `\b(([Uu]r)|([Bb]r)|(r[Bb]?))(Q)` of a regular-expression
    /// string with quote `Q`.
    fn regexp_begin(&mut self, p: usize, flavour: Flavour) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let (a, b, c) = (self.at(p), self.at(p + 1), self.at(p + 2));
        let quote = flavour.quote();
        // The alternatives in order; `r[Bb]?` takes the `B` first.
        let (end, group) = if matches!(a, b'U' | b'u') && b == b'r' {
            (p + 2, 2)
        } else if matches!(a, b'B' | b'b') && b == b'r' {
            (p + 2, 3)
        } else if a == b'r' {
            if matches!(b, b'B' | b'b') && self.starts(p + 2, quote) {
                (p + 2, 4)
            } else {
                (p + 1, 4)
            }
        } else {
            return None;
        };
        let _ = c;
        if !self.starts(end, quote) {
            return None;
        }
        self.cap(group, p, end);
        self.cap(5, end, end + quote.len());
        Some(end + quote.len())
    }

    /// The end `(')|(?<!\\)(\n)` or `(''')` of a regular-expression string.
    fn regexp_line_end(&mut self, p: usize, flavour: Flavour) -> Option<usize> {
        let quote = flavour.quote();
        if self.starts(p, quote) {
            self.cap(1, p, p + quote.len());
            return Some(p + quote.len());
        }
        if !flavour.three() && p == self.lim() && !self.prev_is(p, b'\\') {
            self.cap(2, p, p + 1);
            return Some(p + 1);
        }
        None
    }

    /// The end `(\)|(?="))|((?=(?<!\\)\n))` of a nested rule that closes with
    /// `close`: the character is group 1, and the string's end or a line
    /// feed ends it before them.
    fn regexp_close(&mut self, p: usize, flavour: Flavour, close: u8) -> Option<usize> {
        if self.at(p) == close && p < self.lim() {
            self.cap(1, p, p + 1);
            return Some(p + 1);
        }
        if self.starts(p, flavour.quote()) {
            return Some(p);
        }
        (!flavour.three() && p == self.lim() && !self.prev_is(p, b'\\')).then_some(p)
    }

    /// The patterns written once for each quote style.
    fn flavoured(&mut self, flavour: Flavour, kind: Re, p: usize) -> Option<usize> {
        match kind {
            Re::CharacterSetAt0 => self.empty_set(p),
            Re::CharacterSetAt1 => self.set_begin(p),
            Re::CharacterSetAt1End => self.regexp_close(p, flavour, b']'),
            Re::CharacterSetAt1At1 => (p < self.lim()).then(|| self.char_end(p)),
            Re::Comments => self.starts(p, b"(?#").then_some(p + 3),
            Re::NamedGroup => self.named_group(p),
            Re::Lookahead => self.group_open(p, b"(?="),
            Re::LookaheadNegative => self.group_open(p, b"(?!"),
            Re::Lookbehind => self.group_open(p, b"(?<="),
            Re::LookbehindNegative => self.group_open(p, b"(?<!"),
            Re::Conditional => self.conditional(p),
            Re::ParenthesesNonCapturing => self.starts(p, b"(?:").then_some(p + 3),
            Re::Parentheses => (self.at(p) == b'(' && p < self.lim()).then_some(p + 1),
            Re::CommentsEnd
            | Re::NamedGroupEnd
            | Re::LookaheadEnd
            | Re::LookaheadNegativeEnd
            | Re::LookbehindEnd
            | Re::LookbehindNegativeEnd
            | Re::ConditionalEnd
            | Re::ParenthesesNonCapturingEnd
            | Re::ParenthesesEnd => self.regexp_close(p, flavour, b')'),
        }
    }

    /// The match `\[\^?](?!.*?])`: an empty set with no `]` after it.
    fn empty_set(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'[' || p >= self.lim() {
            return None;
        }
        let q = p + 1 + usize::from(self.at(p + 1) == b'^' && p + 1 < self.lim());
        if self.at(q) != b']' || q >= self.lim() {
            return None;
        }
        self.last(Ahead::CloseBracket)
            .is_none_or(|last| last <= q)
            .then_some(q + 1)
    }

    /// The begin `(\[)(\^)?(])?`.
    fn set_begin(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'[' || p >= self.lim() {
            return None;
        }
        self.cap(1, p, p + 1);
        let mut q = p + 1;
        if self.at(q) == b'^' && q < self.lim() {
            self.cap(2, q, q + 1);
            q += 1;
        }
        if self.at(q) == b']' && q < self.lim() {
            self.cap(3, q, q + 1);
            q += 1;
        }
        Some(q)
    }

    /// `\w+(?:\s+\p{alnum}+)?` at `i`, followed by `close`: where `close`
    /// ends. The optional part is tried first.
    fn word_and_alnum(&self, i: usize, close: u8) -> Option<usize> {
        let w = self.word_end(i);
        if w == i {
            return None;
        }
        let sp = self.space_end(w);
        if sp > w {
            let mut e = sp;
            while let Some((c, next)) = self.char_at(e)
                && e < self.lim()
                && super::chars::is_alnum(c)
            {
                e = next;
            }
            if e > sp && self.at(e) == close && e < self.lim() {
                return Some(e + 1);
            }
        }
        (self.at(w) == close && w < self.lim()).then_some(w + 1)
    }

    /// The begin `(\()(\?P<\w+(?:\s+\p{alnum}+)?>)`.
    fn named_group(&mut self, p: usize) -> Option<usize> {
        if !self.starts(p, b"(?P<") {
            return None;
        }
        let end = self.word_and_alnum(p + 4, b'>')?;
        self.cap(1, p, p + 1);
        self.cap(2, p + 1, end);
        Some(end)
    }

    /// The begin `(\()\?=` and its kin: `open` is the whole text.
    fn group_open(&mut self, p: usize, open: &[u8]) -> Option<usize> {
        if !self.starts(p, open) {
            return None;
        }
        self.cap(1, p, p + 1);
        Some(p + open.len())
    }

    /// The begin `(\()\?\((\w+(?:\s+\p{alnum}+)?|\d+)\)`.
    fn conditional(&mut self, p: usize) -> Option<usize> {
        if !self.starts(p, b"(?(") {
            return None;
        }
        let end = self.word_and_alnum(p + 3, b')')?;
        self.cap(1, p, p + 1);
        Some(end)
    }

    /// The match `\{(\d+|\d+,(\d+)?|,\d+)}`.
    fn quantifier(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'{' || p >= self.lim() {
            return None;
        }
        let digits = |i: usize| self.digit_end(i);
        let d = digits(p + 1);
        let close = |i: usize| (self.at(i) == b'}' && i < self.lim()).then_some(i + 1);
        if d > p + 1 {
            if let Some(end) = close(d) {
                return Some(end);
            }
            if self.at(d) == b',' && d < self.lim() {
                return close(digits(d + 1));
            }
            return None;
        }
        if self.at(p + 1) == b',' && p + 1 < self.lim() {
            let e = digits(p + 2);
            if e > p + 2 {
                return close(e);
            }
        }
        None
    }

    /// The match `\\(x\h{2}|0[0-7]{1,2}|[0-7]{3})`.
    fn escape_character(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'\\' || p >= self.lim() {
            return None;
        }
        let hex = |i: usize| self.at(i).is_ascii_hexdigit() && i < self.lim();
        let oct = |i: usize| matches!(self.at(i), b'0'..=b'7') && i < self.lim();
        if self.at(p + 1) == b'x' && hex(p + 2) && hex(p + 3) {
            return Some(p + 4);
        }
        if self.at(p + 1) == b'0' && p + 1 < self.lim() && oct(p + 2) {
            return Some(p + 3 + usize::from(oct(p + 3)));
        }
        (oct(p + 1) && oct(p + 2) && oct(p + 3)).then_some(p + 4)
    }

    /// The match `\\(u\h{4}|U\h{8})`.
    fn escape_unicode(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'\\' || p >= self.lim() {
            return None;
        }
        let hex = |from: usize, n: usize| {
            (from..from + n).all(|i| self.at(i).is_ascii_hexdigit() && i < self.lim())
        };
        match self.at(p + 1) {
            b'u' if hex(p + 2, 4) => Some(p + 6),
            b'U' if hex(p + 2, 8) => Some(p + 10),
            _ => None,
        }
    }

    /// The match `(\\[1-9]\d?)`.
    fn backreference_number(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'\\' || !matches!(self.at(p + 1), b'1'..=b'9') || p + 1 >= self.lim() {
            return None;
        }
        // `\d?`: one more digit, of any script.
        let end = match self.char_at(p + 2) {
            Some((c, next)) if p + 2 < self.lim() && super::chars::is_digit(c) => next,
            _ => p + 2,
        };
        self.cap(1, p, end);
        Some(end)
    }

    /// The match `\\(.|\n)`.
    fn escape_catchall(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'\\' || p >= self.lim() {
            return None;
        }
        // The next character, or the line feed.
        Some(if p + 1 < self.lim() {
            self.char_end(p + 1)
        } else {
            p + 2
        })
    }

    /// The match `\(\?[Laimsux]+\)`.
    fn flags(&self, p: usize) -> Option<usize> {
        if !self.starts(p, b"(?") {
            return None;
        }
        let e = self.skip(p + 2, |b| b"Laimsux".contains(&b));
        (e > p + 2 && self.at(e) == b')' && e < self.lim()).then_some(e + 1)
    }

    /// The match `(\()(\?P=\w+(?:\s+\p{alnum}+)?)(\))`.
    fn named_backreference(&mut self, p: usize) -> Option<usize> {
        if !self.starts(p, b"(?P=") {
            return None;
        }
        let end = self.word_and_alnum(p + 4, b')')?;
        self.cap(1, p, p + 1);
        self.cap(2, p + 1, end - 1);
        self.cap(3, end - 1, end);
        Some(end)
    }

    /// The match `\\([0-7]{1,3})` of a character set.
    fn octal_escape(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'\\' || p >= self.lim() {
            return None;
        }
        let mut e = p + 1;
        while e < p + 4 && matches!(self.at(e), b'0'..=b'7') && e < self.lim() {
            e += 1;
        }
        (e > p + 1).then_some(e)
    }
}
