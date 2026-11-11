//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups of its match with [`Lexer::capture`].
//! The engine names those the grammar names.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would.
//! Where a quantifier could give characters back, the doc comment shows why it
//! cannot matter, or how the matcher gives them back.
//!
//! Most patterns sit between a lookbehind and the lookahead `(?=([()\s]))`.
//! No word of their alternations holds `(`, `)` or a blank.
//! So a match is the whole run of characters up to the next of those, which
//! [`Lexer::run_end`] measures once for every pattern that asks.
//! `(?i)` compares words through [`crate::case_fold`].
//! Under `(?i)`, a symbol class takes `ſ` for `s` and the Kelvin sign for `k`.
//! `\s` reads Unicode, through [`crate::classes`].

mod defs;
mod marks;
mod strings;

use super::engine::{Lexer, Lowered, Run};
use super::grammar::{Pattern, Rule};
use super::words;
use crate::classes::{is_ascii_space, is_space};
use crate::rules::Chars;
use crate::{bytes, case_fold};

/// Whether `b` is in the grammar's symbol class `[]!#-\&*+\--:<-\[^_a-{}~]`.
fn symbol_byte(b: u8) -> bool {
    matches!(
        b,
        b'!' | b'#'..=b'&'
            | b'*'
            | b'+'
            | b'-'..=b':'
            | b'<'..=b'['
            | b']'
            | b'^'
            | b'_'
            | b'a'..=b'{'
            | b'}'
            | b'~'
    )
}

/// Whether `b` is in the package class `[]!$%\&*+\--9<-\[^_a-{}~]`, which
/// leaves out `#` and `:`.
fn package_byte(b: u8) -> bool {
    matches!(
        b,
        b'!' | b'$'..=b'&'
            | b'*'
            | b'+'
            | b'-'..=b'9'
            | b'<'..=b'['
            | b']'
            | b'^'
            | b'_'
            | b'a'..=b'{'
            | b'}'
            | b'~'
    )
}

/// Whether `c`, a character above ASCII, is in a `(?i)` class that holds
/// every ASCII letter: `ſ` and the Kelvin sign, which fold to one letter.
fn folds_to_letter(c: char) -> bool {
    matches!(case_fold::letters(c), Some([_]))
}

impl Lexer<'_> {
    /// Whether `pattern` matches at `p`, and where its match ends.
    ///
    /// The arms come in the grammar's order.
    pub(super) fn matcher(&mut self, pattern: Pattern, p: usize) -> Option<usize> {
        match pattern {
            Pattern::Comment => self.comment(p),
            Pattern::CommentEnd => (!self.at_g(p)).then_some(p),
            Pattern::CommentAt0 => self.lit(p, b";"),
            Pattern::CommentAt0End => self.line_feed(p),
            Pattern::BlockComment => self.lit(p, b"#|"),
            Pattern::BlockCommentEnd => self.lit(p, b"|#"),
            Pattern::String | Pattern::StringEnd => {
                let end = self.lit(p, b"\"")?;
                self.capture(1, p, end);
                Some(end)
            }
            Pattern::StringAt0 => self.escaped(p),
            Pattern::StringAt1 => self.directive(p, strings::Directive::Bracket),
            Pattern::StringAt2 => self.directive(p, strings::Directive::Letter),
            Pattern::StringAt3 => self.directive(p, strings::Directive::Call),
            Pattern::StringAt4 => self.tilde_line_feed(p),
            Pattern::Escape => self.character(p),
            Pattern::ConstantAt0 => self.listed(p, self.after_form(p), words::CONSTANTS),
            Pattern::ConstantAt1 => self.number(p, marks::Number::Rational),
            Pattern::ConstantAt2 => self.dot(p),
            Pattern::ConstantAt3 => self.number(p, marks::Number::Float),
            Pattern::LambdaList => self.marked_symbol(p, b'&', self.after_open(p)),
            Pattern::FunctionAt0 => self.listed(p, self.after_function(p), words::ACCESSORS),
            Pattern::FunctionAt1 => self.listed(p, self.after_function(p), words::SIDE_EFFECTS),
            Pattern::FunctionAt2 => self.listed(p, self.after_function(p), words::FUNCTIONS),
            Pattern::FunctionAt3 => self.listed(p, self.after_function(p), words::GENERICS),
            Pattern::FunctionAt4 => self.listed(p, self.after_function(p), words::REINITIALIZE),
            Pattern::FunctionAt5 => self.listed(p, self.after_function(p), words::TYPE_SPECIFIERS),
            Pattern::StyleGuideAt0 => self.quoted_earmuffs(p),
            Pattern::StyleGuideAt1 => self.earmuffs_after(p, b'+'),
            Pattern::StyleGuideAt2 => self.earmuffs_after(p, b'*'),
            Pattern::PackageAt0 => self.package(p),
            Pattern::Nested0At1 | Pattern::Nested1At1 => {
                self.non_space(p).then(|| self.char_end(p))
            }
            Pattern::DefNameAt0 => self.def_function(p),
            Pattern::DefNameAt1 => self.def_named(p, words::DEF_TYPES),
            Pattern::DefNameAt2 => self.def_named(p, words::DEF_CONSTANT),
            Pattern::DefNameAt3 => self.def_variable(p),
            Pattern::DefNameAt4 => self.def_struct(p),
            Pattern::DefNameAt5 => self.local_function(p),
            Pattern::MacroAt0 => self.listed(p, self.after_open(p), words::PURE_MACROS),
            Pattern::MacroAt1 => self.listed(p, self.after_open(p), words::MACROS),
            Pattern::MacroAt2 => self.listed(p, self.after_open(p), words::SPECIAL_FORMS),
            Pattern::Symbol => self.listed(p, self.after_open(p), words::SYMBOLS),
            Pattern::SpecialOperator => self.special_operator(p),
            Pattern::Declaration => self.listed(p, self.after_open(p), words::DECLARATIONS),
            Pattern::Type => self.listed(p, self.after_open(p), words::TYPES),
            Pattern::Class => self.listed(p, self.after_open(p), words::CLASSES),
            Pattern::ConditionType => self.listed(p, self.after_open(p), words::CONDITIONS),
            Pattern::VariableAt0 => self.listed(p, self.after_form(p), words::GLOBALS),
            Pattern::VariableAt1 => self.listed(p, self.after_form(p), words::REPL),
            Pattern::PunctuationAt0 => self.quote(p),
            Pattern::PunctuationAt1 => self.marked_symbol(p, b':', self.after_form(p)),
            Pattern::PunctuationAt2 => self.sharp_digits(p, marks::Sharp::Paren),
            Pattern::PunctuationAt3 => self.sharp_digits(p, marks::Sharp::Bits),
            Pattern::PunctuationAt4 => self.sharp_star(p),
            Pattern::PunctuationAt5 => self.sharp_digits(p, marks::Sharp::Array),
            Pattern::PunctuationAt6 => self.sharp_digits(p, marks::Sharp::Label),
            Pattern::PunctuationAt7 => self.sharp_digits(p, marks::Sharp::Reference),
            Pattern::PunctuationAt8 => self.sharp_sign(p),
            Pattern::PunctuationAt9 => self.sharp_reader(p),
            Pattern::PunctuationAt10 => self.sharp_keyword(p),
            Pattern::PunctuationAt11 => self.backquote(p),
        }
    }

    // Lookbehinds.

    /// Whether the character before `p` is a `\s`.
    fn space_before(&self, p: usize) -> bool {
        match self.prev(p) {
            None => false,
            Some(b) if b < 0x80 => is_ascii_space(b),
            Some(_) => self.char_before(p).is_some_and(is_space),
        }
    }

    /// `(?<=^|[(\s])` at `p`.
    pub(super) fn after_open(&self, p: usize) -> bool {
        p == 0 || self.prev(p) == Some(b'(') || self.space_before(p)
    }

    /// `(?<=^|[(\s]|,@|,\.?)` at `p`: after a form opens, or after a comma
    /// with its `@` or `.`.
    pub(super) fn after_form(&self, p: usize) -> bool {
        match self.prev(p) {
            Some(b',') => true,
            Some(b'@' | b'.') => p >= 2 && self.at(p - 2) == b',',
            _ => self.after_open(p),
        }
    }

    /// `(?<=^|[(\s]|#')` at `p`: after a form opens, or after `#'`.
    fn after_function(&self, p: usize) -> bool {
        (p >= 2 && self.at(p - 1) == b'\'' && self.at(p - 2) == b'#') || self.after_open(p)
    }

    // Runs and words.

    /// Whether a delimiter, `(`, `)` or a `\s`, stands at `i`.
    /// The line feed at the end of the line is a `\s`.
    pub(super) fn delimiter(&self, i: usize) -> bool {
        matches!(self.at(i), b'(' | b')') || self.space_at(i) > 0
    }

    /// Whether a `\S` character stands at `i`: `(?=\S)`.
    pub(super) fn non_space(&self, i: usize) -> bool {
        i < self.lim() && self.space_at(i) == 0
    }

    /// Where the next delimiter at or after `p` stands, or the end of the text.
    /// A place inside the run last measured shares its end.
    pub(super) fn run_end(&self, p: usize) -> usize {
        let run = self.x.run.get();
        if run.text == self.lim() && run.start <= p && p <= run.end {
            return run.end;
        }
        let mut i = p;
        while i < self.lim() && !self.delimiter(i) {
            i = self.char_end(i);
        }
        self.spend(i - p);
        self.x.run.set(Run {
            start: p,
            end: i,
            text: self.lim(),
        });
        i
    }

    /// The run `p..end` lowered as `(?i)` reads it, once for every word list
    /// that asks about it.
    fn lowered(&self, p: usize, end: usize) -> Lowered {
        let kept = self.x.lowered.get();
        if kept.start == p && kept.text == self.lim() {
            return kept;
        }
        let mut word = Lowered {
            start: p,
            text: self.lim(),
            len: None,
            letters: [0; words::LONGEST],
        };
        word.len = case_fold::lower_into(self.text(p, end), &mut word.letters);
        self.x.lowered.set(word);
        word
    }

    /// The end of the run at `p` when a delimiter ends it and `words` holds
    /// it: `(?i)(?:…)(?=([()\s]))` for the alternation `words`, after a
    /// lookbehind that `behind` says holds.
    ///
    /// No word holds a delimiter, so an alternative can only match the whole
    /// run.
    pub(super) fn listed(&self, p: usize, behind: bool, words: &[&[u8]]) -> Option<usize> {
        debug_assert!(
            words.is_sorted(),
            "a word list is sorted for its binary search"
        );
        if !behind {
            return None;
        }
        let end = self.run_end(p);
        if end == p || !self.delimiter(end) {
            return None;
        }
        let word = self.lowered(p, end);
        let letters = word.letters.get(..word.len?)?;
        words
            .binary_search_by(|word| bytes::order(word, letters))
            .is_ok()
            .then_some(end)
    }

    /// The end of the symbol run from `i`.
    pub(super) fn symbol_end(&self, i: usize) -> usize {
        self.class_end(i, self.lim(), symbol_byte)
    }

    /// The end of the symbol run from `i`, stopping at `stop`.
    pub(super) fn symbol_end_before(&self, i: usize, stop: usize) -> usize {
        self.class_end(i, stop, symbol_byte)
    }

    /// The end of the run from `i` whose characters `ascii` accepts, or fold to
    /// an ASCII letter under `(?i)`, stopping at `stop`.
    fn class_end(&self, mut i: usize, stop: usize, ascii: fn(u8) -> bool) -> usize {
        let from = i;
        while i < stop
            && let Some(&b) = self.s.get(i)
        {
            if b < 0x80 {
                if !ascii(b) {
                    break;
                }
                i += 1;
            } else {
                match self.char_at(i) {
                    Some((c, end)) if folds_to_letter(c) => i = end,
                    _ => break,
                }
            }
        }
        self.spend(i - from);
        i
    }

    /// `[]!#-\&*+\--:<-\[^_a-{}~]+?(?=([()\s]))` at `i`: the end of a symbol
    /// that a delimiter ends.
    ///
    /// The class holds no delimiter, so the lazy `+?` takes the whole run.
    pub(super) fn delimited_symbol(&self, i: usize) -> Option<usize> {
        let end = self.symbol_end(i);
        (end > i && self.delimiter(end)).then_some(end)
    }

    /// `&` or `:` as `mark`, then a delimited symbol, after the lookbehind
    /// that `behind` says holds: `LambdaList` and `PunctuationAt1`.
    ///
    /// The lambda list's words after its `[…]+?` branch are symbols too, so
    /// that branch decides.
    fn marked_symbol(&self, p: usize, mark: u8, behind: bool) -> Option<usize> {
        if !behind || self.at(p) != mark {
            return None;
        }
        self.delimited_symbol(p + 1)
    }

    /// `(?i)(?<=^|[(\s]|,@|,\.?)(([]!$%\&*+\--9<-\[^_a-{}~]+?)|(#))(?=::?)`:
    /// a package name before a colon, as group 2, or `#` as group 3.
    ///
    /// The class leaves out `:`, so the lazy `+?` takes the whole run.
    fn package(&mut self, p: usize) -> Option<usize> {
        if !self.after_form(p) {
            return None;
        }
        let end = self.class_end(p, self.lim(), package_byte);
        if end > p && self.at(end) == b':' {
            self.capture(2, p, end);
            return Some(end);
        }
        if self.at(p) == b'#' && self.at(p + 1) == b':' {
            self.capture(3, p, p + 1);
            return Some(p + 1);
        }
        None
    }

    /// `(?i)(\(\s*)(…)(?=([()\s]))`: an open parenthesis, blanks and a
    /// special operator, as group 2.
    ///
    /// Giving back a blank would start the word at a blank.
    fn special_operator(&mut self, p: usize) -> Option<usize> {
        let open = self.lit(p, b"(")?;
        let word = self.space_end(open);
        let end = self.listed(word, true, words::SPECIAL_OPERATORS)?;
        self.capture(2, word, end);
        Some(end)
    }

    // Comments.

    /// `(^[\t ]+)?(?=;)`: the blanks that start a line before `;`, as group
    /// 1, or nothing before a `;` anywhere.
    fn comment(&mut self, p: usize) -> Option<usize> {
        if p == 0 {
            let blanks = self.skip(0, |b| b == b' ' || b == b'\t');
            if blanks > 0 && self.at(blanks) == b';' {
                self.capture(1, 0, blanks);
                return Some(blanks);
            }
        }
        (self.at(p) == b';').then_some(p)
    }

    /// `\n`: the line feed that ends the line.
    fn line_feed(&self, p: usize) -> Option<usize> {
        (p == self.lim() && self.nl).then_some(p + 1)
    }

    /// Where the scan goes on after no pattern matched at `p`.
    ///
    /// Outside the root, every byte may start a pattern.
    /// In the root, a pattern starts at `"`, `;`, `#` or `(`, or at the start
    /// of the line.
    /// It also starts after a byte that one of the lookbehinds reads.
    /// Those bytes are `(`, a blank, `,`, `@`, `.`, `'`, `:`, or a byte above
    /// ASCII, which may end a `\s`.
    /// Every other place is skipped.
    pub(super) fn after_miss(&self, p: usize) -> usize {
        let mut i = self.char_end(p);
        if self.top_rule() != Rule::Root {
            return i;
        }
        let from = i;
        while let Some(&b) = self.s.get(i) {
            let behind = self.at(i - 1);
            if matches!(b, b'"' | b';' | b'#' | b'(')
                || behind >= 0x80
                || is_ascii_space(behind)
                || matches!(behind, b'(' | b',' | b'@' | b'.' | b'\'' | b':')
            {
                break;
            }
            i = self.char_end(i);
        }
        self.spend(i - from);
        i
    }
}
