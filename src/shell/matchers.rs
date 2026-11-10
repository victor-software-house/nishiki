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
            Pattern::Comment => self.match_comment(p),
            Pattern::PipeTime => {
                self.match_keyword_between(p, b"time", scopes![S::KeywordOther], Follow::Separator)
            }
            Pattern::PipeOperator => self.match_pipe(p),
            Pattern::Separator => self.match_separator(p),
            Pattern::LineContinuation => self.match_line_continuation(p),
            Pattern::VariableAt => self.match_variable_at(p),
            Pattern::VariableDigit => self.match_variable_digit(p),
            Pattern::VariableSpecial => self.match_variable_special(p),
            Pattern::VariableName => self.match_variable_name(p),
            Pattern::HerestringWord => self.match_herestring_word(p),
            Pattern::RedirectOperator => self.match_redirect_operator(p),
            Pattern::RedirectNumber => self.match_redirect_number(p),
            Pattern::Tilde => self.match_tilde(p),
            Pattern::Glob => self.match_glob(p),
            Pattern::FloatingKeyword => self.match_floating_keyword(p),
            Pattern::Support => self.match_support(p),
            Pattern::Escape => self.match_escape(p),
            Pattern::Boolean | Pattern::BooleanArgument => self.match_boolean(p),
            Pattern::Numeric => self.match_numeric(p),
            Pattern::Unquoted => self.match_unquoted(p),
            Pattern::DoubleQuoteEscape => self.match_escape_in(p, b"\n\"$\\`"),
            Pattern::BacktickEscape => self.match_escape_in(p, b"$\\`"),
            Pattern::CaseDefault => self.match_case_default(p),
            Pattern::CaseStar => self.match_single(p, b'*', CASE_STAR),
            Pattern::CasePlus => self.match_single(p, b'+', CASE_PLUS),
            Pattern::CaseQuestion => self.match_single(p, b'?', CASE_QUESTION),
            Pattern::CaseAt => self.match_single(p, b'@', CASE_AT),
            Pattern::CasePipe => self.match_single(p, b'|', CASE_PIPE),
            Pattern::CaseParenOpen => self.match_case_paren_open(p),
            Pattern::CaseChar => self.match_case_char(p),
            Pattern::AnsiEscape => self.match_ansi_escape(p),
            Pattern::AnsiOctal => self.match_ansi_octal(p),
            Pattern::AnsiHex => self.match_ansi_hex(p),
            Pattern::AnsiControl => self.match_ansi_control(p),
            Pattern::MathOperator => self.match_math_operator(p),
            Pattern::MathHex => self.match_hex(p),
            Pattern::MathSemicolon => {
                self.match_single(p, b';', scopes![S::PunctuationSeparatorSemicolon])
            }
            Pattern::MathOctal => self.match_octal(p, scopes![S::ConstantNumericOctal]),
            Pattern::MathOther => self.match_other_base(p, scopes![S::ConstantNumericOther]),
            Pattern::MathInteger => self.match_integer(p),
            Pattern::MathName => self.match_math_name(p),
            Pattern::SpecialExpansion => self.match_special_expansion(p),
            Pattern::ArrayAccess => self.match_array_access_inline(p),
            Pattern::ParameterDigits => self.match_parameter_digits(p),
            Pattern::ParameterName => self.match_parameter_name(p),
            Pattern::ArgumentMatch => self.match_argument_word(p),
            Pattern::WildcardStar => {
                self.match_single(p, b'*', scopes![S::VariableLanguageSpecialWildcard])
            }
            Pattern::CommandKeyword => self.match_command_keyword(p),
            Pattern::CommandBuiltin => self.match_command_builtin(p),
            Pattern::CommandName => self.match_command_name(p),
            Pattern::ArrayNameEquals => self.match_array_name_equals(p),
            Pattern::ArrayBracketEquals => self.match_array_bracket_equals(p),
            Pattern::ModifiedOption => self.match_option_word(p),
            Pattern::ModifiedMatch => self.match_modified_match(p),
            Pattern::LogicalOperator => self.match_logical_operator(p),
            Pattern::LogicalTest => self.match_logical_test(p),
            Pattern::RegexCompare => {
                self.match_text(p, b"=~", scopes![S::KeywordOperatorLogicalRegex])
            }
            Pattern::FunctionBlank => self.match_function_blank(p),
            Pattern::RedirectFix => self.match_redirect_fix(p),
            Pattern::AliasBegin => self.match_alias(p),
            Pattern::ArgumentBegin => self.match_argument_begin(p),
            Pattern::ArithmeticDoubleBegin => {
                self.match_text(p, b"((", scopes![S::PunctuationSectionArithmeticDouble])
            }
            Pattern::ArithmeticSingleBegin => {
                self.match_single(p, b'(', scopes![S::PunctuationSectionArithmeticSingle])
            }
            Pattern::ArrayValueBegin => self.match_array_value(p),
            Pattern::CaseBegin => self.match_case(p),
            Pattern::CasePatternBegin => self.match_case_pattern_begin(p),
            Pattern::CaseBodyBegin => self.match_after(p, b')'),
            Pattern::CaseGroupBegin => self.match_case_group(p),
            Pattern::CaseClassBegin => {
                self.match_single(p, b'[', scopes![S::PunctuationDefinitionCharacterClass])
            }
            Pattern::CommandStatementBegin => self.match_command_statement(p),
            Pattern::CommandNameBegin => self.at_g(p).then_some(p),
            Pattern::CommandQuotedBegin => self.match_command_quoted(p),
            Pattern::ContinuationDoubleBegin => {
                (self.at_g(p) && self.prev(p) == Some(b'"')).then_some(p)
            }
            Pattern::ContinuationSingleBegin => {
                (self.at_g(p) && self.prev(p) == Some(b'\'')).then_some(p)
            }
            Pattern::ForInBegin => self.match_for_in(p),
            Pattern::ForBegin => self.match_for(p),
            Pattern::FunctionBegin => self.match_function(p),
            Pattern::FunctionBraceBegin => self.match_single(
                p,
                b'{',
                scopes![
                    S::PunctuationDefinitionGroup,
                    S::PunctuationSectionFunctionDefinition,
                ],
            ),
            Pattern::FunctionParenBegin => self.match_single(
                p,
                b'(',
                scopes![
                    S::PunctuationDefinitionGroup,
                    S::PunctuationSectionFunctionDefinition,
                ],
            ),
            Pattern::HeredocQuotedIndentBegin => self.match_heredoc_quoted(p, Heredoc::Indented),
            Pattern::HeredocQuotedBegin => self.match_heredoc_quoted(p, Heredoc::Plain),
            Pattern::HeredocIndentBegin => self.match_heredoc_plain(p, Heredoc::Indented),
            Pattern::HeredocBegin => self.match_heredoc_plain(p, Heredoc::Plain),
            Pattern::HerestringSingleBegin => self.match_herestring_quote(
                p,
                b'\'',
                scopes![S::StringQuotedSingle, S::PunctuationDefinitionStringBegin],
            ),
            Pattern::HerestringDoubleBegin => self.match_herestring_quote(
                p,
                b'"',
                scopes![S::StringQuotedDouble, S::PunctuationDefinitionStringBegin],
            ),
            Pattern::BacktickBegin => self.match_single(
                p,
                b'`',
                scopes![S::PunctuationDefinitionEvaluationBackticks],
            ),
            Pattern::BacktickCommentBegin => self.match_backtick_comment(p),
            Pattern::BacktickCommentLineBegin => {
                self.match_single(p, b'#', scopes![S::PunctuationDefinitionComment])
            }
            Pattern::LogicalDoubleBegin => {
                self.match_text(p, b"[[", scopes![S::PunctuationDefinitionLogicalExpression])
            }
            Pattern::LogicalSingleBegin => {
                self.match_single(p, b'[', scopes![S::PunctuationDefinitionLogicalExpression])
            }
            Pattern::LoopForBegin => self.match_loop_for(p),
            Pattern::LoopWhileBegin => self.match_loop_while(p),
            Pattern::LoopSelectBegin => self.match_loop_select(p),
            Pattern::LoopIfBegin => self.match_keyword_between(
                p,
                b"if",
                scopes![S::KeywordControlIf],
                Follow::Separator,
            ),
            Pattern::GroupBegin => self.match_group(p),
            Pattern::ModifiedBegin => self.match_modified(p),
            Pattern::NormalAssignBegin => self.match_normal_assign(p),
            Pattern::EnvCommandBegin => self.match_env_command(p),
            Pattern::NormalStatementBegin => self.match_normal_statement(p),
            Pattern::OptionBegin => self.match_option_begin(p),
            Pattern::ParenBegin => self.match_single(p, b'(', scopes![S::PunctuationSectionParen]),
            Pattern::ExtglobBegin => self.match_extglob(p),
            Pattern::ProcessSubstitutionBegin => self.match_process_substitution(p),
            Pattern::StringSingleBegin => {
                self.match_single(p, b'\'', scopes![S::PunctuationDefinitionStringBegin])
            }
            Pattern::StringDoubleBegin => self.match_string_double(p),
            Pattern::StringAnsiBegin => {
                self.match_text(p, b"$'", scopes![S::PunctuationDefinitionStringBegin])
            }
            Pattern::SubshellBegin => {
                self.match_text(p, b"$(", scopes![S::PunctuationDefinitionSubshellSingle])
            }
            Pattern::VariablePositionalBegin => self.match_variable_positional_begin(p),
            Pattern::VariableBraceBegin => self.match_variable_brace_begin(p),
            Pattern::WhileBegin => {
                self.match_keyword_word(p, b"while", scopes![S::KeywordControlWhile])
            }
            Pattern::EndStatement => self.match_end_statement(p),
            Pattern::EndOption => self.match_end_option(p),
            Pattern::EndArgument => self.match_end_argument(p),
            Pattern::EndAlias => self.match_end_alias(p),
            Pattern::EndArithmeticDouble => self.match_end_arithmetic_double(p),
            Pattern::EndCase => self.match_end_case(p),
            Pattern::EndCasePattern => self.match_end_case_pattern(p),
            Pattern::EndCaseBody => self.match_end_case_body(p),
            Pattern::EndCommandName => self.match_end_command_name(p),
            Pattern::EndCommandQuotedDouble => self.match_end_command_quoted(p, b'"'),
            Pattern::EndCommandQuotedSingle => self.match_end_command_quoted(p, b'\''),
            Pattern::EndFunction => matches!(self.prev(p), Some(b')' | b'}')).then_some(p),
            Pattern::EndHeredoc => self.match_end_heredoc(p),
            Pattern::EndDone => self.match_end_done(p),
            Pattern::EndFi => self.match_keyword_between(
                p,
                b"fi",
                scopes![S::KeywordControlFi],
                Follow::Separator,
            ),
            Pattern::EndBacktickComment => (!self.at_g(p)).then_some(p),
            Pattern::EndBacktickCommentLine => (self.at(p) == b'`').then_some(p),
            Pattern::EndLiteral => self.match_end_literal(p),
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
    pub(super) fn match_text(&mut self, p: usize, text: &[u8], scopes: Scopes<S>) -> Option<usize> {
        if !self.starts(p, text) {
            return None;
        }
        self.span(p, p + text.len(), scopes);
        Some(p + text.len())
    }

    /// Matches the byte `b`, naming it `scopes`.
    pub(super) fn match_single(&mut self, p: usize, b: u8, scopes: Scopes<S>) -> Option<usize> {
        if self.at(p) != b || !self.avail(p) {
            return None;
        }
        self.span(p, p + 1, scopes);
        Some(p + 1)
    }

    /// A zero-width match after `b`: `(?<=\))`.
    pub(super) fn match_after(&mut self, p: usize, b: u8) -> Option<usize> {
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
