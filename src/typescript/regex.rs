//! Regular expression literals: where one may start, and its body.

use super::bytes::Bytes;
use super::chars::Before;
use super::kind::Kind;
use super::lexer::Lexer;
use super::scope::S;

/// Which of the grammar's two regular-expression rules a lookahead follows.
#[derive(Clone, Copy, Debug)]
pub(crate) enum RegexForm {
    /// The form after an operator, whose lookahead spells character classes
    /// and groups out in full.
    Strict,
    /// The other forms, whose lookahead reads them more loosely.
    Loose,
}

/// Which end of a character-class range an item is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RangeItem {
    /// The item before `-`.
    Low,
    /// The item after `-`.
    High,
}

impl<const D: u8> Lexer<'_, D> {
    pub(crate) fn regex(&mut self, p: usize) -> Option<usize> {
        if self.full() {
            return None;
        }
        let q = self.regex_start(p)?;
        self.flush(p);
        self.capture(
            Kind::Regex,
            q,
            q + 1,
            &[S::PunctuationDefinitionStringBegin],
        );
        self.push(Kind::Regex);
        Some(q + 1)
    }

    /// The `/` that opens a regular expression whose match starts at `p`.
    ///
    /// The first form: after an operator, with blanks.
    ///
    /// The second form: not after a name or a closing bracket.
    fn regex_start(&self, p: usize) -> Option<usize> {
        let q = self.ws(p);
        if self.byte(q) == b'/'
            && !matches!(self.byte(q + 1), b'*' | b'/')
            && self.regex_before_strict(p)
            && self.regex_body(q, RegexForm::Strict)
        {
            return Some(q);
        }
        if self.byte(p) == b'/'
            && !matches!(self.byte(p + 1), b'*' | b'/')
            && self.regex_before_loose(p)
            && self.regex_body(p, RegexForm::Loose)
        {
            return Some(p);
        }
        if (self.word_before_text(p, "return") || self.word_before_text(p, "case"))
            && self.byte(q) == b'/'
            && !matches!(self.byte(q + 1), b'*' | b'/')
            && self.regex_body(q, RegexForm::Loose)
        {
            return Some(q);
        }
        None
    }

    /// `(?<!\+\+|--|})(?<=[!(+,:=?\[]|^return|…return|^case|…case|=>|&&|\|\||\*
    /// /)`.
    fn regex_before_strict(&self, p: usize) -> bool {
        const STRICT: Before = Before {
            bytes: Bytes::of(b"!(+,:=?["),
            operators: &["=>", "&&", "||", "*/"],
            words: &["return", "case"],
            never: &["++", "--", "}"],
        };
        self.operator_before(p, &STRICT)
    }

    /// `(?<![]$)_[:alnum:]]|\+\+|--|}|\*/)`: not after a name, a closing
    /// bracket, an increment, a brace or a comment.
    fn regex_before_loose(&self, p: usize) -> bool {
        let Some(head) = self.bytes.get(..p) else {
            return true;
        };
        if head.ends_with(b"++")
            || head.ends_with(b"--")
            || head.ends_with(b"}")
            || head.ends_with(b"*/")
        {
            return false;
        }
        !self
            .prev_char(p)
            .is_some_and(|c| c == ']' || c == ')' || super::chars::is_alnum_char(c))
    }

    /// The lookahead of a regular expression that starts at the `/` at `q`:
    /// a body, a closing `/`, and flags that no name follows.
    #[expect(
        clippy::many_single_char_names,
        reason = "positions are named by role: `p` where the rule starts, `e` where a part ends, `j` and `k` for the cursor"
    )]
    pub(crate) fn regex_body(&self, q: usize, form: RegexForm) -> bool {
        let strict = matches!(form, RegexForm::Strict);
        let end = self.window();
        let mut j = q + 1;
        let mut items = 0;
        loop {
            if j >= end {
                return false;
            }
            match self.byte(j) {
                b'/' => break,
                b'\\' => j += 1 + self.char_len(j + 1),
                b'[' => {
                    let mut k = j + 1;
                    let mut n = 0;
                    if strict {
                        loop {
                            if k >= end {
                                return false;
                            }
                            match self.byte(k) {
                                b']' => break,
                                b'\\' => k += 1 + self.char_len(k + 1),
                                _ => k += self.char_len(k),
                            }
                            n += 1;
                        }
                        if n == 0 {
                            return false;
                        }
                    } else {
                        loop {
                            if k >= end {
                                return false;
                            }
                            match self.byte(k) {
                                b']' => break,
                                b'\\' => k += 1 + self.char_len(k + 1),
                                _ => k += self.char_len(k),
                            }
                        }
                    }
                    j = k + 1;
                }
                b'(' if strict => {
                    let mut k = j + 1;
                    let mut n = 0;
                    loop {
                        if k >= end {
                            return false;
                        }
                        match self.byte(k) {
                            b')' => break,
                            b'\\' => k += 1 + self.char_len(k + 1),
                            _ => k += self.char_len(k),
                        }
                        n += 1;
                    }
                    if n == 0 {
                        return false;
                    }
                    j = k + 1;
                }
                b'(' | b')' if strict => return false,
                _ => j += self.char_len(j),
            }
            items += 1;
        }
        if items == 0 {
            return false;
        }
        let f = j + 1;
        let g = self.regex_flags_end(f);
        let name_follows = |i: usize| {
            let k = self.ws(i);
            let c = self.byte(k);
            c == b'$' || c == b'_' || c.is_ascii_alphanumeric()
        };
        (g > f && !name_follows(g))
            || (!matches!(self.byte(f), b'*' | b'/') && !name_follows(f))
            || (self.starts(f, "/*") && !name_follows(f))
    }

    /// `[dgimsuvy]*` from `i`: the end of a regular expression's flags.
    pub(crate) fn regex_flags_end(&self, i: usize) -> usize {
        let mut e = i;
        while matches!(
            self.byte(e),
            b'd' | b'g' | b'i' | b'm' | b's' | b'u' | b'v' | b'y'
        ) {
            e += 1;
        }
        e
    }

    /// Inside a regular expression.
    pub(crate) fn regexp(&mut self, p: usize) -> Option<usize> {
        let b = self.byte(p);
        let b1 = self.byte(p + 1);
        if b == b'\\' {
            if matches!(b1, b'B' | b'b') {
                return self.token_rule(p, p + 2, S::KeywordControlAnchorRegexp);
            }
            if (b'1'..=b'9').contains(&b1) {
                let mut e = p + 2;
                while self.byte(e).is_ascii_digit() {
                    e += 1;
                }
                return self.token_rule(p, e, S::KeywordOtherBackReferenceRegexp);
            }
            if b1 == b'k'
                && self.byte(p + 2) == b'<'
                && let Some(n) = self.ident(p + 3)
                && self.byte(n) == b'>'
            {
                self.token(p, p + 3, &[S::KeywordOtherBackReferenceRegexp]);
                self.capture(
                    Kind::Top,
                    p + 3,
                    n,
                    &[S::KeywordOtherBackReferenceRegexp, S::VariableOtherRegexp],
                );
                self.capture(Kind::Top, n, n + 1, &[S::KeywordOtherBackReferenceRegexp]);
                return Some(n + 1);
            }
        }
        if b == b'$' || b == b'^' {
            return self.token_rule(p, p + 1, S::KeywordControlAnchorRegexp);
        }
        if matches!(b, b'*' | b'+' | b'?') {
            return self.token_rule(p, p + 1, S::KeywordOperatorQuantifierRegexp);
        }
        if b == b'{'
            && let Some(e) = self.regex_quantifier(p)
        {
            return self.token_rule(p, e, S::KeywordOperatorQuantifierRegexp);
        }
        if b == b'|' {
            return self.token_rule(p, p + 1, S::KeywordOperatorOrRegexp);
        }
        if b == b'(' {
            return self.regex_group(p);
        }
        if b == b'[' && !self.full() {
            let neg = b1 == b'^';
            self.flush(p);
            self.capture(
                Kind::RxSet,
                p,
                p + 1,
                &[S::PunctuationDefinitionCharacterClassRegexp],
            );
            let mut e = p + 1;
            if neg {
                self.capture(
                    Kind::RxSet,
                    p + 1,
                    p + 2,
                    &[S::KeywordOperatorNegationRegexp],
                );
                e = p + 2;
            }
            self.push(Kind::RxSet);
            return Some(e);
        }
        self.regex_character_class(p)
    }

    /// `\{(\d+,\d+|\d+,|,\d+|\d+)}\??`.
    fn regex_quantifier(&self, p: usize) -> Option<usize> {
        let mut j = p + 1;
        let d1 = j;
        while self.byte(j).is_ascii_digit() {
            j += 1;
        }
        let has1 = j > d1;
        let mut has2 = false;
        let comma = self.byte(j) == b',';
        if comma {
            j += 1;
            let d2 = j;
            while self.byte(j).is_ascii_digit() {
                j += 1;
            }
            has2 = j > d2;
        }
        if !(has1 || (comma && has2)) || self.byte(j) != b'}' {
            return None;
        }
        j += 1;
        if self.byte(j) == b'?' {
            j += 1;
        }
        Some(j)
    }

    fn regex_group(&mut self, p: usize) -> Option<usize> {
        if self.full() {
            return None;
        }
        self.flush(p);
        let b1 = self.byte(p + 1);
        let b2 = self.byte(p + 2);
        if b1 == b'?' {
            let assertion = match (b2, self.byte(p + 3)) {
                (b'=', _) => Some((3, S::MetaAssertionLookAheadRegexp)),
                (b'!', _) => Some((3, S::MetaAssertionNegativeLookAheadRegexp)),
                (b'<', b'=') => Some((4, S::MetaAssertionLookBehindRegexp)),
                (b'<', b'!') => Some((4, S::MetaAssertionNegativeLookBehindRegexp)),
                _ => None,
            };
            if let Some((len, scope)) = assertion {
                self.capture(
                    Kind::RxAssert,
                    p,
                    p + 1,
                    &[S::PunctuationDefinitionGroupRegexp],
                );
                self.capture(
                    Kind::RxAssert,
                    p + 1,
                    p + len,
                    &[S::PunctuationDefinitionGroupAssertionRegexp, scope],
                );
                self.push(Kind::RxAssert);
                return Some(p + len);
            }
            if b2 == b':' {
                self.capture(
                    Kind::RxGroup,
                    p,
                    p + 1,
                    &[S::PunctuationDefinitionGroupRegexp],
                );
                self.capture(
                    Kind::RxGroup,
                    p + 1,
                    p + 3,
                    &[
                        S::PunctuationDefinitionGroupRegexp,
                        S::PunctuationDefinitionGroupNoCaptureRegexp,
                    ],
                );
                self.push(Kind::RxGroup);
                return Some(p + 3);
            }
            if b2 == b'<'
                && let Some(n) = self.ident(p + 3)
                && self.byte(n) == b'>'
            {
                self.capture(
                    Kind::RxGroup,
                    p,
                    p + 1,
                    &[S::PunctuationDefinitionGroupRegexp],
                );
                self.capture(
                    Kind::RxGroup,
                    p + 1,
                    p + 3,
                    &[S::PunctuationDefinitionGroupRegexp],
                );
                self.capture(
                    Kind::RxGroup,
                    p + 3,
                    n,
                    &[S::PunctuationDefinitionGroupRegexp, S::VariableOtherRegexp],
                );
                self.capture(
                    Kind::RxGroup,
                    n,
                    n + 1,
                    &[S::PunctuationDefinitionGroupRegexp],
                );
                self.push(Kind::RxGroup);
                return Some(n + 1);
            }
        }
        self.capture(
            Kind::RxGroup,
            p,
            p + 1,
            &[S::PunctuationDefinitionGroupRegexp],
        );
        self.push(Kind::RxGroup);
        Some(p + 1)
    }

    /// `\\[DSWdfnrstvw]|\.`, numeric and control escapes, and any other escape.
    pub(crate) fn regex_character_class(&mut self, p: usize) -> Option<usize> {
        let b = self.byte(p);
        if b == b'.' {
            return self.token_rule(p, p + 1, S::ConstantOtherCharacterClassRegexp);
        }
        if b != b'\\' {
            return None;
        }
        let c = self.byte(p + 1);
        if matches!(
            c,
            b'D' | b'S' | b'W' | b'd' | b'f' | b'n' | b'r' | b's' | b't' | b'v' | b'w'
        ) {
            return self.token_rule(p, p + 2, S::ConstantOtherCharacterClassRegexp);
        }
        if let Some(e) = self.regex_numeric_escape(p) {
            return self.token_rule(p, e, S::ConstantCharacterNumericRegexp);
        }
        if c == b'c' && self.byte(p + 2).is_ascii_uppercase() {
            return self.token_rule(p, p + 3, S::ConstantCharacterControlRegexp);
        }
        let e = p + 1 + self.char_len(p + 1);
        self.token_rule(p, e, S::ConstantCharacterEscapeBackslashRegexp)
    }

    /// `\\([0-7]{3}|x\h{2}|u\h{4})`.
    fn regex_numeric_escape(&self, p: usize) -> Option<usize> {
        let c = self.byte(p + 1);
        let n = |from: usize, count: usize, ok: fn(u8) -> bool| {
            (0..count).all(|k| ok(self.byte(from + k)))
        };
        if n(p + 1, 3, |b| (b'0'..=b'7').contains(&b))
            || (c == b'x' && n(p + 2, 2, |b| b.is_ascii_hexdigit()))
        {
            Some(p + 4)
        } else if c == b'u' && n(p + 2, 4, |b| b.is_ascii_hexdigit()) {
            Some(p + 6)
        } else {
            None
        }
    }

    /// Inside a character class: a range, or a class item.
    pub(crate) fn regex_set_rules(&mut self, p: usize) -> Option<usize> {
        if let Some(e) = self.regex_range(p) {
            return Some(e);
        }
        self.regex_character_class(p)
    }

    /// The ways the grammar's range rule reads an item at `i`.
    /// They come in the order its alternatives are written:
    ///
    /// 1. a plain character: any, for the first item; not `]` or a backslash,
    ///    for the second;
    /// 2. a numeric escape;
    /// 3. a control escape;
    /// 4. any other escape.
    fn range_items(&self, i: usize, item: RangeItem) -> [Option<(usize, Option<S>)>; 4] {
        let any = item == RangeItem::Low;
        let plain = (i < self.len && (any || !matches!(self.byte(i), b']' | b'\\')))
            .then(|| (i + self.char_len(i), None));
        let numeric = self
            .regex_numeric_escape(i)
            .map(|e| (e, Some(S::ConstantCharacterNumericRegexp)));
        let control = (self.byte(i) == b'\\'
            && self.byte(i + 1) == b'c'
            && self.byte(i + 2).is_ascii_uppercase())
        .then_some((i + 3, Some(S::ConstantCharacterControlRegexp)));
        let escape = (self.byte(i) == b'\\' && i + 1 < self.len).then(|| {
            (
                i + 1 + self.char_len(i + 1),
                Some(S::ConstantCharacterEscapeBackslashRegexp),
            )
        });
        [plain, numeric, control, escape]
    }

    /// `ITEM-ITEM` inside a character class.
    fn regex_range(&mut self, p: usize) -> Option<usize> {
        let (a_end, a_scope, b_end, b_scope) = self
            .range_items(p, RangeItem::Low)
            .into_iter()
            .flatten()
            .filter(|&(a_end, _)| self.byte(a_end) == b'-')
            .find_map(|(a_end, a_scope)| {
                self.range_items(a_end + 1, RangeItem::High)
                    .into_iter()
                    .flatten()
                    .next()
                    .map(|(b_end, b_scope)| (a_end, a_scope, b_end, b_scope))
            })?;
        self.flush(p);
        let range = S::ConstantOtherCharacterClassRangeRegexp;
        match a_scope {
            Some(s) => self.capture(Kind::Top, p, a_end, &[range, s]),
            None => self.capture(Kind::Top, p, a_end, &[range]),
        }
        self.capture(Kind::Top, a_end, a_end + 1, &[range]);
        match b_scope {
            Some(s) => self.capture(Kind::Top, a_end + 1, b_end, &[range, s]),
            None => self.capture(Kind::Top, a_end + 1, b_end, &[range]),
        }
        Some(b_end)
    }
}
