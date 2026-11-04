//! Bounds on the work and memory one line may cost, and the report of which
//! bounds a line reached.

use std::cell::Cell;
use std::time::Duration;

/// The limits a highlighter applies to every line.
///
/// Each field is `None` for no limit. Build from [`Limits::default`],
/// [`Limits::unlimited`] or [`Limits::for_budget`], then override any field:
///
/// ```
/// use nishiki::Limits;
///
/// let limits = Limits {
///     depth: Some(1_000),
///     ..Limits::default()
/// };
/// assert_eq!(limits.depth, Some(1_000));
/// ```
///
/// Limits count bytes and lexer steps, never time, so the same input and
/// limits give the same tokens on every machine.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Limits {
    /// Longest line lexed, in bytes. A longer line is one token under the
    /// current scope stack, and the state passes over it unchanged, as Shiki
    /// does past `tokenizeMaxLineLength`.
    pub line_bytes: Option<usize>,
    /// Lexer steps per line. When a line runs out, the rest of it is one
    /// token under the current scope stack and the next line starts from the
    /// state at that point, as Shiki does at its time limit.
    pub work: Option<Work>,
    /// Most contexts open at once inside a language's root context. The
    /// root context is not counted, and with `Some(0)` nothing opens.
    /// Deeper contexts are not opened, and the constructs inside them add no
    /// scope. Shiki has no such limit. Some of its grammars leave a rule open
    /// for good, such as Ruby's rule for a call whose `)` a here document
    /// takes. A long file can then hold hundreds of contexts open.
    pub depth: Option<usize>,
    /// Most scope stacks, and most states, a highlighter interns. A token
    /// whose stack would be new gets its context's stack instead.
    pub nodes: Option<usize>,
}

/// The step budget of one line: `per_byte` steps for each of its bytes, plus
/// `base`.
///
/// A step is one turn of a lexer's main loop or one byte a lookahead reads,
/// so a linear lexer spends a few steps per byte and a superlinear path runs
/// into the limit.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Work {
    /// Steps allowed per byte of the line.
    pub per_byte: u32,
    /// Steps allowed on top, whatever the line's length.
    pub base: u32,
}

impl Work {
    /// The steps allowed for a line of `len` bytes.
    pub(crate) fn steps(self, len: usize) -> u64 {
        let len = u64::try_from(len).unwrap_or(u64::MAX);
        len.saturating_mul(u64::from(self.per_byte))
            .saturating_add(u64::from(self.base))
    }
}

/// The steps one line spends against its [`Limits::work`]: the turns of a
/// lexer's main loop and the bytes its lookaheads read.
///
/// A lexer counts in a `Steps` of its own, from 0 against the limit its
/// [`Limits`] give its text. The lexers on one line still spend one budget:
///
/// - A lexer that embeds another passes it limits whose work is what the line
///   has left ([`Steps::lend`]).
/// - Every lexer returns what it spent beside its state ([`Finished`]).
/// - The embedding lexer adds that to its own count ([`Steps::take_back`]).
///
/// `Steps` has no `Drop` impl (`CODING_STANDARDS.md` 2.6).
///
/// The count is a `Cell`, so a scan that only reads the line can still spend.
#[derive(Debug)]
#[must_use = "a lexer returns its count through `Steps::finish`"]
pub(crate) struct Steps {
    spent: Cell<u64>,
    /// The line's allowance; `u64::MAX` for no limit.
    limit: u64,
}

impl Steps {
    /// Nothing spent yet on a text of `len` bytes under `limits`.
    #[inline]
    pub(crate) fn new(limits: &Limits, len: usize) -> Self {
        Steps {
            spent: Cell::new(0),
            limit: limits.work.map_or(u64::MAX, |work| work.steps(len)),
        }
    }

    /// Counts `steps` more.
    #[inline]
    pub(crate) fn spend(&self, steps: usize) {
        let steps = u64::try_from(steps).unwrap_or(u64::MAX);
        self.spent.set(self.spent.get().saturating_add(steps));
    }

    /// Counts one turn of a lexer's main loop, and says whether the line
    /// has now spent more than it may.
    #[inline]
    pub(crate) fn step(&self) -> bool {
        let spent = self.spent.get().saturating_add(1);
        self.spent.set(spent);
        spent > self.limit
    }

    /// Whether the line has spent more than it may.
    #[inline]
    pub(crate) fn exhausted(&self) -> bool {
        self.spent.get() > self.limit
    }

    /// The limits a lexer this one embeds runs under: `limits` with the
    /// work this line has left, or no work limit when this one has none.
    ///
    /// [`Work::base`] holds a `u32`, and an embedded part gets at most
    /// `u32::MAX` steps. That is far above the default limits of the longest
    /// line they allow. The function is out of line, because few lines embed
    /// a language.
    #[cold]
    pub(crate) fn lend(&self, limits: &Limits) -> Limits {
        let left = self.limit.saturating_sub(self.spent.get());
        Limits {
            work: limits.work.map(|_| Work {
                per_byte: 0,
                base: u32::try_from(left).unwrap_or(u32::MAX),
            }),
            ..*limits
        }
    }

    /// Counts what an embedded lexer spent, from the [`Finished`] it
    /// returned, and gives back its state.
    #[inline]
    pub(crate) fn take_back(&self, finished: Finished) -> u32 {
        self.add(finished.spent);
        finished.state
    }

    /// Counts `spent` steps that an embedded lexer spent testing the line.
    #[inline]
    pub(crate) fn add(&self, spent: u64) {
        self.spent.set(self.spent.get().saturating_add(spent));
    }

    /// Ends the lexer's count: `state`, the state after the line, with what
    /// the lexer spent.
    #[inline]
    pub(crate) fn finish(self, state: u32) -> Finished {
        Finished {
            state,
            spent: self.spent.get(),
        }
    }

    /// What the lexer spent, for a lexer that only tests the line and
    /// returns no state.
    #[inline]
    pub(crate) fn into_spent(self) -> u64 {
        self.spent.get()
    }
}

/// The state after a line and the steps the lexer spent on it. The lexer
/// that embeds it adds the steps to its own count ([`Steps::take_back`]).
/// The highlighter ignores them.
///
/// A lexer with a count makes one only through [`Steps::finish`]. An exit
/// that skips it does not compile. The two words come back in registers, and
/// the top-level path stores nothing.
#[derive(Clone, Copy, Debug)]
#[must_use]
pub(crate) struct Finished {
    state: u32,
    spent: u64,
}

impl Finished {
    /// A line on which the lexer spent nothing. It is an empty line, or one
    /// whose count nobody takes back. nishiki's `TextMate` engine on the
    /// highlighter's own language is such a case.
    #[inline]
    pub(crate) fn unspent(state: u32) -> Self {
        Finished { state, spent: 0 }
    }

    /// The state after the line.
    #[inline]
    pub(crate) fn state(self) -> u32 {
        self.state
    }
}

/// Resources a caller grants a highlighter, from which
/// [`Limits::for_budget`] derives the limits.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Budget {
    /// Memory for interned scope stacks and states, in bytes.
    pub memory: Option<usize>,
    /// Time one line may take.
    pub line_time: Option<Duration>,
    /// The lexer steps this machine runs per second, which turns
    /// `line_time` into steps. Measure it once on the target machine and
    /// keep the number, so later runs give the same output.
    pub steps_per_second: u64,
}

impl Budget {
    /// An estimate of the steps per second of one core of a 2026 laptop,
    /// used when the caller has not measured its own.
    pub const REFERENCE_STEPS_PER_SECOND: u64 = 200_000_000;
}

impl Default for Budget {
    fn default() -> Self {
        Budget {
            memory: None,
            line_time: None,
            steps_per_second: Self::REFERENCE_STEPS_PER_SECOND,
        }
    }
}

/// Bytes one interned node costs: the node and its hash map entry. An
/// estimate for [`Limits::for_budget`], rounded up.
const NODE_BYTES: usize = 64;

impl Limits {
    /// Whether [`Limits::depth`] keeps one more context from opening where
    /// `open` contexts are open above the language's root. Every lexer asks
    /// here, so a pinned depth means the same in every language.
    pub(crate) fn depth_reached(&self, open: usize) -> bool {
        self.depth.is_some_and(|max| open >= max)
    }

    /// No limit at all.
    #[must_use]
    pub const fn unlimited() -> Self {
        Limits {
            line_bytes: None,
            work: None,
            depth: None,
            nodes: None,
        }
    }

    /// Limits from the resources in `budget`; [`Limits::default`] for any
    /// resource the budget leaves out.
    ///
    /// ```
    /// use std::time::Duration;
    ///
    /// use nishiki::{Budget, Limits};
    ///
    /// let budget = Budget {
    ///     line_time: Some(Duration::from_millis(4)),
    ///     steps_per_second: 100_000_000,
    ///     ..Budget::default()
    /// };
    /// let limits = Limits::for_budget(budget);
    /// assert_eq!(limits.work.map(|work| work.base), Some(400_000));
    /// ```
    #[must_use]
    pub fn for_budget(budget: Budget) -> Self {
        let mut limits = Limits::default();
        if let Some(memory) = budget.memory {
            limits.nodes = Some(memory / NODE_BYTES);
        }
        if let Some(time) = budget.line_time {
            let steps = time
                .as_nanos()
                .saturating_mul(u128::from(budget.steps_per_second))
                / 1_000_000_000;
            let base = u32::try_from(steps).unwrap_or(u32::MAX);
            limits.work = Some(Work { per_byte: 0, base });
        }
        limits
    }
}

impl Default for Limits {
    /// Limits no line of nishiki's oracle fixtures comes near: 1 MiB lines,
    /// 64 steps per byte plus 4,096, 4,096 open contexts and a million nodes.
    fn default() -> Self {
        Limits {
            line_bytes: Some(1 << 20),
            work: Some(Work {
                per_byte: 64,
                base: 4_096,
            }),
            depth: Some(4_096),
            nodes: Some(1 << 20),
        }
    }
}

/// One limit of [`Limits`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Limit {
    /// [`Limits::line_bytes`].
    LineBytes,
    /// [`Limits::work`].
    Work,
    /// [`Limits::depth`].
    Depth,
    /// [`Limits::nodes`].
    Nodes,
}

impl Limit {
    const ALL: [Limit; 4] = [Limit::LineBytes, Limit::Work, Limit::Depth, Limit::Nodes];

    fn bit(self) -> u8 {
        match self {
            Limit::LineBytes => 1,
            Limit::Work => 2,
            Limit::Depth => 4,
            Limit::Nodes => 8,
        }
    }
}

/// The limits one line reached; empty for a line lexed in full.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[must_use]
pub struct LineReport(u8);

impl LineReport {
    /// Whether the line was lexed in full, reaching no limit.
    #[must_use]
    pub fn is_complete(self) -> bool {
        self.0 == 0
    }

    /// Whether the line reached `limit`.
    #[must_use]
    pub fn reached(self, limit: Limit) -> bool {
        self.0 & limit.bit() != 0
    }

    /// Every limit the line reached.
    pub fn limits(self) -> impl Iterator<Item = Limit> {
        Limit::ALL
            .into_iter()
            .filter(move |&limit| self.reached(limit))
    }

    pub(crate) fn record(&mut self, limit: Limit) {
        self.0 |= limit.bit();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_report_names_what_it_records() {
        let mut report = LineReport::default();
        assert!(report.is_complete());
        report.record(Limit::Depth);
        report.record(Limit::Work);
        assert!(!report.is_complete());
        assert_eq!(
            report.limits().collect::<Vec<_>>(),
            [Limit::Work, Limit::Depth]
        );
    }

    #[test]
    fn work_saturates() {
        assert_eq!(
            Work {
                per_byte: u32::MAX,
                base: 1
            }
            .steps(usize::MAX),
            u64::MAX
        );
        assert_eq!(
            Work {
                per_byte: 2,
                base: 3
            }
            .steps(5),
            13
        );
    }

    #[test]
    fn depth_counts_the_contexts_above_the_root() {
        let limits = Limits {
            depth: Some(2),
            ..Limits::unlimited()
        };
        assert!(!limits.depth_reached(1));
        assert!(limits.depth_reached(2));
        assert!(!Limits::unlimited().depth_reached(usize::MAX));
        let none = Limits {
            depth: Some(0),
            ..Limits::unlimited()
        };
        assert!(none.depth_reached(0));
    }

    #[test]
    fn a_budget_sets_only_what_it_names() {
        let limits = Limits::for_budget(Budget {
            memory: Some(6_400),
            ..Budget::default()
        });
        assert_eq!(limits.nodes, Some(100));
        assert_eq!(limits.work, Limits::default().work);
    }
}
