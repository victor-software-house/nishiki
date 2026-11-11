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

/// A root pattern's test of a place: whether it matches there.
type Test = fn(&Cx<'_>, usize, &mut Captures) -> bool;

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
pub(super) const TESTS: &[(Pattern, Test)] = &[
    (Pattern::Separator, |cx, p, _| {
        block::separator(cx, p).is_some()
    }),
    (Pattern::Heading, |cx, p, captures| {
        block::heading(cx, p, captures).is_some()
    }),
    (Pattern::Quote, |cx, p, captures| {
        block::quote(cx, p, captures).is_some()
    }),
    (Pattern::Bullet, |cx, p, captures| {
        block::bullet(cx, p, captures).is_some()
    }),
    (Pattern::Ordered, |cx, p, captures| {
        block::ordered(cx, p, captures).is_some()
    }),
    (Pattern::Fence, |cx, p, captures| {
        block::fence_open(cx, p, captures).is_some()
    }),
    (Pattern::RawBlock, |cx, p, _| {
        block::raw_block(cx, p).is_some()
    }),
    (Pattern::LinkDefinition, |cx, p, captures| {
        block::link_definition(cx, p, captures).is_some()
    }),
    (Pattern::Comment, |cx, p, captures| {
        block::comment_open(cx, p, captures).is_some()
    }),
    #[cfg(feature = "lang-html-derivative")]
    (Pattern::HtmlRaw, |cx, p, _| html::raw_open(cx, p).is_some()),
    #[cfg(feature = "lang-html-derivative")]
    (Pattern::HtmlBlock, |cx, p, _| {
        html::block_open(cx, p).is_some()
    }),
    (Pattern::Table, |cx, p, captures| {
        block::table(cx, p, captures).is_some()
    }),
    (Pattern::Paragraph, |cx, p, _| {
        block::paragraph(cx, p).is_some()
    }),
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
    /// Whether `^` or `\G` held at the last place [`RootStart::first`] gave.
    anchored: Cell<bool>,
}

/// A search for a link definition: the text it read, where it started, and
/// the first start it found.
#[derive(Clone, Copy)]
struct Search {
    /// The length of the text, without its line feed.
    len: usize,
    /// The end of the text, after its line feed if it has one.
    end: usize,
    from: usize,
    start: Option<usize>,
}

impl<'a> RootPatterns<'a> {
    pub(super) fn new(facts: &'a LineFacts) -> Self {
        RootPatterns {
            facts,
            definition: Cell::new(None),
            anchored: Cell::new(false),
        }
    }

    /// Whether `^` or `\G` held at the last place [`RootStart::first`] gave.
    /// Markdown then tries every root pattern there.
    /// Where neither held, only a link definition can match there.
    pub(super) fn anchored(&self) -> bool {
        self.anchored.get()
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
    fn definition_from(&self, cx: &Cx<'_>, from: usize) -> Option<usize> {
        if let Some(last) = self.definition.get()
            && last.len == cx.line.len()
            && last.end == cx.end
            && last.from <= from
            && last.start.is_none_or(|start| start >= from)
        {
            return last.start;
        }
        let start = definition_start(cx, from);
        self.definition.set(Some(Search {
            len: cx.line.len(),
            end: cx.end,
            from,
            start,
        }));
        start
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
        let anchored = cx.at_start(from) && opens_at(&cx, from);
        self.anchored.set(anchored);
        if anchored {
            return Some(from);
        }
        self.definition_from(&cx, from)
    }
}

/// Whether one of the root patterns matches at `p`, where `^` or `\G` holds.
fn opens_at(cx: &Cx<'_>, p: usize) -> bool {
    let at = cx.line.get(p).copied().unwrap_or(b'\n');
    let first = cx.at(cx.skip_space(p));
    let mut captures = Captures::new();
    TESTS.iter().any(|&(pattern, test)| {
        captures.clear();
        pattern.opens_block(true, at, first) && test(cx, p, &mut captures)
    })
}

/// The first start of a link definition in `from..`.
///
/// A definition leads with `\s*`.
/// It starts at the first blank before its `[`, or at `from` when the blanks
/// run back past it.
/// A line whose work is spent has none.
fn definition_start(cx: &Cx<'_>, from: usize) -> Option<usize> {
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
        if block::link_definition(cx, start, &mut captures).is_some() {
            return Some(start);
        }
        if cx.over() {
            return None;
        }
        search = bracket + 1;
    }
    None
}
