//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where. A matcher records the
//! capture groups the grammar names with [`Lexer::cap`]; the engine names them.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would. Where a quantifier could give characters back, the doc comment says
//! why it cannot matter, or the matcher tries the shorter matches.

mod chars;
mod comments;
mod emmydoc;
mod names;
mod numbers;
mod strings;

use chars::{diagnostic, type_part, type_start};

use super::engine::Lexer;
use super::grammar::Pat;
use crate::rules::Chars;

impl Lexer<'_> {
    /// Whether `pat` matches at `p`, and where its match ends.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm for each expression of the grammar"
    )]
    pub(super) fn matcher(&mut self, pat: Pat, p: usize) -> Option<usize> {
        match pat {
            Pat::RootAt0 => self.function_head(p),
            Pat::RootAt0End => self.after_function_head(p),
            Pat::RootAt0At1 => self.group_lit(p, b"(", 1),
            Pat::RootAt0At1End => self.parameters_end(p),
            Pat::RootAt0At1At1 => self.vararg_parameter(p),
            Pat::RootAt0At1At2 => self.ascii_name(p),
            Pat::RootAt0At1At3 | Pat::EmmydocAt19At0At1 => self.lit(p, b","),
            Pat::RootAt0At1At4 | Pat::EmmydocAt9At0At0 => self.lit(p, b":"),
            Pat::RootAt0At1At4End => self.ahead_in(p, b"),").then_some(p),
            Pat::RootAt0At2 => self.function_name_before_colon(p),
            Pat::RootAt0At3 | Pat::EmmydocAt9At0 => self.bounded_name(p),
            Pat::RootAt1 => self.hex_number(p),
            Pat::RootAt2 => self.hex_fraction(p),
            Pat::RootAt3 => self.hex_integer(p),
            Pat::RootAt4 => self.decimal_number(p),
            Pat::RootAt5 => self.decimal_fraction(p),
            Pat::RootAt6 => self.decimal_integer(p),
            Pat::StringAt0 => self.lit(p, b"'"),
            Pat::StringAt0End => self.quote_end(p, b'\''),
            Pat::StringAt1 => self.lit(p, b"\""),
            Pat::StringAt1End => self.quote_end(p, b'"'),
            Pat::StringAt2 => self.lit(p, b"`"),
            Pat::StringAt2End => self.quote_end(p, b'`'),
            Pat::StringAt3 => self.cdef_open(p),
            Pat::StringAt3End => self.cdef_close(p),
            Pat::StringAt4 => self.long_string_open(p),
            Pat::StringAt4End => self.long_string_close(p),
            Pat::EscapedCharAt0 => self.simple_escape(p),
            Pat::EscapedCharAt1 => self.skip_escape(p),
            Pat::EscapedCharAt2 => self.decimal_escape(p),
            Pat::EscapedCharAt3 => self.hex_escape(p),
            Pat::EscapedCharAt4 => self.unicode_escape(p),
            Pat::EscapedCharAt5 => self.any_escape(p),
            Pat::RootAt8 => self.shebang(p),
            Pat::CommentAt0 => self.comment_start(p),
            Pat::CommentAt0End => self.comment_end(p),
            Pat::CommentAt0At0 => self.code_comment_open(p),
            Pat::CommentAt0At1 => self.long_comment_open(p),
            Pat::CommentAt0At0End | Pat::CommentAt0At1End => self.long_comment_close(p),
            Pat::CommentAt0At2 => self.lit(p, b"----"),
            Pat::CommentAt0At3 => self.lit(p, b"---"),
            Pat::CommentAt0At4 => self.lit(p, b"--"),
            Pat::CommentAt0At2End | Pat::CommentAt0At3End | Pat::CommentAt0At4End => {
                self.line_feed(p)
            }
            Pat::EmmydocAt1At0End | Pat::EmmydocAt5At0End | Pat::EmmydocAt19At0End => {
                self.ahead_in(p, b"\n").then_some(p)
            }
            Pat::CommentAt1 => self.lit(p, b"/*"),
            Pat::CommentAt1End => self.lit(p, b"*/"),
            Pat::LdocTag => self.ldoc_tag(p),
            Pat::EmmydocAt0 => self.doc_tag(p, b"@class"),
            Pat::EmmydocAt1 => self.doc_tag(p, b"@enum"),
            Pat::EmmydocAt2 => self.doc_tag(p, b"@type"),
            Pat::EmmydocAt3 => self.doc_tag(p, b"@alias"),
            Pat::EmmydocAt4 => self.operator_tag(p),
            Pat::EmmydocAt5 => self.doc_tag(p, b"@cast"),
            Pat::EmmydocAt6 => self.doc_tag(p, b"@param"),
            Pat::EmmydocAt7 => self.doc_tag(p, b"@return"),
            Pat::EmmydocAt8 => self.doc_tag(p, b"@field"),
            Pat::EmmydocAt9 => self.doc_tag(p, b"@generic"),
            Pat::EmmydocAt10 => self.doc_tag(p, b"@vararg"),
            Pat::EmmydocAt11 => self.doc_tag(p, b"@overload"),
            Pat::EmmydocAt12 => self.doc_tag(p, b"@deprecated"),
            Pat::EmmydocAt13 => self.doc_tag(p, b"@meta"),
            Pat::EmmydocAt14 => self.doc_tag(p, b"@private"),
            Pat::EmmydocAt15 => self.doc_tag(p, b"@protected"),
            Pat::EmmydocAt16 => self.doc_tag(p, b"@package"),
            Pat::EmmydocAt17 => self.doc_tag(p, b"@version"),
            Pat::EmmydocAt18 => self.doc_tag(p, b"@see"),
            Pat::EmmydocAt19 => self.doc_tag(p, b"@diagnostic"),
            Pat::EmmydocAt20 => self.doc_tag(p, b"@module"),
            Pat::EmmydocAt21 => self.flag_tag(p),
            Pat::EmmydocAt22 => self.enum_item(p),
            Pat::EmmydocAt0End
            | Pat::EmmydocAt1End
            | Pat::EmmydocAt2End
            | Pat::EmmydocAt3End
            | Pat::EmmydocAt4End
            | Pat::EmmydocAt5End
            | Pat::EmmydocAt6End
            | Pat::EmmydocAt7End
            | Pat::EmmydocAt8End
            | Pat::EmmydocAt9End
            | Pat::EmmydocAt10End
            | Pat::EmmydocAt11End
            | Pat::EmmydocAt12End
            | Pat::EmmydocAt13End
            | Pat::EmmydocAt14End
            | Pat::EmmydocAt15End
            | Pat::EmmydocAt16End
            | Pat::EmmydocAt17End
            | Pat::EmmydocAt18End
            | Pat::EmmydocAt19End
            | Pat::EmmydocAt20End
            | Pat::EmmydocAt22End => self.ahead_in(p, b"\n#@").then_some(p),
            Pat::EmmydocAt3At0End | Pat::EmmydocAt6At0End | Pat::EmmydocAt8At0End => {
                self.ahead_in(p, b"\n#").then_some(p)
            }
            Pat::EmmydocAt0At0
            | Pat::EmmydocAt1At0
            | Pat::EmmydocAt3At0
            | Pat::EmmydocAt5At0
            | Pat::EmmydocAt18At0 => self.bounded_run(p, type_start, type_part),
            Pat::EmmydocAt19At0At0 => self.bounded_run(p, type_start, diagnostic),
            Pat::EmmydocAt0At1 => self.byte_of(p, b",:"),
            Pat::EmmydocAt5At0At1 => self.cast_sign(p),
            Pat::EmmydocAt6At0 => self.param_name(p),
            Pat::EmmydocAt7At0 => self.lit(p, b"?"),
            Pat::EmmydocAt8At0 => self.field_name(p),
            Pat::EmmydocAt8At0At2 => self.lit(p, b"]"),
            Pat::EmmydocAt9At0End => self.generic_end(p),
            Pat::EmmydocAt17At0 => self.lua_version(p),
            Pat::EmmydocAt17At1 => self.byte_of(p, b",<>"),
            Pat::EmmydocAt18At1 => self.lit(p, b"#"),
            Pat::EmmydocAt19At0 => self.diagnostic_name(p),
            Pat::EmmydocTypeAt0 => self.keyword(p, b"fun"),
            Pat::EmmydocTypeAt0End => self.type_end_ahead(p, b"#"),
            Pat::EmmydocTypeAt0At0 => self.punctuation_then_tabs(p, b"](),:<>?["),
            Pat::EmmydocTypeAt0At1 => self.type_field_name(p),
            Pat::EmmydocTypeAt1 | Pat::RootAt13 => self.angle_name(p),
            Pat::EmmydocTypeAt2 => self.keyword(p, b"async"),
            Pat::EmmydocTypeAt3 => self.punctuation_then_tabs(p, b",:?`{|}"),
            Pat::EmmydocTypeAt4 => self.type_start_ahead(p),
            Pat::EmmydocTypeAt4End => self.type_end_ahead(p, b"#),:?|}"),
            Pat::EmmydocTypeAt4At0 => self.type_word(p),
            Pat::EmmydocTypeAt4At1 => self.ellipsis_then_tabs(p),
            Pat::RootAt10 => self.goto_label(p),
            Pat::RootAt11 => self.label(p),
            Pat::RootAt12 => self.attribute(p),
            Pat::RootAt14 => self.word_in(p, names::KEYWORDS),
            Pat::RootAt15 => self.keyword(p, b"local"),
            Pat::RootAt16 => self.global_declaration(p),
            Pat::RootAt17 => self.function_keyword(p),
            Pat::RootAt18 => self.constant(p),
            Pat::RootAt19 => self.not_member_word(p, &[b"self"]),
            Pat::RootAt20 => self.builtin_function(p),
            Pat::RootAt21 => self.async_word(p),
            Pat::RootAt22 => self.library_function(p),
            Pat::RootAt23 => self.symbol_operator(p),
            Pat::RootAt24 => self.word_in(p, &[b"and", b"or", b"not"]),
            Pat::RootAt25 => self.call_name(p),
            Pat::RootAt26 => self.name_before_colon(p),
            Pat::RootAt27 => self.member_name(p),
            Pat::RootAt28 => self.variable_name(p),
            Pat::RootAt29 => self.function_variable(p),
            Pat::RootAt30 => self.operator(p),
        }
    }

    /// `(text)` with `text` as capture group `group`: the end of it.
    fn group_lit(&mut self, p: usize, text: &[u8], group: usize) -> Option<usize> {
        let end = self.lit(p, text)?;
        self.cap(group, p, end);
        Some(end)
    }
}
