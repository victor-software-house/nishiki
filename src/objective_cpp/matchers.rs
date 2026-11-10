//! The matchers of the expressions only Objective-C++ has.
//! Each is the code that decides whether its expression matches at a
//! position, where it ends, and the capture groups the grammar names.
//!
//! The grammar's other expressions are Objective-C's, and its matchers answer
//! them (see [`crate::objective_c`]).
//! These come from the grammar's `cpp_lang` entry.
//! Each follows its expression as Oniguruma's backtracking would.

use super::engine::Lexer;
use super::grammar::Rx;
use crate::rules::Chars;

/// The longest delimiter that group 2 of a raw string's begin takes, in
/// characters.
/// A longer one is group 3, and the end names it invalid.
const RAW_DELIMITER_MAX: usize = 16;

/// The capture group, past the groups of the raw string's begin, that holds
/// its delimiter, whichever of groups 2 and 3 took it.
/// The rule's payload repeats it for the end `\)\2(\3)"`.
pub(super) const RAW_DELIMITER: usize = 4;

/// The access words of a base class list: `public`, `protected` and `private`.
const ACCESS: &[&[u8]] = &[b"public", b"protected", b"private"];

impl Lexer<'_> {
    /// Whether `rx`, one of Objective-C++'s own expressions, matches at `p`,
    /// and where its match ends.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm per expression: the table of what each matches"
    )]
    pub(super) fn own_matcher(&mut self, rx: Rx, p: usize) -> Option<usize> {
        match rx {
            Rx::CppSpecialBlockAt0 => self.using_namespace(p),
            Rx::CppSpecialBlockAt1 => {
                let end = self.keyword(p, b"namespace")?;
                let name = self.space_end(end);
                self.capture(1, p, end);
                match self.bounded_ident(name) {
                    Some(stop) => {
                        self.capture(2, name, stop);
                        Some(stop)
                    }
                    None => Some(name),
                }
            }
            Rx::CppSpecialBlockAt1End => {
                if self.prev_is(p, b'}') {
                    return Some(p);
                }
                let closes = matches!(
                    self.at(p),
                    b']' | b'(' | b')' | b',' | b';' | b'=' | b'>' | b'['
                );
                (closes && p < self.lim()).then(|| {
                    self.capture(1, p, p + 1);
                    p
                })
            }
            Rx::CppSpecialBlockAt1At0End => self.lit(p, b"}"),
            Rx::CppConstructorAt1 => {
                if self.at(p) != b':' || p >= self.lim() {
                    return None;
                }
                let name = self.space_end(p + 1);
                let stop = self.scoped_ident_end(name)?;
                self.paren_after(stop)?;
                self.capture(1, p, p + 1);
                Some(p + 1)
            }
            Rx::CppConstructorAt1End => (self.at(p) == b'{' && p < self.lim()).then_some(p),
            Rx::CppSpecialBlockAt2 => self.class_head(p),
            Rx::CppSpecialBlockAt2End => {
                let closes = matches!(self.at(p), b']' | b'(' | b')' | b';' | b'=' | b'>' | b'[');
                (self.prev_is(p, b'}') || (closes && p < self.lim())).then_some(p)
            }
            Rx::CppSpecialBlockAt2Capture7At0 => self.access(p),
            Rx::CppSpecialBlockAt2Capture7At1 => self.ascii_ident_end(p),
            Rx::CppSpecialBlockAt2At1End => {
                let end = self.lit(p, b"}")?;
                self.capture(1, p, end);
                let blanks = self.blank_end(end);
                if blanks == self.lim() && self.at(blanks) == b'\n' {
                    self.capture(2, end, blanks + 1);
                    return Some(blanks + 1);
                }
                Some(end)
            }
            Rx::CppSpecialBlockAt3 => {
                if self.word_before(p) {
                    return None;
                }
                let end = self.lit(p, b"extern")?;
                let quote = self.space_end(end);
                (self.at(quote) == b'"' && quote < self.lim()).then(|| {
                    self.capture(1, p, end);
                    end
                })
            }
            Rx::CppSpecialBlockAt3End => {
                (self.prev_is(p, b'}') || self.word_at(p).is_some() || self.endif_ahead(p))
                    .then_some(p)
            }
            Rx::CppSpecialBlockAt3At0End => match self.lit(p, b"}") {
                Some(end) => Some(end),
                None => self.endif_ahead(p).then_some(p),
            },
            Rx::CppStringsAt0 => match self.encoding(p, b'"') {
                Some(quote) => {
                    self.capture(1, p, quote);
                    Some(quote + 1)
                }
                None => (self.at(p) == b'"' && p < self.lim()).then_some(p + 1),
            },
            Rx::CppStringsAt0At0 => {
                let (digits, count) = match self.lit(p, b"\\u") {
                    Some(at) => (at, 4),
                    None => (self.lit(p, b"\\U")?, 8),
                };
                let end = self
                    .skip(digits, |b| b.is_ascii_hexdigit())
                    .min(digits + count);
                (end == digits + count).then_some(end)
            }
            Rx::CppStringsAt0At1 => {
                let escaped = matches!(
                    self.at(p + 1),
                    b'"' | b'\'' | b'?' | b'\\' | b'a' | b'b' | b'f' | b'n' | b'r' | b't' | b'v'
                );
                (self.at(p) == b'\\' && escaped && p + 1 < self.lim()).then_some(p + 2)
            }
            Rx::CppStringsAt0At2 => {
                let digits = self.lit(p, b"\\")?;
                let end = self
                    .skip(digits, |b| matches!(b, b'0'..=b'7'))
                    .min(digits + 3);
                (end > digits).then_some(end)
            }
            Rx::CppStringsAt0At3 => {
                let digits = self.lit(p, b"\\x")?;
                let end = self.skip(digits, |b| b.is_ascii_hexdigit());
                (end > digits).then_some(end)
            }
            Rx::CppStringsAt1 => self.raw_begin(p),
            Rx::CppStringsAt1End => self.raw_end(p),
            Rx::CppLangAt2 => self.word_in(
                p,
                &[
                    b"friend",
                    b"explicit",
                    b"virtual",
                    b"override",
                    b"final",
                    b"noexcept",
                ],
            ),
            Rx::CppLangAt3 => {
                if self.word_before(p) {
                    return None;
                }
                [&b"private:"[..], b"protected:", b"public:"]
                    .iter()
                    .find_map(|word| self.lit(p, word))
            }
            Rx::CppLangAt4 => self.word_in(p, &[b"catch", b"try", b"throw", b"using"]),
            Rx::CppLangAt5 => {
                if let Some(end) = self.keyword(p, b"delete") {
                    let brackets = self.space_end(end);
                    return Some(self.lit(brackets, b"[]").unwrap_or(end));
                }
                let end = self.keyword(p, b"new")?;
                (self.at(end) != b']' || end >= self.lim()).then_some(end)
            }
            Rx::CppLangAt6 => self
                .prefixed_name(p, b'f')
                .or_else(|| self.prefixed_name(p, b'm')),
            Rx::CppLangAt7 => self.keyword(p, b"this"),
            Rx::CppLangAt8 => self.keyword(p, b"nullptr"),
            Rx::CppTemplateDefinition => {
                if self.word_before(p) {
                    return None;
                }
                let end = self.lit(p, b"template")?;
                let open = self.space_end(end);
                (self.at(open) == b'<' && open < self.lim()).then(|| {
                    self.capture(1, p, end);
                    self.capture(2, open, open + 1);
                    self.space_end(open + 1)
                })
            }
            Rx::CppTemplateDefinitionArgument => self.template_argument(p),
            Rx::CppLangAt10 => {
                let end = self.keyword(p, b"template")?;
                Some(self.space_end(end))
            }
            Rx::CppLangAt11 => {
                let end = self.word_in(
                    p,
                    &[
                        b"const_cast",
                        b"dynamic_cast",
                        b"reinterpret_cast",
                        b"static_cast",
                    ],
                )?;
                Some(self.space_end(end))
            }
            Rx::CppLangAt12 => self.scope_resolution(p),
            Rx::CppLangAt13 => self.word_in(
                p,
                &[
                    b"and", b"and_eq", b"bitand", b"bitor", b"compl", b"not", b"not_eq", b"or",
                    b"or_eq", b"typeid", b"xor", b"xor_eq", b"alignof", b"alignas",
                ],
            ),
            Rx::CppLangAt14 => {
                self.word_in(p, &[b"decltype", b"wchar_t", b"char16_t", b"char32_t"])
            }
            Rx::CppLangAt15 => self.word_in(
                p,
                &[
                    b"constexpr",
                    b"export",
                    b"mutable",
                    b"typename",
                    b"thread_local",
                ],
            ),
            Rx::CppLangAt16 => self.destructor(p),
        }
    }

    /// `[A-Z_a-z][0-9A-Z_a-z]*\b` at `i`: where it ends.
    /// After the longest run, `\b` fails only before a non-ASCII letter, and
    /// a shorter run ends between two word characters, so it fails there too.
    fn bounded_ident(&self, i: usize) -> Option<usize> {
        self.ascii_ident_end(i).filter(|&end| self.boundary(end))
    }

    /// `[A-Z_a-z][0-:A-Z_a-z]*` at `i`, a name that may hold `:`: where it
    /// ends.
    fn scoped_ident_end(&self, i: usize) -> Option<usize> {
        let b = self.at(i);
        ((b == b'_' || b.is_ascii_alphabetic()) && i < self.lim()).then(|| {
            self.skip(i + 1, |c| {
                c == b'_' || c == b':' || c.is_ascii_alphanumeric()
            })
        })
    }

    /// `\s*(\()` at `i`: where the `(` stands.
    fn paren_after(&self, i: usize) -> Option<usize> {
        let open = self.space_end(i);
        (self.at(open) == b'(' && open < self.lim()).then_some(open)
    }

    /// One of the [`ACCESS`] words at `i`, with no `\b` after it: where it
    /// ends.
    fn access(&self, i: usize) -> Option<usize> {
        ACCESS.iter().find_map(|word| self.lit(i, word))
    }

    /// `(?=\s*#\s*endif\b)` at `i`.
    fn endif_ahead(&self, i: usize) -> bool {
        let hash = self.space_end(i);
        if self.at(hash) != b'#' || hash >= self.lim() {
            return false;
        }
        let word = self.space_end(hash + 1);
        self.lit(word, b"endif")
            .is_some_and(|end| self.boundary(end))
    }

    /// `(u8??|[LU])?` and then `next` at `p`, with the prefix there: where
    /// `next` stands.
    /// `u8??` takes the `8` only when `next` does not follow the `u`.
    fn encoding(&self, p: usize, next: u8) -> Option<usize> {
        match self.at(p) {
            b'u' | b'L' | b'U' if self.at(p + 1) == next => Some(p + 1),
            b'u' if self.at(p + 1) == b'8' && self.at(p + 2) == next => Some(p + 2),
            _ => None,
        }
        .filter(|&at| at < self.lim())
    }

    /// `CppSpecialBlockAt0`:
    /// `\b(using)\b\s*(namespace)\b\s*((?:[A-Z_a-z][0-9A-Z_a-z]*\b(::)?)*)`.
    fn using_namespace(&mut self, p: usize) -> Option<usize> {
        let using = self.keyword(p, b"using")?;
        let word = self.space_end(using);
        let namespace = self.keyword(word, b"namespace")?;
        let names = self.space_end(namespace);
        let mut end = names;
        while let Some(stop) = self.bounded_ident(end) {
            end = self.lit(stop, b"::").unwrap_or(stop);
        }
        self.capture(1, p, using);
        self.capture(2, word, namespace);
        self.capture(3, names, end);
        Some(end)
    }

    /// `CppSpecialBlockAt2`: `\b(?:(class)|(struct))\b\s*(name\b)?+`, and
    /// then a base list `(\s*:\s*(access)\s*(name)\b((\s*,\s*(access)\s*
    /// name\b)*))?`.
    fn class_head(&mut self, p: usize) -> Option<usize> {
        let (group, word) = match self.keyword(p, b"class") {
            Some(end) => (1, end),
            None => (2, self.keyword(p, b"struct")?),
        };
        self.capture(group, p, word);
        let name = self.space_end(word);
        let mut end = name;
        if let Some(stop) = self.bounded_ident(name) {
            self.capture(3, name, stop);
            end = stop;
        }
        let Some([access, access_end, base, list]) = self.base_head(end) else {
            return Some(end);
        };
        let mut stop = list;
        while let Some(next) = self.base_more(stop) {
            stop = next;
        }
        self.capture(5, access, access_end);
        self.capture(6, base, list);
        self.capture(7, list, stop);
        Some(stop)
    }

    /// `\s*:\s*(access)\s*(name)\b` at `i`: where the access word starts and
    /// ends, then where the name starts and ends.
    fn base_head(&self, i: usize) -> Option<[usize; 4]> {
        let colon = self.space_end(i);
        if self.at(colon) != b':' || colon >= self.lim() {
            return None;
        }
        let access = self.space_end(colon + 1);
        let access_end = self.access(access)?;
        let name = self.space_end(access_end);
        let stop = self.bounded_ident(name)?;
        Some([access, access_end, name, stop])
    }

    /// One more base `\s*,\s*(access)\s*name\b` at `i`: where it ends.
    fn base_more(&self, i: usize) -> Option<usize> {
        let comma = self.space_end(i);
        if self.at(comma) != b',' || comma >= self.lim() {
            return None;
        }
        let access = self.space_end(comma + 1);
        let access_end = self.access(access)?;
        self.bounded_ident(self.space_end(access_end))
    }

    /// `CppStringsAt1`: `(u8??|[LU])?R"(?:([^\t ()\\]{0,16})|([^\t ()\\]*))\(`.
    /// The delimiter is also recorded as group [`RAW_DELIMITER`], which the
    /// rule keeps for its end.
    fn raw_begin(&mut self, p: usize) -> Option<usize> {
        let r = match self.encoding(p, b'R') {
            Some(r) => {
                self.capture(1, p, r);
                r
            }
            None => p,
        };
        if self.at(r) != b'R' || self.at(r + 1) != b'"' || r + 1 >= self.lim() {
            return None;
        }
        let from = r + 2;
        let mut end = from;
        let mut chars = 0;
        while end < self.lim() && !matches!(self.at(end), b'\t' | b' ' | b'(' | b')' | b'\\') {
            end = self.char_end(end);
            chars += 1;
        }
        self.spend(end - from);
        if self.at(end) != b'(' || end >= self.lim() {
            return None;
        }
        let group = if chars <= RAW_DELIMITER_MAX { 2 } else { 3 };
        self.capture(group, from, end);
        self.capture(RAW_DELIMITER, from, end);
        Some(end + 1)
    }

    /// `CppStringsAt1End`: `\)\2(\3)"`, where the rule's payload is the
    /// delimiter its begin took.
    /// A delimiter past [`RAW_DELIMITER_MAX`] characters was group 3, which
    /// group 1 repeats.
    fn raw_end(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b')' || p >= self.lim() {
            return None;
        }
        let delimiter = self.payload_text();
        let long = delimiter.chars().count() > RAW_DELIMITER_MAX;
        let stop = self.lit(p + 1, delimiter.as_bytes())?;
        if self.at(stop) != b'"' || stop >= self.lim() {
            return None;
        }
        if long {
            self.capture(1, p + 1, stop);
        } else {
            self.capture(1, stop, stop);
        }
        Some(stop + 1)
    }

    /// `CppTemplateDefinitionArgument`: blanks, then one of four shapes of a
    /// template parameter, then `(,|(?=>))` as group 11.
    /// The shapes, in the order Oniguruma tries them:
    /// - `(name\s*)`;
    /// - `((?:name\s+)*)(name)`;
    /// - `(name)\s*(\.\.\.)\s*(name)`;
    /// - `((?:name\s+)*)(name)\s*(=)\s*(\w+)`.
    fn template_argument(&mut self, p: usize) -> Option<usize> {
        let start = self.space_end(p);
        let first = self.ascii_ident_end(start)?;
        let blanks = self.space_end(first);
        if let Some(end) = self.argument_end(blanks) {
            self.capture(1, start, blanks);
            return Some(end);
        }
        let (last, stop) = self.last_of_names(start, first);
        if let Some(end) = self.argument_end(stop) {
            self.capture(2, start, last);
            self.capture(3, last, stop);
            return Some(end);
        }
        if let Some(dots) = self.lit(blanks, b"...") {
            let name = self.space_end(dots);
            if let Some(name_end) = self.ascii_ident_end(name)
                && let Some(end) = self.argument_end(name_end)
            {
                self.capture(4, start, first);
                self.capture(5, blanks, dots);
                self.capture(6, name, name_end);
                return Some(end);
            }
        }
        let equals = self.space_end(stop);
        if self.at(equals) != b'=' || equals >= self.lim() {
            return None;
        }
        let value = self.space_end(equals + 1);
        let value_end = self.word_end(value);
        if value_end == value {
            return None;
        }
        let end = self.argument_end(value_end)?;
        self.capture(7, start, last);
        self.capture(8, last, stop);
        self.capture(9, equals, equals + 1);
        self.capture(10, value, value_end);
        Some(end)
    }

    /// The names from `start`, whose first ends at `first`, each after
    /// blanks: where the last one starts and ends.
    /// `(?:name\s+)*(name)` ends at that name, whatever follows it.
    fn last_of_names(&self, start: usize, first: usize) -> (usize, usize) {
        let (mut last, mut stop) = (start, first);
        loop {
            let next = self.space_end(stop);
            if next == stop {
                return (last, stop);
            }
            match self.ascii_ident_end(next) {
                Some(end) => (last, stop) = (next, end),
                None => return (last, stop),
            }
        }
    }

    /// `(,|(?=>))` at `i`, recorded as group 11: where the match ends.
    fn argument_end(&mut self, i: usize) -> Option<usize> {
        if i >= self.lim() {
            return None;
        }
        match self.at(i) {
            b',' => {
                self.capture(11, i, i + 1);
                Some(i + 1)
            }
            b'>' => {
                self.capture(11, i, i);
                Some(i)
            }
            _ => None,
        }
    }

    /// `CppLangAt12`: `((?:name::)*)(name)(::)`, which gives back names until
    /// one is followed by `::`: the last name of the chain before its `::`.
    fn scope_resolution(&mut self, p: usize) -> Option<usize> {
        let mut at = p;
        let mut last = None;
        while let Some(stop) = self.ascii_ident_end(at)
            && let Some(after) = self.lit(stop, b"::")
        {
            last = Some((at, stop));
            at = after;
        }
        let (name, stop) = last?;
        self.capture(1, p, name);
        self.capture(2, name, stop);
        self.capture(3, stop, stop + 2);
        Some(stop + 2)
    }

    /// `CppLangAt16`: `(?:^|(?<!else|new|=))((?:name::)*+~name)\s*(\()`.
    fn destructor(&mut self, p: usize) -> Option<usize> {
        let after_word = |word: &[u8]| p >= word.len() && self.text(p - word.len(), p) == word;
        if p != 0 && (after_word(b"else") || after_word(b"new") || after_word(b"=")) {
            return None;
        }
        let mut at = p;
        while let Some(stop) = self.ascii_ident_end(at)
            && let Some(after) = self.lit(stop, b"::")
        {
            at = after;
        }
        let tilde = self.lit(at, b"~")?;
        let stop = self.ascii_ident_end(tilde)?;
        let open = self.paren_after(stop)?;
        self.capture(1, p, stop);
        self.capture(2, open, open + 1);
        Some(open + 1)
    }
}
