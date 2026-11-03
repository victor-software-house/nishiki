//! Property names and values: keywords, functions, numbers and colours.

use super::super::ahead::Find;
use super::super::engine::Lexer;
use super::super::words;
use crate::rules::Chars;

impl Lexer<'_> {
    /// `(?<![-\w])(name)(\()` for the names in `list`: the name is group 1 and
    /// the parenthesis group 2.
    fn function_begin(&mut self, p: usize, list: &[&str]) -> Option<usize> {
        if self.wordish_before(p) {
            return None;
        }
        let name = self.wordish_run(p);
        if name == p || self.at(name) != b'(' || name >= self.lim() || !self.in_list(p, name, list)
        {
            return None;
        }
        self.cap(1, p, name);
        self.cap(2, name, name + 1);
        Some(name + 1)
    }

    /// `{`
    pub(in crate::css) fn property_values_at0(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b"{")
    }

    /// `}`
    pub(in crate::css) fn property_values_at0_end(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b"}")
    }

    /// `\(`
    pub(in crate::css) fn function_nesting(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b"(")
    }

    /// `(?i)(?<![-\w])(calc)(\()`
    pub(in crate::css) fn functions_at0(&mut self, p: usize) -> Option<usize> {
        self.function_begin(p, &["calc"])
    }

    /// `[*/]|(?<=\s|^)[-+](?=\s|$)`
    pub(in crate::css) fn arithmetic_operators(&mut self, p: usize) -> Option<usize> {
        match self.s.get(p)? {
            b'*' | b'/' => Some(p + 1),
            b'-' | b'+' => {
                let behind = p == 0 || self.space_before(p);
                let ahead = self.space_at(p + 1) > 0 || self.at_end(p + 1);
                (behind && ahead).then_some(p + 1)
            }
            _ => None,
        }
    }

    /// `(?i)(?<![-\w])(rgba?|hsla?|hwb|lab|oklab|lch|oklch|color)(\()`
    pub(in crate::css) fn functions_at1(&mut self, p: usize) -> Option<usize> {
        self.function_begin(p, words::FUNCTION_COLORS)
    }

    /// The gradient functions.
    pub(in crate::css) fn functions_at2(&mut self, p: usize) -> Option<usize> {
        self.function_begin(p, words::FUNCTION_GRADIENTS)
    }

    /// `(?i)(?<![-\w])(from|to|at|in|hue)(?![-\w])`
    pub(in crate::css) fn functions_at2_at0(&mut self, p: usize) -> Option<usize> {
        let end = self.standalone_word(p, &["at", "from", "hue", "in", "to"])?;
        self.cap(1, p, end);
        Some(end)
    }

    /// `(?i)(?<![-\w])(-webkit-gradient)(\()`
    pub(in crate::css) fn functions_at3(&mut self, p: usize) -> Option<usize> {
        self.function_begin(p, &["-webkit-gradient"])
    }

    /// `(?i)(?<![-\w])(from|to|color-stop)(\()`
    pub(in crate::css) fn functions_at3_at0(&mut self, p: usize) -> Option<usize> {
        self.function_begin(p, &["color-stop", "from", "to"])
    }

    /// The functions of the grammar's long list, followed by a parenthesis.
    pub(in crate::css) fn functions_at4(&mut self, p: usize) -> Option<usize> {
        self.function_begin(p, words::FUNCTIONS)
    }

    /// `(?i)(?<=[",\s]|\*/|^)\d+x(?=["'),\s]|/\*|$)`
    pub(in crate::css) fn functions_at4_at0(&mut self, p: usize) -> Option<usize> {
        let behind = p == 0
            || matches!(self.s.get(p - 1), Some(b'"' | b','))
            || self.space_before(p)
            || (p >= 2 && self.s.get(p - 2..p) == Some(b"*/"));
        if !behind {
            return None;
        }
        let digits = self.digit_end(p);
        if digits == p {
            return None;
        }
        let end = self.lit_ci(digits, "x")?;
        let ahead = matches!(self.s.get(end), Some(b'"' | b'\'' | b')' | b','))
            || self.space_at(end) > 0
            || self.starts(end, b"/*")
            || self.at_end(end);
        ahead.then_some(end)
    }

    /// `[^"'),\s]+`
    pub(in crate::css) fn functions_at4_at3(&mut self, p: usize) -> Option<usize> {
        let end = self.skip_chars(p, |b| !matches!(b, b'"' | b'\'' | b')' | b','));
        (end > p).then_some(end)
    }

    /// The end of the run of characters from `p` that `class` accepts and that
    /// are not white space; `class` sees every byte of a character.
    fn skip_chars(&self, p: usize, class: impl Fn(u8) -> bool) -> usize {
        let mut i = p;
        while i < self.lim() && class(self.s[i]) && self.space_at(i) == 0 {
            i = self.char_end(i);
        }
        self.spend(i - p + 1);
        i
    }

    /// `(?i)(?<![-\w])(circle|ellipse|inset|polygon|rect)(\()`
    pub(in crate::css) fn functions_at5(&mut self, p: usize) -> Option<usize> {
        self.function_begin(p, &["circle", "ellipse", "inset", "polygon", "rect"])
    }

    /// `(?i)(?<=\s|^|\*/)(at|round)(?=\s|/\*|$)`
    pub(in crate::css) fn functions_at5_at0(&mut self, p: usize) -> Option<usize> {
        let behind =
            p == 0 || self.space_before(p) || (p >= 2 && self.s.get(p - 2..p) == Some(b"*/"));
        if !behind {
            return None;
        }
        let end = self.lit_ci(p, "at").or_else(|| self.lit_ci(p, "round"))?;
        let ahead = self.space_at(end) > 0 || self.starts(end, b"/*") || self.at_end(end);
        ahead.then(|| {
            self.cap(1, p, end);
            end
        })
    }

    /// `(?i)(?<![-\w])(cubic-bezier|steps)(\()`
    pub(in crate::css) fn functions_at6(&mut self, p: usize) -> Option<usize> {
        self.function_begin(p, &["cubic-bezier", "steps"])
    }

    /// `(?i)(?<![-\w])(start|end)(?=\s*\)|$)`
    pub(in crate::css) fn functions_at6_at0(&mut self, p: usize) -> Option<usize> {
        if self.wordish_before(p) {
            return None;
        }
        let end = self.lit_ci(p, "start").or_else(|| self.lit_ci(p, "end"))?;
        let after = self.spaces(end);
        let ahead = (self.at(after) == b')' && after < self.lim()) || self.at_end(end);
        ahead.then(|| {
            self.cap(1, p, end);
            end
        })
    }

    /// The transform functions.
    pub(in crate::css) fn functions_at7(&mut self, p: usize) -> Option<usize> {
        self.function_begin(p, words::FUNCTION_TRANSFORMS)
    }

    /// `(?i)(?<![-@\w])(url)(\()`
    pub(in crate::css) fn url(&mut self, p: usize) -> Option<usize> {
        if self.char_before(p) == Some('@') {
            return None;
        }
        self.function_begin(p, &["url"])
    }

    /// `[^"')\s]+`
    pub(in crate::css) fn url_at0(&mut self, p: usize) -> Option<usize> {
        let end = self.skip_chars(p, |b| !matches!(b, b'"' | b'\'' | b')'));
        (end > p).then_some(end)
    }

    /// `(?<![-\w])(--[-A-Z_a-z[^\x00-\x7F]](?:...)*)(\()`
    pub(in crate::css) fn functions_at9(&mut self, p: usize) -> Option<usize> {
        if self.wordish_before(p) {
            return None;
        }
        let name = self.custom_property(p)?;
        if self.at(name) != b'(' || name >= self.lim() {
            return None;
        }
        self.cap(1, p, name);
        self.cap(2, name, name + 1);
        Some(name + 1)
    }

    /// `--[-A-Z_a-z[^\x00-\x7F]](?:[-0-9A-Z_a-z[^\x00-\x7F]]|\\(?:\h{1,6}|.))*`
    fn custom_property(&self, p: usize) -> Option<usize> {
        let dashes = self.lit(p, b"--")?;
        self.name_end(dashes)
    }

    /// `(?i)(?<![-\w])(var)(\()`
    pub(in crate::css) fn functions_at10(&mut self, p: usize) -> Option<usize> {
        self.function_begin(p, &["var"])
    }

    /// `--[-A-Z_a-z[^\x00-\x7F]](?:...)*`
    pub(in crate::css) fn functions_at10_at0(&mut self, p: usize) -> Option<usize> {
        self.custom_property(p)
    }

    /// `(?<![-\w])--...` as a property name inside a rule list.
    pub(in crate::css) fn rule_list_innards_at3(&mut self, p: usize) -> Option<usize> {
        if self.wordish_before(p) {
            return None;
        }
        self.custom_property(p)
    }

    /// The keywords of the grammar's first list.
    pub(in crate::css) fn property_keywords_at0(&mut self, p: usize) -> Option<usize> {
        let end = self.standalone_word(p, words::PROPERTY_KEYWORDS)?;
        self.cap(1, p, end);
        Some(end)
    }

    /// The list styles.
    pub(in crate::css) fn property_keywords_at1(&mut self, p: usize) -> Option<usize> {
        let end = self.standalone_word(p, words::LIST_STYLES)?;
        self.cap(1, p, end);
        Some(end)
    }

    /// `(?<![-\w])(?i:-(?:ah|apple|...)|(?:mso|prince))-[-A-Za-z]+`
    pub(in crate::css) fn property_keywords_at2(&mut self, p: usize) -> Option<usize> {
        if self.wordish_before(p) {
            return None;
        }
        self.vendor_prefixed(p, false)
    }

    /// The font families.
    pub(in crate::css) fn property_keywords_at3(&mut self, p: usize) -> Option<usize> {
        self.standalone_word(p, words::FONT_FAMILIES)
    }

    /// `(?i)(?<![-\w])(?:accent-color|...)(?![-\w])`
    pub(in crate::css) fn property_names_at0(&mut self, p: usize) -> Option<usize> {
        self.standalone_word(p, words::PROPERTY_NAMES)
    }

    /// The W3C standard colour names.
    pub(in crate::css) fn color_keywords_at0(&mut self, p: usize) -> Option<usize> {
        let end = self.standalone_word(p, words::COLORS_STANDARD)?;
        self.cap(1, p, end);
        Some(end)
    }

    /// The extended colour names.
    pub(in crate::css) fn color_keywords_at1(&mut self, p: usize) -> Option<usize> {
        let end = self.standalone_word(p, words::COLORS_EXTENDED)?;
        self.cap(1, p, end);
        Some(end)
    }

    /// `(?i)(?<![-\w])currentColor(?![-\w])`
    pub(in crate::css) fn color_keywords_at2(&mut self, p: usize) -> Option<usize> {
        self.standalone_word(p, &["currentcolor"])
    }

    /// The system colour names.
    pub(in crate::css) fn color_keywords_at3(&mut self, p: usize) -> Option<usize> {
        let end = self.standalone_word(p, words::COLORS_SYSTEM)?;
        self.cap(1, p, end);
        Some(end)
    }

    /// `!\s*important(?![-\w])`
    pub(in crate::css) fn property_values_at10(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b"!")?;
        let word = self.spaces(p + 1);
        let end = self.lit(word, b"important")?;
        (!self.wordish_at(end)).then_some(end)
    }

    /// `(?<![-\w])[Uu]\+[?\h]{1,6}(?:(-)\h{1,6})?(?![-\w])`
    pub(in crate::css) fn unicode_range(&mut self, p: usize) -> Option<usize> {
        if self.wordish_before(p) || !matches!(self.at(p), b'U' | b'u') || self.at(p + 1) != b'+' {
            return None;
        }
        let first = p + 2;
        let max = (first..first + 6)
            .take_while(|&i| i < self.lim() && (self.s[i] == b'?' || self.s[i].is_ascii_hexdigit()))
            .count();
        for n in (1..=max).rev() {
            let at = first + n;
            if self.at(at) == b'-' && at < self.lim() {
                let hex = (at + 1..at + 7)
                    .take_while(|&i| i < self.lim() && self.s[i].is_ascii_hexdigit())
                    .count();
                for k in (1..=hex).rev() {
                    let end = at + 1 + k;
                    if !self.wordish_at(end) {
                        self.cap(0, p, end);
                        self.cap(1, at, at + 1);
                        return Some(end);
                    }
                }
            }
            if !self.wordish_at(at) {
                self.cap(0, p, at);
                return Some(at);
            }
        }
        None
    }

    /// `(#)(?:\h{3,4}|\h{6}|\h{8})\b`
    pub(in crate::css) fn numeric_values_at0(&mut self, p: usize) -> Option<usize> {
        if self.s.get(p) != Some(&b'#') {
            return None;
        }
        let end = self.skip(p + 1, |b| b.is_ascii_hexdigit());
        let digits = end - (p + 1);
        (matches!(digits, 3 | 4 | 6 | 8) && self.word_at(end).is_none()).then(|| {
            self.cap(1, p, p + 1);
            end
        })
    }

    /// The number the grammar's units follow:
    /// `(?<![-\w])[-+]?(?:[0-9]+(?:\.[0-9]+)?|\.[0-9]+)(?:(?<=[0-9])E[-+]?
    /// [0-9]+)?`.
    fn number_end(&self, p: usize) -> Option<usize> {
        let (at, end) = self.x.number.get();
        if at == p {
            return end;
        }
        let end = self.number_scan(p);
        self.x.number.set((p, end));
        end
    }

    fn number_scan(&self, p: usize) -> Option<usize> {
        if self.wordish_before(p) {
            return None;
        }
        let sign = usize::from(matches!(self.at(p), b'+' | b'-') && p < self.lim());
        let digits = |i: usize| self.skip(i, |b| b.is_ascii_digit());
        let mut end = p + sign;
        let whole = digits(end);
        if whole > end {
            end = whole;
            if self.at(end) == b'.' && self.at(end + 1).is_ascii_digit() {
                end = digits(end + 1);
            }
        } else if self.at(end) == b'.' && self.at(end + 1).is_ascii_digit() {
            end = digits(end + 1);
        } else {
            return None;
        }
        if matches!(self.at(end), b'E' | b'e') && self.s[end - 1].is_ascii_digit() {
            let exponent = end + 1 + usize::from(matches!(self.at(end + 1), b'+' | b'-'));
            if self.at(exponent).is_ascii_digit() {
                end = digits(exponent);
            }
        }
        Some(end)
    }

    /// The number followed by `unit`, in any case, with a boundary after it.
    pub(in crate::css) fn numeric_unit(&mut self, p: usize, unit: &str) -> Option<usize> {
        let number = self.number_end(p)?;
        let end = self.word_run(number);
        (end > number && self.is_word_ci(number, end, unit)).then(|| {
            self.cap(1, number, end);
            end
        })
    }

    /// The number followed by `%`.
    pub(in crate::css) fn numeric_percent(&mut self, p: usize) -> Option<usize> {
        let number = self.number_end(p)?;
        (self.at(number) == b'%' && number < self.lim()).then(|| {
            self.cap(1, number, number + 1);
            number + 1
        })
    }

    /// The number alone.
    pub(in crate::css) fn numeric_bare(&mut self, p: usize) -> Option<usize> {
        self.number_end(p)
    }

    /// `;`
    pub(in crate::css) fn rule_list_innards_at9(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b";")
    }

    /// `(?<![-A-Za-z])(?=[-A-Za-z])`
    pub(in crate::css) fn rule_list_innards_at7(&mut self, p: usize) -> Option<usize> {
        let letter = |b: u8| b.is_ascii_alphabetic() || b == b'-';
        let behind = p > 0 && self.s.get(p - 1).is_some_and(|&b| letter(b));
        (!behind && self.s.get(p).is_some_and(|&b| letter(b))).then_some(p)
    }

    /// `$|(?![-A-Za-z])`
    pub(in crate::css) fn rule_list_innards_at7_end(&mut self, p: usize) -> Option<usize> {
        let letter = self
            .s
            .get(p)
            .is_some_and(|&b| b.is_ascii_alphabetic() || b == b'-');
        (self.at_end(p) || !letter).then_some(p)
    }

    /// `(:)\s*`
    pub(in crate::css) fn rule_list_innards_at8(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b":")?;
        self.cap(1, p, p + 1);
        Some(self.spaces(p + 1))
    }

    /// `\s*(;)|\s*(?=[)}])`
    pub(in crate::css) fn rule_list_innards_at8_end(&mut self, p: usize) -> Option<usize> {
        let at = self.spaces(p);
        match self.s.get(at) {
            Some(b';') => {
                self.cap(1, at, at + 1);
                Some(at + 1)
            }
            Some(b')' | b'}') => Some(at),
            _ => None,
        }
    }

    /// The lookbehind `(?<=^|(^|[^:])\s|[{}]|\*/|\\(?:\h{1,6}|.))` of the
    /// shared names.
    fn shared_behind(&self, p: usize) -> bool {
        if p == 0 {
            return true;
        }
        let blank = self.space_before(p)
            && self.char_before(p).is_some_and(|c| {
                let start = p - c.len_utf8();
                start == 0 || self.char_before(start) != Some(':')
            });
        blank
            || matches!(self.s.get(p - 1), Some(b'{' | b'}'))
            || (p >= 2 && self.s.get(p - 2..p) == Some(b"*/"))
            || self.escape_before_shared(p)
    }

    /// `\\(?:\h{1,6}|.)` that ends at `p`.
    fn escape_before_shared(&self, p: usize) -> bool {
        self.escape_before(p)
    }

    /// The names that need a check on what follows them: begin the selector
    /// rule when `words` has one at `p` that is not followed by what a
    /// property would be; `pseudo` says what follows a `:` in the second list.
    fn shared_name(&mut self, p: usize, words: &[&str], pseudo: bool) -> Option<usize> {
        if !self.shared_behind(p) {
            return None;
        }
        for word in words {
            let Some(end) = self.lit_ci(p, word) else {
                continue;
            };
            if !self.shared_name_follows(end, pseudo) {
                return Some(p);
            }
        }
        None
    }

    /// Whether `(?!.*;|[^{]*}|-|:\s+)` (or the longer lookahead of the second
    /// list) fails at `end`, that is, one of its alternatives matches.
    fn shared_name_follows(&self, end: usize, pseudo: bool) -> bool {
        if self.find(Find::Semicolon, end).is_some() {
            return true;
        }
        let brace = self.find(Find::Open, end).unwrap_or(self.lim());
        if self
            .find(Find::Close, end)
            .is_some_and(|close| close < brace)
        {
            return true;
        }
        if self.at(end) == b'-' && end < self.lim() {
            return true;
        }
        if self.at(end) != b':' || end >= self.lim() {
            return false;
        }
        if !pseudo {
            return self.space_at(end + 1) > 0;
        }
        !self.after_colon_is_pseudo(end + 1)
    }

    /// The lookahead `(?![^}]*\{|active|...)` after a colon at `p - 1` holds
    /// when none of its alternatives matches: whether one does.
    fn after_colon_is_pseudo(&self, p: usize) -> bool {
        let open = self.find(Find::Open, p);
        if open.is_some_and(|open| self.find(Find::Close, p).is_none_or(|close| close > open)) {
            return true;
        }
        if PSEUDO_PREFIXES
            .iter()
            .any(|word| self.lit_ci(p, word).is_some())
        {
            return true;
        }
        if PSEUDO_FUNCTIONS.iter().any(|word| {
            self.lit_ci(p, word)
                .is_some_and(|end| self.at(end) == b'(' && end < self.lim())
        }) {
            return true;
        }
        self.vendor_prefixed(p, true).is_some()
    }

    /// The first list of names that begin a selector.
    pub(in crate::css) fn shared_names_at0(&mut self, p: usize) -> Option<usize> {
        self.shared_name(
            p,
            &[
                "header",
                "image",
                "label",
                "marquee",
                "mask",
                "nav",
                "ruby",
                "shadow",
                "span",
                "style",
                "color-profile",
                "line",
                "text",
            ],
            false,
        )
    }

    /// The second list of names that begin a selector.
    pub(in crate::css) fn shared_names_at1(&mut self, p: usize) -> Option<usize> {
        self.shared_name(p, &["content", "font", "mark", "cursor", "filter"], true)
    }

    /// `(?i)(?=\s|/\*|[,{]|$)`
    pub(in crate::css) fn shared_names_at0_end(&mut self, p: usize) -> Option<usize> {
        let ahead = self.space_at(p) > 0
            || self.starts(p, b"/*")
            || matches!(self.s.get(p), Some(b',' | b'{'))
            || self.at_end(p);
        ahead.then_some(p)
    }
}

/// The pseudo-classes and pseudo-elements the second list's lookahead lets
/// follow a colon, which it matches as prefixes.
const PSEUDO_PREFIXES: &[&str] = &[
    "active",
    "any-link",
    "checked",
    "disabled",
    "empty",
    "enabled",
    "first",
    "first-child",
    "first-of-type",
    "last-child",
    "last-of-type",
    "only-child",
    "only-of-type",
    "focus",
    "focus-visible",
    "focus-within",
    "fullscreen",
    "host",
    "hover",
    "in-range",
    "indeterminate",
    "invalid",
    "link",
    "out-of-range",
    "read-only",
    "read-write",
    "required",
    "root",
    "scope",
    "target",
    "unresolved",
    "valid",
    "visited",
    "after",
    "before",
    "first-letter",
    "first-line",
];

/// The functional pseudo-classes of the same lookahead, which need a `(`.
const PSEUDO_FUNCTIONS: &[&str] = &[
    "dir",
    "lang",
    "not",
    "has",
    "matches",
    "where",
    "is",
    "nth-child",
    "nth-last-child",
    "nth-of-type",
    "nth-last-of-type",
];
