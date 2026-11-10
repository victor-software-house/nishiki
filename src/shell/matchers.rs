//! The matchers: one function per regular expression of the grammar, tried at
//! one position, each leaving the scopes of its captures as spans.
//!
//! Each follows its regular expression's semantics, including the lookbehinds
//! and the possessive quantifiers, which never give characters back.

mod blocks;
mod expansions;
mod statements;

use blocks::{Follow, Heredoc};

use super::engine::{Lexer, NameAnswer};
use super::rules::Pattern;
use super::scope::S;
use crate::classes;
use crate::scope::{Scopes, scopes};

pub(super) fn is_blank(b: u8) -> bool {
    matches!(b, b' ' | b'\t')
}

/// The class `[-0-9A-Z_a-z]` the grammar spells out for names.
#[inline]
pub(super) fn is_name(b: u8) -> bool {
    NAME_BYTES[usize::from(b)]
}

/// [`is_name`] for every byte.
/// A scan is then one load a byte.
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

/// The class `` [^\t\n\r "\&');->`{|] `` of a command name, as a byte test.
pub(super) fn is_command_name(b: u8) -> bool {
    !matches!(
        b,
        b'\t'
            | b'\n'
            | b'\r'
            | b' '
            | b'"'
            | b'&'
            | b'\''
            | b')'
            | b';'
            | b'<'
            | b'='
            | b'>'
            | b'`'
            | b'{'
            | b'|'
    )
}

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
    /// Runs the matcher of `pattern` at `p`, and returns where the match ends.
    #[expect(clippy::too_many_lines, reason = "one arm per pattern, a table")]
    pub(super) fn matcher(&mut self, pattern: Pattern, p: usize) -> Option<usize> {
        match pattern {
            Pattern::Comment => self.m_comment(p),
            Pattern::PipeTime => {
                self.m_keyword_between(p, b"time", scopes![S::KeywordOther], Follow::Separator)
            }
            Pattern::PipeOp => self.m_pipe(p),
            Pattern::Sep => self.m_separator(p),
            Pattern::LineCont => self.m_line_continuation(p),
            Pattern::VariableAt => self.m_var_at(p),
            Pattern::VariableDigit => self.m_var_digit(p),
            Pattern::VariableSpecial => self.m_var_special(p),
            Pattern::VariableName => self.m_var_name(p),
            Pattern::HerestringWord => self.m_herestring_word(p),
            Pattern::RedirectOp => self.m_redirect_op(p),
            Pattern::RedirectNum => self.m_redirect_number(p),
            Pattern::Tilde => self.m_tilde(p),
            Pattern::Glob => self.m_glob(p),
            Pattern::FloatKw => self.m_floating_keyword(p),
            Pattern::Support => self.m_support(p),
            Pattern::Escape => self.m_escape(p),
            Pattern::Boolean | Pattern::BoolArg => self.m_boolean(p),
            Pattern::Numeric => self.m_numeric(p),
            Pattern::Unquoted => self.m_unquoted(p),
            Pattern::DqEscape => self.m_escape_in(p, b"\n\"$\\`"),
            Pattern::BtEscape => self.m_escape_in(p, b"$\\`"),
            Pattern::CaseDefault => self.m_case_default(p),
            Pattern::CaseStar => self.m_single(p, b'*', CASE_STAR),
            Pattern::CasePlus => self.m_single(p, b'+', CASE_PLUS),
            Pattern::CaseQuestion => self.m_single(p, b'?', CASE_QUESTION),
            Pattern::CaseAt => self.m_single(p, b'@', CASE_AT),
            Pattern::CasePipe => self.m_single(p, b'|', CASE_PIPE),
            Pattern::CaseParenOpen => self.m_case_paren_open(p),
            Pattern::CaseChar => self.m_case_char(p),
            Pattern::AnsiEscape => self.m_ansi_escape(p),
            Pattern::AnsiOctal => self.m_ansi_octal(p),
            Pattern::AnsiHex => self.m_ansi_hex(p),
            Pattern::AnsiControl => self.m_ansi_control(p),
            Pattern::MathOp => self.m_math_op(p),
            Pattern::MathHex => self.m_hex(p),
            Pattern::MathSemi => self.m_single(p, b';', scopes![S::PunctuationSeparatorSemicolon]),
            Pattern::MathOctal => self.m_octal(p, scopes![S::ConstantNumericOctal]),
            Pattern::MathOther => self.m_other_base(p, scopes![S::ConstantNumericOther]),
            Pattern::MathInt => self.m_integer(p),
            Pattern::MathName => self.m_math_name(p),
            Pattern::SpecialExpansion => self.m_special_expansion(p),
            Pattern::ArrayAccess => self.m_array_access_inline(p),
            Pattern::ParamDigits => self.m_param_digits(p),
            Pattern::ParamName => self.m_param_name(p),
            Pattern::ArgMatch => self.m_argument_word(p),
            Pattern::WildcardStar => {
                self.m_single(p, b'*', scopes![S::VariableLanguageSpecialWildcard])
            }
            Pattern::CmdKeyword => self.m_command_keyword(p),
            Pattern::CmdBuiltin => self.m_command_builtin(p),
            Pattern::CmdName => self.m_command_name(p),
            Pattern::ArrayNameEq => self.m_array_name_eq(p),
            Pattern::ArrayBracketEq => self.m_array_bracket_eq(p),
            Pattern::ModOption => self.m_option_word(p),
            Pattern::ModMatch => self.m_modified_match(p),
            Pattern::LogicalOp => self.m_logical_op(p),
            Pattern::LogicalTest => self.m_logical_test(p),
            Pattern::RegexCompare => self.m_text(p, b"=~", scopes![S::KeywordOperatorLogicalRegex]),
            Pattern::FuncBlank => self.m_function_blank(p),
            Pattern::RedirectFix => self.m_redirect_fix(p),
            Pattern::AliasB => self.m_alias(p),
            Pattern::ArgB => self.m_argument_begin(p),
            Pattern::ArithmeticDoubleB => {
                self.m_text(p, b"((", scopes![S::PunctuationSectionArithmeticDouble])
            }
            Pattern::ArithmeticSingleB => {
                self.m_single(p, b'(', scopes![S::PunctuationSectionArithmeticSingle])
            }
            Pattern::ArrayValueB => self.m_array_value(p),
            Pattern::CaseB => self.m_case(p),
            Pattern::CasePatternB => self.m_case_pattern_begin(p),
            Pattern::CaseBodyB => self.m_after(p, b')'),
            Pattern::CaseGroupB => self.m_case_group(p),
            Pattern::CaseClassB => {
                self.m_single(p, b'[', scopes![S::PunctuationDefinitionCharacterClass])
            }
            Pattern::CmdStatementB => self.m_command_statement(p),
            Pattern::CmdNameB => self.at_g(p).then_some(p),
            Pattern::CmdQuotedB => self.m_command_quoted(p),
            Pattern::ContinuationDoubleB => {
                (self.at_g(p) && self.prev(p) == Some(b'"')).then_some(p)
            }
            Pattern::ContinuationSingleB => {
                (self.at_g(p) && self.prev(p) == Some(b'\'')).then_some(p)
            }
            Pattern::ForInB => self.m_for_in(p),
            Pattern::ForB => self.m_for(p),
            Pattern::FunctionB => self.m_function(p),
            Pattern::FuncBraceB => self.m_single(
                p,
                b'{',
                scopes![
                    S::PunctuationDefinitionGroup,
                    S::PunctuationSectionFunctionDefinition,
                ],
            ),
            Pattern::FuncParenB => self.m_single(
                p,
                b'(',
                scopes![
                    S::PunctuationDefinitionGroup,
                    S::PunctuationSectionFunctionDefinition,
                ],
            ),
            Pattern::HeredocQuotedIndentB => self.m_heredoc_quoted(p, Heredoc::Indented),
            Pattern::HeredocQuotedB => self.m_heredoc_quoted(p, Heredoc::Plain),
            Pattern::HeredocIndentB => self.m_heredoc_plain(p, Heredoc::Indented),
            Pattern::HeredocB => self.m_heredoc_plain(p, Heredoc::Plain),
            Pattern::HerestringSingleB => self.m_herestring_quote(
                p,
                b'\'',
                scopes![S::StringQuotedSingle, S::PunctuationDefinitionStringBegin],
            ),
            Pattern::HerestringDoubleB => self.m_herestring_quote(
                p,
                b'"',
                scopes![S::StringQuotedDouble, S::PunctuationDefinitionStringBegin],
            ),
            Pattern::BacktickB => self.m_single(
                p,
                b'`',
                scopes![S::PunctuationDefinitionEvaluationBackticks],
            ),
            Pattern::BacktickCommentB => self.m_backtick_comment(p),
            Pattern::BacktickCommentLineB => {
                self.m_single(p, b'#', scopes![S::PunctuationDefinitionComment])
            }
            Pattern::LogicalDoubleB => {
                self.m_text(p, b"[[", scopes![S::PunctuationDefinitionLogicalExpression])
            }
            Pattern::LogicalSingleB => {
                self.m_single(p, b'[', scopes![S::PunctuationDefinitionLogicalExpression])
            }
            Pattern::LoopForB => self.m_loop_for(p),
            Pattern::LoopWhileB => self.m_loop_while(p),
            Pattern::LoopSelectB => self.m_loop_select(p),
            Pattern::LoopIfB => {
                self.m_keyword_between(p, b"if", scopes![S::KeywordControlIf], Follow::Separator)
            }
            Pattern::GroupB => self.m_group(p),
            Pattern::ModifiedB => self.m_modified(p),
            Pattern::NormalAssignB => self.m_normal_assign(p),
            Pattern::EnvCommandB => self.m_env_command(p),
            Pattern::NormalStatementB => self.m_normal_statement(p),
            Pattern::OptionB => self.m_option_begin(p),
            Pattern::ParenB => self.m_single(p, b'(', scopes![S::PunctuationSectionParen]),
            Pattern::ExtglobB => self.m_extglob(p),
            Pattern::ProcessSubstitutionB => self.m_process_substitution(p),
            Pattern::StringSingleB => {
                self.m_single(p, b'\'', scopes![S::PunctuationDefinitionStringBegin])
            }
            Pattern::StringDoubleB => self.m_string_double(p),
            Pattern::StringAnsiB => {
                self.m_text(p, b"$'", scopes![S::PunctuationDefinitionStringBegin])
            }
            Pattern::SubshellB => {
                self.m_text(p, b"$(", scopes![S::PunctuationDefinitionSubshellSingle])
            }
            Pattern::VariablePositionalB => self.m_var_positional_begin(p),
            Pattern::VariableBraceB => self.m_var_brace_begin(p),
            Pattern::WhileB => self.m_keyword_word(p, b"while", scopes![S::KeywordControlWhile]),
            Pattern::EndStatement => self.m_end_statement(p),
            Pattern::EndOption => self.m_end_option(p),
            Pattern::EndArg => self.m_end_argument(p),
            Pattern::EndAlias => self.m_end_alias(p),
            Pattern::EndArithmeticDouble => self.m_end_arith_double(p),
            Pattern::EndCase => self.m_end_case(p),
            Pattern::EndCasePattern => self.m_end_case_pattern(p),
            Pattern::EndCaseBody => self.m_end_case_body(p),
            Pattern::EndCmdName => self.m_end_command_name(p),
            Pattern::EndCmdQuotedDouble => self.m_end_command_quoted(p, b'"'),
            Pattern::EndCmdQuotedSingle => self.m_end_command_quoted(p, b'\''),
            Pattern::EndFunction => matches!(self.prev(p), Some(b')' | b'}')).then_some(p),
            Pattern::EndHeredoc => self.m_end_heredoc(p),
            Pattern::EndDone => self.m_end_done(p),
            Pattern::EndFi => {
                self.m_keyword_between(p, b"fi", scopes![S::KeywordControlFi], Follow::Separator)
            }
            Pattern::EndBacktickComment => (!self.at_g(p)).then_some(p),
            Pattern::EndBacktickCommentLine => (self.at(p) == b'`').then_some(p),
            Pattern::EndLit => self.m_end_lit(p),
        }
    }

    // Shared building blocks.

    /// The character that starts at `i`.
    /// `i` is a character boundary.
    fn char_at(&self, i: usize) -> Option<char> {
        if !self.avail(i) {
            return None;
        }
        if i == self.lim {
            return Some('\n');
        }
        crate::utf8::char_at(self.s.get(..self.lim)?, i).map(|(c, _)| c)
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
            _ => self.char_at(i).is_some_and(classes::is_word),
        }
    }

    /// Whether a word character ends at `p`.
    pub(super) fn word_before(&self, p: usize) -> bool {
        match p.checked_sub(1).map(|k| self.at(k)) {
            None => false,
            Some(b) if b < 0x80 => b.is_ascii_alphanumeric() || b == b'_',
            Some(_) => self.char_before(p).is_some_and(classes::is_word),
        }
    }

    /// The regex `\s` at `i`.
    pub(super) fn space_at(&self, i: usize) -> bool {
        match self.at(i) {
            b if b < 0x80 => self.avail(i) && classes::is_ascii_space(b),
            _ => self.char_at(i).is_some_and(classes::is_space),
        }
    }

    /// The regex `\s` before `p`.
    pub(super) fn space_before(&self, p: usize) -> bool {
        match p.checked_sub(1).map(|k| self.at(k)) {
            None => false,
            Some(b) if b < 0x80 => classes::is_ascii_space(b),
            Some(_) => self.char_before(p).is_some_and(classes::is_space),
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
        let known = self.names.get();
        if known.at == p && known.lim == self.lim {
            return known.end;
        }
        let end = self.scan_name_run(p);
        self.names.set(NameAnswer {
            at: p,
            lim: self.lim,
            end,
        });
        end
    }

    /// [`Lexer::name_run`] without the cache.
    /// An ASCII byte before decides `(?<!\w)` without decoding.
    /// `(?!\w)` fails before a non-ASCII word character.
    /// The run then gives back characters until a `-` follows, which is no
    /// word character.
    fn scan_name_run(&self, p: usize) -> Option<usize> {
        if p >= self.lim || !is_name(self.s[p]) {
            return None;
        }
        if let Some(&before) = p.checked_sub(1).and_then(|k| self.s.get(k)) {
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
        while self.word_at(e) {
            e -= 1;
            if e == p {
                return None;
            }
        }
        Some(e)
    }

    /// Matches the literal `text`, naming it `scopes`.
    pub(super) fn m_text(&mut self, p: usize, text: &[u8], scopes: Scopes<S>) -> Option<usize> {
        if !self.starts(p, text) {
            return None;
        }
        self.span(p, p + text.len(), scopes);
        Some(p + text.len())
    }

    /// Matches the byte `b`, naming it `scopes`.
    pub(super) fn m_single(&mut self, p: usize, b: u8, scopes: Scopes<S>) -> Option<usize> {
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

pub(super) const CASE_STAR: Scopes<S> = scopes![
    S::VariableLanguageSpecialQuantifierStar,
    S::KeywordOperatorQuantifierStar,
    S::PunctuationDefinitionArbitraryRepetition,
    S::PunctuationDefinitionRegexArbitraryRepetition,
];
pub(super) const CASE_PLUS: Scopes<S> = scopes![
    S::VariableLanguageSpecialQuantifierPlus,
    S::KeywordOperatorQuantifierPlus,
    S::PunctuationDefinitionArbitraryRepetition,
    S::PunctuationDefinitionRegexArbitraryRepetition,
];
pub(super) const CASE_QUESTION: Scopes<S> = scopes![
    S::VariableLanguageSpecialQuantifierQuestion,
    S::KeywordOperatorQuantifierQuestion,
    S::PunctuationDefinitionArbitraryRepetition,
    S::PunctuationDefinitionRegexArbitraryRepetition,
];
pub(super) const CASE_AT: Scopes<S> = scopes![
    S::VariableLanguageSpecialAt,
    S::KeywordOperatorAt,
    S::PunctuationDefinitionRegexAt,
];
pub(super) const CASE_PIPE: Scopes<S> = scopes![
    S::KeywordOperatorOrvariableLanguageSpecialOr,
    S::KeywordOperatorAlternationRuby,
    S::PunctuationDefinitionRegexAlternation,
    S::PunctuationSeparatorRegexAlternation,
];
