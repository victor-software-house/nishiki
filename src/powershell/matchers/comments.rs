//! Comments, the help keywords inside them and the `#requires` directive.

use super::super::engine::Lexer;
use crate::classes::{is_digit, is_letter};
use crate::rules::Chars;

/// The help keywords that stand alone on their line, in the grammar's order.
const HELP: &[&[u8]] = &[
    b"COMPONENT",
    b"DESCRIPTION",
    b"EXAMPLE",
    b"FUNCTIONALITY",
    b"INPUTS",
    b"LINK",
    b"NOTES",
    b"OUTPUTS",
    b"ROLE",
    b"SYNOPSIS",
];

/// The help keywords that take an argument, in the grammar's order.
const HELP_WITH_ARGUMENT: &[&[u8]] = &[
    b"EXTERNALHELP",
    b"FORWARDHELPCATEGORY",
    b"FORWARDHELPTARGETNAME",
    b"PARAMETER",
    b"REMOTEHELPRUNSPACE",
];

/// The parameters of `#requires`, in the grammar's order.
const REQUIRES: &[&[u8]] = &[
    b"Modules",
    b"PSSnapin",
    b"RunAsAdministrator",
    b"ShellId",
    b"Version",
    b"Assembly",
    b"PSEdition",
];

impl Lexer<'_> {
    /// ``(?<![-\\`])(#)#*``: the start of a line comment, group 1 around its
    /// first `#`.
    pub(super) fn line_comment(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'#' || matches!(self.prev(p), Some(b'-' | b'\\' | b'`')) {
            return None;
        }
        self.capture(1, p, p + 1);
        Some(self.skip(p + 1, |b| b == b'#'))
    }

    /// `$\n?`: the end of the line, with its line feed.
    pub(super) fn line_comment_end(&self, p: usize) -> Option<usize> {
        if !self.at_eol(p) {
            return None;
        }
        Some(if p == self.lim() && self.avail(p) {
            p + 1
        } else {
            p
        })
    }

    /// Where a help keyword's dot stands, when `p` starts a line or is `\G`
    /// and only blanks come before the dot: `(?:^|\G)\s*(\.)`.
    fn help_dot(&self, p: usize) -> Option<usize> {
        if p != 0 && !self.at_g(p) {
            return None;
        }
        let dot = self.space_end(p);
        (self.at(dot) == b'.').then_some(dot)
    }

    /// `(?:^|\G)(?i:\s*(\.)(COMPONENT|…|SYNOPSIS))\s*$`: a help keyword alone
    /// on its line, group 1 around the dot and group 2 around the keyword.
    ///
    /// The `\s*` may take the line feed.
    pub(super) fn help_keyword(&mut self, p: usize) -> Option<usize> {
        let dot = self.help_dot(p)?;
        let (word, end) = HELP.iter().find_map(|word| {
            let word = self.folded(dot + 1, word)?;
            let end = self.space_end(word);
            self.at_eol(end).then_some((word, end))
        })?;
        self.capture(1, dot, dot + 1);
        self.capture(2, dot + 1, word);
        Some(end)
    }

    /// `(?:^|\G)(?i:\s*(\.)(EXTERNALHELP|…|REMOTEHELPRUNSPACE))\s+(.+?)\s*$`:
    /// a help keyword and its argument, group 1 around the dot, group 2
    /// around the keyword and group 3 around the argument.
    pub(super) fn help_keyword_with_argument(&mut self, p: usize) -> Option<usize> {
        let dot = self.help_dot(p)?;
        let (word, argument) = HELP_WITH_ARGUMENT.iter().find_map(|word| {
            let word = self.folded(dot + 1, word)?;
            self.help_argument(word).map(|argument| (word, argument))
        })?;
        self.capture(1, dot, dot + 1);
        self.capture(2, dot + 1, word);
        self.capture(3, argument.0, argument.1);
        Some(self.space_end(argument.1))
    }

    /// `\s+(.+?)\s*$` after a help keyword that ends at `word`: where the
    /// argument starts and ends.
    ///
    /// The argument ends at the last character that is not a blank.
    /// When only blanks follow the keyword, the `\s+` gives back its last
    /// blank, which `.+?` takes.
    /// That needs two blanks before the end of the text.
    fn help_argument(&self, word: usize) -> Option<(usize, usize)> {
        let blanks = self.space_end(word);
        if blanks == word {
            return None;
        }
        if blanks < self.lim() {
            let mut end = self.lim();
            while self.space_at(self.char_start_before(end)) > 0 {
                end = self.char_start_before(end);
            }
            return Some((blanks, end));
        }
        let last = self.char_start_before(self.lim());
        (last > word).then_some((last, self.lim()))
    }

    /// `(?<=#)(?i:(requires))\s`: the `#requires` directive, group 1 around
    /// its word.
    pub(super) fn requires(&mut self, p: usize) -> Option<usize> {
        if !self.prev_is(p, b'#') {
            return None;
        }
        let word = self.folded(p, b"requires")?;
        let blank = self.space_at(word);
        if blank == 0 {
            return None;
        }
        self.capture(1, p, word);
        Some(word + blank)
    }

    /// `-(?i:Modules|PSSnapin|…|PSEdition)`: a parameter of `#requires`.
    pub(super) fn requires_parameter(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'-' {
            return None;
        }
        self.first_folded(p + 1, REQUIRES, |_, _| true)
    }

    /// `(?<!-)\b\p{L}+|\d+(?:\.\d+)*`: a word or a version after `#requires`.
    pub(super) fn requires_value(&self, p: usize) -> Option<usize> {
        if !self.prev_is(p, b'-') && self.boundary(p) && self.letter_at(p) {
            return Some(self.class_end(p, is_letter));
        }
        let mut end = self.class_end(p, is_digit);
        if end == p {
            return None;
        }
        while self.at(end) == b'.' && self.char_at(end + 1).is_some_and(|(c, _)| is_digit(c)) {
            end = self.class_end(end + 1, is_digit);
        }
        Some(end)
    }
}
