//! The preprocessor directives: `#if`, `#define`, `#region`, `#pragma`,
//! and the `#:` directives of a file-based app.

use super::super::engine::Lexer;
use crate::rules::Chars;

impl Lexer<'_> {
    /// `^\s*(#)\s*`
    pub(in crate::csharp) fn preprocessor(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let hash = self.blanks(0);
        if self.at(hash) != b'#' || hash >= self.lim() {
            return None;
        }
        self.cap(1, hash, hash + 1);
        Some(self.blanks(hash + 1))
    }

    /// `(?<=$)`, which holds where `$` does: at the end of the line.
    pub(in crate::csharp) fn after_line_end(&self, p: usize) -> Option<usize> {
        self.at_eol(p).then_some(p)
    }

    /// `(?=$)`
    pub(in crate::csharp) fn before_line_end(&self, p: usize) -> Option<usize> {
        self.at_eol(p).then_some(p)
    }

    /// `(//).*(?=$)`
    pub(in crate::csharp) fn preprocessor_comment(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"//")?;
        self.cap(1, p, end);
        Some(self.lim())
    }

    /// `\b(?:(first)|(second))\b`, with the second alternative absent for a
    /// single word: the end of the word and which one it was.
    fn one_of_two(&mut self, p: usize, first: &[u8], second: Option<&[u8]>) -> Option<usize> {
        if let Some(end) = self.keyword(p, first) {
            self.cap(1, p, end);
            return Some(end);
        }
        let end = self.keyword(p, second?)?;
        self.cap(2, p, end);
        Some(end)
    }

    /// `\b(?:(define)|(undef))\b\s*\b([_[:alpha:]][_[:alnum:]]*)\b`
    pub(in crate::csharp) fn preprocessor_define(&mut self, p: usize) -> Option<usize> {
        let end = self.one_of_two(p, b"define", Some(b"undef"))?;
        let start = self.blanks(end);
        let name = self.plain_ident(start)?;
        if !self.boundary(start) || self.word_at(name).is_some() {
            return None;
        }
        self.cap(3, start, name);
        Some(name)
    }

    /// `\b(?:(if)|(elif))\b`
    pub(in crate::csharp) fn preprocessor_if(&mut self, p: usize) -> Option<usize> {
        self.one_of_two(p, b"if", Some(b"elif"))
    }

    /// `\b(?:(true)|(false)|([_[:alpha:]][_[:alnum:]]*))\b`
    pub(in crate::csharp) fn preprocessor_symbol(&mut self, p: usize) -> Option<usize> {
        if let Some(end) = self.keyword(p, b"true") {
            self.cap(1, p, end);
            return Some(end);
        }
        if let Some(end) = self.keyword(p, b"false") {
            self.cap(2, p, end);
            return Some(end);
        }
        let end = self.plain_ident(p)?;
        if self.word_before(p) || self.word_at(end).is_some() {
            return None;
        }
        self.cap(3, p, end);
        Some(end)
    }

    /// `([!=]=)|(!|&&|\|\|)`
    pub(in crate::csharp) fn preprocessor_operator(&mut self, p: usize) -> Option<usize> {
        if let Some(end) = self.lit(p, b"!=").or_else(|| self.lit(p, b"==")) {
            self.cap(1, p, end);
            return Some(end);
        }
        let end = self
            .lit(p, b"!")
            .or_else(|| self.lit(p, b"&&"))
            .or_else(|| self.lit(p, b"||"))?;
        self.cap(2, p, end);
        Some(end)
    }

    /// `\b(?:(else)|(endif))\b`
    pub(in crate::csharp) fn preprocessor_else(&mut self, p: usize) -> Option<usize> {
        self.one_of_two(p, b"else", Some(b"endif"))
    }

    /// The rest of the line after blanks, as capture group `group`:
    /// `\s*(.*)`, the end of the line.
    fn message(&mut self, p: usize, group: usize) -> usize {
        let start = self.blanks(p);
        let end = self.lim().max(start);
        self.cap(group, start, end);
        end
    }

    /// `\b(?:(warning)|(error))\b\s*(.*)(?=$)`
    pub(in crate::csharp) fn preprocessor_warning(&mut self, p: usize) -> Option<usize> {
        let end = self.one_of_two(p, b"warning", Some(b"error"))?;
        Some(self.message(end, 3))
    }

    /// `\b(region)\b\s*(.*)(?=$)`
    pub(in crate::csharp) fn preprocessor_region(&mut self, p: usize) -> Option<usize> {
        let end = self.keyword(p, b"region")?;
        self.cap(1, p, end);
        Some(self.message(end, 2))
    }

    /// `"[^"]*"`
    pub(in crate::csharp) fn quoted_text(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'"' || p >= self.lim() {
            return None;
        }
        self.find_byte(p + 1, b'"').map(|close| close + 1)
    }

    /// `\b(default|hidden)`
    pub(in crate::csharp) fn preprocessor_line_word(&mut self, p: usize) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        let end = self.lit(p, b"default").or_else(|| self.lit(p, b"hidden"))?;
        self.cap(1, p, end);
        Some(end)
    }

    /// `[0-9]+`
    pub(in crate::csharp) fn ascii_digits(&self, p: usize) -> Option<usize> {
        let end = self.skip(p, |b| b.is_ascii_digit());
        (end > p).then_some(end)
    }

    /// `\b(pragma)\b\s*\b(warning)\b\s*\b(?:(disable)|(restore))\b(\s*[0-9]+(?:
    /// \s*,\s*[0-9]+)?)?`
    pub(in crate::csharp) fn pragma_warning(&mut self, p: usize) -> Option<usize> {
        let pragma_end = self.keyword(p, b"pragma")?;
        let warning_start = self.blanks(pragma_end);
        let warning_end = self.keyword(warning_start, b"warning")?;
        let action_start = self.blanks(warning_end);
        let action_end = self
            .keyword(action_start, b"disable")
            .inspect(|&end| self.cap(3, action_start, end))
            .or_else(|| {
                self.keyword(action_start, b"restore")
                    .inspect(|&end| self.cap(4, action_start, end))
            })?;
        self.cap(1, p, pragma_end);
        self.cap(2, warning_start, warning_end);
        let first = self.blanks(action_end);
        let Some(digits) = self.ascii_digits(first) else {
            return Some(action_end);
        };
        let mut end = digits;
        let comma = self.blanks(digits);
        if self.at(comma) == b','
            && comma < self.lim()
            && let Some(more) = self.ascii_digits(self.blanks(comma + 1))
        {
            end = more;
        }
        self.cap(5, action_end, end);
        Some(end)
    }

    /// `\b(pragma)\b\s*\b(checksum)\b\s*("[^"]*")\s*("[^"]*")\s*("[^"]*")`
    pub(in crate::csharp) fn pragma_checksum(&mut self, p: usize) -> Option<usize> {
        let pragma_end = self.keyword(p, b"pragma")?;
        let checksum_start = self.blanks(pragma_end);
        let checksum_end = self.keyword(checksum_start, b"checksum")?;
        let mut at = self.blanks(checksum_end);
        let mut groups = [(0, 0); 3];
        for (i, group) in groups.iter_mut().enumerate() {
            let end = self.quoted_text(at)?;
            *group = (at, end);
            at = end;
            if i < 2 {
                at = self.blanks(at);
            }
        }
        self.cap(1, p, pragma_end);
        self.cap(2, checksum_start, checksum_end);
        for (i, (from, to)) in groups.into_iter().enumerate() {
            self.cap(3 + i, from, to);
        }
        Some(at)
    }

    /// `\s*(:)\s*`
    pub(in crate::csharp) fn app_directive(&mut self, p: usize) -> Option<usize> {
        let colon = self.blanks(p);
        if self.at(colon) != b':' || colon >= self.lim() {
            return None;
        }
        self.cap(1, colon, colon + 1);
        Some(self.blanks(colon + 1))
    }

    /// `(\.)([_[:alpha:]][_[:alnum:]]*)`
    pub(in crate::csharp) fn package_name_part(&mut self, p: usize) -> Option<usize> {
        let dot = self.lit(p, b".")?;
        let end = self.plain_ident(dot)?;
        self.cap(1, p, dot);
        self.cap(2, dot, end);
        Some(end)
    }

    /// `\b(word)\b\s*(.*)?\s*`: a directive word and the text after it.
    pub(in crate::csharp) fn app_directive_text(&mut self, p: usize, word: &[u8]) -> Option<usize> {
        let end = self.keyword(p, word)?;
        self.cap(1, p, end);
        let text_end = self.message(end, 2);
        Some(self.blanks(text_end))
    }

    /// `\b(package)\b\s*([_[:alpha:]][._[:alnum:]]*)?(@)?(.*)?\s*`, and the
    /// same for `sdk`.
    pub(in crate::csharp) fn app_directive_package(
        &mut self,
        p: usize,
        word: &[u8],
    ) -> Option<usize> {
        let end = self.keyword(p, word)?;
        self.cap(1, p, end);
        let name_start = self.blanks(end);
        let mut at = name_start;
        if let Some(first) = self.alpha_at(name_start) {
            at = first;
            while let Some((c, next)) = self.cp(at) {
                if c == '.' || c == '_' || c.is_alphanumeric() {
                    at = next;
                } else {
                    break;
                }
            }
            self.cap(2, name_start, at);
        }
        if self.at(at) == b'@' && at < self.lim() {
            self.cap(3, at, at + 1);
            at += 1;
        }
        let rest = self.lim().max(at);
        self.cap(4, at, rest);
        Some(self.blanks(rest))
    }

    /// `\b(property)\b\s*([_[:alpha:]][_[:alnum:]]*)?(=)?(.*)?\s*`
    pub(in crate::csharp) fn app_directive_property(&mut self, p: usize) -> Option<usize> {
        let end = self.keyword(p, b"property")?;
        self.cap(1, p, end);
        let name_start = self.blanks(end);
        let mut at = name_start;
        if let Some(name) = self.plain_ident(name_start) {
            self.cap(2, name_start, name);
            at = name;
        }
        if self.at(at) == b'=' && at < self.lim() {
            self.cap(3, at, at + 1);
            at += 1;
        }
        let rest = self.lim().max(at);
        self.cap(4, at, rest);
        Some(self.blanks(rest))
    }

    /// `\b(.*)?\s*`
    pub(in crate::csharp) fn app_directive_generic(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let rest = self.lim().max(p);
        self.cap(1, p, rest);
        Some(self.blanks(rest))
    }
}
