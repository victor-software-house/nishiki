//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! The grammar names only whole matches, so no matcher records a group.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would.
//! Where a quantifier could give characters back, the doc comment says why it
//! cannot matter.
//! `\w`, `\s` and `\d` take non-ASCII characters, as in Oniguruma, through
//! [`Chars`] and [`crate::classes`].
//! A lookbehind reads the whole line, the host's text included where another
//! language embeds PostCSS, as vscode-textmate's does.

use super::engine::Lexer;
use super::grammar::Pattern;
use crate::bytes;
use crate::classes::{is_digit, is_space};
use crate::rules::Chars;

/// The units of `unit`, in the grammar's order.
/// No unit is a prefix of one after it, so the first that matches is the one
/// the alternation takes.
const UNITS: &[&[u8]] = &[
    b"ch", b"cm", b"deg", b"dpcm", b"dpi", b"dppx", b"em", b"ex", b"grad", b"Hz", b"in", b"kHz",
    b"mm", b"ms", b"pc", b"pt", b"px", b"rad", b"rem", b"s", b"turn", b"vh", b"vmax", b"vmin",
    b"vw", b"%",
];

/// The flags after `!`.
const FLAGS: &[&[u8]] = &[b"important", b"default", b"optional", b"global"];

/// The media words an at-rule's space may stand before without ending it.
const MEDIA: &[&[u8]] = &[
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

/// The tag names of the selector rule, `h1` to `h6`, `u` and `x` included,
/// sorted for a binary search.
const TAGS: &[&[u8]] = &[
    b"a",
    b"abbr",
    b"acronym",
    b"address",
    b"applet",
    b"area",
    b"article",
    b"aside",
    b"audio",
    b"b",
    b"base",
    b"big",
    b"blockquote",
    b"body",
    b"br",
    b"button",
    b"canvas",
    b"caption",
    b"center",
    b"circle",
    b"cite",
    b"code",
    b"col",
    b"colgroup",
    b"datalist",
    b"dd",
    b"del",
    b"details",
    b"dfn",
    b"dialog",
    b"div",
    b"dl",
    b"dt",
    b"ellipse",
    b"em",
    b"embed",
    b"eventsource",
    b"fieldset",
    b"figcaption",
    b"figure",
    b"footer",
    b"form",
    b"frame",
    b"frameset",
    b"h1",
    b"h2",
    b"h3",
    b"h4",
    b"h5",
    b"h6",
    b"head",
    b"header",
    b"hgroup",
    b"hr",
    b"html",
    b"i",
    b"iframe",
    b"img",
    b"input",
    b"ins",
    b"kbd",
    b"label",
    b"legend",
    b"li",
    b"line",
    b"link",
    b"main",
    b"map",
    b"mark",
    b"menu",
    b"meta",
    b"meter",
    b"nav",
    b"noframes",
    b"noscript",
    b"object",
    b"ol",
    b"optgroup",
    b"option",
    b"output",
    b"p",
    b"param",
    b"path",
    b"picture",
    b"polygon",
    b"polyline",
    b"pre",
    b"progress",
    b"q",
    b"rect",
    b"ruby",
    b"samp",
    b"script",
    b"section",
    b"select",
    b"small",
    b"source",
    b"span",
    b"strike",
    b"strong",
    b"style",
    b"sub",
    b"summary",
    b"sup",
    b"svg",
    b"table",
    b"tbody",
    b"td",
    b"text",
    b"textarea",
    b"tfoot",
    b"th",
    b"thead",
    b"time",
    b"title",
    b"tr",
    b"tt",
    b"u",
    b"ul",
    b"var",
    b"video",
    b"x",
];

/// The words of `reserved-words`.
const RESERVED: &[&[u8]] = &[
    b"false", b"from", b"in", b"not", b"null", b"through", b"to", b"true",
];

/// Whether `b` is in the grammar's `[-:a-z]`.
fn is_pseudo(b: u8) -> bool {
    b == b'-' || b == b':' || b.is_ascii_lowercase()
}

/// Whether `b` is in the grammar's `[-a-z]`.
fn is_property(b: u8) -> bool {
    b == b'-' || b.is_ascii_lowercase()
}

impl Lexer<'_> {
    /// Whether `pattern` matches at `p`, and where its match ends.
    ///
    /// The arms follow the grammar's order: comments, strings and
    /// interpolations, values, at-rules, selectors, then properties.
    pub(super) fn matcher(&mut self, pattern: Pattern, p: usize) -> Option<usize> {
        match pattern {
            Pattern::RootAt0 => self.lit(p, b"/*"),
            Pattern::RootAt0End => self.lit(p, b"*/"),
            Pattern::CommentTag => self.lit(p, b"{{"),
            Pattern::CommentTagEnd => self.lit(p, b"}}"),
            Pattern::CommentTagAt0 | Pattern::RootAt13At0 | Pattern::PropertyValue => {
                self.dash_words(p)
            }
            Pattern::DoubleSlash => self.lit(p, b"//"),
            Pattern::DoubleSlashEnd => self.at_eol(p).then_some(p),
            Pattern::DoubleQuoted | Pattern::DoubleQuotedEnd => self.lit(p, b"\""),
            Pattern::SingleQuoted | Pattern::SingleQuotedEnd => self.lit(p, b"'"),
            Pattern::QuotedInterpolation | Pattern::Interpolation => self.lit(p, b"#{"),
            Pattern::QuotedInterpolationEnd | Pattern::InterpolationEnd => self.lit(p, b"}"),
            Pattern::Variable => self.dash_words(self.lit(p, b"$")?),
            Pattern::Numeric => self.numeric(p),
            Pattern::Operator => self.operator(p),
            Pattern::Unit => self.unit(p),
            Pattern::PlaceholderSelector => {
                let end = self.lit(p, b"%")?;
                (!self.digit_before(p) && !self.digit_at(end)).then_some(end)
            }
            Pattern::PlaceholderSelectorEnd => self
                .line_end(p)
                .or_else(|| self.space(p))
                .or_else(|| matches!(self.at(p), b';' | b'{').then_some(p)),
            Pattern::VariableRootCss => {
                if self.prev(p) == Some(b'&') {
                    return None;
                }
                self.dash_words(self.lit(p, b"--")?)
            }
            Pattern::Flag => {
                let name = self.lit(p, b"!")?;
                FLAGS.iter().find_map(|flag| self.lit(name, flag))
            }
            Pattern::Dotdotdot => self.lit(p, b"..."),
            Pattern::RootAt12 => self.lit(p, b"@include"),
            Pattern::RootAt12End => matches!(self.at(p), b'\n' | b'(' | b';' | b'{').then_some(p),
            Pattern::RootAt13 => {
                let name = self.lit(p, b"@")?;
                self.lit(name, b"mixin")
                    .or_else(|| self.lit(name, b"function"))
            }
            Pattern::RootAt13End => self
                .line_end(p)
                .or_else(|| matches!(self.at(p), b'(' | b'{').then_some(p)),
            Pattern::RootAt14 => self.import_path(p),
            Pattern::RootAt15 => self.lit(p, b"@"),
            Pattern::RootAt15End => self.at_rule_end(p),
            Pattern::RootAt16 => self.lit(p, b"#"),
            Pattern::RootAt16End => self.line_end(p).or_else(|| {
                (matches!(
                    self.at(p),
                    b'(' | b')' | b',' | b'.' | b';' | b'>' | b'[' | b'{'
                ) || self.space_at(p) > 0)
                    .then_some(p)
            }),
            Pattern::PseudoClass => self.pseudo_class(p),
            Pattern::RootAt17 => self.class_head(p),
            Pattern::RootAt17End => self.line_end(p).or_else(|| {
                (matches!(self.at(p), b'(' | b')' | b',' | b';' | b'>' | b'[' | b'{')
                    || self.space_at(p) > 0)
                    .then_some(p)
            }),
            Pattern::RootAt18 => self.lit(p, b"["),
            Pattern::RootAt18End => self.lit(p, b"]"),
            Pattern::RootAt18At2 => {
                matches!(self.at(p), b'$' | b'*' | b'^' | b'~').then_some(p + 1)
            }
            Pattern::RootAt19 => self.selector_pseudo_class(p),
            Pattern::RootAt20 => self.lit(p, b":"),
            Pattern::RootAt20End => self.property_list_end(p),
            Pattern::RgbValue => self.rgb_value(p),
            Pattern::Function => self.function(p),
            Pattern::FunctionContent => self.function_content(p),
            Pattern::FunctionContentVar => {
                if !self.ends_with(p, b"var(") {
                    return None;
                }
                let end = self.dash_words(p)?;
                (self.at(end) == b')').then_some(end)
            }
            Pattern::ParentSelector => self.lit(p, b"&"),
            Pattern::RootAt24 => self.tag(p),
            Pattern::RootAt24End => (matches!(
                self.at(p),
                b'-' | b'(' | b')' | b',' | b'.' | b';' | b'>' | b'[' | b'_' | b'{'
            ) || self.space_at(p) > 0)
                .then_some(p),
            Pattern::RootAt26 => {
                let end = self.skip(p, is_property);
                (end > p && (self.at(end) == b':' || self.starts(end, b"#{"))).then_some(end)
            }
            Pattern::ReservedWords => self.word_in(p, RESERVED),
        }
    }

    /// `$\n?`: the end of the line, with its line feed when it has one.
    fn line_end(&self, p: usize) -> Option<usize> {
        if !self.at_eol(p) {
            return None;
        }
        Some(if self.at(p) == b'\n' { p + 1 } else { p })
    }

    /// `\s`: the end of the space character at `p`.
    fn space(&self, p: usize) -> Option<usize> {
        let width = self.space_at(p);
        (width > 0).then_some(p + width)
    }

    /// `[-\w]+`: the end of the run from `p`, which must not be empty.
    fn dash_words(&self, p: usize) -> Option<usize> {
        let mut i = p;
        loop {
            if self.at(i) == b'-' {
                i += 1;
            } else if let Some(width) = self.word_at(i) {
                i += width;
            } else {
                break;
            }
        }
        self.spend(i - p);
        (i > p).then_some(i)
    }

    /// Whether the text before `p` ends with `text`.
    fn ends_with(&self, p: usize, text: &[u8]) -> bool {
        p.checked_sub(text.len())
            .is_some_and(|start| self.text(start, p) == text)
    }

    /// Whether the character before `p` is a `\d`.
    fn digit_before(&self, p: usize) -> bool {
        self.char_before(p).is_some_and(is_digit)
    }

    /// Whether the character at `p` is a `\d`.
    fn digit_at(&self, p: usize) -> bool {
        self.char_at(p).is_some_and(|(c, _)| is_digit(c))
    }

    /// `([-.])?[0-9]+(\.[0-9]+)?`.
    ///
    /// A sign or dot with no digit after it is given back.
    /// The digits that follow it are then not `[0-9]`, so nothing matches.
    fn numeric(&self, p: usize) -> Option<usize> {
        let digits = if matches!(self.at(p), b'-' | b'.') {
            p + 1
        } else {
            p
        };
        let end = self.skip(digits, |b| b.is_ascii_digit());
        if end == digits {
            return None;
        }
        if self.at(end) == b'.' {
            let fraction = self.skip(end + 1, |b| b.is_ascii_digit());
            if fraction > end + 1 {
                return Some(fraction);
            }
        }
        Some(end)
    }

    /// `\+|\s-\s|\s-(?=\$)|(?<=\()-(?=\$)|\s-(?=\()|[!%*/<=>~]`, the first
    /// alternative that matches.
    fn operator(&self, p: usize) -> Option<usize> {
        match self.at(p) {
            b'+' | b'!' | b'%' | b'*' | b'/' | b'<' | b'=' | b'>' | b'~' => return Some(p + 1),
            b'-' => {
                return (self.prev(p) == Some(b'(') && self.at(p + 1) == b'$').then_some(p + 1);
            }
            _ => {}
        }
        let dash = self.space(p)?;
        if self.at(dash) != b'-' {
            return None;
        }
        let after = dash + 1;
        self.space(after)
            .or_else(|| matches!(self.at(after), b'$' | b'(').then_some(after))
    }

    /// `(?<=[}\d])(ch|cm|…|vw|%)`.
    fn unit(&self, p: usize) -> Option<usize> {
        if !(self.prev(p) == Some(b'}') || self.digit_before(p)) {
            return None;
        }
        UNITS.iter().find_map(|unit| self.lit(p, unit))
    }

    /// `(?<=@import)\s[-*./\w]+`.
    fn import_path(&self, p: usize) -> Option<usize> {
        if !self.ends_with(p, b"@import") {
            return None;
        }
        let start = self.space(p)?;
        let mut i = start;
        loop {
            if matches!(self.at(i), b'-' | b'*' | b'.' | b'/') {
                i += 1;
            } else if let Some(width) = self.word_at(i) {
                i += width;
            } else {
                break;
            }
        }
        self.spend(i - start);
        (i > start).then_some(i)
    }

    /// The end of an at-rule:
    /// `$\n?|\s(?!(all|…|not)([,\s]))|(?=;)`.
    ///
    /// A space ends the rule unless a media word and a comma or space follow
    /// it.
    fn at_rule_end(&self, p: usize) -> Option<usize> {
        if let Some(end) = self.line_end(p) {
            return Some(end);
        }
        if let Some(after) = self.space(p) {
            let media = MEDIA.iter().any(|word| {
                self.lit(after, word)
                    .is_some_and(|end| self.at(end) == b',' || self.space_at(end) > 0)
            });
            if !media {
                return Some(after);
            }
        }
        (self.at(p) == b';').then_some(p)
    }

    /// `:[-:a-z]+`.
    fn pseudo_class(&self, p: usize) -> Option<usize> {
        let name = self.lit(p, b":")?;
        let end = self.skip(name, is_pseudo);
        (end > name).then_some(end)
    }

    /// `\.|(?<=&)([-_])`.
    /// The grammar names no group of it.
    fn class_head(&self, p: usize) -> Option<usize> {
        (self.at(p) == b'.' || (self.prev(p) == Some(b'&') && matches!(self.at(p), b'-' | b'_')))
            .then_some(p + 1)
    }

    /// `(?<=[])]|not\(|[*>]|>\s):[-:a-z]+|(:[-:])[-:a-z]+`.
    ///
    /// The second alternative needs a character of `[-:a-z]` after its first
    /// two, so `::` alone does not match it.
    fn selector_pseudo_class(&self, p: usize) -> Option<usize> {
        let name = self.lit(p, b":")?;
        let after_space = self.char_before(p).is_some_and(is_space)
            && self.ends_with(self.char_start_before(p), b">");
        let behind = matches!(self.prev(p), Some(b']' | b')' | b'*' | b'>'))
            || self.ends_with(p, b"not(")
            || after_space;
        if behind {
            let end = self.skip(name, is_pseudo);
            if end > name {
                return Some(end);
            }
        }
        if !matches!(self.at(name), b'-' | b':') {
            return None;
        }
        let end = self.skip(name + 1, is_pseudo);
        (end > name + 1).then_some(end)
    }

    /// The end of a property list: `$\n?|(?=;|\s\(|and\(|[{}]|\),)`.
    fn property_list_end(&self, p: usize) -> Option<usize> {
        if let Some(end) = self.line_end(p) {
            return Some(end);
        }
        let ahead = match self.at(p) {
            b';' | b'{' | b'}' => true,
            b')' => self.at(p + 1) == b',',
            b'a' => self.starts(p, b"and("),
            _ => self.space(p).is_some_and(|after| self.at(after) == b'('),
        };
        ahead.then_some(p)
    }

    /// `(#)(\h{3}|\h{6})\b`.
    ///
    /// Three digits that a word character follows give way to six.
    fn rgb_value(&self, p: usize) -> Option<usize> {
        let digits = self.lit(p, b"#")?;
        let end = self.skip(digits, |b| b.is_ascii_hexdigit());
        [3, 6]
            .into_iter()
            .map(|count| digits + count)
            .find(|&stop| stop <= end && self.boundary(stop))
    }

    /// `(?<=[(,:|\s])(?!url|format|attr)[-\w][-\w]*(?=\()`.
    ///
    /// Giving back a character of `[-\w]` cannot put the `(` after the run.
    fn function(&self, p: usize) -> Option<usize> {
        let behind = matches!(self.prev(p), Some(b'(' | b',' | b':' | b'|'))
            || self.char_before(p).is_some_and(is_space);
        if !behind
            || [&b"url"[..], b"format", b"attr"]
                .iter()
                .any(|word| self.starts(p, word))
        {
            return None;
        }
        let end = self.dash_words(p)?;
        (self.at(end) == b'(').then_some(end)
    }

    /// `(?<=url\(|format\(|attr\().+?(?=\))`: at least one character, up to
    /// the first `)` after it, within the line.
    fn function_content(&self, p: usize) -> Option<usize> {
        if !(self.ends_with(p, b"url(")
            || self.ends_with(p, b"format(")
            || self.ends_with(p, b"attr("))
        {
            return None;
        }
        if p >= self.lim() {
            return None;
        }
        let from = self.char_end(p);
        let close = memchr::memchr(b')', self.text(from, self.lim()))?;
        self.spend(close + 1);
        Some(from + close)
    }

    /// The tag rule's begin:
    /// `(?<![-(])\b(a|abbr|…|[ux])\b(?![-)]|:\s)|&`.
    ///
    /// Every tag is word characters, so only the one that spans the whole
    /// word at `p` reaches the `\b` after it.
    fn tag(&self, p: usize) -> Option<usize> {
        if self.at(p) == b'&' {
            return Some(p + 1);
        }
        if matches!(self.prev(p), Some(b'-' | b'(')) || !self.boundary(p) {
            return None;
        }
        let end = self.word_end(p);
        let word = self.text(p, end);
        if TAGS
            .binary_search_by(|tag| bytes::order(tag, word))
            .is_err()
        {
            return None;
        }
        let blocked = matches!(self.at(end), b'-' | b')')
            || (self.at(end) == b':' && self.space_at(end + 1) > 0);
        (!blocked).then_some(end)
    }
}
