//! The per-line lexer: cursor, token emission and the context stack.

use std::cell::Cell;

use memchr::memchr2;

use super::scalar;
use super::scope::S;
use super::state::Kind;
use crate::arena::{Arena, Frame};
use crate::limits::{Limit, Limits, LineReport};
use crate::{Stack, Token};

/// Lexes one line. `pos` is the end of the last token; the bytes between it
/// and the cursor are plain text of the current context.
pub(crate) struct Lexer<'a> {
    s: &'a [u8],
    out: &'a mut Vec<Token>,
    arena: &'a mut Arena,
    limits: &'a Limits,
    report: &'a mut LineReport,
    pos: usize,
    /// The current state, and its frame.
    state: u32,
    frame: Frame,
    /// Steps spent on this line, counted in scans too.
    steps: Cell<u64>,
    /// Steps this line may spend; `u64::MAX` for no limit.
    budget: u64,
    /// How many spaces start the line, and how many spaces and tabs.
    lead_spaces: usize,
    lead_blanks: usize,
    /// Where the last tab is, which decides whether a comment is taken.
    last_tab: Option<usize>,
    /// Whether `^` matches at the start of `s`. It does not when a Markdown
    /// fence inside a container hands over the rest of its line.
    line_start: bool,
}

fn is_blank(b: u8) -> bool {
    b == b' ' || b == b'\t'
}

fn is_flow_indicator(b: u8) -> bool {
    matches!(b, b',' | b'[' | b']' | b'{' | b'}')
}

/// The kinds of directive the grammar tells apart.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Directive {
    Yaml,
    Tag,
    Reserved,
}

/// How many bytes the character that starts with `lead` takes.
fn char_len(lead: u8) -> usize {
    match lead {
        0xf0.. => 4,
        0xe0.. => 3,
        0xc0.. => 2,
        _ => 1,
    }
}

/// Whether `text` is made of URI characters and `%` escapes, and the extra
/// bytes `extra` allows. No `!`: it ends a handle.
fn uri_text(text: &[u8], extra: &[u8]) -> bool {
    let mut i = 0;
    while i < text.len() {
        let b = text[i];
        if b == b'%' {
            let hex = text.get(i + 1..i + 3);
            if !hex.is_some_and(|h| h.iter().all(u8::is_ascii_hexdigit)) {
                return false;
            }
            i += 3;
            continue;
        }
        if !(b.is_ascii_alphanumeric() || b"-_.~*'():@&=+$/?#;".contains(&b) || extra.contains(&b))
        {
            return false;
        }
        i += 1;
    }
    true
}

/// Whether a tag is well formed: `!`, a shorthand `!suffix`, `!!suffix` or
/// `!handle!suffix`, or a verbatim `!<uri>`.
fn tag_valid(tag: &[u8]) -> bool {
    match tag {
        b"!" => true,
        [b'!', b'<', inner @ .., b'>'] => !inner.is_empty() && uri_text(inner, b",!"),
        [b'!', rest @ ..] => match rest.iter().position(|&b| b == b'!') {
            None => uri_text(rest, b""),
            Some(bang) => {
                let (handle, suffix) = (&rest[..bang], &rest[bang + 1..]);
                handle
                    .iter()
                    .all(|&b| b.is_ascii_alphanumeric() || b == b'-')
                    && !suffix.is_empty()
                    && uri_text(suffix, b"")
            }
        },
        _ => false,
    }
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(
        arena: &'a mut Arena,
        limits: &'a Limits,
        s: &'a [u8],
        line_start: bool,
        out: &'a mut Vec<Token>,
        state: u32,
        report: &'a mut LineReport,
    ) -> Self {
        let frame = arena.frame(state);
        let budget = limits.work.map_or(u64::MAX, |work| work.steps(s.len()));
        let lead_spaces = s.iter().take_while(|&&b| b == b' ').count();
        let lead_blanks = s.iter().take_while(|&&b| is_blank(b)).count();
        Lexer {
            s,
            out,
            arena,
            limits,
            report,
            pos: 0,
            state,
            frame,
            steps: Cell::new(0),
            budget,
            lead_spaces,
            lead_blanks,
            last_tab: memchr::memrchr(b'\t', s),
            line_start,
        }
    }

    pub(crate) fn into_state(self) -> u32 {
        self.state
    }

    pub(crate) fn run(&mut self) {
        let len = self.s.len();
        let mut p = match self.top() {
            Kind::Block => self.block_line(),
            Kind::Double => self.double_body(0),
            Kind::Single => self.single_body(0),
            Kind::Top
            | Kind::FlowSeq
            | Kind::FlowMap
            | Kind::Pair
            | Kind::Complex
            | Kind::Explicit => 0,
        };
        while p < len {
            self.spend(1);
            if self.steps.get() > self.budget {
                self.report.record(Limit::Work);
                break;
            }
            let from = p;
            p = match self.top() {
                Kind::Top | Kind::Complex => self.block_step(p),
                Kind::FlowSeq | Kind::FlowMap | Kind::Pair | Kind::Explicit => self.flow_step(p),
                Kind::Double => self.double_body(p),
                Kind::Single => self.single_body(p),
                Kind::Block => self.block_line(),
            };
            debug_assert!(p > from, "the YAML lexer made no progress at byte {from}");
        }
        self.gap(len);
    }

    // Bytes.

    /// The byte at `i`, or 0 past the end.
    fn at(&self, i: usize) -> u8 {
        self.s.get(i).copied().unwrap_or(0)
    }

    /// Whether `i` is the end of the line or a blank: what a `-`, `?` or `:`
    /// indicator needs after it.
    #[inline]
    fn blank_or_end(&self, i: usize) -> bool {
        i >= self.s.len() || self.blank_at(i) > 0
    }

    /// The length of the blank at `i`, or 0: a space, a tab, or a Unicode
    /// space, which Shiki's `\s` reads as a blank.
    #[inline]
    fn blank_at(&self, i: usize) -> usize {
        match self.s.get(i) {
            Some(&b) if b < 0xc2 => usize::from(b == b' ' || b == b'\t'),
            Some(_) => crate::space::len(&self.s[i..]),
            None => 0,
        }
    }

    /// `skip_blanks` over spaces and tabs only, for the places where the
    /// grammar spells the blank `[ \t]` and not `\s`.
    fn skip_ascii_blanks(&self, mut p: usize) -> usize {
        let from = p;
        while self.s.get(p).is_some_and(|&b| is_blank(b)) {
            p += 1;
        }
        self.spend(p - from);
        p
    }

    fn skip_blanks(&self, mut p: usize) -> usize {
        let from = p;
        loop {
            let n = self.blank_at(p);
            if n == 0 {
                break;
            }
            p += n;
        }
        self.spend(p - from);
        p
    }

    /// Counts `bytes` read ahead of the cursor against the line's work.
    fn spend(&self, bytes: usize) {
        let bytes = u64::try_from(bytes).unwrap_or(u64::MAX);
        self.steps.set(self.steps.get().saturating_add(bytes));
    }

    // Emission.

    /// Emits plain text of the current context up to `to`.
    fn gap(&mut self, to: usize) {
        self.tok(to, &[]);
    }

    /// Emits the bytes up to `to` with the current context plus `leaf`.
    fn tok(&mut self, to: usize, leaf: &[S]) {
        if to <= self.pos {
            return;
        }
        let mut stack = self.frame.stack;
        for scope in leaf {
            let Some(child) = self.arena.child(stack, scope.scope()) else {
                self.report.record(Limit::Nodes);
                break;
            };
            stack = child;
        }
        self.out.push(Token {
            end: to,
            stack: Stack(stack),
        });
        self.pos = to;
    }

    /// Emits the gap before `p`, then the bytes up to `end` with `leaf`.
    fn token(&mut self, p: usize, end: usize, leaf: &[S]) -> usize {
        self.gap(p);
        self.tok(end, leaf);
        end
    }

    // Contexts.

    fn top(&self) -> Kind {
        Kind::from_code(self.frame.kind)
    }

    /// Opens a context with `payload`; false when a limit keeps it closed.
    fn push(&mut self, kind: Kind, payload: u32) -> bool {
        let depth = usize::try_from(self.frame.depth).unwrap_or(usize::MAX);
        if self.limits.depth.is_some_and(|max| depth >= max) {
            self.report.record(Limit::Depth);
            return false;
        }
        let mut stack = Some(self.frame.stack);
        for scope in kind.scopes() {
            stack = stack.and_then(|stack| self.arena.child(stack, scope.scope()));
        }
        let Some(state) =
            stack.and_then(|stack| self.arena.push(self.state, kind.code(), payload, stack))
        else {
            self.report.record(Limit::Nodes);
            return false;
        };
        self.enter(state);
        true
    }

    fn pop(&mut self) {
        self.enter(self.frame.parent);
    }

    fn enter(&mut self, state: u32) {
        self.state = state;
        self.frame = self.arena.frame(state);
    }

    // Block scalars.

    /// A line of a block scalar's content, or the first line after it.
    /// Returns where lexing resumes: the end of the line, or 0 when the line
    /// is not content and the scalar ends.
    fn block_line(&mut self) -> usize {
        if !self.line_start {
            // The rules that continue a block scalar start with `^`, so the
            // scalar ends here, and the grammar leaves the first character
            // of the line with no scope.
            self.pop();
            let first = char_len(self.s[0]).min(self.s.len());
            self.gap(first);
            return first;
        }
        let len = self.s.len();
        let indent = self.lead_spaces;
        let blank = indent == len;
        let blank_text = self.lead_blanks == len;
        self.spend(len);
        let fixed = usize::try_from(self.frame.payload).unwrap_or(usize::MAX);
        if fixed == 0 {
            // The first line fixes the indentation, a blank one included.
            if indent == 0 {
                if !blank {
                    self.pop();
                    return 0;
                }
            } else {
                let parent = self.frame.parent;
                let stack = self.frame.stack;
                let width = u32::try_from(indent).unwrap_or(u32::MAX);
                if let Some(state) = self.arena.push(parent, Kind::Block.code(), width, stack) {
                    self.enter(state);
                }
            }
        } else if !blank_text && indent < fixed {
            self.pop();
            return 0;
        }
        self.gap(len);
        len
    }

    /// `|` or `>` at `p`: the header, whose scalar's lines follow. After the
    /// indicator come an indentation digit with an optional chomping sign, or
    /// a chomping sign alone, then a comment; anything else is a mistake.
    fn block_header(&mut self, p: usize) -> usize {
        let len = self.s.len();
        let leaf = if self.s[p] == b'|' {
            S::BlockLiteral
        } else {
            S::BlockFolded
        };
        let mut at = self.token(p, p + 1, &[leaf]);
        match self.at(at) {
            b'1'..=b'9' => {
                at = self.token(at, at + 1, &[S::IndentIndicator]);
                if matches!(self.at(at), b'+' | b'-') {
                    at = self.token(at, at + 1, &[S::Chomping]);
                }
            }
            b'+' | b'-' => at = self.token(at, at + 1, &[S::Chomping]),
            _ => {}
        }
        let rest = self.skip_ascii_blanks(at);
        if rest < len && rest > at && self.comment_at(rest) {
            self.comment(rest);
        } else if rest < len {
            self.token(at, len, &[S::InvalidCommentNewline]);
        }
        self.gap(len);
        self.push(Kind::Block, 0);
        len
    }

    // Comments.

    /// `#` at `p` to the end of the line. A comment on a line of its own is
    /// indented by a scope of its own.
    fn comment(&mut self, p: usize) -> usize {
        let len = self.s.len();
        if self.line_start && p > 0 && self.pos == 0 && p <= self.lead_blanks {
            self.tok(p, &[S::CommentLeading]);
        }
        self.gap(p);
        self.tok(p + 1, &[S::Comment, S::PunctComment]);
        self.tok(len, &[S::Comment]);
        len
    }

    /// Whether a `#` at `p` starts a comment.
    /// The grammar does not take a comment whose text holds a tab.
    fn comment_at(&self, p: usize) -> bool {
        self.s[p] == b'#'
            && if p == 0 {
                self.line_start
            } else {
                is_blank(self.s[p - 1])
            }
            && self.last_tab.is_none_or(|tab| tab < p)
    }

    // Quoted scalars.

    fn quoted(&mut self, p: usize) -> usize {
        let double = self.s[p] == b'"';
        self.gap(p);
        let kind = if double { Kind::Double } else { Kind::Single };
        if !self.push(kind, 0) {
            // The text stays plain: no context to hold the string.
            return self.token(p, p + 1, &[]);
        }
        self.tok(p + 1, &[S::PunctStrBegin]);
        if double {
            self.double_body(p + 1)
        } else {
            self.single_body(p + 1)
        }
    }

    /// The text of a double-quoted string from `p`, up to its closing quote
    /// or the end of the line.
    fn double_body(&mut self, p: usize) -> usize {
        let len = self.s.len();
        let mut i = p;
        while i < len {
            match memchr2(b'"', b'\\', &self.s[i..]) {
                None => break,
                Some(offset) => i += offset,
            }
            self.spend(1);
            if self.s[i] == b'"' {
                self.gap(i);
                self.tok(i + 1, &[S::PunctStrEnd]);
                self.pop();
                return i + 1;
            }
            // A backslash.
            if i + 1 == len {
                self.gap(i);
                self.tok(len, &[S::EscapeNewline]);
                return len;
            }
            let width = self.escape_len(i);
            if width > 0 {
                self.gap(i);
                self.tok(i + width, &[S::Escape]);
                i += width;
            } else {
                i += 1;
            }
        }
        self.gap(len);
        len
    }

    /// The length of the escape that starts at `i`, or 0 when the backslash
    /// is only text. The grammar reads `\x`, `\u` and `\U` with decimal digits
    /// only, and `\p` where YAML says `\P`.
    fn escape_len(&self, i: usize) -> usize {
        let digits = |n: usize| {
            self.s
                .get(i + 2..i + 2 + n)
                .is_some_and(|d| d.len() == n && d.iter().all(u8::is_ascii_digit))
        };
        match self.at(i + 1) {
            b'0' | b'a' | b'b' | b't' | b'n' | b'v' | b'f' | b'r' | b'e' | b' ' | b'"' | b'/'
            | b'\\' | b'N' | b'_' | b'L' | b'p' => 2,
            b'x' if digits(2) => 4,
            b'u' if digits(4) => 6,
            b'U' if digits(8) => 10,
            _ => 0,
        }
    }

    /// The text of a single-quoted string from `p`.
    fn single_body(&mut self, p: usize) -> usize {
        let len = self.s.len();
        let mut i = p;
        while let Some(offset) = memchr::memchr(b'\'', &self.s[i..]) {
            i += offset;
            self.spend(1);
            if self.at(i + 1) == b'\'' {
                self.gap(i);
                self.tok(i + 2, &[S::EscapeSingle]);
                i += 2;
            } else {
                self.gap(i);
                self.tok(i + 1, &[S::PunctStrEnd]);
                self.pop();
                return i + 1;
            }
        }
        self.gap(len);
        len
    }

    // Properties and aliases.

    /// The end of a name that starts at `from`: up to a blank, a flow
    /// indicator or a `/`.
    fn name_end(&self, from: usize) -> usize {
        let mut i = from;
        while i < self.s.len()
            && self.blank_at(i) == 0
            && !is_flow_indicator(self.s[i])
            && self.s[i] != b'/'
        {
            i += 1;
        }
        self.spend(i - from);
        i
    }

    fn in_flow(&self) -> bool {
        matches!(
            self.top(),
            Kind::FlowSeq | Kind::FlowMap | Kind::Pair | Kind::Explicit
        )
    }

    /// `&name`. An anchor without a name is an error, and so is what follows
    /// the name before the next blank.
    fn anchor(&mut self, p: usize) -> usize {
        let name = self.name_end(p + 1);
        if name == p + 1 {
            let end = self.word_end(p);
            return self.token(p, end, &[S::MetaProperty, S::InvalidTagHandle]);
        }
        let at = self.token(
            p,
            p + 1,
            &[S::MetaProperty, S::AnchorKeyword, S::PunctAnchor],
        );
        let at = self.token(at, name, &[S::MetaProperty, S::AnchorName]);
        let end = self.word_end(at);
        self.token(at, end, &[S::MetaProperty, S::InvalidAnchor])
    }

    fn tag(&mut self, p: usize) -> usize {
        let end = self.word_end(p);
        // A tag ends at a Unicode space too, but is valid only before a space,
        // a tab or the line's end.
        let leaf = if tag_valid(&self.s[p..end]) && self.s.get(end).is_none_or(|&b| is_blank(b)) {
            S::TagHandle
        } else {
            S::InvalidTagHandle
        };
        self.token(p, end, &[S::MetaProperty, leaf])
    }

    /// `*name`; a `*` without one is plain text with no scope.
    fn alias(&mut self, p: usize) -> usize {
        let end = self.name_end(p + 1);
        if end == p + 1 {
            return self.token(p, p + 1, &[]);
        }
        let at = self.token(p, p + 1, &[S::AliasKeyword, S::PunctAlias]);
        let at = self.token(at, end, &[S::AliasName]);
        if matches!(self.at(at), b'[' | b'{' | b'/') {
            // The grammar reads the opening bracket as part of the alias.
            let word = self.word_end(at);
            return self.token(at, word, &[S::InvalidAnchor]);
        }
        at
    }

    // Plain scalars.

    /// Scans the plain scalar that starts at `p`. Returns where its text ends
    /// (before the blanks that trail it) and whether a `:` that makes it a
    /// key follows.
    fn scan_plain(&self, p: usize, flow: bool) -> (usize, bool) {
        let len = self.s.len();
        let mut i = p;
        let mut end = p;
        while i < len {
            let b = self.s[i];
            let blank = self.blank_at(i);
            if blank > 0 {
                if i + blank < len && self.s[i + blank] == b'#' {
                    break;
                }
                i += blank;
                continue;
            }
            if i > p {
                if flow && is_flow_indicator(b) {
                    break;
                }
                if b == b':' {
                    let next = self.s.get(i + 1).copied();
                    let blank = self.blank_or_end(i + 1);
                    // A lone `-` or `:` cannot be a key: the scalar goes on.
                    let lone = end == p + 1 && matches!(self.s[p], b'-' | b':');
                    if (blank || (flow && next.is_some_and(is_flow_indicator))) && !lone {
                        break;
                    }
                }
            }
            i += 1;
            end = i;
        }
        self.spend(i - p);
        let key = self
            .s
            .get(self.skip_blanks_noop(end))
            .is_some_and(|&b| b == b':')
            && self.blank_or_end(self.skip_blanks_noop(end) + 1);
        (end, key)
    }

    /// `skip_ascii_blanks` without counting work.
    fn skip_ascii_blanks_noop(&self, mut p: usize) -> usize {
        while self.s.get(p).is_some_and(|&b| is_blank(b)) {
            p += 1;
        }
        p
    }

    /// `skip_blanks` without counting work: the scan has counted the bytes.
    fn skip_blanks_noop(&self, mut p: usize) -> usize {
        loop {
            let n = self.blank_at(p);
            if n == 0 {
                return p;
            }
            p += n;
        }
    }

    fn plain(&mut self, p: usize) -> usize {
        let flow = self.in_flow();
        let (end, key) = self.scan_plain(p, flow);
        // The grammar tags a key only when the colons that end it, together
        // with the one that follows it directly, pair up.
        let trailing = self.s[p..end]
            .iter()
            .rev()
            .take_while(|&&b| b == b':')
            .count();
        let key = key && (self.at(end) != b':' || trailing % 2 == 0);
        // A pair's value holds no key of its own.
        let key = key && self.top() != Kind::Pair;
        let body = if flow { S::PlainIn } else { S::PlainOut };
        let end = end.max(p + char_len(self.s[p]).min(self.s.len() - p));
        let text = &self.s[p..end];
        match scalar::constant(text) {
            Some(constant) if flow && key => self.token(p, end, &[S::MetaFlowPairKey, constant]),
            Some(constant) => self.token(p, end, &[constant]),
            None if flow && key => self.token(p, end, &[S::MetaFlowPairKey, body, S::KeyName]),
            None if key => self.token(p, end, &[body, S::KeyName]),
            None => self.token(p, end, &[body]),
        }
    }

    // Block context.

    fn block_step(&mut self, p: usize) -> usize {
        let b = self.s[p];
        if self.blank_at(p) > 0 {
            return self.skip_blanks(p);
        }
        if p == 0 && self.line_start && self.top() == Kind::Top {
            if let Some(end) = self.document_marker() {
                return end;
            }
            if b == b'%' {
                return self.directive();
            }
        }
        match b {
            b'#' if self.comment_at(p) => self.comment(p),
            b'-' if self.blank_or_end(p + 1) => self.token(p, p + 1, &[S::PunctSeqItem]),
            b'?' => self.complex_key(p),
            b':' if self.top() == Kind::Complex => {
                // A `:` after the key's text is a mistake; one that starts
                // the line is the value indicator.
                let leaf = if self.line_start && p <= self.lead_spaces {
                    S::PunctKeyValue
                } else {
                    S::InvalidNewline
                };
                let end = self.token(p, p + 1, &[leaf]);
                self.pop();
                end
            }
            b':' if self.blank_or_end(p + 1) => self.token(p, p + 1, &[S::PunctKeyValue]),
            b'&' => self.anchor(p),
            b'!' => self.tag(p),
            b'*' => self.alias(p),
            b'"' | b'\'' => self.quoted(p),
            b'[' | b'{' => self.open_flow(p),
            b'|' | b'>' => self.block_header(p),
            b'#' | b',' | b']' | b'}' | b'@' | b'`' | b'%' => self.token(p, p + 1, &[]),
            _ => self.plain(p),
        }
    }

    /// `---` or `...` at the start of the line, and the node after it.
    fn document_marker(&mut self) -> Option<usize> {
        let leaf = match self.s.get(..3)? {
            b"---" => S::DocBegin,
            b"..." => S::DocEnd,
            _ => return None,
        };
        Some(self.token(0, 3, &[leaf]))
    }

    /// `%` at the start of the line: a directive. Each word has a scope of its
    /// own, and what the grammar does not expect is marked, with every word
    /// after it.
    fn directive(&mut self) -> usize {
        let len = self.s.len();
        let meta = |leaf: S| [S::MetaDirective, leaf];
        let mut at = self.token(0, 1, &meta(S::PunctDirective));
        let word_chars = |b: u8| b.is_ascii_alphanumeric() || b == b'_' || b >= 0x80;
        let name_end = self.run_end(at, word_chars);
        let name = self.s[at..name_end].to_vec();
        let version = self.version_len(self.skip_ascii_blanks_noop(name_end));
        let kind = match name.as_slice() {
            b"YAML" if version > 0 => Directive::Yaml,
            b"TAG" => Directive::Tag,
            _ => Directive::Reserved,
        };
        let mut bad = name_end == at;
        if bad {
            // No name: every word is a mistake.
        } else {
            let leaf = match kind {
                Directive::Yaml => S::DirectiveYaml,
                Directive::Tag => S::DirectiveTag,
                Directive::Reserved => S::DirectiveReserved,
            };
            at = self.token(at, name_end, &meta(leaf));
            if self.s.get(at).is_some_and(|&b| !is_blank(b)) {
                bad = true;
            }
        }
        let mut index = 0;
        loop {
            let start = self.skip_blanks(at);
            if start >= len || (self.s[start] == b'#' && self.comment_at(start)) {
                break;
            }
            let word = self.word_end(start);
            self.token(at, start, &[S::MetaDirective]);
            let mut end = word;
            let mut leaf = S::InvalidUnrecognized;
            if !bad {
                (end, leaf) = match (kind, index) {
                    (Directive::Yaml, 0) => (start + self.version_len(start), S::YamlVersion),
                    (Directive::Tag, 0) => match self.handle_len(start) {
                        0 => (word, S::InvalidUnrecognized),
                        n => (start + n, S::TagHandle),
                    },
                    (Directive::Tag, 1) => {
                        let ok = self.prefix_len(start, word);
                        if ok == 0 {
                            (word, S::InvalidUnrecognized)
                        } else {
                            (start + ok, S::TagPrefix)
                        }
                    }
                    (Directive::Reserved, 0) => (self.run_end(start, word_chars), S::DirectiveName),
                    (Directive::Reserved, _) => {
                        (self.run_end(start, word_chars), S::DirectiveParam)
                    }
                    (Directive::Yaml | Directive::Tag, _) => (word, S::InvalidUnrecognized),
                };
                if end == start {
                    (end, leaf) = (word, S::InvalidUnrecognized);
                }
            }
            at = self.token(start, end, &meta(leaf));
            if leaf == S::InvalidUnrecognized {
                bad = true;
            } else if end < word {
                // The rest of the word is a mistake.
                at = self.token(end, word, &meta(S::InvalidUnrecognized));
                bad = true;
            }
            index += 1;
        }
        let start = self.skip_blanks(at);
        self.token(at, start, &[S::MetaDirective]);
        if start < len {
            // A `#` that is no comment is read as the start of the next node.
            return if self.comment_at(start) {
                self.comment(start)
            } else {
                start
            };
        }
        len
    }

    /// The end of the run of bytes from `from` that `class` accepts.
    fn run_end(&self, from: usize, class: fn(u8) -> bool) -> usize {
        let mut i = from;
        while i < self.s.len() && class(self.s[i]) && self.blank_at(i) == 0 {
            i += 1;
        }
        self.spend(i - from);
        i
    }

    /// How much of the word `from..word` is a tag prefix: URI characters
    /// and `%` escapes.
    fn prefix_len(&self, from: usize, word: usize) -> usize {
        let mut i = from;
        while i < word {
            let b = self.s[i];
            let escape = b == b'%'
                && self
                    .s
                    .get(i + 1..i + 3)
                    .is_some_and(|h| h.iter().all(u8::is_ascii_hexdigit));
            if escape {
                i += 3;
            } else if b.is_ascii_alphanumeric() || b"-_.~*'():@&=+$/?#;,[]!".contains(&b) {
                i += 1;
            } else {
                break;
            }
        }
        i - from
    }

    /// The length of `[0-9]+\.[0-9]+` at `from`, or 0.
    fn version_len(&self, from: usize) -> usize {
        let digits = |i: usize| self.run_end(i, |b| b.is_ascii_digit());
        let major = digits(from);
        if major == from || self.at(major) != b'.' {
            return 0;
        }
        let minor = digits(major + 1);
        if minor == major + 1 { 0 } else { minor - from }
    }

    /// The length of the tag handle `!`, `!!` or `!name!` at `from`.
    fn handle_len(&self, from: usize) -> usize {
        if self.at(from) != b'!' {
            return 0;
        }
        let name = self.run_end(from + 1, |b| b.is_ascii_alphanumeric() || b == b'-');
        if self.at(name) == b'!' {
            name + 1 - from
        } else {
            1
        }
    }

    /// The end of the run of non-blank bytes that starts at `from`.
    fn word_end(&self, from: usize) -> usize {
        let mut i = from;
        while i < self.s.len() && self.blank_at(i) == 0 {
            i += 1;
        }
        self.spend(i - from);
        i
    }

    /// `? ` in a block mapping: the explicit key.
    fn complex_key(&mut self, p: usize) -> usize {
        self.gap(p);
        if self.top() != Kind::Complex && !self.push(Kind::Complex, 0) {
            return self.token(p, p + 1, &[]);
        }
        self.tok(p + 1, &[]);
        p + 1
    }

    // Flow context.

    fn open_flow(&mut self, p: usize) -> usize {
        let seq = self.s[p] == b'[';
        self.gap(p);
        let kind = if seq { Kind::FlowSeq } else { Kind::FlowMap };
        if !self.push(kind, 0) {
            return self.token(p, p + 1, &[]);
        }
        let leaf = if seq {
            S::PunctSeqBegin
        } else {
            S::PunctMapBegin
        };
        self.tok(p + 1, &[leaf]);
        p + 1
    }

    fn flow_step(&mut self, p: usize) -> usize {
        let b = self.s[p];
        if self.blank_at(p) > 0 {
            return self.skip_blanks(p);
        }
        match b {
            b'#' if self.comment_at(p) => self.comment(p),
            b'[' | b'{' => self.open_flow(p),
            b']' | b'}' => self.close_flow(p),
            b'?' => {
                if self.top() == Kind::Pair {
                    let bare = self.blank_or_end(p + 1) || is_flow_indicator(self.at(p + 1));
                    return if bare {
                        self.token(p, p + 1, &[])
                    } else {
                        self.plain(p)
                    };
                }
                self.gap(p);
                if self.push(Kind::Explicit, 0) {
                    self.tok(p + 1, &[S::PunctKeyValueBegin]);
                    p + 1
                } else {
                    self.token(p, p + 1, &[])
                }
            }
            b'-' if self.blank_or_end(p + 1) || is_flow_indicator(self.at(p + 1)) => {
                self.token(p, p + 1, &[])
            }
            b'#' | b'|' | b'>' | b'@' | b'`' | b'%' => self.token(p, p + 1, &[]),
            b',' => {
                self.gap(p);
                self.pop_pair_and_explicit();
                let leaf = if self.top() == Kind::FlowMap {
                    S::PunctMapSep
                } else {
                    S::PunctSeqSep
                };
                self.token(p, p + 1, &[leaf])
            }
            b':' if self.at(p + 1) == 0
                || self.blank_at(p + 1) > 0
                || is_flow_indicator(self.at(p + 1)) =>
            {
                self.pair(p)
            }
            b'&' => self.anchor(p),
            b'!' => self.tag(p),
            b'*' => self.alias(p),
            b'"' | b'\'' => self.quoted(p),
            _ => self.plain(p),
        }
    }

    /// `:` of a flow pair: it opens the pair's value.
    fn pair(&mut self, p: usize) -> usize {
        if self.top() == Kind::Pair {
            return self.token(p, p + 1, &[]);
        }
        self.gap(p);
        self.tok(p + 1, &[S::MetaFlowPair, S::PunctKeyValue]);
        self.push(Kind::Pair, 0);
        p + 1
    }

    /// Closes the open pair and explicit key, which end where their
    /// collection's next entry or its end begins.
    fn pop_pair_and_explicit(&mut self) {
        while matches!(self.top(), Kind::Pair | Kind::Explicit) {
            self.pop();
        }
    }

    /// `]` or `}`: the end of the flow collection it matches.
    fn close_flow(&mut self, p: usize) -> usize {
        let want = if self.s[p] == b']' {
            Kind::FlowSeq
        } else {
            Kind::FlowMap
        };
        self.gap(p);
        self.pop_pair_and_explicit();
        if self.top() != want {
            return self.token(p, p + 1, &[]);
        }
        let leaf = if want == Kind::FlowSeq {
            S::PunctSeqEnd
        } else {
            S::PunctMapEnd
        };
        self.tok(p + 1, &[leaf]);
        self.pop();
        p + 1
    }
}
