//! Operators, keywords, hashtable keys and type names.

use super::super::engine::Lexer;
use crate::classes::{is_digit, is_letter, is_number, is_space};
use crate::rules::Chars;

/// The control keywords, lowercase.
const CONTROL: &[&[u8]] = &[
    b"begin",
    b"break",
    b"catch",
    b"clean",
    b"continue",
    b"data",
    b"default",
    b"define",
    b"do",
    b"dynamicparam",
    b"else",
    b"elseif",
    b"end",
    b"exit",
    b"finally",
    b"for",
    b"from",
    b"if",
    b"in",
    b"inlinescript",
    b"parallel",
    b"param",
    b"process",
    b"return",
    b"sequence",
    b"switch",
    b"throw",
    b"trap",
    b"try",
    b"until",
    b"var",
    b"while",
];

/// The comparison operators after `-` and an optional `c` or `i`, in the
/// order the grammar's alternation tries them.
const COMPARISONS: &[&[u8]] = &[
    b"eq",
    b"ne",
    b"ge",
    b"gt",
    b"le",
    b"lt",
    b"notlike",
    b"notmatch",
    b"notcontains",
    b"notin",
    b"like",
    b"match",
    b"contains",
    b"in",
    b"replace",
];

/// The unary operators after `-`, in the grammar's order.
pub(super) const UNARY: &[&[u8]] = &[b"join", b"split"];

/// The logical operators after `-`, in the grammar's order.
pub(super) const LOGICAL: &[&[u8]] = &[b"and", b"or", b"not", b"xor"];

/// The bitwise operators after `-`, in the grammar's order.
pub(super) const BITWISE: &[&[u8]] = &[b"band", b"bor", b"bnot", b"bxor", b"shl", b"shr"];

impl Lexer<'_> {
    /// `[2-6]>&1|>>?|<<|[<>]|>\||[1-6]>|[1-6]>>`: a redirection.
    ///
    /// `>\|` and `[1-6]>>` never match.
    /// `>>?` and `[1-6]>` before them take their first characters.
    pub(super) fn redirection(&self, p: usize) -> Option<usize> {
        match self.at(p) {
            b'2'..=b'6' if self.starts(p + 1, b">&1") => Some(p + 4),
            b'>' => Some(if self.at(p + 1) == b'>' { p + 2 } else { p + 1 }),
            b'<' => Some(if self.at(p + 1) == b'<' { p + 2 } else { p + 1 }),
            b'1'..=b'6' if self.at(p + 1) == b'>' => Some(p + 2),
            _ => None,
        }
    }

    /// `[%?](?!\w)`.
    fn short_keyword(&self, p: usize) -> Option<usize> {
        (matches!(self.at(p), b'%' | b'?') && self.word_at(p + 1).is_none()).then_some(p + 1)
    }

    /// `(?<![-.\w])((?i:begin|…|while)|[%?])(?!\w)`: a control keyword.
    pub(super) fn control(&self, p: usize) -> Option<usize> {
        if matches!(self.prev(p), Some(b'-' | b'.')) || self.word_before(p) {
            return None;
        }
        self.short_keyword(p)
            .or_else(|| self.folded_word(p, CONTROL))
    }

    /// `(?<![-\w]|[^)]\.)((?i:(foreach|where)(?!-object))|[%?])(?!\w)`:
    /// `foreach` or `where` as a keyword.
    ///
    /// After a dot it stands only when a `)` comes before the dot.
    pub(super) fn foreach_where(&self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'-') || self.word_before(p) {
            return None;
        }
        if self.prev_is(p, b'.') && p >= 2 && self.at(p - 2) != b')' {
            return None;
        }
        self.first_folded(p, &[b"foreach", b"where"], |lexer, end| {
            lexer.folded(end, b"-object").is_none() && lexer.word_at(end).is_none()
        })
        .or_else(|| self.short_keyword(p))
    }

    /// `(?<!\w)((?i:hidden|static))(?!\w)`: a modifier.
    pub(super) fn modifier(&self, p: usize) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        self.folded_word(p, &[b"hidden", b"static"])
    }

    /// `(?<!\w)-(?i:is(?:not)?|as)\b`: a type operator.
    pub(super) fn type_operator(&self, p: usize) -> Option<usize> {
        if self.word_before(p) || self.at(p) != b'-' {
            return None;
        }
        self.first_folded(p + 1, &[b"isnot", b"is", b"as"], Chars::boundary)
    }

    /// `(?<!\w)-(?i:[ci]?(?:eq|ne|[gl][et]|(?:not)?(?:
    /// like|match|contains|in)|replace))(?!\p{L})`: a comparison operator.
    ///
    /// The `c` or `i` is tried taken first, then left to the operator, as in
    /// `-in`.
    pub(super) fn comparison(&self, p: usize) -> Option<usize> {
        if self.word_before(p) || self.at(p) != b'-' {
            return None;
        }
        let not_letter = |lexer: &Self, at: usize| !lexer.letter_at(at);
        if matches!(self.at(p + 1), b'c' | b'C' | b'i' | b'I')
            && let Some(end) = self.first_folded(p + 2, COMPARISONS, not_letter)
        {
            return Some(end);
        }
        self.first_folded(p + 1, COMPARISONS, not_letter)
    }

    /// `(?<!\w)-(?i:a|b|…)(?!\p{L})` for the operator words `words`.
    pub(super) fn dash_word(&self, p: usize, words: &[&[u8]]) -> Option<usize> {
        if self.word_before(p) || self.at(p) != b'-' {
            return None;
        }
        self.first_folded(p + 1, words, |lexer, at| !lexer.letter_at(at))
    }

    /// `(?<!\w)-(?i:a|b|…)(?!\p{L})|!` for the operator words `words`.
    pub(super) fn dash_word_or_bang(&self, p: usize, words: &[&[u8]]) -> Option<usize> {
        self.dash_word(p, words)
            .or_else(|| (self.at(p) == b'!').then_some(p + 1))
    }

    /// `[-%*+/]?=|[-%*+/]`: an assignment or an arithmetic operator.
    pub(super) fn assignment(&self, p: usize) -> Option<usize> {
        match self.at(p) {
            b'-' | b'%' | b'*' | b'+' | b'/' => {
                Some(if self.at(p + 1) == b'=' { p + 2 } else { p + 1 })
            }
            b'=' => Some(p + 1),
            _ => None,
        }
    }

    /// `\|{2}|&{2}|;`: a statement terminator.
    pub(super) fn terminator(&self, p: usize) -> Option<usize> {
        self.lit(p, b"||")
            .or_else(|| self.lit(p, b"&&"))
            .or_else(|| self.lit(p, b";"))
    }

    /// ``&|(?<!\w)\.(?= )|[,`|]``: the call, dot-source, comma, backtick and
    /// pipe operators.
    pub(super) fn other_operator(&self, p: usize) -> Option<usize> {
        match self.at(p) {
            b'&' | b',' | b'`' | b'|' => Some(p + 1),
            b'.' if !self.word_before(p) && self.at(p + 1) == b' ' => Some(p + 1),
            _ => None,
        }
    }

    /// `(?<!\s|^)\.\.(?=-?\d|[$(])`: the range operator.
    pub(super) fn range(&self, p: usize) -> Option<usize> {
        if p == 0 || self.char_before(p).is_some_and(is_space) {
            return None;
        }
        let end = self.lit(p, b"..")?;
        let digit = |at: usize| self.char_at(at).is_some_and(|(c, _)| is_digit(c));
        let next = digit(end)
            || (self.at(end) == b'-' && digit(end + 1))
            || matches!(self.at(end), b'$' | b'(');
        next.then_some(end)
    }

    /// `\b(["']?)(\w+)(["']?)\s+{0,1}(=)\s+{0,1}`: a hashtable key and its
    /// `=`, groups 1 to 4 around the quotes, the key and the `=`.
    ///
    /// Giving back characters of the key cannot help.
    /// A word character follows, which is no quote, blank or `=`.
    pub(super) fn key_assignment(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let key_at = if matches!(self.at(p), b'"' | b'\'') {
            p + 1
        } else {
            p
        };
        let key = self.word_end(key_at);
        if key == key_at {
            return None;
        }
        let quoted = if matches!(self.at(key), b'"' | b'\'') {
            key + 1
        } else {
            key
        };
        let equals = self.space_end(quoted);
        if self.at(equals) != b'=' {
            return None;
        }
        self.capture(1, p, key_at);
        self.capture(2, key_at, key);
        self.capture(3, key, quoted);
        self.capture(4, equals, equals + 1);
        Some(self.space_end(equals + 1))
    }

    /// `(?!\d+|\.)[.\p{L}\p{N}]+`: a type's name, which does not start with a
    /// digit or a dot.
    pub(super) fn type_name(&self, p: usize) -> Option<usize> {
        let (c, _) = self.char_at(p)?;
        if c == '.' || is_digit(c) {
            return None;
        }
        let end = self.class_end(p, |c| c == '.' || is_letter(c) || is_number(c));
        (end > p).then_some(end)
    }
}
