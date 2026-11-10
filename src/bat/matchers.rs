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
use super::grammar::Pattern;
use crate::case_fold;
use crate::rules::Chars;

impl Lexer<'_> {
    /// Whether `pattern` matches at `p`, and where its match ends.
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
    pub(super) fn matcher(&mut self, pattern: Pattern, p: usize) -> Option<usize> {
        match pattern {
            Pattern::CommandsAt0 => self.command(p),
            Pattern::CommandsAt1 | Pattern::CommandsAt1InFor => self.echo(p),
            Pattern::CommandsAt2 => self.setlocal(p),
            Pattern::CommandSetAt0 | Pattern::CommandSetAt0InFor => self.set_command(p),
            Pattern::CommandsAt1End
            | Pattern::CommandSetAt0End
            | Pattern::CommentsAt1End
            | Pattern::CommandSetInsideAt6End
            | Pattern::CommandSetInsideAt7End
            | Pattern::CommandSetInsideAt8End
            | Pattern::CommandSetInsideAt8At1End => self.command_end(p),

            Pattern::CommentsAt0 | Pattern::CommentsAt0InFor => self.colon_comment(p),
            Pattern::CommentsAt0End | Pattern::CommentsAt2End => self.line_feed_or(p, None),
            Pattern::CommentsAt0At0 | Pattern::CommentsAt0At0InFor => self.colon_mark(p),
            Pattern::CommentsAt0At0End => (self.at(p) == b'\n').then_some(p),
            Pattern::CommentsAt1 | Pattern::CommentsAt1InFor => self.rem_dot(p),
            Pattern::CommentsAt2 | Pattern::CommentsAt2InFor => self.rem(p),
            Pattern::CommentsAt2At0 => matches!(self.at(p), b'<' | b'>' | b'|').then_some(p + 1),

            Pattern::ConstantsAt0 => self.nul(p),
            Pattern::ControlsAt0 => self.call_exit_goto(p),
            Pattern::ControlsAt1 => self.if_test(p),
            Pattern::ControlsAt2 => self.if_else(p),
            Pattern::ControlsAt3 => self.for_word(p),
            Pattern::ControlsAt3At0InFor => self.in_word(p),
            Pattern::LabelsAt0 => self.label(p),

            Pattern::EscapedCharactersAt0 => self.escape(p),
            Pattern::VariablesAt0 => self.parameter(p),
            Pattern::RepeatParameterAt0 => {
                if self.x.embedded {
                    None
                } else {
                    self.repeat_parameter(p)
                }
            }
            Pattern::VariableAt0 | Pattern::VariableAt0InFor => self.expansion(p, b'%'),
            Pattern::VariableDelayedExpansionAt0 | Pattern::VariableDelayedExpansionAt0InFor => {
                self.expansion(p, b'!')
            }
            Pattern::VariableAt0End => self.line_feed_or(p, Some(b'%')),
            Pattern::VariableDelayedExpansionAt0End => self.line_feed_or(p, Some(b'!')),
            Pattern::VariableAt0At0
            | Pattern::VariableAt0At0InFor
            | Pattern::VariableDelayedExpansionAt0At0
            | Pattern::VariableDelayedExpansionAt0At0InFor => self.lit(p, b":~"),
            Pattern::VariableAt0At1
            | Pattern::VariableAt0At1InFor
            | Pattern::VariableDelayedExpansionAt0At1
            | Pattern::VariableDelayedExpansionAt0At1InFor => self.lit_byte(p, b':'),
            Pattern::VariableAt0At1At1
            | Pattern::VariableAt0At1At1InFor
            | Pattern::VariableDelayedExpansionAt0At1At3
            | Pattern::VariableDelayedExpansionAt0At1At3InFor => self.lit_byte(p, b'='),
            Pattern::VariableAt0At0End
            | Pattern::VariableAt0At1End
            | Pattern::VariableAt0At1At1End => self.before_line_feed_or(p, b'%'),
            Pattern::VariableDelayedExpansionAt0At0End
            | Pattern::VariableDelayedExpansionAt0At1End
            | Pattern::VariableDelayedExpansionAt0At1At3End => self.before_line_feed_or(p, b'!'),
            Pattern::VariableSubstringAt0 => self.substring(p),
            Pattern::VariableReplaceAt0 => self.replace_text(p),
            Pattern::VariableAt0At1At1At1 => self.run_without(p, b'%'),
            Pattern::VariableDelayedExpansionAt0At1At3At1 => self.run_without(p, b'!'),

            Pattern::CommandSetStringsAt0 | Pattern::CommandSetStringsAt0InFor => {
                self.quoted_assignment(p)
            }
            Pattern::CommandSetInsideAt6
            | Pattern::CommandSetInsideAt6InFor
            | Pattern::CommandSetInsideAt8At1
            | Pattern::CommandSetInsideAt8At1InFor => self.assignment(p),
            Pattern::CommandSetInsideAt7 | Pattern::CommandSetInsideAt7InFor => {
                self.set_switch(p, b'a')
            }
            Pattern::CommandSetInsideAt8 | Pattern::CommandSetInsideAt8InFor => {
                self.set_switch(p, b'p')
            }
            Pattern::CommandSetOperatorsAt0 => self.augmented_assignment(p),
            Pattern::CommandSetOperatorsAt1 => self.arithmetic(p),
            Pattern::CommandSetOperatorsAt2 => self.lit_byte(p, b'!'),
            Pattern::CommandSetOperatorsAt3 => self.plain_assignment(p),
            Pattern::CommandSetInsideArithmeticAt2 => self.lit_byte(p, b','),

            Pattern::StringsAt0
            | Pattern::StringsAt0InFor
            | Pattern::CommandSetStringsAt0End
            | Pattern::CommandSetInsideAt7At0
            | Pattern::CommandSetInsideAt7At0InFor
            | Pattern::CommandSetInsideAt7At0End => self.lit_byte(p, b'"'),
            Pattern::StringsAt0End => self.string_end(p),
            Pattern::StringsAt0At0 => self.lit(p, b"%%"),
            Pattern::ParensAt0
            | Pattern::ParensAt0InFor
            | Pattern::CommandSetGroupAt0
            | Pattern::CommandSetGroupAt0InFor => self.lit_byte(p, b'('),
            Pattern::ParensAt0End | Pattern::CommandSetGroupAt0End => self.lit_byte(p, b')'),
            Pattern::ParensAt0At0 => matches!(self.at(p), b',' | b';').then_some(p + 1),
            Pattern::NumbersAt0 => self.number(p),
            Pattern::OperatorsAt0 => self.at_sign(p),
            Pattern::OperatorsAt1 => self.comparison(p),
            Pattern::OperatorsAt2 => self.not_word(p),
            Pattern::OperatorsAt3 => self.conditional(p),
            Pattern::OperatorsAt4 => {
                (self.at(p) == b'|' && !self.prev_is(p, b'^')).then_some(p + 1)
            }
            Pattern::OperatorsAt5 => self.redirection(p),

            Pattern::ControlsAt3EndInFor => self.unless_repeat(p, |l| l.line_feed_or(p, None)),
            Pattern::ControlsAt3At0EndInFor => self.unless_repeat(p, |l| l.do_word(p)),
            Pattern::CommentsAt0EndInFor | Pattern::CommentsAt2EndInFor => {
                self.unless_repeat(p, |l| l.line_feed_or(p, None))
            }
            Pattern::VariableAt0EndInFor => {
                self.unless_repeat(p, |l| l.line_feed_or(p, Some(b'%')))
            }
            Pattern::CommentsAt0At0EndInFor => {
                self.unless_repeat(p, |l| (l.at(p) == b'\n').then_some(p))
            }
            Pattern::VariableDelayedExpansionAt0EndInFor => {
                self.unless_repeat(p, |l| l.line_feed_or(p, Some(b'!')))
            }
            Pattern::CommandsAt1EndInFor
            | Pattern::CommandSetAt0EndInFor
            | Pattern::CommentsAt1EndInFor
            | Pattern::CommandSetInsideAt6EndInFor
            | Pattern::CommandSetInsideAt7EndInFor
            | Pattern::CommandSetInsideAt8EndInFor
            | Pattern::CommandSetInsideAt8At1EndInFor => {
                self.unless_repeat(p, |l| l.command_end(p))
            }
            Pattern::VariableAt0At0EndInFor
            | Pattern::VariableAt0At1EndInFor
            | Pattern::VariableAt0At1At1EndInFor => {
                self.unless_repeat(p, |l| l.before_line_feed_or(p, b'%'))
            }
            Pattern::VariableDelayedExpansionAt0At0EndInFor
            | Pattern::VariableDelayedExpansionAt0At1EndInFor
            | Pattern::VariableDelayedExpansionAt0At1At3EndInFor => {
                self.unless_repeat(p, |l| l.before_line_feed_or(p, b'!'))
            }
            Pattern::StringsAt0EndInFor => self.unless_repeat(p, |l| l.string_end(p)),
            Pattern::ParensAt0EndInFor | Pattern::CommandSetGroupAt0EndInFor => {
                self.unless_repeat(p, |l| l.lit_byte(p, b')'))
            }
            Pattern::CommandSetStringsAt0EndInFor | Pattern::CommandSetInsideAt7At0EndInFor => {
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
        self.capture(1, p, p + 1);
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
            self.capture(1, p, p + 1);
            return Some(p + 1);
        }
        if p == self.lim() && self.at(p) == b'\n' {
            self.capture(2, p, p + 1);
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
        self.capture(1, p, end);
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
