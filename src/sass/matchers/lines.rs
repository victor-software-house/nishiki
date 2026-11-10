//! Comments, at-rules and the rules that end with the line.

use super::{Lexer, is_pseudo};
use crate::rules::Chars;

/// The media types and keywords after which a blank does not end an at-rule.
const MEDIA_WORDS: &[&[u8]] = &[
    b"all",
    b"braille",
    b"embossed",
    b"handheld",
    b"print",
    b"projection",
    b"screen",
    b"speech",
    b"tty",
    b"tv",
    b"if",
    b"only",
    b"not",
];

/// Whether `b` is in `[\t ]`.
fn is_tab_or_space(b: u8) -> bool {
    matches!(b, b'\t' | b' ')
}

impl Lexer<'_> {
    /// `^(\s*)(/\*)`: a block comment, whose indent the end refers back to.
    /// The blanks never give one back, since `/` is no blank.
    /// The grammar names no group, but group 1 is the indent the end repeats.
    pub(super) fn block_comment(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let slash = self.space_end(p);
        let end = self.lit(slash, b"/*")?;
        self.capture(1, p, slash);
        Some(end)
    }

    /// `(\*/)|^(?!\s\1)`: the comment closes at `*/`, or at the start of a line
    /// that is not a blank and the comment's indent.
    /// An empty line holds its line feed, a blank, so it keeps an unindented
    /// comment open.
    pub(super) fn block_comment_end(&self, p: usize) -> Option<usize> {
        if let Some(end) = self.lit(p, b"*/") {
            return Some(end);
        }
        if p != 0 {
            return None;
        }
        let blank = self.space_at(p);
        let indent = self.begin_text().as_bytes();
        let indented = blank > 0 && (indent.is_empty() || self.starts(blank, indent));
        (!indented).then_some(p)
    }

    /// `^[\t ]*/?//[\t ]*[IRS][\t ]*$`: a formatter action, such as `// I`.
    /// `/?//` takes three slashes when they stand, and two otherwise.
    pub(super) fn formatter_action(&self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let slashes = self.skip(p, is_tab_or_space);
        let after = if self.starts(slashes, b"///") {
            slashes + 3
        } else {
            self.lit(slashes, b"//")?
        };
        let action = self.skip(after, is_tab_or_space);
        if !matches!(self.at(action), b'I' | b'R' | b'S') {
            return None;
        }
        let end = self.skip(action + 1, is_tab_or_space);
        self.at_eol(end).then_some(end)
    }

    /// `^[\t ]*//[\t ]*(import)[\t ]*(css-variables)[\t ]*(from)`.
    pub(super) fn css_variables_import(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let slashes = self.skip(p, is_tab_or_space);
        let import = self.skip(self.lit(slashes, b"//")?, is_tab_or_space);
        let import_end = self.lit(import, b"import")?;
        let variables = self.skip(import_end, is_tab_or_space);
        let variables_end = self.lit(variables, b"css-variables")?;
        let from = self.skip(variables_end, is_tab_or_space);
        let end = self.lit(from, b"from")?;
        self.capture(1, import, import_end);
        self.capture(2, variables, variables_end);
        self.capture(3, from, end);
        Some(end)
    }

    /// `["']?\.{0,2}[/\w]+["']?`.
    /// Giving back a dot leaves a dot where `[/\w]` must start, so neither
    /// the quote nor the dots give anything back.
    pub(super) fn import_quotes(&self, p: usize) -> Option<usize> {
        let mut i = if matches!(self.at(p), b'"' | b'\'') {
            p + 1
        } else {
            p
        };
        for _ in 0..2 {
            if self.at(i) == b'.' {
                i += 1;
            }
        }
        let from = i;
        loop {
            if self.at(i) == b'/' {
                i += 1;
            } else if let Some(width) = self.word_at(i) {
                i += width;
            } else {
                break;
            }
        }
        self.spend(i - from);
        if i == from {
            return None;
        }
        Some(if matches!(self.at(i), b'"' | b'\'') {
            i + 1
        } else {
            i
        })
    }

    /// `^@import(.*?)( as.*)?$`.
    ///
    /// The lazy group grows a character at a time.
    /// Before each, ` as` takes the rest of the line, or `$` ends the match.
    /// So group 1 runs to the first ` as`, and group 2 holds the rest.
    pub(super) fn import(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let path = self.lit(p, b"@import")?;
        let lim = self.lim();
        let alias = (path..lim).find(|&i| self.starts(i, b" as"));
        self.spend(alias.unwrap_or(lim) - path);
        self.capture(1, path, alias.unwrap_or(lim));
        if let Some(start) = alias {
            self.capture(2, start, lim);
        }
        Some(lim)
    }

    /// `@mixin|^[\t ]*=|@function`.
    pub(super) fn mixin(&self, p: usize) -> Option<usize> {
        if let Some(end) = self.lit(p, b"@mixin").or_else(|| self.lit(p, b"@function")) {
            return Some(end);
        }
        if p != 0 {
            return None;
        }
        let equals = self.skip(p, is_tab_or_space);
        self.lit(equals, b"=")
    }

    /// `@include|\+(?![\W\d])`: a `+` before a word character that is no
    /// digit.
    pub(super) fn include(&self, p: usize) -> Option<usize> {
        if let Some(end) = self.lit(p, b"@include") {
            return Some(end);
        }
        let next = self.lit(p, b"+")?;
        (self.word_at(next).is_some() && self.digit_end(next) == next).then_some(next)
    }

    /// `$\n?|(?=\)(?:\s\)|\n))`: a map's end, at the line's end or at a `)`
    /// before a blank and another `)`, or before the line feed.
    pub(super) fn map_end(&self, p: usize) -> Option<usize> {
        if let Some(end) = self.line_end(p) {
            return Some(end);
        }
        if self.at(p) != b')' {
            return None;
        }
        let blank = self.space_at(p + 1);
        let closes = (blank > 0 && self.at(p + 1 + blank) == b')') || self.at(p + 1) == b'\n';
        closes.then_some(p)
    }

    /// `$\n?|\s(?!(all|braille|...|not)([,\s]))`: an at-rule ends with its
    /// line, or at a blank before anything but a media word and its `,` or
    /// blank.
    pub(super) fn at_rule_end(&self, p: usize) -> Option<usize> {
        if let Some(end) = self.line_end(p) {
            return Some(end);
        }
        let blank = self.space_at(p);
        if blank == 0 {
            return None;
        }
        let word = p + blank;
        let media = MEDIA_WORDS.iter().any(|text| {
            self.lit(word, text)
                .is_some_and(|end| self.at(end) == b',' || self.space_at(end) > 0)
        });
        (!media).then_some(word)
    }

    /// `$\n?|(?=\s\(|and\(|\),)`: a property list ends with its line, or before
    /// a blank and `(`, `and(` or `),`.
    pub(super) fn property_end(&self, p: usize) -> Option<usize> {
        if let Some(end) = self.line_end(p) {
            return Some(end);
        }
        let blank = self.space_at(p);
        let ends = (blank > 0 && self.at(p + blank) == b'(')
            || self.starts(p, b"and(")
            || self.starts(p, b"),");
        ends.then_some(p)
    }

    /// `--.+?(?=\))`: a custom property, up to the first `)` after at least
    /// one character.
    /// The lazy `.+?` may take a `)` as its first character.
    pub(super) fn custom_property(&self, p: usize) -> Option<usize> {
        let from = self.lit(p, b"--")?;
        let lim = self.lim();
        if from >= lim {
            return None;
        }
        let first = self.char_end(from);
        let close = (first..lim).find(|&i| self.at(i) == b')');
        self.spend(close.unwrap_or(lim) - from);
        close
    }

    /// `^((?<=[])]|not\(|[*>]|>\s)|\n*):[-:a-z]+|(:[-:])[-:a-z]+`.
    ///
    /// At the start of a line nothing stands before `p`, so the lookbehind
    /// fails and `\n*` takes nothing.
    /// The first branch is then a `:` and a run at the line's start.
    /// The second is a `:`, a `-` or `:`, and a run, anywhere.
    pub(super) fn line_pseudo_class(&self, p: usize) -> Option<usize> {
        let colon = self.lit(p, b":")?;
        if p == 0 {
            let run = self.skip(colon, is_pseudo);
            if run > colon {
                return Some(run);
            }
        }
        if !matches!(self.at(colon), b'-' | b':') {
            return None;
        }
        let run = self.skip(colon + 1, is_pseudo);
        (run > colon + 1).then_some(run)
    }
}
