//! Roxygen comments: their lead, their tags, their Markdown, their LaTeX and
//! their `@examples` blocks.

use super::super::engine::Lexer;
use crate::rules::Chars;

impl Lexer<'_> {
    /// The end of `#+'` at `i`: a run of `#` and a `'`.
    /// The run gives nothing back, since a `'` must follow it.
    fn hashes_quote(&self, i: usize) -> Option<usize> {
        let end = self.skip(i, |b| b == b'#');
        (end > i && self.at(end) == b'\'').then_some(end + 1)
    }

    /// `^(\s*#+')`, recorded as `group`, or `^\s*#+'` when `group` is 0.
    /// The blanks give nothing back, since a `#` must follow them.
    pub(super) fn roxygen_lead(&mut self, p: usize, group: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let end = self.hashes_quote(self.space_end(0))?;
        if group > 0 {
            self.cap(group, 0, end);
        }
        Some(end)
    }

    /// `^(\s*#+')\s*(?:(@examples)\s*|(@examplesIf)\s+(.*))$`: the line that
    /// opens an `@examples` block.
    ///
    /// `@examples` must end the line, apart from blanks, so `@examplesIf`
    /// fails the first branch.
    /// The blanks before `$` take the line feed, as `\s*` does.
    pub(super) fn examples(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let lead = self.hashes_quote(self.space_end(0))?;
        let tag = self.space_end(lead);
        let name = self.lit(tag, b"@examples")?;
        let blanks = self.space_end(name);
        if self.at_eol(blanks) {
            self.cap(1, 0, lead);
            self.cap(2, tag, name);
            return Some(blanks);
        }
        let name = self.lit(tag, b"@examplesIf")?;
        let blanks = self.space_end(name);
        if blanks == name {
            return None;
        }
        let end = blanks.max(self.lim());
        self.cap(1, 0, lead);
        self.cap(3, tag, name);
        self.cap(4, blanks, end);
        Some(end)
    }

    /// `^(?:\s*(?=#+'\s*@)|\s*(?!#+'))`: the start of a line that ends an
    /// `@examples` block.
    ///
    /// A line that goes on with `#'` and a tag ends it before the tag.
    /// A line that does not start `#'` ends it.
    /// An indented `#'` line ends it too, one blank short of the `#`: the
    /// second branch gives a blank back, and the `#` then fails `(?!#+')`.
    pub(super) fn examples_end(&self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let blanks = self.space_end(0);
        let Some(lead) = self.hashes_quote(blanks) else {
            return Some(blanks);
        };
        if self.at(self.space_end(lead)) == b'@' {
            return Some(blanks);
        }
        (blanks > 0).then(|| self.char_start_before(blanks))
    }

    /// `` (`{3,})\s*(.*) ``: a fence in a roxygen comment.
    /// The fence's end repeats its backticks.
    /// The blanks take the line feed of a line that ends after them.
    /// The run of backticks gives nothing back: `\s*(.*)` matches after any
    /// run.
    pub(super) fn fence(&mut self, p: usize) -> Option<usize> {
        let ticks = self.skip(p, |b| b == b'`');
        if ticks - p < 3 {
            return None;
        }
        let rest = self.space_end(ticks);
        let end = rest.max(self.lim());
        self.cap(1, p, ticks);
        self.cap(2, rest, end);
        Some(end)
    }

    /// `(\1)`: the fence's backticks again.
    pub(super) fn fence_end(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, self.payload_text().as_bytes())?;
        self.cap(1, p, end);
        Some(end)
    }

    /// `(\[)(?:(\w+)(:{2,3}))?(\w+)(\(\))?(])`: a link to a function, such as
    /// `[pkg::f()]`.
    ///
    /// The package and its colons are tried first.
    /// `:{2,3}` takes three colons, or two when a third is missing.
    /// With four or more, neither leaves a word after the colons.
    pub(super) fn markdown_link(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'[' {
            return None;
        }
        let first = self.word_end(p + 1);
        if first > p + 1 && self.at(first) == b':' {
            let colons = self.skip(first, |b| b == b':') - first;
            for take in (2..=colons.min(3)).rev() {
                let word = first + take;
                let end = self.word_end(word);
                if end > word
                    && let Some(close) = self.link_close(end)
                {
                    self.cap(2, p + 1, first);
                    self.cap(3, first, word);
                    return Some(self.link_caps(p, word, end, close));
                }
            }
        }
        if first == p + 1 {
            return None;
        }
        let close = self.link_close(first)?;
        Some(self.link_caps(p, p + 1, first, close))
    }

    /// `(\(\))?(])` at `i`: where the `]` stands, with or without the `()`
    /// before it.
    fn link_close(&self, i: usize) -> Option<usize> {
        if self.starts(i, b"()]") {
            return Some(i + 2);
        }
        (self.at(i) == b']').then_some(i)
    }

    /// Records the link's bracket, name, parentheses and closing bracket,
    /// and returns its end.
    fn link_caps(&mut self, open: usize, word: usize, end: usize, close: usize) -> usize {
        self.cap(1, open, open + 1);
        self.cap(4, word, end);
        if close > end {
            self.cap(5, end, close);
        }
        self.cap(6, close, close + 1);
        close + 1
    }

    /// `(\s+|^)`: the blanks before `p`'s emphasis, or the line's start.
    /// The blanks give nothing back, since the emphasis starts with `_`, which
    /// they cannot hold.
    /// So the start of the line is tried only where no blank stands.
    fn emphasis_lead(&self, p: usize) -> Option<usize> {
        let blanks = self.space_end(p);
        if blanks > p {
            return Some(blanks);
        }
        (p == 0).then_some(0)
    }

    /// `(\s+|^)(__.+?__)\b`: bold words.
    /// The lazy `.+?` stops at the first `__` with a boundary after it.
    pub(super) fn markdown_bold_words(&mut self, p: usize) -> Option<usize> {
        let open = self.emphasis_lead(p)?;
        if !self.starts(open, b"__") || open + 2 >= self.lim() {
            return None;
        }
        let mut at = self.char_end(open + 2);
        while at + 1 < self.lim() {
            if self.starts(at, b"__") && self.boundary(at + 2) {
                self.spend(at - open);
                return Some(at + 2);
            }
            at += 1;
        }
        self.spend(at - open);
        None
    }

    /// `(\s+|^)(_(?=[^_])(?:\\.|[^\\_])*?_)\b`: italic words.
    ///
    /// The lazy repetition tries the closing `_` first at each place.
    /// A `_` that no boundary follows cannot close, and no unit can take it,
    /// so the match fails there.
    pub(super) fn markdown_italic_words(&mut self, p: usize) -> Option<usize> {
        let open = self.emphasis_lead(p)?;
        if self.at(open) != b'_' || self.at(open + 1) == b'_' || !self.avail(open + 1) {
            return None;
        }
        let mut at = open + 1;
        loop {
            let b = self.at(at);
            if !self.avail(at) {
                break;
            }
            if b == b'_' {
                self.spend(at - open);
                return self.boundary(at + 1).then_some(at + 1);
            }
            at = self.emphasis_unit(at, b'_')?;
        }
        self.spend(at - open);
        None
    }

    /// One unit of `(?:\\.|[^\\X])`, where `stop` is X.
    /// It is a backslash and a character other than the line feed.
    /// Or it is any character but a backslash and `stop`, the line feed
    /// included.
    fn emphasis_unit(&self, at: usize, stop: u8) -> Option<usize> {
        let b = self.at(at);
        if b == b'\\' {
            return (at + 1 < self.lim()).then(|| self.char_end(at + 1));
        }
        if b == stop {
            return None;
        }
        Some(if at < self.lim() {
            self.char_end(at)
        } else {
            at + 1
        })
    }

    /// `(\*\*.+?\*\*)`: bold text, to the first `**` after one character.
    pub(super) fn markdown_bold(&self, p: usize) -> Option<usize> {
        if !self.starts(p, b"**") || p + 2 >= self.lim() {
            return None;
        }
        let from = self.char_end(p + 2);
        let close = memchr::memmem::find(self.text(from, self.lim()), b"**")?;
        self.spend(close);
        Some(from + close + 2)
    }

    /// `(\*(?=[^*\s])(?:\\.|[^*\\])*?\*)`: italic text.
    /// The lazy repetition stops at the first `*` that no backslash escapes.
    pub(super) fn markdown_italic(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'*'
            || self.at(p + 1) == b'*'
            || self.space_at(p + 1) > 0
            || p + 1 >= self.lim()
        {
            return None;
        }
        let mut at = p + 1;
        while self.avail(at) {
            if self.at(at) == b'*' {
                self.spend(at - p);
                return Some(at + 1);
            }
            at = self.emphasis_unit(at, b'*')?;
        }
        self.spend(at - p);
        None
    }

    /// `` (`(?:[^\\`]|\\.)*`) ``: inline code.
    ///
    /// The units part the text one way only, since a backslash starts the
    /// escape and nothing else.
    /// So the repetition stops at the first backtick that no backslash
    /// escapes, and giving units back finds no other.
    pub(super) fn markdown_code(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'`' {
            return None;
        }
        let mut at = p + 1;
        while self.avail(at) {
            if self.at(at) == b'`' {
                self.spend(at - p);
                return Some(at + 1);
            }
            at = self.emphasis_unit(at, b'`')?;
        }
        self.spend(at - p);
        None
    }

    /// `(<)([^>]*)(>)`: a link in angle brackets, to the first `>`.
    pub(super) fn markdown_angle(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'<' {
            return None;
        }
        let close = p + 1 + memchr::memchr(b'>', self.text(p + 1, self.lim()))?;
        self.spend(close - p);
        self.cap(1, p, p + 1);
        self.cap(2, p + 1, close);
        self.cap(3, close, close + 1);
        Some(close + 1)
    }

    /// `(@(?:param|field|slot))\s*`: a tag that names parameters.
    /// No boundary follows the tag, so `@parameters` starts with one.
    pub(super) fn param_tag(&mut self, p: usize) -> Option<usize> {
        let tag = self.lit(p, b"@")?;
        let name = [b"param".as_slice(), b"field", b"slot"]
            .into_iter()
            .find_map(|word| self.lit(tag, word))?;
        self.cap(1, p, name);
        Some(self.space_end(name))
    }

    /// `\s|$`: a blank, the line feed included, or the end of the line.
    pub(super) fn param_end(&self, p: usize) -> Option<usize> {
        let width = self.space_at(p);
        if width > 0 {
            return Some(p + width);
        }
        self.at_eol(p).then_some(p)
    }

    /// `@(?!@)\w*`: a tag.
    pub(super) fn tag(&self, p: usize) -> Option<usize> {
        (self.at(p) == b'@' && self.at(p + 1) != b'@').then(|| self.word_end(p + 1))
    }

    /// `\\\w+`: a LaTeX command.
    pub(super) fn latex(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'\\' {
            return None;
        }
        let end = self.word_end(p + 1);
        (end > p + 1).then_some(end)
    }
}
