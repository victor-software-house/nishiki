//! Comments, and the XML of documentation comments.

use super::super::engine::Lexer;
use super::chars::is_name_char;
use crate::rules::Chars;

/// A comment marker of the grammar.
#[derive(Clone, Copy, Debug)]
enum Marker {
    /// `///`, which no `/` may follow.
    DocLine,
    /// `//`.
    Line,
    /// `/**`, which no `/` may follow.
    DocBlock,
    /// `*/`.
    BlockClose,
}

impl Marker {
    fn text(self) -> &'static [u8] {
        match self {
            Marker::DocLine => b"///",
            Marker::Line => b"//",
            Marker::DocBlock => b"/**",
            Marker::BlockClose => b"*/",
        }
    }

    /// Whether the grammar's `(?!/)` follows the marker.
    fn refuses_slash(self) -> bool {
        match self {
            Marker::DocLine | Marker::DocBlock => true,
            Marker::Line | Marker::BlockClose => false,
        }
    }
}

impl Lexer<'_> {
    /// The marker at `at`, with no `/` after it when the marker refuses one:
    /// the end of the marker.
    fn marker_at(&self, at: usize, marker: Marker) -> Option<usize> {
        let end = self.lit(at, marker.text())?;
        (!marker.refuses_slash() || self.at(end) != b'/' || end >= self.lim()).then_some(end)
    }

    /// `(^\s+)?(marker)`: a comment's opening at `p`. Group 1 is the blanks
    /// that lead the line, group 2 the marker. The end of the marker.
    fn comment_open(&mut self, p: usize, marker: Marker) -> Option<usize> {
        if p == 0 {
            let blanks = self.blanks1(0);
            if let Some(after) = blanks
                && let Some(end) = self.marker_at(after, marker)
            {
                self.cap(1, 0, after);
                self.cap(2, after, end);
                return Some(end);
            }
        }
        let end = self.marker_at(p, marker)?;
        self.cap(2, p, end);
        Some(end)
    }

    /// `(^\s+)?(///)(?!/)`
    pub(in crate::csharp) fn doc_line_open(&mut self, p: usize) -> Option<usize> {
        self.comment_open(p, Marker::DocLine)
    }

    /// `^(\s*)(marker)`: a line that continues a run of comment lines. The
    /// end of the marker.
    fn comment_line(&mut self, p: usize, marker: Marker) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let after = self.blanks(0);
        let end = self.marker_at(after, marker)?;
        if after > 0 {
            self.cap(1, 0, after);
        }
        self.cap(2, after, end);
        Some(end)
    }

    /// `^(\s*)(///)(?!/)`
    pub(in crate::csharp) fn doc_line_continues(&mut self, p: usize) -> Option<usize> {
        self.comment_line(p, Marker::DocLine)
    }

    /// `(^\s+)?(//).*$`
    pub(in crate::csharp) fn line_comment_open(&mut self, p: usize) -> Option<usize> {
        self.comment_open(p, Marker::Line)?;
        Some(self.lim())
    }

    /// `^(\s*)(//).*$`
    pub(in crate::csharp) fn line_comment_continues(&mut self, p: usize) -> Option<usize> {
        self.comment_line(p, Marker::Line)?;
        Some(self.lim())
    }

    /// `(^\s+)?(/\*\*)(?!/)`
    pub(in crate::csharp) fn doc_block_open(&mut self, p: usize) -> Option<usize> {
        self.comment_open(p, Marker::DocBlock)
    }

    /// `(^\s+)?(\*/)`
    pub(in crate::csharp) fn doc_block_close(&mut self, p: usize) -> Option<usize> {
        self.comment_open(p, Marker::BlockClose)
    }

    /// Whether the text from `p` to the end of the line lacks `*/`: the
    /// grammar's `(?~\*/)$`.
    fn no_block_end(&self, p: usize) -> bool {
        let rest = self.text(p, self.lim());
        let found = memchr::memmem::find(rest, b"*/");
        self.spend(found.unwrap_or(rest.len()));
        found.is_none()
    }

    /// `\G(?=(?~\*/)$)`
    pub(in crate::csharp) fn doc_block_body_open(&self, p: usize) -> Option<usize> {
        (self.at_g(p) && self.no_block_end(p)).then_some(p)
    }

    /// `^(\s*+)(\*(?!/))?(?=(?~\*/)$)`: the end of the prefix, which may be
    /// empty.
    pub(in crate::csharp) fn doc_block_line_continues(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let after = self.blanks(0);
        let star = self.at(after) == b'*' && after < self.lim() && self.at(after + 1) != b'/';
        let end = if star && self.no_block_end(after + 1) {
            self.cap(2, after, after + 1);
            after + 1
        } else if self.no_block_end(after) {
            after
        } else {
            return None;
        };
        if after > 0 {
            self.cap(1, 0, after);
        }
        Some(end)
    }

    /// `(&)([:_[:alpha:]][-.:_[:alnum:]]*|#\d+|#x\h+)(;)`
    pub(in crate::csharp) fn xml_character_entity(&mut self, p: usize) -> Option<usize> {
        let amp = self.lit(p, b"&")?;
        let end = if let Some(first) = self
            .at(amp)
            .eq(&b':')
            .then_some(amp + 1)
            .or_else(|| self.alpha_at(amp))
        {
            let mut at = first;
            while let Some((c, next)) = self.cp(at) {
                if c == '-' || c == '.' || c == ':' || super::chars::is_alnum(c) {
                    at = next;
                } else {
                    break;
                }
            }
            at
        } else if self.at(amp) == b'#' && amp < self.lim() {
            let digits = self.skip(amp + 1, |b| b.is_ascii_digit());
            if digits > amp + 1 {
                digits
            } else if self.at(amp + 1) == b'x' && amp + 1 < self.lim() {
                let hex = self.skip(amp + 2, |b| b.is_ascii_hexdigit());
                if hex > amp + 2 {
                    hex
                } else {
                    return None;
                }
            } else {
                return None;
            }
        } else {
            return None;
        };
        let semi = self.lit(end, b";")?;
        self.cap(1, p, amp);
        self.cap(3, end, semi);
        Some(semi)
    }

    /// `[-_[:alnum:]]+` from `p`: the end of the run.
    fn xml_name(&self, mut p: usize) -> Option<usize> {
        let from = p;
        while let Some((c, next)) = self.cp(p) {
            if is_name_char(c) {
                p = next;
            } else {
                break;
            }
        }
        (p > from).then_some(p)
    }

    /// `((?:([-_[:alnum:]]+)(:))?([-_[:alnum:]]+))` at `p`: the end, and
    /// capture groups `base`, `base + 1`, `base + 2` and `base + 3` for the
    /// whole name, its namespace, the colon and its local name.
    fn xml_qualified_name(&mut self, p: usize, base: usize) -> Option<usize> {
        if let Some(ns_end) = self.xml_name(p)
            && self.at(ns_end) == b':'
            && ns_end < self.lim()
            && let Some(local_end) = self.xml_name(ns_end + 1)
        {
            self.cap(base, p, local_end);
            self.cap(base + 1, p, ns_end);
            self.cap(base + 2, ns_end, ns_end + 1);
            self.cap(base + 3, ns_end + 1, local_end);
            return Some(local_end);
        }
        let end = self.xml_name(p)?;
        self.cap(base, p, end);
        self.cap(base + 3, p, end);
        Some(end)
    }

    /// `(</?)((?:([-_[:alnum:]]+)(:))?([-_[:alnum:]]+))`
    ///
    /// Group 2 is the whole name, 3 the namespace, 4 the colon and 5 the local
    /// name, as `xml_qualified_name` numbers them from 2.
    pub(in crate::csharp) fn xml_tag_open(&mut self, p: usize) -> Option<usize> {
        let lt = self.lit(p, b"<")?;
        let slash = self.lit(lt, b"/").unwrap_or(lt);
        let end = self.xml_qualified_name(slash, 2)?;
        self.cap(1, p, slash);
        Some(end)
    }

    /// `(/?>)`
    pub(in crate::csharp) fn xml_tag_close(&mut self, p: usize) -> Option<usize> {
        let slash = self.lit(p, b"/").unwrap_or(p);
        let end = self.lit(slash, b">")?;
        self.cap(1, p, end);
        Some(end)
    }

    /// `(?:^|\s+)((?:([-_[:alnum:]]+)(:))?([-_[:alnum:]]+))(=)`
    pub(in crate::csharp) fn xml_attribute(&mut self, p: usize) -> Option<usize> {
        let at_start = |lexer: &mut Self, at: usize| -> Option<usize> {
            let name_end = lexer.xml_qualified_name(at, 1)?;
            let eq = lexer.lit(name_end, b"=")?;
            lexer.cap(5, name_end, eq);
            Some(eq)
        };
        if p == 0
            && let Some(end) = at_start(self, 0)
        {
            return Some(end);
        }
        let name = self.blanks1(p)?;
        at_start(self, name)
    }
}
