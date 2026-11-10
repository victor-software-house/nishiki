//! The `@` tags of `---` documentation comments and the types they name.

use super::super::engine::Lexer;
use super::chars::{diagnostic, name_start, type_part};
use crate::rules::Chars;

/// `[-\]*,.0-9<>A-\[_a-z]`, a character of a type expression's word.
fn type_word_part(b: u8) -> bool {
    matches!(
        b,
        b'-' | b']' | b'*' | b',' | b'.' | b'0'..=b'9' | b'<' | b'>' | b'A'..=b'[' | b'_'
            | b'a'..=b'z'
    )
}

impl Lexer<'_> {
    /// `(?<=---)[\t ]*`: the end of the blanks after `---` at `p`.
    fn after_dashes(&self, p: usize) -> Option<usize> {
        (p >= 3 && self.text(p - 3, p) == b"---").then(|| self.tabs(p))
    }

    /// `(?<=---)[\t ]*@tag`, with no `\b` after the tag.
    pub(super) fn doc_tag(&self, p: usize, tag: &[u8]) -> Option<usize> {
        self.lit(self.after_dashes(p)?, tag)
    }

    /// `(?<=---)[\t ]*(@operator)\s*(\b[a-z]+)?`: the tag as capture group 1
    /// and the operator's name as 2.
    pub(super) fn operator_tag(&mut self, p: usize) -> Option<usize> {
        let at = self.after_dashes(p)?;
        let tag = self.lit(at, b"@operator")?;
        self.capture(1, at, tag);
        let name = self.space_end(tag);
        if self.boundary(name) {
            let end = self.skip(name, |b| b.is_ascii_lowercase());
            if end > name {
                self.capture(2, name, end);
                return Some(end);
            }
        }
        Some(name)
    }

    /// `(?<=---)[\t ]*@(async|nodiscard)`
    pub(super) fn flag_tag(&self, p: usize) -> Option<usize> {
        let name = self.lit(self.after_dashes(p)?, b"@")?;
        self.lit(name, b"async")
            .or_else(|| self.lit(name, b"nodiscard"))
    }

    /// `(?<=---)\|\s*[+>]?`: a line that lists a value of an enumerated type.
    pub(super) fn enum_item(&self, p: usize) -> Option<usize> {
        if p < 3 || self.text(p - 3, p) != b"---" {
            return None;
        }
        let blanks = self.space_end(self.lit(p, b"|")?);
        Some(self.byte_of(blanks, b"+>").unwrap_or(blanks))
    }

    /// `([+-|])`: one byte from `+` to `|`.
    pub(super) fn cast_sign(&self, p: usize) -> Option<usize> {
        (p < self.lim() && (b'+'..=b'|').contains(&self.at(p))).then_some(p + 1)
    }

    /// `\b([A-Z_a-z][0-9A-Z_a-z]*)\b(\??)`: a parameter's name as capture group
    /// 1, and the `?` that makes it optional, or nothing, as 2.
    pub(super) fn param_name(&mut self, p: usize) -> Option<usize> {
        let name = self.bounded_name(p)?;
        let end = self.byte_of(name, b"?").unwrap_or(name);
        self.capture(1, p, name);
        self.capture(2, name, end);
        Some(end)
    }

    /// `(\b([A-Z_a-z][0-9A-Z_a-z]*)\b|(\[))(\??)`: a field's name as capture
    /// group 2, or the bracket of an indexed field as 3, then an optional `?`.
    pub(super) fn field_name(&mut self, p: usize) -> Option<usize> {
        let name = if let Some(end) = self.bounded_name(p) {
            self.capture(2, p, end);
            end
        } else {
            let end = self.lit(p, b"[")?;
            self.capture(3, p, end);
            end
        };
        Some(self.byte_of(name, b"?").unwrap_or(name))
    }

    /// `(?=\n)|(,)`
    pub(super) fn generic_end(&self, p: usize) -> Option<usize> {
        if self.avail(p) && self.at(p) == b'\n' {
            return Some(p);
        }
        self.lit(p, b",")
    }

    /// `\b(5\.1|5\.2|5\.3|5\.4|JIT)\b`
    pub(super) fn lua_version(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let end = [&b"5.1"[..], b"5.2", b"5.3", b"5.4", b"JIT"]
            .into_iter()
            .find_map(|version| self.lit(p, version))?;
        self.word_at(end).is_none().then_some(end)
    }

    /// `([-0-9A-Z_a-z]+)[\t ]*(:)?`: a diagnostic's name as capture group 1,
    /// and the colon that begins its list as 2.
    pub(super) fn diagnostic_name(&mut self, p: usize) -> Option<usize> {
        let name = self.skip(p, diagnostic);
        if name == p {
            return None;
        }
        self.capture(1, p, name);
        let blanks = self.tabs(name);
        let Some(end) = self.lit(blanks, b":") else {
            return Some(blanks);
        };
        self.capture(2, blanks, end);
        Some(end)
    }

    /// `(?=[set\s])`: a byte of `set` or a blank, the line feed included.
    pub(super) fn type_end_ahead(&self, p: usize, set: &[u8]) -> Option<usize> {
        (self.ahead_in(p, set) || self.space_at(p) > 0).then_some(p)
    }

    /// `[set][\t ]*`: one byte of `set` and the blanks after it.
    pub(super) fn punctuation_then_tabs(&self, p: usize, set: &[u8]) -> Option<usize> {
        Some(self.tabs(self.byte_of(p, set)?))
    }

    /// `([A-Z_a-z][-*.0-9A-Z_a-z]*)(?<!,)[\t ]*(?=\??:)`: a field of a
    /// function type, before its colon.
    /// Neither run can give back a character that `?` or `:` would accept.
    /// The longest runs are therefore the only ones tried.
    pub(super) fn type_field_name(&self, p: usize) -> Option<usize> {
        if p >= self.lim() || !name_start(self.at(p)) {
            return None;
        }
        let end = self.tabs(self.skip(p + 1, type_part));
        let colon = self.byte_of(end, b"?").unwrap_or(end);
        (self.at(colon) == b':' && colon < self.lim()).then_some(end)
    }

    /// `<[*A-Z_a-z][-*.0-9A-Z_a-z]*>`: a name in angle brackets.
    /// The run cannot give back a `>`.
    /// The longest run is therefore the only one tried.
    pub(super) fn angle_name(&self, p: usize) -> Option<usize> {
        let name = self.lit(p, b"<")?;
        if name >= self.lim() || !super::chars::type_start(self.at(name)) {
            return None;
        }
        self.lit(self.skip(name + 1, type_part), b">")
    }

    /// `(?=["'*.A-\[_a-z])`: where a type expression's word or string starts.
    pub(super) fn type_start_ahead(&self, p: usize) -> Option<usize> {
        let starts = p < self.lim()
            && matches!(self.at(p), b'"' | b'\'' | b'*' | b'.' | b'A'..=b'[' | b'_' | b'a'..=b'z');
        starts.then_some(p)
    }

    /// `([-\]*,.0-9<>A-\[_a-z]+)(?<!,)[\t ]*`: a word of a type expression.
    /// The run gives back the commas that end it.
    /// `(?<!,)` refuses them.
    pub(super) fn type_word(&self, p: usize) -> Option<usize> {
        let mut end = self.skip(p, type_word_part);
        while end > p && self.prev_is(end, b',') {
            end -= 1;
        }
        (end > p).then(|| self.tabs(end))
    }

    /// `(\.\.\.)[\t ]*`
    pub(super) fn ellipsis_then_tabs(&self, p: usize) -> Option<usize> {
        Some(self.tabs(self.lit(p, b"...")?))
    }
}
