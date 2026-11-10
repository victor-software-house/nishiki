//! The block that the grammar puts between tokens:
//!
//! ```text
//! ((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<=\W)|(?=\W)|^|\n?$|\A|\Z)
//! ```
//!
//! It is white space and comments, or just a place where a token may start.
//! It can match in several ways, and what follows it decides which one the
//! matcher takes.
//! [`Lexer::gaps`] therefore gives every way, in the order the regular
//! expression tries them.

use super::super::engine::Lexer;
use super::super::few::Few;
use super::super::limits::MAX_COMMENT_RUNS;
use super::chars::{is_space, is_word};
use crate::classes::{is_ascii_word, space_width};
use crate::rules::Chars;

/// One way the block matches at a position.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Gap {
    /// Where it ends.
    pub(crate) end: usize,
    /// Whether the first alternative, a run of comments and white space,
    /// matched.
    pub(crate) run: bool,
    /// The last comment of that run: its marker, its text with the end, and
    /// the end.
    pub(crate) comment: Option<[(usize, usize); 3]>,
}

/// The two forms the grammar writes the gap block in.
/// They group its parts differently.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum GapForm {
    /// The comment run is one group.
    Plain,
    /// Each part of the last comment is a group.
    Parts,
}

/// The gaps at a position, in the order the expression tries them.
/// The comment runs come first, longest first, and most positions have none.
/// Up to three ways without a comment follow.
/// It keeps the longest [`MAX_COMMENT_RUNS`] runs, so making it allocates
/// nothing.
///
/// It holds where the first kept run opens, not every run.
/// It finds each run again from there as it gives it.
/// It then stays small enough to be returned without a call to copy it.
pub(crate) struct Gaps<'a> {
    /// The text the runs lie in.
    s: &'a [u8],
    /// Whether a line feed follows the text.
    nl: bool,
    /// Where the first kept run opens.
    first: usize,
    /// How many runs are kept.
    kept: usize,
    /// How many of the kept runs were given.
    taken: usize,
    /// The ends of the ways without a comment.
    plain: Few<usize, 3>,
    /// How many of those were given.
    plain_taken: usize,
}

/// A run of comments and white space: where its last comment opens and
/// closes, and where the run ends.
#[derive(Clone, Copy)]
struct Run {
    open: usize,
    close: usize,
    end: usize,
}

impl Run {
    /// The run whose comment opens at `open` in `s`.
    /// `s` has a `/*` there.
    fn at(s: &[u8], nl: bool, open: usize) -> Option<Run> {
        let close = comment_close(s, open)?;
        Some(Run {
            open,
            close,
            end: blank_end(s, nl, close + 2),
        })
    }
}

/// The index of the `*/` that closes a comment whose `/*` is at `open`.
fn comment_close(s: &[u8], open: usize) -> Option<usize> {
    memchr::memmem::find(s.get(open + 2..)?, b"*/").map(|i| open + 2 + i)
}

/// The end of the `\s*` from `i` in `s`.
/// It takes the line feed after it when `nl` holds, as [`Lexer::skip_space`]
/// does.
///
/// It mirrors [`Chars::space_end`] over the text alone, because `Gaps` holds
/// no lexer.
/// One function shared by both cost Java 2% of its instructions at one
/// codegen unit, even inlined.
/// Each therefore keeps its own loop.
fn blank_end(s: &[u8], nl: bool, mut i: usize) -> usize {
    loop {
        let width = space_width(s, i);
        if width == 0 {
            return if nl && i == s.len() { i + 1 } else { i };
        }
        i += width;
    }
}

impl Iterator for Gaps<'_> {
    type Item = Gap;

    #[expect(
        clippy::inline_always,
        reason = "every matcher with a gap loops over it, and out of line it cost C++ 3.5% of its instructions"
    )]
    #[inline(always)]
    fn next(&mut self) -> Option<Gap> {
        if self.taken < self.kept {
            let mut run = Run::at(self.s, self.nl, self.first)?;
            for _ in 1..self.kept - self.taken {
                run = Run::at(self.s, self.nl, run.end)?;
            }
            self.taken += 1;
            return Some(Gap {
                end: run.end,
                run: true,
                comment: Some([
                    (run.open, run.open + 2),
                    (run.open + 2, run.close + 2),
                    (run.close, run.close + 2),
                ]),
            });
        }
        let end = *self.plain.get(self.plain_taken)?;
        self.plain_taken += 1;
        Some(Gap {
            end,
            run: false,
            comment: None,
        })
    }
}

impl<'a> Lexer<'a> {
    /// Every way the block matches at `p`, in the order the expression tries
    /// them.
    ///
    /// The first alternative is iterations of white space, a comment and
    /// white space, taken greedily and given back one at a time.
    /// The others are `\s++`, the zero-width `(?<=\W)`, `(?=\W)` and `^`, and
    /// `\n?$`.
    /// `\n?$` takes the line feed when one follows.
    pub(crate) fn gaps(&self, p: usize) -> Gaps<'a> {
        let spaces = self.skip_space(p);
        let (first, kept) = self.comment_runs(spaces);
        let mut out = Gaps {
            s: self.s,
            nl: self.nl,
            first,
            kept,
            taken: 0,
            plain: Few::new(),
            plain_taken: 0,
        };
        if spaces > p {
            out.plain.push(spaces);
        }
        let before_non_word = match p.checked_sub(1).and_then(|k| self.s.get(k)) {
            Some(&b) if b < 0x80 => !is_ascii_word(b),
            _ => self.ch_before(p).is_some_and(|c| !is_word(c)),
        };
        let ahead_non_word = match self.s.get(p) {
            Some(&b) if b < 0x80 => !is_ascii_word(b),
            _ => self.ch(p).is_some_and(|(c, _)| !is_word(c)),
        };
        let boundary = before_non_word || ahead_non_word || p == 0;
        if boundary {
            out.plain.push(p);
        }
        if self.nl && p == self.lim() {
            out.plain.push(p + 1);
        }
        if self.at_eol(p) && !boundary {
            out.plain.push(p);
        }
        out
    }

    /// Where the first of the longest [`MAX_COMMENT_RUNS`] comment runs from
    /// `spaces` opens, and how many runs that is.
    fn comment_runs(&self, spaces: usize) -> (usize, usize) {
        if !self.starts(spaces, b"/*") {
            return (spaces, 0);
        }
        let mut opens = [spaces; MAX_COMMENT_RUNS];
        let mut found = 0;
        let mut q = spaces;
        while self.starts(q, b"/*")
            && let Some(close) = comment_close(self.s, q)
        {
            opens[found % MAX_COMMENT_RUNS] = q;
            found += 1;
            q = self.skip_space(self.skip_space(close + 2));
        }
        (opens[found % MAX_COMMENT_RUNS], found.min(MAX_COMMENT_RUNS))
    }

    /// Records a way the block matched, as the groups from `base`, in the form
    /// `form` of the expression.
    /// The match started at `p`.
    pub(crate) fn put_gap(&mut self, base: usize, form: GapForm, p: usize, gap: &Gap) {
        self.capture(base, p, gap.end);
        match form {
            GapForm::Plain => {
                if gap.run {
                    self.capture(base + 1, p, gap.end);
                }
            }
            GapForm::Parts => {
                if let Some(parts) = gap.comment {
                    for (i, (a, b)) in parts.iter().enumerate() {
                        self.capture(base + 1 + i, *a, *b);
                    }
                }
            }
        }
    }

    /// `(?=\w)` at `p`.
    pub(crate) fn ahead_word(&self, p: usize) -> bool {
        self.word_at(p).is_some()
    }

    /// The width of the white space character before `p`: whether `\s` ends
    /// there.
    #[expect(
        dead_code,
        reason = "for the expressions that look behind for white space"
    )]
    pub(crate) fn space_before(&self, p: usize) -> bool {
        self.ch_before(p).is_some_and(is_space)
    }
}
