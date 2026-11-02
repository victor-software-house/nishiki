//! The per-line lexer: the `TextMate` engine's loop over the rules of the
//! Markdown grammar, with the regular expressions replaced by the matchers in
//! [`block`] and [`inline`].
//!
//! A line starts by checking the `while` rule of each open container, which
//! says whether the container continues; the first one that fails closes
//! itself and every container inside it. Then the rules of the innermost
//! container, and its closing rule, are tried at each position from the
//! cursor on. The match that starts first wins, and among matches that start
//! together the rule listed first. A match opens a container, closes one, or
//! only scopes text.

use std::cell::Cell;

use super::caps::Caps;
use super::rules::{self, Capture, End, List, P};
use super::scope::S;
use super::state::{Fence, Kind, Lang};
use super::text::Cx;
use super::{block, inline};
use crate::arena::{Arena, ROOT};
use crate::limits::{Limit, Limits, LineReport};
use crate::{Stack, Token, rust};

/// What a frame on the context stack is.
#[derive(Clone, Copy, Debug)]
enum Rule {
    /// An open container, which the state keeps between lines.
    Open(Kind),
    /// The text of a capture group, which hands it to a list of rules and is
    /// gone when the group's text ends.
    Capture(List),
}

#[derive(Clone, Copy, Debug)]
struct Frame {
    rule: Rule,
    /// The interned scope stack of the frame's own text, its opening and
    /// closing included.
    name: u32,
    /// The scope stack of the text between opening and closing.
    content: u32,
    /// The `\G` position to restore when the frame closes.
    saved_anchor: Option<usize>,
    /// What the frame carries: a fence's [`Fence`], or the state of the
    /// embedded Rust lexer.
    payload: u32,
    /// The interned state with this frame on top, for a container that stays
    /// open between lines; none for the text of a capture group.
    state: Option<u32>,
}

/// A rule that matched.
struct Hit {
    /// Which rule, or none for the closing rule of the innermost container.
    rule: Option<P>,
    start: usize,
    end: usize,
    caps: Caps,
}

/// Lexes one line.
pub(crate) struct Lexer<'a> {
    arena: &'a mut Arena,
    limits: &'a Limits,
    report: &'a mut LineReport,
    line: &'a str,
    out: &'a mut Vec<Token>,
    /// Where this line's tokens start in `out`.
    first_token: usize,
    frames: Vec<Frame>,
    /// The cursor: the end of the last match.
    pos: usize,
    /// Where `\G` matches.
    anchor: Option<usize>,
    /// The end of the text the rules see: one past the line while lexing the
    /// line, which stands for its newline, and the end of a capture group while
    /// lexing that.
    limit: usize,
    /// The end of the last token emitted.
    last: usize,
    /// Steps spent on this line, counted in the matchers' scans too.
    steps: Cell<u64>,
    /// Steps this line may spend; `u64::MAX` for no limit.
    budget: u64,
    /// The fence the last match of the fence rule opened.
    opened: Option<block::FenceOpen>,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(
        arena: &'a mut Arena,
        limits: &'a Limits,
        line: &'a str,
        out: &'a mut Vec<Token>,
        state: u32,
        report: &'a mut LineReport,
    ) -> Self {
        let first_token = out.len();
        let mut frames = Vec::new();
        let mut at = state;
        while at != ROOT {
            let frame = arena.frame(at);
            let kind = Kind::from_code(frame.kind);
            let name = if matches!(kind, Kind::Body(_)) {
                arena.frame(frame.parent).stack
            } else {
                frame.stack
            };
            frames.push(Frame {
                rule: Rule::Open(kind),
                name,
                content: frame.stack,
                saved_anchor: None,
                payload: frame.payload,
                state: Some(at),
            });
            at = frame.parent;
        }
        frames.reverse();
        Lexer {
            arena,
            limits,
            report,
            line,
            out,
            first_token,
            frames,
            pos: 0,
            anchor: None,
            limit: line.len() + 1,
            last: 0,
            steps: Cell::new(0),
            budget: limits.work.map_or(u64::MAX, |work| work.steps(line.len())),
            opened: None,
        }
    }

    /// The state after the line.
    pub(crate) fn into_state(self) -> u32 {
        self.frames
            .iter()
            .rev()
            .find_map(|frame| frame.state)
            .unwrap_or(ROOT)
    }

    pub(crate) fn run(&mut self) {
        self.check_whiles();
        while self.step() {}
        let node = self.content();
        self.emit(self.line.len(), node);
    }

    // The scope stacks.

    /// The scope stack of the text between the innermost frame's opening and
    /// closing.
    fn content(&self) -> u32 {
        self.frames.last().map_or(ROOT, |frame| frame.content)
    }

    /// The stack `parent` with `scope` on top; `parent` when the node limit
    /// keeps a new stack from being made.
    fn child(&mut self, parent: u32, scope: S) -> u32 {
        self.arena.child(parent, scope.scope()).unwrap_or_else(|| {
            self.report.record(Limit::Nodes);
            parent
        })
    }

    fn path(&mut self, parent: u32, scopes: &[S]) -> u32 {
        scopes
            .iter()
            .fold(parent, |stack, &scope| self.child(stack, scope))
    }

    fn emit(&mut self, upto: usize, node: u32) {
        let upto = upto.min(self.line.len());
        if upto <= self.last {
            return;
        }
        let stack = Stack(node);
        match self
            .out
            .get_mut(self.first_token..)
            .and_then(<[Token]>::last_mut)
        {
            Some(token) if token.stack == stack => token.end = upto,
            _ => self.out.push(Token { end: upto, stack }),
        }
        self.last = upto;
    }

    fn cx(&self) -> Cx<'_> {
        Cx {
            line: self.line.as_bytes(),
            end: self.limit,
            anchor: self.anchor,
            steps: &self.steps,
            budget: self.budget,
        }
    }

    // Containers that continue.

    /// Runs the `while` rule of each open container, outermost first. The
    /// first that fails closes its container and everything inside it.
    fn check_whiles(&mut self) {
        let mut index = 0;
        while index < self.frames.len() {
            let Rule::Open(kind) = self.frames[index].rule else {
                break;
            };
            let mut caps = Caps::new();
            let cx = self.cx();
            let pos = self.pos;
            let end = match kind {
                Kind::Quote => block::quote_while(&cx, pos, &mut caps),
                Kind::ListBullet | Kind::ListOrdered => block::list_while(&cx, pos),
                Kind::Body(_) => {
                    (cx.at_start(pos) && !block::is_fence_line(&cx, pos)).then_some(pos)
                }
                Kind::RawBlock => block::raw_block(&cx, pos),
                Kind::Table => block::table_while(&cx, pos).then_some(pos),
                Kind::Paragraph => block::paragraph_while(&cx, pos),
                Kind::Base
                | Kind::Fence(_)
                | Kind::FencePlain
                | Kind::Comment
                | Kind::BoldStar
                | Kind::BoldUnder
                | Kind::ItalicStar
                | Kind::ItalicUnder => {
                    index += 1;
                    continue;
                }
            };
            let Some(end) = end else {
                self.frames.truncate(index);
                break;
            };
            let node = self.frames[index].content;
            self.emit(pos, node);
            // Only a quote's rule has capture groups.
            self.captures(node, end, &caps, |group| rules::capture(P::Quote, group));
            self.emit(end, node);
            self.pos = self.pos.max(end);
            self.anchor = Some(end);
            index += 1;
        }
    }

    // The loop.

    /// Takes the next match from the cursor. False when nothing matches, or
    /// the line has spent its work.
    fn step(&mut self) -> bool {
        self.steps.set(self.steps.get().saturating_add(1));
        if self.steps.get() > self.budget {
            self.report.record(Limit::Work);
            return false;
        }
        let from = (self.pos, self.frames.len());
        if let Some(Frame {
            rule: Rule::Open(Kind::Body(Lang::Rust)),
            ..
        }) = self.frames.last()
        {
            self.embedded();
        }
        let Some(hit) = self.find() else {
            return false;
        };
        self.apply(&hit);
        debug_assert!(
            (self.pos, self.frames.len()) != from,
            "the Markdown lexer made no progress at byte {}",
            from.0
        );
        true
    }

    /// The first match at or after the cursor: the closing rule first at each
    /// position, then the container's rules in order.
    fn find(&mut self) -> Option<Hit> {
        let (end, list) = match self.frames.last() {
            None => (None, rules::patterns(Kind::Base)),
            Some(frame) => match frame.rule {
                Rule::Open(kind) => (
                    rules::end(kind).map(|end| (end, kind)),
                    rules::patterns(kind),
                ),
                Rule::Capture(list) => (None, rules::list_patterns(list)),
            },
        };
        for start in self.pos..=self.limit {
            if let Some((end, kind)) = end
                && let Some(hit) = self.try_end(end, kind, start)
            {
                return Some(hit);
            }
            for &rule in list {
                if let Some(hit) = self.try_rule(rule, start) {
                    return Some(hit);
                }
            }
        }
        None
    }

    fn try_end(&mut self, end: End, kind: Kind, start: usize) -> Option<Hit> {
        let cx = self.cx();
        let mut caps = Caps::new();
        let end_pos = match end {
            End::Fence => {
                let payload = self.frames.last().map_or(0, |frame| frame.payload);
                block::fence_close(&cx, start, Fence::from_payload(payload), &mut caps)
            }
            End::Comment => block::comment_close(&cx, start, &mut caps),
            End::Bold => {
                let mark = if kind == Kind::BoldStar { b'*' } else { b'_' };
                inline::bold_close(&cx, start, mark, &mut caps)
            }
            End::Italic => {
                let mark = if kind == Kind::ItalicStar { b'*' } else { b'_' };
                inline::italic_close(&cx, start, mark, &mut caps)
            }
        }?;
        Some(Hit {
            rule: None,
            start,
            end: end_pos,
            caps,
        })
    }

    fn try_rule(&mut self, rule: P, start: usize) -> Option<Hit> {
        let cx = self.cx();
        let mut caps = Caps::new();
        let mut rule = rule;
        let end = match rule {
            P::Separator => block::separator(&cx, start),
            P::Heading => block::heading(&cx, start, &mut caps),
            P::Quote => block::quote(&cx, start, &mut caps),
            P::Bullet => block::bullet(&cx, start, &mut caps),
            P::Ordered => block::ordered(&cx, start, &mut caps),
            P::Fence => {
                let open = block::fence_open(&cx, start, &mut caps)?;
                let end = open.end;
                rule = if open.lang.is_none() {
                    P::FencePlain
                } else {
                    P::FenceLanguage
                };
                self.opened = Some(open);
                Some(end)
            }
            P::RawBlock => block::raw_block(&cx, start),
            P::LinkDef => {
                // `\s*` leads the rule, so only the start of a run of blanks
                // can start a match.
                if start > self.pos && cx.space_before(start) > 0 {
                    return None;
                }
                block::link_def(&cx, start, &mut caps)
            }
            P::Comment => block::comment_open(&cx, start, &mut caps),
            P::Table => block::table(&cx, start, &mut caps),
            P::Paragraph => block::paragraph(&cx, start),
            P::EmbedBegin => embed_begin(&cx, start),
            P::TablePipe => (cx.get(start) == Some(b'|')).then_some(start + 1),
            P::TableSeparator => block::table_separator(&cx, start, &mut caps),
            P::TableCell => block::table_cell(&cx, start, &mut caps),
            P::Setext1 => block::setext(&cx, start, b'='),
            P::Setext2 => block::setext(&cx, start, b'-'),
            P::Level1 => block::heading_part(&cx, start, 1, &mut caps),
            P::Level2 => block::heading_part(&cx, start, 2, &mut caps),
            P::Level3 => block::heading_part(&cx, start, 3, &mut caps),
            P::Level4 => block::heading_part(&cx, start, 4, &mut caps),
            P::Level5 => block::heading_part(&cx, start, 5, &mut caps),
            P::Level6 => block::heading_part(&cx, start, 6, &mut caps),
            P::Ampersand => inline::ampersand(&cx, start),
            P::Bracket => inline::bracket(&cx, start),
            P::Bold => inline::emphasis_open(&cx, start, inline::Emphasis::Bold, &mut caps),
            P::Italic => inline::emphasis_open(&cx, start, inline::Emphasis::Italic, &mut caps),
            P::Raw => inline::raw(&cx, start, &mut caps),
            P::Strike => inline::strikethrough(&cx, start, &mut caps),
            P::Escape => inline::escape(&cx, start),
            P::ImageInline => inline::image_inline(&cx, start, &mut caps),
            P::ImageRef => inline::image_reference(&cx, start, &mut caps),
            P::LinkEmail => inline::link_email(&cx, start, &mut caps),
            P::LinkInet => inline::link_inet(&cx, start, &mut caps),
            P::LinkInline => inline::link_inline(&cx, start, &mut caps),
            P::LinkRef => inline::link_reference(&cx, start, &mut caps),
            P::LinkRefLiteral => inline::link_reference_literal(&cx, start, &mut caps),
            P::LinkRefShortcut => inline::link_reference_shortcut(&cx, start, &mut caps),
            P::FenceLanguage | P::FencePlain => None,
        }?;
        Some(Hit {
            rule: Some(rule),
            start,
            end,
            caps,
        })
    }

    // Matches.

    fn apply(&mut self, hit: &Hit) {
        match hit.rule {
            None => self.close(hit),
            Some(rule) => match self.container(rule, hit.start) {
                Some(kind) => self.open(kind, rule, hit),
                None => self.scope_match(rule, hit),
            },
        }
    }

    /// The container a rule opens, or none for a rule that only scopes text.
    fn container(&self, rule: P, start: usize) -> Option<Kind> {
        let byte = self.line.as_bytes().get(start).copied();
        Some(match rule {
            P::Quote => Kind::Quote,
            P::Bullet => Kind::ListBullet,
            P::Ordered => Kind::ListOrdered,
            P::FenceLanguage => Kind::Fence(self.opened.as_ref().and_then(|open| open.lang)?),
            P::FencePlain => Kind::FencePlain,
            P::EmbedBegin => match self.frames.last()?.rule {
                Rule::Open(Kind::Fence(lang)) => Kind::Body(lang),
                Rule::Open(_) | Rule::Capture(_) => return None,
            },
            P::RawBlock => Kind::RawBlock,
            P::Comment => Kind::Comment,
            P::Table => Kind::Table,
            P::Paragraph => Kind::Paragraph,
            P::Bold if byte == Some(b'*') => Kind::BoldStar,
            P::Bold => Kind::BoldUnder,
            P::Italic if byte == Some(b'*') => Kind::ItalicStar,
            P::Italic => Kind::ItalicUnder,
            P::Separator
            | P::Heading
            | P::Fence
            | P::LinkDef
            | P::TablePipe
            | P::TableSeparator
            | P::TableCell
            | P::Setext1
            | P::Setext2
            | P::Level1
            | P::Level2
            | P::Level3
            | P::Level4
            | P::Level5
            | P::Level6
            | P::Ampersand
            | P::Bracket
            | P::Raw
            | P::Strike
            | P::Escape
            | P::ImageInline
            | P::ImageRef
            | P::LinkEmail
            | P::LinkInet
            | P::LinkInline
            | P::LinkRef
            | P::LinkRefLiteral
            | P::LinkRefShortcut => return None,
        })
    }

    /// Keeps a container that cannot open out of the tree: its opening text,
    /// and the rest of the line when it opens with no text, stay in the
    /// container that is open.
    fn refuse(&mut self, hit: &Hit, parent: u32) {
        let to = if hit.end > hit.start {
            hit.end
        } else {
            self.limit
        };
        self.emit(to, parent);
        self.pos = to;
    }

    fn open(&mut self, kind: Kind, rule: P, hit: &Hit) {
        let parent = self.content();
        self.emit(hit.start, parent);
        let opened = self.opened.take();
        if self
            .limits
            .depth
            .is_some_and(|max| self.frames.len() >= max)
        {
            self.report.record(Limit::Depth);
            self.refuse(hit, parent);
            return;
        }
        let name = self.path(parent, rules::names(kind));
        let content = match rules::content_name(kind) {
            Some(scope) => self.child(name, scope),
            None => name,
        };
        let payload = match kind {
            Kind::Fence(_) | Kind::FencePlain => opened.map_or(0, |open| open.fence.payload()),
            Kind::Body(Lang::Rust) => rust::start_embedded(self.arena, content),
            Kind::Base
            | Kind::Quote
            | Kind::ListBullet
            | Kind::ListOrdered
            | Kind::Body(Lang::Markdown)
            | Kind::RawBlock
            | Kind::Table
            | Kind::Paragraph
            | Kind::Comment
            | Kind::BoldStar
            | Kind::BoldUnder
            | Kind::ItalicStar
            | Kind::ItalicUnder => 0,
        };
        // A container opened outside a capture group stays open between
        // lines, so it is a state.
        let in_capture = self
            .frames
            .iter()
            .any(|frame| matches!(frame.rule, Rule::Capture(_)));
        let state = if in_capture {
            None
        } else {
            let parent_state = self.frames.iter().rev().find_map(|frame| frame.state);
            let Some(state) =
                self.arena
                    .push(parent_state.unwrap_or(ROOT), kind.code(), payload, content)
            else {
                self.report.record(Limit::Nodes);
                self.refuse(hit, parent);
                return;
            };
            Some(state)
        };
        self.frames.push(Frame {
            rule: Rule::Open(kind),
            name,
            content: name,
            saved_anchor: self.anchor,
            payload,
            state,
        });
        self.captures(name, hit.end, &hit.caps, |group| {
            rules::capture(rule, group)
        });
        self.emit(hit.end, name);
        if let Some(frame) = self.frames.last_mut() {
            frame.content = content;
        }
        self.anchor = Some(hit.end);
        self.pos = self.pos.max(hit.end);
    }

    fn close(&mut self, hit: &Hit) {
        let Some(frame) = self.frames.last().copied() else {
            return;
        };
        let Rule::Open(kind) = frame.rule else {
            return;
        };
        self.emit(hit.start, frame.content);
        if let Some(end) = rules::end(kind) {
            self.captures(frame.name, hit.end, &hit.caps, |group| {
                rules::end_capture(end, group)
            });
        }
        self.emit(hit.end, frame.name);
        self.frames.pop();
        self.anchor = frame.saved_anchor;
        self.pos = self.pos.max(hit.end);
    }

    fn scope_match(&mut self, rule: P, hit: &Hit) {
        let parent = self.content();
        self.emit(hit.start, parent);
        let node = match rules::match_name(rule) {
            Some(scope) => self.child(parent, scope),
            None => parent,
        };
        self.captures(node, hit.end, &hit.caps, |group| {
            rules::capture(rule, group)
        });
        self.emit(hit.end, node);
        self.pos = self.pos.max(hit.end);
    }

    // Capture groups.

    /// Gives each capture group of a match its scope, or its rules, in group
    /// order. A group that lies inside an earlier one is scoped inside it. A
    /// group that is empty, or starts after the match, does nothing.
    fn captures(
        &mut self,
        base: u32,
        match_end: usize,
        caps: &Caps,
        capture: impl Fn(usize) -> Capture,
    ) {
        let mut open: [(u32, usize); 4] = [(0, 0); 4];
        let mut depth = 0;
        for (group, (start, end)) in caps.iter() {
            let capture = capture(group);
            if capture.is_none() || start == end {
                continue;
            }
            if start > match_end {
                break;
            }
            while depth > 0 && open[depth - 1].1 <= start {
                self.emit(open[depth - 1].1, open[depth - 1].0);
                depth -= 1;
            }
            let outer = if depth > 0 { open[depth - 1].0 } else { base };
            self.emit(start, outer);
            if let Some(list) = capture.list {
                let node = match capture.scope {
                    Some(scope) => self.child(base, scope),
                    None => base,
                };
                self.subrun(list, start, end, node);
            } else if let Some(scope) = capture.scope
                && depth < open.len()
            {
                let node = self.child(outer, scope);
                open[depth] = (node, end);
                depth += 1;
            }
        }
        while depth > 0 {
            self.emit(open[depth - 1].1, open[depth - 1].0);
            depth -= 1;
        }
    }

    /// Lexes the text from `start` to `end` with `list`'s rules alone, as a
    /// capture group that has rules of its own is. The line looks as if it
    /// ended at `end`, and every container the text opens ends with it.
    fn subrun(&mut self, list: List, start: usize, end: usize, node: u32) {
        if self
            .limits
            .depth
            .is_some_and(|max| self.frames.len() >= max)
        {
            self.report.record(Limit::Depth);
            self.emit(end, node);
            return;
        }
        let saved = (self.limit, self.pos, self.anchor);
        self.frames.push(Frame {
            rule: Rule::Capture(list),
            name: node,
            content: node,
            saved_anchor: None,
            payload: 0,
            state: None,
        });
        let floor = self.frames.len();
        self.limit = end;
        self.pos = start;
        self.anchor = None;
        while self.step() {}
        let top = self.content();
        self.emit(end, top);
        self.frames.truncate(floor - 1);
        (self.limit, self.pos, self.anchor) = saved;
    }

    // Embedded languages.

    /// Takes the rest of the line for the language of the open fence.
    fn embedded(&mut self) {
        let start = self.pos;
        if start >= self.line.len() {
            return;
        }
        let Some(frame) = self.frames.last().copied() else {
            return;
        };
        self.emit(start, frame.content);
        let first = self.out.len();
        let text = &self.line.as_bytes()[start..];
        let state = rust::line(
            self.arena,
            self.limits,
            text,
            frame.payload,
            self.out,
            self.report,
        );
        for token in &mut self.out[first..] {
            token.end += start;
        }
        self.last = self.line.len();
        self.pos = self.limit;
        if state == frame.payload {
            return;
        }
        // The state the next line resumes from: this body, with the Rust
        // lexer's new state.
        let below = self
            .frames
            .iter()
            .rev()
            .skip(1)
            .find_map(|frame| frame.state);
        let pushed = self.arena.push(
            below.unwrap_or(ROOT),
            Kind::Body(Lang::Rust).code(),
            state,
            frame.content,
        );
        if let Some(top) = self.frames.last_mut() {
            top.payload = state;
            if pushed.is_some() {
                top.state = pushed;
            } else {
                self.report.record(Limit::Nodes);
            }
        }
    }
}

/// `(^|\G)(\s*)(.*)`, the rule that starts the lines of an embedded
/// language. It matches at the end of the fence's opening line, where `\G`
/// is, and takes no text; on a line it only matches for itself, it takes the
/// whole line.
fn embed_begin(cx: &Cx<'_>, p: usize) -> Option<usize> {
    if !cx.at_start(p) {
        return None;
    }
    let text = cx.skip_space(p);
    if text >= cx.end {
        return Some(cx.end);
    }
    Some(cx.skip(text, |b| b != b'\n'))
}
