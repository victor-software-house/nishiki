//! What the writer proves about a pattern's tree before it writes code.
//!
//! Each proof is local to a node and to what follows it.
//! A proof that fails leaves the general code in place, never a wrong one.

use nishiki::unstable::regex::{Anchor, Greediness, Node};

use super::chars::Chars;

/// Adds the characters a match of `node` can start with to `set`.
/// Returns true when the node can match without reading.
/// It works on characters, not lead bytes: `\w` and `\s` share every
/// non-ASCII lead byte, but no character.
pub(super) fn first_chars(node: &Node, set: &mut Chars) -> bool {
    match node {
        Node::Lookaround {
            ahead: true,
            negative: false,
            node,
        } => {
            let mut inner = Chars::default();
            if first_chars(node, &mut inner) {
                return true;
            }
            set.add(&inner);
            false
        }
        Node::Empty | Node::Anchor(_) | Node::Lookaround { .. } => true,
        Node::Char(c) => {
            set.add(&Chars::one(*c));
            false
        }
        Node::Class(class) => {
            set.add(&Chars::of_class(class));
            false
        }
        Node::Any { .. } => {
            set.add(&Chars::all());
            false
        }
        Node::BackReference(..)
        | Node::NamedBackReference(..)
        | Node::Subroutine(_)
        | Node::NamedSubroutine(_) => {
            set.add(&Chars::all());
            true
        }
        Node::Sequence(items) => items.iter().all(|item| first_chars(item, set)),
        Node::Alternation(alts) | Node::Folds(alts) => alts
            .iter()
            .fold(false, |empty, alt| first_chars(alt, set) | empty),
        Node::Repeat { node, min, .. } => first_chars(node, set) || *min == 0,
        Node::Group { node, .. } | Node::Atomic(node) => first_chars(node, set),
    }
}

/// Adds the characters a match of `node` can end with to `set`.
/// Returns true when the node can match without reading.
/// A lookaround and an anchor read nothing, so the character before them is
/// the one before the node.
fn last_chars(node: &Node, set: &mut Chars) -> bool {
    match node {
        Node::Empty | Node::Anchor(_) | Node::Lookaround { .. } => true,
        Node::Char(c) => {
            set.add(&Chars::one(*c));
            false
        }
        Node::Class(class) => {
            set.add(&Chars::of_class(class));
            false
        }
        Node::Any { .. } => {
            set.add(&Chars::all());
            false
        }
        Node::BackReference(..)
        | Node::NamedBackReference(..)
        | Node::Subroutine(_)
        | Node::NamedSubroutine(_) => {
            set.add(&Chars::all());
            true
        }
        Node::Sequence(items) => items.iter().rev().all(|item| last_chars(item, set)),
        Node::Alternation(alts) | Node::Folds(alts) => alts
            .iter()
            .fold(false, |empty, alt| last_chars(alt, set) | empty),
        Node::Repeat { node, min, .. } => last_chars(node, set) || *min == 0,
        Node::Group { node, .. } | Node::Atomic(node) => last_chars(node, set),
    }
}

/// The one-sided test that replaces a `\b` or `\B` when one side is settled.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum WordTest {
    /// The character at the position must not be a word character.
    NoWordAt,
    /// The character at the position must be a word character.
    WordAt,
    /// The character before the position must not be a word character.
    NoWordBefore,
    /// The character before the position must be a word character.
    WordBefore,
}

/// A one-sided test for `\b` or `\B`, when one side of it is settled.
///
/// - `before` are the nodes before the anchor in its sequence. When they must
///   read, the character before the anchor is one they end with.
/// - `after` is what follows the anchor. When it must read, the match fails
///   unless the character at the anchor is one it starts with.
///
/// When every such character is a `\w`, or none is, that side is known.
/// The test then reads only the other side.
pub(super) fn word_test(boundary: bool, before: &[Node], after: &Follow) -> Option<WordTest> {
    let mut last = Chars::default();
    let reads_before = !before.iter().rev().all(|node| last_chars(node, &mut last));
    let reads_after = !after.always && !after.empty;
    let word = Chars::word();
    let (at_word, before_word) = if reads_before && last.within(&word) {
        (Some(false), None)
    } else if reads_before && !last.meets(&word) {
        (Some(true), None)
    } else if reads_after && after.first.within(&word) {
        (None, Some(false))
    } else if reads_after && !after.first.meets(&word) {
        (None, Some(true))
    } else {
        (None, None)
    };
    match (at_word, before_word) {
        (Some(word_wanted), _) => Some(if word_wanted == boundary {
            WordTest::WordAt
        } else {
            WordTest::NoWordAt
        }),
        (None, Some(word_wanted)) => Some(if word_wanted == boundary {
            WordTest::WordBefore
        } else {
            WordTest::NoWordBefore
        }),
        (None, None) => None,
    }
}

/// What can follow a node, as far as the analysis needs.
#[derive(Clone, Debug)]
pub(super) struct Follow {
    /// What follows succeeds at every position, without reading.
    pub(super) always: bool,
    /// The characters what follows can start with, when it reads.
    pub(super) first: Chars,
    /// What follows can succeed without reading, under some condition.
    pub(super) empty: bool,
}

impl Follow {
    /// The end of the pattern.
    pub(super) fn always() -> Self {
        Follow {
            always: true,
            first: Chars::all(),
            empty: true,
        }
    }

    /// What follows `rest`, then `self`.
    /// A rest that can match empty reads nothing of its own, so it succeeds
    /// without reading only where `self` can.
    pub(super) fn after(&self, rest: &[Node]) -> Follow {
        if self.always && rest.iter().all(unconditional) {
            return Follow::always();
        }
        let mut first = Chars::default();
        for node in rest {
            if !first_chars(node, &mut first) {
                return Follow {
                    always: false,
                    first,
                    empty: false,
                };
            }
        }
        first.add(&self.first);
        Follow {
            always: false,
            first,
            empty: self.empty || self.always,
        }
    }

    /// Whether what follows must read a character outside `set`.
    pub(super) fn shuns(&self, set: &Chars) -> bool {
        !self.always && !self.empty && !self.first.meets(set)
    }

    /// What follows a turn of a repeat whose body starts with `set`.
    pub(super) fn after_turn(&self, set: &Chars, empty: bool) -> Follow {
        let mut first = set.clone();
        first.add(&self.first);
        Follow {
            always: false,
            first,
            empty: self.empty || self.always || empty,
        }
    }
}

/// The characters a match of `node` can start with, and whether it can be
/// empty.
pub(super) fn first_of(node: &Node) -> (Chars, bool) {
    let mut set = Chars::default();
    let empty = first_chars(node, &mut set);
    (set, empty)
}

/// Whether `node` succeeds at every position without reading.
fn unconditional(node: &Node) -> bool {
    match node {
        Node::Empty | Node::Repeat { min: 0, .. } => true,
        Node::Group { node, .. } | Node::Atomic(node) => unconditional(node),
        Node::Sequence(nodes) => nodes.iter().all(unconditional),
        Node::Alternation(nodes) | Node::Folds(nodes) => nodes.iter().any(unconditional),
        Node::Char(_)
        | Node::Any { .. }
        | Node::Class(_)
        | Node::Repeat { .. }
        | Node::Lookaround { .. }
        | Node::BackReference(..)
        | Node::NamedBackReference(..)
        | Node::Subroutine(_)
        | Node::NamedSubroutine(_)
        | Node::Anchor(_) => false,
    }
}

/// Whether the first match of `node`, followed by `follow`, is the only match
/// that can lead to success.
/// Backtracking into such a node never finds another match of the pattern,
/// so a turn of a repeat can keep its first match.
///
/// The proof is local:
///
/// - A one-character node, an anchor, a lookaround or an atomic group has one
///   match.
/// - A sequence is unique when each part is, given what follows that part.
/// - An alternation is unique when its arms are, and no two arms can start with
///   the same character.
/// - A greedy repeat that can give back a turn is unique when what follows must
///   read a character its body cannot start with. Giving back a turn leaves a
///   position where the body matched, so what follows fails there.
/// - A lazy repeat is unique only with a fixed count. Its first match takes the
///   fewest turns, and only the most turns can lead to success.
pub(super) fn unique(node: &Node, follow: &Follow) -> bool {
    if follow.always {
        return true;
    }
    match node {
        Node::Empty
        | Node::Char(_)
        | Node::Any { .. }
        | Node::Class(_)
        | Node::Anchor(_)
        | Node::Lookaround { .. }
        | Node::Atomic(_)
        | Node::Repeat {
            greediness: Greediness::Possessive,
            ..
        } => true,
        Node::BackReference(..)
        | Node::NamedBackReference(..)
        | Node::Subroutine(_)
        | Node::NamedSubroutine(_) => false,
        Node::Group { node, .. } => unique(node, follow),
        Node::Sequence(nodes) => nodes
            .iter()
            .enumerate()
            .all(|(i, node)| unique(node, &follow.after(nodes.get(i + 1..).unwrap_or(&[])))),
        Node::Alternation(arms) | Node::Folds(arms) => {
            let mut seen = Chars::default();
            arms.iter().all(|arm| {
                let (mut set, empty) = first_of(arm);
                if empty {
                    if follow.empty {
                        return false;
                    }
                    set.add(&follow.first);
                }
                let clash = set.meets(&seen);
                seen.add(&set);
                !clash && unique(arm, follow)
            })
        }
        Node::Repeat {
            node,
            min,
            max,
            greediness,
        } => {
            let (set, empty) = first_of(node);
            let fixed = *max == Some(*min);
            let longest = *greediness == Greediness::Greedy && !empty && follow.shuns(&set);
            (fixed || longest) && unique(node, &follow.after_turn(&set, empty))
        }
    }
}

/// Makes every capturing group outside `reads` a plain group.
/// `reads` holds the groups the lexer reads and the groups a
/// back-reference reads.
pub(super) fn keep_read_groups(node: &mut Node, reads: &[usize]) {
    match node {
        Node::Group { index, node } => {
            if index.is_some_and(|group| !reads.contains(&group)) {
                *index = None;
            }
            keep_read_groups(node, reads);
        }
        Node::Sequence(nodes) | Node::Alternation(nodes) | Node::Folds(nodes) => {
            for node in nodes {
                keep_read_groups(node, reads);
            }
        }
        Node::Repeat { node, .. } | Node::Lookaround { node, .. } | Node::Atomic(node) => {
            keep_read_groups(node, reads);
        }
        Node::Empty
        | Node::Char(_)
        | Node::Any { .. }
        | Node::Class(_)
        | Node::BackReference(..)
        | Node::NamedBackReference(..)
        | Node::Subroutine(_)
        | Node::NamedSubroutine(_)
        | Node::Anchor(_) => {}
    }
}

/// Adds the group of each back-reference in `node` to `groups`.
pub(super) fn back_reference_groups(node: &Node, groups: &mut Vec<usize>) {
    match node {
        Node::BackReference(group, _) => groups.push(*group),
        Node::Sequence(nodes) | Node::Alternation(nodes) | Node::Folds(nodes) => {
            for node in nodes {
                back_reference_groups(node, groups);
            }
        }
        Node::Group { node, .. }
        | Node::Repeat { node, .. }
        | Node::Lookaround { node, .. }
        | Node::Atomic(node) => back_reference_groups(node, groups),
        Node::Empty
        | Node::Char(_)
        | Node::Any { .. }
        | Node::Class(_)
        | Node::NamedBackReference(..)
        | Node::Subroutine(_)
        | Node::NamedSubroutine(_)
        | Node::Anchor(_) => {}
    }
}

/// The body of capturing group `index` in `node`.
pub(super) fn group_body(node: &Node, index: usize) -> Option<&Node> {
    match node {
        Node::Group {
            index: Some(group),
            node,
        } if *group == index => Some(node),
        Node::Sequence(nodes) | Node::Alternation(nodes) | Node::Folds(nodes) => {
            nodes.iter().find_map(|node| group_body(node, index))
        }
        Node::Group { node, .. }
        | Node::Repeat { node, .. }
        | Node::Lookaround { node, .. }
        | Node::Atomic(node) => group_body(node, index),
        Node::Empty
        | Node::Char(_)
        | Node::Any { .. }
        | Node::Class(_)
        | Node::BackReference(..)
        | Node::NamedBackReference(..)
        | Node::Subroutine(_)
        | Node::NamedSubroutine(_)
        | Node::Anchor(_) => None,
    }
}

/// The first byte every match of `node` reads, when `node` starts with a
/// literal character.
/// It sees through groups and the first item of a sequence.
/// A capturing group notes its start without reading, so it adds no read
/// before the character.
pub(super) fn first_byte(node: &Node) -> Option<u8> {
    match node {
        Node::Char(c) => c.to_string().bytes().next(),
        Node::Sequence(nodes) => nodes.first().and_then(first_byte),
        Node::Group { node, .. } => first_byte(node),
        Node::Empty
        | Node::Any { .. }
        | Node::Class(_)
        | Node::Alternation(_)
        | Node::Folds(_)
        | Node::Repeat { .. }
        | Node::Lookaround { .. }
        | Node::Atomic(_)
        | Node::BackReference(..)
        | Node::NamedBackReference(..)
        | Node::Subroutine(_)
        | Node::NamedSubroutine(_)
        | Node::Anchor(_) => None,
    }
}

/// The position anchor every match of `node` starts with, if any.
///
/// Such a pattern matches only where its anchor holds.
/// Its dispatch arm tests the anchor before it calls the pattern's function.
/// The scan can try a pattern at every byte, so the saved call can be per byte.
/// Word boundaries stay in the function, where the writer reads one side from
/// the next node.
pub(super) fn leading_anchor(node: &Node) -> Option<Anchor> {
    match node {
        Node::Anchor(
            anchor @ (Anchor::LineStart
            | Anchor::LineEnd
            | Anchor::TextStart
            | Anchor::TextEnd
            | Anchor::TextEndOrFinalNewline
            | Anchor::LastMatchEnd),
        ) => Some(*anchor),
        Node::Sequence(nodes) => nodes.first().and_then(leading_anchor),
        Node::Group { node, .. } => leading_anchor(node),
        Node::Anchor(Anchor::WordBoundary | Anchor::NotWordBoundary)
        | Node::Empty
        | Node::Char(_)
        | Node::Any { .. }
        | Node::Class(_)
        | Node::Alternation(_)
        | Node::Folds(_)
        | Node::Repeat { .. }
        | Node::Lookaround { .. }
        | Node::Atomic(_)
        | Node::BackReference(..)
        | Node::NamedBackReference(..)
        | Node::Subroutine(_)
        | Node::NamedSubroutine(_) => None,
    }
}

/// The characters of a node that reads exactly one character, a character
/// or a class.
/// Any other node gives `None`.
pub(super) fn one_character_set(node: &Node) -> Option<Chars> {
    match node {
        Node::Char(c) => Some(Chars::one(*c)),
        Node::Class(class) => Some(Chars::of_class(class)),
        Node::Empty
        | Node::Any { .. }
        | Node::Sequence(_)
        | Node::Alternation(_)
        | Node::Folds(_)
        | Node::Repeat { .. }
        | Node::Group { .. }
        | Node::Lookaround { .. }
        | Node::Atomic(_)
        | Node::BackReference(..)
        | Node::NamedBackReference(..)
        | Node::Subroutine(_)
        | Node::NamedSubroutine(_)
        | Node::Anchor(_) => None,
    }
}

/// Whether the arm of `optional` that takes its character can go, because
/// `run` right after it gives the same match without it.
///
/// It holds for `X? Y+` and `X? Y*` side by side in one sequence, the arm
/// drop of `field-pattern-fixes`, design section 2:
///
/// - `X` reads one character, and every character of `X` is in `Y`;
/// - `Y` repeats one character with no upper bound;
/// - `Y` gives back greedily, or `X?` is lazy.
///
/// Then the unit `X Y+ S` ends wherever `Y+ S` ends, with the same groups.
/// When `X?` is greedy, its arm's ends come first in the same order.
/// So the first end where the rest succeeds is the same without the arm.
fn covered_arm(optional: &Node, run: &Node) -> bool {
    let Node::Repeat {
        node: taken,
        min: 0,
        max: Some(1),
        greediness: optional_greediness,
    } = optional
    else {
        return false;
    };
    let Node::Repeat {
        node: repeated,
        max: None,
        greediness: run_greediness,
        ..
    } = run
    else {
        return false;
    };
    let gives_back_in_order = match (optional_greediness, run_greediness) {
        (Greediness::Lazy, Greediness::Greedy | Greediness::Lazy)
        | (Greediness::Greedy, Greediness::Greedy) => true,
        (Greediness::Greedy, Greediness::Lazy)
        | (Greediness::Possessive, _)
        | (_, Greediness::Possessive) => false,
    };
    gives_back_in_order
        && one_character_set(taken).is_some_and(|taken| {
            one_character_set(repeated).is_some_and(|repeated| taken.within(&repeated))
        })
}

/// Drops each optional character that the run right after it covers, and
/// returns how many it dropped.
/// It leaves a lookbehind alone, since the writer reads one backwards.
pub(super) fn drop_covered_arms(node: &mut Node) -> usize {
    match node {
        Node::Sequence(nodes) => {
            let mut dropped = 0;
            let mut index = 0;
            while let (Some(optional), Some(run)) = (nodes.get(index), nodes.get(index + 1)) {
                if covered_arm(optional, run) {
                    nodes.remove(index);
                    dropped += 1;
                } else {
                    index += 1;
                }
            }
            dropped + nodes.iter_mut().map(drop_covered_arms).sum::<usize>()
        }
        Node::Alternation(nodes) | Node::Folds(nodes) => {
            nodes.iter_mut().map(drop_covered_arms).sum()
        }
        Node::Repeat { node, .. }
        | Node::Group { node, .. }
        | Node::Lookaround {
            ahead: true, node, ..
        }
        | Node::Atomic(node) => drop_covered_arms(node),
        Node::Lookaround { ahead: false, .. }
        | Node::Empty
        | Node::Char(_)
        | Node::Any { .. }
        | Node::Class(_)
        | Node::BackReference(..)
        | Node::NamedBackReference(..)
        | Node::Subroutine(_)
        | Node::NamedSubroutine(_)
        | Node::Anchor(_) => 0,
    }
}

/// The scan skip a pattern earns, from the `field-pattern-fixes` design,
/// section 3.
pub(super) struct ScanSkip<'a> {
    /// The character the run `R` repeats.
    pub(super) run: &'a Node,
    /// `k`: the most bytes of run characters a match of the prefix `P`
    /// begins with.
    /// Counted in bytes, it gives the skip's end with one subtraction.
    pub(super) reach: usize,
}

/// What a node of the prefix `P` can read of a run, in bytes.
#[derive(Clone, Copy)]
struct Reach {
    /// Whether the node matches empty.
    empty: bool,
    /// The longest match made only of run characters, when one exists.
    whole: Option<usize>,
    /// The most bytes of run characters a match begins with.
    begins: usize,
}

impl Reach {
    /// The reach of a node that matches only empty.
    const EMPTY: Reach = Reach {
        empty: true,
        whole: Some(0),
        begins: 0,
    };

    /// The reach of an alternation with no arm yet.
    const NONE: Reach = Reach {
        empty: false,
        whole: None,
        begins: 0,
    };

    /// The reach of a node that reads one character.
    /// `width` is the UTF-8 length of the widest run character it can read,
    /// or 0 when it reads none.
    fn one(width: usize) -> Reach {
        Reach {
            empty: false,
            whole: (width > 0).then_some(width),
            begins: width,
        }
    }

    /// The reach of this node followed by `next`.
    fn then(self, next: Reach) -> Reach {
        Reach {
            empty: self.empty && next.empty,
            whole: self.whole.zip(next.whole).map(|(a, b)| a + b),
            begins: self
                .whole
                .map_or(self.begins, |whole| self.begins.max(whole + next.begins)),
        }
    }

    /// The reach of this node or `other`.
    fn or(self, other: Reach) -> Reach {
        Reach {
            empty: self.empty || other.empty,
            whole: self.whole.max(other.whole),
            begins: self.begins.max(other.begins),
        }
    }

    /// The reach of a greedy or lazy repeat of `turn`.
    fn repeat(turn: Reach, min: u32, max: Option<u32>) -> Option<Reach> {
        let empty = min == 0 || turn.empty;
        match (turn.whole, max) {
            (Some(whole), None) if whole > 0 => None,
            (Some(whole), Some(max)) => {
                let max = usize::try_from(max).ok()?;
                Some(Reach {
                    empty,
                    whole: Some(whole.checked_mul(max)?),
                    begins: match max.checked_sub(1) {
                        Some(full) => whole.checked_mul(full)?.checked_add(turn.begins)?,
                        None => 0,
                    },
                })
            }
            (Some(_), None) => Some(Reach {
                empty,
                whole: Some(0),
                begins: turn.begins,
            }),
            (None, _) => Some(Reach {
                empty,
                whole: (min == 0).then_some(0),
                begins: turn.begins,
            }),
        }
    }
}

/// The reach of `node`, a part of the prefix `P`, over a run of `set`.
/// It gives `None` for a node the skip refuses in `P`:
///
/// - an assertion or a lookaround, which holds at some starts only;
/// - an atomic group or a possessive repeat, which can refuse the empty path;
/// - a back-reference or a call;
/// - a repeat with no upper bound of run characters, which makes `k` unbounded.
///
/// Each count is an upper bound, which keeps the skip exact.
fn reach(node: &Node, set: &Chars) -> Option<Reach> {
    match node {
        Node::Empty => Some(Reach::EMPTY),
        Node::Char(c) => Some(Reach::one(Chars::one(*c).widest_shared(set))),
        Node::Class(class) => Some(Reach::one(Chars::of_class(class).widest_shared(set))),
        Node::Any { .. } => Some(Reach::one(Chars::all().widest_shared(set))),
        Node::Sequence(nodes) => nodes.iter().try_fold(Reach::EMPTY, |sofar, node| {
            Some(sofar.then(reach(node, set)?))
        }),
        Node::Alternation(nodes) | Node::Folds(nodes) => nodes
            .iter()
            .try_fold(Reach::NONE, |sofar, node| Some(sofar.or(reach(node, set)?))),
        Node::Repeat {
            node,
            min,
            max,
            greediness: Greediness::Greedy | Greediness::Lazy,
        } => Reach::repeat(reach(node, set)?, *min, *max),
        Node::Group { node, .. } => reach(node, set),
        Node::Repeat {
            greediness: Greediness::Possessive,
            ..
        }
        | Node::Lookaround { .. }
        | Node::Atomic(_)
        | Node::BackReference(..)
        | Node::NamedBackReference(..)
        | Node::Subroutine(_)
        | Node::NamedSubroutine(_)
        | Node::Anchor(_) => None,
    }
}

/// The character a run repeats, when `node` starts with a run that gives
/// back.
/// The run is a repeat of one character or class with no upper bound,
/// greedy or lazy.
/// It is `node` itself, or first inside the groups and sequences of `node`,
/// never inside an atomic group.
fn leading_run(node: &Node) -> Option<&Node> {
    match node {
        Node::Repeat {
            node,
            max: None,
            greediness: Greediness::Greedy | Greediness::Lazy,
            ..
        } => one_character_set(node).map(|_| &**node),
        Node::Group { node, .. } => leading_run(node),
        Node::Sequence(nodes) => nodes.first().and_then(leading_run),
        Node::Repeat { .. }
        | Node::Empty
        | Node::Char(_)
        | Node::Any { .. }
        | Node::Class(_)
        | Node::Alternation(_)
        | Node::Folds(_)
        | Node::Lookaround { .. }
        | Node::Atomic(_)
        | Node::BackReference(..)
        | Node::NamedBackReference(..)
        | Node::Subroutine(_)
        | Node::NamedSubroutine(_)
        | Node::Anchor(_) => None,
    }
}

/// Whether a try's result depends only on the text and the start.
/// `\G` reads where the search began.
/// A back-reference reads a group, and a call can run one.
fn reads_only_the_text(node: &Node) -> bool {
    match node {
        Node::Anchor(Anchor::LastMatchEnd)
        | Node::BackReference(..)
        | Node::NamedBackReference(..)
        | Node::Subroutine(_)
        | Node::NamedSubroutine(_) => false,
        Node::Sequence(nodes) | Node::Alternation(nodes) | Node::Folds(nodes) => {
            nodes.iter().all(reads_only_the_text)
        }
        Node::Group { node, .. }
        | Node::Repeat { node, .. }
        | Node::Lookaround { node, .. }
        | Node::Atomic(node) => reads_only_the_text(node),
        Node::Empty | Node::Char(_) | Node::Any { .. } | Node::Class(_) | Node::Anchor(_) => true,
    }
}

/// The scan skip `root` earns, under the conditions of the
/// `field-pattern-fixes` design, section 3.1.
///
/// The pattern is a sequence `P R S`:
///
/// - `R` is a run that gives back, found by [`leading_run`] in one item.
/// - `P` is the items before it, and matches empty.
/// - [`reach`] accepts `P` and bounds `k`.
/// - [`reads_only_the_text`] holds for the whole pattern.
///
/// Of several such items, the last is `R`, so `P` takes the earlier runs.
///
/// A run of `\s` characters earns no skip.
/// The scan already passes the later blanks of a run of spaces and tabs.
/// Measured on Protocol Buffers, four such skips cost 65,000 instructions
/// and saved none.
pub(super) fn scan_skip(root: &Node) -> Option<ScanSkip<'_>> {
    let Node::Sequence(nodes) = root else {
        return None;
    };
    if !reads_only_the_text(root) {
        return None;
    }
    let space = Chars::space();
    nodes.iter().enumerate().rev().find_map(|(index, node)| {
        let run = leading_run(node)?;
        let set = one_character_set(run)?;
        if set.within(&space) {
            return None;
        }
        let prefix = nodes
            .get(..index)?
            .iter()
            .try_fold(Reach::EMPTY, |sofar, node| {
                Some(sofar.then(reach(node, &set)?))
            })?;
        prefix.empty.then_some(ScanSkip {
            run,
            reach: prefix.begins,
        })
    })
}
