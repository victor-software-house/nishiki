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

use super::caps::{Caps, NESTING};
use super::rules::{self, Capture, End, List, P};
use super::scope::S;
use super::state::{Fence, Kind};
use super::text::Cx;
use super::{block, inline};
use crate::arena::{Arena, ROOT};
use crate::language::{Context, Line};
use crate::limits::{Limit, Limits, LineReport};
use crate::{Language, Stack, Token};

/// The window on the line that the rules see, built from the fields it needs
/// so that a rule can fill in `self.caps` while it is held.
macro_rules! cx {
    ($lexer:expr) => {
        Cx {
            line: $lexer.line.as_bytes(),
            end: $lexer.limit,
            anchor: $lexer.anchor,
            steps: &$lexer.steps,
            budget: $lexer.budget,
        }
    };
}

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
    /// embedded Rust lexer. It changes only with `state`, so the frames kept
    /// for the next line match the frames rebuilt from the arena.
    payload: u32,
    /// The interned state with this frame on top, for a container that stays
    /// open between lines; none for the text of a capture group.
    state: Option<u32>,
}

/// Where the rules of a container can start a match, so that the lexer tries
/// them only at those positions. A position it skips is one where every rule
/// fails at its first byte, so the matches do not change.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Starts {
    /// Nowhere: the container has no rule and no closing rule.
    Nowhere,
    /// Where `^` or `\G` matches, and at a blank or `[` (a definition may
    /// start anywhere).
    Block,
    /// Where `^` or `\G` matches only.
    Anchored,
    /// At a byte that opens an inline rule, and at the start of the line,
    /// where a setext underline may.
    Inline,
    /// At `-`, where `-->` starts.
    Dash,
    /// Anywhere.
    Anywhere,
}

impl Starts {
    fn of(kind: Kind) -> Starts {
        match kind {
            Kind::Base
            | Kind::Quote
            | Kind::ListBullet
            | Kind::ListOrdered
            | Kind::Body(Language::Markdown) => Starts::Block,
            Kind::Fence(_) | Kind::FencePlain | Kind::FrontMatter => Starts::Anchored,
            Kind::Body(_) | Kind::FrontMatterBody | Kind::RawBlock => Starts::Nowhere,
            Kind::Comment => Starts::Dash,
            Kind::Table => Starts::Anywhere,
            Kind::Paragraph
            | Kind::BoldStar
            | Kind::BoldUnder
            | Kind::ItalicStar
            | Kind::ItalicUnder => Starts::Inline,
        }
    }

    fn of_list(list: List) -> Starts {
        match list {
            List::Heading => Starts::Anywhere,
            List::Inline | List::LinkText | List::StrikeText => Starts::Inline,
        }
    }

    fn may_start(self, cx: &Cx<'_>, p: usize) -> bool {
        match self {
            Starts::Nowhere => false,
            Starts::Anywhere => true,
            Starts::Anchored => cx.at_start(p),
            Starts::Block => {
                cx.at_start(p)
                    || cx.get(p).is_some_and(|b| {
                        b == b'[' || b.is_ascii_whitespace() || b == 0x0b || b >= 0x80
                    })
            }
            Starts::Inline => cx.get(p).is_some_and(|b| {
                matches!(
                    b,
                    b'&' | b'<' | b'*' | b'_' | b'`' | b'~' | b'\\' | b'!' | b'['
                ) || (p == 0 && matches!(b, b'-' | b'='))
            }),
            Starts::Dash => cx.get(p) == Some(b'-'),
        }
    }
}

/// The bytes that open an inline rule, as a table.
#[expect(
    clippy::as_conversions,
    reason = "`usize::from` is not `const`, and a byte always fits"
)]
const INLINE_START: [bool; 256] = {
    let mut table = [false; 256];
    let bytes = *b"&<*_`~\\![";
    let mut i = 0;
    while i < bytes.len() {
        table[bytes[i] as usize] = true;
        i += 1;
    }
    table
};

/// A rule that matched.
struct Hit {
    /// Which rule, or none for the closing rule of the innermost container.
    rule: Option<P>,
    start: usize,
    end: usize,
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
    /// The capture groups of the rule being tried.
    caps: Caps,
    /// What the highlighter keeps from line to line.
    scratch: &'a mut crate::scratch::Scratch,
    /// Whether this is the first line of the document, where `\A` matches.
    /// The document's own state is its root with a payload of 1; after the
    /// first line the root has none, so `\A` matches on one line only.
    first_line: bool,
}

/// What the Markdown lexer keeps from one line to the next, so that a line
/// allocates nothing and does not look up what the last line interned.
#[derive(Debug, Default)]
pub(crate) struct Scratch {
    /// The open containers: after a line, those of `resume`, and otherwise
    /// the buffer the next line builds them in.
    frames: Vec<Frame>,
    /// The state `frames` holds the containers of.
    resume: Option<u32>,
    /// The paragraph last opened: the state it opened in, the stack of its
    /// text, and the state it made.
    paragraph: Option<(u32, u32, u32)>,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(
        scratch: &'a mut crate::scratch::Scratch,
        arena: &'a mut Arena,
        limits: &'a Limits,
        line: &'a str,
        out: &'a mut Vec<Token>,
        state: u32,
        report: &'a mut LineReport,
    ) -> Self {
        let first_token = out.len();
        let mut frames = std::mem::take(&mut scratch.markdown.frames);
        if scratch.markdown.resume == Some(state) {
            for frame in &mut frames {
                frame.saved_anchor = None;
            }
        } else {
            frames.clear();
            let mut at = state;
            while at != ROOT {
                let frame = arena.frame(at);
                let kind = Kind::from_code(frame.kind);
                let name = if rules::content_name(kind).is_some() {
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
        }
        let first_line = frames
            .first()
            .is_some_and(|root| matches!(root.rule, Rule::Open(Kind::Base)) && root.payload == 1);
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
            caps: Caps::new(),
            scratch,
            first_line,
        }
    }

    /// The state after the line, with its containers kept for the next one.
    ///
    /// After the first line the containers rest on the document's start
    /// state, so they move onto a root that is not the start, and the next
    /// line rebuilds them.
    pub(crate) fn finish(self) -> u32 {
        let state = self
            .frames
            .iter()
            .rev()
            .find_map(|frame| frame.state)
            .unwrap_or(ROOT);
        self.scratch.markdown.frames = self.frames;
        if self.first_line {
            self.scratch.markdown.resume = None;
            return after_first_line(self.arena, state).unwrap_or(state);
        }
        self.scratch.markdown.resume = Some(state);
        state
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

    // Containers that continue.

    /// Runs the `while` rule of each open container, outermost first. The
    /// first that fails closes its container and everything inside it. Only
    /// a quote's rule has capture groups.
    fn check_whiles(&mut self) {
        let mut index = 0;
        while index < self.frames.len() {
            let Rule::Open(kind) = self.frames[index].rule else {
                break;
            };
            self.caps.clear();
            let cx = cx!(self);
            let pos = self.pos;
            let end = match kind {
                Kind::Quote => block::quote_while(&cx, pos, &mut self.caps),
                Kind::ListBullet | Kind::ListOrdered => block::list_while(&cx, pos),
                Kind::FrontMatterBody => {
                    let dashes = index
                        .checked_sub(1)
                        .and_then(|outer| self.frames.get(outer))
                        .map_or(3, |outer| usize::try_from(outer.payload).unwrap_or(3));
                    let mut caps = Caps::new();
                    block::front_matter_close(&cx, pos, dashes, &mut caps)
                        .is_none()
                        .then_some(pos)
                }
                Kind::Body(_) => {
                    (cx.at_start(pos) && !block::is_fence_line(&cx, pos)).then_some(pos)
                }
                Kind::RawBlock => block::raw_block(&cx, pos),
                Kind::Table => block::table_while(&cx, pos).then_some(pos),
                Kind::Paragraph => block::paragraph_while(&cx, pos),
                Kind::Base
                | Kind::Fence(_)
                | Kind::FencePlain
                | Kind::FrontMatter
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
            let caps = self.caps;
            self.emit(pos, node);
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
        if self
            .frames
            .last()
            .and_then(|frame| embedded_language(frame.rule))
            .is_some()
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
    ///
    /// In inline text it skips to the next byte that opens an inline rule;
    /// the first byte of the line may also start a setext underline.
    fn find(&mut self) -> Option<Hit> {
        let (end, list, starts) = match self.frames.last() {
            None => (None, rules::patterns(Kind::Base), Starts::Block),
            Some(frame) => match frame.rule {
                Rule::Open(kind) => (
                    rules::end(kind).map(|end| (end, kind)),
                    rules::patterns(kind),
                    Starts::of(kind),
                ),
                Rule::Capture(list) => (None, rules::list_patterns(list), Starts::of_list(list)),
            },
        };
        let line = self.line.as_bytes();
        let mut start = self.pos;
        while start <= self.limit {
            if starts == Starts::Inline {
                let stop = self.limit.min(line.len());
                if start >= stop {
                    return None;
                }
                let first = start == 0 && matches!(line[0], b'-' | b'=');
                if !first {
                    start += line[start..stop]
                        .iter()
                        .position(|&b| INLINE_START[usize::from(b)])?;
                }
            } else if !starts.may_start(&cx!(self), start) {
                start += 1;
                continue;
            }
            if let Some((end, kind)) = end
                && let Some(hit) = self.try_end(end, kind, start)
            {
                return Some(hit);
            }
            let byte = line.get(start).copied().unwrap_or(b'\n');
            let (anchored, first) = if starts == Starts::Block {
                let cx = cx!(self);
                (cx.at_start(start), cx.at(cx.skip_space(start)))
            } else {
                (false, 0)
            };
            for &rule in list {
                let skip = match starts {
                    Starts::Inline => !rule.opens_with(byte),
                    Starts::Block => !rule.opens_block(anchored, byte, first),
                    Starts::Nowhere | Starts::Anchored | Starts::Dash | Starts::Anywhere => false,
                };
                if skip {
                    debug_assert!(
                        self.try_rule(rule, start).is_none(),
                        "{rule:?} matches at byte {start} outside the bytes it opens with"
                    );
                    continue;
                }
                if let Some(hit) = self.try_rule(rule, start) {
                    return Some(hit);
                }
            }
            start += 1;
        }
        None
    }

    /// The match of the open container's closing rule at `start`.
    ///
    /// Front matter's closing rule applies last, so on the first line, which
    /// it would close, the front matter begins instead.
    fn try_end(&mut self, end: End, kind: Kind, start: usize) -> Option<Hit> {
        self.caps.clear();
        let cx = cx!(self);
        let end_pos = match end {
            End::Fence => {
                let payload = self.frames.last().map_or(0, |frame| frame.payload);
                block::fence_close(&cx, start, Fence::from_payload(payload), &mut self.caps)
            }
            End::FrontMatter => {
                if self.first_line {
                    return None;
                }
                let payload = self.frames.last().map_or(3, |frame| frame.payload);
                let dashes = usize::try_from(payload).unwrap_or(3);
                block::front_matter_close(&cx, start, dashes, &mut self.caps)
            }
            End::Comment => block::comment_close(&cx, start, &mut self.caps),
            End::Bold => {
                let mark = if kind == Kind::BoldStar { b'*' } else { b'_' };
                inline::bold_close(&cx, start, mark, &mut self.caps)
            }
            End::Italic => {
                let mark = if kind == Kind::ItalicStar { b'*' } else { b'_' };
                inline::italic_close(&cx, start, mark, &mut self.caps)
            }
        }?;
        Some(Hit {
            rule: None,
            start,
            end: end_pos,
        })
    }

    /// The match of `rule` at `start`.
    ///
    /// A link definition leads with `\s*`, so only the start of a run of
    /// blanks can start one.
    fn try_rule(&mut self, rule: P, start: usize) -> Option<Hit> {
        self.caps.clear();
        let cx = cx!(self);
        let mut rule = rule;
        let end = match rule {
            P::FrontMatter => {
                if !self.first_line || self.frames.len() != 1 {
                    return None;
                }
                block::front_matter_open(&cx, start)
            }
            P::FrontMatterBegin => block::front_matter_begin(&cx, start, &mut self.caps),
            P::Separator => block::separator(&cx, start),
            P::Heading => block::heading(&cx, start, &mut self.caps),
            P::Quote => block::quote(&cx, start, &mut self.caps),
            P::Bullet => block::bullet(&cx, start, &mut self.caps),
            P::Ordered => block::ordered(&cx, start, &mut self.caps),
            P::Fence => {
                let open = block::fence_open(&cx, start, &mut self.caps)?;
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
                if start > self.pos && cx.space_before(start) > 0 {
                    return None;
                }
                block::link_def(&cx, start, &mut self.caps)
            }
            P::Comment => block::comment_open(&cx, start, &mut self.caps),
            P::Table => block::table(&cx, start, &mut self.caps),
            P::Paragraph => block::paragraph(&cx, start),
            P::EmbedBegin => embed_begin(&cx, start),
            P::TablePipe => (cx.get(start) == Some(b'|')).then_some(start + 1),
            P::TableSeparator => block::table_separator(&cx, start, &mut self.caps),
            P::TableCell => block::table_cell(&cx, start, &mut self.caps),
            P::Setext1 => block::setext(&cx, start, b'='),
            P::Setext2 => block::setext(&cx, start, b'-'),
            P::Level1 => block::heading_part(&cx, start, 1, &mut self.caps),
            P::Level2 => block::heading_part(&cx, start, 2, &mut self.caps),
            P::Level3 => block::heading_part(&cx, start, 3, &mut self.caps),
            P::Level4 => block::heading_part(&cx, start, 4, &mut self.caps),
            P::Level5 => block::heading_part(&cx, start, 5, &mut self.caps),
            P::Level6 => block::heading_part(&cx, start, 6, &mut self.caps),
            P::Ampersand => inline::ampersand(&cx, start),
            P::Bracket => inline::bracket(&cx, start),
            P::Bold => inline::emphasis_open(&cx, start, inline::Emphasis::Bold, &mut self.caps),
            P::Italic => {
                inline::emphasis_open(&cx, start, inline::Emphasis::Italic, &mut self.caps)
            }
            P::Raw => inline::raw(&cx, start, &mut self.caps),
            P::Strike => inline::strikethrough(&cx, start, &mut self.caps),
            P::Escape => inline::escape(&cx, start),
            P::ImageInline => inline::image_inline(&cx, start, &mut self.caps),
            P::ImageRef => inline::image_reference(&cx, start, &mut self.caps),
            P::LinkEmail => inline::link_email(&cx, start, &mut self.caps),
            P::LinkInet => inline::link_inet(&cx, start, &mut self.caps),
            P::LinkInline => inline::link_inline(&cx, start, &mut self.caps),
            P::LinkRef => inline::link_reference(&cx, start, &mut self.caps),
            P::LinkRefLiteral => inline::link_reference_literal(&cx, start, &mut self.caps),
            P::LinkRefShortcut => inline::link_reference_shortcut(&cx, start, &mut self.caps),
            P::FenceLanguage | P::FencePlain => None,
        }?;
        Some(Hit {
            rule: Some(rule),
            start,
            end,
        })
    }

    // Matches.

    /// Applies a match: closes, opens or scopes. The capture groups are
    /// copied out first, because the rules the groups hand text to overwrite
    /// them.
    fn apply(&mut self, hit: &Hit) {
        let caps = self.caps;
        match hit.rule {
            None => self.close(hit, &caps),
            Some(rule) => match self.container(rule, hit.start) {
                Some(kind) => self.open(kind, rule, hit, &caps),
                None => self.scope_match(rule, hit, &caps),
            },
        }
    }

    /// The container a rule opens, or none for a rule that only scopes text.
    fn container(&self, rule: P, start: usize) -> Option<Kind> {
        let byte = self.line.as_bytes().get(start).copied();
        Some(match rule {
            P::FrontMatter => Kind::FrontMatter,
            P::FrontMatterBegin => Kind::FrontMatterBody,
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

    /// Whether the depth limit keeps another frame from opening above the
    /// document's own.
    fn at_depth_limit(&self) -> bool {
        self.limits
            .depth_reached(self.frames.len().saturating_sub(1))
    }

    /// Keeps a container that cannot open out of the tree: its opening text,
    /// and the rest of the line when it opens with no text, stay in the
    /// container that is open. A refusal with no text ends the window, which
    /// moves the cursor past it: the rule would match again at its end and
    /// be refused again without progress.
    fn refuse(&mut self, hit: &Hit, parent: u32) {
        if hit.end > hit.start {
            self.emit(hit.end, parent);
            self.pos = hit.end;
        } else {
            self.emit(self.limit, parent);
            self.pos = self.limit + 1;
        }
    }

    /// Opens a container of `kind` for a match of `rule`.
    ///
    /// A container opened outside a capture group stays open between lines,
    /// so it is a state. A paragraph opens on nearly every line in the same
    /// state, so the last one opened is reused.
    fn open(&mut self, kind: Kind, rule: P, hit: &Hit, caps: &Caps) {
        let parent = self.content();
        self.emit(hit.start, parent);
        let opened = self.opened.take();
        if self.at_depth_limit() {
            self.report.record(Limit::Depth);
            self.refuse(hit, parent);
            return;
        }
        let in_capture = self
            .frames
            .iter()
            .any(|frame| matches!(frame.rule, Rule::Capture(_)));
        let parent_state = if in_capture {
            None
        } else {
            self.frames.iter().rev().find_map(|frame| frame.state)
        };
        let cached = match (kind, parent_state, self.scratch.markdown.paragraph) {
            (Kind::Paragraph, Some(from), Some((was, stack, state))) if from == was => {
                Some((stack, state))
            }
            _ => None,
        };
        let (name, content, payload, state) = if let Some((stack, state)) = cached {
            (stack, stack, 0, Some(state))
        } else {
            let name = self.path(parent, rules::names(kind));
            let content = match rules::content_name(kind) {
                Some(scope) => self.child(name, scope),
                None => name,
            };
            let payload = match kind {
                Kind::Fence(_) | Kind::FencePlain => opened.map_or(0, |open| open.fence.payload()),
                Kind::Body(lang) if lang != Language::Markdown => {
                    lang.start_embedded(self.arena, content)
                }
                #[cfg(feature = "lang-yaml")]
                Kind::FrontMatterBody => Language::Yaml.start_embedded(self.arena, content),
                #[cfg(not(feature = "lang-yaml"))]
                Kind::FrontMatterBody => 0,
                Kind::FrontMatter => {
                    let dashes = self.line.bytes().take_while(|&b| b == b'-').count();
                    u32::try_from(dashes).unwrap_or(u32::MAX)
                }
                Kind::Base
                | Kind::Quote
                | Kind::ListBullet
                | Kind::ListOrdered
                | Kind::Body(_)
                | Kind::RawBlock
                | Kind::Table
                | Kind::Paragraph
                | Kind::Comment
                | Kind::BoldStar
                | Kind::BoldUnder
                | Kind::ItalicStar
                | Kind::ItalicUnder => 0,
            };
            let state = if in_capture {
                None
            } else {
                let Some(state) =
                    self.arena
                        .push(parent_state.unwrap_or(ROOT), kind.code(), payload, content)
                else {
                    self.report.record(Limit::Nodes);
                    self.refuse(hit, parent);
                    return;
                };
                if let (Kind::Paragraph, Some(from)) = (kind, parent_state) {
                    self.scratch.markdown.paragraph = Some((from, content, state));
                }
                Some(state)
            };
            (name, content, payload, state)
        };
        self.frames.push(Frame {
            rule: Rule::Open(kind),
            name,
            content: name,
            saved_anchor: self.anchor,
            payload,
            state,
        });
        self.captures(name, hit.end, caps, |group| rules::capture(rule, group));
        self.emit(hit.end, name);
        if let Some(frame) = self.frames.last_mut() {
            frame.content = content;
        }
        self.anchor = Some(hit.end);
        self.pos = self.pos.max(hit.end);
    }

    fn close(&mut self, hit: &Hit, caps: &Caps) {
        let Some(frame) = self.frames.last().copied() else {
            return;
        };
        let Rule::Open(kind) = frame.rule else {
            return;
        };
        self.emit(hit.start, frame.content);
        if let Some(end) = rules::end(kind) {
            self.captures(frame.name, hit.end, caps, |group| {
                rules::end_capture(end, group)
            });
        }
        self.emit(hit.end, frame.name);
        self.frames.pop();
        self.anchor = frame.saved_anchor;
        self.pos = self.pos.max(hit.end);
    }

    fn scope_match(&mut self, rule: P, hit: &Hit, caps: &Caps) {
        let parent = self.content();
        self.emit(hit.start, parent);
        let node = match rules::match_name(rule) {
            Some(scope) => self.child(parent, scope),
            None => parent,
        };
        self.captures(node, hit.end, caps, |group| rules::capture(rule, group));
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
        let mut open = [(0_u32, 0_usize); NESTING];
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
            } else if let Some(scope) = capture.scope {
                debug_assert!(depth < NESTING, "group {group} nests past {NESTING}");
                if depth < NESTING {
                    let node = self.child(outer, scope);
                    open[depth] = (node, end);
                    depth += 1;
                }
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
        if self.at_depth_limit() {
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

    /// Takes the rest of the line for the language of the open fence, from
    /// the cursor or, when the containers took the line feed too, from the
    /// line's end. The next line resumes from this body with the embedded
    /// lexer's new state.
    fn embedded(&mut self) {
        let start = self.pos;
        let Some(frame) = self.frames.last().copied() else {
            return;
        };
        let (Rule::Open(kind), Some(lang)) = (frame.rule, embedded_language(frame.rule)) else {
            return;
        };
        if start >= self.line.len() && !lang.reads_line_feed() {
            return;
        }
        let start = start.min(self.line.len());
        self.emit(start, frame.content);
        let mut cx = Context {
            scratch: &mut *self.scratch,
            arena: &mut *self.arena,
            limits: self.limits,
            out: &mut *self.out,
            report: &mut *self.report,
        };
        let line = Line {
            text: self.line,
            start,
            embedded: true,
        };
        let state = lang.lex(&mut cx, line, frame.payload);
        self.last = self.line.len();
        self.pos = self.limit;
        if state == frame.payload {
            return;
        }
        let below = self
            .frames
            .iter()
            .rev()
            .skip(1)
            .find_map(|frame| frame.state);
        let pushed = self
            .arena
            .push(below.unwrap_or(ROOT), kind.code(), state, frame.content);
        let Some(pushed) = pushed else {
            self.report.record(Limit::Nodes);
            return;
        };
        if let Some(top) = self.frames.last_mut() {
            top.payload = state;
            top.state = Some(pushed);
        }
    }
}

/// The language whose lexer takes the rest of each line inside `rule`: an
/// embedded fence's, or YAML in front matter. A `markdown` fence takes
/// Markdown's own rules instead.
fn embedded_language(rule: Rule) -> Option<Language> {
    match rule {
        Rule::Open(Kind::Body(Language::Markdown)) => None,
        Rule::Open(Kind::Body(lang)) => Some(lang),
        #[cfg(feature = "lang-yaml")]
        Rule::Open(Kind::FrontMatterBody) => Some(Language::Yaml),
        Rule::Open(_) | Rule::Capture(_) => None,
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

/// The state `state`, whose root is the document's start state, rebuilt on a
/// root that is not, so that the lines after the first are told apart from it.
fn after_first_line(arena: &mut Arena, state: u32) -> Option<u32> {
    let mut chain = Vec::new();
    let mut at = state;
    while at != ROOT {
        let frame = arena.frame(at);
        chain.push(frame);
        at = frame.parent;
    }
    let root = chain.pop()?;
    let mut rebuilt = arena.push(ROOT, root.kind, 0, root.stack)?;
    while let Some(frame) = chain.pop() {
        rebuilt = arena.push(rebuilt, frame.kind, frame.payload, frame.stack)?;
    }
    Some(rebuilt)
}
