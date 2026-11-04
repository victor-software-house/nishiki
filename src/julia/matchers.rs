//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups of its match with [`Lexer::cap`].
//! The engine names those the grammar names.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would.
//! Where a quantifier could give characters back, the doc comment says why it
//! cannot matter, or how the matcher gives them back.
//!
//! An identifier is `START REST*`, two large classes measured on Shiki
//! ([`super::chars`]).
//! `REST` holds every character of `START`.
//! No character either holds can follow an identifier in a pattern that needs
//! it to, so `REST*` never gives a character back.

mod names;
mod numbers;
mod operators;
mod strings;

use super::chars::{is_ident_rest, is_ident_start};
use super::classes::{IDENT_REST_ASCII, IDENT_START_ASCII};
use super::engine::Lexer;
use super::grammar::Pat;
use crate::rules::Chars;

/// Whether ASCII byte `b` is in the class whose ASCII characters are the
/// bits of `mask`.
fn in_ascii(mask: u128, b: u8) -> bool {
    (mask >> b) & 1 == 1
}

impl Lexer<'_> {
    /// Whether `pat` matches at `p`, and where its match ends.
    ///
    /// The arms come in groups, in this order:
    /// - operators;
    /// - brackets;
    /// - strings and their escapes;
    /// - functions, keywords and macros;
    /// - numbers and constants;
    /// - comments;
    /// - types, symbols and punctuation.
    #[expect(clippy::too_many_lines, reason = "one arm per pattern of the grammar")]
    pub(super) fn matcher(&mut self, pat: Pat, p: usize) -> Option<usize> {
        match pat {
            Pat::Arrow => self.arrow(p),
            Pat::Update => self.update(p),
            Pat::Shift => self.shift(p),
            Pat::TypeRelation => self.type_relation(p),
            Pat::Relation => self.relation(p),
            Pat::TernaryQuestion => self.between_spaces(p, b"?"),
            Pat::TernaryColon => self.between_spaces(p, b":"),
            Pat::Boolean => self.boolean(p),
            Pat::RangeColon => self.range_colon(p),
            Pat::Applies => self.lit(p, b"|>"),
            Pat::Bitwise => self.bitwise(p),
            Pat::Arithmetic => self.arithmetic(p),
            Pat::Compose => self.lit(p, "∘".as_bytes()),
            Pat::Isa => self
                .lit(p, b"::")
                .or_else(|| self.between_spaces(p, b"isa")),
            Pat::In => self.between_spaces(p, b"in"),
            Pat::Dots => self.dots(p),
            Pat::InterpolationOperator => self.interpolation_operator(p),
            Pat::TransposedVariable => self.transposed_variable(p),
            Pat::TransposedMatrix => self.transposed(p, b']'),
            Pat::TransposedParens => self.transposed(p, b')'),

            Pat::Array => self.lit(p, b"["),
            Pat::ArrayEnd => self.closer(p, b']'),
            Pat::Parentheses => self.lit(p, b"("),
            Pat::ParenthesesEnd => self.closer(p, b')'),
            Pat::Bracket => self.lit(p, b"{"),
            Pat::BracketEnd => self.closer(p, b'}'),
            Pat::ArrayBegin => self.keyword(p, b"begin"),
            Pat::ArrayEndWord => self.keyword(p, b"end"),
            Pat::Interpolation => self.interpolation(p),
            Pat::InterpolationEnd => self.lit(p, b")"),
            Pat::InterpolatedName => self.interpolated_name(p),

            Pat::DocMacro => self.doc_macro(p),
            Pat::DocMacroEnd => self.doc_macro_end(p),
            Pat::Docstring => self.docstring(p),
            Pat::DocstringEnd => self.group_one(p, b"\"\"\""),
            Pat::StringEscapedChar => self.escape(p),
            Pat::Cxx => self.cxx(p),
            Pat::Py => self.prefixed(p, b"py", b"\"\"\""),
            Pat::Js => self.prefixed(p, b"js", b"\"\"\""),
            Pat::RMacro => self.prefixed(p, b"R", b"\"\"\""),
            Pat::RawTriple => self.prefixed(p, b"raw", b"\"\"\""),
            Pat::Raw => self.prefixed(p, b"raw", b"\""),
            Pat::Sql => self.prefixed(p, b"sql", b"\"\"\""),
            Pat::PyEnd => self.python_end(p),
            Pat::CxxEnd
            | Pat::JsEnd
            | Pat::RMacroEnd
            | Pat::RawTripleEnd
            | Pat::SqlEnd
            | Pat::VarTripleEnd
            | Pat::Triple
            | Pat::TripleEnd => self.lit(p, b"\"\"\""),
            Pat::RawEnd | Pat::VarEnd | Pat::QuotedEnd => self.lit(p, b"\""),
            Pat::RawEscapedQuote => self.raw_escaped_quote(p),
            Pat::RawEscapedBackslashes => self.raw_escaped_backslashes(p),
            Pat::VarTriple => self.lit(p, b"var\"\"\""),
            Pat::Var => self.lit(p, b"var\""),
            Pat::Char => self.lit(p, b"'"),
            Pat::CharEnd => self.char_quote_end(p),
            Pat::Quoted => self.quoted(p),
            Pat::RegexTriple => self.lit(p, b"r\"\"\""),
            Pat::Regex => self.lit(p, b"r\""),
            Pat::RegexTripleEnd => self.regex_end(p, b"\"\"\""),
            Pat::RegexEnd => self.regex_end(p, b"\""),
            Pat::MacroTriple => self.string_macro(p, b"\"\"\""),
            Pat::Macro => self.string_macro(p, b"\""),
            Pat::MacroTripleEnd => self.macro_end(p, b"\"\"\""),
            Pat::MacroEnd => self.escapable_macro_end(p, b'"'),
            Pat::CommandTriple => self.command(p, b"```"),
            Pat::Command => self.command(p, b"`"),
            Pat::CommandTripleEnd => self.macro_end(p, b"```"),
            Pat::CommandEnd => self.escapable_macro_end(p, b'`'),

            Pat::FunctionAssignment => self.function_assignment(p),
            Pat::FunctionKeyword => self.function_keyword(p),
            Pat::FunctionCall => self.function_call(p),
            Pat::FunctionCallEnd => self.function_call_end(p),
            Pat::KeywordOther => self.keyword_other(p),
            Pat::KeywordControl => self.keyword_in(p, names::CONTROL),
            Pat::KeywordEnd => self.keyword_in(p, &[b"end"]),
            Pat::StorageModifier => self.keyword_in(p, &[b"global", b"local", b"const"]),
            Pat::Export => self.keyword_in(p, &[b"export"]),
            Pat::Public => self.public(p),
            Pat::Import => self.keyword_in(p, &[b"import"]),
            Pat::Using => self.keyword_in(p, &[b"using"]),
            Pat::As => self.as_keyword(p),
            Pat::MacroName => self.macro_name(p),

            Pat::Number => self.number(p),
            Pat::GlobalConstant => self.word_in(p, numbers::GLOBALS),
            Pat::LanguageConstant => self.word_in(p, &[b"true", b"false", b"nothing", b"missing"]),

            Pat::CommentBlock => self.lit(p, b"#="),
            Pat::CommentBlockEnd => self.lit(p, b"=#"),
            Pat::Todo => self.keyword(p, b"TODO"),
            Pat::Fixme => self.keyword(p, b"FIXME"),
            Pat::Changed => self.keyword(p, b"CHANGED"),
            Pat::Xxx => self.keyword(p, b"XXX"),
            Pat::LineComment => self.lit(p, b"#"),
            Pat::LineCommentEnd => (p == self.lim() && self.nl).then_some(p + 1),

            Pat::TypeDecl => self.type_decl(p),
            Pat::Symbol => self.symbol(p),
            Pat::Comma => self.lit(p, b","),
            Pat::Semicolon => self.lit(p, b";"),
            Pat::ForBlock => self.keyword(p, b"for"),
            Pat::ForBlockEnd => self.for_end(p),
            Pat::Outer => self.keyword(p, b"outer"),
        }
    }

    /// The end of the character at `p` when `class` holds it.
    fn class_char(&self, p: usize, class: fn(char) -> bool) -> Option<usize> {
        let (c, end) = self.char_at(p)?;
        class(c).then_some(end)
    }

    /// The end of the character at `p` when an identifier can start with it.
    fn ident_start(&self, p: usize) -> Option<usize> {
        match *self.s.get(p)? {
            b if b < 0x80 => in_ascii(IDENT_START_ASCII, b).then_some(p + 1),
            _ => self.class_char(p, is_ident_start),
        }
    }

    /// The end of the run of `REST` characters from `p`.
    /// The run found last is remembered: a search from inside it ends where
    /// it does.
    fn rest_end(&self, p: usize) -> usize {
        self.spend(1);
        let key = self.lim() * 2 + usize::from(self.nl);
        if self.x.end.get() == key {
            let (from, to) = self.x.run.get();
            if from <= p && p < to {
                return to;
            }
        }
        let mut q = p;
        while let Some(&b) = self.s.get(q) {
            let end = if b < 0x80 {
                in_ascii(IDENT_REST_ASCII, b).then_some(q + 1)
            } else {
                self.class_char(q, is_ident_rest)
            };
            let Some(end) = end else { break };
            q = end;
        }
        self.spend(q - p);
        self.x.end.set(key);
        self.x.run.set((p, q));
        q
    }

    /// Where the scan goes on after no pattern matched at `p`.
    ///
    /// At an ASCII letter or `_` that no quote or backtick precedes, it goes
    /// on after the run of ASCII word characters there.
    /// No pattern can match inside that run:
    /// - a pattern that starts with an identifier needs the same text after the
    ///   identifier at every place in the run, and it missed at `p`;
    /// - a string macro the grammar names, such as `py"""`, needs a quote after
    ///   the run, and then the general string macro would have matched at `p`;
    /// - the end of `py"""` takes the whole run and needs quotes after it;
    /// - every other pattern that can start with a word character needs ``, a
    ///   white space or no identifier character before it.
    pub(super) fn after_missed_word(&self, p: usize) -> usize {
        let starts_word = self.at(p).is_ascii_alphabetic() || self.at(p) == b'_';
        if !starts_word || (p > 0 && matches!(self.at(p - 1), b'"' | b'`')) {
            return p + 1;
        }
        self.skip(p + 1, |b| b.is_ascii_alphanumeric() || b == b'_')
    }

    /// `START REST*` at `p`: the end of the identifier there.
    fn ident(&self, p: usize) -> Option<usize> {
        let start = self.ident_start(p)?;
        Some(self.rest_end(start))
    }

    /// `\{(?:[^{}]|\{(?:[^{}]|\{[^{}]*})*})*}` at `p`: braces nested at most
    /// three deep, as a type's parameters are.
    /// Each brace has one place in a match, so nothing backtracks.
    /// A fourth level, or a brace left open, fails the whole group.
    fn braces(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'{' {
            return None;
        }
        let mut depth = 1_u8;
        let mut q = p + 1;
        while depth > 0 {
            match *self.s.get(q)? {
                b'{' if depth == 3 => return None,
                b'{' => depth += 1,
                b'}' => depth -= 1,
                _ => {}
            }
            q += 1;
        }
        self.spend(q - p);
        Some(q)
    }

    /// `('|(\.'))*\.?'` at `p`: transposes, each `'` or `.'`, at least one.
    /// The loop takes them all, then gives the last back to `\.?'`, so the
    /// match ends after the last.
    fn primes(&self, p: usize) -> Option<usize> {
        let mut q = p;
        loop {
            match self.at(q) {
                b'\'' => q += 1,
                b'.' if self.at(q + 1) == b'\'' => q += 2,
                _ => break,
            }
        }
        (q > p).then_some(q)
    }

    /// `(X)(\.?'*)` at `p`, for the closing bracket `X` of an array, a
    /// parenthesis or a brace: group 1 is the bracket, group 2 a dot and the
    /// transposes after it.
    fn closer(&mut self, p: usize, bracket: u8) -> Option<usize> {
        if self.at(p) != bracket {
            return None;
        }
        let dot = p + 1 + usize::from(self.at(p + 1) == b'.');
        let end = self.skip(dot, |b| b == b'\'');
        self.cap(1, p, p + 1);
        self.cap(2, p + 1, end);
        Some(end)
    }

    /// `(text)` at `p`, with the text its group 1.
    fn group_one(&mut self, p: usize, text: &[u8]) -> Option<usize> {
        let end = self.lit(p, text)?;
        self.cap(1, p, end);
        Some(end)
    }
}
