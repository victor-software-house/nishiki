//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups the grammar names with
//! [`Lexer::capture`]. The engine names them.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would.
//! Where a quantifier could give characters back, the doc comment says why it
//! cannot matter.

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
            Pattern::RootAt0 => self.class_definition(p),
            Pattern::RootAt1 => self.module_definition(p),
            Pattern::RootAt2 => self.singleton_class(p),
            Pattern::RootAt3 => self.else_if(p),
            Pattern::RootAt4 => self.line_logical_assignment(p),
            Pattern::RootAt5 => self.condition_logical_assignment(p),
            Pattern::RootAt6 => self.line_augmented_assignment(p),
            Pattern::RootAt7 => self.condition_augmented_assignment(p),
            Pattern::RootAt8 => self.line_assignment(p),
            Pattern::RootAt9 => self.condition_assignment(p),
            Pattern::RootAt10 => self.hash_key(p),
            Pattern::RootAt11 => self.arrow_key(p),
            Pattern::RootAt12 => self.control_word(p),
            Pattern::RootAt13 => self.do_word(p),
            Pattern::RootAt14 => self.blanks_after_brace(p),
            Pattern::RootAt15 => self.pseudo_method(p),
            Pattern::RootAt16 => self.nil_word(p),
            Pattern::RootAt17 => self.boolean_word(p),
            Pattern::RootAt18 => self.file_line_word(p),
            Pattern::RootAt19 => self.self_word(p),
            Pattern::RootAt20 => self.special_method(p),
            Pattern::RootAt21 => self.require_word(p),
            Pattern::RootAt21End => self.require_end(p),
            Pattern::RootAt22 => self.sigil_name(p, b"@"),
            Pattern::RootAt23 => self.sigil_name(p, b"@@"),
            Pattern::RootAt24 => self.sigil_name(p, b"$"),
            Pattern::RootAt25 => self.special_global(p),
            Pattern::RootAt26 => self.env_index(p),
            Pattern::RootAt27 => self.constant_before_call(p),
            Pattern::RootAt28 => self.kernel_method(p),
            Pattern::RootAt29 => self.constant(p),
            Pattern::RootAt30 => self.lambda_parens(p),
            Pattern::RootAt30At0 | Pattern::RootAt31At0 | Pattern::RootAt32At0 => {
                self.param_start(p)
            }
            Pattern::RootAt30At0End | Pattern::RootAt31At0End => self.param_end(p),
            Pattern::RootAt31 => self.def_with_parens(p),
            Pattern::Params => self.param_name(p),
            Pattern::RootAt32 => self.def_with_bare_params(p),
            Pattern::RootAt32End => self.bare_params_end(p),
            Pattern::RootAt32At0End => self.bare_param_end(p),
            Pattern::RootAt33 => self.def_without_params(p),
            Pattern::RootAt34 => self.number(p),
            Pattern::RootAt35 => self.lit(p, b":'"),
            Pattern::RootAt35At0 | Pattern::RootAt38At0 => self.escaped_one_of(p, b"'\\"),
            Pattern::RootAt36 => self.lit(p, b":\""),
            Pattern::RootAt37 => self.divide_assign(p),
            Pattern::RootAt35End | Pattern::RootAt38 | Pattern::RootAt38End => self.lit(p, b"'"),
            Pattern::RootAt36End | Pattern::RootAt39 | Pattern::RootAt39End => self.lit(p, b"\""),
            Pattern::InterpolatedRubyAt0 => self.lit(p, b"#{"),
            Pattern::InterpolatedRubyAt1 => self.sigil_name(p, b"#@"),
            Pattern::InterpolatedRubyAt2 => self.sigil_name(p, b"#@@"),
            Pattern::InterpolatedRubyAt3 => self.sigil_name(p, b"#$"),
            Pattern::EscapedChar => self.escaped_char(p),
            Pattern::RootAt40 => self.command_start(p),
            Pattern::RootAt40End => self.lit(p, b"`"),
            Pattern::RootAt41 => self.regexp_start(p),
            Pattern::RootAt41End => self.regexp_end(p),
            Pattern::RegexSubAt2 => self.regexp_repetition(p),
            Pattern::RegexSubAt3 => self.regexp_class_start(p),
            Pattern::RegexSubAt4 => self.lit(p, b"(?#"),
            Pattern::RegexSubAt6 => self.regexp_comment(p),
            Pattern::RegexSubAt6End => self.line_end_and_feed(p),
            Pattern::RootAt42 => self.lit(p, b"%r{"),
            Pattern::RootAt42End => self.close_with_flags(p, b'}'),
            Pattern::RootAt43 => self.lit(p, b"%r["),
            Pattern::RootAt43End => self.close_with_flags(p, b']'),
            Pattern::RootAt44 => self.lit(p, b"%r("),
            Pattern::RootAt44End => self.close_with_flags(p, b')'),
            Pattern::RootAt45 => self.lit(p, b"%r<"),
            Pattern::RootAt45End => self.close_with_flags(p, b'>'),
            Pattern::RootAt46 => self.percent_any(p, b"%r"),
            Pattern::RootAt46End => self.repeated_delimiter_flags(p),
            Pattern::RootAt47 => self.lit(p, b"%I["),
            Pattern::RootAt48 => self.lit(p, b"%I("),
            Pattern::RootAt49 => self.lit(p, b"%I<"),
            Pattern::RootAt50 => self.lit(p, b"%I{"),
            Pattern::RootAt51 => self.percent_any(p, b"%I"),
            Pattern::RootAt52 => self.lit(p, b"%i["),
            Pattern::RootAt53 => self.lit(p, b"%i("),
            Pattern::RootAt54 => self.lit(p, b"%i<"),
            Pattern::RootAt55 => self.lit(p, b"%i{"),
            Pattern::RootAt56 => self.percent_any(p, b"%i"),
            Pattern::RootAt57 => self.lit(p, b"%W["),
            Pattern::RootAt58 => self.lit(p, b"%W("),
            Pattern::RootAt59 => self.lit(p, b"%W<"),
            Pattern::RootAt60 => self.lit(p, b"%W{"),
            Pattern::RootAt61 => self.percent_any(p, b"%W"),
            Pattern::RootAt62 => self.lit(p, b"%w["),
            Pattern::RootAt63 => self.lit(p, b"%w("),
            Pattern::RootAt64 => self.lit(p, b"%w<"),
            Pattern::RootAt65 => self.lit(p, b"%w{"),
            Pattern::RootAt66 => self.percent_any(p, b"%w"),
            Pattern::RootAt67 => self.percent_quoted(p, b'('),
            Pattern::RootAt68 => self.percent_quoted(p, b'['),
            Pattern::RootAt69 => self.percent_quoted(p, b'{'),
            Pattern::RootAt70 => self.percent_quoted(p, b'<'),
            Pattern::RootAt71 => self.percent_quoted_any(p),
            Pattern::RootAt72 => self.percent_bare(p),
            Pattern::RootAt73 => self.lit(p, b"%q("),
            Pattern::RootAt74 => self.lit(p, b"%q<"),
            Pattern::RootAt75 => self.lit(p, b"%q["),
            Pattern::RootAt76 => self.lit(p, b"%q{"),
            Pattern::RootAt77 => self.percent_any(p, b"%q"),
            Pattern::RootAt78 => self.lit(p, b"%s("),
            Pattern::RootAt79 => self.lit(p, b"%s<"),
            Pattern::RootAt80 => self.lit(p, b"%s["),
            Pattern::RootAt81 => self.lit(p, b"%s{"),
            Pattern::RootAt82 => self.percent_any(p, b"%s"),
            Pattern::RootAt51End
            | Pattern::RootAt56End
            | Pattern::RootAt61End
            | Pattern::RootAt66End
            | Pattern::RootAt71End
            | Pattern::RootAt72End
            | Pattern::RootAt77End
            | Pattern::RootAt82End => self.repeated_delimiter(p),
            Pattern::RootAt52At0
            | Pattern::RootAt62At0
            | Pattern::RootAt75At0
            | Pattern::RootAt80At0 => self.escaped_one_of(p, b"]\\"),
            Pattern::RootAt53At0
            | Pattern::RootAt63At0
            | Pattern::RootAt73At0
            | Pattern::RootAt78At0 => self.escaped_one_of(p, b")\\"),
            Pattern::RootAt54At0
            | Pattern::RootAt64At0
            | Pattern::RootAt74At0
            | Pattern::RootAt79At0 => self.escaped_one_of(p, b">\\"),
            Pattern::RootAt55At0
            | Pattern::RootAt65At0
            | Pattern::RootAt76At0
            | Pattern::RootAt81At0 => self.escaped_one_of(p, b"}\\"),
            Pattern::RootAt56At0
            | Pattern::RootAt66At0
            | Pattern::RootAt77At0
            | Pattern::RootAt82At0 => self.escaped_any(p),
            Pattern::RootAt83 => self.symbol(p),
            Pattern::RootAt84 => self.line_start_word(p, b"=begin"),
            Pattern::RootAt84End => self.line_start_word(p, b"=end"),
            Pattern::YardComment => self.yard_comment(p),
            Pattern::YardParamTypes => self.yard_param_types(p),
            Pattern::YardOption => self.yard_option(p),
            Pattern::YardTag => self.yard_private(p),
            Pattern::YardTypes => self.yard_types(p),
            Pattern::YardDirective => self.yard_directive(p),
            Pattern::YardSee => self.yard_see(p),
            Pattern::YardMacroAttribute => self.yard_macro_attribute(p),
            Pattern::YardCommentEnd
            | Pattern::YardParamTypesEnd
            | Pattern::YardOptionEnd
            | Pattern::YardTypesEnd
            | Pattern::YardDirectiveEnd
            | Pattern::YardSeeEnd
            | Pattern::YardMacroAttributeEnd => self.yard_end(p),
            Pattern::YardContinuation => self.yard_continuation(p),
            Pattern::RootAt86 => self.comment_start(p),
            Pattern::RootAt86End => self.away_from_anchor(p),
            Pattern::RootAt86At0 => self.lit(p, b"#"),
            Pattern::RootAt86At0End => self.line_feed(p),
            Pattern::RootAt87 => self.character_literal(p),
            Pattern::RootAt88 => self.data_marker(p),
            Pattern::RootAt88End | Pattern::RootAt88At0End => None,
            Pattern::RootAt88At0 => self.data_html(p),
            Pattern::RootAt89 => self.embedded_heredoc_ahead(p, EMBEDDED_NAMES[0]),
            Pattern::RootAt89End => self.embedded_heredoc_end(p, EMBEDDED_NAMES[0]),
            Pattern::RootAt89At0 => self.embedded_heredoc(p, EMBEDDED_NAMES[0]),
            Pattern::RootAt89At0While => self.embedded_heredoc_goes_on(p, EMBEDDED_NAMES[0]),
            Pattern::RootAt90 => self.embedded_heredoc_ahead(p, EMBEDDED_NAMES[1]),
            Pattern::RootAt90End => self.embedded_heredoc_end(p, EMBEDDED_NAMES[1]),
            Pattern::RootAt90At0 => self.embedded_heredoc(p, EMBEDDED_NAMES[1]),
            Pattern::RootAt90At0While => self.embedded_heredoc_goes_on(p, EMBEDDED_NAMES[1]),
            Pattern::RootAt91 => self.embedded_heredoc_ahead(p, EMBEDDED_NAMES[2]),
            Pattern::RootAt91End => self.embedded_heredoc_end(p, EMBEDDED_NAMES[2]),
            Pattern::RootAt91At0 => self.embedded_heredoc(p, EMBEDDED_NAMES[2]),
            Pattern::RootAt91At0While => self.embedded_heredoc_goes_on(p, EMBEDDED_NAMES[2]),
            Pattern::RootAt92 => self.embedded_heredoc_ahead(p, EMBEDDED_NAMES[3]),
            Pattern::RootAt92End => self.embedded_heredoc_end(p, EMBEDDED_NAMES[3]),
            Pattern::RootAt92At0 => self.embedded_heredoc(p, EMBEDDED_NAMES[3]),
            Pattern::RootAt92At0While => self.embedded_heredoc_goes_on(p, EMBEDDED_NAMES[3]),
            Pattern::RootAt93 => self.embedded_heredoc_ahead(p, EMBEDDED_NAMES[4]),
            Pattern::RootAt93End => self.embedded_heredoc_end(p, EMBEDDED_NAMES[4]),
            Pattern::RootAt93At0 => self.embedded_heredoc(p, EMBEDDED_NAMES[4]),
            Pattern::RootAt93At0While => self.embedded_heredoc_goes_on(p, EMBEDDED_NAMES[4]),
            Pattern::RootAt94 => self.embedded_heredoc_ahead(p, EMBEDDED_NAMES[5]),
            Pattern::RootAt94End => self.embedded_heredoc_end(p, EMBEDDED_NAMES[5]),
            Pattern::RootAt94At0 => self.embedded_heredoc(p, EMBEDDED_NAMES[5]),
            Pattern::RootAt94At0While => self.embedded_heredoc_goes_on(p, EMBEDDED_NAMES[5]),
            Pattern::RootAt95 => self.embedded_heredoc_ahead(p, EMBEDDED_NAMES[6]),
            Pattern::RootAt95End => self.embedded_heredoc_end(p, EMBEDDED_NAMES[6]),
            Pattern::RootAt95At0 => self.embedded_heredoc(p, EMBEDDED_NAMES[6]),
            Pattern::RootAt95At0While => self.embedded_heredoc_goes_on(p, EMBEDDED_NAMES[6]),
            Pattern::RootAt96 => self.embedded_heredoc_ahead(p, EMBEDDED_NAMES[7]),
            Pattern::RootAt96End => self.embedded_heredoc_end(p, EMBEDDED_NAMES[7]),
            Pattern::RootAt96At0 => self.embedded_heredoc(p, EMBEDDED_NAMES[7]),
            Pattern::RootAt96At0While => self.embedded_heredoc_goes_on(p, EMBEDDED_NAMES[7]),
            Pattern::RootAt97 => self.embedded_heredoc_ahead(p, EMBEDDED_NAMES[8]),
            Pattern::RootAt97End => self.embedded_heredoc_end(p, EMBEDDED_NAMES[8]),
            Pattern::RootAt97At0 => self.embedded_heredoc(p, EMBEDDED_NAMES[8]),
            Pattern::RootAt97At0While => self.embedded_heredoc_goes_on(p, EMBEDDED_NAMES[8]),
            Pattern::RootAt98 => self.embedded_heredoc_ahead(p, EMBEDDED_NAMES[9]),
            Pattern::RootAt98End => self.embedded_heredoc_end(p, EMBEDDED_NAMES[9]),
            Pattern::RootAt98At0 => self.embedded_heredoc(p, EMBEDDED_NAMES[9]),
            Pattern::RootAt98At0While => self.embedded_heredoc_goes_on(p, EMBEDDED_NAMES[9]),
            Pattern::RootAt99 => self.embedded_heredoc_ahead(p, EMBEDDED_NAMES[10]),
            Pattern::RootAt99End => self.embedded_heredoc_end(p, EMBEDDED_NAMES[10]),
            Pattern::RootAt99At0 => self.embedded_heredoc(p, EMBEDDED_NAMES[10]),
            Pattern::RootAt99At0While => self.embedded_heredoc_goes_on(p, EMBEDDED_NAMES[10]),
            Pattern::RootAt100 => self.embedded_heredoc_ahead(p, EMBEDDED_NAMES[11]),
            Pattern::RootAt100End => self.embedded_heredoc_end(p, EMBEDDED_NAMES[11]),
            Pattern::RootAt100At0 => self.embedded_heredoc(p, EMBEDDED_NAMES[11]),
            Pattern::RootAt100At0While => self.embedded_heredoc_goes_on(p, EMBEDDED_NAMES[11]),
            Pattern::RootAt101 => self.embedded_heredoc_ahead(p, EMBEDDED_NAMES[12]),
            Pattern::RootAt101End => self.embedded_heredoc_end(p, EMBEDDED_NAMES[12]),
            Pattern::RootAt101At0 => self.embedded_heredoc(p, EMBEDDED_NAMES[12]),
            Pattern::RootAt101At0While => self.embedded_heredoc_goes_on(p, EMBEDDED_NAMES[12]),
            Pattern::RootAt102 => self.embedded_heredoc_ahead(p, EMBEDDED_NAMES[13]),
            Pattern::RootAt102End => self.embedded_heredoc_end(p, EMBEDDED_NAMES[13]),
            Pattern::RootAt102At0 => self.embedded_heredoc(p, EMBEDDED_NAMES[13]),
            Pattern::RootAt102At0While => self.embedded_heredoc_goes_on(p, EMBEDDED_NAMES[13]),
            Pattern::RootAt103 => self.embedded_heredoc_ahead(p, EMBEDDED_NAMES[14]),
            Pattern::RootAt103End => self.embedded_heredoc_end(p, EMBEDDED_NAMES[14]),
            Pattern::RootAt103At0 => self.embedded_heredoc(p, EMBEDDED_NAMES[14]),
            Pattern::RootAt103At0While => self.embedded_heredoc_goes_on(p, EMBEDDED_NAMES[14]),
            Pattern::RootAt104 => self.assigned_heredoc(p),
            Pattern::RootAt104End => self.assigned_heredoc_end(p),
            Pattern::Heredoc => self.heredoc_at_line_start(p),
            Pattern::HeredocEnd => self.line_end(p),
            Pattern::RootAt105 => self.heredoc_openers(p),
            Pattern::RootAt105End => self.heredoc_openers_end(p),
            Pattern::RootAt106 => self.block_params_open(p),
            Pattern::RootAt106End => self.block_params_close(p),
            Pattern::RootAt106At0 => self.block_param_start(p),
            Pattern::RootAt106At0End => self.block_param_end(p),
            Pattern::RootAt106At0At0 => self.block_param_name(p),
            Pattern::RootAt107 => self.lit(p, b"=>"),
            Pattern::RootAt108 => self.lit(p, b"->"),
            Pattern::RootAt109 => self.assignment_operator(p),
            Pattern::RootAt110 => self.comparison(p),
            Pattern::RootAt111 => self.logical_word(p),
            Pattern::RootAt112 => self.logical_operator(p),
            Pattern::RootAt113 => self.safe_navigation(p),
            Pattern::RootAt114 => self.arithmetic(p),
            Pattern::RootAt115 => self.lit(p, b"="),
            Pattern::RootAt116 => self.bitwise(p),
            Pattern::RootAt117 => self.lit(p, b";"),
            Pattern::RootAt106At1 | Pattern::RootAt118 => self.lit(p, b","),
            Pattern::RootAt119 => self.namespace_separator(p),
            Pattern::RootAt120 => self.method_separator(p),
            Pattern::RootAt121 => self.lit(p, b":"),
            Pattern::RootAt126 => self.paren(p),
            Pattern::RootAt127 => self.method_call_start(p),
            Pattern::RootAt127End => self.method_call_end(p),
            Pattern::RootAt127At0 => self.method_call_name(p),
            Pattern::RootAt128 => self.call_with_parens(p),
            Pattern::RootAt128End => self.closing_paren(p),
            Pattern::Parens
            | Pattern::RegexSubAt5
            | Pattern::NestParensR
            | Pattern::NestParensI
            | Pattern::NestParens => self.lit(p, b"("),
            Pattern::ParensEnd
            | Pattern::RootAt30End
            | Pattern::RootAt31End
            | Pattern::RegexSubAt4End
            | Pattern::RegexSubAt5End
            | Pattern::NestParensREnd
            | Pattern::RootAt48End
            | Pattern::NestParensIEnd
            | Pattern::RootAt53End
            | Pattern::NestParensEnd
            | Pattern::RootAt58End
            | Pattern::RootAt63End
            | Pattern::RootAt67End
            | Pattern::RootAt73End
            | Pattern::RootAt78End => self.lit(p, b")"),
            Pattern::Braces
            | Pattern::NestCurlyAndSelfAt0
            | Pattern::NestCurlyR
            | Pattern::NestCurlyI
            | Pattern::NestCurly
            | Pattern::RootAt122 => self.lit(p, b"{"),
            Pattern::BracesEnd
            | Pattern::InterpolatedRubyAt0End
            | Pattern::NestCurlyAndSelfAt0End
            | Pattern::NestCurlyREnd
            | Pattern::RootAt50End
            | Pattern::NestCurlyIEnd
            | Pattern::RootAt55End
            | Pattern::NestCurlyEnd
            | Pattern::RootAt60End
            | Pattern::RootAt65End
            | Pattern::RootAt69End
            | Pattern::RootAt76End
            | Pattern::RootAt81End
            | Pattern::RootAt123 => self.lit(p, b"}"),
            Pattern::Brackets
            | Pattern::NestBracketsR
            | Pattern::NestBracketsI
            | Pattern::NestBrackets
            | Pattern::RootAt124 => self.lit(p, b"["),
            Pattern::BracketsEnd
            | Pattern::RootAt26End
            | Pattern::RegexSubAt3End
            | Pattern::NestBracketsREnd
            | Pattern::RootAt47End
            | Pattern::NestBracketsIEnd
            | Pattern::RootAt52End
            | Pattern::NestBracketsEnd
            | Pattern::RootAt57End
            | Pattern::RootAt62End
            | Pattern::RootAt68End
            | Pattern::RootAt75End
            | Pattern::RootAt80End
            | Pattern::RootAt125 => self.lit(p, b"]"),
            Pattern::NestLtgtR | Pattern::NestLtgtI | Pattern::NestLtgt => self.lit(p, b"<"),
            Pattern::NestLtgtREnd
            | Pattern::RootAt49End
            | Pattern::NestLtgtIEnd
            | Pattern::RootAt54End
            | Pattern::NestLtgtEnd
            | Pattern::RootAt59End
            | Pattern::RootAt64End
            | Pattern::RootAt70End
            | Pattern::RootAt74End
            | Pattern::RootAt79End => self.lit(p, b">"),
        }
    }

    /// `(\))`
    fn closing_paren(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, b")")?;
        self.capture(1, p, end);
        Some(end)
    }
}
