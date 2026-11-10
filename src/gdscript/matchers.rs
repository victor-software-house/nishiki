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
//! `^` holds only at the start of the line: Shiki's engine does not match it
//! after the line feed that ends the line.
//! `.` matches any character but the line feed, and a negated class matches
//! the line feed too.
//! `\w`, `\s` and `\d` read Unicode, through [`crate::classes`].

mod declarations;
mod nodes;
mod strings;

use super::engine::Lexer;
use super::grammar::{CONTROL_FLOW, KEYWORDS, Pattern, STATEMENT_KEYWORDS};
use crate::rules::Chars;

impl Lexer<'_> {
    /// Whether `pattern` matches at `p`, and where its match ends.
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
    pub(super) fn matcher(&mut self, pattern: Pattern, p: usize) -> Option<usize> {
        match pattern {
            Pattern::ExtendsStatement => self.extends(p),
            Pattern::GetterSetterGodot4At0 => self.getter(p),
            Pattern::GetterSetterGodot4At1 => self.setter(p),
            Pattern::LambdaDeclaration => self.lambda(p),
            Pattern::LambdaDeclarationEnd => self
                .group_lit(p, b":")
                .or_else(|| self.closes_head(p).then_some(p)),
            Pattern::Parameters | Pattern::FunctionArguments => self.group_lit(p, b"("),
            Pattern::ParametersEnd | Pattern::FunctionCallEnd | Pattern::NodepathFunctionEnd => {
                self.group_lit(p, b")")
            }
            Pattern::AnnotatedParameter => self.annotated_parameter(p),
            Pattern::AnnotatedParameterEnd | Pattern::LooseDefaultEnd => self
                .group_lit(p, b",")
                .or_else(|| self.starts(p, b")").then_some(p)),
            Pattern::AnnotatedParameterAt1 | Pattern::FunctionArgumentsAt2 => self.lone_equals(p),
            Pattern::VariableDeclarationAt1 => self.lit(p, b":=").or_else(|| self.lone_equals(p)),
            Pattern::ParametersAt1 => self.parameter(p),
            Pattern::LooseDefault => self.group_lit(p, b"="),
            Pattern::ClassDeclaration => self.class_declaration(p),
            Pattern::VariableDeclaration => self.variable_keyword(p),
            Pattern::VariableDeclarationEnd => {
                if self.at_eol(p) {
                    Some(p)
                } else {
                    self.lit(p, b";")
                }
            }
            Pattern::VariableDeclarationAt0 => self.legacy_accessor(p),
            Pattern::VariableDeclarationAt2 => self.type_annotation(p),
            Pattern::VariableDeclarationAt3 => self.setget(p),
            Pattern::SignalDeclarationBare => self.signal_bare(p),
            Pattern::SignalDeclaration => self.named_head(p, b"signal"),
            Pattern::FunctionDeclaration => self.named_head(p, b"func"),
            Pattern::SignalDeclarationEnd => self.closes_head(p).then_some(p),
            Pattern::FunctionDeclarationEnd => {
                self.group_lit(p, b":").or_else(|| self.line_feed(p))
            }
            Pattern::FunctionCall => self.call_ahead(p),
            Pattern::FunctionNameAt1 => self.word_group(p, &[b"preload"]),
            Pattern::FunctionNameAt2 => self.name_group(p),
            Pattern::FunctionArgumentsEnd => self.arguments_end(p),
            Pattern::FunctionArgumentsAt0 => self.group_lit(p, b","),
            Pattern::FunctionArgumentsAt1 => self.named_argument(p),
            Pattern::FunctionArgumentsAt4 => self.chained_call(p),
            Pattern::ClassName => self.class_name(p),
            Pattern::ClassNew => self.class_new(p),
            Pattern::ClassIs => self.class_is(p),
            Pattern::ClassEnum => self.class_enum(p),
            Pattern::AnyMethod => self.any_method(p),
            Pattern::AnyVariable => self.any_variable(p),
            Pattern::AnyProperty => self.any_property(p),

            Pattern::BuiltinGetNodeShorthandQuoted => self.quoted_node(p),
            Pattern::BuiltinGetNodeShorthandQuotedEnd
            | Pattern::NodepathObjectAt0End
            | Pattern::NodepathFunctionAt0End
            | Pattern::StringsEnd => self.repeated(p),
            Pattern::BuiltinGetNodeShorthandQuotedAt0
            | Pattern::NodepathObjectAt0At0
            | Pattern::NodepathFunctionAt0At0 => self.lit(p, b"%"),
            Pattern::BuiltinGetNodeShorthandBare => self.bare_node(p),
            Pattern::BuiltinGetNodeShorthandBareMulti => self.node_head(p),
            Pattern::BuiltinGetNodeShorthandBareMultiEnd => self.node_path_end(p),
            Pattern::BuiltinGetNodeShorthandBareMultiAt0 => self.node_segment(p),
            Pattern::NodepathObject => self.node_path_object(p),
            Pattern::NodepathObjectAt0 | Pattern::NodepathFunctionAt0 => self.quote_group(p),
            Pattern::NodepathFunction => self.node_path_function(p),
            Pattern::Strings => self.string_open(p),
            Pattern::StringsAt0 => self.escape(p),
            Pattern::StringPercentPlaceholders => self.percent_placeholder(p),
            Pattern::StringBracketPlaceholdersAt0 => self.brace_placeholder(p),
            Pattern::StringBracketPlaceholdersAt1 => self.nested_brace_placeholder(p),

            Pattern::BuiltinClasses => self.builtin_class(p),
            Pattern::ConstVars => self.const_var(p),
            Pattern::Keywords => self.word_in(p, KEYWORDS),
            Pattern::ControlFlow => self.word_in(p, CONTROL_FLOW),
            Pattern::WordlikeOperator => self.word_in(p, &[b"and", b"or", b"not"]),
            Pattern::Letter => self.word_in(p, &[b"true", b"false", b"null"]),
            Pattern::StatementKeywordAt0 => self.statement_keyword(p, STATEMENT_KEYWORDS),
            Pattern::StatementKeywordAt1 => self.statement_keyword(p, &[b"class"]),
            Pattern::StatementKeywordAt2 => self.case_or_match(p, &[b"case", b"match"], true),
            Pattern::MatchKeyword => self.case_or_match(p, &[b"match"], false),
            Pattern::InKeywordAt0 => self.word_group(p, &[b"for"]),
            Pattern::InKeywordAt0End => self.lit(p, b":"),
            Pattern::InKeywordAt0At0 | Pattern::InKeywordAt1 => self.word_in(p, &[b"in"]),
            Pattern::Func => self.word_in(p, &[b"func"]),
            Pattern::Annotations => self.annotation(p),
            Pattern::PascalCaseClass => self.pascal_case(p),
            Pattern::NumbersAt0 => self.radix(p, b"0b", |b| matches!(b, b'0' | b'1' | b'_')),
            Pattern::NumbersAt1 => self.radix(p, b"0x", |b| b == b'_' || b.is_ascii_hexdigit()),
            Pattern::NumbersAt2 => self.point_number(p),
            Pattern::NumbersAt3 => self.decimal(p, false),
            Pattern::NumbersAt4 => self.decimal(p, true),
            Pattern::NumbersAt5 => self.exponent_integer(p),
            Pattern::NumbersAt6 => self.integer(p),

            Pattern::Comment => self.comment(p),
            Pattern::Region => self.region(p),
            Pattern::LineContinuationAt0 => self.continued_text(p),
            Pattern::LineContinuationAt1 => self.continuation(p),
            Pattern::LineContinuationAt1End => self.continuation_end(p),
            Pattern::AssignmentOperator => self.lit(p, b"="),
            Pattern::BooleanOperator => self.first_group(p, &[b"&&", b"||"]),
            Pattern::ArithmeticOperator => self
                .first_lit(
                    p,
                    &[
                        b"->", b"+=", b"-=", b"**=", b"*=", b"^=", b"/=", b"%=", b"&=", b"~=",
                        b"|=", b"**",
                    ],
                )
                .or_else(|| self.one_of(p, b"-%*+/")),
            Pattern::BitwiseOperator => self
                .one_of(p, b"&|")
                .or_else(|| self.first_lit(p, &[b"<<=", b">>=", b"<<", b">>"]))
                .or_else(|| self.one_of(p, b"^~")),
            Pattern::CompareOperator => self
                .first_lit(p, &[b"<=", b">=", b"=="])
                .or_else(|| self.one_of(p, b"<>"))
                .or_else(|| self.first_lit(p, &[b"!=", b"!"])),
            Pattern::CurlyBraces => self.lit(p, b"{"),
            Pattern::CurlyBracesEnd => self.lit(p, b"}"),
            Pattern::SquareBraces => self.lit(p, b"["),
            Pattern::SquareBracesEnd => self.lit(p, b"]"),
            Pattern::RoundBraces => self.lit(p, b"("),
            Pattern::RoundBracesEnd | Pattern::NodepathObjectEnd => self.lit(p, b")"),
        }
    }

    /// `(text)`: a literal, with group 1 around it.
    fn group_lit(&mut self, p: usize, text: &[u8]) -> Option<usize> {
        let end = self.lit(p, text)?;
        self.capture(1, p, end);
        Some(end)
    }

    /// `(a|b)`: the first of `options` at `p`, with group 1 around it.
    fn first_group(&mut self, p: usize, options: &[&[u8]]) -> Option<usize> {
        let end = self.first_lit(p, options)?;
        self.capture(1, p, end);
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
        self.capture(1, p, end);
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
        self.capture(1, p, end);
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
