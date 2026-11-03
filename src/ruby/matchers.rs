//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where. A matcher records the
//! capture groups the grammar names with [`Lexer::cap`]; the engine names them.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would. Where a quantifier could give characters back, the doc comment says
//! why it cannot matter.

mod chars;
mod comments;
mod defs;
mod embeds;
mod heredocs;
mod literals;
mod names;
mod operators;
mod regexp;
mod words;

use embeds::EMBEDDED_NAMES;

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
            Pat::RootAt0 => self.class_definition(p),
            Pat::RootAt1 => self.module_definition(p),
            Pat::RootAt2 => self.singleton_class(p),
            Pat::RootAt3 => self.else_if(p),
            Pat::RootAt4 => self.line_logical_assignment(p),
            Pat::RootAt5 => self.condition_logical_assignment(p),
            Pat::RootAt6 => self.line_augmented_assignment(p),
            Pat::RootAt7 => self.condition_augmented_assignment(p),
            Pat::RootAt8 => self.line_assignment(p),
            Pat::RootAt9 => self.condition_assignment(p),
            Pat::RootAt10 => self.hash_key(p),
            Pat::RootAt11 => self.arrow_key(p),
            Pat::RootAt12 => self.control_word(p),
            Pat::RootAt13 => self.do_word(p),
            Pat::RootAt14 => self.blanks_after_brace(p),
            Pat::RootAt15 => self.pseudo_method(p),
            Pat::RootAt16 => self.nil_word(p),
            Pat::RootAt17 => self.boolean_word(p),
            Pat::RootAt18 => self.file_line_word(p),
            Pat::RootAt19 => self.self_word(p),
            Pat::RootAt20 => self.special_method(p),
            Pat::RootAt21 => self.require_word(p),
            Pat::RootAt21End => self.require_end(p),
            Pat::RootAt22 => self.sigil_name(p, b"@"),
            Pat::RootAt23 => self.sigil_name(p, b"@@"),
            Pat::RootAt24 => self.sigil_name(p, b"$"),
            Pat::RootAt25 => self.special_global(p),
            Pat::RootAt26 => self.env_index(p),
            Pat::RootAt27 => self.constant_before_call(p),
            Pat::RootAt28 => self.kernel_method(p),
            Pat::RootAt29 => self.constant(p),
            Pat::RootAt30 => self.lambda_parens(p),
            Pat::RootAt30At0 | Pat::RootAt31At0 | Pat::RootAt32At0 => self.param_start(p),
            Pat::RootAt30At0End | Pat::RootAt31At0End => self.param_end(p),
            Pat::RootAt31 => self.def_with_parens(p),
            Pat::Params => self.param_name(p),
            Pat::RootAt32 => self.def_with_bare_params(p),
            Pat::RootAt32End => self.bare_params_end(p),
            Pat::RootAt32At0End => self.bare_param_end(p),
            Pat::RootAt33 => self.def_without_params(p),
            Pat::RootAt34 => self.number(p),
            Pat::RootAt35 => self.lit(p, b":'"),
            Pat::RootAt35At0 | Pat::RootAt38At0 => self.escaped_one_of(p, b"'\\"),
            Pat::RootAt36 => self.lit(p, b":\""),
            Pat::RootAt37 => self.divide_assign(p),
            Pat::RootAt35End | Pat::RootAt38 | Pat::RootAt38End => self.lit(p, b"'"),
            Pat::RootAt36End | Pat::RootAt39 | Pat::RootAt39End => self.lit(p, b"\""),
            Pat::InterpolatedRubyAt0 => self.lit(p, b"#{"),
            Pat::InterpolatedRubyAt1 => self.sigil_name(p, b"#@"),
            Pat::InterpolatedRubyAt2 => self.sigil_name(p, b"#@@"),
            Pat::InterpolatedRubyAt3 => self.sigil_name(p, b"#$"),
            Pat::EscapedChar => self.escaped_char(p),
            Pat::RootAt40 => self.command_start(p),
            Pat::RootAt40End => self.lit(p, b"`"),
            Pat::RootAt41 => self.regexp_start(p),
            Pat::RootAt41End => self.regexp_end(p),
            Pat::RegexSubAt2 => self.regexp_repetition(p),
            Pat::RegexSubAt3 => self.regexp_class_start(p),
            Pat::RegexSubAt4 => self.lit(p, b"(?#"),
            Pat::RegexSubAt6 => self.regexp_comment(p),
            Pat::RegexSubAt6End => self.line_end_and_feed(p),
            Pat::RootAt42 => self.lit(p, b"%r{"),
            Pat::RootAt42End => self.close_with_flags(p, b'}'),
            Pat::RootAt43 => self.lit(p, b"%r["),
            Pat::RootAt43End => self.close_with_flags(p, b']'),
            Pat::RootAt44 => self.lit(p, b"%r("),
            Pat::RootAt44End => self.close_with_flags(p, b')'),
            Pat::RootAt45 => self.lit(p, b"%r<"),
            Pat::RootAt45End => self.close_with_flags(p, b'>'),
            Pat::RootAt46 => self.percent_any(p, b"%r"),
            Pat::RootAt46End => self.repeated_delimiter_flags(p),
            Pat::RootAt47 => self.lit(p, b"%I["),
            Pat::RootAt48 => self.lit(p, b"%I("),
            Pat::RootAt49 => self.lit(p, b"%I<"),
            Pat::RootAt50 => self.lit(p, b"%I{"),
            Pat::RootAt51 => self.percent_any(p, b"%I"),
            Pat::RootAt52 => self.lit(p, b"%i["),
            Pat::RootAt53 => self.lit(p, b"%i("),
            Pat::RootAt54 => self.lit(p, b"%i<"),
            Pat::RootAt55 => self.lit(p, b"%i{"),
            Pat::RootAt56 => self.percent_any(p, b"%i"),
            Pat::RootAt57 => self.lit(p, b"%W["),
            Pat::RootAt58 => self.lit(p, b"%W("),
            Pat::RootAt59 => self.lit(p, b"%W<"),
            Pat::RootAt60 => self.lit(p, b"%W{"),
            Pat::RootAt61 => self.percent_any(p, b"%W"),
            Pat::RootAt62 => self.lit(p, b"%w["),
            Pat::RootAt63 => self.lit(p, b"%w("),
            Pat::RootAt64 => self.lit(p, b"%w<"),
            Pat::RootAt65 => self.lit(p, b"%w{"),
            Pat::RootAt66 => self.percent_any(p, b"%w"),
            Pat::RootAt67 => self.percent_quoted(p, b'('),
            Pat::RootAt68 => self.percent_quoted(p, b'['),
            Pat::RootAt69 => self.percent_quoted(p, b'{'),
            Pat::RootAt70 => self.percent_quoted(p, b'<'),
            Pat::RootAt71 => self.percent_quoted_any(p),
            Pat::RootAt72 => self.percent_bare(p),
            Pat::RootAt73 => self.lit(p, b"%q("),
            Pat::RootAt74 => self.lit(p, b"%q<"),
            Pat::RootAt75 => self.lit(p, b"%q["),
            Pat::RootAt76 => self.lit(p, b"%q{"),
            Pat::RootAt77 => self.percent_any(p, b"%q"),
            Pat::RootAt78 => self.lit(p, b"%s("),
            Pat::RootAt79 => self.lit(p, b"%s<"),
            Pat::RootAt80 => self.lit(p, b"%s["),
            Pat::RootAt81 => self.lit(p, b"%s{"),
            Pat::RootAt82 => self.percent_any(p, b"%s"),
            Pat::RootAt51End
            | Pat::RootAt56End
            | Pat::RootAt61End
            | Pat::RootAt66End
            | Pat::RootAt71End
            | Pat::RootAt72End
            | Pat::RootAt77End
            | Pat::RootAt82End => self.repeated_delimiter(p),
            Pat::RootAt52At0 | Pat::RootAt62At0 | Pat::RootAt75At0 | Pat::RootAt80At0 => {
                self.escaped_one_of(p, b"]\\")
            }
            Pat::RootAt53At0 | Pat::RootAt63At0 | Pat::RootAt73At0 | Pat::RootAt78At0 => {
                self.escaped_one_of(p, b")\\")
            }
            Pat::RootAt54At0 | Pat::RootAt64At0 | Pat::RootAt74At0 | Pat::RootAt79At0 => {
                self.escaped_one_of(p, b">\\")
            }
            Pat::RootAt55At0 | Pat::RootAt65At0 | Pat::RootAt76At0 | Pat::RootAt81At0 => {
                self.escaped_one_of(p, b"}\\")
            }
            Pat::RootAt56At0 | Pat::RootAt66At0 | Pat::RootAt77At0 | Pat::RootAt82At0 => {
                self.escaped_any(p)
            }
            Pat::RootAt83 => self.symbol(p),
            Pat::RootAt84 => self.line_start_word(p, b"=begin"),
            Pat::RootAt84End => self.line_start_word(p, b"=end"),
            Pat::YardComment => self.yard_comment(p),
            Pat::YardParamTypes => self.yard_param_types(p),
            Pat::YardOption => self.yard_option(p),
            Pat::YardTag => self.yard_private(p),
            Pat::YardTypes => self.yard_types(p),
            Pat::YardDirective => self.yard_directive(p),
            Pat::YardSee => self.yard_see(p),
            Pat::YardMacroAttribute => self.yard_macro_attribute(p),
            Pat::YardCommentEnd
            | Pat::YardParamTypesEnd
            | Pat::YardOptionEnd
            | Pat::YardTypesEnd
            | Pat::YardDirectiveEnd
            | Pat::YardSeeEnd
            | Pat::YardMacroAttributeEnd => self.yard_end(p),
            Pat::YardContinuation => self.yard_continuation(p),
            Pat::RootAt86 => self.comment_start(p),
            Pat::RootAt86End => self.away_from_anchor(p),
            Pat::RootAt86At0 => self.lit(p, b"#"),
            Pat::RootAt86At0End => self.line_feed(p),
            Pat::RootAt87 => self.character_literal(p),
            Pat::RootAt88 => self.data_marker(p),
            Pat::RootAt88End | Pat::RootAt88At0End => None,
            Pat::RootAt88At0 => self.data_html(p),
            Pat::RootAt89 => self.embedded_heredoc_ahead(p, EMBEDDED_NAMES[0]),
            Pat::RootAt89End => self.embedded_heredoc_end(p, EMBEDDED_NAMES[0]),
            Pat::RootAt89At0 => self.embedded_heredoc(p, EMBEDDED_NAMES[0]),
            Pat::RootAt89At0While => self.embedded_heredoc_goes_on(p, EMBEDDED_NAMES[0]),
            Pat::RootAt90 => self.embedded_heredoc_ahead(p, EMBEDDED_NAMES[1]),
            Pat::RootAt90End => self.embedded_heredoc_end(p, EMBEDDED_NAMES[1]),
            Pat::RootAt90At0 => self.embedded_heredoc(p, EMBEDDED_NAMES[1]),
            Pat::RootAt90At0While => self.embedded_heredoc_goes_on(p, EMBEDDED_NAMES[1]),
            Pat::RootAt91 => self.embedded_heredoc_ahead(p, EMBEDDED_NAMES[2]),
            Pat::RootAt91End => self.embedded_heredoc_end(p, EMBEDDED_NAMES[2]),
            Pat::RootAt91At0 => self.embedded_heredoc(p, EMBEDDED_NAMES[2]),
            Pat::RootAt91At0While => self.embedded_heredoc_goes_on(p, EMBEDDED_NAMES[2]),
            Pat::RootAt92 => self.embedded_heredoc_ahead(p, EMBEDDED_NAMES[3]),
            Pat::RootAt92End => self.embedded_heredoc_end(p, EMBEDDED_NAMES[3]),
            Pat::RootAt92At0 => self.embedded_heredoc(p, EMBEDDED_NAMES[3]),
            Pat::RootAt92At0While => self.embedded_heredoc_goes_on(p, EMBEDDED_NAMES[3]),
            Pat::RootAt93 => self.embedded_heredoc_ahead(p, EMBEDDED_NAMES[4]),
            Pat::RootAt93End => self.embedded_heredoc_end(p, EMBEDDED_NAMES[4]),
            Pat::RootAt93At0 => self.embedded_heredoc(p, EMBEDDED_NAMES[4]),
            Pat::RootAt93At0While => self.embedded_heredoc_goes_on(p, EMBEDDED_NAMES[4]),
            Pat::RootAt94 => self.embedded_heredoc_ahead(p, EMBEDDED_NAMES[5]),
            Pat::RootAt94End => self.embedded_heredoc_end(p, EMBEDDED_NAMES[5]),
            Pat::RootAt94At0 => self.embedded_heredoc(p, EMBEDDED_NAMES[5]),
            Pat::RootAt94At0While => self.embedded_heredoc_goes_on(p, EMBEDDED_NAMES[5]),
            Pat::RootAt95 => self.embedded_heredoc_ahead(p, EMBEDDED_NAMES[6]),
            Pat::RootAt95End => self.embedded_heredoc_end(p, EMBEDDED_NAMES[6]),
            Pat::RootAt95At0 => self.embedded_heredoc(p, EMBEDDED_NAMES[6]),
            Pat::RootAt95At0While => self.embedded_heredoc_goes_on(p, EMBEDDED_NAMES[6]),
            Pat::RootAt96 => self.embedded_heredoc_ahead(p, EMBEDDED_NAMES[7]),
            Pat::RootAt96End => self.embedded_heredoc_end(p, EMBEDDED_NAMES[7]),
            Pat::RootAt96At0 => self.embedded_heredoc(p, EMBEDDED_NAMES[7]),
            Pat::RootAt96At0While => self.embedded_heredoc_goes_on(p, EMBEDDED_NAMES[7]),
            Pat::RootAt97 => self.embedded_heredoc_ahead(p, EMBEDDED_NAMES[8]),
            Pat::RootAt97End => self.embedded_heredoc_end(p, EMBEDDED_NAMES[8]),
            Pat::RootAt97At0 => self.embedded_heredoc(p, EMBEDDED_NAMES[8]),
            Pat::RootAt97At0While => self.embedded_heredoc_goes_on(p, EMBEDDED_NAMES[8]),
            Pat::RootAt98 => self.embedded_heredoc_ahead(p, EMBEDDED_NAMES[9]),
            Pat::RootAt98End => self.embedded_heredoc_end(p, EMBEDDED_NAMES[9]),
            Pat::RootAt98At0 => self.embedded_heredoc(p, EMBEDDED_NAMES[9]),
            Pat::RootAt98At0While => self.embedded_heredoc_goes_on(p, EMBEDDED_NAMES[9]),
            Pat::RootAt99 => self.embedded_heredoc_ahead(p, EMBEDDED_NAMES[10]),
            Pat::RootAt99End => self.embedded_heredoc_end(p, EMBEDDED_NAMES[10]),
            Pat::RootAt99At0 => self.embedded_heredoc(p, EMBEDDED_NAMES[10]),
            Pat::RootAt99At0While => self.embedded_heredoc_goes_on(p, EMBEDDED_NAMES[10]),
            Pat::RootAt100 => self.embedded_heredoc_ahead(p, EMBEDDED_NAMES[11]),
            Pat::RootAt100End => self.embedded_heredoc_end(p, EMBEDDED_NAMES[11]),
            Pat::RootAt100At0 => self.embedded_heredoc(p, EMBEDDED_NAMES[11]),
            Pat::RootAt100At0While => self.embedded_heredoc_goes_on(p, EMBEDDED_NAMES[11]),
            Pat::RootAt101 => self.embedded_heredoc_ahead(p, EMBEDDED_NAMES[12]),
            Pat::RootAt101End => self.embedded_heredoc_end(p, EMBEDDED_NAMES[12]),
            Pat::RootAt101At0 => self.embedded_heredoc(p, EMBEDDED_NAMES[12]),
            Pat::RootAt101At0While => self.embedded_heredoc_goes_on(p, EMBEDDED_NAMES[12]),
            Pat::RootAt102 => self.embedded_heredoc_ahead(p, EMBEDDED_NAMES[13]),
            Pat::RootAt102End => self.embedded_heredoc_end(p, EMBEDDED_NAMES[13]),
            Pat::RootAt102At0 => self.embedded_heredoc(p, EMBEDDED_NAMES[13]),
            Pat::RootAt102At0While => self.embedded_heredoc_goes_on(p, EMBEDDED_NAMES[13]),
            Pat::RootAt103 => self.embedded_heredoc_ahead(p, EMBEDDED_NAMES[14]),
            Pat::RootAt103End => self.embedded_heredoc_end(p, EMBEDDED_NAMES[14]),
            Pat::RootAt103At0 => self.embedded_heredoc(p, EMBEDDED_NAMES[14]),
            Pat::RootAt103At0While => self.embedded_heredoc_goes_on(p, EMBEDDED_NAMES[14]),
            Pat::RootAt104 => self.assigned_heredoc(p),
            Pat::RootAt104End => self.assigned_heredoc_end(p),
            Pat::Heredoc => self.heredoc_at_line_start(p),
            Pat::HeredocEnd => self.line_end(p),
            Pat::RootAt105 => self.heredoc_openers(p),
            Pat::RootAt105End => self.heredoc_openers_end(p),
            Pat::RootAt106 => self.block_params_open(p),
            Pat::RootAt106End => self.block_params_close(p),
            Pat::RootAt106At0 => self.block_param_start(p),
            Pat::RootAt106At0End => self.block_param_end(p),
            Pat::RootAt106At0At0 => self.block_param_name(p),
            Pat::RootAt107 => self.lit(p, b"=>"),
            Pat::RootAt108 => self.lit(p, b"->"),
            Pat::RootAt109 => self.assignment_operator(p),
            Pat::RootAt110 => self.comparison(p),
            Pat::RootAt111 => self.logical_word(p),
            Pat::RootAt112 => self.logical_operator(p),
            Pat::RootAt113 => self.safe_navigation(p),
            Pat::RootAt114 => self.arithmetic(p),
            Pat::RootAt115 => self.lit(p, b"="),
            Pat::RootAt116 => self.bitwise(p),
            Pat::RootAt117 => self.lit(p, b";"),
            Pat::RootAt106At1 | Pat::RootAt118 => self.lit(p, b","),
            Pat::RootAt119 => self.namespace_separator(p),
            Pat::RootAt120 => self.method_separator(p),
            Pat::RootAt121 => self.lit(p, b":"),
            Pat::RootAt126 => self.paren(p),
            Pat::RootAt127 => self.method_call_start(p),
            Pat::RootAt127End => self.method_call_end(p),
            Pat::RootAt127At0 => self.method_call_name(p),
            Pat::RootAt128 => self.call_with_parens(p),
            Pat::RootAt128End => self.closing_paren(p),
            Pat::Parens
            | Pat::RegexSubAt5
            | Pat::NestParensR
            | Pat::NestParensI
            | Pat::NestParens => self.lit(p, b"("),
            Pat::ParensEnd
            | Pat::RootAt30End
            | Pat::RootAt31End
            | Pat::RegexSubAt4End
            | Pat::RegexSubAt5End
            | Pat::NestParensREnd
            | Pat::RootAt48End
            | Pat::NestParensIEnd
            | Pat::RootAt53End
            | Pat::NestParensEnd
            | Pat::RootAt58End
            | Pat::RootAt63End
            | Pat::RootAt67End
            | Pat::RootAt73End
            | Pat::RootAt78End => self.lit(p, b")"),
            Pat::Braces
            | Pat::NestCurlyAndSelfAt0
            | Pat::NestCurlyR
            | Pat::NestCurlyI
            | Pat::NestCurly
            | Pat::RootAt122 => self.lit(p, b"{"),
            Pat::BracesEnd
            | Pat::InterpolatedRubyAt0End
            | Pat::NestCurlyAndSelfAt0End
            | Pat::NestCurlyREnd
            | Pat::RootAt50End
            | Pat::NestCurlyIEnd
            | Pat::RootAt55End
            | Pat::NestCurlyEnd
            | Pat::RootAt60End
            | Pat::RootAt65End
            | Pat::RootAt69End
            | Pat::RootAt76End
            | Pat::RootAt81End
            | Pat::RootAt123 => self.lit(p, b"}"),
            Pat::Brackets
            | Pat::NestBracketsR
            | Pat::NestBracketsI
            | Pat::NestBrackets
            | Pat::RootAt124 => self.lit(p, b"["),
            Pat::BracketsEnd
            | Pat::RootAt26End
            | Pat::RegexSubAt3End
            | Pat::NestBracketsREnd
            | Pat::RootAt47End
            | Pat::NestBracketsIEnd
            | Pat::RootAt52End
            | Pat::NestBracketsEnd
            | Pat::RootAt57End
            | Pat::RootAt62End
            | Pat::RootAt68End
            | Pat::RootAt75End
            | Pat::RootAt80End
            | Pat::RootAt125 => self.lit(p, b"]"),
            Pat::NestLtgtR | Pat::NestLtgtI | Pat::NestLtgt => self.lit(p, b"<"),
            Pat::NestLtgtREnd
            | Pat::RootAt49End
            | Pat::NestLtgtIEnd
            | Pat::RootAt54End
            | Pat::NestLtgtEnd
            | Pat::RootAt59End
            | Pat::RootAt64End
            | Pat::RootAt70End
            | Pat::RootAt74End
            | Pat::RootAt79End => self.lit(p, b">"),
        }
    }

    /// `(\))`
    fn closing_paren(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, b")")?;
        self.cap(1, p, end);
        Some(end)
    }
}
