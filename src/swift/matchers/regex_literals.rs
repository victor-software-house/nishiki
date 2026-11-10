//! Matchers: `regex_literals`.
//!
//! The regex literals of Swift: `/…/` and `#/…/#`, and the regex syntax inside
//! them.
//! The two literal matchers, `At1` and `At2`, read a whole literal on one
//! line.
//! They carry the memos that keep a hostile line linear.

use std::cell::{Cell, RefCell};

use super::super::engine::Lexer;
use crate::classes::is_space;
use crate::rules::Chars;

/// A scan for the first position at or after `from` that satisfies a test:
/// `(key, from, found)`.
/// It records that no position in `from..found` satisfies the test.
/// The key separates queries whose test differs, such as the `#` count.
type Found = Cell<Option<(usize, usize, usize)>>;

/// What the matchers of this file remember about a line.
#[derive(Debug, Default)]
pub(in crate::swift) struct Memo {
    /// The first `)`.
    paren: Found,
    /// The first `}`.
    brace: Found,
    /// The first `,,`.
    comma2: Found,
    /// The first end of a `\Q` quote.
    quote: Found,
    /// The first run of `}` as long as the key.
    braces: Found,
    /// The last run of `#` that a matcher scanned: where it starts and ends.
    /// A search that starts inside it ends where it does.
    hashes: Cell<(usize, usize)>,
    /// The last long scan of a literal's contents.
    scan: Cell<Option<Scan>>,
    /// The states that scan passed through, in order.
    /// Swift keeps its buffer from line to line.
    /// It is therefore allocated once.
    path: RefCell<Vec<(usize, usize)>>,
}

impl Memo {
    /// A memo for a new line whose path reuses the buffer `path`.
    pub(in crate::swift) fn with_path(mut path: Vec<(usize, usize)>) -> Memo {
        path.clear();
        Memo {
            path: RefCell::new(path),
            ..Memo::default()
        }
    }

    /// The path's buffer, for the next line.
    pub(in crate::swift) fn into_path(self) -> Vec<(usize, usize)> {
        self.path.into_inner()
    }
}

/// The outcome of the last scan of a literal's contents that was long enough
/// to remember.
/// It holds the key, where the scan started, and what it found.
/// The key is the `#` count plus one.
#[derive(Clone, Copy, Debug)]
struct Scan {
    key: usize,
    start: usize,
    end: Option<usize>,
}

/// A scan this long (in iterations) leaves its path in the memo.
const LONG_SCAN: usize = 48;

/// The capture groups of a match, kept until the match is known to succeed.
#[derive(Clone, Copy, Debug, Default)]
struct Spans {
    list: [(usize, usize, usize); 14],
    len: usize,
}

impl Spans {
    /// Notes group `group` as `start..end`.
    fn push(&mut self, group: usize, start: usize, end: usize) {
        if let Some(slot) = self.list.get_mut(self.len) {
            *slot = (group, start, end);
            self.len += 1;
        }
    }
}

/// How a walk through a literal's contents treats the memo's path.
#[derive(Clone, Copy, Debug)]
enum Mode {
    /// Walks, and stops where the remembered path was (when `merge`).
    Walk { merge: bool },
    /// Walks and writes the path.
    Record,
}

/// One step through the contents of a literal.
#[derive(Clone, Copy, Debug)]
enum Step {
    /// An item of the contents ends here.
    Item(usize),
    /// A `(` opened a group.
    /// Its contents start here.
    Open(usize),
    /// No item starts here.
    Stop,
}

/// The first alternative that starts a group with the options before a `:`.
const GROUP_NAMES: [&[u8]; 17] = [
    b"atomic",
    b"pla",
    b"positive_lookahead",
    b"nla",
    b"negative_lookahead",
    b"plb",
    b"positive_lookbehind",
    b"nlb",
    b"negative_lookbehind",
    b"napla",
    b"non_atomic_positive_lookahead",
    b"naplb",
    b"non_atomic_positive_lookbehind",
    b"sr",
    b"script_run",
    b"asr",
    b"atomic_script_run",
];

/// The PCRE verbs of `(*VERB)` and `(*VERB:name)`, in the grammar's order.
/// The empty one is `(?=:)`: a name with no verb.
const VERBS: [&[u8]; 9] = [
    b"ACCEPT", b"FAIL", b"F", b"MARK", b"", b"COMMIT", b"PRUNE", b"SKIP", b"THEN",
];

/// The global matching options of `(*OPTION)`, in the grammar's order.
const OPTIONS: [&[u8]; 16] = [
    b"CRLF",
    b"CR",
    b"ANYCRLF",
    b"ANY",
    b"LF",
    b"NUL",
    b"BSR_ANYCRLF",
    b"BSR_UNICODE",
    b"NOTEMPTY_ATSTART",
    b"NOTEMPTY",
    b"NO_AUTO_POSSESS",
    b"NO_DOTSTAR_ANCHOR",
    b"NO_JIT",
    b"NO_START_OPT",
    b"UTF",
    b"UCP",
];

impl Lexer<'_> {
    /// The end of the run of `#` from `p`.
    /// Every start inside a run looks it up.
    /// The last run is therefore kept.
    fn hash_run_end(&self, p: usize) -> usize {
        let (start, end) = self.x.regex_literals.hashes.get();
        if start <= p && p < end {
            return end;
        }
        let end = self.skip(p, |b| b == b'#');
        if end > p {
            self.x.regex_literals.hashes.set((p, end));
        }
        end
    }
}

impl Lexer<'_> {
    /// `(#+)/\n`, for `LiteralsRegularExpressionLiteralAt0`.
    ///
    /// The `/` must be the last character.
    /// The match therefore takes the line feed.
    pub(in crate::swift) fn rx_literals_regular_expression_literal_at_0(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let hashes = self.hash_run_end(p);
        if hashes == p || hashes + 1 != self.lim() || self.at(hashes) != b'/' {
            return None;
        }
        self.capture(1, p, hashes);
        Some(hashes + 2)
    }

    /// `/\1`, for `LiteralsRegularExpressionLiteralAt0End`.
    pub(in crate::swift) fn rx_literals_regular_expression_literal_at_0_end(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        if self.at(p) != b'/' {
            return None;
        }
        self.begin_text_at(p + 1).map(|width| p + 1 + width)
    }

    /// `\\Q`, for `LiteralsRegularExpressionLiteralQuote`.
    pub(in crate::swift) fn rx_literals_regular_expression_literal_quote(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let end = self.lit(p, b"\\Q")?;
        self.capture(0, p, end);
        Some(end)
    }

    /// `\\E|(\n)`, for `LiteralsRegularExpressionLiteralQuoteEnd`.
    pub(in crate::swift) fn rx_literals_regular_expression_literal_quote_end(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        if let Some(end) = self.lit(p, b"\\E") {
            self.capture(0, p, end);
            return Some(end);
        }
        if p == self.lim() && self.nl {
            self.capture(0, p, p + 1);
            self.capture(1, p, p + 1);
            return Some(p + 1);
        }
        None
    }

    /// `\(\?#`, for `LiteralsRegularExpressionLiteralRegexGutsAt1`.
    pub(in crate::swift) fn rx_literals_regular_expression_literal_regex_guts_at_1(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let end = self.lit(p, b"(?#")?;
        self.capture(0, p, end);
        Some(end)
    }

    /// `<\{`, for `LiteralsRegularExpressionLiteralRegexGutsAt2`.
    pub(in crate::swift) fn rx_literals_regular_expression_literal_regex_guts_at_2(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let end = self.lit(p, b"<{")?;
        self.capture(0, p, end);
        Some(end)
    }

    /// `}>`, for `LiteralsRegularExpressionLiteralRegexGutsAt2End`.
    pub(in crate::swift) fn rx_literals_regular_expression_literal_regex_guts_at_2_end(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let end = self.lit(p, b"}>")?;
        self.capture(0, p, end);
        Some(end)
    }

    /// `\\(?:u\{\s*(?:\h+\s*)+}|u\h{4}|x\{\h+}|x\h{0,2}|U\h{8}|o\
    /// {[0-7]+}|0[0-7]{0,3}|N\{(?:U\+\h{1,8}|[-\s\w]+)})`, for
    /// `LiteralsRegularExpressionLiteralUnicodeScalars`.
    pub(in crate::swift) fn rx_literals_regular_expression_literal_unicode_scalars(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        if self.at(p) != b'\\' {
            return None;
        }
        let q = p + 2;
        match self.at(p + 1) {
            b'u' => self.unicode_braced(q).or_else(|| self.exact_hex(q, 4)),
            b'x' => self
                .braced(q, |b| b.is_ascii_hexdigit())
                .or_else(|| Some(q + self.hex_count(q, 2))),
            b'U' => self.exact_hex(q, 8),
            b'o' => self.braced(q, |b| matches!(b, b'0'..=b'7')),
            b'0' => Some(
                q + (0..3)
                    .take_while(|k| matches!(self.at(q + k), b'0'..=b'7'))
                    .count(),
            ),
            b'N' => self.named_scalar(q),
            _ => None,
        }
    }

    /// `\\[Pp]\{([-\s\w]+(?:=[-\s\w]+)?)}|(\[:)([-\s\w]+(?:=[-\s\w]+)?)(:])`,
    /// for `LiteralsRegularExpressionLiteralCharacterProperties`.
    pub(in crate::swift) fn rx_literals_regular_expression_literal_character_properties(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        if self.at(p) == b'\\' && matches!(self.at(p + 1), b'P' | b'p') && self.at(p + 2) == b'{' {
            let body = self.property_end(p + 3)?;
            if self.at(body) != b'}' {
                return None;
            }
            self.capture(1, p + 3, body);
            return Some(body + 1);
        }
        let body = self
            .lit(p, b"[:")
            .and_then(|open| self.property_end(open))?;
        let end = self.lit(body, b":]")?;
        self.capture(2, p, p + 2);
        self.capture(3, p + 2, body);
        self.capture(4, body, end);
        Some(end)
    }

    /// `[$^]|\\[ABGYZbyz]|\\K`, for
    /// `LiteralsRegularExpressionLiteralRegexGutsAt5`.
    pub(in crate::swift) fn rx_literals_regular_expression_literal_regex_guts_at_5(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        match self.at(p) {
            b'$' | b'^' => Some(p + 1),
            b'\\'
                if matches!(self.at(p + 1), b'A' | b'B' | b'G' | b'Y' | b'Z' | b'K')
                    || matches!(self.at(p + 1), b'b' | b'y' | b'z') =>
            {
                Some(p + 2)
            }
            _ => None,
        }
    }

    /// `(\(\*)(?:(ACCEPT|FAIL|F|MARK(?=:)|(?=:)|COMMIT|PRUNE|SKIP|THEN)(?:(:
    /// )([^)]+))?|(LIMIT_(?:DEPTH|HEAP|MATCH))(=)(\
    /// d+)|(CRLF|CR|ANYCRLF|ANY|LF|NUL|BSR_ANYCRLF|BSR_UNICODE|NOTEMPTY_ATSTART|NOTEMPTY|NO_AUTO_POSSESS|NO_DOTSTAR_ANCHOR|NO_JIT|NO_START_OPT|UTF|UCP))(\
    /// ))`, for `LiteralsRegularExpressionLiteralBacktrackingDirectiveOrGlobalMatchingOption`.
    pub(in crate::swift) fn rx_literals_regular_expression_literal_backtracking_directive_or_global_matching_option(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let q = self.lit(p, b"(*")?;
        let mut spans = Spans::default();
        spans.push(1, p, q);
        let end = self
            .verb(q, &mut spans)
            .or_else(|| self.limit(q, &mut spans))
            .or_else(|| self.global_option(q, &mut spans))?;
        self.record(&spans);
        Some(end)
    }

    /// The three callouts:
    /// - `(?C…)`, with a number or a name that doubles its closing delimiter;
    /// - `(*name[tag]{args})`;
    /// - `(?{code}[tag]X)`, whose braces nest through the recursive group 20.
    ///
    /// The regex is the `regular-expression-literal-callout` begin of the
    /// grammar, for `LiteralsRegularExpressionLiteralCallout`.
    pub(in crate::swift) fn rx_literals_regular_expression_literal_callout(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        if self.at(p) != b'(' {
            return None;
        }
        let mut spans = Spans::default();
        let end = match self.at(p + 1) {
            b'?' if self.at(p + 2) == b'C' => self.numbered_callout(p, &mut spans),
            b'*' => self.named_callout(p, &mut spans),
            b'?' => self.code_callout(p, &mut spans),
            _ => None,
        }?;
        self.record(&spans);
        Some(end)
    }

    /// `(\\g\{)(?:((?!\d)\w+)(?:([-+])(\d+))?|([-+]?\d+)(?:([-+])(\d+))?)(})`,
    /// for `LiteralsRegularExpressionLiteralBackreferenceOrSubpatternAt0`.
    pub(in crate::swift) fn rx_literals_regular_expression_literal_backreference_or_subpattern_at_0(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        if !self.starts(p, b"\\g{") {
            return None;
        }
        self.reference(p, b'}')
    }

    /// `(\\g)([-+]?\d+)(?:([-+])(\d+))?`, for
    /// `LiteralsRegularExpressionLiteralBackreferenceOrSubpatternAt1`.
    pub(in crate::swift) fn rx_literals_regular_expression_literal_backreference_or_subpattern_at_1(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let a = self.lit(p, b"\\g")?;
        let number = self.number_end(a)?;
        let mut spans = Spans::default();
        spans.push(1, p, a);
        spans.push(2, a, number);
        let end = self.offset(number, 3, &mut spans);
        self.record(&spans);
        Some(end)
    }

    /// `(\\[gk]<)(?:((?!\d)\w+)(?:([-+])(\d+))?|([-+]?\d+)(?:([-+])(\d+))?
    /// )(>)`, for
    /// `LiteralsRegularExpressionLiteralBackreferenceOrSubpatternAt2`.
    pub(in crate::swift) fn rx_literals_regular_expression_literal_backreference_or_subpattern_at_2(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        if self.at(p) != b'\\' || !matches!(self.at(p + 1), b'g' | b'k') || self.at(p + 2) != b'<' {
            return None;
        }
        self.reference(p, b'>')
    }

    /// `(\\[gk]')(?:((?!\d)\w+)(?:([-+])(\d+))?|([-+]?\d+)(?:([-+])(\d+))?)('
    /// )`, for `LiteralsRegularExpressionLiteralBackreferenceOrSubpatternAt3`.
    pub(in crate::swift) fn rx_literals_regular_expression_literal_backreference_or_subpattern_at_3(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        if self.at(p) != b'\\' || !matches!(self.at(p + 1), b'g' | b'k') || self.at(p + 2) != b'\''
        {
            return None;
        }
        self.reference(p, b'\'')
    }

    /// `(\\k\{)((?!\d)\w+)(?:([-+])(\d+))?(})`, for
    /// `LiteralsRegularExpressionLiteralBackreferenceOrSubpatternAt4`.
    pub(in crate::swift) fn rx_literals_regular_expression_literal_backreference_or_subpattern_at_4(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let open = self.lit(p, b"\\k{")?;
        self.named_reference(p, open, b'}')
    }

    /// `\\[1-9][0-9]+`, for
    /// `LiteralsRegularExpressionLiteralBackreferenceOrSubpatternAt5`.
    pub(in crate::swift) fn rx_literals_regular_expression_literal_backreference_or_subpattern_at_5(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        if self.at(p) != b'\\' || !matches!(self.at(p + 1), b'1'..=b'9') {
            return None;
        }
        let end = self.skip(p + 2, |b| b.is_ascii_digit());
        (end > p + 2).then_some(end)
    }

    /// `(\(\?(?:P[=>]|&))((?!\d)\w+)(?:([-+])(\d+))?(\))`, for
    /// `LiteralsRegularExpressionLiteralBackreferenceOrSubpatternAt6`.
    pub(in crate::swift) fn rx_literals_regular_expression_literal_backreference_or_subpattern_at_6(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let q = self.lit(p, b"(?")?;
        let open = match self.at(q) {
            b'P' if matches!(self.at(q + 1), b'=' | b'>') => q + 2,
            b'&' => q + 1,
            _ => return None,
        };
        self.named_reference(p, open, b')')
    }

    /// `\(\?R\)`, for
    /// `LiteralsRegularExpressionLiteralBackreferenceOrSubpatternAt7`.
    pub(in crate::swift) fn rx_literals_regular_expression_literal_backreference_or_subpattern_at_7(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.lit(p, b"(?R)")
    }

    /// `(\(\?)([-+]?\d+)(?:([-+])(\d+))?(\))`, for
    /// `LiteralsRegularExpressionLiteralBackreferenceOrSubpatternAt8`.
    pub(in crate::swift) fn rx_literals_regular_expression_literal_backreference_or_subpattern_at_8(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let a = self.lit(p, b"(?")?;
        let number = self.number_end(a)?;
        let mut spans = Spans::default();
        spans.push(1, p, a);
        spans.push(2, a, number);
        let end = self.offset(number, 3, &mut spans);
        if self.at(end) != b')' {
            return None;
        }
        spans.push(5, end, end + 1);
        self.record(&spans);
        Some(end + 1)
    }

    /// `\.|\\[CDHNORSVWXdhsvw]`, for
    /// `LiteralsRegularExpressionLiteralRegexGutsAt9`.
    pub(in crate::swift) fn rx_literals_regular_expression_literal_regex_guts_at_9(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        match self.at(p) {
            b'.' => Some(p + 1),
            b'\\' if b"CDHNORSVWXdhsvw".contains(&self.at(p + 1)) => Some(p + 2),
            _ => None,
        }
    }

    /// `\\c.`, for `LiteralsRegularExpressionLiteralRegexGutsAt10`.
    ///
    /// `.` takes no line feed.
    pub(in crate::swift) fn rx_literals_regular_expression_literal_regex_guts_at_10(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let a = self.lit(p, b"\\c")?;
        (a < self.lim()).then(|| self.char_end(a))
    }

    /// `\\[^c]`, for `LiteralsRegularExpressionLiteralRegexGutsAt11`.
    ///
    /// `[^c]` takes the line feed.
    pub(in crate::swift) fn rx_literals_regular_expression_literal_regex_guts_at_11(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        if self.at(p) != b'\\' || p + 1 > self.lim() || self.at(p + 1) == b'c' {
            return None;
        }
        Some(if p + 1 == self.lim() {
            p + 2
        } else {
            self.char_end(p + 1)
        })
    }

    /// `\|`, for `LiteralsRegularExpressionLiteralRegexGutsAt12`.
    pub(in crate::swift) fn rx_literals_regular_expression_literal_regex_guts_at_12(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.lit(p, b"|")
    }

    /// `[*+?]`, for `LiteralsRegularExpressionLiteralRegexGutsAt13`.
    pub(in crate::swift) fn rx_literals_regular_expression_literal_regex_guts_at_13(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        matches!(self.at(p), b'*' | b'+' | b'?').then_some(p + 1)
    }

    /// `\{(?:\s*\d+\s*(?:,\s*\d*\s*)?}|\s*,\s*\d+\s*})`, for
    /// `LiteralsRegularExpressionLiteralRegexGutsAt14`.
    pub(in crate::swift) fn rx_literals_regular_expression_literal_regex_guts_at_14(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let a = self.lit(p, b"{")?;
        let first = self.space_end(a);
        let closer = if self.at(first) == b',' {
            let digits = self.space_end(first + 1);
            let end = self.digit_end(digits);
            (end > digits).then(|| self.space_end(end))?
        } else {
            let end = self.digit_end(first);
            if end == first {
                return None;
            }
            let after = self.space_end(end);
            if self.at(after) == b',' {
                let digits = self.space_end(after + 1);
                self.space_end(self.digit_end(digits))
            } else {
                after
            }
        };
        (self.at(closer) == b'}').then_some(closer + 1)
    }

    /// `(\[)(\^)?`, for `LiteralsRegularExpressionLiteralCustomCharClassAt0`.
    pub(in crate::swift) fn rx_literals_regular_expression_literal_custom_char_class_at_0(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let a = self.lit(p, b"[")?;
        self.capture(1, p, a);
        if self.at(a) == b'^' {
            self.capture(2, a, a + 1);
            return Some(a + 1);
        }
        Some(a)
    }

    /// `]`, for `LiteralsRegularExpressionLiteralCustomCharClassAt0End`,
    /// `ExpressionsWithoutTrailingClosuresOrMemberReferencesSubscriptExpressionEnd`.
    pub(in crate::swift) fn rx_literals_regular_expression_literal_custom_char_class_at_0_end(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let end = self.lit(p, b"]")?;
        self.capture(0, p, end);
        Some(end)
    }

    /// `\\b`, for `LiteralsRegularExpressionLiteralCustomCharClassMembersAt0`.
    pub(in crate::swift) fn rx_literals_regular_expression_literal_custom_char_class_members_at_0(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.lit(p, b"\\b")
    }

    /// `&&`, for `LiteralsRegularExpressionLiteralSetOperatorsAt0`.
    pub(in crate::swift) fn rx_literals_regular_expression_literal_set_operators_at_0(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.lit(p, b"&&")
    }

    /// `--`, for `LiteralsRegularExpressionLiteralSetOperatorsAt1`.
    pub(in crate::swift) fn rx_literals_regular_expression_literal_set_operators_at_1(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.lit(p, b"--")
    }

    /// `~~`, for `LiteralsRegularExpressionLiteralSetOperatorsAt2`.
    pub(in crate::swift) fn rx_literals_regular_expression_literal_set_operators_at_2(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.lit(p, b"~~")
    }

    /// `\(\?(?:\^(?:[DJPSUWimnswx]|xx|y\{[gw]})*|(?:[DJPSUWimnswx]|xx|y\
    /// {[gw]})+|(?:[DJPSUWimnswx]|xx|y\{[gw]})*-(?:[DJPSUWimnswx]|xx|y\{[gw]})*
    /// )\)`, for `LiteralsRegularExpressionLiteralGroupOptionToggle`.
    pub(in crate::swift) fn rx_literals_regular_expression_literal_group_option_toggle(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let a = self.lit(p, b"(?")?;
        let end = self.option_toggle(a, b')')?;
        Some(end + 1)
    }

    /// `(\()(\?~)`, for
    /// `LiteralsRegularExpressionLiteralGroupOrConditionalAt0`.
    pub(in crate::swift) fn rx_literals_regular_expression_literal_group_or_conditional_at_0(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let end = self.lit(p, b"(?~")?;
        self.capture(1, p, p + 1);
        self.capture(2, p + 1, end);
        Some(end)
    }

    /// `(\()(?<cond>\?\()(?:(?<NumberRef>(?<num>[-+]?\d+)(?:(?<op>[-+])(?<num>\
    /// d+))?)|(?<cond>R)\g<NumberRef>?|(?<cond>R&)(?<NamedRef>(?<name>(?!\d)\
    /// w+)(?:(?<op>[-+])(?<num>\d+))?)|(?<cond><)(?:\g<NamedRef>|\
    /// g<NumberRef>)(?<cond>>)|(?<cond>')(?:\g<NamedRef>|\g<NumberRef>)(?
    /// <cond>')|(?<cond>DEFINE)|(?<cond>VERSION)(?<compar>>?=)(?<num>\d+\.\
    /// d+))(?<cond>\))|(\()(?<cond>\?)(?=\()`, for
    /// `LiteralsRegularExpressionLiteralGroupOrConditionalAt1`.
    pub(in crate::swift) fn rx_literals_regular_expression_literal_group_or_conditional_at_1(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let q = self.lit(p, b"(")?;
        if self.starts(q, b"?(") {
            let mut spans = Spans::default();
            spans.push(1, p, q);
            spans.push(2, q, q + 2);
            if let Some(end) = self.condition(q + 2, &mut spans) {
                self.record(&spans);
                return Some(end);
            }
        }
        if self.at(q) == b'?' && self.at(q + 1) == b'(' {
            self.capture(22, p, q);
            self.capture(23, q, q + 1);
            return Some(q + 1);
        }
        None
    }

    /// `(\()((\?)(?:([!*:=>|]|<[!*=])|P?<(?:((?!\d)\w+)(-))?((?!\d)\w+)>|'(?:
    /// ((?!\d)\w+)(-))?((?!\d)\w+)'|(?:\^(?:[DJPSUWimnswx]|xx|y\{[gw]})*|(?:
    /// [DJPSUWimnswx]|xx|y\{[gw]})+|(?:[DJPSUWimnswx]|xx|y\{[gw]})*-(?:
    /// [DJPSUWimnswx]|xx|y\{[gw]})*):)|\*
    /// (atomic|pla|positive_lookahead|nla|negative_lookahead|plb|positive_lookbehind|nlb|negative_lookbehind|napla|non_atomic_positive_lookahead|naplb|non_atomic_positive_lookbehind|sr|script_run|asr|atomic_script_run):
    /// )?+`, for `LiteralsRegularExpressionLiteralGroupOrConditionalAt2`.
    ///
    /// The whole group is optional and possessive: a `(` always matches.
    pub(in crate::swift) fn rx_literals_regular_expression_literal_group_or_conditional_at_2(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let q = self.lit(p, b"(")?;
        let mut spans = Spans::default();
        spans.push(1, p, q);
        let end = self.group_head(q, &mut spans).unwrap_or(q);
        self.record(&spans);
        Some(end)
    }

    /// `(#).*$`, for `LiteralsRegularExpressionLiteralLineComment`.
    pub(in crate::swift) fn rx_literals_regular_expression_literal_line_comment(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let a = self.lit(p, b"#")?;
        self.capture(1, p, a);
        Some(self.lim())
    }

    /// `(/)(?!\s)(?!/)(?:\\\s(?=/)|(?<guts>(?>(?:\\Q(?:(?!\\E)(?!/).)*+(?:\\
    /// E|(?=/))|\\.|\(\?#[^)]*\)|\(\?(?>\{(?:[^{].*?|\{[^{].*?}|\{\{[^{].*?}}|\
    /// {\{\{[^{].*?}}}|\{\{\{\{[^{].*?}}}}|\{\{\{\{\{.+?}}}}})})(?:\[(?!\d)\
    /// w+])?[<>X]?\)|\[(?:\\.|[^]\[\\]|\[(?:\\.|[^]\[\\]|\[(?:\\.|[^]\[\\]|\[(?
    /// :\\.|[^]\[\\])+])+])+])+]|\(\g<guts>?+\)|(?:(?!/)[^()\[\\])+)+))?+(?<!\
    /// s))(/)`, for `LiteralsRegularExpressionLiteralAt1`.
    ///
    /// `\\\s(?=/)`: an escaped blank.
    /// The contents cannot hold it, because a literal may not end in one.
    pub(in crate::swift) fn rx_literals_regular_expression_literal_at_1(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let a = self.lit(p, b"/")?;
        if self.space_at(a) > 0 || self.at(a) == b'/' {
            return None;
        }
        if self.at(a) == b'\\' {
            let close = a + 1 + self.space_at(a + 1);
            if close > a + 1 && self.at(close) == b'/' {
                self.capture(0, p, close + 1);
                self.capture(1, p, a);
                self.capture(3, close, close + 1);
                return Some(close + 1);
            }
        }
        let end = self.contents_end(a, 1, &[])?;
        if self.at(end) != b'/' || self.char_before(end).is_some_and(is_space) {
            return None;
        }
        self.capture(0, p, end + 1);
        self.capture(1, p, a);
        self.capture(3, end, end + 1);
        Some(end + 1)
    }

    /// `((#+)/)(?<guts>(?>(?:\\Q(?:(?!\\E)(?!/\2).)*+(?:\\E|(?=/\2))|\\.|\(\?#
    /// [^)]*\)|\(\?(?>\{(?:[^{].*?|\{[^{].*?}|\{\{[^{].*?}}|\{\{\{[^{].*?}}}|\
    /// {\{\{\{[^{].*?}}}}|\{\{\{\{\{.+?}}}}})})(?:\[(?!\d)\w+])?[<>X]?\)|\[(?:\
    /// \.|[^]\[\\]|\[(?:\\.|[^]\[\\]|\[(?:\\.|[^]\[\\]|\[(?:\\.|[^]\[\\
    /// ])+])+])+])+]|\(\g<guts>?+\)|(?:(?!/\2)[^()\[\\])+)+))?+(/\2)|#+/.+(\
    /// n)`, for `LiteralsRegularExpressionLiteralAt2`.
    ///
    /// `#+/.+(\n)`: an unterminated literal runs to the end of the line.
    pub(in crate::swift) fn rx_literals_regular_expression_literal_at_2(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let hashes_end = self.hash_run_end(p);
        if hashes_end == p || self.at(hashes_end) != b'/' {
            return None;
        }
        let a = hashes_end + 1;
        let hashes = self.text(p, hashes_end);
        if let Some(end) = self.contents_end(a, hashes.len() + 1, hashes)
            && self.closes(end, hashes)
        {
            let after = end + 1 + hashes.len();
            self.capture(0, p, after);
            self.capture(1, p, a);
            self.capture(4, end, after);
            return Some(after);
        }
        if a < self.lim() {
            self.capture(0, p, self.lim() + 1);
            self.capture(5, self.lim(), self.lim() + 1);
            return Some(self.lim() + 1);
        }
        None
    }
}

/// Helpers.
impl Lexer<'_> {
    /// Records the groups of a match.
    fn record(&mut self, spans: &Spans) {
        for &(group, start, end) in spans.list.iter().take(spans.len) {
            self.capture(group, start, end);
        }
    }

    /// The first `i` at or after `from` where `hit` holds, or the end of the
    /// line when none does.
    /// `memo` keeps the answer.
    /// A line of many starts that all look for the same stop then scans it
    /// once.
    fn find_from(
        &self,
        memo: &Found,
        key: usize,
        from: usize,
        hit: impl Fn(usize) -> bool,
    ) -> usize {
        if let Some((k, start, found)) = memo.get()
            && k == key
            && start <= from
            && from <= found
        {
            return found;
        }
        let mut i = from;
        while i < self.lim() && !hit(i) {
            i += 1;
        }
        self.spend(i.saturating_sub(from) + 1);
        memo.set(Some((key, from, i)));
        i
    }

    /// The first `)` at or after `from`, or the end of the line.
    fn paren_from(&self, from: usize) -> usize {
        let memo = &self.x.regex_literals.paren;
        self.find_from(memo, 0, from, |i| self.at(i) == b')')
    }

    /// The first `}` at or after `from`, or the end of the line.
    fn brace_from(&self, from: usize) -> usize {
        let memo = &self.x.regex_literals.brace;
        self.find_from(memo, 0, from, |i| self.at(i) == b'}')
    }

    /// The start of the first run of `need` `}` at or after `from`, or the
    /// end of the line.
    fn braces_from(&self, from: usize, need: usize) -> usize {
        let memo = &self.x.regex_literals.braces;
        if let Some((k, start, found)) = memo.get()
            && k == need
            && start <= from
            && from <= found
        {
            return found;
        }
        let mut i = from;
        loop {
            while i < self.lim() && self.at(i) != b'}' {
                i += 1;
            }
            if i >= self.lim() {
                break;
            }
            let run_end = self.skip(i, |b| b == b'}');
            if run_end - i >= need {
                break;
            }
            i = run_end;
        }
        self.spend(i.saturating_sub(from) + 1);
        memo.set(Some((need, from, i)));
        i
    }

    /// Whether a digit stands at `i`.
    fn digit_at(&self, i: usize) -> bool {
        i < self.lim() && self.digit_end(i) > i
    }

    /// The end of `(?!\d)\w+` at `i`.
    fn name_end(&self, i: usize) -> Option<usize> {
        if self.digit_at(i) {
            return None;
        }
        self.word_at(i)?;
        Some(self.word_end(i))
    }

    /// The end of `[-+]?\d+` at `i`.
    fn number_end(&self, i: usize) -> Option<usize> {
        let digits = if matches!(self.at(i), b'+' | b'-') {
            i + 1
        } else {
            i
        };
        let end = self.digit_end(digits);
        (end > digits).then_some(end)
    }

    /// `(?:([-+])(\d+))?` at `i`.
    /// It notes the sign as group `group` and the digits as the next group.
    /// It returns the end, which is `i` for no match.
    fn offset(&self, i: usize, group: usize, spans: &mut Spans) -> usize {
        match self.offset_end(i) {
            Some(end) => {
                spans.push(group, i, i + 1);
                spans.push(group + 1, i + 1, end);
                end
            }
            None => i,
        }
    }

    /// The end of `([-+])(\d+)` at `i`.
    fn offset_end(&self, i: usize) -> Option<usize> {
        if !matches!(self.at(i), b'+' | b'-') {
            return None;
        }
        let end = self.digit_end(i + 1);
        (end > i + 1).then_some(end)
    }

    /// The width of `count` hexadecimal digits at most, from `i`.
    fn hex_count(&self, i: usize, count: usize) -> usize {
        (0..count)
            .take_while(|k| self.at(i + k).is_ascii_hexdigit())
            .count()
    }

    /// The end of exactly `count` hexadecimal digits at `i`.
    fn exact_hex(&self, i: usize, count: usize) -> Option<usize> {
        (self.hex_count(i, count) == count).then_some(i + count)
    }

    /// The end of `\{<class>+}` at `i`.
    fn braced(&self, i: usize, class: impl Fn(u8) -> bool) -> Option<usize> {
        if self.at(i) != b'{' {
            return None;
        }
        let end = self.skip(i + 1, class);
        (end > i + 1 && self.at(end) == b'}').then_some(end + 1)
    }

    /// The end of `\{\s*(?:\h+\s*)+}` at `i`: blanks, then at least one hex
    /// digit among hex digits and blanks, then `}`.
    fn unicode_braced(&self, i: usize) -> Option<usize> {
        if self.at(i) != b'{' {
            return None;
        }
        let mut j = self.space_end(i + 1);
        if !self.at(j).is_ascii_hexdigit() {
            return None;
        }
        loop {
            let width = if self.at(j).is_ascii_hexdigit() {
                1
            } else {
                self.space_at(j)
            };
            if width == 0 {
                break;
            }
            j += width;
        }
        self.spend(j - i);
        (self.at(j) == b'}').then_some(j + 1)
    }

    /// The end of `N\{(?:U\+\h{1,8}|[-\s\w]+)}` after the `N`, at `i`.
    fn named_scalar(&self, i: usize) -> Option<usize> {
        if self.at(i) != b'{' {
            return None;
        }
        if self.at(i + 1) == b'U' && self.at(i + 2) == b'+' {
            let count = self.hex_count(i + 3, 8);
            if count > 0 && self.at(i + 3 + count) == b'}' {
                return Some(i + 4 + count);
            }
        }
        let end = self.property_run(i + 1);
        (end > i + 1 && self.at(end) == b'}').then_some(end + 1)
    }

    /// The end of the run of `[-\s\w]` from `i`.
    fn property_run(&self, mut i: usize) -> usize {
        let from = i;
        loop {
            let width = if self.at(i) == b'-' {
                1
            } else if self.space_at(i) > 0 {
                self.space_at(i)
            } else if let Some(width) = self.word_at(i) {
                width
            } else {
                break;
            };
            i += width;
        }
        self.spend(i - from);
        i
    }

    /// The end of `[-\s\w]+(?:=[-\s\w]+)?` at `i`.
    fn property_end(&self, i: usize) -> Option<usize> {
        let end = self.property_run(i);
        if end == i {
            return None;
        }
        if self.at(end) == b'=' {
            let value = self.property_run(end + 1);
            if value > end + 1 {
                return Some(value);
            }
        }
        Some(end)
    }

    /// The end of one option of a group: `[DJPSUWimnswx]`, `xx` (which the
    /// first branch already reads as two) or `y\{[gw]}`.
    fn option_end(&self, i: usize) -> Option<usize> {
        if b"DJPSUWimnswx".contains(&self.at(i)) {
            return Some(i + 1);
        }
        (self.at(i) == b'y'
            && self.at(i + 1) == b'{'
            && matches!(self.at(i + 2), b'g' | b'w')
            && self.at(i + 3) == b'}')
            .then_some(i + 4)
    }

    /// The end of the options from `i`.
    fn options_end(&self, mut i: usize) -> usize {
        while let Some(end) = self.option_end(i) {
            i = end;
        }
        i
    }

    /// The position of `close` after `^options`, `options` or
    /// `options-options` from `i`.
    /// Those are the three forms of a group's option toggle.
    fn option_toggle(&self, i: usize, close: u8) -> Option<usize> {
        if self.at(i) == b'^' {
            let end = self.options_end(i + 1);
            return (self.at(end) == close).then_some(end);
        }
        let end = self.options_end(i);
        if end > i && self.at(end) == close {
            return Some(end);
        }
        if self.at(end) == b'-' {
            let after = self.options_end(end + 1);
            return (self.at(after) == close).then_some(after);
        }
        None
    }

    /// `(open)(?:(name)(?:([-+])(\d+))?|([-+]?\d+)(?:([-+])(\d+))?)(close)`
    /// with an opener of three characters.
    fn reference(&mut self, p: usize, close: u8) -> Option<usize> {
        let a = p + 3;
        let mut spans = Spans::default();
        spans.push(1, p, a);
        let end = if let Some(name) = self.name_end(a) {
            spans.push(2, a, name);
            self.offset(name, 3, &mut spans)
        } else {
            let number = self.number_end(a)?;
            spans.push(5, a, number);
            self.offset(number, 6, &mut spans)
        };
        if self.at(end) != close {
            return None;
        }
        spans.push(8, end, end + 1);
        self.record(&spans);
        Some(end + 1)
    }

    /// `(open)((?!\d)\w+)(?:([-+])(\d+))?(close)`, with the opener up to
    /// `open`.
    fn named_reference(&mut self, p: usize, open: usize, close: u8) -> Option<usize> {
        let name = self.name_end(open)?;
        let mut spans = Spans::default();
        spans.push(1, p, open);
        spans.push(2, open, name);
        let end = self.offset(name, 3, &mut spans);
        if self.at(end) != close {
            return None;
        }
        spans.push(5, end, end + 1);
        self.record(&spans);
        Some(end + 1)
    }

    /// The verb alternative of a backtracking directive, after `(*` at `q`.
    ///
    /// `MARK(?=:)` and the empty verb `(?=:)` need the colon.
    fn verb(&self, q: usize, spans: &mut Spans) -> Option<usize> {
        for verb in VERBS {
            let end = q + verb.len();
            if self.lit(q, verb).is_none() {
                continue;
            }
            let needs_colon = verb == b"MARK" || verb.is_empty();
            let mut found = *spans;
            if !verb.is_empty() {
                found.push(2, q, end);
            }
            if self.at(end) == b')' && !needs_colon {
                found.push(9, end, end + 1);
                *spans = found;
                return Some(end + 1);
            }
            if self.at(end) == b':' {
                let close = self.paren_from(end + 1);
                if close < self.lim() && close > end + 1 {
                    found.push(3, end, end + 1);
                    found.push(4, end + 1, close);
                    found.push(9, close, close + 1);
                    *spans = found;
                    return Some(close + 1);
                }
            }
        }
        None
    }

    /// `(LIMIT_(?:DEPTH|HEAP|MATCH))(=)(\d+)` and the `)`, at `q`.
    fn limit(&self, q: usize, spans: &mut Spans) -> Option<usize> {
        let name = b"LIMIT_";
        let a = self.lit(q, name)?;
        let word = [b"DEPTH".as_slice(), b"HEAP", b"MATCH"]
            .into_iter()
            .find_map(|word| self.lit(a, word))?;
        if self.at(word) != b'=' {
            return None;
        }
        let digits = self.digit_end(word + 1);
        if digits == word + 1 || self.at(digits) != b')' {
            return None;
        }
        spans.push(5, q, word);
        spans.push(6, word, word + 1);
        spans.push(7, word + 1, digits);
        spans.push(9, digits, digits + 1);
        Some(digits + 1)
    }

    /// `(CRLF|CR|…|UCP)` and the `)`, at `q`.
    fn global_option(&self, q: usize, spans: &mut Spans) -> Option<usize> {
        for option in OPTIONS {
            let end = q + option.len();
            if self.starts(q, option) && self.at(end) == b')' {
                spans.push(8, q, end);
                spans.push(9, end, end + 1);
                return Some(end + 1);
            }
        }
        None
    }

    /// The closing delimiter of a quoted name from `i`, where the delimiter
    /// doubled stands for itself: `(?:[^d]|dd)*d`.
    fn quoted_end(&self, mut i: usize, delimiter: u8) -> Option<usize> {
        let from = i;
        while i < self.lim() {
            if self.at(i) == delimiter {
                if self.at(i + 1) != delimiter {
                    self.spend(i - from);
                    return Some(i);
                }
                i += 2;
            } else {
                i = self.char_end(i);
            }
        }
        self.spend(i - from);
        None
    }

    /// `(?:\[(?<tag>(?!\d)\w+)])?` at `i`: the end and the tag's span.
    fn tag_end(&self, i: usize) -> (usize, Option<(usize, usize)>) {
        if self.at(i) == b'['
            && let Some(name) = self.name_end(i + 1)
            && self.at(name) == b']'
        {
            return (name + 1, Some((i + 1, name)));
        }
        (i, None)
    }

    /// The callout `(?C…)` at `p`.
    fn numbered_callout(&self, p: usize, spans: &mut Spans) -> Option<usize> {
        let a = p + 3;
        spans.push(1, p, p + 1);
        spans.push(2, p + 1, a);
        let (group, delimiter) = match self.at(a) {
            b'`' => (4, b'`'),
            b'\'' => (5, b'\''),
            b'"' => (6, b'"'),
            b'^' => (7, b'^'),
            b'%' => (8, b'%'),
            b'#' => (9, b'#'),
            b'$' => (10, b'$'),
            b'{' => (11, b'}'),
            _ => {
                let end = self.digit_end(a);
                let end = if end > a {
                    spans.push(3, a, end);
                    end
                } else {
                    a
                };
                if self.at(end) != b')' {
                    return None;
                }
                spans.push(12, end, end + 1);
                return Some(end + 1);
            }
        };
        let close = self.quoted_end(a + 1, delimiter)?;
        if self.at(close + 1) != b')' {
            return None;
        }
        spans.push(group, a + 1, close);
        spans.push(12, close + 1, close + 2);
        Some(close + 2)
    }

    /// The callout `(*name[tag]{args})` at `p`.
    fn named_callout(&self, p: usize, spans: &mut Spans) -> Option<usize> {
        let a = p + 2;
        let name = self.name_end(a)?;
        spans.push(13, p, p + 1);
        spans.push(14, p + 1, a);
        spans.push(15, a, name);
        let (end, tag) = self.tag_end(name);
        if let Some((start, stop)) = tag {
            spans.push(16, start, stop);
        }
        let end = if self.at(end) == b'{' {
            self.arguments_end(end)?
        } else {
            end
        };
        if self.at(end) != b')' {
            return None;
        }
        spans.push(17, end, end + 1);
        Some(end + 1)
    }

    /// The end of `\{[^,}]+(?:,[^,}]+)*}` at `i`: at least one character in
    /// each comma-separated piece, up to the first `}`.
    fn arguments_end(&self, i: usize) -> Option<usize> {
        let close = self.brace_from(i + 1);
        if close >= self.lim() || close == i + 1 || self.at(i + 1) == b',' {
            return None;
        }
        let memo = &self.x.regex_literals.comma2;
        let double = self.find_from(memo, 0, i + 1, |j| self.starts(j, b",,"));
        (self.at(close - 1) != b',' && double + 1 >= close).then_some(close + 1)
    }

    /// The callout `(?{code}[tag]X)` at `p`.
    ///
    /// The group calls itself for each `{` that opens the code.
    /// The code therefore ends at the first run of as many `}`.
    /// The atomic group keeps the first way the recursion found.
    fn code_callout(&self, p: usize, spans: &mut Spans) -> Option<usize> {
        let open = p + 2;
        if self.at(open) != b'{' {
            return None;
        }
        let depth = self.skip(open, |b| b == b'{') - open;
        let close = self.braces_from(open + depth, depth);
        if close >= self.lim() {
            return None;
        }
        let (end, tag) = self.tag_end(close + depth);
        let keyword = matches!(self.at(end), b'<' | b'>' | b'X');
        let after = end + usize::from(keyword);
        if self.at(after) != b')' {
            return None;
        }
        spans.push(18, p, p + 1);
        spans.push(19, p + 1, open);
        if let Some((start, stop)) = tag {
            spans.push(21, start, stop);
        }
        if keyword {
            spans.push(22, end, after);
        }
        spans.push(23, after, after + 1);
        Some(after + 1)
    }

    /// A conditional's reference, `[-+]?\d+` and an offset, as `NumberRef`.
    fn number_ref(&self, i: usize, spans: &mut Spans) -> Option<usize> {
        let number = self.number_end(i)?;
        spans.push(4, i, number);
        Some(self.offset(number, 5, spans))
    }

    /// A conditional's reference, a name and an offset, as `NamedRef`.
    fn named_ref(&self, i: usize, spans: &mut Spans) -> Option<usize> {
        let name = self.name_end(i)?;
        spans.push(10, i, name);
        Some(self.offset(name, 11, spans))
    }

    /// The condition of `(?(…)` from `r`, up to and including the `)`.
    fn condition(&self, r: usize, spans: &mut Spans) -> Option<usize> {
        let close = |end: usize, mut found: Spans| {
            (self.at(end) == b')').then(|| {
                found.push(21, end, end + 1);
                (end + 1, found)
            })
        };
        let mut found = *spans;
        let outcome = if let Some(end) = self.number_ref(r, &mut found) {
            close(end, found)
        } else if self.at(r) == b'R' && self.at(r + 1) != b'&' {
            found.push(7, r, r + 1);
            let end = self.number_ref(r + 1, &mut found).unwrap_or(r + 1);
            close(end, found)
        } else if self.starts(r, b"R&") {
            found.push(8, r, r + 2);
            self.named_ref(r + 2, &mut found)
                .and_then(|end| close(end, found))
        } else if matches!(self.at(r), b'<' | b'\'') {
            let (open, shut) = if self.at(r) == b'<' {
                (13, 14)
            } else {
                (15, 16)
            };
            let delimiter = if self.at(r) == b'<' { b'>' } else { b'\'' };
            found.push(open, r, r + 1);
            let end = self
                .named_ref(r + 1, &mut found)
                .or_else(|| self.number_ref(r + 1, &mut found));
            end.filter(|&end| self.at(end) == delimiter)
                .and_then(|end| {
                    found.push(shut, end, end + 1);
                    close(end + 1, found)
                })
        } else if self.starts(r, b"DEFINE") {
            found.push(17, r, r + 6);
            close(r + 6, found)
        } else if self.starts(r, b"VERSION") {
            self.version(r, &mut found)
                .and_then(|end| close(end, found))
        } else {
            None
        };
        let (end, found) = outcome?;
        *spans = found;
        Some(end)
    }

    /// `VERSION>?=\d+\.\d+` at `r`.
    fn version(&self, r: usize, spans: &mut Spans) -> Option<usize> {
        let word = r + 7;
        spans.push(18, r, word);
        let equals = word + usize::from(self.at(word) == b'>');
        if self.at(equals) != b'=' {
            return None;
        }
        spans.push(19, word, equals + 1);
        let major = self.digit_end(equals + 1);
        if major == equals + 1 || self.at(major) != b'.' {
            return None;
        }
        let minor = self.digit_end(major + 1);
        if minor == major + 1 {
            return None;
        }
        spans.push(20, equals + 1, minor);
        Some(minor)
    }

    /// What follows `(` in a group: `(\?)(?:…)` or `\*(name):`, from `q`.
    fn group_head(&self, q: usize, spans: &mut Spans) -> Option<usize> {
        if self.at(q) == b'*' {
            let name_end = GROUP_NAMES
                .iter()
                .find_map(|name| self.lit(q + 1, name).filter(|&end| self.at(end) == b':'))?;
            spans.push(2, q, name_end + 1);
            return Some(name_end + 1);
        }
        if self.at(q) != b'?' {
            return None;
        }
        let r = q + 1;
        let mut found = *spans;
        found.push(3, q, r);
        let end = self.group_alternatives(r, &mut found)?;
        found.push(2, q, end);
        *spans = found;
        Some(end)
    }

    /// The alternatives after `(?` in a group, from `r`.
    fn group_alternatives(&self, r: usize, spans: &mut Spans) -> Option<usize> {
        if b"!*:=>|".contains(&self.at(r)) && r < self.lim() {
            spans.push(4, r, r + 1);
            return Some(r + 1);
        }
        if self.at(r) == b'<' && matches!(self.at(r + 1), b'!' | b'*' | b'=') {
            spans.push(4, r, r + 2);
            return Some(r + 2);
        }
        let (open, groups, delimiter) = if self.at(r) == b'\'' {
            (r, [8, 9, 10], b'\'')
        } else {
            (r + usize::from(self.at(r) == b'P'), [5, 6, 7], b'>')
        };
        let opener = if delimiter == b'>' { b'<' } else { b'\'' };
        if self.at(open) == opener
            && let Some(end) = self.group_name(open + 1, groups, delimiter, spans)
        {
            return Some(end);
        }
        self.option_toggle(r, b':').map(|end| end + 1)
    }

    /// `(?:(name)(-))?(name)<delimiter>` from `i`.
    fn group_name(
        &self,
        i: usize,
        groups: [usize; 3],
        delimiter: u8,
        spans: &mut Spans,
    ) -> Option<usize> {
        let [first, dash, last] = groups;
        let name = self.name_end(i)?;
        if self.at(name) == b'-'
            && let Some(second) = self.name_end(name + 1)
            && self.at(second) == delimiter
        {
            spans.push(first, i, name);
            spans.push(dash, name, name + 1);
            spans.push(last, name + 1, second);
            return Some(second + 1);
        }
        if self.at(name) != delimiter {
            return None;
        }
        spans.push(last, i, name);
        Some(name + 1)
    }

    /// Whether the literal's closing `/` and its `#`s stand at `i`.
    fn closes(&self, i: usize, hashes: &[u8]) -> bool {
        self.at(i) == b'/' && (hashes.is_empty() || self.starts(i + 1, hashes))
    }

    /// The end of the contents of a literal from `start`: the position after
    /// the last item, where the closing delimiter must stand.
    /// It is `None` when an unbalanced `(` makes the match fail.
    ///
    /// The contents are items: an escape, a quote, a comment, a class, a code
    /// block, a group or a run of plain characters.
    /// The grammar takes them possessively.
    /// A position and the parenthesis depth therefore decide everything after
    /// them.
    ///
    /// A scan that passes through many items leaves its path in the memo.
    /// A later start of the same literal kind that lands on that path ends
    /// the same way.
    fn contents_end(&mut self, start: usize, key: usize, hashes: &[u8]) -> Option<usize> {
        if let Some(scan) = self.x.regex_literals.scan.get()
            && scan.key == key
            && scan.start == start
        {
            return scan.end;
        }
        let merge = self
            .x
            .regex_literals
            .scan
            .get()
            .is_some_and(|scan| scan.key == key);
        let (end, items) = self.scan_contents(start, hashes, Mode::Walk { merge });
        if items >= LONG_SCAN {
            self.x.regex_literals.path.borrow_mut().clear();
            self.scan_contents(start, hashes, Mode::Record);
            let scan = Scan { key, start, end };
            self.x.regex_literals.scan.set(Some(scan));
        }
        end
    }

    /// Walks the contents from `start`, and returns where they end and how
    /// many items they had.
    /// A recording walk notes each `(position, depth)` in the memo's path.
    /// A merging walk that reaches a state of the remembered path ends as
    /// that path did.
    fn scan_contents(&self, start: usize, hashes: &[u8], mode: Mode) -> (Option<usize>, usize) {
        let mut i = start;
        let mut depth = 0_usize;
        let mut items = 0;
        loop {
            if i >= self.lim() {
                return (None, items);
            }
            match mode {
                Mode::Walk { merge: true } => {
                    if let Some(scan) = self.merged(i, depth) {
                        return (scan.end, items);
                    }
                }
                Mode::Walk { merge: false } => {}
                Mode::Record => self.x.regex_literals.path.borrow_mut().push((i, depth)),
            }
            match self.item(i, hashes) {
                Step::Item(end) => i = end,
                Step::Open(end) => {
                    depth += 1;
                    i = end;
                }
                Step::Stop if depth == 0 => return (Some(i), items),
                Step::Stop if self.at(i) == b')' => {
                    depth -= 1;
                    i += 1;
                }
                Step::Stop => return (None, items),
            }
            self.spend(1);
            items += 1;
        }
    }

    /// The remembered scan, when it passed through the state `(i, depth)`:
    /// a walk from there ends as it did.
    fn merged(&self, i: usize, depth: usize) -> Option<Scan> {
        let scan = self.x.regex_literals.scan.get()?;
        let path = self.x.regex_literals.path.borrow();
        let at = path
            .binary_search_by_key(&i, |&(position, _)| position)
            .ok()?;
        path.get(at)
            .filter(|&&(_, seen)| seen == depth)
            .map(|_| scan)
    }

    /// The item that starts at `i` in a literal's contents.
    fn item(&self, i: usize, hashes: &[u8]) -> Step {
        match self.at(i) {
            b'\\' => self.escape_item(i, hashes),
            b'(' => {
                if self.starts(i, b"(?#") {
                    let close = self.paren_from(i + 3);
                    if close < self.lim() {
                        return Step::Item(close + 1);
                    }
                }
                if self.starts(i, b"(?{")
                    && let Some(end) = self.code_item(i)
                {
                    return Step::Item(end);
                }
                Step::Open(i + 1)
            }
            b'[' => self.class_end(i, 1).map_or(Step::Stop, Step::Item),
            b')' => Step::Stop,
            _ => {
                let mut j = i;
                while j < self.lim()
                    && !self.closes(j, hashes)
                    && !matches!(self.at(j), b'(' | b')' | b'[' | b'\\')
                {
                    j += 1;
                }
                self.spend(j - i);
                if j > i { Step::Item(j) } else { Step::Stop }
            }
        }
    }

    /// An item that starts with a backslash: a `\Q` quote or an escape.
    ///
    /// `\\.`: the escaped character is any but the line feed.
    fn escape_item(&self, i: usize, hashes: &[u8]) -> Step {
        if self.starts(i, b"\\Q") {
            let memo = &self.x.regex_literals.quote;
            let stop = self.find_from(memo, hashes.len(), i + 2, |j| {
                self.starts(j, b"\\E") || self.closes(j, hashes)
            });
            if self.starts(stop, b"\\E") {
                return Step::Item(stop + 2);
            }
            if self.closes(stop, hashes) {
                return Step::Item(stop);
            }
        }
        if i + 1 < self.lim() {
            Step::Item(self.char_end(i + 1))
        } else {
            Step::Stop
        }
    }

    /// The end of the code block `(?{…}[tag]X)` at `i` in a literal.
    /// The body is `{` and as many `{` as the longest form below, up to the
    /// matching run of `}`.
    ///
    /// `\{(?:[^{].*?|\{[^{].*?}|…|\{\{\{\{\{.+?}}}}})}`: with `extra` more
    /// `{` (up to five) the body ends at the first run of `extra + 1` `}`
    /// after a character that is not `{` (any character for five).
    fn code_item(&self, i: usize) -> Option<usize> {
        let open = i + 2;
        let extra = (0..5).take_while(|k| self.at(open + 1 + k) == b'{').count();
        let body = open + extra + 1;
        if extra == 5 {
            if body >= self.lim() {
                return None;
            }
        } else if body > self.lim() {
            return None;
        }
        let close = self.braces_from(body + 1, extra + 1);
        if close >= self.lim() {
            return None;
        }
        let (end, _) = self.tag_end(close + extra + 1);
        let end = end + usize::from(matches!(self.at(end), b'<' | b'>' | b'X'));
        (self.at(end) == b')').then_some(end + 1)
    }

    /// The end of the character class `\[(?:\\.|[^]\[\\]|\[…])+]` at `i`,
    /// which nests four deep.
    fn class_end(&self, i: usize, level: u8) -> Option<usize> {
        let mut j = i + 1;
        let mut members = 0_usize;
        loop {
            match self.at(j) {
                b'\\' if j + 1 < self.lim() => j = self.char_end(j + 1),
                b'[' if level < 4 => j = self.class_end(j, level + 1)?,
                b'\\' | b'[' | b']' => break,
                _ if j > self.lim() => break,
                _ if j == self.lim() => j += 1,
                _ => j = self.char_end(j),
            }
            members += 1;
        }
        self.spend(members + 1);
        (members > 0 && self.at(j) == b']').then_some(j + 1)
    }
}
