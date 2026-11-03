//! Strings, template literals, regular expressions and comments.

use super::bytes::Bytes;
use super::calls::Before;
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
    // Strings.

    pub(crate) fn string(&mut self, p: usize) -> Option<usize> {
        match self.b(p) {
            b'\'' => self.open1(
                Kind::StrSingle,
                p,
                p + 1,
                &[S::PunctuationDefinitionStringBegin],
            ),
            b'"' => self.open1(
                Kind::StrDouble,
                p,
                p + 1,
                &[S::PunctuationDefinitionStringBegin],
            ),
            _ => self.template(p),
        }
    }

    /// `\\(x\h{2}|u\h{4}|u\{\h+}|[012][0-7]{0,2}|3[0-6][0-7]?|37[0-7]?
    /// |[4-7][0-7]?|.|$)`.
    #[expect(
        clippy::many_single_char_names,
        reason = "positions are named by role: `p` where the rule starts, `e` where a part ends, `j` and `k` for the cursor"
    )]
    pub(crate) fn string_character_escape(&mut self, p: usize) -> Option<usize> {
        if self.b(p) != b'\\' {
            return None;
        }
        let j = p + 1;
        let hex = |i: usize, n: usize| (0..n).all(|k| self.b(i + k).is_ascii_hexdigit());
        let oct = |i: usize| (b'0'..=b'7').contains(&self.b(i));
        let c = self.b(j);
        let e = if c == b'x' && hex(j + 1, 2) {
            j + 3
        } else if c == b'u' && hex(j + 1, 4) {
            j + 5
        } else if c == b'u' && self.b(j + 1) == b'{' && self.b(j + 2).is_ascii_hexdigit() {
            let mut k = j + 2;
            while self.b(k).is_ascii_hexdigit() {
                k += 1;
            }
            if self.b(k) == b'}' { k + 1 } else { j + 1 }
        } else if matches!(c, b'0'..=b'2') {
            let mut k = j + 1;
            let mut n = 0;
            while n < 2 && oct(k) {
                k += 1;
                n += 1;
            }
            k
        } else if c == b'3' && (b'0'..=b'7').contains(&self.b(j + 1)) {
            if oct(j + 2) { j + 3 } else { j + 2 }
        } else if matches!(c, b'4'..=b'7') {
            if oct(j + 1) { j + 2 } else { j + 1 }
        } else if j >= self.len {
            j
        } else {
            j + self.char_len(j)
        };
        self.tok(p, e, S::ConstantCharacterEscape)
    }

    /// A template literal, with or without a tag.
    fn template(&mut self, p: usize) -> Option<usize> {
        if self.tagged_template_ahead(p) && !self.full() {
            return self.open0(Kind::TemplateCall, p);
        }
        self.template_type_arguments(p)
            .or_else(|| self.template_begin(p, Kind::Template))
    }

    /// A template tag whose type arguments follow blanks, or that has no name:
    /// an optional name, blanks, and a `<…>` group before the backtick.
    fn template_type_arguments(&mut self, p: usize) -> Option<usize> {
        if !self.has_backtick_after(p) || self.full() {
            return None;
        }
        let tag = self.ident(p);
        let j = self.ws(tag.unwrap_or(p));
        let end = self.tpb(j).filter(|_| self.b(j) == b'<')?;
        if self.b(self.ws(end)) != b'`' {
            return None;
        }
        self.flush(p);
        if let Some(e) = tag {
            self.cap(Kind::Top, p, e, &[S::EntityNameFunctionTaggedTemplate]);
        }
        self.fin(Kind::TemplateCall2, j);
        self.push(Kind::TemplateCall2);
        Some(j)
    }

    /// The opening of a template literal, with an optional tag name before the
    /// backtick.
    fn template_begin(&mut self, p: usize, kind: Kind) -> Option<usize> {
        if self.full() {
            return None;
        }
        let (tag_end, tick) = match self.ident(p) {
            Some(e) if self.b(e) == b'`' => (Some(e), e),
            _ if self.b(p) == b'`' => (None, p),
            _ => return None,
        };
        self.flush(p);
        if let Some(e) = tag_end {
            self.cap(Kind::Top, p, e, &[S::EntityNameFunctionTaggedTemplate]);
        }
        self.cap(
            Kind::Top,
            tick,
            tick + 1,
            &[
                S::StringTemplate,
                S::PunctuationDefinitionStringTemplateBegin,
            ],
        );
        self.push(kind);
        Some(tick + 1)
    }

    /// A template literal in a type position.
    pub(crate) fn template_type(&mut self, p: usize) -> Option<usize> {
        if self.tagged_template_ahead(p) && !self.full() {
            return self.open0(Kind::TemplateCall, p);
        }
        self.template_type_arguments(p)
            .or_else(|| self.template_begin(p, Kind::TemplateType))
    }

    /// `${` inside a template.
    pub(crate) fn template_substitution(&mut self, p: usize, kind: Kind) -> Option<usize> {
        if self.b(p) != b'$' || self.b(p + 1) != b'{' {
            return None;
        }
        self.open1(
            kind,
            p,
            p + 2,
            &[S::PunctuationDefinitionTemplateExpressionBegin],
        )
    }

    /// The lookahead of a tagged template: a name chain, optionally with type
    /// arguments, then a backtick.
    fn tagged_template_ahead(&self, p: usize) -> bool {
        if !self.has_backtick_after(p) {
            return false;
        }
        let mut s = p;
        if matches!(self.b(s), b'?' | b'.') {
            let mut j = s;
            if self.b(j) == b'?' {
                j += 1;
            }
            if self.b(j) != b'.' {
                return false;
            }
            s = self.ws(j + 1);
        }
        let limit = self.window();
        while s < limit {
            self.spend(1);
            let Some(e) = self.ident(s) else {
                return false;
            };
            if self.b(e) == b'`' {
                return true;
            }
            if self.b(e) == b'<' && self.tpb(e).is_some_and(|t| self.b(self.ws(t)) == b'`') {
                return true;
            }
            let mut j = self.ws(e);
            if self.b(j) == b'?' {
                j += 1;
            }
            if self.b(j) != b'.' {
                return false;
            }
            s = self.ws(j + 1);
        }
        false
    }

    // Regular expressions.

    pub(crate) fn regex(&mut self, p: usize) -> Option<usize> {
        if self.full() {
            return None;
        }
        let q = self.regex_start(p)?;
        self.flush(p);
        self.cap(
            Kind::Regex,
            q,
            q + 1,
            &[S::PunctuationDefinitionStringBegin],
        );
        self.push(Kind::Regex);
        Some(q + 1)
    }

    /// The `/` that opens a regular expression whose match starts at `p`.
    fn regex_start(&self, p: usize) -> Option<usize> {
        // The first form: after an operator, with blanks.
        let q = self.ws(p);
        if self.b(q) == b'/'
            && !matches!(self.b(q + 1), b'*' | b'/')
            && self.regex_before_strict(p)
            && self.regex_body(q, RegexForm::Strict)
        {
            return Some(q);
        }
        // The second form: not after a name or a closing bracket.
        if self.b(p) == b'/'
            && !matches!(self.b(p + 1), b'*' | b'/')
            && self.regex_before_loose(p)
            && self.regex_body(p, RegexForm::Loose)
        {
            return Some(p);
        }
        if (self.word_before_text(p, "return") || self.word_before_text(p, "case"))
            && self.b(q) == b'/'
            && !matches!(self.b(q + 1), b'*' | b'/')
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
        let Some(head) = self.s.get(..p) else {
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
            .is_some_and(|c| c == ']' || c == ')' || super::lexer::is_alnum_char(c))
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
            match self.b(j) {
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
                            match self.b(k) {
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
                            match self.b(k) {
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
                        match self.b(k) {
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
            let c = self.b(k);
            c == b'$' || c == b'_' || c.is_ascii_alphanumeric()
        };
        (g > f && !name_follows(g))
            || (!matches!(self.b(f), b'*' | b'/') && !name_follows(f))
            || (self.starts(f, "/*") && !name_follows(f))
    }

    /// `[dgimsuvy]*` from `i`: the end of a regular expression's flags.
    pub(crate) fn regex_flags_end(&self, i: usize) -> usize {
        let mut e = i;
        while matches!(
            self.b(e),
            b'd' | b'g' | b'i' | b'm' | b's' | b'u' | b'v' | b'y'
        ) {
            e += 1;
        }
        e
    }

    /// Inside a regular expression.
    pub(crate) fn regexp(&mut self, p: usize) -> Option<usize> {
        let b = self.b(p);
        let b1 = self.b(p + 1);
        if b == b'\\' {
            if matches!(b1, b'B' | b'b') {
                return self.tok(p, p + 2, S::KeywordControlAnchorRegexp);
            }
            if (b'1'..=b'9').contains(&b1) {
                let mut e = p + 2;
                while self.b(e).is_ascii_digit() {
                    e += 1;
                }
                return self.tok(p, e, S::KeywordOtherBackReferenceRegexp);
            }
            if b1 == b'k'
                && self.b(p + 2) == b'<'
                && let Some(n) = self.ident(p + 3)
                && self.b(n) == b'>'
            {
                self.one(p, p + 3, &[S::KeywordOtherBackReferenceRegexp]);
                self.cap(
                    Kind::Top,
                    p + 3,
                    n,
                    &[S::KeywordOtherBackReferenceRegexp, S::VariableOtherRegexp],
                );
                self.cap(Kind::Top, n, n + 1, &[S::KeywordOtherBackReferenceRegexp]);
                return Some(n + 1);
            }
        }
        if b == b'$' || b == b'^' {
            return self.tok(p, p + 1, S::KeywordControlAnchorRegexp);
        }
        if matches!(b, b'*' | b'+' | b'?') {
            return self.tok(p, p + 1, S::KeywordOperatorQuantifierRegexp);
        }
        if b == b'{'
            && let Some(e) = self.regex_quantifier(p)
        {
            return self.tok(p, e, S::KeywordOperatorQuantifierRegexp);
        }
        if b == b'|' {
            return self.tok(p, p + 1, S::KeywordOperatorOrRegexp);
        }
        if b == b'(' {
            return self.regex_group(p);
        }
        if b == b'[' && !self.full() {
            let neg = b1 == b'^';
            self.flush(p);
            self.cap(
                Kind::RxSet,
                p,
                p + 1,
                &[S::PunctuationDefinitionCharacterClassRegexp],
            );
            let mut e = p + 1;
            if neg {
                self.cap(
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
        while self.b(j).is_ascii_digit() {
            j += 1;
        }
        let has1 = j > d1;
        let mut has2 = false;
        let comma = self.b(j) == b',';
        if comma {
            j += 1;
            let d2 = j;
            while self.b(j).is_ascii_digit() {
                j += 1;
            }
            has2 = j > d2;
        }
        if !(has1 || (comma && has2)) || self.b(j) != b'}' {
            return None;
        }
        j += 1;
        if self.b(j) == b'?' {
            j += 1;
        }
        Some(j)
    }

    fn regex_group(&mut self, p: usize) -> Option<usize> {
        if self.full() {
            return None;
        }
        self.flush(p);
        let b1 = self.b(p + 1);
        let b2 = self.b(p + 2);
        if b1 == b'?' {
            let assertion = match (b2, self.b(p + 3)) {
                (b'=', _) => Some((3, S::MetaAssertionLookAheadRegexp)),
                (b'!', _) => Some((3, S::MetaAssertionNegativeLookAheadRegexp)),
                (b'<', b'=') => Some((4, S::MetaAssertionLookBehindRegexp)),
                (b'<', b'!') => Some((4, S::MetaAssertionNegativeLookBehindRegexp)),
                _ => None,
            };
            if let Some((len, scope)) = assertion {
                self.cap(
                    Kind::RxAssert,
                    p,
                    p + 1,
                    &[S::PunctuationDefinitionGroupRegexp],
                );
                self.cap(
                    Kind::RxAssert,
                    p + 1,
                    p + len,
                    &[S::PunctuationDefinitionGroupAssertionRegexp, scope],
                );
                self.push(Kind::RxAssert);
                return Some(p + len);
            }
            if b2 == b':' {
                self.cap(
                    Kind::RxGroup,
                    p,
                    p + 1,
                    &[S::PunctuationDefinitionGroupRegexp],
                );
                self.cap(
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
                && self.b(n) == b'>'
            {
                self.cap(
                    Kind::RxGroup,
                    p,
                    p + 1,
                    &[S::PunctuationDefinitionGroupRegexp],
                );
                self.cap(
                    Kind::RxGroup,
                    p + 1,
                    p + 3,
                    &[S::PunctuationDefinitionGroupRegexp],
                );
                self.cap(
                    Kind::RxGroup,
                    p + 3,
                    n,
                    &[S::PunctuationDefinitionGroupRegexp, S::VariableOtherRegexp],
                );
                self.cap(
                    Kind::RxGroup,
                    n,
                    n + 1,
                    &[S::PunctuationDefinitionGroupRegexp],
                );
                self.push(Kind::RxGroup);
                return Some(n + 1);
            }
        }
        self.cap(
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
        let b = self.b(p);
        if b == b'.' {
            return self.tok(p, p + 1, S::ConstantOtherCharacterClassRegexp);
        }
        if b != b'\\' {
            return None;
        }
        let c = self.b(p + 1);
        if matches!(
            c,
            b'D' | b'S' | b'W' | b'd' | b'f' | b'n' | b'r' | b's' | b't' | b'v' | b'w'
        ) {
            return self.tok(p, p + 2, S::ConstantOtherCharacterClassRegexp);
        }
        if let Some(e) = self.regex_numeric_escape(p) {
            return self.tok(p, e, S::ConstantCharacterNumericRegexp);
        }
        if c == b'c' && self.b(p + 2).is_ascii_uppercase() {
            return self.tok(p, p + 3, S::ConstantCharacterControlRegexp);
        }
        let e = p + 1 + self.char_len(p + 1);
        self.tok(p, e, S::ConstantCharacterEscapeBackslashRegexp)
    }

    /// `\\([0-7]{3}|x\h{2}|u\h{4})`.
    fn regex_numeric_escape(&self, p: usize) -> Option<usize> {
        let c = self.b(p + 1);
        let n = |from: usize, count: usize, ok: fn(u8) -> bool| {
            (0..count).all(|k| ok(self.b(from + k)))
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

    /// The ways the grammar's range rule reads an item at `i`, in the order its
    /// alternatives are written: a plain character (any, for the first item;
    /// not `]` or a backslash, for the second), a numeric escape, a control
    /// escape and any other escape.
    fn range_items(&self, i: usize, item: RangeItem) -> [Option<(usize, Option<S>)>; 4] {
        let any = item == RangeItem::Low;
        let plain = (i < self.len && (any || !matches!(self.b(i), b']' | b'\\')))
            .then(|| (i + self.char_len(i), None));
        let numeric = self
            .regex_numeric_escape(i)
            .map(|e| (e, Some(S::ConstantCharacterNumericRegexp)));
        let control =
            (self.b(i) == b'\\' && self.b(i + 1) == b'c' && self.b(i + 2).is_ascii_uppercase())
                .then_some((i + 3, Some(S::ConstantCharacterControlRegexp)));
        let escape = (self.b(i) == b'\\' && i + 1 < self.len).then(|| {
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
            .filter(|&(a_end, _)| self.b(a_end) == b'-')
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
            Some(s) => self.cap(Kind::Top, p, a_end, &[range, s]),
            None => self.cap(Kind::Top, p, a_end, &[range]),
        }
        self.cap(Kind::Top, a_end, a_end + 1, &[range]);
        match b_scope {
            Some(s) => self.cap(Kind::Top, a_end + 1, b_end, &[range, s]),
            None => self.cap(Kind::Top, a_end + 1, b_end, &[range]),
        }
        Some(b_end)
    }

    // Comments.

    pub(crate) fn comment(&mut self, p: usize) -> Option<usize> {
        match self.b(p) {
            b'/' => {
                if self.starts(p, "/**") && self.b(p + "/**".len()) != b'/' {
                    return self.open1(
                        Kind::CommentDoc,
                        p,
                        p + "/**".len(),
                        &[S::PunctuationDefinitionComment],
                    );
                }
                if self.b(p + 1) == b'*' {
                    return self.block_comment_begin(p);
                }
                if self.b(p + 1) == b'/' {
                    return self.line_comment_begin(p, p, Kind::CommentLine);
                }
                None
            }
            b' ' | b'\t' => {
                // `^[\t ]+` before `//`.
                if !self.bol(p) {
                    return None;
                }
                let mut j = p;
                while matches!(self.b(j), b' ' | b'\t') {
                    j += 1;
                }
                (self.starts(j, "//")).then(|| self.line_comment_begin(p, j, Kind::CommentLine))?
            }
            _ => None,
        }
    }

    /// `(/\*)(?:\s*((@)internal)(?=\s|(\*/)))?`.
    fn block_comment_begin(&mut self, p: usize) -> Option<usize> {
        if self.full() {
            return None;
        }
        self.flush(p);
        self.cap(
            Kind::CommentBlock,
            p,
            p + 2,
            &[S::PunctuationDefinitionComment],
        );
        let mut e = p + 2;
        let j = self.ws(e);
        if self.starts(j, "@internal") {
            let after = j + "@internal".len();
            if self.space_at(after) || self.starts(after, "*/") || after >= self.len {
                self.cap(
                    Kind::CommentBlock,
                    j,
                    j + 1,
                    &[
                        S::StorageTypeInternaldeclaration,
                        S::PunctuationDecoratorInternaldeclaration,
                    ],
                );
                self.cap(
                    Kind::CommentBlock,
                    j + 1,
                    after,
                    &[S::StorageTypeInternaldeclaration],
                );
                e = after;
            }
        }
        self.push(Kind::CommentBlock);
        Some(e)
    }

    /// `(^[\t ]+)?((//)(?:\s*((@)internal)(?=\s|$))?)`: `start` is where the
    /// match starts and `slashes` where the `//` is.
    pub(crate) fn line_comment_begin(
        &mut self,
        start: usize,
        slashes: usize,
        kind: Kind,
    ) -> Option<usize> {
        if self.full() {
            return None;
        }
        self.flush(start);
        let line = S::CommentLineDoubleSlash;
        if slashes > start {
            self.cap(
                kind,
                start,
                slashes,
                &[S::PunctuationWhitespaceCommentLeading],
            );
        }
        self.cap(
            kind,
            slashes,
            slashes + 2,
            &[line, S::PunctuationDefinitionComment],
        );
        let mut e = slashes + 2;
        let j = self.ws(e);
        if self.starts(j, "@internal") {
            let after = j + "@internal".len();
            if self.space_at(after) || after >= self.len {
                self.cap(kind, e, j, &[line]);
                self.cap(
                    kind,
                    j,
                    j + 1,
                    &[
                        line,
                        S::StorageTypeInternaldeclaration,
                        S::PunctuationDecoratorInternaldeclaration,
                    ],
                );
                self.cap(
                    kind,
                    j + 1,
                    after,
                    &[line, S::StorageTypeInternaldeclaration],
                );
                e = after;
            }
        }
        self.push(kind);
        Some(e)
    }

    /// `(^[\t ]+)?((//)…)` that runs to the start of the next line.
    pub(crate) fn single_line_comment_consuming_line_ending(&mut self, p: usize) -> Option<usize> {
        if self.b(p) == b'/' && self.b(p + 1) == b'/' {
            return self.line_comment_begin(p, p, Kind::CommentLineNl);
        }
        if self.bol(p) && matches!(self.b(p), b' ' | b'\t') {
            let mut j = p;
            while matches!(self.b(j), b' ' | b'\t') {
                j += 1;
            }
            if self.starts(j, "//") {
                return self.line_comment_begin(p, j, Kind::CommentLineNl);
            }
        }
        None
    }

    /// `(?=((ID\s*\??\.\s*)*|(\??\.\s*)?)(ID))`: where a template tag starts.
    pub(crate) fn template_target_start(&self, p: usize) -> bool {
        let mut s = p;
        if matches!(self.b(s), b'?' | b'.') {
            let mut j = s;
            if self.b(j) == b'?' {
                j += 1;
            }
            if self.b(j) != b'.' {
                return false;
            }
            s = self.ws(j + 1);
        }
        self.ident(s).is_some()
    }
}
