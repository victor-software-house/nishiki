//! The lookahead that opens a variable definition:
//!
//! `(?=\b((void|boolean|byte|char|short|int|float|long|double)|`
//! `(?>(\w+\.)*[A-Z_]+\w*))\b\s*(<[],.<>?\[\w\s]*>)?\s*((\[])*)?\s+`
//! `[$A-Z_a-z][$\w]*([]$,\[\w][],\[\w\s]*)?\s*([:;=]))`

use crate::java::engine::Lexer;
use crate::rules::Chars;

impl Lexer<'_> {
    /// Where the type at `p` ends: a primitive type, or dotted names ending
    /// in one that starts with a capital or an underscore.
    /// The second alternative is atomic, so only its longest match counts.
    fn type_ends(&self, p: usize) -> [Option<usize>; 2] {
        let primitive = [
            &b"void"[..],
            b"boolean",
            b"byte",
            b"char",
            b"short",
            b"int",
            b"float",
            b"long",
            b"double",
        ]
        .iter()
        .find_map(|word| self.lit(p, word).filter(|&end| self.word_at(end).is_none()));
        let mut qualified = None;
        let mut i = p;
        loop {
            let b = self.at(i);
            if i < self.lim() && (b.is_ascii_uppercase() || b == b'_') {
                qualified = Some(self.word_end(i));
            }
            let word = self.word_end(i);
            if word == i || self.at(word) != b'.' {
                break;
            }
            i = word + 1;
        }
        [primitive, qualified]
    }

    /// The `[],.<>?\[\w\s]` characters of a generic argument list.
    fn generic_char(&self, i: usize) -> usize {
        match self.at(i) {
            _ if i > self.lim() => 0,
            b']' | b',' | b'.' | b'<' | b'>' | b'?' | b'[' if i < self.lim() => 1,
            _ => {
                let width = self.space_at(i);
                if width > 0 {
                    width
                } else {
                    self.word_at(i).unwrap_or(0)
                }
            }
        }
    }

    /// `\s*(<...>)?\s*((\[\])*)?\s+` after a type that ends at `end`, then
    /// the name and what ends the definition.
    ///
    /// `>` can be any of the closing characters in the run, the last first.
    /// The run is the same for every `<` in it.
    /// So is the last `>` that works.
    fn after_type(&self, end: usize) -> bool {
        let angle = self.space_end(end);
        if self.at(angle) == b'<' {
            let (start, run_end, best) = self.x.memo.generics.get();
            let best = if start <= angle && angle < run_end {
                best
            } else {
                let mut run_end = angle + 1;
                loop {
                    let width = self.generic_char(run_end);
                    if width == 0 {
                        break;
                    }
                    run_end += width;
                }
                self.spend(run_end - angle);
                let mut close = run_end;
                let mut best = None;
                while close > angle + 1 {
                    close -= 1;
                    if self.s.get(close) == Some(&b'>') && self.after_generics(close + 1) {
                        best = Some(close);
                        break;
                    }
                }
                self.x.memo.generics.set((angle, run_end, best));
                best
            };
            if best.is_some_and(|close| close > angle) {
                return true;
            }
        }
        self.after_generics(end)
    }

    /// `\s*((\[\])*)?\s+` from `i`, then the name.
    fn after_generics(&self, i: usize) -> bool {
        let blanks = self.space_end(i);
        let mut pairs = blanks;
        while self.starts(pairs, b"[]") {
            pairs += 2;
        }
        let name = if pairs == blanks {
            if blanks == i {
                return false;
            }
            blanks
        } else {
            let after = self.space_end(pairs);
            if after == pairs {
                return false;
            }
            after
        };
        self.definition_tail(name)
    }

    /// `[$A-Z_a-z][$\w]*([]$,\[\w][],\[\w\s]*)?\s*([:;=])` at `name`.
    ///
    /// It reads the run of characters that the name and the group after it
    /// can be made of, with the last `$` in it.
    /// The name takes its `[$\w]*`.
    /// The group takes the rest, from a character that is not a `$` onward.
    /// Only blanks after the name are the `\s*` that ends the lookahead.
    fn definition_tail(&self, name: usize) -> bool {
        let b = self.at(name);
        if name >= self.lim() || !(b.is_ascii_alphabetic() || b == b'_' || b == b'$') {
            return false;
        }
        let (start, end, dollar) = self.x.memo.names.get();
        let (end, dollar) = if start <= name && name < end {
            (end, dollar)
        } else {
            let mut e = name;
            let mut last_dollar = None;
            while e < self.lim() {
                let b = self.s[e];
                if b == b'$' {
                    last_dollar = Some(e);
                    e += 1;
                } else if matches!(b, b']' | b'[' | b',') {
                    e += 1;
                } else if let Some(width) = self.word_at(e) {
                    e += width;
                } else {
                    let width = self.space_at(e);
                    if width == 0 {
                        break;
                    }
                    e += width;
                }
            }
            self.spend(e - name);
            self.x.memo.names.set((name, e, last_dollar));
            (e, last_dollar)
        };
        if !matches!(self.at(end), b':' | b';' | b'=') || end >= self.lim() {
            return false;
        }
        let plain = self.name_end(name + 1);
        if plain >= end || self.space_end(plain) >= end {
            return true;
        }
        let group = match self.at(plain) {
            b']' | b'[' | b',' => plain,
            _ if plain >= name + 2 => plain - 1,
            _ => return false,
        };
        dollar.is_none_or(|d| d <= group)
    }

    /// The whole lookahead, at `p`.
    pub(super) fn variable_ahead(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        self.type_ends(p)
            .into_iter()
            .flatten()
            .any(|end| self.after_type(end))
            .then_some(p)
    }
}
