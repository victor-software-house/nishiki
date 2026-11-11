//! The matchers of Blade's PHP: for each regular expression of the grammar,
//! the function that decides whether it matches at a position and where.
//!
//! Where an expression is PHP's word for word, the arm calls PHP's matcher in
//! [`crate::php_family`].
//! Blade's copy of PHP is older than Shiki's `php` grammar.
//! Its name class is `[_a-z\x7F-ÿ]` under `(?i)` and ends at U+00FF, so
//! Blade's names are read by character, never by PHP's byte test.

use super::engine::Lexer;
use super::grammar::{Pattern, table_patterns};
use crate::rules::Chars;
use crate::utf8::char_at;

mod declarations;
mod expressions;
mod text;

/// `[_a-z\x7F-ÿ]` under `(?i)` for the character `c`.
/// Case folding adds the characters that fold into the class: `ſ`, the Kelvin
/// sign, `Ÿ`, the Angstrom sign, and `μ` and `Μ` through `µ`.
fn is_name_start_char(c: char) -> bool {
    c == '_'
        || c.is_ascii_alphabetic()
        || ('\u{7f}'..='\u{ff}').contains(&c)
        || matches!(c, 'ſ' | '\u{212a}' | 'Ÿ' | '\u{212b}' | 'μ' | 'Μ')
}

/// `[0-9_a-z\x7F-ÿ]` under `(?i)` for the character `c`.
fn is_name_char(c: char) -> bool {
    c.is_ascii_digit() || is_name_start_char(c)
}

/// Where the name start character at `i` of `s` ends, or none.
pub(super) fn name_start_at(s: &[u8], i: usize) -> Option<usize> {
    let (c, len) = char_at(s, i)?;
    is_name_start_char(c).then_some(i + len)
}

/// Where the name character at `i` of `s` ends, or none.
pub(super) fn name_char_at(s: &[u8], i: usize) -> Option<usize> {
    let (c, len) = char_at(s, i)?;
    is_name_char(c).then_some(i + len)
}

/// `[0-9\\_a-z\x7F-ÿ]` under `(?i)` for the character `c`: a name character or
/// a backslash.
fn is_qualified_char(c: char) -> bool {
    c == '\\' || is_name_char(c)
}

/// `[0-9_a-z]` under `(?i)` for the character `c`.
/// The Kelvin sign and `ſ` fold into it.
fn is_ascii_name_char(c: char) -> bool {
    c == '_' || c.is_ascii_alphanumeric() || matches!(c, 'ſ' | '\u{212a}')
}

/// `[A-Z_a-z\x7F-ÿ]` for the character `c`, without case folding.
fn is_exact_name_start(c: char) -> bool {
    c == '_' || c.is_ascii_alphabetic() || ('\u{7f}'..='\u{ff}').contains(&c)
}

/// `[0-9A-Z_a-z\x7F-ÿ]` for the character `c`, without case folding.
fn is_exact_name_char(c: char) -> bool {
    c.is_ascii_digit() || is_exact_name_start(c)
}

impl Lexer<'_> {
    /// The end of the character at `p` when `class` holds for it.
    /// The line feed after the text is no character of a class here.
    fn char_if(&self, p: usize, class: fn(char) -> bool) -> Option<usize> {
        if p >= self.lim() {
            return None;
        }
        let (c, end) = self.char_at(p)?;
        class(c).then_some(end)
    }

    /// The end of the run from `p` whose characters satisfy `class`.
    fn run_end(&self, mut p: usize, class: fn(char) -> bool) -> usize {
        let from = p;
        while let Some(end) = self.char_if(p, class) {
            p = end;
        }
        self.spend(p - from);
        p
    }

    /// `[_a-z\x7F-ÿ][0-9_a-z\x7F-ÿ]*` under `(?i)` at `p`: a name of Blade's
    /// PHP.
    pub(super) fn bname(&self, p: usize) -> Option<usize> {
        let first = self.char_if(p, is_name_start_char)?;
        Some(self.run_end(first, is_name_char))
    }

    /// `[A-Z_a-z\x7F-ÿ][0-9A-Z_a-z\x7F-ÿ]*` at `p`, without case folding.
    pub(super) fn bname_exact(&self, p: usize) -> Option<usize> {
        let first = self.char_if(p, is_exact_name_start)?;
        Some(self.run_end(first, is_exact_name_char))
    }

    /// A name and `\b`: the longest name at `p` that a word boundary ends.
    pub(super) fn bname_bounded(&self, p: usize) -> Option<usize> {
        let mut end = self.bname(p)?;
        while !self.boundary(end) {
            end = self.char_start_before(end);
            if end <= p {
                return None;
            }
        }
        Some(end)
    }

    /// The end of the run of `[0-9\\_a-z\x7F-ÿ]` under `(?i)` from `p`.
    pub(super) fn qualified_end(&self, p: usize) -> usize {
        self.run_end(p, is_qualified_char)
    }

    /// `(?i)(?=[^0-9\\_a-z\x7F-ÿ])` at `p`: a character that is not a name
    /// character or a backslash, the line feed included.
    pub(super) fn unqualified_at(&self, p: usize) -> bool {
        if p < self.lim() {
            return self.char_if(p, is_qualified_char).is_none();
        }
        self.avail(p)
    }

    /// Whether the character at `p` is `[_a-z\x7F-ÿ]` under `(?i)`.
    pub(super) fn name_starts(&self, p: usize) -> bool {
        self.char_if(p, is_name_start_char).is_some()
    }

    /// Whether the character at `p` is `[0-9\\_a-z\x7F-ÿ]` under `(?i)`.
    pub(super) fn qualified_at(&self, p: usize) -> bool {
        self.char_if(p, is_qualified_char).is_some()
    }

    /// Whether the character at `p` is `[0-9_a-z\x7F-ÿ]` under `(?i)`.
    pub(super) fn name_continues(&self, p: usize) -> bool {
        self.char_if(p, is_name_char).is_some()
    }

    /// The end of the run of `[0-9_a-z\x7F-ÿ]` under `(?i)` from `p`.
    pub(super) fn name_run_end(&self, p: usize) -> usize {
        self.run_end(p, is_name_char)
    }

    /// The end of the run of `[0-9_a-z]` under `(?i)` from `p`.
    pub(super) fn ascii_name_end(&self, p: usize) -> usize {
        self.run_end(p, is_ascii_name_char)
    }

    /// Whether the character at `p` is `[0-9\\_a-z]` under `(?i)`.
    pub(super) fn ascii_qualified_at(&self, p: usize) -> bool {
        self.char_if(p, |c| c == '\\' || is_ascii_name_char(c))
            .is_some()
    }

    /// Whether `pattern` matches at `p`, and where its match ends.
    pub(super) fn matcher(&mut self, pattern: Pattern, p: usize) -> Option<usize> {
        match pattern {
            Pattern::CommentsAt0 => self.doc_comment(p),
            Pattern::CommentsAt0End | Pattern::CommentsAt1End => self.lit(p, b"*/"),
            Pattern::PhpDocAt0 => self.doc_line(p),
            Pattern::PhpDocAt2 => self.doc_xlink(p),
            Pattern::PhpDocAt3End => (self.space_at(p) > 0 || self.starts(p, b"*/")).then_some(p),
            Pattern::PhpDocTypesArrayMultiple
            | Pattern::SwitchStatementAt1At0
            | Pattern::LanguageAt13At0
            | Pattern::LanguageAt49 => self.lit(p, b"("),
            Pattern::NamespaceAt0 => self.lit(p, b"\\"),
            Pattern::Nested1At2 => self.lit(p, b"|"),
            Pattern::PhpDocAt4 => self.php_doc_tag(p),
            Pattern::PhpDocAt5 => self.doc_inline_tag(p),
            Pattern::CommentsAt1 => self.lit(p, b"/*"),
            Pattern::CommentsAt2 => self.line_comment(p, b'/'),
            Pattern::CommentsAt2End
            | Pattern::CommentsAt3End
            | Pattern::HeredocAt0End
            | Pattern::HeredocAt1End => (!self.at_g(p)).then_some(p),
            Pattern::CommentsAt2At0 => self.lit(p, b"//"),
            Pattern::CommentsAt2At0End | Pattern::CommentsAt3At0End => self.comment_end(p),
            Pattern::CommentsAt3At0 => self.lit(p, b"#"),
            Pattern::Nested2At1 | Pattern::FunctionParametersAt1 => self.lit(p, b","),
            Pattern::LanguageAt4 => self.namespace_statement(p, false),
            Pattern::LanguageAt4End => (self.prev_is(p, b'}') || self.php_close(p)).then_some(p),
            Pattern::LanguageAt4At2
            | Pattern::LanguageAt6At1
            | Pattern::LanguageAt7At3
            | Pattern::LanguageAt47
            | Pattern::SwitchStatementAt1At1 => self.lit(p, b"{"),
            Pattern::LanguageAt4At2End
            | Pattern::LanguageAt7End
            | Pattern::SwitchStatementAt1End
            | Pattern::LanguageAt47End => self.close_or_php(p, b'}'),
            Pattern::LanguageAt5 => self.spaces_before(p, b"use"),
            Pattern::LanguageAt6 => self.ikeyword(p, b"use"),
            Pattern::LanguageAt6At0 => {
                let end = self.word_in(p, &[b"const", b"function"])?;
                self.capture(1, p, end);
                Some(end)
            }
            Pattern::LanguageAt6At1End
            | Pattern::InterpolationAt4End
            | Pattern::VariablesAt4End
            | Pattern::ObjectAt0End => self.lit(p, b"}"),
            Pattern::Nested4At0 => self.word_in(p, &[b"self", b"static", b"parent"]),
            Pattern::VarGlobal => self.global_variable(p),
            Pattern::VarGlobalSafer => self.superglobal(p),
            Pattern::ScopeResolutionAt1End
            | Pattern::SwitchStatementAt1At0End
            | Pattern::LanguageAt20End
            | Pattern::ObjectAt1End
            | Pattern::FunctionCallAt0End
            | Pattern::FunctionCallAt1End
            | Pattern::LanguageAt49End => self.close_or_php(p, b')'),
            Pattern::LanguageAt7At1 => self.word_then_spaces(p, b"extends"),
            Pattern::LanguageAt7At2 => self.word_then_spaces(p, b"implements"),
            Pattern::LanguageAt7At3End | Pattern::SwitchStatementAt1At1End => {
                (self.at(p) == b'}' || self.php_close(p)).then_some(p)
            }
            Pattern::SwitchStatementAt0 => self.spaces_before(p, b"switch"),
            Pattern::SwitchStatementAt1 => self.switch(p),
            Pattern::LanguageAt10End => (matches!(self.at(p), b';')
                || self.space_at(p) > 0
                || self.at_eol(p)
                || self.php_close(p))
            .then_some(p),
            Pattern::LanguageAt11 => self.catch(p),
            Pattern::LanguageAt11End
            | Pattern::LanguageAt13At0End
            | Pattern::FunctionParametersAt2End
            | Pattern::ParameterDefaultTypesAt7End
            | Pattern::LanguageAt13At1End => self.lit(p, b")"),
            Pattern::LanguageAt12 => self.exception_keyword(p),
            Pattern::RegexDoubleQuoted => self.quoted_regex_open(p, b'"'),
            Pattern::RegexDoubleQuotedEnd => self.regex_close(p, b'"'),
            Pattern::RegexDoubleQuotedAt0 => self.regex_escape(p),
            Pattern::InterpolationAt0 => self.octal_escape(p),
            Pattern::InterpolationAt1 => self.hex_escape(p),
            Pattern::InterpolationAt2 => self.unicode_escape(p),
            Pattern::InterpolationAt4 => self.brace_interpolation(p),
            Pattern::SqlStringDoubleQuotedEnd
            | Pattern::StringDoubleQuoted
            | Pattern::StringDoubleQuotedEnd => self.lit(p, b"\""),
            Pattern::SqlStringDoubleQuotedAt0 => self.sql_string_comment(p, b"#", b'"'),
            Pattern::SqlStringDoubleQuotedAt1 => self.sql_string_comment(p, b"--", b'"'),
            Pattern::SqlStringDoubleQuotedAt2 => self.sql_escape(p),
            Pattern::SqlStringDoubleQuotedAt3 => self.sql_quote(p, b'\'', b'"'),
            Pattern::SqlStringDoubleQuotedAt4 => self.sql_quote(p, b'`', b'"'),
            Pattern::SqlStringDoubleQuotedAt5
            | Pattern::SqlStringDoubleQuotedAt5End
            | Pattern::SqlStringSingleQuotedEnd
            | Pattern::StringSingleQuoted
            | Pattern::StringSingleQuotedEnd => self.lit(p, b"'"),
            Pattern::SqlStringDoubleQuotedAt6
            | Pattern::SqlStringDoubleQuotedAt6End
            | Pattern::StringBacktick
            | Pattern::StringBacktickEnd => self.lit(p, b"`"),
            Pattern::RegexSingleQuoted => self.quoted_regex_open(p, b'\''),
            Pattern::RegexSingleQuotedEnd => self.regex_close(p, b'\''),
            Pattern::SingleQuoteRegexEscape => self.single_quoted_regex_escape(p),
            Pattern::RegexSingleQuotedAt1 | Pattern::HeredocInteriorAt7At2 => self.regex_repeat(p),
            Pattern::RegexSingleQuotedAt2
            | Pattern::HeredocInteriorAt7At3
            | Pattern::RegexDoubleQuotedAt3 => self.regex_class_open(p),
            Pattern::RegexSingleQuotedAt2End
            | Pattern::HeredocInteriorAt7At3End
            | Pattern::RegexDoubleQuotedAt3End => self.lit(p, b"]"),
            Pattern::RegexSingleQuotedAt3 => self.regex_operator(p),
            Pattern::SqlStringSingleQuotedAt0 => self.sql_string_comment(p, b"#", b'\''),
            Pattern::SqlStringSingleQuotedAt1 => self.sql_string_comment(p, b"--", b'\''),
            Pattern::SqlStringSingleQuotedAt3 => self.sql_quote(p, b'`', b'\''),
            Pattern::SqlStringSingleQuotedAt4 => self.sql_quote(p, b'"', b'\''),
            Pattern::StringSingleQuotedAt0 => self.single_quote_escape(p),
            Pattern::VarLanguage => {
                let end = self.lit(p, b"$this")?;
                self.capture(1, p, p + 1);
                self.word_at(end).is_none().then_some(end)
            }
            Pattern::VariablesAt4 => self.dollar_brace(p),
            Pattern::ScopeResolutionAt2 => self.class_constant_name(p),
            Pattern::LanguageAt19 => self.empty_array(p),
            Pattern::LanguageAt25 => self.lit(p, b";"),
            Pattern::LanguageAt26 => self.lit(p, b":"),
            Pattern::HeredocInteriorAt0 => self.labelled_heredoc_open(p, &[b"HTML"]),
            Pattern::HeredocInteriorAt1 => self.labelled_heredoc_open(p, &[b"BLADE"]),
            Pattern::HeredocInteriorAt2 => self.labelled_heredoc_open(p, &[b"XML"]),
            Pattern::HeredocInteriorAt3 => self.labelled_heredoc_open(p, &[b"SQL"]),
            Pattern::HeredocInteriorAt4Javascript => {
                self.labelled_heredoc_open(p, &[b"JAVASCRIPT"])
            }
            Pattern::HeredocInteriorAt4Js => self.labelled_heredoc_open(p, &[b"JS"]),
            Pattern::HeredocInteriorAt5 => self.labelled_heredoc_open(p, &[b"JSON"]),
            Pattern::HeredocInteriorAt6 => self.labelled_heredoc_open(p, &[b"CSS"]),
            Pattern::HeredocInteriorAt7 => self.labelled_heredoc_open(p, &[b"REGEXP", b"REGEX"]),
            Pattern::HeredocInteriorAt7At3At0 => self.regex_class_escape(p),
            Pattern::HeredocInteriorAt7At5End => self.at_eol(p).then_some(p),
            Pattern::NowdocInteriorAt0 => self.nowdoc_open(p, &[b"HTML"]),
            Pattern::NowdocInteriorAt1 => self.nowdoc_open(p, &[b"BLADE"]),
            Pattern::NowdocInteriorAt2 => self.nowdoc_open(p, &[b"XML"]),
            Pattern::NowdocInteriorAt3 => self.nowdoc_open(p, &[b"SQL"]),
            Pattern::NowdocInteriorAt4Javascript => self.nowdoc_open(p, &[b"JAVASCRIPT"]),
            Pattern::NowdocInteriorAt4Js => self.nowdoc_open(p, &[b"JS"]),
            Pattern::NowdocInteriorAt5 => self.nowdoc_open(p, &[b"JSON"]),
            Pattern::NowdocInteriorAt6 => self.nowdoc_open(p, &[b"CSS"]),
            Pattern::NowdocInteriorAt7 => self.nowdoc_open(p, &[b"REGEXP", b"REGEX"]),
            Pattern::LanguageAt29 => self.ikeyword(p, b"clone"),
            Pattern::LanguageAt30 => self.lit(p, b".=").or_else(|| self.lit(p, b".")),
            Pattern::LanguageAt31 => self.lit(p, b"=>"),
            Pattern::LanguageAt32 => self.reference(p),
            Pattern::LanguageAt33 => self.lit(p, b"@"),
            Pattern::LanguageAt34 => self.equals_or_differs(p),
            Pattern::LanguageAt36 => self.relation(p),
            Pattern::LanguageAt37 => self.lit(p, b"--").or_else(|| self.lit(p, b"++")),
            Pattern::LanguageAt41 => self.bitwise(p),
            Pattern::LanguageAt42 => self.instanceof(p),
            Pattern::LanguageAt48 => self.lit(p, b"["),
            Pattern::LanguageAt48End => self.close_or_php(p, b']'),
            Pattern::ConstantsAt0 => self.iword_in(
                p,
                &[
                    b"true",
                    b"false",
                    b"null",
                    b"__file__",
                    b"__dir__",
                    b"__function__",
                    b"__class__",
                    b"__method__",
                    b"__line__",
                    b"__namespace__",
                    b"on",
                    b"off",
                    b"yes",
                    b"no",
                    b"nl",
                    b"br",
                    b"tab",
                ],
            ),
            Pattern::ParameterDefaultTypesAt5 | Pattern::FunctionParametersAt4At0 => {
                self.lit(p, b"=")
            }
            Pattern::ParameterDefaultTypesAt6 => {
                (self.at(p) == b'&' && self.at(self.space_end(p + 1)) == b'$').then_some(p + 1)
            }
            Pattern::ParameterDefaultTypesAt7 => self.array_paren(p),
            Pattern::LanguageAt13At1 => self.closure_use(p),
            Pattern::Nested12At1 => (p < self.lim()).then_some(self.lim()),
            Pattern::UseInnerAt1 => self.use_as(p),
            Pattern::LanguageAt4At3 => self.non_spaces(p),
            Pattern::PhpDocAt1 => self.doc_access(p),
            Pattern::PhpDocAt3 => self.doc_typed_tag(p),
            Pattern::PhpDocTypesArrayMultipleEnd => self.doc_types_close(p),
            Pattern::Nested1At0 => self.word_in(
                p,
                &[
                    b"string",
                    b"integer",
                    b"int",
                    b"boolean",
                    b"bool",
                    b"float",
                    b"double",
                    b"object",
                    b"mixed",
                    b"array",
                    b"resource",
                    b"void",
                    b"null",
                    b"callback",
                    b"false",
                    b"true",
                    b"self",
                ],
            ),
            Pattern::ClassNameAt0 => self.ahead_ascii_namespaced(p),
            Pattern::ClassNameAt0End | Pattern::ClassNameAt2End => self.ascii_last_segment(p),
            Pattern::Namespace => self.namespace_part(p),
            Pattern::NamespaceEnd => self.namespace_end(p),
            Pattern::ClassNameAt2 => self.ahead_class_start(p),
            Pattern::PhpDocTypes => self.doc_types(p),
            Pattern::PhpDocTypesArraySingle => self.doc_type_array(p),
            Pattern::CommentsAt3 => self.hash_comment(p),
            Pattern::LanguageAt1 => self.interface(p),
            Pattern::Nested2At0
            | Pattern::Nested5At0
            | Pattern::Nested8At1
            | Pattern::Nested11At0
            | Pattern::ConstantsAt5
            | Pattern::UseInnerAt1End => self.bname(p),
            Pattern::LanguageAt1End => self.extended_interfaces(p),
            Pattern::LanguageAt2 => self.trait_declaration(p),
            Pattern::LanguageAt2End | Pattern::LanguageAt13End => self.ahead_brace(p),
            Pattern::LanguageAt3 => self.namespace_line(p),
            Pattern::LanguageAt4At1 => self.qualified_name(p),
            Pattern::LanguageAt6End => self.after_brace_or_semicolon(p),
            Pattern::Nested4At1 => {
                let end = self.word_end(p);
                (end > p).then_some(end)
            }
            Pattern::VariableNameAt2 => self.interpolated_variable(p),
            Pattern::VariableNameAt3 => self.braced_variable(p),
            Pattern::ScopeResolutionAt0 => self.class_before_scope(p),
            Pattern::ScopeResolutionAt1 => self.static_call(p),
            Pattern::LanguageAt7 => self.class_declaration(p),
            Pattern::LanguageAt7At1End | Pattern::InstantiationEnd => self.ahead_unqualified(p),
            Pattern::LanguageAt7At1At0 => self.ahead_namespaced(p),
            Pattern::LanguageAt7At1At0End => self.last_segment(p),
            Pattern::LanguageAt7At2End => self.ahead_body(p),
            Pattern::LanguageAt7At2At1 => self.ahead_qualified(p),
            Pattern::LanguageAt7At2At1End => self.implemented_end(p),
            Pattern::LanguageAt9 => self.control_keyword(p),
            Pattern::LanguageAt10 => self.include(p),
            Pattern::LanguageAt11At1 => self.caught(p),
            Pattern::LanguageAt13 => self.closure(p),
            Pattern::FunctionParametersAt2 => self.array_default_parameter(p),
            Pattern::InterpolationAt3 => self.string_escape(p),
            Pattern::Nested6At0 => [
                b"final".as_slice(),
                b"abstract",
                b"public",
                b"private",
                b"protected",
                b"static",
            ]
            .iter()
            .find_map(|word| self.lit(p, word)),
            Pattern::LanguageAt14 => self.method(p),
            Pattern::LanguageAt14End => self.parameters_close(p),
            Pattern::SqlStringDoubleQuoted => self.sql_open(p, b'"'),
            Pattern::SqlStringSingleQuoted => self.sql_open(p, b'\''),
            Pattern::NumbersAt0 => self.radix(p, b"xX", |b| b.is_ascii_hexdigit()),
            Pattern::NumbersAt1 => self.radix(p, b"bB", |b| matches!(b, b'0' | b'1')),
            Pattern::NumbersAt2 => self.octal(p),
            Pattern::NumbersAt3 => self.decimal(p),
            Pattern::NumbersAt4 => self.integer(p),
            Pattern::StringBacktickAt0 => self.any_escape(p),
            Pattern::VarBasic => self.variable(p),
            Pattern::InvokeCall => self.invoke(p),
            Pattern::ScopeResolutionAt3 => self.static_access(p),
            Pattern::LanguageAt20 => self.array_call(p),
            Pattern::LanguageAt21 => self.cast(p),
            Pattern::LanguageAt22 => self.iword_in(
                p,
                &[
                    b"array",
                    b"real",
                    b"double",
                    b"float",
                    b"int",
                    b"integer",
                    b"bool",
                    b"boolean",
                    b"string",
                    b"class",
                    b"var",
                    b"function",
                    b"interface",
                    b"trait",
                    b"parent",
                    b"self",
                    b"object",
                ],
            ),
            Pattern::LanguageAt23 => self.iword_in(
                p,
                &[
                    b"global",
                    b"abstract",
                    b"const",
                    b"extends",
                    b"implements",
                    b"final",
                    b"private",
                    b"protected",
                    b"public",
                    b"static",
                ],
            ),
            Pattern::ObjectAt0 => self.object_brace(p),
            Pattern::ObjectAt1 => self.method_call(p),
            Pattern::ObjectAt2 => self.property_access(p),
            Pattern::HeredocAt0 => self.heredoc_ahead(p),
            Pattern::HeredocAt1 => self.nowdoc_ahead(p),
            Pattern::HeredocInteriorAt0End | Pattern::NowdocInteriorAt0End => {
                self.label_line(p, Some(b"HTML"))
            }
            Pattern::HeredocInteriorAt1End | Pattern::NowdocInteriorAt1End => {
                self.label_line(p, Some(b"BLADE"))
            }
            Pattern::HeredocInteriorAt2End | Pattern::NowdocInteriorAt2End => {
                self.label_line(p, Some(b"XML"))
            }
            Pattern::HeredocInteriorAt3End | Pattern::NowdocInteriorAt3End => {
                self.label_line(p, Some(b"SQL"))
            }
            Pattern::HeredocInteriorAt4JavascriptEnd | Pattern::NowdocInteriorAt4JavascriptEnd => {
                self.label_line(p, Some(b"JAVASCRIPT"))
            }
            Pattern::HeredocInteriorAt4JsEnd | Pattern::NowdocInteriorAt4JsEnd => {
                self.label_line(p, Some(b"JS"))
            }
            Pattern::HeredocInteriorAt5End | Pattern::NowdocInteriorAt5End => {
                self.label_line(p, Some(b"JSON"))
            }
            Pattern::HeredocInteriorAt6End | Pattern::NowdocInteriorAt6End => {
                self.label_line(p, Some(b"CSS"))
            }
            Pattern::HeredocInteriorAt7End
            | Pattern::HeredocInteriorAt8End
            | Pattern::NowdocInteriorAt7End
            | Pattern::NowdocInteriorAt8End => self.label_line(p, None),
            Pattern::HeredocInteriorAt7At5 => self.regex_comment(p),
            Pattern::HeredocInteriorAt8 => self.heredoc_open_any(p),
            Pattern::NowdocInteriorAt8 => self.nowdoc_open_any(p),
            Pattern::LanguageAt35 => self.assignment(p),
            Pattern::LanguageAt38 => self.arithmetic(p),
            Pattern::LanguageAt39 => self.logical(p),
            Pattern::FunctionCallAt0 => self.qualified_call(p),
            Pattern::SupportAt1 => {
                self.iword_in(p, &[b"isset", b"unset", b"eval", b"empty", b"list"])
            }
            Pattern::SupportAt1End => self.never_end(p),
            Pattern::FunctionCallAt1 => self.call(p),
            Pattern::LanguageAt42End => self.not_lower_name_at(p),
            Pattern::Instantiation => self.new_keyword(p),
            Pattern::InstantiationAt0 => self.class_keyword(p),
            Pattern::LanguageAt44 => self.goto(p),
            Pattern::LanguageAt45 => self.label(p),
            Pattern::ParameterDefaultTypesAt9 => self.ahead_scope(p),
            Pattern::ParameterDefaultTypesAt9End => self.scope_member(p),
            Pattern::FunctionParametersAt3 => self.array_parameter(p),
            Pattern::FunctionParametersAt4 => self.typed_parameter(p),
            Pattern::FunctionParametersAt4End
            | Pattern::FunctionParametersAt4At0End
            | Pattern::FunctionParametersAt6End => self.parameter_ends(p),
            Pattern::FunctionParametersAt5 => self.untyped_parameter(p),
            Pattern::FunctionParametersAt6 => self.defaulted_parameter(p),
            Pattern::LanguageAt13At1At0 => self.closure_parameter(p),
            Pattern::LanguageAt6At1At1 => self.use_as_modifier(p),
            Pattern::Nested12At0 => self.modifier_alone(p),
            Pattern::LanguageAt6At1At2 => self.use_as_name(p),
            Pattern::LanguageAt6At1At3 => self.insteadof(p),
            table_patterns!() => {
                let (table, list) = pattern.words()?;
                self.table_list(table.words(), list, p)
            }
        }
    }
}
