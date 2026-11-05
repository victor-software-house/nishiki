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
//! `^` holds only at the start of the line: Shiki's engine does not match it
//! after the line feed that ends the line.
//! `.` matches any character but the line feed, and a negated class matches
//! the line feed too.
//! `\w`, `\s` and `\d` read Unicode, through [`crate::classes`].

mod declarations;
mod nodes;
mod strings;

use super::engine::Lexer;
use super::grammar::{CONTROL_FLOW, KEYWORDS, Pat, STATEMENT_KEYWORDS};
use crate::rules::Chars;

impl Lexer<'_> {
    /// Whether `pat` matches at `p`, and where its match ends.
    ///
    /// The arms come in groups, in this order:
    /// - declarations, parameters and calls;
    /// - node paths and strings;
    /// - keywords, names and numbers;
    /// - comments, operators and brackets.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm for each expression of the grammar"
    )]
    pub(super) fn matcher(&mut self, pat: Pat, p: usize) -> Option<usize> {
        match pat {
            Pat::ExtendsStatement => self.extends(p),
            Pat::GetterSetterGodot4At0 => self.getter(p),
            Pat::GetterSetterGodot4At1 => self.setter(p),
            Pat::LambdaDeclaration => self.lambda(p),
            Pat::LambdaDeclarationEnd => self
                .group_lit(p, b":")
                .or_else(|| self.closes_head(p).then_some(p)),
            Pat::Parameters | Pat::FunctionArguments => self.group_lit(p, b"("),
            Pat::ParametersEnd | Pat::FunctionCallEnd | Pat::NodepathFunctionEnd => {
                self.group_lit(p, b")")
            }
            Pat::AnnotatedParameter => self.annotated_parameter(p),
            Pat::AnnotatedParameterEnd | Pat::LooseDefaultEnd => self
                .group_lit(p, b",")
                .or_else(|| self.starts(p, b")").then_some(p)),
            Pat::AnnotatedParameterAt1 | Pat::FunctionArgumentsAt2 => self.lone_equals(p),
            Pat::VariableDeclarationAt1 => self.lit(p, b":=").or_else(|| self.lone_equals(p)),
            Pat::ParametersAt1 => self.parameter(p),
            Pat::LooseDefault => self.group_lit(p, b"="),
            Pat::ClassDeclaration => self.class_declaration(p),
            Pat::VariableDeclaration => self.variable_keyword(p),
            Pat::VariableDeclarationEnd => {
                if self.at_eol(p) {
                    Some(p)
                } else {
                    self.lit(p, b";")
                }
            }
            Pat::VariableDeclarationAt0 => self.legacy_accessor(p),
            Pat::VariableDeclarationAt2 => self.type_annotation(p),
            Pat::VariableDeclarationAt3 => self.setget(p),
            Pat::SignalDeclarationBare => self.signal_bare(p),
            Pat::SignalDeclaration => self.named_head(p, b"signal"),
            Pat::FunctionDeclaration => self.named_head(p, b"func"),
            Pat::SignalDeclarationEnd => self.closes_head(p).then_some(p),
            Pat::FunctionDeclarationEnd => self.group_lit(p, b":").or_else(|| self.line_feed(p)),
            Pat::FunctionCall => self.call_ahead(p),
            Pat::FunctionNameAt1 => self.word_group(p, &[b"preload"]),
            Pat::FunctionNameAt2 => self.name_group(p),
            Pat::FunctionArgumentsEnd => self.arguments_end(p),
            Pat::FunctionArgumentsAt0 => self.group_lit(p, b","),
            Pat::FunctionArgumentsAt1 => self.named_argument(p),
            Pat::FunctionArgumentsAt4 => self.chained_call(p),
            Pat::ClassName => self.class_name(p),
            Pat::ClassNew => self.class_new(p),
            Pat::ClassIs => self.class_is(p),
            Pat::ClassEnum => self.class_enum(p),
            Pat::AnyMethod => self.any_method(p),
            Pat::AnyVariable => self.any_variable(p),
            Pat::AnyProperty => self.any_property(p),

            Pat::BuiltinGetNodeShorthandQuoted => self.quoted_node(p),
            Pat::BuiltinGetNodeShorthandQuotedEnd
            | Pat::NodepathObjectAt0End
            | Pat::NodepathFunctionAt0End
            | Pat::StringsEnd => self.repeated(p),
            Pat::BuiltinGetNodeShorthandQuotedAt0
            | Pat::NodepathObjectAt0At0
            | Pat::NodepathFunctionAt0At0 => self.lit(p, b"%"),
            Pat::BuiltinGetNodeShorthandBare => self.bare_node(p),
            Pat::BuiltinGetNodeShorthandBareMulti => self.node_head(p),
            Pat::BuiltinGetNodeShorthandBareMultiEnd => self.node_path_end(p),
            Pat::BuiltinGetNodeShorthandBareMultiAt0 => self.node_segment(p),
            Pat::NodepathObject => self.node_path_object(p),
            Pat::NodepathObjectAt0 | Pat::NodepathFunctionAt0 => self.quote_group(p),
            Pat::NodepathFunction => self.node_path_function(p),
            Pat::Strings => self.string_open(p),
            Pat::StringsAt0 => self.escape(p),
            Pat::StringPercentPlaceholders => self.percent_placeholder(p),
            Pat::StringBracketPlaceholdersAt0 => self.brace_placeholder(p),
            Pat::StringBracketPlaceholdersAt1 => self.nested_brace_placeholder(p),

            Pat::BuiltinClasses => self.builtin_class(p),
            Pat::ConstVars => self.const_var(p),
            Pat::Keywords => self.word_in(p, KEYWORDS),
            Pat::ControlFlow => self.word_in(p, CONTROL_FLOW),
            Pat::WordlikeOperator => self.word_in(p, &[b"and", b"or", b"not"]),
            Pat::Letter => self.word_in(p, &[b"true", b"false", b"null"]),
            Pat::StatementKeywordAt0 => self.statement_keyword(p, STATEMENT_KEYWORDS),
            Pat::StatementKeywordAt1 => self.statement_keyword(p, &[b"class"]),
            Pat::StatementKeywordAt2 => self.case_or_match(p, &[b"case", b"match"], true),
            Pat::MatchKeyword => self.case_or_match(p, &[b"match"], false),
            Pat::InKeywordAt0 => self.word_group(p, &[b"for"]),
            Pat::InKeywordAt0End => self.lit(p, b":"),
            Pat::InKeywordAt0At0 | Pat::InKeywordAt1 => self.word_in(p, &[b"in"]),
            Pat::Func => self.word_in(p, &[b"func"]),
            Pat::Annotations => self.annotation(p),
            Pat::PascalCaseClass => self.pascal_case(p),
            Pat::NumbersAt0 => self.radix(p, b"0b", |b| matches!(b, b'0' | b'1' | b'_')),
            Pat::NumbersAt1 => self.radix(p, b"0x", |b| b == b'_' || b.is_ascii_hexdigit()),
            Pat::NumbersAt2 => self.point_number(p),
            Pat::NumbersAt3 => self.decimal(p, false),
            Pat::NumbersAt4 => self.decimal(p, true),
            Pat::NumbersAt5 => self.exponent_integer(p),
            Pat::NumbersAt6 => self.integer(p),

            Pat::Comment => self.comment(p),
            Pat::Region => self.region(p),
            Pat::LineContinuationAt0 => self.continued_text(p),
            Pat::LineContinuationAt1 => self.continuation(p),
            Pat::LineContinuationAt1End => self.continuation_end(p),
            Pat::AssignmentOperator => self.lit(p, b"="),
            Pat::BooleanOperator => self.first_group(p, &[b"&&", b"||"]),
            Pat::ArithmeticOperator => self
                .first_lit(
                    p,
                    &[
                        b"->", b"+=", b"-=", b"**=", b"*=", b"^=", b"/=", b"%=", b"&=", b"~=",
                        b"|=", b"**",
                    ],
                )
                .or_else(|| self.one_of(p, b"-%*+/")),
            Pat::BitwiseOperator => self
                .one_of(p, b"&|")
                .or_else(|| self.first_lit(p, &[b"<<=", b">>=", b"<<", b">>"]))
                .or_else(|| self.one_of(p, b"^~")),
            Pat::CompareOperator => self
                .first_lit(p, &[b"<=", b">=", b"=="])
                .or_else(|| self.one_of(p, b"<>"))
                .or_else(|| self.first_lit(p, &[b"!=", b"!"])),
            Pat::CurlyBraces => self.lit(p, b"{"),
            Pat::CurlyBracesEnd => self.lit(p, b"}"),
            Pat::SquareBraces => self.lit(p, b"["),
            Pat::SquareBracesEnd => self.lit(p, b"]"),
            Pat::RoundBraces => self.lit(p, b"("),
            Pat::RoundBracesEnd | Pat::NodepathObjectEnd => self.lit(p, b")"),
        }
    }

    /// `(text)`: a literal, with group 1 around it.
    fn group_lit(&mut self, p: usize, text: &[u8]) -> Option<usize> {
        let end = self.lit(p, text)?;
        self.cap(1, p, end);
        Some(end)
    }

    /// `(a|b)`: the first of `options` at `p`, with group 1 around it.
    fn first_group(&mut self, p: usize, options: &[&[u8]]) -> Option<usize> {
        let end = self.first_lit(p, options)?;
        self.cap(1, p, end);
        Some(end)
    }

    /// The end of the first of `options` at `p`, in their order: an
    /// alternation of literals, which takes the first that matches.
    fn first_lit(&self, p: usize, options: &[&[u8]]) -> Option<usize> {
        options.iter().find_map(|&option| self.lit(p, option))
    }

    /// `[…]` for the bytes `set`: one of them at `p`.
    fn one_of(&self, p: usize, set: &[u8]) -> Option<usize> {
        (p < self.lim() && set.contains(&self.at(p))).then_some(p + 1)
    }

    /// `\n`: the line feed that ends the line.
    fn line_feed(&self, p: usize) -> Option<usize> {
        (p == self.lim() && self.at(p) == b'\n').then_some(p + 1)
    }

    /// `\b(word)\b` for a word of `words`, with group 1 around it.
    fn word_group(&mut self, p: usize, words: &[&[u8]]) -> Option<usize> {
        let end = self.word_in(p, words)?;
        self.cap(1, p, end);
        Some(end)
    }

    /// The end of `[A-Z_a-z]\w*` at `p`.
    /// A run of `\w` ends where `\b` holds, so the name gives nothing back.
    fn name(&self, p: usize) -> Option<usize> {
        let b = self.at(p);
        (p < self.lim() && (b == b'_' || b.is_ascii_alphabetic())).then(|| self.word_end(p + 1))
    }

    /// `\b([A-Z_a-z]\w*)\b`: a name, with group 1 around it.
    fn name_group(&mut self, p: usize) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        let end = self.name(p)?;
        self.cap(1, p, end);
        Some(end)
    }

    /// The end of the run of `\s` from `p` that stops before the line feed.
    /// A lookahead such as `(?=[\n#)=])` after it still sees the line feed.
    fn blanks_end(&self, p: usize) -> usize {
        self.space_end(p).min(self.lim().max(p))
    }

    /// `=(?!=)`.
    fn lone_equals(&self, p: usize) -> Option<usize> {
        (self.starts(p, b"=") && self.at(p + 1) != b'=').then_some(p + 1)
    }

    /// `(?=[\n"#'])`: a line feed, a quote or a comment ahead.
    fn closes_head(&self, p: usize) -> bool {
        self.avail(p) && matches!(self.at(p), b'\n' | b'"' | b'#' | b'\'')
    }

    /// `\s+` at `p`: where the spaces end, when there is one.
    fn spaces1(&self, p: usize) -> Option<usize> {
        let end = self.space_end(p);
        (end > p).then_some(end)
    }
}
