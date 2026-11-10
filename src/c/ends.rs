//! The end patterns of the grammar's rules.

use super::engine::Lexer;
use super::scope::S;
use super::state::Rule;
use crate::scope::{Scopes, scopes};

impl Lexer<'_> {
    /// An end that matches `start..end`, with `capture` on it.
    pub(super) fn end_at(&mut self, start: usize, end: usize, capture: Option<Scopes<S>>) -> bool {
        if let Some(scopes) = capture {
            self.span(start, end, scopes);
        }
        self.plain(start, end)
    }

    /// One byte `c` that ends the rule, with `capture` on it.
    #[expect(
        clippy::inline_always,
        reason = "measured at one codegen unit: called from both instances of the scan, the halting one and the plain one, it stopped being inlined, at a call per place"
    )]
    #[inline(always)]
    fn end_byte(&mut self, p: usize, c: u8, capture: Scopes<S>) -> bool {
        p < self.len_all() && self.b(p) == c && self.end_at(p, p + 1, Some(capture))
    }

    /// `(?<!\\)(?=\n)`.
    fn end_line(&mut self, p: usize) -> bool {
        self.nl && p == self.lim() && self.prev(p) != Some(b'\\') && self.end_at(p, p, None)
    }

    /// `(?<!\\)(?=\s*\n)`.
    fn end_blank(&mut self, p: usize) -> bool {
        self.nl
            && p <= self.lim()
            && self.prev(p) != Some(b'\\')
            && self.skip_space(p) > self.lim()
            && self.end_at(p, p, None)
    }

    /// `(?=/[*/])`.
    fn end_comment_ahead(&mut self, p: usize) -> bool {
        (self.starts(p, b"//") || self.starts(p, b"/*")) && self.end_at(p, p, None)
    }

    /// `(?=\s*#\s*e(?:lif|lse|ndif)\b)`: a conditional directive ahead,
    /// possibly after white space.
    fn end_else_ahead(&mut self, p: usize) -> bool {
        if self.b(p) != b'#' && self.space_at(p) == 0 {
            return false;
        }
        let q = self.skip_space(p);
        if self.b(q) != b'#' || q >= self.len_all() {
            return false;
        }
        let k = self.skip_space(q + 1);
        let hit = ["elif", "else", "endif"].iter().any(|keyword| {
            self.starts(k, keyword.as_bytes()) && self.word_at(k + keyword.len()) == 0
        });
        hit && self.end_at(p, p, None)
    }

    /// `(?=^\s*((#)\s*KEYWORD)\b)`: a directive at the start of the line.
    fn end_look(&mut self, p: usize, keywords: &[&str]) -> bool {
        self.directive(p, keywords).is_some() && self.end_at(p, p, None)
    }

    /// `^\s*((#)\s*endif)\b`, with the directive's captures.
    fn end_endif(&mut self, p: usize) -> bool {
        let Some(directive) = self.directive(p, &["endif"]) else {
            return false;
        };
        self.span(0, directive.keyword.1, scopes![S::MetaPreprocessor]);
        self.span(
            directive.hash,
            directive.keyword.1,
            scopes![S::DirectiveConditional],
        );
        self.span(
            directive.hash,
            directive.hash + 1,
            scopes![S::PunctDirective],
        );
        self.end_at(0, directive.keyword.1, None)
    }

    /// `}|(?=\s*#\s*e(?:lif|lse|ndif)\b)` and the macro variants.
    fn end_brace(&mut self, p: usize, in_define: bool) -> bool {
        if self.end_byte(p, b'}', scopes![S::PunctBlockEnd]) || self.end_else_ahead(p) {
            return true;
        }
        in_define && self.end_blank(p)
    }

    /// `\)` or the end of a macro's line.
    #[expect(
        clippy::inline_always,
        reason = "measured at one codegen unit: called from both instances of the scan, the halting one and the plain one, it stopped being inlined, at a call per place"
    )]
    #[inline(always)]
    fn end_paren(&mut self, p: usize, capture: Scopes<S>, in_define: bool) -> bool {
        self.end_byte(p, b')', capture) || (in_define && self.end_blank(p))
    }

    /// `[!*]*\*/`: the end of a documentation comment.
    fn end_doc_block(&mut self, p: usize) -> bool {
        let mut r = p;
        while matches!(self.b(r), b'!' | b'*') && r < self.len_all() {
            r += 1;
        }
        self.spend(r - p);
        r > p
            && self.b(r) == b'/'
            && r < self.len_all()
            && self.b(r - 1) == b'*'
            && self.end_at(p, r + 1, Some(scopes![S::PunctCommentEndDoc]))
    }

    /// `(?<=\n)(?<!\\\n)`: after the line feed, unless a backslash ends the
    /// line.
    fn end_after_line(&mut self, p: usize) -> bool {
        self.nl
            && p == self.lim() + 1
            && !(self.lim() > 0 && self.b(self.lim() - 1) == b'\\')
            && self.end_at(p, p, None)
    }

    /// `(?<=\))` with `(?!\w)` when `no_word` is set.
    fn end_after_paren(&mut self, p: usize, no_word: bool) -> bool {
        self.prev(p) == Some(b')') && (!no_word || self.word_at(p) == 0) && self.end_at(p, p, None)
    }

    /// The end of the rule `r` at `p`, if it ends there.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm per rule: the table of end patterns"
    )]
    #[expect(
        clippy::inline_always,
        reason = "the search tries it at every position, and a call there costs C 3% of its instructions"
    )]
    #[inline(always)]
    pub(super) fn try_end(&mut self, r: Rule, p: usize) -> bool {
        match r {
            Rule::Top
            | Rule::Doxygen
            | Rule::Inline
            | Rule::InlineClose
            | Rule::Access
            | Rule::DefineParameters
            | Rule::ParameterDirections
            | Rule::NumberSeparator
            | Rule::Number => false,
            Rule::Block | Rule::BlockInner => self.end_brace(p, false),
            Rule::Parens | Rule::ParensBlock | Rule::FunctionParens | Rule::CallParens => {
                self.end_paren(p, scopes![S::PunctParensEnd], false)
            }
            Rule::FunctionParameters => self.end_paren(p, scopes![S::PunctParamEnd], false),
            Rule::CallArguments => self.end_paren(p, scopes![S::PunctArgsEnd], false),
            Rule::Method => self.end_paren(p, scopes![S::PunctArgsEndMember], false),
            Rule::Initialization => self.end_paren(p, scopes![S::PunctParensEndInit], false),
            Rule::Bracket => self.end_byte(p, b']', scopes![S::PunctEndSquare]),
            Rule::Ternary | Rule::ConditionalTernary => {
                self.end_byte(p, b':', scopes![S::OpTernary])
            }
            Rule::FunctionDefinition => !self.at_g(p) && self.end_after_paren(p, false),
            Rule::FunctionCall => self.end_after_paren(p, true),
            Rule::DocLine | Rule::LineComment => self.end_after_line(p),
            Rule::DocBlock => self.end_doc_block(p),
            Rule::BlockComment => {
                self.starts(p, b"*/") && self.end_at(p, p + 2, Some(scopes![S::PunctCommentEnd]))
            }
            Rule::LineLeading | Rule::Asm => !self.at_g(p) && self.end_at(p, p, None),
            Rule::LineSlash => self.nl && p == self.lim() && self.end_at(p, p, None),
            Rule::DoubleQuote | Rule::IncludeDoubleQuote => {
                self.end_byte(p, b'"', scopes![S::PunctStringEnd])
            }
            Rule::SingleQuote => self.end_byte(p, b'\'', scopes![S::PunctStringEnd]),
            Rule::IncludeAngle => self.end_byte(p, b'>', scopes![S::PunctStringEnd]),
            Rule::DefineDoubleQuote | Rule::DiagnosticDoubleQuote => {
                self.end_byte(p, b'"', scopes![S::PunctStringEnd]) || self.end_blank(p)
            }
            Rule::DefineSingleQuote | Rule::DiagnosticSingleQuote => {
                self.end_byte(p, b'\'', scopes![S::PunctStringEnd]) || self.end_blank(p)
            }
            Rule::DiagnosticText => self.end_blank(p),
            Rule::AsmString => self.end_byte(p, b'"', scopes![S::PunctStringEndAsm]),
            Rule::Define | Rule::Diagnostic => self.end_line(p),
            Rule::DefineBlock | Rule::DefineBlockInner => self.end_brace(p, true),
            Rule::DefineFunction => self.end_after_paren(p, true) || self.end_blank(p),
            Rule::DefineCall => self.end_paren(p, scopes![S::PunctArgsEnd], true),
            Rule::DefineParens => self.end_paren(p, scopes![S::PunctParensEnd], true),
            Rule::Include | Rule::LineDirective | Rule::Undef | Rule::PragmaDirective => {
                self.end_comment_ahead(p) || self.end_line(p)
            }
            Rule::ConditionalIf
            | Rule::ConditionalIfBlock
            | Rule::EnabledIf
            | Rule::EnabledIfBlock
            | Rule::DisabledIf
            | Rule::DisabledIfBlock => self.end_endif(p),
            Rule::HeadContinuation | Rule::ConditionalElif => self.head_end(p, true),
            Rule::HeadPlain => self.head_end(p, false),
            Rule::ConditionalParens => {
                self.end_byte(p, b')', scopes![S::PunctParensEnd]) || self.head_end(p, true)
            }
            Rule::LiveElif
            | Rule::LiveElifBlock
            | Rule::ElifTail
            | Rule::ElifTailBlock
            | Rule::ElifElse
            | Rule::ElifElseBlock
            | Rule::LiveElse
            | Rule::LiveElseBlock
            | Rule::ElseBranch
            | Rule::ElseBranchBlock => self.end_look(p, &["endif"]),
            Rule::ElifElif
            | Rule::DisabledElif
            | Rule::DisabledElifBody
            | Rule::DisabledElifOther
            | Rule::DisabledElifOtherBlock
            | Rule::DisabledBody
            | Rule::DisabledBodyBlock
            | Rule::ElifBranch
            | Rule::ElifBranchBlock
            | Rule::EnabledBody
            | Rule::EnabledBodyBlock => self.end_look(p, &["else", "elif", "endif"]),
            Rule::Nest => self
                .directive(p, &["endif"])
                .is_some_and(|directive| self.end_at(0, directive.keyword.1, None)),
            Rule::Switch => {
                (self.after_close(p)
                    || (matches!(self.b(p), b']' | b';' | b'=' | b'>' | b'[')
                        && p < self.len_all()))
                    && self.end_at(p, p, None)
            }
            Rule::SwitchHead => {
                if self.b(p) == b'{' && p < self.len_all() {
                    return self.end_at(p, p + 1, Some(scopes![S::PunctBlockBeginSwitch]));
                }
                if self.starts(p, b"<%") {
                    return self.end_at(p, p + 2, Some(scopes![S::PunctBlockBeginSwitch]));
                }
                if self.starts(p, b"??<") {
                    return self.end_at(p, p + 3, Some(scopes![S::PunctBlockBeginSwitch]));
                }
                self.b(p) == b';' && p < self.len_all() && self.end_at(p, p, None)
            }
            Rule::SwitchBody => {
                for lit in [&b"}"[..], b"%>", b"??>"] {
                    if self.starts(p, lit) {
                        return self.end_at(
                            p,
                            p + lit.len(),
                            Some(scopes![S::PunctBlockEndSwitch]),
                        );
                    }
                }
                false
            }
            Rule::SwitchTail => {
                let end = self.skip_space(p);
                self.b(end) == b';' && end < self.len_all() && self.end_at(p, end, None)
            }
            Rule::SwitchParens => self.end_byte(p, b')', scopes![S::PunctParensEndSwitch]),
            Rule::Case => self.end_byte(p, b':', scopes![S::PunctColonCase]),
            Rule::Default => self.end_byte(p, b':', scopes![S::PunctColonDefault]),
            Rule::AsmParens => self.end_byte(p, b')', scopes![S::PunctParensEndAsm]),
            Rule::AsmInner => self.end_byte(p, b')', scopes![S::PunctParensEndAsmInner]),
        }
    }
}
