//! The operators.

use super::super::chars::{is_arrow, is_ident_rest, is_plus, is_relation, is_times};
use super::super::engine::Lexer;
use crate::classes::{is_alpha, is_letter, is_space, is_word};
use crate::rules::Chars;

/// The updates of `keyword.operator.update` before the plain `=`, in the
/// grammar's order.
const UPDATES: &[&str] = &[
    ":=", "+=", "-=", "*=", "//=", "/=", ".//=", "./=", ".*=", "\\=", ".\\=", "^=", ".^=", "%=",
    ".%=", "÷=", ".÷=", "|=", "&=", ".&=", "⊻=", ".⊻=", "$=", "<<=", ">>=", ">>>=",
];

/// Whether `c` is in `[!.′⁺-ₜ[:word:]]`, the characters a type name goes on
/// with after `::`.
fn is_type_rest(c: char) -> bool {
    matches!(c, '!' | '.' | '′' | '\u{207a}'..='\u{209c}') || is_word(c)
}

/// Whether `c` is in `[]!)}′⁺-ₜ∇[:word:]]`, the characters after which a
/// colon is a range.
pub(super) fn ends_operand(c: char) -> bool {
    matches!(
        c,
        ']' | '!' | ')' | '}' | '′' | '\u{207a}'..='\u{209c}' | '∇'
    ) || is_word(c)
}

impl Lexer<'_> {
    /// `\.?(?:<-->|->|-->|<--|[ARROW]|=>)`.
    /// No alternative starts with a dot, so `\.?` never gives its dot back.
    pub(super) fn arrow(&self, p: usize) -> Option<usize> {
        let q = p + usize::from(self.at(p) == b'.');
        [&b"<-->"[..], b"->", b"-->", b"<--"]
            .iter()
            .find_map(|text| self.lit(q, text))
            .or_else(|| self.class_char(q, is_arrow))
            .or_else(|| self.lit(q, b"=>"))
    }

    /// `:=|\+=|…|>>>=|=(?!=)`: the first of [`UPDATES`] at `p`, or an `=` that
    /// no `=` follows.
    pub(super) fn update(&self, p: usize) -> Option<usize> {
        UPDATES
            .iter()
            .find_map(|text| self.lit(p, text.as_bytes()))
            .or_else(|| (self.at(p) == b'=' && self.at(p + 1) != b'=').then_some(p + 1))
    }

    /// `<<|>>>?|\.>>>?|\.<<`.
    pub(super) fn shift(&self, p: usize) -> Option<usize> {
        let q = p + usize::from(self.at(p) == b'.');
        if self.starts(q, b">>") {
            return Some(q + 2 + usize::from(self.at(q + 2) == b'>'));
        }
        self.lit(q, b"<<")
    }

    /// `\s*([:<>]:)\s*((?:Union)?\([^)]*\)|NAME)(?:\.\.\.)?(\.?'*)`: a type
    /// after `::`, `<:` or `>:`.
    /// Group 1 is the operator, group 2 the type, group 3 the transposes.
    /// Every part after the type's start is optional and greedy, so once the
    /// type starts the match cannot fail.
    pub(super) fn type_relation(&mut self, p: usize) -> Option<usize> {
        let op = self.space_end(p);
        if !matches!(self.at(op), b':' | b'<' | b'>') || self.at(op + 1) != b':' {
            return None;
        }
        let start = self.space_end(op + 2);
        let ty = self.union_type(start).or_else(|| self.named_type(start))?;
        let dots = if self.starts(ty, b"...") { ty + 3 } else { ty };
        let dot = dots + usize::from(self.at(dots) == b'.');
        let end = self.skip(dot, |b| b == b'\'');
        self.cap(1, op, op + 2);
        self.cap(2, start, ty);
        self.cap(3, dots, end);
        Some(end)
    }

    /// `(?:Union)?\([^)]*\)`: parentheses up to the first `)`.
    /// A `Union` that no `(` follows is given back, and `(` cannot stand at
    /// its `U`.
    fn union_type(&self, p: usize) -> Option<usize> {
        let open = if self.starts(p, b"Union") { p + 5 } else { p };
        if self.at(open) != b'(' {
            return None;
        }
        let close = memchr::memchr(b')', self.s.get(open + 1..)?)?;
        Some(open + 2 + close)
    }

    /// `[$_∇[:alpha:]][!.′⁺-ₜ[:word:]]*(?:BRACES|".+?(?<!\\)")?`: a type name,
    /// with its parameters in braces or a string after it.
    /// The braces are tried first.
    fn named_type(&self, p: usize) -> Option<usize> {
        let (c, mut q) = self.char_at(p)?;
        if !(matches!(c, '$' | '_' | '∇') || is_alpha(c)) {
            return None;
        }
        while let Some(end) = self.class_char(q, is_type_rest) {
            q = end;
        }
        self.spend(q - p);
        Some(self.braces(q).or_else(|| self.quoted_type(q)).unwrap_or(q))
    }

    /// `".+?(?<!\\)"`: a string of at least one character, to the first
    /// later `"` that no `\` precedes.
    fn quoted_type(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'"' {
            return None;
        }
        let mut q = self.char_end(p + 1);
        while q < self.lim() {
            if self.at(q) == b'"' && self.at(q - 1) != b'\\' {
                self.spend(q - p);
                return Some(q + 1);
            }
            q += 1;
        }
        None
    }

    /// `(\.?((?<!<)<=|(?<!>)>=|[<>≤≥]|===?|≡|!=|≠|!==|[RELATION]|<:|>:))`.
    /// `!==` comes after `!=`, and `<:` and `>:` after `[<>≤≥]`, so they
    /// never match.
    pub(super) fn relation(&self, p: usize) -> Option<usize> {
        let q = p + usize::from(self.at(p) == b'.');
        let before = q.checked_sub(1).map(|k| self.at(k));
        if self.starts(q, b"<=") && before != Some(b'<') {
            return Some(q + 2);
        }
        if self.starts(q, b">=") && before != Some(b'>') {
            return Some(q + 2);
        }
        if self.starts(q, b"==") {
            return Some(q + 2 + usize::from(self.at(q + 2) == b'='));
        }
        if self.starts(q, b"!=") {
            return Some(q + 2);
        }
        self.class_char(q, |c| {
            matches!(c, '<' | '>' | '≤' | '≥' | '≡' | '≠') || is_relation(c)
        })
    }

    /// `(?<=\s)X(?=\s)`: `text` with white space on both sides.
    pub(super) fn between_spaces(&self, p: usize, text: &[u8]) -> Option<usize> {
        let end = self.lit(p, text)?;
        (self.char_before(p).is_some_and(is_space) && self.space_at(end) > 0).then_some(end)
    }

    /// `\|\||&&|(?<!REST)!`.
    pub(super) fn boolean(&self, p: usize) -> Option<usize> {
        self.lit(p, b"||")
            .or_else(|| self.lit(p, b"&&"))
            .or_else(|| {
                (self.at(p) == b'!' && !self.char_before(p).is_some_and(is_ident_rest))
                    .then_some(p + 1)
            })
    }

    /// `(?<=[]!)}′⁺-ₜ∇[:word:]]):`: a colon after an operand.
    pub(super) fn range_colon(&self, p: usize) -> Option<usize> {
        (self.at(p) == b':' && self.char_before(p).is_some_and(ends_operand)).then_some(p + 1)
    }

    /// `\||\.\||&|\.&|[~¬]|\.~|⊻|\.⊻`.
    /// A leading dot is tried only before `|`, `&`, `~` and `⊻`.
    pub(super) fn bitwise(&self, p: usize) -> Option<usize> {
        let q = p + usize::from(self.at(p) == b'.');
        match self.char_at(q)? {
            ('|' | '&' | '~' | '⊻', end) => Some(end),
            ('¬', end) if q == p => Some(end),
            _ => None,
        }
    }

    /// `\.?(?:\+\+|--|[PLUS]|//?|[TIMES])`.
    pub(super) fn arithmetic(&self, p: usize) -> Option<usize> {
        let q = p + usize::from(self.at(p) == b'.');
        self.lit(q, b"++")
            .or_else(|| self.lit(q, b"--"))
            .or_else(|| self.class_char(q, is_plus))
            .or_else(|| {
                let slash = self.lit(q, b"/")?;
                Some(slash + usize::from(self.at(slash) == b'/'))
            })
            .or_else(|| self.class_char(q, is_times))
    }

    /// `\.(?=[@_\p{L}])|\.\.+|[…⁝⋮-⋱]`.
    pub(super) fn dots(&self, p: usize) -> Option<usize> {
        if self.at(p) == b'.' {
            if self.at(p + 1) == b'.' {
                return Some(self.skip(p, |b| b == b'.'));
            }
            let named = self
                .char_at(p + 1)
                .is_some_and(|(c, _)| matches!(c, '@' | '_') || is_letter(c));
            return named.then_some(p + 1);
        }
        self.class_char(p, |c| matches!(c, '…' | '⁝' | '⋮'..='⋱'))
    }

    /// `\$(?=.+)`: a `$` that a character other than the line feed follows.
    pub(super) fn interpolation_operator(&self, p: usize) -> Option<usize> {
        (self.at(p) == b'$' && p + 1 < self.lim()).then_some(p + 1)
    }

    /// `(IDENT)(('|(\.'))*\.?')`: a transposed variable, whose transposes are
    /// group 2.
    pub(super) fn transposed_variable(&mut self, p: usize) -> Option<usize> {
        let name = self.ident(p)?;
        let end = self.primes(name)?;
        self.cap(2, name, end);
        Some(end)
    }

    /// `(X)((?:\.??')*\.?')`, for `]` or `)`: group 1 is the bracket, group 2
    /// its transposes.
    pub(super) fn transposed(&mut self, p: usize, bracket: u8) -> Option<usize> {
        if self.at(p) != bracket {
            return None;
        }
        let end = self.primes(p + 1)?;
        self.cap(1, p, p + 1);
        self.cap(2, p + 1, end);
        Some(end)
    }
}
