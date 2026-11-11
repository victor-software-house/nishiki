//! The word tables: the words that a grammar's big alternations spell out,
//! so a matcher can test a whole word against a list.
//!
//! Each module under `words/` writes one language's tables.
//! It names the rules it reads and where each list sits in their patterns.
//! This module holds what they share: it loads a grammar, finds a rule and
//! cuts a list out of a pattern.
//! It then expands the list into words and lays out a table.

mod c;
mod coffee;
mod css;
mod html;
mod less;
mod python;
mod sql;

use std::collections::BTreeSet;
use std::path::Path;

use nishiki::unstable::regex::{Class, Dialect, Greediness, Member, Node, parse};

use crate::File;
use crate::error::Error;
use crate::numbers::fill;
use crate::source::{Source, has_name};

/// Every word file, generated.
pub(crate) fn files(root: &Path) -> Result<Vec<File>, Error> {
    let mut files = vec![c::file(root)?, coffee::file(root)?, css::file(root)?];
    files.extend(html::files(root)?);
    files.extend([less::file(root)?, python::file(root)?, sql::file(root)?]);
    Ok(files)
}

/// Where a word list sits in a pattern.
#[derive(Clone, Copy, Debug)]
enum Cut {
    /// The pattern is the first text, the list, then the second.
    Inside(&'static str, &'static str),
    /// The list follows the first text where the pattern first holds it, and
    /// ends where the pattern last holds the second.
    After(&'static str, &'static str),
    /// The pattern opens with the first text, and the list ends where the
    /// second first follows it.
    Until(&'static str, &'static str),
}

impl Cut {
    /// The list's text in `pattern`.
    fn apply(self, pattern: &str) -> Result<&str, Error> {
        let found = match self {
            Cut::Inside(prefix, suffix) => pattern
                .strip_prefix(prefix)
                .and_then(|rest| rest.strip_suffix(suffix)),
            Cut::After(prefix, suffix) => {
                let start = pattern.find(prefix).map(|at| at + prefix.len());
                let end = pattern.rfind(suffix);
                start
                    .zip(end)
                    .and_then(|(start, end)| pattern.get(start..end))
            }
            Cut::Until(prefix, suffix) => pattern
                .strip_prefix(prefix)
                .and_then(|rest| rest.find(suffix).and_then(|end| rest.get(..end))),
        };
        found.ok_or_else(|| {
            let opening: String = pattern.chars().take(60).collect();
            Error::Shape(format!("{self:?} does not hold a list in {opening}"))
        })
    }
}

/// `body` with its alternative `branch` replaced by `with`, or dropped when
/// `with` is empty.
/// The branch follows a `|` in `body`.
fn replace_branch(body: &str, branch: &str, with: &str) -> Result<String, Error> {
    let found = format!("|{branch}");
    if !body.contains(&found) {
        return Err(Error::Shape(format!("no branch {branch}")));
    }
    let replacement = if with.is_empty() {
        String::new()
    } else {
        format!("|{with}")
    };
    Ok(body.replacen(&found, &replacement, 1))
}

/// One word that a list spells.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Word {
    /// The word.
    text: String,
    /// Whether every letter of it matches both cases, as `(?i)` makes it.
    /// Its letters are then in lower case.
    folded: bool,
}

/// A word as a list's nodes spell it, before the cases of its letters are
/// settled.
#[derive(Clone, Debug, Default)]
struct Spelling {
    /// The word, with each letter that matches both cases in the case its
    /// class lists first.
    text: String,
    /// The byte offset in `text` of each letter that matches both cases, with
    /// that letter's other case.
    either: Vec<(usize, char)>,
    /// Whether a letter of `text` matches one case only.
    cased: bool,
}

impl Spelling {
    /// The spelling of one character.
    fn of(c: char, cased: bool) -> Self {
        Self {
            text: c.to_string(),
            either: Vec::new(),
            cased,
        }
    }

    /// The words of the spelling.
    ///
    /// A word whose every letter matches both cases is one word, folded.
    /// In any other word, each letter that matches both cases is spelled in
    /// both, the case its class lists first first.
    fn settle(self) -> Vec<Word> {
        if !self.cased && !self.either.is_empty() {
            return vec![Word {
                text: self.text.to_ascii_lowercase(),
                folded: true,
            }];
        }
        let mut texts = vec![self.text];
        for &(at, other) in &self.either {
            texts = texts
                .into_iter()
                .flat_map(|text| {
                    let mut swapped = text.clone();
                    swapped.replace_range(at..=at, other.encode_utf8(&mut [0; 4]));
                    [text, swapped]
                })
                .collect();
        }
        texts
            .into_iter()
            .map(|text| Word {
                text,
                folded: false,
            })
            .collect()
    }
}

/// The words that `list` spells, in the order a backtracking matcher tries
/// them.
///
/// `list` is a pattern of characters, classes of characters, groups,
/// alternatives and bounded repetitions.
/// A greedy repetition tries the most iterations first, a lazy one the
/// fewest.
/// Any other node is an error: the list is then no finite set of words.
fn expand(list: &str) -> Result<Vec<Word>, Error> {
    let parsed = parse(list, Dialect::Grammar).map_err(|source| Error::Pattern {
        pattern: list.to_owned(),
        source,
    })?;
    let spellings = spell(&parsed.root)
        .ok_or_else(|| Error::Shape(format!("{list} is not a finite list of words")))?;
    Ok(spellings.into_iter().flat_map(Spelling::settle).collect())
}

/// The words of `list`, each once, sorted.
fn sorted(list: &str) -> Result<Vec<String>, Error> {
    let words: BTreeSet<String> = expand(list)?.into_iter().map(|word| word.text).collect();
    Ok(words.into_iter().collect())
}

/// The spellings of `node`, or `None` when it spells no finite list.
fn spell(node: &Node) -> Option<Vec<Spelling>> {
    match node {
        Node::Empty => Some(vec![Spelling::default()]),
        Node::Char(c) => Some(vec![Spelling::of(*c, c.is_ascii_alphabetic())]),
        Node::Class(class) => letters(class),
        Node::Sequence(items) => items
            .iter()
            .try_fold(vec![Spelling::default()], |spellings, item| {
                Some(product(&spellings, &spell(item)?))
            }),
        Node::Alternation(items) => {
            let lists = items.iter().map(spell).collect::<Option<Vec<_>>>()?;
            Some(lists.concat())
        }
        Node::Folds(items) => spell(items.first()?),
        Node::Group { node, .. } => spell(node),
        Node::Repeat {
            node,
            min,
            max: Some(max),
            greediness,
        } => repeat(&spell(node)?, *min, *max, *greediness),
        Node::Repeat { max: None, .. }
        | Node::Any { .. }
        | Node::Lookaround { .. }
        | Node::Atomic(_)
        | Node::BackReference(..)
        | Node::NamedBackReference(..)
        | Node::Subroutine(_)
        | Node::NamedSubroutine(_)
        | Node::Anchor(_) => None,
    }
}

/// The spellings of the characters of a class, in the order it lists them.
///
/// When every ASCII letter of the class comes with its other case, as under
/// `(?i)`, each pair is one letter that matches both cases.
/// A character beyond ASCII that changes case into one of the class's ASCII
/// letters, such as U+212A KELVIN SIGN, then adds nothing.
/// Any other class spells each of its characters.
fn letters(class: &Class) -> Option<Vec<Spelling>> {
    if class.negated {
        return None;
    }
    let [union] = class.unions.as_slice() else {
        return None;
    };
    let ascii = class.ascii();
    let has = |c: char| holds(ascii, c);
    let paired = (0..128u8)
        .map(char::from)
        .filter(|&c| has(c) && c.is_ascii_alphabetic())
        .all(|c| has(other_case(c)));
    let mut chars = Vec::new();
    for member in union {
        let Member::Range(lo, hi) = member else {
            return None;
        };
        for c in *lo..=*hi {
            if c.is_ascii() {
                chars.push(c);
            } else if !(paired && changes_into(c, ascii)) {
                return None;
            }
        }
    }
    let mut spellings: Vec<Spelling> = Vec::new();
    for c in chars {
        if !paired || !c.is_ascii_alphabetic() {
            spellings.push(Spelling::of(c, c.is_ascii_alphabetic()));
        } else if !spellings.iter().any(|s| s.text.starts_with(other_case(c))) {
            spellings.push(Spelling {
                text: c.to_string(),
                either: vec![(0, other_case(c))],
                cased: false,
            });
        }
    }
    Some(spellings)
}

/// The other case of the ASCII letter `c`.
fn other_case(c: char) -> char {
    if c.is_ascii_uppercase() {
        c.to_ascii_lowercase()
    } else {
        c.to_ascii_uppercase()
    }
}

/// Whether `ascii`, which holds one bit per ASCII code point, holds `c`.
fn holds(ascii: u128, c: char) -> bool {
    c.is_ascii() && ascii & (1u128 << u32::from(c)) != 0
}

/// Whether `c` changes case into one character that `ascii` holds.
fn changes_into(c: char, ascii: u128) -> bool {
    let one = |changed: String| {
        let mut chars = changed.chars();
        matches!((chars.next(), chars.next()), (Some(single), None) if holds(ascii, single))
    };
    one(c.to_lowercase().collect()) || one(c.to_uppercase().collect())
}

/// Each spelling of `left` followed by each of `right`, in that order.
fn product(left: &[Spelling], right: &[Spelling]) -> Vec<Spelling> {
    left.iter()
        .flat_map(|a| {
            right.iter().map(move |b| Spelling {
                text: [a.text.as_str(), &b.text].concat(),
                either: a
                    .either
                    .iter()
                    .copied()
                    .chain(
                        b.either
                            .iter()
                            .map(|&(at, other)| (at + a.text.len(), other)),
                    )
                    .collect(),
                cased: a.cased || b.cased,
            })
        })
        .collect()
}

/// The spellings of `spellings` repeated `min` to `max` times, in the order
/// `greediness` tries the counts.
fn repeat(
    spellings: &[Spelling],
    min: u32,
    max: u32,
    greediness: Greediness,
) -> Option<Vec<Spelling>> {
    let counts: Vec<u32> = match greediness {
        Greediness::Greedy => (min..=max).rev().collect(),
        Greediness::Lazy => (min..=max).collect(),
        Greediness::Possessive => return None,
    };
    let mut out = Vec::new();
    for count in counts {
        let mut spelled = vec![Spelling::default()];
        for _ in 0..count {
            spelled = product(&spelled, spellings);
        }
        out.extend(spelled);
    }
    Some(out)
}

/// Appends a table to `text`, after a blank line: its doc lines, then
/// `declaration`, then `items`, filled into lines.
fn table(
    text: &mut String,
    doc: &[&str],
    declaration: &str,
    items: impl IntoIterator<Item = String>,
) {
    text.push('\n');
    for line in doc {
        text.push_str("/// ");
        text.push_str(line);
        text.push('\n');
    }
    text.push_str(declaration);
    text.push_str(" = &[\n");
    fill(text, items);
    text.push_str("];\n");
}

/// Each word of `words` as a string literal.
fn quoted(words: &[String]) -> impl Iterator<Item = String> {
    words.iter().map(|word| format!("{word:?}"))
}

/// Each word of `words` as a byte string literal.
fn bytes(words: &[String]) -> impl Iterator<Item = String> {
    words.iter().map(|word| format!("b{word:?}"))
}
