//! A match whose scope name holds `$1`, as one pattern for each alternative of
//! its first group.
//!
//! The grammar names such a match after the text its first group matched,
//! such as `storage.modifier.$1.java` for `\b(extends|super)\b`.
//! Each alternative becomes its own pattern, with a fixed scope name.
//! Together the patterns match the same text as the one match.
//! The patterns take the match's place in its list, in the group's order.
//!
//! The split ports `expandBackrefs` and the hand-written splits of the
//! `oracle/<language>-grammar.ts` scripts.

use super::error::GenerateError;
use super::tables::Split;
use crate::textmate::compile::Text;

/// The expression of each pattern a match of `regex`, named with `names`,
/// becomes, or `None` when no name holds `$1`.
///
/// The first group is the first `(` not followed by `?`, outside a character
/// class.
/// It ends at the next `)`.
///
/// # Errors
///
/// [`GenerateError::Split`], naming `id`, when the expression has no such
/// group, or when the group holds a `(`, a `\` or a `[`.
/// The split reads the group as words between `|`, which such a group does
/// not hold.
pub(super) fn alternatives(
    regex: &str,
    names: &[Text],
    split: Split,
    id: &str,
) -> Result<Option<Vec<String>>, GenerateError> {
    if !names.iter().any(|name| name.contains("$1")) {
        return Ok(None);
    }
    let refused = || GenerateError::Split { id: id.to_owned() };
    let (start, end) = first_group(regex).ok_or_else(refused)?;
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

/// The byte offsets of the first group's `(` and of the next `)`.
fn first_group(regex: &str) -> Option<(usize, usize)> {
    let bytes = regex.as_bytes();
    let mut in_class = false;
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'\\' => index += 1,
            b']' if in_class => in_class = false,
            _ if in_class => {}
            b'[' => in_class = true,
            b'(' if bytes.get(index + 1) != Some(&b'?') => {
                let end = index + regex[index..].find(')')?;
                return Some((index, end));
            }
            _ => {}
        }
        index += 1;
    }
    None
}
