//! C on the shared rule engine ([`crate::rules`]): the grammar's rules and
//! patterns as C's hand-written tables, the search that finds the next match,
//! and what the matchers remember about a line.
//!
//! The search is C's own rather than the engine's table scan: at each
//! position it tries the end of the innermost rule and then only the patterns
//! that the kind of position allows (`lists::kind`), and it does not skip a
//! run of blanks at once, because some C patterns start inside one.

use std::cell::Cell;

use super::lists::{EOT, P, Tables, blank_run_live, kind};
use super::scope::S;
use super::state::R;
use crate::language::Halt;
use crate::rules::{self, Act, Captures, Grammar, ScopeName};

/// The C lexer for one line.
pub(super) type Lexer<'a> = rules::Lexer<'a, C>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<C>;

/// The `c` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct C;

/// The capture groups the grammar tokenizes with patterns of their own:
/// none, since a span lexed again names its rule.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Nested {}

/// What the C matchers remember about a line.
#[derive(Debug)]
pub(crate) struct Extra {
    /// Where the last match starts and ends.
    pub(super) start: usize,
    pub(super) end: usize,
    /// The end of the text the caches below were filled from, with its line
    /// feed: lexing a capture again moves it, and a run found up to another
    /// end is not the run up to this one.
    pub(super) text_end: Cell<usize>,
    /// The last run of each class found.
    pub(super) runs: Runs,
    /// The last scoped name found. A later search that steps onto that walk
    /// ends where it did.
    pub(super) scoped: Cell<Run>,
    /// The last word looked up.
    pub(super) lists: Cell<WordLists>,
    /// Whether the line starts with a directive.
    pub(super) head: Cell<Head>,
}

impl Default for Extra {
    fn default() -> Self {
        Extra {
            start: 0,
            end: 0,
            text_end: Cell::new(usize::MAX),
            runs: Runs::default(),
            scoped: Cell::new(Run::NONE),
            lists: Cell::new(WordLists::NONE),
            head: Cell::new(Head::Unknown),
        }
    }
}

/// A stretch of the text a search found: where it starts and ends.
#[derive(Clone, Copy, Debug)]
pub(super) struct Run {
    pub(super) start: usize,
    pub(super) end: usize,
}

impl Run {
    /// No run: it contains no position.
    pub(super) const NONE: Run = Run {
        start: usize::MAX,
        end: 0,
    };

    /// Whether a search from `i` lies in the run, and so ends where it does.
    pub(super) fn holds(self, i: usize) -> bool {
        self.start <= i && i <= self.end
    }
}

/// The last run of each character class found. A search from inside a run
/// ends where the run does, so a run is scanned once.
#[derive(Debug)]
pub(super) struct Runs {
    pub(super) space: Cell<Run>,
    pub(super) word: Cell<Run>,
    pub(super) ascii_name: Cell<Run>,
    pub(super) colons: Cell<Run>,
}

impl Default for Runs {
    fn default() -> Self {
        Runs {
            space: Cell::new(Run::NONE),
            word: Cell::new(Run::NONE),
            ascii_name: Cell::new(Run::NONE),
            colons: Cell::new(Run::NONE),
        }
    }
}

impl Runs {
    fn reset(&self) {
        for run in [&self.space, &self.word, &self.ascii_name, &self.colons] {
            run.set(Run::NONE);
        }
    }
}

/// A word looked up in the word lists: where it starts and ends, and the
/// lists it is in, an empty set where no word starts.
#[derive(Clone, Copy, Debug)]
pub(super) struct WordLists {
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) lists: u32,
}

impl WordLists {
    const NONE: WordLists = WordLists {
        start: usize::MAX,
        end: 0,
        lists: 0,
    };
}

/// Whether the line starts with a directive, found once per line.
#[derive(Clone, Copy, Debug)]
pub(super) enum Head {
    /// Not looked up yet.
    Unknown,
    /// The line is no directive.
    Plain,
    /// The line is a directive: where its `#` is and where its word starts.
    Directive { hash: usize, word: usize },
}

impl Grammar for C {
    type Scope = S;
    type Rule = R;
    type Pat = P;
    type Nested = Nested;
    type Extra = Extra;
    type Kept = ();

    const NAME: &'static str = "C";
    const SOURCE: crate::scope::Scope = S::Source.scope();
    const ROOT: R = R::Top;
    const GROUPS: usize = 1;
    const CAPTURED_EOL: bool = true;
    const NESTED_SPANS: bool = true;

    fn scope(scope: S) -> ScopeName {
        ScopeName::Fixed(scope.scope())
    }

    fn rule_code(rule: R) -> u16 {
        rule.code()
    }

    fn rule_from_code(code: u16) -> R {
        R::from_code(code)
    }

    fn rule_name(rule: R) -> &'static [S] {
        rule.name()
    }

    fn rule_content(rule: R) -> &'static [S] {
        rule.content()
    }

    fn pat_act(pat: P) -> Act<Self> {
        pat.act()
    }

    fn pat_captures(_: P) -> Captures<Self> {
        (&[], &[])
    }

    fn scan(lexer: &mut Lexer<'_>) -> Option<(usize, P, usize)> {
        lexer.find()
    }

    fn matcher(lexer: &mut Lexer<'_>, pat: P, p: usize) -> Option<usize> {
        lexer.match_pat(pat, p).then_some(lexer.x.end)
    }

    fn tokenize_nested(_: &mut Lexer<'_>, kind: Nested, _: usize, _: usize, _: u32) {
        match kind {}
    }
}

impl Lexer<'_> {
    /// Forgets the runs and words found when the text they were found in
    /// has changed since, as it does while a capture is lexed again. The
    /// matchers run only inside `find`, which calls it first.
    fn fresh(&self) {
        let end = self.len_all();
        if self.x.text_end.get() != end {
            self.x.text_end.set(end);
            self.x.runs.reset();
            self.x.scoped.set(Run::NONE);
            self.x.lists.set(WordLists::NONE);
            self.x.head.set(Head::Unknown);
        }
    }

    /// The first match from the cursor on: see `find_from`. A language that
    /// embeds C has the search test its halts while only the root is open.
    pub(super) fn find(&mut self) -> Option<(usize, P, usize)> {
        match self.root_halt() {
            Some(halt) => {
                let after = self.root_after();
                self.find_from::<true>(halt, after)
            }
            None => self.find_from::<false>(|_, _| false, None),
        }
    }

    /// The first match from the cursor on: the earliest position where the
    /// innermost rule's end or one of its patterns matches, the end first,
    /// as where it starts, the pattern and where it ends. `None` when nothing
    /// matches, the work runs out, or, when `HALTING`, a language that embeds
    /// C takes the text back: where `halt` holds before the patterns at a
    /// place, or `after` once they fail there. Without `HALTING` the tests
    /// compile out. A begin of a rule the depth limit keeps
    /// closed is tried only to record the limit, and its text stays plain.
    ///
    /// Each position tries only the patterns its kind of position allows
    /// (`lists::kind`): inside an ASCII word the search tried at its start,
    /// the patterns that read only the word's chain are left out.
    ///
    /// It calls the matchers itself rather than through the engine's
    /// `try_pat`: C's matchers read no capture groups, so a try clears only
    /// the spans the last one left.
    fn find_from<const HALTING: bool>(
        &mut self,
        halt: Halt,
        after: Option<Halt>,
    ) -> Option<(usize, P, usize)> {
        self.fresh();
        let rule = self.top_rule();
        let tables = Tables::get();
        let full = self.at_depth_limit();
        let from = self.pos;
        let top = self.len_all();
        let mut p = from;
        let mut blank_end = 0;
        let mut blank_dead = false;
        let mut word_end = 0;
        let mut word_scan = 0;
        let mut word_nocall = false;
        loop {
            self.spend(1);
            if self.over_budget() {
                return None;
            }
            if HALTING && halt(self.s, p) {
                self.halt_here(p);
                return None;
            }
            let index = if p >= top {
                EOT
            } else {
                usize::from(self.b(p))
            };
            self.clear_spans();
            if self.try_end(rule, p) {
                return Some((self.x.start, P::End, self.x.end));
            }
            let lead = self.b(p);
            let letter = lead.is_ascii_alphabetic() || lead == b'_';
            let mut blank = false;
            let kind = if p < word_end {
                let mut flags = kind::AFTER_WORD | kind::IN_WORD;
                if word_nocall {
                    flags |= kind::NO_CALL;
                }
                if !letter {
                    flags |= kind::NO_OBJECT;
                }
                flags
            } else {
                blank = p < top && self.space_at(p) > 0;
                if blank && p >= blank_end {
                    blank_end = self.skip_space(p);
                    blank_dead = !blank_run_live(self.b(blank_end));
                }
                let no_object = !letter && !matches!(self.prev(p), Some(b']' | b')'));
                let mut flags = 0;
                if p == 0 {
                    flags |= kind::START;
                }
                if self.word_before(p) {
                    flags |= kind::AFTER_WORD;
                }
                if blank && blank_dead {
                    flags |= kind::BLANK_DEAD;
                }
                if no_object {
                    flags |= kind::NO_OBJECT;
                }
                flags
            };
            for &pat in tables.candidates(rule, kind, index) {
                self.clear_spans();
                if !self.match_pat(pat, p) {
                    continue;
                }
                if full && matches!(pat.act(), Act::Begin(_)) {
                    self.record_depth();
                    continue;
                }
                return Some((self.x.start, pat, self.x.end));
            }
            if HALTING && after.is_some_and(|after| after(self.s, p)) {
                self.halt_here(p);
                return None;
            }
            blank_dead |= blank;
            if p >= word_scan && letter && p < top {
                word_scan = self.skip_word(p);
                if self.text(p, word_scan).is_ascii() {
                    word_end = word_scan;
                    word_nocall = !self.call_shape(p);
                }
            }
            if p >= top {
                return None;
            }
            p += self.width_at(p);
        }
    }
}
