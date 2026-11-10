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
//! A regular expression sees the line feed at the end of a line.
//! So `\s` and a negated class such as `[^>]` match it there, and a match can
//! end one past the text.
//! `.` never matches it.

mod heredocs;
mod pod;
mod quotes;
mod variables;
mod words;

use super::engine::Lexer;
use super::grammar::Pattern;
use crate::rules::Chars;

impl Lexer<'_> {
    /// Whether `pattern` matches at `p`, and where its match ends.
    ///
    /// The arms come in groups, in this order:
    /// - comments and POD;
    /// - variables and escapes;
    /// - quote-like operators, regular expressions and strings;
    /// - heredocs;
    /// - declarations, keywords, the function table and punctuation.
    #[expect(clippy::too_many_lines, reason = "one arm per pattern of the grammar")]
    pub(super) fn matcher(&mut self, pattern: Pattern, p: usize) -> Option<usize> {
        match pattern {
            Pattern::LineComment => self.comment_lead(p),
            Pattern::LineCommentEnd => (!self.at_g(p)).then_some(p),
            Pattern::LineCommentAt0 => self.lit(p, b"#"),
            Pattern::LineCommentAt0End
            | Pattern::HeredocAt0At0End
            | Pattern::HeredocAt1IndentedAt0End
            | Pattern::HeredocAt2IndentedAt0End
            | Pattern::HeredocAt3IndentedAt0End
            | Pattern::HeredocAt4IndentedAt0End
            | Pattern::HeredocAt8IndentedAt0End
            | Pattern::HeredocAt9IndentedAt0End
            | Pattern::HeredocAt10IndentedAt0End
            | Pattern::HeredocAt11IndentedAt0End
            | Pattern::HeredocAt12IndentedAt0End
            | Pattern::HeredocAt13IndentedAt0End => self.line_feed(p),
            Pattern::RootAt1 => self.pod_start(p),
            Pattern::RootAt1End => self.pod_cut(p),
            Pattern::PodAt0 => self.pod_command(p),
            Pattern::PodAt1 => self.pod_html_begin(p),
            Pattern::PodAt1End => self.pod_html_end(p),
            Pattern::PodAt2 => self.pod_heading(p),
            Pattern::PodFormattingAt0 => self.pod_markup(p, b'I'),
            Pattern::PodFormattingAt1 => self.pod_markup(p, b'B'),
            Pattern::PodFormattingAt2 => self.pod_code(p),
            Pattern::PodFormattingAt3 => self.pod_link(p),
            Pattern::PodFormattingAt4 => self.pod_other(p),

            Pattern::VariableAt0 => self.punct_var(p, b'&'),
            Pattern::VariableAt1 => self.punct_var(p, b'`'),
            Pattern::VariableAt2 => self.punct_var(p, b'\''),
            Pattern::VariableAt3 => self.punct_var(p, b'+'),
            Pattern::VariableAt4 => self.punct_var(p, b'"'),
            Pattern::VariableAt5 => self.punct_var(p, b'0'),
            Pattern::VariableAt6 => self.special_var(p),
            Pattern::VariableAt7 => self.digit_var(p),
            Pattern::VariableAt8 => self.named_var(p),
            Pattern::VariableAt9 => self.braced_var(p),
            Pattern::VariableAt10 => self.short_var(p),
            Pattern::EscapedCharAt0 => self.escape_digits(p),
            Pattern::EscapedCharAt1 => self.escape_control(p),
            Pattern::EscapedCharAt2 => self.escape_group(p),
            Pattern::EscapedCharAt3 => self.escape_named(p),
            Pattern::EscapedCharAt4 => self.escape_char_name(p),
            Pattern::EscapedCharAt5 => self.escape_octal(p),
            Pattern::EscapedCharAt6 => self.escape_property(p),
            Pattern::EscapedCharAt7 => self.escape_hex(p),
            Pattern::EscapedCharAt8 => self.escape_any(p),

            Pattern::RootAt3 => self.qr_ahead(p),
            Pattern::RootAt3End | Pattern::RootAt4End => self.flags(p, FlagsEnd::Hash),
            Pattern::RootAt5End => self.flags(p, FlagsEnd::Angle),
            Pattern::RootAt6End => self.flags(p, FlagsEnd::Plain),
            Pattern::RootAt7End => self.x_flags(p),
            Pattern::RootAt8End => self.slash_end(p),
            Pattern::RootAt3At0 => self.op_open(p, b"qr", b'{'),
            Pattern::RootAt3At1 => self.op_open(p, b"qr", b'['),
            Pattern::RootAt3At2 => self.op_open(p, b"qr", b'<'),
            Pattern::RootAt3At3 => self.op_open(p, b"qr", b'('),
            Pattern::RootAt3At4 => self.op_open(p, b"qr", b'\''),
            Pattern::RootAt3At5 => self.op_delim(p, b"qr", Delim::Qr),
            Pattern::RootAt4 => self.match_ahead(p),
            Pattern::RootAt4At0 => self.op_open(p, b"m", b'{'),
            Pattern::RootAt4At1 => self.op_open(p, b"m", b'['),
            Pattern::RootAt4At2 => self.op_open(p, b"m", b'<'),
            Pattern::RootAt4At3 => self.op_open(p, b"m", b'('),
            Pattern::RootAt4At4 => self.op_open(p, b"m", b'\''),
            Pattern::RootAt4At5 => self.match_delim(p),
            Pattern::RootAt5 => self.subst_ahead(p),
            Pattern::RootAt5At0 => self.op_open(p, b"s", b'{'),
            Pattern::RootAt5At1 => self.op_open(p, b"s", b'['),
            Pattern::RootAt5At2 => self.op_open(p, b"s", b'<'),
            Pattern::RootAt5At3 => self.op_open(p, b"s", b'('),
            Pattern::RootAt5At9 => self.one_delim(p, Delim::Subst),
            Pattern::RootAt5At10 => {
                let end = self.space_end(p);
                (end > p).then_some(end)
            }
            Pattern::RootAt6 => self.subst_line_ahead(p),
            Pattern::RootAt6At0 => self.subst_open(p),
            Pattern::RootAt6At2 => self.one_delim(p, Delim::SubstLine),
            Pattern::RootAt7 => self.subst_x_ahead(p),
            Pattern::RootAt7At0 => self.subst_any_open(p),
            Pattern::RootAt7At2 => self.dot(p),
            Pattern::RootAt7At1End => self.quote_x_end(p),
            Pattern::RootAt7At2End => self.repeat_x_end(p),
            Pattern::RootAt8 => self.slash_open(p),
            Pattern::RootAt8At0 => (self.starts(p, b"$/")).then_some(p + 1),
            Pattern::RootAt3At3At0 => self.dollar_before(p, Delim::QrParen),
            Pattern::NestedParensInterpolatedAt0 => self.dollar_before(p, Delim::Qr),
            Pattern::RootAt4At5At0 => self.dollar_before(p, Delim::MatchQuote),
            Pattern::RootAt5At8At0 | Pattern::RootAt6At1At0 => self.quote_escape(p),
            Pattern::RootAt3At5End
            | Pattern::RootAt4At5End
            | Pattern::RootAt5At9End
            | Pattern::RootAt6At2End
            | Pattern::RootAt24End
            | Pattern::RootAt25End
            | Pattern::RootAt28End => self.repeat(p),
            Pattern::RootAt6At0End | Pattern::RootAt7At0End => self.repeat_ahead(p),
            Pattern::RootAt24 => self.quote_word(p, b"qq", Lead::Boundary, Delim::Word),
            Pattern::RootAt25 => self.quote_word(p, b"qx", Lead::Boundary, Delim::Qr),
            Pattern::RootAt28 => self.q_words(p),
            Pattern::RootAt26 => self.op_bracket(p, b"qx", Lead::Boundary, b'\''),
            Pattern::RootAt31 => self.op_bracket(p, b"qq", Lead::NoArrow, b'('),
            Pattern::RootAt32 => self.op_bracket(p, b"qq", Lead::Boundary, b'{'),
            Pattern::RootAt33 => self.op_bracket(p, b"qq", Lead::Boundary, b'['),
            Pattern::RootAt34 => self.op_bracket(p, b"qq", Lead::Boundary, b'<'),
            Pattern::RootAt35 => self.op_bracket(p, b"qx", Lead::NoArrow, b'('),
            Pattern::RootAt36 => self.op_bracket(p, b"qx", Lead::Boundary, b'{'),
            Pattern::RootAt37 => self.op_bracket(p, b"qx", Lead::Boundary, b'['),
            Pattern::RootAt38 => self.op_bracket(p, b"qx", Lead::Boundary, b'<'),
            Pattern::RootAt39 => self.q_bracket(p, Lead::NoArrow, b'('),
            Pattern::RootAt40 => self.q_bracket(p, Lead::Boundary, b'{'),
            Pattern::RootAt41 => self.q_bracket(p, Lead::Boundary, b'['),
            Pattern::RootAt42 => self.q_bracket(p, Lead::Boundary, b'<'),
            Pattern::NestedBracesInterpolated | Pattern::NestedBraces | Pattern::RootAt5At4 => {
                self.lit(p, b"{")
            }
            Pattern::NestedBracketsInterpolated
            | Pattern::RootAt4At5At3
            | Pattern::NestedBrackets
            | Pattern::RootAt5At5 => self.lit(p, b"["),
            Pattern::NestedLtgtInterpolated | Pattern::NestedLtgt | Pattern::RootAt5At6 => {
                self.lit(p, b"<")
            }
            Pattern::NestedParensInterpolated | Pattern::NestedParens | Pattern::RootAt5At7 => {
                self.lit(p, b"(")
            }
            Pattern::RootAt3At0End
            | Pattern::NestedBracesInterpolatedEnd
            | Pattern::RootAt4At0End
            | Pattern::RootAt5At0End
            | Pattern::NestedBracesEnd
            | Pattern::RootAt5At4End
            | Pattern::RootAt32End
            | Pattern::RootAt36End
            | Pattern::RootAt40End => self.lit(p, b"}"),
            Pattern::RootAt3At1End
            | Pattern::NestedBracketsInterpolatedEnd
            | Pattern::RootAt4At1End
            | Pattern::RootAt4At5At3End
            | Pattern::RootAt5At1End
            | Pattern::NestedBracketsEnd
            | Pattern::RootAt5At5End
            | Pattern::RootAt33End
            | Pattern::RootAt37End
            | Pattern::RootAt41End => self.lit(p, b"]"),
            Pattern::RootAt3At2End
            | Pattern::NestedLtgtInterpolatedEnd
            | Pattern::RootAt4At2End
            | Pattern::RootAt5At2End
            | Pattern::NestedLtgtEnd
            | Pattern::RootAt5At6End
            | Pattern::RootAt34End
            | Pattern::RootAt38End
            | Pattern::RootAt42End => self.lit(p, b">"),
            Pattern::RootAt3At3End
            | Pattern::NestedParensInterpolatedEnd
            | Pattern::RootAt4At3End
            | Pattern::RootAt5At3End
            | Pattern::NestedParensEnd
            | Pattern::RootAt5At7End
            | Pattern::RootAt31End
            | Pattern::RootAt35End
            | Pattern::RootAt39End => self.lit(p, b")"),
            Pattern::RootAt3At4End
            | Pattern::RootAt4At4End
            | Pattern::RootAt5At8
            | Pattern::RootAt5At8End
            | Pattern::RootAt6At1
            | Pattern::RootAt6At1End
            | Pattern::RootAt7At1
            | Pattern::RootAt26End
            | Pattern::RootAt29
            | Pattern::RootAt29End => self.lit(p, b"'"),
            Pattern::RootAt27 | Pattern::RootAt27End => self.lit(p, b"\""),
            Pattern::RootAt30 | Pattern::RootAt30End => self.lit(p, b"`"),

            Pattern::HeredocAt0Indented => {
                self.heredoc(p, Tilde::Yes, Quote::Single, Label::Is(b"HTML"))
            }
            Pattern::HeredocAt0 => self.heredoc(p, Tilde::No, Quote::Single, Label::Is(b"HTML")),
            Pattern::HeredocAt1Indented => {
                self.heredoc(p, Tilde::Yes, Quote::Single, Label::Is(b"XML"))
            }
            Pattern::HeredocAt1 => self.heredoc(p, Tilde::No, Quote::Single, Label::Is(b"XML")),
            Pattern::HeredocAt2Indented => {
                self.heredoc(p, Tilde::Yes, Quote::Single, Label::Is(b"CSS"))
            }
            Pattern::HeredocAt2 => self.heredoc(p, Tilde::No, Quote::Single, Label::Is(b"CSS")),
            Pattern::HeredocAt3Indented => {
                self.heredoc(p, Tilde::Yes, Quote::Single, Label::Is(b"JAVASCRIPT"))
            }
            Pattern::HeredocAt3 => {
                self.heredoc(p, Tilde::No, Quote::Single, Label::Is(b"JAVASCRIPT"))
            }
            Pattern::HeredocAt4Indented => {
                self.heredoc(p, Tilde::Yes, Quote::Single, Label::Is(b"SQL"))
            }
            Pattern::HeredocAt4 => self.heredoc(p, Tilde::No, Quote::Single, Label::Is(b"SQL")),
            Pattern::HeredocAt6Indented => {
                self.heredoc(p, Tilde::Yes, Quote::Single, Label::Quoted)
            }
            Pattern::HeredocAt6 => self.heredoc(p, Tilde::No, Quote::Single, Label::Quoted),
            Pattern::HeredocAt7Indented => {
                self.heredoc(p, Tilde::Yes, Quote::Backslash, Label::Bare)
            }
            Pattern::HeredocAt7 => self.heredoc(p, Tilde::No, Quote::Backslash, Label::Bare),
            Pattern::HeredocAt8Indented => {
                self.heredoc(p, Tilde::Yes, Quote::Double, Label::Is(b"HTML"))
            }
            Pattern::HeredocAt8 => self.heredoc(p, Tilde::No, Quote::Double, Label::Is(b"HTML")),
            Pattern::HeredocAt9Indented => {
                self.heredoc(p, Tilde::Yes, Quote::Double, Label::Is(b"XML"))
            }
            Pattern::HeredocAt9 => self.heredoc(p, Tilde::No, Quote::Double, Label::Is(b"XML")),
            Pattern::HeredocAt10Indented => {
                self.heredoc(p, Tilde::Yes, Quote::Double, Label::Is(b"CSS"))
            }
            Pattern::HeredocAt10 => self.heredoc(p, Tilde::No, Quote::Double, Label::Is(b"CSS")),
            Pattern::HeredocAt11Indented => {
                self.heredoc(p, Tilde::Yes, Quote::Double, Label::Is(b"JAVASCRIPT"))
            }
            Pattern::HeredocAt11 => {
                self.heredoc(p, Tilde::No, Quote::Double, Label::Is(b"JAVASCRIPT"))
            }
            Pattern::HeredocAt12Indented => {
                self.heredoc(p, Tilde::Yes, Quote::Double, Label::Is(b"SQL"))
            }
            Pattern::HeredocAt12 => self.heredoc(p, Tilde::No, Quote::Double, Label::Is(b"SQL")),
            Pattern::HeredocAt13Indented => {
                self.heredoc(p, Tilde::Yes, Quote::Double, Label::Is(b"POSTSCRIPT"))
            }
            Pattern::HeredocAt13 => {
                self.heredoc(p, Tilde::No, Quote::Double, Label::Is(b"POSTSCRIPT"))
            }
            Pattern::HeredocAt14Indented => {
                self.heredoc(p, Tilde::Yes, Quote::Double, Label::Quoted)
            }
            Pattern::HeredocAt14 => self.heredoc(p, Tilde::No, Quote::Double, Label::Quoted),
            Pattern::HeredocAt15Indented => {
                self.heredoc(p, Tilde::Yes, Quote::None, Label::Is(b"HTML"))
            }
            Pattern::HeredocAt15 => self.heredoc(p, Tilde::No, Quote::None, Label::Is(b"HTML")),
            Pattern::HeredocAt16Indented => {
                self.heredoc(p, Tilde::Yes, Quote::None, Label::Is(b"XML"))
            }
            Pattern::HeredocAt16 => self.heredoc(p, Tilde::No, Quote::None, Label::Is(b"XML")),
            Pattern::HeredocAt17Indented => {
                self.heredoc(p, Tilde::Yes, Quote::None, Label::Is(b"CSS"))
            }
            Pattern::HeredocAt17 => self.heredoc(p, Tilde::No, Quote::None, Label::Is(b"CSS")),
            Pattern::HeredocAt18Indented => {
                self.heredoc(p, Tilde::Yes, Quote::None, Label::Is(b"JAVASCRIPT"))
            }
            Pattern::HeredocAt18 => {
                self.heredoc(p, Tilde::No, Quote::None, Label::Is(b"JAVASCRIPT"))
            }
            Pattern::HeredocAt19Indented => {
                self.heredoc(p, Tilde::Yes, Quote::None, Label::Is(b"SQL"))
            }
            Pattern::HeredocAt19 => self.heredoc(p, Tilde::No, Quote::None, Label::Is(b"SQL")),
            Pattern::HeredocAt20Indented => {
                self.heredoc(p, Tilde::Yes, Quote::None, Label::Is(b"POSTSCRIPT"))
            }
            Pattern::HeredocAt20 => {
                self.heredoc(p, Tilde::No, Quote::None, Label::Is(b"POSTSCRIPT"))
            }
            Pattern::HeredocAt21Indented => self.heredoc(p, Tilde::Yes, Quote::None, Label::Bare),
            Pattern::HeredocAt21 => self.heredoc(p, Tilde::No, Quote::None, Label::Bare),
            Pattern::HeredocAt22Indented => {
                self.heredoc(p, Tilde::Yes, Quote::Backtick, Label::Quoted)
            }
            Pattern::HeredocAt22 => self.heredoc(p, Tilde::No, Quote::Backtick, Label::Quoted),
            Pattern::HeredocAt0IndentedEnd
            | Pattern::HeredocAt8IndentedEnd
            | Pattern::HeredocAt15IndentedEnd => self.heredoc_end(p, Tilde::Yes, Some(b"HTML")),
            Pattern::HeredocAt0End | Pattern::HeredocAt8End | Pattern::HeredocAt15End => {
                self.heredoc_end(p, Tilde::No, Some(b"HTML"))
            }
            Pattern::HeredocAt1IndentedEnd
            | Pattern::HeredocAt9IndentedEnd
            | Pattern::HeredocAt16IndentedEnd => self.heredoc_end(p, Tilde::Yes, Some(b"XML")),
            Pattern::HeredocAt1End | Pattern::HeredocAt9End | Pattern::HeredocAt16End => {
                self.heredoc_end(p, Tilde::No, Some(b"XML"))
            }
            Pattern::HeredocAt2IndentedEnd
            | Pattern::HeredocAt10IndentedEnd
            | Pattern::HeredocAt17IndentedEnd => self.heredoc_end(p, Tilde::Yes, Some(b"CSS")),
            Pattern::HeredocAt2End | Pattern::HeredocAt10End | Pattern::HeredocAt17End => {
                self.heredoc_end(p, Tilde::No, Some(b"CSS"))
            }
            Pattern::HeredocAt3IndentedEnd
            | Pattern::HeredocAt11IndentedEnd
            | Pattern::HeredocAt18IndentedEnd => {
                self.heredoc_end(p, Tilde::Yes, Some(b"JAVASCRIPT"))
            }
            Pattern::HeredocAt3End | Pattern::HeredocAt11End | Pattern::HeredocAt18End => {
                self.heredoc_end(p, Tilde::No, Some(b"JAVASCRIPT"))
            }
            Pattern::HeredocAt4IndentedEnd
            | Pattern::HeredocAt12IndentedEnd
            | Pattern::HeredocAt19IndentedEnd => self.heredoc_end(p, Tilde::Yes, Some(b"SQL")),
            Pattern::HeredocAt4End | Pattern::HeredocAt12End | Pattern::HeredocAt19End => {
                self.heredoc_end(p, Tilde::No, Some(b"SQL"))
            }
            Pattern::HeredocAt13IndentedEnd | Pattern::HeredocAt20IndentedEnd => {
                self.heredoc_end(p, Tilde::Yes, Some(b"POSTSCRIPT"))
            }
            Pattern::HeredocAt13End | Pattern::HeredocAt20End => {
                self.heredoc_end(p, Tilde::No, Some(b"POSTSCRIPT"))
            }
            Pattern::HeredocAt6IndentedEnd
            | Pattern::HeredocAt7IndentedEnd
            | Pattern::HeredocAt14IndentedEnd
            | Pattern::HeredocAt21IndentedEnd
            | Pattern::HeredocAt22IndentedEnd => self.heredoc_end(p, Tilde::Yes, None),
            Pattern::HeredocAt6End
            | Pattern::HeredocAt7End
            | Pattern::HeredocAt14End
            | Pattern::HeredocAt21End
            | Pattern::HeredocAt22End => self.heredoc_end(p, Tilde::No, None),
            Pattern::HeredocAt0At0
            | Pattern::HeredocAt1IndentedAt0
            | Pattern::HeredocAt2IndentedAt0
            | Pattern::HeredocAt3IndentedAt0
            | Pattern::HeredocAt4IndentedAt0
            | Pattern::HeredocAt8IndentedAt0
            | Pattern::HeredocAt9IndentedAt0
            | Pattern::HeredocAt10IndentedAt0
            | Pattern::HeredocAt11IndentedAt0
            | Pattern::HeredocAt12IndentedAt0
            | Pattern::HeredocAt13IndentedAt0 => (p == 0).then_some(0),

            Pattern::RootAt9 => self.hash_key(p),
            Pattern::RootAt10 => self.braced_word(p),
            Pattern::RootAt11 => self.package(p),
            Pattern::RootAt12 => self.sub(p),
            Pattern::RootAt13 => self.special_block(p),
            Pattern::RootAt14 => self.indent_start(p),
            Pattern::RootAt14End => self.indent_end(p),
            Pattern::RootAt14At0 => self.indent(p),
            Pattern::RootAt15 => self.transliteration(p),
            Pattern::RootAt16 => self.magic_constant(p),
            Pattern::RootAt17 => self.data_section(p),
            Pattern::RootAt17End => None,
            Pattern::RootAt18 => self.control_keyword(p),
            Pattern::RootAt19 => self.word_in(p, &[b"my", b"our", b"local"]),
            Pattern::RootAt20 => self.file_test(p),
            Pattern::RootAt21 => self.word_in(p, &[b"and", b"or", b"xor", b"as", b"not"]),
            Pattern::RootAt22 => self.arrow(p),
            Pattern::RootAt43 => self.dunder_line(p),
            Pattern::RootAt43End => self.at_eol(p).then_some(p),
            Pattern::RootAt44 => self.format(p),
            Pattern::RootAt44End => self.format_end(p),
            Pattern::RootAt45 => self.repetition(p),
            Pattern::RootAt46 => self.function(p),
            Pattern::RootAt47 => self.empty_pair(p, b'{', b'}'),
            Pattern::RootAt48 => self.empty_pair(p, b'(', b')'),
        }
    }

    /// `\n`: the line feed that ends the line.
    fn line_feed(&self, p: usize) -> Option<usize> {
        (p == self.lim() && self.avail(p)).then_some(p + 1)
    }

    /// `.`: one character that is not the line feed.
    fn dot(&mut self, p: usize) -> Option<usize> {
        if p >= self.lim() {
            return None;
        }
        let end = self.char_end(p);
        self.capture(1, p, end);
        Some(end)
    }

    /// The end of the one character at `p` when `class` accepts it.
    /// A character past ASCII goes to `class` as its first byte, at least
    /// `0x80`.
    /// The line feed after the text counts, as a regular expression sees it.
    fn char_in(&self, p: usize, class: impl Fn(u8) -> bool) -> Option<usize> {
        if !self.avail(p) || !class(self.at(p)) {
            return None;
        }
        Some(if p < self.lim() {
            self.char_end(p)
        } else {
            p + 1
        })
    }

    /// Whether the character at `p` is `\s`.
    fn is_space_at(&self, p: usize) -> bool {
        self.space_at(p) > 0
    }

    /// `(\{)(})` and `(\()(\))`: an empty pair.
    fn empty_pair(&mut self, p: usize, open: u8, close: u8) -> Option<usize> {
        (self.at(p) == open && self.at(p + 1) == close && p + 1 < self.lim()).then(|| {
            self.capture(1, p, p + 1);
            self.capture(2, p + 1, p + 2);
            p + 2
        })
    }
}

/// What stands before a quote-like operator's name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Lead {
    /// `\b`.
    Boundary,
    /// `(?<!->)\b`: a word boundary not after `->`.
    NoArrow,
}

/// The delimiters a quote-like operator takes after its name and blanks, as
/// each pattern's negated class spells them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Delim {
    /// `[^'(<\[{\s\w]`, after `qr` and `qx`.
    Qr,
    /// `[^'(<\[\\{\s\w]`, before a `$` in `qr(…)`.
    QrParen,
    /// `[^(<\[{\w\s]`, after `qq` and `q`.
    Word,
    /// `[^(;<\[{\s\w]`, after `s`.
    Subst,
    /// `[^'(0-9<A-\[a-{\s]`, after `m`.
    MatchQuote,
    /// `[^(0-9<A-\[a-{\s]`, after `s` in a substitution on one line.
    SubstLine,
    /// `[^(<>\[{\s\w]`, after `s` in a substitution with the `x` flag.
    SubstX,
    /// `[^0-9A-Za-z\s]`, after `m` in its lookahead and after `tr` and `y`.
    NotAlnum,
    /// `[^\s\w]`, after `qr` in its lookahead.
    NotWord,
}

/// What follows a substitution's or a match's flags.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FlagsEnd {
    /// `(?=(\s+\S|\s*[#),;{}]|\s*$))`.
    Hash,
    /// `(?=(\s+\S|\s*[]),;>{}]|\s*$))`.
    Angle,
    /// `(?=([),;}]|\s+|\s*$))`.
    Plain,
}

/// Whether a heredoc's opener has the `~` of `<<~`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tilde {
    /// `<<~`: the closing line may be indented.
    Yes,
    /// `<<`: the closing line holds the label alone.
    No,
}

/// How a heredoc's opener quotes its label.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Quote {
    /// `'LABEL'`.
    Single,
    /// `"LABEL"`.
    Double,
    /// `` `LABEL` ``.
    Backtick,
    /// `\LABEL`.
    Backslash,
    /// `LABEL`.
    None,
}

/// The label a heredoc pattern takes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Label {
    /// The label the pattern names, such as `HTML`.
    Is(&'static [u8]),
    /// Any text up to the closing quote: `[^']*`, `[^"]*` or ``[^`]*``.
    Quoted,
    /// ``(?![ $(=\d])[^"'),;`\s]*``.
    Bare,
}
