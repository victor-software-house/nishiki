//! Comments, operators, variables, strings, numbers and redirections.

use super::{is_blank, is_unquoted};
use crate::shell::engine::{DynName, Lexer};
use crate::shell::scope::S;
use crate::shell::state::Rule;

const NUMERIC_HEX: &[S] = &[S::ConstantNumeric, S::ConstantNumericHex];
const NUMERIC_OCTAL: &[S] = &[S::ConstantNumeric, S::ConstantNumericOctal];
const NUMERIC_OTHER: &[S] = &[S::ConstantNumeric, S::ConstantNumericOther];
const NUMERIC_DECIMAL: &[S] = &[S::ConstantNumeric, S::ConstantNumericDecimal];
const NUMERIC_VERSION: &[S] = &[S::ConstantNumeric, S::ConstantNumericVersion];
const NUMERIC_INTEGER: &[S] = &[S::ConstantNumeric, S::ConstantNumericInteger];

/// The words `floating_keyword` names, in the grammar's order.
const FLOATING_KEYWORDS: [(&[u8], &[S]); 8] = [
    (b"then", &[S::KeywordControlThen]),
    (b"elif", &[S::KeywordControlElif]),
    (b"else", &[S::KeywordControlElse]),
    (b"done", &[S::KeywordControlDone]),
    (b"end", &[S::KeywordControlEnd]),
    (b"do", &[S::KeywordControlDo]),
    (b"if", &[S::KeywordControlIf]),
    (b"fi", &[S::KeywordControlFi]),
];

impl Lexer<'_> {
    /// `(?:^|[\t ]++)(?:((#!).*)|((#).*))`
    pub(in crate::shell) fn m_comment(&mut self, p: usize) -> Option<usize> {
        let hash = if p == 0 && self.at(0) == b'#' && self.avail(0) {
            0
        } else {
            let j = self.blanks(p);
            if j == p || !self.avail(j) || self.at(j) != b'#' {
                return None;
            }
            j
        };
        if self.at(hash + 1) == b'!' && hash + 1 < self.lim {
            self.span(
                hash,
                hash + 2,
                &[
                    S::CommentLineNumberSign,
                    S::MetaShebang,
                    S::PunctuationDefinitionCommentShebang,
                ],
            );
            self.span(
                hash + 2,
                self.lim,
                &[S::CommentLineNumberSign, S::MetaShebang],
            );
        } else {
            self.span(
                hash,
                hash + 1,
                &[S::CommentLineNumberSign, S::PunctuationDefinitionComment],
            );
            self.span(hash + 1, self.lim, &[S::CommentLineNumberSign]);
        }
        Some(self.lim)
    }

    /// `[!|]`
    pub(in crate::shell) fn m_pipe(&mut self, p: usize) -> Option<usize> {
        if self.avail(p) && matches!(self.at(p), b'!' | b'|') {
            self.span(p, p + 1, &[S::KeywordOperatorPipe]);
            return Some(p + 1);
        }
        None
    }

    /// `(?:(?:(;)|(&&))|(\|\|))|(&)`
    pub(in crate::shell) fn m_separator(&mut self, p: usize) -> Option<usize> {
        let (len, scope): (usize, &'static [S]) = match (self.at(p), self.at(p + 1)) {
            (b';', _) if self.avail(p) => (1, &[S::PunctuationTerminatorStatementSemicolon]),
            (b'&', b'&') => (2, &[S::PunctuationSeparatorStatementAnd]),
            (b'|', b'|') => (2, &[S::PunctuationSeparatorStatementOr]),
            (b'&', _) if self.avail(p) => (1, &[S::PunctuationSeparatorStatementBackground]),
            _ => return None,
        };
        self.span(p, p + len, scope);
        Some(p + len)
    }

    /// `\\(?=\n)`
    pub(in crate::shell) fn m_line_continuation(&mut self, p: usize) -> Option<usize> {
        if self.at(p) == b'\\' && p + 1 == self.lim && self.nl {
            self.span(p, p + 1, &[S::ConstantCharacterEscapeLineContinuation]);
            return Some(p + 1);
        }
        None
    }

    // Variables.

    /// `(\$)(@(?!\w))`
    pub(in crate::shell) fn m_var_at(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'$' || self.at(p + 1) != b'@' || self.word_at(p + 2) {
            return None;
        }
        self.dollar_and(
            p,
            p + 2,
            &[
                S::PunctuationDefinitionVariable,
                S::VariableParameterPositionalAll,
            ],
            &[S::VariableParameterPositionalAll],
        );
        Some(p + 2)
    }

    /// `(\$)([0-9](?!\w))`
    pub(in crate::shell) fn m_var_digit(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'$' || !self.at(p + 1).is_ascii_digit() || self.word_at(p + 2) {
            return None;
        }
        self.dollar_and(
            p,
            p + 2,
            &[
                S::PunctuationDefinitionVariable,
                S::VariableParameterPositional,
            ],
            &[S::VariableParameterPositional],
        );
        Some(p + 2)
    }

    /// `(\$)([-!#$*0?_](?!\w))`
    pub(in crate::shell) fn m_var_special(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'$'
            || !self.avail(p + 1)
            || !matches!(
                self.at(p + 1),
                b'-' | b'!' | b'#' | b'$' | b'*' | b'0' | b'?' | b'_'
            )
            || self.word_at(p + 2)
        {
            return None;
        }
        self.dollar_and(
            p,
            p + 2,
            &[S::PunctuationDefinitionVariable, S::VariableLanguageSpecial],
            &[S::VariableLanguageSpecial],
        );
        Some(p + 2)
    }

    /// `(\$)(\w+(?!\w))`
    pub(in crate::shell) fn m_var_name(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'$' {
            return None;
        }
        let e = self.skip_word(p + 1);
        if e == p + 1 {
            return None;
        }
        self.dollar_and(
            p,
            e,
            &[S::PunctuationDefinitionVariable, S::VariableOtherNormal],
            &[S::VariableOtherNormal],
        );
        Some(e)
    }

    /// Names the `$` at `p` with `dollar` and `p + 1..end` with `rest`.
    fn dollar_and(&mut self, p: usize, end: usize, dollar: &'static [S], rest: &'static [S]) {
        self.span(p, p + 1, dollar);
        self.span(p + 1, end, rest);
    }

    /// `(\$)(\{)[\t ]*+(?=\d)`
    pub(in crate::shell) fn m_var_positional_begin(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'$' || self.at(p + 1) != b'{' {
            return None;
        }
        let j = self.blanks(p + 2);
        if !self.avail(j) || !self.at(j).is_ascii_digit() {
            return None;
        }
        self.span(
            p,
            p + 1,
            &[
                S::PunctuationDefinitionVariable,
                S::VariableParameterPositional,
            ],
        );
        self.span(
            p + 1,
            p + 2,
            &[
                S::PunctuationSectionBracketCurlyVariableBegin,
                S::PunctuationDefinitionVariable,
                S::VariableParameterPositional,
            ],
        );
        Some(j)
    }

    /// `(\$)(\{)`
    pub(in crate::shell) fn m_var_brace_begin(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'$' || self.at(p + 1) != b'{' {
            return None;
        }
        self.span(p, p + 1, &[S::PunctuationDefinitionVariable]);
        self.span(
            p + 1,
            p + 2,
            &[
                S::PunctuationSectionBracketCurlyVariableBegin,
                S::PunctuationDefinitionVariable,
            ],
        );
        Some(p + 2)
    }

    /// `[0-9]+`
    pub(in crate::shell) fn m_param_digits(&mut self, p: usize) -> Option<usize> {
        let e = self.skip(p, |b| b.is_ascii_digit());
        if e == p {
            return None;
        }
        self.span(p, e, &[S::VariableParameterPositional]);
        Some(e)
    }

    /// `(?<!\w)[-0-9A-Z_a-z]+(?!\w)`
    pub(in crate::shell) fn m_param_name(&mut self, p: usize) -> Option<usize> {
        let e = self.name_run(p)?;
        self.span(p, e, &[S::VariableOtherNormal]);
        Some(e)
    }

    /// `!|:[-=?]?|[*@]|##?|%%|[%/]`
    pub(in crate::shell) fn m_special_expansion(&mut self, p: usize) -> Option<usize> {
        if !self.avail(p) {
            return None;
        }
        let len = match self.at(p) {
            b'!' | b'*' | b'@' | b'/' => 1,
            b':' => {
                1 + usize::from(matches!(self.at(p + 1), b'-' | b'=' | b'?') && self.avail(p + 1))
            }
            b'#' => 1 + usize::from(self.at(p + 1) == b'#'),
            b'%' => 1 + usize::from(self.at(p + 1) == b'%'),
            _ => return None,
        };
        self.span(p, p + len, &[S::KeywordOperatorExpansion]);
        Some(p + len)
    }

    /// `(\[)([^]\[]+)(])`
    pub(in crate::shell) fn m_array_access_inline(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'[' || !self.avail(p) {
            return None;
        }
        let mut e = p + 1;
        while self.avail(e) && !matches!(self.at(e), b'[' | b']') {
            self.spend(1);
            e += 1;
        }
        if e == p + 1 || self.at(e) != b']' || !self.avail(e) {
            return None;
        }
        self.span(p, p + 1, &[S::PunctuationSectionArray]);
        self.span_retok(p + 1, e, &[], Rule::RetokArrayAccess);
        self.span(e, e + 1, &[S::PunctuationSectionArray]);
        Some(e + 1)
    }

    // Strings.

    /// `\\.`
    pub(in crate::shell) fn m_escape(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'\\' || p + 1 >= self.lim {
            return None;
        }
        let e = self.char_end(p + 1);
        self.span(p, e, &[S::ConstantCharacterEscape]);
        Some(e)
    }

    /// A backslash and one byte of `set`; `\n` stands for the line feed.
    pub(in crate::shell) fn m_escape_in(&mut self, p: usize, set: &[u8]) -> Option<usize> {
        if self.at(p) != b'\\' || !self.avail(p + 1) || !set.contains(&self.at(p + 1)) {
            return None;
        }
        self.span(p, p + 2, &[S::ConstantCharacterEscape]);
        Some(p + 2)
    }

    /// `\$?"`
    pub(in crate::shell) fn m_string_double(&mut self, p: usize) -> Option<usize> {
        let q = p + usize::from(self.at(p) == b'$');
        if self.at(q) != b'"' || !self.avail(q) {
            return None;
        }
        self.span(p, q + 1, &[S::PunctuationDefinitionStringBegin]);
        Some(q + 1)
    }

    /// `\\['\\abefnrtv]`
    pub(in crate::shell) fn m_ansi_escape(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'\\'
            || !self.avail(p + 1)
            || !matches!(
                self.at(p + 1),
                b'\'' | b'\\' | b'a' | b'b' | b'e' | b'f' | b'n' | b'r' | b't' | b'v'
            )
        {
            return None;
        }
        self.span(p, p + 2, &[S::ConstantCharacterEscapeAnsiC]);
        Some(p + 2)
    }

    /// `\\[0-9]{3}"`, which the grammar spells with the closing quote.
    pub(in crate::shell) fn m_ansi_octal(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'\\'
            || !(1..=3).all(|k| self.at(p + k).is_ascii_digit())
            || self.at(p + 4) != b'"'
            || !self.avail(p + 4)
        {
            return None;
        }
        self.span(p, p + 5, &[S::ConstantCharacterEscapeOctal]);
        Some(p + 5)
    }

    /// `\\x\h{2}"`
    pub(in crate::shell) fn m_ansi_hex(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'\\'
            || self.at(p + 1) != b'x'
            || !(2..=3).all(|k| self.at(p + k).is_ascii_hexdigit())
            || self.at(p + 4) != b'"'
            || !self.avail(p + 4)
        {
            return None;
        }
        self.span(p, p + 5, &[S::ConstantCharacterEscapeHex]);
        Some(p + 5)
    }

    /// `\\c."`
    pub(in crate::shell) fn m_ansi_control(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'\\' || self.at(p + 1) != b'c' || p + 2 >= self.lim {
            return None;
        }
        let q = self.char_end(p + 2);
        if self.at(q) != b'"' || !self.avail(q) {
            return None;
        }
        self.span(p, q + 1, &[S::ConstantCharacterEscapeControlChar]);
        Some(q + 1)
    }

    /// `` [^\t\n "$\&-);<>\\`|] ``: one character of unquoted text.
    pub(in crate::shell) fn m_unquoted(&mut self, p: usize) -> Option<usize> {
        if p >= self.lim || !is_unquoted(self.s[p]) {
            return None;
        }
        let e = self.char_end(p);
        self.span(p, e, &[S::StringUnquoted]);
        Some(e)
    }

    // Words and numbers.

    /// `\b(?:true|false)\b`
    pub(in crate::shell) fn m_boolean(&mut self, p: usize) -> Option<usize> {
        if !self.not_after_word(p) {
            return None;
        }
        let (len, scope): (usize, &'static [S]) = if self.starts(p, b"true") {
            (4, &[S::ConstantLanguageTrue])
        } else if self.starts(p, b"false") {
            (5, &[S::ConstantLanguageFalse])
        } else {
            return None;
        };
        if self.word_at(p + len) {
            return None;
        }
        self.span(p, p + len, scope);
        Some(p + len)
    }

    /// `(?<=[\t =]|^|[(\[{])` before a number.
    pub(in crate::shell) fn after_number_start(&self, p: usize) -> bool {
        self.prev(p)
            .is_none_or(|b| matches!(b, b'\t' | b' ' | b'=' | b'(' | b'[' | b'{'))
    }

    /// `(?=[\t ]|$|[);}])` after a number.
    fn before_number_end(&self, i: usize) -> bool {
        self.at_eol(i) || (self.avail(i) && matches!(self.at(i), b'\t' | b' ' | b')' | b';' | b'}'))
    }

    /// The numeric literal rule: its scope and end, without naming it.
    pub(in crate::shell) fn number_at(&self, p: usize) -> Option<(usize, &'static [S])> {
        let digits = |i: usize| self.skip(i, |b| b.is_ascii_digit());
        let ok = |e: usize| self.before_number_end(e);
        if self.at(p) == b'0' && matches!(self.at(p + 1), b'x' | b'X') {
            let e = self.skip(p + 2, |b| b.is_ascii_hexdigit());
            if e > p + 2 && ok(e) {
                return Some((e, NUMERIC_HEX));
            }
        }
        if self.at(p) == b'0' {
            let e = digits(p + 1);
            if e > p + 1 && ok(e) {
                return Some((e, NUMERIC_OCTAL));
            }
        }
        if let Some(e) = self.other_base_end(p)
            && ok(e)
        {
            return Some((e, NUMERIC_OTHER));
        }
        let start = p + usize::from(self.at(p) == b'-');
        let int_end = digits(start);
        if int_end > start {
            if self.at(int_end) == b'.' {
                let frac = digits(int_end + 1);
                if frac > int_end + 1 && ok(frac) {
                    return Some((frac, NUMERIC_DECIMAL));
                }
                let mut e = int_end;
                while self.at(e) == b'.' && digits(e + 1) > e + 1 {
                    e = digits(e + 1);
                }
                if e > int_end && ok(e) {
                    return Some((e, NUMERIC_VERSION));
                }
            }
            if ok(int_end) {
                return Some((int_end, NUMERIC_INTEGER));
            }
        }
        None
    }

    /// The end of `\d{1,2}#[0-9@-Z_a-z]+` at `p`.
    fn other_base_end(&self, p: usize) -> Option<usize> {
        let digits = self.skip(p, |b| b.is_ascii_digit()).min(p + 2) - p;
        for width in (1..=digits).rev() {
            if self.at(p + width) == b'#' {
                let e = self.skip(p + width + 1, |b| {
                    b.is_ascii_digit()
                        || (b'@'..=b'Z').contains(&b)
                        || b == b'_'
                        || b.is_ascii_lowercase()
                });
                if e > p + width + 1 {
                    return Some(e);
                }
            }
        }
        None
    }

    /// `numeric_literal`
    pub(in crate::shell) fn m_numeric(&mut self, p: usize) -> Option<usize> {
        if !self.after_number_start(p) {
            return None;
        }
        let (e, scopes) = self.number_at(p)?;
        self.span(p, e, scopes);
        Some(e)
    }

    /// `0[Xx]\h+`
    pub(in crate::shell) fn m_hex(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'0' || !matches!(self.at(p + 1), b'x' | b'X') {
            return None;
        }
        let e = self.skip(p + 2, |b| b.is_ascii_hexdigit());
        if e == p + 2 {
            return None;
        }
        self.span(p, e, &[S::ConstantNumericHex]);
        Some(e)
    }

    /// `0\d+`
    pub(in crate::shell) fn m_octal(&mut self, p: usize, scopes: &'static [S]) -> Option<usize> {
        if self.at(p) != b'0' {
            return None;
        }
        let e = self.skip(p + 1, |b| b.is_ascii_digit());
        if e == p + 1 {
            return None;
        }
        self.span(p, e, scopes);
        Some(e)
    }

    /// `\d{1,2}#[0-9@-Z_a-z]+`
    pub(in crate::shell) fn m_other_base(
        &mut self,
        p: usize,
        scopes: &'static [S],
    ) -> Option<usize> {
        let e = self.other_base_end(p)?;
        self.span(p, e, scopes);
        Some(e)
    }

    /// `\d+`
    pub(in crate::shell) fn m_integer(&mut self, p: usize) -> Option<usize> {
        let e = self.skip(p, |b| b.is_ascii_digit());
        if e == p {
            return None;
        }
        self.span(p, e, &[S::ConstantNumericInteger]);
        Some(e)
    }

    /// `(?<!\w)[0-9A-Z_a-z]+(?!\w)`
    pub(in crate::shell) fn m_math_name(&mut self, p: usize) -> Option<usize> {
        if !self.not_after_word(p) {
            return None;
        }
        let e = self.skip(p, |b| b.is_ascii_alphanumeric() || b == b'_');
        if e == p || self.word_at(e) {
            return None;
        }
        self.span(p, e, &[S::VariableOtherNormal]);
        Some(e)
    }

    /// The arithmetic operators, in the grammar's order, including the `^`
    /// that is an empty match at the start of a line.
    pub(in crate::shell) fn m_math_op(&mut self, p: usize) -> Option<usize> {
        let c = self.at(p);
        let next = self.at(p + 1);
        let live = self.avail(p);
        let len = match c {
            b'+' | b'-' | b'*' | b'&' | b'|' if live => {
                // `\+{1,2}` and `-{1,2}` and `\*{1,2}` come before the start
                // of line; `\|{1,2}` and `&{1,2}` after it.
                if p == 0 && matches!(c, b'&' | b'|') {
                    0
                } else {
                    1 + usize::from(next == c)
                }
            }
            b'!' | b'~' | b'%' | b'/' if live => 1,
            b'<' if live => 1 + usize::from(matches!(next, b'<' | b'=') && self.avail(p + 1)),
            b'>' if live => 1 + usize::from(matches!(next, b'=' | b'>') && self.avail(p + 1)),
            b'=' if live && next == b'=' => 2,
            b',' | b':' | b'=' | b'?' if live => usize::from(p != 0),
            b'^' if live && p != 0 && next == b'=' => 2,
            _ if p == 0 => 0,
            _ => return None,
        };
        self.span(p, p + len, &[S::KeywordOperatorArithmetic]);
        Some(p + len)
    }

    /// `(?<=^|[\t \&;])(?:then|elif|else|done|end|do|if|fi)(?=[\t \&;]|$)`
    pub(in crate::shell) fn m_floating_keyword(&mut self, p: usize) -> Option<usize> {
        if !self.after_terminator(p) {
            return None;
        }
        for (word, scopes) in FLOATING_KEYWORDS {
            if self.starts(p, word) && self.before_terminator(p + word.len()) {
                self.span(p, p + word.len(), scopes);
                return Some(p + word.len());
            }
        }
        None
    }

    /// `(?<=^|[\&;\s])[.:](?=[\&;\s]|$)`
    pub(in crate::shell) fn m_support(&mut self, p: usize) -> Option<usize> {
        if !self.after_separator(p)
            || !self.avail(p)
            || !matches!(self.at(p), b'.' | b':')
            || !self.before_separator(p + 1)
        {
            return None;
        }
        self.span(p, p + 1, &[S::SupportFunctionBuiltin]);
        Some(p + 1)
    }

    // Redirections and paths.

    /// `(?<=[\t ])(?:(1)|(2)|(\d+))(?=>)`
    pub(in crate::shell) fn m_redirect_number(&mut self, p: usize) -> Option<usize> {
        if !self.prev(p).is_some_and(is_blank) {
            return None;
        }
        if self.at(p + 1) == b'>' && self.avail(p + 1) {
            if self.at(p) == b'1' {
                self.span(p, p + 1, &[S::KeywordOperatorRedirectStdout]);
                return Some(p + 1);
            }
            if self.at(p) == b'2' {
                self.span(p, p + 1, &[S::KeywordOperatorRedirectStderr]);
                return Some(p + 1);
            }
        }
        let e = self.skip(p, |b| b.is_ascii_digit());
        if e == p || self.at(e) != b'>' || !self.avail(e) {
            return None;
        }
        self.span_dyn(
            p,
            e,
            &[],
            DynName {
                prefix: "keyword.operator.redirect.",
                start: p,
                end: e,
                suffix: ".shell",
            },
        );
        Some(e)
    }

    /// `(?<![<>])(&>|\d*>&\d*|\d*(>>|[<>])|\d*<&|\d*<>)(?![<>])`
    pub(in crate::shell) fn m_redirect_op(&mut self, p: usize) -> Option<usize> {
        if matches!(self.prev(p), Some(b'<' | b'>')) {
            return None;
        }
        let free = |i: usize| !(self.avail(i) && matches!(self.at(i), b'<' | b'>'));
        let live = |i: usize, b: u8| self.avail(i) && self.at(i) == b;
        if live(p, b'&') && live(p + 1, b'>') && free(p + 2) {
            return Some(p + 2);
        }
        let d = self.skip(p, |b| b.is_ascii_digit());
        if live(d, b'>') && live(d + 1, b'&') {
            let r = self.skip(d + 2, |b| b.is_ascii_digit());
            if free(r) {
                return Some(r);
            }
            if r > d + 2 {
                return Some(r - 1);
            }
        }
        if live(d, b'>') && live(d + 1, b'>') && free(d + 2) {
            return Some(d + 2);
        }
        if (live(d, b'<') || live(d, b'>')) && free(d + 1) {
            return Some(d + 1);
        }
        if live(d, b'<') && live(d + 1, b'&') && free(d + 2) {
            return Some(d + 2);
        }
        if live(d, b'<') && live(d + 1, b'>') && free(d + 2) {
            return Some(d + 2);
        }
        None
    }

    /// `[<>]\(`
    pub(in crate::shell) fn m_process_substitution(&mut self, p: usize) -> Option<usize> {
        if self.avail(p) && matches!(self.at(p), b'<' | b'>') && self.at(p + 1) == b'(' {
            self.span(p, p + 2, &[S::PunctuationDefinitionStringBegin]);
            return Some(p + 2);
        }
        None
    }

    /// `(?<=[:=\s]|^)~`
    pub(in crate::shell) fn m_tilde(&mut self, p: usize) -> Option<usize> {
        let after = p == 0 || matches!(self.prev(p), Some(b':' | b'=')) || self.space_before(p);
        if self.at(p) != b'~' || !self.avail(p) || !after {
            return None;
        }
        self.span(p, p + 1, &[S::KeywordOperatorTilde]);
        Some(p + 1)
    }

    /// `[*?]`
    pub(in crate::shell) fn m_glob(&mut self, p: usize) -> Option<usize> {
        if self.avail(p) && matches!(self.at(p), b'*' | b'?') {
            self.span(p, p + 1, &[S::KeywordOperatorGlob]);
            return Some(p + 1);
        }
        None
    }

    /// `([!*+?@])(\()`
    pub(in crate::shell) fn m_extglob(&mut self, p: usize) -> Option<usize> {
        if self.avail(p)
            && matches!(self.at(p), b'!' | b'*' | b'+' | b'?' | b'@')
            && self.at(p + 1) == b'('
        {
            self.span(p, p + 1, &[S::KeywordOperatorExtglob]);
            self.span(p + 1, p + 2, &[S::PunctuationDefinitionExtglob]);
            return Some(p + 2);
        }
        None
    }

    /// `(<<<)\s*(('))` and its double-quoted twin.
    pub(in crate::shell) fn m_herestring_quote(
        &mut self,
        p: usize,
        quote: u8,
        scopes: &'static [S],
    ) -> Option<usize> {
        if !self.starts(p, b"<<<") {
            return None;
        }
        let q = self.skip_space(p + 3);
        if q >= self.lim || self.at(q) != quote {
            return None;
        }
        self.span(p, p + 3, &[S::KeywordOperatorHerestring]);
        self.span(q, q + 1, scopes);
        Some(q + 1)
    }

    /// `(<<<)\s*(([^)\\\s]|\\.)+)`
    pub(in crate::shell) fn m_herestring_word(&mut self, p: usize) -> Option<usize> {
        if !self.starts(p, b"<<<") {
            return None;
        }
        let w = self.skip_space(p + 3);
        let mut e = w;
        while e < self.lim {
            self.spend(1);
            let b = self.s[e];
            if b == b'\\' {
                if e + 1 >= self.lim {
                    break;
                }
                e = self.char_end(e + 1);
            } else if b != b')' && !self.space_at(e) {
                e = self.char_end(e);
            } else {
                break;
            }
        }
        if e == w {
            return None;
        }
        self.span(p, p + 3, &[S::KeywordOperatorHerestring]);
        self.span_retok(w, e, &[S::StringUnquotedHerestring], Rule::RetokInitial);
        Some(e)
    }

    /// `` (>>?)[\t ]*+([^\t\n "$\&-);<>\\`|]+) ``
    pub(in crate::shell) fn m_redirect_fix(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'>' || !self.avail(p) {
            return None;
        }
        for op in [2, 1] {
            if op == 2 && self.at(p + 1) != b'>' {
                continue;
            }
            let w = self.blanks(p + op);
            let e = self.skip(w, is_unquoted);
            if e > w {
                self.span(p, p + op, &[S::KeywordOperatorRedirect]);
                self.span(w, e, &[S::StringUnquotedArgument]);
                return Some(e);
            }
        }
        None
    }
}
