//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups of its match with [`Lexer::cap`].
//! The engine names those the grammar names.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would.
//! Where a quantifier could give characters back, the doc comment says why it
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
use super::grammar::{Pat, table_pats};
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
    /// Whether `pat` matches at `p`, and where its match ends.
    ///
    /// The arms come in groups, in this order:
    /// - comments and doc comments;
    /// - attributes, XHP, interfaces, generics and type annotations;
    /// - class names and namespaces;
    /// - strings, regular expressions, SQL strings and heredocs;
    /// - the declarations and statements of `#language`;
    /// - variables, calls, operators and numbers;
    /// - the word tables.
    pub(super) fn matcher(&mut self, pat: Pat, p: usize) -> Option<usize> {
        match pat {
            Pat::CommentsAt0 => self.doc_open(p),
            Pat::CommentsAt0End | Pat::CommentsAt1End => self.lit(p, b"*/"),
            Pat::PhpDocAt0 => self.doc_plain_line(p),
            Pat::PhpDocAt1 => self.doc_access(p),
            Pat::PhpDocAt2 => self.doc_xlink(p),
            Pat::PhpDocAt3 => self.doc_tag(p),
            Pat::PhpDocAt4 => self.doc_link(p),
            Pat::CommentsAt1 => self.lit(p, b"/*"),
            Pat::CommentsAt2 => self.line_comment_lead(p),
            Pat::CommentsAt2End => (!self.at_g(p)).then_some(p),
            Pat::CommentsAt2At0 => self.lit(p, b"//"),
            Pat::CommentsAt2At0End => self.comment_end(p),

            Pat::LanguageAt1 => self.attribute_line(p),
            Pat::LanguageAt1End => (p >= 2 && self.text(p - 2, p) == b">>").then_some(p),
            Pat::Attributes => self.attribute_open(p),
            Pat::AttributesEnd => self.cap_lit(p, b">>"),
            Pat::AttributesAt1 => self.ident(p),
            Pat::AttributesAt2 | Pat::LanguageAt18At0 | Pat::LanguageAt19At1 => {
                self.cap_lit(p, b"(")
            }
            Pat::AttributesAt2End
            | Pat::LanguageAt18At0End
            | Pat::LanguageAt18At1End
            | Pat::LanguageAt19At2 => self.cap_lit(p, b")"),
            Pat::Xhp => self.xhp_open(p),
            Pat::XhpEnd => (p < self.lim()).then_some(p),
            Pat::XhpTagElementName => self.xhp_tag(p),
            Pat::XhpTagElementNameEnd => self.xhp_tag_end(p),
            Pat::XhpTagTermination => self.xhp_tag_close(p),
            Pat::XhpTagTerminationEnd => self.cap_lit(p, b"</"),
            Pat::XhpEvaluatedCode | Pat::LanguageAt50 | Pat::LanguageAt6End => self.lit(p, b"{"),
            Pat::XhpEvaluatedCodeEnd | Pat::LanguageAt50End => self.lit(p, b"}"),
            Pat::XhpEntitiesAt0 => self.xhp_entity(p),
            Pat::XhpEntitiesAt1 => self.xhp_loose_entity(p),
            Pat::XhpHtmlComments => self.lit(p, b"<!--"),
            Pat::XhpHtmlCommentsEnd => self.xhp_comment_end(p),
            Pat::XhpHtmlCommentsAt0 => self.xhp_comment_dashes(p),
            Pat::XhpAttributeName => self.xhp_attribute_name(p),
            Pat::XhpAssignment => self.xhp_assignment(p),
            Pat::XhpStringDoubleQuotedEnd => self.unescaped_quote(p, b'"'),
            Pat::XhpStringSingleQuotedEnd => self.unescaped_quote(p, b'\''),
            Pat::Interface => self.interface(p),
            Pat::InterfaceEnd
            | Pat::LanguageAt12End
            | Pat::LanguageAt19End
            | Pat::LanguageAt19At2End
            | Pat::WhereClauseEnd
            | Pat::ImplementsEnd => matches!(self.at(p), b';' | b'{').then_some(p),
            Pat::InterfaceAt1 | Pat::LanguageAt5At0 => self.word_group(p, &[b"extends"]),
            Pat::InterfaceAt4 => {
                let end = self.chars_while(p, is_iword);
                (end > p).then_some(end)
            }
            Pat::Generics => self.cap_lit(p, b"<"),
            Pat::GenericsEnd => self.cap_lit(p, b">"),
            Pat::GenericsAt2 => self.generic_parameter(p),
            Pat::TypeAnnotationAt0 => self.generic_type(p),
            Pat::TypeAnnotationAt0End => self.lit(p, b">"),
            Pat::TypeAnnotationAt1 => self.word_in(
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
            Pat::TypeAnnotationAt2 => self.lit(p, b"shape("),
            Pat::TypeAnnotationAt2End => self.shape_close(p),
            Pat::TypeAnnotationAt3
            | Pat::FunctionArgumentsAt3At0At0
            | Pat::SqlStringDoubleQuotedAt0 => self.lit(p, b"("),
            Pat::TypeAnnotationAt3End
            | Pat::FunctionArgumentsAt3At0At0End
            | Pat::LanguageAt15End
            | Pat::LanguageAt25End => self.lit(p, b")"),

            Pat::ClassNameAt0 | Pat::ImplementsAt1At0 => self.ahead_qualified(p),
            Pat::ClassNameAt0End
            | Pat::ClassNameAt2End
            | Pat::ConstantsAt0End
            | Pat::ImplementsAt1At0End => self.last_segment(p),
            Pat::Namespace => self.namespace_part(p),
            Pat::NamespaceEnd => self.namespace_end(p),
            Pat::NamespaceAt0 => {
                let end = self.chars_while(p, is_iword);
                (end > p && self.at(end) == b'\\').then_some(end)
            }
            Pat::NamespaceAt1 => self.cap_lit(p, b"\\"),
            Pat::ClassNameAt2 => (is_ident_start(self.at(p)) || self.at(p) == b'\\').then_some(p),
            Pat::ImplementsAt1 => self.char_is(p, is_ipath).then_some(p),
            Pat::ImplementsAt1End => self.implements_separator(p),
            Pat::ImplementsAt1At3 => self.iname(p),
            Pat::Implements => self.word_then_spaces(p, b"implements"),
            Pat::LanguageAt12At3 => self.word_then_spaces(p, b"extends"),
            Pat::Instantiation => self.word_then_spaces(p, b"new"),
            Pat::LanguageAt12At3End => self.not_path_ahead(p, PathChars::Folded),
            Pat::InstantiationEnd => self.not_path_ahead(p, PathChars::FoldedDollar),
            Pat::LanguageAt42End => self.not_path_ahead(p, PathChars::Dollar),

            Pat::RegexDoubleQuoted => self.regex_open(p, b'"'),
            Pat::RegexSingleQuoted => self.regex_open(p, b'\''),
            Pat::RegexDoubleQuotedEnd => self.regex_close(p, b'"'),
            Pat::RegexSingleQuotedEnd => self.regex_close(p, b'\''),
            Pat::RegexDoubleQuotedAt0 => self.regex_escape(p),
            Pat::RegexSingleQuotedAt2 => self.single_regex_escape(p),
            Pat::RegexDoubleQuotedAt2 => self.regex_repeat(p),
            Pat::RegexDoubleQuotedAt3 | Pat::RegexSingleQuotedAt3 => self.regex_class_open(p),
            Pat::RegexDoubleQuotedAt3End | Pat::RegexSingleQuotedAt3End | Pat::LanguageAt48End => {
                self.lit(p, b"]")
            }
            Pat::RegexDoubleQuotedAt4 => self.regex_operator(p),
            Pat::RegexSingleQuotedAt3At0 => self.regex_class_escape(p),
            Pat::InterpolationAt0 => self.octal_escape(p),
            Pat::InterpolationAt1 => self.hex_escape(p),
            Pat::InterpolationAt2 => (self.at(p) == b'\\'
                && matches!(self.at(p + 1), b'"' | b'$' | b'\\' | b'n' | b'r' | b't'))
            .then_some(p + 2),
            Pat::InterpolationAt3 => self.brace_variable(p),
            Pat::InterpolationAt4 => self.simple_variable(p),
            Pat::SqlStringDoubleQuoted => self.sql_open(p, b'"'),
            Pat::SqlStringSingleQuoted => self.sql_open(p, b'\''),
            Pat::SqlStringDoubleQuotedEnd
            | Pat::XhpStringDoubleQuoted
            | Pat::StringDoubleQuoted
            | Pat::StringDoubleQuotedEnd => self.lit(p, b"\""),
            Pat::SqlStringDoubleQuotedAt1 => self.sql_comment(p, b"#", b'"'),
            Pat::SqlStringDoubleQuotedAt2 => self.sql_comment(p, b"--", b'"'),
            Pat::SqlStringSingleQuotedAt1 => self.sql_comment(p, b"#", b'\''),
            Pat::SqlStringSingleQuotedAt2 => self.sql_comment(p, b"--", b'\''),
            Pat::SqlStringDoubleQuotedAt3 => self.sql_escape(p),
            Pat::SqlStringDoubleQuotedAt4 => self.sql_quote(p, b'\'', b'"'),
            Pat::SqlStringDoubleQuotedAt5 => self.sql_quote(p, b'`', b'"'),
            Pat::SqlStringSingleQuotedAt4 => self.sql_quote(p, b'`', b'\''),
            Pat::SqlStringSingleQuotedAt5 => self.sql_quote(p, b'"', b'\''),
            Pat::SqlStringDoubleQuotedAt6
            | Pat::XhpStringSingleQuoted
            | Pat::SqlStringDoubleQuotedAt6End
            | Pat::SqlStringSingleQuotedEnd
            | Pat::StringSingleQuoted
            | Pat::StringSingleQuotedEnd => self.lit(p, b"'"),
            Pat::SqlStringDoubleQuotedAt7 | Pat::SqlStringDoubleQuotedAt7End => self.lit(p, b"`"),
            Pat::StringSingleQuotedAt0 => self.single_quote_escape(p),
            Pat::HeredocAt0 => self.heredoc_open(p, b'"'),
            Pat::HeredocAt1 => self.heredoc_open(p, b'\''),
            Pat::HeredocAt0End | Pat::HeredocAt1End => self.heredoc_close(p),

            Pat::ConstantsAt0 => self.ahead_namespaced_constant(p),
            Pat::ConstantsAt1 => {
                let at = p + usize::from(self.at(p) == b'\\');
                self.char_is(at, is_name_start).then_some(p)
            }
            Pat::ConstantsAt1End => (self.avail(p)
                && !(self.char_is(p, is_name_start) || self.at(p) == b'\\'))
                .then_some(p),
            Pat::ConstantsAt1At0 => self.language_constant(p),
            Pat::LanguageAt4 => self.typedef(p),
            Pat::LanguageAt4End | Pat::LanguageAt17End => self.cap_lit(p, b";"),
            Pat::LanguageAt4At2 | Pat::FunctionArgumentsAt3At0 => self.cap_lit(p, b"="),
            Pat::LanguageAt5 => self.enum_class(p),
            Pat::LanguageAt5End | Pat::LanguageAt7End | Pat::LanguageAt8End => {
                (self.at(p) == b'{').then_some(p)
            }
            Pat::LanguageAt6 => self.enum_declaration(p),
            Pat::LanguageAt7 => self.trait_declaration(p),
            Pat::LanguageAt8 => self.module_declaration(p, Module::New),
            Pat::LanguageAt9 => self.module_declaration(p, Module::Plain),
            Pat::LanguageAt9End | Pat::LanguageAt14End => {
                (self.at_eol(p) || self.at(p) == b';' || self.space_at(p) > 0).then_some(p)
            }
            Pat::LanguageAt11At1At1End => {
                (self.at_eol(p) || matches!(self.at(p), b',' | b';') || self.space_at(p) > 0)
                    .then_some(p)
            }
            Pat::LanguageAt10 => self.namespace_declaration(p),
            Pat::LanguageAt10End => self.namespace_declaration_end(p),
            Pat::LanguageAt10At0 => self.lit(p, b"\\"),
            Pat::LanguageAt11 => self.use_statement(p),
            Pat::LanguageAt11End => self.use_end(p),
            Pat::LanguageAt11At1 | Pat::LanguageAt11At1At1 => {
                let at = self.space_end(p);
                self.char_is(at, is_ipath).then_some(at)
            }
            Pat::LanguageAt11At1End => self.use_alias(p),
            Pat::LanguageAt11At2 => {
                let comma = self.space_end(p);
                (self.at(comma) == b',').then(|| self.space_end(comma + 1))
            }
            Pat::Nested0At0 => self.first_lit(p, &[b"final", b"abstract", b"public", b"internal"]),
            Pat::Nested2At0 => self.first_lit(
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
            Pat::LanguageAt12 => self.class_declaration(p),
            Pat::LanguageAt13 => self.control_keyword(p),
            Pat::LanguageAt14 => self.include(p),
            Pat::LanguageAt15 => self.catch(p),
            Pat::Nested1At0 => self.name(p),
            Pat::ConstantsAt1At5 => self.hname(p),
            Pat::Nested1At1 => self.lit(p, b"|"),
            Pat::LanguageAt15At1 => self.caught(p),
            Pat::LanguageAt16 => self.exception_keyword(p),
            Pat::LanguageAt17 => self.const_declaration(p),
            Pat::LanguageAt17At1 => self.const_name(p),
            Pat::LanguageAt17At3 => self.lit(p, b"="),
            Pat::LanguageAt18 => self.closure(p),
            Pat::LanguageAt18End => matches!(self.at(p), b')' | b'{').then_some(p + 1),
            Pat::LanguageAt18At1 => self.closure_use(p),
            Pat::LanguageAt18At1At0 => self.closure_use_variable(p),
            Pat::FunctionArgumentsAt3 => self.parameter(p),
            Pat::FunctionArgumentsAt3End => self.parameter_end(p),
            Pat::FunctionArgumentsAt3At0End => matches!(self.at(p), b')' | b',').then_some(p),
            Pat::LanguageAt19At1End => (self.at(p) == b')').then_some(p),
            Pat::LanguageAt19 => self.function_declaration(p),
            Pat::FunctionReturnType => self.cap_lit(p, b":"),
            Pat::FunctionReturnTypeEnd => (matches!(self.at(p), b';' | b'{')
                || self.keyword(p, b"where").is_some())
            .then_some(p),
            Pat::WhereClause => self.word_group(p, &[b"where"]),
            Pat::WhereClauseAt1 => self.word_group(p, &[b"as", b"super"]),

            Pat::InvokeCall => self.invoke_call(p),
            Pat::LanguageAt21 => self.ahead_static_access(p),
            Pat::LanguageAt21End => self.static_access(p),
            Pat::LanguageAt21At0 => self
                .first_lit(p, &[b"self", b"static", b"parent"])
                .filter(|&end| self.word_at(end).is_none()),
            Pat::VarGlobal => self.global_variable(p),
            Pat::VarGlobalSafer => self.superglobal(p),
            Pat::VariableNameAt2 => self.variable(p),
            Pat::VariableNameAt3 => self.braced_variable(p),
            Pat::VarBasic => self.basic_variable(p),
            Pat::VariablesAt3 => self.variable_brace_open(p),
            Pat::VariablesAt3End | Pat::ObjectAt0End | Pat::LiteralCollectionsEnd => {
                self.cap_lit(p, b"}")
            }
            Pat::LanguageAt24 => self.empty_array(p),
            Pat::LanguageAt25 => self.array_open(p),
            Pat::LanguageAt26 => self.cast(p),
            Pat::LanguageAt27 => self.type_keyword(p),
            Pat::LanguageAt28 => self.modifier_keyword(p),
            Pat::ObjectAt0 => self.object_brace(p),
            Pat::ObjectAt1 => self.object_member(p),
            Pat::LanguageAt30 => self.lit(p, b";"),
            Pat::LanguageAt32 => self.lit(p, b".=").or_else(|| self.lit(p, b".")),
            Pat::LanguageAt33 => self.lit(p, b"=>"),
            Pat::LanguageAt34 => self.lit(p, b"==>"),
            Pat::LanguageAt35 => self.lit(p, b"|>"),
            Pat::LanguageAt36 => self.equality(p),
            Pat::LanguageAt37 => self.assignment(p),
            Pat::LanguageAt38 => self.comparison(p),
            Pat::LanguageAt39 => self.lit(p, b"--").or_else(|| self.lit(p, b"++")),
            Pat::LanguageAt40 => {
                matches!(self.at(p), b'-' | b'%' | b'*' | b'+' | b'/').then_some(p + 1)
            }
            Pat::LanguageAt41 => self
                .lit(p, b"!")
                .or_else(|| self.lit(p, b"&&"))
                .or_else(|| self.lit(p, b"||")),
            Pat::LanguageAt42 => self.type_operator(p),
            Pat::LanguageAt43 => self.iword_in(p, &[b"is", b"as", b"upcast"]),
            Pat::FunctionCallAt0 => self.ahead_qualified_call(p),
            Pat::FunctionCallAt0End | Pat::FunctionCallAt2End => self.ahead_call(p).then_some(p),
            Pat::UserFunctionCall => self.ahead_user_call(p),
            Pat::UserFunctionCallEnd => {
                let end = self.iname(p)?;
                self.ahead_call(end).then_some(end)
            }
            Pat::FunctionCallAt1 => self.iword_in(p, &[b"print", b"echo"]),
            Pat::FunctionCallAt2 => self.call_name(p),
            Pat::FunctionCallAt2At0 => self.builtin_call(p),
            Pat::LanguageAt45 => self.bitwise(p),
            Pat::Numbers => self.number(p),
            Pat::InstantiationAt0 => self.instantiated_keyword(p),
            Pat::LanguageAt48 => self.lit(p, b"["),
            Pat::LiteralCollections => self.literal_collection(p),
            table_pats!() => {
                let (table, list) = pat.words()?;
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
    fn cap_lit(&mut self, p: usize, text: &[u8]) -> Option<usize> {
        let end = self.lit(p, text)?;
        self.cap(1, p, end);
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
        self.cap(1, p, end);
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
