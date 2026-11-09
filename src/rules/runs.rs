//! The run of `\w` characters a line's matchers found last.
//!
//! A pattern tried at every position of a long word then does not scan the
//! rest of it again from each one.

use std::cell::Cell;

use super::{Chars, Grammar, Lexer};

/// The last run of `\w` characters found, where it starts and ends; a search
/// from inside it ends where it does.
#[derive(Debug)]
pub(crate) struct Runs {
    /// The end of the text the run was found in, with its line feed.
    /// Lexing a capture group's text again moves it.
    /// A run found up to another end is not the run up to this one.
    end: Cell<usize>,
    word: Cell<(usize, usize)>,
}

impl Default for Runs {
    fn default() -> Self {
        Runs {
            end: Cell::new(usize::MAX),
            word: Cell::new((usize::MAX, 0)),
        }
    }
}

/// A grammar's per-line memory that holds [`Runs`].
pub(crate) trait HasRuns {
    /// The runs found on this line.
    fn runs(&self) -> &Runs;
}

/// `\w*` from a position, with the last run remembered.
pub(crate) trait CachedRuns {
    /// The end of the run of `\w` characters from `p`.
    fn word_run(&self, p: usize) -> usize;
}

impl<G: Grammar> CachedRuns for Lexer<'_, G>
where
    G::Extra: HasRuns,
{
    fn word_run(&self, p: usize) -> usize {
        self.spend(1);
        let runs = self.x.runs();
        let end = self.lim() * 2 + usize::from(self.nl);
        if runs.end.get() == end {
            let (from, to) = runs.word.get();
            if from <= p && p < to {
                return to;
            }
        }
        let to = self.word_end(p);
        runs.end.set(end);
        runs.word.set((p, to));
        to
    }
}
