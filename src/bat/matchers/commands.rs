//! Commands, comments, control words and labels.

use super::super::engine::Lexer;
use super::super::grammar::{COMMAND_PAIRS, COMMANDS, LONGEST_COMMAND};
use crate::case_fold;
use crate::rules::Chars;

/// The bytes that may follow the `:` of a comment: `[ +,:;=]`.
fn is_comment_mark(byte: u8) -> bool {
    matches!(byte, b' ' | b'+' | b',' | b':' | b';' | b'=')
}

impl Lexer<'_> {
    /// `(?<=^|[@\s])(?i:adprep|…|xcopy)(?=$|\s)`: a command's name.
    ///
    /// The lookahead lets an alternative match only a whole run of
    /// characters other than `\s`, or a pair of runs joined by one space,
    /// such as `net user`.
    /// So the matcher lowers the run, or the pair, and looks it up.
    /// Each pair comes before its first word in the grammar, so a pair wins.
    pub(super) fn command(&self, p: usize) -> Option<usize> {
        if !self.after_command_start(p) {
            return None;
        }
        let first = self.token_end(p);
        if first == p {
            return None;
        }
        if self.at(first) == b' ' && first < self.lim() {
            let second = self.token_end(first + 1);
            if second > first + 1
                && self.lowered_is(p, second, |word| COMMAND_PAIRS.contains(&word))
            {
                return Some(second);
            }
        }
        self.lowered_is(p, first, |word| COMMANDS.binary_search(&word).is_ok())
            .then_some(first)
    }

    /// Whether the text `from..to`, lowered as `(?i)` reads it, passes
    /// `test`.
    /// A text that lowers to more bytes than the longest command fails.
    fn lowered_is(&self, from: usize, to: usize, test: impl Fn(&[u8]) -> bool) -> bool {
        let mut buf = [0; LONGEST_COMMAND];
        case_fold::lower_into(self.text(from, to), &mut buf).is_some_and(|len| test(&buf[..len]))
    }

    /// `(?i)(?<=^|[@\s])(echo)(?:(?=$|[.:])|\s+(?:(o(?:n|ff))(?=\s*$))?)`,
    /// with group 1 around `echo` and group 2 around `on` or `off`.
    ///
    /// `\s+` gives no character back to good effect: the optional group after
    /// it fails at a blank, and the match succeeds without it.
    pub(super) fn echo(&mut self, p: usize) -> Option<usize> {
        if !self.after_command_start(p) {
            return None;
        }
        let word = self.folded(p, b"echo")?;
        if self.at_eol(word) || (word < self.lim() && matches!(self.at(word), b'.' | b':')) {
            self.cap(1, p, word);
            return Some(word);
        }
        let blanks = self.space_end(word);
        if blanks == word {
            return None;
        }
        self.cap(1, p, word);
        let state = self
            .folded(blanks, b"on")
            .or_else(|| self.folded(blanks, b"off"));
        if let Some(state) = state
            && self.at_eol(self.space_end(state))
        {
            self.cap(2, blanks, state);
            return Some(state);
        }
        Some(blanks)
    }

    /// `(?i)(?<=^|[@\s])(setlocal)(?:\s*$|\s+((?:En|Dis)able(?:
    /// Extensions|DelayedExpansion))(?=\s*$))`, with group 1 around `setlocal`
    /// and group 2 around the option.
    ///
    /// `\s*$` holds only where the blanks reach the end of the line.
    /// Giving blanks back leaves `$` before a blank, where it fails.
    pub(super) fn setlocal(&mut self, p: usize) -> Option<usize> {
        if !self.after_command_start(p) {
            return None;
        }
        let word = self.folded(p, b"setlocal")?;
        let blanks = self.space_end(word);
        if self.at_eol(blanks) {
            self.cap(1, p, word);
            return Some(blanks);
        }
        if blanks == word {
            return None;
        }
        let able = self
            .folded(blanks, b"en")
            .or_else(|| self.folded(blanks, b"dis"))?;
        let able = self.folded(able, b"able")?;
        let option = self
            .folded(able, b"extensions")
            .or_else(|| self.folded(able, b"delayedexpansion"))?;
        if !self.at_eol(self.space_end(option)) {
            return None;
        }
        self.cap(1, p, word);
        self.cap(2, blanks, option);
        Some(option)
    }

    /// `(?<=^|[@\s])(?i:SET)(?=$|\s)`.
    pub(super) fn set_command(&self, p: usize) -> Option<usize> {
        if !self.after_command_start(p) {
            return None;
        }
        self.folded(p, b"set").filter(|&e| self.space_or_eol(e))
    }

    /// `(?:^|(&))\s*(?=(:[ +,:;=]))`: the blanks before a `:` comment, at the
    /// start of the line or after `&`, with group 1 around the `&`.
    ///
    /// At the start of the line, `^` is tried first, then `&`.
    /// `\s*` gives no blank back to good effect, since a blank is not `:`.
    pub(super) fn colon_comment(&mut self, p: usize) -> Option<usize> {
        let comment_at = |l: &Self, from: usize| {
            let colon = l.space_end(from);
            (colon + 1 < l.lim() && l.at(colon) == b':' && is_comment_mark(l.at(colon + 1)))
                .then_some(colon)
        };
        if p == 0
            && let Some(end) = comment_at(self, p)
        {
            return Some(end);
        }
        self.lit_byte(p, b'&')?;
        let end = comment_at(self, p + 1)?;
        self.cap(1, p, p + 1);
        Some(end)
    }

    /// `(:[ +,:;=])`, with group 1 around both bytes.
    pub(super) fn colon_mark(&mut self, p: usize) -> Option<usize> {
        if !(p + 1 < self.lim() && self.at(p) == b':' && is_comment_mark(self.at(p + 1))) {
            return None;
        }
        self.cap(1, p, p + 2);
        Some(p + 2)
    }

    /// `(?<=^|[@\s])(?i)(REM)(\.)`, with group 1 around `rem` and group 2
    /// around the dot.
    pub(super) fn rem_dot(&mut self, p: usize) -> Option<usize> {
        if !self.after_command_start(p) {
            return None;
        }
        let word = self.folded(p, b"rem")?;
        self.lit_byte(word, b'.')?;
        self.cap(1, p, word);
        self.cap(2, word, word + 1);
        Some(word + 1)
    }

    /// `(?<=^|[@\s])(?i:rem)\b`.
    pub(super) fn rem(&self, p: usize) -> Option<usize> {
        if !self.after_command_start(p) {
            return None;
        }
        self.folded(p, b"rem")
            .filter(|&e| self.word_at(e).is_none())
    }

    /// `\b(?i:NUL)\b`.
    pub(super) fn nul(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        self.folded(p, b"nul")
            .filter(|&e| self.word_at(e).is_none())
    }

    /// `(?i)(?<=^|\s)(?:call|exit(?=$|\s)|goto(?=$|[:\s]))`.
    pub(super) fn call_exit_goto(&self, p: usize) -> Option<usize> {
        if !self.after_space(p) {
            return None;
        }
        self.folded(p, b"call")
            .or_else(|| self.folded(p, b"exit").filter(|&e| self.space_or_eol(e)))
            .or_else(|| {
                self.folded(p, b"goto")
                    .filter(|&e| self.space_or_eol(e) || (e < self.lim() && self.at(e) == b':'))
            })
    }

    /// `(?<=^|\s)(?i)(if)\s+(?:(not)\s+)?
    /// (exist|defined|errorlevel|cmdextversion)(?=\s)`, with groups around
    /// `if`, `not` and the test.
    ///
    /// No `\s+` gives a blank back to good effect: a word never starts with a
    /// blank.
    /// The optional `not` is tried first, then the test right after `if`.
    pub(super) fn if_test(&mut self, p: usize) -> Option<usize> {
        if !self.after_space(p) {
            return None;
        }
        let word = self.folded(p, b"if")?;
        let blanks = self.space_end(word);
        if blanks == word {
            return None;
        }
        if let Some(not) = self.folded(blanks, b"not") {
            let test = self.space_end(not);
            if test > not
                && let Some(end) = self.test_word(test)
            {
                self.cap(1, p, word);
                self.cap(2, blanks, not);
                self.cap(3, test, end);
                return Some(end);
            }
        }
        let end = self.test_word(blanks)?;
        self.cap(1, p, word);
        self.cap(3, blanks, end);
        Some(end)
    }

    /// `(?i:exist|defined|errorlevel|cmdextversion)(?=\s)` at `i`.
    fn test_word(&self, i: usize) -> Option<usize> {
        [&b"exist"[..], b"defined", b"errorlevel", b"cmdextversion"]
            .iter()
            .find_map(|word| self.folded(i, word).filter(|&e| self.space_at(e) > 0))
    }

    /// `(?<=^|\s)(?i)(?:if|else)(?=$|\s)`.
    pub(super) fn if_else(&self, p: usize) -> Option<usize> {
        if !self.after_space(p) {
            return None;
        }
        self.folded(p, b"if")
            .filter(|&e| self.space_or_eol(e))
            .or_else(|| self.folded(p, b"else").filter(|&e| self.space_or_eol(e)))
    }

    /// `(?<=^|[\&(^\s])(?i)for(?=\s)`.
    pub(super) fn for_word(&self, p: usize) -> Option<usize> {
        let after = p == 0
            || self.prev_is(p, b'&')
            || self.prev_is(p, b'(')
            || self.prev_is(p, b'^')
            || self.space_before(p);
        if !after {
            return None;
        }
        self.folded(p, b"for").filter(|&e| self.space_at(e) > 0)
    }

    /// `(?<=[\^\s])(?i)in(?=\s)`.
    pub(super) fn in_word(&self, p: usize) -> Option<usize> {
        if !(self.prev_is(p, b'^') || self.space_before(p)) {
            return None;
        }
        self.folded(p, b"in").filter(|&e| self.space_at(e) > 0)
    }

    /// `(?<=[)^\s])(?i)do(?=\s)|\n`: the end of a `for` line's `in` clause.
    ///
    /// An inline `(?i)` takes the rest of its group, the alternation
    /// included, so the expression reads `(?<=[)^\s])(?i:do(?=\s)|\n)`.
    /// The line feed ends the clause only after `)`, `^` or a blank.
    pub(super) fn do_word(&mut self, p: usize) -> Option<usize> {
        if !(self.prev_is(p, b')') || self.prev_is(p, b'^') || self.space_before(p)) {
            return None;
        }
        self.folded(p, b"do")
            .filter(|&e| self.space_at(e) > 0)
            .or_else(|| self.line_feed_or(p, None))
    }

    /// `(?i)(?:^\s*|(?<=call|goto)\s*)(:)([^+,:;=\s]\S*)`: a label, with
    /// group 1 around the `:` and group 2 around the name.
    ///
    /// `\s*` gives no blank back to good effect, since a blank is not `:`.
    /// The lookbehind reads four ASCII letters: no character above ASCII
    /// folds to a letter of `call` or `goto`.
    pub(super) fn label(&mut self, p: usize) -> Option<usize> {
        let after_word = p >= 4
            && [&b"call"[..], b"goto"]
                .iter()
                .any(|word| self.text(p - 4, p).eq_ignore_ascii_case(word));
        if p != 0 && !after_word {
            return None;
        }
        let colon = self.space_end(p);
        self.lit_byte(colon, b':')?;
        let name = colon + 1;
        if name >= self.lim()
            || self.space_at(name) > 0
            || matches!(self.at(name), b'+' | b',' | b':' | b';' | b'=')
        {
            return None;
        }
        let end = self.token_end(self.char_end(name));
        self.cap(1, colon, name);
        self.cap(2, name, end);
        Some(end)
    }
}
