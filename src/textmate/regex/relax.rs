//! A pattern as a plain regular expression.
//! The expression matches everything the pattern matches, and perhaps more.
//! An automaton uses it to find where a match can start.
//!
//! What no plain expression can express is dropped or widened:
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
/// So the two in a row match wherever a match can start.
/// The cut applies on the path from the root, through groups, atomic groups
/// and lookaheads, as each one is walked.
pub(super) fn relaxed(node: &Node) -> Hir {
    match node {
        Node::Sequence(items) => {
            let mut parts = Vec::with_capacity(items.len());
            for item in items {
                if let Node::Lookaround {
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
        Node::Lookaround {
            ahead: true,
            negative: false,
            node: body,
        } => relaxed(body),
        Node::Empty
        | Node::Char(..)
        | Node::Any { .. }
        | Node::Class(..)
        | Node::Alternation(_)
        | Node::Folds(_)
        | Node::Repeat { .. }
        | Node::Lookaround { .. }
        | Node::BackReference(..)
        | Node::NamedBackReference(..)
        | Node::Subroutine(_)
        | Node::NamedSubroutine(_)
        | Node::Anchor(_) => hir(node),
    }
}

/// The node as an expression for `regex-automata`, with no cut.
fn hir(node: &Node) -> Hir {
    match node {
        Node::Empty
        | Node::Lookaround { .. }
        | Node::Anchor(
            Anchor::TextStart
            | Anchor::LastMatchEnd
            | Anchor::WordBoundary
            | Anchor::NotWordBoundary,
        ) => Hir::empty(),
        Node::Char(c) => Hir::literal(c.encode_utf8(&mut [0; 4]).as_bytes()),
        Node::Any { newline: true } => Hir::dot(Dot::AnyChar),
        Node::Any { newline: false } => Hir::dot(Dot::AnyCharExceptLF),
        Node::Class(class) => class_hir(class),
        Node::Sequence(items) => Hir::concat(items.iter().map(hir).collect()),
        Node::Alternation(alts) | Node::Folds(alts) => {
            Hir::alternation(alts.iter().map(hir).collect())
        }
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
        Node::BackReference(..)
        | Node::NamedBackReference(..)
        | Node::Subroutine(_)
        | Node::NamedSubroutine(_) => Hir::repetition(Repetition {
            min: 0,
            max: None,
            greedy: true,
            sub: Box::new(Hir::dot(Dot::AnyChar)),
        }),
        Node::Anchor(Anchor::LineStart) => Hir::look(Look::StartLF),
        Node::Anchor(Anchor::LineEnd | Anchor::TextEndOrFinalNewline) => Hir::look(Look::EndLF),
        Node::Anchor(Anchor::TextEnd) => Hir::look(Look::End),
    }
}

/// Whether a class can hold a character beyond ASCII.
fn reaches_beyond_ascii(class: &Class) -> bool {
    class.negated
        || class.unions.iter().flatten().any(|item| match item {
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
