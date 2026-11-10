//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups of its match with [`Lexer::capture`].
//! The engine names those the grammar names.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would.
//! Where a quantifier could give characters back, the doc comment shows why it
//! cannot matter, or how the matcher gives them back.
//! An expression that PHP's grammar spells the same way uses PHP's matcher,
//! from [`crate::php_family`].
//!
//! A regular expression sees the line feed at the end of a line.
//! So `\s` and a negated class such as `[^;]` match it there.
//!
//! Hack's names are `[A-Z_a-z\x7F-ÿ][0-9A-Z_a-z\x7F-ÿ]*`: the class stops at
//! U+00FF, so a name is read by character.
//! `(?i)` lets `[a-z]` match `ſ` and the Kelvin sign too, as Oniguruma folds
//! them to `s` and `k`.

mod code;
mod strings;
mod xhp;

use code::Module;

use super::engine::Lexer;
use super::grammar::{Pattern, table_patterns};
use crate::rules::Chars;

/// The characters that keep the end of a path from matching.
#[derive(Clone, Copy, Debug)]
enum PathChars {
    /// `(?i)[0-9\\_a-z]`.
    Folded,
    /// `(?i)[$0-9\\_a-z]`.
    FoldedDollar,
    /// `[$0-9A-Z\\_a-z]`, with case as written.
    Dollar,
}

/// `[A-Z_a-z\x7F-ÿ]`: a character that starts a name.
fn is_name_start(c: char) -> bool {
    c == '_' || c.is_ascii_alphabetic() || ('\x7f'..='\u{ff}').contains(&c)
}

/// `[0-9A-Z_a-z\x7F-ÿ]`: a character of a name.
fn is_name(c: char) -> bool {
    is_name_start(c) || c.is_ascii_digit()
}

/// `(?i)[_a-z]`: an ASCII letter, `_`, `ſ` or the Kelvin sign.
fn is_iletter(c: char) -> bool {
    c == '_' || c.is_ascii_alphabetic() || c == 'ſ' || c == '\u{212a}'
}

/// `(?i)[0-9_a-z]`.
fn is_iword(c: char) -> bool {
    is_iletter(c) || c.is_ascii_digit()
}

/// `(?i)[0-9\\_a-z]`.
fn is_ipath(c: char) -> bool {
    is_iword(c) || c == '\\'
}

/// `(?i)[_a-z\x7F-ÿ]`: the class and every character that `(?i)` folds into
/// it, such as `Ÿ` into `ÿ` and `ẞ` into `ß`.
fn is_iname_start(c: char) -> bool {
    is_name_start(c)
        || matches!(
            c,
            'ſ' | '\u{212a}' | '\u{212b}' | 'Ÿ' | 'μ' | 'Μ' | '\u{1e9e}'
        )
}

/// `(?i)[0-9_a-z\x7F-ÿ]`.
fn is_iname(c: char) -> bool {
    is_iname_start(c) || c.is_ascii_digit()
}

/// `[A-Z_a-z]`.
fn is_ident_start(b: u8) -> bool {
    b == b'_' || b.is_ascii_alphabetic()
}

/// `[0-9A-Z_a-z]`.
fn is_ident(b: u8) -> bool {
    is_ident_start(b) || b.is_ascii_digit()
}

impl Lexer<'_> {
    /// Whether `pattern` matches at `p`, and where its match ends.
    ///
    /// The arms come in groups, in this order:
    /// - comments and doc comments;
    /// - attributes, XHP, interfaces, generics and type annotations;
    /// - class names and namespaces;
    /// - strings, regular expressions, SQL strings and heredocs;
    /// - the declarations and statements of `#language`;
    /// - variables, calls, operators and numbers;
    /// - the word tables.
    pub(super) fn matcher(&mut self, pattern: Pattern, p: usize) -> Option<usize> {
        match pattern {
            Pattern::CommentsAt0 => self.doc_open(p),
            Pattern::CommentsAt0End | Pattern::CommentsAt1End => self.lit(p, b"*/"),
            Pattern::PhpDocAt0 => self.doc_plain_line(p),
            Pattern::PhpDocAt1 => self.doc_access(p),
            Pattern::PhpDocAt2 => self.doc_xlink(p),
            Pattern::PhpDocAt3 => self.doc_tag(p),
            Pattern::PhpDocAt4 => self.doc_link(p),
            Pattern::CommentsAt1 => self.lit(p, b"/*"),
            Pattern::CommentsAt2 => self.line_comment_lead(p),
            Pattern::CommentsAt2End => (!self.at_g(p)).then_some(p),
            Pattern::CommentsAt2At0 => self.lit(p, b"//"),
            Pattern::CommentsAt2At0End => self.comment_end(p),

            Pattern::LanguageAt1 => self.attribute_line(p),
            Pattern::LanguageAt1End => (p >= 2 && self.text(p - 2, p) == b">>").then_some(p),
            Pattern::Attributes => self.attribute_open(p),
            Pattern::AttributesEnd => self.capture_lit(p, b">>"),
            Pattern::AttributesAt1 => self.ident(p),
            Pattern::AttributesAt2 | Pattern::LanguageAt18At0 | Pattern::LanguageAt19At1 => {
                self.capture_lit(p, b"(")
            }
            Pattern::AttributesAt2End
            | Pattern::LanguageAt18At0End
            | Pattern::LanguageAt18At1End
            | Pattern::LanguageAt19At2 => self.capture_lit(p, b")"),
            Pattern::Xhp => self.xhp_open(p),
            Pattern::XhpEnd => (p < self.lim()).then_some(p),
            Pattern::XhpTagElementName => self.xhp_tag(p),
            Pattern::XhpTagElementNameEnd => self.xhp_tag_end(p),
            Pattern::XhpTagTermination => self.xhp_tag_close(p),
            Pattern::XhpTagTerminationEnd => self.capture_lit(p, b"</"),
            Pattern::XhpEvaluatedCode | Pattern::LanguageAt50 | Pattern::LanguageAt6End => {
                self.lit(p, b"{")
            }
            Pattern::XhpEvaluatedCodeEnd | Pattern::LanguageAt50End => self.lit(p, b"}"),
            Pattern::XhpEntitiesAt0 => self.xhp_entity(p),
            Pattern::XhpEntitiesAt1 => self.xhp_loose_entity(p),
            Pattern::XhpHtmlComments => self.lit(p, b"<!--"),
            Pattern::XhpHtmlCommentsEnd => self.xhp_comment_end(p),
            Pattern::XhpHtmlCommentsAt0 => self.xhp_comment_dashes(p),
            Pattern::XhpAttributeName => self.xhp_attribute_name(p),
            Pattern::XhpAssignment => self.xhp_assignment(p),
            Pattern::XhpStringDoubleQuotedEnd => self.unescaped_quote(p, b'"'),
            Pattern::XhpStringSingleQuotedEnd => self.unescaped_quote(p, b'\''),
            Pattern::Interface => self.interface(p),
            Pattern::InterfaceEnd
            | Pattern::LanguageAt12End
            | Pattern::LanguageAt19End
            | Pattern::LanguageAt19At2End
            | Pattern::WhereClauseEnd
            | Pattern::ImplementsEnd => matches!(self.at(p), b';' | b'{').then_some(p),
            Pattern::InterfaceAt1 | Pattern::LanguageAt5At0 => self.word_group(p, &[b"extends"]),
            Pattern::InterfaceAt4 => {
                let end = self.chars_while(p, is_iword);
                (end > p).then_some(end)
            }
            Pattern::Generics => self.capture_lit(p, b"<"),
            Pattern::GenericsEnd => self.capture_lit(p, b">"),
            Pattern::GenericsAt2 => self.generic_parameter(p),
            Pattern::TypeAnnotationAt0 => self.generic_type(p),
            Pattern::TypeAnnotationAt0End => self.lit(p, b">"),
            Pattern::TypeAnnotationAt1 => self.word_in(
                p,
                &[
                    b"bool",
                    b"int",
                    b"float",
                    b"string",
                    b"resource",
                    b"mixed",
                    b"arraykey",
                    b"nonnull",
                    b"dict",
                    b"vec",
                    b"keyset",
                ],
            ),
            Pattern::TypeAnnotationAt2 => self.lit(p, b"shape("),
            Pattern::TypeAnnotationAt2End => self.shape_close(p),
            Pattern::TypeAnnotationAt3
            | Pattern::FunctionArgumentsAt3At0At0
            | Pattern::SqlStringDoubleQuotedAt0 => self.lit(p, b"("),
            Pattern::TypeAnnotationAt3End
            | Pattern::FunctionArgumentsAt3At0At0End
            | Pattern::LanguageAt15End
            | Pattern::LanguageAt25End => self.lit(p, b")"),

            Pattern::ClassNameAt0 | Pattern::ImplementsAt1At0 => self.ahead_qualified(p),
            Pattern::ClassNameAt0End
            | Pattern::ClassNameAt2End
            | Pattern::ConstantsAt0End
            | Pattern::ImplementsAt1At0End => self.last_segment(p),
            Pattern::Namespace => self.namespace_part(p),
            Pattern::NamespaceEnd => self.namespace_end(p),
            Pattern::NamespaceAt0 => {
                let end = self.chars_while(p, is_iword);
                (end > p && self.at(end) == b'\\').then_some(end)
            }
            Pattern::NamespaceAt1 => self.capture_lit(p, b"\\"),
            Pattern::ClassNameAt2 => {
                (is_ident_start(self.at(p)) || self.at(p) == b'\\').then_some(p)
            }
            Pattern::ImplementsAt1 => self.char_is(p, is_ipath).then_some(p),
            Pattern::ImplementsAt1End => self.implements_separator(p),
            Pattern::ImplementsAt1At3 => self.iname(p),
            Pattern::Implements => self.word_then_spaces(p, b"implements"),
            Pattern::LanguageAt12At3 => self.word_then_spaces(p, b"extends"),
            Pattern::Instantiation => self.word_then_spaces(p, b"new"),
            Pattern::LanguageAt12At3End => self.not_path_ahead(p, PathChars::Folded),
            Pattern::InstantiationEnd => self.not_path_ahead(p, PathChars::FoldedDollar),
            Pattern::LanguageAt42End => self.not_path_ahead(p, PathChars::Dollar),

            Pattern::RegexDoubleQuoted => self.regex_open(p, b'"'),
            Pattern::RegexSingleQuoted => self.regex_open(p, b'\''),
            Pattern::RegexDoubleQuotedEnd => self.regex_close(p, b'"'),
            Pattern::RegexSingleQuotedEnd => self.regex_close(p, b'\''),
            Pattern::RegexDoubleQuotedAt0 => self.regex_escape(p),
            Pattern::RegexSingleQuotedAt2 => self.single_regex_escape(p),
            Pattern::RegexDoubleQuotedAt2 => self.regex_repeat(p),
            Pattern::RegexDoubleQuotedAt3 | Pattern::RegexSingleQuotedAt3 => {
                self.regex_class_open(p)
            }
            Pattern::RegexDoubleQuotedAt3End
            | Pattern::RegexSingleQuotedAt3End
            | Pattern::LanguageAt48End => self.lit(p, b"]"),
            Pattern::RegexDoubleQuotedAt4 => self.regex_operator(p),
            Pattern::RegexSingleQuotedAt3At0 => self.regex_class_escape(p),
            Pattern::InterpolationAt0 => self.octal_escape(p),
            Pattern::InterpolationAt1 => self.hex_escape(p),
            Pattern::InterpolationAt2 => (self.at(p) == b'\\'
                && matches!(self.at(p + 1), b'"' | b'$' | b'\\' | b'n' | b'r' | b't'))
            .then_some(p + 2),
            Pattern::InterpolationAt3 => self.brace_variable(p),
            Pattern::InterpolationAt4 => self.simple_variable(p),
            Pattern::SqlStringDoubleQuoted => self.sql_open(p, b'"'),
            Pattern::SqlStringSingleQuoted => self.sql_open(p, b'\''),
            Pattern::SqlStringDoubleQuotedEnd
            | Pattern::XhpStringDoubleQuoted
            | Pattern::StringDoubleQuoted
            | Pattern::StringDoubleQuotedEnd => self.lit(p, b"\""),
            Pattern::SqlStringDoubleQuotedAt1 => self.sql_comment(p, b"#", b'"'),
            Pattern::SqlStringDoubleQuotedAt2 => self.sql_comment(p, b"--", b'"'),
            Pattern::SqlStringSingleQuotedAt1 => self.sql_comment(p, b"#", b'\''),
            Pattern::SqlStringSingleQuotedAt2 => self.sql_comment(p, b"--", b'\''),
            Pattern::SqlStringDoubleQuotedAt3 => self.sql_escape(p),
            Pattern::SqlStringDoubleQuotedAt4 => self.sql_quote(p, b'\'', b'"'),
            Pattern::SqlStringDoubleQuotedAt5 => self.sql_quote(p, b'`', b'"'),
            Pattern::SqlStringSingleQuotedAt4 => self.sql_quote(p, b'`', b'\''),
            Pattern::SqlStringSingleQuotedAt5 => self.sql_quote(p, b'"', b'\''),
            Pattern::SqlStringDoubleQuotedAt6
            | Pattern::XhpStringSingleQuoted
            | Pattern::SqlStringDoubleQuotedAt6End
            | Pattern::SqlStringSingleQuotedEnd
            | Pattern::StringSingleQuoted
            | Pattern::StringSingleQuotedEnd => self.lit(p, b"'"),
            Pattern::SqlStringDoubleQuotedAt7 | Pattern::SqlStringDoubleQuotedAt7End => {
                self.lit(p, b"`")
            }
            Pattern::StringSingleQuotedAt0 => self.single_quote_escape(p),
            Pattern::HeredocAt0 => self.heredoc_open(p, b'"'),
            Pattern::HeredocAt1 => self.heredoc_open(p, b'\''),
            Pattern::HeredocAt0End | Pattern::HeredocAt1End => self.heredoc_close(p),

            Pattern::ConstantsAt0 => self.ahead_namespaced_constant(p),
            Pattern::ConstantsAt1 => {
                let at = p + usize::from(self.at(p) == b'\\');
                self.char_is(at, is_name_start).then_some(p)
            }
            Pattern::ConstantsAt1End => (self.avail(p)
                && !(self.char_is(p, is_name_start) || self.at(p) == b'\\'))
                .then_some(p),
            Pattern::ConstantsAt1At0 => self.language_constant(p),
            Pattern::LanguageAt4 => self.typedef(p),
            Pattern::LanguageAt4End | Pattern::LanguageAt17End => self.capture_lit(p, b";"),
            Pattern::LanguageAt4At2 | Pattern::FunctionArgumentsAt3At0 => self.capture_lit(p, b"="),
            Pattern::LanguageAt5 => self.enum_class(p),
            Pattern::LanguageAt5End | Pattern::LanguageAt7End | Pattern::LanguageAt8End => {
                (self.at(p) == b'{').then_some(p)
            }
            Pattern::LanguageAt6 => self.enum_declaration(p),
            Pattern::LanguageAt7 => self.trait_declaration(p),
            Pattern::LanguageAt8 => self.module_declaration(p, Module::New),
            Pattern::LanguageAt9 => self.module_declaration(p, Module::Plain),
            Pattern::LanguageAt9End | Pattern::LanguageAt14End => {
                (self.at_eol(p) || self.at(p) == b';' || self.space_at(p) > 0).then_some(p)
            }
            Pattern::LanguageAt11At1At1End => {
                (self.at_eol(p) || matches!(self.at(p), b',' | b';') || self.space_at(p) > 0)
                    .then_some(p)
            }
            Pattern::LanguageAt10 => self.namespace_declaration(p),
            Pattern::LanguageAt10End => self.namespace_declaration_end(p),
            Pattern::LanguageAt10At0 => self.lit(p, b"\\"),
            Pattern::LanguageAt11 => self.use_statement(p),
            Pattern::LanguageAt11End => self.use_end(p),
            Pattern::LanguageAt11At1 | Pattern::LanguageAt11At1At1 => {
                let at = self.space_end(p);
                self.char_is(at, is_ipath).then_some(at)
            }
            Pattern::LanguageAt11At1End => self.use_alias(p),
            Pattern::LanguageAt11At2 => {
                let comma = self.space_end(p);
                (self.at(comma) == b',').then(|| self.space_end(comma + 1))
            }
            Pattern::Nested0At0 => {
                self.first_lit(p, &[b"final", b"abstract", b"public", b"internal"])
            }
            Pattern::Nested2At0 => self.first_lit(
                p,
                &[
                    b"final",
                    b"abstract",
                    b"public",
                    b"private",
                    b"protected",
                    b"internal",
                    b"static",
                    b"async",
                ],
            ),
            Pattern::LanguageAt12 => self.class_declaration(p),
            Pattern::LanguageAt13 => self.control_keyword(p),
            Pattern::LanguageAt14 => self.include(p),
            Pattern::LanguageAt15 => self.catch(p),
            Pattern::Nested1At0 => self.name(p),
            Pattern::ConstantsAt1At5 => self.hname(p),
            Pattern::Nested1At1 => self.lit(p, b"|"),
            Pattern::LanguageAt15At1 => self.caught(p),
            Pattern::LanguageAt16 => self.exception_keyword(p),
            Pattern::LanguageAt17 => self.const_declaration(p),
            Pattern::LanguageAt17At1 => self.const_name(p),
            Pattern::LanguageAt17At3 => self.lit(p, b"="),
            Pattern::LanguageAt18 => self.closure(p),
            Pattern::LanguageAt18End => matches!(self.at(p), b')' | b'{').then_some(p + 1),
            Pattern::LanguageAt18At1 => self.closure_use(p),
            Pattern::LanguageAt18At1At0 => self.closure_use_variable(p),
            Pattern::FunctionArgumentsAt3 => self.parameter(p),
            Pattern::FunctionArgumentsAt3End => self.parameter_end(p),
            Pattern::FunctionArgumentsAt3At0End => matches!(self.at(p), b')' | b',').then_some(p),
            Pattern::LanguageAt19At1End => (self.at(p) == b')').then_some(p),
            Pattern::LanguageAt19 => self.function_declaration(p),
            Pattern::FunctionReturnType => self.capture_lit(p, b":"),
            Pattern::FunctionReturnTypeEnd => (matches!(self.at(p), b';' | b'{')
                || self.keyword(p, b"where").is_some())
            .then_some(p),
            Pattern::WhereClause => self.word_group(p, &[b"where"]),
            Pattern::WhereClauseAt1 => self.word_group(p, &[b"as", b"super"]),

            Pattern::InvokeCall => self.invoke_call(p),
            Pattern::LanguageAt21 => self.ahead_static_access(p),
            Pattern::LanguageAt21End => self.static_access(p),
            Pattern::LanguageAt21At0 => self
                .first_lit(p, &[b"self", b"static", b"parent"])
                .filter(|&end| self.word_at(end).is_none()),
            Pattern::VarGlobal => self.global_variable(p),
            Pattern::VarGlobalSafer => self.superglobal(p),
            Pattern::VariableNameAt2 => self.variable(p),
            Pattern::VariableNameAt3 => self.braced_variable(p),
            Pattern::VarBasic => self.basic_variable(p),
            Pattern::VariablesAt3 => self.variable_brace_open(p),
            Pattern::VariablesAt3End | Pattern::ObjectAt0End | Pattern::LiteralCollectionsEnd => {
                self.capture_lit(p, b"}")
            }
            Pattern::LanguageAt24 => self.empty_array(p),
            Pattern::LanguageAt25 => self.array_open(p),
            Pattern::LanguageAt26 => self.cast(p),
            Pattern::LanguageAt27 => self.type_keyword(p),
            Pattern::LanguageAt28 => self.modifier_keyword(p),
            Pattern::ObjectAt0 => self.object_brace(p),
            Pattern::ObjectAt1 => self.object_member(p),
            Pattern::LanguageAt30 => self.lit(p, b";"),
            Pattern::LanguageAt32 => self.lit(p, b".=").or_else(|| self.lit(p, b".")),
            Pattern::LanguageAt33 => self.lit(p, b"=>"),
            Pattern::LanguageAt34 => self.lit(p, b"==>"),
            Pattern::LanguageAt35 => self.lit(p, b"|>"),
            Pattern::LanguageAt36 => self.equality(p),
            Pattern::LanguageAt37 => self.assignment(p),
            Pattern::LanguageAt38 => self.comparison(p),
            Pattern::LanguageAt39 => self.lit(p, b"--").or_else(|| self.lit(p, b"++")),
            Pattern::LanguageAt40 => {
                matches!(self.at(p), b'-' | b'%' | b'*' | b'+' | b'/').then_some(p + 1)
            }
            Pattern::LanguageAt41 => self
                .lit(p, b"!")
                .or_else(|| self.lit(p, b"&&"))
                .or_else(|| self.lit(p, b"||")),
            Pattern::LanguageAt42 => self.type_operator(p),
            Pattern::LanguageAt43 => self.iword_in(p, &[b"is", b"as", b"upcast"]),
            Pattern::FunctionCallAt0 => self.ahead_qualified_call(p),
            Pattern::FunctionCallAt0End | Pattern::FunctionCallAt2End => {
                self.ahead_call(p).then_some(p)
            }
            Pattern::UserFunctionCall => self.ahead_user_call(p),
            Pattern::UserFunctionCallEnd => {
                let end = self.iname(p)?;
                self.ahead_call(end).then_some(end)
            }
            Pattern::FunctionCallAt1 => self.iword_in(p, &[b"print", b"echo"]),
            Pattern::FunctionCallAt2 => self.call_name(p),
            Pattern::FunctionCallAt2At0 => self.builtin_call(p),
            Pattern::LanguageAt45 => self.bitwise(p),
            Pattern::Numbers => self.number(p),
            Pattern::InstantiationAt0 => self.instantiated_keyword(p),
            Pattern::LanguageAt48 => self.lit(p, b"["),
            Pattern::LiteralCollections => self.literal_collection(p),
            table_patterns!() => {
                let (table, list) = pattern.words()?;
                self.table_list(table.words(), list, p)
            }
        }
    }

    /// The end of the run of characters from `p` that `class` accepts.
    fn chars_while(&self, mut p: usize, class: fn(char) -> bool) -> usize {
        while p < self.lim() {
            match self.char_at(p) {
                Some((c, end)) if class(c) => p = end,
                Some(_) | None => break,
            }
        }
        p
    }

    /// Whether the character at `p` is one that `class` accepts.
    fn char_is(&self, p: usize, class: fn(char) -> bool) -> bool {
        p < self.lim() && self.char_at(p).is_some_and(|(c, _)| class(c))
    }

    /// `[A-Z_a-z][0-9A-Z_a-z]*`.
    fn ident(&self, p: usize) -> Option<usize> {
        is_ident_start(self.at(p)).then(|| self.skip(p + 1, is_ident))
    }

    /// `(?i)[_a-z][0-9_a-z]*`.
    fn iname(&self, p: usize) -> Option<usize> {
        self.char_is(p, is_iletter)
            .then(|| self.chars_while(p, is_iword))
    }

    /// `[A-Z_a-z\x7F-ÿ][0-9A-Z_a-z\x7F-ÿ]*`: a Hack name.
    fn hname(&self, p: usize) -> Option<usize> {
        self.char_is(p, is_name_start)
            .then(|| self.chars_while(p, is_name))
    }

    /// `(literal)`, with the literal as group 1.
    fn capture_lit(&mut self, p: usize, text: &[u8]) -> Option<usize> {
        let end = self.lit(p, text)?;
        self.capture(1, p, end);
        Some(end)
    }

    /// The first of `words` at `p`, with no `\b` around it.
    fn first_lit(&self, p: usize, words: &[&[u8]]) -> Option<usize> {
        words.iter().find_map(|word| self.lit(p, word))
    }

    /// `\bword\b`, case as written.
    fn keyword(&self, p: usize, word: &[u8]) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let end = self.lit(p, word)?;
        self.word_at(end).is_none().then_some(end)
    }

    /// `\b(a|b|…)\b`, case as written, with the word as group 1.
    fn word_group(&mut self, p: usize, words: &[&[u8]]) -> Option<usize> {
        let end = self.word_in(p, words)?;
        self.capture(1, p, end);
        Some(end)
    }

    /// `(?=[^…])`: a character ahead that `chars` does not hold.
    fn not_path_ahead(&self, p: usize, chars: PathChars) -> Option<usize> {
        if !self.avail(p) {
            return None;
        }
        let blocked = match chars {
            PathChars::Folded => self.char_is(p, is_ipath),
            PathChars::FoldedDollar => self.at(p) == b'$' || self.char_is(p, is_ipath),
            PathChars::Dollar => self.at(p) == b'$' || self.at(p) == b'\\' || is_ident(self.at(p)),
        };
        (!blocked).then_some(p)
    }

    /// `(?=\s*\()`.
    fn ahead_call(&self, p: usize) -> bool {
        self.at(self.space_end(p)) == b'('
    }
}
