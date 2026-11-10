//! Quote-like operators: `qr`, `m`, `s`, `tr` and `y`, `q`, `qq`, `qw` and
//! `qx`, with any delimiter, and the bare `/regex/`.
//!
//! A rule whose begin takes a delimiter character repeats it in its end
//! (`\1` or `\2`), which the payload holds.
//! The flags after a closing delimiter, `[acdegil-prsux]*`, are followed by
//! a lookahead that a flag letter cannot satisfy.
//! So only the longest run of flags can match, and the matchers take it
//! without giving letters back.

use super::super::engine::Lexer;
use super::{Delim, FlagsEnd, Lead};
use crate::rules::Chars;

/// `[acdegil-prsux]`: a flag of a match or a substitution.
fn is_flag(b: u8) -> bool {
    matches!(
        b,
        b'a' | b'c' | b'd' | b'e' | b'g' | b'i' | b'l'..=b'p' | b'r' | b's' | b'u' | b'x'
    )
}

/// `[acdegilmoprsu]`: a flag other than `n` and `x`, in the flags of a
/// substitution with the `x` flag.
fn is_x_flag(b: u8) -> bool {
    matches!(
        b,
        b'a' | b'c' | b'd' | b'e' | b'g' | b'i' | b'l' | b'm' | b'o' | b'p' | b'r' | b's' | b'u'
    )
}

impl Delim {
    /// Whether the ASCII byte `b` is in the class.
    /// `\s` is left to the caller, which tests it for every character.
    fn admits(self, b: u8) -> bool {
        let word = b.is_ascii_alphanumeric() || b == b'_';
        match self {
            Delim::Qr => !matches!(b, b'\'' | b'(' | b'<' | b'[' | b'{') && !word,
            Delim::QrParen => !matches!(b, b'\'' | b'(' | b'<' | b'[' | b'\\' | b'{') && !word,
            Delim::Word => !matches!(b, b'(' | b'<' | b'[' | b'{') && !word,
            Delim::Subst => !matches!(b, b'(' | b';' | b'<' | b'[' | b'{') && !word,
            Delim::MatchQuote => {
                !matches!(b, b'\'' | b'(' | b'0'..=b'9' | b'<' | b'A'..=b'[' | b'a'..=b'{')
            }
            Delim::SubstLine => !matches!(b, b'(' | b'0'..=b'9' | b'<' | b'A'..=b'[' | b'a'..=b'{'),
            Delim::SubstX => !matches!(b, b'(' | b'<' | b'>' | b'[' | b'{') && !word,
            Delim::NotAlnum => !b.is_ascii_alphanumeric(),
            Delim::NotWord => !word,
        }
    }

    /// Whether the class leaves out `\w`, so a word character past ASCII is
    /// no delimiter.
    fn excludes_words(self) -> bool {
        match self {
            Delim::Qr
            | Delim::QrParen
            | Delim::Word
            | Delim::Subst
            | Delim::SubstX
            | Delim::NotWord => true,
            Delim::MatchQuote | Delim::SubstLine | Delim::NotAlnum => false,
        }
    }
}

impl Lead {
    /// Whether the operator's name can start at `p`.
    fn holds(self, lexer: &Lexer<'_>, p: usize) -> bool {
        match self {
            Lead::Boundary => lexer.boundary(p),
            Lead::NoArrow => lexer.boundary(p) && !(p >= 2 && lexer.starts(p - 2, b"->")),
        }
    }
}

impl Lexer<'_> {
    /// The end of the one delimiter character at `p` that `class` admits.
    /// Every class leaves out `\s`.
    pub(super) fn delim_at(&self, p: usize, class: Delim) -> Option<usize> {
        if !self.avail(p) || self.is_space_at(p) {
            return None;
        }
        let b = self.at(p);
        if b < 0x80 {
            return class.admits(b).then_some(p + 1);
        }
        if class.excludes_words() && self.word_at(p).is_some() {
            return None;
        }
        Some(self.char_end(p))
    }

    /// `\b(?=qr\s*[^\s\w])`: `qr` before a delimiter.
    pub(super) fn qr_ahead(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) || !self.starts(p, b"qr") {
            return None;
        }
        self.delim_at(self.space_end(p + 2), Delim::NotWord)
            .map(|_| p)
    }

    /// `(?<![-+{])\b(?=m\s*[^0-9A-Za-z\s])`: `m` before a delimiter.
    pub(super) fn match_ahead(&self, p: usize) -> Option<usize> {
        if self
            .prev(p)
            .is_some_and(|b| matches!(b, b'-' | b'+' | b'{'))
        {
            return None;
        }
        if !self.boundary(p) || self.at(p) != b'm' || p >= self.lim() {
            return None;
        }
        self.delim_at(self.space_end(p + 1), Delim::NotAlnum)
            .map(|_| p)
    }

    /// `\G(?<![-+{])(m)(?!_)\s*([^'(0-9<A-\[a-{\s])`: `m` and its delimiter,
    /// right where the rule's lookahead began.
    pub(super) fn match_delim(&mut self, p: usize) -> Option<usize> {
        if !self.at_g(p)
            || self
                .prev(p)
                .is_some_and(|b| matches!(b, b'-' | b'+' | b'{'))
        {
            return None;
        }
        if self.at(p) != b'm' || p >= self.lim() || self.at(p + 1) == b'_' {
            return None;
        }
        let at = self.space_end(p + 1);
        let end = self.delim_at(at, Delim::MatchQuote)?;
        self.capture(1, p, p + 1);
        self.capture(2, at, end);
        Some(end)
    }

    /// `\b(?=(?<!&)(s)(\s+\S|\s*[(),;<\[{}]|$))`: `s` before a substitution.
    pub(super) fn subst_ahead(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) || self.prev(p) == Some(b'&') || self.at(p) != b's' || p >= self.lim()
        {
            return None;
        }
        let after = p + 1;
        let at = self.space_end(after);
        let mark = self.avail(at)
            && !self.is_space_at(at)
            && (at > after
                || matches!(
                    self.at(at),
                    b'(' | b')' | b',' | b';' | b'<' | b'[' | b'{' | b'}'
                ));
        (mark || self.at_eol(after)).then_some(p)
    }

    /// `\b(?=s([^(0-9<A-\[a-{\s]).*\1([acdegil-prsux]*)([),;}]|\s+))`: `s`
    /// and a delimiter, which the line repeats later before flags and a
    /// closing mark.
    ///
    /// `.*` takes the rest of the line and gives it back from the end.
    /// So the last repeat that the flags and the mark follow decides, and
    /// any repeat at all is enough for the lookahead.
    pub(super) fn subst_line_ahead(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) || self.at(p) != b's' || p >= self.lim() {
            return None;
        }
        let end = self.delim_at(p + 1, Delim::SubstLine)?;
        let delim = self.text(p + 1, end);
        let lim = self.lim();
        let mut at = end;
        while at + delim.len() <= lim {
            if self.starts(at, delim) {
                let flags = self.skip(at + delim.len(), is_flag);
                let closes = matches!(self.at(flags), b')' | b',' | b';' | b'}') && flags < lim;
                if closes || self.is_space_at(flags) {
                    self.spend(at - end);
                    return Some(p);
                }
            }
            at += 1;
        }
        self.spend(at - end);
        None
    }

    /// `\b(?=(?<!\\)s\s*([^(<>\[{\s\w]))`: `s` before a delimiter, for a
    /// substitution with the `x` flag.
    pub(super) fn subst_x_ahead(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) || self.prev(p) == Some(b'\\') || self.at(p) != b's' || p >= self.lim()
        {
            return None;
        }
        self.delim_at(self.space_end(p + 1), Delim::SubstX)
            .map(|_| p)
    }

    /// `(s\s*)([^(0-9<A-\[a-{\s])`: `s`, its blanks and its delimiter.
    pub(super) fn subst_open(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b's' || p >= self.lim() {
            return None;
        }
        let at = self.space_end(p + 1);
        let end = self.delim_at(at, Delim::SubstLine)?;
        self.capture(1, p, at);
        self.capture(2, at, end);
        Some(end)
    }

    /// `(s)\s*(.)`: `s` and any character after its blanks.
    pub(super) fn subst_any_open(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b's' || p >= self.lim() {
            return None;
        }
        let at = self.space_end(p + 1);
        if at >= self.lim() {
            return None;
        }
        let end = self.char_end(at);
        self.capture(1, p, p + 1);
        self.capture(2, at, end);
        Some(end)
    }

    /// `(name)\s*<open>`: an operator's name, its blanks and an opening
    /// bracket or quote.
    /// The rule's lookahead has tested the word boundary.
    pub(super) fn op_open(&mut self, p: usize, name: &[u8], open: u8) -> Option<usize> {
        let after = self.lit(p, name)?;
        let at = self.space_end(after);
        (self.at(at) == open && at < self.lim()).then(|| {
            self.capture(1, p, after);
            at + 1
        })
    }

    /// `(qr)\s*([^'(<\[{\s\w])`: an operator's name, its blanks and a
    /// delimiter that its end repeats.
    pub(super) fn op_delim(&mut self, p: usize, name: &[u8], class: Delim) -> Option<usize> {
        let after = self.lit(p, name)?;
        let at = self.space_end(after);
        let end = self.delim_at(at, class)?;
        self.capture(1, p, after);
        self.capture(2, at, end);
        Some(end)
    }

    /// `\bqq\s*([^(<\[{\w\s])` and `\bqx\s*([^'(<\[{\w\s])`: an operator's
    /// name, its blanks and a delimiter that its end repeats.
    pub(super) fn quote_word(
        &mut self,
        p: usize,
        name: &[u8],
        lead: Lead,
        class: Delim,
    ) -> Option<usize> {
        if !lead.holds(self, p) {
            return None;
        }
        let at = self.space_end(self.lit(p, name)?);
        let end = self.delim_at(at, class)?;
        self.capture(1, at, end);
        Some(end)
    }

    /// `(?<!->)\bqw?\s*([^(<\[{\w\s])`: `q` or `qw`, its blanks and a
    /// delimiter that its end repeats.
    /// A `w` after the `q` is never the delimiter, which leaves out `\w`, so
    /// the `w?` gives nothing back.
    pub(super) fn q_words(&mut self, p: usize) -> Option<usize> {
        if !Lead::NoArrow.holds(self, p) || self.at(p) != b'q' || p >= self.lim() {
            return None;
        }
        let name = p + 1 + usize::from(self.at(p + 1) == b'w' && p + 1 < self.lim());
        let at = self.space_end(name);
        let end = self.delim_at(at, Delim::Word)?;
        self.capture(1, at, end);
        Some(end)
    }

    /// `\bqx\s*\(` and the like: an operator's name, its blanks and an
    /// opening bracket or quote.
    pub(super) fn op_bracket(&self, p: usize, name: &[u8], lead: Lead, open: u8) -> Option<usize> {
        if !lead.holds(self, p) {
            return None;
        }
        let at = self.space_end(self.lit(p, name)?);
        (self.at(at) == open && at < self.lim()).then_some(at + 1)
    }

    /// `\bqw?\s*\(` and the like: `q` or `qw`, its blanks and an opening
    /// bracket.
    pub(super) fn q_bracket(&self, p: usize, lead: Lead, open: u8) -> Option<usize> {
        if !lead.holds(self, p) || self.at(p) != b'q' || p >= self.lim() {
            return None;
        }
        let name = p + 1 + usize::from(self.at(p + 1) == b'w' && p + 1 < self.lim());
        let at = self.space_end(name);
        (self.at(at) == open && at < self.lim()).then_some(at + 1)
    }

    /// `([^(;<\[{\s\w])` and `([^(0-9<A-\[a-{\s])`: one delimiter that the
    /// rule's end repeats.
    pub(super) fn one_delim(&mut self, p: usize, class: Delim) -> Option<usize> {
        let end = self.delim_at(p, class)?;
        self.capture(1, p, end);
        Some(end)
    }

    /// `\$(?=…)`: a `$` before a delimiter, which is no variable.
    pub(super) fn dollar_before(&self, p: usize, class: Delim) -> Option<usize> {
        (self.at(p) == b'$' && p < self.lim() && self.delim_at(p + 1, class).is_some())
            .then_some(p + 1)
    }

    /// `\\['\\]`: an escaped quote or backslash.
    pub(super) fn quote_escape(&self, p: usize) -> Option<usize> {
        (self.at(p) == b'\\' && matches!(self.at(p + 1), b'\'' | b'\\') && p + 1 < self.lim())
            .then_some(p + 2)
    }

    /// `\1` or `\2`: the delimiter that the rule's begin took.
    pub(super) fn repeat(&self, p: usize) -> Option<usize> {
        self.lit(p, self.payload_text().as_bytes())
    }

    /// `(?=\2)`: before the delimiter that the rule's begin took.
    pub(super) fn repeat_ahead(&self, p: usize) -> Option<usize> {
        self.lit(p, self.payload_text().as_bytes()).map(|_| p)
    }

    /// The flags after a closing delimiter, `((([acdegil-prsux]*)))`, and
    /// the lookahead after them.
    pub(super) fn flags(&mut self, p: usize, end: FlagsEnd) -> Option<usize> {
        let flags = self.skip(p, is_flag);
        self.flags_close(flags, end).then(|| {
            self.capture(1, p, flags);
            self.capture(2, p, flags);
            self.capture(3, p, flags);
            flags
        })
    }

    /// The lookahead after a run of flags that ends at `p`.
    fn flags_close(&self, p: usize, end: FlagsEnd) -> bool {
        let at = self.space_end(p);
        let lim = self.lim();
        let mark = |b: u8| match end {
            FlagsEnd::Hash => matches!(b, b'#' | b')' | b',' | b';' | b'{' | b'}'),
            FlagsEnd::Angle => matches!(b, b']' | b')' | b',' | b';' | b'>' | b'{' | b'}'),
            FlagsEnd::Plain => matches!(b, b')' | b',' | b';' | b'}'),
        };
        match end {
            FlagsEnd::Hash | FlagsEnd::Angle => {
                let next = at < lim && (at > p || mark(self.at(at))) && !self.is_space_at(at);
                next || self.at_eol(at)
            }
            FlagsEnd::Plain => (p < lim && mark(self.at(p))) || at > p || self.at_eol(p),
        }
    }

    /// `((([acdegilmoprsu]*x[acdegilmoprsu]*)))\b`: flags with one `x`.
    pub(super) fn x_flags(&mut self, p: usize) -> Option<usize> {
        let x = self.skip(p, is_x_flag);
        if self.at(x) != b'x' || x >= self.lim() {
            return None;
        }
        let end = self.skip(x + 1, is_x_flag);
        self.boundary(end).then(|| {
            self.capture(1, p, end);
            self.capture(2, p, end);
            self.capture(3, p, end);
            end
        })
    }

    /// Whether flags with one `x`, `[acdegilmoprsu]*x`, start at `p`.
    fn x_flags_at(&self, p: usize) -> bool {
        let x = self.skip(p, is_x_flag);
        self.at(x) == b'x' && x < self.lim()
    }

    /// `'(?=[acdegilmoprsu]*x[acdegilmoprsu]*)\b`: the closing quote of a
    /// substitution with the `x` flag.
    /// The flags start with a word character, so the `\b` after the quote
    /// holds.
    pub(super) fn quote_x_end(&self, p: usize) -> Option<usize> {
        (self.at(p) == b'\'' && p < self.lim() && self.x_flags_at(p + 1)).then_some(p + 1)
    }

    /// `\1(?=[acdegilmoprsu]*x[acdegilmoprsu]*)\b`: the closing delimiter of
    /// a substitution with the `x` flag.
    pub(super) fn repeat_x_end(&self, p: usize) -> Option<usize> {
        let end = self.repeat(p)?;
        (self.x_flags_at(end) && self.boundary(end)).then_some(end)
    }

    /// `(?<=[\&({|~]|if|unless|^)\s*((/))`: a `/` that opens a regular
    /// expression, after an operator, `if`, `unless` or the line's start.
    pub(super) fn slash_open(&mut self, p: usize) -> Option<usize> {
        let after = p == 0
            || matches!(self.prev(p), Some(b'&' | b'(' | b'{' | b'|' | b'~'))
            || (p >= 2 && self.starts(p - 2, b"if"))
            || (p >= 6 && self.starts(p - 6, b"unless"));
        if !after {
            return None;
        }
        let at = self.space_end(p);
        (self.at(at) == b'/' && at < self.lim()).then(|| {
            self.capture(1, at, at + 1);
            self.capture(2, at, at + 1);
            at + 1
        })
    }

    /// `((\1([acdegil-prsux]*)))(?=(\s+\S|\s*[#),;{}]|\s*$))`: the closing
    /// `/` of a regular expression, its flags and the lookahead after them.
    pub(super) fn slash_end(&mut self, p: usize) -> Option<usize> {
        let close = self.repeat(p)?;
        let flags = self.skip(close, is_flag);
        self.flags_close(flags, FlagsEnd::Hash).then(|| {
            self.capture(1, p, flags);
            self.capture(2, p, flags);
            self.capture(3, close, flags);
            flags
        })
    }

    /// `\b(tr|y)\s*([^0-9A-Za-z\s])(.*?)(?<!\\)(\\{2})*(\2)(.*?)(?<!\\)(\\{2})*
    /// (\2)`: a transliteration on one line.
    ///
    /// Each `(.*?)` stops at the first delimiter that an even run of
    /// backslashes precedes, the run itself not after a backslash.
    /// A later part that finds no delimiter makes the earlier part go on to
    /// its next one.
    pub(super) fn transliteration(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let name = if self.starts(p, b"tr") {
            p + 2
        } else if self.at(p) == b'y' && p < self.lim() {
            p + 1
        } else {
            return None;
        };
        let at = self.space_end(name);
        let open = self.delim_at(at, Delim::NotAlnum)?;
        let delim = self.text(at, open);
        let mut from = open;
        while let Some(middle) = self.unescaped(from, delim) {
            let after = middle + delim.len();
            if let Some(close) = self.unescaped(after, delim) {
                self.capture(1, p, name);
                self.capture(2, at, open);
                self.capture(5, middle, after);
                self.capture(8, close, close + delim.len());
                return Some(close + delim.len());
            }
            from = self.char_end(middle);
        }
        None
    }

    /// `(.*?)(?<!\\)(\\{2})*(\2)` from `from`: where the first delimiter
    /// starts that an even run of backslashes precedes, the run not after a
    /// backslash.
    /// The delimiter starts past the backslashes.
    fn unescaped(&self, from: usize, delim: &[u8]) -> Option<usize> {
        let lim = self.lim();
        let mut at = from;
        while at <= lim {
            if self.prev(at) != Some(b'\\') {
                let run = self.skip(at, |b| b == b'\\') - at;
                let mut pairs = run / 2;
                loop {
                    let close = at + 2 * pairs;
                    if self.starts(close, delim) {
                        self.spend(at - from);
                        return Some(close);
                    }
                    if pairs == 0 {
                        break;
                    }
                    pairs -= 1;
                }
            }
            if at == lim {
                break;
            }
            at = self.char_end(at);
        }
        self.spend(at - from);
        None
    }
}
