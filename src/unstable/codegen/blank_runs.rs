//! Whether a pattern treats a wide space as a space.
//!
//! A wide space is a character above ASCII that `\s` matches.
//!
//! The scan of a grammar on `BlankRuns::Skip` passes a run of spaces and tabs
//! in one step.
//! The scan can pass a run that also holds wide spaces when every pattern
//! treats a wide space as it treats a space.
//! Then each pattern reads the run as it reads a run of spaces and tabs.

use std::collections::HashMap;
use std::sync::LazyLock;

use crate::classes::is_space;
use crate::textmate::regex::ast::Node;
use crate::textmate::regex::parse::{Dialect, parse};

/// The space and the wide spaces.
/// A pattern must match all of them or none of them.
static ALIKE: LazyLock<Vec<char>> = LazyLock::new(|| {
    std::iter::once(' ')
        .chain(('\u{80}'..=char::MAX).filter(|&c| is_space(c)))
        .collect()
});

/// Whether every match of `regex` that starts inside a run of spaces, tabs
/// and wide spaces treats each wide space of the run as a space.
///
/// The walk follows a match from such a start while it may still be inside
/// the run.
/// Each character it can read there must match the space and every wide
/// space, or none of them.
///
/// - A lookbehind can read back into the run from anywhere, so the same holds
///   for every character inside one.
/// - A back reference passes only when it repeats a group of its own pattern
///   that closed before it and holds no space and no wide space.
/// - In a pattern that repeats its begin's text, an end or a while, `\N`
///   repeats the begin's text, so it fails.
/// - A call passes only once the match has left the run, outside a lookbehind.
/// - A pattern nishiki's parser refuses fails.
#[must_use]
pub fn treats_wide_spaces_as_spaces(regex: &str, repeats_begin: bool) -> bool {
    let Ok(parsed) = parse(regex, Dialect::Grammar) else {
        return false;
    };
    let mut walk = Walk {
        groups: vec![Group::default(); parsed.groups + 1],
        names: &parsed.names,
        repeats_begin,
    };
    walk.node(&parsed.root, true, false).is_ok()
}

/// A capture group, as the walk has met it so far.
#[derive(Clone, Copy, Debug, Default)]
struct Group {
    /// Whether the walk has entered the group.
    open: bool,
    /// Whether the walk has left the group.
    closed: bool,
    /// Whether the group can hold a space or a wide space.
    alike: bool,
}

/// A pattern that can match a wide space where it fails on a space, or that
/// the walk cannot prove matches both alike.
#[derive(Debug)]
struct Unproven;

/// The walk over one pattern's tree.
#[derive(Debug)]
struct Walk<'p> {
    /// Each capture group, by its number from 1.
    groups: Vec<Group>,
    /// The number of each named group.
    names: &'p HashMap<String, usize>,
    /// Whether `\N` repeats the begin's text.
    repeats_begin: bool,
}

impl Walk<'_> {
    /// Walks `node` from a position inside the run when `in_run` holds.
    /// Returns whether the match may still be inside the run after it.
    /// `strict` holds inside a lookbehind, whose characters can read the run
    /// from anywhere in the match.
    fn node(&mut self, node: &Node, in_run: bool, strict: bool) -> Result<bool, Unproven> {
        match node {
            Node::Empty | Node::Anchor(_) => Ok(in_run),
            Node::Char(c) => self.character(|x| x == *c, in_run, strict),
            Node::Any { .. } => self.character(|_| true, in_run, strict),
            Node::Class(class) => self.character(|x| class.matches(x), in_run, strict),
            Node::Sequence(items) => items
                .iter()
                .try_fold(in_run, |stays, item| self.node(item, stays, strict)),
            Node::Alternation(alternatives) | Node::Folds(alternatives) => {
                alternatives.iter().try_fold(false, |stays, alternative| {
                    Ok(self.node(alternative, in_run, strict)? || stays)
                })
            }
            Node::Repeat { node, min, .. } => {
                let stays = self.node(node, in_run, strict)?;
                Ok(if *min == 0 { in_run } else { stays })
            }
            Node::Group { index, node } => {
                if let Some(group) = index.and_then(|index| self.groups.get_mut(index)) {
                    group.open = true;
                }
                let stays = self.node(node, in_run, strict)?;
                if let Some(group) = index.and_then(|index| self.groups.get_mut(index)) {
                    group.closed = true;
                }
                Ok(stays)
            }
            Node::Lookaround { ahead, node, .. } => {
                self.node(node, in_run, strict || !*ahead)?;
                Ok(in_run)
            }
            Node::Atomic(node) => self.node(node, in_run, strict),
            Node::BackReference(index, _) if self.names.is_empty() => {
                self.reference(*index, in_run)
            }
            Node::BackReference(..) => Err(Unproven),
            Node::NamedBackReference(name, _) => {
                let index = *self.names.get(name).ok_or(Unproven)?;
                self.reference(index, in_run)
            }
            Node::Subroutine(_) | Node::NamedSubroutine(_) => {
                if in_run || strict {
                    return Err(Unproven);
                }
                self.mark_open_alike();
                Ok(false)
            }
        }
    }

    /// One character of the match, whose set `member` tests.
    fn character(
        &mut self,
        member: impl Fn(char) -> bool,
        in_run: bool,
        strict: bool,
    ) -> Result<bool, Unproven> {
        let taken = ALIKE.iter().filter(|&&c| member(c)).count();
        if taken > 0 {
            self.mark_open_alike();
        }
        if !in_run && !strict {
            return Ok(false);
        }
        if taken != 0 && taken != ALIKE.len() {
            return Err(Unproven);
        }
        Ok(in_run && (taken > 0 || member('\t')))
    }

    /// A back reference to group `index`.
    /// It repeats text that holds no space and no wide space only when its
    /// group closed before it and can hold neither.
    /// Then the repeated text matches a run of spaces where it matches a
    /// mixed run.
    fn reference(&self, index: usize, in_run: bool) -> Result<bool, Unproven> {
        match self.groups.get(index) {
            Some(group) if !self.repeats_begin && group.closed && !group.alike => Ok(in_run),
            Some(_) | None => Err(Unproven),
        }
    }

    /// Marks each group the walk is inside as one that can hold a space or a
    /// wide space.
    fn mark_open_alike(&mut self) {
        for group in &mut self.groups {
            if group.open && !group.closed {
                group.alike = true;
            }
        }
    }
}
