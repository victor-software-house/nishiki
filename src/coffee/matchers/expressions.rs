//! Functions, calls, numbers, objects, properties and operators.

use super::{ARGUMENT_STOPS, Coffee, Dialect, Lexer, is_ident_start};
use crate::rules::Chars;

/// Whether a bare argument may start with a lone `@`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum LoneAt {
    /// `(@|@?[$\w]+|…)`, after a method name.
    Taken,
    /// `(@?[$\w]+|…)`, after a function name.
    Refused,
}

/// What must follow a property.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Ahead {
    /// Another property or `::`: the property is an object.
    Member,
    /// Anything.
    Any,
}

impl<const D: u8> Lexer<'_, D>
where
    Coffee<D>: Dialect,
{
    /// `[-=]>`.
    pub(super) fn arrow(&self, p: usize) -> Option<usize> {
        (matches!(self.at(p), b'-' | b'=') && self.at(p + 1) == b'>').then_some(p + 2)
    }

    /// `(?=(\([^()]*\)\s*)?[-=]>)` at `p`.
    /// The parameter list is tried first.
    /// `[^()]*` stops only at a parenthesis, so giving it back leaves no `)`.
    pub(super) fn function_ahead(&self, p: usize) -> bool {
        if self.at(p) == b'(' {
            let close = self.skip(p + 1, |b| b != b'(' && b != b')');
            if self.at(close) == b')' && self.arrow(self.space_end(close + 1)).is_some() {
                return true;
            }
        }
        self.arrow(p).is_some()
    }

    /// `(?<=\s|^)((@)?[$A-Z_a-z][$\w]*)\s*([:=])\s*(?=(\([^()]*\)\s*)?[-=]>)`.
    /// Without the `@`, the name would have to start at it, which it cannot.
    /// Giving back the `\s*` after the sign leaves a blank where the
    /// lookahead needs `(`, `-` or `=`.
    pub(super) fn function_assignment(&mut self, p: usize) -> Option<usize> {
        if !self.after_space(p) {
            return None;
        }
        let from = self.lit(p, b"@").unwrap_or(p);
        let name = self.ident(from)?;
        let (sign, end) = self.assigned_function(name)?;
        self.capture(1, p, name);
        if from > p {
            self.capture(2, p, from);
        }
        self.capture(3, sign, sign + 1);
        Some(end)
    }

    /// `\s*([:=])\s*(?=(\([^()]*\)\s*)?[-=]>)` at `p`: where the sign stands,
    /// and where the match ends.
    fn assigned_function(&self, p: usize) -> Option<(usize, usize)> {
        let sign = self.space_end(p);
        if !matches!(self.at(sign), b':' | b'=') {
            return None;
        }
        let end = self.space_end(sign + 1);
        self.function_ahead(end).then_some((sign, end))
    }

    /// `(?<=\s|^)(?:((')([^']*?)('))|((")([^"]*?)(")))\s*([:=])\s*(?=…)`.
    /// The lazy body cannot take its quote, so it ends at the first one.
    pub(super) fn quoted_function_assignment(&mut self, p: usize) -> Option<usize> {
        if !self.after_space(p) {
            return None;
        }
        let quote = self.at(p);
        let first = match quote {
            b'\'' => 1,
            b'"' => 5,
            _ => return None,
        };
        let close = p + 1 + memchr::memchr(quote, self.text(p + 1, self.lim()))?;
        let (sign, end) = self.assigned_function(close + 1)?;
        self.capture(first, p, close + 1);
        self.capture(first + 1, p, p + 1);
        self.capture(first + 2, p + 1, close);
        self.capture(first + 3, close, close + 1);
        self.capture(9, sign, sign + 1);
        Some(end)
    }

    /// `([$A-Z_a-z][$\w]*)(\.\.\.)?`.
    pub(super) fn param(&mut self, p: usize) -> Option<usize> {
        let name = self.ident(p)?;
        self.capture(1, p, name);
        Some(self.splat_after(name))
    }

    /// `(@(?:[$A-Z_a-z][$\w]*)?)(\.\.\.)?`.
    pub(super) fn instance_param(&mut self, p: usize) -> Option<usize> {
        let at = self.lit(p, b"@")?;
        let name = self.ident(at).unwrap_or(at);
        self.capture(1, p, name);
        Some(self.splat_after(name))
    }

    /// `(\.\.\.)?` at `p`, as group 2.
    fn splat_after(&mut self, p: usize) -> usize {
        let Some(end) = self.lit(p, b"...") else {
            return p;
        };
        self.capture(2, p, end);
        end
    }

    /// `(?<=\s|^)(\{)(?=[^"#']+?}[]}\s]*=)`, or the same with `[` and `]`.
    /// The lazy body grows one character at a time and may take a `close`.
    /// Every close inside one run of `[]}\s]` is followed by the same run.
    /// After a run that `=` does not end, the next close to try is therefore
    /// past it.
    pub(super) fn destructured(&mut self, p: usize, open: u8, close: u8) -> Option<usize> {
        if !self.after_space(p) || self.at(p) != open {
            return None;
        }
        let mut next = p + 1;
        loop {
            if matches!(self.at(next), 0 | b'"' | b'#' | b'\'') {
                return None;
            }
            let candidate = if next >= self.lim() {
                next + 1
            } else {
                self.char_end(next)
            };
            if self.at(candidate) != close {
                next = candidate;
                continue;
            }
            let run = self.closing_run(candidate + 1);
            if self.at(run) == b'=' {
                self.capture(1, p, p + 1);
                return Some(p + 1);
            }
            next = run;
        }
    }

    /// The end of the run of `[]}\s]` characters from `i`.
    fn closing_run(&self, mut i: usize) -> usize {
        loop {
            if matches!(self.at(i), b']' | b'}') {
                i += 1;
                continue;
            }
            let width = self.space_at(i);
            if width == 0 {
                return i;
            }
            i += width;
        }
    }

    /// `\.` as group 1 or `::` as group 2 at `p`: where it ends, and its
    /// group.
    fn member_sign(&self, p: usize) -> Option<(usize, usize)> {
        match self.at(p) {
            b'.' => Some((p + 1, 1)),
            b':' if self.at(p + 1) == b':' => Some((p + 2, 2)),
            _ => None,
        }
    }

    /// `(?:(\.)|(::))\s*([$\w]+)\s*(?=\()`.
    /// Giving back the name leaves a `[$\w]` character where `(` must stand.
    pub(super) fn method_call(&mut self, p: usize) -> Option<usize> {
        let (sign, group) = self.member_sign(p)?;
        let from = self.space_end(sign);
        let name = self.dollar_word_end(from);
        if name == from {
            return None;
        }
        let end = self.space_end(name);
        if self.at(end) != b'(' {
            return None;
        }
        self.capture(group, p, sign);
        self.capture(3, from, name);
        Some(end)
    }

    /// `(?:(\.)|(::))\s*([$\w]+)\s*(?=\s+(?!stop)(?=(@|@?[$\w]+|[-=]>|-\d|["'\
    /// [{])))`.
    pub(super) fn bare_method_call(&mut self, p: usize) -> Option<usize> {
        let (sign, group) = self.member_sign(p)?;
        let from = self.space_end(sign);
        let name = self.dollar_word_end(from);
        if name == from {
            return None;
        }
        let end = self.bare_arguments_ahead(name, LoneAt::Taken)?;
        self.capture(group, p, sign);
        self.capture(3, from, name);
        Some(end)
    }

    /// `\s*(?=\s+(?!stop)(?=argument))` after a bare call's name at `p`:
    /// where the match ends.
    /// The `\s*` gives back the last blank to the lookahead's `\s+`.
    /// Giving back more moves no blank past the lookahead.
    fn bare_arguments_ahead(&self, p: usize, lone: LoneAt) -> Option<usize> {
        let argument = self.space_end(p);
        if argument == p || self.stop_word_at(argument) || !self.argument_ahead(argument, lone) {
            return None;
        }
        Some(self.char_start_before(argument))
    }

    /// `(?<![$\w])(of|in|…|instanceof)(?![$\w])` at `p`, after a blank.
    /// Each word must be the whole run of `[$\w]`.
    fn stop_word_at(&self, p: usize) -> bool {
        let end = self.dollar_word_end(p);
        crate::bytes::contains(ARGUMENT_STOPS, self.text(p, end))
    }

    /// `(?=\s*(?<![$\w])(of|…)(?![$\w]))|(?=\s*([]#)}]|$))` at `p`.
    /// Giving back the `\s*` leaves a blank where a word, a bracket or the
    /// end must stand.
    pub(super) fn arguments_end(&self, p: usize) -> bool {
        let at = self.space_end(p);
        (!self.dollar_word_before(at) && self.stop_word_at(at))
            || matches!(self.at(at), b']' | b'#' | b')' | b'}')
            || self.at_eol(at)
    }

    /// `(?=(@|@?[$\w]+|[-=]>|-\d|["'\[{]))` at `p`, where `lone` says whether
    /// the first branch, a lone `@`, is there.
    pub(super) fn argument_ahead(&self, p: usize, lone: LoneAt) -> bool {
        match self.at(p) {
            b'@' => lone == LoneAt::Taken || self.dollar_word_at(p + 1).is_some(),
            b'-' => self.at(p + 1) == b'>' || self.digit_end(p + 1) > p + 1,
            b'=' => self.at(p + 1) == b'>',
            b'"' | b'\'' | b'[' | b'{' => true,
            _ => self.dollar_word_at(p).is_some(),
        }
    }

    /// `(@)?([$\w]+)(?=\()`.
    pub(super) fn function_call(&mut self, p: usize) -> Option<usize> {
        let from = self.lit(p, b"@").unwrap_or(p);
        let name = self.dollar_word_end(from);
        if name == from || self.at(name) != b'(' {
            return None;
        }
        if from > p {
            self.capture(1, p, from);
        }
        self.capture(2, from, name);
        Some(name)
    }

    /// `(@)?([$\w]+)\s*(?=\s+(?!stop)(?=(@?[$\w]+|[-=]>|-\d|["'\[{])))`.
    /// Giving back the name leaves a `[$\w]` character where the lookahead
    /// needs a blank.
    pub(super) fn bare_function_call(&mut self, p: usize) -> Option<usize> {
        let from = self.lit(p, b"@").unwrap_or(p);
        let name = self.dollar_word_end(from);
        if name == from {
            return None;
        }
        let end = self.bare_arguments_ahead(name, LoneAt::Refused)?;
        if from > p {
            self.capture(1, p, from);
        }
        self.capture(2, from, name);
        Some(end)
    }

    /// `\b(?<!\$)0([Xx])\h+\b(?!\$)` or `\b(?<!\$)0([Bb])[01]+\b(?!\$)`, with
    /// `letter` lower case.
    pub(super) fn prefixed_number(
        &self,
        p: usize,
        letter: u8,
        digit: fn(u8) -> bool,
    ) -> Option<usize> {
        if self.prev(p) == Some(b'$') || !self.boundary(p) || self.at(p) != b'0' {
            return None;
        }
        if self.at(p + 1).to_ascii_lowercase() != letter {
            return None;
        }
        let end = self.skip(p + 2, digit);
        self.number_end(p + 2, end)
    }

    /// `\b(?<!\$)0([Oo])?[0-7]+\b(?!\$)`.
    pub(super) fn octal_number(&self, p: usize) -> Option<usize> {
        if self.prev(p) == Some(b'$') || !self.boundary(p) || self.at(p) != b'0' {
            return None;
        }
        let from = if matches!(self.at(p + 1), b'O' | b'o') {
            p + 2
        } else {
            p + 1
        };
        let end = self.skip(from, |b| matches!(b, b'0'..=b'7'));
        self.number_end(from, end)
    }

    /// `end` when the digits `from..end` are some, `\b` holds at `end` and
    /// no `$` stands there.
    fn number_end(&self, from: usize, end: usize) -> Option<usize> {
        (end > from && self.boundary(end) && self.at(end) != b'$').then_some(end)
    }

    /// The nine branches of the decimal number, in the grammar's order, each
    /// followed by `(?!\$)`, which sends a failing branch on to the next.
    /// Digit runs are whole: giving one back leaves a digit where `\b`, `.`
    /// or an exponent must stand.
    pub(super) fn decimal(&mut self, p: usize) -> Option<usize> {
        if self.prev(p) == Some(b'$') {
            return None;
        }
        let word = self.boundary(p);
        let whole = self.digits(p);
        let numbered = word && whole > p;
        let point = numbered && self.at(whole) == b'.';
        let fraction = if point { self.digits(whole + 1) } else { whole };
        let lead = !word && self.at(p) == b'.';
        let lead_fraction = if lead { self.digits(p + 1) } else { p };
        let fractional = point && fraction > whole + 1;
        let lead_fractional = lead && lead_fraction > p + 1;
        let branches = [
            (
                fractional.then(|| self.exponent(fraction)).flatten(),
                Some((1, whole)),
            ),
            (
                point.then(|| self.exponent(whole + 1)).flatten(),
                Some((2, whole)),
            ),
            (
                lead_fractional
                    .then(|| self.exponent(lead_fraction))
                    .flatten(),
                Some((3, p)),
            ),
            (numbered.then(|| self.exponent(whole)).flatten(), None),
            (
                (fractional && self.boundary(fraction)).then_some(fraction),
                Some((4, whole)),
            ),
            (
                (numbered && self.starts(whole, b"..")).then_some(whole),
                None,
            ),
            (
                (point && !self.boundary(whole + 1)).then_some(whole + 1),
                Some((5, whole)),
            ),
            (
                (lead_fractional && self.boundary(lead_fraction)).then_some(lead_fraction),
                Some((6, p)),
            ),
            (
                (numbered && self.boundary(whole) && self.at(whole) != b'.').then_some(whole),
                None,
            ),
        ];
        let (end, dot) = branches.into_iter().find_map(|(end, dot)| {
            end.filter(|&end| self.at(end) != b'$')
                .map(|end| (end, dot))
        })?;
        if let Some((group, at)) = dot {
            self.capture(group, at, at + 1);
        }
        Some(end)
    }

    /// The end of the run of `[0-9]` from `i`.
    fn digits(&self, i: usize) -> usize {
        self.skip(i, |b| b.is_ascii_digit())
    }

    /// `[Ee][-+]?[0-9]+\b` at `i`.
    fn exponent(&self, i: usize) -> Option<usize> {
        if !matches!(self.at(i), b'E' | b'e') {
            return None;
        }
        let from = if matches!(self.at(i + 1), b'-' | b'+') {
            i + 2
        } else {
            i + 1
        };
        let end = self.digits(from);
        (end > from && self.boundary(end)).then_some(end)
    }

    /// `(?=\s*\??(\.\s*[$A-Z_a-z]\w*|::))` at `p`.
    /// Without the `?`, the `.` or `::` would have to stand at it.
    pub(super) fn object_ahead(&self, p: usize) -> bool {
        let at = self.space_end(p);
        let member = self.lit(at, b"?").unwrap_or(at);
        match self.at(member) {
            b'.' => is_ident_start(self.at(self.space_end(member + 1))),
            b':' => self.at(member + 1) == b':',
            _ => false,
        }
    }

    /// `[A-Z][$0-9A-Z_]*(?=\s*\??(\.\s*[$A-Z_a-z]\w*|::))`.
    /// Giving back the run leaves one of its characters where the lookahead
    /// needs a blank, `?`, `.` or `:`.
    pub(super) fn constant_object(&self, p: usize) -> Option<usize> {
        if !self.at(p).is_ascii_uppercase() {
            return None;
        }
        let end = self.constant_run(p + 1);
        self.object_ahead(end).then_some(end)
    }

    /// The end of the run of `[$0-9A-Z_]` from `i`.
    fn constant_run(&self, i: usize) -> usize {
        self.skip(i, |b| {
            b == b'$' || b == b'_' || b.is_ascii_digit() || b.is_ascii_uppercase()
        })
    }

    /// `(?:(\.)|(::))\s*([A-Z][$0-9A-Z_]*\b\$*)`, followed by
    /// `(?=\s*\??(\.\s*[$A-Z_a-z]\w*|::))` when `ahead` is [`Ahead::Member`].
    /// The run takes `$` too, so it gives characters back until `\b` holds:
    /// between a word character and a `$`, or at its end.
    pub(super) fn constant_property(&mut self, p: usize, ahead: Ahead) -> Option<usize> {
        let (sign, group) = self.member_sign(p)?;
        let from = self.space_end(sign);
        if !self.at(from).is_ascii_uppercase() {
            return None;
        }
        let run = self.constant_run(from + 1);
        let end = (from + 1..=run).rev().find_map(|cut| {
            if !self.boundary(cut) {
                return None;
            }
            let end = self.run_of(cut, b'$');
            (ahead == Ahead::Any || self.object_ahead(end)).then_some(end)
        })?;
        self.capture(group, p, sign);
        self.capture(3, from, end);
        Some(end)
    }

    /// `(?:(\.)|(::))\s*(\$*[$A-Z_a-z][$\w]*)`, followed by the member
    /// lookahead when `ahead` is [`Ahead::Member`].
    /// However the `$`s split between `\$*` and the class, the name is the
    /// whole run of `[$\w]`, and it starts with a `[$A-Z_a-z]` character.
    pub(super) fn variable_property(&mut self, p: usize, ahead: Ahead) -> Option<usize> {
        let (sign, group) = self.member_sign(p)?;
        let from = self.space_end(sign);
        let end = self.ident(from)?;
        if ahead == Ahead::Member && !self.object_ahead(end) {
            return None;
        }
        self.capture(group, p, sign);
        self.capture(3, from, end);
        Some(end)
    }

    /// `(?:(\.)|(::))\s*([0-9][$\w]*)`.
    pub(super) fn invalid_property(&mut self, p: usize) -> Option<usize> {
        let (sign, group) = self.member_sign(p)?;
        let from = self.space_end(sign);
        if !self.at(from).is_ascii_digit() {
            return None;
        }
        let end = self.dollar_word_end(from);
        self.capture(group, p, sign);
        self.capture(3, from, end);
        Some(end)
    }

    /// `(?<!\$)\b[0-9]+[$\w]*`: digits start a run of `[$\w]`.
    pub(super) fn invalid_identifier(&self, p: usize) -> Option<usize> {
        let starts = self.prev(p) != Some(b'$') && self.boundary(p) && self.at(p).is_ascii_digit();
        starts.then(|| self.dollar_word_end(p))
    }

    /// `(?:([$A-Z_a-z][$\w]*)?\s+|(?<![$\w]))(and=|or=)`.
    /// A name with no blank after it fails the first branch, but the second
    /// may still match at `p`.
    pub(super) fn word_assignment(&mut self, p: usize) -> Option<usize> {
        let name = self.ident(p);
        let blank = self.space_end(name.unwrap_or(p));
        if blank > name.unwrap_or(p)
            && let Some(end) = self.word_assigner(blank)
        {
            if let Some(name) = name {
                self.capture(1, p, name);
            }
            self.capture(2, blank, end);
            return Some(end);
        }
        if self.dollar_word_before(p) {
            return None;
        }
        let end = self.word_assigner(p)?;
        self.capture(2, p, end);
        Some(end)
    }

    /// `and=|or=`.
    fn word_assigner(&self, p: usize) -> Option<usize> {
        self.lit(p, b"and=").or_else(|| self.lit(p, b"or="))
    }

    /// `([$A-Z_a-z][$\w]*)?\s*(operator)`.
    /// Every operator starts with a character that no name takes, so a name
    /// that the operator does not follow fails the whole pattern at `p`.
    pub(super) fn assignment(
        &mut self,
        p: usize,
        operator: fn(&Self, usize) -> Option<usize>,
    ) -> Option<usize> {
        let name = self.ident(p);
        let from = self.space_end(name.unwrap_or(p));
        let end = operator(self, from)?;
        if let Some(name) = name {
            self.capture(1, p, name);
        }
        self.capture(2, from, end);
        Some(end)
    }

    /// `(?:[-%*+]|&&|\|\||\?|(?<!\()/)=`.
    pub(super) fn compound_operator(&self, p: usize) -> Option<usize> {
        match self.at(p) {
            b'-' | b'%' | b'*' | b'+' | b'?' => self.lit(p + 1, b"="),
            b'/' if self.prev(p) != Some(b'(') => self.lit(p + 1, b"="),
            b'&' => self.lit(p, b"&&="),
            b'|' => self.lit(p, b"||="),
            _ => None,
        }
    }

    /// `(?:[\&^]|<<|>>>??|\|)=`.
    /// The lazy `>>>??` tries `>>=` before `>>>=`.
    pub(super) fn bitwise_operator(&self, p: usize) -> Option<usize> {
        match self.at(p) {
            b'&' | b'^' | b'|' => self.lit(p + 1, b"="),
            b'<' => self.lit(p, b"<<="),
            b'>' => self.lit(p, b">>=").or_else(|| self.lit(p, b">>>=")),
            _ => None,
        }
    }

    /// `(=|:(?!:))(?![=>])`.
    pub(super) fn plain_assignment(&self, p: usize) -> Option<usize> {
        let sign = match self.at(p) {
            b'=' => true,
            b':' => self.at(p + 1) != b':',
            _ => false,
        };
        (sign && !matches!(self.at(p + 1), b'=' | b'>')).then_some(p + 1)
    }

    /// `<<|>>>?`.
    pub(super) fn shift(&self, p: usize) -> Option<usize> {
        self.lit(p, b"<<")
            .or_else(|| self.lit(p, b">>>"))
            .or_else(|| self.lit(p, b">>"))
    }

    /// `!=|<=|>=|==|[<>]`.
    pub(super) fn comparison(&self, p: usize) -> Option<usize> {
        match self.at(p) {
            b'!' | b'=' => self.lit(p + 1, b"="),
            b'<' | b'>' => Some(self.lit(p + 1, b"=").unwrap_or(p + 1)),
            _ => None,
        }
    }

    /// `&&|!|\|\|`.
    pub(super) fn logical(&self, p: usize) -> Option<usize> {
        match self.at(p) {
            b'!' => Some(p + 1),
            b'&' => self.lit(p, b"&&"),
            b'|' => self.lit(p, b"||"),
            _ => None,
        }
    }

    /// `(?<!\.)\.{3}` or `(?<!\.)\.{2}`, as `dots` says.
    pub(super) fn slice(&self, p: usize, dots: &[u8]) -> Option<usize> {
        if self.prev(p) == Some(b'.') {
            return None;
        }
        self.lit(p, dots)
    }
}
