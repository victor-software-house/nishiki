//! Which plain scalars the grammar scopes as constants, numbers or dates.
//!
//! The grammar follows the YAML 1.1 type tags, whose patterns are public.
//! The rules below are those patterns as Shiki's output shows them.
//! For example, `0o17` is not a number, and an exponent needs its sign.
//! A scalar is a constant only when the whole of it matches.

use super::scope::S;

/// The scope of the constant a plain scalar spells, if it spells one.
pub(crate) fn constant(s: &[u8]) -> Option<S> {
    match s {
        b"~" | b"null" | b"Null" | b"NULL" => Some(S::Null),
        b"<<" => Some(S::Merge),
        b"=" => Some(S::Value),
        b"y" | b"Y" | b"yes" | b"Yes" | b"YES" | b"n" | b"N" | b"no" | b"No" | b"NO" | b"true"
        | b"True" | b"TRUE" | b"false" | b"False" | b"FALSE" | b"on" | b"On" | b"ON" | b"off"
        | b"Off" | b"OFF" => Some(S::Boolean),
        _ if is_int(s) => Some(S::Integer),
        _ if is_float(s) => Some(S::Float),
        _ if is_timestamp(s) => Some(S::Timestamp),
        _ => None,
    }
}

fn digits(s: &[u8], class: fn(u8) -> bool) -> bool {
    !s.is_empty() && s.iter().all(|&b| class(b))
}

fn is_digit(b: u8) -> bool {
    b.is_ascii_digit()
}

fn is_digit_or_underscore(b: u8) -> bool {
    b.is_ascii_digit() || b == b'_'
}

/// `[-+]?` and the rest.
fn unsigned(s: &[u8]) -> &[u8] {
    match s {
        [b'-' | b'+', rest @ ..] => rest,
        _ => s,
    }
}

/// `:[0-5]?[0-9]` repeated at least once: the base-60 tail of a number.
fn sexagesimal_tail(s: &[u8]) -> bool {
    let mut parts = s.split(|&b| b == b':');
    if parts.next().is_none() {
        return false;
    }
    let mut any = false;
    for part in parts {
        let ok = match part {
            [a] => a.is_ascii_digit(),
            [a, b] => (b'0'..=b'5').contains(a) && b.is_ascii_digit(),
            _ => false,
        };
        if !ok {
            return false;
        }
        any = true;
    }
    any
}

fn is_int(s: &[u8]) -> bool {
    let r = unsigned(s);
    if r == b"0" {
        return true;
    }
    if let Some(rest) = r.strip_prefix(b"0b") {
        return digits(rest, |b| matches!(b, b'0' | b'1' | b'_'));
    }
    if let Some(rest) = r.strip_prefix(b"0x") {
        return digits(rest, |b| b.is_ascii_hexdigit() || b == b'_');
    }
    if let Some(rest) = r.strip_prefix(b"0") {
        return digits(rest, |b| matches!(b, b'0'..=b'7' | b'_'));
    }
    let Some((&first, _)) = r.split_first() else {
        return false;
    };
    if !(b'1'..=b'9').contains(&first) {
        return false;
    }
    match r.iter().position(|&b| b == b':') {
        None => r.iter().all(|&b| is_digit_or_underscore(b)),
        Some(colon) => {
            r[..colon].iter().all(|&b| is_digit_or_underscore(b)) && sexagesimal_tail(&r[colon..])
        }
    }
}

/// Whether `s` is a float.
///
/// The checks match, in order:
/// - `.inf`, after an optional sign, and `.nan`, each in three spellings;
/// - `[0-9][0-9_]*(:[0-5]?[0-9])+\.[0-9_]*`, the base-60 form;
/// - `([0-9][0-9_]*)?\.[0-9.]*([eE][-+][0-9]+)?`.
fn is_float(s: &[u8]) -> bool {
    let r = unsigned(s);
    if matches!(r, b".inf" | b".Inf" | b".INF") {
        return true;
    }
    if matches!(s, b".nan" | b".NaN" | b".NAN") {
        return true;
    }
    let Some(dot) = r.iter().position(|&b| b == b'.') else {
        return false;
    };
    let (head, tail) = (&r[..dot], &r[dot + 1..]);
    if let Some(colon) = head.iter().position(|&b| b == b':') {
        let lead = &head[..colon];
        let ok = lead.first().is_some_and(|b| is_digit(*b))
            && lead.iter().all(|&b| is_digit_or_underscore(b))
            && sexagesimal_tail(&head[colon..]);
        return ok && tail.iter().all(|&b| is_digit_or_underscore(b));
    }
    let head_ok =
        head.is_empty() || (is_digit(head[0]) && head.iter().all(|&b| is_digit_or_underscore(b)));
    if !head_ok {
        return false;
    }
    let mantissa = tail
        .iter()
        .position(|&b| !(b.is_ascii_digit() || b == b'.'))
        .unwrap_or(tail.len());
    match &tail[mantissa..] {
        [] => true,
        [b'e' | b'E', b'-' | b'+', exp @ ..] => digits(exp, is_digit),
        _ => false,
    }
}

/// `\d{4}-\d{2}-\d{2}`, or that date with `\d{1,2}` fields, a time and an
/// optional zone.
///
/// The separator before the time is `T`, `t` or blanks.
/// The zone is `[ \t]*Z` or `[-+]\d{1,2}(:\d{2})?`.
fn is_timestamp(s: &[u8]) -> bool {
    let n = |from: usize, min: usize, max: usize| -> Option<usize> {
        let len = s
            .get(from..)?
            .iter()
            .take_while(|b| b.is_ascii_digit())
            .count();
        (len >= min).then_some(len.min(max)).filter(|&l| l >= min)
    };
    let Some(year) = n(0, 4, 4) else { return false };
    if year != 4 || s.get(4) != Some(&b'-') {
        return false;
    }
    let Some(month) = n(5, 1, 2) else {
        return false;
    };
    let at = 5 + month;
    if s.get(at) != Some(&b'-') {
        return false;
    }
    let Some(day) = n(at + 1, 1, 2) else {
        return false;
    };
    let at = at + 1 + day;
    if month == 2 && day == 2 && at == s.len() {
        return true;
    }
    let mut at = at;
    match s.get(at) {
        Some(b'T' | b't') => at += 1,
        Some(b' ' | b'\t') => {
            while matches!(s.get(at), Some(b' ' | b'\t')) {
                at += 1;
            }
        }
        _ => return false,
    }
    let Some(hour) = n(at, 1, 2) else {
        return false;
    };
    at += hour;
    if s.get(at) != Some(&b':') {
        return false;
    }
    if n(at + 1, 2, 2) != Some(2) || s.get(at + 3) != Some(&b':') || n(at + 4, 2, 2) != Some(2) {
        return false;
    }
    at += 6;
    if s.get(at) == Some(&b'.') {
        at += 1;
        while s.get(at).is_some_and(u8::is_ascii_digit) {
            at += 1;
        }
    }
    if at == s.len() {
        return true;
    }
    let rest = &s[at..];
    let blanks = rest
        .iter()
        .take_while(|&&b| b == b' ' || b == b'\t')
        .count();
    if rest[blanks..] == *b"Z" {
        return true;
    }
    match rest {
        [b'-' | b'+', zone @ ..] => {
            let hours = zone.iter().take_while(|b| b.is_ascii_digit()).count();
            if !(1..=2).contains(&hours) {
                return false;
            }
            match &zone[hours..] {
                [] => true,
                [b':', minutes @ ..] => {
                    minutes.len() == 2 && minutes.iter().all(u8::is_ascii_digit)
                }
                _ => false,
            }
        }
        _ => false,
    }
}
