//! The per-line lexer: the `TextMate` engine's loop over the rules of the
//! Markdown grammar, with the regular expressions replaced by the matchers in
//! [`block`] and [`inline`].
//!
//! A line starts by checking the `while` rule of each open container.
//! That rule decides whether the container continues. The first one that
//! fails closes itself and every container inside it.
//!
//! Then the rules of the innermost container, and its closing rule, are tried
//! at each position, from the cursor on. The match that starts first wins, and
//! among matches that start together the rule listed first. A match opens a
//! container, closes one, or only scopes text.

use super::captures::{Captures, NESTING};
#[cfg(feature = "lang-html-derivative")]
use super::html;
use super::inline::Emphasis;
#[cfg(feature = "lang-objective-c")]
use super::root::{Definition, RootPatterns, Slot};
use super::rules::{self, Capture, End, List, Pattern};
use super::runs::Runs;
use super::scope::S;
use super::state::{Container, Fence};
use super::text::{Cx, LineFacts};
use super::{block, inline};
use crate::arena::{Arena, Frame as ArenaFrame, ROOT};
#[cfg(feature = "lang-vue")]
use crate::language::Halt;
use crate::language::{Context, Line, Parts, StopReason, Stopped};
use crate::limits::{Finished, Limit, Limits, LineReport, Steps};
use crate::{Language, Stack, Token};

/// The window on the line that the rules see, built from the fields it needs.
/// A rule can then borrow other fields, such as the runs, while it is held.
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
    Open(Container),
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
    carried: u32,
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
    fn of(container: Container) -> Starts {
        if rules::injects_vue_fence(container) {
            return Starts::Anchored;
        }
        match container {
            Container::Base | Container::Quote | Container::ListBullet | Container::ListOrdered => {
                Starts::Block
            }
            Container::Body(language) => {
                if language == Language::Markdown {
                    Starts::Block
                } else {
                    Starts::Nowhere
                }
            }
            Container::Fence(_) | Container::FencePlain | Container::FrontMatter => {
                Starts::Anchored
            }
            #[cfg(feature = "lang-html")]
            Container::FencePhp => Starts::Anchored,
            #[cfg(feature = "lang-html")]
            Container::BodyPhp => Starts::Nowhere,
            Container::FrontMatterBody
            | Container::RawBlock
            | Container::HtmlRawBody
            | Container::HtmlBlock
            | Container::HtmlInline
            | Container::HtmlWrap => Starts::Nowhere,
            Container::Comment => Starts::Dash,
            Container::HtmlRaw | Container::Table => Starts::Anywhere,
            Container::Paragraph
            | Container::BoldStar
            | Container::BoldUnder
            | Container::ItalicStar
            | Container::ItalicUnder => Starts::Inline,
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
    rule: Option<Pattern>,
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
    /// The end of the text the rules see.
    /// While lexing the line, it is one past the line, which stands for its
    /// newline. While lexing a capture group, it is the end of the group.
    limit: usize,
    /// The end of the last token emitted.
    last: usize,
    /// Steps spent on this line, with the character before the line's start.
    /// The steps count the matchers' scans too, and are lent to the languages
    /// of its fences.
    facts: LineFacts,
    /// The fence the last match of the fence rule opened.
    opened: Option<block::FenceOpen>,
    /// What the highlighter keeps from line to line.
    scratch: &'a mut crate::scratch::Scratch,
    /// Whether this is the first line of the document, where `\A` matches.
    /// The document's own state is its root with a carried value of 1.
    /// After the first line the root has none, so `\A` matches on one line
    /// only.
    first_line: bool,
}

/// What the Markdown lexer keeps from one line to the next.
/// A line then allocates nothing and does not look up what the last line
/// interned.
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
    /// The root pattern that took the line at a `$base` slot of an
    /// Objective-C fence.
    /// Only [`lex_with_root`] sets it, for the hand-back that follows.
    #[cfg(feature = "lang-objective-c")]
    slot: Option<Slot>,
    /// The match of the last link definition that took a slot.
    /// It is boxed, since a link definition takes a slot rarely and the
    /// scratch of every language holds this one.
    /// The first such line allocates the box, and later ones overwrite it,
    /// so a scratch allocates for this once in its life.
    /// It is read only where `slot` names a link definition, which
    /// [`lex_with_root`] writes it for.
    #[cfg(feature = "lang-objective-c")]
    definition: Option<Box<Definition>>,
    /// Whether a fence's language stopped where a `vue` fence opens.
    /// Only [`lex_to_vue_fence`] sets it, for the hand-back that follows.
    #[cfg(feature = "lang-vue")]
    vue_fence: bool,
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
                let container = Container::from_code(frame.rule);
                let name = if rules::content_name(container).is_empty() {
                    frame.stack
                } else {
                    arena.frame(frame.parent).stack
                };
                frames.push(Frame {
                    rule: Rule::Open(container),
                    name,
                    content: frame.stack,
                    saved_anchor: None,
                    carried: frame.carried,
                    state: Some(at),
                });
                at = frame.parent;
            }
            frames.reverse();
        }
        let first_line = frames.first().is_some_and(|root| {
            matches!(root.rule, Rule::Open(Container::Base)) && root.carried == 1
        });
        let before_start = match scratch.markdown.start {
            Start::Document => None,
            Start::Embedded { before, .. } => before,
        };
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
                before_start,
            },
            opened: None,
            scratch,
            first_line,
        }
    }

    /// The state after the line, with its containers kept for the next one.
    ///
    /// After the first line the containers rest on the document's start
    /// state. They therefore move onto a root that is not the start, and the
    /// next line rebuilds them. The line's count goes back to the lender.
    /// The frames this line opened lose their saved anchors, which belong
    /// to this line.
    ///
    /// At sixteen codegen units, out of line, it took the lexer by a libc copy.
    /// The copy cost 815k instructions on the Markdown fixtures.
    /// Inlined, Markdown ran 1.6% fewer instructions.
    /// At one codegen unit, the count did not change.
    #[expect(
        clippy::inline_always,
        reason = "out of line at sixteen codegen units, it copied the lexer through libc"
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
    /// limit. It does so though the scan that ran out gave up without a match
    /// and the loop then found nothing more.
    ///
    /// A line that another language embeds can stop at its start. It stops
    /// there when its [`Start`] marks a stop and the open containers' `while`
    /// rules closed them all.
    /// That is the one place the embedding rule's end can hold.
    ///
    /// At the document's level every line opens a block at its start, and
    /// inside a block the embedding rule's end is not tried.
    ///
    /// [`Lexer::new`] reads the character before an embedded start, so the
    /// `while` rules see it.
    /// A start with a character before it is not the line's start, so a bare
    /// `^` fails there ([`Cx::at_line_start`]).
    pub(crate) fn run(&mut self) {
        let mut captures = Captures::new();
        self.check_whiles(&mut captures);
        if let Start::Embedded { stop: true, .. } = self.scratch.markdown.start
            && self.frames.len() == 1
        {
            self.scratch.markdown.stopped = true;
            return;
        }
        while self.step(&mut captures) {}
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
    ///
    /// Out of line, it cost a call per token.
    #[expect(
        clippy::inline_always,
        reason = "emit went out of line when step changed size at one codegen unit"
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
    fn check_whiles(&mut self, captures: &mut Captures) {
        let mut index = 0;
        while index < self.frames.len() {
            let Rule::Open(container) = self.frames[index].rule else {
                break;
            };
            captures.clear();
            let cx = cx!(self);
            let pos = self.pos;
            let end = match container {
                Container::Quote => block::quote_while(&cx, pos, captures),
                Container::ListBullet | Container::ListOrdered => block::list_while(&cx, pos),
                Container::FrontMatterBody => {
                    let dashes = index
                        .checked_sub(1)
                        .and_then(|outer| self.frames.get(outer))
                        .map_or(3, |outer| usize::try_from(outer.carried).unwrap_or(3));
                    let mut captures = Captures::new();
                    block::front_matter_close(&cx, pos, dashes, &mut captures)
                        .is_none()
                        .then_some(pos)
                }
                #[cfg(feature = "lang-vue")]
                Container::Body(Language::Vue) => {
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
                Container::Body(_) => {
                    (cx.at_start(pos) && !block::is_fence_line(&cx, pos)).then_some(pos)
                }
                #[cfg(feature = "lang-html")]
                Container::BodyPhp => {
                    (cx.at_start(pos) && !block::is_fence_line(&cx, pos)).then_some(pos)
                }
                Container::RawBlock => block::raw_block(&cx, pos),
                Container::Table => block::table_while(&cx, pos).then_some(pos),
                Container::Paragraph => block::paragraph_while(&cx, pos),
                #[cfg(feature = "lang-html-derivative")]
                Container::HtmlRawBody => html::raw_while(&cx, pos),
                #[cfg(feature = "lang-html-derivative")]
                Container::HtmlBlock => html::block_while(&cx, pos),
                #[cfg(not(feature = "lang-html-derivative"))]
                Container::HtmlRawBody | Container::HtmlBlock => None,
                #[cfg(feature = "lang-html")]
                Container::FencePhp => {
                    index += 1;
                    continue;
                }
                Container::Base
                | Container::Fence(_)
                | Container::FencePlain
                | Container::FrontMatter
                | Container::Comment
                | Container::BoldStar
                | Container::BoldUnder
                | Container::ItalicStar
                | Container::ItalicUnder
                | Container::HtmlRaw
                | Container::HtmlInline
                | Container::HtmlWrap => {
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
            self.captures(node, end, captures, |group| {
                rules::capture(Pattern::Quote, group)
            });
            self.emit(end, node);
            self.pos = self.pos.max(end);
            self.anchor = Some(end);
            index += 1;
        }
    }

    // The loop.

    /// Takes the next match from the cursor. False when nothing matches, or
    /// the line has spent its work.
    fn step(&mut self, captures: &mut Captures) -> bool {
        if self.facts.steps.step() {
            self.report.record(Limit::Work);
            return false;
        }
        let from = (self.pos, self.frames.len());
        if self.frames.last().is_some_and(|frame| embeds(frame.rule)) {
            self.embedded();
        }
        let Some(hit) = self.find(captures) else {
            return false;
        };
        self.apply(&hit, captures);
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
    /// In inline text it skips to the next byte that opens an inline rule.
    /// The first byte of the line may also start a setext underline.
    fn find(&mut self, captures: &mut Captures) -> Option<Hit> {
        let (end, list, starts) = match self.frames.last() {
            None => (None, rules::patterns(Container::Base), Starts::Block),
            Some(frame) => match frame.rule {
                Rule::Open(container) => (
                    rules::end(container).map(|end| (end, container)),
                    rules::patterns(container),
                    Starts::of(container),
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
            if let Some((end, container)) = end
                && let Some(hit) = self.try_end(end, container, start, captures)
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
                        self.try_rule(rule, start, captures).is_none(),
                        "{rule:?} matches at byte {start} outside the bytes it opens with"
                    );
                    continue;
                }
                if let Some(hit) = self.try_rule(rule, start, captures) {
                    return Some(hit);
                }
            }
            start += 1;
        }
        None
    }

    /// The match of the open container's closing rule at `start`.
    ///
    /// Front matter's closing rule applies last, so on the first line that
    /// it would close, the front matter begins instead.
    fn try_end(
        &mut self,
        end: End,
        container: Container,
        start: usize,
        captures: &mut Captures,
    ) -> Option<Hit> {
        captures.clear();
        let cx = cx!(self);
        let end_pos = match end {
            End::Fence => {
                let carried = self.frames.last().map_or(0, |frame| frame.carried);
                block::fence_close(&cx, start, Fence::from_carried(carried), captures)
            }
            End::FrontMatter => {
                if self.first_line {
                    return None;
                }
                let carried = self.frames.last().map_or(3, |frame| frame.carried);
                let dashes = usize::try_from(carried).unwrap_or(3);
                block::front_matter_close(&cx, start, dashes, captures)
            }
            End::Comment => block::comment_close(&cx, start, captures),
            #[cfg(feature = "lang-html-derivative")]
            End::HtmlRaw => html::raw_close(&cx, start, captures),
            End::Bold => {
                let mark = if container == Container::BoldStar {
                    b'*'
                } else {
                    b'_'
                };
                inline::bold_close(&cx, start, mark, captures)
            }
            End::Italic => {
                let mark = if container == Container::ItalicStar {
                    b'*'
                } else {
                    b'_'
                };
                inline::italic_close(&cx, start, mark, captures)
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
    fn try_rule(&mut self, rule: Pattern, start: usize, captures: &mut Captures) -> Option<Hit> {
        captures.clear();
        #[cfg(feature = "lang-html-derivative")]
        if rule == Pattern::Html {
            return self.html_opens(start).then_some(Hit {
                rule: Some(rule),
                start,
                end: start,
            });
        }
        let cx = cx!(self);
        let mut rule = rule;
        let end = match rule {
            Pattern::FrontMatter => {
                if !self.first_line || self.frames.len() != 1 {
                    return None;
                }
                block::front_matter_open(&cx, start)
            }
            Pattern::FrontMatterBegin => block::front_matter_begin(&cx, start, captures),
            Pattern::Separator => block::separator(&cx, start),
            Pattern::Heading => block::heading(&cx, start, captures),
            Pattern::Quote => block::quote(&cx, start, captures),
            Pattern::Bullet => block::bullet(&cx, start, captures),
            Pattern::Ordered => block::ordered(&cx, start, captures),
            Pattern::Fence | Pattern::VueFence => {
                let open = if rule == Pattern::VueFence {
                    block::injected_fence_open(&cx, start, captures)
                } else {
                    block::fence_open(&cx, start, captures)
                }?;
                let end = open.end;
                rule = if open.embed.is_none() {
                    Pattern::FencePlain
                } else {
                    Pattern::FenceLanguage
                };
                self.opened = Some(open);
                Some(end)
            }
            Pattern::RawBlock => block::raw_block(&cx, start),
            Pattern::LinkDefinition => {
                if start > self.pos && cx.space_before(start) > 0 {
                    return None;
                }
                block::link_definition(&cx, start, captures)
            }
            Pattern::Comment => block::comment_open(&cx, start, captures),
            Pattern::Table => block::table(&cx, start, captures),
            Pattern::Paragraph => block::paragraph(&cx, start),
            Pattern::EmbedBegin => embed_begin(&cx, start),
            Pattern::TablePipe => (cx.get(start) == Some(b'|')).then_some(start + 1),
            Pattern::TableSeparator => block::table_separator(&cx, start, captures),
            Pattern::TableCell => block::table_cell(&cx, start, captures),
            Pattern::Setext1 => block::setext(&cx, start, b'='),
            Pattern::Setext2 => block::setext(&cx, start, b'-'),
            Pattern::Level1 => block::heading_part(&cx, start, 1, captures),
            Pattern::Level2 => block::heading_part(&cx, start, 2, captures),
            Pattern::Level3 => block::heading_part(&cx, start, 3, captures),
            Pattern::Level4 => block::heading_part(&cx, start, 4, captures),
            Pattern::Level5 => block::heading_part(&cx, start, 5, captures),
            Pattern::Level6 => block::heading_part(&cx, start, 6, captures),
            Pattern::Ampersand => inline::ampersand(&cx, start),
            Pattern::Bracket => inline::bracket(&cx, start),
            Pattern::Bold => inline::emphasis_open(
                &cx,
                &mut self.scratch.markdown.runs,
                start,
                Emphasis::Bold,
                captures,
            ),
            Pattern::Italic => inline::emphasis_open(
                &cx,
                &mut self.scratch.markdown.runs,
                start,
                Emphasis::Italic,
                captures,
            ),
            Pattern::Raw => inline::raw(&cx, &mut self.scratch.markdown.runs, start, captures),
            Pattern::Strike => {
                inline::strikethrough(&cx, &mut self.scratch.markdown.runs, start, captures)
            }
            Pattern::Escape => inline::escape(&cx, start),
            Pattern::ImageInline => {
                inline::image_inline(&cx, &mut self.scratch.markdown.runs, start, captures)
            }
            Pattern::ImageReference => inline::image_reference(&cx, start, captures),
            Pattern::LinkEmail => inline::link_email(&cx, start, captures),
            Pattern::LinkInternet => inline::link_internet(&cx, start, captures),
            Pattern::LinkInline => {
                inline::link_inline(&cx, &mut self.scratch.markdown.runs, start, captures)
            }
            Pattern::LinkReference => inline::link_reference(&cx, start, captures),
            Pattern::LinkReferenceLiteral => inline::link_reference_literal(&cx, start, captures),
            Pattern::LinkReferenceShortcut => inline::link_reference_shortcut(&cx, start, captures),
            Pattern::FenceLanguage | Pattern::FencePlain => None,
            #[cfg(feature = "lang-html-derivative")]
            Pattern::HtmlRaw => html::raw_open(&cx, start),
            #[cfg(feature = "lang-html-derivative")]
            Pattern::HtmlRawBody => Some(cx.skip_space(start)),
            #[cfg(feature = "lang-html-derivative")]
            Pattern::HtmlBlock => html::block_open(&cx, start),
            #[cfg(feature = "lang-html-derivative")]
            Pattern::HtmlWrap => html::wrap_open(&cx, start),
            #[cfg(feature = "lang-html-derivative")]
            Pattern::Html => None,
        }?;
        Some(Hit {
            rule: Some(rule),
            start,
            end,
        })
    }

    // Matches.

    /// Applies a match: closes, opens or scopes.
    /// `captures` holds the match's capture groups.
    /// The rules that a group hands text to fill groups of their own, in
    /// [`Lexer::subrun`], so `captures` stays as the match left it.
    fn apply(&mut self, hit: &Hit, captures: &Captures) {
        match hit.rule {
            None => self.close(hit, captures),
            Some(rule) => match self.opens(rule, hit.start) {
                Some(container) => self.open(container, rule, hit, captures),
                None => self.scope_match(rule, hit, captures),
            },
        }
    }

    /// The container a rule opens, or none for a rule that only scopes text.
    fn opens(&self, rule: Pattern, start: usize) -> Option<Container> {
        let byte = self.line.as_bytes().get(start).copied();
        Some(match rule {
            Pattern::FrontMatter => Container::FrontMatter,
            Pattern::FrontMatterBegin => Container::FrontMatterBody,
            Pattern::Quote => Container::Quote,
            Pattern::Bullet => Container::ListBullet,
            Pattern::Ordered => Container::ListOrdered,
            Pattern::FenceLanguage => self.opened.as_ref().and_then(|open| open.embed)?.fence(),
            Pattern::FencePlain => Container::FencePlain,
            Pattern::EmbedBegin => match self.frames.last()?.rule {
                Rule::Open(Container::Fence(lang)) => Container::Body(lang),
                #[cfg(feature = "lang-html")]
                Rule::Open(Container::FencePhp) => Container::BodyPhp,
                Rule::Open(_) | Rule::Capture(_) => return None,
            },
            Pattern::RawBlock => Container::RawBlock,
            Pattern::Comment => Container::Comment,
            Pattern::Table => Container::Table,
            Pattern::Paragraph => Container::Paragraph,
            Pattern::Bold if byte == Some(b'*') => Container::BoldStar,
            Pattern::Bold => Container::BoldUnder,
            Pattern::Italic if byte == Some(b'*') => Container::ItalicStar,
            Pattern::Italic => Container::ItalicUnder,
            #[cfg(feature = "lang-html-derivative")]
            Pattern::HtmlRaw => Container::HtmlRaw,
            #[cfg(feature = "lang-html-derivative")]
            Pattern::HtmlRawBody => Container::HtmlRawBody,
            #[cfg(feature = "lang-html-derivative")]
            Pattern::HtmlBlock => Container::HtmlBlock,
            #[cfg(feature = "lang-html-derivative")]
            Pattern::Html => Container::HtmlInline,
            #[cfg(feature = "lang-html-derivative")]
            Pattern::HtmlWrap => Container::HtmlWrap,
            Pattern::Separator
            | Pattern::Heading
            | Pattern::Fence
            | Pattern::VueFence
            | Pattern::LinkDefinition
            | Pattern::TablePipe
            | Pattern::TableSeparator
            | Pattern::TableCell
            | Pattern::Setext1
            | Pattern::Setext2
            | Pattern::Level1
            | Pattern::Level2
            | Pattern::Level3
            | Pattern::Level4
            | Pattern::Level5
            | Pattern::Level6
            | Pattern::Ampersand
            | Pattern::Bracket
            | Pattern::Raw
            | Pattern::Strike
            | Pattern::Escape
            | Pattern::ImageInline
            | Pattern::ImageReference
            | Pattern::LinkEmail
            | Pattern::LinkInternet
            | Pattern::LinkInline
            | Pattern::LinkReference
            | Pattern::LinkReferenceLiteral
            | Pattern::LinkReferenceShortcut => return None,
        })
    }

    /// Whether the depth limit keeps another frame from opening above the
    /// document's own.
    fn at_depth_limit(&self) -> bool {
        self.limits
            .depth_reached(self.frames.len().saturating_sub(1))
    }

    /// Keeps a container that cannot open out of the tree.
    ///
    /// Its opening text stays in the container that is open. So does the rest
    /// of the line when it opens with no text. A refusal with no text ends
    /// the window, which moves the cursor past it. Otherwise the rule would
    /// match again at its end and be refused again without progress.
    fn refuse(&mut self, hit: &Hit, parent: u32) {
        if hit.end > hit.start {
            self.emit(hit.end, parent);
            self.pos = hit.end;
        } else {
            self.emit(self.limit, parent);
            self.pos = self.limit + 1;
        }
    }

    /// Opens `container` for a match of `rule`.
    ///
    /// A container opened outside a capture group stays open between lines,
    /// so it is a state. A paragraph opens on nearly every line in the same
    /// state, so the last one opened is reused.
    fn open(&mut self, container: Container, rule: Pattern, hit: &Hit, captures: &Captures) {
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
        let cached = match (container, parent_state, self.scratch.markdown.paragraph) {
            (Container::Paragraph, Some(from), Some((was, stack, state))) if from == was => {
                Some((stack, state))
            }
            _ => None,
        };
        let (name, content, carried, state) = if let Some((stack, state)) = cached {
            (stack, stack, 0, Some(state))
        } else {
            let name = self.path(parent, rules::names(container));
            let content = self.path(name, rules::content_name(container));
            let carried = self.carried(container, opened, content);
            let state = if in_capture {
                None
            } else {
                let Some(state) = self.arena.push(
                    parent_state.unwrap_or(ROOT),
                    container.code(),
                    carried,
                    content,
                ) else {
                    self.report.record(Limit::Nodes);
                    self.refuse(hit, parent);
                    return;
                };
                if let (Container::Paragraph, Some(from)) = (container, parent_state) {
                    self.scratch.markdown.paragraph = Some((from, content, state));
                }
                Some(state)
            };
            (name, content, carried, state)
        };
        self.anchored = self.anchored.min(self.frames.len());
        self.frames.push(Frame {
            rule: Rule::Open(container),
            name,
            content: name,
            saved_anchor: self.anchor,
            carried,
            state,
        });
        self.captures(name, hit.end, captures, |group| rules::capture(rule, group));
        self.emit(hit.end, name);
        if let Some(frame) = self.frames.last_mut() {
            frame.content = content;
        }
        self.anchor = Some(hit.end);
        self.pos = self.pos.max(hit.end);
    }

    /// The carried value of a frame that opens `container`, whose content
    /// scopes are `content`.
    ///
    /// A fence keeps what its opening line held in `opened`.
    /// A body that another language lexes keeps that language's first state.
    /// Front matter keeps the number of its dashes.
    fn carried(
        &mut self,
        container: Container,
        opened: Option<block::FenceOpen>,
        content: u32,
    ) -> u32 {
        match container {
            Container::Fence(_) | Container::FencePlain => {
                opened.map_or(0, |open| open.fence.carried())
            }
            Container::Body(lang) if lang != Language::Markdown => {
                lang.start_embedded(self.arena, content)
            }
            #[cfg(feature = "lang-html")]
            Container::FencePhp => opened.map_or(0, |open| open.fence.carried()),
            #[cfg(feature = "lang-html")]
            Container::BodyPhp => Language::Html.start_embedded(self.arena, content),
            #[cfg(feature = "lang-yaml")]
            Container::FrontMatterBody => Language::Yaml.start_embedded(self.arena, content),
            #[cfg(not(feature = "lang-yaml"))]
            Container::FrontMatterBody => 0,
            #[cfg(feature = "lang-html-derivative")]
            Container::HtmlRawBody
            | Container::HtmlBlock
            | Container::HtmlInline
            | Container::HtmlWrap => Language::HtmlDerivative.start_embedded(self.arena, content),
            #[cfg(not(feature = "lang-html-derivative"))]
            Container::HtmlRawBody
            | Container::HtmlBlock
            | Container::HtmlInline
            | Container::HtmlWrap => 0,
            Container::FrontMatter => {
                let dashes = self.line.bytes().take_while(|&b| b == b'-').count();
                u32::try_from(dashes).unwrap_or(u32::MAX)
            }
            Container::Base
            | Container::Quote
            | Container::ListBullet
            | Container::ListOrdered
            | Container::Body(_)
            | Container::RawBlock
            | Container::Table
            | Container::Paragraph
            | Container::Comment
            | Container::BoldStar
            | Container::BoldUnder
            | Container::ItalicStar
            | Container::ItalicUnder
            | Container::HtmlRaw => 0,
        }
    }

    fn close(&mut self, hit: &Hit, captures: &Captures) {
        let Some(frame) = self.frames.last().copied() else {
            return;
        };
        let Rule::Open(container) = frame.rule else {
            return;
        };
        self.emit(hit.start, frame.content);
        if let Some(end) = rules::end(container) {
            self.end_captures(end, frame.name, hit.end, captures);
        }
        self.emit(hit.end, frame.name);
        self.frames.pop();
        self.anchor = frame.saved_anchor;
        self.pos = self.pos.max(hit.end);
    }

    /// The capture groups of `end`'s match, which ends at `match_end`.
    fn end_captures(&mut self, end: End, base: u32, match_end: usize, captures: &Captures) {
        match end {
            #[cfg(feature = "lang-html-derivative")]
            End::HtmlRaw => self.html_raw_end(base, captures),
            End::FrontMatter | End::Fence | End::Comment | End::Bold | End::Italic => {
                self.captures(base, match_end, captures, |group| {
                    rules::end_capture(end, group)
                });
            }
        }
    }

    fn scope_match(&mut self, rule: Pattern, hit: &Hit, captures: &Captures) {
        let parent = self.content();
        self.emit(hit.start, parent);
        let node = match rules::match_name(rule) {
            Some(scope) => self.child(parent, scope),
            None => parent,
        };
        self.captures(node, hit.end, captures, |group| rules::capture(rule, group));
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
        captures: &Captures,
        capture: impl Fn(usize) -> Capture,
    ) {
        let mut open = [(0_u32, 0_usize); NESTING];
        let mut depth = 0;
        for (group, (start, end)) in captures.iter() {
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
            carried: 0,
            state: None,
        });
        let floor = self.frames.len();
        self.limit = end;
        self.pos = start;
        self.anchor = None;
        let mut captures = Captures::new();
        while self.step(&mut captures) {}
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
        let mut cx = Context::new(
            &mut *self.scratch,
            &mut *self.arena,
            &lent,
            &mut *self.out,
            &mut *self.report,
        )
        .with_first_token(self.first_token);
        let line = Line::new(text, start).with_anchor(true);
        let (opens, spent) = crate::html::derivative_opens_at(&mut cx, line, state, span);
        self.facts.steps.add(spent);
        opens
    }

    /// Gives the line back from html-derivative, which gave it back at `at`,
    /// closing its container. A wrapper that closes where it opened would
    /// open again there, so, as in `TextMate`, it stays open and takes the
    /// rest of the text.
    #[cfg(feature = "lang-html-derivative")]
    fn give_back_at(
        &mut self,
        at: usize,
        start: usize,
        end: usize,
        container: Container,
        content: u32,
    ) {
        self.last = self.last.max(at);
        if at == start && matches!(container, Container::HtmlWrap) {
            self.emit(end, content);
            self.pos = self.limit + 1;
            return;
        }
        if let Some(frame) = self.frames.pop() {
            self.anchor = frame.saved_anchor;
        }
        self.pos = at;
    }

    /// The closing match of a raw HTML block.
    ///
    /// The text before the closing tag goes to html-derivative from its
    /// start, as a capture group. The tag is named after its own spelling, as
    /// `$4` names it.
    #[cfg(feature = "lang-html-derivative")]
    fn html_raw_end(&mut self, base: u32, captures: &Captures) {
        let mut groups = [(0, 0); 6];
        for (group, range) in captures.iter() {
            if let Some(slot) = groups.get_mut(group) {
                *slot = range;
            }
        }
        let [_, (text, close), _, _, (name, name_end), _] = groups;
        if text < close {
            let state = crate::html::start_embedded(self.arena, base);
            let lent = self.facts.steps.lend(self.limits);
            let mut cx = Context::new(
                &mut *self.scratch,
                &mut *self.arena,
                &lent,
                &mut *self.out,
                &mut *self.report,
            )
            .with_first_token(self.first_token);
            let line = Line::new(&self.line[..close], text);
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

    /// Whether a fence run may start at `p`, where `^` or `\G` holds.
    /// Only there can [`Self::vue_fence_ahead`] find a `vue` fence.
    #[cfg(feature = "lang-vue")]
    #[inline]
    fn vue_fence_may_open(&self, p: usize) -> bool {
        let cx = cx!(self);
        cx.at_start(p) && matches!(cx.get(cx.skip_space(p)), Some(b'`' | b'~'))
    }

    /// Whether a `vue` fence opens at the cursor inside the body of a fence
    /// in `lang`.
    /// The body then leaves the line to [`Self::find`], whose fence rule opens
    /// it.
    ///
    /// Vue's `markdown-vue` grammar adds its fence as an injection.
    /// The injection applies in every rule of the document.
    /// That covers each embedded language's rules, Vue's own included.
    /// It wins over the rule's patterns that match at the same place.
    ///
    /// The new fence sits on the scopes of the innermost text.
    /// Inside a block that another language lexes, that language's scopes hold
    /// it.
    /// So the body's content is set to them here.
    ///
    /// A Vue body has no content scope.
    /// So `check_whiles` sets it back to the body's name on the next line.
    /// Any other body's `while` rule fails at every fence line.
    /// So the new fence closes only with the body, and the body keeps no
    /// content it was set to.
    #[cfg(feature = "lang-vue")]
    #[cold]
    #[inline(never)]
    fn vue_fence_ahead(&mut self, lang: Language) -> bool {
        let Some(body) = self.frames.len().checked_sub(1) else {
            return false;
        };
        let mut captures = Captures::new();
        let cx = cx!(self);
        if block::vue_fence_open(&cx, self.pos, &mut captures).is_none() {
            return false;
        }
        let state = self.frames[body].carried;
        self.frames[body].content = match lang {
            Language::Vue => crate::vue::innermost_stack(self.arena, state),
            #[cfg(feature = "lang-html")]
            Language::Html => crate::html::innermost_stack(self.arena, state),
            #[cfg(feature = "lang-html-derivative")]
            Language::HtmlDerivative if state != ROOT => {
                crate::html::innermost_stack(self.arena, state)
            }
            _ if state == ROOT => self.frames[body].content,
            _ => self.arena.frame(state).stack,
        };
        true
    }

    /// Takes the rest of the line, or of the capture group, for the language
    /// of the open container.
    ///
    /// It takes it from the cursor, or from the line's end when the
    /// containers took the line feed too. The next line resumes from this
    /// container with the embedded lexer's new state.
    ///
    /// html-derivative in a paragraph, a heading or emphasis gives the line
    /// back after its construct, which closes the container.
    ///
    /// A fence's `while` rule puts `\G` at the start of every fence line.
    /// A `vue` fence has no `while` rule.
    /// Its lines have `\G` there only where `self.anchor` is
    /// (`vue_body_line`).
    ///
    /// It reads the four fields it needs from the top frame.
    /// It does not copy the frame.
    /// With a copy, one `Language` row's flags could change how `step`
    /// lowers it, since `step` inlines this function.
    /// Haskell's row turned it into a copy of the stack frame and a
    /// jump table.
    /// `step` then ran 2.1% more of its own instructions.
    ///
    /// [`Lexer::open_at_slot`] calls it too, out of line.
    /// The attribute keeps `step` inlining it all the same.
    #[expect(
        clippy::inline_always,
        reason = "a second call site must not move it out of `step`"
    )]
    #[inline(always)]
    fn embedded(&mut self) {
        let start = self.pos;
        let Some(&Frame {
            rule,
            content,
            carried,
            state: kept,
            ..
        }) = self.frames.last()
        else {
            return;
        };
        let (Rule::Open(container), Some(lang)) = (rule, embedded_language(rule)) else {
            return;
        };
        #[cfg(feature = "lang-vue")]
        let vue = matches!(lang, Language::Vue);
        #[cfg(feature = "lang-vue")]
        if rules::injects_vue_fence(container)
            && self.vue_fence_may_open(start)
            && self.vue_fence_ahead(lang)
        {
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
        let mut cx = Context::new(
            &mut *self.scratch,
            &mut *self.arena,
            &lent,
            &mut *self.out,
            &mut *self.report,
        )
        .with_first_token(self.first_token);
        #[cfg(feature = "lang-vue")]
        let anchored = !vue || self.anchor == Some(start);
        #[cfg(not(feature = "lang-vue"))]
        let anchored = true;
        let line = Line::new(&self.line[..end], start).with_anchor(anchored);
        #[cfg(feature = "lang-html-derivative")]
        let ((state, popped), back) = match html_give_back(container) {
            Some(give_back) => {
                let (state, back) = crate::html::lex_derivative_until(
                    &mut cx,
                    line,
                    carried,
                    span,
                    Some(give_back),
                );
                ((state, None), back)
            }
            None => (lex_fence(&mut cx, lang, line, carried, &self.facts), None),
        };
        #[cfg(feature = "lang-html-derivative")]
        let state = self.facts.steps.take_back(state);
        #[cfg(feature = "lang-html-derivative")]
        if let Some(at) = back {
            self.give_back_at(at, start, end, container, content);
            return;
        }
        #[cfg(not(feature = "lang-html-derivative"))]
        let (state, popped) = lex_fence(&mut cx, lang, line, carried, &self.facts);
        #[cfg(not(feature = "lang-html-derivative"))]
        let state = self.facts.steps.take_back(state);
        if let Some(at) = popped {
            self.hand_back(at, end, state);
            return;
        }
        self.last = end;
        self.pos = self.limit;
        self.keep_body_state(container, content, carried, kept, state);
    }

    /// Keeps `state`, the state the body of `container` reached, in the body's
    /// frame.
    /// It goes in the frame's carried value, and in a state of its own where
    /// the frame keeps one (`kept`).
    ///
    /// Out of line, it cost Markdown 0.2% in calls once a fence line.
    #[expect(
        clippy::inline_always,
        reason = "with `#[inline]`, it stayed out of `step` at one codegen unit"
    )]
    #[inline(always)]
    fn keep_body_state(
        &mut self,
        container: Container,
        content: u32,
        carried: u32,
        kept: Option<u32>,
        state: u32,
    ) {
        if state == carried {
            return;
        }
        if kept.is_none() {
            if let Some(top) = self.frames.last_mut() {
                top.carried = state;
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
            .push(below.unwrap_or(ROOT), container.code(), state, content);
        let Some(pushed) = pushed else {
            self.report.record(Limit::Nodes);
            return;
        };
        if let Some(top) = self.frames.last_mut() {
            top.carried = state;
            top.state = Some(pushed);
        }
    }

    /// Takes the line back at `at`, where the fence's language stopped.
    /// The language reached `state` there.
    ///
    /// A language stops for one of three reasons, the first two of which mark
    /// the scratch:
    ///
    /// - A `vue` fence opens where one of its rules began
    ///   ([`lex_to_vue_fence`]).
    /// - A `$base` slot of an Objective-C fence takes the line
    ///   ([`lex_with_root`]).
    /// - The language pops the fence's body.
    #[cold]
    #[inline(never)]
    #[cfg_attr(
        not(any(feature = "lang-objective-c", feature = "lang-vue")),
        expect(
            unused_variables,
            reason = "only an Objective-C fence's slot and a `vue` fence read the state"
        )
    )]
    fn hand_back(&mut self, at: usize, end: usize, state: u32) {
        #[cfg(feature = "lang-vue")]
        if std::mem::take(&mut self.scratch.markdown.vue_fence) {
            self.open_vue_fence_at(at, state);
            return;
        }
        #[cfg(feature = "lang-objective-c")]
        if let Some(slot) = self.scratch.markdown.slot.take() {
            let definition = (slot.pattern == Pattern::LinkDefinition)
                .then(|| self.scratch.markdown.definition.as_deref().copied())
                .flatten();
            self.open_at_slot(at, slot, definition.as_ref(), state);
            return;
        }
        self.pop_body(at, end);
    }

    /// Opens the root pattern of Markdown that took a `$base` slot of an
    /// Objective-C fence at `at`.
    ///
    /// vscode-textmate pushes that rule onto the fence language's stack.
    /// So the fence's frame keeps the language's state, `state`.
    /// The rule opens above it, inside the language's scopes.
    /// When the rule closes, the language goes on from the state kept.
    ///
    /// Where the rule leaves the line to a language, that language takes the
    /// rest of it at once:
    ///
    /// - An HTML block hands its text to html-derivative.
    /// - A rule that only matches, such as a heading, gives the line back to
    ///   the fence's language at its end.
    ///
    /// `step` would otherwise try only the open container's own rules.
    ///
    /// `slot` is the pattern that matched at `at`, and whether `^` or `\G`
    /// held there.
    /// The pattern always matches again:
    ///
    /// - The slot's test read the whole line and its line feed, as
    ///   [`Lexer::try_rule`] does in a fence's body.
    /// - `^` or `\G` holds at `at` here where it held for the slot's test.
    /// - Of the root patterns, only a link definition reads the line's work,
    ///   which the first try spent. Its match comes as `definition` and is
    ///   applied as it is.
    #[cfg(feature = "lang-objective-c")]
    fn open_at_slot(&mut self, at: usize, slot: Slot, definition: Option<&Definition>, state: u32) {
        let Some(&Frame {
            rule: Rule::Open(container),
            content,
            carried,
            state: kept,
            ..
        }) = self.frames.last()
        else {
            return;
        };
        self.anchor = slot.anchored.then_some(at);
        let mut captures = Captures::new();
        let hit = match definition {
            Some(definition) => {
                captures = definition.captures;
                Some(Hit {
                    rule: Some(slot.pattern),
                    start: at,
                    end: definition.end,
                })
            }
            None => self.try_rule(slot.pattern, at, &mut captures),
        };
        let Some(hit) = hit else {
            return;
        };
        self.keep_body_state(container, content, carried, kept, state);
        self.last = at;
        self.pos = at;
        let body = self.frames.len().saturating_sub(1);
        let language = self.arena.frame(state).stack;
        if let Some(frame) = self.frames.get_mut(body) {
            frame.content = language;
        }
        self.apply(&hit, &captures);
        if let Some(frame) = self.frames.get_mut(body) {
            frame.content = content;
        }
        if self.frames.last().is_some_and(|frame| embeds(frame.rule)) {
            self.embedded();
        }
    }

    /// Pops the fence's body where its language stopped at `at`, at its root,
    /// without advancing ([`Language::pops_host`]).
    /// vscode-textmate pops the innermost rule there, the body.
    /// The rest of the line, up to `end`, sits on the fence instead of the
    /// tokens the language gave it.
    /// The fence's next line opens the body again, as its rule does.
    #[cold]
    #[inline(never)]
    fn pop_body(&mut self, at: usize, end: usize) {
        crate::language::cut_tokens(self.out, self.first_token, at);
        self.last = at;
        if let Some(frame) = self.frames.pop() {
            self.anchor = frame.saved_anchor;
        }
        if let Some(fence) = self.frames.last() {
            self.emit(end, fence.content);
        }
        self.pos = self.limit;
    }

    /// Resumes the line at `at`, where the fence's language stopped for a
    /// `vue` fence ([`lex_to_vue_fence`]).
    /// The language reached `state` there, and the body keeps it.
    ///
    /// vscode-textmate puts `\G` at `at`, the end of the rule's begin match.
    /// So the fence opens there, over the language's scopes
    /// ([`Self::vue_fence_ahead`]).
    #[cfg(feature = "lang-vue")]
    fn open_vue_fence_at(&mut self, at: usize, state: u32) {
        let Some(&Frame {
            rule: Rule::Open(container),
            content,
            carried,
            state: kept,
            ..
        }) = self.frames.last()
        else {
            return;
        };
        let Some(lang) = embedded_language(Rule::Open(container)) else {
            return;
        };
        self.keep_body_state(container, content, carried, kept, state);
        self.last = at;
        self.pos = at;
        self.anchor = Some(at);
        self.vue_fence_ahead(lang);
    }
}

/// Lexes a fence's line in `lang` from `state`, and returns where the
/// language gave the line back, if it did.
///
/// A language that [`Language::pops_host`] names can pop the fence's body.
/// A `$base` slot of Objective-C can take the line for Markdown's root
/// patterns ([`lex_with_root`]).
/// `facts` are the line's, for those patterns.
#[expect(
    clippy::inline_always,
    reason = "out of line at one codegen unit, its call on every fence line cost Markdown 0.4%"
)]
#[inline(always)]
#[cfg_attr(
    not(any(feature = "lang-objective-c", feature = "lang-vue")),
    expect(
        unused_variables,
        reason = "only an Objective-C fence's slots and a `vue` fence read the facts"
    )
)]
fn lex_fence(
    cx: &mut Context<'_>,
    lang: Language,
    line: Line<'_>,
    state: u32,
    facts: &LineFacts,
) -> (Finished, Option<usize>) {
    if lang.pops_host() {
        return lex_popping(cx, lang, line, state);
    }
    #[cfg(feature = "lang-objective-c")]
    if matches!(lang, Language::ObjectiveC) {
        return lex_with_root(cx, line, state, facts);
    }
    #[cfg(feature = "lang-vue")]
    if halts_at_vue_fence(lang) {
        return lex_halting_fence(cx, lang, line, state, facts);
    }
    (lang.lex_embedded(cx, line, state), None)
}

/// Lexes a fence's line in a language of [`halts_at_vue_fence`].
/// The line stops where a `vue` fence opens, if [`block::vue_fence_later`]
/// finds one may.
///
/// Out of line, it keeps that test out of `step`.
/// Inlined there, `step` ran 0.46% more of Markdown's instructions
/// (Callgrind, 2026-11-12).
#[cfg(feature = "lang-vue")]
#[inline(never)]
fn lex_halting_fence(
    cx: &mut Context<'_>,
    lang: Language,
    line: Line<'_>,
    state: u32,
    facts: &LineFacts,
) -> (Finished, Option<usize>) {
    let window = Cx {
        line: line.text.as_bytes(),
        end: line.text.len(),
        anchor: None,
        facts,
    };
    if block::vue_fence_later(&window, line.start) {
        return lex_to_vue_fence(cx, lang, line, state, facts);
    }
    (lang.lex_embedded(cx, line, state), None)
}

/// Whether `lang`'s lexer stops where a rule begins before a `vue` fence
/// ([`Halt::anchored`]).
/// Only fence lines in those languages pay for [`block::vue_fence_later`].
/// On every fence line, that test cost Markdown 2% (Callgrind, 2026-11-12).
/// Each of them is the body of a fence, which the injection reaches.
#[cfg(feature = "lang-vue")]
const fn halts_at_vue_fence(lang: Language) -> bool {
    matches!(
        lang,
        Language::TypeScript | Language::JavaScript | Language::Jsx | Language::Tsx
    )
}

/// The `vue` fence of Shiki's `markdown-vue` injection, as a halt of the
/// fence's language.
/// It holds only where a rule of the language has begun
/// ([`Halt::anchored`]).
#[cfg(feature = "lang-vue")]
struct VueFence<'a> {
    facts: &'a LineFacts,
}

#[cfg(feature = "lang-vue")]
impl Halt for VueFence<'_> {
    fn first(&self, _: &[u8], _: usize, _: usize) -> Option<usize> {
        None
    }

    fn anchored(&self, s: &[u8], p: usize) -> bool {
        let cx = Cx {
            line: s,
            end: s.len(),
            anchor: Some(p),
            facts: self.facts,
        };
        block::vue_fence_open(&cx, p, &mut Captures::new()).is_some()
    }
}

/// Lexes a fence's line in `lang` up to where a `vue` fence opens, if one
/// does.
/// The scratch then marks the stop for [`Lexer::hand_back`].
///
/// It costs a call at each rule the language opens, so only a line that
/// [`block::vue_fence_later`] passes takes it.
#[cfg(feature = "lang-vue")]
#[cold]
#[inline(never)]
fn lex_to_vue_fence(
    cx: &mut Context<'_>,
    lang: Language,
    line: Line<'_>,
    state: u32,
    facts: &LineFacts,
) -> (Finished, Option<usize>) {
    let halt = VueFence { facts };
    match lang.lex_until(cx, line, state, &halt, None) {
        Some((
            finished,
            Some(Stopped {
                at,
                reason: StopReason::Halt,
            }),
        )) => {
            cx.scratch.markdown.vue_fence = true;
            (finished, Some(at))
        }
        #[cfg(feature = "_rule-engine")]
        Some((
            finished,
            Some(Stopped {
                at,
                reason: StopReason::PopHost,
            }),
        )) => (finished, Some(at)),
        Some((finished, None)) => (finished, None),
        None => (lang.lex_embedded(cx, line, state), None),
    }
}

/// Lexes a line of an Objective-C fence, whose `$base` slots try Markdown's
/// root patterns ([`RootPatterns`]).
///
/// It returns where Objective-C gave the line back.
/// Where a slot took the line, the scratch holds the root pattern that
/// matched there, for [`Lexer::hand_back`].
/// The halt stops the scan only at the place the last query gave, so that
/// query's pattern is the one that took the line.
/// Elsewhere Objective-C popped the fence's body, as at [`lex_popping`].
///
/// Like [`lex_popping`], it costs a call at each byte Objective-C tries.
#[cfg(feature = "lang-objective-c")]
#[cold]
#[inline(never)]
fn lex_with_root(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    facts: &LineFacts,
) -> (Finished, Option<usize>) {
    let root = RootPatterns::new(facts);
    match crate::objective_c::lex_until(cx, line, state, &root, None) {
        (
            finished,
            Some(Stopped {
                at,
                reason: StopReason::Halt,
            }),
        ) => {
            cx.scratch.markdown.slot = root.slot();
            if let Some(definition) = root.definition() {
                match &mut cx.scratch.markdown.definition {
                    Some(kept) => **kept = definition,
                    kept @ None => *kept = Some(Box::new(definition)),
                }
            }
            (finished, Some(at))
        }
        #[cfg(feature = "_rule-engine")]
        (
            finished,
            Some(Stopped {
                at,
                reason: StopReason::PopHost,
            }),
        ) => (finished, Some(at)),
        (finished, None) => (finished, None),
    }
}

/// Lexes a fence's line in a language that can pop the fence's body, through
/// [`Language::lex_until`] with a halt that never holds.
/// The halt lets the language report the pop ([`StopReason::PopHost`]).
/// It costs a call at each byte the language tries at its root.
#[cold]
#[inline(never)]
fn lex_popping(
    cx: &mut Context<'_>,
    lang: Language,
    line: Line<'_>,
    state: u32,
) -> (Finished, Option<usize>) {
    let (never, _) = crate::language::NO_HALTS;
    match lang.lex_until(cx, line, state, never, None) {
        #[cfg(feature = "_rule-engine")]
        Some((
            finished,
            Some(Stopped {
                at,
                reason: StopReason::PopHost,
            }),
        )) => (finished, Some(at)),
        Some((
            finished,
            Some(Stopped {
                reason: StopReason::Halt,
                ..
            })
            | None,
        )) => (finished, None),
        None => (lang.lex_embedded(cx, line, state), None),
    }
}

/// Whether a language's lexer takes the rest of each line inside `rule`.
/// Its answer matches [`embedded_language`].
///
/// It reads only the rule's container and builds no `Language`.
/// `step` tests it before every match.
/// Its lowering must not depend on the `Language` table.
/// With `embedded_language` there, one `Language` row's flags could change
/// that lowering.
/// Batch's row turned the bit mask into a copy of the frame and a jump table.
/// `step` then ran 2.2% more of its own instructions.
const fn embeds(rule: Rule) -> bool {
    match rule {
        Rule::Open(Container::Body(lang)) => !matches!(lang, Language::Markdown),
        #[cfg(feature = "lang-html")]
        Rule::Open(Container::BodyPhp) => true,
        #[cfg(feature = "lang-yaml")]
        Rule::Open(Container::FrontMatterBody) => true,
        #[cfg(feature = "lang-html-derivative")]
        Rule::Open(
            Container::HtmlRawBody
            | Container::HtmlBlock
            | Container::HtmlInline
            | Container::HtmlWrap,
        ) => true,
        Rule::Open(_) | Rule::Capture(_) => false,
    }
}

/// The language whose lexer takes the rest of each line inside `rule`: an
/// embedded fence's, YAML in front matter, or html-derivative in HTML. A
/// `markdown` fence takes Markdown's own rules instead.
fn embedded_language(rule: Rule) -> Option<Language> {
    match rule {
        Rule::Open(Container::Body(lang)) => (lang != Language::Markdown).then_some(lang),
        #[cfg(feature = "lang-html")]
        Rule::Open(Container::BodyPhp) => Some(Language::Html),
        #[cfg(feature = "lang-yaml")]
        Rule::Open(Container::FrontMatterBody) => Some(Language::Yaml),
        #[cfg(feature = "lang-html-derivative")]
        Rule::Open(
            Container::HtmlRawBody
            | Container::HtmlBlock
            | Container::HtmlInline
            | Container::HtmlWrap,
        ) => Some(Language::HtmlDerivative),
        Rule::Open(_) | Rule::Capture(_) => None,
    }
}

/// Where html-derivative gives the line back inside `container`:
/// none where it takes whole lines.
#[cfg(feature = "lang-html-derivative")]
fn html_give_back(container: Container) -> Option<crate::html::GiveBack<'static>> {
    match container {
        Container::HtmlInline => Some(crate::html::GiveBack::AtRoot),
        Container::HtmlWrap => Some(crate::html::GiveBack::AfterTag),
        Container::Base
        | Container::Quote
        | Container::ListBullet
        | Container::ListOrdered
        | Container::Fence(_)
        | Container::FencePlain
        | Container::FencePhp
        | Container::Body(_)
        | Container::BodyPhp
        | Container::RawBlock
        | Container::Table
        | Container::Paragraph
        | Container::Comment
        | Container::BoldStar
        | Container::BoldUnder
        | Container::ItalicStar
        | Container::ItalicUnder
        | Container::FrontMatter
        | Container::FrontMatterBody
        | Container::HtmlRaw
        | Container::HtmlRawBody
        | Container::HtmlBlock => None,
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
/// It takes the fields it changes instead of the lexer.
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
    let carried = body.carried;
    let anchored = frames.last().is_some_and(|top| {
        matches!(top.rule, Rule::Open(Container::Body(Language::Vue)))
            && crate::vue::at_root(arena, top.carried)
    });
    if anchored && anchor.is_none() {
        *anchor = Some(pos);
    }
    if index + 1 < frames.len() {
        if !crate::vue::while_holds(arena, carried, cx.line, pos) {
            frames.truncate(index + 1);
        }
        return true;
    }
    if !crate::vue::at_root(arena, carried) {
        return true;
    }
    let fence = index
        .checked_sub(1)
        .and_then(|outer| frames.get(outer))
        .map_or(0, |outer| outer.carried);
    let mut captures = Captures::new();
    block::fence_close(cx, pos, Fence::from_carried(fence), &mut captures).is_none()
}

/// `(^|\G)(\s*)(.*)`, the rule that starts the lines of an embedded
/// language. It matches at the end of the fence's opening line, where `\G`
/// is, and takes no text. On a line it only matches for itself, it takes the
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
/// root that is not.
/// The lines after the first are then told apart from it.
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
    let mut rebuilt = arena.push(ROOT, root.rule, 0, root.stack)?;
    while let Some(frame) = chain.pop() {
        rebuilt = arena.push(rebuilt, frame.rule, frame.carried, frame.stack)?;
    }
    Some(rebuilt)
}

#[cfg(test)]
mod tests {
    use super::{Container, Language, List, Rule, embedded_language, embeds};

    /// `step`'s guard and `embedded` give the same answer on every rule a
    /// frame can hold.
    ///
    /// The rules are each fixed container, a fence and a body for every
    /// language in the build, and a capture group.
    #[test]
    fn embeds_agrees_with_embedded_language() {
        let languages = Language::ALL
            .iter()
            .flat_map(|&lang| [Container::Fence(lang), Container::Body(lang)]);
        let rules = Container::FIXED
            .iter()
            .copied()
            .chain(languages)
            .map(Rule::Open)
            .chain([Rule::Capture(List::Inline)]);
        for rule in rules {
            assert_eq!(embeds(rule), embedded_language(rule).is_some(), "{rule:?}");
        }
    }

    /// The `$base` slots of an Objective-C fence and Markdown's own root give
    /// the same place where Markdown's root patterns first match.
    #[cfg(feature = "lang-objective-c")]
    mod root_slot {
        use super::super::{Captures, Container, Lexer, Pattern, rules};
        use crate::arena::{Arena, ROOT};
        use crate::language::{Parts, RootStart};
        use crate::limits::{Limits, LineReport, Steps};
        use crate::markdown::root::{RootPatterns, TESTS};
        use crate::markdown::text::LineFacts;

        /// A place for a slot: where the scan starts, and where `\G` holds.
        type Place = (usize, Option<usize>);

        /// A place where a root pattern first matches, and that pattern.
        type Found = Option<(usize, Pattern)>;

        /// Where Markdown's rules at the document's root first match in
        /// `line`, from `from`, with `\G` at `anchor`, and which rule.
        fn markdown_first(line: &str, (from, anchor): Place) -> Found {
            let mut arena = Arena::new(None);
            let limits = Limits::default();
            let mut out = Vec::new();
            let mut report = LineReport::default();
            let mut scratch = crate::scratch::Scratch::default();
            let parts = Parts {
                arena: &mut arena,
                limits: &limits,
                out: &mut out,
                report: &mut report,
            };
            let mut lexer = Lexer::new(parts, &mut scratch, line, ROOT);
            lexer.pos = from;
            lexer.anchor = anchor;
            let hit = lexer.find(&mut Captures::new())?;
            Some((hit.start, hit.rule?))
        }

        /// What one slot query of `line` answers at each place, in order,
        /// with the pattern it keeps for the hand-back.
        /// The places share the query, and so its kept search.
        fn slot_first(line: &str, places: &[Place]) -> Vec<Found> {
            let limits = Limits::default();
            let facts = LineFacts {
                steps: Steps::new(&limits, line.len()),
                before_start: None,
            };
            let root = RootPatterns::new(&facts);
            places
                .iter()
                .map(|&(from, anchor)| {
                    let at = RootStart::first(&root, line.as_bytes(), true, from, anchor)?;
                    Some((at, root.slot()?.pattern))
                })
                .collect()
        }

        /// Checks that the slot and Markdown both answer `expected` at each
        /// place of `line`, with the same pattern.
        fn agree(line: &str, places: &[Place], expected: &[Option<usize>]) {
            let markdown: Vec<Found> = places
                .iter()
                .map(|&place| markdown_first(line, place))
                .collect();
            let starts: Vec<Option<usize>> = markdown
                .iter()
                .map(|found| found.map(|(at, _)| at))
                .collect();
            assert_eq!(starts, expected, "Markdown on {line:?}");
            assert_eq!(slot_first(line, places), markdown, "the slot on {line:?}");
        }

        /// The tests are Markdown's root patterns, in order, without front
        /// matter.
        #[test]
        fn the_slot_tests_follow_the_root_patterns() {
            let tested: Vec<Pattern> = TESTS.iter().map(|&(pattern, _)| pattern).collect();
            let root: Vec<Pattern> = rules::patterns(Container::Base)
                .iter()
                .copied()
                .filter(|&pattern| pattern != Pattern::FrontMatter)
                .collect();
            assert_eq!(tested, root);
        }

        /// One to three spaces and a tab, then a tag, after `(`, where `\G`
        /// holds: an HTML block opens there, as in Shiki.
        #[cfg(feature = "lang-html-derivative")]
        #[test]
        fn a_tag_after_blanks_and_a_tab_opens_an_html_block() {
            for line in ["f( \t<div>", "f(  \t<div>", "f(   \t<div>"] {
                agree(line, &[(2, Some(2))], &[Some(2)]);
            }
        }

        /// A `<` and a space after the tab opens nothing.
        /// The tab keeps a paragraph from opening, so Objective-C lexes on.
        #[test]
        fn a_lone_angle_after_blanks_and_a_tab_opens_nothing() {
            for line in ["f( \t< x)", "f(  \t< x)", "f(   \t< x)"] {
                agree(line, &[(2, Some(2))], &[None]);
            }
        }

        /// Two such starts on one line: the first opens nothing, and the
        /// second, after the nested `(`, opens an HTML block.
        #[cfg(feature = "lang-html-derivative")]
        #[test]
        fn two_starts_on_one_line_answer_each_for_itself() {
            agree(
                "f( \t< x, g( \t<div>",
                &[(2, Some(2)), (11, Some(11))],
                &[None, Some(11)],
            );
        }

        /// A link definition starts at the first blank before its `[`, but not
        /// before the place the scan starts from.
        #[test]
        fn a_link_definition_starts_at_its_blanks() {
            agree(
                "g(x  [a]: b",
                &[(2, None), (3, None), (4, None)],
                &[Some(3), Some(3), Some(4)],
            );
        }

        /// The kept search never gives a later place a start that lies behind
        /// it.
        /// `[y]` is no definition, and `[a]: b` is one.
        /// From 6 the start found from 1 is behind, so it is searched again.
        /// From 7 no `[` is left.
        #[test]
        fn the_kept_search_never_gives_a_start_behind_the_place() {
            agree(
                "x [y] [a]: b",
                &[(1, None), (3, None), (6, None), (7, None), (8, None)],
                &[Some(5), Some(5), Some(6), None, None],
            );
        }
    }
}
