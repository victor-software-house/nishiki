//! The per-line engine: it runs the grammar's rules the way a `TextMate`
//! engine does, with hand-written matchers instead of regular expressions.
//!
//! At each position the engine tries the end pattern of the innermost rule.
//! Then it tries the rule's patterns in order.
//! It takes the first that matches at the earliest position.
//! A begin match opens a rule and an end match closes it.
//! A match's capture groups name parts of it.
//!
//! The corner cases of `TextMate` engines are kept because Shiki's output
//! depends on them:
//!
//! - `\G` matches only where the enclosing begin ended.
//! - A rule that opens and closes without advancing stays open for the rest of
//!   the line.
//! - A match rule that does not advance closes the rule it sits in.
//! - A capture group that the grammar tokenizes with patterns of its own is
//!   lexed on its text alone.
//!
//! The grammar's injection, which flags a bare `<`, joins the search: it
//! wins only where it starts before every match of the current rule. In an
//! ERB document ERB's injection takes its place, and opens Ruby code there.

use std::cell::Cell;

use memchr::memchr;

use super::blade::{self, Blade, PhpHalt};
use super::erb::{self, Erb};
use super::scope::S;
use super::state::{self, Injection, ROOT_DERIVATIVE_FIRST, ROOT_EMBEDDED, ROOT_SVELTE, Rule};
#[cfg(feature = "lang-vue")]
use super::vue as vue_rules;
use super::vue::VuePattern;
use crate::arena::{Arena, ROOT};
#[cfg(feature = "lang-vue")]
use crate::language::cut_tokens;
use crate::language::{Context, Halt, Line, StopReason, Stopped};
use crate::limits::{Finished, Limit, Limits, LineReport, Steps};
use crate::scratch::Scratch as AllScratch;
use crate::{Stack, Token, case_fold};

/// A piece of a scope name built at run time.
#[derive(Clone, Copy)]
pub(super) enum Part<'a> {
    Lit(&'a str),
    Name(u32),
}

/// What a match is: the begin of a rule, a match rule, an end, or the
/// injection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Pattern {
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
    /// A tag: `<`, the name, and the tag's class in `bits`.
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
    /// A pattern of ERB's injection, or of the rules that wrap a comment or
    /// code that its line does not close.
    Erb(Erb),
    /// ERB: `(#).*?(?=-?%>)` in code, a comment up to the code's end.
    ErbHash,
    /// A pattern of Vue's rules.
    Vue(VuePattern),
    /// A pattern of Blade's injection or of Blade's rules that opens a rule.
    Blade(Blade),
    /// A pattern of Blade's injection or of Blade's rules that opens none.
    BladeMatch(Blade),
}

/// The grammar a document is lexed with.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Document {
    /// HTML.
    Html,
    /// HTML embedded in a document whose grammar compiles html-derivative's
    /// rules before HTML's, such as Markdown.
    ///
    /// It is HTML, except for the grammar's rule for invalid tags.
    /// Compiled through html-derivative, that rule's attributes resolve to
    /// HTML's list.
    /// Compiled through HTML, they resolve to MathML's.
    DerivativeFirst,
    /// html-derivative: HTML with a rule of its own for the tags no other
    /// rule takes, which Vue's templates use.
    #[cfg_attr(
        not(feature = "lang-html-derivative"),
        expect(dead_code, reason = "only the html-derivative language lexes it")
    )]
    Derivative,
    /// ERB: HTML with ERB's injection in place of the HTML grammar's.
    #[cfg_attr(
        not(feature = "lang-erb"),
        expect(dead_code, reason = "only the ERB language lexes ERB")
    )]
    Erb,
    /// Vue: a single-file component, whose template is html-derivative with
    /// Vue's own tag rules. No injection applies.
    #[cfg_attr(
        not(feature = "lang-vue"),
        expect(dead_code, reason = "only the Vue language lexes Vue")
    )]
    Vue,
    /// Blade: html-derivative with Blade's injection in place of the
    /// grammar's.
    #[cfg_attr(
        not(feature = "lang-blade"),
        expect(dead_code, reason = "only the Blade language lexes Blade")
    )]
    Blade,
}

/// How a `<script>`'s JavaScript is lexed, by the document around it.
#[derive(Clone, Copy, Debug)]
enum Script {
    /// As JavaScript.
    Plain,
    /// As JavaScript that also stops where ERB code begins.
    Erb,
    /// As JavaScript with Svelte's `$` prefixes, in a Svelte document
    /// ([`ROOT_SVELTE`]).
    Svelte,
}

impl Document {
    /// The document when a grammar that compiles html-derivative first
    /// embeds it.
    /// Markdown has no ERB fence, and html-derivative and Vue already read
    /// HTML's attribute list for invalid tags.
    fn derivative_first(self) -> Self {
        match self {
            Document::Html | Document::DerivativeFirst => Document::DerivativeFirst,
            Document::Derivative | Document::Erb | Document::Blade | Document::Vue => self,
        }
    }
}

/// Where HTML, lexed inside a rule of another grammar that includes the
/// grammar's root patterns, gives the line back to that rule.
#[derive(Clone, Copy, Debug)]
pub(crate) enum GiveBack<'a> {
    /// Back at the root after one root-level construct, because the
    /// rule's own patterns compete at every position: a Markdown paragraph
    /// or heading.
    AtRoot,
    /// At the root, at the first position after a `>` where no root rule
    /// matches.
    /// There the end `(?<=>)` of the rule that Markdown's emphasis opens at a
    /// tag is tried, after the patterns.
    AfterTag,
    /// At the root, where `end` holds before the root's patterns at a place,
    /// or `after` holds where none of them matches.
    /// These are the embedding rule's patterns listed before and after the
    /// grammar (see [`Halt`]), such as the interpolation and escapes of a
    /// Ruby here document.
    Halts(&'a crate::language::Halts<'a>),
}

/// A match: the bytes `start..end`, and the positions and bits that the
/// pattern's captures need.
#[derive(Clone, Copy, Debug)]
pub(super) struct Hit {
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) pattern: Pattern,
    /// Positions inside the match, by pattern.
    pub(super) at: [usize; 5],
    /// The bits the match carries, such as a tag's class.
    pub(super) bits: u32,
}

impl Hit {
    pub(super) fn new(pattern: Pattern, start: usize, end: usize) -> Hit {
        Hit {
            start,
            end,
            pattern,
            at: [0; 5],
            bits: 0,
        }
    }

    pub(super) fn with(mut self, at: [usize; 5], bits: u32) -> Hit {
        self.at = at;
        self.bits = bits;
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
    /// Whether a template's injection applies to the rule's text, and how
    /// its matches rank against the rule's own.
    pub(super) injection: Injection,
}

/// What a highlighter keeps from line to line, so that lexing a line
/// allocates nothing once the buffers have grown.
#[derive(Debug, Default)]
pub(crate) struct Scratch {
    frames: Vec<Frame>,
    /// Buffers of frames for HTML lexers that run while another holds
    /// `frames`, one for each level of nesting.
    /// One such lexer runs a heredoc's HTML inside a Blade document's PHP.
    #[cfg(feature = "lang-blade")]
    spare: Vec<Vec<Frame>>,
    chain: Vec<(u32, crate::arena::Frame)>,
    /// The text of a scope name being built.
    name: String,
}

#[cfg(feature = "lang-blade")]
impl Scratch {
    /// A spare buffer for an HTML lexer that starts while another holds
    /// `frames`.
    #[cold]
    #[inline(never)]
    fn spare_frames(&mut self) -> Vec<Frame> {
        self.spare.pop().unwrap_or_default()
    }

    /// Keeps `frames` for the next line where an HTML lexer that ran inside
    /// this one already gave its buffer back to `frames`.
    #[cold]
    #[inline(never)]
    fn keep_spare(&mut self, frames: Vec<Frame>) {
        self.spare.push(frames);
    }
}

/// The most steps in a row that do not advance.
/// Each opens or closes a rule at one byte, and a rule that repeats stops the
/// line.
/// A run is thus at most an open and a close per rule of the grammar, far
/// below this.
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
    /// Where the physical line's tokens start in `out`
    /// ([`Context::first_token`]).
    first_token: usize,
    /// The text ends here: the line, or the capture group being lexed.
    pub(super) lim: usize,
    /// Whether a line feed follows the text, where the line ends.
    pub(super) lf: bool,
    /// Steps spent on this line, counted in scans too, and lent to the
    /// languages this one embeds.
    steps: Steps,
    /// End of the last token.
    last: usize,
    /// Where the current search starts.
    pub(super) pos: usize,
    /// Where `\G` matches.
    pub(super) anchor: Option<usize>,
    /// The open rules, the root first.
    frames: Vec<Frame>,
    /// A search's last answer, as the position searched from and the place
    /// found. It holds only for the text it was found in: a region, which
    /// ends the text early, forgets it on the way in and out.
    /// It caches one of two searches, by document:
    /// - in an HTML or html-derivative document, the next `<` where the
    ///   grammar's injection applies ([`Lexer::find_lt`]);
    /// - in a Blade document, the next place where Blade's own rules match at a
    ///   heredoc's root ([`Lexer::find_blade_heredoc`]).
    ///
    /// The two never meet: `find_lt` runs only in the HTML and html-derivative
    /// arms of [`Lexer::search`], and `find_blade_heredoc` only in its Blade
    /// arm. A Blade document stays Blade for the whole line
    /// ([`Document::derivative_first`] keeps it).
    next_lt: Cell<(usize, Option<usize>)>,
    /// The grammar the document is lexed with.
    pub(super) document: Document,
    /// The last place found where ERB's or Blade's injection matches, kept
    /// like `next_lt`.
    next_erb: Cell<(usize, Option<usize>)>,
    /// Where the line goes back to the grammar that includes this one; none
    /// for a document or a body that takes whole lines.
    pub(super) give_back: Option<GiveBack<'a>>,
    /// Whether a match has been applied on this line.
    applied: bool,
    /// Where the line went back to the including grammar.
    given_back: Option<usize>,
}

impl<'a> Lexer<'a> {
    /// A lexer for `line` from `state`, with the open rules of `state` as its
    /// frames, from the root down, in a `document` of that grammar.
    pub(super) fn new(
        cx: &'a mut Context<'_>,
        line: Line<'a>,
        state: u32,
        document: Document,
    ) -> Self {
        let s = line.text.as_bytes();
        let steps = Steps::new(cx.limits, s.len());
        let mut frames = std::mem::take(&mut cx.scratch.html.frames);
        #[cfg(feature = "lang-blade")]
        if frames.capacity() == 0 {
            frames = cx.scratch.html.spare_frames();
        }
        frames.clear();
        let mut lexer = Lexer {
            s,
            text: line.text,
            scratch: &mut *cx.scratch,
            arena: &mut *cx.arena,
            limits: cx.limits,
            out: &mut *cx.out,
            report: &mut *cx.report,
            first_token: cx.first_token,
            lim: s.len(),
            lf: true,
            steps,
            last: line.start,
            pos: line.start,
            anchor: line.embedded.then_some(line.start),
            frames,
            next_lt: Cell::new((usize::MAX, None)),
            document,
            next_erb: Cell::new((usize::MAX, None)),
            give_back: None,
            applied: false,
            given_back: None,
        };
        let mut chain = std::mem::take(&mut lexer.scratch.html.chain);
        chain.clear();
        let mut id = state;
        loop {
            let frame = lexer.arena.frame(id);
            chain.push((id, frame));
            if Rule::from_code(frame.rule) == Rule::Root || frame.parent == ROOT {
                break;
            }
            id = frame.parent;
        }
        for (index, &(id, frame)) in chain.iter().rev().enumerate() {
            if index == 0 {
                if matches!(frame.payload, ROOT_DERIVATIVE_FIRST | ROOT_SVELTE) {
                    lexer.document = lexer.document.derivative_first();
                }
                lexer.frames.push(Frame {
                    rule: Rule::Root,
                    enter: None,
                    anchor: None,
                    name: frame.stack,
                    content: frame.stack,
                    state: id,
                    payload: frame.payload,
                    no_inject: matches!(
                        frame.payload,
                        ROOT_EMBEDDED | ROOT_DERIVATIVE_FIRST | ROOT_SVELTE
                    ),
                    injection: if matches!(
                        frame.payload,
                        state::ROOT_BLADE_HEREDOC | state::ROOT_BLADE_HEREDOC_SEALED
                    ) {
                        Injection::ShutOut
                    } else {
                        Injection::Applies
                    },
                });
                continue;
            }
            let rule = Rule::from_code(frame.rule);
            let top = *lexer.top();
            let (parent, injection) = lexer.frame_links(&top, rule);
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
                injection,
            });
        }
        lexer.scratch.html.chain = chain;
        lexer
    }

    /// Lexes the line and returns the state after it.
    pub(super) fn run(self) -> Finished {
        self.run_until().0
    }

    /// Lexes the line and returns the state after it, with the position
    /// where the line went back to the including grammar, if it did.
    ///
    /// A Vue body's `while` rule, `^(?!\s*</script[>\s])`, is checked first.
    /// Its `^` fails where a Markdown quote or list took the line's start.
    /// The check sits here, not in `tokenize`: before its loop, LLVM copied
    /// the loop's first block and gave `give_back_at` a second call.
    pub(super) fn run_until(mut self) -> (Finished, Option<usize>) {
        #[cfg(feature = "lang-vue")]
        if self.document == Document::Vue && self.top().rule == Rule::VueBodyWhile {
            self.vue_while();
        }
        self.tokenize();
        let state = self.top().state;
        self.keep_frames();
        (self.steps.finish(state), self.given_back)
    }

    /// Whether a root pattern matches at `p`, with the steps the test spent.
    /// The test lexes nothing.
    #[cfg(feature = "lang-html-derivative")]
    pub(super) fn opens_at(mut self, p: usize) -> (bool, u64) {
        let opens = self.match_at(Rule::Root, p).is_some();
        self.keep_frames();
        (opens, self.steps.into_spent())
    }

    /// Gives the frames' buffer back to the scratch for the next line.
    /// Where an HTML lexer that ran inside this one gave its own back first,
    /// this one goes to the spares.
    fn keep_frames(&mut self) {
        let frames = std::mem::take(&mut self.frames);
        #[cfg(feature = "lang-blade")]
        if self.scratch.html.frames.capacity() != 0 {
            self.scratch.html.keep_spare(frames);
            return;
        }
        self.scratch.html.frames = frames;
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
        self.steps.spend(steps);
    }

    pub(super) fn over_budget(&self) -> bool {
        self.steps.exhausted()
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

    /// The first place at or after `from` where ERB's injection matches,
    /// found once for the positions in a run.
    fn find_erb(&self, from: usize) -> Option<usize> {
        let (searched, found) = self.next_erb.get();
        if searched <= from && found.is_none_or(|at| at >= from) && searched != usize::MAX {
            return found;
        }
        let text = &self.s[..self.lim];
        let mut found = (from == 0 && erb::at(text, 0).is_some()).then_some(0);
        let mut at = from;
        while found.is_none() {
            let Some(lt) = memchr(b'<', text.get(at..).unwrap_or_default()) else {
                break;
            };
            let lt = lt + at;
            if erb::at(text, lt).is_some() {
                found = Some(lt);
            }
            at = lt + 1;
        }
        self.spend(found.unwrap_or(self.lim).saturating_sub(from) / 16);
        self.next_erb.set((from, found));
        found
    }

    /// The match of ERB's injection at `at`, which [`Lexer::find_erb`] found.
    fn erb_hit(&self, at: usize) -> Option<Hit> {
        erb::at(&self.s[..self.lim], at).map(|(erb, end)| Hit::new(Pattern::Erb(erb), at, end))
    }

    /// The first place at or after the position where Blade's injection
    /// matches, found once for the positions in a run, as
    /// [`Lexer::find_erb`] does.
    /// Where `injection` shuts it out, Blade's own rules at a heredoc's root
    /// ([`Lexer::find_blade_heredoc`]).
    /// One call serves both, so the search keeps one arm for Blade.
    #[inline(never)]
    fn find_blade(&self, injection: Injection) -> Option<usize> {
        if injection == Injection::ShutOut {
            return self.find_blade_heredoc();
        }
        let from = self.pos;
        let (searched, found) = self.next_erb.get();
        if searched <= from && found.is_none_or(|at| at >= from) && searched != usize::MAX {
            return found;
        }
        let found = blade::find(&self.s[..self.lim], from);
        self.spend(found.unwrap_or(self.lim).saturating_sub(from) / 16);
        self.next_erb.set((from, found));
        found
    }

    /// The match of Blade's injection at `at`, which [`Lexer::find_blade`]
    /// found, or of Blade's own rules at a heredoc's root.
    #[inline(never)]
    fn blade_hit(&self, at: usize) -> Option<Hit> {
        let text = &self.s[..self.lim];
        let found = if self.blade_heredoc_root() {
            blade::rules_at(text, at)
        } else {
            blade::at(text, at)
        };
        found.map(|(blade, end)| Hit::new(Pattern::blade(blade), at, end))
    }

    /// Whether only the root of a Blade heredoc is open
    /// ([`state::ROOT_BLADE_HEREDOC`]).
    pub(super) fn blade_heredoc_root(&self) -> bool {
        self.frames.len() == 1
            && matches!(
                self.frames[0].payload,
                state::ROOT_BLADE_HEREDOC | state::ROOT_BLADE_HEREDOC_SEALED
            )
    }

    /// Where Blade's own rules match next at a heredoc's root, where the
    /// injection does not apply. No other rule of a heredoc tries them.
    /// The place is found once for the searches of a run, as
    /// [`Lexer::find_blade`] does, and kept in `next_lt`.
    /// A heredoc line with many tags then reads the line once, not once a tag.
    #[cold]
    #[inline(never)]
    fn find_blade_heredoc(&self) -> Option<usize> {
        if !self.blade_heredoc_root() {
            return None;
        }
        let from = self.pos;
        let (searched, found) = self.next_lt.get();
        if searched <= from && found.is_none_or(|at| at >= from) && searched != usize::MAX {
            return found;
        }
        let found = blade::find_rules(&self.s[..self.lim], from);
        self.spend(found.unwrap_or(self.lim).saturating_sub(from) / 16);
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
    pub(super) fn capture(&mut self, base: u32, from: usize, to: usize, scopes: &[S]) {
        if from >= to {
            return;
        }
        self.tok(base, from);
        let stack = self.path(base, scopes);
        self.tok(stack, to);
    }

    /// Emits the text `from..to` under the stack `stack`.
    pub(super) fn capture_stack(&mut self, base: u32, from: usize, to: usize, stack: u32) {
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
    #[expect(
        clippy::too_many_lines,
        reason = "the loop dispatches on every rule; out of it, \
                  LLVM gave give_back_at a second call"
    )]
    fn tokenize(&mut self) {
        let mut stalled = 0_u32;
        loop {
            self.spend(1);
            if self.over_budget() {
                self.report.record(Limit::Work);
                self.finish();
                return;
            }
            if matches!(self.give_back, Some(GiveBack::AtRoot))
                && self.applied
                && self.frames.len() == 1
            {
                self.given_back = Some(self.pos);
                return;
            }
            let found = match self.top().rule {
                Rule::BodyCss | Rule::BodyJs => self.embedded(),
                Rule::ErbBlock | Rule::ErbLine => self.ruby(),
                Rule::BladeCommentPhpBlock
                | Rule::BladePhpBlock
                | Rule::BladePhpLineCode
                | Rule::BladeEchoTriple
                | Rule::BladeEcho
                | Rule::BladeEchoRaw
                | Rule::BladeEchoEscaped
                | Rule::BladeDirective
                | Rule::BladeDirectiveCustom
                | Rule::BladePhpSection
                | Rule::BladeAttribute => self.blade_php(),
                Rule::VueBody | Rule::VueBodyWhile | Rule::VueExpression => self.vue_embedded(),
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
                | Rule::AttributeValue
                | Rule::AttributeValueCss
                | Rule::AttributeValueJs
                | Rule::WrapCss
                | Rule::WrapJs
                | Rule::StringPlain
                | Rule::StringCss
                | Rule::StringJs
                | Rule::JsLineComment
                | Rule::JsBlockComment
                | Rule::Region
                | Rule::ErbCommentWrap
                | Rule::ErbTagWrap
                | Rule::ErbComment
                | Rule::BladeComment
                | Rule::BladeCommentPhpWrap
                | Rule::BladeCommentPhpLine
                | Rule::BladePhpWrap
                | Rule::BladePhpLine
                | Rule::BladeArgumentsComment
                | Rule::BladeBalance
                | Rule::VueComment
                | Rule::VueCommentKeyValue
                | Rule::VueSelfClosing
                | Rule::VueBlock
                | Rule::VueElement
                | Rule::VueTagStuff
                | Rule::VueScriptTagStuff
                | Rule::VueStyleTagStuff
                | Rule::VueScriptStuff
                | Rule::VueStyleStuff
                | Rule::VueLangGroup
                | Rule::VueTemplateBody
                | Rule::VueTemplateTag
                | Rule::VueTemplateTagEnd
                | Rule::VueCapitalized
                | Rule::VueCapitalizedAttributes
                | Rule::VueDirective
                | Rule::VueExpressionString
                | Rule::VueExpressionBare => self.search(),
            };
            if let Some(at) = self.give_back_at(found.as_ref()) {
                let content = self.top().content;
                self.tok(content, at);
                self.pos = at;
                self.given_back = Some(at);
                return;
            }
            let Some(hit) = found else {
                if self.over_budget() {
                    self.report.record(Limit::Work);
                }
                self.finish();
                return;
            };
            let before = self.pos;
            self.applied = true;
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

    /// The `while` rule of a Vue body, `^(?!\s*</script[>\s])`, at the start
    /// of the line: the body closes where it fails.
    #[cfg(feature = "lang-vue")]
    #[cold]
    #[inline(never)]
    fn vue_while(&mut self) {
        if self.pos != 0 || vue_rules::script_end(self.s, self.skip_ascii_ws(0)) {
            self.frames.pop();
        }
    }

    /// Where the line goes back to the including grammar before `found`, the
    /// root's next match, while only the root is open: see [`GiveBack`].
    fn give_back_at(&self, found: Option<&Hit>) -> Option<usize> {
        let give_back = self.give_back?;
        if self.frames.len() != 1 {
            return None;
        }
        match give_back {
            GiveBack::AtRoot => None,
            GiveBack::AfterTag => self.after_tag(found),
            GiveBack::Halts(&(end, after)) => self.halt_before(found, end, after),
        }
    }

    /// Where the end `(?<=>)` of [`GiveBack::AfterTag`] matches before `found`:
    /// after a `>` at or past the position.
    fn after_tag(&self, found: Option<&Hit>) -> Option<usize> {
        let before = found.map_or(self.lim, |hit| hit.start.saturating_sub(1));
        let from = self.pos.max(1) - 1;
        let window = self.s.get(from..before)?;
        let at = memchr(b'>', window);
        self.spend(at.unwrap_or(window.len()) / 16);
        at.map(|at| from + at + 1)
    }

    /// Where [`GiveBack::Halts`] takes the line back before `found`, the root's
    /// next match (see [`crate::language::halt_before`]).
    fn halt_before(
        &self,
        found: Option<&Hit>,
        end: &dyn Halt,
        after: Option<&dyn Halt>,
    ) -> Option<usize> {
        if self.over_budget() {
            return None;
        }
        let to = found.map_or(self.lim, |hit| hit.start);
        let (halted, scanned) =
            crate::language::halt_before(self.s, self.pos, to, self.lim, end, after);
        self.spend(scanned);
        halted
    }

    /// The first match from the position.
    /// At each candidate position it tries the innermost rule's end pattern
    /// and then its patterns in order.
    /// The injection is tried where it comes before every one of them, as
    /// [`Injection::precedes`] ranks it.
    fn search(&mut self) -> Option<Hit> {
        let rule = self.top().rule;
        let injection = self.top().injection;
        let end = self.end_pos();
        let inject = match self.document {
            Document::Erb if injection == Injection::ShutOut => None,
            Document::Erb => self.find_erb(self.pos),
            Document::Blade => self.find_blade(injection),
            Document::Html | Document::DerivativeFirst | Document::Derivative
                if self.top().no_inject =>
            {
                None
            }
            Document::Html | Document::DerivativeFirst | Document::Derivative => {
                self.find_lt(self.pos)
            }
            Document::Vue => None,
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
                && injection.precedes(at, candidate)
            {
                return self.injection(at);
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
        inject.and_then(|at| self.injection(at))
    }

    /// The injection's match at `at`: ERB's in an ERB document, the flagged
    /// `<` otherwise.
    fn injection(&self, at: usize) -> Option<Hit> {
        match self.document {
            Document::Erb => self.erb_hit(at),
            Document::Blade => self.blade_hit(at),
            Document::Html | Document::DerivativeFirst | Document::Derivative => {
                Some(Hit::new(Pattern::Inject, at, at + 1))
            }
            Document::Vue => None,
        }
    }

    /// The CSS or JavaScript of the innermost rule.
    /// It is the rest of the line up to the end tag that the embedded lexer
    /// halts at.
    /// That end tag is then the rule's end.
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
        let lent = self.steps.lend(self.limits);
        let mut cx = Context {
            scratch: &mut *self.scratch,
            arena: &mut *self.arena,
            limits: &lent,
            out: &mut *self.out,
            report: &mut *self.report,
            first_token: self.first_token,
        };
        let line = Line {
            text: self.text,
            start: pos,
            embedded: false,
        };
        let script = match self.document {
            Document::Erb => Script::Erb,
            Document::Html
            | Document::DerivativeFirst
            | Document::Derivative
            | Document::Blade
            | Document::Vue
                if self
                    .frames
                    .first()
                    .is_some_and(|root| root.payload == ROOT_SVELTE) =>
            {
                Script::Svelte
            }
            Document::Html
            | Document::DerivativeFirst
            | Document::Derivative
            | Document::Blade
            | Document::Vue => Script::Plain,
        };
        let (next, halt) = if css {
            Self::lex_style(&mut cx, line, inner, matches!(script, Script::Erb))
        } else {
            Self::lex_script(&mut cx, line, inner, script)
        };
        let next = self.steps.take_back(next);
        self.advance_to(pos, halt);
        let Some(at) = halt else {
            self.keep_inner(next);
            return None;
        };
        let end_tag = if css { style_end } else { script_end };
        if end_tag(self.s, at) {
            return Some(Hit::new(Pattern::End, at, at));
        }
        self.keep_inner(next);
        self.erb_hit(at)
    }

    /// The text of a Vue body or directive value: the rest of the line up to
    /// where its rule ends.
    /// Its language's lexer reads it.
    /// A body in a language that nishiki does not lex is plain text.
    #[cfg(feature = "lang-vue")]
    #[cold]
    #[inline(never)]
    fn vue_embedded(&mut self) -> Option<Hit> {
        let top = *self.top();
        let parent = self
            .frames
            .get(self.frames.len().saturating_sub(2))
            .copied()
            .unwrap_or(top);
        let pos = self.pos;
        self.tok(top.content, pos);
        let (lexed, halt) = vue_rules::target(top.rule, parent.rule, parent.payload);
        let end = halt.unwrap_or(vue_rules::never);
        let Some(lexed) = lexed else {
            let at = vue_rules::halt_at(&self.s[..self.lim], pos, end);
            self.spend(at.unwrap_or(self.lim).saturating_sub(pos) / 16 + 1);
            let Some(at) = at else {
                self.finish();
                self.pos = self.lim;
                return None;
            };
            self.tok(top.content, at);
            self.pos = at;
            return Some(Hit::new(Pattern::End, at, at));
        };
        let mut inner = self.arena.frame(top.state).payload;
        if inner == 0 {
            inner = vue_start(self.arena, lexed, top.content);
        }
        if pos >= self.lim {
            return None;
        }
        let lent = self.steps.lend(self.limits);
        let mut cx = Context {
            scratch: &mut *self.scratch,
            arena: &mut *self.arena,
            limits: &lent,
            out: &mut *self.out,
            report: &mut *self.report,
            first_token: self.first_token,
        };
        let line = Line {
            text: self.text,
            start: pos,
            embedded: false,
        };
        let (next, stopped) = vue_lex(&mut cx, lexed, line, inner, end);
        let next = self.steps.take_back(next);
        match stopped {
            Some(Stopped {
                at,
                reason: StopReason::Halt,
            }) => {
                self.advance_to(pos, Some(at));
                Some(Hit::new(Pattern::End, at, at))
            }
            Some(Stopped {
                at,
                reason: StopReason::PopHost,
            }) => {
                self.advance_to(pos, None);
                self.pop_body(at);
                None
            }
            None => {
                self.advance_to(pos, None);
                self.keep_inner(next);
                None
            }
        }
    }

    /// Pops the Vue body on top, as vscode-textmate does where the language
    /// it embeds matched an empty text at its root without advancing
    /// ([`StopReason::PopHost`]).
    /// The rest of the line from `at` sits on the rule below, in place of the
    /// tokens the language gave it.
    /// The line stops there.
    #[cfg(feature = "lang-vue")]
    #[cold]
    fn pop_body(&mut self, at: usize) {
        cut_tokens(self.out, self.first_token, at);
        self.last = at;
        self.frames.pop();
        self.finish();
        self.pos = self.lim;
    }

    /// Without Vue, no frame is a Vue body.
    #[cfg(not(feature = "lang-vue"))]
    fn vue_embedded(&mut self) -> Option<Hit> {
        self.search()
    }

    /// Moves past the text that an embedded lexer took from `pos`: up to its
    /// halt, or the end of the text.
    fn advance_to(&mut self, pos: usize, halt: Option<usize>) {
        let to = halt.unwrap_or(self.lim);
        self.spend(to.saturating_sub(pos) + 1);
        if to > pos {
            self.last = to;
            self.pos = to;
        }
    }

    /// Keeps `inner`, the state of the language the innermost rule embeds,
    /// as that rule's payload, for the rest of the line or the next one.
    fn keep_inner(&mut self, inner: u32) {
        let top = *self.top();
        let parent = self.frames[self.frames.len() - 2].state;
        let state = self
            .arena
            .push(parent, top.rule.code(), inner, top.content)
            .unwrap_or_else(|| {
                self.report.record(Limit::Nodes);
                top.state
            });
        let last = self.frames.len() - 1;
        self.frames[last].state = state;
    }

    /// Lexes CSS up to `</style>`, and in an ERB document up to ERB code too.
    /// It stays out of line, so `tokenize` never builds the `&dyn Halt` it
    /// passes. Inlined, that value cost every HTML line a store.
    #[cold]
    #[inline(never)]
    fn lex_style(
        cx: &mut Context<'_>,
        line: Line<'_>,
        state: u32,
        in_erb: bool,
    ) -> (Finished, Option<usize>) {
        let (next, stopped) = if in_erb {
            crate::css::lex_until(cx, line, state, &style_end_or_erb, None)
        } else {
            crate::css::lex_until(cx, line, state, &style_end, None)
        };
        (next, resumes_at(stopped))
    }

    /// Lexes JavaScript up to `</script>`, as `script` says.
    /// It stays out of line, as [`Lexer::lex_style`] does.
    #[cold]
    #[inline(never)]
    fn lex_script(
        cx: &mut Context<'_>,
        line: Line<'_>,
        state: u32,
        script: Script,
    ) -> (Finished, Option<usize>) {
        let dialect = crate::typescript::Dialect::JavaScript;
        let (next, stopped) = match script {
            Script::Plain => {
                crate::typescript::lex_until(cx, dialect, line, state, &script_end, None)
            }
            Script::Erb => crate::typescript::lex_until(
                cx,
                dialect,
                line,
                state,
                &script_end,
                Some(&erb::begins),
            ),
            #[cfg(feature = "lang-svelte")]
            Script::Svelte => {
                crate::typescript::lex_until_svelte(cx, dialect, line, state, &script_end, None)
            }
            #[cfg(not(feature = "lang-svelte"))]
            Script::Svelte => {
                crate::typescript::lex_until(cx, dialect, line, state, &script_end, None)
            }
        };
        (next, resumes_at(stopped))
    }

    /// Lexes the Ruby of ERB code up to the code's end.
    /// It stays out of line, as [`Lexer::lex_style`] does.
    #[cold]
    #[inline(never)]
    fn lex_erb_ruby(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> (Finished, Option<usize>) {
        let (next, stopped) = erb::lex_ruby(cx, line, state, &erb::ruby_halt);
        (next, resumes_at(stopped))
    }

    /// The Ruby of ERB code: the rest of the line up to the code's end, or up
    /// to the comment pattern.
    /// The code tries that pattern before Ruby's patterns.
    /// Both halt the Ruby lexer.
    fn ruby(&mut self) -> Option<Hit> {
        let top = *self.top();
        let pos = self.pos;
        self.tok(top.content, pos);
        let mut inner = self.arena.frame(top.state).payload;
        if inner == 0 {
            inner = erb::ruby_start(self.arena, top.content);
        }
        if pos >= self.lim {
            return None;
        }
        let lent = self.steps.lend(self.limits);
        let mut cx = Context {
            scratch: &mut *self.scratch,
            arena: &mut *self.arena,
            limits: &lent,
            out: &mut *self.out,
            report: &mut *self.report,
            first_token: self.first_token,
        };
        let line = Line {
            text: self.text.get(..self.lim).unwrap_or(self.text),
            start: pos,
            embedded: false,
        };
        let (next, halt) = Self::lex_erb_ruby(&mut cx, line, inner);
        let next = self.steps.take_back(next);
        self.advance_to(pos, halt);
        let text = &self.s[..self.lim];
        let Some(at) = halt else {
            self.keep_inner(next);
            return None;
        };
        if let Some(end) = erb::code_end(text, at) {
            return Some(Hit::new(Pattern::End, at, end).with([end - 1, 0, 0, 0, 0], 0));
        }
        self.keep_inner(next);
        let to = erb::hash_comment_end(text, at)?;
        Some(Hit::new(Pattern::ErbHash, at, to))
    }

    /// Lexes the PHP of a Blade rule up to its halts: the rule's end, and
    /// Blade's injection where it applies.
    /// It stays out of line, as [`Lexer::lex_style`] does.
    #[cold]
    #[inline(never)]
    fn lex_blade_php(
        cx: &mut Context<'_>,
        line: Line<'_>,
        state: u32,
        rule: blade::BladeRule,
        injection: Injection,
    ) -> (Finished, Option<usize>) {
        let halt = PhpHalt { rule, injection };
        let begins: &dyn Halt = &blade::begins;
        let after = (injection == Injection::Applies).then_some(begins);
        let (next, stopped) = blade::lex_php(cx, line, state, (&halt, after));
        (next, resumes_at(stopped))
    }

    /// The PHP of a Blade rule: the rest of the line up to the rule's end.
    /// It stops sooner where Blade's injection opens, at any depth of the PHP.
    /// The PHP's state stays the rule's payload while a rule of the injection
    /// is open inside it.
    #[cold]
    #[inline(never)]
    fn blade_php(&mut self) -> Option<Hit> {
        let top = *self.top();
        let rule = top.rule.blade()?;
        let pos = self.pos;
        self.tok(top.content, pos);
        let mut inner = self.arena.frame(top.state).payload;
        if inner == 0 {
            if pos >= self.lim {
                return None;
            }
            inner = blade::php_start(self.arena, top.content);
        }
        let lent = self.steps.lend(self.limits);
        let mut cx = Context {
            scratch: &mut *self.scratch,
            arena: &mut *self.arena,
            limits: &lent,
            out: &mut *self.out,
            report: &mut *self.report,
            first_token: self.first_token,
        };
        let line = Line {
            text: self.text.get(..self.lim).unwrap_or(self.text),
            start: pos,
            embedded: false,
        };
        let (next, halt) = Self::lex_blade_php(&mut cx, line, inner, rule, top.injection);
        let next = self.steps.take_back(next);
        self.advance_to(pos, halt);
        let Some(at) = halt else {
            self.keep_inner(next);
            return None;
        };
        let text = &self.s[..self.lim];
        if top.injection != Injection::ShutOut
            && let Some((blade, end)) = blade::at(text, at)
        {
            self.keep_inner(next);
            return Some(Hit::new(Pattern::blade(blade), at, end));
        }
        rule.php_end(text, at)
            .map(|end| Hit::new(Pattern::End, at, end))
    }

    /// The stack a rule opened inside `top` sits on in a Blade document.
    /// Inside a Blade rule's PHP, it is the stack of the PHP's innermost open
    /// rule.
    /// That is because Blade's injection opens at any depth of the PHP.
    #[cold]
    #[inline(never)]
    pub(super) fn blade_base(&self, rule: Rule, state: u32, content: u32) -> u32 {
        if rule.blade().is_some_and(blade::BladeRule::holds_php) {
            let inner = self.arena.frame(state).payload;
            if inner != 0 {
                return self.arena.frame(inner).stack;
            }
        }
        content
    }

    // Applying matches.

    /// Applies a match; true when the line stops.
    fn apply(&mut self, hit: Hit) -> bool {
        match hit.pattern {
            Pattern::End => self.close(&hit),
            Pattern::Inject => self.inject(&hit),
            Pattern::ElementVoid => self.element_void(&hit),
            Pattern::EntityNamed
            | Pattern::EntityDecimal
            | Pattern::EntityHexadecimal
            | Pattern::EntityAmbiguous
            | Pattern::AttributeIllegal
            | Pattern::EqualsIllegal
            | Pattern::CommentIllegal
            | Pattern::DoctypeKeyword
            | Pattern::DoctypeWord
            | Pattern::Unquoted
            | Pattern::UnquotedCss
            | Pattern::UnquotedJs
            | Pattern::JsText
            | Pattern::ErbHash
            | Pattern::BladeMatch(_)
            | Pattern::Vue(VuePattern::Modifier) => self.match_rule(&hit),
            Pattern::Zero(_)
            | Pattern::Processing
            | Pattern::Comment
            | Pattern::Doctype
            | Pattern::Cdata
            | Pattern::StyleWrap
            | Pattern::ScriptWrap
            | Pattern::Tag
            | Pattern::Element
            | Pattern::StyleTag
            | Pattern::ScriptTag
            | Pattern::ScriptJs
            | Pattern::Attribute
            | Pattern::Equals
            | Pattern::Quote
            | Pattern::DoctypeQuote
            | Pattern::JsLine
            | Pattern::JsBlock
            | Pattern::Erb(_)
            | Pattern::Vue(
                VuePattern::CommentKv
                | VuePattern::Comment
                | VuePattern::SelfClosing
                | VuePattern::Block
                | VuePattern::Element
                | VuePattern::LangGroup
                | VuePattern::TemplateTag
                | VuePattern::TemplateTagEnd
                | VuePattern::Capitalized
                | VuePattern::DirectiveControl
                | VuePattern::DirectiveOriginal
                | VuePattern::ExprString
                | VuePattern::ExprBare,
            )
            | Pattern::Blade(_) => self.open(&hit),
        }
    }

    /// Applies the injection: the `<` is a token of its own.
    fn inject(&mut self, hit: &Hit) -> bool {
        let top = *self.top();
        self.tok(top.content, hit.start);
        self.capture(top.content, hit.start, hit.end, &[S::BadAngleBracket]);
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

    /// Applies the begin of a rule; true when the line stops.
    ///
    /// A rule the depth limit keeps from opening leaves its text plain.
    /// A rule that matched nothing would match again where it did.
    /// The search therefore goes on after one character.
    /// At the end of the line no character is left, so the line stops.
    ///
    /// A pattern that names no rule leaves its text plain.
    /// A rule that opens again where it opened without advancing stops the
    /// line.
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
            if next <= self.pos {
                return true;
            }
            self.pos = next;
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
        let (base, injection) = self.frame_links(&top, rule);
        let name = self.name_stack(base, rule, payload);
        let content = self.content_stack(name, rule, payload);
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
            injection,
        });
    }

    /// What a rule opened inside `top` takes from it.
    /// - The stack it sits on: `top`'s content. Inside a `<script>` element's
    ///   code, it also has the `source.js` that the JavaScript lexer adds to
    ///   its own stack. ERB opened there keeps it.
    /// - How a template's injection applies inside it: by ERB's selectors, or
    ///   by Blade's in a Blade document ([`Lexer::blade_links`]).
    #[expect(
        clippy::inline_always,
        reason = "measured at one codegen unit: \
                  with two call sites it went out of line and cost HTML 124k instructions in calls"
    )]
    #[inline(always)]
    fn frame_links(&mut self, top: &Frame, rule: Rule) -> (u32, Injection) {
        if matches!(self.document, Document::Blade) {
            return self.blade_links(top.rule, top.state, top.content, top.injection, rule);
        }
        let base = if top.rule == Rule::BodyJs {
            self.path(top.content, &[S::SourceJs])
        } else {
            top.content
        };
        (base, rule.injection(top.injection))
    }

    /// [`Lexer::frame_links`] in a Blade document.
    /// - Inside a Blade rule's PHP, the stack is the PHP's
    ///   ([`Lexer::blade_base`]).
    /// - Under a sealed Blade heredoc, no rule lets the injection back in.
    ///
    /// It stays out of line, so the frame push of other documents tests the
    /// document once.
    #[cold]
    #[inline(never)]
    fn blade_links(
        &mut self,
        top_rule: Rule,
        state: u32,
        content: u32,
        parent: Injection,
        rule: Rule,
    ) -> (u32, Injection) {
        let base = if top_rule == Rule::BodyJs {
            self.path(content, &[S::SourceJs])
        } else {
            self.blade_base(top_rule, state, content)
        };
        let injection = if parent == Injection::ShutOut
            && self.frames[0].payload == state::ROOT_BLADE_HEREDOC_SEALED
        {
            Injection::ShutOut
        } else {
            rule.blade_injection(parent)
        };
        (base, injection)
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
    /// `namespace`.
    /// The grammar does so for a capture group of an element's start tag.
    /// It lexes on that text alone, under `base`, and the rules it leaves open
    /// end with it.
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
            injection: top.injection,
        });
        self.tok(top.content, from);
        self.lim = to;
        self.lf = false;
        self.pos = from;
        self.anchor = None;
        self.next_lt.set((usize::MAX, None));
        self.next_erb.set((usize::MAX, None));
        self.tokenize();
        self.tok(self.top().content, to);
        self.frames.truncate(depth);
        self.lim = lim;
        self.lf = lf;
        self.pos = pos;
        self.anchor = anchor;
        self.next_lt.set((usize::MAX, None));
        self.next_erb.set((usize::MAX, None));
        self.last = self.last.max(to);
    }

    /// Lexes the text `from..to` with the JavaScript lexer, from a fresh
    /// state, under `base`.
    /// It does so as `TextMate` lexes a capture group again.
    /// The text before `from` stays visible to lookbehinds, and no line feed
    /// follows `to`.
    pub(super) fn javascript(&mut self, from: usize, to: usize, base: u32) {
        if from >= to {
            return;
        }
        self.tok(base, from);
        self.spend(to - from);
        let state = crate::typescript::start_embedded(self.arena, base);
        let lent = self.steps.lend(self.limits);
        let mut cx = Context {
            scratch: &mut *self.scratch,
            arena: &mut *self.arena,
            limits: &lent,
            out: &mut *self.out,
            report: &mut *self.report,
            first_token: self.first_token,
        };
        let text = self.text.get(..to).unwrap_or(self.text);
        let line = Line {
            text,
            start: from,
            embedded: false,
        };
        let finished = crate::typescript::lex_capture(&mut cx, line, state);
        self.steps.take_back(finished);
        self.last = self.last.max(to);
    }
}

/// The start state of `lexed` for a Vue body or directive value whose text
/// sits on `base`.
#[cfg(feature = "lang-vue")]
fn vue_start(arena: &mut Arena, lexed: vue_rules::Lexed, base: u32) -> u32 {
    use vue_rules::Lexed;
    match lexed {
        Lexed::Css => crate::css::start_embedded(arena, base),
        Lexed::Scss => crate::scss::start_embedded(arena, base),
        Lexed::Less => crate::less::start_embedded(arena, base),
        Lexed::Script(
            crate::typescript::Dialect::JavaScript
            | crate::typescript::Dialect::TypeScript
            | crate::typescript::Dialect::Jsx
            | crate::typescript::Dialect::Tsx,
        ) => crate::typescript::start_embedded(arena, base),
        Lexed::Yaml => crate::yaml::start_embedded(arena, base),
        Lexed::Graphql => crate::graphql::start_embedded(arena, base),
        #[cfg(feature = "lang-sass")]
        Lexed::Sass => crate::Language::Sass.start_embedded(arena, base),
        #[cfg(feature = "lang-coffee")]
        Lexed::Coffee => crate::Language::CoffeeScript.start_embedded(arena, base),
        Lexed::Expression => crate::typescript::start_expression(arena, base),
    }
}

/// Lexes the text of a Vue body or directive value with `lexed`, from
/// `state`, up to where `end` holds while the lexer has no rule open.
#[cfg(feature = "lang-vue")]
fn vue_lex(
    cx: &mut Context<'_>,
    lexed: vue_rules::Lexed,
    line: Line<'_>,
    state: u32,
    end: vue_rules::VueHalt,
) -> (Finished, Option<Stopped>) {
    use vue_rules::Lexed;
    match lexed {
        Lexed::Css => crate::css::lex_until(cx, line, state, &end, None),
        Lexed::Scss => crate::scss::lex_until(cx, line, state, &end, None),
        Lexed::Less => crate::less::lex_until(cx, line, state, &end, None),
        Lexed::Script(dialect) => {
            crate::typescript::lex_until(cx, dialect, line, state, &end, None)
        }
        Lexed::Expression => crate::typescript::lex_until(
            cx,
            crate::typescript::Dialect::TypeScript,
            line,
            state,
            &end,
            None,
        ),
        Lexed::Yaml => crate::yaml::lex_until(cx, line, state, &end, None),
        Lexed::Graphql => crate::graphql::lex_until(cx, line, state, &end, None),
        #[cfg(feature = "lang-sass")]
        Lexed::Sass => engine_until(cx, crate::Language::Sass, line, state, &end),
        #[cfg(feature = "lang-coffee")]
        Lexed::Coffee => engine_until(cx, crate::Language::CoffeeScript, line, state, &end),
    }
}

/// Lexes a Vue body in `language`, through its `Language` row, like
/// [`vue_lex`].
/// A language that cannot halt lexes the whole line, as `rules::lex_until`
/// does.
#[cfg(all(
    feature = "lang-vue",
    any(feature = "lang-sass", feature = "lang-coffee")
))]
fn engine_until(
    cx: &mut Context<'_>,
    language: crate::Language,
    line: Line<'_>,
    state: u32,
    end: &dyn Halt,
) -> (Finished, Option<Stopped>) {
    language
        .lex_until(cx, line, state, end, None)
        .unwrap_or_else(|| (language.lex_embedded(cx, line, state), None))
}

/// Where the HTML goes on after a language it embeds in a `<style>`,
/// `<script>` or ERB body stopped as `stopped` says: where a halt held.
/// A language that popped the body lexed the rest of the line, as one that
/// did not stop did.
fn resumes_at(stopped: Option<Stopped>) -> Option<usize> {
    match stopped {
        Some(Stopped {
            at,
            reason: StopReason::Halt,
        }) => Some(at),
        Some(Stopped {
            reason: StopReason::PopHost,
            ..
        })
        | None => None,
    }
}

/// The end of a `<style>` element's CSS: `</style`, in any case.
fn style_end(s: &[u8], p: usize) -> bool {
    end_tag_at(s, p, b"</style")
}

/// The end of a `<style>` element's CSS in an ERB document, or a place where
/// ERB's injection opens.
fn style_end_or_erb(s: &[u8], p: usize) -> bool {
    style_end(s, p) || erb::begins(s, p)
}

/// The end of a `<script>` element's JavaScript: `</script`, in any case.
fn script_end(s: &[u8], p: usize) -> bool {
    end_tag_at(s, p, b"</script")
}

/// Whether the end tag `tag`, `</` and a lower-case name, stands at `p` of
/// `s` in any case. It is compared as ASCII. A fold such as `ſ` can start
/// with a byte above ASCII. Only a `</` followed by such a byte is read
/// through the folds ([`case_fold::literal_end`]), out of line.
fn end_tag_at(s: &[u8], p: usize, tag: &[u8]) -> bool {
    s.get(p..p + tag.len())
        .is_some_and(|text| text.eq_ignore_ascii_case(tag))
        || s.get(p..p + 2) == Some(b"</")
            && s.get(p + 2).is_some_and(|b| !b.is_ascii())
            && folded_end_tag_at(s, p, tag)
}

/// [`end_tag_at`] through the folds.
#[cold]
fn folded_end_tag_at(s: &[u8], p: usize, tag: &[u8]) -> bool {
    case_fold::literal_end(s, p, tag).is_some()
}

/// Packs a name's interned id with the bits that say how to spell it.
pub(super) fn packed(bits: u32, name: u32) -> u32 {
    state::pack(bits, name)
}
