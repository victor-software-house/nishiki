//! Comments and POD: `#` comments, the `=pod` … `=cut` blocks, their
//! commands and their formatting codes, such as `B<bold>`.

use super::super::engine::Lexer;
use crate::rules::Chars;

/// The POD commands that `^(=(?:head[1-4]|item|over|encoding|begin|end|for))\b`
/// names, in the grammar's order, after `head[1-4]`.
const COMMANDS: [&[u8]; 6] = [b"item", b"over", b"encoding", b"begin", b"end", b"for"];

/// The start of the special blocks that
/// `^\s*(BEGIN|UNITCHECK|CHECK|INIT|END|DESTROY)\b` names, in the grammar's
/// order.
const BLOCKS: [&[u8]; 6] = [
    b"BEGIN",
    b"UNITCHECK",
    b"CHECK",
    b"INIT",
    b"END",
    b"DESTROY",
];

impl Lexer<'_> {
    /// `(^[\t ]+)?(?=#)`: before a `#`, with the blanks that lead a line.
    ///
    /// The blanks give characters back only to a `#`, which they cannot
    /// hold.
    pub(super) fn comment_lead(&mut self, p: usize) -> Option<usize> {
        if p == 0 {
            let blanks = self.skip(0, |b| b == b' ' || b == b'\t');
            if blanks > 0 && self.at(blanks) == b'#' && blanks < self.lim() {
                self.capture(1, 0, blanks);
                return Some(blanks);
            }
        }
        (self.at(p) == b'#' && p < self.lim()).then_some(p)
    }

    /// `^(?==[A-Za-z]+)`: a line that starts a POD block.
    pub(super) fn pod_start(&self, p: usize) -> Option<usize> {
        (p == 0 && self.at(0) == b'=' && self.at(1).is_ascii_alphabetic() && self.lim() > 1)
            .then_some(0)
    }

    /// `^(=cut\b.*)$`: the line that ends a POD block.
    pub(super) fn pod_cut(&mut self, p: usize) -> Option<usize> {
        if p != 0 || !self.starts(0, b"=cut") || self.word_at(4).is_some() {
            return None;
        }
        let lim = self.lim();
        self.capture(1, 0, lim);
        Some(lim)
    }

    /// `^=(pod|back|cut)\b`: a POD command alone.
    pub(super) fn pod_command(&self, p: usize) -> Option<usize> {
        if p != 0 || self.at(0) != b'=' || self.lim() == 0 {
            return None;
        }
        [b"pod".as_slice(), b"back", b"cut"]
            .into_iter()
            .find_map(|word| self.lit(1, word).filter(|&end| self.word_at(end).is_none()))
    }

    /// `^(=begin)\s+(html)\s*$`: the start of a POD block of HTML.
    /// The blanks after `html` take the line feed, as `\s*` does before
    /// `$`.
    pub(super) fn pod_html_begin(&mut self, p: usize) -> Option<usize> {
        if p != 0 || !self.starts(0, b"=begin") {
            return None;
        }
        let at = self.space_end(6);
        if at == 6 {
            return None;
        }
        let html = self.lit(at, b"html")?;
        let end = self.space_end(html);
        self.at_eol(end).then(|| {
            self.capture(1, 0, 6);
            self.capture(2, at, html);
            end
        })
    }

    /// `^(?:(=end)\s+(html)|(?==cut))`: the end of a POD block of HTML.
    pub(super) fn pod_html_end(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        if self.starts(0, b"=end") {
            let at = self.space_end(4);
            if at > 4
                && let Some(html) = self.lit(at, b"html")
            {
                self.capture(1, 0, 4);
                self.capture(2, at, html);
                return Some(html);
            }
        }
        self.starts(0, b"=cut").then_some(0)
    }

    /// `^(=(?:head[1-4]|item|over|encoding|begin|end|for))\b\s*(.*)`: a POD
    /// command and the rest of its line.
    /// `\s*` takes the line feed of a line that ends after the command.
    /// Then `.*` is empty after it.
    pub(super) fn pod_heading(&mut self, p: usize) -> Option<usize> {
        if p != 0 || self.at(0) != b'=' || self.lim() == 0 {
            return None;
        }
        let head = self
            .lit(1, b"head")
            .filter(|&end| matches!(self.at(end), b'1'..=b'4') && end < self.lim())
            .map(|end| end + 1);
        let command = head
            .into_iter()
            .chain(COMMANDS.iter().filter_map(|word| self.lit(1, word)))
            .find(|&end| self.word_at(end).is_none())?;
        let rest = self.space_end(command);
        let end = rest.max(self.lim());
        self.capture(1, 0, command);
        self.capture(2, rest, end);
        Some(end)
    }

    /// `I(?:<([^<>]+)>|<+(\s+(?:(?<!\s)>|[^>])+\s+)>+)` and the same with `B`:
    /// italic or bold text.
    pub(super) fn pod_markup(&mut self, p: usize, letter: u8) -> Option<usize> {
        if self.at(p) != letter || self.at(p + 1) != b'<' || p + 1 >= self.lim() {
            return None;
        }
        if let Some(end) = self.pod_plain(p + 2) {
            return Some(end);
        }
        self.pod_spaced(p + 1)
    }

    /// `<([^<>]+)>` from the text after the `<` at `p`: text without angle
    /// brackets, to the first `>`.
    fn pod_plain(&mut self, p: usize) -> Option<usize> {
        let end = self.skip(p, |b| b != b'<' && b != b'>');
        (end > p && self.at(end) == b'>' && end < self.lim()).then(|| {
            self.capture(1, p, end);
            end + 1
        })
    }

    /// `<+(\s+(?:(?<!\s)>|[^>])+\s+)>+` from the `<` at `p`: text set off by
    /// blanks inside a run of angle brackets.
    ///
    /// The `<` run gives nothing back, since a blank must follow it.
    /// The text stops at the first `>` after a blank, which it cannot take.
    /// It gives characters back from there, so that one blank closes it
    /// before that `>`.
    /// So the text needs a blank, then at least one character, then the blank
    /// before the `>`.
    fn pod_spaced(&mut self, p: usize) -> Option<usize> {
        let open = self.skip(p, |b| b == b'<');
        if !self.is_space_at(open) {
            return None;
        }
        let mut close = open + 1;
        loop {
            if close > self.lim() {
                return None;
            }
            if self.at(close) == b'>' && close < self.lim() && self.is_space_at(close - 1) {
                break;
            }
            close += 1;
        }
        self.spend(close - open);
        if close - open < 3 {
            return None;
        }
        let end = self.skip(close, |b| b == b'>');
        self.capture(2, open, close);
        Some(end)
    }

    /// `C(?:<([^<>]+)>|<+(\\s+(?:(?<!\\s)>|[^>])+\\s+)>+)`: code.
    /// The grammar writes `\\s`, a backslash and an `s`, where it means a
    /// blank.
    /// So the second form sets the text off with `\s` and `\sss`, literally.
    pub(super) fn pod_code(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'C' || self.at(p + 1) != b'<' || p + 1 >= self.lim() {
            return None;
        }
        if let Some(end) = self.pod_plain(p + 2) {
            return Some(end);
        }
        let open = self.skip(p + 1, |b| b == b'<');
        if !self.starts(open, b"\\s") {
            return None;
        }
        let mut close = open + 2;
        loop {
            if close >= self.lim() {
                return None;
            }
            if self.at(close) == b'>' && self.starts(close - 2, b"\\s") {
                break;
            }
            close += 1;
        }
        self.spend(close - open);
        if close - open < 5 {
            return None;
        }
        let end = self.skip(close, |b| b == b'>');
        self.capture(2, open, close);
        Some(end)
    }

    /// `L<([^>]+)>`: a link.
    pub(super) fn pod_link(&mut self, p: usize) -> Option<usize> {
        if !self.starts(p, b"L<") {
            return None;
        }
        let end = self.skip(p + 2, |b| b != b'>');
        (end > p + 2 && self.at(end) == b'>' && end < self.lim()).then(|| {
            self.capture(1, p + 2, end);
            end + 1
        })
    }

    /// `[EFSXZ]<[^>]*>`: an escape, a file name, unbroken text, an index
    /// entry or a zero-width code.
    pub(super) fn pod_other(&self, p: usize) -> Option<usize> {
        if !matches!(self.at(p), b'E' | b'F' | b'S' | b'X' | b'Z')
            || self.at(p + 1) != b'<'
            || p + 1 >= self.lim()
        {
            return None;
        }
        let end = self.skip(p + 2, |b| b != b'>');
        (self.at(end) == b'>' && end < self.lim()).then_some(end + 1)
    }

    /// `^\s*(BEGIN|UNITCHECK|CHECK|INIT|END|DESTROY)\b`: a special block.
    pub(super) fn special_block(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let at = self.space_end(0);
        let end = BLOCKS
            .iter()
            .filter_map(|word| self.lit(at, word))
            .find(|&end| self.word_at(end).is_none())?;
        self.capture(1, at, end);
        Some(end)
    }
}
