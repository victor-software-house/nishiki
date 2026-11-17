//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups of its match with [`Lexer::capture`].
//! The engine names those the grammar names.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would.
//! Where a quantifier could give characters back, the doc comment shows why it
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
//! Where a `\s` follows a name, the name gives the last such space back
//! ([`Lexer::q_ends`]).
//! Where a name follows a `\s*` or a `\s+`, the run gives its last such space
//! back when no name starts after it ([`Lexer::after_spaces`]).

mod comments;
mod declarations;
mod expressions;
mod strings;

use super::engine::Lexer;
use super::grammar::{Pattern, table_patterns};
use crate::classes::is_space;
use crate::php_family::matchers::{is_name, is_name_start, is_qualified};
use crate::rules::Chars;

impl Lexer<'_> {
    /// Whether `pattern` matches at `p`, and where its match ends.
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
    pub(super) fn matcher(&mut self, pattern: Pattern, p: usize) -> Option<usize> {
        match pattern {
            Pattern::Attribute => self.lit(p, b"#["),
            Pattern::AttributeEnd
            | Pattern::RegexDoubleQuotedAt3End
            | Pattern::RegexSingleQuotedAt2End
            | Pattern::HeredocInteriorAt6At3End => self.lit(p, b"]"),
            Pattern::AttributeAt0 | Pattern::InterfaceExtendsAt1 => self.lit(p, b","),
            Pattern::AttributeNameAt0 | Pattern::ClassNameAt0 | Pattern::InheritanceSingleAt0 => {
                self.ahead_namespaced(p)
            }
            Pattern::AttributeNameAt0End
            | Pattern::AttributeNameAt2End
            | Pattern::ClassNameAt0End
            | Pattern::ClassNameAt2End => self.last_segment(p),
            Pattern::Namespace => self.namespace_part(p),
            Pattern::NamespaceEnd => self.namespace_end(p),
            Pattern::NamespaceAt0 => self.lit(p, b"\\"),
            Pattern::AttributeNameAt1 => self.attribute_name(p),
            Pattern::AttributeNameAt2 | Pattern::ClassNameAt2 | Pattern::Nested8At1 => {
                (p < self.lim() && (self.at(p) == b'\\' || is_name_start(self.at(p)))).then_some(p)
            }
            Pattern::AttributeAt1 => self.attribute_call(p),
            Pattern::NamedArguments => self.named_argument(p),
            Pattern::InheritanceSingleAt0End => self.last_segment_before_other(p),
            Pattern::InheritanceSingleAt3
            | Pattern::Nested14At1
            | Pattern::ConstantsAt5
            | Pattern::UseInnerAt1End => self.name(p),
            Pattern::RootAt3At1 => self.qualified(p),
            Pattern::Nested8At1End => self.name_alone(p),

            Pattern::CommentsAt0 => self.doc_comment(p),
            Pattern::CommentsAt0End | Pattern::CommentsAt1End => self.lit(p, b"*/"),
            Pattern::CommentsAt1 => self.lit(p, b"/*"),
            Pattern::CommentsAt2 => self.line_comment(p, b'/'),
            Pattern::CommentsAt3 => self.line_comment(p, b'#'),
            Pattern::CommentsAt2End
            | Pattern::CommentsAt3End
            | Pattern::HeredocAt0End
            | Pattern::HeredocAt1End => (!self.at_g(p)).then_some(p),
            Pattern::CommentsAt2At0 => self.lit(p, b"//"),
            Pattern::CommentsAt3At0 => self.lit(p, b"#"),
            Pattern::CommentsAt2At0End | Pattern::CommentsAt3At0End => self.comment_end(p),
            Pattern::PhpDocAt0 => self.doc_line(p),
            Pattern::PhpDocAt1 => self.doc_access(p),
            Pattern::PhpDocAt2 => self.doc_xlink(p),
            Pattern::PhpDocAt3 => self.doc_typed_tag(p),
            Pattern::PhpDocAt3End => (self.space_at(p) > 0 || self.starts(p, b"*/")).then_some(p),
            Pattern::PhpDocAt4 => self.php_doc_tag(p),
            Pattern::PhpDocAt5 => self.doc_inline_tag(p),
            Pattern::PhpDocTypesArrayMultiple
            | Pattern::PhpTypesAt4
            | Pattern::RootAt62
            | Pattern::MatchStatementAt1At0
            | Pattern::SwitchStatementAt1At0 => self.lit(p, b"("),
            Pattern::PhpDocTypesArrayMultipleEnd => self.doc_types_close(p),
            Pattern::PhpDocTypes => self.doc_types(p),
            Pattern::PhpDocTypesArraySingle => self.doc_type_array(p),
            Pattern::Nested2At0 | Pattern::TernaryExpression => self.lit(p, b"?"),
            Pattern::Nested2At1 => self.word_in(
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
            Pattern::Nested2At3 => matches!(self.at(p), b'&' | b'|').then_some(p + 1),

            Pattern::RootAt2 => self.namespace_statement(p, true),
            Pattern::RootAt3 => self.namespace_statement(p, false),
            Pattern::RootAt3End => (self.prev_is(p, b'}') || self.php_close(p)).then_some(p),
            Pattern::RootAt3At2
            | Pattern::RootAt5At1
            | Pattern::RootAt6At1
            | Pattern::RootAt7At2
            | Pattern::RootAt8At2
            | Pattern::RootAt9At4
            | Pattern::RootAt60
            | Pattern::MatchStatementAt1At1
            | Pattern::SwitchStatementAt1At1 => self.lit(p, b"{"),
            Pattern::RootAt3At2End
            | Pattern::RootAt6End
            | Pattern::RootAt7End
            | Pattern::RootAt8End
            | Pattern::RootAt9End
            | Pattern::RootAt60End
            | Pattern::MatchStatementAt1End
            | Pattern::SwitchStatementAt1End => self.close_or_php(p, b'}'),
            Pattern::RootAt3At3 => self.non_spaces(p),
            Pattern::RootAt4 => self.spaces_before(p, b"use"),
            Pattern::MatchStatementAt0 => self.spaces_before(p, b"match"),
            Pattern::SwitchStatementAt0 => self.spaces_before(p, b"switch"),
            Pattern::RootAt5 => self.ikeyword(p, b"use"),
            Pattern::RootAt5End => {
                (self.prev_is(p, b'}') || self.at(p) == b';' || self.php_close(p)).then_some(p)
            }
            Pattern::RootAt5At0 => {
                let end = self.word_in(p, &[b"const", b"function"])?;
                self.capture(1, p, end);
                Some(end)
            }
            Pattern::RootAt5At1End
            | Pattern::ObjectAt0End
            | Pattern::InterpolationAt4End
            | Pattern::VariablesAt4End => self.lit(p, b"}"),
            Pattern::RootAt5At1At1 => self.trait_alias_modifier(p),
            Pattern::Nested19At0 => self.storage_modifier_of(
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
            Pattern::Nested19At1 => (p < self.lim()).then_some(self.lim()),
            Pattern::RootAt5At1At2 => self.keyword_name(p, b"as"),
            Pattern::RootAt5At1At3 => self.keyword_name(p, b"insteadof"),
            Pattern::UseInnerAt1 => self.use_as(p),
            Pattern::Nested4At0 => self.word_in(p, &[b"self", b"static", b"parent"]),

            Pattern::RootAt6 => self.keyword_name(p, b"trait"),
            Pattern::RootAt7 => self.keyword_name(p, b"interface"),
            Pattern::RootAt6At1End
            | Pattern::RootAt7At2End
            | Pattern::RootAt8At2End
            | Pattern::RootAt9At4End
            | Pattern::MatchStatementAt1At1End
            | Pattern::SwitchStatementAt1At1End => {
                (self.at(p) == b'}' || self.php_close(p)).then_some(p)
            }
            Pattern::InterfaceExtends | Pattern::ClassExtends => {
                self.word_then_spaces(p, b"extends")
            }
            Pattern::ClassImplements => self.word_then_spaces(p, b"implements"),
            Pattern::InterfaceExtendsEnd | Pattern::ClassImplementsEnd => {
                (self.at(p) == b'{').then_some(p)
            }
            Pattern::ClassExtendsEnd | Pattern::RootAt52End => {
                self.ahead_other(p, pattern == Pattern::RootAt52End)
            }
            Pattern::PhpTypesAt2 => self.iword_in(
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
            Pattern::PhpTypesAt3 => self.iword_in(p, &[b"parent", b"self"]),
            Pattern::PhpTypesAt5
            | Pattern::RootAt15End
            | Pattern::RootAt17At1End
            | Pattern::RootAt18At0End
            | Pattern::RootAt17At2End
            | Pattern::ParameterDefaultTypesAt7End
            | Pattern::AttributeAt1End => self.lit(p, b")"),
            Pattern::ClassConstant => self.class_constant(p),
            Pattern::RootAt8 => self.enum_head(p),
            Pattern::RootAt8At2At0 => self.enum_case(p),
            Pattern::Nested6At0 => {
                self.modifier(p, &[(b"final", 1), (b"abstract", 1), (b"readonly", 2)])
            }
            Pattern::Nested9At0 => self.modifier(
                p,
                &[
                    (b"final", 1),
                    (b"abstract", 1),
                    (b"public", 2),
                    (b"private", 2),
                    (b"protected", 2),
                ],
            ),
            Pattern::Nested11At0 => self.modifier(
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
            Pattern::Nested10At0 => self.visibility(p, false),
            Pattern::Nested13At0 => self.visibility(p, true),
            Pattern::RootAt9 => self.class_head(p),
            Pattern::RootAt9At0 => self.anonymous_class_arguments(p),
            Pattern::RootAt9At0End
            | Pattern::ScopeResolutionAt1End
            | Pattern::InvokeCallEnd
            | Pattern::RootAt27End
            | Pattern::ObjectAt1End
            | Pattern::FunctionCallAt0End
            | Pattern::FunctionCallAt1End
            | Pattern::InstantiationAt1End
            | Pattern::RootAt62End
            | Pattern::MatchStatementAt1At0End
            | Pattern::SwitchStatementAt1At0End => self.close_or_php(p, b')'),
            Pattern::RootAt17 => self.function_keyword(p, b"function"),
            Pattern::RootAt18 => self.function_keyword(p, b"fn"),
            Pattern::RootAt17End => (self.at(self.space_end(p)) == b'{').then_some(p),
            Pattern::RootAt17At1 => self.parameters_open(p),
            Pattern::RootAt18At0 => self.arrow_parameters_open(p),
            Pattern::RootAt18End | Pattern::RootAt40 | Pattern::MatchStatementAt1At1At0 => {
                self.lit(p, b"=>")
            }
            Pattern::RootAt17At2 => self.closure_use(p),
            Pattern::RootAt17At2At1 => self.closure_use_variable(p),
            Pattern::RootAt17At3 => self.return_type(p, ReturnEnd::Brace),
            Pattern::RootAt18At1 => self.return_type(p, ReturnEnd::Arrow),
            Pattern::FunctionParametersAt3 => self.variadic_parameter(p),
            Pattern::FunctionParametersAt4 => self.typed_parameter(p),
            Pattern::FunctionParametersAt5 => self.untyped_parameter(p),
            Pattern::FunctionParametersAt6 => self.defaulted_parameter(p),
            Pattern::FunctionParametersAt4End
            | Pattern::FunctionParametersAt4At0End
            | Pattern::FunctionParametersAt6End
            | Pattern::RootAt19At2End => self.parameter_end(p).then_some(p),
            Pattern::FunctionParametersAt4At0 | Pattern::ParameterDefaultTypesAt5 => {
                self.lit(p, b"=")
            }
            Pattern::RootAt19 => self.constructor(p),
            Pattern::RootAt19End => self.constructor_close(p),
            Pattern::RootAt19At2 => self.promoted_parameter(p),
            Pattern::RootAt20 => self.method_head(p),
            Pattern::Nested12At0 => self.keyword(p, b"static"),
            Pattern::Nested12At1 => self.keyword(p, b"never"),
            Pattern::RootAt20End => self.method_close(p),
            Pattern::RootAt21 => self.property(p),
            Pattern::ParameterDefaultTypesAt6 => {
                (self.at(p) == b'&' && self.at(self.space_end(p + 1)) == b'$').then_some(p + 1)
            }
            Pattern::ParameterDefaultTypesAt7 | Pattern::RootAt27 => self.array_paren(p),
            Pattern::ParameterDefaultTypesAt10 => {
                let end = self.qualified(p)?;
                self.starts(end, b"::").then_some(p)
            }
            Pattern::ParameterDefaultTypesAt10End => self.static_member(p),

            Pattern::Nested17At0 => {
                let end = [b"parent".as_slice(), b"static", b"self"]
                    .iter()
                    .find_map(|word| self.ilit(p, word))?;
                (!is_name(self.at(end))).then_some(end)
            }
            Pattern::VarGlobal => self.global_variable(p),
            Pattern::VarGlobalSafer => self.superglobal(p),
            Pattern::VariableNameAt2 => self.interpolated_variable(p),
            Pattern::VariableNameAt3 => self.braced_variable(p),
            Pattern::VarLanguage => {
                let end = self.lit(p, b"$this")?;
                self.capture(1, p, p + 1);
                self.word_at(end).is_none().then_some(end)
            }
            Pattern::VarBasic => self.variable(p),
            Pattern::VariablesAt4 => self.dollar_brace(p),
            Pattern::ScopeResolutionAt0 => self.class_before_scope(p),
            Pattern::ScopeResolutionAt1 => self.static_call(p),
            Pattern::ScopeResolutionAt2 => self.class_constant_name(p),
            Pattern::ScopeResolutionAt3 => self.static_access(p),
            Pattern::InvokeCall => self.invoke(p),
            Pattern::RootAt26 => self.empty_array(p),
            Pattern::RootAt28 => self.cast(p),
            Pattern::RootAt29 => self.iword_in(
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
            Pattern::RootAt30 => self.ikeyword(p, b"const"),
            Pattern::RootAt31 => self.storage_modifier_of(
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
            Pattern::RootAt37 => self.ikeyword(p, b"clone"),
            Pattern::RootAt49 => self.ikeyword(p, b"as"),
            Pattern::ObjectAt0 => self.object_brace(p),
            Pattern::ObjectAt1 => self.method_call(p),
            Pattern::ObjectAt2 => self.property_access(p),
            Pattern::RootAt33 => self.lit(p, b";"),
            Pattern::RootAt34 => self.lit(p, b":"),
            Pattern::RootAt38 => self.lit(p, b"..."),
            Pattern::RootAt39 => self.lit(p, b".=").or_else(|| self.lit(p, b".")),
            Pattern::RootAt41 => self.reference(p),
            Pattern::RootAt42 => self.lit(p, b"@"),
            Pattern::RootAt43 => self.equals_or_differs(p),
            Pattern::RootAt44 => self.assignment(p),
            Pattern::RootAt45 => self.relation(p),
            Pattern::RootAt46 => self.lit(p, b"--").or_else(|| self.lit(p, b"++")),
            Pattern::RootAt47 => self.arithmetic(p),
            Pattern::RootAt48 => self.logical(p),
            Pattern::RootAt51 => self.bitwise(p),
            Pattern::FunctionCallAt0 => self.qualified_call(p),
            Pattern::FunctionCallAt1 => self.call(p),
            Pattern::RootAt52 => self.instanceof(p),
            Pattern::InstantiationAt0 => self.instantiation(p, false),
            Pattern::InstantiationAt1 => self.instantiation(p, true),
            Pattern::RootAt54 => self.word_spaces_name(p, b"goto"),
            Pattern::RootAt55 => self.label(p),
            Pattern::TernaryShorthand => self.lit(p, b"?:"),
            Pattern::NullCoalescing => self.lit(p, b"??"),
            Pattern::TernaryExpressionEnd => {
                (self.at(p) == b':' && !self.prev_is(p, b':') && self.at(p + 1) != b':')
                    .then_some(p + 1)
            }
            Pattern::RootAt61 => self.lit(p, b"["),
            Pattern::RootAt61End => self.close_or_php(p, b']'),
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
            Pattern::TernaryExpressionAt0 => self.ternary_label(p),
            Pattern::MatchStatementAt1 => self.keyword(p, b"match"),
            Pattern::SwitchStatementAt1 => self.switch(p),
            Pattern::RootAt12 => self.yield_from(p),
            Pattern::RootAt13 => self.control_keyword(p),
            Pattern::RootAt14 => self.include(p),
            Pattern::RootAt14End => (matches!(self.at(p), b';')
                || self.space_at(p) > 0
                || self.at_eol(p)
                || self.php_close(p))
            .then_some(p),
            Pattern::RootAt15 => self.catch(p),
            Pattern::Nested8At0 => self.lit(p, b"|"),
            Pattern::RootAt15At0 => self.caught(p),
            Pattern::RootAt16 => self.exception_keyword(p),
            Pattern::NumbersAt0 => self.radix(p, b"xX", |b| b.is_ascii_hexdigit()),
            Pattern::NumbersAt1 => self.radix(p, b"bB", |b| matches!(b, b'0' | b'1')),
            Pattern::NumbersAt2 => self.radix(p, b"oO", |b| matches!(b, b'0'..=b'7')),
            Pattern::NumbersAt3 => self.octal(p),
            Pattern::NumbersAt4 => self.decimal(p),
            Pattern::NumbersAt5 => self.integer(p),

            Pattern::RegexDoubleQuoted => self.quoted_regex_open(p, b'"'),
            Pattern::RegexSingleQuoted => self.quoted_regex_open(p, b'\''),
            Pattern::RegexDoubleQuotedEnd => self.regex_close(p, b'"'),
            Pattern::RegexSingleQuotedEnd => self.regex_close(p, b'\''),
            Pattern::RegexDoubleQuotedAt0 => self.regex_escape(p),
            Pattern::RegexDoubleQuotedAt3
            | Pattern::RegexSingleQuotedAt2
            | Pattern::HeredocInteriorAt6At3 => self.regex_class_open(p),
            Pattern::SingleQuoteRegexEscape => self.single_quoted_regex_escape(p),
            Pattern::RegexSingleQuotedAt1 | Pattern::HeredocInteriorAt6At2 => self.regex_repeat(p),
            Pattern::RegexSingleQuotedAt3 => self.regex_operator(p),
            Pattern::HeredocInteriorAt6At3At0 => self.regex_class_escape(p),
            Pattern::HeredocInteriorAt6At5 => self.regex_comment(p),
            Pattern::HeredocInteriorAt6At5End => self.at_eol(p).then_some(p),
            Pattern::InterpolationDoubleQuotedAt0 => self.lit(p, b"\\\""),
            Pattern::InterpolationAt0 => self.octal_escape(p),
            Pattern::InterpolationAt1 => self.hex_escape(p),
            Pattern::InterpolationAt2 => self.unicode_escape(p),
            Pattern::InterpolationAt3 => (self.at(p) == b'\\'
                && matches!(
                    self.at(p + 1),
                    b'$' | b'\\' | b'e' | b'f' | b'n' | b'r' | b't' | b'v'
                ))
            .then_some(p + 2),
            Pattern::InterpolationAt4 => self.brace_interpolation(p),
            Pattern::SqlStringDoubleQuoted => self.sql_open(p, b'"'),
            Pattern::SqlStringSingleQuoted => self.sql_open(p, b'\''),
            Pattern::SqlStringDoubleQuotedAt0 => self.sql_string_comment(p, b"#", b'"'),
            Pattern::SqlStringDoubleQuotedAt1 => self.sql_string_comment(p, b"--", b'"'),
            Pattern::SqlStringSingleQuotedAt0 => self.sql_string_comment(p, b"#", b'\''),
            Pattern::SqlStringSingleQuotedAt1 => self.sql_string_comment(p, b"--", b'\''),
            Pattern::SqlStringDoubleQuotedAt2 => self.sql_escape(p),
            Pattern::SqlStringDoubleQuotedAt3 => self.sql_quote(p, b'\'', b'"'),
            Pattern::SqlStringDoubleQuotedAt4 => self.sql_quote(p, b'`', b'"'),
            Pattern::SqlStringSingleQuotedAt4 => self.sql_quote(p, b'`', b'\''),
            Pattern::SqlStringSingleQuotedAt5 => self.sql_quote(p, b'"', b'\''),
            Pattern::SqlStringSingleQuotedAt3 => {
                (self.at(p) == b'N' && self.at(p + 1) == b'\'').then_some(p + 1)
            }
            Pattern::SqlStringDoubleQuotedEnd
            | Pattern::StringDoubleQuoted
            | Pattern::StringDoubleQuotedEnd => self.lit(p, b"\""),
            Pattern::SqlStringDoubleQuotedAt5
            | Pattern::SqlStringDoubleQuotedAt5End
            | Pattern::SqlStringSingleQuotedEnd
            | Pattern::StringSingleQuoted
            | Pattern::StringSingleQuotedEnd => self.lit(p, b"'"),
            Pattern::SqlStringDoubleQuotedAt6
            | Pattern::SqlStringDoubleQuotedAt6End
            | Pattern::StringBacktick
            | Pattern::StringBacktickEnd => self.lit(p, b"`"),
            Pattern::StringSingleQuotedAt0 => self.single_quote_escape(p),
            Pattern::StringBacktickAt0 => self.lit(p, b"\\`"),
            Pattern::HeredocAt0 => self.heredoc_ahead(p, b'"'),
            Pattern::HeredocAt1 => self.heredoc_ahead(p, b'\''),
            Pattern::HeredocInteriorAt0 => self.labelled_heredoc_open(p, &[b"HTML"]),
            Pattern::HeredocInteriorAt1 => self.labelled_heredoc_open(p, &[b"XML"]),
            Pattern::HeredocInteriorAt2Dql => self.labelled_heredoc_open(p, &[b"DQL"]),
            Pattern::HeredocInteriorAt2Sql => self.labelled_heredoc_open(p, &[b"SQL"]),
            Pattern::HeredocInteriorAt3Javascript => {
                self.labelled_heredoc_open(p, &[b"JAVASCRIPT"])
            }
            Pattern::HeredocInteriorAt3Js => self.labelled_heredoc_open(p, &[b"JS"]),
            Pattern::HeredocInteriorAt4 => self.labelled_heredoc_open(p, &[b"JSON"]),
            Pattern::HeredocInteriorAt5 => self.labelled_heredoc_open(p, &[b"CSS"]),
            Pattern::HeredocInteriorAt6 => self.labelled_heredoc_open(p, &[b"REGEXP", b"REGEX"]),
            Pattern::HeredocInteriorAt7 => self.labelled_heredoc_open(p, &[b"BLADE"]),
            Pattern::HeredocInteriorAt8 => self.heredoc_open_any(p),
            Pattern::NowdocInteriorAt0 => self.nowdoc_open(p, &[b"HTML"]),
            Pattern::NowdocInteriorAt1 => self.nowdoc_open(p, &[b"XML"]),
            Pattern::NowdocInteriorAt2Dql => self.nowdoc_open(p, &[b"DQL"]),
            Pattern::NowdocInteriorAt2Sql => self.nowdoc_open(p, &[b"SQL"]),
            Pattern::NowdocInteriorAt3Javascript => self.nowdoc_open(p, &[b"JAVASCRIPT"]),
            Pattern::NowdocInteriorAt3Js => self.nowdoc_open(p, &[b"JS"]),
            Pattern::NowdocInteriorAt4 => self.nowdoc_open(p, &[b"JSON"]),
            Pattern::NowdocInteriorAt5 => self.nowdoc_open(p, &[b"CSS"]),
            Pattern::NowdocInteriorAt6 => self.nowdoc_open(p, &[b"REGEXP", b"REGEX"]),
            Pattern::NowdocInteriorAt7 => self.nowdoc_open(p, &[b"BLADE"]),
            Pattern::NowdocInteriorAt8 => self.nowdoc_open_any(p),
            Pattern::HeredocInteriorAt0End | Pattern::NowdocInteriorAt0End => {
                self.label_line(p, Some(b"HTML"))
            }
            Pattern::HeredocInteriorAt1End | Pattern::NowdocInteriorAt1End => {
                self.label_line(p, Some(b"XML"))
            }
            Pattern::HeredocInteriorAt2DqlEnd | Pattern::NowdocInteriorAt2DqlEnd => {
                self.label_line(p, Some(b"DQL"))
            }
            Pattern::HeredocInteriorAt2SqlEnd | Pattern::NowdocInteriorAt2SqlEnd => {
                self.label_line(p, Some(b"SQL"))
            }
            Pattern::HeredocInteriorAt3JavascriptEnd | Pattern::NowdocInteriorAt3JavascriptEnd => {
                self.label_line(p, Some(b"JAVASCRIPT"))
            }
            Pattern::HeredocInteriorAt3JsEnd | Pattern::NowdocInteriorAt3JsEnd => {
                self.label_line(p, Some(b"JS"))
            }
            Pattern::HeredocInteriorAt4End | Pattern::NowdocInteriorAt4End => {
                self.label_line(p, Some(b"JSON"))
            }
            Pattern::HeredocInteriorAt5End | Pattern::NowdocInteriorAt5End => {
                self.label_line(p, Some(b"CSS"))
            }
            Pattern::HeredocInteriorAt6End
            | Pattern::HeredocInteriorAt7End
            | Pattern::HeredocInteriorAt8End
            | Pattern::NowdocInteriorAt6End
            | Pattern::NowdocInteriorAt7End
            | Pattern::NowdocInteriorAt8End => self.label_line(p, None),

            table_patterns!() => {
                let (table, list) = pattern.words()?;
                self.table_list(table.words(), list, p)
            }
        }
    }

    // Shared pieces.

    /// The ends a run of name characters `from..end` can give a `\s` that
    /// follows it.
    /// First is its whole length.
    /// Second is the start of its last character, when that is a space past
    /// ASCII and not its only one.
    ///
    /// Oniguruma gives back one character at a time, so the longest shorter
    /// name that a `\s` can follow ends before that last space.
    /// `S` and two U+00A0 before `$j` give the type `S` and one U+00A0.
    fn q_ends(&self, from: usize, end: usize) -> [Option<usize>; 2] {
        let last = self.char_start_before(end);
        let gives_back =
            last > from && self.at(end - 1) >= 0x80 && self.char_before(end).is_some_and(is_space);
        [Some(end), gives_back.then_some(last)]
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
        let (at, name) = self.after_spaces(end, true, Self::name)?;
        self.capture(1, p, end);
        self.capture(2, at, name);
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
    /// A `Q` gives back only the space past ASCII it ends with
    /// ([`Lexer::q_ends`]). No shorter `Q` changes what follows it: the next
    /// character would be a name character, which no part after a type starts
    /// with.
    fn each_type<R>(&self, p: usize, then: &mut dyn FnMut(&Self, usize) -> Option<R>) -> Option<R> {
        let named = if self.at(p) == b'?' {
            self.after_spaces(p + 1, false, |lexer, from| lexer.q_then(from, &mut *then))
                .map(|(_, found)| found)
        } else {
            self.q_then(p, then)
        };
        if named.is_some() {
            return named;
        }
        for end in self.type_atom(p).into_iter().flatten() {
            if let Some(found) = self.type_union(end, true, then) {
                return Some(found);
            }
        }
        None
    }

    /// `then` at each end of the qualified name `Q` at `from`
    /// ([`Lexer::q_ends`]), until it accepts one.
    fn q_then<R>(&self, from: usize, then: &mut dyn FnMut(&Self, usize) -> Option<R>) -> Option<R> {
        let end = self.qualified(from)?;
        self.q_ends(from, end)
            .into_iter()
            .flatten()
            .find_map(|end| then(self, end))
    }

    /// The ends of `Q|\(\s*Q(?:\s*&\s*Q)+\s*\)` at `p`.
    fn type_atom(&self, p: usize) -> [Option<usize>; 2] {
        if self.at(p) != b'(' {
            return match self.qualified(p) {
                Some(end) => self.q_ends(p, end),
                None => [None, None],
            };
        }
        let Some((_, mut at)) = self.after_spaces(p + 1, false, Self::qualified) else {
            return [None, None];
        };
        let mut parts = 0;
        loop {
            let amp = self.space_end(at);
            if self.at(amp) != b'&' {
                break;
            }
            let Some((_, end)) = self.after_spaces(amp + 1, false, Self::qualified) else {
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
            let atom = self.after_spaces(sep + 1, false, |lexer, at| {
                let ends = lexer.type_atom(at);
                ends[0].map(|_| ends)
            });
            for end in atom.map(|(_, ends)| ends).into_iter().flatten().flatten() {
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
