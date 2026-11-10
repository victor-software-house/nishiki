//! A pattern as a plain regular expression.
//! The expression matches everything the pattern matches, and perhaps more.
//! An automaton uses it to find where a match can start.
//!
//! What a plain expression cannot express is dropped or widened:
//!
//! - lookaround, `\A`, `\G` and `\b` become nothing;
//! - a back-reference or a call matches any text;
//! - an atomic group or a possessive repeat becomes an ordinary one;
//! - a class keeps its ASCII members and takes every other character.
//!
//! A position the automaton reports is then a position a match may start at.
//! The exact machine decides.

use regex_syntax::hir::{self, ClassUnicode, ClassUnicodeRange, Dot, Hir, Look, Repetition};

use super::ast::{Anchor, Class, Greediness, Member, Node};

/// The pattern as an expression for `regex-automata`, in one walk of its tree.
///
/// A sequence is cut at its first positive lookahead.
/// The lookahead's pattern replaces the lookahead and everything after it.
///
/// A match that starts at `s` has the text before the lookahead.
/// The lookahead's pattern then matches at the end of that text.
/// The two in a row therefore match wherever a match can start.
/// The cut applies on the path from the root through groups, atomic groups
/// and lookaheads, as each one is walked.
pub(super) fn relaxed(node: &Node) -> Hir {
    match node {
        Node::Seq(items) => {
            let mut parts = Vec::with_capacity(items.len());
            for item in items {
                if let Node::Look {
                    ahead: true,
                    negative: false,
                    node: body,
                } = item
                {
                    parts.push(relaxed(body));
                    break;
                }
                parts.push(hir(item));
            }
            Hir::concat(parts)
        }
        Node::Group { node: inner, .. } | Node::Atomic(inner) => relaxed(inner),
        Node::Look {
            ahead: true,
            negative: false,
            node: body,
        } => relaxed(body),
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
        | Node::Anchor(_) => hir(node),
    }
}

/// The node as an expression for `regex-automata`, with no cut.
fn hir(node: &Node) -> Hir {
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
            greediness,
        } => Hir::repetition(Repetition {
            min: *min,
            max: *max,
            greedy: match greediness {
                Greediness::Greedy | Greediness::Possessive => true,
                Greediness::Lazy => false,
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
            Member::Range(_, hi) => !hi.is_ascii(),
            Member::Property(..) => true,
            Member::Nested(nested) => reaches_beyond_ascii(nested),
        })
}

/// A class with its exact ASCII members, and its members beyond ASCII.
///
/// Beyond ASCII it takes its ranges when it is a union of ranges, or every
/// other character when it can have any.
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
            if let Member::Range(lo, hi) = item
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
