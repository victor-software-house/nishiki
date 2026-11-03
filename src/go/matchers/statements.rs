//! The statements: `switch` headers and their cases, `const` and `var`
//! declarations, labels and slice indexes.

use super::super::engine::Lexer;
use super::params::Commas;
use super::types::{Lead, Prefix};
use crate::rules::Chars;

/// Whether `b` is one of `[-\]%&(-+./<>\[|]`, the punctuation of a switch
/// header's operands.
fn operand_punctuation(b: u8) -> bool {
    matches!(
        b,
        b'-' | b']'
            | b'%'
            | b'&'
            | b'('
            | b')'
            | b'*'
            | b'+'
            | b'.'
            | b'/'
            | b'<'
            | b'>'
            | b'['
            | b'|'
    )
}

/// Whether `b` is one of `[-!%&+,/:<=>|]`, the operators of a switch
/// header's expression.
fn operator(b: u8) -> bool {
    matches!(
        b,
        b'-' | b'!' | b'%' | b'&' | b'+' | b',' | b'/' | b':' | b'<' | b'=' | b'>' | b'|'
    )
}

impl Lexer<'_> {
    /// The end of `\s*/[*/]\s*.*` at `p`: a comment after a statement, which
    /// reaches the end of the line, or the line feed when only blanks follow
    /// its opening.
    fn spaced_comment(&self, p: usize) -> Option<usize> {
        let slash = self.spaces(p);
        self.trailing_comment(slash)?;
        let text = self.spaces(slash + 2);
        Some(text.max(self.lim()))
    }

    /// The end of `[-\]%&(-+./<>\[|\w]+{0,1}` at `p`.
    fn operand(&self, p: usize) -> usize {
        self.words_and(p, operand_punctuation)
    }

    /// ```text
    /// (?<=\bswitch\b)\s*(\w+\s*:=)?\s*([-\]%&(-+./<>\[|\w]+)(\.\(\btype\b\)\s*)(\{)
    /// ```
    ///
    /// The operand's run takes `.(type)`, and gives back exactly those seven
    /// characters when blanks and `{` follow them.
    pub(in crate::go) fn switch_types(&mut self, p: usize) -> Option<usize> {
        if !self.word_ends_at(p, b"switch") {
            return None;
        }
        let start = self.spaces(p);
        let binding = self.binding(start);
        let (value, (assertion, brace)) = binding
            .map(|end| self.spaces(end))
            .and_then(|value| self.type_switch_value(value).map(|found| (value, found)))
            .or_else(|| self.type_switch_value(start).map(|found| (start, found)))?;
        if let Some(end) = binding.filter(|_| value != start) {
            self.cap(1, start, end);
        }
        self.cap(2, value, assertion);
        self.cap(3, assertion, brace);
        self.cap(4, brace, brace + 1);
        Some(brace + 1)
    }

    /// The end of `\w+\s*:=` at `p`.
    fn binding(&self, p: usize) -> Option<usize> {
        let name = self.word_name(p);
        let assign = self.spaces(name);
        (name > p && self.starts(assign, b":=")).then_some(assign + 2)
    }

    /// Where `.(type)` starts after the operand at `p`, and where the `{`
    /// after it stands.
    fn type_switch_value(&self, p: usize) -> Option<(usize, usize)> {
        let run = self.operand(p);
        let assertion = run.checked_sub(7).filter(|&at| at > p)?;
        if self.text(assertion, run) != b".(type)" {
            return None;
        }
        let brace = self.spaces(run);
        self.opens_curly(brace).then_some((assertion, brace))
    }

    /// The case words before a case's expression: `^\s*\b(case)\b\s+`,
    /// where the blanks end, and where `case` stands and ends.
    fn case_start(&self, p: usize) -> Option<(usize, usize, usize)> {
        if p != 0 {
            return None;
        }
        let word = self.spaces(p);
        let end = self.keyword(word, b"case")?;
        let blank = self.spaces(end);
        (blank > end).then_some((word, end, blank))
    }

    /// `^\s*\b(case)\b\s+([!*,.<=>\w\s]+)(:)(\s*/[*/]\s*.*)?$`
    ///
    /// The second group's run cannot take `:`, so it is taken whole; when it
    /// is empty, the matcher gives a blank back to it.
    pub(in crate::go) fn switch_types_at0(&mut self, p: usize) -> Option<usize> {
        let (word, end, blank) = self.case_start(p)?;
        let mut run = blank;
        while run < self.lim() {
            let b = self.at(run);
            let takes = if b < 0x80 {
                matches!(b, b'!' | b'*' | b',' | b'.' | b'<' | b'=' | b'>')
                    || b.is_ascii_alphanumeric()
                    || b == b'_'
                    || self.space_at(run) > 0
            } else {
                self.word_at(run).is_some() || self.space_at(run) > 0
            };
            if !takes {
                break;
            }
            run = self.char_end(run);
        }
        self.spend(run - blank + 1);
        let start = if run > blank {
            blank
        } else {
            let last = self.char_start_before(blank);
            if last <= end {
                return None;
            }
            last
        };
        if run >= self.lim() || self.at(run) != b':' {
            return None;
        }
        let after = run + 1;
        let comment = self.spaced_comment(after);
        let finish = match comment {
            Some(finish) => finish,
            None if self.at_eol(after) => after,
            None => return None,
        };
        self.cap(1, word, end);
        self.cap(2, start, run);
        self.cap(3, run, after);
        if comment.is_some() {
            self.cap(4, after, finish);
        }
        Some(finish)
    }

    /// `^\s*\b(case)\b\s+([\s\S]+:\s*(?:/[*/].*)?)$`
    ///
    /// The second group first takes the whole line and gives it back to the
    /// last `:` that the rest matches after.
    pub(in crate::go) fn switch_variables_at0(&mut self, p: usize) -> Option<usize> {
        let (word, end, blank) = self.case_start(p)?;
        let after_colon = |lx: &Self, colon: usize| {
            let rest = lx.spaces(colon + 1);
            lx.trailing_comment(rest)
                .or_else(|| lx.at_eol(rest).then_some(rest))
        };
        let mut limit = self.lim();
        while let Some(colon) = self
            .text(blank + 1, limit)
            .iter()
            .rposition(|&b| b == b':')
            .map(|at| blank + 1 + at)
        {
            self.spend(limit - colon);
            if let Some(finish) = after_colon(self, colon) {
                self.cap(1, word, end);
                self.cap(2, blank, finish);
                return Some(finish);
            }
            limit = colon;
        }
        let last = self.char_start_before(blank);
        if last <= end || blank >= self.lim() || self.at(blank) != b':' {
            return None;
        }
        let finish = after_colon(self, blank)?;
        self.cap(1, word, end);
        self.cap(2, last, finish);
        Some(finish)
    }

    /// `(?<=\bswitch\b)\s*((?:E)?\s*O*\s*(?:;\s*O+\s*)?)(\{)`, with O an
    /// operand character and E `[.\w]+(?:\s*P+\s*[.\w]+)*\s*P+` for P an
    /// operator character.
    ///
    /// No part of the first group takes `{`, so it ends at the first one,
    /// and the expression matches when the text before it parses.
    pub(in crate::go) fn switch_variables_at1(&mut self, p: usize) -> Option<usize> {
        if !self.word_ends_at(p, b"switch") {
            return None;
        }
        let start = self.spaces(p);
        let brace = self.find_byte(start, b'{')?;
        if !self.switch_header(start, brace) {
            return None;
        }
        self.cap(1, start, brace);
        self.cap(2, brace, brace + 1);
        Some(brace + 1)
    }

    /// Whether `start..brace` parses as the first group of
    /// [`Lexer::switch_variables_at1`].
    ///
    /// The names of E are whole runs, since an operator or a blank follows
    /// each; so is each operator run but the last, which a name follows; the
    /// last may stop anywhere, leaving the operand characters among the
    /// operators to the operand.
    fn switch_header(&self, start: usize, brace: usize) -> bool {
        let operands = |at: usize| {
            let blank = self.spaces(self.operand(self.spaces(at)));
            if blank == brace {
                return true;
            }
            if blank >= brace || self.at(blank) != b';' {
                return false;
            }
            let operand = self.spaces(blank + 1);
            let end = self.operand(operand);
            end > operand && self.spaces(end) == brace
        };
        if operands(start) {
            return true;
        }
        let mut name_end = self.dot_words(start);
        if name_end == start {
            return false;
        }
        loop {
            let operators = self.spaces(name_end);
            let operators_end = self.skip(operators, operator);
            if operators_end == operators {
                return false;
            }
            if (operators + 1..=operators_end).any(&operands) {
                return true;
            }
            let name = self.spaces(operators_end);
            name_end = self.dot_words(name);
            if name_end == name {
                return false;
            }
        }
    }

    /// `(?<=\bconst\b)\s*\b(N)\s*(T)?`, the names and type of a constant;
    /// see [`Lexer::assignment`].
    pub(in crate::go) fn const_assignment_at0(&mut self, p: usize) -> Option<usize> {
        if !self.word_ends_at(p, b"const") {
            return None;
        }
        self.assignment(p)
    }

    /// `(?<=\bvar\b)\s*\b(N)\s*(T)?`, the names and type of a variable.
    pub(in crate::go) fn var_assignment_at0(&mut self, p: usize) -> Option<usize> {
        if !self.word_ends_at(p, b"var") {
            return None;
        }
        self.assignment(p)
    }

    /// `^\s*\b(N)\s*(T)?`, a line of a `const` or `var` group.
    pub(in crate::go) fn const_assignment_at1_at0(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        self.assignment(p)
    }

    /// ```text
    /// \s*\b([.\w]+(?:,\s*[.\w]+)*)\s*((?:(?:CH)+(?:\([^)]+\))?)?(?!B\b(?:struct|func|map)\b)(?:[]*.\[\w]+(?:,\s*[]*.\[\w]+)*)?\s*=?)?
    /// ```
    ///
    /// The second group ends the expression and matches nothing at worst,
    /// so it is the first of its ways that passes the lookahead, or none.
    fn assignment(&mut self, p: usize) -> Option<usize> {
        let start = self.spaces(p);
        if !self.boundary(start) {
            return None;
        }
        let names = self.names(start, Self::dot_words, Commas::Tight)?;
        let blank = self.spaces(names);
        let value = self.after_chans(blank, Lead::Tight, |lx, q, prefix| match prefix {
            Prefix::Taken => lx
                .filled_parens(q)
                .and_then(|after| lx.assigned_type(after))
                .or_else(|| lx.assigned_type(q)),
            Prefix::Empty => lx.assigned_type(q),
        });
        self.cap(1, start, names);
        Some(value.map_or(blank, |end| {
            self.cap(2, blank, end);
            end
        }))
    }

    /// `(?!B\b(?:struct|func|map)\b)(?:[]*.\[\w]+(?:,\s*[]*.\[\w]+)*)?\s*=?`
    /// at `p`.
    fn assigned_type(&self, p: usize) -> Option<usize> {
        if !self.not_marked_word(p, &[b"func", b"map", b"struct"]) {
            return None;
        }
        let mut end = self.type_run(p);
        if end > p {
            while end < self.lim() && self.at(end) == b',' {
                let item = self.spaces(end + 1);
                let item_end = self.type_run(item);
                if item_end == item {
                    break;
                }
                end = item_end;
            }
        }
        let blank = self.spaces(end);
        Some(blank + usize::from(blank < self.lim() && self.at(blank) == b'='))
    }

    /// `(?<=\bconst\b)\s*(\()`
    pub(in crate::go) fn const_assignment_at1(&mut self, p: usize) -> Option<usize> {
        if !self.word_ends_at(p, b"const") {
            return None;
        }
        self.group_open(p)
    }

    /// `(?<=\bvar\b)\s*(\()`
    pub(in crate::go) fn var_assignment_at1(&mut self, p: usize) -> Option<usize> {
        if !self.word_ends_at(p, b"var") {
            return None;
        }
        self.group_open(p)
    }

    /// `\s*(\()` at `p`.
    fn group_open(&mut self, p: usize) -> Option<usize> {
        let open = self.spaces(p);
        (open < self.lim() && self.at(open) == b'(').then(|| {
            self.cap(1, open, open + 1);
            open + 1
        })
    }

    /// `^(\s*\w+:\s*|\s*\b(?:break|goto|continue)\b\s+\w+(?:\s*/[*/]\s*.*)?)$`
    pub(in crate::go) fn label_loop_variables(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let start = self.spaces(p);
        let label = self.word_name(start);
        if label > start && label < self.lim() && self.at(label) == b':' {
            let end = self.spaces(label + 1);
            if self.at_eol(end) {
                self.cap(1, p, end);
                return Some(end);
            }
        }
        let word = self.word_in(start, &[b"break", b"continue", b"goto"])?;
        let name = self.spaces(word);
        let name_end = self.word_name(name);
        if name == word || name_end == name {
            return None;
        }
        let end = match self.spaced_comment(name_end) {
            Some(end) => end,
            None if self.at_eol(name_end) => name_end,
            None => return None,
        };
        self.cap(1, p, end);
        Some(end)
    }

    /// `(?<=\w\[)((?:\bK+:|:\bK+)(?:\bK+)?(?::\bK+)?)(?=])`, with K
    /// `[-%&*+./<>|\w]`.
    ///
    /// Each run of K ends at a character K lacks, which only `:` or `]` may
    /// be, so every run is taken whole.
    pub(in crate::go) fn slice_index_variables(&mut self, p: usize) -> Option<usize> {
        if p < 2 || p > self.lim() || self.at(p - 1) != b'[' || !self.word_before(p - 1) {
            return None;
        }
        let index = |lx: &Self, at: usize| {
            lx.words_and(at, |b| {
                matches!(
                    b,
                    b'-' | b'%' | b'&' | b'*' | b'+' | b'.' | b'/' | b'<' | b'>' | b'|'
                )
            })
        };
        let named = |lx: &Self, at: usize| at < lx.lim() && lx.word_at(at).is_some();
        let colon = |lx: &Self, at: usize| at < lx.lim() && lx.at(at) == b':';
        let mut end = if named(self, p) {
            let first = index(self, p);
            if !colon(self, first) {
                return None;
            }
            let second = first + 1;
            if named(self, second) {
                index(self, second)
            } else {
                second
            }
        } else if colon(self, p) && named(self, p + 1) {
            index(self, p + 1)
        } else {
            return None;
        };
        if colon(self, end) && named(self, end + 1) {
            end = index(self, end + 1);
        }
        if end >= self.lim() || self.at(end) != b']' {
            return None;
        }
        self.cap(1, p, end);
        Some(end)
    }
}
