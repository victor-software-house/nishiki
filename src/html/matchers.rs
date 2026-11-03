//! Where the grammar's patterns match: byte-level matchers for each rule's
//! end and patterns, in the grammar's order, and the candidate positions
//! the search visits.

use memchr::{memchr, memchr2};

use super::engine::{Hit, Lexer, Pat};
use super::state::{
    AttributeKind, Category, ElementInfo, Flavor, Namespace, Rule, TagInfo, TagKind,
};
use super::words;
use crate::classes::{is_space, is_word};

/// How a tag's category is told apart from the list that names it.
#[derive(Clone, Copy)]
enum Deprecation {
    No,
    Deprecated,
    Unsupported,
}

impl Deprecation {
    const fn bits(self) -> u32 {
        match self {
            Deprecation::No => 0,
            Deprecation::Deprecated => 1,
            Deprecation::Unsupported => 2,
        }
    }
}

/// The longest tag name the lists hold, with room to spare.
const MAX_NAME: usize = 32;

/// The attribute lists of a namespace.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Attributes {
    Html,
    Svg,
    Math,
}

fn contains(list: &[&str], word: &str) -> bool {
    list.binary_search(&word).is_ok()
}

/// `word`, lower-cased into `buffer`, when it fits.
fn lowered<'b>(word: &[u8], buffer: &'b mut [u8; MAX_NAME]) -> Option<&'b str> {
    let slot = buffer.get_mut(..word.len())?;
    slot.copy_from_slice(word);
    slot.make_ascii_lowercase();
    std::str::from_utf8(slot).ok()
}

impl Lexer<'_> {
    // Characters.

    /// The length of the white space (`\s`) that stands at `i`: the line
    /// feed at the end of a line counts, and 0 means none.
    pub(super) fn ws_len(&self, i: usize) -> usize {
        if i >= self.lim {
            return usize::from(i == self.lim && self.lf);
        }
        let b = self.s[i];
        if b < 0x80 {
            return usize::from(crate::classes::is_ascii_space(b));
        }
        match self.text.get(i..self.lim).and_then(|t| t.chars().next()) {
            Some(c) if is_space(c) => c.len_utf8(),
            _ => 0,
        }
    }

    /// The end of the ASCII white space that starts at `i`: the blanks no
    /// attribute pattern starts at, since an unrecognized attribute's name may
    /// begin with any other character, U+00A0 included.
    pub(super) fn skip_ascii_ws(&self, mut i: usize) -> usize {
        let from = i;
        while i < self.lim && crate::classes::is_ascii_space(self.s[i]) {
            i += 1;
        }
        self.spend((i - from) / 8 + 1);
        i
    }

    /// The end of the white space that starts at `i`.
    pub(super) fn skip_ws(&self, mut i: usize) -> usize {
        let from = i;
        loop {
            let n = self.ws_len(i);
            if n == 0 {
                break;
            }
            i += n;
        }
        self.spend((i - from) / 8 + 1);
        i
    }

    /// The character at `i` and its length.
    pub(super) fn char_at(&self, i: usize) -> Option<(char, usize)> {
        let c = self.text.get(i..self.lim)?.chars().next()?;
        Some((c, c.len_utf8()))
    }

    /// The character that ends at `i`.
    fn char_before(&self, i: usize) -> Option<char> {
        let mut start = i.checked_sub(1)?;
        while start > 0 && self.s.get(start).is_some_and(|b| b & 0xc0 == 0x80) {
            start -= 1;
        }
        self.text.get(start..i)?.chars().next()
    }

    /// The length of the `\w` character at `i`, or `None`.
    fn word_len(&self, i: usize) -> Option<usize> {
        let (c, n) = self.char_at(i)?;
        is_word(c).then_some(n)
    }

    /// Whether the text at `i` starts with `literal`.
    pub(super) fn starts(&self, i: usize, literal: &[u8]) -> bool {
        self.s
            .get(i..self.lim)
            .is_some_and(|rest| rest.starts_with(literal))
    }

    /// Whether the text at `i` starts with `literal`, which is lower case,
    /// without regard to case.
    pub(super) fn starts_ci(&self, i: usize, literal: &[u8]) -> bool {
        self.s.get(i..self.lim).is_some_and(|rest| {
            rest.len() >= literal.len() && rest[..literal.len()].eq_ignore_ascii_case(literal)
        })
    }

    /// Whether `\G` matches at `p`.
    pub(super) fn g(&self, p: usize) -> bool {
        self.anchor == Some(p)
    }

    // Candidates.

    /// The first position at or after `p` where a pattern or the end of
    /// `rule` can match, or `None` when there is none before `cut`, which is
    /// as far as the search needs to look: the injection comes there.
    /// In a tag, ` />` ends it, so a blank before `/>` is a place, and one
    /// match decides there: the end, the `=`, or no match this line. Inside a
    /// run of blanks nothing can match, because the end needs a character
    /// before it that is not blank.
    pub(super) fn candidate(&self, rule: Rule, p: usize, cut: usize) -> Option<usize> {
        let rest = self.s.get(p..cut.min(self.lim)).unwrap_or(&[]);
        let find = |found: Option<usize>| {
            self.spend(found.unwrap_or(rest.len()) / 16);
            found.map(|at| at + p)
        };
        let anchor = self.anchor.filter(|&at| at >= p);
        let earliest = |found: Option<usize>| match (found, anchor) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (a, b) => a.or(b),
        };
        let quote = u8::try_from(self.top().payload).unwrap_or(b'"');
        match rule {
            Rule::Root | Rule::BodyTemplate => {
                if p == 0 && matches!(self.at(0), b' ' | b'\t') {
                    return Some(0);
                }
                find(memchr2(b'<', b'&', rest))
            }
            Rule::BodyOther => find(memchr(b'<', rest)),
            Rule::Comment => earliest(find(memchr2(b'-', b'<', rest))),
            Rule::Cdata => find(memchr(b']', rest)),
            Rule::DoctypeString => find(memchr(b'"', rest)),
            Rule::Doctype => {
                let q = self.skip_ws(p);
                (q <= self.end_pos()).then_some(q)
            }
            Rule::Processing
            | Rule::OpenStyle
            | Rule::OpenScript
            | Rule::ScriptAttributes
            | Rule::ElementStart
            | Rule::Region => {
                let q = self.skip_ascii_ws(p);
                (q <= self.end_pos()).then_some(q)
            }
            Rule::Tag => {
                let mut q = p;
                if TagInfo::from_bits(super::state::info(self.top().payload)).kind
                    == TagKind::Unrecognized
                {
                    while q < self.lim
                        && crate::classes::is_ascii_space(self.s[q])
                        && !(self.s[q] == b' ' && self.starts(q + 1, b"/>"))
                    {
                        q += 1;
                    }
                    return (q <= self.end_pos()).then_some(q);
                }
                let q = self.skip_ascii_ws(p);
                (q <= self.end_pos()).then_some(q)
            }
            Rule::Attribute => (self.skip_ws(p) < self.lim).then_some(p),
            Rule::Value | Rule::ValueCss | Rule::ValueJs => {
                let blank_before = self.char_before(p).is_some_and(is_space);
                let q = if p > 0 && blank_before && self.ws_len(p) > 0 {
                    self.skip_ws(p)
                } else {
                    p
                };
                (q <= self.end_pos()).then_some(q)
            }
            Rule::StyleWrap
            | Rule::ScriptWrap
            | Rule::StyleTag
            | Rule::ScriptHead
            | Rule::ScriptTemplateOuter
            | Rule::ScriptOtherOuter
            | Rule::WrapCss
            | Rule::WrapJs
            | Rule::StringJs
            | Rule::BodyCss
            | Rule::BodyJs => Some(p),
            Rule::ScriptTag => earliest(find(memchr(b'/', rest))),
            Rule::ScriptJsOuter => earliest(find(memchr(b'<', rest))),
            Rule::Element => earliest(find(memchr2(b'<', b'/', rest))),
            Rule::StringPlain | Rule::StringCss => find(memchr2(quote, b'&', rest)),
            Rule::JsLineComment => {
                let found = find(memchr(quote, rest));
                match (found, self.lf) {
                    (Some(at), true) => Some(at.min(self.lim)),
                    (Some(at), false) => Some(at),
                    (None, true) => Some(self.lim),
                    (None, false) => None,
                }
            }
            Rule::JsBlockComment => find(memchr2(quote, b'*', rest)),
        }
    }

    /// The first of `rule`'s end and patterns that matches at `p`.
    pub(super) fn match_at(&self, rule: Rule, p: usize) -> Option<Hit> {
        self.end_at(rule, p).or_else(|| self.patterns_at(rule, p))
    }

    // Ends.

    fn zero(p: usize) -> Hit {
        Hit::new(Pat::End, p, p)
    }

    /// The end pattern of `rule` at `p`.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm per rule, in the order of the grammar"
    )]
    fn end_at(&self, rule: Rule, p: usize) -> Option<Hit> {
        let b = self.at(p);
        let one = |len: usize| Hit::new(Pat::End, p, p + len);
        match rule {
            Rule::Root | Rule::BodyCss | Rule::BodyJs | Rule::Region => None,
            Rule::Comment => self.starts(p, b"-->").then(|| one(3)),
            Rule::Cdata => self.starts(p, b"]]>").then(|| one(3)),
            Rule::Doctype | Rule::OpenStyle | Rule::OpenScript => (b == b'>').then(|| one(1)),
            Rule::DoctypeString => (b == b'"').then(|| one(1)),
            Rule::Processing => self.starts(p, b"?>").then(|| one(2)),
            Rule::Tag => {
                let kind = TagInfo::from_bits(super::state::info(self.top().payload)).kind;
                match kind {
                    TagKind::Start | TagKind::End | TagKind::CustomEnd => {
                        (b == b'>').then(|| one(1))
                    }
                    TagKind::Void | TagKind::CustomStart => {
                        if b == b'>' {
                            Some(one(1))
                        } else {
                            self.starts(p, b"/>").then(|| one(2))
                        }
                    }
                    TagKind::Unrecognized => {
                        if b == b'>' {
                            Some(one(1))
                        } else if self.starts(p, b"/>") {
                            Some(one(2))
                        } else {
                            self.starts(p, b" />").then(|| one(3))
                        }
                    }
                }
            }
            Rule::Element => self.element_end(p),
            Rule::ElementStart => {
                let root = ElementInfo::from_bits(super::state::info(self.top().payload)).root;
                if b == b'>' {
                    Some(one(1))
                } else if !root && self.starts(p, b"/>") {
                    Some(Self::zero(p))
                } else {
                    None
                }
            }
            Rule::StyleWrap | Rule::ScriptWrap => {
                if self.g(p) {
                    return None;
                }
                let mut q = p;
                while matches!(self.at(q), b' ' | b'\t') {
                    q += 1;
                }
                self.spend((q - p) / 8);
                if q == self.lim {
                    Some(Hit::new(Pat::End, p, self.end_pos()))
                } else {
                    Some(Self::zero(p))
                }
            }
            Rule::StyleTag => {
                if !self.starts_ci(p, b"</style") {
                    return None;
                }
                let gt = self.skip_ws(p + 7);
                (self.at(gt) == b'>')
                    .then(|| Hit::new(Pat::End, p, gt + 1).with([p + 2, p + 7, gt, 0, 0], 0))
            }
            Rule::ScriptTag => {
                (self.starts_ci(p, b"/script") && self.at(p + 7) == b'>').then(|| one(8))
            }
            Rule::ScriptHead => (b == b'/').then(|| Self::zero(p)),
            Rule::ScriptAttributes => {
                (b == b'>' || self.script_type_ends(p)).then(|| Self::zero(p))
            }
            Rule::ScriptJsOuter | Rule::ScriptTemplateOuter | Rule::ScriptOtherOuter => {
                (b == b'<' && self.starts_ci(p + 1, b"/script")).then(|| one(1))
            }
            Rule::BodyTemplate | Rule::BodyOther => {
                self.starts_ci(p, b"</script").then(|| Self::zero(p))
            }
            Rule::Attribute => {
                let q = self.skip_ws(p);
                (q < self.lim && self.at(q) != b'=').then(|| Self::zero(p))
            }
            Rule::Value | Rule::ValueCss | Rule::ValueJs => self.value_end(p),
            Rule::WrapCss | Rule::WrapJs => (!self.g(p)).then(|| Self::zero(p)),
            Rule::StringPlain | Rule::StringCss | Rule::StringJs => {
                (u32::from(b) == self.top().payload).then(|| one(1))
            }
            Rule::JsLineComment => {
                if u32::from(b) == self.top().payload {
                    Some(Self::zero(p))
                } else {
                    (p == self.lim && self.lf).then(|| one(1))
                }
            }
            Rule::JsBlockComment => {
                if u32::from(b) == self.top().payload {
                    Some(Self::zero(p))
                } else {
                    self.starts(p, b"*/").then(|| one(2))
                }
            }
        }
    }

    /// `(?<=[^=\s])(?!\s*=)|(?=/?>)`: the end of an attribute's value.
    fn value_end(&self, p: usize) -> Option<Hit> {
        let b = self.at(p);
        if b == b'>' || self.starts(p, b"/>") {
            return Some(Self::zero(p));
        }
        let before = self.char_before(p)?;
        if before == '=' || is_space(before) {
            return None;
        }
        let q = self.skip_ws(p);
        (self.at(q) != b'=').then(|| Self::zero(p))
    }

    /// `(?i)(</)(\2)\s*(>)|(/>)|(?=</\w+)`: the end of an element.
    fn element_end(&self, p: usize) -> Option<Hit> {
        let top = self.top();
        let element = ElementInfo::from_bits(super::state::info(top.payload));
        if self.at(p) == b'/' {
            return (!element.root && self.starts(p, b"/>"))
                .then(|| Hit::new(Pat::End, p, p + 2).with([0; 5], 1));
        }
        if !self.starts(p, b"</") {
            return None;
        }
        let name_start = p + 2;
        let word = self.word(super::state::name(top.payload)).as_bytes();
        let name_end = name_start + word.len();
        if self
            .s
            .get(name_start..name_end)
            .is_some_and(|text| text.eq_ignore_ascii_case(word))
            && name_end <= self.lim
        {
            let gt = self.skip_ws(name_end);
            if self.at(gt) == b'>' {
                return Some(
                    Hit::new(Pat::End, p, gt + 1).with([name_start, name_end, gt, 0, 0], 0),
                );
            }
        }
        (!element.root && self.word_len(name_start).is_some())
            .then(|| Hit::new(Pat::End, p, p).with([0; 5], 2))
    }

    // Patterns.

    /// The patterns of `rule` at `p`, in the grammar's order.
    /// The rule of the tags the grammar does not know is compiled first from
    /// inside the `math` rules, so its `#attribute` is the `math` list wherever
    /// the rule is used.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm per rule, in the order of the grammar"
    )]
    fn patterns_at(&self, rule: Rule, p: usize) -> Option<Hit> {
        let b = self.at(p);
        match rule {
            Rule::Root | Rule::BodyTemplate => self.root_patterns(p),
            Rule::Comment => self.comment_patterns(p),
            Rule::Doctype => {
                if self.g(p) && self.starts_ci(p, b"doctype") {
                    return Some(Hit::new(Pat::DoctypeKeyword, p, p + 7));
                }
                if b == b'"' {
                    return Some(Hit::new(Pat::DoctypeQuote, p, p + 1));
                }
                if p < self.lim && self.ws_len(p) == 0 && b != b'>' {
                    let mut q = p;
                    while q < self.lim && self.ws_len(q) == 0 && self.at(q) != b'>' {
                        q += crate::utf8::char_len(self.at(q));
                    }
                    self.spend((q - p) / 8);
                    return Some(Hit::new(Pat::DoctypeWord, p, q.min(self.lim)));
                }
                None
            }
            Rule::Processing | Rule::OpenStyle | Rule::OpenScript | Rule::ScriptAttributes => {
                self.attribute_patterns(p, Attributes::Html)
            }
            Rule::Tag => {
                let kind = TagInfo::from_bits(super::state::info(self.top().payload)).kind;
                let which = if kind == TagKind::Unrecognized {
                    Attributes::Math
                } else {
                    Attributes::Html
                };
                self.attribute_patterns(p, which)
            }
            Rule::Region => {
                let which = match self.top().payload {
                    2 => Attributes::Math,
                    _ => Attributes::Svg,
                };
                self.attribute_patterns(p, which)
            }
            Rule::ElementStart => {
                let element = ElementInfo::from_bits(super::state::info(self.top().payload));
                let which = match element.namespace {
                    Namespace::Svg => Attributes::Svg,
                    Namespace::Math => Attributes::Math,
                };
                self.attribute_patterns(p, which)
            }
            Rule::StyleWrap => {
                (self.g(p) && self.style_open(p)).then(|| Hit::new(Pat::StyleTag, p, p + 6))
            }
            Rule::ScriptWrap => (self.g(p) && b == b'<' && self.starts_ci(p + 1, b"script"))
                .then(|| Hit::new(Pat::ScriptTag, p, p + 7)),
            Rule::StyleTag => Some(Hit::new(
                Pat::Zero(if self.g(p) {
                    Rule::OpenStyle
                } else {
                    Rule::BodyCss
                }),
                p,
                p,
            )),
            Rule::ScriptTag => self
                .g(p)
                .then(|| Hit::new(Pat::Zero(Rule::ScriptHead), p, p)),
            Rule::ScriptHead => {
                if b == b'>' {
                    return Some(Hit::new(Pat::ScriptJs, p, p + 1));
                }
                if self.g(p) {
                    return Some(Hit::new(Pat::Zero(Rule::ScriptAttributes), p, p));
                }
                if self.script_template_ahead(p) {
                    return Some(Hit::new(Pat::Zero(Rule::ScriptTemplateOuter), p, p));
                }
                self.starts_ci(p, b"type")
                    .then(|| Hit::new(Pat::Zero(Rule::ScriptOtherOuter), p, p))
            }
            Rule::ScriptJsOuter => self.g(p).then(|| Hit::new(Pat::Zero(Rule::BodyJs), p, p)),
            Rule::ScriptTemplateOuter => Some(Hit::new(
                Pat::Zero(if self.g(p) {
                    Rule::OpenScript
                } else {
                    Rule::BodyTemplate
                }),
                p,
                p,
            )),
            Rule::ScriptOtherOuter => Some(Hit::new(
                Pat::Zero(if self.g(p) {
                    Rule::OpenScript
                } else {
                    Rule::BodyOther
                }),
                p,
                p,
            )),
            Rule::Attribute => {
                let q = self.skip_ws(p);
                (q < self.lim && self.at(q) == b'=').then(|| Hit::new(Pat::Equals, q, q + 1))
            }
            Rule::Value => {
                if let Some(end) = self.unquoted_end(p) {
                    return Some(Hit::new(Pat::Unquoted, p, end));
                }
                if matches!(b, b'"' | b'\'') {
                    return Some(Hit::new(Pat::Quote, p, p + 1).with([0; 5], u32::from(b)));
                }
                (b == b'=').then(|| Hit::new(Pat::EqualsIllegal, p, p + 1))
            }
            Rule::ValueCss | Rule::ValueJs => {
                if self.value_starts(p) {
                    let wrap = if rule == Rule::ValueCss {
                        Rule::WrapCss
                    } else {
                        Rule::WrapJs
                    };
                    return Some(Hit::new(Pat::Zero(wrap), p, p));
                }
                (b == b'=').then(|| Hit::new(Pat::EqualsIllegal, p, p + 1))
            }
            Rule::WrapCss | Rule::WrapJs => {
                if let Some(end) = self.unquoted_end(p) {
                    let pat = if rule == Rule::WrapCss {
                        Pat::UnquotedCss
                    } else {
                        Pat::UnquotedJs
                    };
                    return Some(Hit::new(pat, p, end));
                }
                matches!(b, b'"' | b'\'')
                    .then(|| Hit::new(Pat::Quote, p, p + 1).with([0; 5], u32::from(b)))
            }
            Rule::StringPlain | Rule::StringCss => self.entity(p),
            Rule::StringJs => self.js_string_patterns(p),
            Rule::Element => self.element_patterns(p),
            Rule::Cdata
            | Rule::DoctypeString
            | Rule::JsLineComment
            | Rule::JsBlockComment
            | Rule::BodyOther
            | Rule::BodyCss
            | Rule::BodyJs => None,
        }
    }

    /// `[^"'/<=>\x60\s]|/(?!>)` as the first character of a value.
    fn value_starts(&self, p: usize) -> bool {
        if p >= self.lim {
            return false;
        }
        match self.at(p) {
            b'/' => self.at(p + 1) != b'>',
            b'<' | b'=' | b'>' | b'`' => false,
            _ => self.ws_len(p) == 0,
        }
    }

    /// The end of `([^"'/<=>\x60\s]|/(?!>))+` at `p`.
    fn unquoted_end(&self, p: usize) -> Option<usize> {
        let mut q = p;
        loop {
            if q >= self.lim {
                break;
            }
            match self.s[q] {
                b'/' if self.at(q + 1) != b'>' => q += 1,
                b'"' | b'\'' | b'/' | b'<' | b'=' | b'>' | b'`' => break,
                0x80.. => {
                    if self.ws_len(q) > 0 {
                        break;
                    }
                    q += crate::utf8::char_len(self.s[q]);
                }
                b if crate::classes::is_ascii_space(b) => break,
                _ => q += 1,
            }
        }
        self.spend((q - p) / 8);
        (q > p).then_some(q.min(self.lim))
    }

    /// The patterns of a quoted value of an event handler.
    /// A run of text is `([^\n"/]|/(?![*/]))+`.
    fn js_string_patterns(&self, p: usize) -> Option<Hit> {
        let quote = u8::try_from(self.top().payload).ok()?;
        let mut q = p;
        while q < self.lim {
            let b = self.s[q];
            if b == quote {
                break;
            }
            if b == b'/' && matches!(self.at(q + 1), b'*' | b'/') {
                break;
            }
            q += crate::utf8::char_len(b);
        }
        self.spend((q - p) / 8);
        if q > p {
            return Some(Hit::new(Pat::JsText, p, q.min(self.lim)));
        }
        if self.starts(p, b"//") {
            return Some(Hit::new(Pat::JsLine, p, p + 2));
        }
        self.starts(p, b"/*")
            .then(|| Hit::new(Pat::JsBlock, p, p + 2))
    }

    /// The patterns of a comment.
    fn comment_patterns(&self, p: usize) -> Option<Hit> {
        let illegal = |len: usize| Hit::new(Pat::CommentIllegal, p, p + len);
        if self.g(p) {
            if self.at(p) == b'>' {
                return Some(illegal(1));
            }
            if self.starts(p, b"->") {
                return Some(illegal(2));
            }
        }
        if self.starts(p, b"<!--") && self.at(p + 4) != b'>' {
            return Some(illegal(4));
        }
        self.starts(p, b"--!>").then(|| illegal(4))
    }

    // Entities.

    /// The entity patterns at `p`.
    pub(super) fn entity(&self, p: usize) -> Option<Hit> {
        if self.at(p) != b'&' {
            return None;
        }
        let next = self.at(p + 1);
        if next == b'#' {
            let hex = matches!(self.at(p + 2), b'x' | b'X');
            let digits = p + 2 + usize::from(hex);
            let mut q = digits;
            while self.at(q).is_ascii_digit() || (hex && self.at(q).is_ascii_hexdigit()) {
                q += 1;
            }
            self.spend((q - digits) / 8);
            if q > digits && self.at(q) == b';' {
                let pat = if hex {
                    Pat::EntityHexadecimal
                } else {
                    Pat::EntityDecimal
                };
                return Some(Hit::new(pat, p, q + 1));
            }
            return None;
        }
        let mut q = p + 1;
        while self.at(q).is_ascii_alphanumeric() {
            q += 1;
        }
        self.spend((q - p) / 8);
        if q == p + 1 || self.at(q) != b';' {
            return None;
        }
        if next.is_ascii_alphabetic()
            && self
                .text
                .get(p + 1..q)
                .is_some_and(|name| contains(words::ENTITIES, name))
        {
            return Some(Hit::new(Pat::EntityNamed, p, q + 1).with([p + 1, q, q, 0, 0], 0));
        }
        Some(Hit::new(Pat::EntityAmbiguous, p, p + 1))
    }

    // The root.

    /// `(<)(style)(?=\s|/?>)` at `p`.
    fn style_open(&self, p: usize) -> bool {
        if self.at(p) != b'<' || !self.starts_ci(p + 1, b"style") {
            return false;
        }
        let q = p + 6;
        self.ws_len(q) > 0 || self.at(q) == b'>' || self.starts(q, b"/>")
    }

    /// The lookahead `<name\b(?!-)` of a wrapper rule at `p`.
    fn wrapper_ahead(&self, p: usize, name: &[u8]) -> bool {
        if self.at(p) != b'<' || !self.starts_ci(p + 1, name) {
            return false;
        }
        let q = p + 1 + name.len();
        let boundary = self.word_len(q).is_none();
        boundary && self.at(q) != b'-'
    }

    /// The patterns of the root.
    fn root_patterns(&self, p: usize) -> Option<Hit> {
        let b = self.at(p);
        if p == 0 && matches!(b, b' ' | b'\t') {
            let mut q = 0;
            while matches!(self.at(q), b' ' | b'\t') {
                q += 1;
            }
            self.spend(q / 8);
            if self.wrapper_ahead(q, b"style") {
                return Some(Hit::new(Pat::StyleWrap, 0, q));
            }
            if self.wrapper_ahead(q, b"script") {
                return Some(Hit::new(Pat::ScriptWrap, 0, q));
            }
            return None;
        }
        if b == b'&' {
            return self.entity(p);
        }
        if b != b'<' {
            return None;
        }
        if self.starts(p, b"<?xml") {
            return Some(Hit::new(Pat::Processing, p, p + 5));
        }
        if self.starts(p, b"<!--") {
            return Some(Hit::new(Pat::Comment, p, p + 4));
        }
        if self.starts(p, b"<!") && self.starts_ci(p + 2, b"doctype") && self.ws_len(p + 9) > 0 {
            return Some(Hit::new(Pat::Doctype, p, p + 2));
        }
        if self.starts(p, b"<![CDATA[") {
            return Some(Hit::new(Pat::Cdata, p, p + 9));
        }
        if self.wrapper_ahead(p, b"style") {
            return Some(Hit::new(Pat::StyleWrap, p, p));
        }
        if self.wrapper_ahead(p, b"script") {
            return Some(Hit::new(Pat::ScriptWrap, p, p));
        }
        self.html_tag(p)
            .or_else(|| self.element_root(p, Namespace::Math))
            .or_else(|| self.element_root(p, Namespace::Svg))
            .or_else(|| self.custom_tag(p))
            .or_else(|| self.invalid_tag(p))
    }

    /// The name of a tag at `p` that the lists could hold, and where it ends:
    /// `<name(?=\s|/?>)` or `</name(?=\s|/?>)`.
    /// No name of the lists is longer than `MAX_NAME`, so a longer run is no
    /// match.
    fn tag_name(&self, p: usize) -> Option<(usize, usize, bool)> {
        let closing = self.at(p + 1) == b'/';
        let start = p + 1 + usize::from(closing);
        let mut q = start;
        while q < self.lim {
            if q - start > MAX_NAME {
                return None;
            }
            let b = self.s[q];
            if b >= 0x80 {
                if self.ws_len(q) > 0 {
                    break;
                }
                return None;
            }
            if crate::classes::is_ascii_space(b) || b == b'/' || b == b'>' {
                break;
            }
            q += 1;
        }
        self.spend((q - start) / 8);
        if q == start {
            return None;
        }
        let follows = self.ws_len(q) > 0
            || self.at(q) == b'>'
            || (self.at(q) == b'/' && self.at(q + 1) == b'>');
        follows.then_some((start, q, closing))
    }

    /// A tag of the grammar's own lists at `p`.
    fn html_tag(&self, p: usize) -> Option<Hit> {
        let (start, end, closing) = self.tag_name(p)?;
        let mut buffer = [0; MAX_NAME];
        let name = lowered(&self.s[start..end], &mut buffer)?;
        let lists: &[(&[&str], Category, Deprecation, bool)] = &[
            (words::METADATA, Category::Metadata, Deprecation::No, false),
            (
                words::STRUCTURE,
                Category::Structure,
                Deprecation::No,
                false,
            ),
            (words::INLINE, Category::Inline, Deprecation::No, false),
            (words::OBJECT, Category::Object, Deprecation::No, false),
            (
                words::DEPRECATED_STRUCTURE,
                Category::Structure,
                Deprecation::Deprecated,
                false,
            ),
            (
                words::DEPRECATED_INLINE,
                Category::Inline,
                Deprecation::Deprecated,
                false,
            ),
            (
                words::DEPRECATED_OBJECT,
                Category::Object,
                Deprecation::Deprecated,
                false,
            ),
            (
                words::UNSUPPORTED,
                Category::Other,
                Deprecation::Unsupported,
                false,
            ),
        ];
        let voids: &[(&[&str], Category, Deprecation, bool)] = &[
            (
                words::METADATA_VOID,
                Category::Metadata,
                Deprecation::No,
                true,
            ),
            (
                words::STRUCTURE_VOID,
                Category::Structure,
                Deprecation::No,
                true,
            ),
            (words::INLINE_VOID, Category::Inline, Deprecation::No, true),
            (words::OBJECT_VOID, Category::Object, Deprecation::No, true),
            (
                words::DEPRECATED_METADATA_VOID,
                Category::Metadata,
                Deprecation::Deprecated,
                true,
            ),
            (
                words::DEPRECATED_OBJECT_VOID,
                Category::Object,
                Deprecation::Deprecated,
                true,
            ),
        ];
        let tag = |category, deprecation: Deprecation, kind| {
            let info = TagInfo { category, kind }.bits() | deprecation.bits() << 8;
            Hit::new(Pat::Tag, p, end).with([start, end, 0, 0, 0], info)
        };
        if closing {
            return lists
                .iter()
                .find(|(list, ..)| contains(list, name))
                .map(|&(_, category, deprecation, _)| tag(category, deprecation, TagKind::End));
        }
        voids
            .iter()
            .find(|(list, ..)| contains(list, name))
            .map(|&(_, category, deprecation, _)| tag(category, deprecation, TagKind::Void))
            .or_else(|| {
                lists.iter().find(|(list, ..)| contains(list, name)).map(
                    |&(_, category, deprecation, _)| tag(category, deprecation, TagKind::Start),
                )
            })
    }

    /// Whether `c` can be in a custom element's name besides `-`.
    fn pcen(c: char) -> bool {
        matches!(c,
            '.' | '0'..='9' | '_' | 'A'..='Z' | 'a'..='z' | '\u{b7}'
            | '\u{c0}'..='\u{d6}' | '\u{d8}'..='\u{f6}' | '\u{f8}'..='\u{37d}'
            | '\u{37f}'..='\u{1fff}' | '\u{200c}'..='\u{200d}' | '\u{203f}'..='\u{2040}'
            | '\u{2070}'..='\u{218f}' | '\u{2c00}'..='\u{2fef}' | '\u{3001}'..='\u{d7ff}'
            | '\u{f900}'..='\u{fdcf}' | '\u{fdf0}'..='\u{fffd}' | '\u{10000}'..='\u{effff}')
    }

    /// A custom element's tag at `p`: `<name-name` and `</name-name`.
    fn custom_tag(&self, p: usize) -> Option<Hit> {
        let closing = self.at(p + 1) == b'/';
        let start = p + 1 + usize::from(closing);
        if !self.at(start).is_ascii_alphabetic() {
            return None;
        }
        let mut q = start + 1;
        let mut hyphen = false;
        while let Some((c, n)) = self.char_at(q) {
            if c == '-' {
                hyphen = true;
            } else if !Self::pcen(c) {
                break;
            }
            q += n;
        }
        self.spend((q - start) / 8);
        let follows = self.ws_len(q) > 0
            || self.at(q) == b'>'
            || (self.at(q) == b'/' && self.at(q + 1) == b'>');
        if !hyphen || !follows {
            return None;
        }
        let kind = if closing {
            TagKind::CustomEnd
        } else {
            TagKind::CustomStart
        };
        let info = TagInfo {
            category: Category::Other,
            kind,
        }
        .bits();
        Some(Hit::new(Pat::Tag, p, q).with([start, q, 0, 0, 0], info))
    }

    /// `(</?)((\w[^>\s]*))(?<!/)`: a tag the grammar does not know.
    fn invalid_tag(&self, p: usize) -> Option<Hit> {
        let closing = self.at(p + 1) == b'/';
        let start = p + 1 + usize::from(closing);
        let first = self.word_len(start)?;
        let mut q = start + first;
        while q < self.lim && self.ws_len(q) == 0 && self.s[q] != b'>' {
            q += crate::utf8::char_len(self.s[q]);
        }
        let mut q = q.min(self.lim);
        self.spend((q - start) / 8);
        while q > start + first && self.s[q - 1] == b'/' {
            q -= 1;
        }
        let info = TagInfo {
            category: Category::Other,
            kind: TagKind::Unrecognized,
        }
        .bits();
        Some(Hit::new(Pat::Tag, p, q).with([start, q, 0, 0, 0], info))
    }

    // Elements of an SVG or MathML tree.

    /// The end of `(([^"'>]|"[^"]*"|'[^']*')*)` from `from`, which is where
    /// the run of attributes stops: at a `>`, or at a quote that nothing
    /// closes.
    fn attribute_run(&self, from: usize) -> usize {
        let mut i = from;
        while i < self.lim {
            match self.s[i] {
                b'>' => break,
                quote @ (b'"' | b'\'') => match memchr(quote, &self.s[i + 1..self.lim]) {
                    Some(close) => i += close + 2,
                    None => break,
                },
                _ => i += 1,
            }
        }
        self.spend((i - from) / 4);
        i
    }

    /// An element's tag at `p` that opens a rule, with the attributes of
    /// the start tag when the line holds it whole: `(?:(attrs)(>))?`.
    fn element_open(&self, p: usize, name_start: usize, name_end: usize, info: ElementInfo) -> Hit {
        let run = self.attribute_run(name_end);
        let hit = Hit::new(Pat::Element, p, name_end);
        if run < self.lim && self.s[run] == b'>' {
            let mut hit = hit.with([name_start, name_end, name_end, run, 0], info.bits());
            hit.end = run + 1;
            return hit;
        }
        hit.with([name_start, name_end, usize::MAX, 0, 0], info.bits())
    }

    /// An element's tag at `p` that is one match: `(attrs)(/>)`.
    fn element_whole(
        &self,
        p: usize,
        name_start: usize,
        name_end: usize,
        info: ElementInfo,
    ) -> Option<Hit> {
        let run = self.attribute_run(name_end);
        if run < self.lim && self.s[run] == b'>' && run > name_end && self.s[run - 1] == b'/' {
            let mut hit = Hit::new(Pat::ElementVoid, p, run + 1);
            hit.at = [name_start, name_end, name_end, run - 1, 0];
            hit.info = info.bits();
            return Some(hit);
        }
        None
    }

    /// An `svg` or `math` element at `p`: the root of a tree.
    fn element_root(&self, p: usize, namespace: Namespace) -> Option<Hit> {
        let (start, end, closing) = self.tag_name(p)?;
        if closing || !self.s[start..end].eq_ignore_ascii_case(namespace.word().as_bytes()) {
            return None;
        }
        let info = ElementInfo {
            namespace,
            category: Category::Structure,
            flavor: Flavor::Known,
            root: true,
        };
        Some(self.element_open(p, start, end, info))
    }

    /// The patterns of an element's content: its start tag, comments and
    /// the namespace's tags.
    fn element_patterns(&self, p: usize) -> Option<Hit> {
        let top = self.top();
        let element = ElementInfo::from_bits(super::state::info(top.payload));
        if self.g(p) && self.char_before(p) != Some('>') {
            return Some(Hit::new(Pat::Zero(Rule::ElementStart), p, p));
        }
        if self.at(p) != b'<' {
            return None;
        }
        if self.starts(p, b"<!--") {
            return Some(Hit::new(Pat::Comment, p, p + 4));
        }
        if self.starts(p, b"<![CDATA[") {
            return Some(Hit::new(Pat::Cdata, p, p + 9));
        }
        self.child_element(p, element.namespace)
            .or_else(|| self.invalid_tag(p))
    }

    /// An element inside an `svg` or `math` element, from the namespace's
    /// lists and then the tags it does not list.
    /// The patterns are `(<)(([:\w]+))(?=\s|/?>)(attrs)(/>)` and
    /// `(<)((\w[^>\s]*))(?=\s|/?>)`.
    fn child_element(&self, p: usize, namespace: Namespace) -> Option<Hit> {
        if self.at(p + 1) == b'/' {
            return None;
        }
        if let Some((start, end, _)) = self.tag_name(p) {
            let mut buffer = [0; MAX_NAME];
            if let Some(name) = lowered(&self.s[start..end], &mut buffer) {
                let categories = match namespace {
                    Namespace::Svg => SVG_LISTS,
                    Namespace::Math => MATH_LISTS,
                };
                for &(category, void_list, open_list, flavor) in categories {
                    let info = ElementInfo {
                        namespace,
                        category,
                        flavor,
                        root: false,
                    };
                    if contains(void_list, name)
                        && let Some(hit) = self.element_whole(p, start, end, info)
                    {
                        return Some(hit);
                    }
                    if contains(open_list, name) {
                        return Some(self.element_open(p, start, end, info));
                    }
                }
            }
        }
        let info = ElementInfo {
            namespace,
            category: Category::Other,
            flavor: Flavor::Invalid,
            root: false,
        };
        let start = p + 1;
        let mut q = start;
        while let Some((c, n)) = self.char_at(q) {
            if c == ':' || is_word(c) {
                q += n;
            } else {
                break;
            }
        }
        let follows = |q: usize| {
            self.ws_len(q) > 0
                || self.at(q) == b'>'
                || (self.at(q) == b'/' && self.at(q + 1) == b'>')
        };
        if q > start
            && follows(q)
            && let Some(hit) = self.element_whole(p, start, q, info)
        {
            return Some(hit);
        }
        let first = self.word_len(start)?;
        let mut q = start + first;
        while q < self.lim && self.ws_len(q) == 0 && self.s[q] != b'>' {
            q += crate::utf8::char_len(self.s[q]);
        }
        let q = q.min(self.lim);
        (follows(q) && (q < self.lim || self.lf)).then(|| self.element_open(p, start, q, info))
    }

    // Attributes.

    /// The patterns of the attribute list `which` at `p`.
    /// A name of a list is the whole of the longest run of `[-:\w]` that starts
    /// at `p`, as the lookahead after the name requires; any other name is
    /// `[^>\s]+`.
    fn attribute_patterns(&self, p: usize, which: Attributes) -> Option<Hit> {
        if p >= self.lim || crate::classes::is_ascii_space(self.s[p]) || self.s[p] == b'>' {
            return None;
        }
        let mut run = p;
        while let Some((c, n)) = self.char_at(run) {
            if c == '-' || c == ':' || is_word(c) {
                run += n;
            } else {
                break;
            }
        }
        self.spend((run - p) / 8);
        let word = self.text.get(p..run).unwrap_or("");
        let attribute = |kind: AttributeKind, from: usize, to: usize| {
            Hit::new(Pat::Attribute, p, run).with([from, to, 0, 0, 0], kind.bits())
        };
        let list = match which {
            Attributes::Html => words::HTML_ATTRIBUTES,
            Attributes::Svg => words::SVG_ATTRIBUTES,
            Attributes::Math => words::MATH_ATTRIBUTES,
        };
        if run > p {
            if contains(list, word) {
                return Some(attribute(AttributeKind::Known, p, run));
            }
            if which == Attributes::Html {
                if word == "style" {
                    return Some(attribute(AttributeKind::Style, p, run));
                }
                if let Some(rest) = word.strip_prefix("on")
                    && contains(words::EVENT_HANDLERS, rest)
                {
                    return Some(attribute(AttributeKind::Event, p + 2, run));
                }
                if let Some(rest) = word.strip_prefix("data-")
                    && !rest.is_empty()
                    && rest.bytes().all(|b| b == b'-' || b.is_ascii_lowercase())
                {
                    return Some(attribute(AttributeKind::Data, p, run));
                }
                if matches!(word, "align" | "bgcolor" | "border") {
                    return Some(attribute(AttributeKind::Deprecated, p, run));
                }
            }
        }
        let mut q = p;
        while let Some((c, n)) = self.char_at(q) {
            if Self::attribute_character(c) {
                q += n;
            } else {
                break;
            }
        }
        self.spend((q - p) / 8);
        if q > p {
            return Some(
                Hit::new(Pat::Attribute, p, q)
                    .with([p, q, 0, 0, 0], AttributeKind::Unrecognized.bits()),
            );
        }
        let mut q = p;
        while q < self.lim && self.ws_len(q) == 0 && self.s[q] != b'>' {
            q += crate::utf8::char_len(self.s[q]);
        }
        self.spend((q - p) / 8);
        Some(Hit::new(Pat::AttributeIllegal, p, q.min(self.lim)))
    }

    /// The class of a character of an unrecognized attribute's name.
    fn attribute_character(c: char) -> bool {
        let code = u32::from(c);
        !(code <= 0x20
            || matches!(c, '"' | '\'' | '/' | '<' | '=' | '>')
            || (0x7f..=0x9f).contains(&code)
            || (0xfdd0..=0xfdef).contains(&code)
            || code & 0xfffe == 0xfffe)
    }

    // The type of a script.

    /// The end of a JavaScript type at `p` (`text/javascript` and the
    /// like) that is followed by a quote, `>` or white space.
    fn javascript_type(&self, p: usize) -> bool {
        const TYPES: &[&[u8]] = &[
            b"text/javascript1.0",
            b"text/javascript1.1",
            b"text/javascript1.2",
            b"text/javascript1.3",
            b"text/javascript1.4",
            b"text/javascript1.5",
            b"text/javascript",
            b"text/x-javascript",
            b"text/jscript",
            b"text/livescript",
            b"text/ecmascript",
            b"text/x-ecmascript",
            b"text/babel",
            b"application/javascript",
            b"application/x-javascript",
            b"application/ecmascript",
            b"application/x-ecmascript",
            b"module",
        ];
        TYPES.iter().any(|word| {
            self.starts_ci(p, word) && {
                let after = p + word.len();
                matches!(self.at(after), b'"' | b'\'' | b'>') || self.ws_len(after) > 0
            }
        })
    }

    /// `type(?=[=\s])(?!\s*=\s*(''|""|(["']?)(JavaScript)["'>\s]))` at `p`:
    /// a `type` attribute whose value is not JavaScript.
    fn script_type_ends(&self, p: usize) -> bool {
        if !self.starts_ci(p, b"type") {
            return false;
        }
        let a = p + 4;
        if self.at(a) != b'=' && self.ws_len(a) == 0 {
            return false;
        }
        let q = self.skip_ws(a);
        if self.at(q) != b'=' {
            return true;
        }
        let q = self.skip_ws(q + 1);
        if self.starts(q, b"''") || self.starts(q, b"\"\"") {
            return false;
        }
        let quoted = matches!(self.at(q), b'"' | b'\'');
        if quoted && self.javascript_type(q + 1) {
            return false;
        }
        !self.javascript_type(q)
    }

    /// `type\s*=\s*(["']?)text/(template|html…)["'>\s]` at `p`: a `type`
    /// attribute that names an HTML template.
    fn script_template_ahead(&self, p: usize) -> bool {
        const NAMES: &[&[u8]] = &[
            b"x-handlebars-template",
            b"x-handlebars",
            b"x-template",
            b"ng-template",
            b"template",
            b"html",
        ];
        if !self.starts_ci(p, b"type") {
            return false;
        }
        let q = self.skip_ws(p + 4);
        if self.at(q) != b'=' {
            return false;
        }
        let q = self.skip_ws(q + 1);
        let q = q + usize::from(matches!(self.at(q), b'"' | b'\''));
        if !self.starts_ci(q, b"text/") {
            return false;
        }
        NAMES.iter().any(|word| {
            self.starts_ci(q + 5, word) && {
                let after = q + 5 + word.len();
                matches!(self.at(after), b'"' | b'\'' | b'>') || self.ws_len(after) > 0
            }
        })
    }
}

/// The element lists of a namespace: the category, the elements that are one
/// match, the elements that open a rule, and the flavour of both.
type Lists = (
    Category,
    &'static [&'static str],
    &'static [&'static str],
    Flavor,
);

/// The `svg` lists, in the grammar's order.
const SVG_LISTS: &[Lists] = &[
    (
        Category::Metadata,
        words::SVG_METADATA_VOID,
        words::SVG_METADATA,
        Flavor::Known,
    ),
    (
        Category::Structure,
        words::SVG_STRUCTURE_VOID,
        words::SVG_STRUCTURE,
        Flavor::Known,
    ),
    (
        Category::Inline,
        words::SVG_INLINE_VOID,
        words::SVG_INLINE,
        Flavor::Known,
    ),
    (
        Category::Object,
        words::SVG_OBJECT_VOID,
        words::SVG_OBJECT,
        Flavor::Known,
    ),
    (
        Category::Other,
        words::SVG_OTHER_VOID,
        words::SVG_OTHER,
        Flavor::Deprecated,
    ),
];

/// The `math` lists, in the grammar's order.
const MATH_LISTS: &[Lists] = &[
    (
        Category::Structure,
        words::MATH_STRUCTURE_VOID,
        words::MATH_STRUCTURE,
        Flavor::Known,
    ),
    (
        Category::Inline,
        words::MATH_INLINE_VOID,
        words::MATH_INLINE,
        Flavor::Known,
    ),
    (
        Category::Object,
        words::MATH_OBJECT_VOID,
        words::MATH_OBJECT,
        Flavor::Known,
    ),
];
