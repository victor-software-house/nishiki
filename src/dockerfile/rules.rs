//! The grammar's line-start rules as hand-written matchers.
//!
//! Three rules of the grammar start with `^`: the `FROM ... AS` line, the
//! instruction keywords, and the comment.
//! Only the first byte of a line can start them.
//! A line is therefore tried against them once.
//! The rest of the line holds strings and escapes, which `super::line` scans
//! for.
//! Each matcher answers what its regular expression would, including where
//! backtracking lands.
//!
//! The text the grammar sees ends with a line feed that is not in the line.
//! `\s` matches that line feed, and `.` does not.
//! `RUN` alone on a line is therefore an instruction.

use super::scope::S;
use crate::classes::{space_width, word_before, word_width};
use crate::limits::Steps;
use crate::utf8::char_at;

/// The most pieces a rule's match splits into.
const MAX_SEGS: usize = 5;

/// A run of a match that ends at `to` under the scopes `leaf`, on top of the
/// line's base stack.
#[derive(Clone, Copy, Debug)]
pub(super) struct Seg {
    pub(super) to: usize,
    pub(super) leaf: &'static [S],
}

/// A rule's match: its pieces in order from byte 0.
/// The pieces end at `end`.
#[derive(Clone, Copy, Debug)]
pub(super) struct Hit {
    pub(super) end: usize,
    pub(super) segs: [Option<Seg>; MAX_SEGS],
}

impl Hit {
    fn new(end: usize, segs: &[Seg]) -> Hit {
        let mut out = [None; MAX_SEGS];
        for (slot, seg) in out.iter_mut().zip(segs) {
            *slot = Some(*seg);
        }
        Hit { end, segs: out }
    }
}

/// The instruction keywords, in the grammar's order.
/// None is a prefix of another.
/// The order therefore does not change which one matches.
const INSTRUCTIONS: &[&[u8]] = &[
    b"ADD",
    b"ARG",
    b"CMD",
    b"COPY",
    b"ENTRYPOINT",
    b"ENV",
    b"EXPOSE",
    b"FROM",
    b"HEALTHCHECK",
    b"LABEL",
    b"MAINTAINER",
    b"RUN",
    b"SHELL",
    b"STOPSIGNAL",
    b"USER",
    b"VOLUME",
    b"WORKDIR",
];

/// A line and the bytes its scans have read.
pub(super) struct Line<'a> {
    s: &'a [u8],
    /// The work of the line.
    /// The scans spend it.
    steps: &'a Steps,
}

impl<'a> Line<'a> {
    pub(super) fn new(s: &'a [u8], steps: &'a Steps) -> Self {
        Line { s, steps }
    }

    /// The first rule of the grammar that matches at the start of the line.
    pub(super) fn at_start(&mut self) -> Option<Hit> {
        self.named_stage()
            .or_else(|| self.instruction_line())
            .or_else(|| self.comment())
    }

    /// The width of the white space character at `i`, 0 when there is none;
    /// the line feed after the line counts.
    fn space_at(&self, i: usize) -> usize {
        if i == self.s.len() {
            1
        } else {
            space_width(self.s, i)
        }
    }

    /// The end of the run of white space from `i`.
    fn spaces(&mut self, mut i: usize) -> usize {
        let from = i;
        while let width @ 1.. = self.space_at(i) {
            i += width;
        }
        self.steps.spend(i - from);
        i
    }

    /// Whether the character at `i` is one `\w` matches.
    fn word_at(&self, i: usize) -> bool {
        word_width(self.s, i) > 0
    }

    /// The regex `\b` at `i`.
    fn boundary(&self, i: usize) -> bool {
        word_before(self.s, i) != self.word_at(i)
    }

    /// The end of the keyword `word`, upper case, matched at `i` without
    /// regard to case, as `(?i:...)` does.
    /// The long s reads as `S`, the Kelvin sign as `K`, and the ligatures of
    /// `s` and `t` as `ST`.
    fn keyword(&self, mut i: usize, word: &[u8]) -> Option<usize> {
        let mut k = 0;
        while let Some(&want) = word.get(k) {
            let (c, width) = char_at(self.s, i)?;
            let same = match c {
                'ſ' => want == b'S',
                '\u{212a}' => want == b'K',
                _ => u8::try_from(c).is_ok_and(|b| b.to_ascii_uppercase() == want),
            };
            if same {
                k += 1;
            } else if matches!(c, '\u{fb05}' | '\u{fb06}')
                && want == b'S'
                && word.get(k + 1) == Some(&b'T')
            {
                k += 2;
            } else {
                return None;
            }
            i += width;
        }
        Some(i)
    }

    /// `^\s*\b(?i:(FROM))\b.*?\b(?i:(AS))\b`
    fn named_stage(&mut self) -> Option<Hit> {
        let start = self.spaces(0);
        let from_end = self.keyword(start, b"FROM")?;
        if self.word_at(from_end) {
            return None;
        }
        let len = self.s.len();
        let mut q = from_end;
        loop {
            self.steps.spend(1);
            if self.boundary(q)
                && let Some(end) = self.keyword(q, b"AS")
                && self.boundary(end)
            {
                let special = &[S::Special];
                return Some(Hit::new(
                    end,
                    &[
                        Seg {
                            to: start,
                            leaf: &[],
                        },
                        Seg {
                            to: from_end,
                            leaf: special,
                        },
                        Seg { to: q, leaf: &[] },
                        Seg {
                            to: end,
                            leaf: special,
                        },
                    ],
                ));
            }
            if q >= len {
                return None;
            }
            q += char_at(self.s, q)?.1;
        }
    }

    /// `^\s*(?i:(ONBUILD)\s+)?(?i:(ADD|...|WORKDIR))\s`
    ///
    /// The grammar's `CMD` and `ENTRYPOINT` rule after it is never reached,
    /// because this one matches those first.
    fn instruction_line(&mut self) -> Option<Hit> {
        let start = self.spaces(0);
        if let Some(onbuild_end) = self.keyword(start, b"ONBUILD") {
            let word = self.spaces(onbuild_end);
            if word > onbuild_end
                && let Some((keyword_end, end)) = self.instruction(word)
            {
                return Some(Hit::new(
                    end,
                    &[
                        Seg {
                            to: start,
                            leaf: &[],
                        },
                        Seg {
                            to: onbuild_end,
                            leaf: &[S::Control],
                        },
                        Seg {
                            to: word,
                            leaf: &[],
                        },
                        Seg {
                            to: keyword_end,
                            leaf: &[S::Special],
                        },
                    ],
                ));
            }
        }
        let (keyword_end, end) = self.instruction(start)?;
        Some(Hit::new(
            end,
            &[
                Seg {
                    to: start,
                    leaf: &[],
                },
                Seg {
                    to: keyword_end,
                    leaf: &[S::Special],
                },
            ],
        ))
    }

    /// The instruction keyword at `i` and the white space character after it:
    /// the keyword's end and the match's.
    fn instruction(&self, i: usize) -> Option<(usize, usize)> {
        INSTRUCTIONS.iter().find_map(|word| {
            let end = self.keyword(i, word)?;
            let space = self.space_at(end);
            (space > 0).then_some((end, end + space))
        })
    }

    /// `^(\s*)((#).*$\n?)`
    fn comment(&mut self) -> Option<Hit> {
        let start = self.spaces(0);
        if self.s.get(start) != Some(&b'#') {
            return None;
        }
        let end = self.s.len();
        Some(Hit::new(
            end,
            &[
                Seg {
                    to: start,
                    leaf: &[S::CommentLeading],
                },
                Seg {
                    to: start + 1,
                    leaf: &[S::Comment, S::PunctComment],
                },
                Seg {
                    to: end,
                    leaf: &[S::Comment],
                },
            ],
        ))
    }
}
