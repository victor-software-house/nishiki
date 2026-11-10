//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups of its match with [`Lexer::capture`].
//! The engine names those the grammar names.
//! A filter's begin always records group 1, its indent, which its end repeats.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would.
//! Where a quantifier could give characters back, the doc comment shows why it
//! cannot matter.
//! `\w`, `\s` and `\d` take non-ASCII characters, as in Oniguruma, through
//! [`Chars`].
//!
//! The [`halts`] are the same tests over the line's bytes alone, for the
//! languages HAML embeds, which stop where they hold.

use super::engine::{Filter, Lexer};
use super::grammar::Pattern;
use crate::rules::Chars;

pub(super) mod halts;

impl Lexer<'_> {
    /// Whether `pattern` matches at `p`, and where its match ends.
    #[expect(clippy::too_many_lines, reason = "one arm per pattern of the grammar")]
    pub(super) fn matcher(&mut self, pattern: Pattern, p: usize) -> Option<usize> {
        match pattern {
            Pattern::InterpolatedText => self.indent_then(p, b"=="),
            Pattern::InterpolatedTextEnd | Pattern::TagAt0End => self.line_end_and_feed(p),
            Pattern::InterpolatedRubyAt0 => {
                let end = self.lit(p, b"#{}")?;
                self.capture(1, end - 1, end);
                Some(end)
            }
            Pattern::InterpolatedRubyAt1 => self.lit(p, b"#{"),
            Pattern::InterpolatedRubyAt1End | Pattern::NestCurlyAndSelfEnd => self.lit(p, b"}"),
            Pattern::NestCurlyAndSelf => self.lit(p, b"{"),
            Pattern::VariablesAt0 => self.variable(p, b"#@"),
            Pattern::VariablesAt1 => self.variable(p, b"#@@"),
            Pattern::VariablesAt2 => self.variable(p, b"#$"),
            Pattern::RubyFilter => self.indent_then(p, b":ruby"),
            Pattern::JavascriptFilter => self.indent_then(p, b":javascript"),
            Pattern::Script => self.indent_then(p, b"%script"),
            Pattern::RubyFilterAlone => self.filter_alone(p, &[b":ruby"], None),
            Pattern::RubyOrOpalFilter => self.filter_alone(p, &[b":ruby", b":opal"], None),
            Pattern::SassStyleFilter => self.filter_alone(p, &[b":style", b":sass"], None),
            Pattern::SassStylesFilter => self.filter_alone(p, &[b":styles", b":sass"], None),
            Pattern::SassFilterAlone => self.filter_alone(p, &[b":sass"], Some(2)),
            Pattern::CssFilter => self.css_filter(p),
            Pattern::CoffeeFilter => {
                let end = self.indent_then(p, b":coffee")?;
                Some(self.lit(end, b"script").unwrap_or(end))
            }
            Pattern::RubyFilterKeyword => self.named_filter(p, b":ruby"),
            Pattern::SassFilter => self.named_filter(p, b":sass"),
            Pattern::MarkdownFilter => self.named_filter(p, b":markdown"),
            Pattern::RubyFilterEnd
            | Pattern::JavascriptFilterEnd
            | Pattern::ScriptEnd
            | Pattern::RubyFilterAloneEnd
            | Pattern::RubyOrOpalFilterEnd
            | Pattern::CoffeeFilterEnd
            | Pattern::SassFilterEnd
            | Pattern::MarkdownFilterEnd
            | Pattern::CssFilterEnd
            | Pattern::SassFilterAloneEnd => {
                self.filter_end(p, Filter::Unindented, self.begin_text())
            }
            Pattern::SassStyleFilterEnd | Pattern::SassStylesFilterEnd => {
                self.filter_end(p, Filter::Indented, self.begin_text())
            }
            Pattern::RubyFilterKeywordEnd | Pattern::RubyFilterKeywordBodyEnd => None,
            Pattern::RubyFilterBody
            | Pattern::JavascriptFilterBody
            | Pattern::ScriptBody
            | Pattern::RubyFilterAloneBody
            | Pattern::RubyOrOpalFilterBody
            | Pattern::SassStyleFilterBody
            | Pattern::CoffeeFilterBody
            | Pattern::RubyFilterKeywordBody
            | Pattern::SassFilterBody
            | Pattern::SassStylesFilterBody
            | Pattern::MarkdownFilterBody
            | Pattern::CssFilterBody
            | Pattern::SassFilterAloneBody => self.at_g(p).then_some(p),
            Pattern::RubyFilterBodyEnd
            | Pattern::JavascriptFilterBodyEnd
            | Pattern::ScriptBodyEnd
            | Pattern::RubyFilterAloneBodyEnd
            | Pattern::RubyOrOpalFilterBodyEnd
            | Pattern::CoffeeFilterBodyEnd
            | Pattern::SassFilterBodyEnd
            | Pattern::MarkdownFilterBodyEnd
            | Pattern::CssFilterBodyEnd
            | Pattern::SassFilterAloneBodyEnd => {
                self.filter_end(p, Filter::Unindented, self.parent_begin_text())
            }
            Pattern::SassStyleFilterBodyEnd | Pattern::SassStylesFilterBodyEnd => {
                self.filter_end(p, Filter::Indented, self.parent_begin_text())
            }
            Pattern::Prolog => self.prolog(p),
            Pattern::ConditionalComment => self.conditional_comment(p),
            Pattern::Comment => self.comment(p),
            Pattern::CommentEnd => self.comment_end(p),
            Pattern::Tag => self.tag(p),
            Pattern::TagEnd => self.tag_end(p),
            Pattern::TagAt0 => self.lit(p, b"=="),
            Pattern::TagAt1 => self.selector(p, b'.', NameColon::Takes),
            Pattern::TagAt2 => self.selector(p, b'#', NameColon::Stops),
            Pattern::TagAt3 => halts::attributes_begin(self.s, p, self.nl).then_some(p + 1),
            Pattern::TagAt3End => halts::attributes_end_at(self.s, p),
            Pattern::Continuation => self.continuation(p),
            Pattern::Rubyline => self.rubyline(p),
            Pattern::RubylineEnd => self.rubyline_end(p),
            Pattern::RubylineAt0 => self.keyword_group(halts::php_keyword(self.s, p)),
            Pattern::RubylineAt1 => self.keyword_group(halts::php_include(self.s, p)),
            Pattern::RubylineAt2 => halts::php_exception(self.s, p),
            Pattern::RubylineAt3 => self.keyword_group(halts::php_call(self.s, p, b"function")),
            Pattern::RubylineAt4 => self.keyword_group(halts::php_call(self.s, p, b"use")),
            Pattern::RubylineAt5 => halts::block_opener(self.s, p),
            Pattern::RubylineAt6 => halts::comment(self.s, p),
            Pattern::TagAt4 => self.lit(p, b"("),
            Pattern::TagAt4End => self.lit(p, b")"),
            Pattern::TagAt4At0 => {
                let end = self.name_end(p, NameColon::Stops);
                (end > p).then_some(end)
            }
            Pattern::TagAt4At1 => self.lit(p, b"="),
            Pattern::TagAt4At3 | Pattern::TagAt4At3End => self.lit(p, b"\""),
            Pattern::TagAt4At3At0 => self.attribute_escape(p),
            Pattern::TagAt5 => halts::object_begin(self.s, p).then_some(p + 1),
            Pattern::TagAt5End => halts::object_end_at(self.s, p),
            Pattern::InterpolatedRubyLine => self.lit(p, b"!==").or_else(|| self.lit(p, b"==")),
            Pattern::InterpolatedRubyLineEnd | Pattern::RubyStartEnd => self.at_eol(p).then_some(p),
            Pattern::RubyEscapedChar => self.ruby_escape(p),
            Pattern::TagAt8 => self.lit(p, b"/"),
            Pattern::Escape => {
                let dot = self.line_indent(p)?;
                (self.at(dot) == b'.').then(|| {
                    self.capture(1, dot, dot + 1);
                    dot + 1
                })
            }
            Pattern::RubyStart => {
                let at = self.line_indent(p)?;
                let starts_ruby = matches!(self.at(at), b'-' | b'=' | b'~')
                    || self.lit(at, b"!=").is_some()
                    || self.lit(at, b"&=").is_some();
                starts_ruby.then_some(at)
            }
        }
    }

    /// `^\s*` at `p`: the end of the blanks that start the line, when `p` is
    /// the line's start.
    ///
    /// Each expression goes on with a character that is not `\s`, so giving
    /// back a blank leaves a blank where that character must stand.
    fn line_indent(&self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let mut at = 0;
        while at < self.lim() {
            let width = self.space_at(at);
            if width == 0 {
                break;
            }
            at += width;
        }
        Some(at)
    }

    /// `^(\s*)` and `text`, with group 1 around the blanks: the begin of a
    /// filter, or `^(\s*)==`.
    fn indent_then(&mut self, p: usize, text: &[u8]) -> Option<usize> {
        let indent = self.line_indent(p)?;
        let end = self.lit(indent, text)?;
        self.capture(1, 0, indent);
        Some(end)
    }

    /// `^(\s*)` and one of `names`, the first that matches, then `$`.
    /// With `name_group` `Some(2)`, `^(\s*)(:name)$`, with group 2 around the
    /// name.
    fn filter_alone(
        &mut self,
        p: usize,
        names: &[&[u8]],
        name_group: Option<usize>,
    ) -> Option<usize> {
        let indent = self.line_indent(p)?;
        let end = names
            .iter()
            .filter_map(|name| self.lit(indent, name))
            .find(|&end| self.at_eol(end))?;
        self.capture(1, 0, indent);
        if let Some(group) = name_group {
            self.capture(group, indent, end);
        }
        Some(end)
    }

    /// `^(\s*)(:name)`, with group 2 around the name.
    fn named_filter(&mut self, p: usize, name: &[u8]) -> Option<usize> {
        let end = self.indent_then(p, name)?;
        self.capture(2, end - name.len(), end);
        Some(end)
    }

    /// `^(\s*)(:(css|styles?))$`.
    ///
    /// `styles?` tries `styles` first, then `style`.
    fn css_filter(&mut self, p: usize) -> Option<usize> {
        let indent = self.line_indent(p)?;
        let colon = self.lit(indent, b":")?;
        let end = [&b"css"[..], b"styles", b"style"]
            .iter()
            .filter_map(|name| self.lit(colon, name))
            .find(|&end| self.at_eol(end))?;
        self.capture(1, 0, indent);
        self.capture(2, indent, end);
        self.capture(3, colon, end);
        Some(end)
    }

    /// A filter's end at `p`, for the indent `indent`.
    /// [`Filter::Unindented`] is `^(?!\1\s+|$\n*)`, and [`Filter::Indented`]
    /// is `^(?=\1\s+|$\n*)`; both hold only at the line's start and take no
    /// text.
    fn filter_end(&self, p: usize, filter: Filter, indent: &str) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let stays = self.stays_in_filter(indent.as_bytes());
        let ends = match filter {
            Filter::Unindented => !stays,
            Filter::Indented => stays,
            Filter::AfterLineFeed => false,
        };
        ends.then_some(0)
    }

    /// `\1\s+|$\n*` at the line's start.
    /// It holds when the line starts with the filter's indent and a `\s`,
    /// which may be the line feed.
    /// It also holds when the line is empty.
    fn stays_in_filter(&self, indent: &[u8]) -> bool {
        if self.at_eol(0) {
            return true;
        }
        let repeats = indent.is_empty() || self.lit(0, indent).is_some();
        repeats && self.space_at(indent.len()) > 0
    }

    /// `$\n*` and `$\n?`: the line's end, with its line feed.
    fn line_end_and_feed(&self, p: usize) -> Option<usize> {
        if !self.at_eol(p) {
            return None;
        }
        Some(p + usize::from(self.nl && p == self.lim()))
    }

    /// `(#@)[A-Z_a-z]\w*` and its kin, with `sigil` as group 1.
    fn variable(&mut self, p: usize, sigil: &[u8]) -> Option<usize> {
        let first = self.lit(p, sigil)?;
        let b = self.at(first);
        if !(b.is_ascii_alphabetic() || b == b'_') {
            return None;
        }
        self.capture(1, p, first);
        Some(self.word_end(first + 1))
    }

    /// `^(!!!)($|\s.*)`, with group 1 around the `!!!`.
    fn prolog(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let bangs = self.lit(0, b"!!!")?;
        let end = if self.at_eol(bangs) {
            bangs
        } else if self.space_at(bangs) > 0 {
            self.lim()
        } else {
            return None;
        };
        self.capture(1, 0, bangs);
        Some(end)
    }

    /// `^(\s*)(/\[[^]].*?$\n?)`, with group 1 around the blanks.
    ///
    /// `[^]]` takes any character but `]`, the line feed included.
    /// The lazy `.*?` then runs to the line's end, the first place where `$`
    /// holds.
    fn conditional_comment(&mut self, p: usize) -> Option<usize> {
        let indent = self.line_indent(p)?;
        let open = self.lit(indent, b"/[")?;
        let end = if open < self.lim() {
            if self.at(open) == b']' {
                return None;
            }
            self.lim() + usize::from(self.nl)
        } else if open == self.lim() && self.nl {
            open + 1
        } else {
            return None;
        };
        self.capture(1, 0, indent);
        Some(end)
    }

    /// `^(\s*)(-#|/|-\s*/\*+)`, with group 1 around the blanks and group 2
    /// around the mark.
    fn comment(&mut self, p: usize) -> Option<usize> {
        let indent = self.line_indent(p)?;
        let end = self
            .lit(indent, b"-#")
            .or_else(|| self.lit(indent, b"/"))
            .or_else(|| {
                let dash = self.lit(indent, b"-")?;
                let stars = self.lit(self.space_end(dash), b"/")?;
                let end = self.skip(stars, |b| b == b'*');
                (end > stars).then_some(end)
            })?;
        self.capture(1, 0, indent);
        self.capture(2, indent, end);
        Some(end)
    }

    /// `^(?!\1\s+|\n)`: a line that neither repeats the comment's indent with
    /// a `\s` after it nor is empty.
    fn comment_end(&self, p: usize) -> Option<usize> {
        if p != 0 || self.at(0) == b'\n' {
            return None;
        }
        let indent = self.begin_text().as_bytes();
        let repeats = indent.is_empty() || self.lit(0, indent).is_some();
        (!(repeats && self.space_at(indent.len()) > 0)).then_some(0)
    }

    /// `^\s*(?:((%)([-:\w]+))|(?=[#.]))`: a tag's name after `%`, with
    /// groups 1 to 3, or the blanks before a `#` or `.`.
    fn tag(&mut self, p: usize) -> Option<usize> {
        let at = self.line_indent(p)?;
        match self.at(at) {
            b'%' => {
                let end = self.name_end(at + 1, NameColon::Takes);
                if end == at + 1 {
                    return None;
                }
                self.capture(1, at, end);
                self.capture(2, at, at + 1);
                self.capture(3, at + 1, end);
                Some(end)
            }
            b'#' | b'.' => Some(at),
            _ => None,
        }
    }

    /// `$|(?![#(.\[{]|&amp;|[-=~]|!=|&=|/)`: the line's end, or any place
    /// where none of a tag's parts starts.
    fn tag_end(&self, p: usize) -> Option<usize> {
        if self.at_eol(p) {
            return Some(p);
        }
        let part = matches!(
            self.at(p),
            b'#' | b'(' | b'.' | b'[' | b'{' | b'-' | b'=' | b'~' | b'/'
        ) || self.lit(p, b"&amp;").is_some()
            || self.lit(p, b"!=").is_some()
            || self.lit(p, b"&=").is_some();
        (!part).then_some(p)
    }

    /// The end of the run of `\w` characters, and of `-` and, where the name
    /// takes it, `:`, from `i`.
    fn name_end(&self, mut i: usize, colon: NameColon) -> usize {
        loop {
            let b = self.at(i);
            if i < self.lim() && (b == b'-' || (b == b':' && colon == NameColon::Takes)) {
                i += 1;
            } else if let Some(width) = self.word_at(i) {
                i += width;
            } else {
                return i;
            }
        }
    }

    /// `(\.[-:\w]+)` and `(#[-\w]+)`: `mark` and a name, as group 1.
    fn selector(&mut self, p: usize, mark: u8, colon: NameColon) -> Option<usize> {
        if self.at(p) != mark || p >= self.lim() {
            return None;
        }
        let end = self.name_end(p + 1, colon);
        if end == p + 1 {
            return None;
        }
        self.capture(1, p, end);
        Some(end)
    }

    /// `(\|)\s*\n`: a `|` and blanks up to the line feed, with group 1 around
    /// the `|`.
    ///
    /// `\s*` could take the line feed, but then `\n` would have none left, so
    /// it stops before it.
    fn continuation(&mut self, p: usize) -> Option<usize> {
        let mut at = self.lit(p, b"|")?;
        while at < self.lim() {
            let width = self.space_at(at);
            if width == 0 {
                return None;
            }
            at += width;
        }
        if !self.nl {
            return None;
        }
        self.capture(1, p, p + 1);
        Some(at + 1)
    }

    /// `(&amp|!)?([-=~])`.
    ///
    /// The optional group tries `&amp` and `!` first.
    /// Without either, the mark must stand at `p`, where `&` and `!` are not
    /// marks.
    fn rubyline(&self, p: usize) -> Option<usize> {
        let mark = self
            .lit(p, b"&amp")
            .or_else(|| self.lit(p, b"!"))
            .filter(|&mark| matches!(self.at(mark), b'-' | b'=' | b'~'))
            .unwrap_or(p);
        (mark < self.lim() && matches!(self.at(mark), b'-' | b'=' | b'~')).then_some(mark + 1)
    }

    /// `((do|\{)( \|[*.]+\|)?)$|$|^(?!.*\|\s*)$\n?`, with group 1 around the
    /// block opener and group 2 around `do` or `{`.
    ///
    /// The third branch needs an empty line, where the second, `$`, matches
    /// first.
    fn rubyline_end(&mut self, p: usize) -> Option<usize> {
        if let Some((opener, end)) = halts::block_end(self.s, p) {
            self.capture(1, p, end);
            self.capture(2, p, opener);
            return Some(end);
        }
        self.at_eol(p).then_some(p)
    }

    /// The match of a pattern that names group 1, given as the group's start,
    /// its end and the match's end.
    fn keyword_group(&mut self, found: Option<(usize, usize, usize)>) -> Option<usize> {
        let (start, end, matched) = found?;
        self.capture(1, start, end);
        Some(matched)
    }

    /// `\\(x\h{2}|[012][0-7]{0,2}|3[0-6][0-7]?|37[0-7]?|[4-7][0-7]?|.)`: an
    /// escape in a quoted attribute value.
    fn attribute_escape(&self, p: usize) -> Option<usize> {
        let at = self.lit(p, b"\\")?;
        let octal = |i: usize| matches!(self.at(i), b'0'..=b'7') && i < self.lim();
        let end = match self.at(at) {
            b'x' if self.at(at + 1).is_ascii_hexdigit() && self.at(at + 2).is_ascii_hexdigit() => {
                at + 3
            }
            b'0'..=b'2' => {
                let one = at + 1 + usize::from(octal(at + 1));
                one + usize::from(one > at + 1 && octal(one))
            }
            b'3' if matches!(self.at(at + 1), b'0'..=b'6') => at + 2 + usize::from(octal(at + 2)),
            b'3' if self.at(at + 1) == b'7' => at + 2 + usize::from(octal(at + 2)),
            b'4'..=b'7' => at + 1 + usize::from(octal(at + 1)),
            _ if at < self.lim() => self.char_end(at),
            _ => return None,
        };
        Some(end)
    }

    /// Ruby's `\\(?:[0-7]{1,3}|x[A-Fa-f\d]{1,2}|.)`.
    fn ruby_escape(&self, p: usize) -> Option<usize> {
        let at = self.lit(p, b"\\")?;
        if at >= self.lim() {
            return None;
        }
        let octal = self.skip(at, |b| matches!(b, b'0'..=b'7')).min(at + 3);
        if octal > at {
            return Some(octal);
        }
        if self.at(at) == b'x' {
            let mut end = at + 1;
            for _ in 0..2 {
                if self.at(end).is_ascii_hexdigit() {
                    end += 1;
                } else if let Some(digit) = self.digit_at(end) {
                    end += digit;
                } else {
                    break;
                }
            }
            if end > at + 1 {
                return Some(end);
            }
        }
        Some(self.char_end(at))
    }

    /// The width of the `\d` character at `i`, when there is one.
    fn digit_at(&self, i: usize) -> Option<usize> {
        let end = self.digit_end(i);
        if end == i {
            return None;
        }
        Some(self.char_end(i) - i)
    }
}

/// Whether a name takes `:`, as a tag's and a class's do and an id's does not.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NameColon {
    /// `[-:\w]`.
    Takes,
    /// `[-\w]`.
    Stops,
}
