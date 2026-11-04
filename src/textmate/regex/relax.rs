//! A pattern as a plain regular expression that matches everything the
//! pattern matches, and perhaps more, for an automaton that finds where a
//! match can start.
//!
//! What a plain expression cannot say is dropped or widened: lookaround,
//! `\A`, `\G` and `\b` become nothing, a back-reference or call matches any
//! text, an atomic group or possessive repeat is an ordinary one, and a class
//! keeps its ASCII members and takes every other character. A position the
//! automaton reports is then a position a match may start at, and the exact
//! machine decides.

use std::fmt::Write as _;

use super::ast::{Anchor, Class, Item, Kind, Node};

/// The pattern cut at its first positive lookahead, which is replaced by what
/// it looks for: a match that starts at `s` has the text before the lookahead
/// and then the lookahead's pattern at the end of that text, so the two in a
/// row say where a match can start, and what follows the lookahead, which
/// overlaps it, is left out.
pub(super) fn widen(node: &Node) -> Node {
    match node {
        Node::Seq(items) => {
            for (index, item) in items.iter().enumerate() {
                if let Node::Look {
                    ahead: true,
                    negative: false,
                    node: body,
                } = item
                {
                    let mut kept = items[..index].to_vec();
                    kept.push(widen(body));
                    return Node::Seq(kept);
                }
            }
            node.clone()
        }
        Node::Group { index, node: inner } => Node::Group {
            index: *index,
            node: Box::new(widen(inner)),
        },
        Node::Atomic(inner) => Node::Atomic(Box::new(widen(inner))),
        Node::Look {
            ahead: true,
            negative: false,
            node: body,
        } => widen(body),
        Node::Empty
        | Node::Char(..)
        | Node::Any(_)
        | Node::Class(..)
        | Node::Alt(_)
        | Node::Folds(_)
        | Node::Repeat { .. }
        | Node::Look { .. }
        | Node::Backref(..)
        | Node::NamedBackref(..)
        | Node::Call(_)
        | Node::NamedCall(_)
        | Node::Anchor(_) => node.clone(),
    }
}

/// The widened pattern as source for `regex-automata`.
pub(super) fn source(node: &Node) -> String {
    let mut out = String::new();
    print(node, &mut out);
    out
}

fn print(node: &Node, out: &mut String) {
    match node {
        Node::Empty
        | Node::Look { .. }
        | Node::Anchor(
            Anchor::StartText | Anchor::Search | Anchor::WordBoundary | Anchor::NotWordBoundary,
        ) => {}
        Node::Char(c) => {
            let _ = write!(out, "\\x{{{:x}}}", u32::from(*c));
        }
        Node::Any(true) => out.push_str("(?s:.)"),
        Node::Any(false) => out.push_str("[^\\x{a}]"),
        Node::Class(class) => class_source(class, out),
        Node::Seq(items) => items.iter().for_each(|item| print(item, out)),
        Node::Alt(alts) | Node::Folds(alts) => {
            out.push_str("(?:");
            for (index, alt) in alts.iter().enumerate() {
                if index > 0 {
                    out.push('|');
                }
                print(alt, out);
            }
            out.push(')');
        }
        Node::Repeat {
            node,
            min,
            max,
            kind,
        } => {
            out.push_str("(?:");
            print(node, out);
            out.push(')');
            let _ = match max {
                Some(max) => write!(out, "{{{min},{max}}}"),
                None => write!(out, "{{{min},}}"),
            };
            if *kind == Kind::Lazy {
                out.push('?');
            }
        }
        Node::Group { node, .. } | Node::Atomic(node) => {
            out.push_str("(?:");
            print(node, out);
            out.push(')');
        }
        Node::Backref(..) | Node::NamedBackref(..) | Node::Call(_) | Node::NamedCall(_) => {
            out.push_str("(?s:.*)");
        }
        Node::Anchor(Anchor::Caret) => out.push_str("(?m:^)"),
        Node::Anchor(Anchor::Dollar | Anchor::EndTextNewline) => out.push_str("(?m:$)"),
        Node::Anchor(Anchor::EndText) => out.push_str("\\z"),
    }
}

/// Whether a class can hold a character beyond ASCII.
fn reaches_beyond_ascii(class: &Class) -> bool {
    class.negated
        || class.parts.iter().flatten().any(|item| match item {
            Item::Range(_, hi) => !hi.is_ascii(),
            Item::Prop(..) => true,
            Item::Nested(nested) => reaches_beyond_ascii(nested),
        })
}

/// A class with its exact ASCII members, and beyond ASCII its ranges when it
/// is a union of ranges, or every other character when it can have any.
///
/// A class with no member becomes one that matches nothing.
fn class_source(class: &Class, out: &mut String) {
    out.push('[');
    let members: Vec<bool> = (0..128_u8)
        .map(|byte| class.matches(char::from(byte)))
        .collect();
    let mut empty = true;
    let mut byte = 0;
    while byte < members.len() {
        if !members[byte] {
            byte += 1;
            continue;
        }
        let lo = byte;
        while byte + 1 < members.len() && members[byte + 1] {
            byte += 1;
        }
        let _ = write!(out, "\\x{{{lo:x}}}-\\x{{{byte:x}}}");
        empty = false;
        byte += 1;
    }
    if let Some(ranges) = class.ranges() {
        for item in ranges {
            if let Item::Range(lo, hi) = item
                && !hi.is_ascii()
            {
                let lo = u32::from(*lo).max(0x80);
                let _ = write!(out, "\\x{{{lo:x}}}-\\x{{{:x}}}", u32::from(*hi));
                empty = false;
            }
        }
    } else if reaches_beyond_ascii(class) {
        out.push_str("\\x{80}-\\x{10ffff}");
        empty = false;
    }
    if empty {
        out.push_str("^\\x{0}-\\x{10ffff}");
    }
    out.push(']');
}
