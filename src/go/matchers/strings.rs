//! The `fmt` verbs inside string literals.

use super::super::engine::Lexer;
use crate::rules::Chars;

impl Lexer<'_> {
    /// The end of `\[\d+]` at `p`: an explicit argument index.
    fn argument_index(&self, p: usize) -> Option<usize> {
        if p >= self.lim() || self.at(p) != b'[' {
            return None;
        }
        let digits = self.decimal_digits(p + 1)?;
        (digits < self.lim() && self.at(digits) == b']').then_some(digits + 1)
    }

    /// The end of `\d+` at `p`.
    fn decimal_digits(&self, p: usize) -> Option<usize> {
        let end = self.digit_end(p);
        (end > p).then_some(end)
    }

    /// ```text
    /// %(\[\d+])?([- #+0]{0,2}((\d+|\*)?(\.?(\d+|\*|(\[\d+])\*?)?(\[\d+])?)?))?[%EFGTUXb-gopqstvwx]
    /// ```
    ///
    /// Between the `%` and the verb every part is optional.
    /// Each takes only digits, brackets, `*`, `.` and flags, none of them a
    /// verb.
    /// Taking each part greedily therefore reaches the one place a verb can
    /// follow.
    /// A part given back would leave a character no verb is.
    pub(in crate::go) fn string_placeholder_at0(&mut self, p: usize) -> Option<usize> {
        if p >= self.lim() || self.at(p) != b'%' {
            return None;
        }
        let mut i = self.argument_index(p + 1).unwrap_or(p + 1);
        for _ in 0..2 {
            if i < self.lim() && matches!(self.at(i), b'-' | b' ' | b'#' | b'+' | b'0') {
                i += 1;
            }
        }
        if let Some(end) = self.decimal_digits(i) {
            i = end;
        } else if i < self.lim() && self.at(i) == b'*' {
            i += 1;
        }
        if i < self.lim() && self.at(i) == b'.' {
            i += 1;
        }
        if let Some(end) = self.decimal_digits(i) {
            i = end;
        } else if i < self.lim() && self.at(i) == b'*' {
            i += 1;
        } else if let Some(end) = self.argument_index(i) {
            i = end + usize::from(end < self.lim() && self.at(end) == b'*');
        }
        i = self.argument_index(i).unwrap_or(i);
        let verb = i < self.lim()
            && matches!(
                self.at(i),
                b'%' | b'E' | b'F' | b'G' | b'T' | b'U' | b'X' | b'b'
                    ..=b'g' | b'o' | b'p' | b'q' | b's' | b't' | b'v' | b'w' | b'x'
            );
        verb.then_some(i + 1)
    }
}
