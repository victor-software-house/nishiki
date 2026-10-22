//! The per-line lexer: cursor, token emission and the context stack.

use super::Interner;
use super::scope::S;
use super::state::{Kind, State};
use crate::{Stack, Token};

/// Lexes one line. `pos` is the end of the last token; `s` is scanned ahead
/// of it, and the bytes in between are plain text of the current context.
pub(crate) struct Lexer<'a> {
    pub(crate) s: &'a [u8],
    out: &'a mut Vec<Token>,
    interner: &'a mut Interner,
    pos: usize,
    pub(crate) state: State,
    /// Interned scope stack of each open context, `nodes[depth]` being current.
    nodes: [usize; State::CAPACITY + 1],
    /// End of the ASCII identifier run at the cursor, and of the word run
    /// (which also covers non-ASCII bytes). Set by [`Lexer::scan_word`].
    pub(crate) ident_end: usize,
    pub(crate) word_end: usize,
}

pub(crate) fn is_ws(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\r' | 0x0b | 0x0c)
}

/// ASCII letters, digits and `_`: the character class the grammar spells out.
pub(crate) fn is_ident(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// The regex `\w`, with non-ASCII bytes counted as word characters.
pub(crate) fn is_word(b: u8) -> bool {
    is_ident(b) || b >= 0x80
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(
        interner: &'a mut Interner,
        s: &'a [u8],
        out: &'a mut Vec<Token>,
        state: State,
    ) -> Self {
        let mut nodes = [0; State::CAPACITY + 1];
        for i in 0..state.depth() {
            nodes[i + 1] = match state.kind(i).scope() {
                Some(scope) => interner.child(nodes[i], scope),
                None => nodes[i],
            };
        }
        Lexer {
            s,
            out,
            interner,
            pos: 0,
            state,
            nodes,
            ident_end: 0,
            word_end: 0,
        }
    }

    pub(crate) fn into_state(self) -> State {
        self.state
    }

    pub(crate) fn run(&mut self) {
        let len = self.s.len();
        let mut p = 0;
        while p < len {
            p = match self.state.top() {
                Kind::Str => self.in_string(p),
                Kind::Raw => self.in_raw_string(p),
                Kind::Char => self.in_char(p),
                Kind::Block | Kind::Doc => self.in_block_comment(p),
                Kind::Top
                | Kind::FnDef
                | Kind::Call
                | Kind::Generic
                | Kind::AngleSq
                | Kind::Attr
                | Kind::Use
                | Kind::Import => self.in_code(p),
            };
        }
        self.gap(len);
    }

    // Bytes and boundaries.

    /// The byte at `i`, or 0 past the end.
    pub(crate) fn at(&self, i: usize) -> u8 {
        self.s.get(i).copied().unwrap_or(0)
    }

    pub(crate) fn starts_with(&self, i: usize, prefix: &[u8]) -> bool {
        self.s.get(i..).is_some_and(|rest| rest.starts_with(prefix))
    }

    pub(crate) fn skip(&self, mut i: usize, class: impl Fn(u8) -> bool) -> usize {
        while i < self.s.len() && class(self.s[i]) {
            i += 1;
        }
        i
    }

    /// The regex `\b` at `i`.
    pub(crate) fn boundary(&self, i: usize) -> bool {
        let before = i > 0 && is_word(self.s[i - 1]);
        let after = i < self.s.len() && is_word(self.s[i]);
        before != after
    }

    /// Whether a word starts at `i`, given that the byte there is a word byte.
    pub(crate) fn word_start(&self, i: usize) -> bool {
        i == 0 || !is_word(self.s[i - 1])
    }

    /// Records the identifier and word runs starting at `p`.
    pub(crate) fn scan_word(&mut self, p: usize) {
        self.ident_end = self.skip(p, is_ident);
        self.word_end = if self.at(self.ident_end) >= 0x80 {
            self.skip(self.ident_end, is_word)
        } else {
            self.ident_end
        };
    }

    /// Whether the word at the cursor is exactly `word`.
    pub(crate) fn word_is(&self, p: usize, word: &[u8]) -> bool {
        self.word_end == p + word.len() && self.s[p..self.word_end] == *word
    }

    // Emission.

    /// Emits plain text of the current context up to `to`.
    pub(crate) fn gap(&mut self, to: usize) {
        self.tok(to, &[]);
    }

    /// Emits the bytes up to `to` with the current context plus `leaf`.
    pub(crate) fn tok(&mut self, to: usize, leaf: &[S]) {
        if to <= self.pos {
            return;
        }
        let node = self.nodes[self.state.depth()];
        let stack = Stack(self.interner.path(node, leaf));
        self.out.push(Token { end: to, stack });
        self.pos = to;
    }

    // Contexts.

    /// Opens a context; false when the stack is full. Call [`Lexer::gap`]
    /// first.
    pub(crate) fn push(&mut self, kind: Kind) -> bool {
        if !self.state.push(kind) {
            return false;
        }
        let depth = self.state.depth();
        let parent = self.nodes[depth - 1];
        self.nodes[depth] = match kind.scope() {
            Some(scope) => self.interner.child(parent, scope),
            None => parent,
        };
        true
    }

    pub(crate) fn pop(&mut self) {
        self.state.pop();
    }

    // Code.

    fn in_code(&mut self, p: usize) -> usize {
        let b = self.s[p];
        if is_ws(b) {
            return self.whitespace(p);
        }
        if let Some(end) = self.end_of_context(p, b) {
            return end;
        }
        if is_ident(b) {
            self.scan_word(p);
        }
        let kind = self.state.top();
        if kind == Kind::Top
            && let Some(end) = self.top_rules(p, b)
        {
            return end;
        }
        for &group in super::code::groups(kind) {
            if let Some(end) = self.group(group, p, b) {
                return end;
            }
        }
        p + 1
    }

    /// Skips blanks. A blank run after a word may start the comparison rule.
    fn whitespace(&mut self, p: usize) -> usize {
        if p > 0
            && is_word(self.s[p - 1])
            && super::code::groups(self.state.top()).contains(&super::code::Group::Keywords)
            && let Some(end) = self.comparison(p)
        {
            return end;
        }
        self.skip(p, is_ws)
    }

    /// Closes the innermost code context when its end rule matches at `p`.
    fn end_of_context(&mut self, p: usize, b: u8) -> Option<usize> {
        let leaf = match (self.state.top(), b) {
            (Kind::FnDef, b'{') => S::Curly,
            (Kind::FnDef | Kind::Use | Kind::Import, b';') => S::Semi,
            (Kind::Call, b')') => S::Round,
            (Kind::Generic | Kind::AngleSq, b'>') => S::Angle,
            (Kind::Attr, b']') => S::AttrBracket,
            _ => return None,
        };
        self.gap(p);
        self.tok(p + 1, &[leaf]);
        self.pop();
        Some(p + 1)
    }
}
