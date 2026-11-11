//! The inside of a `set` command: assignments, `/a` arithmetic and `/p`
//! prompts.

use super::super::engine::Lexer;
use crate::rules::Chars;

impl Lexer<'_> {
    /// The end of the character at `i`, the line feed included, when one
    /// stands there and it is not a space: the regex `[^ ]`.
    fn not_space(&self, i: usize) -> Option<usize> {
        if i < self.lim() {
            return (self.at(i) != b' ').then(|| self.char_end(i));
        }
        (i == self.lim() && self.at(i) == b'\n').then_some(i + 1)
    }

    /// `([^ ][^=]*)(=)`: a variable's name and its `=`, with groups around
    /// both.
    ///
    /// `[^=]*` stops at the first `=`, which the match needs next.
    /// Giving characters back leaves no `=` after them.
    pub(super) fn assignment(&mut self, p: usize) -> Option<usize> {
        let after = self.not_space(p)?;
        let equals = self.equals_from(after)?;
        self.capture(1, p, equals);
        self.capture(2, equals, equals + 1);
        Some(equals + 1)
    }

    /// `(")\s*([^ ][^=]*)(=)`: a quoted assignment, with groups around the
    /// quote, the name and the `=`.
    ///
    /// The name starts after the blanks, or, when that leaves no `=` after
    /// its first character, at the last blank that is not a space.
    /// Giving that blank back lets `[^ ]` take it, and the `=` that the name
    /// first took ends the match instead.
    pub(super) fn quoted_assignment(&mut self, p: usize) -> Option<usize> {
        self.lit_byte(p, b'"')?;
        let blanks = self.space_end(p + 1);
        let (name, equals) = if let Some(after) = self.not_space(blanks)
            && let Some(equals) = self.equals_from(after)
        {
            (blanks, equals)
        } else if blanks < self.lim() && self.at(blanks) == b'=' {
            let name = (p + 1..blanks)
                .rev()
                .find(|&i| self.at(i) != b' ' && self.s.get(i).is_some_and(|b| b & 0xc0 != 0x80))?;
            (name, blanks)
        } else {
            return None;
        };
        self.capture(1, p, p + 1);
        self.capture(2, name, equals);
        self.capture(3, equals, equals + 1);
        Some(equals + 1)
    }

    /// `\s+/[Aa]\s+` or `\s+/[Pp]\s+` for the lowercase `switch`.
    ///
    /// The first `\s+` gives no blank back to good effect, since a blank is
    /// not `/`.
    pub(super) fn set_switch(&self, p: usize, switch: u8) -> Option<usize> {
        let slash = self.space_end(p);
        if slash == p || self.lit_byte(slash, b'/').is_none() {
            return None;
        }
        if !(slash + 1 < self.lim() && self.at(slash + 1).eq_ignore_ascii_case(&switch)) {
            return None;
        }
        let end = self.space_end(slash + 2);
        (end > slash + 2).then_some(end)
    }

    /// `([^ ]*)((?:[-*+/]|%%|[\&^|]|<<|>>)=)`: a name and an augmented
    /// assignment, with groups around both.
    ///
    /// `[^ ]*` takes the whole run before a space, then gives characters
    /// back until an operator and `=` follow it.
    /// So the operator is the last one in the run.
    pub(super) fn augmented_assignment(&mut self, p: usize) -> Option<usize> {
        let mut run = self.skip(p, |b| b != b' ');
        if run == self.lim() && self.at(run) == b'\n' {
            run += 1;
        }
        let mut at = run;
        loop {
            if let Some(end) = self.operator_equals(at) {
                self.capture(1, p, at);
                self.capture(2, at, end);
                return Some(end);
            }
            if at == p {
                return None;
            }
            at -= 1;
            self.spend(1);
        }
    }

    /// `(?:[-*+/]|%%|[\&^|]|<<|>>)=` at `i`, and its end.
    fn operator_equals(&self, i: usize) -> Option<usize> {
        let operator = match self.s.get(i..)? {
            [b'-' | b'*' | b'+' | b'/' | b'&' | b'^' | b'|', ..] => i + 1,
            [b'%', b'%', ..] | [b'<', b'<', ..] | [b'>', b'>', ..] => i + 2,
            _ => return None,
        };
        self.lit_byte(operator, b'=')
    }

    /// `[-*+/]|%%|[\&^|]|<<|>>|~`.
    pub(super) fn arithmetic(&self, p: usize) -> Option<usize> {
        match self.s.get(p..)? {
            [b'%', b'%', ..] | [b'<', b'<', ..] | [b'>', b'>', ..] => Some(p + 2),
            [b'-' | b'*' | b'+' | b'/' | b'&' | b'^' | b'|' | b'~', ..] => Some(p + 1),
            _ => None,
        }
    }

    /// `([^ =]*)(=)`: a name and its `=`, with groups around both.
    pub(super) fn plain_assignment(&mut self, p: usize) -> Option<usize> {
        let equals = self.skip(p, |b| !matches!(b, b' ' | b'='));
        self.lit_byte(equals, b'=')?;
        self.capture(1, p, equals);
        self.capture(2, equals, equals + 1);
        Some(equals + 1)
    }
}
