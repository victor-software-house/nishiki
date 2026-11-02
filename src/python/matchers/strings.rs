//! The matchers of plain strings: their prefixes and quotes, escapes,
//! formatting placeholders and docstrings.

use crate::python::engine::{Ahead, Lexer};
use crate::python::grammar::Pat;

/// How a string closes.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Close {
    /// The quote text only: a triple-quoted string.
    Quote,
    /// The quote text, or a line feed that no backslash precedes.
    QuoteOrNewline,
}

impl Lexer<'_> {
    #[expect(
        clippy::wildcard_enum_match_arm,
        reason = "the dispatch sends this function its own patterns only"
    )]
    pub(super) fn string_matcher(&mut self, pat: Pat, p: usize) -> Option<usize> {
        match pat {
            Pat::StringQuotedMultiLine => self.plain_begin(p, true),
            Pat::StringQuotedSingleLine => self.plain_begin(p, false),
            Pat::StringBinQuotedMultiLine => self.bin_begin(p, true),
            Pat::StringBinQuotedSingleLine => self.bin_begin(p, false),
            Pat::StringRawQuotedMultiLine => self.raw_begin(p, true),
            Pat::StringRawQuotedSingleLine => self.raw_begin(p, false),
            Pat::StringRawBinQuotedMultiLine => self.raw_bin_begin(p, true),
            Pat::StringRawBinQuotedSingleLine => self.raw_bin_begin(p, false),
            Pat::StringQuotedMultiLineEnd
            | Pat::StringBinQuotedMultiLineEnd
            | Pat::StringRawQuotedMultiLineEnd
            | Pat::StringRawBinQuotedMultiLineEnd
            | Pat::DocstringAt0End
            | Pat::DocstringAt1End => self.string_end(p, Close::Quote),
            Pat::StringQuotedSingleLineEnd
            | Pat::StringBinQuotedSingleLineEnd
            | Pat::StringRawQuotedSingleLineEnd
            | Pat::StringRawBinQuotedSingleLineEnd
            | Pat::DocstringAt2End
            | Pat::DocstringAt3End => self.string_end(p, Close::QuoteOrNewline),
            Pat::DocstringAt0 => self.docstring_triple(p),
            Pat::DocstringAt1 => self.docstring_raw(p, true),
            Pat::DocstringAt2 => self.docstring_single(p),
            Pat::DocstringAt3 => self.docstring_raw(p, false),
            Pat::DocstringStatement => self.docstring_statement_begin(p),
            Pat::DocstringStatementEnd => self.docstring_statement_end(p),
            Pat::DocstringPrompt => self.docstring_prompt(p),
            Pat::EscapeSequenceUnicodeAt0 => self.unicode_escape(p),
            Pat::EscapeSequence => self.escape(p),
            Pat::StringLineContinuation => {
                (self.at(p) == b'\\' && p < self.lim() && self.at_eol(p + 1)).then_some(p + 1)
            }
            Pat::StringConsumeEscape => self.consume_escape(p),
            Pat::StringFormatting => self.percent_format(p),
            Pat::StringBraceFormattingAt0 => self.brace_format(p),
            Pat::StringBraceFormattingAt1 => self.brace_format_nested(p),
            Pat::StringMultiBadBrace1FormattingUnicode | Pat::StringMultiBadBrace1FormattingRaw => {
                self.bad_brace1(p, true)
            }
            Pat::StringSingleBadBrace1FormattingUnicode
            | Pat::StringSingleBadBrace1FormattingRaw => self.bad_brace1(p, false),
            Pat::StringMultiBadBrace2FormattingUnicode | Pat::StringMultiBadBrace2FormattingRaw => {
                self.bad_brace2(p, true)
            }
            Pat::StringSingleBadBrace2FormattingUnicode
            | Pat::StringSingleBadBrace2FormattingRaw => self.bad_brace2(p, false),
            Pat::StringMultiBadBrace1FormattingUnicodeEnd
            | Pat::StringMultiBadBrace1FormattingRawEnd
            | Pat::StringMultiBadBrace2FormattingUnicodeEnd
            | Pat::StringMultiBadBrace2FormattingRawEnd => self.triple_ahead(p).then_some(p),
            Pat::StringSingleBadBrace1FormattingUnicodeEnd
            | Pat::StringSingleBadBrace1FormattingRawEnd
            | Pat::StringSingleBadBrace2FormattingUnicodeEnd
            | Pat::StringSingleBadBrace2FormattingRawEnd => {
                self.quote_or_newline_ahead(p).then_some(p)
            }
            _ => None,
        }
    }

    /// Whether `'''` or `"""` starts at `p`.
    fn triple_ahead(&self, p: usize) -> bool {
        self.starts(p, b"'''") || self.starts(p, b"\"\"\"")
    }

    /// Whether a quote starts at `p`, or a line feed that no backslash
    /// precedes: `(?=(["'])|((?<!\\)\n))`.
    fn quote_or_newline_ahead(&self, p: usize) -> bool {
        (matches!(self.at(p), b'"' | b'\'') && p < self.lim())
            || (p == self.lim() && !self.prev_is(p, b'\\'))
    }

    /// The width of the quote at `p`: three for a triple quote, one for a
    /// single one, when `triple` says which kind the rule takes.
    fn quote_width(&self, p: usize, triple: bool) -> Option<usize> {
        if triple {
            self.triple_ahead(p).then_some(3)
        } else {
            (matches!(self.at(p), b'"' | b'\'') && p < self.lim()).then_some(1)
        }
    }

    /// The begin `(?:\b([Rr])(?=[Uu]))?([Uu])?('''|""")`, or the same with
    /// `(["'])` for a single-line string.
    fn plain_begin(&mut self, p: usize, triple: bool) -> Option<usize> {
        let mut q = p;
        let mut groups = [None; 2];
        if self.boundary(p)
            && matches!(self.at(p), b'R' | b'r')
            && matches!(self.at(p + 1), b'U' | b'u')
            && p + 1 < self.lim()
        {
            groups[0] = Some((p, p + 1));
            q = p + 1;
        }
        if matches!(self.at(q), b'U' | b'u')
            && q < self.lim()
            && self.quote_width(q + 1, triple).is_some()
        {
            groups[1] = Some((q, q + 1));
            q += 1;
        }
        let width = self.quote_width(q, triple)?;
        if let Some((a, b)) = groups[0] {
            self.cap(1, a, b);
        }
        if let Some((a, b)) = groups[1] {
            self.cap(2, a, b);
        }
        self.cap(3, q, q + width);
        if !triple {
            self.cap(4, q, q + width);
        }
        Some(q + width)
    }

    /// The begin `\b([Bb])('''|""")` or `\b([Bb])((["']))`.
    fn bin_begin(&mut self, p: usize, triple: bool) -> Option<usize> {
        if !self.boundary(p) || !matches!(self.at(p), b'B' | b'b') || p >= self.lim() {
            return None;
        }
        let width = self.quote_width(p + 1, triple)?;
        self.cap(1, p, p + 1);
        self.cap(2, p + 1, p + 1 + width);
        if !triple {
            self.cap(3, p + 1, p + 1 + width);
        }
        Some(p + 1 + width)
    }

    /// The begin `\b(([Uu]R)|(R))('''|""")` or its single-line form.
    fn raw_begin(&mut self, p: usize, triple: bool) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let (prefix_end, group) = if matches!(self.at(p), b'U' | b'u') && self.at(p + 1) == b'R' {
            (p + 2, 2)
        } else if self.at(p) == b'R' {
            (p + 1, 3)
        } else {
            return None;
        };
        let width = self.quote_width(prefix_end, triple)?;
        self.cap(1, p, prefix_end);
        self.cap(group, p, prefix_end);
        self.cap(4, prefix_end, prefix_end + width);
        if !triple {
            self.cap(5, prefix_end, prefix_end + width);
        }
        Some(prefix_end + width)
    }

    /// The begin `\b(R[Bb]|[Bb]R)('''|""")` or its single-line form.
    fn raw_bin_begin(&mut self, p: usize, triple: bool) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let ok = (self.at(p) == b'R' && matches!(self.at(p + 1), b'B' | b'b'))
            || (matches!(self.at(p), b'B' | b'b') && self.at(p + 1) == b'R');
        if !ok || p + 1 >= self.lim() {
            return None;
        }
        let width = self.quote_width(p + 2, triple)?;
        self.cap(1, p, p + 2);
        self.cap(2, p + 2, p + 2 + width);
        if !triple {
            self.cap(3, p + 2, p + 2 + width);
        }
        Some(p + 2 + width)
    }

    /// The end of a string: its quote text, which the begin kept, or for a
    /// single-line string a line feed that no backslash precedes.
    fn string_end(&mut self, p: usize, close: Close) -> Option<usize> {
        if let Some(width) = self.payload_at(p) {
            self.cap(1, p, p + width);
            return Some(p + width);
        }
        if close == Close::QuoteOrNewline && p == self.lim() && !self.prev_is(p, b'\\') {
            self.cap(2, p, p + 1);
            return Some(p + 1);
        }
        None
    }

    /// The begin `('''|""")` of a docstring.
    fn docstring_triple(&mut self, p: usize) -> Option<usize> {
        let width = self.quote_width(p, true)?;
        self.cap(1, p, p + width);
        Some(p + width)
    }

    /// The begin `([Rr])('''|""")` or `([Rr])(["'])` of a raw docstring.
    fn docstring_raw(&mut self, p: usize, triple: bool) -> Option<usize> {
        if !matches!(self.at(p), b'R' | b'r') || p >= self.lim() {
            return None;
        }
        let width = self.quote_width(p + 1, triple)?;
        self.cap(1, p, p + 1);
        self.cap(2, p + 1, p + 1 + width);
        Some(p + 1 + width)
    }

    /// The begin `(["'])` of a single-line docstring.
    fn docstring_single(&mut self, p: usize) -> Option<usize> {
        let width = self.quote_width(p, false)?;
        self.cap(1, p, p + width);
        Some(p + width)
    }

    /// The begin `^(?=\s*[Rr]?('''|"""|["']))` of a docstring statement: the
    /// quote the line starts with is group 1.
    fn docstring_statement_begin(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let (start, width) = self.string_start(self.space_end(0))?;
        self.cap(1, start, start + width);
        Some(p)
    }

    /// `[Rr]?('''|"""|["'])` at `q`: where the quote starts and how wide it is.
    fn string_start(&self, q: usize) -> Option<(usize, usize)> {
        let start = if matches!(self.at(q), b'R' | b'r') && q < self.lim() {
            q + 1
        } else {
            q
        };
        if let Some(width) = self
            .quote_width(start, true)
            .or_else(|| self.quote_width(start, false))
        {
            return Some((start, width));
        }
        // `[Rr]?` is optional: the quote may start at `q` itself.
        let width = self
            .quote_width(q, true)
            .or_else(|| self.quote_width(q, false))?;
        Some((q, width))
    }

    /// The end `((?<=\1)|^)(?!\s*[Rr]?('''|"""|["']))`.
    fn docstring_statement_end(&self, p: usize) -> Option<usize> {
        if !(self.payload_before(p) || p == 0) {
            return None;
        }
        self.string_start(self.space_end(p)).is_none().then_some(p)
    }

    /// The match `(?:^|\G)\s*((?:>>>|\.\.\.)\s)(?=\s*\S)`.
    fn docstring_prompt(&mut self, p: usize) -> Option<usize> {
        if !(p == 0 || (self.g && p == self.pos)) {
            return None;
        }
        let q = self.space_end(p);
        if !(self.starts(q, b">>>") || self.starts(q, b"...")) {
            return None;
        }
        let w = self.space_at(q + 3);
        if w == 0 {
            return None;
        }
        let r = q + 3 + w;
        // `(?=\s*\S)`: a character that is not white space comes later.
        if self.space_end(r) >= self.lim() {
            return None;
        }
        self.cap(1, q, r);
        Some(r)
    }

    /// The match `\\(u\h{4}|U\h{8}|N\{[\w\s]+?})`.
    fn unicode_escape(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'\\' || p >= self.lim() {
            return None;
        }
        let hex = |from: usize, n: usize| {
            (from..from + n).all(|i| self.at(i).is_ascii_hexdigit() && i < self.lim())
        };
        match self.at(p + 1) {
            b'u' if hex(p + 2, 4) => Some(p + 6),
            b'U' if hex(p + 2, 8) => Some(p + 10),
            b'N' if self.at(p + 2) == b'{' && p + 2 < self.lim() => {
                // `[\w\s]+?}`: one or more word or space characters, then `}`.
                let mut i = p + 3;
                loop {
                    if self.at(i) == b'}' && i < self.lim() {
                        return (i > p + 3).then_some(i + 1);
                    }
                    let width = self
                        .word_at(i)
                        .or_else(|| Some(self.space_at(i)).filter(|&w| w > 0))?;
                    i += width;
                }
            }
            _ => None,
        }
    }

    /// The match `\\(x\h{2}|[0-7]{1,3}|["'\\abfnrtv])`.
    fn escape(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'\\' || p >= self.lim() {
            return None;
        }
        let c = self.at(p + 1);
        if p + 1 >= self.lim() {
            return None;
        }
        if c == b'x'
            && self.at(p + 2).is_ascii_hexdigit()
            && self.at(p + 3).is_ascii_hexdigit()
            && p + 3 < self.lim()
        {
            return Some(p + 4);
        }
        if matches!(c, b'0'..=b'7') {
            let mut e = p + 2;
            while e < p + 4 && matches!(self.at(e), b'0'..=b'7') && e < self.lim() {
                e += 1;
            }
            return Some(e);
        }
        matches!(
            c,
            b'"' | b'\'' | b'\\' | b'a' | b'b' | b'f' | b'n' | b'r' | b't' | b'v'
        )
        .then_some(p + 2)
    }

    /// The match `\\[\n"'\\]`.
    fn consume_escape(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'\\' || p >= self.lim() {
            return None;
        }
        // The line feed counts, as the end of the line.
        let c = self.at(p + 1);
        matches!(c, b'\n' | b'"' | b'\'' | b'\\').then_some(p + 2)
    }

    /// The match `(%(\([\w\s]*\))?[-
    /// #+0]*(\d+|\*)?(\.(\d+|\*))?([Lhl])?[%EFGXa-giorsux])`.
    fn percent_format(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'%' || p >= self.lim() {
            return None;
        }
        let mut i = p + 1;
        // `(\([\w\s]*\))?`
        if self.at(i) == b'(' && i < self.lim() {
            let mut j = i + 1;
            loop {
                if self.at(j) == b')' && j < self.lim() {
                    i = j + 1;
                    break;
                }
                let Some(width) = self
                    .word_at(j)
                    .or_else(|| Some(self.space_at(j)).filter(|&w| w > 0))
                else {
                    break;
                };
                j += width;
            }
        }
        i = self.skip(i, |b| matches!(b, b'-' | b' ' | b'#' | b'+' | b'0'));
        // `(\d+|\*)?`
        if self.at(i) == b'*' && i < self.lim() {
            i += 1;
        } else {
            i = self.digit_end(i);
        }
        // `(\.(\d+|\*))?`
        if self.at(i) == b'.' && i < self.lim() {
            if self.at(i + 1) == b'*' && i + 1 < self.lim() {
                i += 2;
            } else {
                let e = self.digit_end(i + 1);
                if e > i + 1 {
                    i = e;
                }
            }
        }
        // `([Lhl])?` then the conversion: the length modifier gives itself
        // back when no conversion follows it.
        let conversion = |b: u8| b"%EFGXabcdefgiorsux".contains(&b);
        if matches!(self.at(i), b'L' | b'h' | b'l')
            && conversion(self.at(i + 1))
            && i + 1 < self.lim()
        {
            self.cap(1, p, i + 2);
            return Some(i + 2);
        }
        if conversion(self.at(i)) && i < self.lim() {
            self.cap(1, p, i + 1);
            return Some(i + 1);
        }
        None
    }

    /// The end of `\{\w*(\.[_[:alpha:]]\w*|\[[^]"']+])*(![ars])?` at `p`,
    /// which starts with the brace: where the field name, attributes and
    /// conversion end, and the conversion's span.
    fn field_head(&self, p: usize) -> (usize, Option<(usize, usize)>) {
        let mut i = self.word_end(p + 1);
        loop {
            if self.at(i) == b'.' && i < self.lim() {
                let Some(e) = self.ident_end(i + 1) else {
                    break;
                };
                i = e;
            } else if self.at(i) == b'[' && i < self.lim() {
                let mut j = i + 1;
                while j < self.lim() && !matches!(self.at(j), b']' | b'"' | b'\'') {
                    j = self.char_end(j);
                }
                if j > i + 1 && self.at(j) == b']' && j < self.lim() {
                    i = j + 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        if self.at(i) == b'!' && matches!(self.at(i + 1), b'a' | b'r' | b's') && i + 1 < self.lim()
        {
            return (i + 2, Some((i, i + 2)));
        }
        (i, None)
    }

    /// The match `(\{\{|}}|\{\w*(\.[_[:alpha:]]\w*|\[[^]"']+])*(![ars])?(:\w?
    /// [<=>^]?[- +]?#?\d*,?(\.\d+)?[%EFGXb-gnosx]?)?})`.
    fn brace_format(&mut self, p: usize) -> Option<usize> {
        if self.starts(p, b"{{") || self.starts(p, b"}}") {
            self.cap(1, p, p + 2);
            return Some(p + 2);
        }
        if self.at(p) != b'{' || p >= self.lim() {
            return None;
        }
        let (head_end, conversion) = self.field_head(p);
        // `(:spec)?` then `}`: with the spec first, then without it.
        if self.at(head_end) == b':'
            && head_end < self.lim()
            && let Some(close) = self.format_spec(head_end + 1)
        {
            self.cap(1, p, close + 1);
            if let Some((a, b)) = conversion {
                self.cap(3, a, b);
            }
            self.cap(4, head_end, close);
            return Some(close + 1);
        }
        if self.at(head_end) == b'}' && head_end < self.lim() {
            self.cap(1, p, head_end + 1);
            if let Some((a, b)) = conversion {
                self.cap(3, a, b);
            }
            return Some(head_end + 1);
        }
        None
    }

    /// The format spec `\w?[<=>^]?[- +]?#?\d*,?(\.\d+)?[%EFGXb-gnosx]?` that
    /// ends at a `}`: where that brace is. Each part is optional and takes
    /// what it can, giving it back when the rest does not fit.
    pub(super) fn format_spec(&self, start: usize) -> Option<usize> {
        self.spec_from(start, 0)
    }

    fn spec_from(&self, i: usize, stage: u8) -> Option<usize> {
        self.spend(1);
        if stage == 8 {
            return (self.at(i) == b'}' && i < self.lim()).then_some(i);
        }
        let b = self.at(i);
        let live = i < self.lim();
        // The part takes a character when it can.
        let taken = match stage {
            0 => self.word_at(i).map(|w| i + w),
            1 => (live && matches!(b, b'<' | b'=' | b'>' | b'^')).then_some(i + 1),
            2 => (live && matches!(b, b'-' | b' ' | b'+')).then_some(i + 1),
            3 => (live && b == b'#').then_some(i + 1),
            4 => {
                let e = self.digit_end(i);
                (e > i).then_some(e)
            }
            5 => (live && b == b',').then_some(i + 1),
            6 => {
                if live && b == b'.' {
                    let e = self.digit_end(i + 1);
                    (e > i + 1).then_some(e)
                } else {
                    None
                }
            }
            _ => (live && b"%EFGXbcdefgnosx".contains(&b)).then_some(i + 1),
        };
        if let Some(next) = taken {
            if let Some(close) = self.spec_from(next, stage + 1) {
                return Some(close);
            }
            // The digit run may give digits back one at a time.
            if stage == 4 {
                let mut e = next;
                while e > i + 1 {
                    e = self.char_before(e).map_or(i, |c| e - c.len_utf8());
                    if e > i
                        && let Some(close) = self.spec_from(e, stage + 1)
                    {
                        return Some(close);
                    }
                }
            }
        }
        self.spec_from(i, stage + 1)
    }

    /// The match `(\{\w*(\.[_[:alpha:]]\w*|\[[^]"']+])*(![ars])?(:)[^\n"'{}]*(?
    /// :\{[^\n"'}]*?}[^\n"'{}]*)*})`.
    fn brace_format_nested(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'{' || p >= self.lim() {
            return None;
        }
        let (head_end, conversion) = self.field_head(p);
        if self.at(head_end) != b':' || head_end >= self.lim() {
            return None;
        }
        let mut i = self.next_stop(head_end + 1, true);
        loop {
            match self.at(i) {
                b'}' if i < self.lim() => break,
                b'{' if i < self.lim() => {
                    // `\{[^\n"'}]*?}`: up to the first `}`, with no quote.
                    let j = self.next_stop(i + 1, false);
                    if !(self.at(j) == b'}' && j < self.lim()) {
                        return None;
                    }
                    i = self.next_stop(j + 1, true);
                }
                _ => return None,
            }
        }
        self.cap(1, p, i + 1);
        if let Some((a, b)) = conversion {
            self.cap(3, a, b);
        }
        self.cap(4, head_end, head_end + 1);
        Some(i + 1)
    }

    /// The begin `(?=\{%(.*?(?!...))%})`: a `{%` with a `%}` after it on
    /// the line. The negative lookahead sits before the `%}`, so it holds.
    fn bad_brace1(&self, p: usize, _triple: bool) -> Option<usize> {
        if !self.starts(p, b"{%") {
            return None;
        }
        (self.last(Ahead::PercentBrace)? >= p + 2).then_some(p)
    }

    /// The begin `(?!\{\{)(?=\{(\w*?(?!q)[^!.:\[}\w]).*?(?!q)})`: a brace group
    /// whose first character after the name is none of `!.:[}`, so it is not a
    /// placeholder.
    fn bad_brace2(&self, p: usize, triple: bool) -> Option<usize> {
        if self.at(p) != b'{' || p >= self.lim() || self.at(p + 1) == b'{' && p + 1 < self.lim() {
            return None;
        }
        let t = self.word_end(p + 1);
        // `(?!q)` before the class character.
        let blocked = |at: usize| {
            if triple {
                self.triple_ahead(at)
            } else {
                (matches!(self.at(at), b'"' | b'\'') && at < self.lim())
                    || (at == self.lim() && !self.prev_is(at, b'\\'))
            }
        };
        if blocked(t) || t > self.lim() {
            return None;
        }
        let c = self.at(t);
        if t < self.lim() && matches!(c, b'!' | b'.' | b':' | b'[' | b'}') {
            return None;
        }
        let after = if t < self.lim() {
            self.char_end(t)
        } else {
            t + 1
        };
        // `.*?(?!q)}`: the first brace after it, on this line.
        (self.last(Ahead::CloseBrace)? >= after).then_some(p)
    }
}
