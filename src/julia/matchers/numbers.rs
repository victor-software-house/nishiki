//! Numbers.

use super::super::chars::is_ident_rest;
use super::super::engine::Lexer;
use crate::rules::Chars;

/// The words of `constant.global`.
pub(super) const GLOBALS: &[&[u8]] = &[
    b"ARGS",
    b"C_NULL",
    b"DEPOT_PATH",
    b"ENDIAN_BOM",
    b"ENV",
    b"LOAD_PATH",
    b"PROGRAM_FILE",
    b"stdin",
    b"stdout",
    b"stderr",
    b"VERSION",
    b"devnull",
];

/// The bit widths `Inf` and `NaN` can end in.
const WIDTHS: [&[u8]; 3] = [b"16", b"32", b"64"];

impl Lexer<'_> {
    /// A number, which is group 1, and the transposes after it, group 2:
    /// - `(?<!REST)\b`;
    /// - `HEX|OCTAL|BINARY|DECIMAL|[0-9]+`;
    /// - `Inf…\b|NaN…\b|π\b|pi\b|ℯ\b`, the named numbers;
    /// - `('*)`, the transposes.
    ///
    /// `REST` holds every word character.
    /// So `\b` holds only before a word character here.
    /// The decimal alternative `\.[0-9]…` starts with a dot and never
    /// matches.
    /// `[0-9]+` comes after the decimal alternative, which matches wherever
    /// a digit stands.
    pub(super) fn number(&mut self, p: usize) -> Option<usize> {
        if self.char_before(p).is_some_and(is_ident_rest) || !self.boundary(p) {
            return None;
        }
        let end = self
            .radix(p, b"0X", u8::is_ascii_hexdigit)
            .or_else(|| self.radix(p, b"0x", u8::is_ascii_hexdigit))
            .or_else(|| self.radix(p, b"0o", |&b| matches!(b, b'0'..=b'7')))
            .or_else(|| self.radix(p, b"0b", |&b| matches!(b, b'0' | b'1')))
            .or_else(|| self.decimal(p))
            .or_else(|| self.float_name(p))?;
        let primes = self.skip(end, |b| b == b'\'');
        self.capture(1, p, end);
        self.capture(2, end, primes);
        Some(primes)
    }

    /// `prefix D(?:_?D)*`, for the digits `D` of a radix.
    fn radix(&self, p: usize, prefix: &[u8], digit: fn(&u8) -> bool) -> Option<usize> {
        let first = self.lit(p, prefix)?;
        digit(&self.at(first)).then(|| self.digit_run(first, digit))
    }

    /// `D(?:_?D)*` from `p`, where a digit stands: digits with single
    /// underscores between them.
    fn digit_run(&self, p: usize, digit: fn(&u8) -> bool) -> usize {
        let mut q = p + 1;
        loop {
            if digit(&self.at(q)) {
                q += 1;
            } else if self.at(q) == b'_' && digit(&self.at(q + 1)) {
                q += 2;
            } else {
                break;
            }
        }
        self.spend(q - p);
        q
    }

    /// `[0-9](?:_?[0-9])*\.?(?!\.)[0-9_]*` with the optional exponent and
    /// suffix after it.
    ///
    /// Before `..` the dot is not taken.
    /// The digits then give one back for `(?!\.)`, and `[0-9_]*` takes it
    /// again, so the match ends before the dots either way.
    /// With a single digit the alternative fails there, and `[0-9]+` matches
    /// the same digit.
    fn decimal(&self, p: usize) -> Option<usize> {
        if !self.at(p).is_ascii_digit() {
            return None;
        }
        let digits = self.digit_run(p, u8::is_ascii_digit);
        let dot = digits + usize::from(self.at(digits) == b'.' && self.at(digits + 1) != b'.');
        let fraction = self.skip(dot, |b| b.is_ascii_digit() || b == b'_');
        let exponent = self.exponent(fraction).unwrap_or(fraction);
        Some(
            self.word(exponent, b"im")
                .or_else(|| self.float_name(exponent))
                .unwrap_or(exponent),
        )
    }

    /// `[Eef][-+]?[0-9](?:_?[0-9])*`.
    fn exponent(&self, p: usize) -> Option<usize> {
        if !matches!(self.at(p), b'E' | b'e' | b'f') {
            return None;
        }
        let sign = p + 1 + usize::from(matches!(self.at(p + 1), b'-' | b'+'));
        self.at(sign)
            .is_ascii_digit()
            .then(|| self.digit_run(sign, u8::is_ascii_digit))
    }

    /// `text\b`, after a word character.
    fn word(&self, p: usize, text: &[u8]) -> Option<usize> {
        let end = self.lit(p, text)?;
        self.word_at(end).is_none().then_some(end)
    }

    /// `Inf(?:16|32|64)?\b|NaN(?:16|32|64)?\b|π\b|pi\b|ℯ\b`.
    /// A width that `\b` does not end is given back.
    fn float_name(&self, p: usize) -> Option<usize> {
        [&b"Inf"[..], b"NaN"]
            .iter()
            .find_map(|name| {
                let end = self.lit(p, name)?;
                WIDTHS
                    .iter()
                    .find_map(|width| self.word(end, width))
                    .or_else(|| self.word(end, b""))
            })
            .or_else(|| self.word(p, "π".as_bytes()))
            .or_else(|| self.word(p, b"pi"))
            .or_else(|| self.word(p, "ℯ".as_bytes()))
    }
}
