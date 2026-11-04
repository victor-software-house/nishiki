//! The end patterns of the grammar's rules.

use super::engine::Lexer;
use super::scope::S;
use super::state::R;
use crate::scope::{Scopes, scopes};

impl Lexer<'_> {
    /// An end that matches `start..end`, with `cap` on it.
    pub(super) fn end_at(&mut self, start: usize, end: usize, cap: Option<Scopes<S>>) -> bool {
        if let Some(scopes) = cap {
            self.span(start, end, scopes);
        }
        self.plain(start, end)
    }

    /// One byte `c` that ends the rule, with `cap` on it.
    #[expect(
        clippy::inline_always,
        reason = "measured at one codegen unit: called from both instances of the scan, the halting one and the plain one, it stopped being inlined, at a call per place"
    )]
    #[inline(always)]
    fn end_byte(&mut self, p: usize, c: u8, cap: Scopes<S>) -> bool {
        p < self.len_all() && self.b(p) == c && self.end_at(p, p + 1, Some(cap))
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
        let hit = ["elif", "else", "endif"]
            .iter()
            .any(|kw| self.starts(k, kw.as_bytes()) && self.word_at(k + kw.len()) == 0);
        hit && self.end_at(p, p, None)
    }

    /// `(?=^\s*((#)\s*KEYWORD)\b)`: a directive at the start of the line.
    fn end_look(&mut self, p: usize, kws: &[&str]) -> bool {
        self.directive(p, kws).is_some() && self.end_at(p, p, None)
    }

    /// `^\s*((#)\s*endif)\b`, with the directive's captures.
    fn end_endif(&mut self, p: usize) -> bool {
        let Some(dir) = self.directive(p, &["endif"]) else {
            return false;
        };
        self.span(0, dir.kw.1, scopes![S::MetaPreprocessor]);
        self.span(dir.hash, dir.kw.1, scopes![S::DirectiveConditional]);
        self.span(dir.hash, dir.hash + 1, scopes![S::PunctDirective]);
        self.end_at(0, dir.kw.1, None)
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
    fn end_paren(&mut self, p: usize, cap: Scopes<S>, in_define: bool) -> bool {
        self.end_byte(p, b')', cap) || (in_define && self.end_blank(p))
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
    pub(super) fn try_end(&mut self, r: R, p: usize) -> bool {
        match r {
            R::Top
            | R::Doxy
            | R::Inline
            | R::InlineClose
            | R::Access
            | R::DefineParams
            | R::ParamDir
            | R::NumSep
            | R::Number => false,
            R::Block | R::BlockInner => self.end_brace(p, false),
            R::Parens | R::ParensBlock | R::FuncParens | R::CallParens => {
                self.end_paren(p, scopes![S::PunctParensEnd], false)
            }
            R::FuncParams => self.end_paren(p, scopes![S::PunctParamEnd], false),
            R::CallArgs => self.end_paren(p, scopes![S::PunctArgsEnd], false),
            R::Method => self.end_paren(p, scopes![S::PunctArgsEndMember], false),
            R::Init => self.end_paren(p, scopes![S::PunctParensEndInit], false),
            R::Bracket => self.end_byte(p, b']', scopes![S::PunctEndSquare]),
            R::Ternary | R::CondTernary => self.end_byte(p, b':', scopes![S::OpTernary]),
            R::FuncDef => !self.at_g(p) && self.end_after_paren(p, false),
            R::FuncCall => self.end_after_paren(p, true),
            R::DocLine | R::LineComment => self.end_after_line(p),
            R::DocBlock => self.end_doc_block(p),
            R::BlockComment => {
                self.starts(p, b"*/") && self.end_at(p, p + 2, Some(scopes![S::PunctCommentEnd]))
            }
            R::LineLeading | R::Asm => !self.at_g(p) && self.end_at(p, p, None),
            R::LineSlash => self.nl && p == self.lim() && self.end_at(p, p, None),
            R::DQuote | R::IncludeDQ => self.end_byte(p, b'"', scopes![S::PunctStringEnd]),
            R::SQuote => self.end_byte(p, b'\'', scopes![S::PunctStringEnd]),
            R::IncludeLt => self.end_byte(p, b'>', scopes![S::PunctStringEnd]),
            R::DefineDQuote | R::DiagDQuote => {
                self.end_byte(p, b'"', scopes![S::PunctStringEnd]) || self.end_blank(p)
            }
            R::DefineSQuote | R::DiagSQuote => {
                self.end_byte(p, b'\'', scopes![S::PunctStringEnd]) || self.end_blank(p)
            }
            R::DiagText => self.end_blank(p),
            R::AsmString => self.end_byte(p, b'"', scopes![S::PunctStringEndAsm]),
            R::Define | R::Diag => self.end_line(p),
            R::DefineBlock | R::DefineBlockInner => self.end_brace(p, true),
            R::DefineFunc => self.end_after_paren(p, true) || self.end_blank(p),
            R::DefineCall => self.end_paren(p, scopes![S::PunctArgsEnd], true),
            R::DefineParens => self.end_paren(p, scopes![S::PunctParensEnd], true),
            R::Include | R::LineDir | R::Undef | R::PragmaDir => {
                self.end_comment_ahead(p) || self.end_line(p)
            }
            R::CondIf
            | R::CondIfB
            | R::EnabledIf
            | R::EnabledIfB
            | R::DisabledIf
            | R::DisabledIfB => self.end_endif(p),
            R::HeadCont | R::CondElif => self.head_end(p, true),
            R::HeadPlain => self.head_end(p, false),
            R::CondParens => {
                self.end_byte(p, b')', scopes![S::PunctParensEnd]) || self.head_end(p, true)
            }
            R::LiveElif
            | R::LiveElifB
            | R::ElifTail
            | R::ElifTailB
            | R::ElifElse
            | R::ElifElseB
            | R::LiveElse
            | R::LiveElseB
            | R::ElseBranch
            | R::ElseBranchB => self.end_look(p, &["endif"]),
            R::ElifElif
            | R::DisabledElif
            | R::DisabledElifBody
            | R::DisabledElifOther
            | R::DisabledElifOtherB
            | R::DisabledBody
            | R::DisabledBodyB
            | R::ElifBranch
            | R::ElifBranchB
            | R::EnabledBody
            | R::EnabledBodyB => self.end_look(p, &["else", "elif", "endif"]),
            R::Nest => self
                .directive(p, &["endif"])
                .is_some_and(|dir| self.end_at(0, dir.kw.1, None)),
            R::Switch => {
                (self.after_close(p)
                    || (matches!(self.b(p), b']' | b';' | b'=' | b'>' | b'[')
                        && p < self.len_all()))
                    && self.end_at(p, p, None)
            }
            R::SwitchHead => {
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
            R::SwitchBody => {
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
            R::SwitchTail => {
                let end = self.skip_space(p);
                self.b(end) == b';' && end < self.len_all() && self.end_at(p, end, None)
            }
            R::SwitchParens => self.end_byte(p, b')', scopes![S::PunctParensEndSwitch]),
            R::Case => self.end_byte(p, b':', scopes![S::PunctColonCase]),
            R::Default => self.end_byte(p, b':', scopes![S::PunctColonDefault]),
            R::AsmParens => self.end_byte(p, b')', scopes![S::PunctParensEndAsm]),
            R::AsmInner => self.end_byte(p, b')', scopes![S::PunctParensEndAsmInner]),
        }
    }
}
