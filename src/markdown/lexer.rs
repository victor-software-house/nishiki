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

use super::caps::{Caps, NESTING};
#[cfg(feature = "lang-html-derivative")]
use super::html;
use super::inline::Emphasis;
use super::rules::{self, Capture, End, List, P};
use super::runs::Runs;
use super::scope::S;
use super::state::{Fence, Kind};
use super::text::{Cx, LineFacts};
use super::{block, inline};
use crate::arena::{Arena, Frame as ArenaFrame, ROOT};
use crate::language::{Context, Line, Parts};
use crate::limits::{Finished, Limit, Limits, LineReport, Steps};
use crate::{Language, Stack, Token};

/// The window on the line that the rules see, built from the fields it needs
/// so that a rule can borrow other fields, such as the runs, while it is held.
macro_rules! cx {
    ($lexer:expr) => {
        Cx {
            line: $lexer.line.as_bytes(),
            end: $lexer.limit,
            anchor: $lexer.anchor,
            facts: &$lexer.facts,
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
            Kind::Base | Kind::Quote | Kind::ListBullet | Kind::ListOrdered => Starts::Block,
            #[cfg(feature = "lang-vue")]
            Kind::Body(Language::Vue) => Starts::Anchored,
            Kind::Body(language) => {
                if language == Language::Markdown {
                    Starts::Block
                } else {
                    Starts::Nowhere
                }
            }
            Kind::Fence(_) | Kind::FencePlain | Kind::FrontMatter => Starts::Anchored,
            #[cfg(feature = "lang-html")]
            Kind::FencePhp => Starts::Anchored,
            #[cfg(feature = "lang-html")]
            Kind::BodyPhp => Starts::Nowhere,
            Kind::FrontMatterBody
            | Kind::RawBlock
            | Kind::HtmlRawBody
            | Kind::HtmlBlock
            | Kind::HtmlInline
            | Kind::HtmlWrap => Starts::Nowhere,
            Kind::Comment => Starts::Dash,
            Kind::HtmlRaw | Kind::Table => Starts::Anywhere,
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
            List::Inline | List::HeadingText | List::LinkText | List::StrikeText => Starts::Inline,
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
    /// The lowest index this line opened a container at.
    /// Frames below it hold no saved anchor, so [`Lexer::finish`] clears
    /// only the frames from here up.
    anchored: usize,
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
    /// Steps spent on this line, counted in the matchers' scans too, and
    /// lent to the languages of its fences, with the character before the
    /// line's start.
    facts: LineFacts,
    /// The fence the last match of the fence rule opened.
    opened: Option<block::FenceOpen>,
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
    /// The open containers: after a line, those of `resume`, with no saved
    /// anchors, and otherwise the buffer the next line builds them in.
    frames: Vec<Frame>,
    /// The state `frames` holds the containers of.
    resume: Option<u32>,
    /// The paragraph last opened: the state it opened in, the stack of its
    /// text, and the state it made.
    paragraph: Option<(u32, u32, u32)>,
    /// The frames of the first line's state, innermost first, while
    /// [`after_first_line`] rebuilds them.
    chain: Vec<ArenaFrame>,
    /// The line's delimiter runs, found when a code span or a
    /// strikethrough first needs them.
    runs: Runs,
    /// How the line starts.
    /// Only [`crate::markdown::lex_until`] sets it, for one line.
    pub(super) start: Start,
    /// Whether the line stopped at its start, with no tokens.
    pub(super) stopped: bool,
}

/// How a line starts: as a line of a document, or as Markdown that another
/// language embeds from `line.start`.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) enum Start {
    #[default]
    Document,
    /// `stop` when the end of the embedding rule holds at the start, so the
    /// line stops there once no container is left open.
    /// `before` is the character before the start
    /// ([`LineFacts::before_start`]).
    Embedded { stop: bool, before: Option<char> },
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(
        Parts {
            arena,
            limits,
            out,
            report,
        }: Parts<'a>,
        scratch: &'a mut crate::scratch::Scratch,
        line: &'a str,
        state: u32,
    ) -> Self {
        let first_token = out.len();
        scratch.markdown.runs.reset();
        let mut frames = std::mem::take(&mut scratch.markdown.frames);
        if scratch.markdown.resume != Some(state) {
            frames.clear();
            let mut at = state;
            while at != ROOT {
                let frame = arena.frame(at);
                let kind = Kind::from_code(frame.kind);
                let name = if rules::content_name(kind).is_empty() {
                    frame.stack
                } else {
                    arena.frame(frame.parent).stack
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
            anchored: frames.len(),
            frames,
            pos: 0,
            anchor: None,
            limit: line.len() + 1,
            last: 0,
            facts: LineFacts {
                steps: Steps::new(limits, line.len()),
                before_start: None,
            },
            opened: None,
            scratch,
            first_line,
        }
    }

    /// The state after the line, with its containers kept for the next one.
    ///
    /// After the first line the containers rest on the document's start
    /// state, so they move onto a root that is not the start, and the next
    /// line rebuilds them. The line's count goes back to the lender.
    /// The frames this line opened lose their saved anchors, which belong
    /// to this line.
    #[expect(
        clippy::inline_always,
        reason = "measured at sixteen codegen units: left out of line, it took the lexer by a libc copy of 815k instructions on the Markdown fixtures; inlined, Markdown ran 1.6% fewer, and one unit was unchanged"
    )]
    #[inline(always)]
    pub(crate) fn finish(mut self) -> Finished {
        let state = self
            .frames
            .iter()
            .rev()
            .find_map(|frame| frame.state)
            .unwrap_or(ROOT);
        if let Some(pushed) = self.frames.get_mut(self.anchored..) {
            for frame in pushed {
                frame.saved_anchor = None;
            }
        }
        self.scratch.markdown.frames = self.frames;
        let next = if self.first_line {
            self.scratch.markdown.resume = None;
            let chain = &mut self.scratch.markdown.chain;
            after_first_line(self.arena, chain, state).unwrap_or(state)
        } else {
            self.scratch.markdown.resume = Some(state);
            state
        };
        self.facts.steps.finish(next)
    }

    /// Lexes the line. A line whose scans spent its work reports the work
    /// limit, though the scan that ran out gave up without a match and the
    /// loop then found nothing more.
    ///
    /// A line that another language embeds stops at its start when its
    /// [`Start`] says so and the open containers' `while` rules closed them
    /// all.
    /// That is the one place the embedding rule's end can hold.
    /// At the document's level every line opens a block at its start, and
    /// inside a block the embedding rule's end is not tried.
    ///
    /// The character before an embedded start is read only here, after the
    /// `while` rules, which read nothing before the start.
    /// A document's line reads only the one test it read before.
    pub(crate) fn run(&mut self) {
        let mut caps = Caps::new();
        self.check_whiles(&mut caps);
        if let Start::Embedded { stop, before } = self.scratch.markdown.start {
            if stop && self.frames.len() == 1 {
                self.scratch.markdown.stopped = true;
                return;
            }
            self.facts.before_start = before;
        }
        while self.step(&mut caps) {}
        if self.facts.steps.exhausted() {
            self.report.record(Limit::Work);
        }
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

    /// Covers the text up to `upto` with a token on the scope stack `node`.
    /// It extends the last token when that token has the same stack.
    ///
    /// Always inlined.
    /// Left to LLVM, it went out of line once `step` read the line-feed
    /// flags from a table, at a call per token.
    #[expect(
        clippy::inline_always,
        reason = "measured at one codegen unit: emit went out of line when step changed size, at a call per token"
    )]
    #[inline(always)]
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
    fn check_whiles(&mut self, caps: &mut Caps) {
        let mut index = 0;
        while index < self.frames.len() {
            let Rule::Open(kind) = self.frames[index].rule else {
                break;
            };
            caps.clear();
            let cx = cx!(self);
            let pos = self.pos;
            let end = match kind {
                Kind::Quote => block::quote_while(&cx, pos, caps),
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
                #[cfg(feature = "lang-vue")]
                Kind::Body(Language::Vue) => {
                    if vue_body_line(
                        &cx,
                        self.arena,
                        &mut self.frames,
                        &mut self.anchor,
                        index,
                        pos,
                    ) {
                        index += 1;
                        continue;
                    }
                    None
                }
                Kind::Body(_) => {
                    (cx.at_start(pos) && !block::is_fence_line(&cx, pos)).then_some(pos)
                }
                #[cfg(feature = "lang-html")]
                Kind::BodyPhp => {
                    (cx.at_start(pos) && !block::is_fence_line(&cx, pos)).then_some(pos)
                }
                Kind::RawBlock => block::raw_block(&cx, pos),
                Kind::Table => block::table_while(&cx, pos).then_some(pos),
                Kind::Paragraph => block::paragraph_while(&cx, pos),
                #[cfg(feature = "lang-html-derivative")]
                Kind::HtmlRawBody => html::raw_while(&cx, pos),
                #[cfg(feature = "lang-html-derivative")]
                Kind::HtmlBlock => html::block_while(&cx, pos),
                #[cfg(not(feature = "lang-html-derivative"))]
                Kind::HtmlRawBody | Kind::HtmlBlock => None,
                #[cfg(feature = "lang-html")]
                Kind::FencePhp => {
                    index += 1;
                    continue;
                }
                Kind::Base
                | Kind::Fence(_)
                | Kind::FencePlain
                | Kind::FrontMatter
                | Kind::Comment
                | Kind::BoldStar
                | Kind::BoldUnder
                | Kind::ItalicStar
                | Kind::ItalicUnder
                | Kind::HtmlRaw
                | Kind::HtmlInline
                | Kind::HtmlWrap => {
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
            self.captures(node, end, caps, |group| rules::capture(P::Quote, group));
            self.emit(end, node);
            self.pos = self.pos.max(end);
            self.anchor = Some(end);
            index += 1;
        }
    }

    // The loop.

    /// Takes the next match from the cursor. False when nothing matches, or
    /// the line has spent its work.
    fn step(&mut self, caps: &mut Caps) -> bool {
        if self.facts.steps.step() {
            self.report.record(Limit::Work);
            return false;
        }
        let from = (self.pos, self.frames.len());
        if self.frames.last().is_some_and(|frame| embeds(frame.rule)) {
            self.embedded();
        }
        let Some(hit) = self.find(caps) else {
            return false;
        };
        self.apply(&hit, caps);
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
    fn find(&mut self, caps: &mut Caps) -> Option<Hit> {
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
                && let Some(hit) = self.try_end(end, kind, start, caps)
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
                        self.try_rule(rule, start, caps).is_none(),
                        "{rule:?} matches at byte {start} outside the bytes it opens with"
                    );
                    continue;
                }
                if let Some(hit) = self.try_rule(rule, start, caps) {
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
    fn try_end(&mut self, end: End, kind: Kind, start: usize, caps: &mut Caps) -> Option<Hit> {
        caps.clear();
        let cx = cx!(self);
        let end_pos = match end {
            End::Fence => {
                let payload = self.frames.last().map_or(0, |frame| frame.payload);
                block::fence_close(&cx, start, Fence::from_payload(payload), caps)
            }
            End::FrontMatter => {
                if self.first_line {
                    return None;
                }
                let payload = self.frames.last().map_or(3, |frame| frame.payload);
                let dashes = usize::try_from(payload).unwrap_or(3);
                block::front_matter_close(&cx, start, dashes, caps)
            }
            End::Comment => block::comment_close(&cx, start, caps),
            #[cfg(feature = "lang-html-derivative")]
            End::HtmlRaw => html::raw_close(&cx, start, caps),
            End::Bold => {
                let mark = if kind == Kind::BoldStar { b'*' } else { b'_' };
                inline::bold_close(&cx, start, mark, caps)
            }
            End::Italic => {
                let mark = if kind == Kind::ItalicStar { b'*' } else { b'_' };
                inline::italic_close(&cx, start, mark, caps)
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
    #[expect(
        clippy::too_many_lines,
        reason = "one arm per rule; the tables are borrowed in the arms that read them"
    )]
    fn try_rule(&mut self, rule: P, start: usize, caps: &mut Caps) -> Option<Hit> {
        caps.clear();
        #[cfg(feature = "lang-html-derivative")]
        if rule == P::Html {
            return self.html_opens(start).then_some(Hit {
                rule: Some(rule),
                start,
                end: start,
            });
        }
        let cx = cx!(self);
        let mut rule = rule;
        let end = match rule {
            P::FrontMatter => {
                if !self.first_line || self.frames.len() != 1 {
                    return None;
                }
                block::front_matter_open(&cx, start)
            }
            P::FrontMatterBegin => block::front_matter_begin(&cx, start, caps),
            P::Separator => block::separator(&cx, start),
            P::Heading => block::heading(&cx, start, caps),
            P::Quote => block::quote(&cx, start, caps),
            P::Bullet => block::bullet(&cx, start, caps),
            P::Ordered => block::ordered(&cx, start, caps),
            P::Fence => {
                let open = block::fence_open(&cx, start, caps)?;
                let end = open.end;
                rule = if open.embed.is_none() {
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
                block::link_def(&cx, start, caps)
            }
            P::Comment => block::comment_open(&cx, start, caps),
            P::Table => block::table(&cx, start, caps),
            P::Paragraph => block::paragraph(&cx, start),
            P::EmbedBegin => embed_begin(&cx, start),
            P::TablePipe => (cx.get(start) == Some(b'|')).then_some(start + 1),
            P::TableSeparator => block::table_separator(&cx, start, caps),
            P::TableCell => block::table_cell(&cx, start, caps),
            P::Setext1 => block::setext(&cx, start, b'='),
            P::Setext2 => block::setext(&cx, start, b'-'),
            P::Level1 => block::heading_part(&cx, start, 1, caps),
            P::Level2 => block::heading_part(&cx, start, 2, caps),
            P::Level3 => block::heading_part(&cx, start, 3, caps),
            P::Level4 => block::heading_part(&cx, start, 4, caps),
            P::Level5 => block::heading_part(&cx, start, 5, caps),
            P::Level6 => block::heading_part(&cx, start, 6, caps),
            P::Ampersand => inline::ampersand(&cx, start),
            P::Bracket => inline::bracket(&cx, start),
            P::Bold => inline::emphasis_open(
                &cx,
                &mut self.scratch.markdown.runs,
                start,
                Emphasis::Bold,
                caps,
            ),
            P::Italic => inline::emphasis_open(
                &cx,
                &mut self.scratch.markdown.runs,
                start,
                Emphasis::Italic,
                caps,
            ),
            P::Raw => inline::raw(&cx, &mut self.scratch.markdown.runs, start, caps),
            P::Strike => inline::strikethrough(&cx, &mut self.scratch.markdown.runs, start, caps),
            P::Escape => inline::escape(&cx, start),
            P::ImageInline => {
                inline::image_inline(&cx, &mut self.scratch.markdown.runs, start, caps)
            }
            P::ImageRef => inline::image_reference(&cx, start, caps),
            P::LinkEmail => inline::link_email(&cx, start, caps),
            P::LinkInet => inline::link_inet(&cx, start, caps),
            P::LinkInline => inline::link_inline(&cx, &mut self.scratch.markdown.runs, start, caps),
            P::LinkRef => inline::link_reference(&cx, start, caps),
            P::LinkRefLiteral => inline::link_reference_literal(&cx, start, caps),
            P::LinkRefShortcut => inline::link_reference_shortcut(&cx, start, caps),
            P::FenceLanguage | P::FencePlain => None,
            #[cfg(feature = "lang-html-derivative")]
            P::HtmlRaw => html::raw_open(&cx, start),
            #[cfg(feature = "lang-html-derivative")]
            P::HtmlRawBody => Some(cx.skip_space(start)),
            #[cfg(feature = "lang-html-derivative")]
            P::HtmlBlock => html::block_open(&cx, start),
            #[cfg(feature = "lang-html-derivative")]
            P::HtmlWrap => html::wrap_open(&cx, start),
            #[cfg(feature = "lang-html-derivative")]
            P::Html => None,
        }?;
        Some(Hit {
            rule: Some(rule),
            start,
            end,
        })
    }

    // Matches.

    /// Applies a match: closes, opens or scopes.
    /// `caps` holds the match's capture groups.
    /// The rules that a group hands text to fill groups of their own, in
    /// [`Lexer::subrun`], so `caps` stays as the match left it.
    fn apply(&mut self, hit: &Hit, caps: &Caps) {
        match hit.rule {
            None => self.close(hit, caps),
            Some(rule) => match self.container(rule, hit.start) {
                Some(kind) => self.open(kind, rule, hit, caps),
                None => self.scope_match(rule, hit, caps),
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
            P::FenceLanguage => self.opened.as_ref().and_then(|open| open.embed)?.fence(),
            P::FencePlain => Kind::FencePlain,
            P::EmbedBegin => match self.frames.last()?.rule {
                Rule::Open(Kind::Fence(lang)) => Kind::Body(lang),
                #[cfg(feature = "lang-html")]
                Rule::Open(Kind::FencePhp) => Kind::BodyPhp,
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
            #[cfg(feature = "lang-html-derivative")]
            P::HtmlRaw => Kind::HtmlRaw,
            #[cfg(feature = "lang-html-derivative")]
            P::HtmlRawBody => Kind::HtmlRawBody,
            #[cfg(feature = "lang-html-derivative")]
            P::HtmlBlock => Kind::HtmlBlock,
            #[cfg(feature = "lang-html-derivative")]
            P::Html => Kind::HtmlInline,
            #[cfg(feature = "lang-html-derivative")]
            P::HtmlWrap => Kind::HtmlWrap,
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
            let content = self.path(name, rules::content_name(kind));
            let payload = match kind {
                Kind::Fence(_) | Kind::FencePlain => opened.map_or(0, |open| open.fence.payload()),
                Kind::Body(lang) if lang != Language::Markdown => {
                    lang.start_embedded(self.arena, content)
                }
                #[cfg(feature = "lang-html")]
                Kind::FencePhp => opened.map_or(0, |open| open.fence.payload()),
                #[cfg(feature = "lang-html")]
                Kind::BodyPhp => Language::Html.start_embedded(self.arena, content),
                #[cfg(feature = "lang-yaml")]
                Kind::FrontMatterBody => Language::Yaml.start_embedded(self.arena, content),
                #[cfg(not(feature = "lang-yaml"))]
                Kind::FrontMatterBody => 0,
                #[cfg(feature = "lang-html-derivative")]
                Kind::HtmlRawBody | Kind::HtmlBlock | Kind::HtmlInline | Kind::HtmlWrap => {
                    Language::HtmlDerivative.start_embedded(self.arena, content)
                }
                #[cfg(not(feature = "lang-html-derivative"))]
                Kind::HtmlRawBody | Kind::HtmlBlock | Kind::HtmlInline | Kind::HtmlWrap => 0,
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
                | Kind::ItalicUnder
                | Kind::HtmlRaw => 0,
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
        self.anchored = self.anchored.min(self.frames.len());
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
            self.end_captures(end, frame.name, hit.end, caps);
        }
        self.emit(hit.end, frame.name);
        self.frames.pop();
        self.anchor = frame.saved_anchor;
        self.pos = self.pos.max(hit.end);
    }

    /// The capture groups of `end`'s match, which ends at `match_end`.
    fn end_captures(&mut self, end: End, base: u32, match_end: usize, caps: &Caps) {
        match end {
            #[cfg(feature = "lang-html-derivative")]
            End::HtmlRaw => self.html_raw_end(base, caps),
            End::FrontMatter | End::Fence | End::Comment | End::Bold | End::Italic => {
                self.captures(base, match_end, caps, |group| {
                    rules::end_capture(end, group)
                });
            }
        }
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
    /// Its rules fill a table of groups of their own, so the match that
    /// called it keeps its groups.
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
        let mut caps = Caps::new();
        while self.step(&mut caps) {}
        let top = self.content();
        self.emit(end, top);
        self.frames.truncate(floor - 1);
        (self.limit, self.pos, self.anchor) = saved;
    }

    // Embedded languages.

    /// Whether the text the rules see is a line or a capture group.
    #[cfg(feature = "lang-html-derivative")]
    fn span(&self) -> crate::html::Span {
        if self.limit > self.line.len() {
            crate::html::Span::Line
        } else {
            crate::html::Span::Capture
        }
    }

    /// Whether one of html-derivative's root patterns matches at `start`.
    #[cfg(feature = "lang-html-derivative")]
    fn html_opens(&mut self, start: usize) -> bool {
        if !matches!(self.line.as_bytes().get(start), Some(b'<' | b'&')) {
            return false;
        }
        let span = self.span();
        let text = &self.line[..self.limit.min(self.line.len())];
        let state = crate::html::start_embedded(self.arena, ROOT);
        let lent = self.facts.steps.lend(self.limits);
        let mut cx = Context {
            scratch: &mut *self.scratch,
            arena: &mut *self.arena,
            limits: &lent,
            out: &mut *self.out,
            report: &mut *self.report,
        };
        let line = Line {
            text,
            start,
            embedded: true,
        };
        let (opens, spent) = crate::html::derivative_opens_at(&mut cx, line, state, span);
        self.facts.steps.add(spent);
        opens
    }

    /// Gives the line back from html-derivative, which gave it back at `at`,
    /// closing its container. A wrapper that closes where it opened would
    /// open again there, so, as in `TextMate`, it stays open and takes the
    /// rest of the text.
    #[cfg(feature = "lang-html-derivative")]
    fn give_back_at(&mut self, at: usize, start: usize, end: usize, kind: Kind, content: u32) {
        self.last = self.last.max(at);
        if at == start && matches!(kind, Kind::HtmlWrap) {
            self.emit(end, content);
            self.pos = self.limit + 1;
            return;
        }
        if let Some(frame) = self.frames.pop() {
            self.anchor = frame.saved_anchor;
        }
        self.pos = at;
    }

    /// The closing match of a raw HTML block: the text before the closing
    /// tag goes to html-derivative from its start, as a capture group, and
    /// the tag is named after its own spelling, as `$4` names it.
    #[cfg(feature = "lang-html-derivative")]
    fn html_raw_end(&mut self, base: u32, caps: &Caps) {
        let mut groups = [(0, 0); 6];
        for (group, range) in caps.iter() {
            if let Some(slot) = groups.get_mut(group) {
                *slot = range;
            }
        }
        let [_, (text, close), _, _, (name, name_end), _] = groups;
        if text < close {
            let state = crate::html::start_embedded(self.arena, base);
            let lent = self.facts.steps.lend(self.limits);
            let mut cx = Context {
                scratch: &mut *self.scratch,
                arena: &mut *self.arena,
                limits: &lent,
                out: &mut *self.out,
                report: &mut *self.report,
            };
            let line = Line {
                text: &self.line[..close],
                start: text,
                embedded: false,
            };
            let (finished, _) = crate::html::lex_derivative_until(
                &mut cx,
                line,
                state,
                crate::html::Span::Capture,
                None,
            );
            self.facts.steps.take_back(finished);
            self.last = self.last.max(close);
        }
        let mut buffer = [0_u8; 40];
        let tag = self.line.as_bytes().get(name..name_end).unwrap_or_default();
        let parts: [&[u8]; 3] = [b"meta.tag.structure.", tag, b".end.html"];
        let mut len = 0;
        for part in parts {
            for &b in part {
                if let Some(slot) = buffer.get_mut(len) {
                    *slot = b;
                    len += 1;
                }
            }
        }
        let named = std::str::from_utf8(&buffer[..len])
            .ok()
            .and_then(|scope| self.arena.named_scope(scope));
        let tag_stack = named
            .and_then(|scope| self.arena.child(base, scope))
            .unwrap_or_else(|| {
                self.report.record(Limit::Nodes);
                base
            });
        self.emit(close, base);
        let begin = self.child(tag_stack, S::HtmlTagBegin);
        self.emit(name, begin);
        let name_stack = self.child(tag_stack, S::HtmlTagName);
        self.emit(name_end, name_stack);
        let end_stack = self.child(tag_stack, S::HtmlTagEnd);
        self.emit(name_end + 1, end_stack);
    }

    /// Whether a `vue` fence opens at the cursor inside a Vue body.
    /// Vue then leaves the line to [`Self::find`], whose fence rule opens it.
    ///
    /// Vue's `markdown-vue` grammar adds its fence as an injection.
    /// The injection applies in every rule of the document, Vue's own included.
    /// It wins over the rule's patterns that match at the same place.
    ///
    /// The new fence sits on the scopes of the innermost text.
    /// Inside a block that another language lexes, that language's scopes hold
    /// it. So the body's content is set to them here.
    /// The body has no content scope, so `check_whiles` sets it back to the
    /// body's name on the next line.
    #[cfg(feature = "lang-vue")]
    #[cold]
    #[inline(never)]
    fn vue_fence_ahead(&mut self) -> bool {
        let Some(body) = self.frames.len().checked_sub(1) else {
            return false;
        };
        let mut caps = Caps::new();
        let cx = cx!(self);
        if block::vue_fence_open(&cx, self.pos, &mut caps).is_none() {
            return false;
        }
        self.frames[body].content =
            crate::vue::innermost_stack(self.arena, self.frames[body].payload);
        true
    }

    /// Takes the rest of the line, or of the capture group, for the language
    /// of the open container, from the cursor or, when the containers took
    /// the line feed too, from the line's end. The next line resumes from
    /// this container with the embedded lexer's new state.
    ///
    /// html-derivative in a paragraph, a heading or emphasis gives the line
    /// back after its construct, which closes the container.
    ///
    /// A fence's `while` rule puts `\G` at the start of each of its lines.
    /// A `vue` fence has no `while` rule.
    /// Its lines have `\G` there only where `self.anchor` is
    /// (`vue_body_line`).
    ///
    /// It reads the four fields it needs from the top frame.
    /// It does not copy the frame.
    /// With a copy, one `Language` row's flags could change how `step`
    /// lowers it, since `step` inlines this function.
    /// Haskell's row turned it into a copy of the frame on the stack and a
    /// jump table.
    /// `step` then ran 2.1% more of its own instructions.
    fn embedded(&mut self) {
        let start = self.pos;
        let Some(&Frame {
            rule,
            content,
            payload,
            state: kept,
            ..
        }) = self.frames.last()
        else {
            return;
        };
        let (Rule::Open(kind), Some(lang)) = (rule, embedded_language(rule)) else {
            return;
        };
        #[cfg(feature = "lang-vue")]
        let vue = matches!(lang, Language::Vue);
        #[cfg(feature = "lang-vue")]
        if vue && self.vue_fence_ahead() {
            return;
        }
        let end = self.limit.min(self.line.len());
        if start >= end && !lang.reads_line_feed() {
            return;
        }
        let start = start.min(end);
        self.emit(start, content);
        #[cfg(feature = "lang-html-derivative")]
        let span = self.span();
        let lent = self.facts.steps.lend(self.limits);
        let mut cx = Context {
            scratch: &mut *self.scratch,
            arena: &mut *self.arena,
            limits: &lent,
            out: &mut *self.out,
            report: &mut *self.report,
        };
        #[cfg(feature = "lang-vue")]
        let anchored = !vue || self.anchor == Some(start);
        #[cfg(not(feature = "lang-vue"))]
        let anchored = true;
        let line = Line {
            text: &self.line[..end],
            start,
            embedded: anchored,
        };
        #[cfg(feature = "lang-html-derivative")]
        let (state, back) = match html_give_back(kind) {
            Some(give_back) => {
                crate::html::lex_derivative_until(&mut cx, line, payload, span, Some(give_back))
            }
            None => (lang.lex_embedded(&mut cx, line, payload), None),
        };
        #[cfg(feature = "lang-html-derivative")]
        let state = self.facts.steps.take_back(state);
        #[cfg(feature = "lang-html-derivative")]
        if let Some(at) = back {
            self.give_back_at(at, start, end, kind, content);
            return;
        }
        #[cfg(not(feature = "lang-html-derivative"))]
        let state = lang.lex_embedded(&mut cx, line, payload);
        #[cfg(not(feature = "lang-html-derivative"))]
        let state = self.facts.steps.take_back(state);
        self.last = end;
        self.pos = self.limit;
        if state == payload {
            return;
        }
        if kept.is_none() {
            if let Some(top) = self.frames.last_mut() {
                top.payload = state;
            }
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
            .push(below.unwrap_or(ROOT), kind.code(), state, content);
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

/// Whether a language's lexer takes the rest of each line inside `rule`.
/// It agrees with [`embedded_language`].
///
/// It reads only the rule's kind and builds no `Language`.
/// `step` tests it before every match.
/// Its lowering must not depend on the `Language` table.
/// With `embedded_language` there, one `Language` row's flags could change
/// that lowering.
/// Batch's row turned the bit mask into a copy of the frame and a jump table.
/// `step` then ran 2.2% more of its own instructions.
const fn embeds(rule: Rule) -> bool {
    match rule {
        Rule::Open(Kind::Body(lang)) => !matches!(lang, Language::Markdown),
        #[cfg(feature = "lang-html")]
        Rule::Open(Kind::BodyPhp) => true,
        #[cfg(feature = "lang-yaml")]
        Rule::Open(Kind::FrontMatterBody) => true,
        #[cfg(feature = "lang-html-derivative")]
        Rule::Open(Kind::HtmlRawBody | Kind::HtmlBlock | Kind::HtmlInline | Kind::HtmlWrap) => true,
        Rule::Open(_) | Rule::Capture(_) => false,
    }
}

/// The language whose lexer takes the rest of each line inside `rule`: an
/// embedded fence's, YAML in front matter, or html-derivative in HTML. A
/// `markdown` fence takes Markdown's own rules instead.
fn embedded_language(rule: Rule) -> Option<Language> {
    match rule {
        Rule::Open(Kind::Body(lang)) => (lang != Language::Markdown).then_some(lang),
        #[cfg(feature = "lang-html")]
        Rule::Open(Kind::BodyPhp) => Some(Language::Html),
        #[cfg(feature = "lang-yaml")]
        Rule::Open(Kind::FrontMatterBody) => Some(Language::Yaml),
        #[cfg(feature = "lang-html-derivative")]
        Rule::Open(Kind::HtmlRawBody | Kind::HtmlBlock | Kind::HtmlInline | Kind::HtmlWrap) => {
            Some(Language::HtmlDerivative)
        }
        Rule::Open(_) | Rule::Capture(_) => None,
    }
}

/// Where html-derivative gives the line back inside a container of `kind`:
/// none where it takes whole lines.
#[cfg(feature = "lang-html-derivative")]
fn html_give_back(kind: Kind) -> Option<crate::html::GiveBack<'static>> {
    match kind {
        Kind::HtmlInline => Some(crate::html::GiveBack::AtRoot),
        Kind::HtmlWrap => Some(crate::html::GiveBack::AfterTag),
        Kind::Base
        | Kind::Quote
        | Kind::ListBullet
        | Kind::ListOrdered
        | Kind::Fence(_)
        | Kind::FencePlain
        | Kind::FencePhp
        | Kind::Body(_)
        | Kind::BodyPhp
        | Kind::RawBlock
        | Kind::Table
        | Kind::Paragraph
        | Kind::Comment
        | Kind::BoldStar
        | Kind::BoldUnder
        | Kind::ItalicStar
        | Kind::ItalicUnder
        | Kind::FrontMatter
        | Kind::FrontMatterBody
        | Kind::HtmlRaw
        | Kind::HtmlRawBody
        | Kind::HtmlBlock => None,
    }
}

/// Whether the Vue body at `index` of `frames` goes on at `pos`.
/// When it does not, the fence's closing rule matches there.
/// It sets `anchor` to `pos` when `\G` is there.
///
/// - The body's content goes back to its name. A nested fence may have set it
///   to the scopes of the innermost text (`Lexer::vue_fence_ahead`).
/// - The fence's begin ends in a run of anything but a backtick or a tilde,
///   which takes the line feed. vscode-textmate then puts `\G` at the start of
///   a line that starts with the fence's rule on top, where Vue is outside
///   every rule. The `while` rules of the containers outside move it.
/// - With a container open inside the body, only the `while` rule of a Vue
///   script body can end it, and the containers inside close with it.
/// - Otherwise the fence's closing rule ends the body where Vue is outside
///   every rule.
///
/// It takes the fields it changes, not the lexer.
/// So `check_whiles`, which calls it in a loop, keeps the lexer's other fields
/// in registers around the call.
#[cfg(feature = "lang-vue")]
#[cold]
#[inline(never)]
fn vue_body_line(
    cx: &Cx<'_>,
    arena: &Arena,
    frames: &mut Vec<Frame>,
    anchor: &mut Option<usize>,
    index: usize,
    pos: usize,
) -> bool {
    let Some(body) = frames.get_mut(index) else {
        return true;
    };
    body.content = body.name;
    let payload = body.payload;
    let anchored = frames.last().is_some_and(|top| {
        matches!(top.rule, Rule::Open(Kind::Body(Language::Vue)))
            && crate::vue::at_root(arena, top.payload)
    });
    if anchored && anchor.is_none() {
        *anchor = Some(pos);
    }
    if index + 1 < frames.len() {
        if !crate::vue::while_holds(arena, payload, cx.line, pos) {
            frames.truncate(index + 1);
        }
        return true;
    }
    if !crate::vue::at_root(arena, payload) {
        return true;
    }
    let fence = index
        .checked_sub(1)
        .and_then(|outer| frames.get(outer))
        .map_or(0, |outer| outer.payload);
    let mut caps = Caps::new();
    block::fence_close(cx, pos, Fence::from_payload(fence), &mut caps).is_none()
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
/// It walks the frames into `chain`, the highlighter's buffer, so a document
/// lexed again allocates nothing on its first line.
fn after_first_line(arena: &mut Arena, chain: &mut Vec<ArenaFrame>, state: u32) -> Option<u32> {
    chain.clear();
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

#[cfg(test)]
mod tests {
    use super::{Kind, Language, List, Rule, embedded_language, embeds};

    /// `step`'s guard and `embedded` agree on every rule a frame can hold:
    /// each kind, a fence and a body for every language in the build, and a
    /// capture group.
    #[test]
    fn embeds_agrees_with_embedded_language() {
        let languages = Language::ALL
            .iter()
            .flat_map(|&lang| [Kind::Fence(lang), Kind::Body(lang)]);
        let rules = Kind::FIXED
            .iter()
            .copied()
            .chain(languages)
            .map(Rule::Open)
            .chain([Rule::Capture(List::Inline)]);
        for rule in rules {
            assert_eq!(embeds(rule), embedded_language(rule).is_some(), "{rule:?}");
        }
    }
}
