//! The per-line engine: it runs the grammar's rules the way a `TextMate`
//! engine does, with hand-written matchers instead of regular expressions.
//!
//! At each position the engine tries the end pattern of the innermost rule and
//! then the rule's patterns in order, and takes the first that matches at the
//! earliest position. A begin match opens a rule and an end match closes it;
//! a match's capture groups name parts of it. The corner cases of `TextMate`
//! engines are kept because Shiki's output depends on them: `\G` matches only
//! where the enclosing begin ended, a rule that opens and closes without
//! advancing stays open for the rest of the line, a match rule that does not
//! advance closes the rule it sits in, and a capture group that the grammar
//! tokenizes with patterns of its own is lexed on its text alone.
//!
//! The grammar's injection, which flags a bare `<`, joins the search: it
//! wins only where it starts before every match of the current rule.

use std::cell::Cell;

use memchr::memchr;

use super::scope::S;
use super::state::{self, ROOT_EMBEDDED, Rule};
use crate::arena::{Arena, ROOT};
use crate::language::{Context, Line};
use crate::limits::{Limit, Limits, LineReport};
use crate::scratch::Scratch as AllScratch;
use crate::{Stack, Token};

/// A piece of a scope name built at run time.
#[derive(Clone, Copy)]
pub(super) enum Part<'a> {
    Lit(&'a str),
    Name(u32),
}

/// What a match is: the begin of a rule, a match rule, an end, or the
/// injection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Pat {
    /// The end of the innermost rule.
    End,
    /// The injection: a `<` that no rule claims.
    Inject,
    /// A rule that opens without text, such as `\G` or `(?!\G)`.
    Zero(Rule),
    /// `<?xml`.
    Processing,
    /// `<!--`.
    Comment,
    /// `<!` before `DOCTYPE`.
    Doctype,
    /// `<![CDATA[`.
    Cdata,
    /// The blanks before a `<style>` element and the lookahead for it.
    StyleWrap,
    /// The blanks before a `<script>` element and the lookahead for it.
    ScriptWrap,
    /// A tag: `<`, the name, and the rule's kind in `info`.
    Tag,
    /// An element of an `svg` or `math` tree that opens a rule.
    Element,
    /// An element of an `svg` or `math` tree that is one match.
    ElementVoid,
    /// `<style`.
    StyleTag,
    /// `<script`.
    ScriptTag,
    /// The `>` that ends a `<script>` start tag and opens its content.
    ScriptJs,
    /// An attribute's name.
    Attribute,
    /// `=` that opens an attribute's value.
    Equals,
    /// A quote that opens a string.
    Quote,
    /// The quote that opens a string of a doctype.
    DoctypeQuote,
    /// `//` in an event handler.
    JsLine,
    /// `/*` in an event handler.
    JsBlock,
    /// An entity with a name.
    EntityNamed,
    /// A decimal character reference.
    EntityDecimal,
    /// A hexadecimal character reference.
    EntityHexadecimal,
    /// An `&` that is followed by a name and a `;` that is not an entity.
    EntityAmbiguous,
    /// What the grammar cannot take as an attribute.
    AttributeIllegal,
    /// An unexpected `=`.
    EqualsIllegal,
    /// A character a comment may not hold.
    CommentIllegal,
    /// `DOCTYPE`.
    DoctypeKeyword,
    /// The other words of a doctype.
    DoctypeWord,
    /// An unquoted value.
    Unquoted,
    /// An unquoted value of a `style` attribute.
    UnquotedCss,
    /// An unquoted value of an event handler.
    UnquotedJs,
    /// JavaScript text in a quoted value of an event handler.
    JsText,
}

/// A match: the bytes `start..end`, and the positions and bits that the
/// pattern's captures need.
#[derive(Clone, Copy, Debug)]
pub(super) struct Hit {
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) pat: Pat,
    /// Positions inside the match, by pattern.
    pub(super) at: [usize; 5],
    /// Bits of the pattern's kind.
    pub(super) info: u32,
}

impl Hit {
    pub(super) fn new(pat: Pat, start: usize, end: usize) -> Hit {
        Hit {
            start,
            end,
            pat,
            at: [0; 5],
            info: 0,
        }
    }

    pub(super) fn with(mut self, at: [usize; 5], info: u32) -> Hit {
        self.at = at;
        self.info = info;
        self
    }
}

/// An open rule.
#[derive(Clone, Copy, Debug)]
pub(super) struct Frame {
    pub(super) rule: Rule,
    /// Where the search stood when the rule opened in this line; `None` for a
    /// rule that an earlier line opened.
    enter: Option<usize>,
    /// The `\G` position when the rule opened.
    anchor: Option<usize>,
    /// Interned scope stack of the rule's begin and end.
    pub(super) name: u32,
    /// Interned scope stack of the rule's content.
    pub(super) content: u32,
    /// The interned state with this rule on top. A rule that a limit kept
    /// out of the arena has its parent's.
    pub(super) state: u32,
    /// What the rule carries.
    pub(super) payload: u32,
    /// Whether the injection stays out of the rule's text.
    pub(super) no_inject: bool,
}

/// What a highlighter keeps from line to line, so that lexing a line
/// allocates nothing once the buffers have grown.
#[derive(Debug, Default)]
pub(crate) struct Scratch {
    frames: Vec<Frame>,
    chain: Vec<(u32, crate::arena::Frame)>,
    /// The text of a scope name being built.
    name: String,
}

/// The most steps in a row that do not advance: each opens or closes a rule
/// at one byte, and a rule that repeats stops the line, so a run is at most
/// an open and a close per rule of the grammar, far below this.
const MAX_STALLED: u32 = 4096;

/// Lexes one line.
pub(super) struct Lexer<'a> {
    pub(super) s: &'a [u8],
    /// The same text, for the CSS and JavaScript lexers.
    pub(super) text: &'a str,
    scratch: &'a mut AllScratch,
    pub(super) arena: &'a mut Arena,
    limits: &'a Limits,
    out: &'a mut Vec<Token>,
    report: &'a mut LineReport,
    /// The text ends here: the line, or the capture group being lexed.
    pub(super) lim: usize,
    /// Whether a line feed follows the text, where the line ends.
    pub(super) lf: bool,
    /// Steps spent on this line, counted in scans too.
    steps: Cell<u64>,
    /// Steps this line may spend; `u64::MAX` for no limit.
    budget: u64,
    /// End of the last token.
    last: usize,
    /// Where the current search starts.
    pub(super) pos: usize,
    /// Where `\G` matches.
    pub(super) anchor: Option<usize>,
    /// The open rules, the root first.
    frames: Vec<Frame>,
    /// The last `<` found, as the position searched from and the answer,
    /// which holds only for the text it was found in: a region, which ends
    /// the text early, forgets it on the way in and out.
    next_lt: Cell<(usize, Option<usize>)>,
}

impl<'a> Lexer<'a> {
    /// A lexer for `line` from `state`, with the open rules of `state` as its
    /// frames, from the root down.
    pub(super) fn new(cx: &'a mut Context<'_>, line: Line<'a>, state: u32) -> Self {
        let s = line.text.as_bytes();
        let budget = cx.limits.work.map_or(u64::MAX, |work| work.steps(s.len()));
        let mut frames = std::mem::take(&mut cx.scratch.html.frames);
        frames.clear();
        let mut lexer = Lexer {
            s,
            text: line.text,
            scratch: &mut *cx.scratch,
            arena: &mut *cx.arena,
            limits: cx.limits,
            out: &mut *cx.out,
            report: &mut *cx.report,
            lim: s.len(),
            lf: true,
            steps: Cell::new(0),
            budget,
            last: line.start,
            pos: line.start,
            anchor: line.embedded.then_some(line.start),
            frames,
            next_lt: Cell::new((usize::MAX, None)),
        };
        let mut chain = std::mem::take(&mut lexer.scratch.html.chain);
        chain.clear();
        let mut id = state;
        loop {
            let frame = lexer.arena.frame(id);
            chain.push((id, frame));
            if Rule::from_code(frame.kind) == Rule::Root || frame.parent == ROOT {
                break;
            }
            id = frame.parent;
        }
        for (index, &(id, frame)) in chain.iter().rev().enumerate() {
            if index == 0 {
                lexer.frames.push(Frame {
                    rule: Rule::Root,
                    enter: None,
                    anchor: None,
                    name: frame.stack,
                    content: frame.stack,
                    state: id,
                    payload: frame.payload,
                    no_inject: frame.payload == ROOT_EMBEDDED,
                });
                continue;
            }
            let rule = Rule::from_code(frame.kind);
            let parent = lexer.top().content;
            let no_inject = lexer.top().no_inject || rule.shuts_out_injection();
            let name = lexer.name_stack(parent, rule, frame.payload);
            lexer.frames.push(Frame {
                rule,
                enter: None,
                anchor: None,
                name,
                content: frame.stack,
                state: id,
                payload: frame.payload,
                no_inject,
            });
        }
        lexer.scratch.html.chain = chain;
        lexer
    }

    /// Lexes the line and returns the state after it.
    pub(super) fn run(mut self) -> u32 {
        self.tokenize();
        let state = self.top().state;
        self.scratch.html.frames = std::mem::take(&mut self.frames);
        state
    }

    // Bytes.

    /// The byte at `i`, with the line feed after the text where one follows,
    /// or 0 where no character stands.
    pub(super) fn at(&self, i: usize) -> u8 {
        match i.cmp(&self.lim) {
            std::cmp::Ordering::Less => self.s[i],
            std::cmp::Ordering::Equal if self.lf => b'\n',
            std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => 0,
        }
    }

    /// The last position a pattern can match at: after the line feed.
    pub(super) fn end_pos(&self) -> usize {
        self.lim + usize::from(self.lf)
    }

    /// Counts `steps` of work against the line's budget.
    pub(super) fn spend(&self, steps: usize) {
        let steps = u64::try_from(steps).unwrap_or(u64::MAX);
        self.steps.set(self.steps.get().saturating_add(steps));
    }

    pub(super) fn over_budget(&self) -> bool {
        self.steps.get() > self.budget
    }

    /// The position of the next `<` at or after `from`, found once for the
    /// positions in a run.
    fn find_lt(&self, from: usize) -> Option<usize> {
        let (searched, found) = self.next_lt.get();
        if searched <= from && found.is_none_or(|at| at >= from) && searched != usize::MAX {
            return found;
        }
        let rest = self.s.get(from..self.lim).unwrap_or(&[]);
        let found = memchr(b'<', rest).map(|at| at + from);
        self.spend(found.map_or(rest.len(), |at| at - from) / 16);
        self.next_lt.set((from, found));
        found
    }

    // Scopes and tokens.

    pub(super) fn top(&self) -> &Frame {
        &self.frames[self.frames.len() - 1]
    }

    /// The stack `base` with `scopes` on top; a node limit keeps the part
    /// that does not fit off.
    pub(super) fn path(&mut self, base: u32, scopes: &[S]) -> u32 {
        let mut stack = base;
        for scope in scopes {
            let Some(child) = self.arena.child(stack, scope.scope()) else {
                self.report.record(Limit::Nodes);
                break;
            };
            stack = child;
        }
        stack
    }

    /// The stack `base` with the scope `meta.tag.…` and the like on top,
    /// named by `parts` joined: made at run time from the text matched.
    pub(super) fn dynamic(&mut self, base: u32, parts: &[Part<'_>]) -> u32 {
        let mut buffer = std::mem::take(&mut self.scratch.html.name);
        buffer.clear();
        for part in parts {
            match *part {
                Part::Lit(text) => buffer.push_str(text),
                Part::Name(id) => buffer.push_str(self.arena.string(id)),
            }
        }
        let scope = self.arena.named_scope(&buffer);
        self.scratch.html.name = buffer;
        let Some(scope) = scope else {
            self.report.record(Limit::Nodes);
            return base;
        };
        let Some(child) = self.arena.child(base, scope) else {
            self.report.record(Limit::Nodes);
            return base;
        };
        child
    }

    /// The text of an interned name, copied out of the arena.
    pub(super) fn word(&self, id: u32) -> &str {
        self.arena.string(id)
    }

    /// Ends a token at `to` with `stack`.
    pub(super) fn tok(&mut self, stack: u32, to: usize) {
        let to = to.min(self.lim);
        if to > self.last {
            self.out.push(Token {
                end: to,
                stack: Stack(stack),
            });
            self.last = to;
        }
    }

    /// Emits the text `from..to` under `base` with `scopes` on top.
    pub(super) fn cap(&mut self, base: u32, from: usize, to: usize, scopes: &[S]) {
        if from >= to {
            return;
        }
        self.tok(base, from);
        let stack = self.path(base, scopes);
        self.tok(stack, to);
    }

    /// Emits the text `from..to` under the stack `stack`.
    pub(super) fn cap_stack(&mut self, base: u32, from: usize, to: usize, stack: u32) {
        if from >= to {
            return;
        }
        self.tok(base, from);
        self.tok(stack, to);
    }

    /// Gives the rest of the text to the innermost rule's content.
    fn finish(&mut self) {
        let stack = self.top().content;
        self.tok(stack, self.lim);
    }

    /// The interned id of the text `from..to`, which a payload can carry.
    pub(super) fn intern(&mut self, from: usize, to: usize) -> u32 {
        let text = self.text.get(from..to).unwrap_or("");
        match self.arena.intern(text) {
            Some(id) if id < 1 << 24 => id,
            _ => {
                self.report.record(Limit::Nodes);
                0
            }
        }
    }

    // The loop.

    /// Takes matches until nothing matches at or after the position, as
    /// `TextMate` does, or a guard against a loop that does not advance stops
    /// the line.
    fn tokenize(&mut self) {
        let mut stalled = 0_u32;
        loop {
            self.spend(1);
            if self.over_budget() {
                self.report.record(Limit::Work);
                self.finish();
                return;
            }
            let found = match self.top().rule {
                Rule::BodyCss | Rule::BodyJs => self.embedded(),
                Rule::Root
                | Rule::Comment
                | Rule::Cdata
                | Rule::Doctype
                | Rule::DoctypeString
                | Rule::Processing
                | Rule::Tag
                | Rule::Element
                | Rule::ElementStart
                | Rule::StyleWrap
                | Rule::StyleTag
                | Rule::OpenStyle
                | Rule::ScriptWrap
                | Rule::ScriptTag
                | Rule::ScriptHead
                | Rule::ScriptAttributes
                | Rule::ScriptJsOuter
                | Rule::ScriptTemplateOuter
                | Rule::OpenScript
                | Rule::BodyTemplate
                | Rule::ScriptOtherOuter
                | Rule::BodyOther
                | Rule::Attribute
                | Rule::Value
                | Rule::ValueCss
                | Rule::ValueJs
                | Rule::WrapCss
                | Rule::WrapJs
                | Rule::StringPlain
                | Rule::StringCss
                | Rule::StringJs
                | Rule::JsLineComment
                | Rule::JsBlockComment
                | Rule::Region => self.search(),
            };
            let Some(hit) = found else {
                if self.over_budget() {
                    self.report.record(Limit::Work);
                }
                self.finish();
                return;
            };
            let before = self.pos;
            if self.apply(hit) {
                return;
            }
            if self.pos > before {
                stalled = 0;
            } else {
                stalled += 1;
                debug_assert!(
                    stalled < MAX_STALLED,
                    "the HTML lexer made no progress at byte {}",
                    self.pos
                );
                if stalled >= MAX_STALLED {
                    self.finish();
                    return;
                }
            }
        }
    }

    /// The first match from the position: for each candidate position the
    /// innermost rule's end pattern and then its patterns in order, and the
    /// injection where it starts before every one of them.
    fn search(&mut self) -> Option<Hit> {
        let rule = self.top().rule;
        let end = self.end_pos();
        let inject = if self.top().no_inject {
            None
        } else {
            self.find_lt(self.pos)
        };
        let cut = inject.map_or(self.lim, |at| at + 1);
        let mut p = self.pos;
        while p <= end {
            self.spend(1);
            if self.over_budget() {
                return None;
            }
            let Some(candidate) = self.candidate(rule, p, cut) else {
                break;
            };
            if let Some(at) = inject
                && at < candidate
            {
                return Some(Hit::new(Pat::Inject, at, at + 1));
            }
            p = candidate;
            if p > end {
                break;
            }
            if let Some(hit) = self.match_at(rule, p) {
                return Some(hit);
            }
            p += crate::utf8::char_len(self.at(p)).min(self.lim.saturating_sub(p).max(1));
        }
        inject.map(|at| Hit::new(Pat::Inject, at, at + 1))
    }

    /// The CSS or JavaScript of the innermost rule: the rest of the line up
    /// to the end tag that the embedded lexer halts at, which is then the
    /// rule's end.
    fn embedded(&mut self) -> Option<Hit> {
        let top = *self.top();
        let pos = self.pos;
        self.tok(top.content, pos);
        let mut inner = self.arena.frame(top.state).payload;
        let css = top.rule == Rule::BodyCss;
        if inner == 0 {
            inner = if css {
                crate::css::start_embedded(self.arena, top.content)
            } else {
                crate::typescript::start_script(self.arena, top.content)
            };
        }
        if pos >= self.lim {
            return None;
        }
        let mut cx = Context {
            scratch: &mut *self.scratch,
            arena: &mut *self.arena,
            limits: self.limits,
            out: &mut *self.out,
            report: &mut *self.report,
        };
        let line = Line {
            text: self.text,
            start: pos,
            embedded: false,
        };
        let (next, halt) = if css {
            crate::css::lex_until(&mut cx, line, inner, style_end)
        } else {
            crate::typescript::lex_until(
                &mut cx,
                crate::typescript::Dialect::JavaScript,
                line,
                inner,
                script_end,
                None,
            )
        };
        let to = halt.unwrap_or(self.lim);
        self.spend(to.saturating_sub(pos) + 1);
        if to > pos {
            self.last = to;
            self.pos = to;
        }
        let Some(at) = halt else {
            let parent = self.frames[self.frames.len() - 2].state;
            let state = self
                .arena
                .push(parent, top.rule.code(), next, top.content)
                .unwrap_or_else(|| {
                    self.report.record(Limit::Nodes);
                    top.state
                });
            let last = self.frames.len() - 1;
            self.frames[last].state = state;
            return None;
        };
        Some(Hit::new(Pat::End, at, at))
    }

    // Applying matches.

    /// Applies a match; true when the line stops.
    fn apply(&mut self, hit: Hit) -> bool {
        match hit.pat {
            Pat::End => self.close(&hit),
            Pat::Inject => self.inject(&hit),
            Pat::ElementVoid => self.element_void(&hit),
            Pat::EntityNamed
            | Pat::EntityDecimal
            | Pat::EntityHexadecimal
            | Pat::EntityAmbiguous
            | Pat::AttributeIllegal
            | Pat::EqualsIllegal
            | Pat::CommentIllegal
            | Pat::DoctypeKeyword
            | Pat::DoctypeWord
            | Pat::Unquoted
            | Pat::UnquotedCss
            | Pat::UnquotedJs
            | Pat::JsText => self.match_rule(&hit),
            Pat::Zero(_)
            | Pat::Processing
            | Pat::Comment
            | Pat::Doctype
            | Pat::Cdata
            | Pat::StyleWrap
            | Pat::ScriptWrap
            | Pat::Tag
            | Pat::Element
            | Pat::StyleTag
            | Pat::ScriptTag
            | Pat::ScriptJs
            | Pat::Attribute
            | Pat::Equals
            | Pat::Quote
            | Pat::DoctypeQuote
            | Pat::JsLine
            | Pat::JsBlock => self.open(&hit),
        }
    }

    /// Applies the injection: the `<` is a token of its own.
    fn inject(&mut self, hit: &Hit) -> bool {
        let top = *self.top();
        self.tok(top.content, hit.start);
        self.cap(top.content, hit.start, hit.end, &[S::BadAngleBracket]);
        self.pos = self.pos.max(hit.end);
        false
    }

    /// Applies an end match; true when the line stops.
    /// A rule that opened and closed without advancing would loop, so the rest
    /// of the line is the rule's.
    fn close(&mut self, hit: &Hit) -> bool {
        let top = *self.top();
        self.tok(top.content, hit.start);
        self.end_captures(&top, hit);
        self.tok(top.name, hit.end);
        let popped = self.frames.pop().unwrap_or(top);
        self.anchor = popped.anchor;
        let advanced = hit.end > self.pos;
        if !advanced && popped.enter == Some(self.pos) {
            self.frames.push(Frame {
                content: popped.name,
                ..popped
            });
            self.tok(popped.name, self.lim);
            return true;
        }
        self.pos = self.pos.max(hit.end);
        false
    }

    /// Applies the begin of a rule; true when the line stops. A rule the depth
    /// limit keeps from opening leaves its text plain, and since a rule that
    /// matched nothing would match again where it did, the search goes on
    /// after one character.
    /// A pattern that names no rule leaves its text plain, and a rule that
    /// opens again where it opened without advancing stops the line.
    fn open(&mut self, hit: &Hit) -> bool {
        let top = *self.top();
        self.tok(top.content, hit.start);
        let Some((rule, payload)) = self.opens(hit, &top) else {
            self.tok(top.content, hit.end);
            self.pos = self.pos.max(hit.end);
            return false;
        };
        let depth = self.frames.len();
        if self.limits.depth_reached(depth - 1) {
            self.report.record(Limit::Depth);
            let next = if hit.end > hit.start {
                hit.end
            } else {
                hit.start + crate::utf8::char_len(self.at(hit.start))
            };
            let next = next.min(self.lim);
            self.tok(top.content, next);
            self.pos = self.pos.max(next);
            return false;
        }
        let advanced = hit.end > self.pos;
        let before = depth - 1;
        self.push_frame(rule, payload);
        let frame = *self.top();
        self.begin_captures(hit, &frame, &top);
        self.tok(frame.name, hit.end);
        self.anchor = Some(hit.end);
        self.pos = self.pos.max(hit.end);
        if !advanced && self.loops_below(before, rule) {
            self.frames.pop();
            let node = self.top().content;
            self.tok(node, self.lim);
            return true;
        }
        false
    }

    /// Pushes `rule` on the stack, with the scopes of its name and content.
    fn push_frame(&mut self, rule: Rule, payload: u32) {
        let top = *self.top();
        let name = self.name_stack(top.content, rule, payload);
        let content = self.content_stack(name, rule);
        let state = self
            .arena
            .push(top.state, rule.code(), payload, content)
            .unwrap_or_else(|| {
                self.report.record(Limit::Nodes);
                top.state
            });
        self.frames.push(Frame {
            rule,
            enter: Some(self.pos),
            anchor: self.anchor,
            name,
            content,
            state,
            payload,
            no_inject: top.no_inject || rule.shuts_out_injection(),
        });
    }

    /// Whether `rule`, just opened above frame `before`, repeats a rule that
    /// opened at the same position without advancing.
    fn loops_below(&self, before: usize, rule: Rule) -> bool {
        let mut i = before;
        loop {
            let frame = &self.frames[i];
            if frame.enter != Some(self.pos) {
                return false;
            }
            if frame.rule == rule && i != self.frames.len() - 1 {
                return true;
            }
            if i == 0 {
                return false;
            }
            i -= 1;
        }
    }

    /// Applies a match rule: its text under its scopes; true when the line
    /// stops.
    /// A match that does not advance would loop, so the enclosing rule is
    /// dropped and the line stops.
    fn match_rule(&mut self, hit: &Hit) -> bool {
        let top = *self.top();
        self.tok(top.content, hit.start);
        let advanced = hit.end > self.pos;
        self.match_captures(hit, &top);
        if !advanced {
            if self.frames.len() > 1 {
                self.frames.pop();
            }
            let node = self.top().content;
            self.tok(node, self.lim);
            return true;
        }
        self.pos = self.pos.max(hit.end);
        false
    }

    /// Lexes the text `from..to` of the line with the attribute rules of
    /// `namespace`, as the grammar does for a capture group of an element's
    /// start tag: on that text alone, under `base`, and the rules it leaves
    /// open end with it.
    pub(super) fn region(&mut self, from: usize, to: usize, base: u32, payload: u32) {
        if from >= to {
            return;
        }
        let top = *self.top();
        let (lim, lf, pos, anchor) = (self.lim, self.lf, self.pos, self.anchor);
        let depth = self.frames.len();
        self.frames.push(Frame {
            rule: Rule::Region,
            enter: None,
            anchor: None,
            name: base,
            content: base,
            state: top.state,
            payload,
            no_inject: top.no_inject,
        });
        self.tok(top.content, from);
        self.lim = to;
        self.lf = false;
        self.pos = from;
        self.anchor = None;
        self.next_lt.set((usize::MAX, None));
        self.tokenize();
        self.tok(self.top().content, to);
        self.frames.truncate(depth);
        self.lim = lim;
        self.lf = lf;
        self.pos = pos;
        self.anchor = anchor;
        self.next_lt.set((usize::MAX, None));
        self.last = self.last.max(to);
    }

    /// Lexes the text `from..to` with the JavaScript lexer, from a fresh
    /// state, under `base`, as `TextMate` lexes a capture group again: the
    /// text before `from` stays visible to lookbehinds, and no line feed
    /// follows `to`.
    pub(super) fn javascript(&mut self, from: usize, to: usize, base: u32) {
        if from >= to {
            return;
        }
        self.tok(base, from);
        self.spend(to - from);
        let state = crate::typescript::start_embedded(self.arena, base);
        let mut cx = Context {
            scratch: &mut *self.scratch,
            arena: &mut *self.arena,
            limits: self.limits,
            out: &mut *self.out,
            report: &mut *self.report,
        };
        let text = self.text.get(..to).unwrap_or(self.text);
        let line = Line {
            text,
            start: from,
            embedded: false,
        };
        crate::typescript::lex_capture(&mut cx, line, state);
        self.last = self.last.max(to);
    }
}

/// The end of a `<style>` element's CSS: `</style`, in any case.
fn style_end(s: &[u8], p: usize) -> bool {
    s.get(p..p + 7)
        .is_some_and(|text| text.eq_ignore_ascii_case(b"</style"))
}

/// The end of a `<script>` element's JavaScript: `</script`, in any case.
fn script_end(s: &[u8], p: usize) -> bool {
    s.get(p..p + 8)
        .is_some_and(|text| text.eq_ignore_ascii_case(b"</script"))
}

/// Packs a name's interned id with the bits that say how to spell it.
pub(super) fn packed(info: u32, name: u32) -> u32 {
    state::pack(info, name)
}
