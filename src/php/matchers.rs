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
//! A regular expression sees the line feed at the end of a line.
//! So `\s` and a negated class such as `[^"]` match it there, and a match can
//! end one past the text.
//! `(?i)` compares words through [`crate::case_fold`], so `ſ` and the Kelvin
//! sign match `s` and `k`.
//!
//! The grammar's name class, `[_a-z\x7F-\x{10FFFF}]` under `(?i)`, holds every
//! character past ASCII.
//! So a name is read by byte: every byte of such a character is at least
//! `0x80`.
//!
//! A space past ASCII, such as U+00A0, is both a name character and `\s`.
//! Where a `\s` follows a name, the name gives such spaces back
//! ([`Lexer::q_ends`]). A `\s*` before a name does not give them back for the
//! name to start with.

mod comments;
mod declarations;
mod expressions;
mod strings;

use super::engine::Lexer;
use super::grammar::{Pat, table_pats};
use crate::classes::is_space;
use crate::php_family::matchers::{is_name, is_name_start, is_qualified};
use crate::rules::Chars;

impl Lexer<'_> {
    /// Whether `pat` matches at `p`, and where its match ends.
    ///
    /// The arms come in groups, in this order:
    /// - attributes, namespaces and names;
    /// - comments and doc comments;
    /// - declarations: `namespace`, `use`, classes, traits, interfaces, enums,
    ///   functions and their parameters;
    /// - variables, objects, calls and the other expressions;
    /// - strings, regular expressions, SQL strings, heredocs and nowdocs;
    /// - the word tables.
    #[expect(clippy::too_many_lines, reason = "one arm per pattern of the grammar")]
    pub(super) fn matcher(&mut self, pat: Pat, p: usize) -> Option<usize> {
        match pat {
            Pat::Attribute => self.lit(p, b"#["),
            Pat::AttributeEnd
            | Pat::RegexDoubleQuotedAt3End
            | Pat::RegexSingleQuotedAt2End
            | Pat::HeredocInteriorAt6At3End => self.lit(p, b"]"),
            Pat::AttributeAt0 | Pat::InterfaceExtendsAt1 => self.lit(p, b","),
            Pat::AttributeNameAt0 | Pat::ClassNameAt0 | Pat::InheritanceSingleAt0 => {
                self.ahead_namespaced(p)
            }
            Pat::AttributeNameAt0End
            | Pat::AttributeNameAt2End
            | Pat::ClassNameAt0End
            | Pat::ClassNameAt2End => self.last_segment(p),
            Pat::Namespace => self.namespace_part(p),
            Pat::NamespaceEnd => self.namespace_end(p),
            Pat::NamespaceAt0 => self.lit(p, b"\\"),
            Pat::AttributeNameAt1 => self.attribute_name(p),
            Pat::AttributeNameAt2 | Pat::ClassNameAt2 | Pat::Nested8At1 => {
                (p < self.lim() && (self.at(p) == b'\\' || is_name_start(self.at(p)))).then_some(p)
            }
            Pat::AttributeAt1 => self.attribute_call(p),
            Pat::NamedArguments => self.named_argument(p),
            Pat::InheritanceSingleAt0End => self.last_segment_before_other(p),
            Pat::InheritanceSingleAt3
            | Pat::Nested14At1
            | Pat::ConstantsAt5
            | Pat::UseInnerAt1End => self.name(p),
            Pat::RootAt3At1 => self.qualified(p),
            Pat::Nested8At1End => self.name_alone(p),

            Pat::CommentsAt0 => self.doc_comment(p),
            Pat::CommentsAt0End | Pat::CommentsAt1End => self.lit(p, b"*/"),
            Pat::CommentsAt1 => self.lit(p, b"/*"),
            Pat::CommentsAt2 => self.line_comment(p, b'/'),
            Pat::CommentsAt3 => self.line_comment(p, b'#'),
            Pat::CommentsAt2End | Pat::CommentsAt3End | Pat::HeredocAt0End | Pat::HeredocAt1End => {
                (!self.at_g(p)).then_some(p)
            }
            Pat::CommentsAt2At0 => self.lit(p, b"//"),
            Pat::CommentsAt3At0 => self.lit(p, b"#"),
            Pat::CommentsAt2At0End | Pat::CommentsAt3At0End => self.comment_end(p),
            Pat::PhpDocAt0 => self.doc_line(p),
            Pat::PhpDocAt1 => self.doc_access(p),
            Pat::PhpDocAt2 => self.doc_xlink(p),
            Pat::PhpDocAt3 => self.doc_typed_tag(p),
            Pat::PhpDocAt3End => (self.space_at(p) > 0 || self.starts(p, b"*/")).then_some(p),
            Pat::PhpDocAt4 => self.php_doc_tag(p),
            Pat::PhpDocAt5 => self.doc_inline_tag(p),
            Pat::PhpDocTypesArrayMultiple
            | Pat::PhpTypesAt4
            | Pat::RootAt62
            | Pat::MatchStatementAt1At0
            | Pat::SwitchStatementAt1At0 => self.lit(p, b"("),
            Pat::PhpDocTypesArrayMultipleEnd => self.doc_types_close(p),
            Pat::PhpDocTypes => self.doc_types(p),
            Pat::PhpDocTypesArraySingle => self.doc_type_array(p),
            Pat::Nested2At0 | Pat::TernaryExpression => self.lit(p, b"?"),
            Pat::Nested2At1 => self.word_in(
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
                    b"static",
                ],
            ),
            Pat::Nested2At3 => matches!(self.at(p), b'&' | b'|').then_some(p + 1),

            Pat::RootAt2 => self.namespace_statement(p, true),
            Pat::RootAt3 => self.namespace_statement(p, false),
            Pat::RootAt3End => (self.prev_is(p, b'}') || self.php_close(p)).then_some(p),
            Pat::RootAt3At2
            | Pat::RootAt5At1
            | Pat::RootAt6At1
            | Pat::RootAt7At2
            | Pat::RootAt8At2
            | Pat::RootAt9At4
            | Pat::RootAt60
            | Pat::MatchStatementAt1At1
            | Pat::SwitchStatementAt1At1 => self.lit(p, b"{"),
            Pat::RootAt3At2End
            | Pat::RootAt6End
            | Pat::RootAt7End
            | Pat::RootAt8End
            | Pat::RootAt9End
            | Pat::RootAt60End
            | Pat::MatchStatementAt1End
            | Pat::SwitchStatementAt1End => self.close_or_php(p, b'}'),
            Pat::RootAt3At3 => self.non_spaces(p),
            Pat::RootAt4 => self.spaces_before(p, b"use"),
            Pat::MatchStatementAt0 => self.spaces_before(p, b"match"),
            Pat::SwitchStatementAt0 => self.spaces_before(p, b"switch"),
            Pat::RootAt5 => self.ikeyword(p, b"use"),
            Pat::RootAt5End => {
                (self.prev_is(p, b'}') || self.at(p) == b';' || self.php_close(p)).then_some(p)
            }
            Pat::RootAt5At0 => {
                let end = self.word_in(p, &[b"const", b"function"])?;
                self.cap(1, p, end);
                Some(end)
            }
            Pat::RootAt5At1End
            | Pat::ObjectAt0End
            | Pat::InterpolationAt4End
            | Pat::VariablesAt4End => self.lit(p, b"}"),
            Pat::RootAt5At1At1 => self.trait_alias_modifier(p),
            Pat::Nested19At0 => self.storage_modifier_of(
                p,
                &[
                    (b"final", 1),
                    (b"abstract", 1),
                    (b"public", 2),
                    (b"private", 2),
                    (b"protected", 2),
                    (b"static", 3),
                ],
            ),
            Pat::Nested19At1 => (p < self.lim()).then_some(self.lim()),
            Pat::RootAt5At1At2 => self.keyword_name(p, b"as"),
            Pat::RootAt5At1At3 => self.keyword_name(p, b"insteadof"),
            Pat::UseInnerAt1 => self.use_as(p),
            Pat::Nested4At0 => self.word_in(p, &[b"self", b"static", b"parent"]),

            Pat::RootAt6 => self.keyword_name(p, b"trait"),
            Pat::RootAt7 => self.keyword_name(p, b"interface"),
            Pat::RootAt6At1End
            | Pat::RootAt7At2End
            | Pat::RootAt8At2End
            | Pat::RootAt9At4End
            | Pat::MatchStatementAt1At1End
            | Pat::SwitchStatementAt1At1End => {
                (self.at(p) == b'}' || self.php_close(p)).then_some(p)
            }
            Pat::InterfaceExtends | Pat::ClassExtends => self.word_then_spaces(p, b"extends"),
            Pat::ClassImplements => self.word_then_spaces(p, b"implements"),
            Pat::InterfaceExtendsEnd | Pat::ClassImplementsEnd => (self.at(p) == b'{').then_some(p),
            Pat::ClassExtendsEnd | Pat::RootAt52End => self.ahead_other(p, pat == Pat::RootAt52End),
            Pat::PhpTypesAt2 => self.iword_in(
                p,
                &[
                    b"null",
                    b"int",
                    b"float",
                    b"bool",
                    b"string",
                    b"array",
                    b"object",
                    b"callable",
                    b"iterable",
                    b"true",
                    b"false",
                    b"mixed",
                    b"void",
                ],
            ),
            Pat::PhpTypesAt3 => self.iword_in(p, &[b"parent", b"self"]),
            Pat::PhpTypesAt5
            | Pat::RootAt15End
            | Pat::RootAt17At1End
            | Pat::RootAt18At0End
            | Pat::RootAt17At2End
            | Pat::ParameterDefaultTypesAt7End
            | Pat::AttributeAt1End => self.lit(p, b")"),
            Pat::ClassConstant => self.class_constant(p),
            Pat::RootAt8 => self.enum_head(p),
            Pat::RootAt8At2At0 => self.enum_case(p),
            Pat::Nested6At0 => {
                self.modifier(p, &[(b"final", 1), (b"abstract", 1), (b"readonly", 2)])
            }
            Pat::Nested9At0 => self.modifier(
                p,
                &[
                    (b"final", 1),
                    (b"abstract", 1),
                    (b"public", 2),
                    (b"private", 2),
                    (b"protected", 2),
                ],
            ),
            Pat::Nested11At0 => self.modifier(
                p,
                &[
                    (b"final", 1),
                    (b"abstract", 1),
                    (b"public", 2),
                    (b"private", 2),
                    (b"protected", 2),
                    (b"static", 3),
                ],
            ),
            Pat::Nested10At0 => self.visibility(p, false),
            Pat::Nested13At0 => self.visibility(p, true),
            Pat::RootAt9 => self.class_head(p),
            Pat::RootAt9At0 => self.anonymous_class_arguments(p),
            Pat::RootAt9At0End
            | Pat::ScopeResolutionAt1End
            | Pat::InvokeCallEnd
            | Pat::RootAt27End
            | Pat::ObjectAt1End
            | Pat::FunctionCallAt0End
            | Pat::FunctionCallAt1End
            | Pat::InstantiationAt1End
            | Pat::RootAt62End
            | Pat::MatchStatementAt1At0End
            | Pat::SwitchStatementAt1At0End => self.close_or_php(p, b')'),
            Pat::RootAt17 => self.function_keyword(p, b"function"),
            Pat::RootAt18 => self.function_keyword(p, b"fn"),
            Pat::RootAt17End => (self.at(self.space_end(p)) == b'{').then_some(p),
            Pat::RootAt17At1 => self.parameters_open(p),
            Pat::RootAt18At0 => self.arrow_parameters_open(p),
            Pat::RootAt18End | Pat::RootAt40 | Pat::MatchStatementAt1At1At0 => self.lit(p, b"=>"),
            Pat::RootAt17At2 => self.closure_use(p),
            Pat::RootAt17At2At1 => self.closure_use_variable(p),
            Pat::RootAt17At3 => self.return_type(p, ReturnEnd::Brace),
            Pat::RootAt18At1 => self.return_type(p, ReturnEnd::Arrow),
            Pat::FunctionParametersAt3 => self.variadic_parameter(p),
            Pat::FunctionParametersAt4 => self.typed_parameter(p),
            Pat::FunctionParametersAt5 => self.untyped_parameter(p),
            Pat::FunctionParametersAt6 => self.defaulted_parameter(p),
            Pat::FunctionParametersAt4End
            | Pat::FunctionParametersAt4At0End
            | Pat::FunctionParametersAt6End
            | Pat::RootAt19At2End => self.parameter_end(p).then_some(p),
            Pat::FunctionParametersAt4At0 | Pat::ParameterDefaultTypesAt5 => self.lit(p, b"="),
            Pat::RootAt19 => self.constructor(p),
            Pat::RootAt19End => self.constructor_close(p),
            Pat::RootAt19At2 => self.promoted_parameter(p),
            Pat::RootAt20 => self.method_head(p),
            Pat::Nested12At0 => self.keyword(p, b"static"),
            Pat::Nested12At1 => self.keyword(p, b"never"),
            Pat::RootAt20End => self.method_close(p),
            Pat::RootAt21 => self.property(p),
            Pat::ParameterDefaultTypesAt6 => {
                (self.at(p) == b'&' && self.at(self.space_end(p + 1)) == b'$').then_some(p + 1)
            }
            Pat::ParameterDefaultTypesAt7 | Pat::RootAt27 => self.array_paren(p),
            Pat::ParameterDefaultTypesAt10 => {
                let end = self.qualified(p)?;
                self.starts(end, b"::").then_some(p)
            }
            Pat::ParameterDefaultTypesAt10End => self.static_member(p),

            Pat::Nested17At0 => {
                let end = [b"parent".as_slice(), b"static", b"self"]
                    .iter()
                    .find_map(|word| self.ilit(p, word))?;
                (!is_name(self.at(end))).then_some(end)
            }
            Pat::VarGlobal => self.global_variable(p),
            Pat::VarGlobalSafer => self.superglobal(p),
            Pat::VariableNameAt2 => self.interpolated_variable(p),
            Pat::VariableNameAt3 => self.braced_variable(p),
            Pat::VarLanguage => {
                let end = self.lit(p, b"$this")?;
                self.cap(1, p, p + 1);
                self.word_at(end).is_none().then_some(end)
            }
            Pat::VarBasic => self.variable(p),
            Pat::VariablesAt4 => self.dollar_brace(p),
            Pat::ScopeResolutionAt0 => self.class_before_scope(p),
            Pat::ScopeResolutionAt1 => self.static_call(p),
            Pat::ScopeResolutionAt2 => self.class_constant_name(p),
            Pat::ScopeResolutionAt3 => self.static_access(p),
            Pat::InvokeCall => self.invoke(p),
            Pat::RootAt26 => self.empty_array(p),
            Pat::RootAt28 => self.cast(p),
            Pat::RootAt29 => self.iword_in(
                p,
                &[
                    b"array",
                    b"real",
                    b"double",
                    b"float",
                    b"integer",
                    b"int",
                    b"boolean",
                    b"bool",
                    b"string",
                    b"class",
                    b"var",
                    b"function",
                    b"interface",
                    b"trait",
                    b"parent",
                    b"self",
                    b"object",
                    b"mixed",
                ],
            ),
            Pat::RootAt30 => self.ikeyword(p, b"const"),
            Pat::RootAt31 => self.storage_modifier_of(
                p,
                &[
                    (b"global", 1),
                    (b"abstract", 2),
                    (b"final", 2),
                    (b"private", 3),
                    (b"protected", 3),
                    (b"public", 3),
                    (b"static", 4),
                ],
            ),
            Pat::RootAt37 => self.ikeyword(p, b"clone"),
            Pat::RootAt49 => self.ikeyword(p, b"as"),
            Pat::ObjectAt0 => self.object_brace(p),
            Pat::ObjectAt1 => self.method_call(p),
            Pat::ObjectAt2 => self.property_access(p),
            Pat::RootAt33 => self.lit(p, b";"),
            Pat::RootAt34 => self.lit(p, b":"),
            Pat::RootAt38 => self.lit(p, b"..."),
            Pat::RootAt39 => self.lit(p, b".=").or_else(|| self.lit(p, b".")),
            Pat::RootAt41 => self.reference(p),
            Pat::RootAt42 => self.lit(p, b"@"),
            Pat::RootAt43 => self.equals_or_differs(p),
            Pat::RootAt44 => self.assignment(p),
            Pat::RootAt45 => self.relation(p),
            Pat::RootAt46 => self.lit(p, b"--").or_else(|| self.lit(p, b"++")),
            Pat::RootAt47 => self.arithmetic(p),
            Pat::RootAt48 => self.logical(p),
            Pat::RootAt51 => self.bitwise(p),
            Pat::FunctionCallAt0 => self.qualified_call(p),
            Pat::FunctionCallAt1 => self.call(p),
            Pat::RootAt52 => self.instanceof(p),
            Pat::InstantiationAt0 => self.instantiation(p, false),
            Pat::InstantiationAt1 => self.instantiation(p, true),
            Pat::RootAt54 => self.word_spaces_name(p, b"goto"),
            Pat::RootAt55 => self.label(p),
            Pat::TernaryShorthand => self.lit(p, b"?:"),
            Pat::NullCoalescing => self.lit(p, b"??"),
            Pat::TernaryExpressionEnd => {
                (self.at(p) == b':' && !self.prev_is(p, b':') && self.at(p + 1) != b':')
                    .then_some(p + 1)
            }
            Pat::RootAt61 => self.lit(p, b"["),
            Pat::RootAt61End => self.close_or_php(p, b']'),
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
            Pat::TernaryExpressionAt0 => self.ternary_label(p),
            Pat::MatchStatementAt1 => self.keyword(p, b"match"),
            Pat::SwitchStatementAt1 => self.switch(p),
            Pat::RootAt12 => self.yield_from(p),
            Pat::RootAt13 => self.control_keyword(p),
            Pat::RootAt14 => self.include(p),
            Pat::RootAt14End => (matches!(self.at(p), b';')
                || self.space_at(p) > 0
                || self.at_eol(p)
                || self.php_close(p))
            .then_some(p),
            Pat::RootAt15 => self.catch(p),
            Pat::Nested8At0 => self.lit(p, b"|"),
            Pat::RootAt15At0 => self.caught(p),
            Pat::RootAt16 => self.exception_keyword(p),
            Pat::NumbersAt0 => self.radix(p, b"xX", |b| b.is_ascii_hexdigit()),
            Pat::NumbersAt1 => self.radix(p, b"bB", |b| matches!(b, b'0' | b'1')),
            Pat::NumbersAt2 => self.radix(p, b"oO", |b| matches!(b, b'0'..=b'7')),
            Pat::NumbersAt3 => self.octal(p),
            Pat::NumbersAt4 => self.decimal(p),
            Pat::NumbersAt5 => self.integer(p),

            Pat::RegexDoubleQuoted => self.quoted_regex_open(p, b'"'),
            Pat::RegexSingleQuoted => self.quoted_regex_open(p, b'\''),
            Pat::RegexDoubleQuotedEnd => self.regex_close(p, b'"'),
            Pat::RegexSingleQuotedEnd => self.regex_close(p, b'\''),
            Pat::RegexDoubleQuotedAt0 => self.regex_escape(p),
            Pat::RegexDoubleQuotedAt3 | Pat::RegexSingleQuotedAt2 | Pat::HeredocInteriorAt6At3 => {
                self.regex_class_open(p)
            }
            Pat::SingleQuoteRegexEscape => self.single_quoted_regex_escape(p),
            Pat::RegexSingleQuotedAt1 | Pat::HeredocInteriorAt6At2 => self.regex_repeat(p),
            Pat::RegexSingleQuotedAt3 => self.regex_operator(p),
            Pat::HeredocInteriorAt6At3At0 => self.regex_class_escape(p),
            Pat::HeredocInteriorAt6At5 => self.regex_comment(p),
            Pat::HeredocInteriorAt6At5End => self.at_eol(p).then_some(p),
            Pat::InterpolationDoubleQuotedAt0 => self.lit(p, b"\\\""),
            Pat::InterpolationAt0 => self.octal_escape(p),
            Pat::InterpolationAt1 => self.hex_escape(p),
            Pat::InterpolationAt2 => self.unicode_escape(p),
            Pat::InterpolationAt3 => (self.at(p) == b'\\'
                && matches!(
                    self.at(p + 1),
                    b'$' | b'\\' | b'e' | b'f' | b'n' | b'r' | b't' | b'v'
                ))
            .then_some(p + 2),
            Pat::InterpolationAt4 => self.brace_interpolation(p),
            Pat::SqlStringDoubleQuoted => self.sql_open(p, b'"'),
            Pat::SqlStringSingleQuoted => self.sql_open(p, b'\''),
            Pat::SqlStringDoubleQuotedAt0 => self.sql_string_comment(p, b"#", b'"'),
            Pat::SqlStringDoubleQuotedAt1 => self.sql_string_comment(p, b"--", b'"'),
            Pat::SqlStringSingleQuotedAt0 => self.sql_string_comment(p, b"#", b'\''),
            Pat::SqlStringSingleQuotedAt1 => self.sql_string_comment(p, b"--", b'\''),
            Pat::SqlStringDoubleQuotedAt2 => self.sql_escape(p),
            Pat::SqlStringDoubleQuotedAt3 => self.sql_quote(p, b'\'', b'"'),
            Pat::SqlStringDoubleQuotedAt4 => self.sql_quote(p, b'`', b'"'),
            Pat::SqlStringSingleQuotedAt4 => self.sql_quote(p, b'`', b'\''),
            Pat::SqlStringSingleQuotedAt5 => self.sql_quote(p, b'"', b'\''),
            Pat::SqlStringSingleQuotedAt3 => {
                (self.at(p) == b'N' && self.at(p + 1) == b'\'').then_some(p + 1)
            }
            Pat::SqlStringDoubleQuotedEnd
            | Pat::StringDoubleQuoted
            | Pat::StringDoubleQuotedEnd => self.lit(p, b"\""),
            Pat::SqlStringDoubleQuotedAt5
            | Pat::SqlStringDoubleQuotedAt5End
            | Pat::SqlStringSingleQuotedEnd
            | Pat::StringSingleQuoted
            | Pat::StringSingleQuotedEnd => self.lit(p, b"'"),
            Pat::SqlStringDoubleQuotedAt6
            | Pat::SqlStringDoubleQuotedAt6End
            | Pat::StringBacktick
            | Pat::StringBacktickEnd => self.lit(p, b"`"),
            Pat::StringSingleQuotedAt0 => self.single_quote_escape(p),
            Pat::StringBacktickAt0 => self.lit(p, b"\\`"),
            Pat::HeredocAt0 => self.heredoc_ahead(p, b'"'),
            Pat::HeredocAt1 => self.heredoc_ahead(p, b'\''),
            Pat::HeredocInteriorAt0 => self.labelled_heredoc_open(p, &[b"HTML"]),
            Pat::HeredocInteriorAt1 => self.labelled_heredoc_open(p, &[b"XML"]),
            Pat::HeredocInteriorAt2Dql => self.labelled_heredoc_open(p, &[b"DQL"]),
            Pat::HeredocInteriorAt2Sql => self.labelled_heredoc_open(p, &[b"SQL"]),
            Pat::HeredocInteriorAt3Javascript => self.labelled_heredoc_open(p, &[b"JAVASCRIPT"]),
            Pat::HeredocInteriorAt3Js => self.labelled_heredoc_open(p, &[b"JS"]),
            Pat::HeredocInteriorAt4 => self.labelled_heredoc_open(p, &[b"JSON"]),
            Pat::HeredocInteriorAt5 => self.labelled_heredoc_open(p, &[b"CSS"]),
            Pat::HeredocInteriorAt6 => self.labelled_heredoc_open(p, &[b"REGEXP", b"REGEX"]),
            Pat::HeredocInteriorAt7 => self.labelled_heredoc_open(p, &[b"BLADE"]),
            Pat::HeredocInteriorAt8 => self.heredoc_open_any(p),
            Pat::NowdocInteriorAt0 => self.nowdoc_open(p, &[b"HTML"]),
            Pat::NowdocInteriorAt1 => self.nowdoc_open(p, &[b"XML"]),
            Pat::NowdocInteriorAt2Dql => self.nowdoc_open(p, &[b"DQL"]),
            Pat::NowdocInteriorAt2Sql => self.nowdoc_open(p, &[b"SQL"]),
            Pat::NowdocInteriorAt3Javascript => self.nowdoc_open(p, &[b"JAVASCRIPT"]),
            Pat::NowdocInteriorAt3Js => self.nowdoc_open(p, &[b"JS"]),
            Pat::NowdocInteriorAt4 => self.nowdoc_open(p, &[b"JSON"]),
            Pat::NowdocInteriorAt5 => self.nowdoc_open(p, &[b"CSS"]),
            Pat::NowdocInteriorAt6 => self.nowdoc_open(p, &[b"REGEXP", b"REGEX"]),
            Pat::NowdocInteriorAt7 => self.nowdoc_open(p, &[b"BLADE"]),
            Pat::NowdocInteriorAt8 => self.nowdoc_open_any(p),
            Pat::HeredocInteriorAt0End | Pat::NowdocInteriorAt0End => {
                self.label_line(p, Some(b"HTML"))
            }
            Pat::HeredocInteriorAt1End | Pat::NowdocInteriorAt1End => {
                self.label_line(p, Some(b"XML"))
            }
            Pat::HeredocInteriorAt2DqlEnd | Pat::NowdocInteriorAt2DqlEnd => {
                self.label_line(p, Some(b"DQL"))
            }
            Pat::HeredocInteriorAt2SqlEnd | Pat::NowdocInteriorAt2SqlEnd => {
                self.label_line(p, Some(b"SQL"))
            }
            Pat::HeredocInteriorAt3JavascriptEnd | Pat::NowdocInteriorAt3JavascriptEnd => {
                self.label_line(p, Some(b"JAVASCRIPT"))
            }
            Pat::HeredocInteriorAt3JsEnd | Pat::NowdocInteriorAt3JsEnd => {
                self.label_line(p, Some(b"JS"))
            }
            Pat::HeredocInteriorAt4End | Pat::NowdocInteriorAt4End => {
                self.label_line(p, Some(b"JSON"))
            }
            Pat::HeredocInteriorAt5End | Pat::NowdocInteriorAt5End => {
                self.label_line(p, Some(b"CSS"))
            }
            Pat::HeredocInteriorAt6End
            | Pat::HeredocInteriorAt7End
            | Pat::HeredocInteriorAt8End
            | Pat::NowdocInteriorAt6End
            | Pat::NowdocInteriorAt7End
            | Pat::NowdocInteriorAt8End => self.label_line(p, None),

            table_pats!() => {
                let (table, list) = pat.words()?;
                self.table_list(table.words(), list, p)
            }
        }
    }

    // Shared pieces.

    /// The ends a run of name characters `from..end` can give a `\s` that
    /// follows it.
    /// The first is its whole length.
    /// The second is the start of the spaces past ASCII it ends with, if any.
    fn q_ends(&self, from: usize, end: usize) -> [Option<usize>; 2] {
        let mut trimmed = end;
        while trimmed > from
            && self.at(trimmed - 1) >= 0x80
            && self.char_before(trimmed).is_some_and(is_space)
        {
            trimmed = self.char_start_before(trimmed);
        }
        [
            Some(end),
            (trimmed > from && trimmed < end).then_some(trimmed),
        ]
    }

    /// `(?i)\b(word)\s+(NAME)`, with the word and the name as groups 1 and 2.
    fn keyword_name(&mut self, p: usize, word: &[u8]) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        self.word_spaces_name(p, word)
    }

    /// `(?i)(word)\s+(NAME)`, with the word and the name as groups 1 and 2.
    fn word_spaces_name(&mut self, p: usize, word: &[u8]) -> Option<usize> {
        let end = self.ilit(p, word)?;
        let at = self.space_end(end);
        if at == end {
            return None;
        }
        let name = self.name(at)?;
        self.cap(1, p, end);
        self.cap(2, at, name);
        Some(name)
    }

    /// The ends of the type `T` at `p` that signatures, properties and
    /// constants share, in the order Oniguruma tries them, each given to
    /// `then` until it accepts one:
    ///
    /// ```text
    /// (?:\?\s*)?Q
    /// | (?:Q|\(\s*Q(?:\s*&\s*Q)+\s*\))(?:\s*[\&|]\s*(?:Q|\(\s*Q(?:\s*&\s*Q)+\s*\)))+
    /// ```
    ///
    /// `Q` is a qualified name.
    /// A `Q` gives back only the spaces past ASCII it ends with
    /// ([`Lexer::q_ends`]). No shorter `Q` changes what follows it: the next
    /// character would be a name character, which no part after a type starts
    /// with.
    fn each_type<R>(&self, p: usize, then: &mut dyn FnMut(&Self, usize) -> Option<R>) -> Option<R> {
        let from = if self.at(p) == b'?' {
            self.space_end(p + 1)
        } else {
            p
        };
        if let Some(end) = self.qualified(from) {
            for end in self.q_ends(from, end).into_iter().flatten() {
                if let Some(found) = then(self, end) {
                    return Some(found);
                }
            }
        }
        for end in self.type_atom(p).into_iter().flatten() {
            if let Some(found) = self.type_union(end, true, then) {
                return Some(found);
            }
        }
        None
    }

    /// The ends of `Q|\(\s*Q(?:\s*&\s*Q)+\s*\)` at `p`.
    fn type_atom(&self, p: usize) -> [Option<usize>; 2] {
        if self.at(p) != b'(' {
            return match self.qualified(p) {
                Some(end) => self.q_ends(p, end),
                None => [None, None],
            };
        }
        let Some(mut at) = self.qualified(self.space_end(p + 1)) else {
            return [None, None];
        };
        let mut parts = 0;
        loop {
            let amp = self.space_end(at);
            if self.at(amp) != b'&' {
                break;
            }
            let Some(end) = self.qualified(self.space_end(amp + 1)) else {
                break;
            };
            at = end;
            parts += 1;
        }
        let close = self.space_end(at);
        [
            (parts > 0 && self.at(close) == b')').then_some(close + 1),
            None,
        ]
    }

    /// `(?:\s*[\&|]\s*ATOM)+` from `at` when `first`, else `*`, greedy: each
    /// further part is tried before `then` takes the end.
    fn type_union<R>(
        &self,
        at: usize,
        first: bool,
        then: &mut dyn FnMut(&Self, usize) -> Option<R>,
    ) -> Option<R> {
        self.spend(1);
        let sep = self.space_end(at);
        if matches!(self.at(sep), b'&' | b'|') {
            for end in self
                .type_atom(self.space_end(sep + 1))
                .into_iter()
                .flatten()
            {
                if let Some(found) = self.type_union(end, false, then) {
                    return Some(found);
                }
            }
        }
        if first { None } else { then(self, at) }
    }

    /// `(?=\s*(?:[),]|/[*/]|#))`, the end of a parameter.
    fn parameter_end(&self, p: usize) -> bool {
        let at = self.space_end(p);
        matches!(self.at(at), b')' | b',' | b'#') && at < self.lim()
            || self.starts(at, b"/*")
            || self.starts(at, b"//")
    }

    /// `(?:(&)\s*)?(\$)NAME` at `p`: a parameter's reference mark, dollar and
    /// name, recorded as `amp`, `dollar` and `whole`, or `None`.
    fn parameter_variable(&self, p: usize) -> Option<ParameterVariable> {
        let (amp, at) = if self.at(p) == b'&' {
            (Some(p), self.space_end(p + 1))
        } else {
            (None, p)
        };
        if self.at(at) != b'$' {
            return None;
        }
        let end = self.name(at + 1)?;
        Some(ParameterVariable {
            amp,
            dollar: at,
            end,
        })
    }
}

/// A parameter's variable: `&`, `$` and the end of its name.
#[derive(Clone, Copy)]
struct ParameterVariable {
    /// Where its `&` is, if it has one.
    amp: Option<usize>,
    /// Where its `$` is.
    dollar: usize,
    /// The end of its name.
    end: usize,
}

/// What a return type is followed by.
#[derive(Clone, Copy, PartialEq, Eq)]
enum ReturnEnd {
    /// A function's body, `{`.
    Brace,
    /// An arrow function's `=>`.
    Arrow,
}
