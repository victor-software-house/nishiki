//! The head of a declaration: storage words, a qualified type and pointer
//! marks.
//! The variable, alias and range `for` expressions share this part.
//! Each expression numbers the groups of the head differently.
//! The caller therefore gives a [`Shape`] with the numbers.

use super::super::engine::Lexer;
use super::directives::Reference;
use super::gaps::{Gap, GapForm};
use super::qualified::{Layout, TypeMatch};
use crate::rules::Chars;

/// The storage words a head may begin with.
pub(super) const STORAGE: &[&[u8]] = &[
    b"thread_local",
    b"volatile",
    b"register",
    b"restrict",
    b"static",
    b"extern",
    b"const",
];

/// Where the groups of the pointer marks sit.
#[derive(Clone, Copy)]
pub(super) struct Pointer {
    /// The group of the whole run.
    pub(super) outer: usize,
    /// The group of the gap before the first mark, and of the gap after each
    /// mark.
    pub(super) first: usize,
    pub(super) each: usize,
    /// The form of the gaps.
    pub(super) gap_form: GapForm,
}

/// Where the groups of the storage loop sit.
#[derive(Clone, Copy)]
pub(super) struct Storage {
    /// The group of the loop.
    pub(super) group: usize,
    /// The first group of the gap in the loop.
    /// The gap has no group for the whole of it.
    /// It has three groups for the parts of a comment, or one for the run.
    pub(super) inline: usize,
    pub(super) parts: bool,
    /// The group of the gap after the loop, if there is one.
    pub(super) after: Option<usize>,
}

/// How a head is numbered.
#[derive(Clone, Copy)]
pub(super) struct Shape {
    pub(super) storage: Option<Storage>,
    /// The group around the type, from before its white space.
    pub(super) outer: Option<usize>,
    pub(super) layout: Layout,
    pub(super) pointer: Pointer,
}

/// The last iteration of a storage loop.
#[derive(Clone, Copy)]
pub(super) struct Iteration {
    pub(super) from: usize,
    pub(super) gap: Gap,
}

/// The storage loop of a head, where it ends and its last iteration, with
/// the gap after it.
#[derive(Clone, Copy)]
struct Stored {
    end: usize,
    last: Iteration,
    after: Option<Gap>,
}

/// One way a head matches.
#[derive(Clone, Copy)]
pub(super) struct Head {
    storage: Option<Stored>,
    /// Where the type begins, before its white space.
    from: usize,
    found: TypeMatch,
    /// The pointer marks, if any.
    pub(super) run: Option<Reference>,
    /// Where the head ends: after the pointer marks, or the type.
    pub(super) end: usize,
}

impl Head {
    /// Where the type of the head ends, before its pointer marks.
    pub(super) fn type_end(&self) -> usize {
        self.found.end
    }
}

/// Whether a pointer mark after a declaration's first word continues its
/// type, for [`Lexer::may_start_declaration`].
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Pointers {
    /// `*` or `&` continues the type.
    Continue,
    /// Only a word or what continues a name does.
    Ignore,
}

impl Lexer<'_> {
    /// A cheap test that a type and a name can follow at `at`.
    /// The expressions for declarations, definitions and constructor calls
    /// pass it before their enumeration of ways.
    ///
    /// It holds when the first word is followed by another word, or by what
    /// continues a type, a pointer mark as `pointers` says.
    /// An attribute or a comment may come first, and any of them passes.
    pub(super) fn may_start_declaration(&self, at: usize, pointers: Pointers) -> bool {
        let lead = self.skip_space(at);
        if lead >= self.lim() {
            return false;
        }
        let first = self.s[lead];
        if matches!(first, b'[' | b':' | b'\\') || self.starts(lead, b"/*") {
            return true;
        }
        if !(first.is_ascii_alphabetic() || first == b'_') {
            return false;
        }
        let end = self.word_end(lead);
        if self.starts(lead, b"__") || self.starts(lead, b"alignas") {
            return true;
        }
        let next = self.skip_space(end);
        if next >= self.lim() {
            return false;
        }
        let c = self.s[next];
        c.is_ascii_alphanumeric()
            || matches!(c, b'_' | b'<' | b':' | b'\\' | b'[')
            || self.starts(next, b"/*")
            || (pointers == Pointers::Continue && matches!(c, b'*' | b'&'))
    }

    /// `(?:GAP STORAGE\s+)+` at `p`, every way, the longest first: the first
    /// for which `rest` matches what follows, given where the loop ends and
    /// its last iteration, with what `rest` found.
    pub(super) fn find_storage_run<R>(
        &self,
        p: usize,
        rest: &mut impl FnMut(&Self, usize, Iteration) -> Option<R>,
    ) -> Option<R> {
        let _depth = self.deeper()?;
        for gap in self.gaps(p) {
            let Some(word) = self.word_in(gap.end, STORAGE) else {
                continue;
            };
            let spaces = self.skip_space(word);
            if spaces == word {
                continue;
            }
            let found = self
                .find_storage_run(spaces, rest)
                .or_else(|| rest(self, spaces, Iteration { from: p, gap }));
            if found.is_some() {
                return found;
            }
        }
        None
    }

    /// `(GAP?(?:[&*]GAP)*[&*])?` at `p`, every way.
    /// It finds the first way for which `rest` matches what follows.
    /// `rest` gets where the run ends, the first gap, and the gap after the
    /// last `*` or `&` that the loop took, with where that gap starts.
    ///
    /// The loop takes as much as it can and gives back one step at a time.
    /// The run itself is optional.
    /// Without one, `rest` is given `None` last.
    pub(super) fn find_pointer_run<R>(
        &self,
        p: usize,
        mut rest: impl FnMut(&Self, Option<Reference>) -> Option<R>,
    ) -> Option<R> {
        self.find_reference_chain(p, |lexer, run| rest(lexer, Some(run)))
            .or_else(|| rest(self, None))
    }

    /// Every way the head of `shape` matches at `p`: the storage loop and
    /// its gap, or none, then a qualified type, then pointer marks, or none.
    /// The first for which `rest` matches what follows, with what `rest`
    /// found.
    pub(super) fn find_head<R>(
        &self,
        p: usize,
        shape: &Shape,
        mut rest: impl FnMut(&Self, &Head) -> Option<R>,
    ) -> Option<R> {
        if let Some(storage) = shape.storage {
            let found = self.find_storage_run(p, &mut |lexer, end, last| {
                if storage.after.is_some() {
                    lexer.gaps(end).find_map(|gap| {
                        let stored = Stored {
                            end,
                            last,
                            after: Some(gap),
                        };
                        lexer.find_head_after(shape, gap.end, Some(stored), &mut rest)
                    })
                } else {
                    let stored = Stored {
                        end,
                        last,
                        after: None,
                    };
                    lexer.find_head_after(shape, end, Some(stored), &mut rest)
                }
            });
            if found.is_some() {
                return found;
            }
        }
        self.find_head_after(shape, p, None, &mut rest)
    }

    /// The ways a head matches from `from`, after its storage loop if any.
    fn find_head_after<R>(
        &self,
        shape: &Shape,
        from: usize,
        storage: Option<Stored>,
        rest: &mut impl FnMut(&Self, &Head) -> Option<R>,
    ) -> Option<R> {
        let at = self.skip_space(from);
        self.find_type(at, &shape.layout, |lexer, found| {
            lexer.find_pointer_run(found.end, |lexer, run| {
                let head = Head {
                    storage,
                    from,
                    found: *found,
                    run,
                    end: run.map_or(found.end, |r| r.0),
                };
                rest(lexer, &head)
            })
        })
    }

    /// Records the groups of a head that began at `p`.
    pub(super) fn put_head(&mut self, p: usize, shape: &Shape, head: &Head) {
        if let (Some(storage), Some(stored)) = (shape.storage, &head.storage) {
            self.capture(storage.group, p, stored.end);
            let Iteration { from, gap } = stored.last;
            if storage.parts {
                if let Some(parts) = gap.comment {
                    for (i, (a, b)) in parts.iter().enumerate() {
                        self.capture(storage.inline + i, *a, *b);
                    }
                }
            } else if gap.run {
                self.capture(storage.inline, from, gap.end);
            }
            if let (Some(base), Some(gap)) = (storage.after, &stored.after) {
                self.put_gap(base, shape.layout.gap_form, stored.end, gap);
            }
        }
        if let Some(outer) = shape.outer {
            self.capture(outer, head.from, head.found.end);
        }
        self.put_type(&shape.layout, &head.found);
        if let Some(run) = head.run {
            self.put_pointer(&shape.pointer, head.found.end, &run);
        }
    }

    /// Records the groups of pointer marks that began at `after`.
    pub(super) fn put_pointer(&mut self, pointer: &Pointer, after: usize, run: &Reference) {
        let (end, first, last) = *run;
        self.capture(pointer.outer, after, end);
        if let Some(gap) = first {
            self.put_gap(pointer.first, pointer.gap_form, after, &gap);
        }
        if let Some((from, gap)) = last {
            self.put_gap(pointer.each, pointer.gap_form, from, &gap);
        }
    }
}
