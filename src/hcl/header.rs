//! Names that open a line's structure: attribute definitions, block headers
//! and object keys.

use memchr::{memchr2, memrchr};

use super::lexer::Lexer;
use super::scope::S;
use super::state::Rule;
use crate::classes::word_before;

impl Lexer<'_> {
    /// `(\()?\b((?!(?:null|false|true)\b)\p{alpha}[-_[:alnum:]]*)(\))?\s*(=(?!
    /// [=>]))\s*` at `p`: an attribute definition.
    pub(super) fn attribute(&mut self, p: usize) -> Option<usize> {
        if p < self.no_attribute_before {
            return None;
        }
        let open = self.s[p] == b'(';
        let name = p + usize::from(open);
        if word_before(self.s, name) || self.alpha_width(name) == 0 {
            return None;
        }
        if matches!(self.whole_word(name), Some(b"null" | b"false" | b"true")) {
            return None;
        }
        let name_end = self.name_end(name);
        let close = self.at(name_end) == b')';
        let close_end = name_end + usize::from(close);
        let eq = self.skip_space(close_end);
        if self.at(eq) != b'=' || matches!(self.at(eq + 1), b'=' | b'>') {
            self.no_attribute_before = name_end;
            return None;
        }
        let end = self.skip_space(eq + 1);
        self.gap(p);
        if open {
            self.tok(name, &[S::Declaration, S::ParensBegin]);
        }
        self.tok(name_end, &[S::Declaration, S::Readwrite]);
        if close {
            self.tok(close_end, &[S::Declaration, S::ParensEnd]);
        }
        self.tok(eq, &[S::Declaration]);
        self.tok(eq + 1, &[S::Declaration, S::Assignment]);
        self.tok(end, &[S::Declaration]);
        Some(end)
    }

    /// A block header at `p`, a word character:
    /// `(\w[-\w]*)(([^\n\r\S]+(\w[-_\w]*|"[^\n\r"]*"))*)[^\n\r\S]*(\{)`.
    ///
    /// Its labels must run up to the `{` on the same line.
    pub(super) fn block(&mut self, p: usize) -> Option<usize> {
        if p < self.no_block_before || !self.word_at(p) {
            return None;
        }
        let type_end = self.dash_word_end(p);
        let mut labels_end = type_end;
        loop {
            let label = self.skip_blanks(labels_end);
            if label == labels_end {
                break;
            }
            if self.word_at(label) {
                labels_end = self.dash_word_end(label);
                continue;
            }
            if self.at(label) != b'"' {
                break;
            }
            let rest = &self.s[label + 1..];
            match memchr2(b'"', b'\r', rest) {
                Some(k) if rest[k] == b'"' => {
                    self.steps.spend(k);
                    labels_end = label + k + 2;
                }
                Some(_) | None => break,
            }
        }
        let brace = self.skip_blanks(labels_end);
        if self.at(brace) != b'{' {
            self.no_block_before = brace;
            return None;
        }
        self.gap(p);
        if !self.push(Rule::Block, 0) {
            return Some(brace + 1);
        }
        self.block_type(p, type_end);
        self.labels(type_end, labels_end);
        self.tok_at(brace, brace + 1, &[S::BlockBegin]);
        Some(brace + 1)
    }

    /// The block type from `p` to `end`, which
    /// `\b(?!null|false|true)\p{alpha}[-_[:alnum:]]*\b` reads at each byte.
    fn block_type(&mut self, p: usize, end: usize) {
        let mut q = p;
        while q < end {
            if let Some(name_end) = self.identifier(q) {
                self.tok_at(q, name_end, &[S::TypeName]);
                q = name_end;
                continue;
            }
            q += self.width_at(q);
        }
    }

    /// The labels from `p` to `end`, which `"[^\n\r"]*"` and
    /// `\p{alpha}[-_[:alnum:]]*` read at each byte, with no `\b` and no
    /// keyword guard.
    fn labels(&mut self, p: usize, end: usize) {
        let mut q = p;
        while q < end {
            let label_end = if self.s[q] == b'"' {
                memchr::memchr(b'"', &self.s[q + 1..end]).map(|k| q + k + 2)
            } else if self.alpha_width(q) > 0 {
                Some(self.name_end(q))
            } else {
                None
            };
            match label_end {
                Some(label_end) => {
                    self.tok_at(q, label_end, &[S::EnumMember]);
                    q = label_end;
                }
                None => q += self.width_at(q),
            }
        }
    }

    /// `\b((?!null|false|true)\p{alpha}[-_[:alnum:]]*)\s*(=(?!=))\s*` at `p`
    /// in an object: a key that is a name.
    /// Unlike an attribute definition, it takes `=>` and leaves out names
    /// that start with `null`, `false` or `true`.
    pub(super) fn key_name(&mut self, p: usize) -> Option<usize> {
        if p < self.no_key_before
            || word_before(self.s, p)
            || self.alpha_width(p) == 0
            || self.keyword_prefix(p)
        {
            return None;
        }
        let name_end = self.name_end(p);
        let eq = self.skip_space(name_end);
        if self.at(eq) != b'=' || self.at(eq + 1) == b'=' {
            self.no_key_before = name_end;
            return None;
        }
        self.tok_at(p, name_end, &[S::MetaMappingKey, S::Readwrite]);
        self.tok_at(eq, eq + 1, &[S::Assignment]);
        Some(self.skip_space(eq + 1))
    }

    /// `^\s*((").*("))\s*(=)\s*` in an object: a quoted key.
    /// The key runs to the line's last `"` that `\s*=` follows.
    pub(super) fn quoted_key(&mut self) -> Option<usize> {
        let open = self.skip_space(0);
        if self.at(open) != b'"' {
            return None;
        }
        let mut rest = &self.s[open + 1..];
        while let Some(k) = memrchr(b'"', rest) {
            let close = open + 1 + k;
            let eq = self.skip_space(close + 1);
            if self.at(eq) == b'=' {
                self.steps.spend(self.s.len() - close);
                self.tok_at(
                    open,
                    open + 1,
                    &[S::MetaMappingKey, S::StrDouble, S::StrBegin],
                );
                self.tok(close, &[S::MetaMappingKey, S::StrDouble]);
                self.tok(close + 1, &[S::MetaMappingKey, S::StrDouble, S::StrEnd]);
                self.tok_at(eq, eq + 1, &[S::Operator]);
                return Some(self.skip_space(eq + 1));
            }
            rest = &rest[..k];
        }
        self.steps.spend(self.s.len() - open);
        None
    }

    /// `^\s*\(` in an object: a key expression, up to `)` and `=` or `:`.
    pub(super) fn paren_key(&mut self) -> Option<usize> {
        let open = self.skip_space(0);
        if self.at(open) != b'(' {
            return None;
        }
        if self.push(Rule::ObjectKey, 0) {
            self.tok(open + 1, &[S::ParensBegin]);
        }
        Some(open + 1)
    }
}
