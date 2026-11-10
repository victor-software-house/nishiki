//! Selectors: custom elements, pseudo-classes and pseudo-elements,
//! combinators and attribute selectors.

use super::super::engine::Lexer;
use super::super::words;
use super::chars::{Class, Escape};
use crate::rules::Chars;

impl Lexer<'_> {
    /// The end of one character of a custom element's name at `i`:
    /// `[-0-9_a-z·]`, `\\\.`, or a CSS name character beyond ASCII.
    fn custom_unit(&self, i: usize) -> Option<usize> {
        match self.at(i) {
            b'-' | b'0'..=b'9' | b'_' | b'a'..=b'z' if i < self.lim() => Some(i + 1),
            b'\\' if self.at(i + 1) == b'.' && i + 1 < self.lim() => Some(i + 2),
            0x80.. => self
                .class_at(i, Class::NameStart)
                .filter(|_| self.char_at(i).is_some_and(|(c, _)| !c.is_ascii()))
                .map(|w| i + w),
            _ => None,
        }
    }

    /// `\b([a-z](?:CHAR)*-(?:CHAR)*)\b`: a custom element's name.
    /// Of the ends after its first `-` where `\b` holds, the backtracking
    /// matcher takes the last.
    /// That is the last end of the run where `\b` holds, when a `-` comes
    /// before it.
    /// A read from inside the last run shares its ends.
    pub(super) fn custom_element(&self, p: usize) -> Option<usize> {
        if !self.at(p).is_ascii_lowercase() || p >= self.lim() || !self.boundary(p) {
            return None;
        }
        let (from, to, last, dash) = self.x.custom.get();
        let (last, dash) = if from < p && p < to {
            (last, dash)
        } else {
            let mut i = p + 1;
            let mut last = None;
            let mut dash = None;
            let mut dash_before = None;
            while let Some(next) = self.custom_unit(i) {
                if self.at(i) == b'-' {
                    dash = Some(i);
                }
                if self.boundary(next) {
                    last = Some(next);
                    dash_before = dash;
                }
                i = next;
            }
            self.spend(i - p);
            self.x.custom.set((p, i, last, dash_before));
            (last, dash_before)
        };
        dash.filter(|&dash| dash > p).and(last)
    }

    /// `(?![-\w[^\x00-\x{9F}]]|\\(\h{1,6} ?|\H)|(@(?=\{)))`: no name
    /// character, escape or interpolation follows.
    pub(super) fn name_ends(&self, p: usize) -> Option<usize> {
        let name = self.class_at(p, Class::DashWordWide).is_some();
        let escape = self.at(p) == b'\\' && p < self.lim() && (p + 1 < self.lim() || self.nl);
        let interpolation = self.starts(p, b"@{");
        (!name && !escape && !interpolation).then_some(p)
    }

    /// `(?!\+_?:)(?:>{1,3}|[+~])(?![+;>}~])`. A run of `>` matches only
    /// whole, at three or fewer: a shorter match leaves a `>` next, which
    /// the lookahead refuses.
    pub(super) fn combinator(&self, p: usize) -> Option<usize> {
        if self.starts(p, b"+:") || self.starts(p, b"+_:") || p >= self.lim() {
            return None;
        }
        let end = match self.at(p) {
            b'>' => {
                let run = (p..p + 4).find(|&i| self.at(i) != b'>').unwrap_or(p + 4);
                if run - p > 3 {
                    return None;
                }
                run
            }
            b'+' | b'~' => p + 1,
            _ => return None,
        };
        (!matches!(self.at(end), b'+' | b';' | b'>' | b'}' | b'~') || end >= self.lim())
            .then_some(end)
    }

    /// `(>{1,3}|[+~]){2,}`: a run of combinators of two characters or more,
    /// since a run of `>` splits into as many repetitions as it needs.
    pub(super) fn combinator_run(&self, p: usize) -> Option<usize> {
        let end = self.skip(p, |b| matches!(b, b'>' | b'+' | b'~'));
        (end >= p + 2).then_some(end)
    }

    /// `\s*([$*^|~]?=)\s*`.
    pub(super) fn attribute_operator(&mut self, p: usize) -> Option<usize> {
        let at = self.spaces(p);
        let eq = if matches!(self.at(at), b'$' | b'*' | b'^' | b'|' | b'~') {
            at + 1
        } else {
            at
        };
        if self.at(eq) != b'=' || eq >= self.lim() {
            return None;
        }
        self.capture(1, at, eq + 1);
        Some(self.spaces(eq + 1))
    }

    /// `(?:\s+([Ii]))?`.
    /// It always matches.
    #[expect(
        clippy::unnecessary_wraps,
        reason = "less_matcher tail-calls it and needs the same return type"
    )]
    pub(super) fn attribute_flag(&mut self, p: usize) -> Option<usize> {
        if let Some(at) = self.spaces_run(p)
            && matches!(self.at(at), b'I' | b'i')
            && at < self.lim()
        {
            self.capture(1, at, at + 1);
            return Some(at + 1);
        }
        Some(p)
    }

    /// `(:)(a|b|…)(?=\()`, the colon in group 1 and the name in group 2.
    pub(super) fn pseudo_func(&mut self, p: usize, names: &[&[u8]]) -> Option<usize> {
        if self.at(p) != b':' {
            return None;
        }
        let end = names
            .iter()
            .find_map(|name| self.lit(p + 1, name).filter(|&end| self.at(end) == b'('))?;
        self.capture(1, p, p + 1);
        self.capture(2, p + 1, end);
        Some(end)
    }

    /// The end of `::?` at `p`.
    fn colons(&self, p: usize) -> Option<usize> {
        (self.at(p) == b':' && p < self.lim())
            .then(|| if self.at(p + 1) == b':' { p + 2 } else { p + 1 })
    }

    /// `(::?)(a|b|…)(?=\s*(\())`, the colons in group 1. A second `:` is no
    /// name, so giving it back cannot help.
    pub(super) fn pseudo_element_func(&mut self, p: usize, names: &[&[u8]]) -> Option<usize> {
        let name = self.colons(p)?;
        let end = names.iter().find_map(|word| {
            self.lit(name, word)
                .filter(|&end| self.spaces_then(end, b"(").is_some())
        })?;
        self.capture(1, p, name);
        Some(end)
    }

    /// `(:)(a|b|…)\b` for the pseudo-classes that take no argument.
    pub(super) fn pseudo_class(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b':' {
            return None;
        }
        let end = self.table_word(p + 1, words::PSEUDO_CLASSES)?;
        self.capture(1, p, p + 1);
        self.capture(2, p + 1, end);
        Some(end)
    }

    /// `(::?)(a|b|…)\b` for the pseudo-elements.
    pub(super) fn pseudo_element(&mut self, p: usize) -> Option<usize> {
        let name = self.colons(p)?;
        let end = self.table_word(name, words::PSEUDO_ELEMENTS)?;
        self.capture(1, p, name);
        self.capture(2, name, end);
        Some(end)
    }

    /// `(::?)(-\w+-)(--|-?NAME)\b`: a vendor's pseudo-element, where the name
    /// has the CSS name characters and gives characters back until `\b`
    /// holds.
    pub(super) fn vendor_pseudo(&mut self, p: usize) -> Option<usize> {
        let vendor = self.colons(p)?;
        if self.at(vendor) != b'-' {
            return None;
        }
        let word = self.word_end(vendor + 1);
        if word == vendor + 1 || self.at(word) != b'-' {
            return None;
        }
        let name = word + 1;
        let end = self
            .lit(name, b"--")
            .filter(|&end| self.boundary(end))
            .or_else(|| {
                let head = if self.at(name) == b'-'
                    && self
                        .unit_at(name + 1, Class::NameStart, Escape::Any)
                        .is_some()
                {
                    name + 1
                } else {
                    name
                };
                let first = self.unit_at(head, Class::NameStart, Escape::Any)?;
                let end = self.units_end(first, Class::NameRest, Escape::Any);
                self.boundary_back(first - 1, end)
                    .filter(|&end| end >= first)
            })?;
        self.capture(1, p, vendor);
        self.capture(2, vendor, name);
        Some(end)
    }

    /// `(?=[#\&*+./>A-\[a-z~]|(:{1,2}\S)|@\{)`: where a selector starts.
    pub(super) fn selector_ahead(&self, p: usize) -> Option<usize> {
        if p >= self.lim() {
            return None;
        }
        let b = self.at(p);
        let starts = matches!(b, b'#' | b'&' | b'*' | b'+' | b'.' | b'/' | b'>' | b'A'..=b'[' | b'a'..=b'z' | b'~')
            || b == b':' && p + 1 < self.lim() && self.space_at(p + 1) == 0
            || self.starts(p, b"@{");
        starts.then_some(p)
    }

    /// `(?=@(?!\{)|[;{])`: where a selector ends.
    pub(super) fn selector_end(&self, p: usize) -> Option<usize> {
        if p >= self.lim() {
            return None;
        }
        let ends = match self.at(p) {
            b'@' => self.at(p + 1) != b'{' || p + 1 >= self.lim(),
            b';' | b'{' => true,
            _ => false,
        };
        ends.then_some(p)
    }
}
