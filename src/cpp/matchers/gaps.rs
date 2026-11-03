//! The block that the grammar puts between tokens:
//! `((?:\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+)+|\s++|(?<=\W)|(?=\W)|^|\n?
//! $|\A|\Z)`, which is white space and comments, or just a place where a token
//! may start. It can match in several ways, and what follows it decides which
//! one the matcher takes, so [`Lexer::gaps`] gives every way in the order
//! the regular expression tries them.

use super::super::engine::Lexer;
use super::super::few::Few;
use super::super::limits::MAX_COMMENT_RUNS;
use super::chars::{is_space, is_word};
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

/// The two forms the grammar writes the gap block in, which group its parts
/// differently.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum GapForm {
    /// The comment run is one group.
    Plain,
    /// Each part of the last comment is a group.
    Parts,
}

/// The gaps at a position, in the order the expression tries them: the
/// comment runs, longest first, which most positions have none of, then up
/// to three ways without a comment. It keeps the longest
/// [`MAX_COMMENT_RUNS`] runs, so making it allocates nothing.
pub(crate) struct Gaps {
    /// The runs found, each at its index modulo [`MAX_COMMENT_RUNS`], so the
    /// longer ones replace the shortest.
    runs: [Run; MAX_COMMENT_RUNS],
    /// How many runs were found.
    found: usize,
    /// How many of the kept runs were given.
    taken: usize,
    /// The ends of the ways without a comment.
    plain: Few<usize, 3>,
    /// How many of those were given.
    plain_taken: usize,
}

/// A run of comments and white space: where its last comment opens and
/// closes, and where the run ends.
#[derive(Clone, Copy, Default)]
struct Run {
    open: usize,
    close: usize,
    end: usize,
}

impl Iterator for Gaps {
    type Item = Gap;

    fn next(&mut self) -> Option<Gap> {
        if self.taken < self.found.min(MAX_COMMENT_RUNS) {
            let run = self.runs[(self.found - 1 - self.taken) % MAX_COMMENT_RUNS];
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

impl Lexer<'_> {
    /// Every way the block matches at `p`, in the order the expression tries
    /// them.
    ///
    /// The first alternative is iterations of white space, a comment and
    /// white space, taken greedily and given back one at a time. The others
    /// are `\s++`, the zero-width `(?<=\W)`, `(?=\W)` and `^`, and `\n?$`,
    /// which takes the line feed when one follows.
    pub(crate) fn gaps(&self, p: usize) -> Gaps {
        let mut out = Gaps {
            runs: [Run::default(); MAX_COMMENT_RUNS],
            found: 0,
            taken: 0,
            plain: Few::new(),
            plain_taken: 0,
        };
        let mut at = p;
        loop {
            let q = self.skip_space(at);
            let Some(close) = self.comment_close(q) else {
                break;
            };
            let end = self.skip_space(close + 2);
            out.runs[out.found % MAX_COMMENT_RUNS] = Run {
                open: q,
                close,
                end,
            };
            out.found += 1;
            if end == at {
                break;
            }
            at = end;
        }
        let spaces = self.skip_space(p);
        if spaces > p {
            out.plain.push(spaces);
        }
        let before_non_word = self.ch_before(p).is_some_and(|c| !is_word(c));
        let ahead_non_word = self.ch(p).is_some_and(|(c, _)| !is_word(c));
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

    /// The index of the `*/` that closes a comment that starts at `q`.
    fn comment_close(&self, q: usize) -> Option<usize> {
        if !self.starts(q, b"/*") {
            return None;
        }
        memchr::memmem::find(&self.s[q + 2..], b"*/").map(|i| q + 2 + i)
    }

    /// Records a way the block matched, which started at `p`, as the groups
    /// from `base`, in the form `form` of the expression.
    pub(crate) fn put_gap(&mut self, base: usize, form: GapForm, p: usize, gap: &Gap) {
        self.cap(base, p, gap.end);
        match form {
            GapForm::Plain => {
                if gap.run {
                    self.cap(base + 1, p, gap.end);
                }
            }
            GapForm::Parts => {
                if let Some(parts) = gap.comment {
                    for (i, (a, b)) in parts.iter().enumerate() {
                        self.cap(base + 1 + i, *a, *b);
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
