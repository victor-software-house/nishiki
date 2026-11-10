//! Comments, embedded documents, `__END__` and the YARD tags in comments.

use super::super::engine::Lexer;
use crate::rules::Chars;

/// The tags of `^(\s*)(#)(\s*)(@)(…)(?=\s|$)`.
const YARD_COMMENT_TAGS: &[&[u8]] = &[
    b"abstract",
    b"api",
    b"author",
    b"deprecated",
    b"example",
    b"macro",
    b"note",
    b"overload",
    b"since",
    b"todo",
    b"version",
];

/// The tags whose name and types follow them, in either order.
const YARD_PARAM_TAGS: &[&[u8]] = &[
    b"attr",
    b"attr_reader",
    b"attr_writer",
    b"yieldparam",
    b"param",
];

/// The tags whose types follow them.
const YARD_TYPE_TAGS: &[&[u8]] = &[b"raise", b"return", b"yield", b"yieldreturn"];

/// The directives written `@!`.
const YARD_DIRECTIVES: &[&[u8]] = &[
    b"endgroup",
    b"group",
    b"method",
    b"parse",
    b"scope",
    b"visibility",
];

/// The directives written `@!` that a name may follow.
const YARD_MACRO_DIRECTIVES: &[&[u8]] = &[b"attribute", b"macro"];

/// The mark a YARD tag starts with.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mark {
    /// `@`, a tag.
    Tag,
    /// `@!`, a directive.
    Directive,
}

impl Lexer<'_> {
    /// `^=begin` or `^=end`: `word` at the start of a line.
    pub(in crate::ruby) fn line_start_word(&mut self, p: usize, word: &[u8]) -> Option<usize> {
        (p == 0).then(|| self.lit(p, word)).flatten()
    }

    /// `(^[\t ]+)?(?=#)`: where a comment starts, with the indentation before
    /// it at the start of a line.
    pub(in crate::ruby) fn comment_start(&mut self, p: usize) -> Option<usize> {
        if p == 0 {
            let hash = self.skip(p, |b| b == b'\t' || b == b' ');
            if hash > p && self.byte_in(hash, |b| b == b'#') {
                self.capture(1, p, hash);
                return Some(hash);
            }
        }
        self.byte_in(p, |b| b == b'#').then_some(p)
    }

    /// `(?!\G)`
    pub(in crate::ruby) fn away_from_anchor(&mut self, p: usize) -> Option<usize> {
        (!self.at_g(p)).then_some(p)
    }

    /// `^__END__\n`
    pub(in crate::ruby) fn data_marker(&mut self, p: usize) -> Option<usize> {
        let end = self.line_start_word(p, b"__END__")?;
        (end == self.lim() && self.nl).then_some(end + 1)
    }

    /// `(?=<?xml|<(?i:html\b)|!DOCTYPE (?i:html\b))`: where HTML starts after
    /// `__END__`.
    pub(in crate::ruby) fn data_html(&mut self, p: usize) -> Option<usize> {
        let html = |at: usize| {
            self.text(at, at + 4).eq_ignore_ascii_case(b"html") && self.word_at(at + 4).is_none()
        };
        let xml = self.starts(p, b"xml") || self.starts(p, b"<xml");
        let tag = self.byte_in(p, |b| b == b'<') && html(p + 1);
        let doctype = self.starts(p, b"!DOCTYPE ") && html(p + 9);
        (xml || tag || doctype).then_some(p)
    }

    /// `^(\s*)(#)(\s*)(@)` or `(@!)` at `p`: where the tag's name starts.
    ///
    /// Group 3 holds the `#` with the blanks after it, though the grammar's
    /// group holds only the blanks.
    /// The end repeats it as `#\3`.
    /// A payload that is never empty can then show whether the line repeats
    /// it.
    fn yard_head(&mut self, p: usize, mark: Mark) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let hash = self.space_end(p);
        if !self.byte_in(hash, |b| b == b'#') {
            return None;
        }
        let at = self.space_end(hash + 1);
        let marks: &[u8] = match mark {
            Mark::Tag => b"@",
            Mark::Directive => b"@!",
        };
        let name = self.lit(at, marks)?;
        self.capture(2, hash, hash + 1);
        self.capture(3, hash, at);
        self.capture(4, at, name);
        Some(name)
    }

    /// The end of the tag's name at `p` when it is one of `names`, recorded as
    /// group 5.
    /// A name ends at a character that is not `\w`, since each name is
    /// followed by a blank or the end of the line.
    fn yard_name(&mut self, p: usize, names: &[&[u8]]) -> Option<usize> {
        let end = self.word_end(p);
        names.contains(&self.text(p, end)).then(|| {
            self.capture(5, p, end);
            end
        })
    }

    /// `(?=\s)` at `p`.
    fn blank_follows(&self, p: usize) -> bool {
        self.space_at(p) > 0
    }

    /// The end of `[_a-z]\w*:?` at `p`.
    fn yard_param_end(&self, p: usize) -> Option<usize> {
        if !self.byte_in(p, |b| b.is_ascii_lowercase() || b == b'_') {
            return None;
        }
        let end = self.word_end(p + 1);
        Some(end + usize::from(self.byte_in(end, |b| b == b':')))
    }

    /// The ends of the closing bytes of `open.+close` at `p`, the last first.
    /// `.+` is greedy.
    /// Each `close` after at least one character is therefore a candidate,
    /// from the end of the line back.
    fn yard_bracketed(&self, p: usize, open: u8, close: u8) -> impl Iterator<Item = usize> + '_ {
        let opened = self.byte_in(p, |b| b == open);
        (p + 2..self.lim())
            .rev()
            .filter(move |&at| opened && self.at(at) == close)
    }

    /// The last of [`Lexer::yard_bracketed`]: where a greedy `open.+close`
    /// closes.
    fn last_bracketed(&self, p: usize, open: u8, close: u8) -> Option<usize> {
        self.yard_bracketed(p, open, close).next()
    }

    /// Records `(open.+close)` at `p` as groups `whole`, `whole + 1` and
    /// `whole + 2`, ending at `close_at`.
    fn record_bracketed(&mut self, whole: usize, p: usize, close_at: usize) -> usize {
        self.capture(whole, p, close_at + 1);
        self.capture(whole + 1, p, p + 1);
        self.capture(whole + 2, close_at, close_at + 1);
        close_at + 1
    }

    /// `(?>\s+(?>([_a-z]\w*:?)|(…)))?` at `p` and its mirror, with the types
    /// first when `types_first`.
    /// It returns the end of the group.
    /// It records the name as `name` and the types from `types`.
    fn yard_name_or_types(
        &mut self,
        p: usize,
        types_first: bool,
        name: usize,
        types: usize,
    ) -> usize {
        let start = self.space_end(p);
        if start == p {
            return p;
        }
        let param = |lx: &mut Self| {
            let end = lx.yard_param_end(start)?;
            lx.capture(name, start, end);
            Some(end)
        };
        let bracketed = |lx: &mut Self| {
            let close = lx.last_bracketed(start, b'[', b']')?;
            Some(lx.record_bracketed(types, start, close))
        };
        let found = if types_first {
            bracketed(self).or_else(|| param(self))
        } else {
            param(self).or_else(|| bracketed(self))
        };
        found.unwrap_or(p)
    }

    /// `^(\s*)(#)(\s*)(@)(abstract|…|version)(?=\s|$)`
    pub(in crate::ruby) fn yard_comment(&mut self, p: usize) -> Option<usize> {
        let name = self.yard_head(p, Mark::Tag)?;
        let end = self.yard_name(name, YARD_COMMENT_TAGS)?;
        (self.blank_follows(end) || self.at_eol(end)).then_some(end)
    }

    /// `^(\s*)(#)(\s*)(@)(private)$`
    pub(in crate::ruby) fn yard_private(&mut self, p: usize) -> Option<usize> {
        let name = self.yard_head(p, Mark::Tag)?;
        let end = self.yard_name(name, &[b"private"])?;
        self.at_eol(end).then_some(end)
    }

    /// `^(\s*)(#)(\s*)(@)(attr|…|param)(?=\s)` and then a name and types in
    /// either order, each atomic and optional.
    pub(in crate::ruby) fn yard_param_types(&mut self, p: usize) -> Option<usize> {
        let name = self.yard_head(p, Mark::Tag)?;
        let end = self.yard_name(name, YARD_PARAM_TAGS)?;
        if !self.blank_follows(end) {
            return None;
        }
        let first = self.yard_name_or_types(end, false, 6, 7);
        Some(self.yard_name_or_types(first, true, 13, 10))
    }

    /// `^(\s*)(#)(\s*)(@)(option)(?=\s)(?>\s+([_a-z]\w*:?))?(?>\s+((\[).+(])))?
    /// (?>\s+((\S*)))?(?>\s+((\().+(\))))?`
    ///
    /// `\S*` may match nothing.
    /// Its group can then take the line feed as its blanks and end after it.
    pub(in crate::ruby) fn yard_option(&mut self, p: usize) -> Option<usize> {
        let name = self.yard_head(p, Mark::Tag)?;
        let mut end = self.yard_name(name, &[b"option"])?;
        if !self.blank_follows(end) {
            return None;
        }
        let start = self.space_end(end);
        if let Some(param) = self.yard_param_end(start).filter(|_| start > end) {
            self.capture(6, start, param);
            end = param;
        }
        let start = self.space_end(end);
        if start > end
            && let Some(close) = self.last_bracketed(start, b'[', b']')
        {
            end = self.record_bracketed(7, start, close);
        }
        let start = self.space_end(end);
        if start > end {
            let key = if start < self.lim() {
                self.skip_non_space(start)
            } else {
                start
            };
            if key > start {
                self.capture(10, start, key);
                self.capture(11, start, key);
            }
            end = key;
        }
        let start = self.space_end(end);
        if start > end
            && let Some(close) = self.last_bracketed(start, b'(', b')')
        {
            end = self.record_bracketed(12, start, close);
        }
        Some(end)
    }

    /// The end of the `\S*` from `p`.
    fn skip_non_space(&self, mut p: usize) -> usize {
        while p < self.lim() && !self.space_char(p) {
            p = self.char_end(p);
        }
        p
    }

    /// `^(\s*)(#)(\s*)(@)(raise|return|yield(?:return)?)(?=\s)(\s+((\[).+(])))?
    /// `
    pub(in crate::ruby) fn yard_types(&mut self, p: usize) -> Option<usize> {
        let name = self.yard_head(p, Mark::Tag)?;
        let end = self.yard_name(name, YARD_TYPE_TAGS)?;
        if !self.blank_follows(end) {
            return None;
        }
        let start = self.space_end(end);
        match self.last_bracketed(start, b'[', b']') {
            Some(close) => Some(self.record_bracketed(7, start, close)),
            None => Some(end),
        }
    }

    /// `(\s+((\[).+(])))?(?=\s)` after a directive's name at `p`: the types
    /// end at the last `]` with a blank after it, or are left out.
    fn directive_types(&mut self, p: usize) -> Option<usize> {
        let start = self.space_end(p);
        if start > p {
            let close = self
                .yard_bracketed(start, b'[', b']')
                .find(|&close| self.blank_follows(close + 1));
            if let Some(close) = close {
                return Some(self.record_bracketed(7, start, close));
            }
        }
        self.blank_follows(p).then_some(p)
    }

    /// `^(\s*)(#)(\s*)(@!)(endgroup|…|visibility)(\s+((\[).+(])))?(?=\s)`
    pub(in crate::ruby) fn yard_directive(&mut self, p: usize) -> Option<usize> {
        let name = self.yard_head(p, Mark::Directive)?;
        let end = self.yard_name(name, YARD_DIRECTIVES)?;
        self.directive_types(end)
    }

    /// `^(\s*)(#)(\s*)(@!)(attribute|macro)(\s+((\[).+(])))?(?=\s)(\s+([_a-z]\
    /// w*:?))?`
    pub(in crate::ruby) fn yard_macro_attribute(&mut self, p: usize) -> Option<usize> {
        let name = self.yard_head(p, Mark::Directive)?;
        let end = self.yard_name(name, YARD_MACRO_DIRECTIVES)?;
        let end = self.directive_types(end)?;
        let start = self.space_end(end);
        match self.yard_param_end(start).filter(|_| start > end) {
            Some(param) => {
                self.capture(11, start, param);
                Some(param)
            }
            None => Some(end),
        }
    }

    /// `^(\s*)(#)(\s*)(@)(see)(?=\s)(\s+(.+?))?(?=\s|$)`
    ///
    /// The lazy `.+?` starts after the blanks.
    /// There it takes characters up to the next blank or the end of the line.
    /// When the blanks reach the end of the line, they give back their last
    /// character for it.
    pub(in crate::ruby) fn yard_see(&mut self, p: usize) -> Option<usize> {
        let name = self.yard_head(p, Mark::Tag)?;
        let end = self.yard_name(name, &[b"see"])?;
        if !self.blank_follows(end) {
            return None;
        }
        let blanks = self.space_end(end);
        let target = if blanks < self.lim() {
            Some(blanks)
        } else {
            Some(self.char_start_before(self.lim())).filter(|&last| last > end)
        };
        let Some(target) = target else {
            return Some(end);
        };
        let mut stop = self.char_end(target);
        while stop < self.lim() && !self.space_char(stop) {
            stop = self.char_end(stop);
        }
        self.capture(7, target, stop);
        Some(stop)
    }

    /// `^(?!\s*#\3\s{2,}|\s*#\s*$)`: the first line that does not continue the
    /// YARD tag.
    /// Group 3 is the `#` and its blanks; see [`Lexer::yard_head`].
    pub(in crate::ruby) fn yard_end(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let hash = self.space_end(p);
        if !self.byte_in(hash, |b| b == b'#') {
            return Some(p);
        }
        let indented = self.repeated_delimiter(hash).is_some_and(|after| {
            let first = self.space_at(after);
            first > 0 && self.space_at(after + first) > 0
        });
        let blank = self.space_end(hash + 1) >= self.lim();
        (!indented && !blank).then_some(p)
    }

    /// `^\s*#`
    pub(in crate::ruby) fn yard_continuation(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let hash = self.space_end(p);
        self.byte_in(hash, |b| b == b'#').then_some(hash + 1)
    }
}
