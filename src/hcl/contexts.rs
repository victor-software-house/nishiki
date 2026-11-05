//! The steps of each context.
//!
//! Each `in_*` step tries the context's end first, then its rules in the
//! grammar's order, at one byte.
//! When none matches, the byte is plain text of the context.
//! A run of word characters that no rule matched at its first byte is skipped
//! whole: no rule can start inside it, apart from an inline `for` or `if` at
//! its end, which [`Tail::ForIf`] looks for.

use super::lexer::{Feed, Lexer, Tail};
use super::scope::S;
use super::state::Kind;
use crate::classes::word_before;

impl Lexer<'_> {
    /// Steps past the text at `p` that no rule matched.
    pub(super) fn skip(&self, p: usize, tail: Tail) -> usize {
        if !self.word_at(p) {
            return p + self.width_at(p);
        }
        let end = self.word_end(p);
        if tail == Tail::ForIf {
            let word = &self.s[p..end];
            if word.len() > 3 && word.ends_with(b"for") {
                return end - 3;
            }
            if word.len() > 2 && word.ends_with(b"if") {
                return end - 2;
            }
        }
        end
    }

    /// The file's body: comments, attribute definitions, blocks and
    /// expressions.
    pub(super) fn in_top(&mut self, p: usize) -> usize {
        if let Some(end) = self.comment(p) {
            return end;
        }
        if let Some(end) = self.attribute(p) {
            return end;
        }
        if let Some(end) = self.block(p) {
            return end;
        }
        if let Some(end) = self.expression(p) {
            return end;
        }
        self.skip(p, Tail::Plain)
    }

    /// A block's body: its `}`, then the body's rules, with expressions
    /// before nested blocks.
    pub(super) fn in_block(&mut self, p: usize) -> usize {
        if self.s[p] == b'}' {
            return self.finish(p, p + 1, S::BlockEnd);
        }
        if let Some(end) = self.comment(p) {
            return end;
        }
        if let Some(end) = self.attribute(p) {
            return end;
        }
        if let Some(end) = self.expression(p) {
            return end;
        }
        if let Some(end) = self.block(p) {
            return end;
        }
        self.skip(p, Tail::Plain)
    }

    /// An object: its `}`, comments, nested objects, inline `for` and `if`,
    /// the three kinds of key, then values.
    pub(super) fn in_objects(&mut self, p: usize) -> usize {
        match self.s[p] {
            b'}' => return self.finish(p, p + 1, S::BracesEnd),
            b'{' => return self.begin(p, p + 1, Kind::Objects, S::BracesBegin),
            _ => {}
        }
        if let Some(end) = self.comment(p) {
            return end;
        }
        if let Some(end) = self.inline_for(p) {
            return end;
        }
        if let Some(end) = self.inline_if(p) {
            return end;
        }
        if let Some(end) = self.key_name(p) {
            return end;
        }
        if p == 0 {
            if let Some(end) = self.quoted_key() {
                return end;
            }
            if let Some(end) = self.paren_key() {
                return end;
            }
        }
        if let Some(end) = self.object_value(p) {
            return end;
        }
        self.skip(p, Tail::ForIf)
    }

    /// A key expression in parentheses: `)` and `=` or `:` end it.
    /// Only attribute access and splats are read inside.
    pub(super) fn in_object_key(&mut self, p: usize) -> usize {
        match self.s[p] {
            b')' => {
                let w = self.skip_space(p + 1);
                if !matches!(self.at(w), b':' | b'=') {
                    return p + 1;
                }
                self.tok_at(p, p + 1, &[S::ParensEnd]);
                self.tok_at(w, w + 1, &[S::Operator]);
                let end = self.skip_space(w + 1);
                self.gap(end);
                self.em.pop();
                end
            }
            b'.' => self.access(p),
            _ => {
                let rest = &self.s[p + 1..];
                let next = memchr::memchr2(b')', b'.', rest).map_or(self.s.len(), |k| p + 1 + k);
                self.steps.spend(next - p);
                next
            }
        }
    }

    /// A bracket: its `]`, splats, commas, comments, inline `for` and `if`,
    /// expressions and names.
    pub(super) fn in_brackets(&mut self, p: usize) -> usize {
        match self.s[p] {
            b']' => return self.finish(p, p + 1, S::BracketsEnd),
            b'*' => return self.mark(p, S::Splat),
            b',' => return self.mark(p, S::Separator),
            _ => {}
        }
        if let Some(end) = self.comment(p) {
            return end;
        }
        if let Some(end) = self.inline_for(p) {
            return end;
        }
        if let Some(end) = self.inline_if(p) {
            return end;
        }
        if let Some(end) = self.expression(p) {
            return end;
        }
        if let Some(end) = self.local(p) {
            return end;
        }
        self.skip(p, Tail::ForIf)
    }

    /// The body of a `for`: the bracket's or brace's end, `=>` where a value
    /// follows a key, then the body's rules.
    pub(super) fn in_for(&mut self, p: usize) -> usize {
        let kind = self.top();
        match (kind, self.s[p]) {
            (Kind::TupleFor, b']') => return self.finish(p, p + 1, S::BracketsEnd),
            (Kind::ObjectFor, b'}') => return self.finish(p, p + 1, S::BracesEnd),
            (Kind::ObjectFor | Kind::InlineFor, b'=') if self.at(p + 1) == b'>' => {
                self.tok_at(p, p + 2, &[S::StorageFunction]);
                return p + 2;
            }
            _ => {}
        }
        if self.word_is(p, b"in") && !word_before(self.s, p) {
            self.tok_at(p, p + 2, &[S::OperatorWord]);
            return p + 2;
        }
        if self.word_is(p, b"if") && !word_before(self.s, p) {
            self.tok_at(p, p + 2, &[S::Conditional]);
            return p + 2;
        }
        if self.s[p] == b':' {
            return self.mark(p, S::Operator);
        }
        if let Some(end) = self.expression(p) {
            return end;
        }
        if let Some(end) = self.comment(p) {
            return end;
        }
        if self.s[p] == b',' {
            return self.mark(p, S::Separator);
        }
        if let Some(end) = self.local(p) {
            return end;
        }
        self.skip(p, Tail::Plain)
    }

    /// A function call's arguments: `)`, comments, expressions and commas.
    pub(super) fn in_call(&mut self, p: usize) -> usize {
        if self.s[p] == b')' {
            return self.finish(p, p + 1, S::ParensEnd);
        }
        if let Some(end) = self.comment(p) {
            return end;
        }
        if let Some(end) = self.expression(p) {
            return end;
        }
        if self.s[p] == b',' {
            return self.mark(p, S::Separator);
        }
        self.skip(p, Tail::Plain)
    }

    /// Parentheses: `)`, comments and expressions.
    pub(super) fn in_parens(&mut self, p: usize) -> usize {
        if self.s[p] == b')' {
            return self.finish(p, p + 1, S::ParensEnd);
        }
        if let Some(end) = self.comment(p) {
            return end;
        }
        if let Some(end) = self.expression(p) {
            return end;
        }
        self.skip(p, Tail::Plain)
    }

    /// An inline `if`: expressions, comments, commas and names, up to the
    /// line feed.
    pub(super) fn in_inline_if(&mut self, p: usize) -> usize {
        if let Some(end) = self.expression(p) {
            return end;
        }
        if let Some(end) = self.comment(p) {
            return end;
        }
        if self.s[p] == b',' {
            return self.mark(p, S::Separator);
        }
        if let Some(end) = self.local(p) {
            return end;
        }
        self.skip(p, Tail::Plain)
    }

    /// `(for)\b(.*)\n` in an object or a bracket.
    /// The rest of the line is lexed in an [`Kind::InlineFor`] context that
    /// the line's end closes.
    /// It needs the line feed, so it never matches inside another.
    pub(super) fn inline_for(&mut self, p: usize) -> Option<usize> {
        if self.feed != Feed::Ahead || !self.word_is(p, b"for") {
            return None;
        }
        self.tok_at(p, p + 3, &[S::Control]);
        let outer = self.em.state;
        if self.push(Kind::InlineFor, 0) {
            self.feed = Feed::Capture { outer };
            return Some(p + 3);
        }
        let len = self.s.len();
        self.gap(len);
        self.feed = Feed::Taken;
        Some(len)
    }

    /// `(if)\b` in an object or a bracket, which the line feed ends.
    pub(super) fn inline_if(&mut self, p: usize) -> Option<usize> {
        if !self.word_is(p, b"if") {
            return None;
        }
        Some(self.begin(p, p + 2, Kind::InlineIf, S::Conditional))
    }
}
