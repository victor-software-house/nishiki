//! Keywords and operators: the grammar's `keywords` group.

use super::lexer::{Lexer, is_ident};
use super::scope::S;

/// The scopes of a word keyword, or none when the word is not one.
fn keyword(word: &[u8]) -> Option<&'static [S]> {
    // Every keyword is two to eight lowercase letters.
    if !(2..=8).contains(&word.len()) || !word[0].is_ascii_lowercase() {
        return None;
    }
    Some(match word {
        b"await" | b"break" | b"continue" | b"do" | b"else" | b"for" | b"if" | b"loop"
        | b"match" | b"return" | b"try" | b"while" | b"yield" => &[S::KwControl],
        b"extern" | b"let" | b"macro" | b"mod" => &[S::KwOther, S::StorageType],
        b"const" | b"abstract" | b"static" => &[S::StorageModifier],
        b"type" => &[S::KwDeclType, S::StorageType],
        b"enum" => &[S::KwDeclEnum, S::StorageType],
        b"trait" => &[S::KwDeclTrait, S::StorageType],
        b"struct" => &[S::KwDeclStruct, S::StorageType],
        b"as" | b"async" | b"become" | b"box" | b"dyn" | b"move" | b"final" | b"gen" | b"impl"
        | b"in" | b"override" | b"priv" | b"ref" | b"typeof" | b"union" | b"unsafe"
        | b"unsized" | b"use" | b"virtual" | b"where" => &[S::KwOther],
        b"fn" => &[S::KwFn],
        b"crate" => &[S::KwCrate],
        b"mut" => &[S::StorageMut],
        _ => return None,
    })
}

fn is_blank(b: u8) -> bool {
    matches!(b, b' ' | b'\t')
}

impl Lexer<'_> {
    pub(crate) fn keywords(&mut self, p: usize, b: u8) -> Option<usize> {
        if is_ident(b) {
            if !self.word_start(p) {
                return None;
            }
            let leaf = keyword(&self.s[p..self.word_end])?;
            self.gap(p);
            self.tok(self.word_end, leaf);
            return Some(self.word_end);
        }
        if matches!(b, b')' | b']' | b'}') {
            return self.comparison(p);
        }
        let (len, leaf) = self.operator(p, b)?;
        self.gap(p);
        self.tok(p + len, &[leaf]);
        Some(p + len)
    }

    /// The length and scope of the operator at `p`, in the grammar's order.
    fn operator(&self, p: usize, b: u8) -> Option<(usize, S)> {
        let (c1, c2) = (self.at(p + 1), self.at(p + 2));
        let not_eq = |c: u8| c != b'=';
        // Logical: `^`, `|`, `&&`, `<<`, `>>` and `!`, none followed by `=`.
        if matches!(b, b'^' | b'|' | b'!') && not_eq(c1) {
            return Some((1, S::OpLogical));
        }
        if matches!((b, c1), (b'&', b'&') | (b'<', b'<') | (b'>', b'>')) && not_eq(c2) {
            return Some((2, S::OpLogical));
        }
        if b == b'&' && !matches!(c1, b'&' | b'=') {
            return Some((1, S::OpBorrowAnd));
        }
        // Compound assignment.
        if matches!(b, b'-' | b'%' | b'&' | b'*' | b'+' | b'/' | b'^' | b'|') && c1 == b'=' {
            return Some((2, S::OpAssign));
        }
        if matches!((b, c1, c2), (b'<', b'<', b'=') | (b'>', b'>', b'=')) {
            return Some((3, S::OpAssign));
        }
        if b == b'='
            && !matches!(p.checked_sub(1).map(|i| self.s[i]), Some(b'<' | b'>'))
            && !matches!(c1, b'=' | b'>')
        {
            return Some((1, S::OpAssignEq));
        }
        // Comparison.
        match (b, c1) {
            (b'=', b'=') if c2 != b'>' => return Some((2, S::OpCompare)),
            (b'=', _) if c1 != b'>' => return Some((1, S::OpCompare)),
            (b'!' | b'<', b'=') => return Some((2, S::OpCompare)),
            (b'>', b'=') if p == 0 || self.s[p - 1] != b'=' => return Some((2, S::OpCompare)),
            _ => {}
        }
        // Math.
        let math = match b {
            b'%' | b'+' => not_eq(c1),
            b'*' => !self.word_at(p + 1) && not_eq(c1),
            b'-' => c1 != b'>',
            b'/' => c1 != b'/',
            _ => false,
        };
        if math {
            return Some((1, S::OpMath));
        }
        if matches!(b, b')' | b']' | b'}') {
            // Handled by the caller, which needs to emit several tokens.
            return None;
        }
        match (b, c1) {
            (b':', b':') => Some((2, S::OpNamespace)),
            (b'*', _) if self.word_at(p + 1) => Some((1, S::OpDeref)),
            (b'@', _) => Some((1, S::OpSubpattern)),
            (b'.', c) if c != b'.' => Some((1, S::OpDot)),
            (b'.', b'.') => {
                let len = 2 + usize::from(matches!(c2, b'.' | b'='));
                Some((len, S::OpRange))
            }
            (b':', _) => Some((1, S::OpKeyValue)),
            (b'-', b'>') | (b'<', b'-') => Some((2, S::OpArrowSkinny)),
            (b'=', b'>') => Some((2, S::OpArrowFat)),
            (b'$', _) => Some((1, S::OpMacroDollar)),
            (b'?', _) => Some((1, S::OpQuestion)),
            _ => None,
        }
    }

    /// A closing bracket, or the blanks after a word, then `<` or `>` between
    /// blanks, then a word or an opening bracket. Only the operator and the
    /// brackets are scoped.
    pub(crate) fn comparison(&mut self, p: usize) -> Option<usize> {
        let bracket = |b: u8| match b {
            b')' | b'(' => Some(S::Round),
            b']' | b'[' => Some(S::Square),
            b'}' | b'{' => Some(S::Curly),
            _ => None,
        };
        let left = bracket(self.at(p)).filter(|_| matches!(self.at(p), b')' | b']' | b'}'));
        let blanks = p + usize::from(left.is_some());
        let op = self.skip(blanks, is_blank);
        if op == blanks || !matches!(self.at(op), b'<' | b'>') {
            return None;
        }
        let right_start = self.skip(op + 1, is_blank);
        if right_start == op + 1 {
            return None;
        }
        let next = self.at(right_start);
        let right = bracket(next).filter(|_| matches!(next, b'(' | b'[' | b'{'));
        if right.is_none() && !(self.word_at(right_start) && right_start < self.s.len()) {
            return None;
        }
        self.gap(p);
        if let Some(scope) = left {
            self.tok(p + 1, &[scope]);
        }
        self.tok(op, &[]);
        self.tok(op + 1, &[S::OpCompare]);
        let end = match right {
            Some(scope) => {
                self.tok(right_start, &[]);
                self.tok(right_start + 1, &[scope]);
                right_start + 1
            }
            None => right_start,
        };
        Some(end)
    }
}
