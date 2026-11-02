//! The matchers: one function per regular expression of the grammar, tried at
//! one position, each leaving the scopes of its captures as spans.
//!
//! Each follows its regular expression's semantics, including the lookbehinds
//! and the possessive quantifiers, which never give characters back.

mod blocks;
mod expansions;
mod statements;

use super::engine::Lexer;
use super::rules::Pat;
use super::scope::S;

/// The regex class `\s`, for ASCII.
pub(super) fn is_space(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c)
}

pub(super) fn is_blank(b: u8) -> bool {
    matches!(b, b' ' | b'\t')
}

/// The regex `\w` for a character: letters and digits, `_`, and the
/// connector punctuation Unicode adds. Combining marks are not word
/// characters here, though the regex counts them.
pub(super) fn is_word_char(c: char) -> bool {
    c.is_alphanumeric()
        || matches!(
            c,
            '_' | '\u{203f}' | '\u{2040}' | '\u{2054}' | '\u{fe33}' | '\u{fe34}' | '\u{fe4d}'
                ..='\u{fe4f}' | '\u{ff3f}'
        )
}

/// The class `[-0-9A-Z_a-z]` the grammar spells out for names.
#[inline]
pub(super) fn is_name(b: u8) -> bool {
    NAME_BYTES[usize::from(b)]
}

/// [`is_name`] for every byte, so a scan is one load a byte.
#[expect(
    clippy::as_conversions,
    reason = "b < 256 and the table is built in a const"
)]
static NAME_BYTES: [bool; 256] = {
    let mut table = [false; 256];
    let mut b = 0_u8;
    loop {
        table[b as usize] = b.is_ascii_alphanumeric() || b == b'_' || b == b'-';
        if b == u8::MAX {
            break table;
        }
        b += 1;
    }
};

/// The class `` [^\t\n "$\&-);<>\\`|] `` of unquoted text, as a byte test.
pub(super) fn is_unquoted(b: u8) -> bool {
    !matches!(
        b,
        b'\t'
            | b'\n'
            | b' '
            | b'"'
            | b'$'
            | b'&'
            | b'\''
            | b'('
            | b')'
            | b';'
            | b'<'
            | b'>'
            | b'\\'
            | b'`'
            | b'|'
    )
}

impl Lexer<'_> {
    /// Runs the matcher of `pat` at `p`, and returns where the match ends.
    #[expect(clippy::too_many_lines, reason = "one arm per pattern, a table")]
    pub(super) fn matcher(&mut self, pat: Pat, p: usize) -> Option<usize> {
        match pat {
            Pat::Comment => self.m_comment(p),
            Pat::PipeTime => self.m_keyword_between(p, b"time", &[S::KeywordOther], false),
            Pat::PipeOp => self.m_pipe(p),
            Pat::Sep => self.m_separator(p),
            Pat::LineCont => self.m_line_continuation(p),
            Pat::VarAt => self.m_var_at(p),
            Pat::VarDigit => self.m_var_digit(p),
            Pat::VarSpecial => self.m_var_special(p),
            Pat::VarName => self.m_var_name(p),
            Pat::HerestringWord => self.m_herestring_word(p),
            Pat::RedirectOp => self.m_redirect_op(p),
            Pat::RedirectNum => self.m_redirect_number(p),
            Pat::Tilde => self.m_tilde(p),
            Pat::Glob => self.m_glob(p),
            Pat::FloatKw => self.m_floating_keyword(p),
            Pat::Support => self.m_support(p),
            Pat::Escape => self.m_escape(p),
            Pat::Boolean | Pat::BoolArg => self.m_boolean(p),
            Pat::Numeric => self.m_numeric(p),
            Pat::Unquoted => self.m_unquoted(p),
            Pat::DqEscape => self.m_escape_in(p, b"\n\"$\\`"),
            Pat::BtEscape => self.m_escape_in(p, b"$\\`"),
            Pat::CaseDefault => self.m_case_default(p),
            Pat::CaseStar => self.m_single(p, b'*', CASE_STAR),
            Pat::CasePlus => self.m_single(p, b'+', CASE_PLUS),
            Pat::CaseQuestion => self.m_single(p, b'?', CASE_QUESTION),
            Pat::CaseAt => self.m_single(p, b'@', CASE_AT),
            Pat::CasePipe => self.m_single(p, b'|', CASE_PIPE),
            Pat::CaseParenOpen => self.m_case_paren_open(p),
            Pat::CaseChar => self.m_case_char(p),
            Pat::AnsiEscape => self.m_ansi_escape(p),
            Pat::AnsiOctal => self.m_ansi_octal(p),
            Pat::AnsiHex => self.m_ansi_hex(p),
            Pat::AnsiControl => self.m_ansi_control(p),
            Pat::MathOp => self.m_math_op(p),
            Pat::MathHex => self.m_hex(p),
            Pat::MathSemi => self.m_single(p, b';', &[S::PunctuationSeparatorSemicolon]),
            Pat::MathOctal => self.m_octal(p, &[S::ConstantNumericOctal]),
            Pat::MathOther => self.m_other_base(p, &[S::ConstantNumericOther]),
            Pat::MathInt => self.m_integer(p),
            Pat::MathName => self.m_math_name(p),
            Pat::SpecialExpansion => self.m_special_expansion(p),
            Pat::ArrayAccess => self.m_array_access_inline(p),
            Pat::ParamDigits => self.m_param_digits(p),
            Pat::ParamName => self.m_param_name(p),
            Pat::ArgMatch => self.m_argument_word(p),
            Pat::WildcardStar => self.m_single(p, b'*', &[S::VariableLanguageSpecialWildcard]),
            Pat::CmdKeyword => self.m_command_keyword(p),
            Pat::CmdBuiltin => self.m_command_builtin(p),
            Pat::CmdName => self.m_command_name(p),
            Pat::ArrayNameEq => self.m_array_name_eq(p),
            Pat::ArrayBracketEq => self.m_array_bracket_eq(p),
            Pat::ModOption => self.m_option_word(p),
            Pat::ModMatch => self.m_modified_match(p),
            Pat::LogicalOp => self.m_logical_op(p),
            Pat::LogicalTest => self.m_logical_test(p),
            Pat::RegexCompare => self.m_text(p, b"=~", &[S::KeywordOperatorLogicalRegex]),
            Pat::FuncBlank => self.m_function_blank(p),
            Pat::RedirectFix => self.m_redirect_fix(p),
            Pat::AliasB => self.m_alias(p),
            Pat::ArgB => self.m_argument_begin(p),
            Pat::ArithDoubleB => self.m_text(p, b"((", &[S::PunctuationSectionArithmeticDouble]),
            Pat::ArithSingleB => self.m_single(p, b'(', &[S::PunctuationSectionArithmeticSingle]),
            Pat::ArrayValueB => self.m_array_value(p),
            Pat::CaseB => self.m_case(p),
            Pat::CasePatternB => self.m_case_pattern_begin(p),
            Pat::CaseBodyB => self.m_after(p, b')'),
            Pat::CaseGroupB => self.m_case_group(p),
            Pat::CaseClassB => self.m_single(p, b'[', &[S::PunctuationDefinitionCharacterClass]),
            Pat::CmdStatementB => self.m_command_statement(p),
            Pat::CmdNameB => self.at_g(p).then_some(p),
            Pat::CmdQuotedB => self.m_command_quoted(p),
            Pat::ContinuationDoubleB => (self.at_g(p) && self.prev(p) == Some(b'"')).then_some(p),
            Pat::ContinuationSingleB => (self.at_g(p) && self.prev(p) == Some(b'\'')).then_some(p),
            Pat::ForInB => self.m_for_in(p),
            Pat::ForB => self.m_for(p),
            Pat::FunctionB => self.m_function(p),
            Pat::FuncBraceB => self.m_single(
                p,
                b'{',
                &[
                    S::PunctuationDefinitionGroup,
                    S::PunctuationSectionFunctionDefinition,
                ],
            ),
            Pat::FuncParenB => self.m_single(
                p,
                b'(',
                &[
                    S::PunctuationDefinitionGroup,
                    S::PunctuationSectionFunctionDefinition,
                ],
            ),
            Pat::HeredocQuotedIndentB => self.m_heredoc_quoted(p, true),
            Pat::HeredocQuotedB => self.m_heredoc_quoted(p, false),
            Pat::HeredocIndentB => self.m_heredoc_plain(p, true),
            Pat::HeredocB => self.m_heredoc_plain(p, false),
            Pat::HerestringSingleB => self.m_herestring_quote(
                p,
                b'\'',
                &[S::StringQuotedSingle, S::PunctuationDefinitionStringBegin],
            ),
            Pat::HerestringDoubleB => self.m_herestring_quote(
                p,
                b'"',
                &[S::StringQuotedDouble, S::PunctuationDefinitionStringBegin],
            ),
            Pat::BacktickB => {
                self.m_single(p, b'`', &[S::PunctuationDefinitionEvaluationBackticks])
            }
            Pat::BacktickCommentB => self.m_backtick_comment(p),
            Pat::BacktickCommentLineB => self.m_single(p, b'#', &[S::PunctuationDefinitionComment]),
            Pat::LogicalDoubleB => {
                self.m_text(p, b"[[", &[S::PunctuationDefinitionLogicalExpression])
            }
            Pat::LogicalSingleB => {
                self.m_single(p, b'[', &[S::PunctuationDefinitionLogicalExpression])
            }
            Pat::LoopForB => self.m_loop_for(p),
            Pat::LoopWhileB => self.m_loop_while(p),
            Pat::LoopSelectB => self.m_loop_select(p),
            Pat::LoopIfB => self.m_keyword_between(p, b"if", &[S::KeywordControlIf], false),
            Pat::GroupB => self.m_group(p),
            Pat::ModifiedB => self.m_modified(p),
            Pat::NormalAssignB => self.m_normal_assign(p),
            Pat::EnvCommandB => self.m_env_command(p),
            Pat::NormalStatementB => self.m_normal_statement(p),
            Pat::OptionB => self.m_option_begin(p),
            Pat::ParenB => self.m_single(p, b'(', &[S::PunctuationSectionParen]),
            Pat::ExtglobB => self.m_extglob(p),
            Pat::ProcessSubstitutionB => self.m_process_substitution(p),
            Pat::StringSingleB => self.m_single(p, b'\'', &[S::PunctuationDefinitionStringBegin]),
            Pat::StringDoubleB => self.m_string_double(p),
            Pat::StringAnsiB => self.m_text(p, b"$'", &[S::PunctuationDefinitionStringBegin]),
            Pat::SubshellB => self.m_text(p, b"$(", &[S::PunctuationDefinitionSubshellSingle]),
            Pat::VarPositionalB => self.m_var_positional_begin(p),
            Pat::VarBraceB => self.m_var_brace_begin(p),
            Pat::WhileB => self.m_keyword_word(p, b"while", &[S::KeywordControlWhile]),
            Pat::EndStatement => self.m_end_statement(p),
            Pat::EndOption => self.m_end_option(p),
            Pat::EndArg => self.m_end_argument(p),
            Pat::EndAlias => self.m_end_alias(p),
            Pat::EndArithDouble => self.m_end_arith_double(p),
            Pat::EndCase => self.m_end_case(p),
            Pat::EndCasePattern => self.m_end_case_pattern(p),
            Pat::EndCaseBody => self.m_end_case_body(p),
            Pat::EndCmdName => self.m_end_command_name(p),
            Pat::EndCmdQuotedDouble => self.m_end_command_quoted(p, b'"'),
            Pat::EndCmdQuotedSingle => self.m_end_command_quoted(p, b'\''),
            Pat::EndFunction => matches!(self.prev(p), Some(b')' | b'}')).then_some(p),
            Pat::EndHeredoc => self.m_end_heredoc(p),
            Pat::EndDone => self.m_end_done(p),
            Pat::EndFi => self.m_keyword_between(p, b"fi", &[S::KeywordControlFi], false),
            Pat::EndBacktickComment => (!self.at_g(p)).then_some(p),
            Pat::EndBacktickCommentLine => (self.at(p) == b'`').then_some(p),
            Pat::EndLit => self.m_end_lit(p),
        }
    }

    // Shared building blocks.

    /// The character that starts at `i`, which is a character boundary.
    fn char_at(&self, i: usize) -> Option<char> {
        if !self.avail(i) {
            return None;
        }
        if i == self.lim {
            return Some('\n');
        }
        let text = std::str::from_utf8(&self.s[i..self.char_end(i)]).ok()?;
        text.chars().next()
    }

    /// The character that ends at `p`.
    fn char_before(&self, p: usize) -> Option<char> {
        if p == 0 {
            return None;
        }
        let mut k = p - 1;
        while k > 0 && k < self.lim && self.s[k] & 0xc0 == 0x80 {
            k -= 1;
        }
        self.char_at(k)
    }

    /// Whether a word character stands at `i`.
    pub(super) fn word_at(&self, i: usize) -> bool {
        match self.at(i) {
            b if b < 0x80 => self.avail(i) && (b.is_ascii_alphanumeric() || b == b'_'),
            _ => self.char_at(i).is_some_and(is_word_char),
        }
    }

    /// Whether a word character ends at `p`.
    pub(super) fn word_before(&self, p: usize) -> bool {
        match p.checked_sub(1).map(|k| self.at(k)) {
            None => false,
            Some(b) if b < 0x80 => b.is_ascii_alphanumeric() || b == b'_',
            Some(_) => self.char_before(p).is_some_and(is_word_char),
        }
    }

    /// The regex `\s` at `i`.
    pub(super) fn space_at(&self, i: usize) -> bool {
        match self.at(i) {
            b if b < 0x80 => self.avail(i) && is_space(b),
            _ => self.char_at(i).is_some_and(char::is_whitespace),
        }
    }

    /// The regex `\s` before `p`.
    pub(super) fn space_before(&self, p: usize) -> bool {
        match p.checked_sub(1).map(|k| self.at(k)) {
            None => false,
            Some(b) if b < 0x80 => is_space(b),
            Some(_) => self.char_before(p).is_some_and(char::is_whitespace),
        }
    }

    /// Skips word characters from `i`.
    pub(super) fn skip_word(&self, mut i: usize) -> usize {
        while i < self.lim && self.word_at(i) {
            self.spend(1);
            i = self.char_end(i);
        }
        i
    }

    /// Skips `\s` from `i`, within the text.
    pub(super) fn skip_space(&self, mut i: usize) -> usize {
        while i < self.lim && self.space_at(i) {
            self.spend(1);
            i = self.char_end(i);
        }
        i
    }

    /// The regex `(?<=^|[\&;\s])`.
    pub(super) fn after_separator(&self, p: usize) -> bool {
        p == 0 || matches!(self.prev(p), Some(b'&' | b';')) || self.space_before(p)
    }

    /// The regex `(?=[\&;\s]|$)`.
    pub(super) fn before_separator(&self, i: usize) -> bool {
        self.at_eol(i) || (self.avail(i) && (matches!(self.at(i), b'&' | b';') || self.space_at(i)))
    }

    /// The regex `(?=[\t \&;]|$)`.
    pub(super) fn before_terminator(&self, i: usize) -> bool {
        self.at_eol(i) || (self.avail(i) && matches!(self.at(i), b'\t' | b' ' | b'&' | b';'))
    }

    /// The regex `(?<=^|[\t \&;])`.
    pub(super) fn after_terminator(&self, p: usize) -> bool {
        self.prev(p)
            .is_none_or(|b| matches!(b, b'\t' | b' ' | b'&' | b';'))
    }

    /// The regex `\b` at `i`.
    pub(super) fn boundary(&self, i: usize) -> bool {
        self.word_before(i) != self.word_at(i)
    }

    /// The regex `(?<!\w)`.
    pub(super) fn not_after_word(&self, p: usize) -> bool {
        !self.word_before(p)
    }

    /// The regex `(?<!\w)[-0-9A-Z_a-z]+(?!\w)` at `p`: where the name ends.
    pub(super) fn name_run(&self, p: usize) -> Option<usize> {
        let (at, lim, answer) = self.names.get();
        if at == p && lim == self.lim {
            return answer;
        }
        let answer = self.scan_name_run(p);
        self.names.set((p, self.lim, answer));
        answer
    }

    fn scan_name_run(&self, p: usize) -> Option<usize> {
        if p >= self.lim || !is_name(self.s[p]) {
            return None;
        }
        if let Some(&before) = p.checked_sub(1).and_then(|k| self.s.get(k)) {
            // A word character before, ASCII decided without decoding.
            if before.is_ascii_alphanumeric() || before == b'_' {
                return None;
            }
            if before >= 0x80 && self.word_before(p) {
                return None;
            }
        }
        let mut e = p + 1;
        while e < self.lim && is_name(self.s[e]) {
            e += 1;
        }
        self.spend(e - p);
        if e == p {
            return None;
        }
        // `(?!\w)` fails before a non-ASCII character, and then the run gives
        // back characters until a `-`, which is not a word character, follows.
        while self.word_at(e) {
            e -= 1;
            if e == p {
                return None;
            }
        }
        Some(e)
    }

    /// Matches the literal `text`, naming it `scopes`.
    pub(super) fn m_text(&mut self, p: usize, text: &[u8], scopes: &'static [S]) -> Option<usize> {
        if !self.starts(p, text) {
            return None;
        }
        self.span(p, p + text.len(), scopes);
        Some(p + text.len())
    }

    /// Matches the byte `b`, naming it `scopes`.
    pub(super) fn m_single(&mut self, p: usize, b: u8, scopes: &'static [S]) -> Option<usize> {
        if self.at(p) != b || !self.avail(p) {
            return None;
        }
        self.span(p, p + 1, scopes);
        Some(p + 1)
    }

    /// A zero-width match after `b`: `(?<=\))`.
    pub(super) fn m_after(&mut self, p: usize, b: u8) -> Option<usize> {
        (self.prev(p) == Some(b)).then_some(p)
    }
}

pub(super) const CASE_STAR: &[S] = &[
    S::VariableLanguageSpecialQuantifierStar,
    S::KeywordOperatorQuantifierStar,
    S::PunctuationDefinitionArbitraryRepetition,
    S::PunctuationDefinitionRegexArbitraryRepetition,
];
pub(super) const CASE_PLUS: &[S] = &[
    S::VariableLanguageSpecialQuantifierPlus,
    S::KeywordOperatorQuantifierPlus,
    S::PunctuationDefinitionArbitraryRepetition,
    S::PunctuationDefinitionRegexArbitraryRepetition,
];
pub(super) const CASE_QUESTION: &[S] = &[
    S::VariableLanguageSpecialQuantifierQuestion,
    S::KeywordOperatorQuantifierQuestion,
    S::PunctuationDefinitionArbitraryRepetition,
    S::PunctuationDefinitionRegexArbitraryRepetition,
];
pub(super) const CASE_AT: &[S] = &[
    S::VariableLanguageSpecialAt,
    S::KeywordOperatorAt,
    S::PunctuationDefinitionRegexAt,
];
pub(super) const CASE_PIPE: &[S] = &[
    S::KeywordOperatorOrvariableLanguageSpecialOr,
    S::KeywordOperatorAlternationRuby,
    S::PunctuationDefinitionRegexAlternation,
    S::PunctuationSeparatorRegexAlternation,
];
