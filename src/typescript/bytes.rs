//! Sets of bytes, which say where a rule can start.

/// A set of byte values, built in `const` context.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Bytes([u64; 4]);

impl Bytes {
    pub(crate) const NONE: Bytes = Bytes([0; 4]);

    /// The set with every byte of `list`.
    pub(crate) const fn of(list: &[u8]) -> Bytes {
        let mut set = Bytes::NONE;
        let mut i = 0;
        while i < list.len() {
            set = set.with(list[i]);
            i += 1;
        }
        set
    }

    /// The set with every byte from `low` to `high`.
    pub(crate) const fn range(low: u8, high: u8) -> Bytes {
        let mut set = Bytes::NONE;
        let mut b = low;
        while b < high {
            set = set.with(b);
            b += 1;
        }
        set.with(high)
    }

    #[expect(
        clippy::as_conversions,
        reason = "`usize::from` is not const; the shifted byte is at most 3"
    )]
    pub(crate) const fn with(self, byte: u8) -> Bytes {
        let mut words = self.0;
        words[(byte >> 6) as usize] |= 1 << (byte & 63);
        Bytes(words)
    }

    pub(crate) const fn or(self, other: Bytes) -> Bytes {
        let mut words = self.0;
        let mut i = 0;
        while i < 4 {
            words[i] |= other.0[i];
            i += 1;
        }
        Bytes(words)
    }

    pub(crate) fn has(self, byte: u8) -> bool {
        self.0[usize::from(byte >> 6)] >> (byte & 63) & 1 != 0
    }
}

/// `a`-`z`.
pub(crate) const LOWER: Bytes = Bytes::range(b'a', b'z');

/// `A`-`Z`.
pub(crate) const UPPER: Bytes = Bytes::range(b'A', b'Z');

/// `0`-`9`.
pub(crate) const DIGIT: Bytes = Bytes::range(b'0', b'9');

/// The bytes of a character at or past U+0080, which may be a letter.
pub(crate) const HIGH: Bytes = Bytes::range(0x80, 0xff);

/// What `\s` reads, with the line's newline, and the first bytes of the
/// Unicode spaces it also reads: U+0085 and U+00A0, U+1680, U+2000 to U+200A,
/// U+2028, U+2029, U+202F and U+205F, and U+3000.
pub(crate) const BLANK: Bytes = Bytes::of(b" \t\n\r\x0b\x0c\xc2\xe1\xe2\xe3");

/// Where a name can start: a letter, `$`, `_` or a non-ASCII character.
pub(crate) const NAME_START: Bytes = LOWER.or(UPPER).or(HIGH).or(Bytes::of(b"$_"));

/// `[$_[:alpha:]]` among ASCII bytes: a letter, `$` or `_`.
pub(crate) const ASCII_NAME_START: Bytes = LOWER.or(UPPER).or(Bytes::of(b"$_"));

/// `[$_[:alnum:]]` among ASCII bytes: a name's start or a digit.
pub(crate) const ASCII_NAME_CHAR: Bytes = ASCII_NAME_START.or(DIGIT);

/// Every byte.
pub(crate) const ANY: Bytes = Bytes::range(0, 255);

/// `a`-`z` and `A`-`Z`.
pub(crate) const LETTER: Bytes = LOWER.or(UPPER);

/// Where a string or a template literal can start: a quote, a tag's name or
/// member chain, or the blanks and `<` of a tag that has type arguments.
pub(crate) const STRING_START: Bytes = NAME_START.or(BLANK).or(Bytes::of(b"'\"`.?<"));

/// Where a comment can start: `/`, or the blanks before a `//` at the line's
/// start.
pub(crate) const COMMENT_START: Bytes = Bytes::of(b"/ \t");

/// Where an arrow function can start: its single parameter or `async`, its
/// `=>`, or the blanks, `(` and `<` before the parameters.
pub(crate) const ARROW_START: Bytes = NAME_START.or(BLANK).or(Bytes::of(b"=(<"));

/// Where a call's target can start: a name, a `.member` or the blanks, `?`,
/// `!`, `<` and `(` after a closing parenthesis.
pub(crate) const CALL_START: Bytes = NAME_START.or(BLANK).or(Bytes::of(b".?!<("));

/// Where a literal can start: a number, a keyword such as `true`, or an array
/// after blanks.
pub(crate) const LITERAL_START: Bytes = DIGIT.or(LETTER).or(BLANK).or(Bytes::of(b".["));

/// Where an operator can start: a keyword operator or a symbol.
pub(crate) const OPERATOR_START: Bytes = Bytes::of(b"aydionstv")
    .or(BLANK)
    .or(Bytes::of(b".*-/%+&^|<>!=?~"));

/// Where a class member can start: a modifier or a name, a `*`, a computed or
/// quoted key, a `#` private name, a number, or the blanks before them.
pub(crate) const MEMBER_START: Bytes = NAME_START
    .or(BLANK)
    .or(DIGIT)
    .or(Bytes::of(b"*[#\"'`(<.-+"));
