//! The per-line lexer: cursor, token emission and the context stack.

use memchr::{memchr2, memmem};

use super::Dialect;
use super::scope::S;
use super::state::Kind;
use crate::classes::{digit_width, is_ascii_space, space_width, word_before, word_width};
use crate::emit::Emitter;
use crate::language::Context;
use crate::limits::{Limit, Steps};
use crate::utf8::char_at;

/// Lexes one line. `pos` is the end of the last token; the bytes between it
/// and the cursor are plain text of the current context.
pub(crate) struct Lexer<'a> {
    s: &'a [u8],
    /// The tokens, the contexts and the end of the last token.
    pub(super) em: Emitter<'a>,
    dialect: Dialect,
    /// The work this line has spent and may spend.
    pub(super) steps: Steps,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(cx: &'a mut Context<'_>, dialect: Dialect, s: &'a [u8], state: u32) -> Self {
        let steps = Steps::new(cx.limits, s.len());
        Lexer {
            s,
            em: Emitter::new(cx, state, 0),
            dialect,
            steps,
        }
    }

    pub(crate) fn into_state(self) -> u32 {
        self.em.state
    }

    pub(crate) fn run(&mut self) {
        let len = self.s.len();
        let mut p = 0;
        while p < len {
            self.steps.spend(1);
            if self.steps.exhausted() {
                self.em.report.record(Limit::Work);
                break;
            }
            let from = p;
            p = match self.top() {
                Kind::Str | Kind::Key => self.in_string(p),
                Kind::Block | Kind::Doc => self.in_comment(p),
                Kind::Top | Kind::Array | Kind::Object | Kind::Pair => self.in_code(p),
            };
            debug_assert!(p > from, "the JSON lexer made no progress at byte {from}");
        }
        self.gap(len);
    }

    // Bytes.

    /// The byte at `i`, or 0 past the end.
    fn at(&self, i: usize) -> u8 {
        self.s.get(i).copied().unwrap_or(0)
    }

    /// The width of the character at `i`, at least 1.
    fn width_at(&self, i: usize) -> usize {
        char_at(self.s, i).map_or(1, |(_, width)| width)
    }

    // Emission.

    /// Emits plain text of the current context up to `to`.
    fn gap(&mut self, to: usize) {
        self.tok(to, &[]);
    }

    /// Emits the bytes up to `to` with the current context plus `leaf`.
    fn tok(&mut self, to: usize, leaf: &[S]) {
        self.em
            .tok(to, leaf.iter().map(|scope| scope.scope(self.dialect)));
    }

    // Contexts.

    fn top(&self) -> Kind {
        Kind::from_code(self.em.frame.kind)
    }

    /// Opens a context; false when a limit keeps it closed.
    fn push(&mut self, kind: Kind) -> bool {
        let dialect = self.dialect;
        let scopes = kind.scopes().iter().map(|scope| scope.scope(dialect));
        self.em.push(kind.code(), 0, scopes)
    }

    /// Opens `kind` at `p`, whose begin text ends at `end` and carries
    /// `leaf`; the text stays plain when a limit keeps the context closed.
    fn begin(&mut self, p: usize, end: usize, kind: Kind, leaf: S) -> usize {
        self.gap(p);
        if self.push(kind) {
            self.tok(end, &[leaf]);
        }
        end
    }

    /// Emits the end text up to `end` with `leaf` and closes the context.
    fn finish(&mut self, p: usize, end: usize, leaf: S) -> usize {
        self.gap(p);
        self.tok(end, &[leaf]);
        self.em.pop();
        end
    }

    // Code.

    /// Takes the next match at or after `p` in an array, object, value or
    /// document context.
    fn in_code(&mut self, p: usize) -> usize {
        let b = self.s[p];
        if b < 0x80 {
            if is_ascii_space(b) {
                return self.skip_space(p);
            }
        } else {
            let width = space_width(self.s, p);
            if width > 0 {
                return self.skip_space(p);
            }
        }
        match self.top() {
            Kind::Top => self.value(p, b).unwrap_or(p + self.width_at(p)),
            Kind::Array => self.in_array(p, b),
            Kind::Object => self.in_object(p, b),
            // Strings and comments take their text in `run`, never here.
            Kind::Pair | Kind::Key | Kind::Str | Kind::Block | Kind::Doc => self.in_pair(p, b),
        }
    }

    fn skip_space(&mut self, mut p: usize) -> usize {
        let from = p;
        while p < self.s.len() {
            let width = if self.s[p] < 0x80 {
                usize::from(is_ascii_space(self.s[p]))
            } else {
                space_width(self.s, p)
            };
            if width == 0 {
                break;
            }
            p += width;
        }
        self.steps.spend(p - from);
        p
    }

    /// An invalid character `[^…\s]`: one whole character.
    fn invalid(&mut self, p: usize, scope: S) -> usize {
        let end = p + self.width_at(p);
        self.gap(p);
        self.tok(end, &[scope]);
        end
    }

    fn in_array(&mut self, p: usize, b: u8) -> usize {
        if b == b']' {
            return self.finish(p, p + 1, S::PunctArrayEnd);
        }
        if let Some(end) = self.value(p, b) {
            return end;
        }
        if b == b',' {
            self.gap(p);
            self.tok(p + 1, &[S::PunctArraySep]);
            return p + 1;
        }
        self.invalid(p, S::InvalidArraySep)
    }

    fn in_object(&mut self, p: usize, b: u8) -> usize {
        match b {
            b'}' => self.finish(p, p + 1, S::PunctDictEnd),
            b'"' => self.begin(p, p + 1, Kind::Key, S::PunctKeyBegin),
            b'/' => self
                .comment(p)
                .unwrap_or_else(|| self.invalid(p, S::InvalidDictSep)),
            b':' => self.begin(p, p + 1, Kind::Pair, S::PunctKeyValue),
            _ => self.invalid(p, S::InvalidDictSep),
        }
    }

    /// In the value after a colon, `,` and the `}` that follows end it.
    fn in_pair(&mut self, p: usize, b: u8) -> usize {
        match b {
            b',' => self.finish(p, p + 1, S::PunctPair),
            b'}' => {
                self.gap(p);
                self.em.pop();
                self.in_code(p)
            }
            _ => self
                .value(p, b)
                .unwrap_or_else(|| self.invalid(p, S::InvalidDictSep)),
        }
    }

    /// The `value` rules at `p`, where the byte is `b`: a constant, number,
    /// string, array, object or comment.
    fn value(&mut self, p: usize, b: u8) -> Option<usize> {
        match b {
            b'"' => Some(self.begin(p, p + 1, Kind::Str, S::PunctStrBegin)),
            b'[' => Some(self.begin(p, p + 1, Kind::Array, S::PunctArrayBegin)),
            b'{' => Some(self.begin(p, p + 1, Kind::Object, S::PunctDictBegin)),
            b'/' => self.comment(p),
            b'-' | b'0'..=b'9' => self.number(p),
            b't' | b'f' | b'n' => self.constant(p),
            _ => None,
        }
    }

    /// `-?(?:0|[1-9]\d*)(?:(?:\.\d+)?(?:[Ee][-+]?\d+)?)?`.
    fn number(&mut self, p: usize) -> Option<usize> {
        let mut i = p + usize::from(self.at(p) == b'-');
        match self.at(i) {
            b'0' => i += 1,
            b'1'..=b'9' => i = self.digits(i),
            _ => return None,
        }
        if self.at(i) == b'.' {
            let end = self.digits(i + 1);
            if end > i + 1 {
                i = end;
            }
        }
        if matches!(self.at(i), b'E' | b'e') {
            let from = i + 1 + usize::from(matches!(self.at(i + 1), b'-' | b'+'));
            let end = self.digits(from);
            if end > from {
                i = end;
            }
        }
        self.steps.spend(i - p);
        self.gap(p);
        self.tok(i, &[S::Number]);
        Some(i)
    }

    /// The end of the run of `\d` characters from `i`.
    fn digits(&self, mut i: usize) -> usize {
        while let width @ 1.. = digit_width(self.s, i) {
            i += width;
        }
        i
    }

    /// `\b(?:true|false|null)\b`.
    fn constant(&mut self, p: usize) -> Option<usize> {
        let word: &[u8] = match self.at(p) {
            b't' => b"true",
            b'f' => b"false",
            _ => b"null",
        };
        let end = p + word.len();
        if self.s.get(p..end) != Some(word) || word_before(self.s, p) || word_width(self.s, end) > 0
        {
            return None;
        }
        self.gap(p);
        self.tok(end, &[S::Constant]);
        Some(end)
    }

    /// `/**`, `/*` and `//` comments at `p`, where the byte is `/`.
    fn comment(&mut self, p: usize) -> Option<usize> {
        match (self.at(p + 1), self.at(p + 2)) {
            (b'*', b'*') if self.at(p + 3) != b'/' => {
                Some(self.begin(p, p + 3, Kind::Doc, S::PunctComment))
            }
            (b'*', _) => Some(self.begin(p, p + 2, Kind::Block, S::PunctComment)),
            (b'/', _) => {
                let len = self.s.len();
                self.gap(p);
                self.tok(p + 2, &[S::CommentLine, S::PunctComment]);
                self.tok(len, &[S::CommentLine]);
                Some(len)
            }
            _ => None,
        }
    }

    // Strings and comments.

    /// Takes the text of a string up to its next escape or closing quote.
    fn in_string(&mut self, p: usize) -> usize {
        let Some(offset) = memchr2(b'"', b'\\', &self.s[p..]) else {
            self.steps.spend(self.s.len() - p);
            return self.s.len();
        };
        let q = p + offset;
        self.steps.spend(offset);
        if self.s[q] == b'"' {
            let leaf = if self.top() == Kind::Key {
                S::PunctKeyEnd
            } else {
                S::PunctStrEnd
            };
            return self.finish(q, q + 1, leaf);
        }
        // `\\(?:["/\\bfnrt]|u\h{4})` is an escape; `\\.` any other one,
        // which a backslash at the end of the line is not.
        let escape = match self.at(q + 1) {
            b'"' | b'/' | b'\\' | b'b' | b'f' | b'n' | b'r' | b't' => Some((2, S::Escape)),
            b'u' if (2..6).all(|k| self.at(q + k).is_ascii_hexdigit()) => Some((6, S::Escape)),
            _ if q + 1 < self.s.len() => Some((1 + self.width_at(q + 1), S::InvalidEscape)),
            _ => None,
        };
        let Some((len, scope)) = escape else {
            return self.s.len();
        };
        self.gap(q);
        self.tok(q + len, &[scope]);
        q + len
    }

    /// Takes the text of a block comment up to its `*/`.
    fn in_comment(&mut self, p: usize) -> usize {
        let Some(offset) = memmem::find(&self.s[p..], b"*/") else {
            self.steps.spend(self.s.len() - p);
            return self.s.len();
        };
        self.steps.spend(offset);
        self.finish(p + offset, p + offset + 2, S::PunctComment)
    }
}
