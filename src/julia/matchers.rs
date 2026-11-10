//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups of its match with [`Lexer::capture`].
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
use super::grammar::Pattern;
use crate::rules::Chars;

/// Whether ASCII byte `b` is in the class whose ASCII characters are the
/// bits of `mask`.
fn in_ascii(mask: u128, b: u8) -> bool {
    (mask >> b) & 1 == 1
}

impl Lexer<'_> {
    /// Whether `pattern` matches at `p`, and where its match ends.
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
    pub(super) fn matcher(&mut self, pattern: Pattern, p: usize) -> Option<usize> {
        match pattern {
            Pattern::Arrow => self.arrow(p),
            Pattern::Update => self.update(p),
            Pattern::Shift => self.shift(p),
            Pattern::TypeRelation => self.type_relation(p),
            Pattern::Relation => self.relation(p),
            Pattern::TernaryQuestion => self.between_spaces(p, b"?"),
            Pattern::TernaryColon => self.between_spaces(p, b":"),
            Pattern::Boolean => self.boolean(p),
            Pattern::RangeColon => self.range_colon(p),
            Pattern::Applies => self.lit(p, b"|>"),
            Pattern::Bitwise => self.bitwise(p),
            Pattern::Arithmetic => self.arithmetic(p),
            Pattern::Compose => self.lit(p, "∘".as_bytes()),
            Pattern::Isa => self
                .lit(p, b"::")
                .or_else(|| self.between_spaces(p, b"isa")),
            Pattern::In => self.between_spaces(p, b"in"),
            Pattern::Dots => self.dots(p),
            Pattern::InterpolationOperator => self.interpolation_operator(p),
            Pattern::TransposedVariable => self.transposed_variable(p),
            Pattern::TransposedMatrix => self.transposed(p, b']'),
            Pattern::TransposedParens => self.transposed(p, b')'),

            Pattern::Array => self.lit(p, b"["),
            Pattern::ArrayEnd => self.closer(p, b']'),
            Pattern::Parentheses => self.lit(p, b"("),
            Pattern::ParenthesesEnd => self.closer(p, b')'),
            Pattern::Bracket => self.lit(p, b"{"),
            Pattern::BracketEnd => self.closer(p, b'}'),
            Pattern::ArrayBegin => self.keyword(p, b"begin"),
            Pattern::ArrayEndWord => self.keyword(p, b"end"),
            Pattern::Interpolation => self.interpolation(p),
            Pattern::InterpolationEnd => self.lit(p, b")"),
            Pattern::InterpolatedName => self.interpolated_name(p),

            Pattern::DocMacro => self.doc_macro(p),
            Pattern::DocMacroEnd => self.doc_macro_end(p),
            Pattern::Docstring => self.docstring(p),
            Pattern::DocstringEnd => self.group_one(p, b"\"\"\""),
            Pattern::StringEscapedChar => self.escape(p),
            Pattern::Cxx => self.cxx(p),
            Pattern::Py => self.prefixed(p, b"py", b"\"\"\""),
            Pattern::Js => self.prefixed(p, b"js", b"\"\"\""),
            Pattern::RMacro => self.prefixed(p, b"R", b"\"\"\""),
            Pattern::RawTriple => self.prefixed(p, b"raw", b"\"\"\""),
            Pattern::Raw => self.prefixed(p, b"raw", b"\""),
            Pattern::Sql => self.prefixed(p, b"sql", b"\"\"\""),
            Pattern::PyEnd => self.python_end(p),
            Pattern::CxxEnd
            | Pattern::JsEnd
            | Pattern::RMacroEnd
            | Pattern::RawTripleEnd
            | Pattern::SqlEnd
            | Pattern::VarTripleEnd
            | Pattern::Triple
            | Pattern::TripleEnd => self.lit(p, b"\"\"\""),
            Pattern::RawEnd | Pattern::VarEnd | Pattern::QuotedEnd => self.lit(p, b"\""),
            Pattern::RawEscapedQuote => self.raw_escaped_quote(p),
            Pattern::RawEscapedBackslashes => self.raw_escaped_backslashes(p),
            Pattern::VarTriple => self.lit(p, b"var\"\"\""),
            Pattern::Var => self.lit(p, b"var\""),
            Pattern::Char => self.lit(p, b"'"),
            Pattern::CharEnd => self.char_quote_end(p),
            Pattern::Quoted => self.quoted(p),
            Pattern::RegexTriple => self.lit(p, b"r\"\"\""),
            Pattern::Regex => self.lit(p, b"r\""),
            Pattern::RegexTripleEnd => self.regex_end(p, b"\"\"\""),
            Pattern::RegexEnd => self.regex_end(p, b"\""),
            Pattern::MacroTriple => self.string_macro(p, b"\"\"\""),
            Pattern::Macro => self.string_macro(p, b"\""),
            Pattern::MacroTripleEnd => self.macro_end(p, b"\"\"\""),
            Pattern::MacroEnd => self.escapable_macro_end(p, b'"'),
            Pattern::CommandTriple => self.command(p, b"```"),
            Pattern::Command => self.command(p, b"`"),
            Pattern::CommandTripleEnd => self.macro_end(p, b"```"),
            Pattern::CommandEnd => self.escapable_macro_end(p, b'`'),

            Pattern::FunctionAssignment => self.function_assignment(p),
            Pattern::FunctionKeyword => self.function_keyword(p),
            Pattern::FunctionCall => self.function_call(p),
            Pattern::FunctionCallEnd => self.function_call_end(p),
            Pattern::KeywordOther => self.keyword_other(p),
            Pattern::KeywordControl => self.keyword_in(p, names::CONTROL),
            Pattern::KeywordEnd => self.keyword_in(p, &[b"end"]),
            Pattern::StorageModifier => self.keyword_in(p, &[b"global", b"local", b"const"]),
            Pattern::Export => self.keyword_in(p, &[b"export"]),
            Pattern::Public => self.public(p),
            Pattern::Import => self.keyword_in(p, &[b"import"]),
            Pattern::Using => self.keyword_in(p, &[b"using"]),
            Pattern::As => self.as_keyword(p),
            Pattern::MacroName => self.macro_name(p),

            Pattern::Number => self.number(p),
            Pattern::GlobalConstant => self.word_in(p, numbers::GLOBALS),
            Pattern::LanguageConstant => {
                self.word_in(p, &[b"true", b"false", b"nothing", b"missing"])
            }

            Pattern::CommentBlock => self.lit(p, b"#="),
            Pattern::CommentBlockEnd => self.lit(p, b"=#"),
            Pattern::Todo => self.keyword(p, b"TODO"),
            Pattern::Fixme => self.keyword(p, b"FIXME"),
            Pattern::Changed => self.keyword(p, b"CHANGED"),
            Pattern::Xxx => self.keyword(p, b"XXX"),
            Pattern::LineComment => self.lit(p, b"#"),
            Pattern::LineCommentEnd => (p == self.lim() && self.nl).then_some(p + 1),

            Pattern::TypeDecl => self.type_decl(p),
            Pattern::Symbol => self.symbol(p),
            Pattern::Comma => self.lit(p, b","),
            Pattern::Semicolon => self.lit(p, b";"),
            Pattern::ForBlock => self.keyword(p, b"for"),
            Pattern::ForBlockEnd => self.for_end(p),
            Pattern::Outer => self.keyword(p, b"outer"),
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
    /// parenthesis or a brace.
    /// Group 1 is the bracket, and group 2 a dot and the transposes after it.
    fn closer(&mut self, p: usize, bracket: u8) -> Option<usize> {
        if self.at(p) != bracket {
            return None;
        }
        let dot = p + 1 + usize::from(self.at(p + 1) == b'.');
        let end = self.skip(dot, |b| b == b'\'');
        self.capture(1, p, p + 1);
        self.capture(2, p + 1, end);
        Some(end)
    }

    /// `(text)` at `p`, with the text its group 1.
    fn group_one(&mut self, p: usize, text: &[u8]) -> Option<usize> {
        let end = self.lit(p, text)?;
        self.capture(1, p, end);
        Some(end)
    }
}
