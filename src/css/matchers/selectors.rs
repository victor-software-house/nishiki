//! Selectors: the rule that finds them, their parts, attribute selectors and
//! pseudo-classes.

use super::super::ahead::Find;
use super::super::engine::CssLike;
use super::super::words::List;
use super::chars::{Letters, Words};
use crate::rules::{Chars, Lexer};

/// The characters that end a name in a selector: `[#\&)+,.:>\[{|~\s]`.
fn selector_stop(b: u8) -> bool {
    matches!(
        b,
        b'#' | b'&' | b')' | b'+' | b',' | b'.' | b':' | b'>' | b'[' | b'{' | b'|' | b'~'
    )
}

/// How [`Lexer::vendor_prefixed`] reads the tail after the prefix.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::css) enum Tail {
    /// ASCII letters and `-`.
    Ascii,
    /// Letters under `(?i)`, and `-`.
    /// The long s and the Kelvin sign count as letters.
    Folded,
}

#[cfg_attr(
    nishiki_unstable,
    expect(private_bounds, reason = "the bound and the methods are crate-private")
)]
impl<G: CssLike> Lexer<'_, G> {
    /// The lookahead `(?=$|[#\&)+,.:>\[{|~\s]|/\*)` at `p`.
    fn selector_stop_ahead(&self, p: usize) -> bool {
        self.at_eol(p)
            || self.s.get(p).is_some_and(|&b| selector_stop(b))
            || self.space_at(p) > 0
            || self.starts(p, b"/*")
    }

    /// `\\(?:\h{1,6}|.)` that ends at `p`.
    pub(in crate::css) fn escape_before(&self, p: usize) -> bool {
        if p > self.lim() {
            return false;
        }
        for digits in 1..=6 {
            if p > digits
                && self.s[p - digits - 1] == b'\\'
                && self.s[p - digits..p].iter().all(u8::is_ascii_hexdigit)
            {
                return true;
            }
        }
        match self.char_before(p) {
            Some(c) if p > c.len_utf8() => self.s[p - c.len_utf8() - 1] == b'\\',
            _ => false,
        }
    }

    /// `/deep/|>>>`
    pub(in crate::css) fn combinator_invalid(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b"/deep/").or_else(|| self.lit(p, b">>>"))
    }

    /// The lookbehind `(?<=^|(^|[^:])\s|[;{}]|\*/|\\(?:\h{1,6}|.))`.
    fn selector_behind(&self, p: usize) -> bool {
        if p == 0 {
            return true;
        }
        let blank_after_non_colon = self.space_before(p)
            && self.char_before(p).is_some_and(|c| {
                let start = p - c.len_utf8();
                start == 0 || self.char_before(start) != Some(':')
            });
        blank_after_non_colon
            || matches!(self.s.get(p - 1), Some(b';' | b'{' | b'}'))
            || (p >= 2 && self.s.get(p - 2..p) == Some(b"*/"))
            || self.escape_before(p)
    }

    /// The lookahead of a selector's start:
    /// `[\&*+>A-\[a-z|~[^\x00-\x7F]]|\\(?:\h{1,6}|.)|[#.](?:[-\w]|\\(?:\h{1,
    /// 6}|.))|::??[-A-Za-z]`.
    fn selector_start_ahead(&self, p: usize) -> bool {
        let Some(&b) = self.s.get(p) else {
            return false;
        };
        if matches!(b, b'&' | b'*' | b'+' | b'>' | b'[' | b'|' | b'~')
            || b.is_ascii_alphabetic()
            || b >= 0x80
        {
            return true;
        }
        if b == b'\\' {
            return self.escape_end(p).is_some();
        }
        if b == b'#' || b == b'.' {
            return self.wordish_at(p + 1) && self.at(p + 1) != 0
                || self.escape_end(p + 1).is_some();
        }
        if b == b':' {
            let next = if self.at(p + 1) == b':' { p + 2 } else { p + 1 };
            return self.letters_ci(next, Letters::Dashed) > next;
        }
        false
    }

    /// The negative lookahead `(?![-\w]*:+\s|[^{]*;|[^{]*}|[^:]+(::):+)` at
    /// `p`: whether none of its alternatives matches.
    ///
    /// It checks, in order:
    /// - `[-\w]*:+\s`;
    /// - `[^{]*;` and `[^{]*}`, a `;` or `}` before any `{`;
    /// - `[^:]+(::):+`.
    fn selector_not_property(&self, p: usize) -> bool {
        let word_end = self.wordish_run(p);
        let colons = self.skip(word_end, |b| b == b':');
        if colons > word_end && self.space_at(colons) > 0 {
            return false;
        }
        let brace = self.find(Find::Open, p).unwrap_or(self.lim());
        let terminator = |found: Option<usize>| found.is_some_and(|at| at < brace);
        if terminator(self.find(Find::Semicolon, p)) || terminator(self.find(Find::Close, p)) {
            return false;
        }
        let plain = self.find(Find::Colon, p).unwrap_or(self.lim());
        if plain > p && self.starts(plain, b":::") {
            return false;
        }
        true
    }

    /// The begin of a selector, which is where one can start.
    pub(in crate::css) fn selector(&mut self, p: usize) -> Option<usize> {
        (self.selector_behind(p) && self.selector_start_ahead(p) && self.selector_not_property(p))
            .then_some(p)
    }

    /// `(?=\s*(?:[)/@{]|$))`
    pub(in crate::css) fn selector_end(&mut self, p: usize) -> Option<usize> {
        let after = self.spaces(p);
        (after > self.lim() || matches!(self.at(after), b')' | b'/' | b'@' | b'{')).then_some(p)
    }

    /// `,`
    pub(in crate::css) fn commas(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b",")
    }

    /// `>>|[+>~]`
    pub(in crate::css) fn combinators_at1(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b">>")
            .or_else(|| matches!(self.s.get(p), Some(b'+' | b'>' | b'~')).then_some(p + 1))
    }

    /// The namespace prefix
    /// `(?:^|(?<=[(,;}\s]))(?![-*\w]+\|(?![-#\&*.:A-\[_a-z[^\x00-\
    /// x7F]]))([-A-Z_a-z[^\x00-\x7F]](?:[-0-9A-Z_a-z[^\x00-\x7F]]|\\(?:\h{1,
    /// 6}|.))*|\*)?(\|)`.
    ///
    /// It checks the negative lookahead
    /// `(?![-*\w]+\|(?![-#\&*.:A-\[_a-z[^\x00-\x7F]]))` first.
    pub(in crate::css) fn selector_innards_at4(&mut self, p: usize) -> Option<usize> {
        let behind = p == 0
            || matches!(self.s.get(p - 1), Some(b'(' | b',' | b';' | b'}'))
            || self.space_before(p);
        if !behind {
            return None;
        }
        let mut run = p;
        while run < self.lim() && (self.s[run] == b'*' || self.wordish_at(run)) {
            run = self.char_end(run);
        }
        if run > p && self.s.get(run) == Some(&b'|') {
            let after = self.s.get(run + 1).copied();
            let selector_char = after.is_some_and(|b| {
                matches!(b, b'-' | b'#' | b'&' | b'*' | b'.' | b':' | b'_' | b'[')
                    || b.is_ascii_alphabetic()
                    || b >= 0x80
            });
            if !selector_char {
                return None;
            }
        }
        let prefix_end = self
            .name_end(p)
            .or_else(|| (self.s.get(p) == Some(&b'*')).then_some(p + 1));
        let bar = match prefix_end {
            Some(end) if self.s.get(end) == Some(&b'|') => {
                self.capture(1, p, end);
                end
            }
            _ if self.s.get(p) == Some(&b'|') => p,
            _ => return None,
        };
        self.capture(2, bar, bar + 1);
        Some(bar + 1)
    }

    /// The tag names of the grammar's `tag-names`:
    /// `(?i)(?<![-:\w])(?:a|abbr|...)(?=[#\&)+,.:>\[{|~\s]|/\*|$)`.
    pub(in crate::css) fn tag_names(&mut self, p: usize) -> Option<usize> {
        if self.wordish_before(p) || self.char_before(p) == Some(':') {
            return None;
        }
        let end = self.wordish_run(p);
        (end > p
            && self.in_list(p, end, Words::Table(List::TagNames))
            && self.selector_stop_ahead(end))
        .then_some(end)
    }

    /// `&`
    pub(in crate::css) fn selector_innards_at6(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b"&")
    }

    /// `\*`
    pub(in crate::css) fn selector_innards_at7(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b"*")
    }

    /// The bad identifier: `(?<![-@\w])([#.])`, then one of:
    /// - a digit;
    /// - a lone `-`;
    /// - a run of identifier characters and escapes that holds a bad character.
    ///   A bad character is one of `]!"%'(*;<?@^|}`, a backtick, `/` not before
    ///   `*`, or `&` before an identifier character.
    ///
    /// After `#` or `.`, the arms match `-?[0-9]`, then
    /// `(?:identchar|escape)*(?:bad)+`.
    pub(in crate::css) fn selector_innards_at8(&mut self, p: usize) -> Option<usize> {
        if self.wordish_before(p) || self.char_before(p) == Some('@') {
            return None;
        }
        if !matches!(self.s.get(p), Some(b'#' | b'.')) {
            return None;
        }
        let body = p + 1;
        let first = if self.at(body) == b'-' && self.at(body + 1).is_ascii_digit() {
            Some(body + 2)
        } else if self.at(body).is_ascii_digit() && body < self.lim()
            || self.at(body) == b'-' && body < self.lim() && self.selector_stop_ahead(body + 1)
        {
            Some(body + 1)
        } else {
            let run = self.ident_run(body);
            let mut end = run;
            loop {
                let b = self.at(end);
                let bad = end < self.lim()
                    && (matches!(
                        b,
                        b']' | b'!'
                            | b'"'
                            | b'%'
                            | b'\''
                            | b'('
                            | b'*'
                            | b';'
                            | b'<'
                            | b'?'
                            | b'@'
                            | b'^'
                            | b'`'
                            | b'|'
                            | b'}'
                    ) || (b == b'/' && self.at(end + 1) != b'*')
                        || (b == b'&' && self.ident_char(end + 1).is_some()));
                if !bad {
                    break;
                }
                end += 1;
            }
            (end > run).then_some(end)
        }?;
        let end = self.ident_run(first);
        self.capture(1, p, p + 1);
        self.capture(2, body, end);
        Some(end)
    }

    /// The class `(\.)((?:identchar|escape)+)(?=$|[#\&)+,.:>\[{|~\s]|/\*)`.
    pub(in crate::css) fn selector_innards_at9(&mut self, p: usize) -> Option<usize> {
        if self.s.get(p) != Some(&b'.') {
            return None;
        }
        let end = self.ident_run_before_stop(p + 1)?;
        self.capture(1, p, p + 1);
        self.capture(2, p + 1, end);
        Some(end)
    }

    /// The id `(#)(-?(?![0-9])(?:identchar|escape)+)(?=$|[#\&)+,.:>\[{|~\s]|/\*
    /// )`.
    pub(in crate::css) fn selector_innards_at10(&mut self, p: usize) -> Option<usize> {
        if self.s.get(p) != Some(&b'#') || self.at(p + 1).is_ascii_digit() {
            return None;
        }
        let end = self.ident_run_before_stop(p + 1)?;
        self.capture(1, p, p + 1);
        self.capture(2, p + 1, end);
        Some(end)
    }

    /// The end of `(?:identchar|escape)+(?=$|[#\&)+,.:>\[{|~\s]|/\*)` at `p`.
    /// The run gives back characters until a stop follows.
    /// Only a white space character that is also an identifier character can
    /// be that stop.
    fn ident_run_before_stop(&self, p: usize) -> Option<usize> {
        let mut i = p;
        let mut best = None;
        while let Some(end) = self.ident_char(i).or_else(|| self.escape_end(i)) {
            i = end;
            if self.at(i) >= 0x80 && self.selector_stop_ahead(i) {
                best = Some(i);
            }
        }
        self.spend(i - p + 1);
        if self.selector_stop_ahead(i) {
            best = Some(i);
        }
        best.filter(|&end| end > p)
    }

    /// `\[`
    pub(in crate::css) fn selector_innards_at11(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b"[")
    }

    /// `]`
    pub(in crate::css) fn selector_innards_at11_end(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b"]")
    }

    /// The case flag `(?<=["'\s]|^|\*/)\s*([Ii])\s*(?=[]\s]|/\*|$)`.
    pub(in crate::css) fn selector_innards_at11_at2(&mut self, p: usize) -> Option<usize> {
        let behind = p == 0
            || matches!(self.s.get(p - 1), Some(b'"' | b'\''))
            || self.space_before(p)
            || (p >= 2 && self.s.get(p - 2..p) == Some(b"*/"));
        if !behind {
            return None;
        }
        let flag = self.spaces(p);
        if !matches!(self.at(flag), b'I' | b'i') || flag >= self.lim() {
            return None;
        }
        let after = self.spaces(flag + 1);
        let ahead = |at: usize| {
            self.at(at) == b']' && at < self.lim() || self.starts(at, b"/*") || self.at_eol(at)
        };
        let end = if ahead(after) {
            after
        } else if after > flag + 1 {
            after - self.space_width_back(after)
        } else {
            return None;
        };
        self.capture(1, flag, flag + 1);
        Some(end)
    }

    /// The width of the white space character that ends at `p`.
    fn space_width_back(&self, p: usize) -> usize {
        if p == self.lim() + 1 {
            1
        } else {
            self.char_before(p).map_or(1, char::len_utf8)
        }
    }

    /// The unquoted value `(?<==)\s*((?!/\*)(?:[^]"'\\\s]|\\.)+)`.
    pub(in crate::css) fn selector_innards_at11_at3(&mut self, p: usize) -> Option<usize> {
        if p == 0 || self.s.get(p - 1) != Some(&b'=') {
            return None;
        }
        let from = self.spaces(p);
        if self.starts(from, b"/*") {
            return None;
        }
        let mut end = from;
        while end < self.lim() {
            let b = self.s[end];
            if b == b'\\' {
                if end + 1 >= self.lim() {
                    break;
                }
                end = self.char_end(end + 1);
            } else if matches!(b, b']' | b'"' | b'\'') || self.space_at(end) > 0 {
                break;
            } else {
                end = self.char_end(end);
            }
        }
        (end > from).then(|| {
            self.capture(1, from, end);
            end
        })
    }

    /// `[$*^|~]?=`
    pub(in crate::css) fn selector_innards_at11_at5(&mut self, p: usize) -> Option<usize> {
        if matches!(self.s.get(p), Some(b'$' | b'*' | b'^' | b'|' | b'~')) && self.at(p + 1) == b'='
        {
            return Some(p + 2);
        }
        self.lit(p, b"=")
    }

    /// `\|`
    pub(in crate::css) fn selector_innards_at11_at6(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b"|")
    }

    /// The attribute's namespace
    /// `(-?(?!\d)(?:[-\w[^\x00-\x7F]]|\\(?:\h{1,6}|.))+|\*)(?=\|(?![=\s]|$|])(?
    /// :-?(?!\d)|[-\\\w[^\x00-\x7F]]))`.
    pub(in crate::css) fn selector_innards_at11_at7(&mut self, p: usize) -> Option<usize> {
        let end = if self.at(p) == b'*' && p < self.lim() {
            p + 1
        } else if self.digit_at(p).is_none() {
            let end = self.ident_run(p);
            if end == p {
                return None;
            }
            end
        } else {
            return None;
        };
        let bar = self.s.get(end) == Some(&b'|');
        let after = end + 1;
        let ok = bar
            && after < self.lim()
            && !matches!(self.s[after], b'=' | b']')
            && self.space_at(after) == 0;
        ok.then(|| {
            self.capture(1, p, end);
            end
        })
    }

    /// The attribute's name
    /// `(-?(?!\d)(?>[-\w[^\x00-\x7F]]|\\(?:\h{1,6}|.))+)\s*(?=[]$*=^|~]|/\*)`.
    pub(in crate::css) fn selector_innards_at11_at8(&mut self, p: usize) -> Option<usize> {
        if self.digit_at(p).is_some() {
            return None;
        }
        let name = self.ident_run(p);
        if name == p {
            return None;
        }
        let end = self.spaces(name);
        let ahead = matches!(self.at(end), b']' | b'$' | b'*' | b'=' | b'^' | b'|' | b'~')
            && end < self.lim()
            || self.starts(end, b"/*");
        ahead.then(|| {
            self.capture(1, p, name);
            end
        })
    }

    /// The custom element
    /// `(?<![-@\w])(?=[a-z]\w*-)(?:(?![A-Z])[-\w])+(?![-(\w])`.
    pub(in crate::css) fn selector_innards_at15(&mut self, p: usize) -> Option<usize> {
        if self.wordish_before(p) || self.char_before(p) == Some('@') {
            return None;
        }
        if !self.s.get(p).is_some_and(u8::is_ascii_lowercase) {
            return None;
        }
        let word = self.word_run(p + 1);
        if self.at(word) != b'-' || word >= self.lim() {
            return None;
        }
        let mut end = p;
        while end < self.lim() && self.wordish_at(end) && !self.s[end].is_ascii_uppercase() {
            end = self.char_end(end);
        }
        let next_ok = !(self.wordish_at(end) || self.at(end) == b'(' && end < self.lim());
        next_ok.then_some(end)
    }

    /// A pseudo-class `(?i)(:)(:*)(?:active|...)(?![-\w]|\s*[;}])`.
    pub(in crate::css) fn pseudo_classes(&mut self, p: usize) -> Option<usize> {
        if self.s.get(p) != Some(&b':') {
            return None;
        }
        let word = self.skip(p + 1, |b| b == b':');
        let end = self.wordish_run(word);
        let after = self.spaces(end);
        if end == word
            || !self.in_list(word, end, Words::Table(List::PseudoClasses))
            || matches!(self.at(after), b';' | b'}') && after < self.lim()
        {
            return None;
        }
        self.capture(1, p, p + 1);
        if word > p + 1 {
            self.capture(2, p + 1, word);
        }
        Some(end)
    }

    /// A pseudo-element
    /// `(?i)(?:(::?)(?:after|before|first-letter|first-line|(?:-(?:ah|...)|(?:
    /// mso|prince))-[-a-z]+)|(::)(?:backdrop|...))(?![-\w]|\s*[;}])`.
    pub(in crate::css) fn pseudo_elements(&mut self, p: usize) -> Option<usize> {
        if self.s.get(p) != Some(&b':') {
            return None;
        }
        let double = self.at(p + 1) == b':';
        let colons = if double { 2 } else { 1 };
        let word = p + colons;
        let run = self.wordish_run(word);
        let plain = run > word && self.in_list(word, run, Words::Table(List::PseudoElements));
        let end = if plain {
            run
        } else if let Some(end) = self.vendor_prefixed(word, Tail::Folded) {
            end
        } else if double
            && run > word
            && self.in_list(word, run, Words::Table(List::PseudoElementsDouble))
        {
            let after = self.spaces(run);
            if matches!(self.at(after), b';' | b'}') && after < self.lim() {
                return None;
            }
            self.capture(2, p, p + 2);
            return Some(run);
        } else {
            return None;
        };
        let after = self.spaces(end);
        if self.wordish_at(end) || matches!(self.at(after), b';' | b'}') && after < self.lim() {
            return None;
        }
        self.capture(1, p, p + colons);
        Some(end)
    }

    /// `(?:-(?:ah|apple|...)|(?:mso|prince))-[-a-z]+` at `p`, without regard
    /// to case in the prefix, and in the tail as `tail` says, and its end.
    pub(in crate::css) fn vendor_prefixed(&self, p: usize, tail: Tail) -> Option<usize> {
        let dash = self.at(p) == b'-';
        let start = if dash { p + 1 } else { p };
        let letters = self.letters_ci(start, Letters::Plain);
        let vendor_ok = if dash {
            self.in_list(start, letters, Words::Table(List::Vendors))
        } else {
            self.is_word_ci(start, letters, "mso") || self.is_word_ci(start, letters, "prince")
        };
        if !vendor_ok || self.at(letters) != b'-' {
            return None;
        }
        let end = match tail {
            Tail::Folded => self.letters_ci(letters + 1, Letters::Dashed),
            Tail::Ascii => self.skip(letters + 1, |b| b.is_ascii_alphabetic() || b == b'-'),
        };
        (end > letters + 1).then_some(end)
    }

    /// `(?i)((:)word)(\()`: a functional pseudo-class named by one of `list`.
    fn functional_pseudo(&mut self, p: usize, list: Words) -> Option<usize> {
        self.lit(p, b":")?;
        let name = self.wordish_run(p + 1);
        let ok = name > p + 1
            && self.at(name) == b'('
            && name < self.lim()
            && self.in_list(p + 1, name, list);
        ok.then(|| {
            self.capture(1, p, name);
            self.capture(2, p, p + 1);
            self.capture(3, name, name + 1);
            name + 1
        })
    }

    /// `(?i)((:)dir)(\()`
    pub(in crate::css) fn functional_pseudo_classes_at0(&mut self, p: usize) -> Option<usize> {
        self.functional_pseudo(p, Words::Spelled(&["dir"]))
    }

    /// `\)`
    pub(in crate::css) fn functional_pseudo_classes_at0_end(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b")")
    }

    /// `(?i)(?<![-\w])(ltr|rtl)(?![-\w])`
    pub(in crate::css) fn functional_pseudo_classes_at0_at2(&mut self, p: usize) -> Option<usize> {
        let end = self.standalone_word(p, Words::Spelled(&["ltr", "rtl"]))?;
        self.capture(1, p, end);
        Some(end)
    }

    /// `(?i)((:)lang)(\()`
    pub(in crate::css) fn functional_pseudo_classes_at1(&mut self, p: usize) -> Option<usize> {
        self.functional_pseudo(p, Words::Spelled(&["lang"]))
    }

    /// `(?<=[(,\s])[A-Za-z]+(-[0-9A-Za-z]*|\\(?:\h{1,6}|.))*(?=[),\s])`
    pub(in crate::css) fn functional_pseudo_classes_at1_at0(&mut self, p: usize) -> Option<usize> {
        let behind =
            p > 0 && (matches!(self.s.get(p - 1), Some(b'(' | b',')) || self.space_before(p));
        if !behind {
            return None;
        }
        let letters = self.skip(p, |b| b.is_ascii_alphabetic());
        if letters == p {
            return None;
        }
        let mut end = letters;
        loop {
            if self.at(end) == b'-' && end < self.lim() {
                end = self.skip(end + 1, |b| b.is_ascii_alphanumeric());
            } else if let Some(next) = self.escape_end(end) {
                end = next;
            } else {
                break;
            }
        }
        let ahead = matches!(self.s.get(end), Some(b')' | b',')) || self.space_at(end) > 0;
        ahead.then_some(end)
    }

    /// `"`
    pub(in crate::css) fn functional_pseudo_classes_at1_at1_end(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.lit(p, b"\"")
    }

    /// The language ranges `(?<=["\s])[*A-Za-z]+(-[*0-9A-Za-z]*)*(?=["\s])` in
    /// a string quoted with `quote`.
    fn language_range(&self, p: usize, quote: u8) -> Option<usize> {
        let behind = p > 0 && (self.s.get(p - 1) == Some(&quote) || self.space_before(p));
        if !behind {
            return None;
        }
        let mut end = self.skip(p, |b| b == b'*' || b.is_ascii_alphabetic());
        if end == p {
            return None;
        }
        while self.at(end) == b'-' && end < self.lim() {
            end = self.skip(end + 1, |b| b == b'*' || b.is_ascii_alphanumeric());
        }
        let ahead = self.s.get(end) == Some(&quote) || self.space_at(end) > 0;
        ahead.then_some(end)
    }

    /// The language ranges of a double-quoted string.
    pub(in crate::css) fn functional_pseudo_classes_at1_at1_at1(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.language_range(p, b'"')
    }

    /// `'`
    pub(in crate::css) fn functional_pseudo_classes_at1_at2_end(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.lit(p, b"'")
    }

    /// The language ranges of a single-quoted string.
    pub(in crate::css) fn functional_pseudo_classes_at1_at2_at1(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.language_range(p, b'\'')
    }

    /// `(?i)((:)(?:not|has|matches|where|is))(\()`
    pub(in crate::css) fn functional_pseudo_classes_at2(&mut self, p: usize) -> Option<usize> {
        self.functional_pseudo(p, Words::Spelled(&["has", "is", "matches", "not", "where"]))
    }

    /// `(?i)((:)nth-(?:last-)?(?:child|of-type))(\()`
    pub(in crate::css) fn functional_pseudo_classes_at3(&mut self, p: usize) -> Option<usize> {
        self.functional_pseudo(
            p,
            Words::Spelled(&[
                "nth-child",
                "nth-last-child",
                "nth-last-of-type",
                "nth-of-type",
            ]),
        )
    }

    /// `(?i)[-+]?(\d+n?|n)(\s*[-+]\s*\d+)?`
    pub(in crate::css) fn functional_pseudo_classes_at3_at0(&mut self, p: usize) -> Option<usize> {
        let sign = usize::from(matches!(self.at(p), b'+' | b'-') && p < self.lim());
        let digits = self.digit_end(p + sign);
        let mut end = if digits > p + sign {
            self.lit_ci(digits, "n").unwrap_or(digits)
        } else {
            self.lit_ci(p + sign, "n")?
        };
        let blank = self.spaces(end);
        if matches!(self.at(blank), b'+' | b'-') && blank < self.lim() {
            let number = self.spaces(blank + 1);
            let after = self.digit_end(number);
            if after > number {
                end = after;
            }
        }
        Some(end)
    }

    /// `(?i)even|odd`
    pub(in crate::css) fn functional_pseudo_classes_at3_at1(&mut self, p: usize) -> Option<usize> {
        self.lit_ci(p, "even").or_else(|| self.lit_ci(p, "odd"))
    }
}
