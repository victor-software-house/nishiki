//! The matchers inside a string: escapes and `format` directives.

use super::super::engine::{Chain, Lexer};
use crate::case_fold;
use crate::rules::Chars;

/// The three directive patterns, which share their parameters and modifiers
/// and differ in the directive that ends them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::common_lisp) enum Directive {
    /// `StringAt1`: a bracket, `[]();<>\[^{}]`, as group 10.
    Bracket,
    /// `StringAt2`: a letter or mark, `[$%\&*?A-GIOPRSTWX_|~]`, as group 10.
    Letter,
    /// `StringAt3`: a function call, `(/)(name)(/)`, as groups 10 to 12.
    Call,
}

impl Directive {
    /// The directive's slot in the line's failed searches.
    fn slot(self) -> usize {
        match self {
            Directive::Bracket => 0,
            Directive::Letter => 1,
            Directive::Call => 2,
        }
    }
}

/// Whether `b` is a letter or mark directive, `[$%\&*?A-GIOPRSTWX_|~]` under
/// `(?i)`.
fn letter_directive(b: u8) -> bool {
    matches!(
        b.to_ascii_uppercase(),
        b'$' | b'%' | b'&' | b'*' | b'?' | b'A'
            ..=b'G' | b'I' | b'O' | b'P' | b'R' | b'S' | b'T' | b'W' | b'X' | b'_' | b'|' | b'~'
    )
}

impl Lexer<'_> {
    /// `\\.`: a backslash and the character after it, which is not the line
    /// feed.
    pub(super) fn escaped(&self, p: usize) -> Option<usize> {
        (self.at(p) == b'\\' && p + 1 < self.lim()).then(|| self.char_end(p + 1))
    }

    /// `(~\n)`: a tilde that ends the line, with the line feed.
    pub(super) fn tilde_line_feed(&self, p: usize) -> Option<usize> {
        (self.at(p) == b'~' && p + 1 == self.lim() && self.nl).then_some(p + 2)
    }

    /// `(?i)(~)(((([-+]?[0-9]+)|('.)|[#V])*?(,)?)*?)((:@|@:|[:@])?)` and the
    /// `directive`: `~` as group 1, the parameters as group 2, the
    /// modifiers as group 8, and the directive.
    ///
    /// The lazy loops try the modifiers and the directive at each place
    /// first, and take one more parameter only when that fails.
    /// A parameter is a number, `'` and any character, `#`, `V` or `,`.
    /// At each place at most one form of parameter can start, so the search
    /// follows one path.
    ///
    /// A number's digits are greedy.
    /// Giving one back leaves a digit, which neither the modifiers nor a
    /// directive take.
    /// The path then rejoins at the number's end.
    /// A `'.` takes any character, a quote or a tilde too, as Shiki does.
    ///
    /// A search that fails is kept.
    /// A tilde inside its path is the character of a `'.`.
    /// A search from that tilde therefore starts on the path, and fails where
    /// the first did.
    /// A line of many such tildes is then lexed in linear time.
    pub(super) fn directive(&mut self, p: usize, directive: Directive) -> Option<usize> {
        if self.at(p) != b'~' {
            return None;
        }
        let mut chains = self.x.chains.get();
        let slot = directive.slot();
        let kept = chains[slot];
        if kept.text == self.lim() && kept.tilde < p && p < kept.fail {
            return None;
        }
        let mut q = p + 1;
        loop {
            if let Some(end) = self.directive_at(q, directive) {
                self.cap(1, p, p + 1);
                if q > p + 1 {
                    self.cap(2, p + 1, q);
                }
                return Some(end);
            }
            let Some(next) = self.parameter(q) else {
                chains[slot] = Chain {
                    tilde: p,
                    fail: q,
                    text: self.lim(),
                };
                self.x.chains.set(chains);
                return None;
            };
            self.spend(next - q);
            q = next;
        }
    }

    /// The end of the parameter at `q`: `[-+]?[0-9]+`, `'.`, `#`, `V` or
    /// `,`.
    fn parameter(&self, q: usize) -> Option<usize> {
        match self.at(q) {
            b'+' | b'-' if self.at(q + 1).is_ascii_digit() => Some(self.digit_run(q + 1)),
            b'0'..=b'9' => Some(self.digit_run(q)),
            b'\'' => (q + 1 < self.lim()).then(|| self.char_end(q + 1)),
            b'#' | b'V' | b'v' | b',' => Some(q + 1),
            _ => None,
        }
    }

    /// The end of the run of ASCII digits from `i`.
    fn digit_run(&self, i: usize) -> usize {
        self.skip(i, |b| b.is_ascii_digit())
    }

    /// `((:@|@:|[:@])?)` and the `directive` at `q`, trying the
    /// modifiers in the grammar's order and then none.
    fn directive_at(&mut self, q: usize, directive: Directive) -> Option<usize> {
        let tries = [
            self.lit(q, b":@"),
            self.lit(q, b"@:"),
            matches!(self.at(q), b':' | b'@').then_some(q + 1),
            Some(q),
        ];
        for modifiers in tries.into_iter().flatten() {
            if let Some(end) = self.directive_end(modifiers, directive) {
                if modifiers > q {
                    self.cap(8, q, modifiers);
                }
                return Some(end);
            }
        }
        None
    }

    /// The `directive` at `m`, with its groups.
    fn directive_end(&mut self, m: usize, directive: Directive) -> Option<usize> {
        let end = match directive {
            Directive::Bracket => matches!(
                self.at(m),
                b']' | b'(' | b')' | b';' | b'<' | b'>' | b'[' | b'^' | b'{' | b'}'
            )
            .then_some(m + 1)?,
            Directive::Letter => self.letter_directive_end(m)?,
            Directive::Call => return self.call(m),
        };
        self.cap(10, m, end);
        Some(end)
    }

    /// The end of a letter or mark directive at `m`.
    /// Under `(?i)` the class takes `ſ`, which folds to `s`.
    fn letter_directive_end(&self, m: usize) -> Option<usize> {
        let b = *self.s.get(m)?;
        if b < 0x80 {
            return letter_directive(b).then_some(m + 1);
        }
        let (c, end) = self.char_at(m)?;
        (case_fold::letters(c) == Some(b"s")).then_some(end)
    }

    /// `(/)([]!#-\&*+\--:<-\[^_a-{}~]+?)(/)` at `m`: a function named
    /// between slashes, as groups 10 to 12.
    ///
    /// The lazy `+?` ends at the first `/` after one symbol character.
    /// The class holds `/`, so the first character may be one.
    /// Every character before that `/` must be a symbol character.
    /// The search finds the `/` first, so it reads no further than the match.
    fn call(&mut self, m: usize) -> Option<usize> {
        let name = self.lit(m, b"/")?;
        let first = self.char_end(name);
        if first == name || self.symbol_end_before(name, first) != first {
            return None;
        }
        let close = first + memchr::memchr(b'/', self.text(first, self.lim()))?;
        if self.symbol_end_before(first, close) != close {
            return None;
        }
        self.cap(10, m, name);
        self.cap(11, name, close);
        self.cap(12, close, close + 1);
        Some(close + 1)
    }
}
