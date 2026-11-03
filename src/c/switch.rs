//! `switch` statements, `case` and `default` labels, and inline assembly.
//!
//! These rules all start with a group of white space and comments that the
//! grammar lexes again as comments; it has two spellings, one that takes a
//! run of white space only after a non-space, and one that is lazy.

use super::engine::{Lexer, Retok};
use super::scope::S;
use super::state::R;

impl Lexer<'_> {
    /// Whether the character before `q` is white space.
    fn space_before(&self, q: usize) -> bool {
        match self.char_before(q) {
            Some(c) => self.space_at(q - c.len_utf8()) > 0,
            None => false,
        }
    }

    /// The regex `(?:(?:(?:(?:\b|(?<=\W))|(?=\W))|\A)|\Z)`: everywhere but
    /// between two word characters.
    fn boundary_or_edge(&self, p: usize) -> bool {
        !(self.word_before(p) && self.word_at(p) > 0)
    }

    /// `(?>(?:(?>(?<!\s)\s+)|(/\*)(…))+|boundary)` at `p`: where it ends.
    fn switch_prefix(&self, p: usize) -> Option<usize> {
        let mut q = p;
        let mut count = 0;
        loop {
            if self.space_at(q) > 0 && !self.space_before(q) {
                q = self.skip_space(q);
            } else if let Some((_, end)) = self.inline_comment_at(q) {
                q = end;
            } else {
                break;
            }
            count += 1;
        }
        if count > 0 {
            return Some(q);
        }
        self.boundary_or_edge(p).then_some(p)
    }

    /// `(?<!\w)word(?!\w)` at `q`.
    fn keyword_at(&self, q: usize, word: &[u8]) -> bool {
        self.starts(q, word) && !self.word_before(q) && self.word_at(q + word.len()) == 0
    }

    /// `(prefix)(word)` that opens `rule`, with `scope` on the word.
    pub(super) fn label_begin(
        &mut self,
        p: usize,
        word: &'static str,
        scope: &'static [S],
        rule: R,
    ) -> bool {
        let Some(q) = self.switch_prefix(p) else {
            return false;
        };
        if !self.keyword_at(q, word.as_bytes()) {
            return false;
        }
        self.cap_retok(p, q, &[], Retok::Inline);
        self.cap(q, q + word.len(), scope);
        self.begin(p, q + word.len(), rule)
    }

    /// `(((prefix)(?<!\w)switch(?!\w))`.
    pub(super) fn switch_begin(&mut self, p: usize) -> bool {
        let Some(q) = self.switch_prefix(p) else {
            return false;
        };
        if !self.keyword_at(q, b"switch") {
            return false;
        }
        self.cap(p, q + 6, &[S::MetaHeadSwitch]);
        self.cap_retok(p, q, &[], Retok::Inline);
        self.cap(q, q + 6, &[S::KeywordSwitch]);
        self.begin(p, q + 6, R::Switch)
    }

    /// `\G ?`: the head of a `switch`, right after its keyword.
    pub(super) fn switch_head(&mut self, p: usize) -> bool {
        if self.anchor != Some(p) {
            return false;
        }
        let end = p + usize::from(self.b(p) == b' ' && p < self.len_all());
        self.begin(p, end, R::SwitchHead)
    }

    /// Whether `{`, `<%` or `??<` ends right before `p`.
    pub(super) fn after_open(&self, p: usize) -> bool {
        self.prev(p) == Some(b'{')
            || (p >= 2 && self.text(p - 2, p) == b"<%")
            || (p >= 3 && self.text(p - 3, p) == b"??<")
    }

    /// Whether `}`, `%>` or `??>` ends right before `p`.
    pub(super) fn after_close(&self, p: usize) -> bool {
        self.prev(p) == Some(b'}')
            || (p >= 2 && self.text(p - 2, p) == b"%>")
            || (p >= 3 && self.text(p - 3, p) == b"??>")
    }

    /// `(?<=\{|<%|\?\?<)`: the body of a `switch`.
    pub(super) fn switch_body(&mut self, p: usize) -> bool {
        self.after_open(p) && self.begin(p, p, R::SwitchBody)
    }

    /// `(?<=}|%>|\?\?>)[\n\s]*`: what follows a `switch` body.
    pub(super) fn switch_tail(&mut self, p: usize) -> bool {
        if !self.after_close(p) {
            return false;
        }
        let end = self.skip_space(p);
        self.begin(p, end, R::SwitchTail)
    }

    /// The parentheses of a `switch`: a prefix and `(`.
    pub(super) fn switch_parens(&mut self, p: usize) -> bool {
        let Some(q) = self.switch_prefix(p) else {
            return false;
        };
        if self.b(q) != b'(' || q >= self.len_all() {
            return false;
        }
        self.cap_retok(p, q, &[], Retok::Inline);
        self.cap(q, q + 1, &[S::PunctParensBeginSwitch]);
        self.begin(p, q + 1, R::SwitchParens)
    }

    /// The end of the lazy prefix `(?:(?>\s+)|(/\*)(…))+?` at `p`, with the
    /// element boundaries tried in order until `then` accepts one. Without
    /// any element, the other alternative, a position that is a boundary.
    fn lazy_prefix(&self, p: usize, then: impl Fn(&Self, usize) -> bool) -> Option<usize> {
        let mut q = p;
        loop {
            if self.space_at(q) > 0 {
                q = self.skip_space(q);
            } else if let Some((_, end)) = self.inline_comment_at(q) {
                q = end;
            } else {
                break;
            }
            if then(self, q) {
                return Some(q);
            }
        }
        (self.boundary_or_edge(p) && then(self, p)).then_some(p)
    }

    /// `\b(__asm__|asm)\b\s*((?:volatile)?)`.
    pub(super) fn asm_begin(&mut self, p: usize) -> bool {
        let Some(end) = self.word_in(p, super::wordset::ASM) else {
            return false;
        };
        let ws = self.skip_space(end);
        self.cap(p, end, &[S::StorageAsm]);
        let mut stop = ws;
        if self.starts(ws, b"volatile") {
            self.cap(ws, ws + 8, &[S::StorageModifier]);
            stop = ws + 8;
        }
        self.begin(p, stop, R::Asm)
    }

    /// `^(prefix)(?:\n|$)`: a line of white space and comments.
    pub(super) fn asm_line_end(&mut self, p: usize) -> bool {
        if p != 0 {
            return false;
        }
        let Some(q) = self.lazy_prefix(0, |lexer, q| q >= lexer.lim) else {
            return false;
        };
        self.cap_retok(0, q, &[], Retok::Inline);
        let end = if q == self.lim && self.nl { q + 1 } else { q };
        self.plain(0, end, &[])
    }

    /// The parentheses of an assembly statement: a prefix and `(`.
    pub(super) fn asm_parens(&mut self, p: usize) -> bool {
        let Some(q) = self.lazy_prefix(p, |lexer, q| lexer.b(q) == b'(' && q < lexer.len_all())
        else {
            return false;
        };
        self.cap(p, q + 1, &[S::PunctParensBeginAsm]);
        self.cap_retok(p, q, &[], Retok::Inline);
        self.begin(p, q + 1, R::AsmParens)
    }

    /// `(R?)(")`: a string of assembly.
    pub(super) fn asm_string(&mut self, p: usize) -> bool {
        let mut q = p;
        if self.b(p) == b'R' && p < self.len_all() {
            self.cap(p, p + 1, &[S::MetaEncoding]);
            q = p + 1;
        }
        if self.b(q) != b'"' || q >= self.len_all() {
            return false;
        }
        self.cap(q, q + 1, &[S::PunctStringBeginAsm]);
        self.begin(p, q + 1, R::AsmString)
    }

    /// `(\()`: an inner parenthesis of assembly.
    pub(super) fn asm_inner(&mut self, p: usize) -> bool {
        if self.b(p) != b'(' || p >= self.len_all() {
            return false;
        }
        self.cap(p, p + 1, &[S::PunctParensBeginAsmInner]);
        self.begin(p, p + 1, R::AsmInner)
    }

    /// `\[(prefix)([A-Z_a-z]\w*)(prefix)]`: an operand name.
    pub(super) fn asm_label(&mut self, p: usize) -> bool {
        if self.b(p) != b'[' || p >= self.len_all() {
            return false;
        }
        let Some(a) = self.lazy_prefix(p + 1, |lexer, q| lexer.ident_end(q).is_some()) else {
            return false;
        };
        let Some(id) = self.ident_end(a) else {
            return false;
        };
        let Some(b) = self.lazy_prefix(id, |lexer, q| lexer.b(q) == b']' && q < lexer.len_all())
        else {
            return false;
        };
        self.cap_retok(p + 1, a, &[], Retok::Inline);
        self.cap(a, id, &[S::VariableAsmLabel]);
        self.cap_retok(id, b, &[], Retok::Inline);
        self.plain(p, b + 1, &[])
    }
}
