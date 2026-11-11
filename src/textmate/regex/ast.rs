//! The syntax tree of a pattern, and the character classes in it.
//!
//! The tree is visible to the whole crate, and public through
//! `nishiki::unstable::regex`.
//! Code that reads a pattern outside the regex engine reads this tree, so
//! the crate has one parser.

use super::analysis::{first_bytes, lead_byte};
use super::folds::{LARGEST_ORBIT, MULTI, NEXT, UNFOLD};

/// A position the engine can test without consuming text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(
    not(all(nishiki_unstable, feature = "_rule-engine")),
    expect(unreachable_pub, reason = "public only through nishiki::unstable")
)]
pub enum Anchor {
    /// `^`: the start of the text, or after a newline that is not its end.
    LineStart,
    /// `$`: the end of the text, or before a newline.
    LineEnd,
    /// `\A`: the start of the document, where the caller marks it.
    TextStart,
    /// `\z`: the end of the text.
    TextEnd,
    /// `\Z`: the end of the text, or before a final newline.
    TextEndOrFinalNewline,
    /// `\G`: where the last match of the grammar ended.
    LastMatchEnd,
    /// `\b`.
    WordBoundary,
    /// `\B`.
    NotWordBoundary,
}

/// Whether letters match only in the case the pattern spells them.
/// Oniguruma's `(?i)` flag decides it.
/// Characters and classes take it in when the pattern parses.
/// A back-reference keeps it, because it compares two pieces of the text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(
    not(all(nishiki_unstable, feature = "_rule-engine")),
    expect(unreachable_pub, reason = "public only through nishiki::unstable")
)]
pub enum Case {
    /// Only in the pattern's case.
    Sensitive,
    /// In any case.
    Insensitive,
}

/// Whether a set of characters matches its members or every other
/// character, as `\d` and `\D` differ.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(
    not(all(nishiki_unstable, feature = "_rule-engine")),
    expect(unreachable_pub, reason = "public only through nishiki::unstable")
)]
pub enum Polarity {
    /// Its members.
    Positive,
    /// Every character but its members.
    Negated,
}

impl Polarity {
    /// The other polarity.
    pub(super) fn inverse(self) -> Polarity {
        match self {
            Polarity::Positive => Polarity::Negated,
            Polarity::Negated => Polarity::Positive,
        }
    }
}

/// How a repeat takes its iterations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(
    not(all(nishiki_unstable, feature = "_rule-engine")),
    expect(unreachable_pub, reason = "public only through nishiki::unstable")
)]
pub enum Greediness {
    /// As many as it can, giving back on failure.
    Greedy,
    /// As few as it can, taking more on failure.
    Lazy,
    /// As many as it can, never giving back.
    Possessive,
}

/// One member of a class.
#[derive(Clone, Debug)]
#[cfg_attr(
    not(all(nishiki_unstable, feature = "_rule-engine")),
    expect(unreachable_pub, reason = "public only through nishiki::unstable")
)]
pub enum Member {
    /// `a-z`, or a single character as `a-a`.
    Range(char, char),
    /// `\d`, `[:alpha:]` or `\p{Lu}`, negated or not.
    Property(Property, Polarity),
    /// A class inside a class.
    Nested(Class),
}

impl Member {
    /// The item's members below 128, one bit per code point.
    fn ascii(&self) -> u128 {
        match self {
            Member::Range(lo, hi) => ascii_range(u32::from(*lo), u32::from(*hi)),
            Member::Property(Property(ranges), polarity) => {
                let held = ranges
                    .iter()
                    .fold(0, |held, &(first, last)| held | ascii_range(first, last));
                match polarity {
                    Polarity::Positive => held,
                    Polarity::Negated => !held,
                }
            }
            Member::Nested(class) => class.ascii(),
        }
    }
}

/// The code points `first..=last` below 128, one bit each.
fn ascii_range(first: u32, last: u32) -> u128 {
    let last = last.min(127);
    if first > last {
        return 0;
    }
    (u128::MAX >> (127 - last)) & (u128::MAX << first)
}

/// A character class: the intersection of unions of items, negated or not.
#[derive(Clone, Debug)]
#[cfg_attr(
    not(all(nishiki_unstable, feature = "_rule-engine")),
    expect(unreachable_pub, reason = "public only through nishiki::unstable")
)]
pub struct Class {
    /// Whether the class matches every character outside its members.
    pub negated: bool,
    /// The sets the class intersects, each a union of its members.
    /// `[a-z&&[^aeiou]]` has two.
    pub unions: Vec<Vec<Member>>,
}

/// A named set of characters: ranges of code points, ascending.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(
    not(all(nishiki_unstable, feature = "_rule-engine")),
    expect(unreachable_pub, reason = "public only through nishiki::unstable")
)]
pub struct Property(&'static [(u32, u32)]);

impl Property {
    /// Every character.
    pub(super) const ANY: Property = Property(&[(0, 0x10_ffff)]);

    /// Whether `c` is in the set.
    pub(super) fn matches(self, c: char) -> bool {
        let code = u32::from(c);
        let at = self.0.partition_point(|&(_, last)| last < code);
        self.0.get(at).is_some_and(|&(first, _)| first <= code)
    }
}

/// The set a `\p{..}` or `[:..:]` name stands for, as Oniguruma measures it:
/// names ignore case, `_`, `-` and spaces.
pub(super) fn property_named(name: &str) -> Option<Property> {
    let lower = name.to_ascii_lowercase().replace(['_', ' ', '-'], "");
    let key = match lower.as_str() {
        "any" => return Some(Property::ANY),
        "letter" => "l",
        "uppercaseletter" => "lu",
        "lowercaseletter" => "ll",
        "titlecaseletter" => "lt",
        "modifierletter" => "lm",
        "otherletter" => "lo",
        "mark" => "m",
        "nonspacingmark" => "mn",
        "spacingmark" => "mc",
        "enclosingmark" => "me",
        "number" => "n",
        "decimalnumber" => "nd",
        "letternumber" => "nl",
        "othernumber" => "no",
        "punctuation" => "p",
        "connectorpunctuation" => "pc",
        "dashpunctuation" => "pd",
        "openpunctuation" => "ps",
        "closepunctuation" => "pe",
        "initialpunctuation" => "pi",
        "finalpunctuation" => "pf",
        "otherpunctuation" => "po",
        "symbol" => "s",
        "mathsymbol" => "sm",
        "currencysymbol" => "sc",
        "modifiersymbol" => "sk",
        "othersymbol" => "so",
        "separator" => "z",
        "spaceseparator" => "zs",
        "lineseparator" => "zl",
        "paragraphseparator" => "zp",
        "other" => "c",
        "control" => "cc",
        "format" => "cf",
        "privateuse" => "co",
        "unassigned" => "cn",
        "surrogate" => "cs",
        "xids" => "xidstart",
        "xidc" => "xidcontinue",
        "ids" => "idstart",
        "idc" => "idcontinue",
        "wspace" => "whitespace",
        "hex" | "hexdigit" | "asciihexdigit" => "xdigit",
        other => other,
    };
    super::unicode::lookup(key).map(Property)
}

/// A set `property_named` is known to have.
pub(super) fn builtin(name: &str) -> Property {
    property_named(name).unwrap_or(Property::ANY)
}

/// `\w`, and what `\b` tests.
///
/// Inlined across codegen units.
/// At 16 units it stayed a call from the machine.
#[inline]
pub(super) fn is_word(c: char) -> bool {
    if c.is_ascii() {
        return c.is_ascii_alphanumeric() || c == '_';
    }
    builtin("word").matches(c)
}

/// The characters `(?i)` takes as one: a character and those its simple case
/// folds join it to, the character itself first.
pub(super) struct Orbit {
    members: [char; LARGEST_ORBIT],
    len: usize,
}

impl Orbit {
    pub(super) fn members(&self) -> &[char] {
        self.members.get(..self.len).unwrap_or(&[])
    }

    fn into_members(self) -> impl Iterator<Item = char> {
        self.members.into_iter().take(self.len)
    }
}

/// The orbit of `c`, by following its cycle in the fold table.
pub(super) fn orbit(c: char) -> Orbit {
    let mut orbit = Orbit {
        members: [c; LARGEST_ORBIT],
        len: 1,
    };
    let mut code = u32::from(c);
    while let Ok(at) = NEXT.binary_search_by_key(&code, |&(member, _)| member) {
        let Some(&(_, next)) = NEXT.get(at) else {
            break;
        };
        let Some(member) = char::from_u32(next).filter(|&m| m != c) else {
            break;
        };
        let Some(slot) = orbit.members.get_mut(orbit.len) else {
            break;
        };
        *slot = member;
        orbit.len += 1;
        code = next;
    }
    orbit
}

/// A character as `(?i)` reads it.
/// It is the character itself when nothing folds with it.
/// Otherwise it is the class of its orbit.
fn insensitive_char(c: char) -> Node {
    match orbit(c).members() {
        [_] => Node::Char(c),
        members => members_class(members.iter().copied()),
    }
}

/// The class of exactly these characters.
fn members_class(members: impl Iterator<Item = char>) -> Node {
    Node::Class(Box::new(Class {
        negated: false,
        unions: vec![members.map(|m| Member::Range(m, m)).collect()],
    }))
}

/// The smallest member of `c`'s orbit, the form folds are written in.
fn canonical(c: char) -> char {
    orbit(c).members().iter().copied().min().unwrap_or(c)
}

/// `c`'s fold when it is more than one character, in canonical form.
fn multi_fold(c: char) -> Option<&'static [u32]> {
    MULTI
        .binary_search_by_key(&u32::from(c), |&(member, _)| member)
        .ok()
        .and_then(|at| MULTI.get(at))
        .map(|&(_, fold)| fold)
}

/// The characters whose fold is `fold`, a canonical string.
fn unfold(fold: &[u32]) -> Option<&'static [u32]> {
    UNFOLD
        .binary_search_by(|&(key, _)| key.cmp(fold))
        .ok()
        .and_then(|at| UNFOLD.get(at))
        .map(|&(_, chars)| chars)
}

/// A fold string as `(?i)` reads it: its characters in turn, each in any case.
fn fold_string(fold: &[u32]) -> Node {
    Node::Sequence(
        fold.iter()
            .filter_map(|&code| char::from_u32(code))
            .map(insensitive_char)
            .collect(),
    )
}

/// A run of literal characters as `(?i)` reads it, as Oniguruma does.
///
/// At each place, the run looks at the next three characters, and failing
/// that at the next two.
/// When their simple folds spell the fold of other characters, they also
/// match those characters.
/// The run then goes on after them.
/// For example, `sss` takes `ßs` and refuses `sß`.
///
/// A single character whose fold is longer also matches its fold in any
/// case.
/// For example, `ß` takes `ss`, `SS` and `ſs`.
///
/// In a lookbehind, only the simple folds apply.
/// Oniguruma reads a lookbehind one character at a time.
pub(super) fn insensitive_run(run: &[char], lookbehind: bool) -> Vec<Node> {
    if lookbehind {
        return run.iter().map(|&c| insensitive_char(c)).collect();
    }
    let mut out = Vec::new();
    let mut at = 0;
    while let Some(&c) = run.get(at) {
        let spelled = [3, 2].into_iter().find_map(|len| {
            let slice = run.get(at..at + len)?;
            let key: Vec<u32> = slice.iter().map(|&s| u32::from(canonical(s))).collect();
            unfold(&key).map(|chars| (slice, chars))
        });
        if let Some((slice, chars)) = spelled {
            out.push(Node::Folds(vec![
                Node::Sequence(slice.iter().map(|&s| insensitive_char(s)).collect()),
                members_class(chars.iter().filter_map(|&code| char::from_u32(code))),
            ]));
            at += slice.len();
            continue;
        }
        out.push(match multi_fold(c) {
            Some(fold) => {
                let mut members: Vec<char> = orbit(c).members().to_vec();
                members.extend(
                    unfold(fold)
                        .unwrap_or(&[])
                        .iter()
                        .filter_map(|&code| char::from_u32(code)),
                );
                members.sort_unstable();
                members.dedup();
                Node::Folds(vec![members_class(members.into_iter()), fold_string(fold)])
            }
            None => insensitive_char(c),
        });
        at += 1;
    }
    out
}

/// A class widened by `(?i)`, tried first, then the fold strings of its
/// members that `keep` accepts, as one trie.
fn class_folds(class: Class, keep: impl Fn(&[u32]) -> bool) -> Node {
    let mut strings: Vec<&[u32]> = Vec::new();
    if !class.negated {
        for &(member, fold) in MULTI {
            if char::from_u32(member).is_some_and(|c| class.matches(c))
                && !strings.contains(&fold)
                && keep(fold)
            {
                strings.push(fold);
            }
        }
    }
    if strings.is_empty() {
        return Node::Class(Box::new(class));
    }
    Node::Folds(vec![Node::Class(Box::new(class)), fold_trie(&strings)])
}

/// Leaves out, throughout `node`, the fold strings that a repetition with no
/// upper bound does not need.
///
/// Take such a repetition of a class, alone or after negative lookaheads.
/// A fold string adds nothing there when two conditions hold:
///
/// - the class matches each of the string's characters;
/// - no lookahead can start with one of those characters.
///
/// The repetition then reaches every end the string would, by the class
/// alone and before the string's turn.
/// The lookaheads checked between the string's characters all pass.
/// For example, `(?i)[\w]+` and `(?i)(?:(?!--)[\w])*` match as before.
/// They do not try `ss` at every character they backtrack over.
///
/// A capturing group keeps its strings.
/// Without them, what it captures would differ.
pub(super) fn prune_repeated_folds(node: &mut Node) {
    match node {
        Node::Repeat {
            node: body,
            max: None,
            ..
        } => {
            prune_loop_body(body);
            prune_repeated_folds(body);
        }
        Node::Repeat { node: inner, .. }
        | Node::Group { node: inner, .. }
        | Node::Lookaround { node: inner, .. }
        | Node::Atomic(inner) => prune_repeated_folds(inner),
        Node::Sequence(items) | Node::Alternation(items) | Node::Folds(items) => {
            items.iter_mut().for_each(prune_repeated_folds);
        }
        Node::Empty
        | Node::Char(..)
        | Node::Any { .. }
        | Node::Class(..)
        | Node::BackReference(..)
        | Node::NamedBackReference(..)
        | Node::Subroutine(_)
        | Node::NamedSubroutine(_)
        | Node::Anchor(_) => {}
    }
}

/// The body of a repetition with no upper bound, with its class's fold
/// strings pruned.
/// They are pruned when the body is that class, alone or after negative
/// lookaheads.
fn prune_loop_body(body: &mut Node) {
    match body {
        Node::Group { index: None, node } => prune_loop_body(node),
        Node::Folds(_) => prune_class_folds(body, &[]),
        Node::Sequence(items) => {
            if let Some((last, looks)) = items.split_last_mut()
                && looks.iter().all(|look| {
                    matches!(
                        look,
                        Node::Lookaround {
                            ahead: true,
                            negative: true,
                            ..
                        }
                    )
                })
            {
                prune_class_folds(last, looks);
            }
        }
        Node::Empty
        | Node::Char(_)
        | Node::Any { .. }
        | Node::Class(_)
        | Node::Alternation(_)
        | Node::Repeat { .. }
        | Node::Group { .. }
        | Node::Lookaround { .. }
        | Node::Atomic(_)
        | Node::BackReference(..)
        | Node::NamedBackReference(..)
        | Node::Subroutine(_)
        | Node::NamedSubroutine(_)
        | Node::Anchor(_) => {}
    }
}

/// Leaves out the fold strings of a class that `looks` and the repetition
/// make redundant.
/// Any other node stays as it is.
fn prune_class_folds(node: &mut Node, looks: &[Node]) {
    let Node::Folds(alts) = node else { return };
    let Some(Node::Class(class)) = alts.first() else {
        return;
    };
    let mut starts = [false; 256];
    for look in looks {
        if let Node::Lookaround { node: body, .. } = look
            && first_bytes(body, &mut starts)
        {
            return;
        }
    }
    let class = (**class).clone();
    let redundant = |fold: &[u32]| {
        fold.iter().all(|&code| {
            char::from_u32(code).is_some_and(|c| {
                orbit(c)
                    .members()
                    .iter()
                    .all(|&m| class.matches(m) && !starts[usize::from(lead_byte(m))])
            })
        })
    };
    let keep: Vec<&[u32]> = MULTI
        .iter()
        .map(|&(_, fold)| fold)
        .filter(|fold| !redundant(fold))
        .collect();
    *node = class_folds(class, |fold| keep.contains(&fold));
}

/// `node` for a line with no character beyond ASCII that a fold of more than
/// one character involves.
/// Each alternative a fold adds that matches no ASCII-only text is left out.
/// Returns `None` when no fold has such an alternative.
///
/// Such an alternative needs one of those characters.
/// On that line, the returned tree matches what `node` matches.
/// It does not try `ß`, the ligatures or the combining marks at every place
/// they could start.
///
/// A fold none of whose alternatives matches ASCII text keeps them all.
pub(super) fn lean(node: &Node) -> Option<Node> {
    if !has_wide_folds(node) {
        return None;
    }
    let mut changed = false;
    let lean = strip_wide_folds(node, &mut changed);
    changed.then_some(lean)
}

/// Whether some fold in `node` has both an alternative that matches no
/// ASCII-only text and one that does.
/// The walk copies nothing.
/// A pattern with no such fold, as most are, then compiles without a second
/// tree.
fn has_wide_folds(node: &Node) -> bool {
    match node {
        Node::Folds(alts) => {
            let ascii = alts.iter().filter(|alt| reaches_ascii(alt)).count();
            (ascii != 0 && ascii != alts.len()) || alts.iter().any(has_wide_folds)
        }
        Node::Sequence(nodes) | Node::Alternation(nodes) => nodes.iter().any(has_wide_folds),
        Node::Repeat { node: inner, .. }
        | Node::Group { node: inner, .. }
        | Node::Lookaround { node: inner, .. }
        | Node::Atomic(inner) => has_wide_folds(inner),
        Node::Empty
        | Node::Char(..)
        | Node::Any { .. }
        | Node::Class(..)
        | Node::BackReference(..)
        | Node::NamedBackReference(..)
        | Node::Subroutine(_)
        | Node::NamedSubroutine(_)
        | Node::Anchor(_) => false,
    }
}

/// `node` with the fold alternatives that match no text of ASCII alone left
/// out, setting `changed` when it leaves one out.
fn strip_wide_folds(node: &Node, changed: &mut bool) -> Node {
    let strip = |nodes: &[Node], changed: &mut bool| -> Vec<Node> {
        nodes
            .iter()
            .map(|node| strip_wide_folds(node, changed))
            .collect()
    };
    match node {
        Node::Folds(alts) => {
            let ascii: Vec<&Node> = alts.iter().filter(|alt| reaches_ascii(alt)).collect();
            if ascii.is_empty() || ascii.len() == alts.len() {
                return Node::Folds(strip(alts, changed));
            }
            *changed = true;
            let kept: Vec<Node> = ascii
                .into_iter()
                .map(|alt| strip_wide_folds(alt, changed))
                .collect();
            match <[Node; 1]>::try_from(kept) {
                Ok([only]) => only,
                Err(kept) => Node::Folds(kept),
            }
        }
        Node::Sequence(items) => Node::Sequence(strip(items, changed)),
        Node::Alternation(alts) => Node::Alternation(strip(alts, changed)),
        Node::Repeat {
            node: inner,
            min,
            max,
            greediness,
        } => Node::Repeat {
            node: Box::new(strip_wide_folds(inner, changed)),
            min: *min,
            max: *max,
            greediness: *greediness,
        },
        Node::Group { index, node: inner } => Node::Group {
            index: *index,
            node: Box::new(strip_wide_folds(inner, changed)),
        },
        Node::Lookaround {
            ahead,
            negative,
            node: inner,
        } => Node::Lookaround {
            ahead: *ahead,
            negative: *negative,
            node: Box::new(strip_wide_folds(inner, changed)),
        },
        Node::Atomic(inner) => Node::Atomic(Box::new(strip_wide_folds(inner, changed))),
        Node::Empty
        | Node::Char(..)
        | Node::Any { .. }
        | Node::Class(..)
        | Node::BackReference(..)
        | Node::NamedBackReference(..)
        | Node::Subroutine(_)
        | Node::NamedSubroutine(_)
        | Node::Anchor(_) => node.clone(),
    }
}

/// Whether `node` can match some ASCII-only text.
/// True where that cannot be told, such as inside a lookaround or a call.
fn reaches_ascii(node: &Node) -> bool {
    match node {
        Node::Char(c) => c.is_ascii(),
        Node::Class(class) => (0..128_u8).any(|b| class.matches(char::from(b))),
        Node::Sequence(items) => items.iter().all(reaches_ascii),
        Node::Alternation(alts) | Node::Folds(alts) => alts.iter().any(reaches_ascii),
        Node::Repeat { node, min, .. } => *min == 0 || reaches_ascii(node),
        Node::Group { node, .. } | Node::Atomic(node) => reaches_ascii(node),
        Node::Empty
        | Node::Any { .. }
        | Node::Lookaround { .. }
        | Node::BackReference(..)
        | Node::NamedBackReference(..)
        | Node::Subroutine(_)
        | Node::NamedSubroutine(_)
        | Node::Anchor(_) => true,
    }
}

/// The fold strings as one node that shares their prefixes.
/// Strings that start alike take one branch, in the order the first of them
/// came.
/// The first string that matches is then the one the plain alternation would
/// take.
fn fold_trie(strings: &[&[u32]]) -> Node {
    let mut alts = Vec::new();
    let mut taken = vec![false; strings.len()];
    for (i, string) in strings.iter().enumerate() {
        if taken.get(i).copied().unwrap_or(true) {
            continue;
        }
        let Some((&first, _)) = string.split_first() else {
            alts.push(Node::Empty);
            continue;
        };
        let mut rests = Vec::new();
        for (j, other) in strings.iter().enumerate().skip(i) {
            if let Some((&head, rest)) = other.split_first()
                && head == first
                && let Some(slot) = taken.get_mut(j)
                && !*slot
            {
                *slot = true;
                rests.push(rest);
            }
        }
        let head = char::from_u32(first).map_or(Node::Empty, insensitive_char);
        let tail = fold_trie(&rests);
        alts.push(if matches!(tail, Node::Empty) {
            head
        } else {
            Node::Sequence(vec![head, tail])
        });
    }
    match alts.len() {
        0 => Node::Empty,
        1 => alts.pop().unwrap_or(Node::Empty),
        _ => Node::Folds(alts),
    }
}

impl Class {
    /// Whether `c` is in the class.
    #[cfg_attr(
        not(all(nishiki_unstable, feature = "_rule-engine")),
        expect(unreachable_pub, reason = "public only through nishiki::unstable")
    )]
    #[must_use]
    pub fn matches(&self, c: char) -> bool {
        let found = self.unions.iter().all(|union| {
            union.iter().any(|item| match item {
                Member::Range(lo, hi) => (*lo..=*hi).contains(&c),
                Member::Property(prop, polarity) => {
                    prop.matches(c) == (*polarity == Polarity::Positive)
                }
                Member::Nested(class) => class.matches(c),
            })
        });
        found != self.negated
    }

    /// The class's members below 128, one bit per code point, as
    /// [`Class::matches`] decides them.
    /// They come from its ranges and sets rather than a test of each
    /// character.
    #[cfg_attr(
        not(all(nishiki_unstable, feature = "_rule-engine")),
        expect(unreachable_pub, reason = "public only through nishiki::unstable")
    )]
    #[must_use]
    pub fn ascii(&self) -> u128 {
        let found = self.unions.iter().fold(u128::MAX, |all, union| {
            all & union.iter().fold(0, |any, item| any | item.ascii())
        });
        if self.negated { !found } else { found }
    }

    /// The class's members when it is one union of ranges, not negated.
    pub(super) fn ranges(&self) -> Option<&[Member]> {
        match self.unions.as_slice() {
            [union]
                if !self.negated && union.iter().all(|item| matches!(item, Member::Range(..))) =>
            {
                Some(union)
            }
            _ => None,
        }
    }

    /// The class as `(?i)` reads it, with the fold strings of its members
    /// whose fold is longer.
    ///
    /// A class that is not negated tries itself first and then those strings,
    /// as Oniguruma does.
    /// For example, `(?i)[\w]x` takes `ssx` from its start.
    /// `(?i)[a-z]x` matches one character later.
    pub(super) fn insensitive_node(self) -> Node {
        class_folds(self.insensitive(), |_| true)
    }

    /// The class as `(?i)` reads it, as Oniguruma does.
    /// The characters it holds before its negation take in the rest of their
    /// orbits.
    /// Its negation applies after.
    /// For example, `(?i)[^k]` refuses the Kelvin sign.
    ///
    /// A class of ASCII ranges meets only the orbits of ASCII characters.
    /// Only those orbits are tried.
    /// They sort first in the table.
    ///
    /// A union of ranges takes the added members as more ranges.
    /// It stays a class that the analysis and the scanner's automaton read
    /// exactly.
    fn insensitive(self) -> Class {
        let negated = self.negated;
        let positive = Class {
            negated: false,
            unions: self.unions,
        };
        let ascii = positive
            .unions
            .iter()
            .flatten()
            .all(|item| matches!(item, Member::Range(_, hi) if hi.is_ascii()));
        let candidates = if ascii {
            NEXT.get(..NEXT.partition_point(|&(member, _)| member < 0x80))
                .unwrap_or(NEXT)
        } else {
            NEXT
        };
        let mut added: Vec<char> = candidates
            .iter()
            .filter_map(|&(member, _)| char::from_u32(member))
            .filter(|&c| positive.matches(c))
            .flat_map(|c| orbit(c).into_members())
            .filter(|&m| !positive.matches(m))
            .collect();
        added.sort_unstable();
        added.dedup();
        if added.is_empty() {
            return Class {
                negated,
                unions: positive.unions,
            };
        }
        let extra = added.into_iter().map(|m| Member::Range(m, m));
        let parts = if positive.ranges().is_some() {
            vec![positive.unions.into_iter().flatten().chain(extra).collect()]
        } else {
            vec![
                std::iter::once(Member::Nested(positive))
                    .chain(extra)
                    .collect(),
            ]
        };
        Class {
            negated,
            unions: parts,
        }
    }
}

/// A node of the tree.
#[derive(Clone, Debug)]
#[cfg_attr(
    not(all(nishiki_unstable, feature = "_rule-engine")),
    expect(unreachable_pub, reason = "public only through nishiki::unstable")
)]
pub enum Node {
    /// Nothing: an empty pattern or alternative.
    Empty,
    /// A character.
    /// Under `(?i)`, the parser writes a character that folds with others as
    /// the class of its orbit.
    Char(char),
    /// `.`: any character.
    Any {
        /// Whether it matches a newline too, as the `m` flag sets.
        newline: bool,
    },
    /// A class.
    /// Under `(?i)`, the parser has already widened it to the orbits of its
    /// members.
    Class(Box<Class>),
    /// The nodes in order.
    Sequence(Vec<Node>),
    /// The nodes as alternatives, the first that matches taken.
    Alternation(Vec<Node>),
    /// The alternatives a case fold adds under `(?i)`, as [`Node::Alternation`]
    /// reads them.
    /// Each is compiled to a split that reads the next byte first.
    /// The split tries only the alternatives that can start with that byte.
    /// A fold therefore costs next to nothing where the text holds none.
    Folds(Vec<Node>),
    /// A node repeated between `min` and `max` times.
    Repeat {
        /// The node repeated.
        node: Box<Node>,
        /// The fewest times it matches.
        min: u32,
        /// The most times it matches, or none for no limit.
        max: Option<u32>,
        /// How it takes its iterations.
        greediness: Greediness,
    },
    /// A group: capturing when it has an index.
    Group {
        /// The capture group's number, counted from 1, when it captures.
        index: Option<usize>,
        /// The node inside the group.
        node: Box<Node>,
    },
    /// A lookahead or lookbehind.
    Lookaround {
        /// Whether it looks ahead of the position; otherwise behind it.
        ahead: bool,
        /// Whether it matches where `node` does not.
        negative: bool,
        /// The node it tests.
        node: Box<Node>,
    },
    /// `(?>..)`.
    Atomic(Box<Node>),
    /// `\1`, case-insensitive when the flag is set.
    BackReference(usize, Case),
    /// `\k<name>`.
    NamedBackReference(String, Case),
    /// `\g<1>`.
    Subroutine(usize),
    /// `\g<name>`.
    NamedSubroutine(String),
    /// An anchor.
    Anchor(Anchor),
}
