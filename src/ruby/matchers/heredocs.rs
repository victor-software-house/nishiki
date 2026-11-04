//! Here documents: the openers on a line and the lines that close them.

use super::super::engine::Lexer;
use crate::rules::Chars;

/// One here document's opener, ``<<[-~]?(["'`]?)([A-Z_a-z]\w*)\N``.
#[derive(Clone, Copy, Debug)]
struct Opener {
    /// Where its name starts.
    name: usize,
    /// Where its name ends.
    name_end: usize,
    /// Where the opener ends, after the closing quote.
    end: usize,
}

impl Lexer<'_> {
    /// The end of ``(["'`]?)([A-Z_a-z]\w*)\1`` at `p`: a name, quoted or not.
    ///
    /// A quote with no closing one leaves no name, since the name cannot
    /// start at the quote.
    /// A shorter name is followed by a `\w`.
    /// That is neither the quote nor what follows the opener.
    fn quoted_name(&self, p: usize) -> Option<Opener> {
        let quote = self
            .byte_in(p, |b| matches!(b, b'"' | b'\'' | b'`'))
            .then(|| self.at(p));
        let name = p + usize::from(quote.is_some());
        let name_end = self.name_end(name)?;
        let end = match quote {
            Some(q) => self.byte_in(name_end, |b| b == q).then_some(name_end + 1)?,
            None => name_end,
        };
        Some(Opener {
            name,
            name_end,
            end,
        })
    }

    /// The opener ``<<[-~]?(["'`]?)([A-Z_a-z]\w*)\N`` at `p`.
    fn heredoc_opener(&self, p: usize) -> Option<Opener> {
        let marks = self.lit(p, b"<<")?;
        let quote = marks + usize::from(self.byte_in(marks, |b| b == b'-' || b == b'~'));
        self.quoted_name(quote)
    }

    /// ``(?>=\s*<<(["'`]?)([A-Z_a-z]\w*)\1)``: a here document assigned with
    /// `=` and no `-` or `~`, whose name group 2 holds for the end.
    pub(in crate::ruby) fn assigned_heredoc(&mut self, p: usize) -> Option<usize> {
        let marks = self.space_end(self.lit(p, b"=")?);
        let opener = self.quoted_name(self.lit(marks, b"<<")?)?;
        self.cap(2, opener.name, opener.name_end);
        Some(opener.end)
    }

    /// `^\2$`: the line that is the here document's name alone.
    pub(in crate::ruby) fn assigned_heredoc_end(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let end = self.repeated_delimiter(p)?;
        self.at_eol(end).then_some(end)
    }

    /// `^<<[-~]?[A-Z_a-z]\w*`
    pub(in crate::ruby) fn heredoc_at_line_start(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let marks = self.lit(p, b"<<")?;
        let name = marks + usize::from(self.byte_in(marks, |b| b == b'-' || b == b'~'));
        self.name_end(name)
    }

    /// `$`
    pub(in crate::ruby) fn line_end(&mut self, p: usize) -> Option<usize> {
        self.at_eol(p).then_some(p)
    }

    /// ``(?>((<<…\3,\s?)*<<[-~]?(["'`]?)([A-Z_a-z]\w*)\5))(.*)``: one or more
    /// here document openers, separated by `,` and one optional blank, and
    /// the rest of the line.
    /// The last opener's name, group 6, closes them.
    ///
    /// An opener repeats only when another opener follows its `,`.
    /// Otherwise the greedy repetition gives back the last one, which is then
    /// the final opener.
    /// The atomic group keeps that first success.
    pub(in crate::ruby) fn heredoc_openers(&mut self, p: usize) -> Option<usize> {
        let mut last = self.heredoc_opener(p)?;
        while self.byte_in(last.end, |b| b == b',') {
            let comma = last.end + 1;
            let next = comma + self.space_at(comma);
            match self.heredoc_opener(next) {
                Some(opener) => last = opener,
                None => break,
            }
        }
        self.cap(1, p, last.end);
        self.cap(6, last.name, last.name_end);
        self.cap(7, last.end, self.lim());
        Some(self.lim())
    }

    /// `^\s*\6$`: the line that closes here documents.
    pub(in crate::ruby) fn heredoc_openers_end(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let name = self.space_end(p);
        let end = self.repeated_delimiter(name)?;
        self.at_eol(end).then_some(end)
    }
}
