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

/// One pattern of a split: its expression and the word it is named by.
pub(super) struct Variant {
    /// The pattern's expression.
    pub(super) regex: String,
    /// The ASCII letters that start the pattern's alternative, such as
    /// `class` for `class(?!\s+func)`.
    /// It is empty when the alternative starts with no letter.
    pub(super) word: String,
}

/// The expression of each pattern that a match or begin `regex`, named with
/// `names`, becomes, or `None` when no name holds a group's text.
/// A begin passes its `end`, or its `while`.
///
/// A name holds group `n`'s text as `$n` or as `${n:/downcase}`.
/// Group `n` is the `n`th group that captures, as [`group`] counts them.
/// Each of its alternatives at its own level gives one pattern, so
/// `\b(class(?!\s+(?:func|var)\b)|struct)\b` gives two.
///
/// # Errors
///
/// [`GenerateError::Split`], naming `id` and `regex`, when:
///
/// - the names hold several groups, or the expression has no such group;
/// - the group is named, since its name would repeat in each pattern;
/// - `split` is [`Split::DropsGroup`] and a later group captures, since
///   dropping the group would change that group's number;
/// - `split` is [`Split::DropsGroup`] and `end` repeats the group, since the
///   begins of the split hold no such group.
pub(super) fn alternatives<'n>(
    regex: &str,
    end: Option<&str>,
    names: impl IntoIterator<Item = &'n str>,
    split: Split,
    id: &str,
) -> Result<Option<Vec<Variant>>, GenerateError> {
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
    let (start, close) = group(regex, number).ok_or_else(refused)?;
    let words = &regex[start + 1..close];
    if words.starts_with('?') {
        return Err(refused());
    }
    if split == Split::DropsGroup
        && (group(regex, number + 1).is_some() || end.is_some_and(|end| repeats(end, number)))
    {
        return Err(refused());
    }
    let (before, after) = (&regex[..start], &regex[close + 1..]);
    Ok(Some(
        top_level(words)
            .into_iter()
            .map(|alternative| Variant {
                regex: match split {
                    Split::KeepsGroup => format!("{before}({alternative}){after}"),
                    Split::DropsGroup => format!("{before}{alternative}{after}"),
                },
                word: word(alternative),
            })
            .collect(),
    ))
}

/// The expression of each pattern that a match `regex` becomes when only
/// its capture names, `names`, hold a group's text, or `None` when none
/// does.
///
/// The innermost group `(?:…)?` around the named group splits, whatever
/// groups lie between them:
///
/// - each of its alternatives gives one pattern, inside `(?:…)`;
/// - a last pattern leaves the group out.
///
/// So `1(?:(%)|(px|em)\b)?` gives `1(?:(%))`, `1(?:(px|em)\b)` and `1`.
/// The named group stays whole, and the language's code names its text.
///
/// # Errors
///
/// [`GenerateError::Split`], naming `id` and `regex`, when:
///
/// - the names hold several groups;
/// - no optional group holds the named one;
/// - the innermost one is lazy or possessive, `(?:…)??` or `(?:…)?+`, since the
///   split reads only a plain `?`;
/// - a group between them has several alternatives, or a `?`, `*` or `{` after
///   it, since the named group could then match nothing.
pub(super) fn optional_alternatives<'n>(
    regex: &str,
    names: impl IntoIterator<Item = &'n str>,
    id: &str,
) -> Result<Option<Vec<Variant>>, GenerateError> {
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
        .filter(|&(open, close)| {
            open < start
                && start < close
                && regex[open..].starts_with("(?:")
                && regex[close + 1..].starts_with('?')
        })
        .max_by_key(|&(open, _)| open)
        .ok_or_else(refused)?;
    if regex[close + 2..].starts_with(['?', '+']) {
        return Err(refused());
    }
    let between = spans(regex)
        .into_iter()
        .filter(|&(inner, end)| open < inner && inner < start && start < end);
    for (inner, end) in between {
        if top_level(&regex[inner + 1..end]).len() > 1
            || regex[end + 1..].starts_with(['?', '*', '{'])
        {
            return Err(refused());
        }
    }
    let (before, after) = (&regex[..open], &regex[close + 2..]);
    let mut out = top_level(&regex[open + 3..close])
        .into_iter()
        .map(|alternative| Variant {
            regex: format!("{before}(?:{alternative}){after}"),
            word: word(alternative),
        })
        .collect::<Vec<_>>();
    out.push(Variant {
        regex: format!("{before}{after}"),
        word: String::new(),
    });
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
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'\\' => index += 1,
            b'[' => index = class_end(bytes, index),
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
    let mut from = 0;
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'\\' => index += 1,
            b'[' => index = class_end(bytes, index),
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

/// The ASCII letters that start `alternative`.
fn word(alternative: &str) -> String {
    alternative
        .chars()
        .take_while(char::is_ascii_alphabetic)
        .collect()
}

/// Whether the end `end` repeats group `number` of its begin.
///
/// It reads `end` as vscode-textmate's `resolveBackReferences` does: every
/// `\\` before digits repeats the group they number.
fn repeats(end: &str, number: usize) -> bool {
    end.match_indices('\\').any(|(at, _)| {
        let rest = &end[at + 1..];
        let digits = rest.bytes().take_while(u8::is_ascii_digit).count();
        rest[..digits].parse() == Ok(number)
    })
}

/// The number of each group whose text the scope name `name` holds.
///
/// `$2` and `${2:/downcase}` each give 2.
/// A `$` before no digit gives nothing.
/// So does `$0`, the whole match, which no group holds.
fn references(name: &str) -> impl Iterator<Item = usize> + '_ {
    name.match_indices('$').filter_map(|(at, _)| {
        let rest = &name[at + 1..];
        let rest = rest.strip_prefix('{').unwrap_or(rest);
        let digits = rest.bytes().take_while(u8::is_ascii_digit).count();
        rest[..digits].parse().ok().filter(|&number| number > 0)
    })
}

/// The byte offsets of group `number`'s `(` and of its matching `)`.
///
/// Groups are numbered as Shiki's Oniguruma numbers them.
/// A `(` not followed by `?` captures, and so does a named group,
/// `(?<name>…)` or `(?'name'…)`.
/// A lookbehind, `(?<=…)` or `(?<!…)`, does not.
///
/// A byte after a `\` is skipped, so an escaped `(`, such as the one in
/// `\(\b(foo|bar)`, opens no group.
/// So is a character class, so a `(` in `[(]` opens none either.
fn group(regex: &str, number: usize) -> Option<(usize, usize)> {
    let bytes = regex.as_bytes();
    let mut count = 0;
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'\\' => index += 1,
            b'[' => index = class_end(bytes, index),
            b'(' if captures(&bytes[index + 1..]) => {
                count += 1;
                if count == number {
                    return spans(regex).into_iter().find(|&(open, _)| open == index);
                }
            }
            _ => {}
        }
        index += 1;
    }
    None
}

/// The index of the `]` that closes the character class whose `[` is at
/// `open`, or the length of `bytes` when none does.
///
/// A `]` first in the class, after any `^`, stands for itself.
/// A class inside it, such as `[:alpha:]` in `[_[:alpha:]]`, closes before
/// it.
fn class_end(bytes: &[u8], open: usize) -> usize {
    let mut index = open + 1;
    if bytes.get(index) == Some(&b'^') {
        index += 1;
    }
    if bytes.get(index) == Some(&b']') {
        index += 1;
    }
    let mut depth = 0_usize;
    while index < bytes.len() {
        match bytes[index] {
            b'\\' => index += 1,
            b'[' => depth += 1,
            b']' if depth == 0 => return index,
            b']' => depth -= 1,
            _ => {}
        }
        index += 1;
    }
    bytes.len()
}

/// Whether a `(` before `after` opens a group that captures.
const fn captures(after: &[u8]) -> bool {
    match after {
        [b'?', b'<', b'=' | b'!', ..] => false,
        [b'?', b'<' | b'\'', ..] => true,
        [b'?', ..] => false,
        _ => true,
    }
}
