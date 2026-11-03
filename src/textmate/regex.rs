//! The regular expressions of `TextMate` grammars: `Oniguruma`'s syntax,
//! compiled to a program a backtracking machine runs.
//!
//! The syntax covers classes with `&&` and POSIX brackets, lookahead and
//! lookbehind, atomic groups, lazy and possessive quantifiers, back-references,
//! named groups, subroutine calls (`\g<name>`), inline flags and the anchors
//! `\A`, `\G`, `^` and `$`. Unicode properties are approximated with the
//! standard library's character tests.

mod analysis;
mod ast;
mod parse;
mod program;
mod vm;

#[cfg(test)]
mod tests;

use analysis::{Starts, anchors, first_bytes, literal_prefix};
use memchr::{memchr, memchr2, memchr3, memmem};
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
}

impl Regex {
    /// Compiles `pattern`.
    pub(crate) fn new(pattern: &str) -> Result<Regex, Error> {
        let parsed = parse::parse(pattern)?;
        let program = compile(&parsed)?;
        let anchored = anchors(&parsed.root);
        let prefix = literal_prefix(&parsed.root);
        let mut set = [false; 256];
        let nullable = first_bytes(&parsed.root, &mut set);
        let members: Vec<u8> = (0..=255_u8).filter(|&b| set[usize::from(b)]).collect();
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
        })
    }

    /// How many capture groups the pattern has.
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
