//! The matchers of Blade's PHP: for each regular expression of the grammar,
//! the function that decides whether it matches at a position and where.
//!
//! Where an expression is PHP's word for word, the arm calls PHP's matcher in
//! [`crate::php_family`].
//! Blade's copy of PHP is older than Shiki's `php` grammar.
//! Its name class is `[_a-z\x7F-ÿ]` under `(?i)`, which ends at U+00FF, so
//! Blade's names are read by character, never by PHP's byte test.

use super::engine::Lexer;
use super::grammar::{Pat, table_pats};
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

    /// The end of the run of characters from `p` for which `class` holds.
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

    /// Whether `pat` matches at `p`, and where its match ends.
    pub(super) fn matcher(&mut self, pat: Pat, p: usize) -> Option<usize> {
        match pat {
            Pat::CommentsAt0 => self.doc_comment(p),
            Pat::CommentsAt0End | Pat::CommentsAt1End => self.lit(p, b"*/"),
            Pat::PhpDocAt0 => self.doc_line(p),
            Pat::PhpDocAt2 => self.doc_xlink(p),
            Pat::PhpDocAt3End => (self.space_at(p) > 0 || self.starts(p, b"*/")).then_some(p),
            Pat::PhpDocTypesArrayMultiple
            | Pat::SwitchStatementAt1At0
            | Pat::LanguageAt13At0
            | Pat::LanguageAt49 => self.lit(p, b"("),
            Pat::NamespaceAt0 => self.lit(p, b"\\"),
            Pat::Nested1At2 => self.lit(p, b"|"),
            Pat::PhpDocAt4 => self.php_doc_tag(p),
            Pat::PhpDocAt5 => self.doc_inline_tag(p),
            Pat::CommentsAt1 => self.lit(p, b"/*"),
            Pat::CommentsAt2 => self.line_comment(p, b'/'),
            Pat::CommentsAt2End | Pat::CommentsAt3End | Pat::HeredocAt0End | Pat::HeredocAt1End => {
                (!self.at_g(p)).then_some(p)
            }
            Pat::CommentsAt2At0 => self.lit(p, b"//"),
            Pat::CommentsAt2At0End | Pat::CommentsAt3At0End => self.comment_end(p),
            Pat::CommentsAt3At0 => self.lit(p, b"#"),
            Pat::Nested2At1 | Pat::FunctionParametersAt1 => self.lit(p, b","),
            Pat::LanguageAt4 => self.namespace_statement(p, false),
            Pat::LanguageAt4End => (self.prev_is(p, b'}') || self.php_close(p)).then_some(p),
            Pat::LanguageAt4At2
            | Pat::LanguageAt6At1
            | Pat::LanguageAt7At3
            | Pat::LanguageAt47
            | Pat::SwitchStatementAt1At1 => self.lit(p, b"{"),
            Pat::LanguageAt4At2End
            | Pat::LanguageAt7End
            | Pat::SwitchStatementAt1End
            | Pat::LanguageAt47End => self.close_or_php(p, b'}'),
            Pat::LanguageAt5 => self.spaces_before(p, b"use"),
            Pat::LanguageAt6 => self.ikeyword(p, b"use"),
            Pat::LanguageAt6At0 => {
                let end = self.word_in(p, &[b"const", b"function"])?;
                self.cap(1, p, end);
                Some(end)
            }
            Pat::LanguageAt6At1End
            | Pat::InterpolationAt4End
            | Pat::VariablesAt4End
            | Pat::ObjectAt0End => self.lit(p, b"}"),
            Pat::Nested4At0 => self.word_in(p, &[b"self", b"static", b"parent"]),
            Pat::VarGlobal => self.global_variable(p),
            Pat::VarGlobalSafer => self.superglobal(p),
            Pat::ScopeResolutionAt1End
            | Pat::SwitchStatementAt1At0End
            | Pat::LanguageAt20End
            | Pat::ObjectAt1End
            | Pat::FunctionCallAt0End
            | Pat::FunctionCallAt1End
            | Pat::LanguageAt49End => self.close_or_php(p, b')'),
            Pat::LanguageAt7At1 => self.word_then_spaces(p, b"extends"),
            Pat::LanguageAt7At2 => self.word_then_spaces(p, b"implements"),
            Pat::LanguageAt7At3End | Pat::SwitchStatementAt1At1End => {
                (self.at(p) == b'}' || self.php_close(p)).then_some(p)
            }
            Pat::SwitchStatementAt0 => self.spaces_before(p, b"switch"),
            Pat::SwitchStatementAt1 => self.switch(p),
            Pat::LanguageAt10End => (matches!(self.at(p), b';')
                || self.space_at(p) > 0
                || self.at_eol(p)
                || self.php_close(p))
            .then_some(p),
            Pat::LanguageAt11 => self.catch(p),
            Pat::LanguageAt11End
            | Pat::LanguageAt13At0End
            | Pat::FunctionParametersAt2End
            | Pat::ParameterDefaultTypesAt7End
            | Pat::LanguageAt13At1End => self.lit(p, b")"),
            Pat::LanguageAt12 => self.exception_keyword(p),
            Pat::RegexDoubleQuoted => self.quoted_regex_open(p, b'"'),
            Pat::RegexDoubleQuotedEnd => self.regex_close(p, b'"'),
            Pat::RegexDoubleQuotedAt0 => self.regex_escape(p),
            Pat::InterpolationAt0 => self.octal_escape(p),
            Pat::InterpolationAt1 => self.hex_escape(p),
            Pat::InterpolationAt2 => self.unicode_escape(p),
            Pat::InterpolationAt4 => self.brace_interpolation(p),
            Pat::SqlStringDoubleQuotedEnd
            | Pat::StringDoubleQuoted
            | Pat::StringDoubleQuotedEnd => self.lit(p, b"\""),
            Pat::SqlStringDoubleQuotedAt0 => self.sql_string_comment(p, b"#", b'"'),
            Pat::SqlStringDoubleQuotedAt1 => self.sql_string_comment(p, b"--", b'"'),
            Pat::SqlStringDoubleQuotedAt2 => self.sql_escape(p),
            Pat::SqlStringDoubleQuotedAt3 => self.sql_quote(p, b'\'', b'"'),
            Pat::SqlStringDoubleQuotedAt4 => self.sql_quote(p, b'`', b'"'),
            Pat::SqlStringDoubleQuotedAt5
            | Pat::SqlStringDoubleQuotedAt5End
            | Pat::SqlStringSingleQuotedEnd
            | Pat::StringSingleQuoted
            | Pat::StringSingleQuotedEnd => self.lit(p, b"'"),
            Pat::SqlStringDoubleQuotedAt6
            | Pat::SqlStringDoubleQuotedAt6End
            | Pat::StringBacktick
            | Pat::StringBacktickEnd => self.lit(p, b"`"),
            Pat::RegexSingleQuoted => self.quoted_regex_open(p, b'\''),
            Pat::RegexSingleQuotedEnd => self.regex_close(p, b'\''),
            Pat::SingleQuoteRegexEscape => self.single_quoted_regex_escape(p),
            Pat::RegexSingleQuotedAt1 | Pat::HeredocInteriorAt7At2 => self.regex_repeat(p),
            Pat::RegexSingleQuotedAt2 | Pat::HeredocInteriorAt7At3 | Pat::RegexDoubleQuotedAt3 => {
                self.regex_class_open(p)
            }
            Pat::RegexSingleQuotedAt2End
            | Pat::HeredocInteriorAt7At3End
            | Pat::RegexDoubleQuotedAt3End => self.lit(p, b"]"),
            Pat::RegexSingleQuotedAt3 => self.regex_operator(p),
            Pat::SqlStringSingleQuotedAt0 => self.sql_string_comment(p, b"#", b'\''),
            Pat::SqlStringSingleQuotedAt1 => self.sql_string_comment(p, b"--", b'\''),
            Pat::SqlStringSingleQuotedAt3 => self.sql_quote(p, b'`', b'\''),
            Pat::SqlStringSingleQuotedAt4 => self.sql_quote(p, b'"', b'\''),
            Pat::StringSingleQuotedAt0 => self.single_quote_escape(p),
            Pat::VarLanguage => {
                let end = self.lit(p, b"$this")?;
                self.cap(1, p, p + 1);
                self.word_at(end).is_none().then_some(end)
            }
            Pat::VariablesAt4 => self.dollar_brace(p),
            Pat::ScopeResolutionAt2 => self.class_constant_name(p),
            Pat::LanguageAt19 => self.empty_array(p),
            Pat::LanguageAt25 => self.lit(p, b";"),
            Pat::LanguageAt26 => self.lit(p, b":"),
            Pat::HeredocInteriorAt0 => self.labelled_heredoc_open(p, &[b"HTML"]),
            Pat::HeredocInteriorAt1 => self.labelled_heredoc_open(p, &[b"BLADE"]),
            Pat::HeredocInteriorAt2 => self.labelled_heredoc_open(p, &[b"XML"]),
            Pat::HeredocInteriorAt3 => self.labelled_heredoc_open(p, &[b"SQL"]),
            Pat::HeredocInteriorAt4Javascript => self.labelled_heredoc_open(p, &[b"JAVASCRIPT"]),
            Pat::HeredocInteriorAt4Js => self.labelled_heredoc_open(p, &[b"JS"]),
            Pat::HeredocInteriorAt5 => self.labelled_heredoc_open(p, &[b"JSON"]),
            Pat::HeredocInteriorAt6 => self.labelled_heredoc_open(p, &[b"CSS"]),
            Pat::HeredocInteriorAt7 => self.labelled_heredoc_open(p, &[b"REGEXP", b"REGEX"]),
            Pat::HeredocInteriorAt7At3At0 => self.regex_class_escape(p),
            Pat::HeredocInteriorAt7At5End => self.at_eol(p).then_some(p),
            Pat::NowdocInteriorAt0 => self.nowdoc_open(p, &[b"HTML"]),
            Pat::NowdocInteriorAt1 => self.nowdoc_open(p, &[b"BLADE"]),
            Pat::NowdocInteriorAt2 => self.nowdoc_open(p, &[b"XML"]),
            Pat::NowdocInteriorAt3 => self.nowdoc_open(p, &[b"SQL"]),
            Pat::NowdocInteriorAt4Javascript => self.nowdoc_open(p, &[b"JAVASCRIPT"]),
            Pat::NowdocInteriorAt4Js => self.nowdoc_open(p, &[b"JS"]),
            Pat::NowdocInteriorAt5 => self.nowdoc_open(p, &[b"JSON"]),
            Pat::NowdocInteriorAt6 => self.nowdoc_open(p, &[b"CSS"]),
            Pat::NowdocInteriorAt7 => self.nowdoc_open(p, &[b"REGEXP", b"REGEX"]),
            Pat::LanguageAt29 => self.ikeyword(p, b"clone"),
            Pat::LanguageAt30 => self.lit(p, b".=").or_else(|| self.lit(p, b".")),
            Pat::LanguageAt31 => self.lit(p, b"=>"),
            Pat::LanguageAt32 => self.reference(p),
            Pat::LanguageAt33 => self.lit(p, b"@"),
            Pat::LanguageAt34 => self.equals_or_differs(p),
            Pat::LanguageAt36 => self.relation(p),
            Pat::LanguageAt37 => self.lit(p, b"--").or_else(|| self.lit(p, b"++")),
            Pat::LanguageAt41 => self.bitwise(p),
            Pat::LanguageAt42 => self.instanceof(p),
            Pat::LanguageAt48 => self.lit(p, b"["),
            Pat::LanguageAt48End => self.close_or_php(p, b']'),
            Pat::ConstantsAt0 => self.iword_in(
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
            Pat::ParameterDefaultTypesAt5 | Pat::FunctionParametersAt4At0 => self.lit(p, b"="),
            Pat::ParameterDefaultTypesAt6 => {
                (self.at(p) == b'&' && self.at(self.space_end(p + 1)) == b'$').then_some(p + 1)
            }
            Pat::ParameterDefaultTypesAt7 => self.array_paren(p),
            Pat::LanguageAt13At1 => self.closure_use(p),
            Pat::Nested12At1 => (p < self.lim()).then_some(self.lim()),
            Pat::UseInnerAt1 => self.use_as(p),
            Pat::LanguageAt4At3 => self.non_spaces(p),
            Pat::PhpDocAt1 => self.doc_access(p),
            Pat::PhpDocAt3 => self.doc_typed_tag(p),
            Pat::PhpDocTypesArrayMultipleEnd => self.doc_types_close(p),
            Pat::Nested1At0 => self.word_in(
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
            Pat::ClassNameAt0 => self.ahead_ascii_namespaced(p),
            Pat::ClassNameAt0End | Pat::ClassNameAt2End => self.ascii_last_segment(p),
            Pat::Namespace => self.namespace_part(p),
            Pat::NamespaceEnd => self.namespace_end(p),
            Pat::ClassNameAt2 => self.ahead_class_start(p),
            Pat::PhpDocTypes => self.doc_types(p),
            Pat::PhpDocTypesArraySingle => self.doc_type_array(p),
            Pat::CommentsAt3 => self.hash_comment(p),
            Pat::LanguageAt1 => self.interface(p),
            Pat::Nested2At0
            | Pat::Nested5At0
            | Pat::Nested8At1
            | Pat::Nested11At0
            | Pat::ConstantsAt5
            | Pat::UseInnerAt1End => self.bname(p),
            Pat::LanguageAt1End => self.extended_interfaces(p),
            Pat::LanguageAt2 => self.trait_declaration(p),
            Pat::LanguageAt2End | Pat::LanguageAt13End => self.ahead_brace(p),
            Pat::LanguageAt3 => self.namespace_line(p),
            Pat::LanguageAt4At1 => self.qualified_name(p),
            Pat::LanguageAt6End => self.after_brace_or_semicolon(p),
            Pat::Nested4At1 => {
                let end = self.word_end(p);
                (end > p).then_some(end)
            }
            Pat::VariableNameAt2 => self.interpolated_variable(p),
            Pat::VariableNameAt3 => self.braced_variable(p),
            Pat::ScopeResolutionAt0 => self.class_before_scope(p),
            Pat::ScopeResolutionAt1 => self.static_call(p),
            Pat::LanguageAt7 => self.class_declaration(p),
            Pat::LanguageAt7At1End | Pat::InstantiationEnd => self.ahead_unqualified(p),
            Pat::LanguageAt7At1At0 => self.ahead_namespaced(p),
            Pat::LanguageAt7At1At0End => self.last_segment(p),
            Pat::LanguageAt7At2End => self.ahead_body(p),
            Pat::LanguageAt7At2At1 => self.ahead_qualified(p),
            Pat::LanguageAt7At2At1End => self.implemented_end(p),
            Pat::LanguageAt9 => self.control_keyword(p),
            Pat::LanguageAt10 => self.include(p),
            Pat::LanguageAt11At1 => self.caught(p),
            Pat::LanguageAt13 => self.closure(p),
            Pat::FunctionParametersAt2 => self.array_default_parameter(p),
            Pat::InterpolationAt3 => self.string_escape(p),
            Pat::Nested6At0 => [
                b"final".as_slice(),
                b"abstract",
                b"public",
                b"private",
                b"protected",
                b"static",
            ]
            .iter()
            .find_map(|word| self.lit(p, word)),
            Pat::LanguageAt14 => self.method(p),
            Pat::LanguageAt14End => self.parameters_close(p),
            Pat::SqlStringDoubleQuoted => self.sql_open(p, b'"'),
            Pat::SqlStringSingleQuoted => self.sql_open(p, b'\''),
            Pat::NumbersAt0 => self.radix(p, b"xX", |b| b.is_ascii_hexdigit()),
            Pat::NumbersAt1 => self.radix(p, b"bB", |b| matches!(b, b'0' | b'1')),
            Pat::NumbersAt2 => self.octal(p),
            Pat::NumbersAt3 => self.decimal(p),
            Pat::NumbersAt4 => self.integer(p),
            Pat::StringBacktickAt0 => self.any_escape(p),
            Pat::VarBasic => self.variable(p),
            Pat::InvokeCall => self.invoke(p),
            Pat::ScopeResolutionAt3 => self.static_access(p),
            Pat::LanguageAt20 => self.array_call(p),
            Pat::LanguageAt21 => self.cast(p),
            Pat::LanguageAt22 => self.iword_in(
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
            Pat::LanguageAt23 => self.iword_in(
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
            Pat::ObjectAt0 => self.object_brace(p),
            Pat::ObjectAt1 => self.method_call(p),
            Pat::ObjectAt2 => self.property_access(p),
            Pat::HeredocAt0 => self.heredoc_ahead(p),
            Pat::HeredocAt1 => self.nowdoc_ahead(p),
            Pat::HeredocInteriorAt0End | Pat::NowdocInteriorAt0End => {
                self.label_line(p, Some(b"HTML"))
            }
            Pat::HeredocInteriorAt1End | Pat::NowdocInteriorAt1End => {
                self.label_line(p, Some(b"BLADE"))
            }
            Pat::HeredocInteriorAt2End | Pat::NowdocInteriorAt2End => {
                self.label_line(p, Some(b"XML"))
            }
            Pat::HeredocInteriorAt3End | Pat::NowdocInteriorAt3End => {
                self.label_line(p, Some(b"SQL"))
            }
            Pat::HeredocInteriorAt4JavascriptEnd | Pat::NowdocInteriorAt4JavascriptEnd => {
                self.label_line(p, Some(b"JAVASCRIPT"))
            }
            Pat::HeredocInteriorAt4JsEnd | Pat::NowdocInteriorAt4JsEnd => {
                self.label_line(p, Some(b"JS"))
            }
            Pat::HeredocInteriorAt5End | Pat::NowdocInteriorAt5End => {
                self.label_line(p, Some(b"JSON"))
            }
            Pat::HeredocInteriorAt6End | Pat::NowdocInteriorAt6End => {
                self.label_line(p, Some(b"CSS"))
            }
            Pat::HeredocInteriorAt7End
            | Pat::HeredocInteriorAt8End
            | Pat::NowdocInteriorAt7End
            | Pat::NowdocInteriorAt8End => self.label_line(p, None),
            Pat::HeredocInteriorAt7At5 => self.regex_comment(p),
            Pat::HeredocInteriorAt8 => self.heredoc_open_any(p),
            Pat::NowdocInteriorAt8 => self.nowdoc_open_any(p),
            Pat::LanguageAt35 => self.assignment(p),
            Pat::LanguageAt38 => self.arithmetic(p),
            Pat::LanguageAt39 => self.logical(p),
            Pat::FunctionCallAt0 => self.qualified_call(p),
            Pat::SupportAt1 => self.iword_in(p, &[b"isset", b"unset", b"eval", b"empty", b"list"]),
            Pat::SupportAt1End => self.never_end(p),
            Pat::FunctionCallAt1 => self.call(p),
            Pat::LanguageAt42End => self.not_lower_name_at(p),
            Pat::Instantiation => self.new_keyword(p),
            Pat::InstantiationAt0 => self.class_keyword(p),
            Pat::LanguageAt44 => self.goto(p),
            Pat::LanguageAt45 => self.label(p),
            Pat::ParameterDefaultTypesAt9 => self.ahead_scope(p),
            Pat::ParameterDefaultTypesAt9End => self.scope_member(p),
            Pat::FunctionParametersAt3 => self.array_parameter(p),
            Pat::FunctionParametersAt4 => self.typed_parameter(p),
            Pat::FunctionParametersAt4End
            | Pat::FunctionParametersAt4At0End
            | Pat::FunctionParametersAt6End => self.parameter_ends(p),
            Pat::FunctionParametersAt5 => self.untyped_parameter(p),
            Pat::FunctionParametersAt6 => self.defaulted_parameter(p),
            Pat::LanguageAt13At1At0 => self.closure_parameter(p),
            Pat::LanguageAt6At1At1 => self.use_as_modifier(p),
            Pat::Nested12At0 => self.modifier_alone(p),
            Pat::LanguageAt6At1At2 => self.use_as_name(p),
            Pat::LanguageAt6At1At3 => self.insteadof(p),
            table_pats!() => {
                let (table, list) = pat.words()?;
                self.table_list(table.words(), list, p)
            }
        }
    }
}
