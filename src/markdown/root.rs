//! Markdown's root patterns, as the `$base` slots of an Objective-C fence try
//! them.
//!
//! vscode-textmate resolves `$base` to the root grammar of the document.
//! In a Markdown document, the `$base` includes of an Objective-C fence
//! therefore try Markdown's root patterns.
//! At each slot, [`RootPatterns`] gives the place where they first match.
//! Where one matches first, the fence hands the line back to Markdown.

use std::cell::Cell;

use super::block;
use super::captures::Captures;
#[cfg(feature = "lang-html-derivative")]
use super::html;
use super::rules::Pattern;
use super::text::{Cx, LineFacts};
use crate::language::{Halt, RootStart};

/// A root pattern's test of a place: the end of its match there.
type Test = fn(&Cx<'_>, usize, &mut Captures) -> Option<usize>;

/// Markdown's root patterns in the grammar's order, each with its test.
///
/// They are the rules Markdown's lexer tries at the document's root, in the
/// same order.
/// The test `the_slot_tests_follow_the_root_patterns` checks that.
/// Front matter is left out.
/// Its `\A` holds only on the document's first line, at Markdown's root, so
/// never inside a fence.
///
/// Every test reads only the line, through `&Cx`.
/// None needs the arena or the lexer, so the slot's answer is exact.
/// The rule that needs the arena, `Pattern::Html`, sits only in the
/// `HEADING_TEXT` and `PARAGRAPH` lists, not at the root.
///
/// Only the link definition's test reads the line's work, which each test
/// spends.
/// Every other test gives the same answer when tried again at the same place.
pub(super) const TESTS: &[(Pattern, Test)] = &[
    (Pattern::Separator, |cx, p, _| block::separator(cx, p)),
    (Pattern::Heading, |cx, p, captures| {
        block::heading(cx, p, captures)
    }),
    (Pattern::Quote, |cx, p, captures| {
        block::quote(cx, p, captures)
    }),
    (Pattern::Bullet, |cx, p, captures| {
        block::bullet(cx, p, captures)
    }),
    (Pattern::Ordered, |cx, p, captures| {
        block::ordered(cx, p, captures)
    }),
    (Pattern::Fence, |cx, p, captures| {
        block::fence_open(cx, p, captures).map(|open| open.end)
    }),
    (Pattern::RawBlock, |cx, p, _| block::raw_block(cx, p)),
    (Pattern::LinkDefinition, |cx, p, captures| {
        block::link_definition(cx, p, captures)
    }),
    (Pattern::Comment, |cx, p, captures| {
        block::comment_open(cx, p, captures)
    }),
    #[cfg(feature = "lang-html-derivative")]
    (Pattern::HtmlRaw, |cx, p, _| html::raw_open(cx, p)),
    #[cfg(feature = "lang-html-derivative")]
    (Pattern::HtmlBlock, |cx, p, _| html::block_open(cx, p)),
    (Pattern::Table, |cx, p, captures| {
        block::table(cx, p, captures)
    }),
    (Pattern::Paragraph, |cx, p, _| block::paragraph(cx, p)),
];

/// Markdown's root patterns on one line of an Objective-C fence, for the
/// fence's `$base` slots.
///
/// As a halt it never holds.
/// It only answers [`Halt::root`].
pub(super) struct RootPatterns<'a> {
    /// The line's work and the character before its start.
    /// The tests spend the line's work, as Markdown's own matches do.
    facts: &'a LineFacts,
    /// The last search for a link definition.
    definition: Cell<Option<Search>>,
    /// The root pattern that matched at the last place [`RootStart::first`]
    /// gave.
    opening: Cell<Option<Opening>>,
}

/// The root pattern of Markdown that matched first at a `$base` slot.
#[derive(Clone, Copy, Debug)]
pub(super) struct Slot {
    /// The pattern that matched.
    pub(super) pattern: Pattern,
    /// Whether `^` or `\G` held there.
    /// Where neither held, only a link definition can match.
    pub(super) anchored: bool,
}

/// The root pattern that matched at a place, with a link definition's match.
///
/// A link definition's test reads the line's work, so a second try could fail
/// where the first matched.
/// Markdown applies that match as it is.
#[derive(Clone, Copy)]
struct Opening {
    slot: Slot,
    definition: Option<Definition>,
}

/// The match of a link definition: its end and its groups.
#[derive(Clone, Copy, Debug)]
pub(super) struct Definition {
    pub(super) end: usize,
    pub(super) captures: Captures,
}

/// A search for a link definition: the text it read, where it started, and
/// the first definition it found.
#[derive(Clone, Copy)]
struct Search {
    /// The length of the text, without its line feed.
    len: usize,
    /// The end of the text, after its line feed if it has one.
    end: usize,
    from: usize,
    found: Option<(usize, Definition)>,
}

impl<'a> RootPatterns<'a> {
    pub(super) fn new(facts: &'a LineFacts) -> Self {
        RootPatterns {
            facts,
            definition: Cell::new(None),
            opening: Cell::new(None),
        }
    }

    /// The root pattern that matched at the last place [`RootStart::first`]
    /// gave, or `None` where it gave none.
    pub(super) fn slot(&self) -> Option<Slot> {
        self.opening.get().map(|opening| opening.slot)
    }

    /// The match of the link definition that matched at the last place
    /// [`RootStart::first`] gave, if one did.
    /// There is one exactly when [`RootPatterns::slot`] gives
    /// `Pattern::LinkDefinition`.
    pub(super) fn definition(&self) -> Option<Definition> {
        self.opening.get()?.definition
    }

    /// The first start of a link definition in `from..`.
    ///
    /// The last search is kept, and answers again where it can:
    ///
    /// - A search reads only the text and `from`.
    /// - A later search of the same text starts at or after the last one. It
    ///   finds the same start when that start is not behind the new place.
    /// - It finds none where the last search found none.
    /// - A start behind the new place is never given. The text is searched
    ///   again from there.
    fn definition_from(&self, cx: &Cx<'_>, from: usize) -> Option<(usize, Definition)> {
        if let Some(last) = self.definition.get()
            && last.len == cx.line.len()
            && last.end == cx.end
            && last.from <= from
            && last.found.is_none_or(|(start, _)| start >= from)
        {
            return last.found;
        }
        let found = first_definition(cx, from);
        self.definition.set(Some(Search {
            len: cx.line.len(),
            end: cx.end,
            from,
            found,
        }));
        found
    }
}

impl Halt for RootPatterns<'_> {
    fn first(&self, _: &[u8], _: usize, _: usize) -> Option<usize> {
        None
    }

    fn root(&self) -> Option<&dyn RootStart> {
        Some(self)
    }
}

impl RootStart for RootPatterns<'_> {
    /// Every root pattern but a link definition starts with `^` or `\G`.
    /// They can hold only at `from`.
    /// A link definition can start anywhere.
    fn first(&self, s: &[u8], nl: bool, from: usize, anchor: Option<usize>) -> Option<usize> {
        let cx = Cx {
            line: s,
            end: s.len() + usize::from(nl),
            anchor,
            facts: self.facts,
        };
        if cx.at_start(from)
            && let Some(opening) = opening_at(&cx, from)
        {
            self.opening.set(Some(opening));
            return Some(from);
        }
        let found = self.definition_from(&cx, from);
        self.opening.set(found.map(|(_, definition)| Opening {
            slot: Slot {
                pattern: Pattern::LinkDefinition,
                anchored: false,
            },
            definition: Some(definition),
        }));
        found.map(|(start, _)| start)
    }
}

/// The first root pattern that matches at `p`, where `^` or `\G` holds.
fn opening_at(cx: &Cx<'_>, p: usize) -> Option<Opening> {
    let at = cx.line.get(p).copied().unwrap_or(b'\n');
    let first = cx.at(cx.skip_space(p));
    let mut captures = Captures::new();
    TESTS.iter().find_map(|&(pattern, test)| {
        captures.clear();
        if !pattern.opens_block(true, at, first) {
            return None;
        }
        let end = test(cx, p, &mut captures)?;
        Some(Opening {
            slot: Slot {
                pattern,
                anchored: true,
            },
            definition: (pattern == Pattern::LinkDefinition)
                .then_some(Definition { end, captures }),
        })
    })
}

/// The first link definition in `from..`: its start and its match.
///
/// A definition leads with `\s*`.
/// It starts at the first blank before its `[`, or at `from` when the blanks
/// run back past it.
/// A line whose work is spent has none.
fn first_definition(cx: &Cx<'_>, from: usize) -> Option<(usize, Definition)> {
    let mut captures = Captures::new();
    let mut search = from;
    while let Some(offset) = memchr::memchr(b'[', cx.line.get(search..)?) {
        let bracket = search + offset;
        let mut start = bracket;
        while start > from {
            let len = cx.space_before(start);
            if len == 0 {
                break;
            }
            start = start.saturating_sub(len).max(from);
        }
        captures.clear();
        if let Some(end) = block::link_definition(cx, start, &mut captures) {
            return Some((start, Definition { end, captures }));
        }
        if cx.over() {
            return None;
        }
        search = bracket + 1;
    }
    None
}
