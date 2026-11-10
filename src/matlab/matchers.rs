//! The matchers, one for each regular expression of the grammar.
//! Each is the code that decides whether its expression matches at a
//! position, where it ends, and the capture groups the grammar names.
//!
//! Each follows its expression as Oniguruma's backtracking would.
//! `\w`, `\s` and `\d` take non-ASCII characters, as in Oniguruma, through
//! [`Chars`].
//! A class such as `[^%]` takes the line feed after the text, as Oniguruma's
//! does on the line Shiki matches.

use super::engine::Lexer;
use super::grammar::Rx;
use crate::rules::Chars;

/// The bytes the lookbehind before a string's quote accepts, besides `\s`:
/// `[\&(*-/:->\[\\^{|~\s]`.
const BEFORE_QUOTE: [bool; 256] = table(b"&(*+,-./:;<=>[\\^{|~");
/// The bytes the lookahead after a string's closing quote accepts, besides
/// `\s`: `[\&(-/:->\[-^{-~\s]`.
const AFTER_QUOTE: [bool; 256] = table(b"&()*+,-./:;<=>[\\]^{|}~");
/// The bytes the lookbehind before a number accepts, besides `\s`:
/// `[(*-\-/:=\[\\{\s]`.
const BEFORE_NUMBER: [bool; 256] = table(b"(*+,-/:=[\\{");
/// The bytes, besides `\s`, that are no one-character first token of a
/// command's arguments: the quotes and the operators.
const NOT_ARGUMENT: [bool; 256] = table(b"\"%&'()*+,-./:;<=>@\\^{|~");
/// The bytes of `[]\&)*,/:->@\\^|}]`, which may follow an operator's blanks
/// in the arguments' first shape.
const CLOSERS: [bool; 256] = table(b"]&)*,/:;<=>@\\^|}");

/// The table of `bytes`: whether each byte is one of them.
///
/// A set of bytes is looked up in a table rather than searched with
/// `<[u8]>::contains`.
///
/// On 16 bytes or more, `contains` calls `memchr`.
/// `memchr` reads bytes one at a time up to the first aligned word.
/// So its cost follows where the set sits in the binary, and another
/// language's tables can move that place.
/// A table lookup costs one load wherever the table sits.
#[expect(
    clippy::as_conversions,
    reason = "`usize::from` is not `const`, and a byte always fits"
)]
const fn table(bytes: &[u8]) -> [bool; 256] {
    let mut table = [false; 256];
    let mut i = 0;
    while i < bytes.len() {
        table[bytes[i] as usize] = true;
        i += 1;
    }
    table
}

/// The operators of `Operators`, in the order its alternation tries them.
/// Each must have `\s` after it.
const SPACED_OPERATORS: &[&[u8]] = &[
    b"==", b"~=", b">", b">=", b"<", b"<=", b"&", b"&&", b":", b"|", b"||", b"-", b"*", b"+",
    b".*", b"/", b"./", b"\\", b".\\", b"^", b".^",
];
/// The words of `Constants`.
const CONSTANTS: &[&[u8]] = &[
    b"eps",
    b"false",
    b"Inf",
    b"inf",
    b"intmax",
    b"intmin",
    b"namelengthmax",
    b"NaN",
    b"nan",
    b"on",
    b"off",
    b"realmax",
    b"realmin",
    b"true",
    b"pi",
];
/// The words of `Variables`.
const VARIABLES: &[&[u8]] = &[b"nargin", b"nargout", b"varargin", b"varargout"];
/// The access words of a `properties` block's attribute list.
const ACCESS: &[&[u8]] = &[b"public", b"protected", b"private"];

/// A class of name characters that [`Lexer::name_run`] measures.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NameClass {
    /// `[A-Za-z][.0-9A-Z_a-z]*`.
    Dotted,
    /// `[A-Za-z][.0-9?A-Z_a-z]*`.
    Validator,
}

/// How a command's arguments end.
#[derive(Clone, Copy, Debug)]
enum Tail {
    /// At a comment that starts at this byte.
    Comment(usize),
    /// At the end of the line.
    Line,
}

/// What an operator of the command syntax's arguments needs after it.
#[derive(Clone, Copy, Debug)]
enum Then {
    /// Nothing more.
    Any,
    /// `(?=[^x])`: a character that is not `x`.
    Other(u8),
    /// `(?!x)`: no `x`, or nothing at all.
    Not(u8),
}

/// The operators that can start a command's arguments, in the order the
/// grammar's alternation tries them, before its single-character ones.
const COMMAND_OPERATORS: &[(&[u8], Then)] = &[
    (b".^", Then::Any),
    (b".*", Then::Any),
    (b"./", Then::Any),
    (b".\\", Then::Any),
    (b".'", Then::Any),
    (b".(", Then::Any),
    (b"&&", Then::Any),
    (b"==", Then::Any),
    (b"||", Then::Any),
    (b"&", Then::Other(b'&')),
    (b"|", Then::Other(b'|')),
    (b"~=", Then::Any),
    (b"<=", Then::Any),
    (b">=", Then::Any),
    (b"~", Then::Not(b'=')),
    (b"<", Then::Not(b'=')),
    (b">", Then::Not(b'=')),
];

impl Lexer<'_> {
    /// Whether `rx` matches at `p`, and where its match ends.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm per expression: the table of what each matches"
    )]
    pub(super) fn matcher(&mut self, rx: Rx, p: usize) -> Option<usize> {
        match rx {
            Rx::ClassdefAt0 => {
                let (start, end) = self.line_word(p, b"classdef")?;
                let rest = self.space_end(end);
                self.capture(2, start, end);
                if rest > self.lim() {
                    self.capture(3, rest, rest);
                    return Some(rest);
                }
                self.capture(3, rest, self.lim());
                Some(self.lim())
            }
            Rx::ClassdefAt0End => self.block_word(p, &[b"end"]),
            Rx::ClassdefAt0Capture3At0 => self.class_head(p),
            Rx::ClassdefAt0Capture3At0Capture1At0 => self.ascii_ident(p),
            Rx::ClassdefAt0Capture3At0Capture1At1 => {
                let end = self.lit(p, b"=")?;
                Some(self.space_end(end))
            }
            Rx::ClassdefAt0Capture3At0Capture1At1End => match self.at(p) {
                b',' if p < self.lim() => Some(p + 1),
                b')' if p < self.lim() => Some(p),
                _ => None,
            },
            Rx::ClassdefAt0Capture3At0Capture1At1At0 => {
                self.lit(p, b"true").or_else(|| self.lit(p, b"false"))
            }
            Rx::StringAt0 => {
                if p != 0 {
                    return None;
                }
                let bang = self.blanks(0);
                if self.at(bang) != b'!' || bang >= self.lim() {
                    return None;
                }
                let end = self.line_end_with_feed();
                self.capture(1, bang, end);
                self.capture(2, bang, bang + 1);
                Some(end)
            }
            Rx::StringAt1 => self.open_quote(p, b'\''),
            Rx::StringAt2 => self.open_quote(p, b'"'),
            Rx::StringAt1End => self.close_quote(p, b'\''),
            Rx::StringAt2End => self.close_quote(p, b'"'),
            Rx::StringAt1At0 => self.lit(p, b"''"),
            Rx::StringAt2At0 => self.lit(p, b"\"\""),
            Rx::StringAt1At1 => self.lone_quote(p, b'\''),
            Rx::StringAt2At1 => self.lone_quote(p, b'"'),
            Rx::StringAt1At2 => self.format_escape(p),
            Rx::ClassdefAt0Capture3At0Capture5At0 => self.dotted_name(p),
            Rx::ClassdefAt0Capture3At0Capture5At1 => self.lit(p, b"&"),
            Rx::CommandDual => self.command_dual(p),
            Rx::LineContinuation => {
                let dots = self.lit(p, b"...")?;
                self.capture(1, p, dots);
                self.capture(2, dots, self.lim());
                Some(self.lim())
            }
            Rx::CommentsAt0 => self.comment_ahead(p, b"%%"),
            Rx::CommentsAt2 => self.comment_ahead(p, b"%"),
            Rx::CommentsAt0End => (!self.at_g(p)).then_some(p),
            Rx::CommentsAt0At0 => self.lit(p, b"%%"),
            Rx::CommentsAt2At0 => self.lit(p, b"%"),
            Rx::CommentsAt0At0End => self.line_feed_at(p).then_some(p + 1),
            Rx::CommentsAt0At0At0 => {
                if !self.at_g(p) {
                    return None;
                }
                if p > self.lim() {
                    return Some(p);
                }
                let end = self.blanks(p);
                (end < self.lim() || !self.nl).then_some(end)
            }
            Rx::CommentsAt0At0At0End => self.line_feed_at(p).then_some(p),
            Rx::CommentBlock => {
                if p != 0 {
                    return None;
                }
                let open = self.blanks(0);
                let brace = self.lit(open, b"%{")?;
                let feed = self.blanks(brace);
                self.line_feed_at(feed).then(|| {
                    self.capture(1, 0, open);
                    feed + 1
                })
            }
            Rx::CommentBlockEnd => {
                if p != 0 {
                    return None;
                }
                let close = self.lit(self.blanks(0), b"%}")?;
                let end = self.blanks(close);
                if self.line_feed_at(end) {
                    return Some(end + 1);
                }
                self.at_eol(end).then_some(end)
            }
            Rx::CommentBlockAt1 => {
                (p == 0 && self.line_feed_at(self.lim())).then(|| self.lim() + 1)
            }
            Rx::ConjugateTranspose => {
                let after_text = p > 0 && self.space_at(self.char_start_before(p)) == 0;
                (after_text && self.at(p) == b'\'' && p < self.lim()).then_some(p + 1)
            }
            Rx::Transpose => self.lit(p, b".'"),
            Rx::Constants => self.unqualified_word(p, CONSTANTS),
            Rx::Variables => self.unqualified_word(p, VARIABLES),
            Rx::Numbers => self.number(p),
            Rx::Operators => self.spaced_operator(p),
            Rx::ClassdefAt0At0 => self.section(p, b"properties"),
            Rx::ClassdefAt0At1 => self.section(p, b"methods"),
            Rx::ClassdefAt0At2 => self.section(p, b"events"),
            Rx::ClassdefAt0At3 => self.section(p, b"enumeration"),
            Rx::FunctionAt0At1 => self.section(p, b"arguments"),
            Rx::ClassdefAt0At0Capture3At1At1 => ACCESS.iter().find_map(|word| self.lit(p, word)),
            Rx::Validators => {
                let start = self.space_end(p);
                if start > self.lim() {
                    return None;
                }
                let name = if self.at(start) == b';' && start < self.lim() {
                    self.space_end(start + 1)
                } else {
                    start
                };
                self.name_run(name, NameClass::Validator)
            }
            Rx::ValidatorsEnd => {
                let end = if self.line_feed_at(p) {
                    p + 1
                } else if matches!(self.at(p), b'%' | b';' | b'=') && p < self.lim() {
                    self.lim()
                } else {
                    return None;
                };
                self.capture(1, p, end);
                Some(end)
            }
            Rx::ValidatorsEndCapture1At0 => {
                self.lit(p, b"%")?;
                self.capture(1, p, self.lim());
                Some(self.lim())
            }
            Rx::ValidatorsEndCapture1At1 => {
                let equals = self.lit(p, b"=")?;
                let end = self.not_run(equals, b";");
                self.capture(1, p, end);
                Some(end)
            }
            Rx::ValidatorsEndCapture1At2 => {
                if !self.line_feed_at(p) && (self.at(p) != b';' || p >= self.lim()) {
                    return None;
                }
                let letter = self.space_end(p + 1);
                (letter < self.lim() && self.at(letter).is_ascii_alphabetic()).then(|| {
                    self.capture(1, p, self.lim());
                    self.lim()
                })
            }
            Rx::ValidatorsAt1 => {
                let open = self.space_end(p);
                if self.at(open) != b'(' || open >= self.lim() {
                    return None;
                }
                let close = self.not_run(open + 1, b")");
                (close < self.lim() && self.at(close) == b')').then_some(close + 1)
            }
            Rx::ValidatorsAt2 => self.name_run(p, NameClass::Dotted),
            Rx::BracedValidatorList => {
                let open = self.space_end(p);
                (self.at(open) == b'{' && open < self.lim()).then(|| {
                    self.capture(1, open, open + 1);
                    self.space_end(open + 1)
                })
            }
            Rx::BracedValidatorListEnd => {
                let end = self.lit(p, b"}")?;
                self.capture(1, p, end);
                Some(end)
            }
            Rx::ValidatorStringsAt0At0At2 => {
                let end = self.not_run(p, b"'");
                (end > p).then_some(end)
            }
            Rx::ValidatorStringsAt0At1At2 => {
                let end = self.not_run(p, b"\"");
                (end > p).then_some(end)
            }
            Rx::BracedValidatorListAt3 => {
                let end = self.not_run(p, b"\"'.{}");
                (end > p).then(|| {
                    self.capture(1, p, end);
                    end
                })
            }
            Rx::BracedValidatorListAt4 => self.lit(p, b"."),
            Rx::FunctionAt0 => self.function_head(p),
            Rx::FunctionAt0End => {
                let end = self.block_word(p, &[b"end"])?;
                let feed = self.space_end(end);
                Some(if feed > self.lim() { feed } else { end })
            }
            Rx::FunctionAt0Capture4At0 => {
                let end = self.word_end(p);
                (end > p).then_some(end)
            }
            Rx::FunctionAt0At0 => {
                (self.at_g(p) && self.at(p) == b'(' && p < self.lim()).then_some(p + 1)
            }
            Rx::FunctionAt0At0End => self.lit(p, b")"),
            Rx::BlocksAt0 => self.block_word(p, &[b"for"]),
            Rx::BlocksAt1 => self.block_word(p, &[b"if"]),
            Rx::BlocksAt2 => self.block_word(p, &[b"parfor"]),
            Rx::BlocksAt3 => self.block_word(p, &[b"spmd"]),
            Rx::BlocksAt4 => self.block_word(p, &[b"switch"]),
            Rx::BlocksAt5 => self.block_word(p, &[b"try"]),
            Rx::BlocksAt6 => self.block_word(p, &[b"while"]),
            Rx::ControlStatements => self.block_word(p, &[b"break", b"continue", b"return"]),
            Rx::BlocksAt1At0 => self.clause(p, b"elseif"),
            Rx::BlocksAt1At1 => self.clause(p, b"else"),
            Rx::BlocksAt4At0 => self.clause(p, b"case"),
            Rx::BlocksAt4At1 => self.clause(p, b"otherwise"),
            Rx::BlocksAt5At0 => self.clause(p, b"catch"),
            Rx::BlocksAt2At0 => (self.at_g(p) && !self.at_eol(p)).then_some(p),
            Rx::BlocksAt2At0End => self
                .at_eol(p)
                .then(|| if self.line_feed_at(p) { p + 1 } else { p }),
            Rx::GlobalPersistent => {
                if p != 0 {
                    return None;
                }
                let start = self.blanks(0);
                let end = self
                    .lit(start, b"global")
                    .or_else(|| self.lit(start, b"persistent"))
                    .filter(|&end| self.boundary(end))?;
                self.capture(1, start, end);
                Some(end)
            }
            Rx::Parens => self.lit(p, b"("),
            Rx::ParensEnd => self.close_or_feed(p, b')'),
            Rx::EndInParens => self.keyword(p, b"end"),
            Rx::SquareBrackets => self.lit(p, b"["),
            Rx::SquareBracketsEnd => self.lit(p, b"]"),
            Rx::IndexingCurlyBrackets => {
                let name = self.name_run(p, NameClass::Dotted)?;
                let open = self.space_end(name);
                (self.at(open) == b'{' && open < self.lim()).then(|| {
                    self.capture(1, p, open);
                    open + 1
                })
            }
            Rx::IndexingCurlyBracketsEnd => self.close_or_feed(p, b'}'),
            Rx::CurlyBrackets => self.lit(p, b"{"),
            Rx::CurlyBracketsEnd => self.lit(p, b"}"),
        }
    }

    // Line structure.

    /// The end of `\s*` from `i`, short of the line feed.
    fn blanks(&self, i: usize) -> usize {
        self.space_end(i).min(self.lim())
    }

    /// `\n` at `i`: the line feed after the text.
    fn line_feed_at(&self, i: usize) -> bool {
        i == self.lim() && self.nl
    }

    /// `.*$\n?` from the text: the end of the line, past its line feed when
    /// there is one.
    fn line_end_with_feed(&self) -> usize {
        if self.nl { self.lim() + 1 } else { self.lim() }
    }

    /// The end of the run from `i` of characters that are not one of `stop`,
    /// as `[^…]*` takes it.
    /// The run takes the line feed when it reaches it and `stop` holds no line
    /// feed.
    fn not_run(&self, i: usize, stop: &[u8]) -> usize {
        let end = self.skip(i, |b| !stop.contains(&b));
        if self.line_feed_at(end) && !stop.contains(&b'\n') {
            end + 1
        } else {
            end
        }
    }

    /// `^(\s*)(word)\b` at `p`: where the word starts and ends.
    fn line_word(&self, p: usize, word: &[u8]) -> Option<(usize, usize)> {
        if p != 0 {
            return None;
        }
        let start = self.blanks(0);
        let end = self.lit(start, word)?;
        self.boundary(end).then_some((start, end))
    }

    /// `\s*(?:^|[,;\s])(word)\b` at `p`, for one of `words`, recording the word
    /// as group 1: where it ends.
    /// `\s*` gives back its last blank for `[,;\s]`, so the word stands right
    /// after the blanks, or after a `,` or `;` that follows them.
    fn block_word(&mut self, p: usize, words: &[&[u8]]) -> Option<usize> {
        let (start, end) = self.word_after_separator(p, words)?;
        self.capture(1, start, end);
        Some(end)
    }

    /// The word of [`Lexer::block_word`]: where it starts and ends.
    fn word_after_separator(&self, p: usize, words: &[&[u8]]) -> Option<(usize, usize)> {
        let blanks = self.space_end(p);
        let word_at = |at: usize| {
            words
                .iter()
                .find_map(|word| self.lit(at, word))
                .filter(|&end| self.boundary(end))
                .map(|end| (at, end))
        };
        if blanks == 0
            && let Some(found) = word_at(0)
        {
            return Some(found);
        }
        if matches!(self.at(blanks), b',' | b';')
            && blanks < self.lim()
            && let Some(found) = word_at(blanks + 1)
        {
            return Some(found);
        }
        if blanks > p {
            return word_at(blanks);
        }
        None
    }

    /// `(\s*)(?:^|[,;\s])(word)\b(.*)$\n?` at `p`, recording the word as group
    /// 2 and the rest of the line as group 3.
    fn clause(&mut self, p: usize, word: &[u8]) -> Option<usize> {
        let (start, end) = self.word_after_separator(p, &[word])?;
        self.capture(2, start, end);
        self.capture(3, end, self.lim());
        Some(self.line_end_with_feed())
    }

    /// `^(\s*)(word)\b([^%]*)\s*(\([^)]*\))?\s*($|(?=%))` at `p`, recording
    /// the word as group 2 and the rest up to a `%` as group 3.
    /// `[^%]*` takes the attributes in parentheses and the line feed, so the
    /// tail always matches where it stops.
    fn section(&mut self, p: usize, word: &[u8]) -> Option<usize> {
        let (start, end) = self.line_word(p, word)?;
        let rest = self.not_run(end, b"%");
        self.capture(2, start, end);
        self.capture(3, end, rest);
        Some(rest)
    }

    /// `(^[\t ]+)?(?=mark\s)` at `p`, recording the indentation as group 1.
    fn comment_ahead(&mut self, p: usize, mark: &[u8]) -> Option<usize> {
        let followed = |at: usize| {
            self.lit(at, mark)
                .is_some_and(|end| mark.len() == 1 || self.space_at(end) > 0)
        };
        if p == 0 {
            let indent = self.skip(0, |b| b == b'\t' || b == b' ');
            if indent > 0 && followed(indent) {
                self.capture(1, 0, indent);
                return Some(indent);
            }
        }
        followed(p).then_some(p)
    }

    /// `(\)|(?<!\.\.\.).\n)` with `close` for `)`: the closing byte, or the
    /// last character of a line that does not end in `...`, with its line
    /// feed.
    fn close_or_feed(&self, p: usize, close: u8) -> Option<usize> {
        if self.at(p) == close && p < self.lim() {
            return Some(p + 1);
        }
        let continued = p >= 3 && self.text(p - 3, p) == b"...";
        (p < self.lim() && self.char_end(p) == self.lim() && self.nl && !continued)
            .then(|| self.lim() + 1)
    }

    // Names.

    /// `[A-Za-z][0-9A-Z_a-z]*` at `i`: where it ends.
    fn ascii_ident(&self, i: usize) -> Option<usize> {
        (self.at(i).is_ascii_alphabetic() && i < self.lim())
            .then(|| self.skip(i + 1, |b| b == b'_' || b.is_ascii_alphanumeric()))
    }

    /// `[A-Za-z][0-9A-Z_a-z]*(\.[A-Za-z][0-9A-Z_a-z]*)*` at `i`: where it ends.
    fn dotted_name(&self, i: usize) -> Option<usize> {
        let mut end = self.ascii_ident(i)?;
        while self.at(end) == b'.'
            && let Some(next) = self.ascii_ident(end + 1)
        {
            end = next;
        }
        Some(end)
    }

    /// A name of `class` at `i`: where it ends.
    /// The last run measured is kept, so a name tried at each of its bytes is
    /// walked once.
    fn name_run(&self, i: usize, class: NameClass) -> Option<usize> {
        if !self.at(i).is_ascii_alphabetic() || i >= self.lim() {
            return None;
        }
        let tag = match class {
            NameClass::Dotted => 1,
            NameClass::Validator => 2,
        };
        let (start, end, len, kept) = self.x.run.get();
        if kept == tag && len == self.s.len() && start <= i && i < end {
            return Some(end);
        }
        let end = self.skip(i + 1, |b| {
            b == b'_'
                || b == b'.'
                || b.is_ascii_alphanumeric()
                || (b == b'?' && class == NameClass::Validator)
        });
        self.x.run.set((i, end, self.s.len(), tag));
        Some(end)
    }

    /// `(?<!\.)\b(words)\b` at `p`.
    fn unqualified_word(&self, p: usize, words: &[&[u8]]) -> Option<usize> {
        (!self.prev_is(p, b'.')).then(|| self.word_in(p, words))?
    }

    // Strings.

    /// Whether the character before `p` is `\s` or one of the bytes of
    /// `set`.
    fn prev_in(&self, p: usize, set: &[bool; 256]) -> bool {
        if p == 0 {
            return false;
        }
        let start = self.char_start_before(p);
        set[usize::from(self.at(start))] || self.space_at(start) > 0
    }

    /// Whether the character at `i` is `\s` or one of the bytes of `set`.
    fn next_in(&self, i: usize, set: &[bool; 256]) -> bool {
        (i < self.lim() && set[usize::from(self.at(i))]) || self.space_at(i) > 0
    }

    /// `((?<=[…])|^)quote`, a string's opening quote.
    fn open_quote(&self, p: usize, quote: u8) -> Option<usize> {
        ((p == 0 || self.prev_in(p, &BEFORE_QUOTE)) && self.at(p) == quote && p < self.lim())
            .then_some(p + 1)
    }

    /// `quote(?=[…])`, a string's closing quote.
    fn close_quote(&self, p: usize, quote: u8) -> Option<usize> {
        (self.at(p) == quote && p < self.lim() && self.next_in(p + 1, &AFTER_QUOTE))
            .then_some(p + 1)
    }

    /// `quote(?=.)`, a quote with a character after it on the line.
    fn lone_quote(&self, p: usize, quote: u8) -> Option<usize> {
        (self.at(p) == quote && p + 1 < self.lim()).then_some(p + 1)
    }

    /// `(%([-+0]?\d{0,3}(\.\d{1,3})?)([EGc-gs]|(([bt])?([Xoux]))))|%%|\\([\\
    /// bfnrt])`.
    fn format_escape(&self, p: usize) -> Option<usize> {
        if let Some(after) = self.lit(p, b"\\") {
            return matches!(self.at(after), b'\\' | b'b' | b'f' | b'n' | b'r' | b't')
                .then(|| after + 1)
                .filter(|&end| end <= self.lim());
        }
        let mut i = self.lit(p, b"%")?;
        if matches!(self.at(i), b'-' | b'+' | b'0') && i < self.lim() {
            i += 1;
        }
        i = self.digits_up_to(i, 3);
        if self.at(i) == b'.' {
            let fraction = self.digits_up_to(i + 1, 3);
            if fraction > i + 1 {
                i = fraction;
            }
        }
        let conversion = match self.at(i) {
            b'E' | b'G' | b'c'..=b'g' | b's' | b'X' | b'o' | b'u' | b'x' => Some(i + 1),
            b'b' | b't' if matches!(self.at(i + 1), b'X' | b'o' | b'u' | b'x') => Some(i + 2),
            _ => None,
        }
        .filter(|&end| end <= self.lim());
        conversion.or_else(|| self.lit(p, b"%%"))
    }

    /// The end of up to `max` `\d` characters from `i`.
    fn digits_up_to(&self, mut i: usize, max: usize) -> usize {
        for _ in 0..max {
            if self.digit_end(i) == i {
                break;
            }
            i = self.char_end(i);
        }
        i
    }

    // Numbers and operators.

    /// `Numbers`:
    /// `(?<=[(*-\-/:=\[\\{\s]|^)\d*\.?\d+([Ee][-+]?\d)?([0-9&&[^.]])*([ij])?\
    /// b`.
    ///
    /// Every character the match can end on is a word character, so `\b`
    /// needs a non-word character after it.
    /// The mantissa ends after the digits that follow a `.`, or else after
    /// the first run of digits, which `\d*` gives back for `\d+`.
    fn number(&self, p: usize) -> Option<usize> {
        if p != 0 && !self.prev_in(p, &BEFORE_NUMBER) {
            return None;
        }
        let whole = self.digit_end(p);
        if self.at(whole) == b'.' {
            let fraction = self.digit_end(whole + 1);
            if fraction > whole + 1
                && let Some(end) = self.number_tail(fraction)
            {
                return Some(end);
            }
        }
        if whole > p {
            return self.number_tail(whole);
        }
        None
    }

    /// `([Ee][-+]?\d)?([0-9])*([ij])?\b` after a mantissa that ends at `m`.
    fn number_tail(&self, m: usize) -> Option<usize> {
        let finish = |from: usize| {
            let digits = self.skip(from, |b| b.is_ascii_digit());
            if matches!(self.at(digits), b'i' | b'j')
                && digits < self.lim()
                && self.word_at(digits + 1).is_none()
            {
                return Some(digits + 1);
            }
            self.word_at(digits).is_none().then_some(digits)
        };
        if matches!(self.at(m), b'E' | b'e') && m < self.lim() {
            let sign = m + 1 + usize::from(matches!(self.at(m + 1), b'-' | b'+'));
            if self.digit_end(sign) > sign
                && let Some(end) = finish(self.char_end(sign))
            {
                return Some(end);
            }
        }
        finish(m)
    }

    /// `(?<=\s)(operator)(?=\s)`, with the operators of [`SPACED_OPERATORS`].
    fn spaced_operator(&self, p: usize) -> Option<usize> {
        if p == 0 || self.space_at(self.char_start_before(p)) == 0 {
            return None;
        }
        SPACED_OPERATORS
            .iter()
            .filter_map(|op| self.lit(p, op))
            .find(|&end| self.space_at(end) > 0)
    }

    // Classes and functions.

    /// `ClassdefAt0Capture3At0`, the text after `classdef`:
    /// `(\([^)]*\))?\s*(([A-Za-z][0-9A-Z_a-z]*)(?:\s*(<)\s*([^%]*))?)\s*
    /// ($|(?=(%|...)).*)`.
    /// The lookahead's `...` takes any three characters.
    fn class_head(&mut self, p: usize) -> Option<usize> {
        let mut name = p;
        let mut attributes = None;
        if self.at(p) == b'(' && p < self.lim() {
            let close = self.not_run(p + 1, b")");
            if close < self.lim() && self.at(close) == b')' {
                attributes = Some(close + 1);
                name = close + 1;
            }
        }
        let start = self.space_end(name);
        let ident = self.ascii_ident(start)?;
        let less = self.space_end(ident);
        if self.at(less) == b'<' && less < self.lim() {
            let supers = self.space_end(less + 1);
            let supers_end = self.not_run(supers, b"%");
            let tail = self.space_end(supers_end);
            let end = self.class_tail(tail)?;
            self.class_groups(p, attributes, (start, supers_end, ident));
            self.capture(4, less, less + 1);
            self.capture(5, supers, supers_end);
            return Some(end);
        }
        let mut at = less;
        loop {
            if let Some(end) = self.class_tail(at) {
                let ident_end = ident.min(at);
                self.class_groups(p, attributes, (start, ident_end, ident_end));
                return Some(end);
            }
            if at <= start + 1 {
                return None;
            }
            at = self.char_start_before(at);
        }
    }

    /// The groups of a class head.
    /// Group 1 is the attributes.
    /// Groups 2 and 3 are the name with its superclasses and the name alone.
    fn class_groups(
        &mut self,
        p: usize,
        attributes: Option<usize>,
        (start, end, ident): (usize, usize, usize),
    ) {
        if let Some(close) = attributes {
            self.capture(1, p, close);
        }
        self.capture(2, start, end);
        self.capture(3, start, ident);
    }

    /// `($|(?=(%|...)).*)` at `at`, recorded as group 6: where the match ends.
    fn class_tail(&mut self, at: usize) -> Option<usize> {
        if self.at_eol(at) {
            self.capture(6, at, at);
            return Some(at);
        }
        let mut chars = 0;
        let mut i = at;
        while chars < 3 && i < self.lim() {
            i = self.char_end(i);
            chars += 1;
        }
        let ahead = (at < self.lim() && self.at(at) == b'%') || chars == 3;
        ahead.then(|| {
            self.capture(6, at, self.lim());
            self.lim()
        })
    }

    /// `FunctionAt0`: `^(\s*)(function)\s+(?:(?:(\[)([^]]*)(])|(name))\s*=\s*)?
    /// (dotted name)\s*`.
    fn function_head(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let start = self.blanks(0);
        let word = self.lit(start, b"function")?;
        let outputs = self.space_end(word);
        if outputs == word || outputs > self.lim() {
            return None;
        }
        let assigned = |lexer: &Self, from: usize| {
            let equals = lexer.space_end(from);
            if lexer.at(equals) != b'=' || equals >= lexer.lim() {
                return None;
            }
            let name = lexer.space_end(equals + 1);
            lexer.dotted_name(name).map(|end| (name, end))
        };
        if self.at(outputs) == b'[' {
            let close = self.not_run(outputs + 1, b"]");
            if close < self.lim()
                && self.at(close) == b']'
                && let Some((name, end)) = assigned(self, close + 1)
            {
                self.capture(2, start, word);
                self.capture(3, outputs, outputs + 1);
                self.capture(4, outputs + 1, close);
                self.capture(5, close, close + 1);
                self.capture(7, name, end);
                return Some(self.space_end(end));
            }
        } else if let Some(output) = self.ascii_ident(outputs)
            && let Some((name, end)) = assigned(self, output)
        {
            self.capture(2, start, word);
            self.capture(6, outputs, output);
            self.capture(7, name, end);
            return Some(self.space_end(end));
        }
        let end = self.dotted_name(outputs)?;
        self.capture(2, start, word);
        self.capture(7, outputs, end);
        Some(self.space_end(end))
    }

    // Command syntax.

    /// `CommandDual`: `^\s*((name)\s+(arguments))(%.*)?$`, a call in command
    /// syntax such as `hold on`, recording the call as group 1, its name as
    /// group 2 and a trailing comment as group 28.
    ///
    /// The name is a whole run of word characters that [`command_name`]
    /// accepts.
    /// The arguments start right after the blanks: an earlier start would be
    /// a blank, which no argument shape takes.
    fn command_dual(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let start = self.blanks(0);
        let name = self.word_end(start);
        if name == start || !command_name(self.text(start, name)) {
            return None;
        }
        let args = self.space_end(name);
        if args == name || args >= self.lim() {
            return None;
        }
        let tail = match self.first_argument(args) {
            Some(first) => self.next_percent(first).map_or(Tail::Line, Tail::Comment),
            None => self.dotted_arguments(args)?,
        };
        self.capture(2, start, name);
        if let Tail::Comment(percent) = tail {
            self.capture(1, start, percent);
            self.capture(28, percent, self.lim());
            return Some(self.lim());
        }
        let end = self.line_end_with_feed();
        self.capture(1, start, end);
        Some(end)
    }

    /// The first `%` from `i`, where the comment of a command starts.
    fn next_percent(&self, i: usize) -> Option<usize> {
        let at = self.skip(i, |b| b != b'%');
        (at < self.lim()).then_some(at)
    }

    /// The first token of the arguments' first shape, at `a`.
    /// The token is a character that is no operator, a quote, or an operator
    /// with what may follow it.
    /// It can also be a `.` with a character that keeps it from a field or an
    /// operator.
    /// Where the token ends; the rest of the line up to a `%` follows it.
    fn first_argument(&self, a: usize) -> Option<usize> {
        let b = self.at(a);
        if b == b'\'' || b == b'"' {
            return Some(a);
        }
        if self.space_at(a) == 0 && !NOT_ARGUMENT[usize::from(b)] {
            return Some(self.char_end(a));
        }
        if let Some(end) = self.operator_argument(a) {
            return Some(end);
        }
        if b == b'.' && a + 1 < self.lim() {
            let next = self.at(a + 1);
            let excluded = b"'(*/\\^";
            if !excluded.contains(&next) && !next.is_ascii_alphabetic() && self.space_at(a + 1) == 0
            {
                return Some(self.char_end(a + 1));
            }
        }
        None
    }

    /// An operator at `a` and what must follow it in the arguments' first
    /// shape: where they end.
    fn operator_argument(&self, a: usize) -> Option<usize> {
        for &(op, then) in COMMAND_OPERATORS {
            let end = self.lit(a, op).filter(|&end| match then {
                Then::Any => true,
                Then::Other(byte) => self.avail(end) && self.at(end) != byte,
                Then::Not(byte) => self.at(end) != byte || end >= self.lim(),
            });
            if let Some(end) = end
                && let Some(follow) = self.after_operator(end)
            {
                return Some(follow);
            }
        }
        if matches!(
            self.at(a),
            b'-' | b'*' | b'+' | b'/' | b':' | b'@' | b'\\' | b'^'
        ) && a < self.lim()
        {
            return self.after_operator(a + 1);
        }
        None
    }

    /// What follows an operator in the arguments' first shape:
    /// `\S|\s*(?=%)|\s+$|\s+([]\&)*,/:->@\\^|}]|(\.(?:[^.\d]|\.[^.])))`.
    /// Where it ends.
    fn after_operator(&self, i: usize) -> Option<usize> {
        if i < self.lim() && self.space_at(i) == 0 {
            return Some(self.char_end(i));
        }
        let blanks = self.space_end(i);
        if blanks < self.lim() && self.at(blanks) == b'%' {
            return Some(blanks);
        }
        if blanks == i {
            return None;
        }
        if self.at_eol(blanks) {
            return Some(blanks);
        }
        if blanks < self.lim() && CLOSERS[usize::from(self.at(blanks))] {
            return Some(blanks + 1);
        }
        if self.at(blanks) == b'.' && blanks < self.lim() {
            let next = blanks + 1;
            if next <= self.lim()
                && self.at(next) != b'.'
                && self.digit_end(next) == next
                && self.avail(next)
            {
                return Some(self.char_end(next).max(next + 1));
            }
            if self.at(next) == b'.'
                && next < self.lim()
                && self.avail(next + 1)
                && self.at(next + 1) != b'.'
            {
                return Some(self.char_end(next + 1).max(next + 2));
            }
        }
        None
    }

    /// The arguments' second shape at `a`: `(\.(?=\s)|\.[A-Za-z]|(?=\{))`
    /// then `([^"%'(=]|==|'[^']*'|"[^"]*"|\(|…)*(\.\.\.[^%]*)?((?=%)|$)`.
    /// The repetition takes every character but a lone quote, `%` and a lone
    /// `=`.
    /// Where it stops short of the end, it gives back characters until a
    /// `...` can start, whose `[^%]*` runs on.
    /// Returns how the line ends after the arguments.
    fn dotted_arguments(&self, a: usize) -> Option<Tail> {
        let mut i = match self.at(a) {
            b'.' if self.space_at(a + 1) > 0 => a + 1,
            b'.' if a + 1 < self.lim() && self.at(a + 1).is_ascii_alphabetic() => a + 2,
            b'{' if a < self.lim() => a,
            _ => return None,
        };
        let mut dots = None;
        let stop = loop {
            if self.starts(i, b"...") {
                dots = Some(i);
            }
            if i >= self.lim() {
                break if self.line_feed_at(i) { i + 1 } else { i };
            }
            match self.at(i) {
                b'=' if self.at(i + 1) == b'=' && i + 1 < self.lim() => i += 2,
                b'%' | b'=' => break i,
                quote @ (b'\'' | b'"') => {
                    let close = self.skip(i + 1, |b| b != quote);
                    if close >= self.lim() {
                        break i;
                    }
                    i = close + 1;
                }
                _ => i = self.char_end(i),
            }
        };
        self.spend(stop - a);
        if self.at_eol(stop) {
            return Some(Tail::Line);
        }
        if self.at(stop) == b'%' && stop < self.lim() {
            return Some(Tail::Comment(stop));
        }
        let from = dots?;
        Some(self.next_percent(from).map_or(Tail::Line, Tail::Comment))
    }
}

/// What a spelled letter of an excluded name lets the next character be,
/// besides the letter that continues the spelling.
#[derive(Clone, Copy, Debug)]
enum Next {
    /// An ASCII word character, but none of these.
    Word(&'static [u8]),
    /// An ASCII letter, but none of these.
    Letter(&'static [u8]),
    /// Any word character.
    Any,
}

/// The names the command syntax never starts with.
const NOT_COMMANDS: &[&[u8]] = &[
    b"ans", b"eps", b"i", b"inf", b"Inf", b"j", b"NaN", b"nan", b"nargin", b"nargout", b"pi",
];

/// The grammar spells the names of [`NOT_COMMANDS`] out letter by letter.
/// For each prefix it spells: the letters that continue a spelling, and what
/// else the next character may be.
/// `narg` rules out `p` too, and `nargou` takes letters only, as the
/// grammar's classes do.
const SPELLED: &[(&[u8], &[u8], Next)] = &[
    (b"a", b"n", Next::Word(b"n")),
    (b"an", b"s", Next::Word(b"s")),
    (b"e", b"p", Next::Word(b"p")),
    (b"ep", b"s", Next::Word(b"s")),
    (b"i", b"n", Next::Word(b"n")),
    (b"in", b"f", Next::Word(b"f")),
    (b"I", b"n", Next::Word(b"n")),
    (b"In", b"f", Next::Word(b"f")),
    (b"j", b"", Next::Any),
    (b"N", b"a", Next::Word(b"a")),
    (b"Na", b"N", Next::Word(b"N")),
    (b"n", b"a", Next::Word(b"a")),
    (b"na", b"nr", Next::Word(b"nr")),
    (b"nar", b"g", Next::Word(b"g")),
    (b"narg", b"io", Next::Word(b"iop")),
    (b"nargi", b"n", Next::Word(b"n")),
    (b"nargo", b"u", Next::Word(b"u")),
    (b"nargou", b"t", Next::Letter(b"t")),
    (b"p", b"i", Next::Word(b"i")),
];

/// Whether `word`, a whole run of word characters, is a name the command
/// syntax takes: an ASCII letter first, and none of [`NOT_COMMANDS`].
/// Along a spelled prefix of [`SPELLED`], the next character must continue
/// the spelling or be one its class allows.
/// Past a whole excluded name, any word characters may follow.
fn command_name(word: &[u8]) -> bool {
    if !word.first().is_some_and(u8::is_ascii_alphabetic) || NOT_COMMANDS.contains(&word) {
        return false;
    }
    let mut k = 1;
    loop {
        let prefix = &word[..k];
        let Some(&(_, continues, next)) = SPELLED.iter().find(|(spelled, ..)| *spelled == prefix)
        else {
            return true;
        };
        let Some(&c) = word.get(k) else {
            return true;
        };
        if continues.contains(&c) {
            k += 1;
            continue;
        }
        return match next {
            Next::Word(not) => (c == b'_' || c.is_ascii_alphanumeric()) && !not.contains(&c),
            Next::Letter(not) => c.is_ascii_alphabetic() && !not.contains(&c),
            Next::Any => true,
        };
    }
}
