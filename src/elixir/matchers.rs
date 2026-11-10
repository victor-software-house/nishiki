//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups of its match with [`Lexer::capture`].
//! The engine names those the grammar names.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would.
//! Where a quantifier could give characters back, the doc comment shows why it
//! cannot matter, or how the matcher gives them back.
//!
//! `^` holds only at the start of the line: Shiki's engine does not match it
//! after the line feed that ends the line.
//! `.` matches any character but the line feed, and a negated class matches
//! the line feed too.
//! `\w`, `\s` and `\d` read Unicode, through [`crate::classes`].

mod heads;
mod literals;
mod symbols;

use super::engine::Lexer;
use super::grammar::{GUARDS, KEYWORDS, Pattern, WORD_OPERATORS};
use crate::rules::Chars;

impl Lexer<'_> {
    /// Whether `pattern` matches at `p`, and where its match ends.
    ///
    /// The arms come in groups, in this order:
    /// - anonymous functions and clauses, modules and `def` heads;
    /// - strings, sigils, documentation and interpolation;
    /// - keywords, names, numbers and atoms;
    /// - comments, operators and punctuation.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm for each expression of the grammar"
    )]
    pub(super) fn matcher(&mut self, pattern: Pattern, p: usize) -> Option<usize> {
        match pattern {
            Pattern::RootAt0 => self.fn_keyword(p, false),
            Pattern::RootAt5 => self.fn_keyword(p, true),
            Pattern::RootAt0End => self.at_eol(p).then_some(p),
            Pattern::RootAt5End | Pattern::RootAt7End => self.clause_end(p),
            Pattern::RootAt7 => (p == 0 && self.clause_head()).then_some(p),
            Pattern::RootAt1 => self.module_call(p),
            Pattern::RootAt2 => self.atom_call(p),
            Pattern::RootAt3 => self.pipe_call(p),
            Pattern::RootAt4 => self.call_name(p),
            Pattern::CoreSyntaxAt0 => self.module_head(p, b"defmodule"),
            Pattern::CoreSyntaxAt1 => self.module_head(p, b"defprotocol"),
            Pattern::CoreSyntaxAt2 => self.module_head(p, b"defimpl"),
            Pattern::CoreSyntaxAt0End | Pattern::CoreSyntaxAt1End | Pattern::CoreSyntaxAt2End => {
                self.do_keyword(p)
            }
            Pattern::CoreSyntaxAt0At0 => {
                let end = self.module_name(p)?;
                (self.at(end) == b'.' && end < self.lim()).then_some(end)
            }
            Pattern::CoreSyntaxAt0At1
            | Pattern::CoreSyntaxAt1At0
            | Pattern::CoreSyntaxAt2At0
            | Pattern::CoreSyntaxAt15 => self.module_name(p),
            Pattern::CoreSyntaxAt3 => {
                self.def_head(p, &[b"", b"macro", b"delegate", b"guard"], false)
            }
            Pattern::CoreSyntaxAt4 => self.def_head(p, &[b"", b"n", b"macro", b"guard"], true),
            Pattern::CoreSyntaxAt3End => {
                self.def_end(p, &[b"", b"n", b"macro", b"delegate", b"guard"])
            }
            Pattern::CoreSyntaxAt4End => self.def_end(p, &[b"p", b"macrop", b"guardp"]),
            Pattern::CoreSyntaxAt3At1 | Pattern::CoreSyntaxAt4At1 => self.default_argument(p),
            Pattern::CoreSyntaxAt3At1End | Pattern::CoreSyntaxAt4At1End => {
                if p < self.lim() && matches!(self.at(p), b')' | b',') {
                    Some(p + 1)
                } else {
                    self.at_eol(p).then_some(p)
                }
            }
            Pattern::CoreSyntaxAt3At2 | Pattern::CoreSyntaxAt4At2 => self.word_in(p, GUARDS),

            Pattern::CoreSyntaxAt5 => self.html_sigil(p, b"~L\"\"\""),
            Pattern::CoreSyntaxAt6 => self.html_sigil(p, b"~H\"\"\""),
            Pattern::CoreSyntaxAt5End
            | Pattern::CoreSyntaxAt6End
            | Pattern::CoreSyntaxAt7End
            | Pattern::CoreSyntaxAt8End => self.spaced_close(p, b"\"\"\""),
            Pattern::CoreSyntaxAt9End | Pattern::CoreSyntaxAt10End => self.spaced_close(p, b"'''"),
            Pattern::CoreSyntaxAt7 => self.doc_heredoc(p, b"\"\"\"", false),
            Pattern::CoreSyntaxAt8 => self.doc_heredoc(p, b"\"\"\"", true),
            Pattern::CoreSyntaxAt9 => self.doc_heredoc(p, b"'''", false),
            Pattern::CoreSyntaxAt10 => self.doc_heredoc(p, b"'''", true),
            Pattern::CoreSyntaxAt11 => {
                let end = self.doc_attribute(p)?;
                self.lit(end, b" false")
            }
            Pattern::CoreSyntaxAt12 => {
                let end = self.doc_attribute(p)?;
                self.lit(end, b" \"")
            }
            Pattern::InterpolatedElixir => self.lit(p, b"#{"),
            Pattern::NestCurlyAndSelfAt0 => self.lit(p, b"{"),
            Pattern::InterpolatedElixirEnd | Pattern::NestCurlyAndSelfAt0End => self.lit(p, b"}"),
            Pattern::EscapedChar => self.escape(p),
            Pattern::CoreSyntaxAt27 => self.lit(p, b":'"),
            Pattern::CoreSyntaxAt28 => self.lit(p, b":\""),
            Pattern::CoreSyntaxAt29 => self.lit(p, b"'''"),
            Pattern::CoreSyntaxAt30 | Pattern::CoreSyntaxAt27End | Pattern::CoreSyntaxAt30End => {
                self.lit(p, b"'")
            }
            Pattern::CoreSyntaxAt31 => self.lit(p, b"\"\"\""),
            Pattern::CoreSyntaxAt32
            | Pattern::CoreSyntaxAt12End
            | Pattern::CoreSyntaxAt28End
            | Pattern::CoreSyntaxAt32End => self.lit(p, b"\""),
            Pattern::CoreSyntaxAt29End => self.line_close(p, b"'''"),
            Pattern::CoreSyntaxAt31End
            | Pattern::CoreSyntaxAt33End
            | Pattern::CoreSyntaxAt39End => self.line_close(p, b"\"\"\""),
            Pattern::CoreSyntaxAt33 => self.sigil_open(p, false, b"\"\"\""),
            Pattern::CoreSyntaxAt34 => self.sigil_open(p, false, b"{"),
            Pattern::CoreSyntaxAt35 => self.sigil_open(p, false, b"["),
            Pattern::CoreSyntaxAt36 => self.sigil_open(p, false, b"<"),
            Pattern::CoreSyntaxAt37 => self.sigil_open(p, false, b"("),
            Pattern::CoreSyntaxAt38 => self.sigil_any(p, false),
            Pattern::CoreSyntaxAt39 => self.sigil_open(p, true, b"\"\"\""),
            Pattern::CoreSyntaxAt40 => self.sigil_open(p, true, b"{"),
            Pattern::CoreSyntaxAt41 => self.sigil_open(p, true, b"["),
            Pattern::CoreSyntaxAt42 => self.sigil_open(p, true, b"<"),
            Pattern::CoreSyntaxAt43 => self.sigil_open(p, true, b"("),
            Pattern::CoreSyntaxAt44 => self.sigil_any(p, true),
            Pattern::CoreSyntaxAt34End | Pattern::CoreSyntaxAt40End => self.sigil_close(p, b'}'),
            Pattern::CoreSyntaxAt35End | Pattern::CoreSyntaxAt41End => self.sigil_close(p, b']'),
            Pattern::CoreSyntaxAt36End | Pattern::CoreSyntaxAt42End => self.sigil_close(p, b'>'),
            Pattern::CoreSyntaxAt37End | Pattern::CoreSyntaxAt43End => self.sigil_close(p, b')'),
            Pattern::CoreSyntaxAt38End | Pattern::CoreSyntaxAt44End => self.sigil_any_close(p),

            Pattern::CoreSyntaxAt13 => {
                let end = self.after_dot_free(p, KEYWORDS)?;
                (!matches!(self.at(end), b'!' | b':' | b'?')).then_some(end)
            }
            Pattern::CoreSyntaxAt14 => self.after_dot_free(p, WORD_OPERATORS),
            Pattern::CoreSyntaxAt16 => {
                let end = self.word_in(p, &[b"nil", b"true", b"false"])?;
                self.not_bang(end)
            }
            Pattern::CoreSyntaxAt17 => {
                let end = self.word_in(
                    p,
                    &[
                        b"__CALLER__",
                        b"__ENV__",
                        b"__MODULE__",
                        b"__DIR__",
                        b"__STACKTRACE__",
                    ],
                )?;
                self.not_bang(end)
            }
            Pattern::CoreSyntaxAt18 => {
                let end = self.name(self.lit(p, b"@")?)?;
                self.capture(1, p, p + 1);
                Some(end)
            }
            Pattern::CoreSyntaxAt19 => self.capture_number(p),
            Pattern::CoreSyntaxAt20 => {
                (self.starts(p, b"&") && self.at(p + 1) != b'&').then_some(p + 1)
            }
            Pattern::CoreSyntaxAt21 => {
                let end = self.lit(p, b"^")?;
                self.function_start(end).then(|| self.word_end(end + 1))
            }
            Pattern::CoreSyntaxAt22 => self.radix(p, b'x', |b| b.is_ascii_hexdigit()),
            Pattern::CoreSyntaxAt23 => self.float(p),
            Pattern::CoreSyntaxAt24 => self.integer(p),
            Pattern::CoreSyntaxAt25 => self.radix(p, b'b', |b| matches!(b, b'0' | b'1')),
            Pattern::CoreSyntaxAt26 => self.radix(p, b'o', |b| matches!(b, b'0'..=b'7')),
            Pattern::CoreSyntaxAt45 => self.atom(p),
            Pattern::CoreSyntaxAt46 => self.keyword_key(p),
            Pattern::CoreSyntaxAt49 => self.underscored(p),
            Pattern::CoreSyntaxAt50 => {
                (self.at(p) == b'_' && p < self.lim() && self.boundary(p) && self.boundary(p + 1))
                    .then_some(p + 1)
            }
            Pattern::CoreSyntaxAt51 => self.char_literal(p),

            Pattern::CoreSyntaxAt47 => self.comment_start(p, b"##"),
            Pattern::CoreSyntaxAt48 => self.comment_start(p, b"#"),
            Pattern::CoreSyntaxAt47End | Pattern::CoreSyntaxAt48End => {
                (self.at(p) != b'#').then_some(p)
            }
            Pattern::CoreSyntaxAt47At0 => self.lit(p, b"##"),
            Pattern::CoreSyntaxAt48At0 => self.lit(p, b"#"),
            Pattern::CoreSyntaxAt47At0End | Pattern::CoreSyntaxAt48At0End => {
                (p == self.lim() && self.at(p) == b'\n').then_some(p + 1)
            }
            Pattern::CoreSyntaxAt52 => self.first_lit(p, &[b"++", b"--", b"<|>"]),
            Pattern::CoreSyntaxAt53 => self.first_lit(
                p,
                &[
                    b"|>", b"<~>", b"<>", b"<<<", b">>>", b"~>>", b"<<~", b"~>", b"<~", b"<|>",
                ],
            ),
            Pattern::CoreSyntaxAt54 => self.first_lit(p, &[b"&&&", b"&&"]),
            Pattern::CoreSyntaxAt55 => self.first_lit(p, &[b"<-", b"\\\\"]),
            Pattern::CoreSyntaxAt56 => {
                self.first_lit(p, &[b"===", b"==", b"!==", b"!=", b"<=", b"<", b">=", b">"])
            }
            Pattern::CoreSyntaxAt57 => {
                self.first_lit(p, &[b"|||", b"&&&", b"^^^", b"<<<", b">>>", b"~~~"])
            }
            Pattern::CoreSyntaxAt58 => self.logical(p),
            Pattern::CoreSyntaxAt59 => self.one_of(p, b"-*+/"),
            Pattern::CoreSyntaxAt60 => self.first_lit(
                p,
                &[
                    b"|", b"++", b"--", b"**", b"\\\\", b"<-", b"<>", b"<<", b">>", b"::", b"..",
                    b"//", b"|>", b"~", b"=>", b"&",
                ],
            ),
            Pattern::CoreSyntaxAt61 => self.lit(p, b"="),
            Pattern::CoreSyntaxAt62 => self.lit(p, b":"),
            Pattern::CoreSyntaxAt63 => self.lit(p, b";"),
            Pattern::CoreSyntaxAt64 => self.lit(p, b","),
            Pattern::CoreSyntaxAt65 => self.lit(p, b"."),
            Pattern::CoreSyntaxAt66 => self.one_of(p, b"{}"),
            Pattern::CoreSyntaxAt67 => self.one_of(p, b"[]"),
            Pattern::CoreSyntaxAt68 => self.one_of(p, b"()"),
        }
    }

    /// The end of the first of `options` at `p`, in their order: an
    /// alternation of literals, which takes the first that matches.
    fn first_lit(&self, p: usize, options: &[&[u8]]) -> Option<usize> {
        options.iter().find_map(|&option| self.lit(p, option))
    }

    /// `[…]` for the bytes `set`: one of them at `p`.
    fn one_of(&self, p: usize, set: &[u8]) -> Option<usize> {
        (p < self.lim() && set.contains(&self.at(p))).then_some(p + 1)
    }

    /// `(?![!?])` at `end`, which a match ending there needs.
    fn not_bang(&self, end: usize) -> Option<usize> {
        (!matches!(self.at(end), b'!' | b'?')).then_some(end)
    }

    /// `(?<!\.)\b(words)\b`: a word of `words` not right after a `.`.
    fn after_dot_free(&self, p: usize, words: &[&[u8]]) -> Option<usize> {
        if self.prev(p) == Some(b'.') {
            return None;
        }
        self.word_in(p, words)
    }

    /// `[A-Z_a-z]\w*`: the end of a name at `p`.
    fn name(&self, p: usize) -> Option<usize> {
        let b = self.at(p);
        (p < self.lim() && (b == b'_' || b.is_ascii_alphabetic())).then(|| self.word_end(p + 1))
    }

    /// `[_a-z]` at `p`.
    fn function_start(&self, p: usize) -> bool {
        p < self.lim() && matches!(self.at(p), b'_' | b'a'..=b'z')
    }

    /// `[_a-z]\w*[!?]?`: the end of a function name at `p`.
    fn function_name(&self, p: usize) -> Option<usize> {
        self.function_start(p)
            .then(|| self.bang_end(self.word_end(p + 1)))
    }

    /// `[!?]?` at `end`.
    fn bang_end(&self, end: usize) -> usize {
        if end < self.lim() && matches!(self.at(end), b'!' | b'?') {
            end + 1
        } else {
            end
        }
    }

    /// `\b[A-Z]\w*`, which `\b` may follow: a run of `\w` always ends at one.
    fn module_name(&self, p: usize) -> Option<usize> {
        (p < self.lim() && self.at(p).is_ascii_uppercase() && !self.word_before(p))
            .then(|| self.word_end(p + 1))
    }
}
