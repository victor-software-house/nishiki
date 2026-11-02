//! Statements, commands, arguments, options and assignments.

use super::{is_blank, is_unquoted};
use crate::shell::engine::Lexer;
use crate::shell::scope::S;
use crate::shell::state::Rule;

/// What `normal_statement` refuses to open before, as `kw\W`.
const STATEMENT_KEYWORDS: &[&[u8]] = &[
    b"nocorrect",
    b"function",
    b"foreach",
    b"repeat",
    b"logout",
    b"coproc",
    b"select",
    b"while",
    b"pushd",
    b"until",
    b"case",
    b"done",
    b"elif",
    b"else",
    b"esac",
    b"popd",
    b"then",
    b"time",
    b"for",
    b"end",
    b"fi",
    b"do",
    b"in",
    b"if",
];

/// What `command_statement` refuses to open before, as `kw `, `kw\t`, `kw$`.
const COMMAND_KEYWORDS: &[&[u8]] = &[
    b"nocorrect",
    b"readonly",
    b"function",
    b"foreach",
    b"coproc",
    b"logout",
    b"export",
    b"select",
    b"repeat",
    b"pushd",
    b"until",
    b"while",
    b"local",
    b"case",
    b"done",
    b"elif",
    b"else",
    b"esac",
    b"popd",
    b"then",
    b"time",
    b"for",
    b"end",
    b"fi",
    b"do",
    b"in",
    b"if",
];

/// Keywords that open a statement one space later: `^until| until|\tuntil|…`.
const BLOCK_KEYWORDS: &[&[u8]] = &[b"until", b"while", b"elif", b"else", b"then", b"do", b"if"];

/// Builtins `command_name_range` names `support.function.builtin`.
const BUILTINS: &[&[u8]] = &[
    b"unfunction",
    b"continue",
    b"autoload",
    b"unsetopt",
    b"bindkey",
    b"builtin",
    b"getopts",
    b"command",
    b"declare",
    b"unalias",
    b"history",
    b"unlimit",
    b"typeset",
    b"suspend",
    b"source",
    b"printf",
    b"unhash",
    b"disown",
    b"ulimit",
    b"return",
    b"which",
    b"alias",
    b"break",
    b"false",
    b"print",
    b"shift",
    b"times",
    b"umask",
    b"unset",
    b"read",
    b"type",
    b"exec",
    b"eval",
    b"wait",
    b"echo",
    b"dirs",
    b"jobs",
    b"kill",
    b"hash",
    b"stat",
    b"exit",
    b"test",
    b"trap",
    b"true",
    b"let",
    b"set",
    b"pwd",
    b"cd",
    b"fg",
    b"bg",
    b"fc",
];

/// The declaration words `modified_assignment_statement` begins with.
const MODIFIERS: [(&[u8], &[S]); 5] = [
    (b"readonly", &[S::StorageModifierReadonly]),
    (b"declare", &[S::StorageModifierDeclare]),
    (b"typeset", &[S::StorageModifierTypeset]),
    (b"export", &[S::StorageModifierExport]),
    (b"local", &[S::StorageModifierLocal]),
];

impl Lexer<'_> {
    /// `normal_statement`'s begin: an empty match where a statement may start.
    pub(in crate::shell) fn m_normal_statement(&mut self, p: usize) -> Option<usize> {
        if p == 0 && self.at_eol(self.skip(0, is_blank)) {
            return None;
        }
        let after_keyword = BLOCK_KEYWORDS.iter().any(|kw| {
            let l = kw.len();
            p > l
                && self.s.get(p - 1..p) == Some(b" ")
                && self.s.get(p - 1 - l..p - 1) == Some(*kw)
                && (p - 1 - l == 0 || matches!(self.s[p - 2 - l], b' ' | b'\t'))
        });
        let after_mark = self
            .prev(p)
            .is_none_or(|b| matches!(b, b'!' | b'&' | b'(' | b';' | b'`' | b'{' | b'|'));
        if !after_keyword && !after_mark {
            return None;
        }
        let j = self.skip(p, is_blank);
        let blocked = STATEMENT_KEYWORDS.iter().any(|kw| {
            self.starts(j, kw) && self.avail(j + kw.len()) && !self.word_at(j + kw.len())
        });
        if blocked { None } else { Some(j) }
    }

    /// `command_statement`'s begin.
    pub(in crate::shell) fn m_command_statement(&mut self, p: usize) -> Option<usize> {
        let j = self.skip(p, is_blank);
        if !self.avail(j) {
            return None;
        }
        let c = self.at(j);
        if matches!(
            c,
            b'\n'
                | b'!'
                | b'#'
                | b'&'
                | b'('
                | b')'
                | b'<'
                | b'>'
                | b'['
                | b'{'
                | b'|'
                | b'\t'
                | b' '
                | b';'
        ) {
            return None;
        }
        let blocked = COMMAND_KEYWORDS.iter().any(|kw| {
            let e = j + kw.len();
            self.starts(j, kw)
                && (self.at_eol(e) || (self.avail(e) && matches!(self.at(e), b' ' | b'\t')))
        });
        if blocked || (c == b'\\' && j + 1 == self.lim) {
            return None;
        }
        Some(j)
    }

    /// `[\t ]++(?![\n#\&(\[|]|$|;)`
    pub(in crate::shell) fn m_argument_begin(&mut self, p: usize) -> Option<usize> {
        let j = self.skip(p, is_blank);
        if j == p || !self.avail(j) {
            return None;
        }
        if matches!(self.at(j), b'\n' | b'#' | b'&' | b'(' | b'[' | b'|' | b';') {
            return None;
        }
        Some(j)
    }

    /// `` [\t ]*+([^\t\n "$\&-);<>\\`|]+(?!>)) ``
    pub(in crate::shell) fn m_argument_word(&mut self, p: usize) -> Option<usize> {
        let w = self.skip(p, is_blank);
        let mut e = self.skip(w, is_unquoted);
        if e == w {
            return None;
        }
        if self.avail(e) && self.at(e) == b'>' {
            // `+` gives back one character so that `>` does not follow.
            let mut k = e - 1;
            while k > w && self.s[k] & 0xc0 == 0x80 {
                k -= 1;
            }
            if k == w {
                return None;
            }
            e = k;
        }
        self.span_retok(w, e, &[S::StringUnquotedArgument], Rule::RetokArgument);
        Some(e)
    }

    /// `[\t ]++(-)((?![\n!#\&()<>\[{|]|$|[\t ;]))`
    pub(in crate::shell) fn m_option_begin(&mut self, p: usize) -> Option<usize> {
        let j = self.skip(p, is_blank);
        if j == p || self.at(j) != b'-' || !self.avail(j) || !self.avail(j + 1) {
            return None;
        }
        if matches!(
            self.at(j + 1),
            b'\n'
                | b'!'
                | b'#'
                | b'&'
                | b'('
                | b')'
                | b'<'
                | b'>'
                | b'['
                | b'{'
                | b'|'
                | b'\t'
                | b' '
                | b';'
        ) {
            return None;
        }
        self.span(
            j,
            j + 1,
            &[S::StringUnquotedArgument, S::ConstantOtherOptionDash],
        );
        Some(j + 1)
    }

    /// `(?<!\w)-\w+\b`
    pub(in crate::shell) fn m_option_word(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'-' || !self.avail(p) || !self.not_after_word(p) {
            return None;
        }
        let e = self.skip_word(p + 1);
        if e == p + 1 {
            return None;
        }
        self.span(p, e, &[S::StringUnquotedArgument, S::ConstantOtherOption]);
        Some(e)
    }

    // Command names.

    /// `(?<!\w)(?:continue|return|break)(?!\w)`
    pub(in crate::shell) fn m_command_keyword(&mut self, p: usize) -> Option<usize> {
        if !self.not_after_word(p) {
            return None;
        }
        let e = self.skip(p, |b| b.is_ascii_alphabetic());
        if self.word_at(e) {
            return None;
        }
        let scopes: &'static [S] = match &self.s[p..e] {
            b"continue" => &[
                S::EntityNameFunctionCall,
                S::EntityNameCommand,
                S::KeywordControlContinue,
            ],
            b"return" => &[
                S::EntityNameFunctionCall,
                S::EntityNameCommand,
                S::KeywordControlReturn,
            ],
            b"break" => &[
                S::EntityNameFunctionCall,
                S::EntityNameCommand,
                S::KeywordControlBreak,
            ],
            _ => return None,
        };
        self.span(p, e, scopes);
        Some(e)
    }

    /// The builtin list of `command_name_range`.
    pub(in crate::shell) fn m_command_builtin(&mut self, p: usize) -> Option<usize> {
        if !self.not_after_word(p) {
            return None;
        }
        let e = if self.avail(p) && matches!(self.at(p), b'.' | b':') {
            p + 1
        } else {
            let e = self.skip(p, |b| b.is_ascii_alphabetic());
            if !BUILTINS.contains(&&self.s[p..e]) {
                return None;
            }
            e
        };
        if (self.avail(e) && matches!(self.at(e), b'/' | b'-')) || self.word_at(e) {
            return None;
        }
        self.span(
            p,
            e,
            &[
                S::EntityNameFunctionCall,
                S::EntityNameCommand,
                S::SupportFunctionBuiltin,
            ],
        );
        Some(e)
    }

    /// `` (?<!\w)(?<=\G|["')}])([^\t\n\r "\&');->`{|]+) ``
    pub(in crate::shell) fn m_command_name(&mut self, p: usize) -> Option<usize> {
        if !self.not_after_word(p)
            || !(self.at_g(p) || matches!(self.prev(p), Some(b'"' | b'\'' | b')' | b'}')))
        {
            return None;
        }
        let e = self.skip(p, |b| {
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
        });
        if e == p {
            return None;
        }
        self.span(p, e, &[S::EntityNameFunctionCall, S::EntityNameCommand]);
        Some(e)
    }

    /// `(?:\G|(?<![\t\n #\&;{|]))(\$?)((")|('))`
    pub(in crate::shell) fn m_command_quoted(&mut self, p: usize) -> Option<usize> {
        if !self.at_g(p)
            && matches!(
                self.prev(p),
                Some(b'\t' | b'\n' | b' ' | b'#' | b'&' | b';' | b'{' | b'|')
            )
        {
            return None;
        }
        let q = p + usize::from(self.at(p) == b'$' && self.avail(p));
        if !self.avail(q) {
            return None;
        }
        let (rule, scopes): (Rule, &'static [S]) = match self.at(q) {
            b'"' => (
                Rule::CommandQuotedDouble,
                &[
                    S::MetaStatementCommandNameQuoted,
                    S::StringQuotedDouble,
                    S::PunctuationDefinitionStringBegin,
                    S::EntityNameFunctionCall,
                    S::EntityNameCommand,
                ],
            ),
            b'\'' => (
                Rule::CommandQuotedSingle,
                &[
                    S::MetaStatementCommandNameQuoted,
                    S::StringQuotedSingle,
                    S::PunctuationDefinitionStringBegin,
                    S::EntityNameFunctionCall,
                    S::EntityNameCommand,
                ],
            ),
            _ => return None,
        };
        if q > p {
            self.span(
                p,
                q,
                &[
                    S::MetaStatementCommandNameQuoted,
                    S::PunctuationDefinitionString,
                    S::EntityNameFunctionCall,
                    S::EntityNameCommand,
                ],
            );
        }
        self.span(q, q + 1, scopes);
        self.chosen = rule;
        Some(q + 1)
    }

    // Ends.

    /// `` (?=[\n\&);`{|}]|[\t ]*#|])(?<!\\) ``
    pub(in crate::shell) fn m_end_statement(&mut self, p: usize) -> Option<usize> {
        let ahead = (self.avail(p)
            && matches!(
                self.at(p),
                b'\n' | b'&' | b')' | b';' | b'`' | b'{' | b'|' | b'}' | b']'
            ))
            || {
                let j = self.skip(p, is_blank);
                self.avail(j) && self.at(j) == b'#'
            };
        if ahead && self.prev(p) != Some(b'\\') {
            Some(p)
        } else {
            None
        }
    }

    /// `(?=[\t ])|` the statement end.
    pub(in crate::shell) fn m_end_option(&mut self, p: usize) -> Option<usize> {
        if self.avail(p) && is_blank(self.at(p)) {
            return Some(p);
        }
        self.m_end_statement(p)
    }

    /// `` (?=[\t \&;|]|$|[\n)`]) ``
    pub(in crate::shell) fn m_end_argument(&mut self, p: usize) -> Option<usize> {
        let ahead = self.at_eol(p)
            || (self.avail(p)
                && matches!(
                    self.at(p),
                    b' ' | b'\t' | b'&' | b';' | b'|' | b'\n' | b')' | b'`'
                ));
        ahead.then_some(p)
    }

    /// `` (?=[\t \&;|]|$|[\n)`])|(?=<) ``
    pub(in crate::shell) fn m_end_command_name(&mut self, p: usize) -> Option<usize> {
        if self.avail(p) && self.at(p) == b'<' {
            return Some(p);
        }
        self.m_end_argument(p)
    }

    /// `(?=[\t ]|$)|(?:(?:(?:(;)|(&&))|(\|\|))|(&))`
    pub(in crate::shell) fn m_end_alias(&mut self, p: usize) -> Option<usize> {
        if self.at_eol(p) || (self.avail(p) && is_blank(self.at(p))) {
            return Some(p);
        }
        self.m_separator(p)
    }

    /// `(?<!\G)(?<=")` and its single-quote twin.
    pub(in crate::shell) fn m_end_command_quoted(&mut self, p: usize, quote: u8) -> Option<usize> {
        (!self.at_g(p) && self.prev(p) == Some(quote)).then_some(p)
    }

    // Assignments.

    /// `(?:(\[)(index)(]))?` after a name: where the index ends, for `[` at
    /// `i`.
    fn array_index(&self, i: usize) -> Option<(usize, usize)> {
        let q = i + 1;
        if let Some(e) = self.name_run_after_dollar(q)
            && self.at(e) == b']'
            && self.avail(e)
        {
            return Some((q, e));
        }
        if self.avail(q) && matches!(self.at(q), b'@' | b'*') && self.at(q + 1) == b']' {
            return Some((q, q + 1));
        }
        None
    }

    /// `\$?(?<!\w)[-0-9A-Z_a-z]+(?!\w)` at `q`.
    fn name_run_after_dollar(&self, q: usize) -> Option<usize> {
        if self.at(q) == b'$'
            && let Some(e) = self.name_run(q + 1)
        {
            return Some(e);
        }
        self.name_run(q)
    }

    /// The name, optional index and operator of an assignment at `p`, which
    /// the grammar spells in four places. `operator` says whether the
    /// operator is required.
    pub(in crate::shell) fn assignment_head(&mut self, p: usize, operator: bool) -> Option<usize> {
        let name_end = self.name_run(p)?;
        let mut i = name_end;
        let mut index = None;
        if self.at(i) == b'['
            && self.avail(i)
            && let Some((start, end)) = self.array_index(i)
        {
            index = Some((i, start, end));
            i = end + 1;
        }
        let (len, scopes): (usize, &'static [S]) = match (self.at(i), self.at(i + 1)) {
            (b'=', _) if self.avail(i) => (1, &[S::KeywordOperatorAssignment]),
            (b'+' | b'-', b'=') if self.avail(i) => (2, &[S::KeywordOperatorAssignmentCompound]),
            _ if !operator => (0, &[]),
            _ => return None,
        };
        self.span(p, name_end, &[S::VariableOtherAssignment]);
        if let Some((open, start, end)) = index {
            self.span(open, open + 1, &[S::PunctuationDefinitionArrayAccess]);
            self.span(start, end, &[S::VariableOtherAssignment]);
            self.span(end, end + 1, &[S::PunctuationDefinitionArrayAccess]);
        }
        self.span(i, i + len, scopes);
        Some(i + len)
    }

    /// `normal_assignment_statement`'s begin.
    pub(in crate::shell) fn m_normal_assign(&mut self, p: usize) -> Option<usize> {
        let w = self.skip(p, is_blank);
        self.assignment_head(w, true)
    }

    /// `array_value`'s begin: an assignment whose value opens a parenthesis.
    pub(in crate::shell) fn m_array_value(&mut self, p: usize) -> Option<usize> {
        let w = self.skip(p, is_blank);
        let e = self.assignment_head(w, true)?;
        let open = self.skip(e, is_blank);
        if self.at(open) != b'(' || !self.avail(open) {
            return None;
        }
        self.span(open, open + 1, &[S::PunctuationDefinitionArray]);
        Some(open + 1)
    }

    /// `alias_statement`'s begin.
    pub(in crate::shell) fn m_alias(&mut self, p: usize) -> Option<usize> {
        let a = self.skip(p, is_blank);
        if !self.starts(a, b"alias") {
            return None;
        }
        let options_start = self.skip(a + 5, is_blank);
        // The options, each `-\w+` and its blanks, greedy; fewer when the
        // rest then fails.
        let mut ends = std::mem::take(&mut self.ends);
        ends.clear();
        ends.push(options_start);
        let mut q = options_start;
        while self.at(q) == b'-' && self.avail(q) && self.not_after_word(q) {
            let e = self.skip_word(q + 1);
            if e == q + 1 {
                break;
            }
            q = self.skip(e, is_blank);
            self.spend(1);
            ends.push(q);
        }
        let mut found = None;
        for (used, &name) in ends.iter().enumerate().rev() {
            self.spend(1);
            self.clear_spans();
            self.span(a, a + 5, &[S::StorageTypeAlias]);
            if used > 0 {
                self.span_retok(options_start, name, &[], Rule::RetokOptions);
            }
            if let Some(e) = self.assignment_head(name, true) {
                found = Some(e);
                break;
            }
        }
        self.ends = ends;
        found
    }

    /// `modifiers` inside a declaration: `readonly`, `declare`, `typeset`,
    /// `export` or `local`.
    pub(in crate::shell) fn m_modified(&mut self, p: usize) -> Option<usize> {
        if !self.after_terminator(p) {
            return None;
        }
        for (word, scopes) in MODIFIERS {
            if self.starts(p, word) && self.before_terminator(p + word.len()) {
                self.span(p, p + word.len(), scopes);
                return Some(p + word.len());
            }
        }
        None
    }

    /// The match of `modified_assignment_statement`: a name, perhaps an
    /// operator, perhaps a number.
    pub(in crate::shell) fn m_modified_match(&mut self, p: usize) -> Option<usize> {
        let e = self.assignment_head(p, false)?;
        if self.after_number_start(e)
            && let Some((end, scopes)) = self.number_at(e)
        {
            self.span(e, end, scopes);
            return Some(end);
        }
        Some(e)
    }

    /// `(?<=[\t ])(?![\t ]|\w+=)`: where a command follows an assignment.
    pub(in crate::shell) fn m_env_command(&mut self, p: usize) -> Option<usize> {
        if !self.prev(p).is_some_and(is_blank) {
            return None;
        }
        if self.avail(p) && is_blank(self.at(p)) {
            return None;
        }
        let e = self.skip_word(p);
        if e > p && self.at(e) == b'=' && self.avail(e) {
            return None;
        }
        Some(p)
    }

    /// `((?<!\w)[-0-9A-Z_a-z]+(?!\w))(=)` in an array.
    pub(in crate::shell) fn m_array_name_eq(&mut self, p: usize) -> Option<usize> {
        let e = self.name_run(p)?;
        if self.at(e) != b'=' || !self.avail(e) {
            return None;
        }
        self.span(
            p,
            e,
            &[S::VariableOtherAssignmentArray, S::EntityOtherAttributeName],
        );
        self.span(
            e,
            e + 1,
            &[
                S::KeywordOperatorAssignment,
                S::PunctuationDefinitionAssignment,
            ],
        );
        Some(e + 1)
    }

    /// `(\[)(.+?)(])(=)` in an array.
    pub(in crate::shell) fn m_array_bracket_eq(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'[' || !self.avail(p) {
            return None;
        }
        // The text between the brackets is lazy: it ends at the first `]=`.
        if p + 1 >= self.lim {
            return None;
        }
        let e = self.find_from(2, self.char_end(p + 1), |lexer, e| {
            e < lexer.lim && lexer.s[e] == b']' && lexer.at(e + 1) == b'=' && lexer.avail(e + 1)
        })?;
        self.span(p, p + 1, &[S::PunctuationDefinitionBracketNamedArray]);
        self.span(
            p + 1,
            e,
            &[S::StringUnquoted, S::EntityOtherAttributeNameBracket],
        );
        self.span(e, e + 1, &[S::PunctuationDefinitionBracketNamedArray]);
        self.span(e + 1, e + 2, &[S::PunctuationDefinitionAssignment]);
        Some(e + 2)
    }
}
