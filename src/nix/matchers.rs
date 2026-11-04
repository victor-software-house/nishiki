//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups the grammar names with [`Lexer::cap`].
//! The engine names them.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would.
//! Where a quantifier could give characters back, the doc comment says why it
//! cannot matter, or what giving them back leaves to test.

mod lookahead;
mod operators;
mod strings;

use super::engine::Lexer;
use super::grammar::Pat;
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
    /// Whether `pat` matches at `p`, and where its match ends.
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
    pub(super) fn matcher(&mut self, pat: Pat, p: usize) -> Option<usize> {
        match pat {
            Pat::ExpressionCont
            | Pat::FunctionDefinition
            | Pat::AttrsetDefinitionBraceOpenedAt1
            | Pat::FunctionDefinitionBraceOpened
            | Pat::FunctionDefinitionBraceOpenedAt1
            | Pat::FunctionDefinitionBraceOpenedAt1At1
            | Pat::FunctionDefinitionAt1 => Some(p),
            Pat::ParensAndContEnd
            | Pat::ListAndContEnd
            | Pat::StringAt0End
            | Pat::StringAt1End
            | Pat::ExpressionContEnd
            | Pat::StringAt2End
            | Pat::StringAt3End
            | Pat::StringAt4End
            | Pat::FunctionForSureAt0End
            | Pat::FunctionDefinitionEnd
            | Pat::FunctionBodyFromColonEnd
            | Pat::AttrsetForSureAt0End
            | Pat::AttrsetDefinitionEnd
            | Pat::AttrsetForSureAt1End
            | Pat::AttrsetOrFunctionAt0End
            | Pat::AttrsetDefinitionBraceOpenedAt0End
            | Pat::LetEnd
            | Pat::LetAt0At0End
            | Pat::LetAt0At0At1End
            | Pat::LetAt1End
            | Pat::IfEnd
            | Pat::IfAt2End
            | Pat::ConstantsAt0End
            | Pat::ConstantsAt1End
            | Pat::ConstantsAt2End
            | Pat::ParameterNameAndContEnd
            | Pat::AttrsetOrFunctionAt1End
            | Pat::FunctionDefinitionBraceOpenedEnd
            | Pat::AttrsetOrFunctionAt3At0End
            | Pat::AttrsetOrFunctionAt3At1End
            | Pat::AttrsetOrFunctionAt3At2End
            | Pat::AttrsetOrFunctionAt3At3End
            | Pat::AttrsetOrFunctionAt3At3At1End
            | Pat::AttrsetDefinitionAt1End => self.stops_at(p, Stop::Close),
            Pat::LetAt0End => self.stops_at(p, Stop::CloseOrIn),
            Pat::AttrsetOrFunctionEnd | Pat::AttrsetOrFunctionAt3End => {
                self.stops_at(p, Stop::CloseNoComma)
            }

            Pat::ParensAndCont => self.peek(p, b"("),
            Pat::Parens | Pat::AttributeInheritAt0 => self.lit(p, b"("),
            Pat::ParensEnd | Pat::AttributeInheritAt0At0 => self.lit(p, b")"),
            Pat::ListAndCont => self.peek(p, b"["),
            Pat::List => self.lit(p, b"["),
            Pat::ListEnd => self.lit(p, b"]"),
            Pat::AttrsetDefinition
            | Pat::AttrsetForSureAt0At0End
            | Pat::LetAt0At0
            | Pat::FunctionHeaderUntilColonNoArg
            | Pat::FunctionDefinitionAt1At1
            | Pat::FunctionHeaderUntilColonWithArg => self.peek(p, b"{"),
            Pat::AttrsetDefinitionAt0
            | Pat::AttrsetOrFunction
            | Pat::LetAt0At0At0
            | Pat::FunctionHeaderOpenBrace => self.lit(p, b"{"),
            Pat::AttrsetDefinitionAt0End
            | Pat::AttrsetDefinitionBraceOpenedAt1End
            | Pat::LetAt0At0At0End
            | Pat::InterpolationEnd
            | Pat::FunctionHeaderCloseBraceWithArg
            | Pat::FunctionHeaderCloseBraceNoArg => self.lit(p, b"}"),
            Pat::FunctionDefinitionBraceOpenedAt1At1End | Pat::FunctionHeaderOpenBraceEnd => {
                self.peek(p, b"}")
            }
            Pat::AttrsetDefinitionBraceOpenedAt0 | Pat::AttrsetDefinitionAt1 => {
                (self.prev(p) == Some(b'}')).then_some(p)
            }
            Pat::AttrsetForSureAt0 => self.keyword(p, b"rec").map(|_| p),
            Pat::AttrsetForSureAt0At0 => self.keyword(p, b"rec"),
            Pat::AttrsetForSureAt1 => self.attrset_for_sure(p),
            Pat::AttrsetOrFunctionAt0 => self.attrset_ahead(p),
            Pat::AttrsetOrFunctionAt1 => self.parameters_ahead(p),
            Pat::AttributeInherit => self.keyword(p, b"inherit"),
            Pat::AttributeInheritEnd | Pat::AttributeBindFromEqualsEnd | Pat::WithAssertEnd => {
                self.lit(p, b";")
            }
            Pat::AttributeInheritAt0End
            | Pat::AttributeInheritAt0At0End
            | Pat::AttributeInheritAt1End => self.peek(p, b";"),
            Pat::AttributeInheritAt1 => is_name_start(self.at(p)).then_some(p),
            Pat::AttributeNameAt1 => self.lit(p, b"."),
            Pat::AttributeBindFromEquals => self.lit(p, b"="),
            Pat::AttrsetOrFunctionAt3At0 => self.peek(p, b"."),
            Pat::AttrsetOrFunctionAt3At1 => self.spaced_comma(p),
            Pat::AttrsetOrFunctionAt3At2 => self.peek(p, b"="),
            Pat::AttrsetOrFunctionAt3At3 => self.peek(p, b"?"),
            Pat::AttrsetOrFunctionAt3At3At1 => self.lit(p, b","),

            Pat::StringAt0 => self.starts(p, b"''").then_some(p),
            Pat::StringAt0At0 => self.lit(p, b"''"),
            Pat::StringAt0At0End => self.indented_close(p),
            Pat::StringAt0At0At0 => self.indented_escape(p),
            Pat::StringAt1 => self.peek(p, b"\""),
            Pat::StringQuoted | Pat::StringQuotedEnd => self.lit(p, b"\""),
            Pat::StringQuotedAt0 => self.escape(p),
            Pat::Interpolation => self.lit(p, b"${"),
            Pat::StringAt2 => self.path_text(p),
            Pat::StringAt3 => self.search_path(p),
            Pat::StringAt4 => self.uri(p),

            Pat::FunctionForSureAt0 => self.function_ahead(p),
            Pat::FunctionBodyFromColon => self.lit(p, b":"),
            Pat::FunctionDefinitionBraceOpenedAt1End
            | Pat::FunctionHeaderCloseBraceWithArgEnd
            | Pat::FunctionHeaderTerminalArgEnd
            | Pat::FunctionHeaderTerminalArgAt0End
            | Pat::FunctionHeaderTerminalArgAt0At0End
            | Pat::FunctionDefinitionAt1End
            | Pat::FunctionDefinitionAt1At0End
            | Pat::FunctionDefinitionAt1At0At0End
            | Pat::FunctionHeaderUntilColonNoArgEnd
            | Pat::FunctionHeaderCloseBraceNoArgEnd
            | Pat::FunctionDefinitionAt1At1End
            | Pat::FunctionHeaderUntilColonWithArgEnd => self.peek(p, b":"),
            Pat::FunctionHeaderTerminalArg => self.peek(p, b"@"),
            Pat::FunctionHeaderTerminalArgAt0 | Pat::FunctionDefinitionAt1At0At0 => {
                self.lit(p, b"@")
            }
            Pat::FunctionParameterAt0 => self.lit(p, b"..."),
            Pat::FunctionParameterAt0End | Pat::FunctionParameterAt1End => self.parameter_end(p),
            Pat::FunctionParameterDefault => self.lit(p, b"?"),
            Pat::FunctionParameterDefaultEnd => self.peek(p, b",}"),

            Pat::Let => self.keyword(p, b"let").map(|_| p),
            Pat::LetAt0 => self.keyword(p, b"let"),
            Pat::LetAt0At0At1 => (p == 0 || self.prev(p) == Some(b'}')).then_some(p),
            Pat::LetAt1 => self.keyword(p, b"in"),
            Pat::If => self.keyword(p, b"if").map(|_| p),
            Pat::IfAt0 => self.keyword(p, b"if"),
            Pat::IfAt0End => self.keyword(p, b"then").map(|_| p + 2),
            Pat::IfAt1 => self.keyword_tail(p, b"th", b"en"),
            Pat::IfAt1End => self.keyword(p, b"else").map(|_| p + 2),
            Pat::IfAt2 => self.keyword_tail(p, b"el", b"se"),
            Pat::WithAssert => self.reserved(p, &[b"with", b"assert"]),
            Pat::BadReserved => self.reserved(p, RESERVED),

            Pat::AttributeNameSingle
            | Pat::AttributeNameAt0
            | Pat::ParameterNameAndCont
            | Pat::FunctionParameterAt1
            | Pat::AttrsetOrFunctionAt3
            | Pat::FunctionHeaderTerminalArgAt0At0
            | Pat::FunctionDefinitionAt1At0
            | Pat::ParameterName => self.name(p),
            Pat::ConstantsAt0 => self.word_in(p, &[b"builtins", b"true", b"false", b"null"]),
            Pat::ConstantsAt1 => self.word_in(p, FUNCTIONS),
            Pat::ConstantsAt2 => self.integer(p),
            Pat::OperatorUnary => matches!(self.at(p), b'-' | b'!').then_some(p + 1),
            Pat::OperatorBinary => self.binary_operator(p),

            Pat::CommentAt0 => self.block_comment_open(p),
            Pat::CommentAt0End => self.lit(p, b"*/"),
            Pat::CommentAt1 => self.lit(p, b"#"),
            Pat::CommentAt1End => self.at_eol(p).then_some(p),
            Pat::Whitespace => Some(self.space_end(p)).filter(|&end| end > p),
            Pat::Illegal => (p < self.lim()).then(|| self.char_end(p)),
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
        self.cap(1, comma, end);
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
