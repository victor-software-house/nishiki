//! What the compiler learns about a pattern before it runs: where a match can
//! start and how long text it needs, so that a search can skip ahead.

use super::ast::{Anchor, Case, Node};

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
        Node::Empty | Node::Anchor(_) | Node::Look { .. } | Node::Call(_) | Node::NamedCall(_) => (
            0,
            if matches!(node, Node::Call(_) | Node::NamedCall(_)) {
                None
            } else {
                Some(0)
            },
        ),
        Node::Char(..) | Node::Any(_) | Node::Class(..) => (1, Some(1)),
        Node::Backref(..) | Node::NamedBackref(..) => (0, None),
        Node::Seq(items) => items.iter().fold((0, Some(0)), |(lo, hi), item| {
            let (a, b) = width(item);
            (lo + a, hi.zip(b).map(|(x, y)| x + y))
        }),
        Node::Alt(alts) => {
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

/// Adds the bytes a match of `node` can start with to `set`; true when the
/// node can match without consuming anything, so what follows counts too.
///
/// A class may hold non-ASCII members, so every non-ASCII lead byte may start
/// one.
pub(super) fn first_bytes(node: &Node, set: &mut [bool; 256]) -> bool {
    let all = |set: &mut [bool; 256]| set.iter_mut().for_each(|b| *b = true);
    match node {
        Node::Empty | Node::Anchor(_) | Node::Look { .. } => true,
        Node::Char(c, case) => {
            let mut buffer = [0; 4];
            if let Some(&lead) = c.encode_utf8(&mut buffer).as_bytes().first() {
                set[usize::from(lead)] = true;
            }
            if *case == Case::Insensitive {
                for other in c.to_lowercase().chain(c.to_uppercase()) {
                    if let Some(&lead) = other.encode_utf8(&mut buffer).as_bytes().first() {
                        set[usize::from(lead)] = true;
                    }
                }
            }
            false
        }
        Node::Class(class, case) => {
            for b in 0..128u8 {
                if class.matches(char::from(b), *case) {
                    set[usize::from(b)] = true;
                }
            }
            for b in 0xc0..=0xffu8 {
                set[usize::from(b)] = true;
            }
            false
        }
        Node::Any(_) => {
            all(set);
            false
        }
        Node::Backref(..) | Node::NamedBackref(..) | Node::Call(_) | Node::NamedCall(_) => {
            all(set);
            true
        }
        Node::Seq(items) => {
            for item in items {
                if !first_bytes(item, set) {
                    return false;
                }
            }
            true
        }
        Node::Alt(alts) => {
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

/// The anchors every match of `node` begins at, when it has some.
pub(super) fn anchors(node: &Node) -> Option<Starts> {
    match node {
        Node::Anchor(Anchor::Caret) => Some(Starts {
            caret: true,
            ..Starts::default()
        }),
        Node::Anchor(Anchor::StartText) => Some(Starts {
            text: true,
            ..Starts::default()
        }),
        Node::Anchor(Anchor::Search) => Some(Starts {
            search: true,
            ..Starts::default()
        }),
        Node::Seq(items) => items.first().and_then(anchors),
        Node::Group { node, .. } | Node::Atomic(node) => anchors(node),
        Node::Alt(alts) => {
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
        | Node::Any(_)
        | Node::Class(..)
        | Node::Repeat { .. }
        | Node::Look { .. }
        | Node::Backref(..)
        | Node::NamedBackref(..)
        | Node::Call(_)
        | Node::NamedCall(_)
        | Node::Anchor(_) => None,
    }
}

/// The bytes every match of `node` starts with, when there are some: the run
/// of case-sensitive characters that opens it.
pub(super) fn literal_prefix(node: &Node) -> Vec<u8> {
    let mut out = Vec::new();
    collect_prefix(node, &mut out);
    out
}

/// Appends the leading literal of `node`; false once something that is not
/// a literal ends the run.
fn collect_prefix(node: &Node, out: &mut Vec<u8>) -> bool {
    match node {
        Node::Char(c, case)
            if *case == Case::Sensitive || c.to_lowercase().eq(c.to_uppercase()) =>
        {
            let mut buffer = [0; 4];
            out.extend_from_slice(c.encode_utf8(&mut buffer).as_bytes());
            true
        }
        Node::Seq(items) => items.iter().all(|item| collect_prefix(item, out)),
        Node::Group { node, .. } => collect_prefix(node, out),
        Node::Empty => true,
        Node::Char(..)
        | Node::Any(_)
        | Node::Class(..)
        | Node::Alt(_)
        | Node::Repeat { .. }
        | Node::Look { .. }
        | Node::Atomic(_)
        | Node::Backref(..)
        | Node::NamedBackref(..)
        | Node::Call(_)
        | Node::NamedCall(_)
        | Node::Anchor(_) => false,
    }
}
