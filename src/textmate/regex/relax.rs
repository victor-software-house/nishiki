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

use regex_syntax::hir::{self, ClassUnicode, ClassUnicodeRange, Dot, Hir, Look, Repetition};

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

/// The widened pattern as an expression for `regex-automata`, built without
/// a round trip through its source.
pub(super) fn hir(node: &Node) -> Hir {
    match node {
        Node::Empty
        | Node::Look { .. }
        | Node::Anchor(
            Anchor::StartText | Anchor::Search | Anchor::WordBoundary | Anchor::NotWordBoundary,
        ) => Hir::empty(),
        Node::Char(c) => Hir::literal(c.encode_utf8(&mut [0; 4]).as_bytes()),
        Node::Any(true) => Hir::dot(Dot::AnyChar),
        Node::Any(false) => Hir::dot(Dot::AnyCharExceptLF),
        Node::Class(class) => class_hir(class),
        Node::Seq(items) => Hir::concat(items.iter().map(hir).collect()),
        Node::Alt(alts) | Node::Folds(alts) => Hir::alternation(alts.iter().map(hir).collect()),
        Node::Repeat {
            node,
            min,
            max,
            kind,
        } => Hir::repetition(Repetition {
            min: *min,
            max: *max,
            greedy: match kind {
                Kind::Greedy | Kind::Possessive => true,
                Kind::Lazy => false,
            },
            sub: Box::new(hir(node)),
        }),
        Node::Group { node, .. } | Node::Atomic(node) => hir(node),
        Node::Backref(..) | Node::NamedBackref(..) | Node::Call(_) | Node::NamedCall(_) => {
            Hir::repetition(Repetition {
                min: 0,
                max: None,
                greedy: true,
                sub: Box::new(Hir::dot(Dot::AnyChar)),
            })
        }
        Node::Anchor(Anchor::Caret) => Hir::look(Look::StartLF),
        Node::Anchor(Anchor::Dollar | Anchor::EndTextNewline) => Hir::look(Look::EndLF),
        Node::Anchor(Anchor::EndText) => Hir::look(Look::End),
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
/// A class with no member matches nothing.
fn class_hir(class: &Class) -> Hir {
    let members = class.ascii();
    let member = |byte: u8| members >> byte & 1 == 1;
    let mut ranges = Vec::new();
    let mut byte = 0;
    while byte < 128 {
        if !member(byte) {
            byte += 1;
            continue;
        }
        let lo = byte;
        while byte + 1 < 128 && member(byte + 1) {
            byte += 1;
        }
        ranges.push(ClassUnicodeRange::new(char::from(lo), char::from(byte)));
        byte += 1;
    }
    if let Some(items) = class.ranges() {
        for item in items {
            if let Item::Range(lo, hi) = item
                && !hi.is_ascii()
            {
                ranges.push(ClassUnicodeRange::new((*lo).max('\u{80}'), *hi));
            }
        }
    } else if reaches_beyond_ascii(class) {
        ranges.push(ClassUnicodeRange::new('\u{80}', char::MAX));
    }
    Hir::class(hir::Class::Unicode(ClassUnicode::new(ranges)))
}
