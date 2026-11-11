//! Blade's own doc tags, comments, strings, numbers and heredocs.

use super::super::engine::Lexer;
use crate::rules::Chars;

impl Lexer<'_> {
    /// `^\s*\*\s*(@access)\s+((p(?:ublic|rivate|rotected))|(.+))\s*$`, with
    /// the visibility in its case.
    /// The spaces after `@access` give back one at a time, so `.+` can start
    /// on the last of them when nothing follows.
    pub(super) fn doc_access(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let star = self.space_end(0);
        if self.at(star) != b'*' || star >= self.lim() {
            return None;
        }
        let tag = self.space_end(star + 1);
        let tag_end = self.lit(tag, b"@access")?;
        let mut at = self.space_end(tag_end);
        while at > tag_end {
            for word in [b"public".as_slice(), b"private", b"protected"] {
                if let Some(end) = self.lit(at, word) {
                    let rest = self.space_end(end);
                    if self.at_eol(rest) {
                        self.capture(1, tag, tag_end);
                        self.capture(3, at, end);
                        return Some(rest);
                    }
                }
            }
            if at < self.lim() {
                self.capture(1, tag, tag_end);
                self.capture(4, at, self.lim());
                return Some(self.space_end(self.lim()));
            }
            at = self.char_start_before(at);
        }
        None
    }

    /// `(@(?:global|param|property(-(read|write))?|return|throws|var))\s+(?
    /// =[(A-Z\\_a-z\x7F-ÿ])`.
    /// U+0085 and U+00A0 are spaces in the class.
    /// The spaces give back to the last of them when the character after the
    /// spaces is not in the class.
    pub(super) fn doc_typed_tag(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'@' {
            return None;
        }
        let words: [&[u8]; 8] = [
            b"global",
            b"param",
            b"property-read",
            b"property-write",
            b"property",
            b"return",
            b"throws",
            b"var",
        ];
        let starts_type = |c: char| {
            matches!(c, '(' | '\\' | '_')
                || c.is_ascii_alphabetic()
                || ('\u{7f}'..='\u{ff}').contains(&c)
        };
        for word in words {
            let Some(end) = self.lit(p + 1, word) else {
                continue;
            };
            let spaces = self.space_end(end);
            if spaces == end {
                continue;
            }
            let types = |lexer: &Self, at: usize| {
                at < lexer.lim() && lexer.char_at(at).is_some_and(|(c, _)| starts_type(c))
            };
            let mut at = spaces;
            while !types(self, at) {
                at = self.char_start_before(at);
                if at <= end {
                    break;
                }
            }
            if at <= end {
                continue;
            }
            self.capture(1, p, end);
            return Some(at);
        }
        None
    }

    /// `(\))(\[])|(?=\*/)`.
    pub(super) fn doc_types_close(&mut self, p: usize) -> Option<usize> {
        if self.at(p) == b')' && self.starts(p + 1, b"[]") {
            self.capture(1, p, p + 1);
            self.capture(2, p + 1, p + 3);
            return Some(p + 3);
        }
        self.starts(p, b"*/").then_some(p)
    }

    /// `(?i)[\\_a-z\x7F-ÿ][0-9\\_a-z\x7F-ÿ]*(\|[\\_a-z\x7F-ÿ][0-9\\_a-z\x7F-ÿ]*
    /// )*`.
    pub(super) fn doc_types(&self, p: usize) -> Option<usize> {
        let mut end = self.doc_type(p)?;
        while self.at(end) == b'|'
            && let Some(more) = self.doc_type(end + 1)
        {
            end = more;
        }
        Some(end)
    }

    /// `(?i)([\\_a-z\x7F-ÿ][0-9\\_a-z\x7F-ÿ]*)(\[])`.
    pub(super) fn doc_type_array(&mut self, p: usize) -> Option<usize> {
        let end = self.doc_type(p)?;
        if !self.starts(end, b"[]") {
            return None;
        }
        self.capture(1, p, end);
        self.capture(2, end, end + 2);
        Some(end + 2)
    }

    /// `(?i)[\\_a-z\x7F-ÿ][0-9\\_a-z\x7F-ÿ]*` at `p`: a type of a doc tag.
    fn doc_type(&self, p: usize) -> Option<usize> {
        (self.at(p) == b'\\' || self.name_starts(p)).then(|| self.qualified_end(p))
    }

    /// `(^\s+)?(?=#)`: where a `#` comment opens, and group 1 holds the spaces
    /// before it at the line's start.
    pub(super) fn hash_comment(&mut self, p: usize) -> Option<usize> {
        if p == 0 {
            let end = self.space_end(0);
            if end > 0 && self.at(end) == b'#' && end < self.lim() {
                self.capture(1, 0, end);
                return Some(end);
            }
        }
        (self.at(p) == b'#' && p < self.lim()).then_some(p)
    }

    /// `\\["$\\efnrtv]`.
    pub(super) fn string_escape(&self, p: usize) -> Option<usize> {
        (self.at(p) == b'\\'
            && matches!(
                self.at(p + 1),
                b'"' | b'$' | b'\\' | b'e' | b'f' | b'n' | b'r' | b't' | b'v'
            ))
        .then_some(p + 2)
    }

    /// `\\.`: a backslash and any character but the line feed.
    pub(super) fn any_escape(&self, p: usize) -> Option<usize> {
        (self.at(p) == b'\\' && p + 1 < self.lim()).then(|| self.char_end(p + 1))
    }

    /// `"\s*(?=(SELECT|INSERT|UPDATE|DELETE|CREATE|REPLACE|ALTER|AND)\b)` for
    /// `"`, and with `'` for `'`.
    pub(super) fn sql_open(&self, p: usize, quote: u8) -> Option<usize> {
        if self.at(p) != quote || p >= self.lim() {
            return None;
        }
        let at = self.space_end(p + 1);
        let words: [&[u8]; 8] = [
            b"SELECT", b"INSERT", b"UPDATE", b"DELETE", b"CREATE", b"REPLACE", b"ALTER", b"AND",
        ];
        words
            .iter()
            .any(|word| self.lit(at, word).is_some_and(|end| self.boundary(end)))
            .then_some(at)
    }

    /// `0`, a letter of `marks` and one or more digits that `digit` accepts:
    /// `0[Xx]\h+` and `0[Bb][01]+`.
    pub(super) fn radix(&self, p: usize, marks: &[u8], digit: fn(u8) -> bool) -> Option<usize> {
        if self.at(p) != b'0' || !marks.contains(&self.at(p + 1)) {
            return None;
        }
        let end = self.skip(p + 2, digit);
        (end > p + 2).then_some(end)
    }

    /// `0[0-7]+`.
    pub(super) fn octal(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'0' {
            return None;
        }
        let end = self.skip(p + 1, |b| matches!(b, b'0'..=b'7'));
        (end > p + 1).then_some(end)
    }

    /// A decimal number with a point or an exponent:
    ///
    /// ```text
    /// [0-9]*(\.)[0-9]+E? | [0-9]+(\.)[0-9]*E? | [0-9]+E
    /// ```
    ///
    /// where `E` is `[Ee][-+]?[0-9]+`.
    pub(super) fn decimal(&mut self, p: usize) -> Option<usize> {
        let digits = |b: u8| b.is_ascii_digit();
        let whole = self.skip(p, digits);
        if self.at(whole) == b'.' && whole < self.lim() {
            let fraction = self.skip(whole + 1, digits);
            if fraction > whole + 1 {
                self.capture(1, whole, whole + 1);
                return Some(self.exponent(fraction).unwrap_or(fraction));
            }
            if whole > p {
                self.capture(2, whole, whole + 1);
                return Some(self.exponent(whole + 1).unwrap_or(whole + 1));
            }
            return None;
        }
        if whole == p {
            return None;
        }
        self.exponent(whole)
    }

    /// `[Ee][-+]?[0-9]+` from `p`.
    fn exponent(&self, p: usize) -> Option<usize> {
        if !matches!(self.at(p), b'e' | b'E') {
            return None;
        }
        let at = p + 1 + usize::from(matches!(self.at(p + 1), b'-' | b'+'));
        let end = self.skip(at, |b| b.is_ascii_digit());
        (end > at).then_some(end)
    }

    /// `0|[1-9][0-9]*`.
    pub(super) fn integer(&self, p: usize) -> Option<usize> {
        match self.at(p) {
            b'0' if p < self.lim() => Some(p + 1),
            b'1'..=b'9' => Some(self.skip(p + 1, |b| b.is_ascii_digit())),
            _ => None,
        }
    }

    /// `(?i)(?=<<<\s*("?)(NAME)(\1)\s*$)`.
    pub(super) fn heredoc_ahead(&self, p: usize) -> Option<usize> {
        let at = self.space_end(self.lit(p, b"<<<")?);
        let quoted = self.at(at) == b'"';
        let name = self.bname(at + usize::from(quoted))?;
        if quoted && self.at(name) != b'"' {
            return None;
        }
        self.at_eol(self.space_end(name + usize::from(quoted)))
            .then_some(p)
    }

    /// `(?=<<<\s*'([A-Z_a-z]+[0-9A-Z_a-z]*)'\s*$)`.
    pub(super) fn nowdoc_ahead(&self, p: usize) -> Option<usize> {
        let at = self.space_end(self.lit(p, b"<<<")?);
        if self.at(at) != b'\'' {
            return None;
        }
        let first = self.at(at + 1);
        if !(first == b'_' || first.is_ascii_alphabetic()) {
            return None;
        }
        let name = self.skip(at + 2, |b| b == b'_' || b.is_ascii_alphanumeric());
        if self.at(name) != b'\'' {
            return None;
        }
        self.at_eol(self.space_end(name + 1)).then_some(p)
    }

    /// `(?i)(<<<)\s*("?)([_a-z\x7F-ÿ]+[0-9_a-z\x7F-ÿ]*)(\2)(\s*)`.
    pub(super) fn heredoc_open_any(&mut self, p: usize) -> Option<usize> {
        let arrows = self.lit(p, b"<<<")?;
        let at = self.space_end(arrows);
        let quoted = self.at(at) == b'"';
        for (quoted, from) in [(quoted, at + usize::from(quoted)), (false, at)] {
            let Some(end) = self.bname(from) else {
                continue;
            };
            if quoted && self.at(end) != b'"' {
                continue;
            }
            let spaces = end + usize::from(quoted);
            let line_end = self.space_end(spaces);
            self.capture(1, p, arrows);
            self.capture(3, from, end);
            self.capture(5, spaces, line_end);
            return Some(line_end);
        }
        None
    }

    /// `(?i)(<<<)\s*'([_a-z\x7F-ÿ]+[0-9_a-z\x7F-ÿ]*)'(\s*)`.
    pub(super) fn nowdoc_open_any(&mut self, p: usize) -> Option<usize> {
        let arrows = self.lit(p, b"<<<")?;
        let at = self.space_end(arrows);
        if self.at(at) != b'\'' {
            return None;
        }
        let end = self.bname(at + 1)?;
        if self.at(end) != b'\'' {
            return None;
        }
        let line_end = self.space_end(end + 1);
        self.capture(1, p, arrows);
        self.capture(2, at + 1, end);
        self.capture(3, end + 1, line_end);
        Some(line_end)
    }

    /// `^\s*(LABEL)\b`, the line that closes a heredoc or a nowdoc, with the
    /// label as group 1.
    /// Without `label`, the label is the one the begin took, `\3` or `\2`.
    pub(super) fn label_line(&mut self, p: usize, label: Option<&'static [u8]>) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let at = self.space_end(0);
        let label = label.map_or_else(|| self.begin_text().as_bytes(), |label| label);
        if label.is_empty() {
            return None;
        }
        let end = self.lit(at, label)?;
        if !self.boundary(end) {
            return None;
        }
        self.capture(1, at, end);
        Some(end)
    }

    /// `(?i)(?<=^|\s)(#)\s(?=[-\t !,.0-9?_a-z\x7F-ÿ[^\x00-\x7F]]*$)`: a
    /// comment in a regular expression's heredoc, whose rest of the line holds
    /// only such characters.
    pub(super) fn regex_comment(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'#' || p >= self.lim() {
            return None;
        }
        if p > 0 && self.space_at(self.char_start_before(p)) == 0 {
            return None;
        }
        let space = self.space_at(p + 1);
        if space == 0 {
            return None;
        }
        let rest = p + 1 + space;
        let mut at = rest;
        while at < self.lim() {
            let b = self.at(at);
            let allowed = b >= 0x7f
                || b.is_ascii_alphanumeric()
                || matches!(b, b'-' | b'\t' | b' ' | b'!' | b',' | b'.' | b'?' | b'_');
            if !allowed {
                return None;
            }
            at += 1;
        }
        self.capture(1, p, p + 1);
        Some(rest)
    }

    /// `\x{FFFF}`: the end that vscode-textmate gives a rule with a begin and
    /// no end, which never matches in practice.
    pub(super) fn never_end(&self, p: usize) -> Option<usize> {
        self.lit(p, "\u{ffff}".as_bytes())
    }
}
