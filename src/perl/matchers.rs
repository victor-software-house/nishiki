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
use super::grammar::Pat;
use crate::rules::Chars;

impl Lexer<'_> {
    /// Whether `pat` matches at `p`, and where its match ends.
    ///
    /// The arms come in groups, in this order:
    /// - comments and POD;
    /// - variables and escapes;
    /// - quote-like operators, regular expressions and strings;
    /// - heredocs;
    /// - declarations, keywords, the function table and punctuation.
    #[expect(clippy::too_many_lines, reason = "one arm per pattern of the grammar")]
    pub(super) fn matcher(&mut self, pat: Pat, p: usize) -> Option<usize> {
        match pat {
            Pat::LineComment => self.comment_lead(p),
            Pat::LineCommentEnd => (!self.at_g(p)).then_some(p),
            Pat::LineCommentAt0 => self.lit(p, b"#"),
            Pat::LineCommentAt0End
            | Pat::HeredocAt0At0End
            | Pat::HeredocAt1IndentedAt0End
            | Pat::HeredocAt2IndentedAt0End
            | Pat::HeredocAt3IndentedAt0End
            | Pat::HeredocAt4IndentedAt0End
            | Pat::HeredocAt8IndentedAt0End
            | Pat::HeredocAt9IndentedAt0End
            | Pat::HeredocAt10IndentedAt0End
            | Pat::HeredocAt11IndentedAt0End
            | Pat::HeredocAt12IndentedAt0End
            | Pat::HeredocAt13IndentedAt0End => self.line_feed(p),
            Pat::RootAt1 => self.pod_start(p),
            Pat::RootAt1End => self.pod_cut(p),
            Pat::PodAt0 => self.pod_command(p),
            Pat::PodAt1 => self.pod_html_begin(p),
            Pat::PodAt1End => self.pod_html_end(p),
            Pat::PodAt2 => self.pod_heading(p),
            Pat::PodFormattingAt0 => self.pod_markup(p, b'I'),
            Pat::PodFormattingAt1 => self.pod_markup(p, b'B'),
            Pat::PodFormattingAt2 => self.pod_code(p),
            Pat::PodFormattingAt3 => self.pod_link(p),
            Pat::PodFormattingAt4 => self.pod_other(p),

            Pat::VariableAt0 => self.punct_var(p, b'&'),
            Pat::VariableAt1 => self.punct_var(p, b'`'),
            Pat::VariableAt2 => self.punct_var(p, b'\''),
            Pat::VariableAt3 => self.punct_var(p, b'+'),
            Pat::VariableAt4 => self.punct_var(p, b'"'),
            Pat::VariableAt5 => self.punct_var(p, b'0'),
            Pat::VariableAt6 => self.special_var(p),
            Pat::VariableAt7 => self.digit_var(p),
            Pat::VariableAt8 => self.named_var(p),
            Pat::VariableAt9 => self.braced_var(p),
            Pat::VariableAt10 => self.short_var(p),
            Pat::EscapedCharAt0 => self.escape_digits(p),
            Pat::EscapedCharAt1 => self.escape_control(p),
            Pat::EscapedCharAt2 => self.escape_group(p),
            Pat::EscapedCharAt3 => self.escape_named(p),
            Pat::EscapedCharAt4 => self.escape_char_name(p),
            Pat::EscapedCharAt5 => self.escape_octal(p),
            Pat::EscapedCharAt6 => self.escape_property(p),
            Pat::EscapedCharAt7 => self.escape_hex(p),
            Pat::EscapedCharAt8 => self.escape_any(p),

            Pat::RootAt3 => self.qr_ahead(p),
            Pat::RootAt3End | Pat::RootAt4End => self.flags(p, FlagsEnd::Hash),
            Pat::RootAt5End => self.flags(p, FlagsEnd::Angle),
            Pat::RootAt6End => self.flags(p, FlagsEnd::Plain),
            Pat::RootAt7End => self.x_flags(p),
            Pat::RootAt8End => self.slash_end(p),
            Pat::RootAt3At0 => self.op_open(p, b"qr", b'{'),
            Pat::RootAt3At1 => self.op_open(p, b"qr", b'['),
            Pat::RootAt3At2 => self.op_open(p, b"qr", b'<'),
            Pat::RootAt3At3 => self.op_open(p, b"qr", b'('),
            Pat::RootAt3At4 => self.op_open(p, b"qr", b'\''),
            Pat::RootAt3At5 => self.op_delim(p, b"qr", Delim::Qr),
            Pat::RootAt4 => self.match_ahead(p),
            Pat::RootAt4At0 => self.op_open(p, b"m", b'{'),
            Pat::RootAt4At1 => self.op_open(p, b"m", b'['),
            Pat::RootAt4At2 => self.op_open(p, b"m", b'<'),
            Pat::RootAt4At3 => self.op_open(p, b"m", b'('),
            Pat::RootAt4At4 => self.op_open(p, b"m", b'\''),
            Pat::RootAt4At5 => self.match_delim(p),
            Pat::RootAt5 => self.subst_ahead(p),
            Pat::RootAt5At0 => self.op_open(p, b"s", b'{'),
            Pat::RootAt5At1 => self.op_open(p, b"s", b'['),
            Pat::RootAt5At2 => self.op_open(p, b"s", b'<'),
            Pat::RootAt5At3 => self.op_open(p, b"s", b'('),
            Pat::RootAt5At9 => self.one_delim(p, Delim::Subst),
            Pat::RootAt5At10 => {
                let end = self.space_end(p);
                (end > p).then_some(end)
            }
            Pat::RootAt6 => self.subst_line_ahead(p),
            Pat::RootAt6At0 => self.subst_open(p),
            Pat::RootAt6At2 => self.one_delim(p, Delim::SubstLine),
            Pat::RootAt7 => self.subst_x_ahead(p),
            Pat::RootAt7At0 => self.subst_any_open(p),
            Pat::RootAt7At2 => self.dot(p),
            Pat::RootAt7At1End => self.quote_x_end(p),
            Pat::RootAt7At2End => self.repeat_x_end(p),
            Pat::RootAt8 => self.slash_open(p),
            Pat::RootAt8At0 => (self.starts(p, b"$/")).then_some(p + 1),
            Pat::RootAt3At3At0 => self.dollar_before(p, Delim::QrParen),
            Pat::NestedParensInterpolatedAt0 => self.dollar_before(p, Delim::Qr),
            Pat::RootAt4At5At0 => self.dollar_before(p, Delim::MatchQuote),
            Pat::RootAt5At8At0 | Pat::RootAt6At1At0 => self.quote_escape(p),
            Pat::RootAt3At5End
            | Pat::RootAt4At5End
            | Pat::RootAt5At9End
            | Pat::RootAt6At2End
            | Pat::RootAt24End
            | Pat::RootAt25End
            | Pat::RootAt28End => self.repeat(p),
            Pat::RootAt6At0End | Pat::RootAt7At0End => self.repeat_ahead(p),
            Pat::RootAt24 => self.quote_word(p, b"qq", Lead::Boundary, Delim::Word),
            Pat::RootAt25 => self.quote_word(p, b"qx", Lead::Boundary, Delim::Qr),
            Pat::RootAt28 => self.q_words(p),
            Pat::RootAt26 => self.op_bracket(p, b"qx", Lead::Boundary, b'\''),
            Pat::RootAt31 => self.op_bracket(p, b"qq", Lead::NoArrow, b'('),
            Pat::RootAt32 => self.op_bracket(p, b"qq", Lead::Boundary, b'{'),
            Pat::RootAt33 => self.op_bracket(p, b"qq", Lead::Boundary, b'['),
            Pat::RootAt34 => self.op_bracket(p, b"qq", Lead::Boundary, b'<'),
            Pat::RootAt35 => self.op_bracket(p, b"qx", Lead::NoArrow, b'('),
            Pat::RootAt36 => self.op_bracket(p, b"qx", Lead::Boundary, b'{'),
            Pat::RootAt37 => self.op_bracket(p, b"qx", Lead::Boundary, b'['),
            Pat::RootAt38 => self.op_bracket(p, b"qx", Lead::Boundary, b'<'),
            Pat::RootAt39 => self.q_bracket(p, Lead::NoArrow, b'('),
            Pat::RootAt40 => self.q_bracket(p, Lead::Boundary, b'{'),
            Pat::RootAt41 => self.q_bracket(p, Lead::Boundary, b'['),
            Pat::RootAt42 => self.q_bracket(p, Lead::Boundary, b'<'),
            Pat::NestedBracesInterpolated | Pat::NestedBraces | Pat::RootAt5At4 => {
                self.lit(p, b"{")
            }
            Pat::NestedBracketsInterpolated
            | Pat::RootAt4At5At3
            | Pat::NestedBrackets
            | Pat::RootAt5At5 => self.lit(p, b"["),
            Pat::NestedLtgtInterpolated | Pat::NestedLtgt | Pat::RootAt5At6 => self.lit(p, b"<"),
            Pat::NestedParensInterpolated | Pat::NestedParens | Pat::RootAt5At7 => {
                self.lit(p, b"(")
            }
            Pat::RootAt3At0End
            | Pat::NestedBracesInterpolatedEnd
            | Pat::RootAt4At0End
            | Pat::RootAt5At0End
            | Pat::NestedBracesEnd
            | Pat::RootAt5At4End
            | Pat::RootAt32End
            | Pat::RootAt36End
            | Pat::RootAt40End => self.lit(p, b"}"),
            Pat::RootAt3At1End
            | Pat::NestedBracketsInterpolatedEnd
            | Pat::RootAt4At1End
            | Pat::RootAt4At5At3End
            | Pat::RootAt5At1End
            | Pat::NestedBracketsEnd
            | Pat::RootAt5At5End
            | Pat::RootAt33End
            | Pat::RootAt37End
            | Pat::RootAt41End => self.lit(p, b"]"),
            Pat::RootAt3At2End
            | Pat::NestedLtgtInterpolatedEnd
            | Pat::RootAt4At2End
            | Pat::RootAt5At2End
            | Pat::NestedLtgtEnd
            | Pat::RootAt5At6End
            | Pat::RootAt34End
            | Pat::RootAt38End
            | Pat::RootAt42End => self.lit(p, b">"),
            Pat::RootAt3At3End
            | Pat::NestedParensInterpolatedEnd
            | Pat::RootAt4At3End
            | Pat::RootAt5At3End
            | Pat::NestedParensEnd
            | Pat::RootAt5At7End
            | Pat::RootAt31End
            | Pat::RootAt35End
            | Pat::RootAt39End => self.lit(p, b")"),
            Pat::RootAt3At4End
            | Pat::RootAt4At4End
            | Pat::RootAt5At8
            | Pat::RootAt5At8End
            | Pat::RootAt6At1
            | Pat::RootAt6At1End
            | Pat::RootAt7At1
            | Pat::RootAt26End
            | Pat::RootAt29
            | Pat::RootAt29End => self.lit(p, b"'"),
            Pat::RootAt27 | Pat::RootAt27End => self.lit(p, b"\""),
            Pat::RootAt30 | Pat::RootAt30End => self.lit(p, b"`"),

            Pat::HeredocAt0Indented => {
                self.heredoc(p, Tilde::Yes, Quote::Single, Label::Is(b"HTML"))
            }
            Pat::HeredocAt0 => self.heredoc(p, Tilde::No, Quote::Single, Label::Is(b"HTML")),
            Pat::HeredocAt1Indented => {
                self.heredoc(p, Tilde::Yes, Quote::Single, Label::Is(b"XML"))
            }
            Pat::HeredocAt1 => self.heredoc(p, Tilde::No, Quote::Single, Label::Is(b"XML")),
            Pat::HeredocAt2Indented => {
                self.heredoc(p, Tilde::Yes, Quote::Single, Label::Is(b"CSS"))
            }
            Pat::HeredocAt2 => self.heredoc(p, Tilde::No, Quote::Single, Label::Is(b"CSS")),
            Pat::HeredocAt3Indented => {
                self.heredoc(p, Tilde::Yes, Quote::Single, Label::Is(b"JAVASCRIPT"))
            }
            Pat::HeredocAt3 => self.heredoc(p, Tilde::No, Quote::Single, Label::Is(b"JAVASCRIPT")),
            Pat::HeredocAt4Indented => {
                self.heredoc(p, Tilde::Yes, Quote::Single, Label::Is(b"SQL"))
            }
            Pat::HeredocAt4 => self.heredoc(p, Tilde::No, Quote::Single, Label::Is(b"SQL")),
            Pat::HeredocAt6Indented => self.heredoc(p, Tilde::Yes, Quote::Single, Label::Quoted),
            Pat::HeredocAt6 => self.heredoc(p, Tilde::No, Quote::Single, Label::Quoted),
            Pat::HeredocAt7Indented => self.heredoc(p, Tilde::Yes, Quote::Backslash, Label::Bare),
            Pat::HeredocAt7 => self.heredoc(p, Tilde::No, Quote::Backslash, Label::Bare),
            Pat::HeredocAt8Indented => {
                self.heredoc(p, Tilde::Yes, Quote::Double, Label::Is(b"HTML"))
            }
            Pat::HeredocAt8 => self.heredoc(p, Tilde::No, Quote::Double, Label::Is(b"HTML")),
            Pat::HeredocAt9Indented => {
                self.heredoc(p, Tilde::Yes, Quote::Double, Label::Is(b"XML"))
            }
            Pat::HeredocAt9 => self.heredoc(p, Tilde::No, Quote::Double, Label::Is(b"XML")),
            Pat::HeredocAt10Indented => {
                self.heredoc(p, Tilde::Yes, Quote::Double, Label::Is(b"CSS"))
            }
            Pat::HeredocAt10 => self.heredoc(p, Tilde::No, Quote::Double, Label::Is(b"CSS")),
            Pat::HeredocAt11Indented => {
                self.heredoc(p, Tilde::Yes, Quote::Double, Label::Is(b"JAVASCRIPT"))
            }
            Pat::HeredocAt11 => self.heredoc(p, Tilde::No, Quote::Double, Label::Is(b"JAVASCRIPT")),
            Pat::HeredocAt12Indented => {
                self.heredoc(p, Tilde::Yes, Quote::Double, Label::Is(b"SQL"))
            }
            Pat::HeredocAt12 => self.heredoc(p, Tilde::No, Quote::Double, Label::Is(b"SQL")),
            Pat::HeredocAt13Indented => {
                self.heredoc(p, Tilde::Yes, Quote::Double, Label::Is(b"POSTSCRIPT"))
            }
            Pat::HeredocAt13 => self.heredoc(p, Tilde::No, Quote::Double, Label::Is(b"POSTSCRIPT")),
            Pat::HeredocAt14Indented => self.heredoc(p, Tilde::Yes, Quote::Double, Label::Quoted),
            Pat::HeredocAt14 => self.heredoc(p, Tilde::No, Quote::Double, Label::Quoted),
            Pat::HeredocAt15Indented => {
                self.heredoc(p, Tilde::Yes, Quote::None, Label::Is(b"HTML"))
            }
            Pat::HeredocAt15 => self.heredoc(p, Tilde::No, Quote::None, Label::Is(b"HTML")),
            Pat::HeredocAt16Indented => self.heredoc(p, Tilde::Yes, Quote::None, Label::Is(b"XML")),
            Pat::HeredocAt16 => self.heredoc(p, Tilde::No, Quote::None, Label::Is(b"XML")),
            Pat::HeredocAt17Indented => self.heredoc(p, Tilde::Yes, Quote::None, Label::Is(b"CSS")),
            Pat::HeredocAt17 => self.heredoc(p, Tilde::No, Quote::None, Label::Is(b"CSS")),
            Pat::HeredocAt18Indented => {
                self.heredoc(p, Tilde::Yes, Quote::None, Label::Is(b"JAVASCRIPT"))
            }
            Pat::HeredocAt18 => self.heredoc(p, Tilde::No, Quote::None, Label::Is(b"JAVASCRIPT")),
            Pat::HeredocAt19Indented => self.heredoc(p, Tilde::Yes, Quote::None, Label::Is(b"SQL")),
            Pat::HeredocAt19 => self.heredoc(p, Tilde::No, Quote::None, Label::Is(b"SQL")),
            Pat::HeredocAt20Indented => {
                self.heredoc(p, Tilde::Yes, Quote::None, Label::Is(b"POSTSCRIPT"))
            }
            Pat::HeredocAt20 => self.heredoc(p, Tilde::No, Quote::None, Label::Is(b"POSTSCRIPT")),
            Pat::HeredocAt21Indented => self.heredoc(p, Tilde::Yes, Quote::None, Label::Bare),
            Pat::HeredocAt21 => self.heredoc(p, Tilde::No, Quote::None, Label::Bare),
            Pat::HeredocAt22Indented => self.heredoc(p, Tilde::Yes, Quote::Backtick, Label::Quoted),
            Pat::HeredocAt22 => self.heredoc(p, Tilde::No, Quote::Backtick, Label::Quoted),
            Pat::HeredocAt0IndentedEnd
            | Pat::HeredocAt8IndentedEnd
            | Pat::HeredocAt15IndentedEnd => self.heredoc_end(p, Tilde::Yes, Some(b"HTML")),
            Pat::HeredocAt0End | Pat::HeredocAt8End | Pat::HeredocAt15End => {
                self.heredoc_end(p, Tilde::No, Some(b"HTML"))
            }
            Pat::HeredocAt1IndentedEnd
            | Pat::HeredocAt9IndentedEnd
            | Pat::HeredocAt16IndentedEnd => self.heredoc_end(p, Tilde::Yes, Some(b"XML")),
            Pat::HeredocAt1End | Pat::HeredocAt9End | Pat::HeredocAt16End => {
                self.heredoc_end(p, Tilde::No, Some(b"XML"))
            }
            Pat::HeredocAt2IndentedEnd
            | Pat::HeredocAt10IndentedEnd
            | Pat::HeredocAt17IndentedEnd => self.heredoc_end(p, Tilde::Yes, Some(b"CSS")),
            Pat::HeredocAt2End | Pat::HeredocAt10End | Pat::HeredocAt17End => {
                self.heredoc_end(p, Tilde::No, Some(b"CSS"))
            }
            Pat::HeredocAt3IndentedEnd
            | Pat::HeredocAt11IndentedEnd
            | Pat::HeredocAt18IndentedEnd => self.heredoc_end(p, Tilde::Yes, Some(b"JAVASCRIPT")),
            Pat::HeredocAt3End | Pat::HeredocAt11End | Pat::HeredocAt18End => {
                self.heredoc_end(p, Tilde::No, Some(b"JAVASCRIPT"))
            }
            Pat::HeredocAt4IndentedEnd
            | Pat::HeredocAt12IndentedEnd
            | Pat::HeredocAt19IndentedEnd => self.heredoc_end(p, Tilde::Yes, Some(b"SQL")),
            Pat::HeredocAt4End | Pat::HeredocAt12End | Pat::HeredocAt19End => {
                self.heredoc_end(p, Tilde::No, Some(b"SQL"))
            }
            Pat::HeredocAt13IndentedEnd | Pat::HeredocAt20IndentedEnd => {
                self.heredoc_end(p, Tilde::Yes, Some(b"POSTSCRIPT"))
            }
            Pat::HeredocAt13End | Pat::HeredocAt20End => {
                self.heredoc_end(p, Tilde::No, Some(b"POSTSCRIPT"))
            }
            Pat::HeredocAt6IndentedEnd
            | Pat::HeredocAt7IndentedEnd
            | Pat::HeredocAt14IndentedEnd
            | Pat::HeredocAt21IndentedEnd
            | Pat::HeredocAt22IndentedEnd => self.heredoc_end(p, Tilde::Yes, None),
            Pat::HeredocAt6End
            | Pat::HeredocAt7End
            | Pat::HeredocAt14End
            | Pat::HeredocAt21End
            | Pat::HeredocAt22End => self.heredoc_end(p, Tilde::No, None),
            Pat::HeredocAt0At0
            | Pat::HeredocAt1IndentedAt0
            | Pat::HeredocAt2IndentedAt0
            | Pat::HeredocAt3IndentedAt0
            | Pat::HeredocAt4IndentedAt0
            | Pat::HeredocAt8IndentedAt0
            | Pat::HeredocAt9IndentedAt0
            | Pat::HeredocAt10IndentedAt0
            | Pat::HeredocAt11IndentedAt0
            | Pat::HeredocAt12IndentedAt0
            | Pat::HeredocAt13IndentedAt0 => (p == 0).then_some(0),

            Pat::RootAt9 => self.hash_key(p),
            Pat::RootAt10 => self.braced_word(p),
            Pat::RootAt11 => self.package(p),
            Pat::RootAt12 => self.sub(p),
            Pat::RootAt13 => self.special_block(p),
            Pat::RootAt14 => self.indent_start(p),
            Pat::RootAt14End => self.indent_end(p),
            Pat::RootAt14At0 => self.indent(p),
            Pat::RootAt15 => self.transliteration(p),
            Pat::RootAt16 => self.magic_constant(p),
            Pat::RootAt17 => self.data_section(p),
            Pat::RootAt17End => None,
            Pat::RootAt18 => self.control_keyword(p),
            Pat::RootAt19 => self.word_in(p, &[b"my", b"our", b"local"]),
            Pat::RootAt20 => self.file_test(p),
            Pat::RootAt21 => self.word_in(p, &[b"and", b"or", b"xor", b"as", b"not"]),
            Pat::RootAt22 => self.arrow(p),
            Pat::RootAt43 => self.dunder_line(p),
            Pat::RootAt43End => self.at_eol(p).then_some(p),
            Pat::RootAt44 => self.format(p),
            Pat::RootAt44End => self.format_end(p),
            Pat::RootAt45 => self.repetition(p),
            Pat::RootAt46 => self.function(p),
            Pat::RootAt47 => self.empty_pair(p, b'{', b'}'),
            Pat::RootAt48 => self.empty_pair(p, b'(', b')'),
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
        self.cap(1, p, end);
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
            self.cap(1, p, p + 1);
            self.cap(2, p + 1, p + 2);
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
