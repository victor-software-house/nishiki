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
//! `(?i)` compares words through [`crate::case_fold`], so `ſ`, the Kelvin sign,
//! `ß` and the Latin ligatures match the letters they fold to.
//!
//! A pattern whose id ends in `InFor` is a copy that the generator made for
//! the rules open inside a `for` line.
//! It matches as the pattern it copies.
//! An end whose id ends in `EndInFor` also fails where the injected
//! `#repeatParameter` matches, since the injection wins a tie with the end.

mod commands;
mod set;
mod variables;

use super::engine::{Lexer, Marks};
use super::grammar::Pat;
use crate::case_fold;
use crate::rules::Chars;

impl Lexer<'_> {
    /// Whether `pat` matches at `p`, and where its match ends.
    ///
    /// The arms come in groups, in this order:
    /// - commands, `echo`, `setlocal` and `set`;
    /// - comments;
    /// - control words and labels;
    /// - escapes and variables;
    /// - the inside of `set`;
    /// - strings, groups, numbers and operators;
    /// - the ends of the copies inside a `for` line.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm for each expression of the grammar"
    )]
    pub(super) fn matcher(&mut self, pat: Pat, p: usize) -> Option<usize> {
        match pat {
            Pat::CommandsAt0 => self.command(p),
            Pat::CommandsAt1 | Pat::CommandsAt1InFor => self.echo(p),
            Pat::CommandsAt2 => self.setlocal(p),
            Pat::CommandSetAt0 | Pat::CommandSetAt0InFor => self.set_command(p),
            Pat::CommandsAt1End
            | Pat::CommandSetAt0End
            | Pat::CommentsAt1End
            | Pat::CommandSetInsideAt6End
            | Pat::CommandSetInsideAt7End
            | Pat::CommandSetInsideAt8End
            | Pat::CommandSetInsideAt8At1End => self.command_end(p),

            Pat::CommentsAt0 | Pat::CommentsAt0InFor => self.colon_comment(p),
            Pat::CommentsAt0End | Pat::CommentsAt2End => self.line_feed_or(p, None),
            Pat::CommentsAt0At0 | Pat::CommentsAt0At0InFor => self.colon_mark(p),
            Pat::CommentsAt0At0End => (self.at(p) == b'\n').then_some(p),
            Pat::CommentsAt1 | Pat::CommentsAt1InFor => self.rem_dot(p),
            Pat::CommentsAt2 | Pat::CommentsAt2InFor => self.rem(p),
            Pat::CommentsAt2At0 => matches!(self.at(p), b'<' | b'>' | b'|').then_some(p + 1),

            Pat::ConstantsAt0 => self.nul(p),
            Pat::ControlsAt0 => self.call_exit_goto(p),
            Pat::ControlsAt1 => self.if_test(p),
            Pat::ControlsAt2 => self.if_else(p),
            Pat::ControlsAt3 => self.for_word(p),
            Pat::ControlsAt3At0InFor => self.in_word(p),
            Pat::LabelsAt0 => self.label(p),

            Pat::EscapedCharactersAt0 => self.escape(p),
            Pat::VariablesAt0 => self.parameter(p),
            Pat::RepeatParameterAt0 => {
                if self.x.embedded {
                    None
                } else {
                    self.repeat_parameter(p)
                }
            }
            Pat::VariableAt0 | Pat::VariableAt0InFor => self.expansion(p, b'%'),
            Pat::VariableDelayedExpansionAt0 | Pat::VariableDelayedExpansionAt0InFor => {
                self.expansion(p, b'!')
            }
            Pat::VariableAt0End => self.line_feed_or(p, Some(b'%')),
            Pat::VariableDelayedExpansionAt0End => self.line_feed_or(p, Some(b'!')),
            Pat::VariableAt0At0
            | Pat::VariableAt0At0InFor
            | Pat::VariableDelayedExpansionAt0At0
            | Pat::VariableDelayedExpansionAt0At0InFor => self.lit(p, b":~"),
            Pat::VariableAt0At1
            | Pat::VariableAt0At1InFor
            | Pat::VariableDelayedExpansionAt0At1
            | Pat::VariableDelayedExpansionAt0At1InFor => self.lit_byte(p, b':'),
            Pat::VariableAt0At1At1
            | Pat::VariableAt0At1At1InFor
            | Pat::VariableDelayedExpansionAt0At1At3
            | Pat::VariableDelayedExpansionAt0At1At3InFor => self.lit_byte(p, b'='),
            Pat::VariableAt0At0End | Pat::VariableAt0At1End | Pat::VariableAt0At1At1End => {
                self.before_line_feed_or(p, b'%')
            }
            Pat::VariableDelayedExpansionAt0At0End
            | Pat::VariableDelayedExpansionAt0At1End
            | Pat::VariableDelayedExpansionAt0At1At3End => self.before_line_feed_or(p, b'!'),
            Pat::VariableSubstringAt0 => self.substring(p),
            Pat::VariableReplaceAt0 => self.replace_text(p),
            Pat::VariableAt0At1At1At1 => self.run_without(p, b'%'),
            Pat::VariableDelayedExpansionAt0At1At3At1 => self.run_without(p, b'!'),

            Pat::CommandSetStringsAt0 | Pat::CommandSetStringsAt0InFor => self.quoted_assignment(p),
            Pat::CommandSetInsideAt6
            | Pat::CommandSetInsideAt6InFor
            | Pat::CommandSetInsideAt8At1
            | Pat::CommandSetInsideAt8At1InFor => self.assignment(p),
            Pat::CommandSetInsideAt7 | Pat::CommandSetInsideAt7InFor => self.set_switch(p, b'a'),
            Pat::CommandSetInsideAt8 | Pat::CommandSetInsideAt8InFor => self.set_switch(p, b'p'),
            Pat::CommandSetOperatorsAt0 => self.augmented_assignment(p),
            Pat::CommandSetOperatorsAt1 => self.arithmetic(p),
            Pat::CommandSetOperatorsAt2 => self.lit_byte(p, b'!'),
            Pat::CommandSetOperatorsAt3 => self.plain_assignment(p),
            Pat::CommandSetInsideArithmeticAt2 => self.lit_byte(p, b','),

            Pat::StringsAt0
            | Pat::StringsAt0InFor
            | Pat::CommandSetStringsAt0End
            | Pat::CommandSetInsideAt7At0
            | Pat::CommandSetInsideAt7At0InFor
            | Pat::CommandSetInsideAt7At0End => self.lit_byte(p, b'"'),
            Pat::StringsAt0End => self.string_end(p),
            Pat::StringsAt0At0 => self.lit(p, b"%%"),
            Pat::ParensAt0
            | Pat::ParensAt0InFor
            | Pat::CommandSetGroupAt0
            | Pat::CommandSetGroupAt0InFor => self.lit_byte(p, b'('),
            Pat::ParensAt0End | Pat::CommandSetGroupAt0End => self.lit_byte(p, b')'),
            Pat::ParensAt0At0 => matches!(self.at(p), b',' | b';').then_some(p + 1),
            Pat::NumbersAt0 => self.number(p),
            Pat::OperatorsAt0 => self.at_sign(p),
            Pat::OperatorsAt1 => self.comparison(p),
            Pat::OperatorsAt2 => self.not_word(p),
            Pat::OperatorsAt3 => self.conditional(p),
            Pat::OperatorsAt4 => (self.at(p) == b'|' && !self.prev_is(p, b'^')).then_some(p + 1),
            Pat::OperatorsAt5 => self.redirection(p),

            Pat::ControlsAt3EndInFor => self.unless_repeat(p, |l| l.line_feed_or(p, None)),
            Pat::ControlsAt3At0EndInFor => self.unless_repeat(p, |l| l.do_word(p)),
            Pat::CommentsAt0EndInFor | Pat::CommentsAt2EndInFor => {
                self.unless_repeat(p, |l| l.line_feed_or(p, None))
            }
            Pat::VariableAt0EndInFor => self.unless_repeat(p, |l| l.line_feed_or(p, Some(b'%'))),
            Pat::CommentsAt0At0EndInFor => {
                self.unless_repeat(p, |l| (l.at(p) == b'\n').then_some(p))
            }
            Pat::VariableDelayedExpansionAt0EndInFor => {
                self.unless_repeat(p, |l| l.line_feed_or(p, Some(b'!')))
            }
            Pat::CommandsAt1EndInFor
            | Pat::CommandSetAt0EndInFor
            | Pat::CommentsAt1EndInFor
            | Pat::CommandSetInsideAt6EndInFor
            | Pat::CommandSetInsideAt7EndInFor
            | Pat::CommandSetInsideAt8EndInFor
            | Pat::CommandSetInsideAt8At1EndInFor => self.unless_repeat(p, |l| l.command_end(p)),
            Pat::VariableAt0At0EndInFor
            | Pat::VariableAt0At1EndInFor
            | Pat::VariableAt0At1At1EndInFor => {
                self.unless_repeat(p, |l| l.before_line_feed_or(p, b'%'))
            }
            Pat::VariableDelayedExpansionAt0At0EndInFor
            | Pat::VariableDelayedExpansionAt0At1EndInFor
            | Pat::VariableDelayedExpansionAt0At1At3EndInFor => {
                self.unless_repeat(p, |l| l.before_line_feed_or(p, b'!'))
            }
            Pat::StringsAt0EndInFor => self.unless_repeat(p, |l| l.string_end(p)),
            Pat::ParensAt0EndInFor | Pat::CommandSetGroupAt0EndInFor => {
                self.unless_repeat(p, |l| l.lit_byte(p, b')'))
            }
            Pat::CommandSetStringsAt0EndInFor | Pat::CommandSetInsideAt7At0EndInFor => {
                self.unless_repeat(p, |l| l.lit_byte(p, b'"'))
            }
        }
    }

    /// The end `end` of a rule open inside a `for` line, which fails where
    /// the injected `#repeatParameter` matches.
    /// In a Markdown fence the injection does not apply.
    fn unless_repeat(
        &mut self,
        p: usize,
        end: impl FnOnce(&mut Self) -> Option<usize>,
    ) -> Option<usize> {
        if !self.x.embedded && self.repeat_parameter_end(p).is_some() {
            return None;
        }
        end(self)
    }

    /// The byte `byte` at `p`.
    fn lit_byte(&self, p: usize, byte: u8) -> Option<usize> {
        (p < self.lim() && self.at(p) == byte).then_some(p + 1)
    }

    /// `(?i:word)` at `i`, for the lowercase `word`, and its end.
    fn folded(&self, i: usize, word: &[u8]) -> Option<usize> {
        case_fold::literal_end(self.s, i, word)
    }

    /// Whether a `\s` character ends at `p`: the regex `(?<=\s)`.
    fn space_before(&self, p: usize) -> bool {
        p > 0 && self.space_at(self.char_start_before(p)) > 0
    }

    /// `(?<=^|\s)` at `p`.
    fn after_space(&self, p: usize) -> bool {
        p == 0 || self.space_before(p)
    }

    /// `(?<=^|[@\s])` at `p`, where a command can start.
    fn after_command_start(&self, p: usize) -> bool {
        p == 0 || self.prev_is(p, b'@') || self.space_before(p)
    }

    /// `(?=$|\s)` at `p`.
    fn space_or_eol(&self, p: usize) -> bool {
        self.at_eol(p) || self.space_at(p) > 0
    }

    /// The end of the run of characters from `i` that are not `\s`: the
    /// regex `\S*`.
    fn token_end(&self, i: usize) -> usize {
        let mut at = i;
        while at < self.lim() && self.space_at(at) == 0 {
            at = self.char_end(at);
        }
        self.spend(at - i);
        at
    }

    /// `(?=$\n|[\&)<>|])`, where a command ends: the line feed, or one of the
    /// bytes that end a command.
    fn command_end(&self, p: usize) -> Option<usize> {
        let ends = if p < self.lim() {
            matches!(self.at(p), b'&' | b')' | b'<' | b'>' | b'|')
        } else {
            p == self.lim() && self.at(p) == b'\n'
        };
        ends.then_some(p)
    }

    /// `\n`, or `(x)|\n` for the byte `x` in `group`, with group 1 around it.
    fn line_feed_or(&mut self, p: usize, group: Option<u8>) -> Option<usize> {
        if p == self.lim() && self.at(p) == b'\n' {
            return Some(p + 1);
        }
        let byte = group?;
        self.lit_byte(p, byte)?;
        self.cap(1, p, p + 1);
        Some(p + 1)
    }

    /// `(?=[\nx])` for the byte `x`.
    fn before_line_feed_or(&self, p: usize, byte: u8) -> Option<usize> {
        let at = self.at(p);
        (at == b'\n' || (p < self.lim() && at == byte)).then_some(p)
    }

    /// The line's last `%` and `!`, found once a line.
    fn marks(&mut self) -> Marks {
        if let Some(marks) = self.x.marks {
            return marks;
        }
        let marks = Marks {
            percent: memchr::memrchr(b'%', self.s),
            bang: memchr::memrchr(b'!', self.s),
        };
        self.spend(self.s.len());
        self.x.marks = Some(marks);
        marks
    }

    /// The first `=` at or after `from` on the line.
    fn equals_from(&self, from: usize) -> Option<usize> {
        let rest = self.s.get(from..)?;
        let found = memchr::memchr(b'=', rest);
        self.spend(found.map_or(rest.len(), |at| at + 1));
        found.map(|at| from + at)
    }

    /// `"`, or the end `(")|(\n)` of a string, with group 1 around the quote
    /// and group 2 around the line feed.
    fn string_end(&mut self, p: usize) -> Option<usize> {
        if self.lit_byte(p, b'"').is_some() {
            self.cap(1, p, p + 1);
            return Some(p + 1);
        }
        if p == self.lim() && self.at(p) == b'\n' {
            self.cap(2, p, p + 1);
            return Some(p + 1);
        }
        None
    }

    /// `(?<=^|[=\s])(0[Xx]\h*|[-+]?\d+)(?=$|[<>\s])`, with group 1 around the
    /// number.
    ///
    /// Neither `\h*` nor `\d+` gives a character back to good effect, since
    /// the lookahead after them fails at a digit.
    fn number(&mut self, p: usize) -> Option<usize> {
        if !(p == 0 || self.prev_is(p, b'=') || self.space_before(p)) {
            return None;
        }
        let ends =
            |l: &Self, e: usize| l.at_eol(e) || matches!(l.at(e), b'<' | b'>') || l.space_at(e) > 0;
        let hex = (self.at(p) == b'0' && matches!(self.at(p + 1), b'x' | b'X'))
            .then(|| self.skip(p + 2, |b| b.is_ascii_hexdigit()))
            .filter(|&e| ends(self, e));
        let end = hex.or_else(|| {
            let digits = if matches!(self.at(p), b'-' | b'+') {
                p + 1
            } else {
                p
            };
            let e = self.digit_end(digits);
            (e > digits && ends(self, e)).then_some(e)
        })?;
        self.cap(1, p, end);
        Some(end)
    }

    /// `@(?=\S)`.
    fn at_sign(&self, p: usize) -> Option<usize> {
        (self.lit_byte(p, b'@').is_some() && p + 1 < self.lim() && self.space_at(p + 1) == 0)
            .then_some(p + 1)
    }

    /// `(?<=\s)(?i:EQU|NEQ|LSS|LEQ|GTR|GEQ)(?=\s)|==`.
    fn comparison(&self, p: usize) -> Option<usize> {
        if self.space_before(p) {
            let word = [b"equ", b"neq", b"lss", b"leq", b"gtr", b"geq"]
                .iter()
                .find_map(|word| self.folded(p, *word).filter(|&e| self.space_at(e) > 0));
            if word.is_some() {
                return word;
            }
        }
        self.lit(p, b"==")
    }

    /// `(?<=\s)(?i)(NOT)(?=\s)`.
    fn not_word(&self, p: usize) -> Option<usize> {
        if !self.space_before(p) {
            return None;
        }
        self.folded(p, b"not").filter(|&e| self.space_at(e) > 0)
    }

    /// `(?<!\^)&&?|\|\|`.
    fn conditional(&self, p: usize) -> Option<usize> {
        if self.lit_byte(p, b'&').is_some() && !self.prev_is(p, b'^') {
            return Some(if self.at(p + 1) == b'&' { p + 2 } else { p + 1 });
        }
        self.lit(p, b"||")
    }

    /// `<&?|>[\&>]?`.
    fn redirection(&self, p: usize) -> Option<usize> {
        match self.s.get(p..p + 1)? {
            b"<" => Some(if self.at(p + 1) == b'&' { p + 2 } else { p + 1 }),
            b">" => Some(if matches!(self.at(p + 1), b'&' | b'>') {
                p + 2
            } else {
                p + 1
            }),
            _ => None,
        }
    }
}
