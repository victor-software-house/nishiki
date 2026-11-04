//! Where emphasis's look-ahead stops, kept per line.
//!
//! Bold and italic look ahead from their opening delimiter.
//! The look-ahead steps over units: tags, code spans, escapes, links and
//! single characters.
//! It stops where a closing delimiter could follow, or at the end of the text.
//! Look-aheads from different openers reach the same positions.
//! From a shared position they stop at the same place.
//! The memo keeps, for each position a long look-ahead passed, where it
//! stopped.
//! A later look-ahead that reaches such a position jumps to the stop.
//! Each position is then walked once per kind of emphasis on the line.

/// The steps a look-ahead takes before it uses the memo.
/// Most look-aheads stop sooner and never touch it.
/// Past this many steps, every position is walked at most once.
/// Look-aheads near the line's end take fewer steps than this.
/// That shortfall is a fixed term per line, below this number squared.
/// Each opener can take this many plain steps.
/// The default work budget gives the three bytes of an opener `*a ` 192 steps.
/// 128 fits under that and measured faster on the Markdown fixtures than 64.
/// At 256, a long line of `*a ` runs out of work.
pub(crate) const PLAIN_STEPS: usize = 128;

/// One kind of emphasis: its delimiter and whether it is bold or italic.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Kind {
    BoldStar,
    BoldUnderscore,
    ItalicStar,
    ItalicUnderscore,
}

impl Kind {
    const fn index(self) -> usize {
        match self {
            Kind::BoldStar => 0,
            Kind::BoldUnderscore => 1,
            Kind::ItalicStar => 2,
            Kind::ItalicUnderscore => 3,
        }
    }
}

/// What one step of a look-ahead did.
pub(crate) enum Step {
    /// It moved on to this position.
    Next(usize),
    /// It stopped where it was.
    Stop,
    /// The line ran out of work.
    GiveUp,
}

/// The memo of where look-aheads stop.
///
/// It lives in the box of [`Runs`](super::runs::Runs), which the first long
/// look-ahead allocates.
/// The runs' per-line reset forgets it too, so a line pays one test for both.
#[derive(Debug, Default)]
pub(crate) struct Stops {
    /// The line's address and length, and the end of its text.
    text: Option<(usize, usize, usize)>,
    /// Bumped whenever `text` changes, so old entries need no clearing.
    generation: u32,
    /// For each kind, for each position, the generation and the stop.
    memo: [Vec<(u32, usize)>; 4],
    /// The positions the current look-ahead passed.
    path: Vec<usize>,
}

impl Stops {
    /// Forgets the stops of the last line.
    pub(crate) fn forget(&mut self) {
        self.text = None;
    }

    /// Where a long look-ahead that `step` takes stops, from `start`.
    /// The caller takes the first steps itself, [`PLAIN_STEPS`] of them on
    /// the lexer's path.
    /// `text` is the line's address and length, and the end of its text.
    /// None means the line ran out of work.
    pub(crate) fn walk(
        &mut self,
        text: (usize, usize, usize),
        kind: Kind,
        start: usize,
        mut step: impl FnMut(usize) -> Step,
    ) -> Option<usize> {
        if self.text != Some(text) {
            self.text = Some(text);
            self.generation = self.generation.wrapping_add(1);
            if self.generation == 0 {
                self.generation = 1;
                for memo in &mut self.memo {
                    memo.clear();
                }
            }
        }
        let generation = self.generation;
        let Stops { memo, path, .. } = self;
        let memo = &mut memo[kind.index()];
        path.clear();
        let mut q = start;
        let found = loop {
            if let Some(&(seen, stop)) = memo.get(q)
                && seen == generation
            {
                break Some(stop);
            }
            path.push(q);
            match step(q) {
                Step::Next(next) => q = next,
                Step::Stop => break Some(q),
                Step::GiveUp => break None,
            }
        };
        if let Some(stop) = found {
            for &at in path.iter() {
                if memo.len() <= at {
                    memo.resize(at + 1, (0, 0));
                }
                memo[at] = (generation, stop);
            }
        }
        found
    }
}
