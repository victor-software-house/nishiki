//! Where a match of a pattern can start: the bytes, and the places in a line.
//!
//! A rule-engine grammar's `Pattern::start` table holds the result for each
//! pattern.
//! The scan then tries a pattern only where a match of it can start.
//!
//! The analysis reads the parser's tree, and over-approximates.
//! A byte or a place it leaves out cannot start a match.
//! It ports `startOf`, `canMatchEmpty` and `takesOneBlank` from
//! `oracle/rule-grammar.ts`, and gives the same answers where that script reads
//! a pattern as the tree does.

use crate::textmate::regex::ast::{Anchor, Class, Member, Node, Polarity};
use crate::textmate::regex::parse::{Dialect, parse};

/// The bytes a back reference can start with.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BackrefStart {
    /// A quote, `'` or `"`, as in the grammars whose back references close
    /// strings.
    Quote,
    /// Any byte, as in Ruby's, which repeat a heredoc's name or a delimiter.
    Any,
}

/// Where a match of a pattern can start.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Start {
    /// The bytes a match can start at: byte `b` is bit `b % 64` of word
    /// `b / 64`.
    pub bytes: [u64; 4],
    /// The places in a line where a match can start.
    pub places: Places,
    /// Whether a match can start where no byte follows.
    pub matches_at_end: bool,
    /// Whether a match can start inside a word at a scan's first position,
    /// though not past it.
    pub in_word_first: bool,
}

/// The places in a line where a match can start.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Places {
    /// Away from the line's start and from words.
    pub away: bool,
    /// At the line's start.
    pub line_start: bool,
    /// Inside a word.
    pub in_word: bool,
}

/// Where a match of `regex` can start.
///
/// A pattern the parser refuses can start anywhere.
#[must_use]
pub fn start(regex: &str, backref: BackrefStart) -> Start {
    let Ok(parsed) = parse(regex, Dialect::Grammar) else {
        return Start {
            bytes: Bytes::ALL.0,
            places: Places {
                away: true,
                line_start: true,
                in_word: true,
            },
            matches_at_end: true,
            in_word_first: false,
        };
    };
    let reader = Reader::new(&parsed.root, regex, backref);
    let found = reader.alternation(&parsed.root);
    let mut bytes = found
        .empty
        .map_or(found.set, |empty| found.set.union(empty));
    if reader.insensitive {
        bytes = folded(bytes);
    }
    let end = Reader {
        reading: Reading::AtEnd,
        ..reader
    }
    .alternation(&parsed.root);
    let strict = Reader {
        reading: Reading::Strict,
        ..reader
    }
    .alternation(&parsed.root);
    let kept = found.kept;
    Start {
        bytes: if found.anywhere {
            Bytes::ALL.0
        } else {
            bytes.0
        },
        places: Places {
            away: !kept.line_start_only,
            line_start: !kept.off_line_start,
            in_word: !kept.line_start_only && !kept.out_of_words,
        },
        matches_at_end: end.anywhere || end.empty.is_some_and(Bytes::is_all),
        in_word_first: kept.out_of_words && !strict.kept.out_of_words && !kept.line_start_only,
    }
}

/// Whether `regex` can match the empty text, so that a `match` with it may
/// not advance.
/// A pattern the parser refuses counts as able to.
#[must_use]
pub fn can_match_empty(regex: &str) -> bool {
    let Ok(parsed) = parse(regex, Dialect::Grammar) else {
        return true;
    };
    Reader::new(&parsed.root, regex, BackrefStart::Any)
        .alternation(&parsed.root)
        .empty
        .is_some()
}

/// Whether some alternative of `regex` takes exactly one blank and then
/// requires a character that is not a blank.
///
/// Such an alternative is one unquantified unit that takes one blank, then a
/// rest whose first byte cannot be a space or a tab.
/// The unit is a single character, such as `\s`, `.` or `[ \t]`.
/// It can also be a group whose alternatives that can start at a blank are
/// each a single character, such as `(?:.|\\.)`.
///
/// A negative lookbehind before the unit is stepped over when no character of
/// its body can match a blank, as in MATLAB's `(?<!\.\.\.).\n`.
///
/// At a blank that follows another blank, such a lookbehind always passes.
/// A body that can match a blank, as in `(?<! )`, could let the pattern fail
/// at the last blank of a run and match at an earlier one.
///
/// Such a pattern can match at the last blank of a run and nowhere before it.
/// A `BlankRuns::ToLast` grammar tries only these patterns there.
#[must_use]
pub fn takes_one_blank(regex: &str, backref: BackrefStart) -> bool {
    let Ok(parsed) = parse(regex, Dialect::Grammar) else {
        return false;
    };
    alternatives(&parsed.root).into_iter().any(|whole| {
        let mut items = whole.as_slice();
        while let Some((
            Node::Lookaround {
                ahead: false,
                negative: true,
                node,
            },
            rest,
        )) = items.split_first()
            && blank_free(node)
        {
            items = rest;
        }
        let Some((first, rest)) = items.split_first() else {
            return false;
        };
        if rest.is_empty() || !takes_one(first, backref) {
            return false;
        }
        let after = items_start(rest, backref);
        let set = after
            .empty
            .map_or(after.set, |empty| after.set.union(empty));
        !after.anywhere && !after.kept.line_start_only && !set.has(b' ') && !set.has(b'\t')
    })
}

/// Whether `unit` takes one blank, and only one character, where it starts at
/// a blank.
fn takes_one(unit: &Node, backref: BackrefStart) -> bool {
    if single(unit) {
        return blank_start(&[unit], backref);
    }
    let Node::Group { node, .. } = unit else {
        return false;
    };
    let mut found = false;
    for alternative in alternatives(node) {
        if !blank_start(&alternative, backref) {
            continue;
        }
        match alternative.as_slice() {
            [only] if single(only) => found = true,
            _ => return false,
        }
    }
    found
}

/// Whether a match of `items` can start at a space or a tab.
fn blank_start(items: &[&Node], backref: BackrefStart) -> bool {
    let found = items_start(items, backref);
    let set = found
        .empty
        .map_or(found.set, |empty| found.set.union(empty));
    found.anywhere || set.has(b' ') || set.has(b'\t')
}

/// The analysis of `items` read as one sequence.
fn items_start(items: &[&Node], backref: BackrefStart) -> Analysis {
    let reader = Reader {
        backref,
        reading: Reading::First,
        refs: items.iter().any(|item| repeats_or_calls(item)),
        insensitive: false,
    };
    reader.sequence(items.iter().copied())
}

/// Whether `node` stands for one character: a literal, a class or `.`.
fn single(node: &Node) -> bool {
    match node {
        Node::Char(_) | Node::Any { .. } | Node::Class(_) | Node::Folds(_) => true,
        Node::Empty
        | Node::Sequence(_)
        | Node::Alternation(_)
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

/// The top-level alternatives of `node`, each a sequence of nodes.
///
/// It looks inside a group or a lookahead that spans the whole of `node`.
/// A sequence that starts with an unquantified group of several alternatives
/// reads as each alternative followed by the rest.
/// So `(;|(^|\s)#\s).*$` reads as `;.*$`, `^#\s.*$` and `\s#\s.*$`.
fn alternatives(node: &Node) -> Vec<Vec<&Node>> {
    match node {
        Node::Alternation(branches) => branches.iter().flat_map(alternatives).collect(),
        Node::Group { node, .. }
        | Node::Lookaround {
            ahead: true,
            negative: false,
            node,
        } => alternatives(node),
        Node::Sequence(items) => sequence_alternatives(items.iter().collect()),
        Node::Empty => vec![Vec::new()],
        Node::Char(_)
        | Node::Any { .. }
        | Node::Class(_)
        | Node::Folds(_)
        | Node::Repeat { .. }
        | Node::Lookaround { .. }
        | Node::Atomic(_)
        | Node::BackReference(..)
        | Node::NamedBackReference(..)
        | Node::Subroutine(_)
        | Node::NamedSubroutine(_)
        | Node::Anchor(_) => vec![vec![node]],
    }
}

/// The alternatives of the sequence `items`, as [`alternatives`] reads them.
fn sequence_alternatives(items: Vec<&Node>) -> Vec<Vec<&Node>> {
    match items.as_slice() {
        [only] => alternatives(only),
        [Node::Group { node, .. }, rest @ ..] => {
            let inner = alternatives(node);
            if inner.len() < 2 {
                return vec![items];
            }
            inner
                .into_iter()
                .flat_map(|mut alternative| {
                    alternative.extend(rest.iter().copied());
                    sequence_alternatives(alternative)
                })
                .collect()
        }
        _ => vec![items],
    }
}

/// Whether no character of `node` can match a space or a tab.
/// A back reference counts as one that can.
fn blank_free(node: &Node) -> bool {
    match node {
        Node::Char(c) => *c != ' ' && *c != '\t',
        Node::Class(class) => !class.matches(' ') && !class.matches('\t'),
        Node::Any { .. } | Node::BackReference(..) | Node::NamedBackReference(..) => false,
        Node::Empty | Node::Anchor(_) | Node::Subroutine(_) | Node::NamedSubroutine(_) => true,
        Node::Sequence(items) | Node::Alternation(items) | Node::Folds(items) => {
            items.iter().all(blank_free)
        }
        Node::Repeat { node, .. }
        | Node::Group { node, .. }
        | Node::Lookaround { node, .. }
        | Node::Atomic(node) => blank_free(node),
    }
}

/// Whether `node` repeats a group's text, calls a group, or holds `\G`.
/// A match of such a pattern cannot move to an earlier start unchanged.
fn repeats_or_calls(node: &Node) -> bool {
    match node {
        Node::BackReference(..)
        | Node::NamedBackReference(..)
        | Node::Subroutine(_)
        | Node::NamedSubroutine(_)
        | Node::Anchor(Anchor::LastMatchEnd) => true,
        Node::Empty | Node::Char(_) | Node::Any { .. } | Node::Class(_) | Node::Anchor(_) => false,
        Node::Sequence(items) | Node::Alternation(items) | Node::Folds(items) => {
            items.iter().any(repeats_or_calls)
        }
        Node::Repeat { node, .. }
        | Node::Group { node, .. }
        | Node::Lookaround { node, .. }
        | Node::Atomic(node) => repeats_or_calls(node),
    }
}

/// `bytes` with each ASCII letter's other case, as `(?i)` reads it.
///
/// Oniguruma also folds the long s (lead byte C5), the sharp s (C3), the st
/// ligatures (EF), the f ligatures (EF) and the Kelvin sign (E2).
fn folded(mut bytes: Bytes) -> Bytes {
    for upper in b'A'..=b'Z' {
        let lower = upper.to_ascii_lowercase();
        if bytes.has(upper) || bytes.has(lower) {
            bytes.add(upper);
            bytes.add(lower);
        }
    }
    if bytes.has(b's') {
        bytes.add(0xc3);
        bytes.add(0xc5);
        bytes.add(0xef);
    }
    if bytes.has(b'f') {
        bytes.add(0xef);
    }
    if bytes.has(b'k') {
        bytes.add(0xe2);
    }
    bytes
}

/// The characters `(?i)` adds to an ASCII letter's class: the long s and the
/// Kelvin sign.
/// [`folded`] adds their lead bytes.
const ORBIT_PARTNERS: [char; 2] = ['\u{17f}', '\u{212a}'];

/// A set of bytes, one bit each.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Bytes([u64; 4]);

impl Bytes {
    /// No byte.
    const NONE: Bytes = Bytes([0; 4]);
    /// Every byte.
    const ALL: Bytes = Bytes([u64::MAX; 4]);
    /// Every byte above ASCII.
    const WIDE: Bytes = Bytes([0, 0, u64::MAX, u64::MAX]);

    /// The set holding `b` alone.
    fn of(b: u8) -> Bytes {
        let mut bytes = Bytes::NONE;
        bytes.add(b);
        bytes
    }

    /// The ASCII bytes of `bits`, one bit per code point below 128.
    fn ascii(bits: u128) -> Bytes {
        let [low, high] =
            [bits, bits >> 64].map(|word| u64::try_from(word & u128::from(u64::MAX)).unwrap_or(0));
        Bytes([low, high, 0, 0])
    }

    /// The ASCII word characters: letters, digits and `_`.
    fn ascii_word() -> Bytes {
        let mut bytes = Bytes::NONE;
        for b in (b'0'..=b'9').chain(b'A'..=b'Z').chain(b'a'..=b'z') {
            bytes.add(b);
        }
        bytes.add(b'_');
        bytes
    }

    /// Adds `b`.
    fn add(&mut self, b: u8) {
        if let Some(word) = self.0.get_mut(usize::from(b >> 6)) {
            *word |= 1 << (b & 63);
        }
    }

    /// Whether the set holds `b`.
    fn has(self, b: u8) -> bool {
        self.0
            .get(usize::from(b >> 6))
            .is_some_and(|word| word >> (b & 63) & 1 == 1)
    }

    /// The bytes in either set.
    fn union(self, other: Bytes) -> Bytes {
        Bytes(std::array::from_fn(|i| self.0[i] | other.0[i]))
    }

    /// The bytes in both sets.
    fn intersect(self, other: Bytes) -> Bytes {
        Bytes(std::array::from_fn(|i| self.0[i] & other.0[i]))
    }

    /// The bytes outside the set.
    fn complement(self) -> Bytes {
        Bytes(self.0.map(|word| !word))
    }

    /// Whether the set is empty.
    fn is_none(self) -> bool {
        self == Bytes::NONE
    }

    /// Whether the set holds every byte.
    fn is_all(self) -> bool {
        self == Bytes::ALL
    }
}

/// What the analysis learns about a piece of a pattern.
#[derive(Clone, Copy, Debug)]
struct Analysis {
    /// The bytes a match that takes a character can start at.
    set: Bytes,
    /// For a piece that can match empty, the bytes that may follow such a
    /// match; none when it cannot.
    empty: Option<Bytes>,
    /// Whether a match can start at any byte, as after `\G`.
    anywhere: bool,
    /// The places the piece keeps every match from starting at.
    kept: Kept,
    /// Whether a match that starts inside a word is impossible or can start
    /// earlier; when none, it follows from `set` and `kept`.
    safe: Option<bool>,
    /// Whether `safe` holds only because a run of a class with every ASCII
    /// word character can move a match's start back to the word's start.
    moves: Option<bool>,
}

impl Analysis {
    /// A piece that takes one character from `set`.
    fn taking(set: Bytes) -> Analysis {
        Analysis {
            set,
            empty: None,
            anywhere: false,
            kept: Kept::NONE,
            safe: None,
            moves: None,
        }
    }

    /// A piece that takes no character, keeps matches from `kept`, and
    /// lets one start at any byte when `anywhere` holds.
    fn zero_width(kept: Kept, anywhere: bool) -> Analysis {
        Analysis {
            set: Bytes::NONE,
            empty: Some(Bytes::ALL),
            anywhere,
            kept,
            safe: None,
            moves: None,
        }
    }

    /// Whether a match of the piece that starts inside a word is impossible or
    /// can start earlier.
    fn safe_in_word(&self) -> bool {
        self.safe
            .unwrap_or(self.kept.out_of_words || self.set.intersect(Bytes::ascii_word()).is_none())
    }

    /// Whether the piece can match empty only where a condition holds, beside
    /// paths that take a character.
    fn conditioned_empty(&self) -> bool {
        self.empty.is_some_and(|empty| {
            !empty.is_all()
                || self.kept.line_start_only
                || self.kept.out_of_words
                || self.kept.off_line_start
        })
    }
}

/// The places a piece of a pattern keeps every match from starting at.
#[derive(Clone, Copy, Debug)]
struct Kept {
    /// Every place but the line's start, as `^` keeps them.
    line_start_only: bool,
    /// The places inside a word, as `\b` keeps them.
    out_of_words: bool,
    /// The line's start, as a lookbehind that needs a byte keeps it.
    off_line_start: bool,
}

impl Kept {
    /// No place.
    const NONE: Kept = Kept {
        line_start_only: false,
        out_of_words: false,
        off_line_start: false,
    };
}

/// Which of the three readings of a pattern's tree a reader makes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Reading {
    /// The bytes and places where a match can start.
    First,
    /// Whether a match can start where no byte follows.
    /// A back reference or a call may match empty there.
    AtEnd,
    /// The places where a match can start at a scan's first position.
    /// A match inside a word cannot start earlier there.
    Strict,
}

/// One reading of a pattern's tree.
#[derive(Clone, Copy, Debug)]
struct Reader {
    /// The bytes a back reference can start with.
    backref: BackrefStart,
    /// Which reading the reader makes.
    reading: Reading,
    /// Whether the pattern repeats a group's text, calls a group, or holds
    /// `\G`.
    refs: bool,
    /// Whether the pattern turns on `(?i)` anywhere.
    insensitive: bool,
}

impl Reader {
    /// The first reading of the pattern `regex`, whose tree is `root`.
    fn new(root: &Node, regex: &str, backref: BackrefStart) -> Reader {
        Reader {
            backref,
            reading: Reading::First,
            refs: repeats_or_calls(root),
            insensitive: regex.contains("(?i"),
        }
    }

    /// The empty path of a back reference or a call.
    fn ref_empty(self) -> Option<Bytes> {
        (self.reading == Reading::AtEnd).then_some(Bytes::ALL)
    }

    /// The alternatives of `node`, each a sequence.
    fn alternation(self, node: &Node) -> Analysis {
        let branches = match node {
            Node::Alternation(branches) | Node::Folds(branches) => branches,
            Node::Sequence(items) => return self.sequence(items),
            Node::Empty
            | Node::Char(_)
            | Node::Any { .. }
            | Node::Class(_)
            | Node::Repeat { .. }
            | Node::Group { .. }
            | Node::Lookaround { .. }
            | Node::Atomic(_)
            | Node::BackReference(..)
            | Node::NamedBackReference(..)
            | Node::Subroutine(_)
            | Node::NamedSubroutine(_)
            | Node::Anchor(_) => return self.sequence(std::iter::once(node)),
        };
        let parts: Vec<Analysis> = branches
            .iter()
            .map(|branch| self.alternation(branch))
            .collect();
        let mut out = Analysis::taking(Bytes::NONE);
        for part in &parts {
            out.set = out.set.union(part.set);
            out.empty = match (out.empty, part.empty) {
                (Some(a), Some(b)) => Some(a.union(b)),
                (a, b) => a.or(b),
            };
            out.anywhere |= part.anywhere;
        }
        out.kept.line_start_only = parts.iter().all(|part| part.kept.line_start_only);
        out.kept.out_of_words = parts.iter().all(|part| part.kept.out_of_words);
        out.kept.off_line_start = parts.iter().all(|part| part.kept.off_line_start);
        out.safe = Some(parts.iter().all(Analysis::safe_in_word));
        out.moves = Some(parts.iter().any(|part| part.moves == Some(true)));
        out
    }

    /// A run of parts.
    ///
    /// Each part counts toward a fact only while the parts before it allow:
    ///
    /// - a lookbehind counts toward `off_line_start` while no part before it
    ///   can consume a byte;
    /// - a `\b` or a lookbehind counts toward `out_of_words` while every part
    ///   before it is safe inside a word;
    /// - a `\G` counts toward `anywhere` while every part before it can match
    ///   empty.
    ///
    /// A zero-width part ends the move of a match's start back to the word's
    /// start.
    /// A lookaround or an anchor that holds inside the word need not hold at
    /// its start.
    fn sequence<'n>(self, items: impl IntoIterator<Item = &'n Node>) -> Analysis {
        let mut out = Analysis::zero_width(Kept::NONE, false);
        let mut safe = true;
        let mut guarded = false;
        let mut conditioned = false;
        let mut moves = false;
        for item in items {
            let part = self.quantified(item);
            let Some(empty) = out.empty else {
                continue;
            };
            out.anywhere |= part.anywhere;
            let first = out.set.is_none();
            out.set = out.set.union(part.set);
            out.kept.line_start_only |= part.kept.line_start_only;
            if part.kept.out_of_words && safe {
                out.kept.out_of_words = true;
                guarded = true;
            }
            let safe_here = part.safe_in_word() && !(part.moves == Some(true) && conditioned);
            if !guarded && !safe_here {
                safe = false;
            }
            if !guarded && part.moves == Some(true) && safe_here {
                moves = true;
            }
            if part.set.is_none() || part.conditioned_empty() {
                conditioned = true;
            }
            if part.kept.off_line_start && first {
                out.kept.off_line_start = true;
            }
            out.empty = part.empty.map(|part| empty.intersect(part));
        }
        out.safe = Some(safe);
        out.moves = Some(moves);
        out
    }

    /// A node and the repeats around it.
    ///
    /// A run of a class that holds every ASCII word character can take the
    /// word characters before it too.
    /// A match inside a word can then start at the word's start, with the
    /// same groups unless the pattern repeats one.
    fn quantified(self, node: &Node) -> Analysis {
        let mut inner = node;
        let mut optional = false;
        let mut unbounded = false;
        while let Node::Repeat { node, min, max, .. } = inner {
            optional |= *min == 0;
            unbounded |= max.is_none();
            inner = node;
        }
        let atom = self.atom(inner);
        let covers = Bytes::ascii_word()
            .intersect(atom.set.complement())
            .is_none();
        let extends =
            self.reading != Reading::Strict && one_char(inner) && unbounded && covers && !self.refs;
        let alone = atom.safe_in_word();
        let safe = Some(extends || alone);
        let moves = Some(if alone {
            atom.moves == Some(true)
        } else {
            extends
        });
        if optional {
            return Analysis {
                empty: Some(Bytes::ALL),
                anywhere: atom.anywhere,
                safe,
                moves,
                ..Analysis::taking(atom.set)
            };
        }
        Analysis {
            safe,
            moves,
            ..atom
        }
    }

    /// One node, read as a single part of a sequence.
    fn atom(self, node: &Node) -> Analysis {
        match node {
            Node::Empty => self.sequence(std::iter::empty()),
            Node::Char(c) => Analysis::taking(self.char_bytes(*c)),
            Node::Any { .. } => Analysis::taking(Bytes::of(b'\n').complement()),
            Node::Class(class) => Analysis::taking(self.class_bytes(class)),
            Node::Sequence(_) | Node::Alternation(_) | Node::Folds(_) => self.alternation(node),
            Node::Repeat { .. } => self.quantified(node),
            Node::Group { node, .. } | Node::Atomic(node) => self.alternation(node),
            Node::Lookaround {
                ahead,
                negative,
                node,
            } => self.lookaround(*ahead, *negative, node),
            Node::BackReference(..) => Analysis {
                empty: self.ref_empty(),
                ..Analysis::taking(match self.backref {
                    BackrefStart::Quote => Bytes::of(b'\'').union(Bytes::of(b'"')),
                    BackrefStart::Any => Bytes::ALL,
                })
            },
            Node::NamedBackReference(..) | Node::Subroutine(_) | Node::NamedSubroutine(_) => {
                Analysis {
                    empty: self.ref_empty(),
                    ..Analysis::taking(Bytes::ALL)
                }
            }
            Node::Anchor(anchor) => match anchor {
                Anchor::LineStart => Analysis::zero_width(
                    Kept {
                        line_start_only: true,
                        ..Kept::NONE
                    },
                    false,
                ),
                Anchor::WordBoundary => Analysis::zero_width(
                    Kept {
                        out_of_words: true,
                        ..Kept::NONE
                    },
                    false,
                ),
                Anchor::LastMatchEnd => Analysis::zero_width(Kept::NONE, true),
                Anchor::LineEnd
                | Anchor::TextStart
                | Anchor::TextEnd
                | Anchor::TextEndOrFinalNewline
                | Anchor::NotWordBoundary => Analysis::zero_width(Kept::NONE, false),
            },
        }
    }

    /// A lookahead or a lookbehind around `node`.
    ///
    /// A lookahead whose body cannot be empty limits the start to the bytes
    /// its body starts with.
    /// `(?<!\w)` and its kin fail after a word character.
    /// A lookbehind whose body cannot be empty needs a byte before the match,
    /// which the line's start lacks.
    /// Inside a word, a lookbehind of one character with no ASCII word
    /// character also fails.
    fn lookaround(self, ahead: bool, negative: bool, node: &Node) -> Analysis {
        match (ahead, negative) {
            (true, false) => {
                let body = self.alternation(node);
                Analysis {
                    empty: Some(if body.empty.is_none() {
                        body.set
                    } else {
                        Bytes::ALL
                    }),
                    ..Analysis::zero_width(Kept::NONE, body.anywhere && body.empty.is_some())
                }
            }
            (true, true) => Analysis::zero_width(Kept::NONE, false),
            (false, true) => Analysis::zero_width(
                Kept {
                    out_of_words: holds_word_class(node),
                    ..Kept::NONE
                },
                false,
            ),
            (false, false) => {
                let body = self.alternation(node);
                let word = !body.set.intersect(Bytes::ascii_word()).is_none();
                Analysis::zero_width(
                    Kept {
                        line_start_only: false,
                        out_of_words: one_char_behind(node) && !word,
                        off_line_start: body.empty.is_none(),
                    },
                    false,
                )
            }
        }
    }

    /// The lead byte of `c`.
    /// Under `(?i)`, a partner of an ASCII letter's orbit adds none here:
    /// [`folded`] adds it.
    fn char_bytes(self, c: char) -> Bytes {
        if self.insensitive && ORBIT_PARTNERS.contains(&c) {
            return Bytes::NONE;
        }
        Bytes::of(lead_byte(c))
    }

    /// The bytes a character of `class` can start with.
    ///
    /// These are its ASCII members and the lead bytes of its wider members.
    /// A negated class can hold any character above ASCII.
    fn class_bytes(self, class: &Class) -> Bytes {
        let ascii = Bytes::ascii(class.ascii());
        if class.negated {
            return ascii.union(Bytes::WIDE);
        }
        let wide = class
            .unions
            .iter()
            .map(|union| {
                union
                    .iter()
                    .map(|member| self.member_lead_bytes(member))
                    .fold(Bytes::NONE, Bytes::union)
            })
            .fold(Bytes::WIDE, Bytes::intersect);
        ascii.union(wide)
    }

    /// The lead bytes above ASCII that a character of `member` can start
    /// with.
    ///
    /// A range gives the run from its first lead byte above ASCII to its
    /// last.
    /// A lead byte never falls as the code point rises, so the run covers
    /// every lead byte of the range.
    /// Under `(?i)`, an orbit partner adds none here: [`folded`] adds it.
    fn member_lead_bytes(self, member: &Member) -> Bytes {
        match member {
            Member::Range(first, last) => {
                if last.is_ascii()
                    || self.insensitive && first == last && ORBIT_PARTNERS.contains(first)
                {
                    return Bytes::NONE;
                }
                let mut bytes = Bytes::NONE;
                for lead in lead_byte((*first).max('\u{80}'))..=lead_byte(*last) {
                    bytes.add(lead);
                }
                bytes
            }
            Member::Property(property, Polarity::Positive) => {
                if property.wide() {
                    Bytes::WIDE
                } else {
                    Bytes::NONE
                }
            }
            Member::Property(_, Polarity::Negated) => Bytes::WIDE,
            Member::Nested(class) => self.class_bytes(class).intersect(Bytes::WIDE),
        }
    }
}

/// The first byte of `c` in UTF-8.
fn lead_byte(c: char) -> u8 {
    let mut buffer = [0; 4];
    c.encode_utf8(&mut buffer);
    buffer[0]
}

/// Whether `node` stands for one character, perhaps in one group: a literal,
/// a class or `.`.
fn one_char(node: &Node) -> bool {
    let inner = if let Node::Group { node: inner, .. } = node {
        inner
    } else {
        node
    };
    single(inner)
}

/// Whether the body of a lookbehind is one character or one escape, not `.`.
///
/// An anchor counts only where it cannot hold between two word characters.
/// `\B` and `\G` can, so a match after them can start inside a word.
fn one_char_behind(node: &Node) -> bool {
    match node {
        Node::Char(_) | Node::Class(_) | Node::Folds(_) | Node::BackReference(..) => true,
        Node::Anchor(anchor) => match anchor {
            Anchor::LineStart
            | Anchor::LineEnd
            | Anchor::TextStart
            | Anchor::TextEnd
            | Anchor::TextEndOrFinalNewline
            | Anchor::WordBoundary => true,
            Anchor::LastMatchEnd | Anchor::NotWordBoundary => false,
        },
        Node::Empty
        | Node::Any { .. }
        | Node::Sequence(_)
        | Node::Alternation(_)
        | Node::Repeat { .. }
        | Node::Group { .. }
        | Node::Lookaround { .. }
        | Node::Atomic(_)
        | Node::NamedBackReference(..)
        | Node::Subroutine(_)
        | Node::NamedSubroutine(_) => false,
    }
}

/// Whether `node` is `\w`, or a class that holds `\w` among its members.
///
/// Under `(?i)`, the parser reads such a class as folds.
/// Their first alternative is the class itself.
fn holds_word_class(node: &Node) -> bool {
    match node {
        Node::Class(class) => class.unions.iter().flatten().any(is_word_member),
        Node::Folds(alternatives) => alternatives.first().is_some_and(holds_word_class),
        Node::Empty
        | Node::Char(_)
        | Node::Any { .. }
        | Node::Sequence(_)
        | Node::Alternation(_)
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

/// Whether `member` is `\w`: a set that holds letters, digits and `_`, and no
/// other ASCII character.
fn is_word_member(member: &Member) -> bool {
    match member {
        Member::Property(property, Polarity::Positive) => {
            let class = Class {
                negated: false,
                unions: vec![vec![Member::Property(*property, Polarity::Positive)]],
            };
            Bytes::ascii(class.ascii()) == Bytes::ascii_word()
        }
        Member::Nested(class) => class.unions.iter().flatten().any(is_word_member),
        Member::Range(..) | Member::Property(_, Polarity::Negated) => false,
    }
}
