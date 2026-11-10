//! The matchers: for each regular expression of the grammar, the function
//! that decides whether it matches at a position and where.
//! A matcher records the capture groups the grammar names with [`Lexer::cap`].
//! The engine names them.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would.
//! Where a quantifier could give characters back, the doc comment says why it
//! cannot matter.
//! Otherwise, the matcher tries the shorter matches.

mod chars;
mod comments;
mod emmydoc;
mod names;
mod numbers;
mod strings;

use chars::{diagnostic, type_part, type_start};

use super::engine::Lexer;
use super::grammar::Pattern;
use crate::rules::Chars;

impl Lexer<'_> {
    /// Whether `pattern` matches at `p`, and where its match ends.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm for each expression of the grammar"
    )]
    pub(super) fn matcher(&mut self, pattern: Pattern, p: usize) -> Option<usize> {
        match pattern {
            Pattern::RootAt0 => self.function_head(p),
            Pattern::RootAt0End => self.after_function_head(p),
            Pattern::RootAt0At1 => self.group_lit(p, b"(", 1),
            Pattern::RootAt0At1End => self.parameters_end(p),
            Pattern::RootAt0At1At1 => self.vararg_parameter(p),
            Pattern::RootAt0At1At2 => self.ascii_name(p),
            Pattern::RootAt0At1At3 | Pattern::EmmydocAt19At0At1 => self.lit(p, b","),
            Pattern::RootAt0At1At4 | Pattern::EmmydocAt9At0At0 => self.lit(p, b":"),
            Pattern::RootAt0At1At4End => self.ahead_in(p, b"),").then_some(p),
            Pattern::RootAt0At2 => self.function_name_before_colon(p),
            Pattern::RootAt0At3 | Pattern::EmmydocAt9At0 => self.bounded_name(p),
            Pattern::RootAt1 => self.hex_number(p),
            Pattern::RootAt2 => self.hex_fraction(p),
            Pattern::RootAt3 => self.hex_integer(p),
            Pattern::RootAt4 => self.decimal_number(p),
            Pattern::RootAt5 => self.decimal_fraction(p),
            Pattern::RootAt6 => self.decimal_integer(p),
            Pattern::StringAt0 => self.lit(p, b"'"),
            Pattern::StringAt0End => self.quote_end(p, b'\''),
            Pattern::StringAt1 => self.lit(p, b"\""),
            Pattern::StringAt1End => self.quote_end(p, b'"'),
            Pattern::StringAt2 => self.lit(p, b"`"),
            Pattern::StringAt2End => self.quote_end(p, b'`'),
            Pattern::StringAt3 => self.cdef_open(p),
            Pattern::StringAt3End => self.cdef_close(p),
            Pattern::StringAt3At0 => self.at_g(p).then_some(p),
            Pattern::StringAt3At0End => self.cdef_body_end(p),
            Pattern::StringAt4 => self.long_string_open(p),
            Pattern::StringAt4End => self.long_string_close(p),
            Pattern::EscapedCharAt0 => self.simple_escape(p),
            Pattern::EscapedCharAt1 => self.skip_escape(p),
            Pattern::EscapedCharAt2 => self.decimal_escape(p),
            Pattern::EscapedCharAt3 => self.hex_escape(p),
            Pattern::EscapedCharAt4 => self.unicode_escape(p),
            Pattern::EscapedCharAt5 => self.any_escape(p),
            Pattern::RootAt8 => self.shebang(p),
            Pattern::CommentAt0 => self.comment_start(p),
            Pattern::CommentAt0End => self.comment_end(p),
            Pattern::CommentAt0At0 => self.code_comment_open(p),
            Pattern::CommentAt0At1 => self.long_comment_open(p),
            Pattern::CommentAt0At0End | Pattern::CommentAt0At1End => self.long_comment_close(p),
            Pattern::CommentAt0At2 => self.lit(p, b"----"),
            Pattern::CommentAt0At3 => self.lit(p, b"---"),
            Pattern::CommentAt0At4 => self.lit(p, b"--"),
            Pattern::CommentAt0At2End | Pattern::CommentAt0At3End | Pattern::CommentAt0At4End => {
                self.line_feed(p)
            }
            Pattern::EmmydocAt1At0End | Pattern::EmmydocAt5At0End | Pattern::EmmydocAt19At0End => {
                self.ahead_in(p, b"\n").then_some(p)
            }
            Pattern::CommentAt1 => self.lit(p, b"/*"),
            Pattern::CommentAt1End => self.lit(p, b"*/"),
            Pattern::LdocTag => self.ldoc_tag(p),
            Pattern::EmmydocAt0 => self.doc_tag(p, b"@class"),
            Pattern::EmmydocAt1 => self.doc_tag(p, b"@enum"),
            Pattern::EmmydocAt2 => self.doc_tag(p, b"@type"),
            Pattern::EmmydocAt3 => self.doc_tag(p, b"@alias"),
            Pattern::EmmydocAt4 => self.operator_tag(p),
            Pattern::EmmydocAt5 => self.doc_tag(p, b"@cast"),
            Pattern::EmmydocAt6 => self.doc_tag(p, b"@param"),
            Pattern::EmmydocAt7 => self.doc_tag(p, b"@return"),
            Pattern::EmmydocAt8 => self.doc_tag(p, b"@field"),
            Pattern::EmmydocAt9 => self.doc_tag(p, b"@generic"),
            Pattern::EmmydocAt10 => self.doc_tag(p, b"@vararg"),
            Pattern::EmmydocAt11 => self.doc_tag(p, b"@overload"),
            Pattern::EmmydocAt12 => self.doc_tag(p, b"@deprecated"),
            Pattern::EmmydocAt13 => self.doc_tag(p, b"@meta"),
            Pattern::EmmydocAt14 => self.doc_tag(p, b"@private"),
            Pattern::EmmydocAt15 => self.doc_tag(p, b"@protected"),
            Pattern::EmmydocAt16 => self.doc_tag(p, b"@package"),
            Pattern::EmmydocAt17 => self.doc_tag(p, b"@version"),
            Pattern::EmmydocAt18 => self.doc_tag(p, b"@see"),
            Pattern::EmmydocAt19 => self.doc_tag(p, b"@diagnostic"),
            Pattern::EmmydocAt20 => self.doc_tag(p, b"@module"),
            Pattern::EmmydocAt21 => self.flag_tag(p),
            Pattern::EmmydocAt22 => self.enum_item(p),
            Pattern::EmmydocAt0End
            | Pattern::EmmydocAt1End
            | Pattern::EmmydocAt2End
            | Pattern::EmmydocAt3End
            | Pattern::EmmydocAt4End
            | Pattern::EmmydocAt5End
            | Pattern::EmmydocAt6End
            | Pattern::EmmydocAt7End
            | Pattern::EmmydocAt8End
            | Pattern::EmmydocAt9End
            | Pattern::EmmydocAt10End
            | Pattern::EmmydocAt11End
            | Pattern::EmmydocAt12End
            | Pattern::EmmydocAt13End
            | Pattern::EmmydocAt14End
            | Pattern::EmmydocAt15End
            | Pattern::EmmydocAt16End
            | Pattern::EmmydocAt17End
            | Pattern::EmmydocAt18End
            | Pattern::EmmydocAt19End
            | Pattern::EmmydocAt20End
            | Pattern::EmmydocAt22End => self.ahead_in(p, b"\n#@").then_some(p),
            Pattern::EmmydocAt3At0End | Pattern::EmmydocAt6At0End | Pattern::EmmydocAt8At0End => {
                self.ahead_in(p, b"\n#").then_some(p)
            }
            Pattern::EmmydocAt0At0
            | Pattern::EmmydocAt1At0
            | Pattern::EmmydocAt3At0
            | Pattern::EmmydocAt5At0
            | Pattern::EmmydocAt18At0 => self.bounded_run(p, type_start, type_part),
            Pattern::EmmydocAt19At0At0 => self.bounded_run(p, type_start, diagnostic),
            Pattern::EmmydocAt0At1 => self.byte_of(p, b",:"),
            Pattern::EmmydocAt5At0At1 => self.cast_sign(p),
            Pattern::EmmydocAt6At0 => self.param_name(p),
            Pattern::EmmydocAt7At0 => self.lit(p, b"?"),
            Pattern::EmmydocAt8At0 => self.field_name(p),
            Pattern::EmmydocAt8At0At2 => self.lit(p, b"]"),
            Pattern::EmmydocAt9At0End => self.generic_end(p),
            Pattern::EmmydocAt17At0 => self.lua_version(p),
            Pattern::EmmydocAt17At1 => self.byte_of(p, b",<>"),
            Pattern::EmmydocAt18At1 => self.lit(p, b"#"),
            Pattern::EmmydocAt19At0 => self.diagnostic_name(p),
            Pattern::EmmydocTypeAt0 => self.keyword_out_of_line(p, b"fun"),
            Pattern::EmmydocTypeAt0End => self.type_end_ahead(p, b"#"),
            Pattern::EmmydocTypeAt0At0 => self.punctuation_then_tabs(p, b"](),:<>?["),
            Pattern::EmmydocTypeAt0At1 => self.type_field_name(p),
            Pattern::EmmydocTypeAt1 | Pattern::RootAt13 => self.angle_name(p),
            Pattern::EmmydocTypeAt2 => self.keyword_out_of_line(p, b"async"),
            Pattern::EmmydocTypeAt3 => self.punctuation_then_tabs(p, b",:?`{|}"),
            Pattern::EmmydocTypeAt4 => self.type_start_ahead(p),
            Pattern::EmmydocTypeAt4End => self.type_end_ahead(p, b"#),:?|}"),
            Pattern::EmmydocTypeAt4At0 => self.type_word(p),
            Pattern::EmmydocTypeAt4At1 => self.ellipsis_then_tabs(p),
            Pattern::RootAt10 => self.goto_label(p),
            Pattern::RootAt11 => self.label(p),
            Pattern::RootAt12 => self.attribute(p),
            Pattern::RootAt14 => self.word_in(p, names::KEYWORDS),
            Pattern::RootAt15 => self.keyword_out_of_line(p, b"local"),
            Pattern::RootAt16 => self.global_declaration(p),
            Pattern::RootAt17 => self.function_keyword(p),
            Pattern::RootAt18 => self.constant(p),
            Pattern::RootAt19 => self.not_member_word(p, &[b"self"]),
            Pattern::RootAt20 => self.builtin_function(p),
            Pattern::RootAt21 => self.async_word(p),
            Pattern::RootAt22 => self.library_function(p),
            Pattern::RootAt23 => self.symbol_operator(p),
            Pattern::RootAt24 => self.word_in(p, &[b"and", b"or", b"not"]),
            Pattern::RootAt25 => self.call_name(p),
            Pattern::RootAt26 => self.name_before_colon(p),
            Pattern::RootAt27 => self.member_name(p),
            Pattern::RootAt28 => self.variable_name(p),
            Pattern::RootAt29 => self.function_variable(p),
            Pattern::RootAt30 => self.operator(p),
        }
    }

    /// `(text)` with `text` as capture group `group`: the end of it.
    fn group_lit(&mut self, p: usize, text: &[u8], group: usize) -> Option<usize> {
        let end = self.lit(p, text)?;
        self.cap(group, p, end);
        Some(end)
    }
}
