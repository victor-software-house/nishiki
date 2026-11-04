//! The regular expressions of `TextMate` grammars: `Oniguruma`'s syntax,
//! compiled to a program a backtracking machine runs.
//!
//! The syntax covers classes with `&&` and POSIX brackets, lookahead and
//! lookbehind, atomic groups, lazy and possessive quantifiers, back-references,
//! named groups, subroutine calls (`\g<name>`), inline flags and the anchors
//! `\A`, `\G`, `^` and `$`. Unicode properties are the character sets
//! Oniguruma applies, measured into [`unicode`]. Under `(?i)` a character
//! matches every character its simple case folds join it to, and a fold of
//! more than one character matches as a string, such as `ß` and `ss`
//! ([`folds`]), decided when the pattern compiles.

mod analysis;
mod ast;
mod folds;
mod parse;
mod program;
mod relax;
mod unicode;
mod vm;

#[cfg(test)]
mod tests;

use analysis::{Starts, anchors, first_bytes, literal_prefix, required_byte, uses_context};
use memchr::{memchr, memchr2, memchr3, memmem};
use parse::Dialect;
pub(crate) use parse::Error;
use program::{Program, compile};
use vm::Machine;
pub(crate) use vm::{Context, Scratch};

/// How a search finds the next position worth trying.
#[derive(Debug)]
enum Prefilter {
    /// Every position.
    None,
    /// A match starts with this byte.
    Byte(u8),
    /// A match starts with one of two bytes.
    Two(u8, u8),
    /// A match starts with one of three bytes.
    Three(u8, u8, u8),
    /// A match starts with a byte of the set.
    Set(Box<[bool; 256]>),
    /// A match starts with these bytes. Boxed, because a finder is several
    /// times the size of the other variants on `x86_64`.
    Literal(Box<memmem::Finder<'static>>),
}

/// Which programs a line runs.
///
/// Under `(?i)`, a fold of more than one character adds alternatives, such as
/// `ss` for `ß` and `ß` for `ss`. The alternatives that match no text of ASCII
/// alone each need a character beyond ASCII that such a fold involves, so a
/// line without any of those characters runs programs without them, which
/// match the same there.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum LineKind {
    /// The line holds none of those characters.
    #[default]
    Lean,
    /// The line holds one of them.
    Full,
}

impl LineKind {
    /// The kind of `line`: one pass that checks it is ASCII, and only for a
    /// line that is not, one that looks up each byte in a table of the first
    /// bytes of those characters' UTF-8 forms and checks the bytes after
    /// it, without decoding.
    ///
    /// The check runs in blocks of 64 bytes, one test per block, and the
    /// lookup starts at the first block with a byte beyond ASCII: no
    /// character crosses into it from the ASCII blocks before it.
    #[inline]
    pub(crate) fn of(line: &[u8]) -> LineKind {
        let (blocks, tail) = line.as_chunks::<64>();
        let start = match blocks.iter().position(|block| !is_ascii(block)) {
            Some(block) => block * 64,
            None if is_ascii(tail) => return LineKind::Lean,
            None => blocks.len() * 64,
        };
        LineKind::beyond_ascii(line.get(start..).unwrap_or_default())
    }

    /// The kind of `line`, which holds a byte beyond ASCII. Only a lead byte,
    /// one with its top two bits set, can start such a character, so the
    /// others are passed over eight at a time by [`next_lead`], and a lead
    /// byte's second byte is checked against [`folds::WIDE_SECOND`] before
    /// the rest: a two-byte character is decided there, and a three-byte one
    /// whose second byte no such character has, such as `∈`, is passed over.
    /// Each turn of the loop takes the rest of the line after the byte it
    /// looks up, so the rest shrinks by at least a byte every turn.
    #[inline(never)]
    fn beyond_ascii(line: &[u8]) -> LineKind {
        let mut rest = line;
        while let Some((&lead, after)) = next_lead(rest)
            .and_then(|start| rest.get(start..))
            .and_then(<[u8]>::split_first)
        {
            rest = after;
            let Some(group) = folds::WIDE_LEAD[usize::from(lead)]
                .checked_sub(1)
                .map(usize::from)
            else {
                continue;
            };
            let (Some(&second), Some(&mask)) = (rest.first(), folds::WIDE_SECOND.get(group)) else {
                continue;
            };
            if second & 0xc0 != 0x80 || (mask >> (second & 0x3f)) & 1 == 0 {
                continue;
            }
            if lead < 0xe0 {
                return LineKind::Full;
            }
            let found = rest
                .get(..2)
                .and_then(|bytes| <[u8; 2]>::try_from(bytes).ok())
                .map(u16::from_be_bytes)
                .zip(folds::WIDE_TAILS.get(group))
                .is_some_and(|(tail, tails)| tails.binary_search(&tail).is_ok());
            if found {
                return LineKind::Full;
            }
        }
        LineKind::Lean
    }
}

/// Whether `line` is all ASCII, eight bytes at a time and inline, with no
/// early exit, so the loop has no call to set up and folds into vector code.
#[inline]
fn is_ascii(line: &[u8]) -> bool {
    let (words, rest) = line.as_chunks::<8>();
    let high = words
        .iter()
        .fold(0, |high, &word| high | u64::from_ne_bytes(word));
    let rest = rest.iter().fold(0, |high, &byte| high | u64::from(byte));
    (high | rest) & 0x8080_8080_8080_8080 == 0
}

/// Where the first lead byte of `bytes` is: a byte with its top two bits set,
/// which starts a character of two bytes or more. Eight bytes at a time, a
/// byte's bit 6 shifted into its bit 7 marks the bytes with both set.
fn next_lead(bytes: &[u8]) -> Option<usize> {
    let (words, rest) = bytes.as_chunks::<8>();
    for (index, &word) in words.iter().enumerate() {
        let word = u64::from_le_bytes(word);
        let leads = word & (word << 1) & 0x8080_8080_8080_8080;
        if leads != 0 {
            let within = usize::try_from(leads.trailing_zeros() / 8).unwrap_or(0);
            return Some(index * 8 + within);
        }
    }
    rest.iter()
        .position(|&byte| byte >= 0xc0)
        .map(|within| words.len() * 8 + within)
}

/// A compiled regular expression.
#[derive(Debug)]
pub(crate) struct Regex {
    program: Program,
    prefilter: Prefilter,
    anchored: Option<Starts>,
    context_free: bool,
    /// The pattern as a plain expression that matches at least what the
    /// pattern does, for an automaton that finds where a match can start;
    /// `None` for a pattern that depends on the document or can match no text.
    relaxed: Option<String>,
    /// The bytes a match can start with; `None` when it can match nothing.
    first: Option<Box<[bool; 256]>>,
    /// A byte every match has at or after its start.
    required: Option<u8>,
}

impl Regex {
    /// Compiles `pattern` as Oniguruma reads it.
    #[cfg(test)]
    pub(crate) fn new(pattern: &str) -> Result<Regex, Error> {
        Regex::build(pattern, Dialect::Oniguruma, LineKind::Full)
    }

    /// Compiles `pattern` as Oniguruma reads it, for a [`LineKind::Lean`]
    /// line.
    #[cfg(test)]
    pub(crate) fn lean(pattern: &str) -> Result<Regex, Error> {
        Regex::build(pattern, Dialect::Oniguruma, LineKind::Lean)
    }

    /// Compiles `pattern` of a grammar, where `\z` never matches, for lines
    /// of `kind`.
    pub(crate) fn for_grammar(pattern: &str, kind: LineKind) -> Result<Regex, Error> {
        Regex::build(pattern, Dialect::Grammar, kind)
    }

    fn build(pattern: &str, dialect: Dialect, kind: LineKind) -> Result<Regex, Error> {
        let mut parsed = parse::parse(pattern, dialect)?;
        match kind {
            LineKind::Lean => {
                if let Some(root) = ast::lean(&parsed.root) {
                    parsed.root = root;
                }
            }
            LineKind::Full => {}
        }
        let program = compile(&parsed)?;
        let anchored = anchors(&parsed.root);
        let prefix = literal_prefix(&parsed.root);
        let mut set = [false; 256];
        let nullable = first_bytes(&parsed.root, &mut set);
        let members: Vec<u8> = (0..=255_u8).filter(|&b| set[usize::from(b)]).collect();
        let context_free = !uses_context(&parsed.root);
        let widened = relax::widen(&parsed.root);
        let relaxed =
            (context_free && analysis::width(&widened).0 > 0).then(|| relax::source(&widened));
        let prefilter = if prefix.len() >= 2 {
            Prefilter::Literal(Box::new(memmem::Finder::new(&prefix).into_owned()))
        } else if nullable || members.len() == 256 {
            Prefilter::None
        } else {
            match members.as_slice() {
                [a] => Prefilter::Byte(*a),
                [a, b] => Prefilter::Two(*a, *b),
                [a, b, c] => Prefilter::Three(*a, *b, *c),
                _ => Prefilter::Set(Box::new(set)),
            }
        };
        Ok(Regex {
            program,
            prefilter,
            anchored,
            context_free,
            relaxed,
            first: (!nullable).then(|| Box::new(set)),
            required: required_byte(&parsed.root),
        })
    }

    /// Whether a match depends only on the text and where the search starts,
    /// so that a match found from one position is the match found from any
    /// position up to its start.
    pub(crate) fn context_free(&self) -> bool {
        self.context_free
    }

    /// The pattern as a plain expression that matches at least what it does.
    pub(crate) fn relaxed(&self) -> Option<&str> {
        self.relaxed.as_deref()
    }

    /// Whether a match can start with `byte`.
    pub(crate) fn can_start_with(&self, byte: u8) -> bool {
        self.first
            .as_ref()
            .is_none_or(|first| first[usize::from(byte)])
    }

    /// Matches `text` at exactly `at`, leaving the groups in `scratch`. Only
    /// for a pattern that is [`context_free`](Regex::context_free).
    pub(crate) fn match_at(
        &self,
        scratch: &mut Scratch,
        text: &[u8],
        at: usize,
        context: Context,
        steps: &mut u64,
    ) -> bool {
        scratch.reset(&self.program);
        let mut machine = Machine {
            program: &self.program,
            text,
            context,
            steps,
            scratch,
        };
        try_at(&mut machine, at)
    }

    /// Whether `pattern` of a grammar compiles to another program for a
    /// [`LineKind::Lean`] line than for a [`LineKind::Full`] one.
    #[cfg(test)]
    pub(crate) fn lean_differs(pattern: &str) -> bool {
        parse::parse(pattern, Dialect::Grammar)
            .is_ok_and(|parsed| ast::lean(&parsed.root).is_some())
    }

    /// How many capture groups the pattern has.
    #[cfg(test)]
    pub(crate) fn groups(&self) -> usize {
        self.program.groups
    }

    /// Searches `text` for the leftmost match that starts at or after
    /// `from`, leaving the groups in `scratch`. `steps` is the budget the
    /// search spends; when it reaches zero the search gives up.
    ///
    /// An anchored pattern is tried only at the start of the document when the
    /// text holds no newline before its last byte: `^` can also start a match
    /// after a newline inside the text, and a line holds none.
    pub(crate) fn find(
        &self,
        scratch: &mut Scratch,
        text: &[u8],
        from: usize,
        context: Context,
        steps: &mut u64,
    ) -> bool {
        if let Some(byte) = self.required
            && text
                .get(from..)
                .is_none_or(|rest| memchr(byte, rest).is_none())
        {
            return false;
        }
        scratch.reset(&self.program);
        let mut machine = Machine {
            program: &self.program,
            text,
            context,
            steps,
            scratch,
        };
        let len = text.len();
        let one_line =
            memchr(b'\n', text.get(..len.saturating_sub(1)).unwrap_or_default()).is_none();
        if let Some(starts) = self.anchored
            && (!starts.caret || one_line)
        {
            let mut candidates = [None; 3];
            if starts.text || starts.caret {
                candidates[0] = Some(0);
            }
            if starts.search {
                candidates[2] = context.search;
            }
            let mut tried = [usize::MAX; 3];
            for (index, candidate) in candidates.iter().enumerate() {
                let Some(at) = *candidate else { continue };
                if at < from || at > len || tried.contains(&at) {
                    continue;
                }
                tried[index] = at;
                if try_at(&mut machine, at) {
                    return true;
                }
                if *machine.steps == 0 {
                    return false;
                }
            }
            return false;
        }
        let mut pos = from;
        while pos <= len {
            let Some(next) = self.next_start(text, pos) else {
                return false;
            };
            pos = next;
            if try_at(&mut machine, pos) {
                return true;
            }
            if *machine.steps == 0 {
                return false;
            }
            pos += 1;
            while pos < len && text[pos] & 0xc0 == 0x80 {
                pos += 1;
            }
        }
        false
    }

    /// The first position at or after `pos` where a match can start.
    fn next_start(&self, text: &[u8], pos: usize) -> Option<usize> {
        let rest = text.get(pos..)?;
        let skip = match &self.prefilter {
            Prefilter::None => return Some(pos),
            Prefilter::Byte(a) => memchr(*a, rest)?,
            Prefilter::Two(a, b) => memchr2(*a, *b, rest)?,
            Prefilter::Three(a, b, c) => memchr3(*a, *b, *c, rest)?,
            Prefilter::Set(set) => rest.iter().position(|&b| set[usize::from(b)])?,
            Prefilter::Literal(finder) => finder.find(rest)?,
        };
        Some(pos + skip)
    }
}

/// Tries a match that starts at `at`, leaving the groups in the scratch.
///
/// The scratch is clear without a reset: the search reset it before its
/// first attempt, and a failed attempt undoes every change it made on the
/// way out of [`Machine::run`].
fn try_at(machine: &mut Machine<'_>, at: usize) -> bool {
    debug_assert!(machine.scratch.is_clear(), "a failed attempt left state");
    let Some(end) = machine.run(0, at, None) else {
        return false;
    };
    if let [start, finish, ..] = machine.scratch.slots.as_mut_slice() {
        *start = u32::try_from(at).unwrap_or(vm::UNSET);
        *finish = u32::try_from(end).unwrap_or(vm::UNSET);
    }
    true
}
