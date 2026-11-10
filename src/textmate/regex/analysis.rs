//! What the compiler learns about a pattern before it runs.
//! It learns where a match can start and how much text a match needs.
//! A search uses both to skip ahead.

use super::ast::{Anchor, Class, Member, Node};

/// Where a match can start, beyond the character at the start.
#[derive(Clone, Copy, Debug, Default)]
pub(super) struct Starts {
    /// At the start of a line.
    pub(super) caret: bool,
    /// At the start of the document.
    pub(super) text: bool,
    /// Where the last match ended.
    pub(super) search: bool,
}

/// The shortest and, when bounded, the longest number of characters a match
/// of `node` takes.
pub(super) fn width(node: &Node) -> (usize, Option<usize>) {
    match node {
        Node::Empty
        | Node::Anchor(_)
        | Node::Lookaround { .. }
        | Node::Subroutine(_)
        | Node::NamedSubroutine(_) => (
            0,
            if matches!(node, Node::Subroutine(_) | Node::NamedSubroutine(_)) {
                None
            } else {
                Some(0)
            },
        ),
        Node::Char(..) | Node::Any { .. } | Node::Class(..) => (1, Some(1)),
        Node::BackReference(..) | Node::NamedBackReference(..) => (0, None),
        Node::Sequence(items) => items.iter().fold((0, Some(0)), |(lo, hi), item| {
            let (a, b) = width(item);
            (lo + a, hi.zip(b).map(|(x, y)| x + y))
        }),
        Node::Alternation(alts) | Node::Folds(alts) => {
            let mut lo = usize::MAX;
            let mut hi = Some(0);
            for alt in alts {
                let (a, b) = width(alt);
                lo = lo.min(a);
                hi = hi.zip(b).map(|(x, y)| x.max(y));
            }
            (if lo == usize::MAX { 0 } else { lo }, hi)
        }
        Node::Repeat { node, min, max, .. } => {
            let (a, b) = width(node);
            let min = usize::try_from(*min).unwrap_or(usize::MAX);
            let high = match (max, b) {
                (Some(max), Some(b)) => {
                    Some(b.saturating_mul(usize::try_from(*max).unwrap_or(usize::MAX)))
                }
                (_, Some(0)) => Some(0),
                _ => None,
            };
            (a.saturating_mul(min), high)
        }
        Node::Group { node, .. } | Node::Atomic(node) => width(node),
    }
}

/// Adds the bytes a match of `node` can start with to `set`.
/// Returns true when the node can match without consuming anything.
/// Then the bytes of what follows count too.
///
/// A class that is a union of ranges adds the lead bytes of its ranges.
/// Any other class may hold any non-ASCII member.
/// For such a class, every non-ASCII lead byte may start a match.
///
/// A lookahead that needs a character limits the start to the characters it
/// can begin with.
pub(super) fn first_bytes(node: &Node, set: &mut [bool; 256]) -> bool {
    let all = |set: &mut [bool; 256]| set.iter_mut().for_each(|b| *b = true);
    match node {
        Node::Lookaround {
            ahead: true,
            negative: false,
            node,
        } => {
            let mut inner = [false; 256];
            if first_bytes(node, &mut inner) {
                return true;
            }
            for (member, wanted) in set.iter_mut().zip(inner) {
                *member |= wanted;
            }
            false
        }
        Node::Empty | Node::Anchor(_) | Node::Lookaround { .. } => true,
        Node::Char(c) => {
            set[usize::from(lead_byte(*c))] = true;
            false
        }
        Node::Class(class) => {
            for b in 0..128u8 {
                if class.matches(char::from(b)) {
                    set[usize::from(b)] = true;
                }
            }
            for (first, last) in wide_lead_bytes(class) {
                for b in first..=last {
                    set[usize::from(b)] = true;
                }
            }
            false
        }
        Node::Any { .. } => {
            all(set);
            false
        }
        Node::BackReference(..)
        | Node::NamedBackReference(..)
        | Node::Subroutine(_)
        | Node::NamedSubroutine(_) => {
            all(set);
            true
        }
        Node::Sequence(items) => {
            for item in items {
                if !first_bytes(item, set) {
                    return false;
                }
            }
            true
        }
        Node::Alternation(alts) | Node::Folds(alts) => {
            let mut nullable = false;
            for alt in alts {
                nullable |= first_bytes(alt, set);
            }
            nullable
        }
        Node::Repeat { node, min, .. } => first_bytes(node, set) || *min == 0,
        Node::Group { node, .. } | Node::Atomic(node) => first_bytes(node, set),
    }
}

/// Adds the lead bytes of the second character of every match of `node` to
/// `set`.
/// Returns whether it could.
/// It can only when each match starts with one character or class, followed
/// by something that consumes.
///
/// The second character separates a fold string from the letter alone.
/// An example is `i` followed by a combining mark.
pub(super) fn second_bytes(node: &Node, set: &mut [bool; 256]) -> bool {
    match node {
        Node::Sequence(items) => match items.split_first() {
            Some((Node::Char(_) | Node::Class(_), rest)) => {
                rest.iter().any(|item| !first_bytes(item, set))
            }
            _ => false,
        },
        Node::Alternation(alts) | Node::Folds(alts) => {
            alts.iter().all(|alt| second_bytes(alt, set))
        }
        Node::Empty
        | Node::Char(..)
        | Node::Any { .. }
        | Node::Class(..)
        | Node::Repeat { .. }
        | Node::Group { .. }
        | Node::Lookaround { .. }
        | Node::Atomic(_)
        | Node::BackReference(..)
        | Node::NamedBackReference(..)
        | Node::Subroutine(_)
        | Node::NamedSubroutine(_)
        | Node::Anchor(_) => false,
    }
}

/// The anchors every match of `node` begins at, when it has some.
pub(super) fn anchors(node: &Node) -> Option<Starts> {
    match node {
        Node::Anchor(Anchor::LineStart) => Some(Starts {
            caret: true,
            ..Starts::default()
        }),
        Node::Anchor(Anchor::TextStart) => Some(Starts {
            text: true,
            ..Starts::default()
        }),
        Node::Anchor(Anchor::LastMatchEnd) => Some(Starts {
            search: true,
            ..Starts::default()
        }),
        Node::Sequence(items) => items.first().and_then(anchors),
        Node::Group { node, .. } | Node::Atomic(node) => anchors(node),
        Node::Alternation(alts) | Node::Folds(alts) => {
            let mut all = Starts::default();
            for alt in alts {
                let one = anchors(alt)?;
                all.caret |= one.caret;
                all.text |= one.text;
                all.search |= one.search;
            }
            Some(all)
        }
        Node::Empty
        | Node::Char(..)
        | Node::Any { .. }
        | Node::Class(..)
        | Node::Repeat { .. }
        | Node::Lookaround { .. }
        | Node::BackReference(..)
        | Node::NamedBackReference(..)
        | Node::Subroutine(_)
        | Node::NamedSubroutine(_)
        | Node::Anchor(_) => None,
    }
}

/// The bytes every match of `node` starts with, when there are some: the run
/// of characters that opens it.
pub(super) fn literal_prefix(node: &Node) -> Vec<u8> {
    let mut out = Vec::new();
    collect_prefix(node, &mut out);
    out
}

/// Appends the leading literal of `node`.
/// Returns false once something that is not a literal ends the run.
fn collect_prefix(node: &Node, out: &mut Vec<u8>) -> bool {
    match node {
        Node::Char(c) => {
            let mut buffer = [0; 4];
            out.extend_from_slice(c.encode_utf8(&mut buffer).as_bytes());
            true
        }
        Node::Sequence(items) => items.iter().all(|item| collect_prefix(item, out)),
        Node::Group { node, .. } => collect_prefix(node, out),
        Node::Empty => true,
        Node::Any { .. }
        | Node::Class(..)
        | Node::Alternation(_)
        | Node::Folds(_)
        | Node::Repeat { .. }
        | Node::Lookaround { .. }
        | Node::Atomic(_)
        | Node::BackReference(..)
        | Node::NamedBackReference(..)
        | Node::Subroutine(_)
        | Node::NamedSubroutine(_)
        | Node::Anchor(_) => false,
    }
}

/// Whether a match can depend on more than the text and where the search
/// starts: `\A` and `\G` read what the caller passes in about the document.
pub(super) fn uses_context(node: &Node) -> bool {
    match node {
        Node::Anchor(Anchor::TextStart | Anchor::LastMatchEnd) => true,
        Node::Anchor(
            Anchor::LineStart
            | Anchor::LineEnd
            | Anchor::TextEnd
            | Anchor::TextEndOrFinalNewline
            | Anchor::WordBoundary
            | Anchor::NotWordBoundary,
        )
        | Node::Empty
        | Node::Char(..)
        | Node::Any { .. }
        | Node::Class(..)
        | Node::BackReference(..)
        | Node::NamedBackReference(..)
        | Node::Subroutine(_)
        | Node::NamedSubroutine(_) => false,
        Node::Sequence(items) | Node::Alternation(items) | Node::Folds(items) => {
            items.iter().any(uses_context)
        }
        Node::Repeat { node, .. }
        | Node::Group { node, .. }
        | Node::Lookaround { node, .. }
        | Node::Atomic(node) => uses_context(node),
    }
}

/// A byte every match of `node` holds at or after its start.
/// It is the rarest ASCII character the pattern needs.
/// A search can give up on text without it.
pub(super) fn required_byte(node: &Node) -> Option<u8> {
    let mut found = Vec::new();
    collect_required(node, &mut found);
    found.into_iter().min_by_key(|&byte| rank(byte))
}

/// How common a byte is in source text, lower for rarer.
fn rank(byte: u8) -> u8 {
    match byte {
        b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' => 3,
        b' ' | b'.' | b',' | b'(' | b')' | b'=' | b'"' | b'\'' => 2,
        _ => 1,
    }
}

/// Adds the bytes `node` must find in the text ahead of its start.
fn collect_required(node: &Node, out: &mut Vec<u8>) {
    match node {
        Node::Char(c) if c.is_ascii() => {
            let mut buffer = [0; 4];
            out.extend(c.encode_utf8(&mut buffer).bytes());
        }
        Node::Sequence(items) => items.iter().for_each(|item| collect_required(item, out)),
        Node::Group { node, .. }
        | Node::Atomic(node)
        | Node::Lookaround {
            ahead: true,
            negative: false,
            node,
        } => collect_required(node, out),
        Node::Repeat { node, min, .. } if *min > 0 => collect_required(node, out),
        Node::Repeat { .. }
        | Node::Empty
        | Node::Char(..)
        | Node::Any { .. }
        | Node::Class(..)
        | Node::Alternation(_)
        | Node::Folds(_)
        | Node::Lookaround { .. }
        | Node::BackReference(..)
        | Node::NamedBackReference(..)
        | Node::Subroutine(_)
        | Node::NamedSubroutine(_)
        | Node::Anchor(_) => {}
    }
}

/// The lead bytes a non-ASCII member of `class` can start with, as inclusive
/// runs.
///
/// For a union of ranges, each range gives the run from its first lead byte
/// to its last.
/// A lead byte never falls as the code point rises.
/// The run therefore covers every lead byte of the range.
///
/// Any other class may hold any non-ASCII character.
fn wide_lead_bytes(class: &Class) -> Vec<(u8, u8)> {
    let Some(ranges) = class.ranges() else {
        return vec![(0xc0, 0xff)];
    };
    ranges
        .iter()
        .filter_map(|item| match item {
            Member::Range(lo, hi) if !hi.is_ascii() => {
                Some((lead_byte((*lo).max('\u{80}')), lead_byte(*hi)))
            }
            Member::Range(..) | Member::Property(..) | Member::Nested(_) => None,
        })
        .collect()
}

/// The first byte of `c` in UTF-8.
pub(super) fn lead_byte(c: char) -> u8 {
    let mut buffer = [0; 4];
    c.encode_utf8(&mut buffer);
    buffer[0]
}
