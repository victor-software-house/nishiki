//! The regular expressions of `TextMate` grammars: `Oniguruma`'s syntax,
//! compiled to a program a backtracking machine runs.
//!
//! The syntax covers classes with `&&` and POSIX brackets, lookahead and
//! lookbehind, atomic groups, lazy and possessive quantifiers, back-references,
//! named groups, subroutine calls (`\g<name>`), inline flags and the anchors
//! `\A`, `\G`, `^` and `$`. Unicode properties are the character sets
//! Oniguruma applies, measured into [`unicode`].

mod analysis;
mod ast;
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
        Regex::build(pattern, Dialect::Oniguruma)
    }

    /// Compiles `pattern` of a grammar, where `\z` never matches.
    pub(crate) fn for_grammar(pattern: &str) -> Result<Regex, Error> {
        Regex::build(pattern, Dialect::Grammar)
    }

    fn build(pattern: &str, dialect: Dialect) -> Result<Regex, Error> {
        let parsed = parse::parse(pattern, dialect)?;
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
fn try_at(machine: &mut Machine<'_>, at: usize) -> bool {
    machine.scratch.clear_attempt();
    let Some(end) = machine.run(0, at, None) else {
        return false;
    };
    if let [start, finish, ..] = machine.scratch.slots.as_mut_slice() {
        *start = u32::try_from(at).unwrap_or(vm::UNSET);
        *finish = u32::try_from(end).unwrap_or(vm::UNSET);
    }
    true
}
