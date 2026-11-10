//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups the grammar names with
//! [`Lexer::capture`]. The engine names them.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would.
//! Where a quantifier could give characters back, the doc comment shows why it
//! cannot matter, or what giving them back leaves to test.

mod lookahead;
mod operators;
mod strings;

use super::engine::Lexer;
use super::grammar::Pattern;
use crate::rules::Chars;

/// Whether `b` can start a name: the grammar's `[A-Z_a-z]`.
fn is_name_start(b: u8) -> bool {
    b == b'_' || b.is_ascii_alphabetic()
}

/// Whether `b` can go on a name: the grammar's `[-'0-9A-Z_a-z]`.
fn is_name(b: u8) -> bool {
    matches!(b, b'-' | b'\'' | b'_') || b.is_ascii_alphanumeric()
}

/// The words of the reserved-word rule, `bad-reserved`.
const RESERVED: &[&[u8]] = &[
    b"if", b"then", b"else", b"assert", b"with", b"let", b"in", b"rec", b"inherit",
];

/// The builtins that the grammar names `support.function`.
const FUNCTIONS: &[&[u8]] = &[
    b"scopedImport",
    b"import",
    b"isNull",
    b"abort",
    b"throw",
    b"baseNameOf",
    b"dirOf",
    b"removeAttrs",
    b"map",
    b"toString",
    b"derivationStrict",
    b"derivation",
];

/// Which lookahead ends a rule that runs until a closing delimiter.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Stop {
    /// `(?=([]),;}]|\b(else|then)\b))`.
    Close,
    /// `(?=([]),;}]|\b(in|else|then)\b))`, which also ends at `in`.
    CloseOrIn,
    /// `(?=([]);}]|\b(else|then)\b))`, which a comma does not end.
    CloseNoComma,
}

impl Lexer<'_> {
    /// Whether `pattern` matches at `p`, and where its match ends.
    ///
    /// The arms come in groups, in this order:
    /// - the rules that run until a closing delimiter, and their ends;
    /// - parentheses, lists and attribute sets;
    /// - strings, paths, URIs and interpolations;
    /// - function heads and their parameters;
    /// - `let`, `if`, `with` and `assert`;
    /// - names, constants and operators;
    /// - comments, blanks and what the grammar marks illegal.
    #[expect(clippy::too_many_lines, reason = "one arm per pattern of the grammar")]
    pub(super) fn matcher(&mut self, pattern: Pattern, p: usize) -> Option<usize> {
        match pattern {
            Pattern::ExpressionCont
            | Pattern::FunctionDefinition
            | Pattern::AttrsetDefinitionBraceOpenedAt1
            | Pattern::FunctionDefinitionBraceOpened
            | Pattern::FunctionDefinitionBraceOpenedAt1
            | Pattern::FunctionDefinitionBraceOpenedAt1At1
            | Pattern::FunctionDefinitionAt1 => Some(p),
            Pattern::ParensAndContEnd
            | Pattern::ListAndContEnd
            | Pattern::StringAt0End
            | Pattern::StringAt1End
            | Pattern::ExpressionContEnd
            | Pattern::StringAt2End
            | Pattern::StringAt3End
            | Pattern::StringAt4End
            | Pattern::FunctionForSureAt0End
            | Pattern::FunctionDefinitionEnd
            | Pattern::FunctionBodyFromColonEnd
            | Pattern::AttrsetForSureAt0End
            | Pattern::AttrsetDefinitionEnd
            | Pattern::AttrsetForSureAt1End
            | Pattern::AttrsetOrFunctionAt0End
            | Pattern::AttrsetDefinitionBraceOpenedAt0End
            | Pattern::LetEnd
            | Pattern::LetAt0At0End
            | Pattern::LetAt0At0At1End
            | Pattern::LetAt1End
            | Pattern::IfEnd
            | Pattern::IfAt2End
            | Pattern::ConstantsAt0End
            | Pattern::ConstantsAt1End
            | Pattern::ConstantsAt2End
            | Pattern::ParameterNameAndContEnd
            | Pattern::AttrsetOrFunctionAt1End
            | Pattern::FunctionDefinitionBraceOpenedEnd
            | Pattern::AttrsetOrFunctionAt3At0End
            | Pattern::AttrsetOrFunctionAt3At1End
            | Pattern::AttrsetOrFunctionAt3At2End
            | Pattern::AttrsetOrFunctionAt3At3End
            | Pattern::AttrsetOrFunctionAt3At3At1End
            | Pattern::AttrsetDefinitionAt1End => self.stops_at(p, Stop::Close),
            Pattern::LetAt0End => self.stops_at(p, Stop::CloseOrIn),
            Pattern::AttrsetOrFunctionEnd | Pattern::AttrsetOrFunctionAt3End => {
                self.stops_at(p, Stop::CloseNoComma)
            }

            Pattern::ParensAndCont => self.peek(p, b"("),
            Pattern::Parens | Pattern::AttributeInheritAt0 => self.lit(p, b"("),
            Pattern::ParensEnd | Pattern::AttributeInheritAt0At0 => self.lit(p, b")"),
            Pattern::ListAndCont => self.peek(p, b"["),
            Pattern::List => self.lit(p, b"["),
            Pattern::ListEnd => self.lit(p, b"]"),
            Pattern::AttrsetDefinition
            | Pattern::AttrsetForSureAt0At0End
            | Pattern::LetAt0At0
            | Pattern::FunctionHeaderUntilColonNoArg
            | Pattern::FunctionDefinitionAt1At1
            | Pattern::FunctionHeaderUntilColonWithArg => self.peek(p, b"{"),
            Pattern::AttrsetDefinitionAt0
            | Pattern::AttrsetOrFunction
            | Pattern::LetAt0At0At0
            | Pattern::FunctionHeaderOpenBrace => self.lit(p, b"{"),
            Pattern::AttrsetDefinitionAt0End
            | Pattern::AttrsetDefinitionBraceOpenedAt1End
            | Pattern::LetAt0At0At0End
            | Pattern::InterpolationEnd
            | Pattern::FunctionHeaderCloseBraceWithArg
            | Pattern::FunctionHeaderCloseBraceNoArg => self.lit(p, b"}"),
            Pattern::FunctionDefinitionBraceOpenedAt1At1End
            | Pattern::FunctionHeaderOpenBraceEnd => self.peek(p, b"}"),
            Pattern::AttrsetDefinitionBraceOpenedAt0 | Pattern::AttrsetDefinitionAt1 => {
                (self.prev(p) == Some(b'}')).then_some(p)
            }
            Pattern::AttrsetForSureAt0 => self.keyword(p, b"rec").map(|_| p),
            Pattern::AttrsetForSureAt0At0 => self.keyword(p, b"rec"),
            Pattern::AttrsetForSureAt1 => self.attrset_for_sure(p),
            Pattern::AttrsetOrFunctionAt0 => self.attrset_ahead(p),
            Pattern::AttrsetOrFunctionAt1 => self.parameters_ahead(p),
            Pattern::AttributeInherit => self.keyword(p, b"inherit"),
            Pattern::AttributeInheritEnd
            | Pattern::AttributeBindFromEqualsEnd
            | Pattern::WithAssertEnd => self.lit(p, b";"),
            Pattern::AttributeInheritAt0End
            | Pattern::AttributeInheritAt0At0End
            | Pattern::AttributeInheritAt1End => self.peek(p, b";"),
            Pattern::AttributeInheritAt1 => is_name_start(self.at(p)).then_some(p),
            Pattern::AttributeNameAt1 => self.lit(p, b"."),
            Pattern::AttributeBindFromEquals => self.lit(p, b"="),
            Pattern::AttrsetOrFunctionAt3At0 => self.peek(p, b"."),
            Pattern::AttrsetOrFunctionAt3At1 => self.spaced_comma(p),
            Pattern::AttrsetOrFunctionAt3At2 => self.peek(p, b"="),
            Pattern::AttrsetOrFunctionAt3At3 => self.peek(p, b"?"),
            Pattern::AttrsetOrFunctionAt3At3At1 => self.lit(p, b","),

            Pattern::StringAt0 => self.starts(p, b"''").then_some(p),
            Pattern::StringAt0At0 => self.lit(p, b"''"),
            Pattern::StringAt0At0End => self.indented_close(p),
            Pattern::StringAt0At0At0 => self.indented_escape(p),
            Pattern::StringAt1 => self.peek(p, b"\""),
            Pattern::StringQuoted | Pattern::StringQuotedEnd => self.lit(p, b"\""),
            Pattern::StringQuotedAt0 => self.escape(p),
            Pattern::Interpolation => self.lit(p, b"${"),
            Pattern::StringAt2 => self.path_text(p),
            Pattern::StringAt3 => self.search_path(p),
            Pattern::StringAt4 => self.uri(p),

            Pattern::FunctionForSureAt0 => self.function_ahead(p),
            Pattern::FunctionBodyFromColon => self.lit(p, b":"),
            Pattern::FunctionDefinitionBraceOpenedAt1End
            | Pattern::FunctionHeaderCloseBraceWithArgEnd
            | Pattern::FunctionHeaderTerminalArgEnd
            | Pattern::FunctionHeaderTerminalArgAt0End
            | Pattern::FunctionHeaderTerminalArgAt0At0End
            | Pattern::FunctionDefinitionAt1End
            | Pattern::FunctionDefinitionAt1At0End
            | Pattern::FunctionDefinitionAt1At0At0End
            | Pattern::FunctionHeaderUntilColonNoArgEnd
            | Pattern::FunctionHeaderCloseBraceNoArgEnd
            | Pattern::FunctionDefinitionAt1At1End
            | Pattern::FunctionHeaderUntilColonWithArgEnd => self.peek(p, b":"),
            Pattern::FunctionHeaderTerminalArg => self.peek(p, b"@"),
            Pattern::FunctionHeaderTerminalArgAt0 | Pattern::FunctionDefinitionAt1At0At0 => {
                self.lit(p, b"@")
            }
            Pattern::FunctionParameterAt0 => self.lit(p, b"..."),
            Pattern::FunctionParameterAt0End | Pattern::FunctionParameterAt1End => {
                self.parameter_end(p)
            }
            Pattern::FunctionParameterDefault => self.lit(p, b"?"),
            Pattern::FunctionParameterDefaultEnd => self.peek(p, b",}"),

            Pattern::Let => self.keyword(p, b"let").map(|_| p),
            Pattern::LetAt0 => self.keyword(p, b"let"),
            Pattern::LetAt0At0At1 => (p == 0 || self.prev(p) == Some(b'}')).then_some(p),
            Pattern::LetAt1 => self.keyword(p, b"in"),
            Pattern::If => self.keyword(p, b"if").map(|_| p),
            Pattern::IfAt0 => self.keyword(p, b"if"),
            Pattern::IfAt0End => self.keyword(p, b"then").map(|_| p + 2),
            Pattern::IfAt1 => self.keyword_tail(p, b"th", b"en"),
            Pattern::IfAt1End => self.keyword(p, b"else").map(|_| p + 2),
            Pattern::IfAt2 => self.keyword_tail(p, b"el", b"se"),
            Pattern::WithAssert => self.reserved(p, &[b"with", b"assert"]),
            Pattern::BadReserved => self.reserved(p, RESERVED),

            Pattern::AttributeNameSingle
            | Pattern::AttributeNameAt0
            | Pattern::ParameterNameAndCont
            | Pattern::FunctionParameterAt1
            | Pattern::AttrsetOrFunctionAt3
            | Pattern::FunctionHeaderTerminalArgAt0At0
            | Pattern::FunctionDefinitionAt1At0
            | Pattern::ParameterName => self.name(p),
            Pattern::ConstantsAt0 => self.word_in(p, &[b"builtins", b"true", b"false", b"null"]),
            Pattern::ConstantsAt1 => self.word_in(p, FUNCTIONS),
            Pattern::ConstantsAt2 => self.integer(p),
            Pattern::OperatorUnary => matches!(self.at(p), b'-' | b'!').then_some(p + 1),
            Pattern::OperatorBinary => self.binary_operator(p),

            Pattern::CommentAt0 => self.block_comment_open(p),
            Pattern::CommentAt0End => self.lit(p, b"*/"),
            Pattern::CommentAt1 => self.lit(p, b"#"),
            Pattern::CommentAt1End => self.at_eol(p).then_some(p),
            Pattern::Whitespace => Some(self.space_end(p)).filter(|&end| end > p),
            Pattern::Illegal => (p < self.lim()).then(|| self.char_end(p)),
        }
    }

    /// The lookahead `(?=[set])` at `p`, for a set of ASCII bytes.
    fn peek(&self, p: usize, set: &[u8]) -> Option<usize> {
        set.contains(&self.at(p)).then_some(p)
    }

    /// The end of `[A-Z_a-z][-'0-9A-Z_a-z]*` at `p`.
    /// The run takes no character the next part of any expression could use,
    /// so the longest is the only one tried.
    fn name_end(&self, p: usize) -> Option<usize> {
        is_name_start(self.at(p)).then(|| self.skip(p + 1, is_name))
    }

    /// `\b[A-Z_a-z][-'0-9A-Z_a-z]*`, a name.
    fn name(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        self.name_end(p)
    }

    /// The lookahead that ends a rule at a closing delimiter or a keyword.
    fn stops_at(&self, p: usize, stop: Stop) -> Option<usize> {
        let closes = match self.at(p) {
            b']' | b')' | b';' | b'}' => true,
            b',' => stop != Stop::CloseNoComma,
            _ => false,
        };
        let keyword = || {
            self.keyword(p, b"else").is_some()
                || self.keyword(p, b"then").is_some()
                || (stop == Stop::CloseOrIn && self.keyword(p, b"in").is_some())
        };
        (closes || keyword()).then_some(p)
    }

    /// `(?<=head)tail\b` at `p`, the second half of a keyword the grammar
    /// splits, as `(?<=th)en\b` is.
    fn keyword_tail(&self, p: usize, head: &[u8], tail: &[u8]) -> Option<usize> {
        let start = p.checked_sub(head.len())?;
        if self.text(start, p) != head {
            return None;
        }
        let end = self.lit(p, tail)?;
        self.boundary(end).then_some(end)
    }

    /// `(?<![-'\w])(a|b|...)(?![-'\w])` at `p`, for the words `words`.
    ///
    /// The lookahead refuses a word that a name character follows.
    /// A word that is a prefix of another is then tried again as that other.
    /// So the match is the whole run of `[-'\w]` characters, when it is one of
    /// the words.
    fn reserved(&self, p: usize, words: &[&[u8]]) -> Option<usize> {
        if matches!(self.prev(p), Some(b'-' | b'\'')) || self.word_before(p) {
            return None;
        }
        let end = self.reserved_run(p);
        words.contains(&self.text(p, end)).then_some(end)
    }

    /// The end of the run of `[-'\w]` characters from `p`.
    fn reserved_run(&self, mut p: usize) -> usize {
        let from = p;
        loop {
            if matches!(self.at(p), b'-' | b'\'') {
                p += 1;
            } else if let Some(width) = self.word_at(p) {
                p += width;
            } else {
                break;
            }
        }
        self.spend(p - from);
        p
    }

    /// `\b[0-9]+\b`.
    /// A shorter run leaves a digit next, which `\b` refuses after a digit.
    fn integer(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) || !self.at(p).is_ascii_digit() {
            return None;
        }
        let end = self.skip(p, |b| b.is_ascii_digit());
        self.boundary(end).then_some(end)
    }

    /// `\s*(,)`, with the comma as capture group 1.
    fn spaced_comma(&mut self, p: usize) -> Option<usize> {
        let comma = self.space_end(p);
        let end = self.lit(comma, b",")?;
        self.capture(1, comma, end);
        Some(end)
    }

    /// `(,|(?=}))`: a comma, or nothing before a closing brace.
    fn parameter_end(&self, p: usize) -> Option<usize> {
        match self.at(p) {
            b',' => Some(p + 1),
            b'}' => Some(p),
            _ => None,
        }
    }
}
