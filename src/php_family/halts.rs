//! Where SQL that a string embeds hands the line back, in the ways PHP's and
//! Hack's grammars share.

/// The patterns that a SQL string of either quote starts with: `#` and `--`
/// comments, and the escape ``\\["'\\`]``.
pub(crate) fn sql_common(s: &[u8], p: usize) -> bool {
    match s.get(p) {
        Some(b'-') => s.get(p + 1) == Some(&b'-'),
        Some(b'\\') => matches!(s.get(p + 1), Some(b'"' | b'\'' | b'\\' | b'`')),
        _ => false,
    }
}

/// `(?=((\\q)|[^oq])*(o|$))` from `at`, after a quote `q` in a SQL string
/// whose own quote is `o`.
/// It holds when the rest of the string has no unescaped `q` before its end
/// or the line's.
pub(crate) fn quoted_to(s: &[u8], mut at: usize, quote: u8, own: u8) -> bool {
    loop {
        match s.get(at) {
            Some(&b'\\') if s.get(at + 1) == Some(&quote) => at += 2,
            Some(&b) if b == own => return true,
            Some(&b) if b == quote => return false,
            Some(b'\n') if at + 1 == s.len() => return true,
            Some(_) => at += 1,
            None => return true,
        }
    }
}
