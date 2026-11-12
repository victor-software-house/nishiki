//! A pattern whose scope name holds a group's text, as one pattern for each
//! alternative of that group.
//!
//! The grammar names such a pattern after the text a group matched.
//! `storage.modifier.$1.java` for `\b(extends|super)\b` names group 1.
//! `meta.at-rule.${3:/downcase}.css` names group 3, in lower case.
//! Each alternative becomes its own pattern, with a fixed scope name.
//! Together the patterns match the same text as the one pattern.
//! The patterns take its place, in the group's order.
//!
//! The split ports `expandBackrefs`, `expandDynamic` and the hand-written
//! splits of the `oracle/<language>-grammar.ts` scripts.

use super::error::GenerateError;
use super::tables::Split;

/// The expression of each pattern that a match or begin `regex`, named with
/// `names`, becomes, or `None` when no name holds a group's text.
///
/// A name holds group `n`'s text as `$n` or as `${n:/downcase}`.
/// Group `n` is the `n`th `(` not followed by `?`, outside a character
/// class.
/// It ends at the next `)`.
///
/// # Errors
///
/// [`GenerateError::Split`], naming `id` and `regex`, when the names hold
/// several groups, when the expression has no such group, or when the group
/// holds a `(`, a `\` or a `[`.
/// The split reads the group as words between `|`, which such a group does
/// not hold.
pub(super) fn alternatives<'n>(
    regex: &str,
    names: impl IntoIterator<Item = &'n str>,
    split: Split,
    id: &str,
) -> Result<Option<Vec<String>>, GenerateError> {
    let refused = || GenerateError::Split {
        id: id.to_owned(),
        regex: regex.to_owned(),
    };
    let mut numbers = names.into_iter().flat_map(references).collect::<Vec<_>>();
    numbers.sort_unstable();
    numbers.dedup();
    let number = match *numbers {
        [] => return Ok(None),
        [number] => number,
        _ => return Err(refused()),
    };
    let (start, end) = group(regex, number).ok_or_else(refused)?;
    let words = &regex[start + 1..end];
    if words.contains(['(', '\\', '[']) {
        return Err(refused());
    }
    let (before, after) = (&regex[..start], &regex[end + 1..]);
    Ok(Some(
        words
            .split('|')
            .map(|word| match split {
                Split::KeepsGroup => format!("{before}({word}){after}"),
                Split::DropsGroup => format!("{before}{word}{after}"),
            })
            .collect(),
    ))
}

/// The expression of each pattern that a match `regex` becomes when only
/// its capture names, `names`, hold a group's text, or `None` when none
/// does.
///
/// The innermost optional group `(?:…)?` around the named group splits.
/// Each of its alternatives gives one pattern, inside `(?:…)`.
/// A last pattern leaves the group out.
/// So `1(?:(%)|(px|em)\b)?` gives `1(?:(%))`, `1(?:(px|em)\b)` and `1`.
/// The named group stays whole, and the language's code names its text.
///
/// # Errors
///
/// [`GenerateError::Split`], naming `id` and `regex`, when the names hold
/// several groups, or when no optional group holds the named one.
pub(super) fn optional_alternatives<'n>(
    regex: &str,
    names: impl IntoIterator<Item = &'n str>,
    id: &str,
) -> Result<Option<Vec<String>>, GenerateError> {
    let refused = || GenerateError::Split {
        id: id.to_owned(),
        regex: regex.to_owned(),
    };
    let mut numbers = names.into_iter().flat_map(references).collect::<Vec<_>>();
    numbers.sort_unstable();
    numbers.dedup();
    let number = match *numbers {
        [] => return Ok(None),
        [number] => number,
        _ => return Err(refused()),
    };
    let (start, _) = group(regex, number).ok_or_else(refused)?;
    let (open, close) = spans(regex)
        .into_iter()
        .filter(|&(open, close)| open < start && start < close)
        .max_by_key(|&(open, _)| open)
        .filter(|&(open, close)| {
            regex[open..].starts_with("(?:") && regex[close + 1..].starts_with('?')
        })
        .ok_or_else(refused)?;
    let (before, after) = (&regex[..open], &regex[close + 2..]);
    let mut out = top_level(&regex[open + 3..close])
        .into_iter()
        .map(|alternative| format!("{before}(?:{alternative}){after}"))
        .collect::<Vec<_>>();
    out.push(format!("{before}{after}"));
    Ok(Some(out))
}

/// The byte offsets of each group's `(` and its matching `)`, in the order
/// the groups close.
///
/// Escaped bytes and character classes open and close no group.
fn spans(regex: &str) -> Vec<(usize, usize)> {
    let bytes = regex.as_bytes();
    let mut open = Vec::new();
    let mut out = Vec::new();
    let mut in_class = false;
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'\\' => index += 1,
            b']' if in_class => in_class = false,
            _ if in_class => {}
            b'[' => in_class = true,
            b'(' => open.push(index),
            b')' => {
                if let Some(start) = open.pop() {
                    out.push((start, index));
                }
            }
            _ => {}
        }
        index += 1;
    }
    out
}

/// The alternatives of `body` at its own level, split at each `|` outside
/// a group or a character class.
fn top_level(body: &str) -> Vec<&str> {
    let bytes = body.as_bytes();
    let mut out = Vec::new();
    let mut depth = 0_usize;
    let mut in_class = false;
    let mut from = 0;
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'\\' => index += 1,
            b']' if in_class => in_class = false,
            _ if in_class => {}
            b'[' => in_class = true,
            b'(' => depth += 1,
            b')' => depth = depth.saturating_sub(1),
            b'|' if depth == 0 => {
                out.push(&body[from..index]);
                from = index + 1;
            }
            _ => {}
        }
        index += 1;
    }
    out.push(&body[from..]);
    out
}

/// The number of each group whose text the scope name `name` holds.
///
/// `$2` and `${2:/downcase}` each give 2.
/// A `$` before no digit gives nothing.
fn references(name: &str) -> impl Iterator<Item = usize> + '_ {
    name.match_indices('$').filter_map(|(at, _)| {
        let rest = &name[at + 1..];
        let rest = rest.strip_prefix('{').unwrap_or(rest);
        let digits = rest.bytes().take_while(u8::is_ascii_digit).count();
        rest[..digits].parse().ok()
    })
}

/// The byte offsets of group `number`'s `(` and of the next `)`.
///
/// A byte after a `\` is skipped, so an escaped `(`, such as the one in
/// `\(\b(foo|bar)`, opens no group.
/// So is a character class, so a `(` in `[(]` opens none either.
fn group(regex: &str, number: usize) -> Option<(usize, usize)> {
    let bytes = regex.as_bytes();
    let mut in_class = false;
    let mut count = 0;
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'\\' => index += 1,
            b']' if in_class => in_class = false,
            _ if in_class => {}
            b'[' => in_class = true,
            b'(' if bytes.get(index + 1) != Some(&b'?') => {
                count += 1;
                if count == number {
                    let end = index + regex[index..].find(')')?;
                    return Some((index, end));
                }
            }
            _ => {}
        }
        index += 1;
    }
    None
}
